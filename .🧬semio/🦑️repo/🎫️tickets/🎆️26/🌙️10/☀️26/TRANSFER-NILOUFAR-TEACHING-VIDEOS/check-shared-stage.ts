import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import vm from "node:vm";
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { spawnSync } from "node:child_process";
const ticket=import.meta.dir,root=process.cwd();
const expectedFontHash="00275cb1bfb43493a3dfbb8612226fcd475120d6f61e2f7f400944f296c553a2";
assert.equal(createHash("sha256").update(readFileSync("🎓️teaching/manim_fonts.py")).digest("hex"),expectedFontHash);
const fixture=JSON.parse(readFileSync(join(ticket,"shared-stage.tests.json"),"utf8"));
const manifest=JSON.parse(readFileSync(join(ticket,"stage-files.json"),"utf8"));
assert.equal(manifest.length,fixture.counts.staged_teaching);
assert.equal(new Set(manifest).size,manifest.length);
for(const p of fixture.required_teaching_files)assert(manifest.includes("🎓️teaching/"+p));
for(const p of manifest)assert(readFileSync(p).length>=0);
const source="C:/git/semio.worktrees/🦢niloufar/⛳wip/tutorial";
function files(folder:string,prefix=""):string[]{return readdirSync(folder,{withFileTypes:true}).flatMap(e=>e.isDirectory()?files(join(folder,e.name),prefix+e.name+"/"):[prefix+e.name]);}
const authored=files(source).filter(p=>!p.split("/").some(part=>[".agents","__pycache__","media","rendered",".venv"].includes(part)));
assert.equal(authored.length,fixture.counts.transferred);
for(const p of authored)assert(manifest.includes("🎓️teaching/"+p),p);
const plan=JSON.parse(readFileSync("🎓️teaching/video_manifest.json","utf8"));
assert.equal(plan.videos.length,fixture.counts.videos);
assert.equal(plan.videos.flatMap(v=>v.chapters).length,fixture.counts.chapters);
assert.equal(plan.videos.flatMap(v=>v.chapters.flatMap(c=>c.segments)).length,fixture.counts.beats);
assert.equal(new Set(plan.videos.map(v=>JSON.stringify(v.intro))).size,fixture.counts.intros);
const readme=readFileSync("🎓️teaching/README.md","utf8");
for(const phrase of fixture.readme_phrases)assert(readme.includes(phrase),"Missing README phrase: "+phrase);
const edits=JSON.parse(readFileSync(join(ticket,"shared-edits.json"),"utf8"));
assert.deepEqual([...Object.keys(edits),"🎓️teaching/README.md"],fixture.shared_paths);
for(const [path,hunks] of Object.entries(edits))for(const hunk of hunks as string[][])assert(readFileSync(path,"utf8").includes(hunk[1]),path);
const src=readFileSync(join(ticket,"shared-stage.ts"),"utf8");
const marker="\n\n## Complete teaching videos\n";
const section=readme.slice(readme.indexOf(marker));
function simulate(existing:string,alternative:boolean){
 const index:Record<string,string>={};
 const originals:Record<string,string>={};
 for(const [path,hunks] of Object.entries(edits))index[path]="concurrent native prefix\n"+(hunks as string[][]).map(h=>alternative&&h[2]?h[2]:h[0]).join("unrelated native line\n")+"concurrent native suffix\n";
 index["🎓️teaching/README.md"]=existing;
 Object.assign(originals,index);
 const blobs=new Map<string,string>();let counter=0;
 const fakeSpawn=(_command:string,args:string[],options:any)=>{let stdout="";if(args[0]==="show")stdout=index[args[1].slice(1)];else if(args[0]==="ls-files")stdout="100644 deadbeef 0\t"+args.at(-1);else if(args[0]==="hash-object"){stdout=String(++counter);blobs.set(stdout,options.input);}else if(args[0]==="update-index"){const [,hash,path]=args[3].split(",");index[path]=blobs.get(hash)!;}else throw Error("Unexpected Git call "+args);return{status:0,stdout,stderr:""};};
 const localRead=(path:string)=>path.endsWith("shared-edits.json")?JSON.stringify(edits):path==="🎓️teaching/README.md"?readme:assert.fail("Unexpected read "+path);
 const transformed=new Bun.Transpiler({loader:"ts"}).transformSync(src.replace(/^import .*;\r?\n/gm,"").replaceAll("import.meta.dir","TICKET"));
 vm.runInNewContext(transformed,{spawnSync:fakeSpawn,readFileSync:localRead,join,TICKET:ticket,console:{log(){}}});
 assert.equal(index["🎓️teaching/README.md"],existing.includes(marker)?existing.slice(0,existing.indexOf(marker))+section+"\n## Other owner\nconcurrent tail\n":existing.trimEnd()+section);
 for(const [path,hunks] of Object.entries(edits)){
  let expected=originals[path];for(const [before,after,...alts]of hunks as string[][]){const found=[before,...alts].find(v=>expected.includes(v));expected=expected.replace(found!,after);}assert.equal(index[path],expected);
 }
 const once=JSON.stringify(index);
 vm.runInNewContext(transformed,{spawnSync:fakeSpawn,readFileSync:localRead,join,TICKET:ticket,console:{log(){}}});
 assert.equal(JSON.stringify(index),once,"Repeated staging must be idempotent");
 return {index,transformed,localRead};
}
simulate("concurrent prefix"+marker+"old task docs\n## Other owner\nconcurrent tail\n",true);
simulate("concurrent prefix\n",false);
const check=spawnSync("git",["diff","--check","--",...fixture.shared_paths],{encoding:"utf8",windowsHide:true});assert.equal(check.status,0,check.stdout+check.stderr);
console.log("[DEBUG] PASS: 131 transferred paths; 141 unique stage paths; 5 videos, 26 chapters, 112 beats, 5 intros; README/native edits coherent; simulated index preserves concurrent sections; real index unchanged.");
