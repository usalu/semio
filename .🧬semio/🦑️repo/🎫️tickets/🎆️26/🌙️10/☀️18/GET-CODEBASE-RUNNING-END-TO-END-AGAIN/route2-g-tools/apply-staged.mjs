/** 🧪 route2-g: applies every staged file to the library tree (gate-g, in one compile-atomic batch), REFUSING (exit 2, nothing written) when a manifest key has no staged copy, when a staged file has no manifest key, or when the original of a file changed since it was staged. usage: bun apply-staged.mjs [--dry] [--check] [key-prefix...] ; --check = refusal verdict only (exit 0 clean, 2 refused) */
import {readFileSync,writeFileSync,readdirSync,statSync,existsSync,mkdirSync} from "node:fs";
import {createHash} from "node:crypto";
import {join,dirname} from "node:path";
const root="/Users/ueli/Documents/semio",C=root+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN",T=C+"/route2-g-tools",build=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",library=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const manifest=existsSync(T+"/staged-base.json")?JSON.parse(readFileSync(T+"/staged-base.json","utf8")):{};
const sha=(bytes)=>createHash("sha256").update(bytes).digest("hex");
const walk=(d,out=[])=>{if(!existsSync(d))return out;for(const e of readdirSync(d)){const p=join(d,e);statSync(p).isDirectory()?walk(p,out):out.push(p);}return out;};
const only=process.argv.slice(2).filter(a=>!a.startsWith("--")),selected=key=>!only.length||only.some(prefix=>key.startsWith(prefix));
const staged=new Map();
for(const file of walk(T+"/staged"))staged.set(file.slice((T+"/staged/").length),file);
for(const file of walk(T+"/staged-lib"))staged.set("lib:"+file.slice((T+"/staged-lib/").length),file);
const targetOf=key=>key.startsWith("lib:")?library+key.slice(4):build+key,live=key=>existsSync(targetOf(key))?sha(readFileSync(targetOf(key))):"absent";
const plan=[],refused=[];
for(const key of Object.keys(manifest))if(selected(key)&&!staged.has(key))refused.push("UNSTAGED "+key+" (manifest base "+manifest[key].slice(0,10)+", live "+live(key).slice(0,10)+"): stage it again or drop it from staged-base.json");
for(const [key,file] of staged){if(!selected(key))continue;const now=live(key);if(now===sha(readFileSync(file)))continue;
 if(manifest[key]===undefined)refused.push("NO-BASE "+key+" (staged file without a manifest entry)");
 else if(manifest[key]!==now)refused.push("CHANGED-SINCE-STAGING "+key+" (base "+manifest[key].slice(0,10)+", live "+now.slice(0,10)+")");
 else plan.push([key,file]);}
if(refused.length){console.log(refused.join("\n"));console.log("refused: "+refused.length+" problem(s), nothing written");process.exit(2);}
if(process.argv.includes("--check")){console.log("clean: "+plan.length+" file(s) would apply");process.exit(0);}
for(const [key,file] of plan){if(process.argv.includes("--dry")){console.log("would apply "+key);continue;}
 mkdirSync(dirname(targetOf(key)),{recursive:true});writeFileSync(targetOf(key),readFileSync(file));console.log("applied "+key);}
