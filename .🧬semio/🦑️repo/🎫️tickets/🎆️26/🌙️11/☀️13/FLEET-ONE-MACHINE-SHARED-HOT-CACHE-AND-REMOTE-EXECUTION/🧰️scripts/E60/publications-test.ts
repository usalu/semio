/** 🪦️ Law: the graph reclaim retires the producer publications of gone processes before anything else, in a dry run only reports them, and never touches the one of a live process. */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {cacheLedgerV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {graphProducerPublicationsV1,reclaimGraphOptionalV1,type GraphReclaimPortsV1} from "../../🚰️reclaim/🟦️.ts";

const GIB=1024**3,ledger=cacheLedgerV1(),target=ledger.areas.find(area=>area.id==="graph-inference")!.reclaim!.targetAvailableBytes,HASH="d".repeat(64);
let root="",gone=0;
const folder=(name:string,ageMs:number):string=>{const path=join(root,"inference","selections",HASH,"inference",name);mkdirSync(path,{recursive:true});writeFileSync(join(path,"emoji-project-json.publication.json"),"x".repeat(9000));const time=(Date.now()-ageMs)/1000;utimesSync(path,time,time);return path;};
const ports=(free:number,log:string[]):GraphReclaimPortsV1=>({
 exclusive:async()=>({release:()=>{log.push("release");}}),
 open:async()=>({close:async()=>{log.push("close");}}),
 free:async()=>free,
 selected:async()=>{log.push("selected");},
 optional:async()=>{log.push("optional");},
 publications:async(_image,dryRun)=>{log.push("publications:"+dryRun);return graphProducerPublicationsV1(root,dryRun,new AbortController().signal);}
});
beforeAll(()=>{
 root=mkdtempSync(join(tmpdir(),"graph-publications-"));
 gone=Number(spawnSync(process.execPath,["--eval","process.stdout.write(String(process.pid))"],{encoding:"utf8"}).stdout);
});
afterAll(()=>{rmSync(root,{recursive:true,force:true});});

test("a dry run reports the publications of gone processes and removes nothing, in both languages",async()=>{
 const dead=folder("p-"+gone,3600000),own=folder("p-"+process.pid,3600000),log:string[]=[];
 expect(await reclaimGraphOptionalV1("/w",ledger,true,"en",ports(target+GIB,log))).toContain("Would remove 1 producer publications of gone processes (");
 expect(await reclaimGraphOptionalV1("/w",ledger,true,"de",ports(target+GIB,log))).toContain("Würde 1 Erzeuger-Veröffentlichungen beendeter Prozesse entfernen (");
 expect(existsSync(dead)&&existsSync(own)).toBe(true);expect(log.filter(line=>line.startsWith("publications")).length).toBe(2);
});

test("a run removes the publication of the gone process, keeps the one of the live process and finds nothing the second time",async()=>{
 const dead=join(root,"inference","selections",HASH,"inference","p-"+gone),own=join(root,"inference","selections",HASH,"inference","p-"+process.pid),log:string[]=[];
 expect(await reclaimGraphOptionalV1("/w",ledger,false,"en",ports(target+GIB,log))).toContain("Removed 1 producer publications of gone processes (");
 expect(existsSync(dead)).toBe(false);expect(existsSync(own)).toBe(true);
 expect(await reclaimGraphOptionalV1("/w",ledger,false,"de",ports(target+GIB,log))).not.toContain("Erzeuger-Veröffentlichungen");
});

test("the sweep runs before the pressure recovery and a cancelled sweep ends the reclaim",async()=>{
 const dead=folder("p-"+gone,3600000),log:string[]=[];
 await reclaimGraphOptionalV1("/w",ledger,false,"en",ports(target-1,log));
 expect(log.indexOf("publications:false")).toBeGreaterThanOrEqual(0);expect(log.indexOf("publications:false")).toBeLessThan(log.indexOf("selected"));expect(existsSync(dead)).toBe(false);
 const controller=new AbortController();controller.abort();folder("p-"+gone,3600000);
 const cancelled=await reclaimGraphOptionalV1("/w",ledger,false,"en",{...ports(target+GIB,[]),publications:(_image,dryRun)=>graphProducerPublicationsV1(root,dryRun,controller.signal)});
 expect(cancelled).toContain("cancel");
});
