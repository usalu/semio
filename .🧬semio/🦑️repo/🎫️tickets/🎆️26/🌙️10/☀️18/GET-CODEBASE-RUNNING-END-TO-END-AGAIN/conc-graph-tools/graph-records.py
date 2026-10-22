import os, sys, json, re
ROOT = "/Users/ueli/Documents/semio"
if "--root" in sys.argv: ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1])
HERE = os.path.dirname(os.path.abspath(__file__))
def resolve(rel, create=False):
    cur = ROOT
    for part in rel.split("/"):
        hits = [n for n in os.listdir(cur) if n.replace("️", "") == part.replace("️", "")] if os.path.isdir(cur) else []
        if not hits and create: cur = os.path.join(cur, part); continue
        assert len(hits) == 1, (part, hits); cur = os.path.join(cur, hits[0])
    return cur
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
BG = LIB + "⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/"
BUDGET = resolve(BG + "🟦️.ts"); POLICY = resolve(BG + "🔣️policy.json"); SCHEMA = resolve(BG + "🧬️schema/🔣️.json"); BTEST = resolve(BG + "🧪️tests/🟦️.ts")
G = resolve(LIB + "🏗️build/🕸️graph/🟦️.ts")
RDIR = os.path.join(os.path.dirname(BUDGET), "📇️records")
files = {}
def get(p):
    if p not in files: files[p] = [open(p, encoding="utf8", newline="").read() if os.path.exists(p) else None, False]
    return files[p]
def sub(p, marker, old, new, label, count=1):
    row = get(p)
    if marker in row[0]: print("applied ", label); return
    assert row[0].count(old) == count, (label, row[0].count(old), old[:90]); row[0] = row[0].replace(old, new); row[1] = True; print("pending ", label)
def put(p, text, label):
    row = get(p)
    if row[0] == text: print("applied ", label); return
    row[0] = text; row[1] = True; print("pending ", label)

# ---- records module, declarations, policy, schema
put(os.path.join(RDIR, "🟨️.mjs"), open(os.path.join(HERE, "codec.mjs"), encoding="utf8").read(), "codec")
put(os.path.join(RDIR, "🟨️.d.mts"), '''/** 🧾️ The finite limits of the graph record format. */
export interface GraphRecordPolicyV1{readonly version:2;readonly projectsPerYield:number;readonly edgesPerRecord:number;readonly externalNodesPerRecord:number;readonly maximumRecordBytes:number;readonly compressionLevel:number;readonly internMinimumItems:number}
/** 📇️ A pinned graph as content-addressed gzip records and a small manifest. */
export interface EncodedGraphRecordsV1{readonly manifestBytes:Buffer;readonly digest:string;readonly records:ReadonlyMap<string,Buffer>;readonly recordBytes:number;readonly bytes:number}
/** 📇️ Encodes a build graph without ever holding it as one string; refuses beyond `maximumBytes` compressed bytes. */
export function encodeGraphRecordsV1(graph:unknown,options:{policy:GraphRecordPolicyV1;maximumBytes:number}):Promise<EncodedGraphRecordsV1>;
/** 🧷️ Collapses long identical string arrays to one retained array. */
export function createArrayInternerV1(minimumItems:number):<T>(value:T)=>T;
/** 📖️ Decodes a pinned graph from its manifest and a record reader. */
export function decodeGraphRecordsV1(manifestBytes:Uint8Array,readRecord:(digest:string)=>Promise<Uint8Array|undefined>,options:{policy:GraphRecordPolicyV1}):Promise<any>;
/** 🔗️ Names the record digests one manifest holds. */
export function graphRecordDigestsV1(manifestBytes:Uint8Array):Set<string>;
''', "codec d.mts")
row = get(POLICY); data = json.loads(row[0])
if "records" not in data:
    data["records"] = {"version": 2, "projectsPerYield": 16, "edgesPerRecord": 4096, "externalNodesPerRecord": 512, "maximumRecordBytes": 268435456, "compressionLevel": 4, "internMinimumItems": 16}
    row[0] = json.dumps(data, ensure_ascii=False) + ("\n" if row[0].endswith("\n") else ""); row[1] = True; print("pending  policy")
