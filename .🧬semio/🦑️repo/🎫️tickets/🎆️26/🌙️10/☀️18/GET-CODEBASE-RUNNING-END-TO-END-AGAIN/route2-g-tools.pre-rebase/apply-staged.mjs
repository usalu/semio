/** 🧪 route2-g: applies every staged file to the library tree (run ONLY after 11:30), refusing a file whose original changed since it was staged. usage: bun apply-staged.mjs [--dry] */
import {readFileSync,writeFileSync,readdirSync,statSync,existsSync,mkdirSync} from "node:fs";
import {createHash} from "node:crypto";
import {join,dirname} from "node:path";
const root="/Users/ueli/Documents/semio",C=root+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN",T=C+"/route2-g-tools",build=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",library=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const manifest=existsSync(T+"/staged-base.json")?JSON.parse(readFileSync(T+"/staged-base.json","utf8")):{};
const sha=(bytes)=>createHash("sha256").update(bytes).digest("hex");
const walk=(d,out=[])=>{for(const e of readdirSync(d)){const p=join(d,e);statSync(p).isDirectory()?walk(p,out):out.push(p);}return out;};
for(const file of [...walk(T+"/staged"),...(existsSync(T+"/staged-lib")?walk(T+"/staged-lib"):[])]){const lib=file.startsWith(T+"/staged-lib/"),rel=file.slice((T+(lib?"/staged-lib/":"/staged/")).length),key=(lib?"lib:":"")+rel,target=(lib?library:build)+rel,now=existsSync(target)?sha(readFileSync(target)):"absent";const only=process.argv.slice(2).filter(a=>!a.startsWith("--"));if(only.length&&!only.some(prefix=>key.startsWith(prefix)))continue;if(now===sha(readFileSync(file)))continue;
 if(manifest[key]&&manifest[key]!==now){console.log("CHANGED-SINCE-STAGING "+key);continue;}
 if(process.argv.includes("--dry")){console.log("would apply "+rel);continue;}
 mkdirSync(dirname(target),{recursive:true});writeFileSync(target,readFileSync(file));console.log("applied "+rel);}
