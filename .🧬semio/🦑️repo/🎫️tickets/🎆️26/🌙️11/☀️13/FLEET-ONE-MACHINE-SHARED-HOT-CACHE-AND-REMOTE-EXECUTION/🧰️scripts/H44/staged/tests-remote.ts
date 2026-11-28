
test("H44: a saturated host offers the file even while its memory verdict is admit, a host that is neither saturated nor under pressure keeps it, and the measured durations are recorded for the next verdict",async()=>{
 const memoRoot=mkdtempSync(join(tmpdir(),"semio-h44-")),calls:string[]=[],offload=(async()=>{calls.push("offload");return{kind:"done",origin:"g1-m06",result:passed("a.ts"),exitCode:0,stderr:"",streamed:false,seconds:4.2};}) as unknown as Offload;
 try{
  const calm=(await createTestFileRemoteV1(root,{},{memoRoot,host:async()=>"admit",load:async()=>"keep",idle:async()=>3,offload}))!;
  expect(await calm("a.ts",1000,input())).toEqual({kind:"local",reason:"local-slot",detail:"",attempted:false});
  const saturated=(await createTestFileRemoteV1(root,{},{memoRoot,host:async()=>"admit",load:async file=>file==="a.ts"?"offload":"keep",idle:async()=>3,offload}))!;
  expect(await saturated("a.ts",1000,input())).toMatchObject({kind:"ran",origin:"g1-m06"});
  expect(await saturated("b.ts",1000,input())).toMatchObject({kind:"local",reason:"local-slot"});
  expect(calls).toEqual(["offload"]);
  const {readCheckoutTestMeasuresV1}=await import("../../../🌐️remote-execution/🚀️offload/🔬️checkout-test/🚦️gate/🟦️.ts");
  expect(readCheckoutTestMeasuresV1(memoRoot).files["a.ts"]?.remote?.seconds).toBe(4.2);
  await saturated.measure!("a.ts",31.4);
  expect(readCheckoutTestMeasuresV1(memoRoot).files["a.ts"]).toMatchObject({local:{seconds:31.4},remote:{seconds:4.2}});
  expect(saturated.parallel).toBeGreaterThan(1);
 }finally{rmSync(memoRoot,{recursive:true,force:true});}
});

test("H44: the live output of a remote file is prefixed with its position and machine, complete lines at a time, and the rest of the output is prefixed the same way",async()=>{
 const memoRoot=mkdtempSync(join(tmpdir(),"semio-h44-")),lines:string[]=[];
 const offload=(async(call:Parameters<Offload>[0])=>{call.onOutput?.("first\nsec","g1-m06");call.onOutput?.("ond\nthi","g1-m06");return{kind:"done",origin:"g1-m06",result:passed("a.ts"),exitCode:0,stderr:"rd\nfourth",streamed:true,seconds:1};}) as unknown as Offload;
 try{
  const remote=(await createTestFileRemoteV1(root,{},{memoRoot,host:async()=>"wait",idle:async()=>1,offload}))!;
  const outcome=await remote("a.ts",1000,{...input(),output:text=>{lines.push(text);},tag:"2/3 "});
  expect(lines).toEqual(["[2/3 g1-m06] first\n","[2/3 g1-m06] second\n"]);
  expect(outcome).toMatchObject({kind:"ran",output:"[2/3 g1-m06] third\n[2/3 g1-m06] fourth\n"});
  const quiet=(await createTestFileRemoteV1(root,{},{memoRoot,host:async()=>"wait",idle:async()=>1,offload:(async()=>({kind:"done",origin:"g1-m06",result:passed("a.ts"),exitCode:0,stderr:"whole output\n",streamed:false,seconds:1})) as unknown as Offload}))!;
  expect(await quiet("a.ts",1000,input())).toMatchObject({kind:"ran",output:"whole output\n"});
 }finally{rmSync(memoRoot,{recursive:true,force:true});}
});

