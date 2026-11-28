/** 📨️ The same file gets an identical envelope from `repo test-file` and from the suite runner under the orchestrator of `repo test`: the Original budget is the orchestrator's (unbounded unless `SEMIO_ORCHESTRATOR_BUDGET_MS` declares one), never the budget of the suite row, which only funds the tests of the file and the deadline guard of the run. */
test("identical envelope: a file run by test-file and by the suite runner under the orchestrator is born with the same Original budget, whatever the suite row declares",async()=>{
 const {runTestFileRoutesV1}=await import("../🏃️run/🟦️.ts"),{nxBootstrapEnvelopeV1}=await import("../../../⚡️caching/🚀️bootstrap/📜️script.ts"),{runProcessTestSuiteV1,processTestCheckoutV1,withProcessTestOutputV1}=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🗂️suites/🏃️run/🟦️.ts"),{createScriptProcessEnvelope,withScriptProcessEnvelope}=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),{issueRepositoryScriptProvidersV1}=await import("../../../🏃️process/🔌️providers/🟦️.ts"),{createBuildBootstrapCapabilitiesV1}=await import("../../📥️caller/📤️child/🟦️.ts");
 const fixture=here+"/📨️envelope/🟦️.ts",tables=new Map() as ReturnType<typeof readTestFileTablesV1>,saved={budget:process.env.SEMIO_ORCHESTRATOR_BUDGET_MS,expect:process.env.SEMIO_TEST_ENVELOPE_EXPECT};
 try{
  for(const budget of [undefined,"7200000"]){
   if(budget===undefined)delete process.env.SEMIO_ORCHESTRATOR_BUDGET_MS;else process.env.SEMIO_ORCHESTRATOR_BUDGET_MS=budget;
   const orchestrator=nxBootstrapEnvelopeV1(root,["nx","run","x:y"],process.env,Date.now());
   expect(orchestrator.policy.maximumElapsedMilliseconds).toBe(budget===undefined?0:Number(budget));
   process.env.SEMIO_TEST_ENVELOPE_EXPECT=JSON.stringify({maximumElapsedMilliseconds:orchestrator.policy.maximumElapsedMilliseconds});
   const routes=routeTestFilesV1(root,[fixture],tables),viaTestFile=await runTestFileRoutesV1(root,routes,{bail:false,json:true,language:"en"},{policy:policy as never,tables,notice:()=>{}});
   expect(viaTestFile.files.map(row=>row.status),String(viaTestFile.files[0]?.error)+String(viaTestFile.files[0]?.tail)).toEqual(["passed"]);
   const row={id:"plain",budgetMs:1000,typecheck:"none" as const,files:["📨️envelope/🟦️.ts"]},table={version:1 as const,suites:[row],codeSuites:[],helpers:[]},tableRoot=resolve(root,dirname(fixture)),checkout=processTestCheckoutV1(tableRoot,root),environment={...process.env,SEMIO_WORKSPACE_ROOT:checkout} as NodeJS.ProcessEnv;
   issueRepositoryScriptProvidersV1(environment);
   const capabilities=await createBuildBootstrapCapabilitiesV1(checkout,testFileEnvelopeArgumentsV1(policy.plain.project,row.id),environment);
   let ran=false;
   await withScriptProcessEnvelope(createScriptProcessEnvelope(orchestrator.policy,capabilities,Date.now()),async invocation=>{await withProcessTestOutputV1(checkout,policy.plain.route,output=>runProcessTestSuiteV1({invocation,root:tableRoot,checkout,table:{...table,suites:[{...row,files:["🟦️.ts"]}]},suite:row.id,owner:policy.plain.owner,output,environment}));ran=true;});
   expect(ran).toBe(true);
  }
 }finally{
  if(saved.budget===undefined)delete process.env.SEMIO_ORCHESTRATOR_BUDGET_MS;else process.env.SEMIO_ORCHESTRATOR_BUDGET_MS=saved.budget;
  if(saved.expect===undefined)delete process.env.SEMIO_TEST_ENVELOPE_EXPECT;else process.env.SEMIO_TEST_ENVELOPE_EXPECT=saved.expect;
 }
},300000);
