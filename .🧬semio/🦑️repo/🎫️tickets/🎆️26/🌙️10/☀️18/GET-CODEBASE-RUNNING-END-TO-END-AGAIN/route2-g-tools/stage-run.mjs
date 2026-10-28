/** 🧪 route2-g: runs a staged replacement of a build-library test file without touching the library tree. usage: bun stage-run.mjs <build-relative-test-file> [name-pattern] [wallSeconds]; the staged file `staged/<build-relative>` gets its relative specifiers and import.meta paths resolved against the ORIGINAL location, is written to the generated folder and run by run1-style census (official-default envelope). */
import {readFileSync,writeFileSync,mkdirSync,existsSync,rmSync} from "node:fs";
import {dirname,resolve,join} from "node:path";
import {pathToFileURL} from "node:url";
import {spawnSync} from "node:child_process";
const root="/Users/ueli/Documents/semio",C=root+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN",T=C+"/route2-g-tools",build=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const [rel,pattern="",wall="200"]=process.argv.slice(2),orig=build+rel,staged=T+"/staged/"+rel;
if(!existsSync(staged)){console.log("no staged file for "+rel);process.exit(2);}
const outDir=C+"/🗑️generated/route2-g/stage/"+rel.split("/").length+"-"+Buffer.from(rel).toString("hex").slice(-24);
rmSync(outDir,{recursive:true,force:true});mkdirSync(outDir,{recursive:true});
const done=new Map();
const transform=(originalPath,stagedPath)=>{
 if(done.has(originalPath))return done.get(originalPath);
 const dir=dirname(originalPath),url=JSON.stringify(pathToFileURL(originalPath).href),relPath=originalPath.slice(build.length),copyPath=outDir+"/mirror/"+relPath;
 done.set(originalPath,copyPath);
 const fix=(spec)=>{if(!spec.startsWith("."))return spec;const abs=resolve(dir,spec);if(abs.startsWith(build)){const overlay=T+"/staged/"+abs.slice(build.length);if(existsSync(overlay))return transform(abs,overlay);}return abs;};
 let text=readFileSync(stagedPath,"utf8");
 if(!stagedPath.endsWith(".json")){
  text=text.replace(/((?:^|[;{}\n])[ \t]*import[ \t]+|\bfrom[ \t]*|\bimport[ \t]*\([ \t]*|\brequire[ \t]*\([ \t]*)(["'])(\.{1,2}\/[^"'\n]*)\2/gu,(m,head,q,spec)=>head+q+fix(spec)+q);
  text=text.replace(/new URL\((["'])(\.{1,2}\/[^"']*)\1\s*,\s*import\.meta\.url\)/gu,(m,q,spec)=>{const abs=resolve(dir,spec),overlay=abs.startsWith(build)?T+"/staged/"+abs.slice(build.length):"";return existsSync(overlay)?"new URL("+JSON.stringify(pathToFileURL(transform(abs,overlay)).href)+")":m;});
 text=text.replace(/fileURLToPath\(import\.meta\.url\)/gu,JSON.stringify(copyPath));
 text=text.replace(/(?<!["\x27])import\.meta\.url(?!["\x27])/gu,url).replace(/(?<!["\x27])import\.meta\.(dir|dirname)\b(?!["\x27])/gu,JSON.stringify(dir)).replace(/(?<!["\x27])import\.meta\.(path|filename)\b(?!["\x27])/gu,JSON.stringify(originalPath));
 }
 mkdirSync(dirname(copyPath),{recursive:true});writeFileSync(copyPath,text);return copyPath;
};
const copy=transform(orig,staged);
const table=JSON.parse(readFileSync(build+"🗂️suites/🔣️.json","utf8")),suiteOf=(file)=>{let best=-1,id="build-census";for(const suite of table.suites)for(const tree of [...(suite.trees??[]),...(suite.files??[])])if((file===tree||file.startsWith(tree.replace(/\/$/u,"")+"/"))&&tree.length>best){best=tree.length;id=suite.id;}return id;};
const list=outDir+"/list.txt";writeFileSync(list,copy+"\n");
const r=spawnSync("nice",["-n","10","bun",T+"/../route-g-tools/census-run.ts",list,outDir+"/o",wall],{cwd:root,env:{...process.env,CENSUS_TAG:"s"+process.pid,CENSUS_ARTROOT:process.env.CENSUS_ARTROOT??"/private/tmp/claude-501/rg2o",CENSUS_PATTERN:pattern,CENSUS_SELECTION:JSON.stringify(["run","@semio-tech/repo:test-"+suiteOf(rel)])},stdio:["ignore","ignore","ignore"]});
console.log(readFileSync(outDir+"/o/summary.txt","utf8").split("\n")[0]);
const log=readFileSync(outDir+"/o/f000.log","utf8").split("\n").filter(l=>/^\(fail\)|^error:|Expected|Received|^ *\d+ (pass|fail)/u.test(l)).map(l=>l.slice(0,300)).slice(0,Number(process.env.LINES_MAX??40));
console.log(log.join("\n"));console.log("log: "+outDir+"/o/f000.log");
