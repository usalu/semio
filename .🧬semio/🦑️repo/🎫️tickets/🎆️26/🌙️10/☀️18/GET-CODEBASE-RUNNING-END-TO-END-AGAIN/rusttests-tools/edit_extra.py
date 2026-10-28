fx = load(HELPER_FIXTURE)
def fsub(old, new):
    global fx
    assert fx.count(old) == 1, ("fixture anchor", old[:60]); fx = fx.replace(old, new)
fsub('import{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"../../🟦️.ts";',
     'import{declareBuildTestEnvelopeV1,registerBuildTestGrantIssuerV1,withBuildTestInvocationV1}from"../../🟦️.ts";\nimport{spawnSync}from"node:child_process";\nimport{scriptInvocationEnvironment}from"../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";')
fsub('if(kind==="artifacts")declareBuildTestEnvelopeV1(',
     '''const fake={tool:"fake",arguments:["repo","run","fixture:declared"]},grant={kind:"native-build-caller" as const,arguments:["run","fixture:declared"],scratch:"sandbox" as const};
if(kind?.startsWith("granted")||kind==="unregistered"||kind==="malformed"){
 if(kind!=="unregistered")registerBuildTestGrantIssuerV1("native-build-caller",{prepare:()=>({environment:{SEMIO_DECLARED_FAKE:"prepared"},release(){console.log("[TRACE] Declared release "+JSON.stringify({fake:process.env.SEMIO_DECLARED_FAKE??null}));}}),capabilities:async()=>fake,matches:async(_grant,inherited)=>JSON.stringify(inherited)===JSON.stringify(fake)});
 declareBuildTestEnvelopeV1({owner:"test:declared-granted",maximumElapsedMilliseconds:60000,capabilities:{grant:kind==="malformed"?{...grant,kind:"unknown" as never}:grant}});
}
else if(kind==="artifacts")declareBuildTestEnvelopeV1(''')
fsub('else if(kind!=="undeclared")declareBuildTestEnvelopeV1(writer);', 'else if(kind!=="undeclared")declareBuildTestEnvelopeV1(writer);')
fsub('test("declared birth "+kind,()=>withBuildTestInvocationV1(async original=>{const facts=original.capabilities as{artifacts?:string;command?:string};console.log(',
     'test("declared birth "+kind,()=>withBuildTestInvocationV1(async original=>{const facts=original.capabilities as{artifacts?:string;command?:string;tool?:string;arguments?:string[]};if(kind?.startsWith("granted"))console.log("[TRACE] Declared grant "+JSON.stringify({tool:facts.tool??null,arguments:facts.arguments??null,fake:process.env.SEMIO_DECLARED_FAKE??null,artifacts:facts.artifacts??null}));if(kind==="granted-nested"){const nested=spawnSync(process.execPath,["test",import.meta.path],{cwd:process.cwd(),env:scriptInvocationEnvironment(original,{...process.env,SEMIO_DECLARED_CASE:"granted"}),encoding:"utf8",timeout:60000});process.stdout.write(nested.stdout);process.stderr.write(nested.stderr);if(nested.status!==0)throw Error("Nested granted birth failed: "+nested.stderr.slice(-2000));}console.log(')
