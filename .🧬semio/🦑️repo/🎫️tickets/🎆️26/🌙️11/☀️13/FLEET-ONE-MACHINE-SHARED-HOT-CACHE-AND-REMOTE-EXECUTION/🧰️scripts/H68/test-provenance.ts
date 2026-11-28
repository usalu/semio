
test("two captured streams with the same diagnostics in another rendering, order or long-type number are the same result, another or additional diagnostic is not, equal to the independent Python computation on every fixture case",async()=>{
 for(const row of fixture.streams){
  expect(provenanceStreamIdentityV1(row.kept)===provenanceStreamIdentityV1(row.refused),row.name).toBe(row.equal);
  const rows=[{path:"⟨stderr⟩",hash:"k"}],other=[{path:"⟨stderr⟩",hash:"r"}],blobs:Record<string,string>={k:row.kept,r:row.refused};
  expect(await provenanceStreamsEquivalentV1(["⟨stderr⟩"],rows,other,async hash=>new TextEncoder().encode(blobs[hash]!)),row.name).toBe(row.equal);
 }
 const rows=[{path:"⟨stderr⟩",hash:"k"},{path:"out/x.rlib",hash:"a"}],other=[{path:"⟨stderr⟩",hash:"r"},{path:"out/x.rlib",hash:"b"}];
 expect(await provenanceStreamsEquivalentV1(["out/x.rlib"],rows,other,async()=>new Uint8Array()),"an output file differs").toBe(false);
 expect(await provenanceStreamsEquivalentV1(["⟨stderr⟩","out/x.rlib"],rows,other,async()=>new Uint8Array()),"a stream and an output file differ").toBe(false);
 expect(await provenanceStreamsEquivalentV1(["⟨stderr⟩"],rows,other,async()=>undefined),"a blob is gone").toBe(false);
 expect(await provenanceStreamsEquivalentV1([],rows,other,async()=>new Uint8Array()),"nothing differs").toBe(false);
});
