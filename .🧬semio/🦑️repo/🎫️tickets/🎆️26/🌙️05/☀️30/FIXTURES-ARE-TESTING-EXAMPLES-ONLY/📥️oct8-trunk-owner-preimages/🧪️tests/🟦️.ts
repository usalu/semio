import { expect, test } from "bun:test";
import { readFileSync, mkdirSync, readdirSync, writeFileSync, existsSync } from "node:fs";
import { join, resolve } from "node:path";
import { homedir } from "node:os";
import ts from "typescript";
import Graph from "graphology";
import Ajv from "ajv";
import { createHash } from "node:crypto";
import { inspectRuntimeGraphV1, runtimeEcmaReferencesV1, runtimeFixturePathV1 } from "../🟦️.ts";
const owner = resolve(import.meta.dir, "..");
const examples = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8")).examples;
const schema = JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8"));
const admit = new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/RuntimeGraphEvidenceV1`)!;

test("actual build-resource observations bind original bytes and refuse fixture ancestry", async () => {
  const { runtimeBuildResourceInputsV1, runtimeRetainedBuildResourceInputsV1 } = await import("../🔎️verification/🟦️.ts");
  const examples = JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")).resourceObservations;
  const admitResource = new Ajv({strict:true}).addSchema(schema).getSchema(`${schema.$id}#/$defs/RuntimeBuildResourceObservationV1`)!;
  for (const row of examples) {
    for (const observation of row.rows) expect(admitResource(observation),JSON.stringify(admitResource.errors)).toBe(true);
    const evidence = runtimeBuildResourceInputsV1(row.rows.map(JSON.stringify).join("\n"), {root:"/workspace",cwd:"/workspace",outDirectory:"/workspace/out",read:(path:string)=>row.files[path],directory:(path:string)=>Object.keys(row.files).filter(value=>value.startsWith(path+"/")&&!value.slice(path.length+1).includes("/"))});
    expect(evidence.findings.some(finding=>finding.code==="runtime-fixture-edge")).toBe(row.forbidden);
    expect(evidence.findings.some(finding=>finding.code==="runtime-input-mismatch")).toBe(row.inputMismatch);
  }
  const row = examples[0], text = row.rows.map(JSON.stringify).join("\n"), hash = (value:string)=>createHash("sha256").update(value).digest("hex");
  const resource = {package_id:"owner",out_dir:"/workspace/out",path:"/workspace/out/semio-runtime-resource-inputs.jsonl",text,sha256:hash(text),observedAtMs:2,resources:row.rows.map((input:any)=>({input,sha256:input.kind==="directory"?null:hash(row.files[input.path]),outputSha256:input.kind==="directory"?null:hash(row.files[input.output]),...(input.kind==="directory"?{observedEntries:input.entries}:{})}))};
  const observation = {version:1 as const,manifest:"/workspace/Cargo.toml",cwd:"/workspace",command:"cargo" as const,args:["build"],builtAtMs:1,observedAtMs:2,status:0,cancelled:false,units:[],buildScripts:[{reason:"build-script-executed" as const,package_id:"owner",cfgs:[],env:[],out_dir:"/workspace/out"}]};
  const context = {root:"/workspace",cwd:"/workspace",outDirectory:"/workspace/out",read:(path:string)=>row.files[path],directory:()=>["assets/logo.svg"]};
  expect(runtimeRetainedBuildResourceInputsV1(observation,resource,context).verified).toBe(true);
  expect(runtimeRetainedBuildResourceInputsV1(observation,resource,{...context,read:(path:string)=>path.endsWith("logo.svg")?"both were changed":row.files[path]}).verified).toBe(false);
  expect(runtimeRetainedBuildResourceInputsV1({...observation,buildScripts:[]},resource,context).verified).toBe(false);
  expect(runtimeRetainedBuildResourceInputsV1(observation,{...resource,sha256:"0".repeat(64)},context).verified).toBe(false);
  expect(runtimeRetainedBuildResourceInputsV1(observation,{...resource,resources:[]},context).verified).toBe(false);
});

