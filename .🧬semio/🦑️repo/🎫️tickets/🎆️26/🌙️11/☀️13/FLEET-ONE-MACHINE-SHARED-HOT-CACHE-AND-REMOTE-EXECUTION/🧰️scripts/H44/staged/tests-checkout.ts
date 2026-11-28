
const memoryFor=(chunkBytes:readonly Uint8Array[],nonce:string,origin="g1-m06")=>{
 const memory=new Memory();
 let offset=0;
 chunkBytes.forEach((data,index)=>{
  const chunk:CheckoutTestChunkV1={version:1,nonce,index,offset,origin,data:Buffer.from(data).toString("base64"),last:index===chunkBytes.length-1};
  offset+=data.length;
  memory.results.set(checkoutTestChunkKeyV1(nonce,index),encodeProtobufV1(REAPI_V1.actionResult,{stderr_raw:encodeCheckoutTestChunkV1(chunk),exit_code:0}));
 });
 return memory;
};
const NONCE="a".repeat(32);

test("H44: the live output pieces, the measured durations and the policy numbers satisfy their closed schemas under both validators and both reject the same mutations",()=>{
 const chunk={version:1,nonce:NONCE,index:0,offset:0,origin:"g1-m06",data:Buffer.from("héllo\n").toString("base64"),last:false},measures={version:1,files:{"a/b.ts":{local:{seconds:12.5,at:1700000000000},remote:{seconds:3.1,at:1700000001000}},"c.ts":{}}};
 for(const [name,value,mutations] of [["Chunk",chunk,{extra:(v:any)=>{v.extra=1;},badNonce:(v:any)=>{v.nonce="x";},negativeIndex:(v:any)=>{v.index=-1;},badOffset:(v:any)=>{v.offset=1.5;},noOrigin:(v:any)=>{delete v.origin;},badData:(v:any)=>{v.data="***";},lastText:(v:any)=>{v.last="yes";},noLast:(v:any)=>{delete v.last;}}],["Measures",measures,{extra:(v:any)=>{v.extra=1;},noFiles:(v:any)=>{delete v.files;},negative:(v:any)=>{v.files["a/b.ts"].local.seconds=-1;},noAt:(v:any)=>{delete v.files["a/b.ts"].remote.at;},unknownSide:(v:any)=>{v.files["c.ts"].other={seconds:1,at:1};}}]] as const){
  const check=validator(name);
  expect(check(value),name+JSON.stringify(check.errors?.[0])).toBe(true);
  expect(checkoutTestProblemV1(name,value),name).toBeUndefined();
  for(const [mutation,mutate] of Object.entries(mutations)){
   const copy=JSON.parse(JSON.stringify(value));mutate(copy);
   expect(check(copy),name+" "+mutation+" ajv").toBe(false);
   expect(checkoutTestProblemV1(name,copy),name+" "+mutation+" in-repo").toBeDefined();
  }
 }
 const policy=validator("Policy");
 for(const [name,mutate] of Object.entries({noLoad:(v:any)=>{delete v.load;},noStream:(v:any)=>{delete v.stream;},badRatio:(v:any)=>{v.load.remoteWorthRatio=2;},zeroParallel:(v:any)=>{v.limits.maximumParallel=0;},zeroChunk:(v:any)=>{v.stream.maximumChunkBytes=0;}})){
  const copy=JSON.parse(JSON.stringify(CHECKOUT_TEST_POLICY_V1));mutate(copy);
  expect(policy(copy),name+" ajv").toBe(false);
  expect(checkoutTestProblemV1("Policy",copy),name+" in-repo").toBeDefined();
 }
 expect(CHECKOUT_TEST_POLICY_V1.wait).toMatchObject({retryMilliseconds:8000,pauseMilliseconds:3000});
});

test("H44: the load verdict follows the language-neutral vectors and an independent Python implementation, and the vectors name the policy they were made for",()=>{
 expect(vectors.policy).toEqual(CHECKOUT_TEST_POLICY_V1.load);
 const now=1_000_000_000,oracle=python({load:vectors.load,policy:vectors.policy,gateLoad:vectors.gateLoad});
 vectors.load.forEach((row,index)=>{
  const side=(timing:{seconds:number;ageSeconds:number}|null)=>timing===null?undefined:{seconds:timing.seconds,at:now-timing.ageSeconds*1000};
  const measure=row.local===null&&row.remote===null?undefined:{...side(row.local)===undefined?{}:{local:side(row.local)!},...side(row.remote)===undefined?{}:{remote:side(row.remote)!}};
  expect(checkoutTestLoadVerdictV1({loadPerCore:row.loadPerCore,measure,now}),row.name).toBe(row.verdict as "offload"|"keep");
  expect(oracle.load[index],row.name+" python").toBe(row.verdict);
 });
 expect(checkoutTestLoadPerCoreV1([24,10,5],8)).toBe(3);
 expect(checkoutTestLoadPerCoreV1([0,0,0],8)).toBe(0);
 expect(checkoutTestLoadPerCoreV1([5],0)).toBe(5);
});

