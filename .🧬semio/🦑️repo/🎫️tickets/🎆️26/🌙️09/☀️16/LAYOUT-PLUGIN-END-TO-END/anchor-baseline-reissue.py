from pathlib import Path
p=Path("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/⚓️envelope/🟦️.ts")
s=p.read_text()
assert "export function inspectNativeDirectoryBaselineNamespaceV1" not in s
s=s.replace('"InitialFacts"|"WitnessRecord",value:unknown','"InitialFacts"|"WitnessRecord"|"BaselineNamespace",value:unknown')
s+=r"""
/** 🗃️ Selects a complete agreeing baseline as DATA while a fresh Native role separately owns current metadata. */
export function inspectNativeDirectoryBaselineNamespaceV1(input:unknown):Readonly<{keeper:string;discard:readonly string[];bytes:Uint8Array;baseline:ReturnType<typeof decodeNativeDirectoryEnvelopeJournalV1>;authority:false}>{
 const value=closed<Readonly<{entries:readonly Readonly<{name:string;text:string}>[]}>>("BaselineNamespace",input),entries=[...value.entries].sort((a,b)=>a.name<b.name?-1:a.name>b.name?1:0),complete:Array<Readonly<{name:string;baseline:ReturnType<typeof decodeNativeDirectoryEnvelopeJournalV1>}>>=[];
 if(new Set(entries.map(row=>row.name)).size!==entries.length)throw Error("Distinct bounded current baseline metadata roles required");
 for(const row of entries){if(Buffer.byteLength(row.text)>policy.maximumRecordBytes||Buffer.from(row.text).toString("utf8")!==row.text)throw Error("Bounded exact current baseline metadata text required");let baseline:ReturnType<typeof decodeNativeDirectoryEnvelopeJournalV1>;try{baseline=readNativeDirectoryEnvelopeRecoveryJournalV1(Buffer.from(row.text)).journal;}catch{continue;}if(baseline.transition)throw Error("Prepared envelope cannot be reissued as unprepared metadata");complete.push({name:row.name,baseline});}
 const keeper=complete[0];if(!keeper)throw Error("A complete durable baseline keeper is required");
 const context=(row:typeof keeper.baseline)=>JSON.stringify({directory:row.request.directory,backing:row.request.backing,receiptSha256:row.request.receiptSha256,before:row.preparation.before,target:row.preparation.target,volume:row.preparation.volume,witness:row.preparation.witness,platform:row.preparation.planned.platform,parent:{dev:row.preparation.planned.parent.dev,ino:row.preparation.planned.parent.ino,mode:row.preparation.planned.parent.mode},windows:row.preparation.planned.windows});
 if(complete.some(row=>context(row.baseline)!==context(keeper.baseline)))throw Error("Conflicting complete Native metadata baselines retained");
 return Object.freeze({keeper:keeper.name,discard:Object.freeze(entries.filter(row=>row.name!==keeper.name).map(row=>row.name)),bytes:encodeNativeDirectoryEnvelopeJournalV1(keeper.baseline.request,keeper.baseline.preparation),baseline:keeper.baseline,authority:false});
}
"""
p.write_text(s)
p=Path("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🎛️owner/⚓️envelope/🟦️.ts")
s=p.read_text();s=s.replace("encodeNativeDirectoryEnvelopeInitialV1,readNativeDirectoryEnvelopeInitialV1,","inspectNativeDirectoryBaselineNamespaceV1,inspectNativeDirectoryUnpreparedMetadataV1,encodeNativeDirectoryEnvelopeInitialV1,readNativeDirectoryEnvelopeInitialV1,")
at=s.index("/** 🚪️ Reissues only fresh actual Native recovery custody")
s=s[:at]+r"""
async function reissueReservedBaseline(ctx:Context):Promise<boolean>{
 const check=()=>ctx.check();return withArtifactRelativeDirectoryV1(ctx.options.directory,check,async root=>{
  if(JSON.stringify(await root.names())!==JSON.stringify(rolePolicy.baseNames))throw Error("Closed no-stage Native baseline reissue base required");
  return withNamespace(ctx,root,async namespace=>{
   const names=await namespace.names();check();if(!names.length||names.includes("intent.json")||names.some(name=>!artifactRelativeFilePublicationStageNameV1("intent.json",name)))return false;if(names.length>2)throw Error("Bounded current Native baseline namespace required");
   const entries=[],held=new Map<string,Awaited<ReturnType<typeof namespace.entry>>>();for(const name of names){const entry=await namespace.entry(name);if(!entry||entry.kind!=="file"||entry.nlink!==1n||entry.size>4096n||process.platform!=="win32"&&(entry.mode&511n)!==384n)throw Error("Bounded current reserved Native baseline file required");const value=await record(namespace,name,check),attestation=await namespace.attestFilePublication(name,entry,4096);check();if(attestation.sha256!==hash(value.bytes)||stamp(attestation.identity)!==JSON.stringify(value.identity))throw Error("Same-entry current Native baseline attestation required");held.set(name,entry);entries.push({name,text:value.bytes.toString("utf8")});if(!Buffer.from(entries.at(-1)!.text).equals(value.bytes))throw Error("Exact bounded Native baseline text required");}
   const selected=inspectNativeDirectoryBaselineNamespaceV1({entries}),baseline=selected.baseline,anchor=await record(root,"fixed.dir",check),old=readNativeDirectoryAnchorV1(anchor.bytes),map=mapping(ctx,old),receipt=await record(root,"image.json",check),published=JSON.parse(receipt.bytes.toString("utf8"));check();
   const anchorEntry=await root.entry("fixed.dir");if(!anchorEntry||anchorEntry.kind!=="file")throw Error("Actual old Native anchor entry required for baseline reissue");const anchorFacts=await root.attestFilePublication("fixed.dir",anchorEntry,4096);check();if(anchorFacts.sha256!==hash(anchor.bytes)||stamp(anchorFacts.identity)!==JSON.stringify(anchor.identity)||JSON.stringify(anchorFacts.windows?{parent:anchorFacts.windows.parent,prior:anchorFacts.windows.entry}:null)!==JSON.stringify(baseline.preparation.planned.windows))throw Error("Exact complete current Native anchor platform facts required");
   if(validateJsonSchemaSubset(receiptSchema.definitions.ImageReceipt,published,receiptSchema,[mountSchema]).length||published.platform!==process.platform||String(published.dev)!==anchor.identity.dev||String(published.ino)!==anchor.identity.ino||String(published.size)!==anchor.identity.size||published.filesystemBytes!==old.filesystemBytes||published.maximumPhysicalBytes!==old.maximumPhysicalBytes||!same(published.mountDirectory,old.underlying))throw Error("Exact coherent retained Native baseline receipt required");
   inspectNativeDirectoryUnpreparedMetadataV1(baseline.request,baseline.preparation,{declaration:{filesystemBytes:ctx.options.filesystemBytes,maximumPhysicalBytes:ctx.options.maximumPhysicalBytes},parent:identity(root.stat()),anchor:{identity:anchor.identity,sha256:hash(anchor.bytes)},receiptSha256:hash(receipt.bytes),root:map.root,volume:observeNativeImageMountDirectoryV1(join(ctx.options.directory,"v")),names:await root.names(),stage:null,pendingSwap:nativeDirectorySwapPendingV1(ctx.options.directory)});
   const remaining=new Set(names),verify=async()=>{check();if(JSON.stringify(await root.names())!==JSON.stringify(rolePolicy.baseNames)||JSON.stringify(await namespace.names())!==JSON.stringify([...remaining].sort()))throw Error("Exact held Native baseline namespace required");};
   const remove=async(name:string)=>{await verify();const entry=held.get(name);if(!entry||entry.kind!=="file")throw Error("Actual retained Native baseline entry required");await namespace.attestFilePublication(name,entry,4096);check();await namespace.remove(name,entry);check();remaining.delete(name);await namespace.sync();await verify();};
   for(const name of selected.discard)await remove(name);
   await verify();const keeper=await record(namespace,selected.keeper,check);if(!keeper.bytes.equals(Buffer.from(entries.find(row=>row.name===selected.keeper)!.text)))throw Error("Complete durable Native baseline keeper changed");
   const expected=await namespace.captureFilePublication("intent.json",4096);check();if(expected.identity!==null)throw Error("Absent canonical Native intent required for reissue");const result=await namespace.publishFile(expected,selected.bytes);check();remaining.add("intent.json");await namespace.sync();await verify();const current=await record(namespace,"intent.json",check);if(result.sha256!==hash(selected.bytes)||!current.bytes.equals(Buffer.from(selected.bytes)))throw Error("Durable exact canonical Native baseline required before keeper retirement");
   await remove(selected.keeper);await verify();return true;
  });
 });
}
""" +s[at:]
old='const loaded=await withArtifactRelativeDirectoryV1(ctx.options.directory,check,root=>withNamespace(ctx,root,async namespace=>{const names=await namespace.names();check();if(names.length>2)throw Error("Closed initial Native metadata namespace required");return names.includes("intent.json")?record(namespace,"intent.json",check):null;}));check();if(!loaded){await resetInitialDirectoryMetadata(ctx,null);return null;}let journal:Journal;'
new='let loaded=await withArtifactRelativeDirectoryV1(ctx.options.directory,check,root=>withNamespace(ctx,root,async namespace=>{const names=await namespace.names();check();if(names.length>2)throw Error("Closed initial Native metadata namespace required");return names.includes("intent.json")?record(namespace,"intent.json",check):null;}));check();if(!loaded){if(await reissueReservedBaseline(ctx))loaded=await withArtifactRelativeDirectoryV1(ctx.options.directory,check,root=>namespaceRecord(ctx,root,check));else{await resetInitialDirectoryMetadata(ctx,null);return null;}}let journal:Journal;'
assert s.count(old)==1
s=s.replace(old,new);p.write_text(s)
print("Fresh reserved baseline current-role reissue authored; no live effects executed.")
