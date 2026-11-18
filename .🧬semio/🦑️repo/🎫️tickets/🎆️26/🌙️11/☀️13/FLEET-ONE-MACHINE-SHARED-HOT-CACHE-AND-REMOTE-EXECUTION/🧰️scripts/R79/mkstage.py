#!/usr/bin/env python3
"""R79 stage builder (landing 1): the Windows relative-handle layer counts and times every native call by symbol name, and the dependency producer prints a binding profile line every 8 bindings (wall, snapshot, publish and fence milliseconds, changed count, native calls), so the cost of one binding on Windows is measured, not guessed."""
import os,shutil,sys
R="/Users/ueli/Documents/semio/"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
DP=L+"⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/"
DS="🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts"
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
        if got!=count:sys.exit("anchor count %d != %d in %s: %r"%(got,count,rel,old[:90]))
        text=text.replace(old,rep)
    open(H+i+".base","w",encoding="utf8").write(base);open(H+i+".new","w",encoding="utf8").write(text);rows.append("%s %s mod"%(i,rel))
PROFILE='''type Scope={descriptor?:unknown;unresolved:boolean;failure?:unknown};
/** 📈️ Counts and times every native call of the Windows relative layer by symbol name, so a slow walk shows which call costs. */
const artifactRelativeNativeProfileV1=new Map<string,[number,number]>();
export function timedArtifactRelativeBodyV1<T>(name:string,run:()=>T):T{const at=performance.now();try{return run();}finally{const row=artifactRelativeNativeProfileV1.get(name)??[0,0];row[0]++;row[1]+=performance.now()-at;artifactRelativeNativeProfileV1.set(name,row);}}
export function takeArtifactRelativeNativeProfileV1():Readonly<Record<string,readonly[number,number]>>{const rows:Record<string,readonly[number,number]>={};for(const[name,[count,milliseconds]]of artifactRelativeNativeProfileV1)rows[name]=Object.freeze([count,Math.round(milliseconds*10)/10]as[number,number]);artifactRelativeNativeProfileV1.clear();return Object.freeze(rows);}
function timedArtifactRelativeSymbolsV1(symbols:Record<string,(...args:any[])=>any>):Record<string,(...args:any[])=>any>{const out:Record<string,(...args:any[])=>any>={};for(const name of Object.keys(symbols)){const call=symbols[name]!;out[name]=(...args:any[])=>timedArtifactRelativeBodyV1(name,()=>call(...args));}return out;}
const openSync=((...args:any[])=>timedArtifactRelativeBodyV1("openSync",()=>(openSyncRaw as any)(...args)))as typeof openSyncRaw;\nconst opendirSync=((...args:any[])=>timedArtifactRelativeBodyV1("opendirSync",()=>(opendirSyncRaw as any)(...args)))as typeof opendirSyncRaw;\nconst closeSync=((...args:any[])=>timedArtifactRelativeBodyV1("closeSync",()=>(closeSyncRaw as any)(...args)))as typeof closeSyncRaw;\nconst fstatSync=((...args:any[])=>timedArtifactRelativeBodyV1("fstatSync",()=>(fstatSyncRaw as any)(...args)))as typeof fstatSyncRaw;\nconst readSync=((...args:any[])=>timedArtifactRelativeBodyV1("readSync",()=>(readSyncRaw as any)(...args)))as typeof readSyncRaw;\nconst writeSync=((...args:any[])=>timedArtifactRelativeBodyV1("writeSync",()=>(writeSyncRaw as any)(...args)))as typeof writeSyncRaw;\nconst fsyncSync=((...args:any[])=>timedArtifactRelativeBodyV1("fsyncSync",()=>(fsyncSyncRaw as any)(...args)))as typeof fsyncSyncRaw;\nconst lstatSync=((...args:any[])=>timedArtifactRelativeBodyV1("lstatSync",()=>(lstatSyncRaw as any)(...args)))as typeof lstatSyncRaw;'''
stage(DS,[
 ('type Scope={descriptor?:unknown;unresolved:boolean;failure?:unknown};',PROFILE,1),
 ('import{constants,openSync,opendirSync,closeSync,fstatSync,readSync,writeSync}from"node:fs";','import{constants,openSync as openSyncRaw,opendirSync as opendirSyncRaw,closeSync as closeSyncRaw,fstatSync as fstatSyncRaw,readSync as readSyncRaw,writeSync as writeSyncRaw}from"node:fs";',1),
 ('import{lstatSync,fsyncSync}from"node:fs";','import{lstatSync as lstatSyncRaw,fsyncSync as fsyncSyncRaw}from"node:fs";',1),
 ('const calls=library.symbols as unknown as Record<string,(...args:any[])=>any>,errno=','const calls=timedArtifactRelativeSymbolsV1(library.symbols as unknown as Record<string,(...args:any[])=>any>),errno=',1),
 ('return{symbols:library.symbols as unknown as Record<string,(...args:any[])=>any>,close(){','return{symbols:timedArtifactRelativeSymbolsV1(library.symbols as unknown as Record<string,(...args:any[])=>any>),close(){',1),
 ('let volume:string|undefined;const facts=(owned:ArtifactWindowsRelativeHandleV1,deleted=false):ArtifactWindowsRelativeFactsV1=>{','let volume:string|undefined;const facts=(owned:ArtifactWindowsRelativeHandleV1,deleted=false):ArtifactWindowsRelativeFactsV1=>timedArtifactRelativeBodyV1("js:facts",()=>factsOf(owned,deleted)),factsOf=(owned:ArtifactWindowsRelativeHandleV1,deleted=false):ArtifactWindowsRelativeFactsV1=>{',1),
])
stage(DP+"🟦️.ts",[
 ('withArtifactRelativeDirectoryV1, type ArtifactRelativeDirectoryV1,','withArtifactRelativeDirectoryV1, takeArtifactRelativeNativeProfileV1, type ArtifactRelativeDirectoryV1,',1),
 ('let work=0,next=limits.progressWorkUnits,metadata=0,fileCount=0,linkCount=0,logical=0n;','let work=0,next=limits.progressWorkUnits,metadata=0,fileCount=0,linkCount=0,logical=0n,steps=0;',1),
 ('await control.publish({resource:"javascript-selected-payload:"+rows.length,mode:"shared",elapsedMs:performance.now()-started});check();','await control.publish({resource:"javascript-selected-payload:"+rows.length,mode:"shared",elapsedMs:performance.now()-started});if(steps++%32===0)await writeOwnedCommandOutputV1(process.stderr,"[DATA] Store dependency payload profile "+JSON.stringify({version:1,purpose:"payload-profile",rows:rows.length,files:fileCount,links:linkCount,directories:directories.length,work,elapsedMs:Math.round(performance.now()-started),native:takeArtifactRelativeNativeProfileV1()})+"\\n",control);check();',1),
 ('let published=0;for(const row of selected)for(const binding of row.plan.bindings){const snapshot=source(),previous=','let published=0;const lap={at:performance.now(),changed:0,snapshot:0,publish:0,fence:0};for(const row of selected)for(const binding of row.plan.bindings){const lapA=performance.now(),snapshot=source(),previous=',1),
 ('const result=await publishJavascriptDependencyPreservingSourceDescriptorV1(binding,previous,snapshot,facts,checkpoint);','const lapB=performance.now(),result=await publishJavascriptDependencyPreservingSourceDescriptorV1(binding,previous,snapshot,facts,checkpoint),lapC=performance.now();lap.snapshot+=lapB-lapA;lap.publish+=lapC-lapB;',1),
 ('if(result.changed)completedBindingsChanged++;','if(result.changed){completedBindingsChanged++;lap.changed++;}',1),
 ('if(++published===1||published%8===0)await control.publish({resource:"dependencies:publish-binding:"+published,mode:"shared",elapsedMs:0});if(published%installSchema.$defs.InstalledPathProtocol.const.fullFenceInterval===0)await fence();else{checkpoint();requireJavascriptDependencyEndpointFenceV1(binding.sourceDirectory,expected,parents,ancestors);checkpoint();}}',
  'if(++published===1||published%8===0){await control.publish({resource:"dependencies:publish-binding:"+published,mode:"shared",elapsedMs:0});await writeOwnedCommandOutputV1(process.stderr,"[DATA] Store dependency binding profile "+JSON.stringify({version:1,purpose:"binding-profile",bindings:published,changed:lap.changed,wallMs:Math.round(performance.now()-lap.at),snapshotMs:Math.round(lap.snapshot),publishMs:Math.round(lap.publish),fenceMs:Math.round(lap.fence),native:takeArtifactRelativeNativeProfileV1()})+"\\n",control);lap.at=performance.now();lap.changed=0;lap.snapshot=0;lap.publish=0;lap.fence=0;}const lapD=performance.now();if(published%installSchema.$defs.InstalledPathProtocol.const.fullFenceInterval===0)await fence();else{checkpoint();requireJavascriptDependencyEndpointFenceV1(binding.sourceDirectory,expected,parents,ancestors);checkpoint();}lap.fence+=performance.now()-lapD;}',1),
])
stage(DP+"🧪️tests/🚧️fence/🟦️.ts",[('expect(producer).toContain("if(++published%installSchema.$defs.InstalledPathProtocol.const.fullFenceInterval===0)await fence();else','expect(producer).toContain("if(published%installSchema.$defs.InstalledPathProtocol.const.fullFenceInterval===0)await fence();else',1)])
TEST='''/** 📈️ Contract: the cost of one dependency binding is measured, not guessed: the relative-handle layer counts and times every native call by symbol, and the producer prints one profile line per progress step with the wall, snapshot, publish and fence milliseconds. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import{mkdtempSync,mkdirSync,writeFileSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{takeArtifactRelativeNativeProfileV1,timedArtifactRelativeBodyV1,withArtifactRelativeDirectoryV1}from"../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";

const source=readFileSync(fileURLToPath(new URL("../../🟦️.ts",import.meta.url)),"utf8");
const layer=readFileSync(fileURLToPath(new URL("../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts",import.meta.url)),"utf8");

test("the profile counts calls and milliseconds by name and is drained by every take",()=>{
 takeArtifactRelativeNativeProfileV1();
 expect(timedArtifactRelativeBodyV1("FlushFileBuffers",()=>7)).toBe(7);timedArtifactRelativeBodyV1("FlushFileBuffers",()=>0);
 expect(()=>timedArtifactRelativeBodyV1("NtCreateFile",()=>{throw Error("x");})).toThrow("x");
 const taken=takeArtifactRelativeNativeProfileV1();
 expect(Object.keys(taken).sort()).toEqual(["FlushFileBuffers","NtCreateFile"]);expect(taken.FlushFileBuffers![0]).toBe(2);expect(taken.NtCreateFile![0]).toBe(1);
 expect(takeArtifactRelativeNativeProfileV1()).toEqual({});
});

test("a real POSIX walk through the relative layer reports its native calls by symbol",async()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-r79-"));
 try{mkdirSync(join(root,"d"));writeFileSync(join(root,"d","a.txt"),"a");takeArtifactRelativeNativeProfileV1();
  await withArtifactRelativeDirectoryV1(root,()=>{},async top=>{const names=await top.names();expect(names).toEqual(["d"]);await top.withDirectory("d",async child=>{expect(await child.entry("a.txt")).not.toBeNull();});});
  const taken=takeArtifactRelativeNativeProfileV1(),total=Object.values(taken).reduce((sum,row)=>sum+row[0],0);
  expect(taken.openat![0]).toBeGreaterThanOrEqual(3);expect(total).toBeGreaterThan(8);expect(total).toBeLessThan(200);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("every Windows native symbol of the relative layer is counted and the producer prints the profile line",()=>{
 expect(layer).toContain("symbols:timedArtifactRelativeSymbolsV1(library.symbols");
 expect(layer).toContain('timedArtifactRelativeBodyV1("js:facts"');
 const from=source.indexOf("await publishJavascriptDependencyPreservingSourceDescriptorV1(binding"),to=source.indexOf("await verifyReady();const finals"),loop=source.slice(from,to);
 expect(loop).toContain('purpose:"binding-profile"');expect(loop).toContain("native:takeArtifactRelativeNativeProfileV1()");
 expect(source).toContain('purpose:"payload-profile"');expect(layer).toContain("fstatSync as fstatSyncRaw");expect(layer).toContain("const calls=timedArtifactRelativeSymbolsV1(library.symbols");
 for(const field of["wallMs","snapshotMs","publishMs","fenceMs","changed"])expect(loop).toContain(field+":");
});
'''
stage(DP+"🧪️tests/📈️profile/🟦️.ts",new=TEST)
open(H+"index.txt","w",encoding="utf8").write("\n".join(rows)+"\n")
print("staged",len(rows))
