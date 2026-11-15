#!/usr/bin/env python3
"""q6: replica test for the tombstone lifetime. usage: q6-tombstone-tests.py <module-root>"""
import sys
root=sys.argv[1]
p=root+"/🔁️replica/🧪️tests/🟦️.ts"
s=open(p,encoding="utf8").read()
s=s.rstrip("\n")+'''

test("a follower that returns after the tombstone lifetime and the segment retention deletes what the fleet deleted long ago, though the snapshot no longer carries the tombstone, and keeps what is still live",async()=>{
 const day=86_400_000,earlier=()=>Date.now()-100*day,store=temp("store"),a=machine("a",store),b=machine("b",store),machines=[a,b];
 for(const who of machines)who.options={...who.options,now:earlier};
 write(a,".gitignore",fixture.gitignore);await seedV1(a.options);await joinV1(b.options);for(const who of machines)await open(who);
 write(a,"old.txt","x");write(a,"stay.txt","y");
 await settle([a,b]);
 expect(read(b,"old.txt")).toBe("x");
 rmSync(at(a,"old.txt"));await publish(a);
 for(const who of machines){await shut(who);who.options={...who.options,now:undefined};}
 await open(a);
 write(a,"late.txt","z");await publish(a);
 await compactV1(store,"a",0);
 const snapshot=await readNewestSnapshotV1(store);
 expect(snapshot?.manifest.has("old.txt")).toBe(false);
 expect(snapshot?.manifest.get("stay.txt")?.operation).toBe("put");
 rmSync(join(store,"v1","origins","a","segments","000000000000.jsonl"));
 await open(b);
 await follow(b);
 expect(read(b,"old.txt")).toBeUndefined();
 expect(read(b,"stay.txt")).toBe("y");
 expect(read(b,"late.txt")).toBe("z");
 expect(b.replica!.files.manifestHex()).toBe(a.replica!.files.manifestHex());
 for(const who of machines)await shut(who);
},60000);
'''
open(p,"w",encoding="utf8").write(s)
print("ok")