test("H44: the gate asks the CPU load only when the host admits, and a saturated host offers the file with the reason host-load",async()=>{
 const oracle=python({gate:vectors.gateLoad,policy:vectors.policy});
 for(const [index,row] of vectors.gateLoad.entries()){
  const asked:string[]=[],answer=await checkoutTestGateV1({mode:row.mode as "auto"|"never"|"always",host:async()=>{asked.push("host");return row.host as "admit"|"wait";},idle:async()=>{asked.push("idle");return row.idle;},load:async()=>{asked.push("load");return row.load as "offload"|"keep";}});
  expect(answer,row.name).toEqual({offload:row.offload,reason:row.reason as never});
  expect(asked,row.name).toEqual(row.asked);
  expect(oracle.gate[index],row.name+" python").toEqual({offload:row.offload,reason:row.reason,asked:row.asked});
 }
});

test("H44: measured durations are recorded atomically per file and side, bounded to the newest files, and a damaged document reads as empty",()=>{
 const root=temporary();
 expect(readCheckoutTestMeasuresV1(root)).toEqual({version:1,files:{}});
 recordCheckoutTestMeasureV1(root,"a.ts","local",100.04,1000);
 recordCheckoutTestMeasureV1(root,"a.ts","remote",14.46,2000);
 recordCheckoutTestMeasureV1(root,"b.ts","local",1,3000);
 expect(readCheckoutTestMeasuresV1(root).files).toEqual({"a.ts":{local:{seconds:100,at:1000},remote:{seconds:14.5,at:2000}},"b.ts":{local:{seconds:1,at:3000}}});
 expect(checkoutTestProblemV1("Measures",readCheckoutTestMeasuresV1(root))).toBeUndefined();
 for(let index=0;index<CHECKOUT_TEST_POLICY_V1.load.maximumFiles+5;index++)recordCheckoutTestMeasureV1(root,`many/${index}.ts`,"local",1,10_000+index);
 const kept=Object.keys(readCheckoutTestMeasuresV1(root).files);
 expect(kept).toHaveLength(CHECKOUT_TEST_POLICY_V1.load.maximumFiles);
 expect(kept).not.toContain("a.ts");
 expect(kept).toContain(`many/${CHECKOUT_TEST_POLICY_V1.load.maximumFiles+4}.ts`);
 writeFileSync(join(root,CHECKOUT_TEST_POLICY_V1.load.file),"{broken");
 expect(readCheckoutTestMeasuresV1(root)).toEqual({version:1,files:{}});
 recordCheckoutTestMeasureV1(root,"x.ts","remote",2,5);
 expect(Object.keys(readCheckoutTestMeasuresV1(root).files)).toEqual(["x.ts"]);
 expect(readdirSync(root).filter(name=>name!==CHECKOUT_TEST_POLICY_V1.load.file)).toEqual([]);
});

test("H44: a piece of live output is keyed by nonce and index, round-trips, and anything else decodes to nothing",()=>{
 const chunk:CheckoutTestChunkV1={version:1,nonce:NONCE,index:3,offset:30,origin:"g1-m06",data:Buffer.from("abc").toString("base64"),last:true};
 expect(decodeCheckoutTestChunkV1(encodeCheckoutTestChunkV1(chunk))).toEqual(chunk);
 expect(checkoutTestChunkKeyV1(NONCE,3)).toBe(sha(`${CHECKOUT_TEST_POLICY_V1.stream.key}/${NONCE}/3`));
 expect(checkoutTestChunkKeyV1(NONCE,3)).not.toBe(checkoutTestChunkKeyV1(NONCE,4));
 expect(checkoutTestChunkKeyV1(NONCE,3)).not.toBe(checkoutTestChunkKeyV1("b".repeat(32),3));
 expect(()=>encodeCheckoutTestChunkV1({...chunk,index:-1})).toThrow();
 for(const bytes of [undefined,new Uint8Array(),new TextEncoder().encode("{"),new TextEncoder().encode(JSON.stringify({...chunk,extra:1})),new Uint8Array(CHECKOUT_TEST_POLICY_V1.stream.maximumChunkBytes*2+1).fill(32)])expect(decodeCheckoutTestChunkV1(bytes)).toBeUndefined();
});

