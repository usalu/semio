import sys,re
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R205/"
def sub(text,old,new):
    assert text.count(old)==1,(text.count(old),old[:80])
    return text.replace(old,new)
a=open(S+"base/adopt.ts",encoding="utf8").read()
old_start=a.index('  const manifestFile=result.files.find')
old_end=a.index('  if(failed)return "miss";\n')+len('  if(failed)return "miss";\n')
new='''  const manifestFile=result.files.find(file=>file.path===fleet.manifestName);if(!manifestFile)return "miss";
  const declared=new Map(result.files.map(file=>[file.path,file.digest])),wanted=new Map<string,number>();
  for(const digest of[manifestFile.digest,...result.files.filter(file=>file.path.startsWith("out/")).map(file=>file.digest),...[result.stdout,result.stderr].filter((stream):stream is NonNullable<typeof stream>=>stream!==undefined)])wanted.set(digest.hash,digest.size);
  if(manifestFile.digest.size>fleet.limits.maximumManifestBytes||wanted.size>fleet.limits.maximumBlobs+3||[...wanted.values()].some(size=>size>fleet.limits.maximumBlobBytes))return "miss";
  const bytes=await downloadBlobsV1(unit.client,wanted);if(bytes===undefined)return "miss";
  const manifest=parseRemoteUnitV1(bytes.get(manifestFile.digest.hash)!,{key:unit.key});
  if(JSON.stringify(manifest.platform)!==JSON.stringify(unit.platform)||manifest.slot!==unit.slot)return "miss";
  if(unit.expected&&uncoveredOutputsV1(manifest.outputs.map(output=>output.name),unit.expected).length>0)return "miss";
  for(const output of manifest.outputs){const file=declared.get("out/"+output.name);if(file?.hash!==output.sha256||file.size!==output.bytes)return "miss";}
  for(const [sha,stream] of [[manifest.stdout,result.stdout],[manifest.stderr,result.stderr]] as const){if(sha!==null&&stream?.hash!==sha)return "miss";}
'''
a=a[:old_start]+new+a[old_end:]
helper='''
/** ⚡️ Downloads every blob at once, the largest first so the longest transfer overlaps the others, `fleet.limits.maximumDownloadConcurrency` at a time; `undefined` as soon as one is missing, short or corrupt. The digests come from the action result, so the manifest travels with the outputs and a unit costs one round trip for its result and one for its blobs. */
export async function downloadBlobsV1(client:FleetClientV1,wanted:ReadonlyMap<string,number>):Promise<Map<string,Uint8Array>|undefined>{
 const downloads=[...wanted.entries()].sort((left,right)=>right[1]-left[1]),bytes=new Map<string,Uint8Array>();let next=0,failed=false;
 await Promise.all(Array.from({length:Math.min(fleet.limits.maximumDownloadConcurrency,downloads.length)},async()=>{
  for(;;){const index=next++;if(index>=downloads.length||failed)return;const [sha,size]=downloads[index]!,blob=await client.getBlob(sha,size);if(!blob){failed=true;return;}bytes.set(sha,blob);}
 }));
 return failed?undefined:bytes;
}
'''
a=sub(a,'/** 📥️ Fetches the unit from the fleet cache and installs it',helper.lstrip("\n")+'\n/** 📥️ Fetches the unit from the fleet cache and installs it')
open(S+"new/adopt.ts","w",encoding="utf8").write(a)
f=open(S+"base/fleet.ts",encoding="utf8").read()
f=sub(f,'''export async function fetchFleetUnitV1(unit:FleetUnitV1):Promise<"hit"|"miss"|"fault">{
 const outcome=await installFleetUnitV1(unit);
 if(outcome!=="hit")return outcome;
 try{
  markOfferedV1(unit.root,unit.key);
  await Promise.race([unit.client.claimSlot(unit.key,unit.slot),new Promise<void>(done=>{const wait=setTimeout(done,600);wait.unref?.();})]);
 }catch{}
 return "hit";
}''','''export async function fetchFleetUnitV1(unit:FleetUnitV1):Promise<"hit"|"miss"|"fault">{
 const outcome=await installFleetUnitV1(unit);
 if(outcome!=="hit")return outcome;
 try{
  markOfferedV1(unit.root,unit.key);
  const pending=Promise.race([unit.client.claimSlot(unit.key,unit.slot).then(()=>undefined,()=>undefined),new Promise<void>(done=>{const wait=setTimeout(done,600);wait.unref?.();})]);
  settling.add(pending);void pending.finally(()=>settling.delete(pending));
 }catch{}
 return "hit";
}

const settling=new Set<Promise<void>>();
/** ⏳️ Waits (at most 600 ms, the bound of each claim) for the slot claims that fleet hits started and did not wait for: a hit serves its unit while the fleet is told, and the caller awaits this once before the process ends. */
export async function awaitFleetSettlesV1():Promise<void>{
 if(settling.size>0)await Promise.allSettled([...settling]);
}''')
open(S+"new/fleet.ts","w",encoding="utf8").write(f)
s=open(S+"base/script.ts",encoding="utf8").read()
s=sub(s,'import {fleetSkipReasonV1,holdFleetKeyV1,','import {awaitFleetSettlesV1,fleetSkipReasonV1,holdFleetKeyV1,')
s=sub(s,'countUnitV1(workspaceOf,"remoteHit");return 0;}}catch{}countFallbackV1(settings.root,"hit-unservable");','countUnitV1(workspaceOf,"remoteHit");await awaitFleetSettlesV1();return 0;}}catch{}countFallbackV1(settings.root,"hit-unservable");')
open(S+"new/script.ts","w",encoding="utf8").write(s)
