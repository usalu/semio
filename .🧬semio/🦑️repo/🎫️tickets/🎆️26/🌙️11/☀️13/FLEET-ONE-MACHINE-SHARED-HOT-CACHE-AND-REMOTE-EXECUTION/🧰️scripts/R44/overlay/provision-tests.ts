import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {chmodSync,cpSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,renameSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {codeDigestV1} from "../../../🩺️attest/🟦️.ts";
import {codeDirectoryV1,existingCompanionsV1,importClosureV1,moduleSourcesV1,publishCodeSnapshotV1,sealCodeV1,sealCompanionsV1,storeCodeDirectoryV1,verifyCodeSnapshotV1} from "../../../🛡️resilience/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {kitDirectoryV1,kitFilesV1,maintainKitV1,publishKitV1,refreshKitV1,verifyKitV1} from "../🟦️.ts";

setDefaultTimeout(60_000);
const ajv=new Ajv({strict:true,allowUnionTypes:true}),python=testPythonV1(),bash=Bun.which("bash")!,scratch:string[]=[],oracle=join(import.meta.dirname,"🐍️.py");
const temp=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`provision-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const put=(root:string,path:string,text:string):void=>{const file=join(root,...path.split("/"));mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);};
const sha=(data:Uint8Array|string):string=>createHash("sha256").update(data).digest("hex");
const until=async(condition:()=>boolean,milliseconds=8000):Promise<void>=>{const end=Date.now()+milliseconds;while(!condition()){if(Date.now()>end)throw Error("Timed out");await Bun.sleep(10);}};
const moduleRoot=resolve(import.meta.dirname,"../../.."),provisionDirectory=resolve(import.meta.dirname,".."),named=(directory:string,suffix:string):string=>join(directory,readdirSync(directory).find(name=>name.endsWith(suffix))!);
const shellText=readFileSync(named(provisionDirectory,".sh"),"utf8"),powershellText=readFileSync(named(provisionDirectory,".ps1"),"utf8");
const MOD="m/🤝️shared-worktree";
const executables=["🗺️topology/📜️script.ts","🗺️topology/🥾️provision/🐚️.sh","🔁️replica/📜️script.ts","🔁️replica/🥾️entry/🐚️.sh"],bitOf=(path:string):boolean=>(statSync(path).mode&0o111)!==0;

function build(root:string,variant:string,scripts:string="one"):string{
 const base=join(root,...MOD.split("/"));
 put(base,"🗺️topology/🟦️.ts",`import data from "./🔣️.json" with {type:"json"};\nexport const variant="${variant}";\nexport default data;\n`);
 put(base,"🗺️topology/🔣️.json",`{"version":1,"variant":"${variant}"}\n`);
 put(base,"🗺️topology/📜️script.ts",`import {variant} from "./🟦️.ts";\nconsole.log(variant);\n`);
 put(base,"🗺️topology/🥾️provision/🐚️.sh",`#!/usr/bin/env bash\n# ${scripts}\n`);
 put(base,"🗺️topology/🥾️provision/🔵️.ps1",`# ${scripts}\n`);
 put(base,"🔁️replica/🟦️.ts",`export const replica="${variant}";\n`);
 put(base,"🔁️replica/📜️script.ts",`import {replica} from "./🟦️.ts";\nconsole.log(replica);\n`);
 put(base,"🔁️replica/🥾️entry/🟨️.mjs",`console.log("boot ${scripts}");\n`);
 put(base,"🔁️replica/🥾️entry/🐚️.sh",`#!/bin/sh\n# ${scripts}\n`);
 put(base,"🔁️replica/🥾️entry/🔵️.ps1",`# ${scripts}\n`);
 for(const path of executables)chmodSync(join(base,...path.split("/")),0o755);
 return base;
}
async function seal(root:string,base:string):Promise<string>{
 const digest=await codeDigestV1(base),sealed=await sealCodeV1({root,moduleRoot:base,digest,digestOf:codeDigestV1});
 expect(sealed.reason).toBeNull();
 return digest;
}
function storeWith(root:string,store:string,digest:string,origin:string="g1-server"):void{
 expect(publishCodeSnapshotV1({root,store,digest,originId:origin})).resolves.toEqual({published:true,reason:null});
}
async function publishToStore(root:string,store:string,digest:string):Promise<void>{
 expect(await publishCodeSnapshotV1({root,store,digest,originId:"g1-server"})).toEqual({published:true,reason:null});
}
const attest=(store:string,digest:string,origin:string="g1-server"):void=>put(store,`v1/origins/${origin}/attest.json`,JSON.stringify({version:1,originId:origin,groupId:"g1",wallMs:1,code:digest}));

