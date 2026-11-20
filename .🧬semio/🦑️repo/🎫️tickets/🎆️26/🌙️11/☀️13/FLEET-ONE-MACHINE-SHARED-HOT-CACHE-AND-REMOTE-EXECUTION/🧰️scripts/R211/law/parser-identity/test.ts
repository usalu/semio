/** 🪪️ Law: a record's analysis rows are judged by one pure function of the record and the files, wherever it runs. The id a process gives a parser hashes the module location and the loaded revision, so it differs between a fresh process, a farm child that already judged other owners, a pool child with another root and another machine; the verdict names the parser by its role and revision and never by that id: the same record holds in every one of them while the files are the same, and a changed file or another parser revision is stale in every one of them.
 * @see ../../🟨️.mjs `registeredAnalysisV1` `sourceAnalysisFailureV1` */
import{afterAll,expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";

const observationPath=fileURLToPath(new URL("../../🟨️.mjs",import.meta.url)),observation=await import("../../🟨️.mjs") as any;
const sha=(text:string)=>createHash("sha256").update(text).digest("hex"),role="build:parser-identity:v1:",revision=sha("parser-identity-revision"),other=sha("parser-identity-other-revision");
const analyze=(_path:string,bytes:Buffer)=>({length:bytes.length,head:bytes.toString("utf8").slice(0,3)});
const provider=observation.registerSourceAnalysisV1({id:role+sha("location of the producer"),revision,analyze});
const root=mkdtempSync(join(realpathSync(tmpdir()),"parser-identity-"));
afterAll(()=>{rmSync(root,{recursive:true,force:true});});
const input=join(root,"input.txt");
writeFileSync(input,"one two three");
const names=Array.from({length:50},(_all,at)=>join(root,"other-"+at+".txt"));for(const name of names)writeFileSync(name,"other "+name);
await new Promise(done=>setTimeout(done,2200));
const record=async(name:string,file:string)=>{const cachePath=join(root,"np",name+".json");await observation.observeInference({cachePath,key:[name]},async()=>({value:observation.observedSourceAnalysisV1(provider,file)}));return cachePath;};
const mine=await record("mine",input),others=await Promise.all(names.map((file,at)=>record("o"+at,file)));
const script=join(root,"judge.ts");
writeFileSync(script,`const observation=await import(${JSON.stringify(observationPath)}) as any;
const spec=JSON.parse(process.argv[2]);
const analyze=(_path:string,bytes:Buffer)=>({length:bytes.length,head:bytes.toString("utf8").slice(0,3)});
for(const parser of spec.parsers)observation.registerSourceAnalysisV1({id:parser.id,revision:parser.revision,analyze});
const said=[];for(const job of spec.judge)said.push((await observation.verifyStoredProjectRecordV1({cachePath:job.cachePath,key:[job.key]})).verdict);
const last=await observation.verifyStoredProjectRecordV1({cachePath:spec.cachePath,key:[spec.key]});
console.log(JSON.stringify({before:said,verdict:last.verdict,why:last.staleRow?.why}));
`);
const judged=async(parsers:{id:string;revision:string}[],judge:{cachePath:string;key:string}[]=[],cachePath=mine,key="mine")=>{
 const preload=process.env.R211_PRELOAD===undefined?[]:["--preload",process.env.R211_PRELOAD];
 const child=Bun.spawn([process.execPath,...preload,script,JSON.stringify({parsers,judge,cachePath,key})],{stdout:"pipe",stderr:"pipe",env:process.env as Record<string,string>}),text=await new Response(child.stdout).text(),errors=await new Response(child.stderr).text();await child.exited;
 if(text.trim()==="")throw Error("the judging process said nothing: "+errors.slice(0,600));
 return JSON.parse(text.trim().split("\n").pop()!);
};

test("the process that made the record holds it",async()=>{
 expect((await observation.verifyStoredProjectRecordV1({cachePath:mine,key:["mine"]})).verdict).toBe("holds");
});

test("a fresh process with another id for the same parser holds the record (another root, another machine)",async()=>{
 const said=await judged([{id:role+sha("location of another machine"),revision}]);
 expect(said.verdict,said.why).toBe("holds");
});

test("a farm child that already judged fifty other owners, with a parser of the same role at another revision registered first, holds it",async()=>{
 const judge=others.map((cachePath,at)=>({cachePath,key:"o"+at}));
 const said=await judged([{id:role+sha("an older loaded revision"),revision:other},{id:role+sha("location of a farm child"),revision}],judge);
 expect(said.before.length).toBe(50);expect(said.verdict,said.why).toBe("holds");
});

test("the same two children agree on a changed file and on another parser revision: stale, with the reason",async()=>{
 const edited=await record("edited",input);
 writeFileSync(input,"one two three, and then some more");
 const changed=await judged([{id:role+sha("location of another machine"),revision}],[],edited,"edited");
 expect(changed.verdict).toBe("stale");expect(changed.why).toMatch(/^replay-result /u);
 writeFileSync(input,"one two three");
 await new Promise(done=>setTimeout(done,2200));
 const moved=await judged([{id:role+sha("location of another machine"),revision:other}]);
 expect(moved.verdict).toBe("stale");expect(moved.why).toMatch(/^revision |^provider-unregistered /u);
});
