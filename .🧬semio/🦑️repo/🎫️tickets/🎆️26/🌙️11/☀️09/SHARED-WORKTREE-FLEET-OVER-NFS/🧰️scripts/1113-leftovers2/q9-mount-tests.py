#!/usr/bin/env python3
"""q9: replica test: a daemon that starts before its store is mounted (X20). usage: q9-mount-tests.py <module-root>"""
import sys
root=sys.argv[1]
p=root+"/🔁️replica/🧪️tests/🟦️.ts"
s=open(p,encoding="utf8").read()
s=s.rstrip("\n")+'''

test("a daemon that starts before its store is mounted waits for the mount, says so once and starts working when the store appears, instead of dying into a restart loop",async()=>{
 const {store,machines}=await fleet(["a"]),[a]=machines as [Machine];
 await shut(a);
 renameSync(join(store,"v1"),join(store,"away"));
 const controller=new AbortController(),events:ReplicaEventV1[]=[];
 let failure:unknown;
 const running=runV1(a.options,controller.signal,event=>events.push(event)).catch(error=>{failure=error;});
 await Bun.sleep(400);
 expect(failure).toBeUndefined();
 expect(events.filter(event=>event.event==="store-unreachable"&&event.reason==="waiting-for-mount")).toHaveLength(1);
 write(a,"early.txt","written before the mount");
 renameSync(join(store,"away"),join(store,"v1"));
 await until(()=>events.some(event=>event.event==="published"));
 expect(events.some(event=>event.event==="store-reachable")).toBe(true);
 expect(events.filter(event=>event.event==="store-unreachable")).toHaveLength(1);
 controller.abort();await running;
 expect(failure).toBeUndefined();
 expect(readFileSync(join(store,"v1","origins","a","segments","000000000000.jsonl"),"utf8")).toContain("early.txt");
},60000);
'''
open(p,"w",encoding="utf8").write(s)
print("ok")
