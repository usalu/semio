#!/usr/bin/env python3
"""q2: replica tests for the blob repair. usage: q2-blobs-tests.py <module-root>"""
import sys,re
root=sys.argv[1]
p=root+"/🔁️replica/🧪️tests/🟦️.ts"
s=open(p,encoding="utf8").read()
m=re.search(r'import \{checkpointReplicaV1,',s)
assert m
s=s.replace("import {checkpointReplicaV1,","import {tendBlobsV1,checkpointReplicaV1,",1)
imp='import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};\n'
assert s.count(imp)==1
s=s.replace(imp,imp+'import {readWantsV1} from "../🩹️blobs/🟦️.ts";\n')
s=s.rstrip("\n")+'''

const shaOf=(text:string)=>createHash("sha256").update(text).digest("hex");
async function newcomer(name:string,store:string):Promise<Machine>{const who=machine(name,store);await joinV1(who.options);await open(who);await follow(who);return who;}

test("a blob that does not hash to its name is never materialised, is asked for, and is put back by a machine that still holds the content",async()=>{
 const {store,machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine],sha=shaOf("right bytes");
 write(a,"docs/f.txt","right bytes");
 await settle([a,b]);
 expect(read(b,"docs/f.txt")).toBe("right bytes");
 writeFileSync(blobPathV1(store,sha),"rotten bytes");
 const c=await newcomer("c",store);
 expect(read(c,"docs/f.txt")).toBeUndefined();
 expect(c.events.some(event=>event.event==="blob-missing"&&event.reason==="blob-corrupt")).toBe(true);
 expect(await tendBlobsV1(c.replica!)).toMatchObject({announced:1,repaired:0});
 expect((await readWantsV1(store,"c"))?.wants).toEqual([{sha256:sha,path:"docs/f.txt"}]);
 expect(c.replica!.failures.summary().map(item=>item.kind)).toContain("blob-wanted");
 expect(await tendBlobsV1(a.replica!)).toMatchObject({repaired:1});
 expect(readFileSync(blobPathV1(store,sha),"utf8")).toBe("right bytes");
 expect(a.events.some(event=>event.event==="blob-repaired"&&event.path==="docs/f.txt")).toBe(true);
 await waitFor(async()=>{await follow(c);return read(c,"docs/f.txt")==="right bytes";});
 expect(await tendBlobsV1(c.replica!)).toMatchObject({announced:0});
 expect((await readWantsV1(store,"c"))?.wants).toEqual([]);
 expect(await tendBlobsV1(b.replica!)).toMatchObject({repaired:0});
 for(const who of [a,b,c])await shut(who);
},60000);

test("a blob that a live entry references and the store lacks is put back, a holder whose file differs from the entry writes nothing, and a blob nobody can supply stays wanted",async()=>{
 const {store,machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine],sha=shaOf("kept bytes");
 write(a,"keep.txt","kept bytes");
 await settle([a,b]);
 rmSync(blobPathV1(store,sha));
 write(b,"keep.txt","edited by b");
 const c=await newcomer("c",store);
 expect(read(c,"keep.txt")).toBeUndefined();
 await tendBlobsV1(c.replica!);
 expect(await tendBlobsV1(b.replica!)).toMatchObject({repaired:0});
 expect(existsSync(blobPathV1(store,sha))).toBe(false);
 expect(c.replica!.failures.summary().map(item=>item.kind)).toContain("blob-wanted");
 expect(await tendBlobsV1(a.replica!)).toMatchObject({repaired:1});
 expect(readFileSync(blobPathV1(store,sha),"utf8")).toBe("kept bytes");
 await waitFor(async()=>{await follow(c);return read(c,"keep.txt")==="kept bytes";});
 for(const who of [a,b,c])await shut(who);
},60000);
'''
open(p,"w",encoding="utf8").write(s)
print("ok")