test("H44: the chunk writer keeps order and offsets, cuts large writes at the piece size, flushes on a timer, ends with a last piece, and stops quietly when a write fails or the pieces are used up",async()=>{
 const pieces:CheckoutTestChunkV1[]=[],writer=createCheckoutTestChunkWriterV1({nonce:NONCE,origin:"g1-m06",flushMilliseconds:20,maximumChunkBytes:8,write:async(_,chunk)=>{pieces.push(chunk);}});
 writer.push(new TextEncoder().encode("ab"));
 expect(pieces).toHaveLength(0);
 await new Promise(done=>setTimeout(done,80));
 expect(pieces.map(piece=>Buffer.from(piece.data,"base64").toString())).toEqual(["ab"]);
 writer.push(new TextEncoder().encode("0123456789ABCDEF!"));
 writer.push(new TextEncoder().encode("xyz"));
 await writer.close();
 const text=pieces.map(piece=>Buffer.from(piece.data,"base64").toString()).join("");
 expect(text).toBe("ab0123456789ABCDEF!xyz");
 expect(pieces.map(piece=>piece.index)).toEqual(pieces.map((_,index)=>index));
 let offset=0;
 for(const piece of pieces){expect(piece.offset).toBe(offset);offset+=Buffer.from(piece.data,"base64").length;expect(Buffer.from(piece.data,"base64").length).toBeLessThanOrEqual(8);expect(piece.origin).toBe("g1-m06");}
 expect(pieces.filter(piece=>piece.last)).toHaveLength(1);
 expect(pieces[pieces.length-1]!.last).toBe(true);
 writer.push(new TextEncoder().encode("late"));
 await writer.close();
 expect(pieces.map(piece=>Buffer.from(piece.data,"base64").toString()).join("")).toBe(text);
 const failing:number[]=[],broken=createCheckoutTestChunkWriterV1({nonce:NONCE,origin:"o",flushMilliseconds:1,maximumChunkBytes:2,write:async index=>{failing.push(index);throw Error("store down");}});
 broken.push(new TextEncoder().encode("abcdef"));
 await broken.close();
 expect(failing).toEqual([0]);
 const few:CheckoutTestChunkV1[]=[],limited=createCheckoutTestChunkWriterV1({nonce:NONCE,origin:"o",maximumChunkBytes:1,maximumChunks:2,write:async(_,chunk)=>{few.push(chunk);}});
 limited.push(new TextEncoder().encode("abcd"));
 await limited.close();
 expect(few.map(piece=>Buffer.from(piece.data,"base64").toString())).toEqual(["a","b"]);
 const stuck=createCheckoutTestChunkWriterV1({nonce:NONCE,origin:"o",drainMilliseconds:50,write:()=>new Promise<void>(()=>{})}),started=Date.now();
 stuck.push(new TextEncoder().encode("x"));
 await stuck.close();
 expect(Date.now()-started,"close never waits longer than the drain").toBeLessThan(2000);
});

test("H44: the follower shows the pieces in order while a file runs, never splits a character, refuses a piece out of order, and finishes the rest of the output with the same decoder",async()=>{
 const euro=new TextEncoder().encode("a€b\n"),memory=memoryFor([euro.subarray(0,2),euro.subarray(2,3),euro.subarray(3)],NONCE),seen:string[]=[],origins:string[]=[];
 const follow=followCheckoutTestChunksV1({nonce:NONCE,read:async hash=>memory.results.get(hash),onText:(text,origin)=>{seen.push(text);origins.push(origin);},signal:new AbortController().signal,pollMilliseconds:5});
 await new Promise(done=>setTimeout(done,60));
 await follow.stop();
 expect(seen.join("")).toBe("a€b\n");
 expect(seen.every(text=>!text.includes("�"))).toBe(true);
 expect(new Set(origins)).toEqual(new Set(["g1-m06"]));
 expect(follow.bytes()).toBe(euro.length);
 const late=memoryFor([new TextEncoder().encode("one\n"),new TextEncoder().encode("two\n")],NONCE),kept:string[]=[];
 const hidden=new Map(late.results);late.results.delete(checkoutTestChunkKeyV1(NONCE,1));
 const waiting=followCheckoutTestChunksV1({nonce:NONCE,read:async hash=>late.results.get(hash),onText:text=>{kept.push(text);},signal:new AbortController().signal,pollMilliseconds:5});
 await new Promise(done=>setTimeout(done,40));
 expect(kept).toEqual(["one\n"]);
 late.results.set(checkoutTestChunkKeyV1(NONCE,1),hidden.get(checkoutTestChunkKeyV1(NONCE,1))!);
 await waiting.stop();
 expect(kept.join("")).toBe("one\ntwo\n");
 expect(waiting.finish(new TextEncoder().encode("rest"))).toBe("rest");
 const skewed=memoryFor([new TextEncoder().encode("zero\n"),new TextEncoder().encode("one\n")],NONCE),swapped=new Map(skewed.results),got:string[]=[];
 skewed.results.set(checkoutTestChunkKeyV1(NONCE,1),swapped.get(checkoutTestChunkKeyV1(NONCE,0))!);
 const checked=followCheckoutTestChunksV1({nonce:NONCE,read:async hash=>skewed.results.get(hash),onText:text=>{got.push(text);},signal:new AbortController().signal,pollMilliseconds:5});
 await checked.stop();
 expect(got.join(""),"a piece that is not the next one ends the following").toBe("zero\n");
 const garbage=new Memory();garbage.results.set(checkoutTestChunkKeyV1(NONCE,0),new Uint8Array([1,2,3]));
 const none:string[]=[],ignoring=followCheckoutTestChunksV1({nonce:NONCE,read:async hash=>garbage.results.get(hash),onText:text=>{none.push(text);},signal:new AbortController().signal,pollMilliseconds:5});
 await ignoring.stop();
 expect(none).toEqual([]);
 expect(ignoring.bytes()).toBe(0);
});

