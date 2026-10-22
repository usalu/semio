#!/usr/bin/env python3
"""Anchored, idempotent edit: the produced workspace edge walk opens each directory from its parent descriptor (one nested handle per level) instead of re-proving the whole path from the store root for every directory (O(depth^2) opens), fences every `fullFenceInterval` entries instead of every entry, and reads the store device once."""
import sys,pathlib
f=pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts")
s=f.read_text(encoding="utf8")
a=s.index(" const pending: string[][]=[[]];let entries=0,work=0,next=policy.progressWorkUnits;const advance=async()=>{checkpoint();if(++work>policy.maximumWork)")
b=s.index(" checkpoint();\n}\n",a)
old=s[a:b]
new=''' let entries=0,work=0,next=policy.progressWorkUnits;const device=base.stat().dev,advance=async()=>{if(++work%policy.fullFenceInterval===1)checkpoint();if(work>policy.maximumWork)throw Error("Produced workspace edge work refused");if(work>=next){next=work+policy.progressWorkUnits;await control.publish({resource:"dependencies:workspace-edges:"+entries,mode:"exclusive",elapsedMs:0});await control.yieldContinuation();checkpoint();}};
 const rewrites:{parts:readonly string[];name:string;target:string;source:string}[]=[];
 const walk=async(directory:ArtifactRelativeDirectoryV1,parts:readonly string[]):Promise<void>=>{
  if(parts.length>policy.maximumDepth)throw Error("Produced workspace edge depth refused");checkpoint();
  const children:string[]=[];
  for(const name of await directory.names()){await advance();if(++entries>policy.maximumEntries||join(generationRoot,...parts,name).length>policy.maximumPathUnits)throw Error("Produced workspace edge capacity refused");const entry=await directory.entry(name);if(!entry||entry.dev!==device)throw Error("Current produced workspace entry changed");if(entry.kind==="directory"){children.push(name);continue;}if(entry.kind!=="link")continue;if(typeof entry.target!=="string")throw Error("Complete produced link target required");const target=resolve(generationRoot,...parts,entry.target),mapping=mappings.find(value=>target===value.projected);if(mapping)rewrites.push({parts,name,target:entry.target,source:mapping.source});else if(mappings.some(value=>target.startsWith(value.projected+sep)&&target!==join(value.projected,"node_modules")&&!target.startsWith(join(value.projected,"node_modules")+sep)))throw Error("Unqualified projected workspace subpath retained");}
  for(const name of children)await directory.withDirectory(name,child=>walk(child,[...parts,name]));
 };
 await withJavascriptDependencyProducedDirectoryV1(base,["j",row.plan.generation.slice(0,20)],false,directory=>walk(directory,[]));
 for(let at=0;at<rewrites.length;){const parts=rewrites[at]!.parts;let end=at;while(end<rewrites.length&&end-at<policy.maximumBindingRewritesPerHandle&&rewrites[end]!.parts===parts)end++;const batch=rewrites.slice(at,end);at=end;await withJavascriptDependencyProducedDirectoryV1(base,["j",row.plan.generation.slice(0,20),...parts],false,async handle=>{for(const rewrite of batch){checkpoint();const before=await handle.captureBinding(rewrite.name);if(before.target!==rewrite.target)throw Error("Projected workspace edge changed");await handle.publishBinding(before,{kind:"directory",path:rewrite.source});}});}
'''
if new in s and old not in s: print("already applied"); sys.exit(0)
if s.count(old)!=1: sys.exit("anchor drift")
if "--dry" in sys.argv: print("would apply, old",len(old),"new",len(new)); sys.exit(0)
f.write_text(s[:a]+new+s[b:],encoding="utf8")
print("applied")
