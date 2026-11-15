#!/usr/bin/env python3
"""q1: journal: repairBlobV1 and the tombstone lifetime. usage: q1-journal.py <module-root>"""
import sys
root=sys.argv[1]
p=root+"/📒️journal/🟦️.ts"
s=open(p,encoding="utf8").read()
anchor="/** 🩹️ What a repair of a segment that was shorter than its head reports"
assert s.count(anchor)==1
new='''/** 🫗️ Puts the right bytes under a blob name: the existing blob is hashed afresh (the per-process memory of verified blobs is not trusted here) and kept when it is intact; otherwise `source`, which must hash to `sha256` while it is copied, replaces it through a staged copy and a rename, so a reader sees the old file or the new one and never a mix. Returns `intact`, `repaired`, or `unstable` (the source changed or hashes differently, nothing was written). */
export async function repairBlobV1(storeRoot:string,originId:string,sha256:string,source:string):Promise<"intact"|"repaired"|"unstable">{
 requireOriginIdV1(originId);
 const target=blobPathV1(storeRoot,sha256),{tmp}=storePathsV1(storeRoot);
 verifiedBlobs.delete(target);
 if(await intact(target,sha256)){await touchBlobV1(target);return "intact";}
 const temporary=stage(tmp,originId);
 try{
  const copied=await inDirectory(storeRoot,tmp,()=>copyHashedV1(source,temporary,{sync:true}));
  if(!copied.stable||copied.sha256!==sha256){await unlink(temporary).catch(()=>{});return "unstable";}
  await inDirectory(storeRoot,dirname(target),()=>renameWithRetry(temporary,target));
  verifiedBlobs.add(target);
  return "repaired";
 }catch(error){await unlink(temporary).catch(()=>{});throw error;}
}

/** ⌛️ How long a delete tombstone is kept in the snapshots, in milliseconds (90 days). A tombstone is what makes an older put lose; once no journal segment can hold such a put and no follower can still lack the delete (a follower further behind than the segment retention must rejoin from a snapshot), it only costs space. A snapshot written at hlc `S` therefore leaves out the deletes older than `S - lifetime`, and a follower that rejoins from it treats a live entry older than that horizon which the snapshot lacks as deleted. */
export const tombstoneLifetimeMillisecondsV1=90*86_400_000;

/** 🕯️ Tells whether a manifest entry is a delete tombstone that a snapshot written at `snapshotHlc` no longer needs. */
export function expiredTombstoneV1(entry:FleetManifestEntryV1,snapshotHlc:string):boolean{
 return entry.operation==="delete"&&parseHlcV1(entry.hlc).ms<parseHlcV1(snapshotHlc).ms-tombstoneLifetimeMillisecondsV1;
}

/** 🦴️ Tells whether an entry hlc is older than the tombstone horizon of a snapshot written at `snapshotHlc`: a live entry that old which the snapshot lacks was deleted, its tombstone has expired. */
export function expiredHorizonV1(entryHlc:string,snapshotHlc:string):boolean{
 return parseHlcV1(entryHlc).ms<parseHlcV1(snapshotHlc).ms-tombstoneLifetimeMillisecondsV1;
}

'''
s=s.replace(anchor,new+anchor)
a2='    if(!isManifestEntryV1(entry))check("ManifestEntry",entry,"Closed fleet snapshot required");\n    text+='
assert s.count(a2)==1
s=s.replace(a2,'    if(!isManifestEntryV1(entry))check("ManifestEntry",entry,"Closed fleet snapshot required");\n    if(expiredTombstoneV1(entry,hlc))continue;\n    text+=')
open(p,"w",encoding="utf8").write(s)
print("ok")