test("actual native WGPU producer exposes original resource reads independently of rustc dep-info", async () => {
  const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifacts) throw Error("Caller-owned SEMIO_TEST_ARTIFACT_DIR is required");
  const {runtimeBuildResourceInputsV1} = await import("../🔎️verification/🟦️.ts"), root = process.cwd();
  const target = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu", source = join(root,target,"🏗️builder/🦀️.rs"), directory = join(artifacts,"runtime-wgpu-resource-oracle"), out = join(directory,"out");
  mkdirSync(out,{recursive:true});
  const binary = join(directory,process.platform === "win32"?"build-resource.exe":"build-resource"), depInfo = join(directory,"producer.d");
  const compile = Bun.spawn(["rustc","--crate-name","runtime_wgpu_resource_oracle","--edition","2024","--emit",`link,dep-info=${depInfo}`,source,"-o",binary],{cwd:root,stdout:"pipe",stderr:"pipe"});
  const errors = await new Response(compile.stderr).text(); expect(await compile.exited,errors).toBe(0);
  const run = Bun.spawn([binary],{cwd:root,env:{...process.env,CARGO_MANIFEST_DIR:join(root,target,"📦️packages/🦀️rust"),OUT_DIR:out},stdout:"pipe",stderr:"pipe"});
  const [stdout,stderr,exit] = await Promise.all([new Response(run.stdout).text(),new Response(run.stderr).text(),run.exited]);
  expect(exit,stderr).toBe(0); expect(stdout).toContain("cargo:rerun-if-changed=");
  const text = readFileSync(join(out,"semio-runtime-resource-inputs.jsonl"),"utf8"), evidence = runtimeBuildResourceInputsV1(text,{root,cwd:root,outDirectory:out,read:path=>{try{return readFileSync(resolve(root,path))}catch{return undefined}},directory:path=>{try{return readdirSync(resolve(root,path)).map(name=>path+"/"+name)}catch{return undefined}}});
  expect(evidence.findings).toEqual([]);
  const glob = await import("fast-glob"), icons = "🧰️framework/🔨️modules/🖼️assets/🔣️icons";
  const expected = (await glob.default("**/*.svg",{cwd:join(root,icons),ignore:["🤖️generated/**"],followSymbolicLinks:false})).map(path=>icons+"/"+path).sort();
  const actual = evidence.observations.filter(row=>row.kind==="copy").map(row=>resolve(root,row.path)).filter(path=>path.startsWith(join(root,icons))).map(path=>path.slice(root.length+1).replaceAll("\\","/")).sort();
  expect(actual).toEqual(expected);
  const dep = readFileSync(depInfo,"utf8"); expect(dep).toContain("📥️resources");
  expect(dep).not.toContain("semio_logo.svg");
  const admitResource = new Ajv({strict:true}).addSchema(schema).getSchema(`${schema.$id}#/$defs/RuntimeBuildResourceObservationV1`)!;
  for(const row of evidence.observations) expect(admitResource(row),JSON.stringify(admitResource.errors)).toBe(true);
  writeFileSync(join(directory,"receipt.json"),JSON.stringify({source,sourceSha256:createHash("sha256").update(readFileSync(source)).digest("hex"),binarySha256:createHash("sha256").update(readFileSync(binary)).digest("hex"),observations:evidence.observations,inputs:evidence.inputs,findings:evidence.findings},null,2)+"\n");
  console.log(`[DEBUG] actual native WGPU resource producer observed ${actual.length} original SVG copies; third-party glob agrees; rustc dep-info excludes original asset bytes`);
});