test("H44: several files of one run go out in parallel up to the width of the port, rows come back in the order of the routes, a width of one or bail keeps them one after the other, and live output reaches stderr as it arrives",async()=>{
 const run=async(parallel:number|undefined,bail=false)=>{
  let active=0,peak=0;
  const asked:string[]=[],remote:TestFileRemoteV1=Object.assign(async(file:string,_budget:number,call:Parameters<TestFileRemoteV1>[2])=>{
   asked.push(file);active++;peak=Math.max(peak,active);
   call.output?.(`[${call.tag}g1-m0${file.charCodeAt(0)-96}] live ${file}\n`);
   await new Promise(done=>setTimeout(done,file==="a.ts"?600:300));
   active--;
   return{kind:"ran",origin:"g1-m0"+(file.charCodeAt(0)-96),seconds:0.1,result:passed(file),output:"tail of "+file+"\n"} as const;
  },parallel===undefined?{}:{parallel});
  const printed:string[]=[],write=process.stderr.write;
  process.stderr.write=((chunk:unknown)=>{printed.push(String(chunk));return true;}) as typeof process.stderr.write;
  const started=Date.now();
  let result;
  try{result=await runTestFileRoutesV1(root,[route("a.ts"),route("b.ts"),route("c.ts")],{bail,json:false,language:"en"},{remote,notice:()=>{},tables:new Map()});}finally{process.stderr.write=write;}
  return{result,peak,printed:printed.join(""),asked,elapsed:Date.now()-started};
 };
 const wide=await run(3);
 expect(wide.peak).toBe(3);
 expect(wide.result.files.map(row=>[row.file,row.status,row.ranOn])).toEqual([["a.ts","passed","g1-m01"],["b.ts","passed","g1-m02"],["c.ts","passed","g1-m03"]]);
 expect(wide.printed).toContain("[1/3 g1-m01] live a.ts\n");
 expect(wide.printed).toContain("[3/3 g1-m03] live c.ts\n");
 expect(wide.printed.endsWith("tail of c.ts\n")).toBe(true);
 expect(wide.printed.indexOf("tail of a.ts")).toBeLessThan(wide.printed.indexOf("tail of b.ts"));
 expect(wide.elapsed,"three files of 600/300/300 ms in parallel take about the longest, not the sum (1200 ms one after the other)").toBeLessThan(1000);
 const narrow=await run(2);
 expect(narrow.peak).toBe(2);
 expect(narrow.result.files.map(row=>row.file)).toEqual(["a.ts","b.ts","c.ts"]);
 for(const [name,outcome] of [["no width",await run(undefined)],["width one",await run(1)],["bail",await run(3,true)]] as const){
  expect(outcome.peak,name).toBe(1);
  expect(outcome.asked,name).toEqual(["a.ts","b.ts","c.ts"]);
  expect(outcome.result.files.map(row=>row.file),name).toEqual(["a.ts","b.ts","c.ts"]);
 }
 const same=await run(3),again=await run(2);
 expect(JSON.stringify(same.result.files.map(row=>[row.file,row.status,row.pass,row.fail,row.ranOn])),"the aggregate is deterministic whatever the width").toBe(JSON.stringify(again.result.files.map(row=>[row.file,row.status,row.pass,row.fail,row.ranOn])));
});

test("H44: a file that runs on no host here is never offered in a parallel run",async()=>{
 const asked:string[]=[],remote:TestFileRemoteV1=Object.assign(async(file:string)=>{asked.push(file);return{kind:"ran",origin:"g1-m06",seconds:0.1,result:passed(file),output:""} as const;},{parallel:3}),notices:string[]=[];
 const result=await runTestFileRoutesV1(root,[route("a.ts"),route("b.ts",undefined,{runsOnHost:false}),route("c.ts")],{bail:false,json:false,language:"en"},{remote,notice:line=>{notices.push(line);},tables:new Map()});
 expect(result.files.map(row=>[row.file,row.status])).toEqual([["a.ts","passed"],["b.ts","skipped"],["c.ts","passed"]]);
 expect([...asked].sort()).toEqual(["a.ts","c.ts"]);
 expect(notices.filter(line=>line.includes("a.ts ran on")||line.includes("c.ts ran on"))).toHaveLength(2);
});
