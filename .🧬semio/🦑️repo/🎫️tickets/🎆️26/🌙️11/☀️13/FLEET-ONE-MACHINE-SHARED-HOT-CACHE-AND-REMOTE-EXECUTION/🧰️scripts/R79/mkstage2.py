#!/usr/bin/env python3
"""R79 stage builder (landing 2): (1) the physical flush of a binding publication is deferred inside the producer loop and ended by one directory flush, because a Windows flush costs about 2 s and the loop paid three per binding; (2) a stale selected-retirement request is refused from the bindings before the payload read instead of after 40 minutes of it."""
import os,shutil,sys
R="/Users/ueli/Documents/semio/"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
DP=L+"⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/"
PUB=L+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/"
DS="🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts"
H=os.path.dirname(os.path.abspath(__file__))+"/stage2/"
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
LST='const lstatSync=((...args:any[])=>timedArtifactRelativeBodyV1("lstatSync",()=>(lstatSyncRaw as any)(...args)))as typeof lstatSyncRaw;'
DEFER=LST+'''
let artifactRelativeDeferredFlushesV1=0;
/** 🚰️ Defers the physical flush of every binding publication until the returned release runs; the caller ends the loop with one directory flush, which on a journalling filesystem (NTFS, ext4) orders every earlier metadata change before it. A Windows flush forces the disk and cost about two seconds, three per binding. */
export function beginArtifactRelativeDeferredFlushV1():()=>void{artifactRelativeDeferredFlushesV1++;let released=false;return()=>{if(released)return;released=true;artifactRelativeDeferredFlushesV1--;};}
/** 🧯️ Tells whether a binding publication may skip its own physical flush. */
export function artifactRelativeFlushDeferredV1():boolean{return artifactRelativeDeferredFlushesV1>0;}'''
stage(DS,[
 (LST,DEFER,1),
 ('const flush=(child:ArtifactWindowsRelativeHandleV1)=>{requireSynchronousArtifactResultV1(checkpoint());if(!call(()=>kernel.symbols.FlushFileBuffers!(child.value)))throw failure("binding flush");};','const flush=(child:ArtifactWindowsRelativeHandleV1)=>{requireSynchronousArtifactResultV1(checkpoint());if(artifactRelativeFlushDeferredV1())return;if(!call(()=>kernel.symbols.FlushFileBuffers!(child.value)))throw failure("binding flush");};',1),
 ('bindingGuard();fsyncSync(descriptor);bindingGuard();const final=await readEntry(name),names=await creationNames()','bindingGuard();if(!artifactRelativeFlushDeferredV1())fsyncSync(descriptor);bindingGuard();const final=await readEntry(name),names=await creationNames()',1),
])
stage(DP+"🟦️.ts",[
 ('withArtifactRelativeDirectoryV1, takeArtifactRelativeNativeProfileV1,','withArtifactRelativeDirectoryV1, takeArtifactRelativeNativeProfileV1, beginArtifactRelativeDeferredFlushV1,',1),
 ('let published=0;const lap={at:performance.now(),changed:0,snapshot:0,publish:0,fence:0};for(const row of selected)for(const binding of row.plan.bindings){','let published=0,lastParent:string|null=null;const lap={at:performance.now(),changed:0,snapshot:0,publish:0,fence:0},releaseDeferred=beginArtifactRelativeDeferredFlushV1();try{for(const row of selected)for(const binding of row.plan.bindings){',1),
 ('if(result.changed){completedBindingsChanged++;lap.changed++;}','if(result.changed){completedBindingsChanged++;lap.changed++;lastParent=dirname(binding.sourceDirectory);}',1),
 ('lap.fence+=performance.now()-lapD;}await verifyReady();','lap.fence+=performance.now()-lapD;}}finally{releaseDeferred();}if(lastParent!==null){checkpoint();await withArtifactRelativeDirectoryV1(lastParent,checkpoint,directory=>directory.sync());checkpoint();}await verifyReady();',1),
])
SEL='''/** ⏳️ Refuses a stale selected-retirement request from the bindings and the roster alone, before any payload byte is read: reading a generation takes tens of minutes and this answer does not depend on it. */
function requireRepositoryDependencySelectionCurrentV1(request:ReturnType<typeof readJavascriptDependencySelectedRetirementCommandV1>,current:Awaited<ReturnType<typeof readJavascriptDependencyCurrentBindingsV1>>,selected:Awaited<ReturnType<typeof captureRepositoryDependencySelectedPlanV1>>["selected"]):void{
 const selection=request.selection;if(!current.scopes.length||current.scopes.some(scope=>scope.generation===null||scope.receiptFingerprint===null||scope.bindings.length!==scope.members.length+1)||current.sha256!==selection.currentBindingsSha256)throw Error("Every current dependency scope must be qualified before inactivity");if(!selected||selected.prefix!==selection.prefix||selected.rootStamp!==selection.rootStamp)throw Error("Exact current selected dependency root required");
}
/** 🎯️ Compares fresh current DATA inside the fixed owner's authentic lifetime without issuing authority. */'''
stage(PUB+"🟦️.ts",[
 ("/** 🎯️ Compares fresh current DATA inside the fixed owner's authentic lifetime without issuing authority. */",SEL,1),
 ('const payload=await(request.selection.kind==="inactive-generation"||request.selection.kind==="empty-generation"?readJavascriptDependencySelectedPayloadV1(','requireRepositoryDependencySelectionCurrentV1(request,bindings,candidate);const payload=await(request.selection.kind==="inactive-generation"||request.selection.kind==="empty-generation"?readJavascriptDependencySelectedPayloadV1(',1),
 ('const selected=plan.selected,selection=request.selection;if(!current.scopes.length||current.scopes.some(scope=>scope.generation===null||scope.receiptFingerprint===null||scope.bindings.length!==scope.members.length+1)||current.sha256!==selection.currentBindingsSha256)throw Error("Every current dependency scope must be qualified before inactivity");if(!selected||selected.prefix!==selection.prefix||selected.rootStamp!==selection.rootStamp||payload.rootStamp!==selection.rootStamp)throw Error("Exact current selected dependency root required");','const selected=plan.selected,selection=request.selection;requireRepositoryDependencySelectionCurrentV1(request,current,selected);if(payload.rootStamp!==selection.rootStamp)throw Error("Exact current selected dependency root required");',1),
])
TEST='''/** 🚰️ Contract: a binding publication inside the producer loop pays no physical flush of its own (a Windows flush forces the disk and cost about two seconds, three per binding), the loop ends with one directory flush before the ready receipts are read, and a stale selected-retirement request is refused before the payload read. */
import{expect,test}from"bun:test";
import{mkdtempSync,mkdirSync,rmSync,readlinkSync}from"node:fs";
import{readFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{takeArtifactRelativeNativeProfileV1,withArtifactRelativeDirectoryV1,beginArtifactRelativeDeferredFlushV1,artifactRelativeFlushDeferredV1}from"../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";

const text=(relative:string)=>readFileSync(fileURLToPath(new URL(relative,import.meta.url)),"utf8");
const producer=text("../../🟦️.ts"),layer=text("../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts"),publication=text("../../../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts");

const publishCount=async(root:string,deferred:boolean,target:string)=>{
 const release=deferred?beginArtifactRelativeDeferredFlushV1():()=>{};takeArtifactRelativeNativeProfileV1();
 try{await withArtifactRelativeDirectoryV1(join(root,"p"),()=>{},async directory=>{const expected=await directory.captureBinding("node_modules");await directory.publishBinding(expected,{kind:"directory",path:target});});}finally{release();}
 return takeArtifactRelativeNativeProfileV1().fsyncSync?.[0]??0;
};

test("a binding publication flushes once on its own and not at all while the flush is deferred, and the link is the same",async()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-r79-flush-"));
 try{mkdirSync(join(root,"p"));mkdirSync(join(root,"a"));mkdirSync(join(root,"b"));
  expect(await publishCount(root,false,join(root,"a"))).toBe(1);expect(readlinkSync(join(root,"p","node_modules"))).toBe(join(root,"a"));
  expect(await publishCount(root,true,join(root,"b"))).toBe(0);expect(readlinkSync(join(root,"p","node_modules"))).toBe(join(root,"b"));
  expect(artifactRelativeFlushDeferredV1()).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the deferral nests, releases once and never leaks",()=>{
 const outer=beginArtifactRelativeDeferredFlushV1(),inner=beginArtifactRelativeDeferredFlushV1();expect(artifactRelativeFlushDeferredV1()).toBe(true);
 inner();inner();expect(artifactRelativeFlushDeferredV1()).toBe(true);outer();expect(artifactRelativeFlushDeferredV1()).toBe(false);
});

test("the Windows binding flush honours the deferral, the loop releases it in a finally and ends with one directory flush before the receipts",()=>{
 expect(layer).toContain('if(artifactRelativeFlushDeferredV1())return;if(!call(()=>kernel.symbols.FlushFileBuffers!(child.value)))throw failure("binding flush")');
 const begin=producer.indexOf("releaseDeferred=beginArtifactRelativeDeferredFlushV1();try{"),finish=producer.indexOf("}finally{releaseDeferred();}"),barrier=producer.indexOf("directory=>directory.sync()",finish),ready=producer.indexOf("await verifyReady();const finals");
 expect(begin).toBeGreaterThan(0);expect(finish).toBeGreaterThan(begin);expect(barrier).toBeGreaterThan(finish);expect(ready).toBeGreaterThan(barrier);
 expect(producer.slice(begin,finish)).toContain("publishJavascriptDependencyPreservingSourceDescriptorV1(binding");
});

test("a stale selected-retirement request is refused before the payload read",()=>{
 const call=publication.indexOf("requireRepositoryDependencySelectionCurrentV1(request,bindings,candidate);"),read=publication.indexOf("readJavascriptDependencySelectedPayloadV1(context,current,request.selection.prefix,members)");
 expect(call).toBeGreaterThan(0);expect(read).toBeGreaterThan(call);
 expect(publication).toContain('current.sha256!==selection.currentBindingsSha256)throw Error("Every current dependency scope must be qualified before inactivity")');
});
'''
stage(DP+"🧪️tests/🚰️deferred-flush/🟦️.ts",new=TEST)
open(H+"index.txt","w",encoding="utf8").write("\n".join(rows)+"\n")
print("staged",len(rows))