else: print("applied  policy")
row = get(SCHEMA); data = json.loads(row[0])
if "records" not in data["properties"]:
    data["required"].append("records")
    data["properties"]["records"] = {"type": "object", "additionalProperties": False, "required": ["version", "projectsPerYield", "edgesPerRecord", "externalNodesPerRecord", "maximumRecordBytes", "compressionLevel", "internMinimumItems"],
        "properties": {"version": {"const": 2}, "projectsPerYield": {"type": "integer", "minimum": 1, "maximum": 1024}, "edgesPerRecord": {"type": "integer", "minimum": 1, "maximum": 1048576}, "externalNodesPerRecord": {"type": "integer", "minimum": 1, "maximum": 1048576}, "maximumRecordBytes": {"type": "integer", "minimum": 1, "maximum": 1073741824}, "compressionLevel": {"type": "integer", "minimum": 1, "maximum": 9}, "internMinimumItems": {"type": "integer", "minimum": 2, "maximum": 65536}}}
    row[0] = json.dumps(data, ensure_ascii=False, indent=2) + "\n"; row[1] = True; print("pending  schema")
else: print("applied  schema")

# ---- writer
sub(BUDGET, "encodeGraphRecordsV1,graphRecordDigestsV1}", 'import policy from "./🔣️policy.json" with {type:"json"};', 'import {encodeGraphRecordsV1,graphRecordDigestsV1} from "./📇️records/🟨️.mjs";\nimport policy from "./🔣️policy.json" with {type:"json"};', "budget import")
# retention: manifests, records and orphans
OLD_RET = BUDGET and re.search(r"async function retainSnapshotBudgetV1\(.*?\n\}\n", get(BUDGET)[0], re.S)
NEW_RET = r'''async function retainSnapshotBudgetV1(store:RepositoryCacheStoreV1,directory:string,desired:string,additionalBytes:number,leaseDirectory:string,control:ResourceLeaseControl,keep:ReadonlySet<string>):Promise<void>{
 checkResourceLeaseControl(control);const names=await readdir(directory);checkResourceLeaseControl(control);const rows:{path:string;bytes:number;stamp:number;records:ReadonlySet<string>}[]=[],records=new Map<string,{path:string;bytes:number}>();
 for(const name of names){checkResourceLeaseControl(control);if(!/^[a-f0-9]{64}(?:\.json|\.gz|~[a-f0-9]{8})$/u.test(name))continue;const path=requireNativeStoragePathV1(store.root,join(directory,name)),status=await lstat(path);checkResourceLeaseControl(control);if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1||status.size>policy.maximumSnapshotBytes)throw Error("Original bounded graph snapshot inventory required");if(name.includes("~")){await store.recheck();checkResourceLeaseControl(control);await rm(path);checkResourceLeaseControl(control);continue;}if(name.endsWith(".gz")){records.set(name.slice(0,64),{path,bytes:status.size});continue;}rows.push({path,bytes:status.size,stamp:status.mtimeMs,records:await manifestRecordsV1(path,status.size)});}
 const references=new Map<string,number>(),reference=(digest:string,step:number)=>references.set(digest,(references.get(digest)??0)+step);for(const row of rows)for(const digest of row.records)reference(digest,1);for(const digest of keep)reference(digest,1);
 const live=()=>{let total=rows.reduce((sum,row)=>sum+row.bytes,0);for(const[digest,row]of records)if((references.get(digest)??0)>0)total+=row.bytes;return total;};
 const sweep=async()=>{for(const[digest,row]of records){if((references.get(digest)??0)>0)continue;checkResourceLeaseControl(control);await store.recheck();checkResourceLeaseControl(control);await rm(row.path,{force:true});records.delete(digest);}};
 await sweep();let bytes=live()+additionalBytes;
 for(const row of [...rows].sort((left,right)=>left.stamp-right.stamp||left.path.localeCompare(right.path))){checkResourceLeaseControl(control);if(bytes<=policy.maximumRetainedBytes)break;if(row.path===desired)continue;let lease:ResourceLease|undefined;try{lease=await acquireResourceLease({directory:leaseDirectory,resource:resource(row.path),mode:"exclusive",signal:control.signal,control,timeoutMs:0});}catch(error){checkResourceLeaseControl(control);if(error instanceof Error&&error.message==="Resource lease timed out: "+resource(row.path))continue;throw error;}let primary:{error:unknown}|undefined;try{checkResourceLeaseControl(control);await store.recheck();checkResourceLeaseControl(control);await inspectSnapshotV1(store,row.path,row.path.split("/").at(-1)!.slice(0,64),control);checkResourceLeaseControl(control);await rm(row.path);checkResourceLeaseControl(control);rows.splice(rows.indexOf(row),1);for(const digest of row.records)reference(digest,-1);await sweep();bytes=live()+additionalBytes;}catch(error){primary={error};throw error;}finally{try{lease.release();}catch(error){if(!primary)throw error;if(primary.error instanceof Error&&Object.isExtensible(primary.error))try{Object.defineProperty(primary.error,"graphArtifactCleanupErrors",{value:[error],configurable:true});}catch{}}}}
 if(bytes>policy.maximumRetainedBytes)throw Error("Pinned build graphs exhaust the finite snapshot budget");
}

/** 🔗️ Names the record digests of one retained snapshot: a record manifest names its records, a whole-graph snapshot of an earlier format names none. */
async function manifestRecordsV1(path:string,size:number):Promise<ReadonlySet<string>>{
 if(size>16777216)return new Set();const bytes=await readFile(path);if(!bytes.subarray(0,13).equals(Buffer.from("{\"version\":2,")))return new Set();
 try{return graphRecordDigestsV1(bytes);}catch{return new Set();}
}
'''
row = get(BUDGET)
if "async function manifestRecordsV1" not in row[0]:
    m = re.search(r"async function retainSnapshotBudgetV1\(.*?\n\}\n", row[0], re.S); assert m
    row[0] = row[0][:m.start()] + NEW_RET + row[0][m.end():]; row[1] = True; print("pending  retention")
