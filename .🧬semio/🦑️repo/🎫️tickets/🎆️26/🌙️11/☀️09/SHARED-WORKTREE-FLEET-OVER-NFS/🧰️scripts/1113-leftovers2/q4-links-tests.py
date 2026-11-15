#!/usr/bin/env python3
"""q4: replica test for a volume without symbolic links (X16). usage: q4-links-tests.py <module-root>"""
import sys
root=sys.argv[1]
p=root+"/🔁️replica/🧪️tests/🟦️.ts"
s=open(p,encoding="utf8").read()
s=s.rstrip("\n")+'''

posixTest("a machine whose volume cannot hold symbolic links keeps the link in its manifest, publishes no delete for it, survives a restart and a repair, and reports the capability difference (simulated)",async()=>{
 const {store,machines}=await fleet(["a"]),[a]=machines as [Machine];
 write(a,"real.txt","target");symlinkSync("real.txt",at(a,"alias.txt"));
 await publish(a);
 const w=machine("w",store);w.options={...w.options,symlinks:false};
 await joinV1(w.options,undefined,event=>w.events.push({...event}));await open(w);await follow(w);
 expect(read(w,"real.txt")).toBe("target");
 expect(existsSync(at(w,"alias.txt"))).toBe(false);
 expect(w.events.some(event=>event.event==="skipped"&&event.path==="alias.txt"&&event.reason==="symlink-unsupported")).toBe(true);
 const link=w.replica!.book.find("alias.txt")!;
 expect(w.replica!.manifest.operationAt(link)).toBe("link");
 expect((await publishOnceV1(w.replica!)).deleted).toBe(0);
 await shut(w);await open(w);
 expect((await publishOnceV1(w.replica!)).deleted).toBe(0);
 expect(await repairV1(w.replica!)).toBe(0);
 await follow(w);
 expect(existsSync(at(w,"alias.txt"))).toBe(false);
 expect(w.replica!.manifest.operationAt(link)).toBe("link");
 expect((await readEventsV1(store,"w",undefined)).events.filter(event=>event.operation==="delete")).toEqual([]);
 await follow(a);
 expect(readlinkSync(at(a,"alias.txt"))).toBe("real.txt");
 expect((await verifyV1(w.options)).drift.filter(item=>item.path==="alias.txt")).toEqual([]);
 await shut(w);await shut(a);
},60000);
'''
open(p,"w",encoding="utf8").write(s)
print("ok")
