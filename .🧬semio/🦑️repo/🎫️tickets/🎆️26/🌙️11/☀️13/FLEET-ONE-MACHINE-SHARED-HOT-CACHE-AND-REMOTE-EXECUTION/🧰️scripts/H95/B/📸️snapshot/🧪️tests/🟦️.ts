import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readlinkSync,realpathSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import policy from "../🔣️.json" with {type:"json"};
import texts from "../📖️texts/🔣️.json" with {type:"json"};
import {landLinkWorkspaceV1} from "../../🚉️land/🔬️gates/🟦️.ts";
import {readSnapshotPolicyV1,snapshotCaptureV1,snapshotDigestV1,snapshotDueV1,snapshotEnumerateV1,snapshotNoticesV1,snapshotPruneV1,snapshotReceiptOfV1,snapshotRunV1,snapshotTakeLaneV1,validateSnapshotReceiptV1} from "../🟦️.ts";

const scratch=(run:(directory:string)=>void|Promise<void>)=>async()=>{const directory=mkdtempSync(join(tmpdir(),"semio-snap-"));try{await run(directory);}finally{rmSync(directory,{recursive:true,force:true});}};
const write=(directory:string,file:string,text:string)=>{mkdirSync(dirname(join(directory,file)),{recursive:true});writeFileSync(join(directory,file),text);};
const read=(directory:string,file:string)=>readFileSync(join(directory,file),"utf8");
const ports=(directory:string,extra:Record<string,unknown>={})=>({base:join(directory,"base"),heavyMembers:["🔌️plugins"],link:async()=>{},sleep:async()=>{},...extra});
const live=(directory:string)=>{
 const root=join(directory,"live");
 write(root,"🧰️framework/a/x.ts","export const x=1;\n");
 write(root,"🧰️framework/a/y.ts","export const y=2;\n");
 write(root,"🧰️framework/target/junk.rs","junk");
 write(root,"🧰️framework/node_modules/p/index.js","dep");
 write(root,"✏️s/Cargo.toml","[workspace]\n");
 write(root,"✏️s/🔌️plugins/big.rs","heavy");
 write(root,"package.json","{}");
 return root;
};
const oracle=(root:string,paths:readonly string[]):string=>{
 const script="import hashlib,sys\nrows=[]\nfor p in sys.argv[2:]:\n rows.append((p,hashlib.sha256(open(sys.argv[1]+'/'+p,'rb').read()).hexdigest()))\nrows.sort()\nh=hashlib.sha256()\nfor p,d in rows:\n h.update((p+'\\0'+d+'\\n').encode())\nprint('sha256:'+h.hexdigest())";
 return spawnSync("python3",["-I","-c",script,root,...paths],{encoding:"utf8"}).stdout.trim();
};

test("the policy and every text have both languages and the closed schema accepts them",()=>{
 expect(readSnapshotPolicyV1().commands.cargo).toBe("root");
 expect(()=>readSnapshotPolicyV1({...policy,surprise:1})).toThrow("snapshot-policy-invalid");
 for(const row of Object.values(texts.messages))expect(row.en.length>0&&row.de.length>0).toBe(true);
});

test("enumeration lists the closure and leaves out target, node_modules and heavy members",scratch(directory=>{
 const root=live(directory);symlinkSync(join(directory,"nowhere"),join(root,"node_modules"));
 const found=[...snapshotEnumerateV1(root,["🔌️plugins"]).keys()].sort();
 expect(found).toEqual(["package.json","✏️s/Cargo.toml","🧰️framework/a/x.ts","🧰️framework/a/y.ts"].sort());
}));

test("a capture copies the closure and its digest equals an independent SHA-256 over the same bytes",scratch(async directory=>{
 const root=live(directory),receipt=await snapshotCaptureV1({root,lane:"h95"},ports(directory));
 expect(receipt.consistent).toBe(true);
 expect(receipt.files).toBe(4);
 expect(validateSnapshotReceiptV1(receipt)).toEqual([]);
 expect(read(receipt.snapshotRoot,"🧰️framework/a/x.ts")).toBe("export const x=1;\n");
 expect(existsSync(join(receipt.snapshotRoot,"🧰️framework/target/junk.rs"))).toBe(false);
 expect(receipt.digest).toBe(oracle(root,["package.json","✏️s/Cargo.toml","🧰️framework/a/x.ts","🧰️framework/a/y.ts"]));
}));

