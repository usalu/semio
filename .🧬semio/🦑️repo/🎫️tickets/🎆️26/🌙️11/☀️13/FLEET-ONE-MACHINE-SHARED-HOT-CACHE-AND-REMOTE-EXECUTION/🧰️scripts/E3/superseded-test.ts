
test("a graph record replaces the older record of its key and the replaced one is kept as evidence and counted as superseded, while a unit, a closure and an owner pool wave stay first-writer-wins",async()=>{
 const store=directory("group3"),a=machine(store,"g1-m01"),b=machine(store,"g1-m02"),server=machine(store,"g1-server");
 for(const root of[a,b,server])expect((await syncFleetIdentityV1({root})).error).toBeUndefined();
 const running=await startRemoteCacheServerV1({directory:directory("cache3"),host:"127.0.0.1",port:0,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"semio",tokens:{},validation:"structure",fleet:{storeRoot:store,group:"g1",networks:["10.77.0.0/16"]}});servers.push(running);
 const base=`http://127.0.0.1:${running.port}/semio`,call=(root:string)=>fleetSignedFetchV1({root,audience:"cache"}),put=(root:string,key:string,bytes:Uint8Array)=>call(root)(`${base}/ac/${key}`,{method:"PUT",body:bytes});
 const record=sha("record-key"),older=named(["record.br","leaf/a.json"],"older"),newer=named(["record.br","leaf/a.json"],"newer");
 expect((await put(a,record,older)).status).toBe(200);expect((await put(b,record,newer)).status).toBe(200);
 expect(Buffer.compare(Buffer.from(await (await call(a)(`${base}/ac/${record}`)).arrayBuffer()),Buffer.from(newer))).toBe(0);
 const wave=sha("wave-key");expect((await put(a,wave,named(["head.br"],"w1"))).status).toBe(200);expect((await put(b,wave,named(["head.br"],"w2"))).status).toBe(409);
 let status=await (await call(a)(`http://127.0.0.1:${running.port}/status`)).json() as {divergences:{count:number;byClass:Record<string,number>;superseded:Record<string,number>;evidence:{count:number}}};
 for(let wait=0;wait<50&&status.divergences.evidence.count<2;wait++){await new Promise(done=>setTimeout(done,40));status=await (await call(a)(`http://127.0.0.1:${running.port}/status`)).json() as typeof status;}
 expect(status.divergences.count).toBe(1);expect(status.divergences.byClass).toEqual({"owner-pool":1});expect(status.divergences.superseded).toEqual({"project-inference":1});
 const listed=await (await call(a)(`http://127.0.0.1:${running.port}/evidence`)).json() as {evidence:{id:string;outcome:string;sort:string;keptOrigin:string;refusedOrigin:string}[]};
 const replaced=listed.evidence.find(row=>row.outcome==="superseded")!;
 expect(replaced).toMatchObject({sort:"project-inference",keptOrigin:"g1-m01",refusedOrigin:"g1-m02"});expect(listed.evidence.find(row=>row.outcome==="refused")!.sort).toBe("owner-pool");
 expect(Buffer.compare(Buffer.from(await (await call(a)(`${base.replace("/semio","")}/evidence/${replaced.id}/kept.ac`)).arrayBuffer()),Buffer.from(older))).toBe(0);
});
