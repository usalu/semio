
test("a lane image is sized from the measured lanes: a host with less room than the declared need still births one, a roomy host keeps the preferred size, an existing image grows by its plan and a holder makes it wait",async()=>{
 const samples=[5*GiB,6*GiB,6*GiB,7*GiB];
 const poor=world(21);
 expect(await ensureLaneImageV1(poor.root,"p000001",poor.ports,poor.fake)).toEqual({kind:"refused",id:"p000001",detail:"host-free"});
 expect(await ensureLaneImageV1(poor.root,"p000002",{...poor.ports,laneSizes:()=>samples},poor.fake)).toMatchObject({kind:"birth",id:"p000002"});
 expect(nativeImageShardFoldV1(readNativeImageShardLedgerV1(poor.directory)).get("p000002")!.capacityBytes).toBe(13*GiB);
 const roomy=world(400);
 await ensureLaneImageV1(roomy.root,"p000001",{...roomy.ports,laneSizes:()=>samples},roomy.fake);
 expect(nativeImageShardFoldV1(readNativeImageShardLedgerV1(roomy.directory)).get("p000001")!.capacityBytes).toBe(16*GiB);
 expect(laneSizesV1(roomy.root)).toEqual([]);
 const full:LaneImagePortsV1={...roomy.ports,measure:()=>({usedBytes:15*GiB,hostFreeBytes:400*GiB})};
 roomy.calls.length=0;
 expect(await ensureLaneImageV1(roomy.root,"p000001",full,roomy.fake)).toEqual({kind:"grown",id:"p000001",detail:String(30*GiB)});
 const tight:LaneImagePortsV1={...full,measure:()=>({usedBytes:28*GiB,hostFreeBytes:4*GiB})};
 expect(await ensureLaneImageV1(roomy.root,"p000001",tight,roomy.fake)).toEqual({kind:"existing",id:"p000001"});
 expect(nativeImageShardFoldV1(readNativeImageShardLedgerV1(roomy.directory)).get("p000001")!.capacityBytes).toBe(30*GiB);
});