save(HELPER_FIXTURE, fx)
tt = load(HELPER_TESTS)
tt += '''
/** 🎫️ A declared grant is issued by its registered issuer to a bare file (environment prepared, capabilities carried, released and restored afterwards), accepted beneath a real parent that carries exactly that grant, and refused beneath a parent that does not, for an unregistered issuer and for a malformed declaration. */
test("a declared grant is issued to a bare file, accepted beneath a parent carrying it and refused otherwise",()=>withBuildTestInvocationV1(async original=>{
 const fixture=join(import.meta.dir,"🧫️declared/🟦️.ts"),control=createIssuedResourceControlV1(original,"fixture:declared-grant",120000),bare=Object.fromEntries(Object.entries(process.env).filter(([name])=>name!=="SEMIO_SCRIPT_PROCESS_INVOCATION"&&name!=="SEMIO_TEST_ARTIFACT_DIR"));
 const run=(kind:string)=>{const result=spawnSync(process.execPath,["test",fixture],{cwd:process.cwd(),env:{...bare,TMPDIR:tmpdir(),SEMIO_DECLARED_CASE:kind},encoding:"utf8",timeout:60000});return{status:result.status,stdout:result.stdout,stderr:result.stderr};};
 const lines=(stdout:string,name:string)=>stdout.split(/\\r?\\n/u).filter(line=>line.startsWith("[TRACE] Declared "+name+" ")||line==="[TRACE] Declared "+name).map(line=>line.slice(("[TRACE] Declared "+name).length).trim()).map(text=>text===""?null:JSON.parse(text));
 const fake={tool:"fake",arguments:["repo","run","fixture:declared"],fake:"prepared",artifacts:null};
 const granted=run("granted");expect(granted.status,granted.stderr.slice(-4000)).toBe(0);expect(lines(granted.stdout,"grant")).toEqual([fake]);expect(lines(granted.stdout,"release")).toEqual([{fake:null}]);
 const nested=run("granted-nested");expect(nested.status,nested.stderr.slice(-4000)).toBe(0);expect(lines(nested.stdout,"grant")).toEqual([fake,{...fake}]);expect(lines(nested.stdout,"release")).toEqual([{fake:null}]);
 const inherited=await captureOwnedCommandResultV1(process.execPath,["test",fixture],process.cwd(),{control,env:scriptInvocationEnvironment(original,{...bare,TMPDIR:tmpdir(),SEMIO_DECLARED_CASE:"granted"}),output:{maximumBytes:65536,maximumLines:512}});expect(inherited.status).not.toBe(0);expect(inherited.stderr).toContain("Inherited build test capabilities differ from the declared envelope of test:declared-granted");expect(lines(inherited.stdout,"grant")).toEqual([]);
 const unregistered=run("unregistered");expect(unregistered.status).not.toBe(0);expect(unregistered.stderr).toContain("Registered build test grant issuer required: native-build-caller");
 const malformed=run("malformed");expect(malformed.status).not.toBe(0);expect(malformed.stderr).toContain("Finite closed build test envelope required");
}),120000);
'''
save(HELPER_TESTS, tt)
def rsub(rel, old, new, count=1):
    text = load(rel); assert text.count(old) == count, (rel, old[:70], text.count(old)); save(rel, text.replace(old, new))
RUST = LIB + "🏗️build/🕸️graph/🦀️rust/"
USE = 'import{withBuildTestInvocationV1}from"@semio-tech/process/testing/invocation";\n'
rel = RUST + "🔒️lifetime/🧪️tests/🟦️.ts"
rsub(rel, "()=>receiveScriptProcessInvocation(process.env,async original=>{const control=scriptCompilerAdmissionControlV1(original,\"test:selected-rust:lifetime\"", "()=>withBuildTestInvocationV1(async original=>{const control=scriptCompilerAdmissionControlV1(original,\"test:selected-rust:lifetime\"")
rsub(rel, 'import{withSelectedRustBuildGraphV1}from"../../🟦️.ts";\n', 'import{withSelectedRustBuildGraphV1}from"../../🟦️.ts";\n' + USE)
rel = RUST + "📥️source/🧪️tests/🟦️.ts"
rsub(rel, "await receiveScriptProcessInvocation(process.env,async original=>{const ticket=", "await withBuildTestInvocationV1(async original=>{const ticket=")
rsub(rel, 'import{receiveScriptProcessInvocation,scriptResourceProcessEnvironmentV1}from', USE + 'import{scriptResourceProcessEnvironmentV1}from')
SHARED = LIB + "⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/🤝️shared/🟦️.ts"
rsub(SHARED, "async()=>receiveScriptProcessInvocation(process.env,async original=>{\n const base=", "async()=>withBuildTestInvocationV1(async original=>{\n const base=")
txt = load(SHARED)
m = re.search(r'import\{receiveScriptProcessInvocation\}from"[^"]*";\n', txt); assert m
save(SHARED, txt.replace(m.group(0), USE))
ROUTES = LIB + "🏗️build/🗂️suites/🧪️tests/🟦️.ts"
rsub(ROUTES, 'import{buildTestAggregationV1,buildTestCoverageV1,buildTestForeignRoutesV1,buildTestRootV1,readBuildTestTableV1}from"../🟦️.ts";\n',
     'import{buildTestAggregationV1,buildTestCoverageV1,buildTestForeignRoutesV1,buildTestRootV1,readBuildTestTableV1}from"../🟦️.ts";\nimport{cachingTestRootV1,readCachingTestTableV1}from"../../../⚡️caching/🗂️suites/🟦️.ts";\n')
