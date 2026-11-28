
test("storage: the shard images of the compiler partition grow between stages by their plan, never while the partition image, a shard lease or a holder is there",async()=>{
 const GiBv=1073741824,directory=join(fresh(),"r");mkdirSync(directory,{recursive:true});
 appendNativeImageShardEventV1(directory,{type:"born",id:"lane1",key:"lane1",root:0,path:"u/l/lane1",capacityBytes:16*GiBv,atMs:1});
 const rule={enabled:true,headroomPercent:100,hostFreeReservePercent:66,roots:[{prefix:"u/l",pattern:"^[a-z0-9]{1,12}$",needBytes:8*GiBv}]} as never,log:string[]=[];
 const lifecycle={create:async()=>{},attach:async()=>{},detach:async()=>{},grow:async(_:unknown,bytes:number)=>{log.push("grow "+bytes/GiBv);},retire:async()=>{}} as never;
 const measure=(()=>({id:"lane1",volume:"v",capacityBytes:16*GiBv,usedBytes:15*GiBv,allocatedBytes:0,hostFreeBytes:100*GiBv})) as never;
 const held=ports(fresh(),{platform:"darwin",command:async()=>"p4242\ncbun\n"});
 const rows=await growShardImagesV1(held,directory,rule,{lifecycle,measure});
 expect(rows).toHaveLength(1);expect(rows[0]).toMatchObject({id:"r/lane1",state:"skipped",kind:"held"});expect(log).toEqual([]);
 const quiet=ports(fresh(),{platform:"darwin",command:async()=>""}),grown=await growShardImagesV1(quiet,directory,rule,{lifecycle,measure});
 expect(grown).toEqual([{id:"r/lane1",state:"grown",addedBytes:14*GiBv}]);expect(log).toEqual(["grow 30"]);
 expect(await growShardImagesV1(ports(fresh(),{platform:"linux"}),directory,rule,{lifecycle,measure})).toEqual([]);
 const empty=join(fresh(),"r");mkdirSync(empty,{recursive:true});expect(await growShardImagesV1(quiet,empty,rule,{lifecycle,measure})).toEqual([]);
 const tight=await growShardImagesV1(quiet,directory,rule,{lifecycle,measure:(()=>({id:"lane1",volume:"v",capacityBytes:30*GiBv,usedBytes:29*GiBv,allocatedBytes:0,hostFreeBytes:4*GiBv})) as never});
 expect(tight[0]).toMatchObject({state:"skipped",kind:"unfit"});expect((tight[0] as {note:string}).note).toContain("must keep");expect((tight[0] as {note:string}).note).toContain("muss");
});
