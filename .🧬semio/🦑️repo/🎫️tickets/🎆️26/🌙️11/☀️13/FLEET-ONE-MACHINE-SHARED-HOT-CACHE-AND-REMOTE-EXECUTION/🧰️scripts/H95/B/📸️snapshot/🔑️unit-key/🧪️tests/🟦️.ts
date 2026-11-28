import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import fixture from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🧳️portable/🧫️fixtures/🔣️.json" with {type:"json"};
import {keyEnvironmentV1,normalizeRustcArgumentsV1,sha256V1,unitKeyV1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts";
import {localKeyV1,portableRootsV1,portableTextV1,remoteKeyV1,type PlatformV1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {snapshotAcquireV1} from "../../⏩️advance/🟦️.ts";
import {landOverlayBaseV1} from "../../../🚉️land/🟦️.ts";

/** 🔑️ Unit-cache keys from a snapshot root: the recorded real unit of the portability test of the unit cache, keyed with the checkout at the live root and with the checkout at the root of a snapshot lane (the real shape `~/.cache/semio/l/<6 hex>/s/<lane>`), everything else equal, must give one key. */
const scratch:string[]=[];
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(tmpdir(),"uk-"+label+"-")));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const platform=fixture.platform as PlatformV1;
type Machine=Readonly<{root:string;home:string;sdk:string}>;

function keyAt(machine:Machine,source:string,mutate:(args:string[])=>string[]=args=>args){
 const lane=machine.root+"/.cache/lane-"+sha256V1(machine.home).slice(0,6)+"/t-"+sha256V1(machine.root).slice(0,8);
 const raw=mutate(fixture.arguments.map(argument=>argument.replaceAll("⟨cargo-build-dir⟩",lane).replaceAll("⟨rust-src⟩",machine.home+"/.rustup/toolchains/nightly/lib/rustlib/src/rust").replaceAll(fixture.root,machine.root)));
 const environment=keyEnvironmentV1({...Object.fromEntries(fixture.environment.map(([name,value])=>[name,value!.replaceAll(fixture.root,machine.root)])),SDKROOT:machine.sdk,PWD:machine.root,HOME:machine.home,CARGO_HOME:machine.home+"/.cargo",TMPDIR:machine.home+"/tmp"},undefined);
 const normalized=normalizeRustcArgumentsV1(raw,new Map(),undefined),input={toolchain:fixture.toolchain,normalized,sourceSha256:sha256V1(source),externs:[],environment};
 const roots=portableRootsV1({cwd:machine.root,workspace:machine.root,hints:[fixture.manifest.replaceAll(fixture.root,machine.root),machine.root]});
 return {remote:remoteKeyV1(platform,input,roots),local:localKeyV1(input,roots),unit:unitKeyV1(input),normalized,environment,input,roots};
}

const sdk=():string=>{const base=directory("sdk"),real=join(base,"MacOSX15.4.sdk");mkdirSync(real,{recursive:true});return real;};

test("the key of one real unit is equal with the checkout at the live root and at the root of a snapshot lane, and the source bytes it hashes are equal too",async()=>{
 const base=directory("live"),root=join(base,"semio"),file="🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust/🦀️.rs";
 mkdirSync(join(root,dirname(file)),{recursive:true});writeFileSync(join(root,file),"pub fn hash()->u32{7}\n");writeFileSync(join(root,"package.json"),"{}");
 const store=join(base,"store"),cache=directory("cache"),home="/Users/ueli",toolsdk=sdk();
 mkdirSync(join(store,"v1","origins","g1-x","segments"),{recursive:true});writeFileSync(join(store,"v1","origins","g1-x","segments","000000000001.jsonl"),"");
 const shortBase=landOverlayBaseV1(root,{SEMIO_LAND_CACHE:cache}),receipt=(await snapshotAcquireV1({root,lane:"keyprobe",mode:"root",base:join(shortBase,"s"),build:true},{store,heavyMembers:[],link:async()=>{}}))!;
 expect(receipt.snapshotRoot).not.toBe(root);expect(receipt.snapshotRoot.length).toBeLessThan(150);
 const liveSource=await Bun.file(join(root,file)).text(),snapSource=await Bun.file(join(receipt.snapshotRoot,file)).text();
 expect(snapSource).toBe(liveSource);
 const live=keyAt({root,home,sdk:toolsdk},liveSource),snap=keyAt({root:receipt.snapshotRoot,home,sdk:toolsdk},snapSource);
 expect(snap.remote).toBe(live.remote);
 expect(snap.local).toBe(live.local);
 expect(snap.unit,"the raw machine-bound key still differs: the portable key is the one that must not").not.toBe(live.unit);
 expect(portableTextV1(JSON.stringify(snap.normalized),snap.roots)).not.toContain(receipt.snapshotRoot);
 expect(portableTextV1(JSON.stringify(snap.environment),snap.roots)).not.toContain(receipt.snapshotRoot);
 expect(portableTextV1(JSON.stringify(snap.normalized),snap.roots)).toBe(portableTextV1(JSON.stringify(live.normalized),live.roots));
});

test("what is an input still moves the key from the snapshot root: the source bytes, the profile and a feature",()=>{
 const root=join(directory("a"),"semio"),other=join(directory("b"),".cache","semio","l","abc123","s","lane");mkdirSync(root,{recursive:true});mkdirSync(other,{recursive:true});
 const home="/Users/ueli",s=sdk(),base=keyAt({root,home,sdk:s},"a").remote;
 expect(keyAt({root:other,home,sdk:s},"a").remote).toBe(base);
 expect(keyAt({root:other,home,sdk:s},"b").remote).not.toBe(base);
 expect(keyAt({root:other,home,sdk:s},"a",args=>args.map(argument=>argument==="opt-level=3"?"opt-level=0":argument)).remote).not.toBe(base);
 expect(keyAt({root:other,home,sdk:s},"a",args=>[...args,"--cfg",'feature="x"']).remote).not.toBe(base);
});

test("a unit whose bytes name a real path of the machine (path-dependent) is keyed on that path as well: from a snapshot root it shares only with another build at the same snapshot root, never with the live root (the declared limit of the snapshot)",()=>{
 const root=join(directory("a"),"semio"),other=join(directory("b"),".cache","semio","l","abc123","s","lane");mkdirSync(root,{recursive:true});mkdirSync(other,{recursive:true});
 const home="/Users/ueli",s=sdk(),live=keyAt({root,home,sdk:s},"a"),snap=keyAt({root:other,home,sdk:s},"a");
 expect(remoteKeyV1(platform,snap.input,snap.roots,[other])).not.toBe(remoteKeyV1(platform,live.input,live.roots,[root]));
 expect(remoteKeyV1(platform,snap.input,snap.roots,[other])).toBe(remoteKeyV1(platform,snap.input,snap.roots,[other]));
});