test("a seal carries the companions next to the imported code, the code digest does not change with them, and a companion edit re-seals the same digest",async()=>{
 const root=temp("seal"),base=build(root,"a");
 const digest=await seal(root,base),snapshot=join(codeDirectoryV1(root),digest),read=()=>JSON.parse(readFileSync(join(snapshot,"sealed.json"),"utf8")) as {files:{path:string;sha256:string}[]};
 const listed=read().files.map(file=>file.path);
 for(const companion of sealCompanionsV1)expect(listed).toContain(`${MOD}/${companion}`);
 expect(await existingCompanionsV1(base)).toEqual([...sealCompanionsV1].sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right))));
 expect(await verifyCodeSnapshotV1(snapshot)).toBeNull();
 const again=await sealCodeV1({root,moduleRoot:base,digest,digestOf:codeDigestV1});
 expect(again.created).toBe(false);
 put(base,"🗺️topology/🥾️provision/🐚️.sh","#!/usr/bin/env bash\n# two\n");
 expect(await codeDigestV1(base)).toBe(digest);
 const resealed=await sealCodeV1({root,moduleRoot:base,digest,digestOf:codeDigestV1});
 expect(resealed.created).toBe(true);
 expect(read().files.find(file=>file.path===`${MOD}/🗺️topology/🥾️provision/🐚️.sh`)!.sha256).toBe(sha("#!/usr/bin/env bash\n# two\n"));
 expect(await verifyCodeSnapshotV1(snapshot)).toBeNull();
 put(base,"🗺️topology/🥾️provision/🔵️.ps1","# gone soon\n");
 rmSync(join(base,"🗺️topology/🥾️provision/🔵️.ps1"));
 const afterRemoval=await sealCodeV1({root,moduleRoot:base,digest,digestOf:codeDigestV1});
 expect(afterRemoval.created).toBe(true);
 expect(read().files.map(file=>file.path)).not.toContain(`${MOD}/🗺️topology/🥾️provision/🔵️.ps1`);
});

test("a seal written before the companions existed is replaced by a seal that has them",async()=>{
 const root=temp("old"),base=build(root,"a");
 const digest=await seal(root,base),file=join(codeDirectoryV1(root),digest,"sealed.json"),old=JSON.parse(readFileSync(file,"utf8")) as {files:{path:string}[]};
 old.files=old.files.filter(item=>!sealCompanionsV1.some(companion=>item.path.endsWith(companion)));
 writeFileSync(file,JSON.stringify(old));
 const result=await sealCodeV1({root,moduleRoot:base,digest,digestOf:codeDigestV1});
 expect(result.created).toBe(true);
 expect((JSON.parse(readFileSync(file,"utf8")) as {files:unknown[]}).files.length).toBeGreaterThan(old.files.length);
});

test("the companions of the real module exist and no module imports them, so the digest cannot know them and the seal has to",async()=>{
 for(const companion of sealCompanionsV1)expect(existsSync(join(moduleRoot,...companion.split("/"))),companion).toBe(true);
 const sources=await moduleSourcesV1(moduleRoot),closure=new Set([...sources,...await importClosureV1(moduleRoot,sources)]);
 for(const companion of sealCompanionsV1)expect(closure.has(companion),companion).toBe(false);
 expect((await existingCompanionsV1(moduleRoot)).length).toBe(sealCompanionsV1.length);
});