rsub(ROUTES, 'import{existsSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";', 'import{existsSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync}from"node:fs";')
rsub(ROUTES, 'import table from"../🔣️.json"with{type:"json"};\n', 'import table from"../🔣️.json"with{type:"json"};\nimport callerSchema from"../../🧬️schema/📥️caller.json"with{type:"json"};\nimport{withBuildTestInvocationV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";\n')
routes = load(ROUTES)
routes += '''
/** 🪪️ The birth a library test file declares names the suite that routes the file, the scratch that suite row declares and an existing project target `test-<suite>`: the declaration cannot drift from the route table. */
test("every declared library test birth names the suite that routes its file, that suite's scratch and an existing target",()=>{
 const rows=discoverProcessTestFilesV1(library).flatMap(file=>[...readFileSync(resolve(library,file),"utf8").matchAll(/declareLibraryBuildTestEnvelopeV1\\(\\{suite:"([^"]+)"(?:,scratch:"(sandbox|checkout)")?\\}\\)/gu)].map(match=>({file,suite:match[1]!,scratch:match[2]??"sandbox"})));
 expect(rows.length).toBeGreaterThanOrEqual(15);
 const routed=new Map<string,{files:Set<string>;scratch:string}>();
 for(const[root,declared]of[[buildTestRootV1,readBuildTestTableV1()],[cachingTestRootV1,readCachingTestTableV1()]]as const){const found=discoverProcessTestFilesV1(root);for(const suite of declared.suites)routed.set(suite.id,{files:new Set(expandProcessTestSuiteV1(suite,found,declared).map(file=>resolve(root,file))),scratch:suite.scratch??"sandbox"});}
 const targets=Object.keys((JSON.parse(readFileSync(join(cachingRoot,"📋️project.json"),"utf8")) as{targets:Record<string,unknown>}).targets);
 for(const row of rows){const suite=routed.get(row.suite);expect(suite,row.file+" declares unknown suite "+row.suite).toBeDefined();expect(suite!.files.has(resolve(library,row.file)),row.file+" is not routed by "+row.suite).toBe(true);expect(row.scratch,row.file).toBe(suite!.scratch);expect(targets,row.suite).toContain("test-"+row.suite);}
});
/** 🎫️ A bare library test file is born with a genuine native build caller grant and a sandboxed artifact directory: the capabilities validate against the caller schema with Ajv and name this checkout, a repo run command and no artifacts capability. */
test("a declared library test birth carries a genuine native build caller grant and an existing artifact directory",()=>withBuildTestInvocationV1(async original=>{
 const facts=original.capabilities as Record<string,unknown>,directory=process.env.SEMIO_TEST_ARTIFACT_DIR;
 expect(new Ajv({strict:true}).validate(callerSchema,facts)).toBe(true);
 expect(facts.workspaceRoot).toBe(realpathSync(process.cwd()));expect(["prototype","release"]).toContain(facts.buildMode);expect((facts.arguments as string[]).slice(0,2)).toEqual(["repo","run"]);expect(facts.launches).toContainEqual(facts.arguments);expect("artifacts" in facts).toBe(false);
 expect(directory!==undefined&&existsSync(directory)).toBe(true);
}),60000);
'''
save(ROUTES, routes)

declare_line = 'import{declareLibraryBuildTestEnvelopeV1}from"%s";\ndeclareLibraryBuildTestEnvelopeV1({suite:"build-route-table"});\n' % relative_import(ROUTES, NATIVE)
rsub(ROUTES, 'import fixture from"./🧫️fixtures/🔣️.json"with{type:"json"};\n', 'import fixture from"./🧫️fixtures/🔣️.json"with{type:"json"};\n' + declare_line)
