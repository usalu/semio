/** 🧪️ Runs the language-agnostic source-classes corpus of the actions law through bazelSourceClassesV1 under an issued script invocation (the law itself needs the repo harness), plain and under the world witness preload, and prints PASS/FAIL. usage: bun classes-check.ts <root> [witness] */
import {mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
const repo=process.argv[2]!,bazel=join(repo,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel"),routing=join(repo,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const corpus=(await import(join(bazel,"🧪️tests/📥️actions/🔣️.json"),{with:{type:"json"}})).default.sourceClasses,api=await import(join(bazel,"📜️script.ts"));
const {createNativeBuildCallerCapabilitiesV1}=await import(join(repo,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts")),{createScriptProcessEnvelope,withScriptProcessEnvelope}=await import(routing);
const root=join(realpathSync(mkdtempSync(join(tmpdir(),"pfc-"))),"r");mkdirSync(root);
try{
 for(const[path,bytes]of Object.entries(corpus.files as Record<string,string>)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),bytes);}
 for(const[path,target]of Object.entries(corpus.links as Record<string,string>))symlinkSync(target,join(root,path));
 await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:planfp-classes",maximumElapsedMilliseconds:60000},createNativeBuildCallerCapabilitiesV1(root,["run","x:y"],"bazel",{}),Date.now()),async(original:never)=>{
  const actual=await api.bazelSourceClassesV1(root,corpus.paths,new Map(),new Map(),original),got=JSON.stringify({files:[...actual.files].sort(),hashed:[...actual.hashed].sort()}),want=JSON.stringify({files:[...corpus.expected.files].sort(),hashed:[...corpus.expected.hashed].sort()});
  console.log(got===want?"PASS source classes":"FAIL source classes\n got  "+got+"\n want "+want);
 });
}finally{rmSync(dirname(root),{recursive:true,force:true});}
(globalThis as {__semioWorldWitnessV1?:{finish():void}}).__semioWorldWitnessV1?.finish();
