#!/usr/bin/env python3
"""R77 stage builder: reads the live files, applies counted literal edits, writes <id>.base/<id>.new/index.txt into ./stage (new files have kind "new" and no .base). Re-run it to restage from the current live tree."""
import os,shutil,sys
R="/Users/ueli/Documents/semio/"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
CL=L+"🏗️build/📥️source/🔏️closure/"
B=L+"🏗️build/"
AU=L+"⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/"
H=os.path.dirname(os.path.abspath(__file__))+"/stage/"
shutil.rmtree(H,ignore_errors=True);os.makedirs(H)
rows=[];n=[0]
def stage(rel,edits=None,new=None):
    i="f%d"%n[0];n[0]+=1
    if new is not None:
        open(H+i+".new","w",encoding="utf8").write(new);rows.append("%s %s new"%(i,rel));return
    text=open(R+rel,encoding="utf8").read();base=text
    for old,rep,count in edits:
        got=text.count(old)
        if got!=count:sys.exit("anchor count %d != %d in %s: %r"%(got,count,rel,old[:80]))
        text=text.replace(old,rep)
    open(H+i+".base","w",encoding="utf8").write(base);open(H+i+".new","w",encoding="utf8").write(text);rows.append("%s %s mod"%(i,rel))

stage(CL+"🧬️schema/🔣️.json",[
 ('"maxItems": 2048,','"maxItems": 16384,',3),
 ('"maximumFiles": 2048,','"maximumFiles": 16384,',1),
 ('"maximumSourceBytes": 20971520,','"maximumSourceBytes": 67108864,',1),
 ('"maximumSnapshotBytes": 33554432,','"maximumSnapshotBytes": 100663296,',2),
 ('"maximum": 33554433','"maximum": 100663297',1),
])
stage(AU+"🧬️schema/🔣️.json",[
 ('      "maxItems": 2048\n','      "maxItems": 16384\n',1),
 ('"SourceCensusPolicy": { "const": { "maximumFiles": 8192, "maximumSourceBytes": 67108864, "progressFiles": 16 } },',
  '"SourceCensusPolicy": { "const": { "maximumFiles": 65536, "maximumSourceBytes": 268435456, "progressFiles": 16 } },\n'
  '    "SourceCapacityTexts": {\n      "const": {\n'
  '        "en": "Authorizer source closure capacity exceeded: files={files}/{maximumFiles}, bytes={bytes}/{maximumBytes}. The closure reached {path} through {chain}. Drop the import that pulls this subtree into the entry, or raise maximumFiles, maximumSourceBytes and maximumSnapshotBytes together in the Policy of build/source/closure/schema (one file per 4096 source bytes, snapshot 1.5 times the sources).",\n'
  '        "de": "Kapazität des Autorisierer-Quellabschlusses überschritten: Dateien={files}/{maximumFiles}, Bytes={bytes}/{maximumBytes}. Der Abschluss erreichte {path} über {chain}. Entferne den Import, der diesen Teilbaum in den Einstieg zieht, oder erhöhe maximumFiles, maximumSourceBytes und maximumSnapshotBytes gemeinsam in der Policy von build/source/closure/schema (eine Datei je 4096 Quellbytes, Snapshot das 1,5-fache der Quellen)."\n'
  '      }\n    },',1),
 ('"files": { "type": "integer", "minimum": 1, "maximum": 8192 },','"files": { "type": "integer", "minimum": 1, "maximum": 65536 },',1),
 ('"sourceBytes": { "type": "integer", "minimum": 0, "maximum": 67108864 },','"sourceBytes": { "type": "integer", "minimum": 0, "maximum": 268435456 },',1),
 ('"sources": { "type": "array", "maxItems": 8192,','"sources": { "type": "array", "maxItems": 65536,',1),
 ('"maximum": 2049\n','"maximum": 16385\n',1),
 ('"maximum": 25165824\n','"maximum": 71303168\n',1),
 ('"maximumDirectories": 1024,','"maximumDirectories": 8192,',1),
 ('"maximumEntries": 1024,','"maximumEntries": 8192,',1),
 ('"maximumResults": 1024,','"maximumResults": 8192,',1),
 ('"maximumDeclaredBytes": 16777216','"maximumDeclaredBytes": 67108864',1),
 ('"maximumFileBytes": 4194304,\n        "concurrency": 24','"maximumFileBytes": 33554432,\n        "concurrency": 24',1),
 ('"maximumFiles": 2400,','"maximumFiles": 12288,',1),
 ('"maximumSourceBytes": 18874368,','"maximumSourceBytes": 50331648,',1),
 ('"maximumSnapshotBytes": 27262976,','"maximumSnapshotBytes": 75497472,',1),
])
stage(AU+"📦️snapshot/🧬️schema/🔣️.json",[('"maxItems": 2048,','"maxItems": 16384,',1)])
stage(AU+"📦️snapshot/🔣️policy.json",[
 ('"maximumSourceBytes": 20971520,','"maximumSourceBytes": 67108864,',1),
 ('"maximumSnapshotBytes": 33554432,','"maximumSnapshotBytes": 100663296,',1),
 ('"maximumOutputBytes": 8388608','"maximumOutputBytes": 33554432',1),
])
CAP=('/** 🗣️ Words one refused source closure in both languages: the bound that stopped it, the import chain that reached the file and what to change; the English sentence keeps the wording the logs are searched for. */\n'
 'export function nxAuthorizerCapacityTextV1(row:Readonly<{files:number;bytes:number;path:string;chain:readonly string[]}>):string{const policy=closureSchema.definitions.Policy.const,texts=authorizerSchema.$defs.SourceCapacityTexts.const,fields:Record<string,string|number>={files:row.files,maximumFiles:policy.maximumFiles,bytes:row.bytes,maximumBytes:policy.maximumSourceBytes,path:row.path,chain:row.chain.join(" <- ")},fill=(text:string)=>text.replace(/\\{(\\w+)\\}/gu,(_all,name:string)=>String(fields[name]));return fill(texts.en)+" | "+fill(texts.de);}\n')