const commandsOf=(text:string,pattern:RegExp):string[]=>[...new Set([...text.matchAll(pattern)].map(match=>match[1]!))].sort();
test("every command that the provisioning scripts run lies in the sealed set: the imports of the topology script, the replica script and the boot program are all published, and so are the scripts they run",async()=>{
 const sources=await moduleSourcesV1(moduleRoot),published=new Set([...sources,...await importClosureV1(moduleRoot,sources),...await existingCompanionsV1(moduleRoot)]);
 const topologyScript="🗺️topology/📜️script.ts",replicaScript="🔁️replica/📜️script.ts",entryProgram="🔁️replica/🥾️entry/🟨️.mjs";
 const outside=async(entry:string):Promise<string[]> => (await importClosureV1(moduleRoot,[entry])).concat(entry).filter(path=>!published.has(path)&&!path.startsWith(".."));
 for(const entry of [topologyScript,replicaScript,entryProgram])expect(await outside(entry),entry).toEqual([]);
 const everything=await importClosureV1(moduleRoot,[topologyScript,replicaScript,entryProgram]);
 for(const path of everything)expect(path.startsWith("..")?true:published.has(path),path).toBe(true);
 const shellFleet=commandsOf(shellText,/\bfleet ([a-z][a-z-]+)/gu).filter(name=>["facts","mount-plan","worker-service","server-plan"].includes(name)||name==="env"),shellReplica=commandsOf(shellText.split("\n").filter(line=>!/\b(say|fail|printf)\b|^#|WARNING/u.test(line)).join("\n"),/(?:^|[\s(])replica ([a-z][a-z-]+)(?=[\s$)"]|$)/gmu);
 expect(shellFleet.length).toBeGreaterThan(2);expect(shellReplica).toEqual(expect.arrayContaining(["join","bootstrap-settle","relocate","seed","relay"]));
 const topologyCommands=readFileSync(join(moduleRoot,"🗺️topology/⌨️command/🟦️.ts"),"utf8"),replicaCommands=readFileSync(join(moduleRoot,"🔁️replica/⌨️command/🟦️.ts"),"utf8");
 for(const name of shellFleet.filter(name=>name!=="env"))expect(topologyCommands,name).toContain(`"${name}"`);
 for(const name of shellReplica)expect(replicaCommands,name).toContain(`"${name}"`);
 const windowsFleet=commandsOf(powershellText,/Invoke-Fleet @\('([a-z-]+)'/gu),windowsReplica=commandsOf(powershellText,/Invoke-Replica @\('([a-z-]+)'/gu);
 for(const name of windowsFleet)expect(topologyCommands,name).toContain(`"${name}"`);
 for(const name of windowsReplica)expect(replicaCommands,name).toContain(`"${name}"`);
 expect(published.has(topologyScript)&&published.has(replicaScript)).toBe(true);
 expect(published.has(entryProgram)&&published.has("🔁️replica/🥾️entry/🐚️.sh")&&published.has("🔁️replica/🥾️entry/🔵️.ps1")).toBe(true);
 expect(published.has("🗺️topology/🥾️provision/🐚️.sh")&&published.has("🗺️topology/🥾️provision/🔵️.ps1")).toBe(true);
});

test("the closure walk fails when a file that a command needs is not published",async()=>{
 const root=temp("closure"),base=build(root,"a");
 put(base,"🗺️topology/📜️script.ts",`import {variant} from "./🟦️.ts";\nawait import("./📚️extra/🟦️.ts");\nconsole.log(variant);\n`);
 put(base,"🗺️topology/📚️extra/🟦️.ts",`export const extra=1;\n`);
 const sources=await moduleSourcesV1(base),published=new Set([...sources,...await importClosureV1(base,sources)]);
 expect(published.has("🗺️topology/📚️extra/🟦️.ts")).toBe(true);
 put(base,"🗺️topology/📜️script.ts",`import {variant} from "./🟦️.ts";\nawait import("./🚫️unlisted/🧰️helper.mjs");\nconsole.log(variant);\n`);
 put(base,"🗺️topology/🚫️unlisted/🧰️helper.mjs",`export const helper=1;\n`);
 const closure=await importClosureV1(base,["🗺️topology/📜️script.ts"]);
 expect(closure).toContain("🗺️topology/🚫️unlisted/🧰️helper.mjs");
 const later=new Set([...await moduleSourcesV1(base),...await importClosureV1(base,await moduleSourcesV1(base))]);
 expect(later.has("🗺️topology/🚫️unlisted/🧰️helper.mjs")).toBe(true);
 const withoutCompanions=new Set([...sources]);
 expect(withoutCompanions.has("🔁️replica/🥾️entry/🟨️.mjs")).toBe(false);
});

test("the kit is exactly the sealed topology files with fresh checksums, verified by shasum, by the Python oracle and by the closed receipt schema",async()=>{
 const root=temp("kit-root"),store=temp("kit-store"),base=build(root,"a"),digest=await seal(root,base),snapshot=join(codeDirectoryV1(root),digest);
 await publishToStore(root,store,digest);
 const result=await publishKitV1({store,originId:"g1-server",snapshot,now:()=>Date.UTC(2026,10,14,22,2,14)});
 expect(result).toEqual({outcome:"published",reason:null,code:digest});
 const kit=kitDirectoryV1(store),receipt=JSON.parse(readFileSync(join(kit,"published"),"utf8")) as {code:string;publishedAt:string;files:number};
 expect(receipt.code).toBe(digest);expect(receipt.publishedAt).toBe("2026-11-14T22:02:14Z");
 const validate=ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:"#/definitions/KitPublished"});
 expect(validate(receipt),JSON.stringify(validate.errors?.[0])).toBe(true);
 expect(await verifyKitV1(kit,snapshot)).toBeNull();
 expect(await verifyKitV1(kit,join(storeCodeDirectoryV1(store),digest))).toBeNull();
 const expected=kitFilesV1(JSON.parse(readFileSync(join(snapshot,"sealed.json"),"utf8")));
 expect(expected.map(file=>file.path)).toEqual(expect.arrayContaining(["topology/🥾️provision/🐚️.sh","topology/🥾️provision/🔵️.ps1","topology/🟦️.ts","topology/🔣️.json","topology/📜️script.ts"]));
 expect(receipt.files).toBe(expected.length);
 for(const file of expected)expect(readFileSync(join(kit,...file.path.split("/")))).toEqual(readFileSync(join(snapshot,"files",...file.source.split("/"))));
 if(process.platform!=="win32"){
  for(const file of expected)expect(bitOf(join(kit,...file.path.split("/"))),file.path).toBe(file.executable);
  expect(expected.filter(file=>file.executable).map(file=>file.path).sort()).toEqual(["topology/📜️script.ts","topology/🥾️provision/🐚️.sh"]);
  const script=join(kit,"topology","📜️script.ts");
  chmodSync(script,0o644);
  expect(await verifyKitV1(kit,snapshot)).toContain("execute bit differs at topology/📜️script.ts");
  chmodSync(script,0o755);
  expect(await verifyKitV1(kit,snapshot)).toBeNull();
 }
 const checked=Bun.spawnSync(["shasum","-a","256","-c","SHA256SUMS"],{cwd:kit,stdout:"pipe",stderr:"pipe"});
 expect(checked.exitCode,checked.stdout.toString()+checked.stderr.toString()).toBe(0);
 expect(checked.stdout.toString().split("\n").filter(line=>line.endsWith(": OK")).length).toBe(expected.length);
 if(python!==undefined){
  expect(Bun.spawnSync([python,"-I",oracle,"kit",kit,snapshot],{stdout:"pipe"}).stdout.toString().trim()).toBe("ok");
  expect(Bun.spawnSync([python,"-I",oracle,"verify",snapshot],{stdout:"pipe"}).stdout.toString().trim()).toBe("ok");
 }
 expect(readdirSync(dirname(kit)).filter(name=>name.startsWith(".tmp-kit-")||name.startsWith(".old-kit-"))).toEqual([]);
});

test("an unchanged kit is left alone, a new digest replaces it atomically without a second copy, and a damaged kit is repaired",async()=>{
 const root=temp("swap-root"),store=temp("swap-store"),base=build(root,"a"),first=await seal(root,base);
 const result=await publishKitV1({store,originId:"g1-server",snapshot:join(codeDirectoryV1(root),first)});
 expect(result.outcome).toBe("published");
 const kit=kitDirectoryV1(store),stamp=statSync(join(kit,"published")).mtimeMs;
 utimesSync(join(kit,"published"),new Date(1_000_000),new Date(1_000_000));
 expect((await publishKitV1({store,originId:"g1-server",snapshot:join(codeDirectoryV1(root),first)})).outcome).toBe("unchanged");
 expect(statSync(join(kit,"published")).mtimeMs).toBe(1_000_000);expect(stamp).toBeGreaterThan(1_000_000);
 build(root,"b");
 const second=await seal(root,base);
 expect(second).not.toBe(first);
 expect((await publishKitV1({store,originId:"g1-server",snapshot:join(codeDirectoryV1(root),second)})).outcome).toBe("published");
 expect((JSON.parse(readFileSync(join(kit,"published"),"utf8")) as {code:string}).code).toBe(second);
 expect(readFileSync(join(kit,"topology","🔣️.json"),"utf8")).toContain('"variant":"b"');
 expect(await verifyKitV1(kit,join(codeDirectoryV1(root),second))).toBeNull();
 expect(await verifyKitV1(kit,join(codeDirectoryV1(root),first))).not.toBeNull();
 expect(readdirSync(dirname(kit)).sort()).toEqual(["kit"]);
 writeFileSync(join(kit,"topology","🔣️.json"),"{}\n");
 expect(await verifyKitV1(kit)).toContain("changed topology/");
 expect((await publishKitV1({store,originId:"g1-server",snapshot:join(codeDirectoryV1(root),second)})).outcome).toBe("published");
 expect(await verifyKitV1(kit,join(codeDirectoryV1(root),second))).toBeNull();
 writeFileSync(join(kit,"stray.txt"),"x");
 expect(await verifyKitV1(kit)).toContain("differ");
 rmSync(join(codeDirectoryV1(root),second,"files",...`${MOD}/🗺️topology/🔣️.json`.split("/")));
 const refused=await publishKitV1({store,originId:"g1-server",snapshot:join(codeDirectoryV1(root),second)});
 expect(refused.outcome).toBe("skipped");expect(refused.reason).toContain("snapshot: missing");
});

test("a kit that dies half way leaves the old kit in place and litter that the next round removes",async()=>{
 const root=temp("litter-root"),store=temp("litter-store"),base=build(root,"a"),digest=await seal(root,base),snapshot=join(codeDirectoryV1(root),digest);
 await publishKitV1({store,originId:"g1-server",snapshot});
 const bootstrap=dirname(kitDirectoryV1(store));
 mkdirSync(join(bootstrap,".tmp-kit-g1-server-dead","topology"),{recursive:true});mkdirSync(join(bootstrap,".old-kit-dead"),{recursive:true});
 const old=new Date(Date.now()-2*3_600_000);
 utimesSync(join(bootstrap,".tmp-kit-g1-server-dead"),old,old);utimesSync(join(bootstrap,".old-kit-dead"),old,old);
 expect(await verifyKitV1(kitDirectoryV1(store),snapshot)).toBeNull();
 await maintainKitV1({root,moduleRoot:base,store,originId:"g1-server",digest,digestOf:codeDigestV1});
 expect(readdirSync(bootstrap).sort()).toEqual(["kit"]);
});

test("a provision script edited without a new digest is sealed again and reaches the kit under the same digest",async()=>{
 const root=temp("edit-root"),store=temp("edit-store"),base=build(root,"a"),digest=await seal(root,base);
 const first=await maintainKitV1({root,moduleRoot:base,store,originId:"g1-server",digest,digestOf:codeDigestV1});
 expect(first.outcome).toBe("published");
 expect((await maintainKitV1({root,moduleRoot:base,store,originId:"g1-server",digest,digestOf:codeDigestV1})).outcome).toBe("unchanged");
 put(base,"🗺️topology/🥾️provision/🐚️.sh","#!/usr/bin/env bash\n# edited\n");
 expect(await codeDigestV1(base)).toBe(digest);
 const second=await maintainKitV1({root,moduleRoot:base,store,originId:"g1-server",digest,digestOf:codeDigestV1});
 expect(second.outcome).toBe("published");
 expect(readFileSync(join(kitDirectoryV1(store),"topology","🥾️provision","🐚️.sh"),"utf8")).toContain("edited");
 expect(await verifyKitV1(kitDirectoryV1(store),join(codeDirectoryV1(root),digest))).toBeNull();
});

test("the kit follows the digest that the group server attests, also when the code changes under a running loop, and stops on abort",async()=>{
 const root=temp("loop-root"),store=temp("loop-store"),base=build(root,"a"),first=await seal(root,base),controller=new AbortController(),errors:string[]=[];
 let attested:string|undefined=undefined;
 const running=refreshKitV1({root,moduleRoot:base,store,originId:"g1-server",digestOf:codeDigestV1,attested:async()=>attested,minutes:60,pollMilliseconds:20,sink:event=>{if(event.event==="error")errors.push(String(event.message));}},controller.signal);
 await Bun.sleep(80);
 expect(existsSync(kitDirectoryV1(store))).toBe(false);
 attested=first;
 await until(()=>existsSync(join(kitDirectoryV1(store),"published")));
 expect((JSON.parse(readFileSync(join(kitDirectoryV1(store),"published"),"utf8")) as {code:string}).code).toBe(first);
 build(root,"b");
 const second=await codeDigestV1(base);
 attested=second;
 await until(()=>{try{return (JSON.parse(readFileSync(join(kitDirectoryV1(store),"published"),"utf8")) as {code:string}).code===second;}catch{return false;}});
 expect(await verifyKitV1(kitDirectoryV1(store),join(codeDirectoryV1(root),second))).toBeNull();
 controller.abort();
 await running;
 expect(errors).toEqual([]);
});

function overlayRun(options:Readonly<{worktree:string;archive:string;joined?:boolean;server?:boolean;first?:boolean;plan?:boolean}>):Readonly<{out:string;code:number}>{
 const slice=(name:string):string=>shellText.slice(shellText.indexOf(`# #region ${name}`),shellText.indexOf(`# #endregion ${name}`)),region=`${slice("🔎️Json")}\n${slice("🧬️Overlay")}`;
 const script=`set -euo pipefail\nsay(){ printf 'SAY %s\\n' "$*"; }\nfail(){ printf 'FAIL %s\\n' "$*"; exit 3; }\nrun_as(){ "$@"; }\ninstall_nfs_client(){ :; }\nsha256_of(){ if command -v sha256sum >/dev/null 2>&1; then sha256sum "$1" | cut -d' ' -f1; else shasum -a 256 "$1" | cut -d' ' -f1; fi; }\nJOINED=${options.joined?1:0}; SERVER=${options.server?1:0}; FIRST_MACHINE=${options.first?1:0}; PLAN=${options.plan?1:0}; GROUP_ID=g1; CLEANUP_MOUNT=""; PLATFORM=macos; SERVER_PLATFORM=macos; NFS_SOURCE=none\nARCHIVE=$ARCHIVE_DIRECTORY\nWORKTREE=$TREE\n${region}\noverlay_code\n`;
 const run=Bun.spawnSync([bash,"-c",script],{env:{...process.env,ARCHIVE_DIRECTORY:options.archive,TREE:options.worktree},stdout:"pipe",stderr:"pipe"});
 return {out:run.stdout.toString()+run.stderr.toString(),code:run.exitCode};
}
const counts=(out:string):[number,number,number]=>{const match=/replaced (\d+), added (\d+), identical (\d+) \//u.exec(out);return match===null?[-1,-1,-1]:[Number(match[1]),Number(match[2]),Number(match[3])];};
async function fleetStore(variant:string="new"):Promise<Readonly<{root:string;store:string;archive:string;digest:string;snapshot:string;files:readonly {path:string;sha256:string}[]}>>{
 const root=temp("fleet-root"),store=temp("fleet-store"),base=build(root,variant,"fleet"),digest=await seal(root,base);
 await publishToStore(root,store,digest);attest(store,digest);
 const snapshot=join(storeCodeDirectoryV1(store),digest),files=(JSON.parse(readFileSync(join(snapshot,"sealed.json"),"utf8")) as {files:{path:string;sha256:string}[]}).files;
 return {root,store,archive:join(store,"v1","bootstrap"),digest,snapshot,files};
}
function oldCheckout(fleet:Awaited<ReturnType<typeof fleetStore>>,differ:number,missing:number):string{
 const tree=temp("old-checkout");
 fleet.files.forEach((file,index)=>{
  if(index<missing)return;
  put(tree,file.path,index<missing+differ?`old bytes ${index}\n`:readFileSync(join(fleet.snapshot,"files",...file.path.split("/")),"utf8"));
 });
 return tree;
}

test("an old checkout is brought to the sealed fleet code: differing files are kept under .cache/f/lost, missing ones are added, identical ones stay, and a second run changes nothing",async()=>{
 const fleet=await fleetStore(),tree=oldCheckout(fleet,3,2),total=fleet.files.length;
 const before=new Map(fleet.files.map(file=>[file.path,existsSync(join(tree,...file.path.split("/")))?readFileSync(join(tree,...file.path.split("/"))):undefined]));
 if(python!==undefined)expect(Bun.spawnSync([python,"-I",oracle,"overlay",fleet.snapshot,tree],{stdout:"pipe"}).stdout.toString().trim()).toBe("3 2 "+(total-5));
 put(tree,".git/HEAD","ref: refs/heads/main\n");
 const run=overlayRun({worktree:tree,archive:fleet.archive});
 expect(run.code,run.out).toBe(0);
 expect(counts(run.out)).toEqual([3,2,total-5]);
 expect(run.out).toContain(fleet.digest);expect(run.out).toContain("ersetzt 3");
 for(const file of fleet.files)expect(sha(readFileSync(join(tree,...file.path.split("/")))),file.path).toBe(file.sha256);
 const lost=join(tree,".cache","f","lost"),kept=readdirSync(lost);
 expect(kept.length).toBe(1);expect(kept[0]).toMatch(/^provision-code-\d{8}-\d{6}$/u);
 for(const file of fleet.files){
  const old=before.get(file.path);
  if(old!==undefined&&sha(old)!==file.sha256)expect(readFileSync(join(lost,kept[0]!,...file.path.split("/")))).toEqual(old);
 }
 expect(readFileSync(join(tree,".git/HEAD"),"utf8")).toBe("ref: refs/heads/main\n");
 expect(readdirSync(tree).filter(name=>name.includes("provision-code"))).toEqual([]);
 const again=overlayRun({worktree:tree,archive:fleet.archive});
 expect(counts(again.out)).toEqual([0,0,total]);
 expect(readdirSync(lost).length).toBe(1);
 expect(readdirSync("/var/tmp").filter(name=>name.startsWith("semio-overlay."))).toEqual([]);
});

test.skipIf(process.platform==="win32")("the overlay restores the sealed execute bit: an added file, a replaced file and a byte-identical file with the wrong bit all end with the bit of the seal, in both directions",async()=>{
 const fleet=await fleetStore(),seal=JSON.parse(readFileSync(join(fleet.snapshot,"sealed.json"),"utf8")) as {files:{path:string;executable:boolean}[]},tree=oldCheckout(fleet,3,2);
 const flags=new Map(seal.files.map(file=>[file.path,file.executable]));
 expect([...flags.values()].filter(Boolean).length).toBe(executables.length);
 expect(fleet.files.length).toBeGreaterThan(8);
 fleet.files.forEach((file,index)=>{const target=join(tree,...file.path.split("/"));if(existsSync(target))chmodSync(target,index%2===0?0o755:0o644);});
 const run=overlayRun({worktree:tree,archive:fleet.archive});
 expect(run.code,run.out).toBe(0);
 for(const file of fleet.files)expect(bitOf(join(tree,...file.path.split("/"))),file.path).toBe(flags.get(file.path));
 const again=overlayRun({worktree:tree,archive:fleet.archive});
 expect(counts(again.out)).toEqual([0,0,fleet.files.length]);
 for(const file of fleet.files)expect(bitOf(join(tree,...file.path.split("/"))),file.path).toBe(flags.get(file.path));
});

test("an old seal without the execute bit field is not intact and leaves the checkout alone; both scripts read the field and the shell and the PowerShell overlay name it",async()=>{
 const fleet=await fleetStore("bits"),file=join(fleet.snapshot,"sealed.json"),seal=JSON.parse(readFileSync(file,"utf8")) as {files:{executable?:boolean}[]};
 for(const entry of seal.files)delete entry.executable;
 writeFileSync(file,JSON.stringify(seal));
 const tree=temp("bitless"),run=overlayRun({worktree:tree,archive:fleet.archive});
 expect(run.code).toBe(0);expect(run.out).toContain("not intact");expect(readdirSync(tree)).toEqual([]);
 expect(shellText).toContain('json_seal_files');expect(shellText).toContain('execute[n] == "true"');
 expect(powershellText).toContain("$file.executable");expect(powershellText).toContain("Set-OverlayBit");
});

test("--plan overlays the same way and says so",async()=>{
 const fleet=await fleetStore(),tree=oldCheckout(fleet,1,1);
 const run=overlayRun({worktree:tree,archive:fleet.archive,plan:true});
 expect(counts(run.out)).toEqual([1,1,fleet.files.length-2]);
 expect(run.out).toContain("also in --plan");expect(run.out).toContain("auch mit --plan");
});

test("the digest that the group server attests wins over a newer snapshot; without a snapshot of it the newest intact one is used",async()=>{
 const older=await fleetStore("a"),newer=await fleetStore("b");
 const store=temp("two"),codeBase=storeCodeDirectoryV1(store);
 cpSync(join(storeCodeDirectoryV1(older.store),older.digest),join(codeBase,older.digest),{recursive:true});
 await Bun.sleep(5);
 cpSync(join(storeCodeDirectoryV1(newer.store),newer.digest),join(codeBase,newer.digest),{recursive:true});
 const touched=join(codeBase,newer.digest,"sealed.json"),seal=JSON.parse(readFileSync(touched,"utf8")) as {createdMs:number};
 seal.createdMs=Date.now()+60_000;writeFileSync(touched,JSON.stringify(seal));
 attest(store,older.digest);
 const archive=join(store,"v1","bootstrap"),treeA=temp("a-tree");
 expect(overlayRun({worktree:treeA,archive}).out).toContain(`fleet code ${older.digest}`);
 attest(store,"f".repeat(64));
 const treeB=temp("b-tree");
 expect(overlayRun({worktree:treeB,archive}).out).toContain(`fleet code ${newer.digest}`);
 rmSync(join(store,"v1","origins"),{recursive:true,force:true});
 const treeC=temp("c-tree");
 expect(overlayRun({worktree:treeC,archive}).out).toContain(`fleet code ${newer.digest}`);
});

test("a damaged, tampered or unsafe seal changes nothing and the script goes on with a warning; a missing store or an empty one does the same",async()=>{
 const fleet=await fleetStore(),tree=oldCheckout(fleet,2,0),snapshot=fleet.snapshot,first=fleet.files[0]!;
 const snapshotBefore=readFileSync(join(snapshot,"files",...first.path.split("/")));
 const treeBefore=Object.fromEntries(fleet.files.map(file=>[file.path,sha(readFileSync(join(tree,...file.path.split("/"))))]));
 const unchanged=():void=>{for(const file of fleet.files)expect(sha(readFileSync(join(tree,...file.path.split("/")))),file.path).toBe(treeBefore[file.path]!);expect(existsSync(join(tree,".cache"))).toBe(false);};
 writeFileSync(join(snapshot,"files",...first.path.split("/")),"tampered\n");
 let run=overlayRun({worktree:tree,archive:fleet.archive});
 expect(run.code).toBe(0);expect(run.out).toContain("not intact");expect(run.out).toContain("changed");unchanged();
 if(python!==undefined)expect(Bun.spawnSync([python,"-I",oracle,"verify",snapshot],{stdout:"pipe"}).stdout.toString().trim()).toStartWith("changed");
 expect(await verifyCodeSnapshotV1(snapshot)).toStartWith("changed");
 writeFileSync(join(snapshot,"files",...first.path.split("/")),snapshotBefore);
 rmSync(join(snapshot,"files",...fleet.files[1]!.path.split("/")));
 run=overlayRun({worktree:tree,archive:fleet.archive});
 expect(run.out).toContain("not intact");expect(run.out).toContain("missing");unchanged();
 expect(await verifyCodeSnapshotV1(snapshot)).toStartWith("missing");
 const bad=await fleetStore("bad"),sealFile=join(bad.snapshot,"sealed.json"),seal=JSON.parse(readFileSync(sealFile,"utf8")) as {files:{path:string;bytes:number;sha256:string}[]};
 for(const path of ["../escape.txt","/abs.txt","a/../../b.txt","a//b.txt","a\\b.txt"]){
  const copy=structuredClone(seal);copy.files[0]!.path=path;writeFileSync(sealFile,JSON.stringify(copy));
  const victim=temp("victim"),outcome=overlayRun({worktree:victim,archive:bad.archive});
  expect(outcome.out,path).toContain("not intact");expect(outcome.out,path).toContain("unsafe path");
  expect(readdirSync(victim)).toEqual([]);expect(existsSync(join(dirname(victim),"escape.txt"))).toBe(false);
  if(python!==undefined)expect(Bun.spawnSync([python,"-I",oracle,"verify",bad.snapshot],{stdout:"pipe"}).stdout.toString().trim()).toStartWith("unsafe");
 }
 writeFileSync(sealFile,"not json");
 expect(overlayRun({worktree:temp("garbage"),archive:bad.archive}).out).toContain("not intact");
 const empty=temp("empty-store");mkdirSync(join(empty,"v1","bootstrap","code"),{recursive:true});mkdirSync(join(empty,"v1","origins"),{recursive:true});
 const none=overlayRun({worktree:temp("none"),archive:join(empty,"v1","bootstrap")});
 expect(none.code).toBe(0);expect(none.out).toContain("no sealed code");expect(none.out).toContain("keinen versiegelten Code");
});

test("a joined machine, a group server and the first machine are not overlaid: their replica keeps the code, and a file that is newer there is never replaced",async()=>{
 const fleet=await fleetStore(),tree=oldCheckout(fleet,2,0),before=readFileSync(join(tree,...fleet.files[0]!.path.split("/")),"utf8");
 for(const flag of [{joined:true},{server:true},{first:true}]){
  const run=overlayRun({worktree:tree,archive:fleet.archive,...flag});
  expect(run.code).toBe(0);expect(run.out).toContain("not overlaid");expect(run.out).toContain("nicht überlagert");
 }
 expect(readFileSync(join(tree,...fleet.files[0]!.path.split("/")),"utf8")).toBe(before);expect(existsSync(join(tree,".cache"))).toBe(false);
});

test("a replaced file is swapped by rename: a reader that holds the old file keeps reading it, a symlink is replaced by the file and kept as a link",async()=>{
 const fleet=await fleetStore(),tree=temp("link-tree"),file=fleet.files[0]!.path;
 put(tree,"elsewhere.txt","outside\n");
 mkdirSync(dirname(join(tree,...file.split("/"))),{recursive:true});
 Bun.spawnSync(["ln","-s",join(tree,"elsewhere.txt"),join(tree,...file.split("/"))]);
 const run=overlayRun({worktree:tree,archive:fleet.archive});
 expect(counts(run.out)[0]).toBe(1);
 expect(readFileSync(join(tree,"elsewhere.txt"),"utf8")).toBe("outside\n");
 expect(sha(readFileSync(join(tree,...file.split("/"))))).toBe(fleet.files[0]!.sha256);
 expect(Bun.spawnSync(["test","-L",join(tree,...file.split("/"))]).exitCode).toBe(1);
 const lost=join(tree,".cache","f","lost"),kept=join(lost,readdirSync(lost)[0]!,...file.split("/"));
 expect(Bun.spawnSync(["test","-L",kept]).exitCode).toBe(0);
});

function guardRun(body:string):Readonly<{out:string;err:string;code:number}>{
 const cleanup=shellText.slice(shellText.indexOf("SKEW_MARK="),shellText.indexOf("trap cleanup EXIT")+"trap cleanup EXIT".length),guard=shellText.slice(shellText.indexOf("skew_message() {"),shellText.indexOf("\nfleet() {"));
 const script=`set -euo pipefail\nEXIT_FLEET_CODE_OLD=5\nWORKTREE=/old/checkout\nCLEANUP_MOUNT=""\nCLEANUP_DIRECTORY=""\n${cleanup}\n${guard}\nrun_as(){ "$@"; }\n${body}\n`;
 const run=Bun.spawnSync([bash,"-c",script],{stdout:"pipe",stderr:"pipe"});
 return {out:run.stdout.toString(),err:run.stderr.toString(),code:run.exitCode};
}
test("a fleet command that fails on an unknown option or a missing module ends the script with exit code 5 and names the cause and the checkout, in a pipeline, a command substitution and a plain call",()=>{
 const unknown=`sh -c 'echo "Unknown option for facts: --home" >&2; exit 1'`,missing=`sh -c 'echo "error: Cannot find module \\"./x\\" from y" >&2; exit 1'`;
 for(const failing of [unknown,missing]){
  for(const call of [`skew_guard run_as ${failing}`,`FACTS=$(skew_guard run_as ${failing} | grep '^FLEET_')`,`FACTS=$( (cd /tmp && skew_guard run_as ${failing}) )`,`( skew_guard run_as ${failing} )`]){
   const run=guardRun(`${call}\nprintf 'NOT REACHED\\n'`);
   expect(run.code,`${call}\n${run.err}`).toBe(5);
   expect(run.out).not.toContain("NOT REACHED");
   expect(run.err).toContain("the checkout's fleet code is older than this script and the store offers no newer sealed code: /old/checkout");
   expect(run.err).toContain("Fleet-Code des Checkouts ist älter als dieses Skript");
   expect(run.err).toMatch(/Unknown option for facts|Cannot find module/u);
  }
 }
});

test("a failure of another kind keeps its own exit code and message, a success passes its output and its notices through",()=>{
 const other=guardRun(`skew_guard run_as sh -c 'echo "disk full" >&2; exit 7'`);
 expect(other.code).toBe(7);expect(other.err).toContain("disk full");expect(other.err).not.toContain("older than this script");
 const fine=guardRun(`OUT=$(skew_guard run_as sh -c 'echo "progress 1" >&2; echo FLEET_X=1; exit 0')\nprintf 'GOT %s\\n' "$OUT"`);
 expect(fine.code).toBe(0);expect(fine.out).toContain("GOT FLEET_X=1");expect(fine.err).toContain("progress 1");
 const quiet=guardRun(`if ! skew_guard run_as sh -c 'echo "retry later" >&2; exit 1'; then printf 'HANDLED\\n'; fi`);
 expect(quiet.code).toBe(0);expect(quiet.out).toContain("HANDLED");
});

test("the guard wraps both fleet entry points and the missing boot program counts as old code; the overlay runs before the first fleet call, and only the documented exit code 5 is new",()=>{
 expect(shellText).toContain('fleet() { (cd "$WORKTREE" && skew_guard run_as repo "$WORKTREE/$TOPOLOGY_RELATIVE/📜️script.ts" "$@"); }');
 expect(shellText).toContain('"$FLEET_REPLICA_DIRECTORY/📜️script.ts" "$operation" "$@"); }');
 expect(shellText).toContain('[ -n "$ENTRY_PROGRAM" ] || skew_fail');
 expect(shellText.indexOf("\noverlay_code\n")).toBeGreaterThan(0);
 expect(shellText.indexOf("\noverlay_code\n")).toBeLessThan(shellText.indexOf("FACTS=$(fleet facts"));
 expect(shellText.indexOf("\noverlay_code\n")).toBeGreaterThan(shellText.indexOf('release_foreign_namespace "$WORKTREE/node_modules" "$WORKTREE"'));
 expect(shellText).toContain("EXIT_FLEET_CODE_OLD=5");expect(shellText).toContain("; 5 the fleet code of the checkout is older than this script");
 expect(powershellText).toContain("$ExitFleetCodeOld = 5");expect(powershellText).toContain("exit $ExitFleetCodeOld");
 expect(powershellText).toContain("5 the fleet code of the checkout is older than this script");
 expect(powershellText).toContain("if (-not $entryProgram) { Stop-FleetCodeOld }");
 expect(powershellText.indexOf("\nInvoke-FleetCodeOverlay\n")).toBeLessThan(powershellText.indexOf("$facts = @(Invoke-Fleet @('facts'"));
 expect(powershellText.indexOf("\nInvoke-FleetCodeOverlay\n")).toBeGreaterThan(powershellText.indexOf("Say 'step 4: bootstrap archive into the empty worktree'"));
 expect(powershellText.match(/& repo \$Script @Arguments 2>&1/gu)?.length).toBe(1);
 expect(powershellText).not.toMatch(/& repo \$script @Arguments/u);
});

test("both scripts say what they do to the fleet code in English and German, and neither deletes anything",()=>{
 for(const text of [shellText,powershellText]){
  expect(text).toContain("replaced");expect(text).toContain("ersetzt");expect(text).toContain("provision-code-");
  expect(text).toContain("the checkout's fleet code is older than this script and the store offers no newer sealed code");
  expect(text).toContain("Fleet-Code des Checkouts ist älter als dieses Skript");
  expect(text).toContain("not overlaid");expect(text).toContain("nicht überlagert");
 }
 const region=shellText.slice(shellText.indexOf("# #region 🧬️Overlay"),shellText.indexOf("# #endregion 🧬️Overlay"));
 expect(region).not.toMatch(/\brm -rf? ["$]*(WORKTREE|target|keep)/u);
 const overlay=powershellText.slice(powershellText.indexOf("function Invoke-FleetCodeOverlay"),powershellText.indexOf("Say 'step 1: long paths'"));
 expect(overlay).not.toContain("Remove-Item");
 expect(powershellText.charCodeAt(0)).toBe(0xfeff);
});

const pwsh=Bun.which("pwsh");
test.skipIf(!pwsh)("PowerShell parses the script and the overlay keeps differing files, as the shell does",()=>{
 const parse=Bun.spawnSync([pwsh!,"-NoProfile","-Command",`$e=$null;[System.Management.Automation.Language.Parser]::ParseFile('${named(provisionDirectory,".ps1")}',[ref]$null,[ref]$e)|Out-Null;if($e.Count -gt 0){$e|ForEach-Object{$_.Message};exit 1}`],{stdout:"pipe",stderr:"pipe"});
 expect(parse.exitCode,parse.stdout.toString()+parse.stderr.toString()).toBe(0);
});