test("retained Cargo observations bind current inputs and staging after capture deletion", async () => {
  const { runtimeCargoProvenanceV1, runtimeCargoUnitInputsV1 } = await import("../🔎️verification/🟦️.ts");
  const files = { "src/lib.rs": "pub const VALUE:u8=1;", "generated/value.txt": "1", "published/lib.rlib": "compiled" };
  const digest = (path: string): string | undefined => path in files ? createHash("sha256").update(files[path as keyof typeof files]).digest("hex") : undefined;
  const message = { reason: "compiler-artifact", package_id: "owner", features: ["live"], profile: {test:false}, target: {src_path:"/workspace/src/lib.rs",kind:["lib"]}, filenames:["/removed/lib.rlib"] };
  const unit = { message, observedAtMs:2, depInfo:[{path:"/removed/lib.d",baseDirectory:"/workspace",text:"/removed/lib.rlib: src/lib.rs generated/value.txt\n"}], inputs:[{path:"/workspace/src/lib.rs",sha256:digest("src/lib.rs")},{path:"/workspace/generated/value.txt",sha256:digest("generated/value.txt")}], artifacts:[{path:"/removed/lib.rlib",sha256:digest("published/lib.rlib"),stagedPath:"/workspace/published/lib.rlib",stagedSha256:digest("published/lib.rlib")}] };
  const receipt = {version:1,manifest:"/workspace/Cargo.toml",cwd:"/workspace",command:"cargo",args:["build","--lib"],builtAtMs:1,observedAtMs:2,status:0,cancelled:false,units:[unit],buildScripts:[{reason:"build-script-executed",package_id:"owner",cfgs:["owned_cfg"],env:[["OWNED","yes"]],out_dir:"/workspace/generated"}]};
  const [observation] = runtimeCargoProvenanceV1(JSON.stringify(receipt));
  const observed = observation!.units[0]!;
  expect(runtimeCargoUnitInputsV1(observation!,observed,"/workspace",digest)).toEqual({inputs:["generated/value.txt","src/lib.rs"],findings:[],verified:true});
  const ajv = new Ajv({strict:true}).addSchema(schema), admitInput = ajv.getSchema(`${schema.$id}#/$defs/RuntimeCargoInputDigestV1`)!, admitBuild = ajv.getSchema(`${schema.$id}#/$defs/RuntimeCargoBuildScriptV1`)!;
  for (const input of observed.inputs) expect(admitInput(input),JSON.stringify(admitInput.errors)).toBe(true);
  expect(admitBuild(observation!.buildScripts[0]),JSON.stringify(admitBuild.errors)).toBe(true);
  expect(runtimeCargoUnitInputsV1(observation!,observed,"/workspace",path=>path==="src/lib.rs"?"0".repeat(64):digest(path)).verified).toBe(false);
  expect(runtimeCargoUnitInputsV1(observation!,{...observed,inputs:observed.inputs.slice(1)},"/workspace",digest).verified).toBe(false);
  expect(()=>runtimeCargoProvenanceV1(JSON.stringify({...receipt,units:[{...unit,depInfo:[{path:"/removed/lib.d",text:"/removed/lib.rlib: src/lib.rs"}]}]}))).toThrow("dep-info content");
  expect(runtimeCargoUnitInputsV1(observation!,{...observed,depInfo:[]},"/workspace",digest).verified).toBe(false);
  expect(runtimeCargoUnitInputsV1(observation!,{...observed,message:{...observed.message,profile:{test:true}}},"/workspace",digest).verified).toBe(false);
  expect(()=>runtimeCargoProvenanceV1(JSON.stringify({...receipt,status:1}))).toThrow("completed invocation");
  expect(()=>runtimeCargoProvenanceV1(JSON.stringify({...receipt,units:[{...unit,artifacts:[{...unit.artifacts[0],stagedSha256:"0".repeat(64)}]}]}))).toThrow("artifact binding");
});

for (const row of examples) test(`runtime graph ${row.id}`, () => {
  const evidence = inspectRuntimeGraphV1(row.roots, { ...row, read: (path: string) => row.files[path] });
  expect(admit(evidence), JSON.stringify(admit.errors)).toBe(true);
  expect(evidence.findings.some(finding => finding.code === "runtime-fixture-edge")).toBe(row.forbidden);
  expect(evidence.findings.some(finding => finding.code === "runtime-unresolved-edge")).toBe(row.unresolved);
  expect(evidence.findings.some(finding => finding.code === "runtime-input-mismatch")).toBe(row.inputMismatch ?? false);
  const graph = new Graph({ multi: true, type: "directed" });
  for (const node of evidence.nodes) graph.addNode(node);
  for (const edge of evidence.edges) graph.addDirectedEdge(edge.from, edge.to);
  const reached = new Set<string>(), queue = [...row.roots];
  for (const node of queue) { if (reached.has(node)) continue; reached.add(node); for (const child of graph.outNeighbors(node)) queue.push(child); }
  expect([...reached].sort()).toEqual(evidence.nodes);
  expect([...reached].some(path => runtimeFixturePathV1(path))).toBe(row.forbidden);
});