const SLOW=(file:string,milliseconds=1500):readonly string[]=>program(file,`process.stderr.write('first line of '+file+'\\n');setTimeout(()=>{process.stderr.write('second line\\n');process.stdout.write(${JSON.stringify(RESULT("X",false))}.replace('"X"',JSON.stringify(file)));process.exit(0);},${milliseconds});`);

test("H44: a running test file shows its output live: the first line arrives long before the file ends, live text and the remainder add up to the whole output, and a store that refuses the pieces only means the output comes at the end",async()=>{
 const {ask}=fixture({argv:(_,file)=>SLOW(file,3000)}),received:{text:string;origin:string;at:number}[]=[],start=Date.now();
 const done=await ask("a/slow.ts",{onOutput:(text,origin)=>{received.push({text,origin,at:Date.now()});}});
 const end=Date.now();
 expect(done.kind).toBe("done");
 if(done.kind==="done"){
  expect(done.streamed).toBe(true);
  expect(received.length).toBeGreaterThan(0);
  expect(received[0]!.text).toContain("first line of a/slow.ts");
  expect(received[0]!.origin).toBe("g1-m06");
  expect(received[0]!.at,"the first line did not wait for the end of the file").toBeLessThan(end-1000);
  expect(received.map(row=>row.text).join("")+done.stderr).toContain("first line of a/slow.ts");
  expect(received.map(row=>row.text).join("")+done.stderr).toContain("second line");
  expect((received.map(row=>row.text).join("")+done.stderr).match(/first line/gu)).toHaveLength(1);
 }
 expect(end-start).toBeGreaterThan(2500);
 const refusing=fixture({argv:(_,file)=>SLOW(file,500)}),shown:string[]=[];
 refusing.memory.writeActionResult=async()=>{throw Error("cache read-only");};
 const whole=await refusing.ask("a/slow.ts",{onOutput:text=>{shown.push(text);}});
 expect(whole.kind).toBe("done");
 if(whole.kind==="done"){expect(whole.streamed).toBe(false);expect(shown).toEqual([]);expect(whole.stderr).toContain("first line of a/slow.ts");expect(whole.stderr).toContain("second line");}
},30000);

test("H44: files that are out at once never share a machine: a claimed machine is skipped, and a run whose Ready machines are all claimed ends here at once",async()=>{
 const claim=new Set<string>(),first=fixture({peers:["g1-m05","g1-m06"],others:{"g1-m05":{}}});
 claim.add("g1-m05");
 const done=await first.ask("a/ok.ts",{claim});
 expect(done.kind).toBe("done");
 expect(first.log.pins).toEqual(["g1-m06"]);
 expect(claim.size,"every claim is released when the run ends").toBe(1);
 const full=fixture({peers:["g1-m05"],others:{"g1-m05":{}}}),taken=new Set(["g1-m05"]);
 const outcome=await full.ask("a/ok.ts",{claim:taken});
 expect(outcome).toMatchObject({kind:"local",reason:"no-idle-machine",detail:expect.stringContaining("already runs")});
 expect(full.log.pins).toEqual([]);
 expect(full.log.pauses,"no retry while every machine is claimed").toEqual([]);
 const both=fixture({peers:["g1-m05","g1-m06"],others:{"g1-m05":{}},argv:(_,file)=>SLOW(file)}),shared=new Set<string>();
 const [one,two]=await Promise.all([both.ask("a/one.ts",{claim:shared}),both.ask("a/two.ts",{claim:shared})]);
 expect([one.kind,two.kind]).toEqual(["done","done"]);
 expect(new Set(both.log.pins).size,"two files in parallel went to two machines").toBe(2);
 expect(shared.size).toBe(0);
});