test("the digest follows the content only, never the modification time",scratch(async directory=>{
 const root=live(directory),first=await snapshotCaptureV1({root,lane:"a"},ports(directory));
 utimesSync(join(root,"🧰️framework/a/x.ts"),new Date(2020,1,1),new Date(2020,1,1));
 const second=await snapshotCaptureV1({root,lane:"b"},ports(directory));
 expect(second.digest).toBe(first.digest);
 write(root,"🧰️framework/a/x.ts","export const x=3;\n");
 expect((await snapshotCaptureV1({root,lane:"c"},ports(directory))).digest===first.digest).toBe(false);
 expect(snapshotDigestV1(new Map([["a","1"],["b","2"]]))).toBe(snapshotDigestV1(new Map([["b","2"],["a","1"]])));
}));

test("a refresh of the same lane copies only what changed and removes what is gone",scratch(async directory=>{
 const root=live(directory),options=ports(directory);
 await snapshotCaptureV1({root,lane:"h95"},options);
 const quiet=await snapshotCaptureV1({root,lane:"h95"},options);
 expect([quiet.copied,quiet.removed,quiet.rounds]).toEqual([0,0,1]);
 write(root,"🧰️framework/a/x.ts","export const x=9;\n");rmSync(join(root,"🧰️framework/a/y.ts"));write(root,"🧰️framework/a/z.ts","z");
 const changed=await snapshotCaptureV1({root,lane:"h95"},options);
 expect([changed.copied,changed.removed]).toEqual([2,1]);
 expect(existsSync(join(changed.snapshotRoot,"🧰️framework/a/y.ts"))).toBe(false);
 expect(read(changed.snapshotRoot,"🧰️framework/a/x.ts")).toBe("export const x=9;\n");
}));

test("an edit while a file is copied is never torn: the copy is retried and the final cut equals the live tree",scratch(async directory=>{
 const root=live(directory);let fired=false;
 const receipt=await snapshotCaptureV1({root,lane:"h95"},ports(directory,{afterCopy:(path:string)=>{if(path==="🧰️framework/a/x.ts"&&!fired){fired=true;write(root,"🧰️framework/a/x.ts","export const x=2222;\n");}}}));
 expect(fired).toBe(true);
 expect(receipt.consistent).toBe(true);
 expect(read(receipt.snapshotRoot,"🧰️framework/a/x.ts")).toBe("export const x=2222;\n");
 expect(receipt.digest).toBe(oracle(root,["package.json","✏️s/Cargo.toml","🧰️framework/a/x.ts","🧰️framework/a/y.ts"]));
}));

test("an edit between rounds is picked up by the next round and the cut ends consistent",scratch(async directory=>{
 const root=live(directory);
 const receipt=await snapshotCaptureV1({root,lane:"h95"},ports(directory,{beforeRound:(round:number)=>{if(round===2)write(root,"🧰️framework/a/y.ts","export const y=77;\n");}}));
 expect(receipt.rounds).toBe(3);
 expect(receipt.consistent).toBe(true);
 expect(read(receipt.snapshotRoot,"🧰️framework/a/y.ts")).toBe("export const y=77;\n");
}));

test("peers that never stop editing end the capture after the declared rounds with an inconsistent receipt and examples, never a failure",scratch(async directory=>{
 const root=live(directory);let counter=0;
 const receipt=await snapshotCaptureV1({root,lane:"h95"},ports(directory,{beforeRound:()=>{write(root,"🧰️framework/a/x.ts",`export const x=${++counter};\n`);}}));
 expect(receipt.rounds).toBe(policy.capture.maximumRounds);
 expect(receipt.consistent).toBe(false);
 expect(receipt.moved).toEqual(["🧰️framework/a/x.ts"]);
 expect(validateSnapshotReceiptV1(receipt)).toEqual([]);
 const lines=snapshotNoticesV1(receipt,["en","de"]);
 expect(lines[0]).toContain("kept editing");expect(lines[1]).toContain("weiter bearbeiteten");
}));

test("an edit after the capture never reaches the snapshot",scratch(async directory=>{
 const root=live(directory),receipt=await snapshotCaptureV1({root,lane:"h95"},ports(directory));
 write(root,"🧰️framework/a/x.ts","export const x=4242;\n");rmSync(join(root,"🧰️framework/a/y.ts"));
 expect(read(receipt.snapshotRoot,"🧰️framework/a/x.ts")).toBe("export const x=1;\n");
 expect(existsSync(join(receipt.snapshotRoot,"🧰️framework/a/y.ts"))).toBe(true);
}));

test("an absolute link into the live tree is bound to the snapshot, other links stay as they are",scratch(async directory=>{
 const root=live(directory);
 symlinkSync(join(root,"🧰️framework/a/x.ts"),join(root,"🧰️framework/a/inside.ts"));
 symlinkSync("/etc/hosts",join(root,"🧰️framework/a/outside.ts"));
 const receipt=await snapshotCaptureV1({root,lane:"h95"},ports(directory));
 expect(readlinkSync(join(receipt.snapshotRoot,"🧰️framework/a/inside.ts"))).toBe(join(receipt.snapshotRoot,"🧰️framework/a/x.ts"));
 expect(readlinkSync(join(receipt.snapshotRoot,"🧰️framework/a/outside.ts"))).toBe("/etc/hosts");
}));

