import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {spawnSync} from "node:child_process";
import {appendFileSync,existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readlinkSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {snapshotAcquireV1,snapshotAdvanceV1,snapshotArgumentPathsV1,snapshotBasePathsV1,snapshotInClosureV1,snapshotRefreshLaneV1,snapshotWithBaseLockV1,type SnapshotAdvancePortsV1} from "../🟦️.ts";
import {validateSnapshotReceiptV1} from "../../🟦️.ts";
import policy from "../../🔣️.json" with {type:"json"};

const scratch=(run:(directory:string)=>void|Promise<void>)=>async()=>{const directory=mkdtempSync(join(tmpdir(),"semio-adv-"));try{await run(directory);}finally{rmSync(directory,{recursive:true,force:true});}};
const write=(directory:string,file:string,text:string)=>{mkdirSync(dirname(join(directory,file)),{recursive:true});writeFileSync(join(directory,file),text);};
const read=(directory:string,file:string)=>readFileSync(join(directory,file),"utf8");
let liveRoot="";
const shaOf=(path:string):string=>{try{return createHash("sha256").update(readFileSync(join(liveRoot,path))).digest("hex");}catch{return "0".repeat(64);}};
const segment=(store:string)=>join(store,"v1","origins","g1-test","segments","000000000001.jsonl");
const event=(store:string,atMs:number,path:string,operation="put",sha=shaOf(path))=>{mkdirSync(dirname(segment(store)),{recursive:true});appendFileSync(segment(store),JSON.stringify({version:1,hlc:`${atMs}-000000-g1-test`,operation,path,sha256:sha,bytes:1,executable:false})+"\n");};
const live=(directory:string)=>{
 const root=join(directory,"live");liveRoot=root;
 write(root,"🧰️framework/a/x.ts","export const x=1;\n");write(root,"🧰️framework/a/y.ts","export const y=2;\n");write(root,"🧰️framework/target/junk.rs","junk");
 write(root,"✏️s/Cargo.toml","[workspace]\n");write(root,"✏️s/🔌️plugins/big.rs","heavy");write(root,"package.json","{}");
 return root;
};
const ports=(directory:string,extra:Partial<SnapshotAdvancePortsV1>={}):SnapshotAdvancePortsV1&{store:string}=>({base:join(directory,"base"),store:join(directory,"store"),heavyMembers:["🔌️plugins"],link:async()=>{},sleep:async()=>{},...extra}) as never;
const oracle=(root:string,paths:readonly string[]):string=>spawnSync("python3",["-I","-c","import hashlib,sys\nrows=sorted((p,hashlib.sha256(open(sys.argv[1]+'/'+p,'rb').read()).hexdigest()) for p in sys.argv[2:])\nh=hashlib.sha256()\nfor p,d in rows: h.update((p+'\\0'+d+'\\n').encode())\nprint('sha256:'+h.hexdigest())",root,...paths],{encoding:"utf8"}).stdout.trim();
const all=["package.json","✏️s/Cargo.toml","🧰️framework/a/x.ts","🧰️framework/a/y.ts"];
const acquire=(root:string,lane:string,options:SnapshotAdvancePortsV1&{store:string},mode:"root"|"digest"="root",build=true)=>snapshotAcquireV1({root,lane,mode,base:options.base!,build},options);

test("the first acquire verifies the whole closure (via listing); the next one reads only the journal and copies only what the journal names",scratch(async directory=>{
 const root=live(directory),options=ports(directory);
 event(options.store,1,"seed.rs");
 const first=(await acquire(root,"h95",options))!;
 expect(first.via).toBe("listing");expect(first.generation).toBe(1);expect(first.digest).toBe(oracle(root,all));
 const quiet=(await acquire(root,"h95",options))!;
 expect([quiet.via,quiet.generation,quiet.copied,quiet.rounds,quiet.digest]).toEqual(["journal",1,0,1,first.digest]);
 write(root,"🧰️framework/a/x.ts","export const x=99;\n");event(options.store,Date.now(),"🧰️framework/a/x.ts");
 const next=(await acquire(root,"h95",options))!;
 expect([next.via,next.generation,next.copied,next.consistent]).toEqual(["journal",2,1,true]);
 expect(read(next.snapshotRoot,"🧰️framework/a/x.ts")).toBe("export const x=99;\n");
 expect(next.digest).toBe(oracle(root,all));
 expect(validateSnapshotReceiptV1(next)).toEqual([]);
}));

test("a warm acquire does not list the closure: it touches only the journal's paths (a live file changed without a journal event stays unseen until a full verification)",scratch(async directory=>{
 const root=live(directory),options=ports(directory);
 event(options.store,1,"seed.rs");
 const first=(await acquire(root,"h95",options))!;
 write(root,"🧰️framework/a/y.ts","export const y=222;\n");
 const unseen=(await acquire(root,"h95",options))!;
 expect(unseen.digest).toBe(first.digest);expect(read(unseen.snapshotRoot,"🧰️framework/a/y.ts")).toBe("export const y=2;\n");
 const fresh=(await snapshotAcquireV1({root,lane:"h95",mode:"root",base:options.base!,build:true},{...options,fresh:true}))!;
 expect([fresh.via,fresh.generation]).toEqual(["listing",2]);expect(read(fresh.snapshotRoot,"🧰️framework/a/y.ts")).toBe("export const y=222;\n");
}));

test("paths named on the command line are verified against the live tree whatever the journal says",scratch(async directory=>{
 const root=live(directory),options=ports(directory);
 event(options.store,1,"seed.rs");await acquire(root,"h95",options);
 write(root,"🧰️framework/a/x.ts","export const x=5;\n");
 const named=(await acquire(root,"h95",{...options,extraPaths:snapshotArgumentPathsV1(root,join(root,"🧰️framework"),["test-file","a/x.ts","--flag","nothing.ts"])}))!;
 expect(read(named.snapshotRoot,"🧰️framework/a/x.ts")).toBe("export const x=5;\n");
 expect(snapshotArgumentPathsV1(root,root,["🧰️framework/a/y.ts","/etc/hosts","../x"])).toEqual(["🧰️framework/a/y.ts"]);
}));

test("generation pin: a lane root does not move while the persistent snapshot advances; the next acquire of that lane picks the new generation; lanes pin independently",scratch(async directory=>{
 const root=live(directory),options=ports(directory);
 event(options.store,1,"seed.rs");
 const one=(await acquire(root,"lane-a",options))!;
 write(root,"🧰️framework/a/x.ts","export const x=7;\n");event(options.store,Date.now(),"🧰️framework/a/x.ts");
 const other=(await acquire(root,"lane-b",options))!;
 expect(read(one.snapshotRoot,"🧰️framework/a/x.ts")).toBe("export const x=1;\n");
 expect(read(other.snapshotRoot,"🧰️framework/a/x.ts")).toBe("export const x=7;\n");
 expect([one.generation,other.generation]).toEqual([1,2]);
 const again=(await acquire(root,"lane-a",options))!;
 expect([again.generation,again.copied]).toEqual([2,1]);expect(read(again.snapshotRoot,"🧰️framework/a/x.ts")).toBe("export const x=7;\n");
}));

test("deletions, creations and files outside the closure",scratch(async directory=>{
 const root=live(directory),options=ports(directory);
 event(options.store,1,"seed.rs");await acquire(root,"h95",options);
 rmSync(join(root,"🧰️framework/a/y.ts"));write(root,"🧰️framework/a/z.ts","z");write(root,"🧰️framework/target/more.rs","m");write(root,"✏️s/🔌️plugins/new.rs","n");
 for(const [path,op] of [["🧰️framework/a/y.ts","delete"],["🧰️framework/a/z.ts","put"],["🧰️framework/target/more.rs","put"],["✏️s/🔌️plugins/new.rs","put"]] as const)event(options.store,Date.now(),path,op);
 const next=(await acquire(root,"h95",options))!;
 expect(existsSync(join(next.snapshotRoot,"🧰️framework/a/y.ts"))).toBe(false);
 expect(read(next.snapshotRoot,"🧰️framework/a/z.ts")).toBe("z");
 expect(existsSync(join(next.snapshotRoot,"🧰️framework/target/more.rs"))).toBe(false);
 expect(existsSync(join(next.snapshotRoot,"✏️s/🔌️plugins/new.rs"))).toBe(false);
 expect(next.digest).toBe(oracle(root,["package.json","✏️s/Cargo.toml","🧰️framework/a/x.ts","🧰️framework/a/z.ts"]));
 expect(snapshotInClosureV1("🧰️framework/target/a.rs",["🔌️plugins"])).toBe(false);
 expect(snapshotInClosureV1("✏️s/🔌️plugins/a.rs",["🔌️plugins"])).toBe(false);
 expect(snapshotInClosureV1("✏️s/a/Cargo.toml",["🔌️plugins"])).toBe(true);
 expect(snapshotInClosureV1("package.json",[])).toBe(true);
}));

test("an edit while a journal path is copied is never torn: it is recopied and the cut ends consistent with the journal",scratch(async directory=>{
 const root=live(directory),options=ports(directory);
 event(options.store,1,"seed.rs");await acquire(root,"h95",options);
 write(root,"🧰️framework/a/x.ts","export const x=10;\n");event(options.store,Date.now(),"🧰️framework/a/x.ts");
 let fired=false;
 const next=(await acquire(root,"h95",{...options,afterCopy:()=>{if(!fired){fired=true;write(root,"🧰️framework/a/x.ts","export const x=11111;\n");event(options.store,Date.now(),"🧰️framework/a/x.ts");}}}))!;
 expect(fired).toBe(true);expect(next.consistent).toBe(true);
 expect(read(next.snapshotRoot,"🧰️framework/a/x.ts")).toBe("export const x=11111;\n");
 expect(next.digest).toBe(oracle(root,all));
}));

test("a lane that fell behind the (compacted) generation log is cloned whole in one directory clone; a small full verification stays incremental",scratch(async directory=>{
 const root=live(directory),options=ports(directory),short={...policy,incremental:{...policy.incremental,changelogKeep:2}} as never;
 event(options.store,1,"seed.rs");
 const one=(await snapshotAcquireV1({root,lane:"lane-a",mode:"root",base:options.base!,build:true},options,short))!;
 for(const index of [0,1,2]){write(root,"🧰️framework/a/x.ts",`export const x=${index+20};\n`);event(options.store,Date.now(),"🧰️framework/a/x.ts");await snapshotAcquireV1({root,lane:"lane-b",mode:"root",base:options.base!,build:true},options,short);}
 const paths=snapshotBasePathsV1(options.base!,short);
 expect(readFileSync(paths.changelog,"utf8").trim().split("\n").length).toBe(2);
 const behind=await snapshotRefreshLaneV1({base:options.base!,lane:"lane-a",generation:4},{...options,liveRoot:root},short);
 expect(behind.whole).toBe(true);expect(read(one.snapshotRoot,"🧰️framework/a/x.ts")).toBe("export const x=22;\n");
 write(root,"🧰️framework/a/y.ts","export const y=44;\n");
 await snapshotAcquireV1({root,lane:"lane-b",mode:"root",base:options.base!,build:true},{...options,fresh:true},short);
 const step=await snapshotRefreshLaneV1({base:options.base!,lane:"lane-a",generation:5},{...options,liveRoot:root},short);
 expect([step.whole,step.copied]).toEqual([false,1]);expect(read(one.snapshotRoot,"🧰️framework/a/y.ts")).toBe("export const y=44;\n");
}));

test("links of the base are rebound to the lane root in a whole clone and in an incremental copy, and the digest does not depend on where the snapshot lives",scratch(async directory=>{
 const root=live(directory),options=ports(directory);
 symlinkSync(join(root,"🧰️framework/a/x.ts"),join(root,"🧰️framework/a/link.ts"));
 event(options.store,1,"seed.rs");
 const one=(await acquire(root,"lane-a",options))!;
 expect(readlinkSync(join(one.snapshotRoot,"🧰️framework/a/link.ts"))).toBe(join(one.snapshotRoot,"🧰️framework/a/x.ts"));
 const moved=ports(join(directory,"elsewhere"));mkdirSync(join(directory,"elsewhere"),{recursive:true});
 const two=(await snapshotAcquireV1({root,lane:"lane-z",mode:"root",base:join(directory,"elsewhere","other"),build:true},{...moved,base:join(directory,"elsewhere","other"),store:options.store}))!;
 expect(two.digest).toBe(one.digest);
 symlinkSync(join(root,"🧰️framework/a/y.ts"),join(root,"🧰️framework/a/link2.ts"));event(options.store,Date.now(),"🧰️framework/a/link2.ts");
 const next=(await acquire(root,"lane-a",options))!;
 expect(readlinkSync(join(next.snapshotRoot,"🧰️framework/a/link2.ts"))).toBe(join(next.snapshotRoot,"🧰️framework/a/y.ts"));
}));

test("a full verification is due after the declared minutes and finds an edit whose journal event was lost",scratch(async directory=>{
 const root=live(directory),options=ports(directory);let clock=Date.now();
 event(options.store,1,"seed.rs");await acquire(root,"h95",{...options,now:()=>clock});
 write(root,"🧰️framework/a/x.ts","export const x=31;\n");
 clock+=31*60_000;
 const next=(await acquire(root,"h95",{...options,now:()=>clock}))!;
 expect(next.via).toBe("listing");expect(read(next.snapshotRoot,"🧰️framework/a/x.ts")).toBe("export const x=31;\n");
}));

test("digest mode refuses to build a missing persistent snapshot and costs no lane refresh once it exists",scratch(async directory=>{
 const root=live(directory),options=ports(directory);
 event(options.store,1,"seed.rs");
 expect(await acquire(root,"h95",options,"digest",false)).toBeUndefined();
 const built=(await acquire(root,"h95",options,"root",true))!;
 const report=(await acquire(root,"h95",options,"digest",false))!;
 expect(report.digest).toBe(built.digest);expect(report.copied).toBe(0);expect(report.snapshotRoot).toBe(snapshotBasePathsV1(options.base!).directory);
}));

test("two advancers take turns on the base lock, a dead owner's lock is taken over",scratch(async directory=>{
 const base=join(directory,"base"),order:string[]=[];
 const slow=snapshotWithBaseLockV1(base,async()=>{order.push("a-in");await new Promise(done=>setTimeout(done,150));order.push("a-out");});
 await new Promise(done=>setTimeout(done,30));
 await snapshotWithBaseLockV1(base,async()=>{order.push("b");},{sleep:milliseconds=>new Promise(done=>setTimeout(done,milliseconds))});
 await slow;
 expect(order).toEqual(["a-in","a-out","b"]);
 write(join(base,"main.lock"),"owner","999999");
 let ran=false;await snapshotWithBaseLockV1(base,async()=>{ran=true;});expect(ran).toBe(true);
}));

test("advance on its own reports what it did and keeps the journal cursor across calls",scratch(async directory=>{
 const root=live(directory),options=ports(directory);
 event(options.store,1,"seed.rs");
 const full=await snapshotAdvanceV1({root,base:options.base!},options);
 expect(full.via).toBe("listing");
 write(root,"🧰️framework/a/x.ts","export const x=12;\n");event(options.store,Date.now()-5000,"🧰️framework/a/x.ts");
 const step=await snapshotAdvanceV1({root,base:options.base!},options);
 expect([step.via,step.changed,step.generation]).toEqual(["journal",1,2]);expect(step.lagSeconds).toBeGreaterThanOrEqual(4);
 const idle=await snapshotAdvanceV1({root,base:options.base!},options);
 expect([idle.changed,idle.generation,idle.digest]).toEqual([0,2,step.digest]);
}));

test("an event of another machine whose file has not reached this checkout yet stays pending and is applied when the file arrives, without a new event; it expires after the declared seconds",scratch(async directory=>{
 const root=live(directory),options=ports(directory);let clock=Date.now();
 event(options.store,1,"seed.rs");await acquire(root,"h95",{...options,now:()=>clock});
 const content="export const late=1;\n",sha=createHash("sha256").update(content).digest("hex");
 event(options.store,Date.now(),"🧰️framework/a/late.ts","put",sha);
 const early=(await acquire(root,"h95",{...options,now:()=>clock}))!;
 expect(early.pending).toBe(1);expect(existsSync(join(early.snapshotRoot,"🧰️framework/a/late.ts"))).toBe(false);
 write(root,"🧰️framework/a/late.ts",content);
 const arrived=(await acquire(root,"h95",{...options,now:()=>clock}))!;
 expect(arrived.pending).toBe(0);expect(read(arrived.snapshotRoot,"🧰️framework/a/late.ts")).toBe(content);
 event(options.store,Date.now(),"🧰️framework/a/never.ts","put","f".repeat(64));
 const waiting=(await acquire(root,"h95",{...options,now:()=>clock}))!;expect(waiting.pending).toBe(1);
 clock+=121_000;
 const expired=(await acquire(root,"h95",{...options,now:()=>clock}))!;expect(expired.pending).toBe(0);
}));

test("a remote delete that has not reached this checkout yet keeps the file in the snapshot (live still has it) and removes it once the delete arrives",scratch(async directory=>{
 const root=live(directory),options=ports(directory);
 event(options.store,1,"seed.rs");await acquire(root,"h95",options);
 event(options.store,Date.now(),"🧰️framework/a/y.ts","delete");
 const early=(await acquire(root,"h95",options))!;
 expect(early.pending).toBe(1);expect(existsSync(join(early.snapshotRoot,"🧰️framework/a/y.ts"))).toBe(true);
 rmSync(join(root,"🧰️framework/a/y.ts"));
 const late=(await acquire(root,"h95",options))!;
 expect(late.pending).toBe(0);expect(existsSync(join(late.snapshotRoot,"🧰️framework/a/y.ts"))).toBe(false);
}));
