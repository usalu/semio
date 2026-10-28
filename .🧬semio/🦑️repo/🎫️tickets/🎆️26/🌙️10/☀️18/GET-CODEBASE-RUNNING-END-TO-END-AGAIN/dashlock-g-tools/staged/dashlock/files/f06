import {test,expect} from "bun:test";
import {chmodSync,existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createHash} from "node:crypto";
import {fileURLToPath} from "node:url";
import {retireArtifactDirectoriesV1,stageArtifacts,stageExecutableDigestV1} from "../../🟦️.ts";
import {testPythonV1} from "../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

const leasesModule=fileURLToPath(new URL("../../../../🔒️leases/🟦️.ts",import.meta.url)),modelPath=fileURLToPath(new URL("./🐍️model.py",import.meta.url)),owner="digest-law";
const payload=(id:string)=>Buffer.from(`#!/bin/sh\n# executable ${id}\n`);
const holderProgram=`import {acquireResourceLease} from ${JSON.stringify(leasesModule)};
const [directory,resource]=process.argv.slice(2);
const lease=await acquireResourceLease({directory,resource,mode:"shared",signal:new AbortController().signal});
process.stdout.write("READY\\n");
for await (const _ of process.stdin) {}
lease.release();`;

/** 🏠️ One law sandbox: a lease directory, the digest cache and the producer's output directory, with live holder processes standing in for running executables. */
function sandbox(){
 const root=realpathSync(mkdtempSync(join(tmpdir(),"dg-"))),leases=join(root,"l"),cache=join(root,"c"),source=join(root,"s"),program=join(root,"holder.ts"),holders=new Map<string,Bun.Subprocess<"pipe","pipe","inherit">[]>();
 for(const directory of[leases,cache,source])mkdirSync(directory);writeFileSync(program,holderProgram);
 const sha=(id:string)=>createHash("sha256").update(payload(id)).digest("hex");
 const build=(id:string)=>{const path=join(source,"semio");writeFileSync(path,payload(id));chmodSync(path,0o755);return path;};
 const run=async(id:string)=>{const child=Bun.spawn([process.execPath,program,leases,"artifact:"+join(cache,sha(id))],{stdin:"pipe",stdout:"pipe",stderr:"inherit"});const reader=child.stdout.getReader(),first=await reader.read();expect(new TextDecoder().decode(first.value)).toBe("READY\n");holders.set(id,[...holders.get(id)??[],child]);};
 const stop=async(id:string)=>{const child=holders.get(id)?.pop();if(!child)return false;child.stdin.end();await child.exited;return true;};
 const publish=(id:string,milliseconds:number)=>{const signal=AbortSignal.timeout(milliseconds);return stageExecutableDigestV1(join(cache,sha(id)),owner,build(id),sha(id),{leaseDirectory:leases,signal});};
 const dispose=async()=>{for(const id of[...holders.keys()])while(await stop(id));rmSync(root,{recursive:true,force:true});};
 return{root,leases,cache,source,sha,build,run,stop,publish,dispose,holders};
}

/** 🚨️ The incident: a human runs the dashboard; a rebuild yielding the same bytes must neither wait nor fail. */
test("a digest that a running process holds is reused without waiting",async()=>{
 const box=sandbox();try{
  await box.publish("a",5000);await box.run("a");const file=join(box.cache,box.sha("a"),"semio"),before=lstatSync(file);
  const started=Date.now();await box.publish("a",5000);expect(Date.now()-started).toBeLessThan(2000);
  const after=lstatSync(file);expect(after.ino).toBe(before.ino);expect(after.mtimeMs).toBe(before.mtimeMs);expect(readFileSync(file)).toEqual(payload("a"));
 }finally{await box.dispose();}
},60000);

/** ⚔️ The contrast that made the incident: the exclusive replace path cannot pass a live holder of the same digest. */
test("replacing the held digest in place is blocked by its running holder",async()=>{
 const box=sandbox();try{
  await box.publish("a",5000);await box.run("a");const signal=AbortSignal.timeout(fixture.blockedMilliseconds/2);
  await expect(stageArtifacts(join(box.cache,box.sha("a")),owner,new Map([["semio",box.build("a")]]),{leaseDirectory:box.leases,signal})).rejects.toBeDefined();expect(signal.aborted).toBe(true);
 }finally{await box.dispose();}
},60000);

/** 🪟️ A new digest lands beside the running one: the running directory keeps its inode and bytes. */
test("a new digest is published beside a running one and leaves it untouched",async()=>{
 const box=sandbox();try{
  await box.publish("a",5000);await box.run("a");const old=join(box.cache,box.sha("a"),"semio"),before=lstatSync(old);
  await box.publish("b",5000);expect(readdirSync(box.cache).sort()).toEqual([box.sha("a"),box.sha("b")].sort());
  const after=lstatSync(old);expect(after.ino).toBe(before.ino);expect(readFileSync(old)).toEqual(payload("a"));expect(readFileSync(join(box.cache,box.sha("b"),"semio"))).toEqual(payload("b"));
 }finally{await box.dispose();}
},60000);

/** 🐍️ The scenario table replayed against real leases, real files and live holder processes equals the independent Python model. */
test("publication and retirement outcomes equal the independent Python table",async()=>{
 const python=testPythonV1()??"python3",oracle=Bun.spawnSync([python,"-I",modelPath],{stdin:Buffer.from(JSON.stringify(fixture)),stdout:"pipe",stderr:"pipe"});expect(oracle.exitCode,oracle.stderr.toString()).toBe(0);
 const expected=JSON.parse(oracle.stdout.toString()) as {name:string;outcomes:string[];digests:Record<string,string>}[];
 for(const [index,scenario] of fixture.scenarios.entries()){
  const box=sandbox(),outcomes:string[]=[];try{
   const damaged=new Set<string>();
   for(const [verb,id] of scenario.operations as [string,string][]){
    const directory=join(box.cache,box.sha(id!)),file=join(directory,"semio");
    if(verb==="build"){
     const existed=existsSync(file),before=existed?lstatSync(file):undefined;
     try{await box.publish(id!,fixture.blockedMilliseconds);const after=lstatSync(file);outcomes.push(before&&after.ino===before.ino&&!damaged.has(id!)?"reused":"published");damaged.delete(id!);}
     catch(error){expect(String(error)).toMatch(/abort|timed out|TimeoutError/iu);outcomes.push("blocked");}
    }else if(verb==="run"){if(existsSync(file)){await box.run(id!);outcomes.push("ok");}else outcomes.push("absent");}
    else if(verb==="stop")outcomes.push(await box.stop(id!)?"ok":"absent");
    else if(verb==="damage"){if(existsSync(file)){chmodSync(file,0o755);writeFileSync(file,"tampered");damaged.add(id!);outcomes.push("ok");}else outcomes.push("absent");}
    else if(verb==="retire"){
     if(!existsSync(directory)){outcomes.push("absent");continue;}
     const result=await retireArtifactDirectoriesV1([directory],{leaseDirectory:box.leases,signal:new AbortController().signal});outcomes.push(result.retired.length?"retired":"held");expect(existsSync(directory)).toBe(result.held.length>0);
     expect(readdirSync(box.cache).filter(name=>name.includes(".retired-"))).toEqual([]);
    }else throw Error("unknown verb "+verb);
   }
   expect(outcomes,scenario.name).toEqual(expected[index]!.outcomes);
   for(const [id,digest] of Object.entries(expected[index]!.digests))expect(box.sha(id),`${scenario.name}: ${id}`).toBe(digest);
  }finally{await box.dispose();}
 }
},180000);
