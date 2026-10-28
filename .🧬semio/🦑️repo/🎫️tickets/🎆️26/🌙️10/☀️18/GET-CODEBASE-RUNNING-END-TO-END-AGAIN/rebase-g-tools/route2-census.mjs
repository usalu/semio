/** 🧪 rebase-g: route2-g census: manifest keys vs staged files vs live (read-only). usage: bun route2-census.mjs */
import {readFileSync,readdirSync,statSync,existsSync} from "node:fs";
import {createHash} from "node:crypto";
import {join} from "node:path";
const root="/Users/ueli/Documents/semio",C=root+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN",T=C+"/route2-g-tools",library=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/",build=library+"🏗️build/";
const sha=b=>createHash("sha256").update(b).digest("hex");
const walk=(d,out=[])=>{if(!existsSync(d))return out;for(const e of readdirSync(d)){const p=join(d,e);statSync(p).isDirectory()?walk(p,out):out.push(p);}return out;};
const manifest=JSON.parse(readFileSync(T+"/staged-base.json","utf8"));
const staged=new Map();
for(const f of walk(T+"/staged"))staged.set(f.slice((T+"/staged/").length),f);
for(const f of walk(T+"/staged-lib"))staged.set("lib:"+f.slice((T+"/staged-lib/").length),f);
const rows={ok:[],applied:[],changed:[],unstaged:[],orphan:[]};
for(const [key,file] of staged){const target=key.startsWith("lib:")?library+key.slice(4):build+key,now=existsSync(target)?sha(readFileSync(target)):"absent";
 if(now===sha(readFileSync(file)))rows.applied.push(key);else if(manifest[key]===undefined)rows.orphan.push(key);else if(manifest[key]!==now)rows.changed.push(key);else rows.ok.push(key);}
for(const key of Object.keys(manifest))if(!staged.has(key)){const target=key.startsWith("lib:")?library+key.slice(4):build+key;rows.unstaged.push(key+" live "+(existsSync(target)?sha(readFileSync(target)).slice(0,10):"absent")+" base "+manifest[key].slice(0,10));}
for(const [k,v] of Object.entries(rows))console.log(k,v.length,...v.map(x=>"\n  "+x));
