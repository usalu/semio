#!/usr/bin/env python3
"""q1: journal tests for repairBlobV1 and the tombstone lifetime. usage: q1-journal-tests.py <module-root>"""
import sys
root=sys.argv[1]
p=root+"/📒️journal/🧪️tests/🟦️.ts"
s=open(p,encoding="utf8").read()
imp='import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};\n'
assert s.count(imp)==1
s=s.replace(imp,imp+'import {expiredHorizonV1,expiredTombstoneV1,repairBlobV1,tombstoneLifetimeMillisecondsV1} from "../🟦️.ts";\n')
s=s.rstrip("\n")+'''

describe("blob repair and tombstone lifetime",()=>{
 test("a blob that does not hash to its name is replaced from a source with the right bytes, an intact blob is left alone and a wrong source writes nothing",async()=>{
  const store=join(temporary(),"store"),source=join(temporary(),"source.txt");
  writeFileSync(source,"right bytes");
  const name=sha("right bytes"),target=blobPathV1(store,name);
  expect(await writeBlobV1(store,"alpha",name,source)).toBe(true);
  expect(await repairBlobV1(store,"alpha",name,source)).toBe("intact");
  writeFileSync(target,"rotten bytes");
  expect(await repairBlobV1(store,"alpha",name,source)).toBe("repaired");
  expect(readFileSync(target,"utf8")).toBe("right bytes");
  rmSync(target);
  expect(await repairBlobV1(store,"alpha",name,source)).toBe("repaired");
  expect(readFileSync(target,"utf8")).toBe("right bytes");
  writeFileSync(target,"rotten again");
  const wrong=join(temporary(),"wrong.txt");writeFileSync(wrong,"other bytes");
  expect(await repairBlobV1(store,"alpha",name,wrong)).toBe("unstable");
  expect(readFileSync(target,"utf8")).toBe("rotten again");
  expect(readdirSync(storePathsV1(store).tmp)).toEqual([]);
 });

 test("a snapshot leaves out the deletes older than the lifetime and keeps younger deletes and every live entry",async()=>{
  const store=join(temporary(),"store"),day=86_400_000,now=1_790_000_000_000,manifest=new Map<string,FleetManifestEntryV1>();
  const remove=(age:number):FleetManifestEntryV1=>({hlc:formatHlcV1({ms:now-age,counter:0},"alpha"),operation:"delete"});
  manifest.set("old-gone.txt",remove(100*day));
  manifest.set("recent-gone.txt",remove(10*day));
  foldEventsV1(manifest,[put("alpha",now-200*day,0,"old-live.txt"),put("alpha",now-day,0,"new-live.txt")]);
  const at=formatHlcV1({ms:now,counter:0},"alpha");
  expect(expiredTombstoneV1(manifest.get("old-gone.txt")!,at)).toBe(true);
  expect(expiredTombstoneV1(manifest.get("recent-gone.txt")!,at)).toBe(false);
  expect(expiredTombstoneV1(manifest.get("old-live.txt")!,at)).toBe(false);
  expect(tombstoneLifetimeMillisecondsV1).toBe(90*day);
  expect(expiredHorizonV1(manifest.get("old-live.txt")!.hlc,at)).toBe(true);
  expect(expiredHorizonV1(manifest.get("new-live.txt")!.hlc,at)).toBe(false);
  await writeSnapshotV1(store,"alpha",at,manifest,{});
  const read=await readNewestSnapshotV1(store);
  expect([...read!.manifest.keys()].sort()).toEqual(["new-live.txt","old-live.txt","recent-gone.txt"]);
 });
});
'''
open(p,"w",encoding="utf8").write(s)
print("ok")