else: print("applied  retention")
# artifact function body
sub(BUDGET, "const encoded=await encodeGraphRecordsV1(", 'const source=JSON.stringify(readBuildGraphV1(graph)),bytes=Buffer.byteLength(source);if(bytes>policy.maximumSnapshotBytes)throw Error("Build graph exceeds finite snapshot byte limit");const digest=hash(source),leaseDirectory=join(workspace,".cache/b/l/m/data");',
 'const encoded=await encodeGraphRecordsV1(readBuildGraphV1(graph),{policy:policy.records as never,maximumBytes:policy.maximumSnapshotBytes}),digest=encoded.digest,leaseDirectory=join(workspace,".cache/b/l/m/data");', "artifact encode")
row = get(BUDGET)
if "const present=new Set(" not in row[0]:
    m = re.search(r"const existing=await inspectSnapshotV1\(store,path,digest,actualControl\);check\(\);const additionalBytes=existing\?0:bytes;.*?check\(\);await inspectSnapshotV1\(store,path,digest,actualControl\);check\(\);\}reader=", row[0], re.S); assert m
    NEW = ('await mkdir(directory,{recursive:true});check();const existing=await inspectSnapshotV1(store,path,digest,actualControl);check();const present=new Set(await readdir(directory)),missing=[...encoded.records].filter(([name])=>!present.has(name+".gz")),additionalBytes=(existing?0:encoded.manifestBytes.length)+missing.reduce((sum,[,bytes])=>sum+bytes.length,0);if(prepare&&fundedBytes!==additionalBytes)return{ready:false as const,additionalBytes};await retainSnapshotBudgetV1(store,directory,path,additionalBytes,leaseDirectory,actualControl,new Set(encoded.records.keys()));check();'
     'const publish=async(target:string,bytes:Uint8Array)=>{const temporary=requireNativeStoragePathV1(store.root,join(directory,target.split("/").at(-1)!.slice(0,64)+"~"+randomUUID().slice(0,8))),file=await open(temporary,"wx");let filePrimary:{error:unknown}|undefined;try{check();await file.writeFile(bytes);check();await file.sync();check();await store.recheck();check();await rename(temporary,target);check();}catch(error){filePrimary={error};throw error;}finally{await cleanup(filePrimary,[()=>file.close(),()=>rm(temporary,{force:true})]);}};'
     'for(const[name,bytes]of missing)await publish(requireNativeStoragePathV1(store.root,join(directory,name+".gz")),bytes);if(!existing)await publish(path,encoded.manifestBytes);'
     'if(missing.length||!existing){const parent=await open(directory,"r");let parentPrimary:{error:unknown}|undefined;try{check();await parent.sync();check();}catch(error){parentPrimary={error};throw error;}finally{await cleanup(parentPrimary,[()=>parent.close()]);}check();await inspectSnapshotV1(store,path,digest,actualControl);check();}reader=')
    row[0] = row[0][:m.start()] + NEW + row[0][m.end():]; row[1] = True; print("pending  artifact writer")
