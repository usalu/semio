#!/usr/bin/env python3
"""q5: replica tests for case-only renames: convergence in separate rounds and the repair (X17). usage: q5-case-tests.py <module-root>"""
import sys
root=sys.argv[1]
p=root+"/🔁️replica/🧪️tests/🟦️.ts"
s=open(p,encoding="utf8").read()
s=s.replace('import {blobPathV1,foldEventsV1,','import {appendEventsV1,writeBlobV1,blobPathV1,foldEventsV1,',1)
s=s.rstrip("\n")+'''

test("a tracked file renamed only by case converges on every machine also when the put and the delete reach the follower in separate rounds, with equal and with changed content",async()=>{
 const {machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine],docs=(who:Machine)=>Object.fromEntries(Object.entries(shared(who)).filter(([path])=>path.startsWith("docs/")));
 write(a,"docs/readme.txt","one");
 await settle([a,b]);
 renameSync(at(a,"docs/readme.txt"),at(a,"docs/README.txt"));
 await publishOnceV1(a.replica!,undefined,["docs/README.txt"]);
 await follow(b);
 await publishOnceV1(a.replica!,undefined,["docs/readme.txt"]);
 await follow(b);
 await settle([a,b]);
 expect(docs(a)).toEqual({"docs/README.txt":"one"});
 expect(docs(b)).toEqual({"docs/README.txt":"one"});
 renameSync(at(a,"docs/README.txt"),at(a,"docs/Readme.txt"));
 write(a,"docs/Readme.txt","two");
 await publishOnceV1(a.replica!,undefined,["docs/Readme.txt"]);
 await follow(b);
 await publishOnceV1(a.replica!,undefined,["docs/README.txt"]);
 await follow(b);
 await settle([a,b]);
 expect(docs(a)).toEqual({"docs/Readme.txt":"two"});
 expect(docs(b)).toEqual({"docs/Readme.txt":"two"});
 expect(oracleCompare(machines[0]!.options.storeRoot,b).report.ok).toBe(true);
 for(const who of machines)await shut(who);
},60000);

test("a case-only rename whose put reaches a follower one round before its delete leaves the file under the new spelling (the volume keeps the old spelling when a file is written over it)",async()=>{
 const {store,machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine],listing=()=>Object.keys(shared(b)).filter(path=>path.startsWith("docs/"));
 write(a,"docs/readme.txt","one");
 await settle([a,b]);
 const x=join(a.options.worktree,"x-source.txt");writeFileSync(x,"two");
 await writeBlobV1(store,"a",shaOf("two"),x);
 const ms=Date.now()+5000,stamp=(counter:number)=>formatHlcV1({ms,counter},"a");
 const put:FleetEventV1={version:1,hlc:stamp(0),operation:"put",path:"docs/README.txt",sha256:shaOf("two"),bytes:3,executable:false,base:null};
 const gone:FleetEventV1={version:1,hlc:stamp(1),operation:"delete",path:"docs/readme.txt",base:null};
 await appendEventsV1(store,"a",[put]);
 await follow(b);
 await appendEventsV1(store,"a",[gone]);
 await follow(b);
 expect(listing()).toEqual(["docs/README.txt"]);
 expect(read(b,"docs/README.txt")).toBe("two");
 for(const who of machines)await shut(who);
},60000);

test("repair finds a file whose name on disk differs from its manifest entry only by case and has it respelled, on a volume that ignores case",async()=>{
 const {machines}=await fleet(["a","b"]),[a,b]=machines as [Machine,Machine];
 b.options={...b.options,caseInsensitive:true};await shut(b);await open(b);
 write(a,"docs/readme.txt","one");
 await settle([a,b]);
 expect(Object.keys(shared(b)).filter(path=>path.startsWith("docs/"))).toEqual(["docs/readme.txt"]);
 renameSync(at(b,"docs/readme.txt"),at(b,"docs/tmp-case.txt"));
 renameSync(at(b,"docs/tmp-case.txt"),at(b,"docs/README.txt"));
 const mismatch=Object.keys(shared(b)).filter(path=>path.startsWith("docs/"));
 expect(mismatch).toEqual(["docs/README.txt"]);
 expect(await repairV1(b.replica!)).toBeGreaterThan(0);
 await follow(b);
 expect(Object.keys(shared(b)).filter(path=>path.startsWith("docs/"))).toEqual(["docs/readme.txt"]);
 for(const who of machines)await shut(who);
},60000);
'''
open(p,"w",encoding="utf8").write(s)
print("ok")