stage(AU+"🟦️.ts",[
 ('/** 📏️ Measures a bounded source closure without issuing capture authority. */',CAP+'/** 📏️ Measures a bounded source closure without issuing capture authority. */',1),
 ('Error(`Authorizer source closure capacity exceeded: files=${files.size}/${maximumSourceFiles}, bytes=${total}/${closureSchema.definitions.Policy.const.maximumSourceBytes}`)','Error(nxAuthorizerCapacityTextV1({files:files.size,bytes:total,path,chain}))',1),
])
stage(L+"🏗️build/📖️texts/🔣️.json",[
 ('"en": "Implementation Source closure capacity exceeded: {path}",','"en": "Implementation Source closure capacity exceeded: {path}. Drop the import that pulls this subtree into the entry, or raise maximumFiles, maximumSourceBytes and maximumSnapshotBytes together in the Policy of build/source/closure/schema.",',1),
 ('"de": "Die Kapazität des Implementierungs-Quellabschlusses ist überschritten: {path}"','"de": "Die Kapazität des Implementierungs-Quellabschlusses ist überschritten: {path}. Entferne den Import, der diesen Teilbaum in den Einstieg zieht, oder erhöhe maximumFiles, maximumSourceBytes und maximumSnapshotBytes gemeinsam in der Policy von build/source/closure/schema."',1),
])
stage(L+"🏗️build/🏛️sealed/🔣️.json",[
 ('"en": "[implementation] The admitted compiler closure exceeds its bound.",','"en": "[implementation] The admitted compiler closure exceeds its bound. Drop the import that pulls the largest subtree into the entry, or raise the Policy of build/source/closure/schema (files, source bytes and snapshot bytes together).",',1),
 ('"de": "[implementation] Der zugelassene Compiler-Abschluss überschreitet seine Grenze."','"de": "[implementation] Der zugelassene Compiler-Abschluss überschreitet seine Grenze. Entferne den Import, der den größten Teilbaum in den Einstieg zieht, oder erhöhe die Policy von build/source/closure/schema (Dateien, Quellbytes und Snapshot-Bytes gemeinsam)."',1),
])
T=AU+"🧪️tests/"
stage(T+"📏️capacity/🔣️.json",[('"parts": 12,','"parts": 17,',1),('"partBytes": 1800000,','"partBytes": 4000000,',1),('"expectedFiles": 13,','"expectedFiles": 18,',1)])
stage(T+"📏️capacity/🧫️bounds.json",[
 ('"id": "source-before", "files": 1, "sourceBytes": 67108863,','"id": "source-before", "files": 1, "sourceBytes": 268435455,',1),
 ('"id": "source-at", "files": 1, "sourceBytes": 67108864,','"id": "source-at", "files": 1, "sourceBytes": 268435456,',1),
 ('"id": "source-over", "files": 1, "sourceBytes": 67108865,','"id": "source-over", "files": 1, "sourceBytes": 268435457,',1),
 ('"files": 8191,','"files": 65535,',1),('"files": 8192,','"files": 65536,',1),('"files": 8193,','"files": 65537,',1),
 ('"files": 2047, "sourceBytes": 0, "snapshotBytes": 0','"files": 16383, "sourceBytes": 0, "snapshotBytes": 0',1),
 ('"files": 2048, "sourceBytes": 0, "snapshotBytes": 0','"files": 16384, "sourceBytes": 0, "snapshotBytes": 0',1),
 ('"files": 2049, "sourceBytes": 0, "snapshotBytes": 0','"files": 16385, "sourceBytes": 0, "snapshotBytes": 0',1),
 ('"sourceBytes": 20971519,','"sourceBytes": 67108863,',1),('"sourceBytes": 20971520,','"sourceBytes": 67108864,',1),('"sourceBytes": 20971521,','"sourceBytes": 67108865,',1),
 ('"snapshotBytes": 33554431,','"snapshotBytes": 100663295,',1),('"snapshotBytes": 33554432,','"snapshotBytes": 100663296,',1),('"snapshotBytes": 33554433,','"snapshotBytes": 100663297,',1),
])
stage(T+"🧫️fixtures/🔣️.json",[
 ('"maximumFiles": 2048,','"maximumFiles": 16384,',1),
 ('"maximumSourceBytes": 20971520,','"maximumSourceBytes": 67108864,',1),
 ('"maximumSnapshotBytes": 33554432,','"maximumSnapshotBytes": 100663296,',1),
 ('"maximumOutputBytes": 8388608','"maximumOutputBytes": 33554432',1),
 ('      {\n        "files": 2048,\n        "accept": true\n      },\n      {\n        "files": 2049,\n        "accept": false\n      }',
  '      {\n        "files": 2049,\n        "accept": true\n      },\n      {\n        "files": 16384,\n        "accept": true\n      },\n      {\n        "files": 16385,\n        "accept": false\n      }',1),
])
stage(AU+"📦️snapshot/🧪️tests/📋️faults.json",[('  {\n   "name": "six files of three and a half megabytes exceed the source maximum",\n   "sources": [\n    {\n     "path": "a.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1750000\n     }\n    },\n    {\n     "path": "b.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1750000\n     }\n    },\n    {\n     "path": "c.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1750000\n     }\n    },\n    {\n     "path": "d.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1750000\n     }\n    },\n    {\n     "path": "e.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1750000\n     }\n    },\n    {\n     "path": "f.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1750000\n     }\n    }\n   ],\n   "entry": "a.ts",\n   "patch": {},\n','  {\n   "name": "seventeen files of three point nine five megabytes exceed the source maximum",\n   "sources": [\n    {\n     "path": "a.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1975000\n     }\n    },\n    {\n     "path": "b.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1975000\n     }\n    },\n    {\n     "path": "c.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1975000\n     }\n    },\n    {\n     "path": "d.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1975000\n     }\n    },\n    {\n     "path": "e.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1975000\n     }\n    },\n    {\n     "path": "f.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1975000\n     }\n    },\n    {\n     "path": "g.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1975000\n     }\n    },\n    {\n     "path": "h.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1975000\n     }\n    },\n    {\n     "path": "i.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1975000\n     }\n    },\n    {\n     "path": "j.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1975000\n     }\n    },\n    {\n     "path": "k.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1975000\n     }\n    },\n    {\n     "path": "l.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1975000\n     }\n    },\n    {\n     "path": "m.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1975000\n     }\n    },\n    {\n     "path": "n.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1975000\n     }\n    },\n    {\n     "path": "o.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1975000\n     }\n    },\n    {\n     "path": "p.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1975000\n     }\n    },\n    {\n     "path": "q.ts",\n     "content": {\n      "repeat": "ab",\n      "times": 1975000\n     }\n    }\n   ],\n   "entry": "a.ts",\n   "patch": {},\n',1)])
stage(AU+"📦️snapshot/🧪️tests/🟦️.ts",[('expect(policy).toEqual({maximumSourceBytes:20971520,maximumFileBytes:4194304,maximumSnapshotBytes:33554432,maximumOutputBytes:8388608});','expect(policy).toEqual({maximumSourceBytes:67108864,maximumFileBytes:4194304,maximumSnapshotBytes:100663296,maximumOutputBytes:33554432});',1)])
stage(T+"🟦️.ts",[('},120000);\n\ntest("immutable authorizer sources agree','},600000);\n\ntest("immutable authorizer sources agree',1)])
stage(T+"📏️capacity/🟦️.ts",[('expect(census.length).toBeLessThanOrEqual(2049)','expect(census.length).toBeLessThanOrEqual(closureSchema.definitions.Policy.const.maximumFiles+1)',1)])
stage(CL+"🧪️tests/📥️stream/🔣️.json",[('"bytes": 33554432,','"bytes": 100663296,',1),('"bytes": 33554433,','"bytes": 100663297,',1)])
stage(CL+"🧪️tests/📏️kernels/🟦️.ts",[
 ('const headroomFiles=32,headroomBytes=1_048_576;','const headroomShare=2;',1),
 ('this test fails while a kernel still fits the policy but is close to it,','this test fails while a kernel still fits the policy but holds more than half of it,',1),
 ('fits:census.files<=policy.maximumFiles-headroomFiles}','fits:census.files<=Math.floor(policy.maximumFiles/headroomShare)}',1),
 ('fits:census.sourceBytes<=policy.maximumSourceBytes-headroomBytes}','fits:census.sourceBytes<=Math.floor(policy.maximumSourceBytes/headroomShare)}',1),
])
stage(L+"🏗️build/🏛️sealed/🧪️tests/📏️closure/🟦️.ts",[
 ('const headroomFiles=64,headroomBytes=2_097_152;','const headroomShare=2;',1),
 ('fits:census.files<=policy.maximumFiles-headroomFiles}','fits:census.files<=Math.floor(policy.maximumFiles/headroomShare)}',1),
 ('fits:census.sourceBytes<=policy.maximumSourceBytes-headroomBytes}','fits:census.sourceBytes<=Math.floor(policy.maximumSourceBytes/headroomShare)}',1),
])
RV=B+"📥️inputs/🔏️revision/"
stage(RV+"🧬️schema/🔣️.json",[('"maxItems": 2048,','"maxItems": 16384,',2),('"maximum": 2048\n','"maximum": 16384\n',2)])
stage(RV+"🧪️tests/🔣️.json",[('"sources": 2048,','"sources": 16384,',1),('"sourceCount": 2048\n','"sourceCount": 16384\n',1)])
stage(B+"🔧️tools/🏗️bazel/🎬️scopes/🧬️schema/🔣️.json",[('"maxItems": 2048,','"maxItems": 16384,',3)])
GROWTH='''/** 📈️ Contract: no normal code addition can cross a source-closure bound. Every bound of the closure family is one proportional set (one file per 4096 source bytes, snapshot 1.5 times the sources, diagnostics with room above the admission), closures past the former limit of 2048 files are captured, and a closure past the real bound is refused in both languages with the chain and the way out. */
import{expect,test}from"bun:test";
import{mkdtemp,mkdir,writeFile,rm}from"node:fs/promises";
import{join,relative}from"node:path";
import{build}from"esbuild";
import{captureNxAuthorizerSourcesV1,nxAuthorizerCapacityTextV1}from"../../🟦️.ts";
import schema from"../../🧬️schema/🔣️.json";
import snapshotSchema from"../../📦️snapshot/🧬️schema/🔣️.json";
import snapshotPolicy from"../../📦️snapshot/🔣️policy.json";
import closureSchema from"../../../../../../🏗️build/📥️source/🔏️closure/🧬️schema/🔣️.json";
import revisionSchema from"../../../../../../🏗️build/📥️inputs/🔏️revision/🧬️schema/🔣️.json";

const policy=closureSchema.definitions.Policy.const,bytesPerFile=4096;

/** 🌳️ Writes a closure of exactly `count` files: one entry importing hubs of at most 64 leaves each, so no file nears the per-file site bound. */
async function closureTreeV1(area:string,count:number):Promise<void>{
 const hubs=Math.ceil((count-1)/65),leaves=count-1-hubs,writes:Promise<void>[]=[];
 for(let leaf=0;leaf<leaves;leaf++)writes.push(writeFile(join(area,"l"+leaf+".ts"),"export default {ordinal:"+leaf+"};"));
 for(let hub=0;hub<hubs;hub++){const own=Array.from({length:Math.max(0,Math.min(64,leaves-hub*64))},(_,index)=>hub*64+index);writes.push(writeFile(join(area,"h"+hub+".ts"),own.map(leaf=>"import l"+leaf+" from \\"./l"+leaf+".ts\\";export{l"+leaf+"};").join("\\n")+"\\nexport const hub"+hub+"=1;"));}
 writes.push(writeFile(join(area,"entry.ts"),Array.from({length:hubs},(_,hub)=>"import*as h"+hub+" from \\"./h"+hub+".ts\\";export{h"+hub+"};").join("\\n")));
 await Promise.all(writes);
}

test("the closure bounds are one proportional family, so one cannot lag behind the others",()=>{
 expect(policy.maximumSourceBytes).toBe(policy.maximumFiles*bytesPerFile);
 expect(policy.maximumSnapshotBytes*2).toBe(policy.maximumSourceBytes*3);
 expect(policy.maximumFileBytes).toBeLessThanOrEqual(policy.maximumSourceBytes);
 expect(schema.properties.sources.maxItems).toBe(policy.maximumFiles);
 expect(snapshotSchema.properties.files.maxItems).toBe(policy.maximumFiles);
 expect(closureSchema.properties.files.maxItems).toBe(policy.maximumFiles);
 expect(snapshotPolicy.maximumSourceBytes).toBe(policy.maximumSourceBytes);
 expect(snapshotPolicy.maximumSnapshotBytes).toBe(policy.maximumSnapshotBytes);
 const revision=revisionSchema.definitions;
 expect(revision.CapturedPhysicalSources.properties.sources.maxItems).toBe(policy.maximumFiles);
 expect(revision.CapturedPhysicalSourceImages.properties.images.maxItems).toBe(policy.maximumFiles);
 const diagnostic=schema.$defs.SourceCapacityDiagnostic.properties;
 expect(diagnostic.files.maximum).toBe(policy.maximumFiles+1);
 expect(diagnostic.bytes.maximum).toBe(policy.maximumSourceBytes+policy.maximumFileBytes);
 expect(snapshotPolicy.maximumOutputBytes*2).toBeGreaterThanOrEqual(snapshotPolicy.maximumSourceBytes);
 const census=schema.$defs.SourceCensusPolicy.const,discovery=schema.$defs.RuntimeDataDiscoveryPolicy.const,law=schema.$defs.CompilerClosureLaw.const,memo=schema.$defs.MemoPolicy.const;
 expect(census.maximumFiles).toBeGreaterThanOrEqual(policy.maximumFiles*4);
 expect(census.maximumSourceBytes).toBeGreaterThanOrEqual(policy.maximumSourceBytes*4);
 for(const key of["maximumDirectories","maximumEntries","maximumResults"]as const)expect(discovery[key],key).toBeGreaterThanOrEqual(policy.maximumFiles/2);
 expect(discovery.maximumDeclaredBytes).toBeGreaterThanOrEqual(policy.maximumSourceBytes);
 expect(memo.maximumFileBytes).toBeGreaterThanOrEqual(policy.maximumFiles*2048);
 expect(law.maximumSourceBytes).toBe(law.maximumFiles*bytesPerFile);
 expect(law.maximumFiles*4).toBeLessThanOrEqual(policy.maximumFiles*3);
 expect(law.maximumSnapshotBytes*4).toBeLessThanOrEqual(policy.maximumSnapshotBytes*3);
});

for(const count of[2049,4097])test("a closure of "+count+" files, past the former limit of 2048, is captured and agrees with esbuild",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Ticket-owned test artifacts required");await mkdir(artifacts,{recursive:true});
 const area=await mkdtemp(join(artifacts,"growth-")),root=process.cwd();
 try{
  await closureTreeV1(area,count);
  const entry=relative(root,join(area,"entry.ts")).replaceAll("\\\\","/"),captured=await captureNxAuthorizerSourcesV1(root,entry,AbortSignal.timeout(120000));
  expect(captured.manifest.sources.length).toBe(count);
  const oracle=await build({absWorkingDir:root,entryPoints:[entry],bundle:true,platform:"node",format:"esm",write:false,metafile:true,logLevel:"silent"});
  expect(Object.keys(oracle.metafile!.inputs).sort()).toEqual(captured.manifest.sources.map(row=>row.path).sort());
 }finally{await rm(area,{recursive:true,force:true});}
},300000);

test("a closure past the real bound is refused in English and German with the chain and the way out",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Ticket-owned test artifacts required");await mkdir(artifacts,{recursive:true});
 const area=await mkdtemp(join(artifacts,"growth-")),root=process.cwd();
 try{
  const count=policy.maximumFiles+1;await closureTreeV1(area,count);
  const entry=relative(root,join(area,"entry.ts")).replaceAll("\\\\","/");let failure:any;
  try{await captureNxAuthorizerSourcesV1(root,entry,AbortSignal.timeout(240000));}catch(error){failure=error;}
  expect(failure?.message).toContain("Authorizer source closure capacity exceeded: files="+count+"/"+policy.maximumFiles);
  expect(failure.message).toContain("Kapazität des Autorisierer-Quellabschlusses überschritten: Dateien="+count+"/"+policy.maximumFiles);
  expect(failure.message).toContain(entry);expect(failure.message).toContain("maximumFiles, maximumSourceBytes and maximumSnapshotBytes together");expect(failure.message).toContain("gemeinsam");
  expect(failure.message).not.toMatch(/\\{\\w+\\}/u);
 }finally{await rm(area,{recursive:true,force:true});}
},300000);

test("the capacity text fills every field in both languages",()=>{
 const text=nxAuthorizerCapacityTextV1({files:7,bytes:11,path:"a/b.ts",chain:["a/b.ts","a/entry.ts"]}),[english,german]=text.split(" | ");
 expect(english).toContain("files=7/"+policy.maximumFiles+", bytes=11/"+policy.maximumSourceBytes);expect(german).toContain("Dateien=7/"+policy.maximumFiles+", Bytes=11/"+policy.maximumSourceBytes);
 for(const part of[english,german]){expect(part).toContain("a/b.ts <- a/entry.ts");expect(part).not.toMatch(/\\{\\w+\\}/u);}
});
'''
stage(T+"📈️growth/🟦️.ts",new=GROWTH)
open(H+"index.txt","w",encoding="utf8").write("\n".join(rows)+"\n")
print(len(rows),"files staged")
