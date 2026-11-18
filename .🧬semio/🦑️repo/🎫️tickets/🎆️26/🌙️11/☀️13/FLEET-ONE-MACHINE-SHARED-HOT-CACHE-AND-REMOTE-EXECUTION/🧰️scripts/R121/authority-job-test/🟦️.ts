/** 🫳️ Contract: the Cargo-manifest assembly authority of an owner is computed once per distinct question by a farm child while one is ready, with the rows the child read replayed into the read, so a changed manifest still makes the record stale; with no child, a late or a failing one the parent computes it as before. @see ../../🟨️.mjs */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
const B=fileURLToPath(new URL("../../../../../",import.meta.url)).replace(/\/$/u,""),CHILD=join(import.meta.dir,"👷️child","🟦️.ts");process.env.R121_B=B;
const inference=await import(B+"/🕸️graph/📥️inference/🟨️.mjs") as any;await inference.libraryBootstrap;
const PN=await import(B+"/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs") as any,OB=await import(B+"/👁️observation/🟨️.mjs") as any,OD=await import(B+"/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs") as any;
const roots:string[]=[];afterAll(()=>{for(const r of roots)rmSync(r,{recursive:true,force:true});});
const workspace=()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"r121a-")));roots.push(root);
 for(const[path,text]of[["Cargo.toml",'[workspace]\nmembers=["owner","dependency"]\nresolver="2"\n'],["owner/Cargo.toml",'[package]\nname="owner"\nversion="0.1.0"\n[dependencies]\ndependency={path="../dependency"}\n'],["dependency/Cargo.toml",'[package]\nname="dependency"\nversion="0.1.0"\n']]){mkdirSync(join(root,path,".."),{recursive:true});writeFileSync(join(root,path),text);}
 return root;
};
const names=new Map([["owner","owner"],["dependency","dependency"]]);
const childReply=(root:string,directory:string=root)=>{const run=Bun.spawnSync([process.execPath,...(process.env.R121_PRELOAD?["--preload",process.env.R121_PRELOAD]:[]),CHILD],{env:{...process.env,R121_B:B,R121_JOB:JSON.stringify({root,nativeRoot:"owner",directory})}});expect(run.exitCode,run.stderr.toString()).toBe(0);return JSON.parse(run.stdout.toString().split("\n").find(l=>l.startsWith("CHILD "))!.slice(6));};

test("a child computes the same authority as the parent and hands over the rows it read",async()=>{
 const root=workspace(),own=OD.projectAssemblyAuthorityV1(root,"owner",names,undefined,new Map(),new Map()),reply=childReply(root);
 expect(reply.digest).toBe(own);expect(reply.layerDigests.length+reply.rows.length).toBeGreaterThan(0);for(const name of reply.layerDigests)expect(Array.isArray(PN.readAuthorityLayerV1(root,name))).toBe(true);expect(reply.layers).toBeUndefined();expect(reply.verifier).not.toBe(process.pid);expect(reply.weight).toBeLessThan(0.1);
});

test("the replayed rows make a record stale when a manifest the child read changes",async()=>{
 const root=workspace();await new Promise(done=>setTimeout(done,2500));const reply=childReply(root),cachePath=join(root,"np","owner.json");let computed=0;
 const look=()=>OB.observeInference({cachePath,key:["owner"]},async()=>{computed++;OB.retainObservedRowsV1([...reply.rows,...reply.layerDigests.flatMap((name:string)=>PN.readAuthorityLayerV1(root,name))]);return{authority:reply.digest};});
 await look();await new Promise(done=>setTimeout(done,2200));await look();expect(computed).toBe(1);
 writeFileSync(join(root,"dependency","Cargo.toml"),'[package]\nname="dependency"\nversion="0.2.0"\n');
 await look();expect(computed).toBe(2);
});

const fakeFarm=(width:number,answer:(job:any)=>any)=>{const calls:any[]=[];return{calls,farm:{width:()=>width,authority:async(job:any)=>{calls.push(job);return answer(job);}}};};
test("the first owner of each distinct question asks a child once, the others take its answer, rows and each shared layer are replayed once",async()=>{
 const{calls,farm}=fakeFarm(2,job=>({digest:"d-"+job.nativeRoot,rows:[{r:job.nativeRoot}],layerDigests:["shared",job.nativeRoot==="b"?"only-b":"shared"]}));const retained:any[]=[],owned:any[]=[],read:string[]=[];
 const resolve=PN.createAuthorityResolverV1({farm:()=>farm,directory:()=>"/read",layer:(_d:string,name:string)=>{read.push(name);return[{l:name}];},own:(root:string)=>{owned.push(root);return"own-"+root;},retain:(rows:any[])=>retained.push(rows)});
 const answers=await Promise.all(["a","a","a","b","a","b",undefined,undefined].map(root=>resolve(root,false)));
 expect(answers).toEqual(["d-a","d-a","d-a","d-b","d-a","d-b","d-undefined","d-undefined"]);expect(calls.length).toBe(3);expect(calls[0].directory).toBe("/read");expect(retained.length).toBe(3);expect(retained[0]).toEqual([{r:"a"},{l:"shared"}]);expect(owned).toEqual([]);expect(read.sort()).toEqual(["only-b","shared"]);
});

test("without a ready child, with a failing one or a silent one the parent computes the authority itself",async()=>{
 const none=fakeFarm(0,()=>({digest:"x",rows:[],layerDigests:[]})),failing=fakeFarm(2,()=>{throw new Error("child lost");}),torn=fakeFarm(2,()=>({digest:7}));
 for(const{calls,farm}of[none,failing,torn]){const owned:any[]=[],resolve=PN.createAuthorityResolverV1({farm:()=>farm,own:(root:string)=>{owned.push(root);return"own-"+root;},directory:()=>"/read",retain:()=>{}});
  expect(await resolve("a",false)).toBe("own-a");expect(owned).toEqual(["a"]);if(farm===none.farm)expect(calls.length).toBe(0);}
 expect(PN.ownerAuthorityCountsV1.parent).toBeGreaterThanOrEqual(2);
 expect(await Bun.file(join(import.meta.dir,"..","..","🏊️project-farm","👷️service","🟨️.mjs")).text()).toContain("if (job.authority === true) return authorityOfOwnerV1(environment, job);");
 expect(await Bun.file(join(import.meta.dir,"..","..","🟨️.mjs")).text()).toContain("await authorityOfV1(native?nativeRoot:undefined,pipeline)");
});
