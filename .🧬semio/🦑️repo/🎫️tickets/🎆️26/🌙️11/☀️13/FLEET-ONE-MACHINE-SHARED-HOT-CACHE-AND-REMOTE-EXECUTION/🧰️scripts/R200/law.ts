import{expect,test}from"bun:test";
import{existsSync}from"node:fs";
import{dirname,join}from"node:path";
import{libraryBootstrap}from"../../../🟨️.mjs";
import{createScriptInputCache,expandedCommandInputsV1,nativeCommandInputs}from"../../🟨️.mjs";
import{observeInferenceSnapshotV1}from"../../../../../👁️observation/🟨️.mjs";

const workspace=(()=>{let at=import.meta.dir;while(!existsSync(join(at,"📋️build-owners.json")))at=dirname(at);return at;})();
await libraryBootstrap;
const fresh=()=>{const scripts:any=createScriptInputCache({workspaceRoot:workspace,registry:null,lateClosures:false});scripts.generatedOutput=()=>undefined;return scripts;};
const expand=(scripts:any,list:any)=>observeInferenceSnapshotV1(()=>expandedCommandInputsV1(list,workspace,scripts));

/** 🗂️ Expanding the shared references of one command input list again and again derives each tool group and provider group once per script input cache, and every expansion answers the files a cold cache answers: the first expansion of an independent cache is the oracle. */
test("expanding the same shared references 6 times derives each group once and answers what a cold cache answers",async()=>{
 const scripts=fresh(),list:any[]=await observeInferenceSnapshotV1(()=>nativeCommandInputs(workspace,scripts));
 const references=list.filter(item=>item!==null&&typeof item==="object"&&typeof item.input==="string"&&item.input.startsWith("toolClosure:"));
 expect(references.length).toBeGreaterThan(0);
 const answers:any[]=[];
 for(let round=0;round<6;round++)answers.push(await expand(scripts,list));
 for(const answer of answers)expect(answer).toEqual(answers[0]);
 expect(answers[5]).toEqual(await expand(fresh(),list));
 expect(scripts.groupComputations).toBeGreaterThan(0);
 expect(scripts.groupComputations).toBeLessThanOrEqual(references.length*2);
 const settled=scripts.groupComputations;
 await expand(scripts,list);
 expect(scripts.groupComputations).toBe(settled);
},{timeout:900000});

/** 🏁 Targets that ask for the same tool group while its first derivation runs share that derivation. */
test("concurrent expansions of one list derive each group once",async()=>{
 const scripts=fresh(),list:any[]=await observeInferenceSnapshotV1(()=>nativeCommandInputs(workspace,scripts));
 const answers=await Promise.all(Array.from({length:8},()=>expand(scripts,list)));
 for(const answer of answers)expect(answer).toEqual(answers[0]);
 const references=list.filter(item=>item!==null&&typeof item==="object"&&typeof item.input==="string"&&item.input.startsWith("toolClosure:")).length;
 expect(scripts.groupComputations).toBeLessThanOrEqual(references*2);
},{timeout:900000});