test("Bun import edges match independent TypeScript parser for static, dynamic, require and resource literals", () => {
  for (const row of examples.filter((row: any) => row.roots[0].endsWith(".ts") && !row.unresolved && row.id !== "aliased-read")) {
    const source = row.files[row.roots[0]], parsed = ts.createSourceFile(row.roots[0], source, ts.ScriptTarget.Latest, true), expected: string[] = [];
    const visit = (node: ts.Node): void => {
      if ((ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier)) expected.push(node.moduleSpecifier.text);
      if (ts.isCallExpression(node) && (node.expression.kind === ts.SyntaxKind.ImportKeyword || node.expression.getText(parsed) === "require" || node.expression.getText(parsed) === "readFileSync") && node.arguments[0] && ts.isStringLiteral(node.arguments[0])) expected.push(node.arguments[0].text);
      if (ts.isNewExpression(node) && node.expression.getText(parsed) === "URL" && node.arguments?.[0] && ts.isStringLiteral(node.arguments[0])) expected.push(node.arguments[0].text);
      ts.forEachChild(node, visit);
    };
    visit(parsed);
    expect(runtimeEcmaReferencesV1(source, row.roots[0]).flatMap(edge => edge.path === null ? [] : [edge.path]).sort()).toEqual(expected.sort());
  }
});

test("actual Rust compiler dep-info agrees with test cfg exclusions and reachable embedded resources", async () => {
  const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifacts) throw Error("Caller-owned SEMIO_TEST_ARTIFACT_DIR is required");
  for (const row of examples.filter((row: any) => row.roots[0].endsWith(".rs") && !row.unresolved && row.id !== "manifest-concat" && row.compilerInputOracle !== false)) {
    const directory = join(artifacts, "runtime-graph-rust-oracle", row.id);
    for (const [path, source] of Object.entries(row.files)) { mkdirSync(join(directory, path, ".."), { recursive: true }); writeFileSync(join(directory, path), source as string); }
    const output = join(directory, "inputs.d");
    const proc = Bun.spawn(["rustc", "--crate-name", "runtime_graph_oracle", "--crate-type", "lib", "--emit", "dep-info", ...row.features.flatMap((feature: string) => ["--cfg", `feature=${JSON.stringify(feature)}`]), row.roots[0], "-o", output], { cwd: directory, stdout: "pipe", stderr: "pipe" });
    const stderr = await new Response(proc.stderr).text();
    expect(await proc.exited, stderr).toBe(0);
    const depInfo = readFileSync(output, "utf8");
    const evidence = inspectRuntimeGraphV1(row.roots, { ...row, read: (path: string) => row.files[path] });
    expect(evidence.nodes.filter(node => runtimeFixturePathV1(node)).some(node => depInfo.includes(node)), row.id).toBe(row.forbidden);
    for (const node of evidence.nodes) expect(depInfo.includes(node), row.id + " " + node).toBe(true);
  }
  console.log("[DEBUG] actual rustc production dep-info matches neutral nested cfg/test/include_str/include_bytes resource graph");
});

test("compiled input mismatch and generated fixture origin refuse evidence", () => {
  const row = examples.find((row: any) => row.id === "real-fixture-name");
  const evidence = inspectRuntimeGraphV1(row.roots, { ...row, compiledInputs: [row.roots[0]], read: (path: string) => row.files[path] });
  expect(evidence.findings.some(finding => finding.code === "runtime-input-mismatch")).toBe(true);
});

