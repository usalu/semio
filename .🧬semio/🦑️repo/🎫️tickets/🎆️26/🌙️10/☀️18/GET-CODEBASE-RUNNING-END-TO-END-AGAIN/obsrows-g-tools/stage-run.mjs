/** 🧪 obsrows-g: runs a staged replacement of a build-library test file under a genuine birth without touching the library tree (route2-g's transform). usage: bun stage-run.mjs <build-relative-test-file> <log> [name-pattern] [birth] */
import {readFileSync,writeFileSync,mkdirSync,existsSync,rmSync} from "node:fs";
import {dirname,resolve,join} from "node:path";
import {pathToFileURL} from "node:url";
import {spawnSync} from "node:child_process";
const root="/Users/ueli/Documents/semio",C=root+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN",T=C+"/obsrows-g-tools",build=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const [rel,log,pattern="",birth="setup-sdk"]=process.argv.slice(2),orig=build+rel,staged=T+"/staged/"+rel;
if(!existsSync(staged)){console.log("no staged file for "+rel);process.exit(2);}
const outDir=C+"/🗑️generated/obsrows-g/stage/"+rel.split("/").length+"-"+Buffer.from(rel).toString("hex").slice(-24);
rmSync(outDir,{recursive:true,force:true});mkdirSync(outDir,{recursive:true});
const done=new Map(),skip=new Set((process.env.SKIP_STAGED??"").split(",").filter(Boolean)),overlayOf=(abs)=>{if(!abs.startsWith(build))return "";const r=abs.slice(build.length);return skip.has(r)?"":T+"/staged/"+r;};
const transform=(originalPath,stagedPath)=>{
 if(done.has(originalPath))return done.get(originalPath);
 const dir=dirname(originalPath),url=JSON.stringify(pathToFileURL(originalPath).href),relPath=originalPath.slice(build.length),copyPath=outDir+"/mirror/"+relPath;
 done.set(originalPath,copyPath);
 const fix=(spec)=>{if(!spec.startsWith("."))return spec;const abs=resolve(dir,spec);if(abs.startsWith(build)){const overlay=overlayOf(abs);if(overlay&&existsSync(overlay))return transform(abs,overlay);}return abs;};
 let text=readFileSync(stagedPath,"utf8");
 if(!stagedPath.endsWith(".json")){
  text=text.replace(/((?:^|[;{}\n])[ \t]*import[ \t]+|\bfrom[ \t]*|\bimport[ \t]*\([ \t]*|\brequire[ \t]*\([ \t]*)(["'])(\.{1,2}\/[^"'\n]*)\2/gu,(m,head,q,spec)=>head+q+fix(spec)+q);
  text=text.replace(/new URL\((["'])(\.{1,2}\/[^"']*)\1\s*,\s*import\.meta\.url\)/gu,(m,q,spec)=>{const abs=resolve(dir,spec),overlay=overlayOf(abs);return overlay&&existsSync(overlay)?"new URL("+JSON.stringify(pathToFileURL(transform(abs,overlay)).href)+")":m;});
 text=text.replace(/fileURLToPath\(import\.meta\.url\)/gu,JSON.stringify(copyPath));
 text=text.replace(/(?<!["\x27])import\.meta\.url(?!["\x27])/gu,url).replace(/(?<!["\x27])import\.meta\.(dir|dirname)\b(?!["\x27])/gu,JSON.stringify(dir)).replace(/(?<!["\x27])import\.meta\.(path|filename)\b(?!["\x27])/gu,JSON.stringify(originalPath));
 }
 mkdirSync(dirname(copyPath),{recursive:true});writeFileSync(copyPath,text);return copyPath;
};
const copy=transform(orig,staged);
import {spawnSync as sp} from "node:child_process";
if(process.env.MIRROR_ONLY==="1"){console.log(copy);process.exit(0);}
const r=process.env.DIRECT==="1"?sp("sh",[T+"/direct.sh",copy,log,pattern],{cwd:root,stdio:"inherit"}):sp("sh",[T+"/run.sh","/__abs__"+copy,log,pattern,birth],{cwd:root,stdio:"inherit"});
process.exit(r.status??1);
