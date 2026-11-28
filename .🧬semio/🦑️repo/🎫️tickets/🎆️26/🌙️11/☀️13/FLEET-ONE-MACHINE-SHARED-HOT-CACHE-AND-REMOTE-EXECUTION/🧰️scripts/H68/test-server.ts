test("provenance: two builds of one unit whose compiler diagnostics differ only in their rendering are the same result and keep the first, another diagnostic is a divergence",async()=>{
 const fleet=await fleetServer(),key=sha("stream unit"),put=(root:string,path:string,bytes:Uint8Array<ArrayBuffer>|string)=>fleet.signed(root)(`${fleet.base}/${path}`,{method:"PUT",body:bytes});
 const line=(message:string,rendered:string)=>JSON.stringify({"$message_type":"diagnostic",message,level:"warning",rendered,children:[{message:"note",rendered:null}]})+"\n";
 const result=async(root:string,stderr:string)=>{
  const hash=sha(stderr);expect((await put(root,`cas/${hash}`,stderr)).status).toBe(200);
  return put(root,`ac/${key}`,encodeProtobufV1(REAPI_V1.actionResult,{output_files:[{path:"unit.json",digest:{hash:sha("m"+stderr),size_bytes:3}},{path:"out/o0",digest:{hash:sha("rlib"),size_bytes:3}}],stderr_digest:{hash,size_bytes:Buffer.byteLength(stderr)},exit_code:0}));
 };
 expect((await result(fleet.a,line("unused","\u001b[1mwarning\u001b[0m: unused\n  --> long\n"))).status).toBe(200);
 expect((await result(fleet.b,line("unused","a.rs:1:1: warning: unused\n"))).status,"another rendering of the same diagnostic").toBe(200);
 expect((await result(fleet.b,line("unused different","a.rs:1:1: warning: unused different\n"))).status,"another diagnostic").toBe(409);
 const status=await (await fleet.signed(fleet.a)(`http://127.0.0.1:${fleet.running.port}/status`)).json() as {divergences:{count:number;advisory:Record<string,number>}};
 expect(status.divergences.count).toBe(1);expect(status.divergences.advisory["stream-rendering"]).toBe(1);
});