test("Cargo runtime dependency closure retains normal/build while isolating dev-only features", async () => {
  const { runtimeCargoPackagesV1, runtimeCompilerArtifactsV1 } = await import("../🔎️verification/🟦️.ts");
  const nodes = [
    {id: "app", features: [], deps: [{pkg:"live",dep_kinds:[{kind:null}]},{pkg:"helper",dep_kinds:[{kind:"dev"}]},{pkg:"builder",dep_kinds:[{kind:"build"}]}]},
    {id: "live", features: [], deps: [{pkg:"helper",dep_kinds:[{kind:"dev"}]}]},
  ];
  const graph = new Graph({type:"directed"});
  for(const id of ["app","live","helper","builder"])graph.addNode(id);
  for(const row of nodes)for(const edge of row.deps)if(edge.dep_kinds.some(kind=>kind.kind!=="dev"))graph.addDirectedEdge(row.id,edge.pkg);
  expect(runtimeCargoPackagesV1(["app"],nodes)).toEqual(["app",...graph.outNeighbors("app")].sort());
  expect(runtimeCargoPackagesV1(["app"],nodes)).not.toContain("helper");
  const message = JSON.stringify({reason:"compiler-artifact",package_id:"app",features:["runtime"],profile:{test:false},target:{src_path:"src/lib.rs",kind:["lib"]},filenames:["app.rmeta"]});
  const records = runtimeCompilerArtifactsV1(`guest-framework-check wasm32-wasip2 (Cargo.toml): app\n${message}`);
  expect(records[0]?.features).toEqual(["runtime"]);
  expect(records[0]?.runtimeTarget).toBe("wasm32-wasip2");
  expect(records[0]?.runtimeWorkspace).toBe("Cargo.toml");
  expect(records[0]?.profile.test).toBe(false);
  expect(()=>runtimeCompilerArtifactsV1(message.replace(',"profile":{"test":false}',''))).toThrow('profile identity');
});

test("production test exclusion agrees with independent esbuild compiler and preserves live dynamic fixture refusals", async () => {
  const { transform } = await import("esbuild");
  const row = examples.find((row: any) => row.id === "production-vitest-exclusion");
  const result = await transform(row.files[row.roots[0]], { loader:"ts", define:{"import.meta.vitest":"undefined"},minifySyntax:true });
  expect(runtimeEcmaReferencesV1(result.code,row.roots[0])).toEqual([]);
  const evidence = inspectRuntimeGraphV1(row.roots,{...row,read:(path:string)=>row.files[path]});
  expect(evidence.findings).toEqual([]);
  expect(evidence.nodes).toEqual(row.roots);
  const live = examples.find((row:any)=>row.id==='dynamic-import');
  expect(inspectRuntimeGraphV1(live.roots,{...live,productionTests:'excluded',read:(path:string)=>live.files[path]}).findings.some(f=>f.code==='runtime-fixture-edge')).toBe(true);
});

test("Cargo host unit cfg and escaped dep-info filesystem inputs retain separate authority", async () => {
  const {runtimeCargoUnitsV1,runtimeDepInfoInputsV1}=await import('../🔎️verification/🟦️.ts');
  const packages=[{id:'app',name:'app',manifest_path:'Cargo.toml',source:null,targets:[{src_path:'src/lib.rs',kind:['lib']}]},{id:'build',name:'build',manifest_path:'build/Cargo.toml',source:null,targets:[{src_path:'build/lib.rs',kind:['lib']}]},{id:'derive',name:'derive',manifest_path:'derive/Cargo.toml',source:null,targets:[{src_path:'derive/lib.rs',kind:['proc-macro']}]}];
  const nodes=[{id:'app',features:[],deps:[{pkg:'build',dep_kinds:[{kind:'build'}]},{pkg:'derive',dep_kinds:[{kind:null}]}]}];
  expect(runtimeCargoUnitsV1(['app'],packages,nodes)).toEqual([{id:'app',host:false},{id:'build',host:true},{id:'derive',host:true}]);
  expect(runtimeDepInfoInputsV1('target/lib.d: src/lib.rs source\\ space/🧫️fixtures/input.json\n','/workspace')).toEqual(['source space/🧫️fixtures/input.json','src/lib.rs']);
});