test("a crashed capture (manifest left open) is rebuilt from scratch",scratch(async directory=>{
 const root=live(directory),options=ports(directory),first=await snapshotCaptureV1({root,lane:"h95"},options);
 write(join(directory,"base"),"h95.manifest.json",JSON.stringify({version:1,open:true,entries:{}}));
 write(first.snapshotRoot,"🧰️framework/stale.ts","stale");
 const again=await snapshotCaptureV1({root,lane:"h95"},options);
 expect(existsSync(join(again.snapshotRoot,"🧰️framework/stale.ts"))).toBe(false);
 expect(again.digest).toBe(first.digest);
}));

test("which commands run on a snapshot: digest by default, root only with MODE=on, never inside a run or when off",()=>{
 expect(snapshotDueV1(["cargo","--","check"],{})).toBe("digest");
 expect(snapshotDueV1(["cargo","--","check"],{SEMIO_SOURCE_SNAPSHOT_MODE:"digest"})).toBe("digest");
 expect(snapshotDueV1(["cargo","--","check"],{SEMIO_SOURCE_SNAPSHOT_MODE:"on"})).toBe("root");
 expect(snapshotDueV1(["test-file","a.ts"],{SEMIO_SOURCE_SNAPSHOT_MODE:"on"})).toBe("root");
 expect(snapshotDueV1(["run","p:build"],{SEMIO_SOURCE_SNAPSHOT_MODE:"on"})).toBe("digest");
 expect(snapshotDueV1(["fleet","status"],{SEMIO_SOURCE_SNAPSHOT_MODE:"on"})).toBeUndefined();
 expect(snapshotDueV1(["land","check"],{})).toBeUndefined();
 expect(snapshotDueV1(["cargo","check"],{SEMIO_SOURCE_SNAPSHOT:"sha256:x",SEMIO_SOURCE_SNAPSHOT_MODE:"on"})).toBeUndefined();
 expect(snapshotDueV1(["cargo","check"],{SEMIO_SOURCE_SNAPSHOT_MODE:"off"})).toBeUndefined();
 expect(snapshotDueV1(["cargo","--help"],{SEMIO_SOURCE_SNAPSHOT_MODE:"on"})).toBeUndefined();
});

test("a lane another live command holds yields a lane of its own, a dead owner's lock is taken over",scratch(directory=>{
 const base=join(directory,"base"),first=snapshotTakeLaneV1(base,"h95");
 expect([first.lane,first.busy]).toEqual(["h95",false]);
 const second=snapshotTakeLaneV1(base,"h95",policy as never,process.pid+0);
 expect(second.busy).toBe(false);
 const other=spawnSync(process.execPath,["-e","setTimeout(()=>{},1500)"],{timeout:1}).pid;
 write(join(base,"dead.lock"),"owner","999999");
 expect(snapshotTakeLaneV1(base,"dead").busy).toBe(false);
 write(join(base,"live.lock"),"owner",String(process.ppid));
 const busy=snapshotTakeLaneV1(base,"live");
 expect(busy.busy).toBe(true);expect(busy.lane).toBe(`live-${process.pid}`);
 busy.release();first.release();
 expect(other===undefined||typeof other==="number").toBe(true);
}));

test("old unheld snapshots are pruned, held and young ones stay",scratch(directory=>{
 const base=join(directory,"base");
 for(const name of ["old","young","held"]){write(base,`${name}/f`,"1");write(base,`${name}.manifest.json`,"{}");}
 write(base,"held.lock/owner",String(process.pid));
 const old=new Date(Date.now()-48*3600_000);utimesSync(join(base,"old.manifest.json"),old,old);utimesSync(join(base,"held.manifest.json"),old,old);
 expect(snapshotPruneV1(base,Date.now())).toEqual(["old"]);
 expect(existsSync(join(base,"held"))).toBe(true);expect(existsSync(join(base,"young"))).toBe(true);expect(existsSync(join(base,"old"))).toBe(false);
}));

const entry=`import {readFileSync,writeFileSync} from "node:fs";
const text=()=>readFileSync(process.cwd().split("/🧰️framework")[0]+"/🧰️framework/a/x.ts","utf8");
const first=text();
await new Promise(done=>setTimeout(done,700));
writeFileSync(process.env.PROBE,JSON.stringify({cwd:process.cwd(),first,second:text(),marker:process.env.SEMIO_SOURCE_SNAPSHOT,live:process.env.SEMIO_SOURCE_SNAPSHOT_LIVE_ROOT,receipt:process.env.SEMIO_SOURCE_SNAPSHOT_RECEIPT,args:process.argv.slice(2)}));
process.exit(7);
`;