else: print("applied  artifact writer")

# ---- reader (every consumer of the pinned graph goes through this function)
sub(G, "decodeGraphRecordsV1}from", 'import{createGenerationStoreV1,', 'import{decodeGraphRecordsV1}from"../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📇️records/🟨️.mjs";\nimport{createGenerationStoreV1,', "G records import")
sub(G, "decodeGraphRecordsV1(Buffer.from(source)", 'const graph=readBuildGraphV1(await portableInputs(root,JSON.parse(source),store.invocation,actual));requireCurrentGraphModuleV1();checkResourceLeaseControl(actual);const value=await body(graph,pin.digest!,actual);',
 'let reads=0;const records=dirname(expected),graph=await decodeGraphRecordsV1(Buffer.from(source),async digest=>{if(++reads%64===0)await store.recheck();checkResourceLeaseControl(actual);const path=requireNativeStoragePathV1(store.root,join(records,digest+".gz")),status=await lstat(path);if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1||status.size>graphSnapshotPolicyV1.maximumSnapshotBytes)throw Error("Original bounded Graph record required");return readFile(path);},{policy:graphSnapshotPolicyV1.records as never}) as BuildGraphV1;requireCurrentGraphModuleV1();checkResourceLeaseControl(actual);const value=await body(graph,pin.digest!,actual);', "G reader")

# ---- tests, corpus and oracle
TESTS = os.path.join(RDIR, "🧪️tests")
CORPUS = {"version": 1, "policy": {"version": 2, "projectsPerYield": 2, "edgesPerRecord": 3, "externalNodesPerRecord": 2, "maximumRecordBytes": 1048576, "compressionLevel": 4, "internMinimumItems": 3},
 "graph": {"version": 1, "namedInputs": {"default": ["{projectRoot}/**/*"]},
  "projects": {"alpha": {"name": "alpha", "root": "pkg/alpha", "targets": {"build": {"inputs": ["{workspaceRoot}/a", "{workspaceRoot}/b", "{workspaceRoot}/c"]}}}, "beta": {"name": "beta", "root": "pkg/beta", "targets": {"build": {"inputs": ["{workspaceRoot}/a", "{workspaceRoot}/b", "{workspaceRoot}/c"], "dependsOn": ["^build"]}}}, "gamma": {"name": "gamma", "root": "pkg/gamma", "targets": {}, "tags": ["ünï😀"]}, "delta": {"name": "delta", "root": "pkg/delta", "targets": {}}, "twin": {"name": "twin", "root": "pkg/delta", "targets": {}}},
  "dependencies": [{"source": "beta", "target": "alpha", "type": "static"}, {"source": "gamma", "target": "alpha"}, {"source": "delta", "target": "beta"}, {"source": "twin", "target": "npm:left-pad"}, {"source": "gamma", "target": "beta"}, {"source": "delta", "target": "alpha"}, {"source": "alpha", "target": "npm:left-pad"}],
  "externalNodes": {"npm:left-pad": {"name": "left-pad", "type": "npm"}, "npm:other": {"name": "other", "type": "npm"}, "npm:third": {"name": "third", "type": "npm"}},
  "unavailable": [{"root": "pkg/z", "source": "pkg/z/project.json", "sha256": "0" * 64, "reason": "bad"}],
  "admission": {"selection": {"kind": "all"}, "registry": "1" * 64, "closure": "2" * 64}},
 "expected": {"projectRecords": 5, "dependencyRecords": 3, "externalRecords": 2, "recordsTotal": 10}}