test("actual compiler roster reconciles only macro expansion and retains unresolved resources", async () => {
  const {reconcileRuntimeMacroInputsV1}=await import('../🔎️verification/🟦️.ts');
  const row=examples.find((row:any)=>row.id==='captured-macro');
  const source=inspectRuntimeGraphV1(row.roots,{...row,read:(path:string)=>row.files[path]});
  expect(reconcileRuntimeMacroInputsV1(source,[])).toEqual(source);
  const compiled=['src/lib.rs','src/runtime.json'];
  const reconciled=reconcileRuntimeMacroInputsV1(source,compiled);
  expect(reconciled.findings).toEqual([]);
  expect(reconciled.edges).toContainEqual({from:'src/lib.rs',to:'src/runtime.json',kind:'dependency'});
  const missing={...source,findings:[...source.findings,{code:'runtime-unresolved-edge' as const,path:'src/lib.rs',detail:'Computed resource has no declared resolved owner'}]};
  expect(reconcileRuntimeMacroInputsV1(missing,compiled).findings).toEqual([missing.findings[1]!]);
});

test("actual native filesystem reads remain resource edges absent from compiler dep-info", async () => {
  const row=examples.find((row:any)=>row.id==='rust-fs-literal'), artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;
  if(!artifacts)throw Error('Caller-owned SEMIO_TEST_ARTIFACT_DIR is required');
  const directory=join(artifacts,'runtime-graph-rust-oracle',row.id);
  for(const[path,source]of Object.entries(row.files)){mkdirSync(join(directory,path,'..'),{recursive:true});writeFileSync(join(directory,path),source as string);}
  writeFileSync(join(directory,'entry.rs'),row.files[row.roots[0]]+'\nfn main(){assert_eq!(payload(),b"{}");println!("[DEBUG] native filesystem fixture bytes observed outside compiler dep-info");}');
  const binary=join(directory,process.platform==='win32'?'oracle.exe':'oracle');
  const compiler=Bun.spawn(['rustc','--crate-name','runtime_resource_oracle','entry.rs','--emit','link,dep-info','-o',binary],{cwd:directory,stdout:'pipe',stderr:'pipe'});
  expect(await compiler.exited,await new Response(compiler.stderr).text()).toBe(0);
  expect(readFileSync(join(directory,'oracle.d'),'utf8')).not.toContain('🧫️fixtures/input.json');
  const child=Bun.spawn([binary],{cwd:directory,stdout:'pipe',stderr:'pipe'});
  expect(await child.exited,await new Response(child.stderr).text()).toBe(0);
  expect(await new Response(child.stdout).text()).toContain('[DEBUG] native filesystem fixture bytes observed');
  expect(inspectRuntimeGraphV1(row.roots,{...row,read:(path:string)=>row.files[path]}).findings.some(f=>f.code==='runtime-fixture-edge')).toBe(true);
});

test("durable Cargo discovery selects only current exact invocation observations", async()=>{
  const {runtimeCargoObservationsForChecksV1,runtimeCargoInvocationV1}=await import("../🔎️verification/🟦️.ts"), parse=(await import("yargs-parser")).default;
  const rows=JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")).cargoObservationSelections;
  const check={workspace:"Cargo.toml",target:"wasm32-wasip2",packages:["owner"],features:["owner/live"]};
  for(const row of rows){
    const observation={version:1 as const,manifest:"/workspace/Cargo.toml",cwd:"/workspace",command:"cargo" as const,args:row.args,builtAtMs:1,observedAtMs:2,status:0,cancelled:false,units:[],buildScripts:[]};
    const selected=runtimeCargoObservationsForChecksV1([observation],[check],"/workspace",()=>row.current);
    expect(selected.length).toBe(row.selected?1:0);
    const oracle=parse(row.args,{array:["p","features"],string:["target","manifest-path"]});
    expect(runtimeCargoInvocationV1(observation,"/workspace").target).toBe(oracle.target);
    expect(runtimeCargoInvocationV1(observation,"/workspace").packages).toEqual(oracle.p);
    expect(runtimeCargoInvocationV1(observation,"/workspace").features).toEqual(oracle.features);
  }
});