test("root mode runs the whole command in a child on the snapshot while a peer edits the live tree: no restart, the child's exit code, the digest in its environment",scratch(async directory=>{
 const root=live(directory),probe=join(directory,"probe.json"),lines:string[]=[];
 write(root,"repo.mjs",entry);
 const environment={...process.env,PROBE:probe} as NodeJS.ProcessEnv;
 delete environment.SEMIO_SOURCE_SNAPSHOT;
 setTimeout(()=>write(root,"🧰️framework/a/x.ts","export const x=EDITED;\n"),250);
 const outcome=await snapshotRunV1({root,cwd:join(root,"🧰️framework"),operation:["cargo"],mode:"root",entry:join(root,"repo.mjs"),arguments:["cargo","--","check"],environment,lane:"h95"},ports(directory,{notice:(line:string)=>lines.push(line)}));
 expect(outcome.kind).toBe("spawned");
 if(outcome.kind!=="spawned")return;
 expect(outcome.code).toBe(7);
 const seen=JSON.parse(readFileSync(probe,"utf8"));
 expect(seen.first).toBe("export const x=1;\n");
 expect(seen.second).toBe("export const x=1;\n");
 expect(read(root,"🧰️framework/a/x.ts")).toBe("export const x=EDITED;\n");
 expect(realpathSync(seen.cwd).startsWith(realpathSync(outcome.receipt.snapshotRoot))).toBe(true);
 expect(seen.cwd.endsWith("🧰️framework")).toBe(true);
 expect(seen.marker).toBe(outcome.receipt.digest);expect(seen.live).toBe(root);
 expect(seen.args).toEqual(["cargo","--","check"]);
 expect(snapshotReceiptOfV1({SEMIO_SOURCE_SNAPSHOT_RECEIPT:seen.receipt})?.digest).toBe(outcome.receipt.digest);
 expect(lines.some(line=>line.includes("Source snapshot sha256:"))&&lines.some(line=>line.includes("Quelltext-Momentaufnahme sha256:"))).toBe(true);
 expect(lines.at(-1)).toContain("Das Ergebnis gehört zur Quelltext-Momentaufnahme");
}));

test("digest mode keeps the command in this process and binds it to the digest",scratch(async directory=>{
 const root=live(directory),environment:NodeJS.ProcessEnv={},outcome=await snapshotRunV1({root,cwd:root,operation:["run"],mode:"digest",entry:join(root,"repo.mjs"),arguments:[],environment,lane:"h95"},ports(directory,{notice:()=>{}}));
 expect(outcome.kind).toBe("inline");
 expect(environment.SEMIO_SOURCE_SNAPSHOT).toBe(outcome.receipt?.digest);
 expect(snapshotDueV1(["cargo"],{...environment,SEMIO_SOURCE_SNAPSHOT_MODE:"on"})).toBeUndefined();
}));

test("a capture that fails is announced in both languages and the command runs live",scratch(async directory=>{
 const lines:string[]=[],outcome=await snapshotRunV1({root:join(directory,"nothing"),cwd:directory,operation:["cargo"],mode:"root",entry:"x",arguments:[],environment:{},lane:"h95"},{base:join("/dev/null","x"),heavyMembers:[],notice:line=>lines.push(line)});
 expect(outcome.kind).toBe("inline");
 expect(lines.some(line=>line.includes("runs on the live tree"))&&lines.some(line=>line.includes("lebenden Baum"))).toBe(true);
}));

test("the workspace link step of the land overlay serves the snapshot: cache mirrored as real folders, heavy members of the Rust root linked, repeatable",scratch(async directory=>{
 const root=live(directory);
 write(root,".cache/a/b/c/data.bin","1");
 const receipt=await snapshotCaptureV1({root,lane:"h95"},ports(directory,{heavyMembers:["node_modules","🔌️plugins"]}));
 await landLinkWorkspaceV1(root,receipt.snapshotRoot);
 await landLinkWorkspaceV1(root,receipt.snapshotRoot);
 expect(lstatSync(join(receipt.snapshotRoot,".cache")).isDirectory()).toBe(true);
 expect(lstatSync(join(receipt.snapshotRoot,".cache/a/b/c")).isSymbolicLink()||lstatSync(join(receipt.snapshotRoot,".cache/a/b")).isDirectory()).toBe(true);
 expect(lstatSync(join(receipt.snapshotRoot,"✏️s/🔌️plugins")).isSymbolicLink()).toBe(true);
 expect(read(receipt.snapshotRoot,"✏️s/Cargo.toml")).toBe("[workspace]\n");
}));
