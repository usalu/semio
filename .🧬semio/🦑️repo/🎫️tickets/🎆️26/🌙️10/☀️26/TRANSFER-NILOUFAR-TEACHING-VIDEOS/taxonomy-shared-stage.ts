import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
const spec=JSON.parse(readFileSync(join(import.meta.dir,"taxonomy-paths.cases.json"),"utf8"));
const current=JSON.parse(readFileSync(spec.registry.path,"utf8"));
/** 🧩 Project only this ticket's registered semantic ownership into an index document. */
function project(index:any, source:any){
 const result=structuredClone(index);
 for(const [field,ids] of [["semanticDirectoryKinds",spec.registry.kindIds],["semanticDirectoryMemberKinds",spec.registry.memberIds]] as const){
  for(const id of ids){if(result[field][id])assert.deepEqual(result[field][id],source[field][id],"Conflicting staged taxonomy kind: "+id);else result[field][id]=structuredClone(source[field][id]);}
 }
 for(const [id,names] of Object.entries(spec.registry.memberAdditions) as [string,string[]][]){const row=result.semanticDirectoryMemberKinds[id];row.memberNames=[...new Set([...row.memberNames,...names])];}
 return result;
}
if(process.argv.includes("--test")){
 const cases=JSON.parse(readFileSync(join(import.meta.dir,"taxonomy-staging.cases.json"),"utf8")).cases;
 for(const test of cases){const base={schemaVersion:7,unrelated:{marker:test.id},semanticDirectoryKinds:{},semanticDirectoryMemberKinds:{"members-of-teaching-energy":{ownerKindIds:["teaching-energy"],memberNames:["🧲️physics"]},"members-of-teaching-topics":{ownerKindIds:["teaching-physics"],memberNames:["❓️quiz"]}}};if(test.extraKind)(base.semanticDirectoryKinds as any).concurrent=test.extraKind;if(test.extraMember)base.semanticDirectoryMemberKinds["members-of-teaching-energy"].memberNames.push(test.extraMember);const result=project(base,current);assert.deepEqual(result.unrelated,base.unrelated);if(test.extraKind)assert.deepEqual(result.semanticDirectoryKinds.concurrent,test.extraKind);if(test.extraMember)assert(result.semanticDirectoryMemberKinds["members-of-teaching-energy"].memberNames.includes(test.extraMember));assert.deepEqual(project(result,current),result);}
 const finalizerSource=readFileSync(join(import.meta.dir,"finish-taxonomy.ts"),"utf8"), transpiler=new Bun.Transpiler({loader:"ts"});transpiler.transformSync(finalizerSource);
 const entriesLine=finalizerSource.split(String.fromCharCode(10)).find(l=>l.startsWith("const entries="))!;
 const parseEntries=new Function(transpiler.transformSync(entriesLine)+";return entries;")();
 const baseline=readFileSync(join(import.meta.dir,"🗑️generated/taxonomy-baseline/index-entries.z"),"utf8"), baselineInfo=JSON.parse(readFileSync(join(import.meta.dir,"🗑️generated/taxonomy-baseline/baseline.json"),"utf8"));
 assert.equal(parseEntries(baseline).size,198074);
 const hashLine=finalizerSource.split(String.fromCharCode(10)).find(l=>l.startsWith("const hash="))!;
 const blobHash=new Function("createHash",transpiler.transformSync(hashLine)+";return hash;")(createHash), sample=Buffer.from("abc");
 assert.equal(blobHash(sample),createHash("sha1").update(Buffer.concat([Buffer.from("blob 3"),Buffer.from([0]),sample])).digest("hex"));
 console.log("[DEBUG] Selective taxonomy projection cases passed: "+cases.length+"; finalizer compiles");
}else{
 const git=(args:string[],input?:string)=>{const p=spawnSync("git",args,{encoding:"utf8",input,maxBuffer:16*1024*1024,windowsHide:true});if(p.status)throw new Error(p.stderr);return p.stdout;};
 const path=spec.registry.path,index=JSON.parse(git(["show",":"+path])),result=project(index,current),mode=git(["ls-files","--stage","--",path]).split(" ")[0];
 const hash=git(["hash-object","-w","--stdin"],JSON.stringify(result,null,2)+String.fromCharCode(10)).trim();git(["update-index","--add","--cacheinfo",mode+","+hash+","+path]);
 console.log("[DEBUG] Staged only owned taxonomy keys and member additions");
}