put(os.path.join(TESTS, "🔣️.json"), json.dumps(CORPUS, ensure_ascii=False, indent=1) + "\n", "corpus")
ORACLE = '''#!/usr/bin/env python3
"""🐍️ Independent reader of the graph record format: rebuilds a pinned graph from its manifest and gzip records with plain Python and prints it as canonical JSON.

usage: oracle.py DIRECTORY MANIFEST_DIGEST"""
import gzip, hashlib, json, os, sys

directory, digest = sys.argv[1], sys.argv[2]
raw = open(os.path.join(directory, digest + ".json"), "rb").read()
assert hashlib.sha256(raw).hexdigest() == digest, "manifest digest"
manifest = json.loads(raw)


def record(name):
    packed = open(os.path.join(directory, name + ".gz"), "rb").read()
    assert hashlib.sha256(packed).hexdigest() == name, "record digest " + name
    return json.loads(gzip.decompress(packed))


graph = dict(manifest["graph"])
graph["projects"] = {row[0]: record(row[1]) for row in manifest["projects"]}
graph["dependencies"] = [edge for name in manifest["dependencies"] for edge in record(name)]
if "externalNodes" in manifest:
    nodes = {}
    for name in manifest["externalNodes"]:
        nodes.update(record(name))
    graph["externalNodes"] = nodes
print(json.dumps(graph, sort_keys=True, separators=(",", ":"), ensure_ascii=False))
'''
put(os.path.join(TESTS, "🐍️oracle.py"), ORACLE, "oracle")
HEAP = r'''import{encodeGraphRecordsV1,decodeGraphRecordsV1}from"../../🟨️.mjs";
import policy from"../../../🔣️policy.json"with{type:"json"};

const projectsCount=Number(process.argv[2]??5000),pathsPerArray=Number(process.argv[3]??4000);
const shared=[0,1,2].map(slot=>Array.from({length:pathsPerArray},(_,index)=>"{workspaceRoot}/framework/modules/m"+slot+"/path/to/source-"+index+".ts"));
const projects={};for(let index=0;index<projectsCount;index++)projects["p"+index]={name:"p"+index,root:"pkg/p"+index,targets:{build:{inputs:shared[index%3],outputs:["{projectRoot}/dist"],metadata:{index,note:"project "+index}}}};
const dependencies=Array.from({length:projectsCount*3},(_,index)=>({source:"p"+(index%projectsCount),target:"p"+((index*7+1)%projectsCount),type:"static"}));
const graph={version:1,projects,dependencies,admission:{selection:{kind:"all"},registry:"1".repeat(64),closure:"2".repeat(64)}};
let peak=0;const sample=()=>{peak=Math.max(peak,process.memoryUsage().rss);},timer=setInterval(sample,20);sample();
let raw=0;for(const name of Object.keys(projects))raw+=Buffer.byteLength(JSON.stringify(projects[name]));
const baseline=peak;
const encoded=await encodeGraphRecordsV1(graph,{policy:policy.records,maximumBytes:policy.maximumSnapshotBytes});
const encodePeak=peak;
const read=async digest=>encoded.records.get(digest);
const decoded=await decodeGraphRecordsV1(encoded.manifestBytes,read,{policy:policy.records});
const decodePeak=peak;sample();clearInterval(timer);
const sameShare=decoded.projects.p0.targets.build.inputs===decoded.projects.p3.targets.build.inputs;
console.log(JSON.stringify({projects:Object.keys(decoded.projects).length,raw,compressed:encoded.bytes,records:encoded.records.size,baseline,encodePeak,decodePeak,equal:JSON.stringify(decoded.projects.p4999??decoded.projects["p"+(projectsCount-1)])===JSON.stringify(projects["p"+(projectsCount-1)]),edges:decoded.dependencies.length===dependencies.length,sameShare,limit:policy.maximumSnapshotBytes}));
'''
put(os.path.join(TESTS, "🧱️heap/🟨️.mjs"), HEAP, "heap child")
TEST = r'''/** 📇️ Contract: a pinned build graph is never one string. It is a small manifest and content-addressed gzip records (one per project, one per chunk of dependencies and of external nodes); every compressed byte is counted as it is produced, a record is verified against its name before it is parsed, identical long arrays are shared again on decode, and a graph whose serialized text would exceed the old whole-string limit round-trips with bounded memory. */
import {expect,test} from "bun:test";
import {mkdtempSync,mkdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv2020 from "ajv/dist/2020";
import {testPythonV1} from "../../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import corpus from "./🔣️.json";
import policy from "../../🔣️policy.json";
import schema from "../../🧬️schema/🔣️.json";
import {decodeGraphRecordsV1,encodeGraphRecordsV1,graphRecordDigestsV1} from "../🟨️.mjs";

const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),canonical=(value:unknown):string=>JSON.stringify(value,(_key,item)=>item&&typeof item==="object"&&!Array.isArray(item)?Object.fromEntries(Object.entries(item).sort(([a],[b])=>a<b?-1:a>b?1:0)):item);

test("the declared record limits are admitted by the policy schema",()=>{expect(new Ajv2020({strict:true}).validate(schema,policy)).toBe(true);});

test("a graph is encoded as one record per project, per chunk of dependencies and of external nodes, and decodes to the same graph",async()=>{
 const encoded=await encodeGraphRecordsV1(corpus.graph,{policy:corpus.policy as never,maximumBytes:policy.maximumSnapshotBytes});
 const manifest=JSON.parse(encoded.manifestBytes.toString("utf8"));
 expect(manifest.projects.length).toBe(corpus.expected.projectRecords);expect(manifest.dependencies.length).toBe(corpus.expected.dependencyRecords);expect(manifest.externalNodes.length).toBe(corpus.expected.externalRecords);expect(encoded.records.size).toBeLessThanOrEqual(corpus.expected.recordsTotal);
 expect(manifest.projects.find((row:string[])=>row[0]==="delta")![1]).not.toBe(manifest.projects.find((row:string[])=>row[0]==="twin")![1]);
 expect(graphRecordDigestsV1(encoded.manifestBytes).size).toBe(encoded.records.size);
 const decoded=await decodeGraphRecordsV1(encoded.manifestBytes,async digest=>encoded.records.get(digest),{policy:corpus.policy as never});
 expect(canonical(decoded)).toBe(canonical(corpus.graph));
 expect(decoded.projects.alpha.targets.build.inputs).toBe(decoded.projects.beta.targets.build.inputs);
});

test("a record that differs from its name, a missing record and an open manifest are refused; a graph beyond the byte limit is refused while it is encoded",async()=>{
 const encoded=await encodeGraphRecordsV1(corpus.graph,{policy:corpus.policy as never,maximumBytes:policy.maximumSnapshotBytes}),first=[...encoded.records.keys()][0]!;
 const read=(replace:(digest:string)=>Buffer|undefined)=>decodeGraphRecordsV1(encoded.manifestBytes,async digest=>replace(digest),{policy:corpus.policy as never});
 await expect(read(digest=>digest===first?Buffer.from("tampered"):encoded.records.get(digest))).rejects.toThrow("Graph record content differs from its name");
 await expect(read(digest=>digest===first?undefined:encoded.records.get(digest))).rejects.toThrow("Graph record content differs from its name");
 await expect(decodeGraphRecordsV1(Buffer.from(JSON.stringify({...JSON.parse(encoded.manifestBytes.toString()),extra:1})),async digest=>encoded.records.get(digest),{policy:corpus.policy as never})).rejects.toThrow("Closed graph record manifest required");
 await expect(encodeGraphRecordsV1(corpus.graph,{policy:corpus.policy as never,maximumBytes:200})).rejects.toThrow("Build graph exceeds finite snapshot byte limit");
 await expect(encodeGraphRecordsV1(corpus.graph,{policy:{...corpus.policy,maximumRecordBytes:20} as never,maximumBytes:policy.maximumSnapshotBytes})).rejects.toThrow("Graph record exceeds its finite size");
});

test("the independent python reader rebuilds the same graph from the written records",async()=>{
 if(!python)return;
 const dir=mkdtempSync(join(realpathSync(tmpdir()),"graph-records-"));
 try{
  const encoded=await encodeGraphRecordsV1(corpus.graph,{policy:corpus.policy as never,maximumBytes:policy.maximumSnapshotBytes});
  writeFileSync(join(dir,encoded.digest+".json"),encoded.manifestBytes);for(const[digest,bytes]of encoded.records)writeFileSync(join(dir,digest+".gz"),bytes);
  const result=Bun.spawnSync([python,"-I",join(here,"🐍️oracle.py"),dir,encoded.digest]);expect(result.exitCode).toBe(0);
  expect(canonical(JSON.parse(result.stdout.toString()))).toBe(canonical(corpus.graph));
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("a 5,000 project graph whose text exceeds the old whole-string limit encodes and decodes with bounded memory",async()=>{
 const result=Bun.spawnSync([process.execPath,join(here,"🧱️heap","🟨️.mjs"),"5000","4000"],{stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 const row=JSON.parse(result.stdout.toString().trim().split("\n").at(-1)!);
 expect(row.projects).toBe(5000);expect(row.raw).toBeGreaterThan(policy.maximumSnapshotBytes);expect(row.compressed).toBeLessThan(row.raw/8);expect(row.compressed).toBeLessThan(policy.maximumSnapshotBytes);
 expect(row.equal).toBe(true);expect(row.edges).toBe(true);expect(row.sameShare).toBe(true);
 expect(row.encodePeak-row.baseline).toBeLessThan(768*1048576);expect(row.decodePeak-row.baseline).toBeLessThan(1024*1048576);
},300000);
'''
put(os.path.join(TESTS, "🟦️.ts"), TEST, "records tests")
# the artifact fixture test reads the manifest now
sub(BTEST, "expect(manifest.version).toBe(2)", '   const bytes=await readFile(artifact.path),oracle=database.query("SELECT length(cast(? AS blob)) AS bytes").get(source) as {bytes:number};\n   expect(bytes.toString()).toBe(source);expect(bytes.byteLength).toBe(oracle.bytes);expect(oracle.bytes).toBe(fixture.expectedBytes);expect(artifact.digest).toBe(Buffer.from(await crypto.subtle.digest("SHA-256",bytes)).toString("hex"));expect(createHash("sha256").update(source).digest("hex")).toBe(artifact.digest);expect(artifact.path).toContain("/.cache/b/g/m/");',
 '   const bytes=await readFile(artifact.path),manifest=JSON.parse(bytes.toString());\n   expect(manifest.version).toBe(2);expect(artifact.digest).toBe(Buffer.from(await crypto.subtle.digest("SHA-256",bytes)).toString("hex"));expect(artifact.path).toContain("/.cache/b/g/m/");expect(Object.keys(manifest.graph).sort()).toEqual(Object.keys(fixture.graph).filter(key=>!["projects","dependencies","externalNodes"].includes(key)).sort());for(const[,digest]of manifest.projects)expect((await readFile(join(artifact.path.replace(/[^/]+$/u,""),digest+".gz"))).byteLength).toBeGreaterThan(0);', "artifact test") if os.path.exists(BTEST) else None

if "--write" in sys.argv:
    for p, (t, ch) in files.items():
        if ch:
            os.makedirs(os.path.dirname(p), exist_ok=True)
            open(p, "w", encoding="utf8", newline="").write(t)
    print("WRITTEN")