test("actual durable Cargo producer acquires current features and refuses stale configuration/artifacts",async()=>{
  const {cargoStreamingStatus}=await import("../../../🏃️process/🟦️.ts"), {runtimeCargoProvenanceV1,runtimeCargoObservationCurrentV1}=await import("../🔎️verification/🟦️.ts");
  const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR; if(!artifacts) throw Error("Caller-owned artifacts are required");
  const program=JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")).cargoCurrentnessProgram,directory=join(artifacts,"runtime-cargo-currentness-oracle"),source=join(directory,"owner"),buildDirectory=join(directory,"build"),manifest=join(source,"Cargo.toml");
  mkdirSync(source,{recursive:true});for(const[path,content]of Object.entries(program.files))writeFileSync(join(source,path),content as string);
  const environment={...process.env,CARGO_TARGET_DIR:join(directory,"target"),CARGO_BUILD_BUILD_DIR:buildDirectory};delete environment.SEMIO_TEST_ARTIFACT_DIR;
  expect(await cargoStreamingStatus(["generate-lockfile","--offline","--manifest-path",manifest],directory,environment,10000)).toBe(0);
  const ledger=join(buildDirectory,"semio-cargo-provenance"),before=new Set(existsSync(ledger)?readdirSync(ledger):[]);
  expect(await cargoStreamingStatus(["check","--locked","--manifest-path",manifest,"--lib","-p",program.package,"--features",program.feature,"--message-format=json"],directory,environment,10000)).toBe(0);
  const receiptPath=readdirSync(ledger).find(name=>!before.has(name)&&name.startsWith("cargo-unit-provenance-"));expect(receiptPath).toBeDefined();
  const [observation]=runtimeCargoProvenanceV1(readFileSync(join(ledger,receiptPath!),"utf8"));
  const context={root:process.cwd(),cwd:directory,outDirectory:"",buildDirectory,cargoHome:resolve(process.env.CARGO_HOME??join(homedir(),".cargo")),digest:(path:string)=>{try{return createHash("sha256").update(readFileSync(path)).digest("hex")}catch{return undefined}},read:(path:string)=>{try{return readFileSync(path)}catch{return undefined}},directory:(path:string)=>{try{return readdirSync(path).map(name=>join(path,name))}catch{return undefined}}};
  expect(runtimeCargoObservationCurrentV1(observation!,context)).toBe(true);
  const unit=observation!.units.find(unit=>unit.message.target.kind.includes("lib"))!;expect(unit.message.features).toEqual([program.feature]);
  expect(runtimeCargoObservationCurrentV1(observation!,{...context,digest:path=>path===unit.artifacts[0]!.path?undefined:context.digest(path)})).toBe(false);
  expect(runtimeCargoObservationCurrentV1(observation!,{...context,digest:path=>path===manifest?"0".repeat(64):context.digest(path)})).toBe(false);
  expect(runtimeCargoObservationCurrentV1({...observation!,units:[]},context)).toBe(false);
  const ajv=new Ajv({strict:true}).addSchema(schema),input=ajv.getSchema(`${schema.$id}#/$defs/RuntimeCargoInvocationInputV1`)!,dep=ajv.getSchema(`${schema.$id}#/$defs/RuntimeCargoDepInfoV1`)!;
  for(const row of observation!.invocationInputs!)expect(input(row),JSON.stringify(input.errors)).toBe(true);
  for(const row of unit.depInfo)expect(dep(row),JSON.stringify(dep.errors)).toBe(true);
  console.log("[DEBUG] actual durable no-optin Cargo producer binds current feature/configuration/artifact inputs under a foreign caller cwd");
});

test("fresh browser compiler bytes require the same actual mounted output",async()=>{
 const {runtimeBrowserArtifactV1}=await import("../🔎️verification/🟦️.ts"),rows=JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")).browserArtifacts;
 for(const row of rows){const artifact=runtimeBrowserArtifactV1(row.path,row.compiled,()=>row.mounted??undefined);expect(artifact.current).toBe(row.current);expect(artifact.compiledSha256).toBe(new Bun.CryptoHasher("sha256").update(row.compiled).digest("hex"));expect(artifact.mountedSha256).toBe(row.mounted===null?null:new Bun.CryptoHasher("sha256").update(row.mounted).digest("hex"));}
});
