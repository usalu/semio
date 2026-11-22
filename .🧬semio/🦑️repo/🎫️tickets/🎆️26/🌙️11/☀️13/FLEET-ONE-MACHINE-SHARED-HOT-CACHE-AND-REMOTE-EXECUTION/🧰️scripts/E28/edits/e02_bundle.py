import json
B="⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️bundle/"
SCHEMA={
 "$schema":"http://json-schema.org/draft-07/schema#",
 "$id":"schema://repo.nx/AuthorizerBundleV1",
 "title":"Nx Authorizer Bundle V1",
 "description":"One compiled authorizer bundle is its own content-addressed directory beside the Nx dependency generation it runs with: the bundle, the digest manifests of its source and compiler closures and a marker naming both its own key and the dependency generation key.",
 "type":"object",
 "$defs":{
  "Policy":{
   "description":"Where bundles live below the tools image, the marker file, the manifests the key covers, the lease every reader holds and how many of the newest bundles retirement keeps.",
   "const":{"version":1,"directory":"nx-authorizers","marker":".semio-nx-authorizers.json","manifests":[".semio-authorizer-source.json",".semio-authorizer-compiler.json"],"bundle":"semio-authorizers.mjs","lease":"nx-authorizer-bundle:","keepNewest":4,"minimumAgeMilliseconds":600000,"maximumCandidates":256,"maximumMarkerBytes":65536}
  },
  "Marker":{
   "type":"object","additionalProperties":False,"required":["version","digest","deps","authorizers"],
   "properties":{"version":{"const":1},"digest":{"type":"string","pattern":"^[a-f0-9]{64}$"},"deps":{"type":"string","pattern":"^[a-f0-9]{64}$"},"authorizers":{"type":"object"}}
  },
  "RetirementRow":{"type":"object","additionalProperties":False,"required":["identity","touchedAt","held"],"properties":{"identity":{"type":"string","pattern":"^[a-f0-9]{64}$"},"touchedAt":{"type":"number","minimum":0},"held":{"type":"boolean"}}},
  "RetirementCase":{"type":"object","additionalProperties":False,"required":["name","nowMs","keepNewest","minimumAgeMilliseconds","rows","expected"],"properties":{"name":{"type":"string","minLength":1},"nowMs":{"type":"number","minimum":0},"keepNewest":{"type":"integer","minimum":0},"minimumAgeMilliseconds":{"type":"integer","minimum":0},"rows":{"type":"array","items":{"$ref":"#/$defs/RetirementRow"}},"expected":{"type":"array","items":{"type":"string","pattern":"^[a-f0-9]{64}$"}}}},
  "RetirementCorpus":{"type":"object","additionalProperties":False,"required":["version","cases"],"properties":{"version":{"const":1},"cases":{"type":"array","minItems":1,"items":{"$ref":"#/$defs/RetirementCase"}}}}
 }
}
MODULE='''import {lstatSync,readFileSync,readdirSync,utimesSync} from "node:fs";
import {basename,dirname,isAbsolute,join,resolve} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {nxAuthorizerBundleIdentityV1} from "../../📦️sdk/🪪️identity/🟦️.ts";
import {nxAuthorizerDigestManifestPolicyV1,nxAuthorizerReceiptHoldsV1,type NxAuthorizerReceiptV1} from "../📤️receipt/🟦️.ts";
import {compileJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 📐️ Names where compiled authorizer bundles live below the tools image and how long and how many of them retirement keeps. @see ./🧬️schema/🔣️.json */
export const nxAuthorizerBundlePolicyV1=schema.$defs.Policy.const;
/** 🏷️ Binds one bundle directory to its own key and to the dependency generation it runs beside. */
export interface NxAuthorizerBundleMarkerV1{readonly version:1;readonly digest:string;readonly deps:string;readonly authorizers:NxAuthorizerReceiptV1}
const markerPlan=compileJsonSchemaSubset({$ref:"#/$defs/Marker"},schema);
const readExact=(directory:string,name:string,maximumBytes:number):Buffer=>{
 const path=join(directory,name),before=lstatSync(path,{bigint:true});
 if(!before.isFile()||before.isSymbolicLink()||before.size>BigInt(maximumBytes))throw Error("Bounded original authorizer bundle file required");
 const bytes=readFileSync(path),after=lstatSync(path,{bigint:true});
 if(before.dev!==after.dev||before.ino!==after.ino||before.size!==after.size||before.mtimeNs!==after.mtimeNs||before.ctimeNs!==after.ctimeNs)throw Error("Authorizer bundle file changed while read");
 return bytes;
};
/** 📍️ The directory of one bundle below a tools image root. */
export const nxAuthorizerBundleDirectoryV1=(toolsRoot:string,identity:string):string=>join(toolsRoot,nxAuthorizerBundlePolicyV1.directory,identity);
/** 📜️ Reads the closed marker of one bundle directory whose name must be the key it declares. */
export function readNxAuthorizerBundleMarkerV1(directory:string):NxAuthorizerBundleMarkerV1{
 const marker=JSON.parse(readExact(directory,nxAuthorizerBundlePolicyV1.marker,nxAuthorizerBundlePolicyV1.maximumMarkerBytes).toString("utf8"));
 if(markerPlan.validate(marker).length||basename(directory)!==marker.digest)throw Error("Exact authorizer bundle marker required");
 return marker;
}
/** ✅️ Holds only when the marker, the key recomputed from both manifests, the receipt and the compiled bytes all agree and the bundle belongs to the named dependency generation. */
export function nxAuthorizerBundleHoldsV1(directory:string,dependencies:string):NxAuthorizerBundleMarkerV1|null{
 try{
  const marker=readNxAuthorizerBundleMarkerV1(directory);if(marker.deps!==dependencies)return null;
  const files=new Map(nxAuthorizerBundlePolicyV1.manifests.map(name=>[name,readExact(directory,name,nxAuthorizerDigestManifestPolicyV1.maximumBytes)] as const));
  if(nxAuthorizerBundleIdentityV1(marker.deps,files)!==marker.digest)return null;
  return nxAuthorizerReceiptHoldsV1(directory,marker.authorizers)?marker:null;
 }catch{return null;}
}
/** 🧭️ Derives from a loaded bundle directory the dependency generation it runs beside: the lease identity and the toolkit root every later use hangs on. */
export function nxAuthorizerBundleToolkitV1(bundleRoot:string):Readonly<{authorizer:string;identity:string;toolkitRoot:string;bundle:string}>{
 const root=resolve(bundleRoot),parent=dirname(root);
 if(!isAbsolute(root)||basename(parent)!==nxAuthorizerBundlePolicyV1.directory)throw Error("Exact loaded authorizer bundle directory required");
 const marker=readNxAuthorizerBundleMarkerV1(root);
 return Object.freeze({authorizer:marker.digest,identity:marker.deps,toolkitRoot:join(dirname(parent),"nx",marker.deps),bundle:join(root,nxAuthorizerBundlePolicyV1.bundle)});
}
/** 🖐️ Marks one bundle as just used, so retirement keeps it for its minimum age; a volume that refuses the mark only loses that protection. */
export function touchNxAuthorizerBundleV1(directory:string):void{try{const now=new Date();utimesSync(directory,now,now);}catch{}}
/** 🔎️ The most recently used bundle that holds beside one dependency generation, for harnesses that start the actual toolkit without having provisioned it. */
export function newestNxAuthorizerBundleV1(toolsRoot:string,dependencies:string):string|undefined{
 let names:string[];try{names=readdirSync(join(toolsRoot,nxAuthorizerBundlePolicyV1.directory));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;}
 const born=(directory:string)=>{const row=lstatSync(directory);return Math.max(row.birthtimeMs>0?row.birthtimeMs:0,row.mtimeMs);};
 const rows=names.filter(name=>/^[a-f0-9]{64}$/u.test(name)).slice(0,nxAuthorizerBundlePolicyV1.maximumCandidates).map(name=>nxAuthorizerBundleDirectoryV1(toolsRoot,name)).filter(directory=>nxAuthorizerBundleHoldsV1(directory,dependencies)!==null).map(directory=>({directory,at:born(directory)}));
 return rows.sort((left,right)=>right.at-left.at||(left.directory<right.directory?-1:1))[0]?.directory;
}
'''
RETIRE='''import {lstatSync,readdirSync,rmSync} from "node:fs";
import {join} from "node:path";
import {nxAuthorizerBundleDirectoryV1,nxAuthorizerBundlePolicyV1 as policy} from "../🟦️.ts";
import {withResourceLeases,type ResourceLeaseControl} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

/** 🗂️ One bundle as the census sees it: its key, when it was born or last marked used and whether a live reader holds it. */
export type NxAuthorizerBundleRowV1=Readonly<{identity:string;touchedAt:number;held:boolean}>;
/** 🧮️ Names the bundles retirement may remove, least recently used first: everything behind the newest ones that no reader holds and that was not used within the minimum age. */
export function planNxAuthorizerBundleRetirementV1(rows:readonly NxAuthorizerBundleRowV1[],nowMs:number,keepNewest:number=policy.keepNewest,minimumAgeMilliseconds:number=policy.minimumAgeMilliseconds):readonly string[]{
 const newestFirst=[...rows].sort((left,right)=>right.touchedAt-left.touchedAt||(left.identity<right.identity?-1:left.identity>right.identity?1:0));
 return newestFirst.slice(keepNewest).filter(row=>!row.held&&nowMs-row.touchedAt>=minimumAgeMilliseconds).map(row=>row.identity).reverse();
}
/** 🧹️ Removes released old bundles of one tools image: each candidate needs the exclusive reader lease without waiting, so a bundle any running Nx process, daemon or provisioner holds stays. */
export async function retireNxAuthorizerBundlesV1(toolsRoot:string,leaseDirectory:string,control?:ResourceLeaseControl,options:Readonly<{nowMs?:number;keepNewest?:number;minimumAgeMilliseconds?:number;signal?:AbortSignal}>={}):Promise<readonly string[]>{
 let names:string[];try{names=readdirSync(join(toolsRoot,policy.directory));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return Object.freeze([]);throw error;}
 const hex=names.filter(name=>/^[a-f0-9]{64}$/u.test(name));if(hex.length>policy.maximumCandidates)throw Error("Bounded original authorizer bundle census required");
 const rows=hex.flatMap(identity=>{try{const row=lstatSync(nxAuthorizerBundleDirectoryV1(toolsRoot,identity));return row.isDirectory()?[{identity,touchedAt:Math.max(0,row.birthtimeMs>0?row.birthtimeMs:0,row.mtimeMs),held:false}]:[];}catch{return [];}});
 const signal=control?.signal??options.signal??new AbortController().signal,retired:string[]=[];
 for(const identity of planNxAuthorizerBundleRetirementV1(rows,options.nowMs??Date.now(),options.keepNewest,options.minimumAgeMilliseconds)){
  signal.throwIfAborted();
  try{await withResourceLeases({directory:leaseDirectory,resources:[{resource:policy.lease+identity,mode:"exclusive"}],signal,timeoutMs:0,...(control?{control}:{})},async()=>{const directory=nxAuthorizerBundleDirectoryV1(toolsRoot,identity),row=lstatSync(directory);if(!row.isDirectory()||row.isSymbolicLink())return;rmSync(directory,{recursive:true,force:true});retired.push(identity);});}
  catch(error){signal.throwIfAborted();if((error as Error).message!=="Resource lease timed out: "+policy.lease+identity)throw error;}
 }
 return Object.freeze(retired);
}
'''
PY='''"""🗑️ Independent Python oracle of the authorizer bundle retirement plan (standard library only): sort the rows newest first (ties by key), keep the newest ones, and retire the rest that no reader holds and that are at least the minimum age old, listed oldest first. Requests arrive as JSON on stdin."""
import json, sys


def plan(case):
    ordered = sorted(case["rows"], key=lambda row: (-row["touchedAt"], row["identity"]))
    behind = ordered[case["keepNewest"]:]
    due = [row["identity"] for row in behind if not row["held"] and case["nowMs"] - row["touchedAt"] >= case["minimumAgeMilliseconds"]]
    return list(reversed(due))


request = json.load(sys.stdin)
print(json.dumps([plan(case) for case in request["cases"]]))
'''
def hx(c):return c*64
def row(c,at,held=False):return {"identity":hx(c),"touchedAt":at,"held":held}
CASES=[
 {"name":"nothing behind the newest ones","nowMs":1000000,"keepNewest":4,"minimumAgeMilliseconds":600000,"rows":[row("1",10),row("2",20),row("3",30)],"expected":[]},
 {"name":"the oldest go first, the newest stay","nowMs":1000000,"keepNewest":2,"minimumAgeMilliseconds":600000,"rows":[row("1",10),row("2",20),row("3",30),row("4",40)],"expected":[hx("1"),hx("2")]},
 {"name":"a held bundle stays however old","nowMs":1000000,"keepNewest":1,"minimumAgeMilliseconds":600000,"rows":[row("1",10,True),row("2",20),row("3",30)],"expected":[hx("2")]},
 {"name":"a young bundle behind the newest stays","nowMs":1000000,"keepNewest":1,"minimumAgeMilliseconds":600000,"rows":[row("1",10),row("2",500000),row("3",900000)],"expected":[hx("1")]},
 {"name":"equal birth times order by key","nowMs":1000000,"keepNewest":1,"minimumAgeMilliseconds":0,"rows":[row("b",5),row("a",5),row("c",5)],"expected":[hx("c"),hx("b")]},
 {"name":"keeping none retires every released bundle","nowMs":1000000,"keepNewest":0,"minimumAgeMilliseconds":0,"rows":[row("1",10),row("2",20,True)],"expected":[hx("1")]},
 {"name":"an empty census retires nothing","nowMs":0,"keepNewest":4,"minimumAgeMilliseconds":600000,"rows":[],"expected":[]},
]
def plan_expected(c):
    o=sorted(c["rows"],key=lambda r:(-r["touchedAt"],r["identity"]))[c["keepNewest"]:]
    return list(reversed([r["identity"] for r in o if not r["held"] and c["nowMs"]-r["touchedAt"]>=c["minimumAgeMilliseconds"]]))
