const closureWorkerV1=fileURLToPath(new URL("../../../👷️closure-read-worker/🟨️.mjs",import.meta.url));
/** 🔬️ Asks the REAL closure read child about one leaf and turns its failure packet into the refusal the child port throws. */
const realRefusalV1=(root:string,file:{path:string;stamp:string;kind:string;bytes:number}):Error=>{
 const input=JSON.stringify({version:1,maximumReplyBytes:16764800})+"\n"+JSON.stringify({files:[file],imageRoot:root,remainingBytes:1<<30,root:process.cwd()})+"\nnull\n";
 const run=spawnSync("node",["--input-type=module","-e","const module=await import(process.argv[1]);await module.runCapturedBuildSourceV1();",pathToFileURL(closureWorkerV1).href],{input,cwd:root,encoding:"utf8"});
 return Object.assign(Error("Original closure read child refused"),{closureReadFailure:{version:1,worker:JSON.parse(run.stdout.split("\n").find(Boolean)!),terminal:{status:run.status,signal:null}}});
};
/** 🧮️ The independent oracle: Python's zlib says whether the bytes are a complete gzip or zlib stream at all. */
const pythonInflatesV1=(path:string):boolean=>{
 const run=Bun.spawnSync(["python3","-I","-c","import sys,zlib\nd=zlib.decompressobj(47)\ntry:\n d.decompress(open(sys.argv[1],'rb').read());print(int(d.eof))\nexcept zlib.error:print(0)",path]);
 return run.stdout.toString().trim()==="1";
};

test("a settled leaf the real closure read child cannot inflate names nothing, a young one and any other refusal keep every inventory",async()=>{
 const build=(age:number)=>{const image=closureImage(),leaf=join(image.np,"e".repeat(24)+".json");writeFileSync(leaf,Buffer.from([0]));const time=(Date.now()-age*3600000)/1000;utimesSync(leaf,time,time);return{...image,leaf};};
 const{root,leaf}=build(2);expect(pythonInflatesV1(leaf),"the independent decoder agrees that the leaf is no complete stream").toBe(false);
 const refusing=(error:()=>Error|undefined):ReadPortV1=>async(requests,accept)=>{for(const file of requests){if(file.path.endsWith("e".repeat(24)+".json")){const refusal=error();if(refusal)throw refusal;}await closureRead()([file],accept);}};
 const real=(image:{root:string}):ReadPortV1=>async(requests,accept)=>{for(const file of requests){if(file.path.endsWith("e".repeat(24)+".json"))throw realRefusalV1(image.root,file as never);await closureRead()([file],accept);}};
 const dry=await scan.supersedeBuildGraphClosuresV1(root,{lease:async()=>({release(){}}),read:real({root})},control(),{dryRun:true,graceMilliseconds:3600000});
 expect(dry,"the settled undecodable leaf does not stall the sweep").toMatchObject({unvouched:0,candidates:2,removed:2});expect(closureNames(root).length).toBe(6);
 const report=await scan.supersedeBuildGraphClosuresV1(root,{lease:async()=>({release(){}}),read:real({root})},control(),{graceMilliseconds:3600000});
 expect(report).toMatchObject({unvouched:0,removed:2,cancelled:false});expect(closureNames(root)).toEqual(["build-input-closures/"+hex("a")+".json","build-input-closures/"+hex("b")+".json","build-input-closures/"+hex("e")+".json","build-lock-closures/"+hex("d")+".json"]);
 const young=build(0);expect(await scan.supersedeBuildGraphClosuresV1(young.root,{lease:async()=>({release(){}}),read:real(young)},control(),{graceMilliseconds:3600000})).toMatchObject({unvouched:1,removed:0});
 const probe=realRefusalV1(root,{path:leaf,stamp:[(row=>[row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].join(":"))(lstatSync(leaf,{bigint:true}))].join(""),kind:"inference",bytes:1}) as Error&{closureReadFailure:{worker:{failure:{code:string};closurePayloadRefusal:{check:string}}}};
 expect(probe.closureReadFailure.worker.failure.code).toBe("Z_BUF_ERROR");expect(probe.closureReadFailure.worker.closurePayloadRefusal.check).toBe("inflation");
 const other=build(2);expect(await scan.supersedeBuildGraphClosuresV1(other.root,{lease:async()=>({release(){}}),read:refusing(()=>Object.assign(Error("refused"),{closureReadFailure:{worker:{failure:{code:"EIO",message:"read failed"},closurePayloadRefusal:null}}}))},control(),{graceMilliseconds:3600000})).toMatchObject({unvouched:1,removed:0});expect(closureNames(other.root).length).toBe(6);
 const memory=build(2);expect(await scan.supersedeBuildGraphClosuresV1(memory.root,{lease:async()=>({release(){}}),read:refusing(()=>Object.assign(Error("refused"),{closureReadFailure:{worker:{failure:{code:"Z_MEM_ERROR",message:"out of memory"},closurePayloadRefusal:{check:"inflation"}}}}))},control(),{graceMilliseconds:3600000})).toMatchObject({unvouched:1,removed:0});
 const budget=build(2);expect(await scan.supersedeBuildGraphClosuresV1(budget.root,{lease:async()=>({release(){}}),read:refusing(()=>Object.assign(Error("refused"),{closureReadFailure:{worker:{failure:{code:"ERR_BUFFER_TOO_LARGE",message:"too large"},closurePayloadRefusal:{check:"inflation"}}}}))},control(),{graceMilliseconds:3600000})).toMatchObject({unvouched:1,removed:0});
});

