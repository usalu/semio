import {afterAll,describe,expect,test} from "bun:test";
import {chmodSync,existsSync,mkdirSync,readFileSync,readdirSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {join} from "node:path";
import {ensureWatcherV1,readWatcherPinV1,watcherManifestV1} from "../../🔭️watch/🏗️provision/🟨️.mjs";
import {toolsPartitionWritableV1} from "../../🔭️watch/🏗️provision/🔌️tools/🟨️.mjs";
import {cleanup,tempRoot} from "../🟦️.ts";

const SOURCE=join(import.meta.dir,"../../🔭️watch/🦀️rust/🦀️.rs"),PIN=join(import.meta.dir,"../../🔭️watch/🦀️rust/🔣️.json");
const roots:string[]=[];afterAll(()=>roots.forEach(cleanup));
const frame='{"v":1,"t":"version","name":"semio-repo-watch","protocol":1,"os":"test"}';
const script=(body:string)=>"#!/bin/sh\n"+body+"\n";

describe("watcher pin",()=>{
 test("the committed pin names exactly the source on disk (regenerate it whenever the source changes)",()=>{
  const pin=readWatcherPinV1(PIN,SOURCE),bytes=readFileSync(SOURCE);
  expect(pin.sha256).toBe(createHash("sha256").update(bytes).digest("hex"));
  const document=JSON.parse(readFileSync(PIN,"utf8"));
  expect(document.sources).toHaveLength(1);expect(document.sources[0].bytes).toBe(bytes.length);expect(document.sources[0]).toMatchObject({platform:"any",architecture:"any"});
 });
 test("a stale pin, a pin for one platform and a broken pin are refused",()=>{
  const root=tempRoot("pin-");roots.push(root);
  const good=JSON.parse(readFileSync(PIN,"utf8"));
  for(const[name,mutate]of[["stale",(d:any)=>{d.sources[0].sha256="0".repeat(64);}],["bytes",(d:any)=>{d.sources[0].bytes+=1;}],["platform",(d:any)=>{d.sources[0].platform="darwin";}],["version",(d:any)=>{d.version=1;}]] as const){
   const copy=JSON.parse(JSON.stringify(good));mutate(copy);const path=join(root,name+".json");writeFileSync(path,JSON.stringify(copy));
   expect(()=>readWatcherPinV1(path,SOURCE)).toThrow();
  }
 });
 test("the build package has no dependency and its own workspace",()=>{
  const manifest=watcherManifestV1();
  expect(manifest).toContain("[workspace]");expect(manifest).not.toContain("[dependencies]");expect(manifest).toContain('path = "main.rs"');
 });
});

describe("ensure the watcher",()=>{
 const harness=()=>{const root=tempRoot("ensure-");roots.push(root);return{root,outputRoot:join(root,"tools","repo-watch")};};
 test("an installed binary is used without compiling",async()=>{
  const{outputRoot}=harness();let compiled=0;
  const result=await ensureWatcherV1({sourcePath:SOURCE,pinPath:PIN,outputRoot,locate:async()=>"/installed/semio-repo-watch",compile:async()=>{compiled++;return"";}});
  expect(result).toMatchObject({path:"/installed/semio-repo-watch",built:false});expect(compiled).toBe(0);
 });
 test("a missing binary is compiled once into the content addressed directory, probed, and found next time",async()=>{
  const{root,outputRoot}=harness();const phases:string[]=[];
  const compile=async({directory}:{directory:string})=>{
   expect(readFileSync(join(directory,"main.rs"),"utf8")).toBe(readFileSync(SOURCE,"utf8"));expect(existsSync(join(directory,"Cargo.toml"))).toBe(true);
   const out=join(directory,"target","release");mkdirSync(out,{recursive:true});const path=join(out,"semio-repo-watch");writeFileSync(path,script("echo '"+frame+"'"));chmodSync(path,0o755);return path;
  };
  const first=await ensureWatcherV1({sourcePath:SOURCE,pinPath:PIN,outputRoot,locate:async()=>undefined,compile,progress:p=>phases.push(p)});
  expect(first.built).toBe(true);expect(first.path).toBe(join(outputRoot,first.sha256.slice(0,16),"semio-repo-watch"));expect(phases).toEqual(["compile","ready"]);
  expect(readdirSync(join(outputRoot,first.sha256.slice(0,16)))).toEqual(["semio-repo-watch"]);
  const second=await ensureWatcherV1({sourcePath:SOURCE,pinPath:PIN,outputRoot,locate:async()=>first.path,compile:async()=>{throw new Error("must not compile");}});
  expect(second).toMatchObject({built:false,path:first.path});void root;
 });
 test("a build that does not answer the protocol is never installed; a failing or cancelled compile leaves nothing",async()=>{
  const{outputRoot}=harness();
  const bad=async({directory}:{directory:string})=>{const path=join(directory,"x");writeFileSync(path,script("echo nonsense"));chmodSync(path,0o755);return path;};
  await expect(ensureWatcherV1({sourcePath:SOURCE,pinPath:PIN,outputRoot,locate:async()=>undefined,compile:bad})).rejects.toThrow(/version frame/u);
  const sha=readWatcherPinV1(PIN,SOURCE).sha256.slice(0,16);
  expect(existsSync(join(outputRoot,sha,"semio-repo-watch"))).toBe(false);
  await expect(ensureWatcherV1({sourcePath:SOURCE,pinPath:PIN,outputRoot,locate:async()=>undefined,compile:async()=>{throw new Error("cancelled");}})).rejects.toThrow("cancelled");
  expect(readdirSync(join(outputRoot,sha)).filter(name=>!name.startsWith("."))).toEqual([]);
 });
 test("an unwritable output root (an image that is not mounted) is refused before anything is compiled or written",async()=>{
  const{outputRoot}=harness();let compiled=0;
  await expect(ensureWatcherV1({sourcePath:SOURCE,pinPath:PIN,outputRoot,locate:async()=>undefined,writable:()=>false,compile:async()=>{compiled++;return"";}})).rejects.toThrow(/not mounted/u);
  expect(compiled).toBe(0);expect(existsSync(outputRoot)?readdirSync(outputRoot):[]).toEqual([]);
  const installed=await ensureWatcherV1({sourcePath:SOURCE,pinPath:PIN,outputRoot,locate:async()=>"/installed/semio-repo-watch",writable:()=>false,compile:async()=>{throw new Error("must not compile");}});
  expect(installed.path).toBe("/installed/semio-repo-watch");
 });
 test("the tools partition is writable without an image, and an image partition is not writable while its root is no mount point",()=>{
  const root=tempRoot("partition-");roots.push(root);
  const image=join(root,".cache","b","t","m"),partition=join(root,".cache","b","t");
  expect(toolsPartitionWritableV1(root)).toBe(true);
  mkdirSync(partition,{recursive:true});writeFileSync(join(partition,"image.json"),"{}");
  expect(toolsPartitionWritableV1(root)).toBe(false);
  mkdirSync(image);expect(toolsPartitionWritableV1(root)).toBe(false);
 });
 test.skipIf(process.env.SEMIO_RESIDENT_CARGO!=="1")("a real cargo build of the pinned source yields a watcher that answers the version frame",async()=>{
  const{outputRoot}=harness();
  const{spawnSync}=await import("node:child_process");
  const compile=async({directory}:{directory:string})=>{const r=spawnSync("cargo",["build","--release","--manifest-path",join(directory,"Cargo.toml"),"--target-dir",join(directory,"target")],{cwd:directory,encoding:"utf8"});if(r.status!==0)throw new Error(r.stderr);return join(directory,"target","release","semio-repo-watch");};
  const result=await ensureWatcherV1({sourcePath:SOURCE,pinPath:PIN,outputRoot,locate:async()=>undefined,compile});
  expect(result.built).toBe(true);
 },300000);
});