for c in CASES:assert plan_expected(c)==c["expected"],c["name"]
TEST='''/** 🔏️ Contract: one compiled authorizer bundle is its own directory beside the dependency generation, holds only while marker, recomputed key, receipt and bytes agree, and retirement removes only released bundles behind the newest ones exactly as an independent Python plan does. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {chmodSync,mkdirSync,mkdtempSync,readdirSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {withResourceLeases} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {compileJsonSchemaSubset} from "../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {nxAuthorizerBundleIdentityV1} from "../../../📦️sdk/🪪️identity/🟦️.ts";
import {newestNxAuthorizerBundleV1,nxAuthorizerBundleDirectoryV1,nxAuthorizerBundleHoldsV1,nxAuthorizerBundlePolicyV1,nxAuthorizerBundleToolkitV1,readNxAuthorizerBundleMarkerV1,touchNxAuthorizerBundleV1} from "../🟦️.ts";
import {planNxAuthorizerBundleRetirementV1,retireNxAuthorizerBundlesV1} from "../🧹️retire/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracle=join(import.meta.dir,"./🐍️oracle/🐍️.py"),hash=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex");
const scratch=mkdtempSync(join(realpathSync(tmpdir()),"nxb-"));

type Parts={dependencies:string;source:string;compiler:string;body:string};
const parts=(dependencies:string,tag=""):Parts=>({dependencies,source:JSON.stringify({version:1,mode:"release",entry:"a"+tag+".ts",resolutions:[],sources:[{path:"a"+tag+".ts",sha256:hash("a"+tag)}]})+"\\n",compiler:JSON.stringify({version:1,mode:"release"})+"\\n",body:"export const authorizers=1"+tag+";\\n"});
function build(tools:string,row:Parts,change:(directory:string,marker:Record<string,unknown>)=>void=()=>{}):string{
 const manifests=new Map([[".semio-authorizer-source.json",Buffer.from(row.source)],[".semio-authorizer-compiler.json",Buffer.from(row.compiler)]]),identity=nxAuthorizerBundleIdentityV1(row.dependencies,manifests),directory=nxAuthorizerBundleDirectoryV1(tools,identity);
 mkdirSync(directory,{recursive:true});
 for(const[name,bytes]of manifests)writeFileSync(join(directory,name),bytes,{mode:0o444});writeFileSync(join(directory,"semio-authorizers.mjs"),row.body,{mode:0o444});
 const marker:Record<string,unknown>={version:1,digest:identity,deps:row.dependencies,authorizers:{version:1,mode:"release",sourceSha256:hash(row.source),entry:"semio-authorizers.mjs",outputs:[{path:"semio-authorizers.mjs",sha256:hash(row.body)}]}};
 change(directory,marker);writeFileSync(join(directory,nxAuthorizerBundlePolicyV1.marker),JSON.stringify(marker),{mode:0o444});
 return directory;
}
const fresh=(label:string)=>{const tools=join(scratch,label);mkdirSync(join(tools,"nx"),{recursive:true});return tools;};

test("the schema and corpus are closed and both validators agree",()=>{
 const ajv=new Ajv({strict:false}),plan=compileJsonSchemaSubset({$ref:"#/$defs/RetirementCorpus"},schema),marker=compileJsonSchemaSubset({$ref:"#/$defs/Marker"},schema);
 ajv.addSchema(schema,"s");expect(ajv.validate({$ref:"s#/$defs/RetirementCorpus"},corpus),JSON.stringify(ajv.errors)).toBe(true);expect(plan.validate(corpus)).toEqual([]);
 const bad={...corpus,cases:[{...corpus.cases[0],rows:[{identity:"x",touchedAt:1,held:false}]}]};expect(ajv.validate({$ref:"s#/$defs/RetirementCorpus"},bad)).toBe(false);expect(plan.validate(bad).length).toBeGreaterThan(0);
 const good={version:1,digest:"a".repeat(64),deps:"b".repeat(64),authorizers:{}};
 for(const value of[good,{...good,extra:1},{...good,deps:"c"},{...good,version:2},{version:1,digest:good.digest,deps:good.deps}])expect(marker.validate(value).length===0,JSON.stringify(value)).toBe(ajv.validate({$ref:"s#/$defs/Marker"},value));
 expect(marker.validate(good)).toEqual([]);expect(marker.validate({...good,extra:1}).length).toBeGreaterThan(0);
});

test("a bundle holds beside its dependency generation and every part of it is bound",()=>{
 const tools=fresh("holds"),dependencies="d".repeat(64),directory=build(tools,parts(dependencies));
 expect(nxAuthorizerBundleHoldsV1(directory,dependencies)?.deps).toBe(dependencies);expect(nxAuthorizerBundleHoldsV1(directory,"e".repeat(64))).toBeNull();
 expect(readNxAuthorizerBundleMarkerV1(directory).digest).toBe(nxAuthorizerBundleIdentityV1(dependencies,new Map([[".semio-authorizer-source.json",Buffer.from(parts(dependencies).source)],[".semio-authorizer-compiler.json",Buffer.from(parts(dependencies).compiler)]])));
 const toolkit=nxAuthorizerBundleToolkitV1(directory);expect(toolkit.identity).toBe(dependencies);expect(toolkit.toolkitRoot).toBe(join(tools,"nx",dependencies));expect(toolkit.bundle).toBe(join(directory,"semio-authorizers.mjs"));
 const faults:[string,(directory:string,marker:Record<string,unknown>)=>void][]=[
  ["marker digest",(_d,marker)=>{marker.digest="f".repeat(64);}],
  ["marker deps",(_d,marker)=>{marker.deps="f".repeat(64);}],
  ["marker extra field",(_d,marker)=>{marker.extra=true;}],
  ["receipt source digest",(_d,marker)=>{(marker.authorizers as {sourceSha256:string}).sourceSha256="0".repeat(64);}],
  ["receipt output digest",(_d,marker)=>{(marker.authorizers as {outputs:{sha256:string}[]}).outputs[0]!.sha256="0".repeat(64);}],
 ];
 for(const[name,change]of faults){const broken=build(fresh("fault-"+name.replaceAll(" ","-")),parts(dependencies),change);expect(nxAuthorizerBundleHoldsV1(broken,dependencies),name).toBeNull();}
 const tampered=build(fresh("tamper"),parts(dependencies));chmodSync(join(tampered,"semio-authorizers.mjs"),0o644);writeFileSync(join(tampered,"semio-authorizers.mjs"),"export const authorizers=2;\\n");expect(nxAuthorizerBundleHoldsV1(tampered,dependencies)).toBeNull();
 const compiler=build(fresh("compiler"),parts(dependencies));chmodSync(join(compiler,".semio-authorizer-compiler.json"),0o644);writeFileSync(join(compiler,".semio-authorizer-compiler.json"),"{}\\n");expect(nxAuthorizerBundleHoldsV1(compiler,dependencies)).toBeNull();
 const linked=build(fresh("linked"),parts(dependencies));rmSync(join(linked,"semio-authorizers.mjs"));writeFileSync(join(scratch,"outside.mjs"),"export const authorizers=1;\\n");symlinkSync(join(scratch,"outside.mjs"),join(linked,"semio-authorizers.mjs"));expect(nxAuthorizerBundleHoldsV1(linked,dependencies)).toBeNull();
 expect(()=>nxAuthorizerBundleToolkitV1(join(scratch,"holds"))).toThrow();
});

test("the newest holding bundle beside a dependency generation is found and others are ignored",async()=>{
 const tools=fresh("newest"),dependencies="d".repeat(64),first=build(tools,parts(dependencies));await Bun.sleep(25);
 const second=build(tools,parts(dependencies,"2")),other=build(tools,parts("e".repeat(64)));
 expect([first,second,other].map(directory=>directory.length).every(length=>length>0)).toBe(true);
 expect(newestNxAuthorizerBundleV1(tools,dependencies)).toBe(second);expect(newestNxAuthorizerBundleV1(tools,"9".repeat(64))).toBeUndefined();expect(newestNxAuthorizerBundleV1(join(scratch,"missing"),dependencies)).toBeUndefined();
});

test("the retirement plan equals the Python plan on every corpus case",()=>{
 const run=spawnSync(python,["-I",oracle],{input:JSON.stringify({cases:corpus.cases}),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);
 const answers=JSON.parse(run.stdout);for(const[index,row]of corpus.cases.entries()){expect(planNxAuthorizerBundleRetirementV1(row.rows,row.nowMs,row.keepNewest,row.minimumAgeMilliseconds),row.name).toEqual(row.expected);expect(answers[index],row.name).toEqual(row.expected);}
});

test("retirement removes only released bundles behind the newest ones and keeps a bundle a reader holds",async()=>{
 const tools=fresh("retire"),leases=join(scratch,"leases"),signal=new AbortController().signal,made:string[]=[];mkdirSync(leases,{recursive:true});
 for(let index=0;index<6;index++){made.push(build(tools,parts("d".repeat(64),String(index))));await Bun.sleep(25);}
 const names=made.map(directory=>directory.split("/").at(-1)!),held=names[1]!;
 const retired=await withResourceLeases({directory:leases,signal,resources:[{resource:nxAuthorizerBundlePolicyV1.lease+held,mode:"shared"}]},()=>retireNxAuthorizerBundlesV1(tools,leases,undefined,{keepNewest:2,minimumAgeMilliseconds:0,signal}));
 const remaining=readdirSync(join(tools,nxAuthorizerBundlePolicyV1.directory)).sort();
 expect([...retired].sort()).toEqual([names[0]!,names[2]!,names[3]!].sort());expect(remaining).toEqual([names[1]!,names[4]!,names[5]!].sort());
 expect(await retireNxAuthorizerBundlesV1(tools,leases,undefined,{keepNewest:2,minimumAgeMilliseconds:0,signal})).toEqual([names[1]!]);
 expect(await retireNxAuthorizerBundlesV1(join(scratch,"absent"),leases,undefined,{signal})).toEqual([]);
 const young=await retireNxAuthorizerBundlesV1(tools,leases,undefined,{keepNewest:0,signal});expect(young).toEqual([]);
});

test("marking a bundle used keeps it ahead of newer bundles",async()=>{
 const tools=fresh("touch"),leases=join(scratch,"leases-touch"),signal=new AbortController().signal,made:string[]=[];mkdirSync(leases,{recursive:true});
 for(let index=0;index<3;index++){made.push(build(tools,parts("d".repeat(64),"t"+index)));await Bun.sleep(25);}
 touchNxAuthorizerBundleV1(made[0]!);
 const names=made.map(directory=>directory.split("/").at(-1)!),retired=await retireNxAuthorizerBundlesV1(tools,leases,undefined,{keepNewest:1,minimumAgeMilliseconds:0,signal});
 expect([...retired].sort()).toEqual([names[1]!,names[2]!].sort());expect(readdirSync(join(tools,nxAuthorizerBundlePolicyV1.directory))).toEqual([names[0]!]);
 expect(()=>touchNxAuthorizerBundleV1(join(scratch,"absent"))).not.toThrow();
});

test("cleanup",()=>{rmSync(scratch,{recursive:true,force:true});});
'''
def plan(p,L):
    b=L+B
    p.create(b+"🧬️schema/🔣️.json",json.dumps(SCHEMA,indent=2,ensure_ascii=False)+"\n")
    p.create(b+"🟦️.ts",MODULE)
    p.create(b+"🧹️retire/🟦️.ts",RETIRE)
    p.create(b+"🧪️tests/🐍️oracle/🐍️.py",PY)
    p.create(b+"🧪️tests/🔣️.json",json.dumps({"version":1,"cases":CASES},indent=1,ensure_ascii=False)+"\n")
    p.create(b+"🧪️tests/🟦️.ts",TEST)
