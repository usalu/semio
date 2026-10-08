import { expect, test } from "bun:test";
import { readFileSync, mkdirSync, readdirSync, writeFileSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { homedir } from "node:os";
import ts from "typescript";
import {cargoDirectoryEntriesV1 as readDirectoryMetadata} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts";
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
    const evidence = runtimeBuildResourceInputsV1(row.rows.map(JSON.stringify).join("\n"), {root:"/workspace",cwd:"/workspace",outDirectory:"/workspace/out",read:(path:string)=>row.files[path],directoryEntries:(path:string)=>Object.keys(row.files).filter(value=>value.startsWith(path+"/")&&!value.slice(path.length+1).includes("/")).map(path=>({path,kind:"file" as const,symlinkTarget:null}))});
    expect(evidence.findings.some(finding=>finding.code==="runtime-fixture-edge")).toBe(row.forbidden);
    expect(evidence.findings.some(finding=>finding.code==="runtime-input-mismatch")).toBe(row.inputMismatch);
  }
  const row = examples[0], text = row.rows.map(JSON.stringify).join("\n"), hash = (value:string)=>createHash("sha256").update(value).digest("hex");
  const resource = {package_id:"owner",out_dir:"/workspace/out",path:"/workspace/out/semio-runtime-resource-inputs.jsonl",text,sha256:hash(text),observedAtMs:2,resources:row.rows.map((input:any)=>({input,sha256:input.kind==="directory"?hash(JSON.stringify(input.entries.map((entry:any)=>[entry.path.slice(entry.path.lastIndexOf("/")+1),entry.kind,entry.symlinkTarget]))):hash(row.files[input.path]),outputSha256:input.kind==="directory"?null:hash(row.files[input.output]),...(input.kind==="directory"?{observedEntries:input.entries}:{})}))};
  const observation = {version:1 as const,manifest:"/workspace/Cargo.toml",cwd:"/workspace",command:"cargo" as const,args:["build"],builtAtMs:1,observedAtMs:2,status:0,cancelled:false,units:[],buildScripts:[{reason:"build-script-executed" as const,package_id:"owner",cfgs:[],env:[],out_dir:"/workspace/out"}]};
  const context = {root:"/workspace",cwd:"/workspace",outDirectory:"/workspace/out",read:(path:string)=>row.files[path],directoryEntries:()=>[{path:"assets/logo.svg",kind:"file" as const,symlinkTarget:null}]};
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
  const text = readFileSync(join(out,"semio-runtime-resource-inputs.jsonl"),"utf8"), evidence = runtimeBuildResourceInputsV1(text,{root,cwd:root,outDirectory:out,read:path=>{try{return readFileSync(resolve(root,path))}catch{return undefined}},directoryEntries:path=>readDirectoryMetadata(resolve(root,path))?.map(([name,kind,symlinkTarget])=>({path:join(resolve(root,path),name),kind,symlinkTarget}))});
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
  const message = { reason: "compiler-artifact", package_id: "owner", features: ["live"], profile: {test:false}, target: {name:"owner",src_path:"/workspace/src/lib.rs",kind:["lib"]}, filenames:["/removed/lib.rlib"] };
  const unit = { message, observedAtMs:2, depInfo:[{path:"/removed/lib.d",baseDirectory:"/workspace",text:"/removed/lib.rlib: src/lib.rs generated/value.txt\n"}], inputs:[{path:"/workspace/src/lib.rs",kind:"file",sha256:digest("src/lib.rs")},{path:"/workspace/generated/value.txt",kind:"file",sha256:digest("generated/value.txt")}], artifacts:[{path:"/removed/lib.rlib",sha256:digest("published/lib.rlib"),stagedPath:"/workspace/published/lib.rlib",stagedSha256:digest("published/lib.rlib")}] };
  const receipt = {version:1,manifest:"/workspace/Cargo.toml",cwd:"/workspace",command:"cargo",args:["build","--lib"],builtAtMs:1,observedAtMs:2,status:0,cancelled:false,compilerResourceRoot:null,compilerResources:[],units:[unit],buildScripts:[{reason:"build-script-executed",package_id:"owner",cfgs:["owned_cfg"],env:[["OWNED","yes"]],out_dir:"/workspace/generated"}]};
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
  const evidence = inspectRuntimeGraphV1(row.roots, { ...row, read: (path: string) => row.files[path], resourceDigest:(path:string,kind:string)=>kind==="directory"?row.directoryHashes?.[path]:row.files[path]===undefined?undefined:createHash("sha256").update(row.files[path]).digest("hex") });
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

test("runtime resource-read owners retain exact file bytes and terminal directory metadata", async () => {
  const ajv=new Ajv({strict:true}).addSchema(schema),admitRead=ajv.getSchema(`${schema.$id}#/$defs/RuntimeResourceReadOwnerV1`)!;
  for(const row of examples.filter((row:any)=>row.resourceReads))for(const owners of Object.values(row.resourceReads) as any[][])for(const owner of owners)expect(admitRead(owner),JSON.stringify(admitRead.errors)).toBe(true);
  const row=examples.find((row:any)=>row.id==="rust-resource-read-directory"),reads:string[]=[];
  const evidence=inspectRuntimeGraphV1(row.roots,{...row,read:(path:string)=>{reads.push(path);return row.files[path];},resourceDigest:(path:string,kind:string)=>kind==="directory"?row.directoryHashes?.[path]:row.files[path]===undefined?undefined:createHash("sha256").update(row.files[path]).digest("hex")});
  expect(evidence.findings).toEqual([]);expect(evidence.nodes).toEqual(["assets","src/lib.rs"]);expect(reads).not.toContain("assets");expect(evidence.nodes.some(path=>path.includes("🧫️fixtures"))).toBe(false);
  const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Caller-owned artifact partition required");
  const dir=join(artifacts,"exact-resource-directory-oracle");mkdirSync(join(dir,"assets/🧫️fixtures"),{recursive:true});writeFileSync(join(dir,"assets/runtime.json"),"{}\n");
  const {cargoDirectoryEntriesV1,cargoInputDigestV1}=await import("../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts");
  const glob=(await import("fast-glob")).default,entries=cargoDirectoryEntriesV1(join(dir,"assets"))!;
  expect(entries.map(entry=>entry[0]).sort()).toEqual((await glob("*",{cwd:join(dir,"assets"),onlyFiles:false,dot:true,deep:1})).sort());
  expect(cargoInputDigestV1(join(dir,"assets"))?.sha256).toBe(new Bun.CryptoHasher("sha256").update(JSON.stringify(entries)).digest("hex"));
  console.log("[DEBUG] exact filesystem callsite bytes and independent directory metadata preserve listing versus child-read ownership");
});

test("dynamic import owners bind one exact computed callsite against independent TypeScript and SHA oracles", () => {
  const admitOwner = new Ajv({strict:true}).addSchema(schema).getSchema(`${schema.$id}#/$defs/RuntimeDynamicImportOwnerV1`)!;
  for (const row of examples.filter((row:any)=>row.dynamicImports)) {
    for (const [path, owners] of Object.entries(row.dynamicImports) as [string, any[]][]) {
      const source=row.files[path],parsed=ts.createSourceFile(path,source,ts.ScriptTarget.Latest,true),computed:ts.CallExpression[]=[];
      const visit=(node:ts.Node):void=>{if(ts.isCallExpression(node)&&(node.expression.kind===ts.SyntaxKind.ImportKeyword||node.expression.getText(parsed)==="require")&&node.arguments[0]&&!ts.isStringLiteral(node.arguments[0]))computed.push(node);ts.forEachChild(node,visit);};
      visit(parsed);
      for (const owner of owners) expect(admitOwner(owner),row.id+JSON.stringify(admitOwner.errors)).toBe(row.ownerAdmission??true);
      if(row.id==="dynamic-exact-callsite-only") {
        expect(computed.map(call=>call.arguments[0]!.getText(parsed))).toEqual(["first","second"]);
        expect(owners[0].sourceSha256).toBe(new Bun.CryptoHasher("sha256").update(source).digest("hex"));
        expect(owners[0].callsiteIndex).toBe(0);
        const evidence=inspectRuntimeGraphV1(row.roots,{...row,read:(path:string)=>row.files[path]});
        expect(evidence.edges).toContainEqual({from:path,to:owners[0].path,kind:"import"});
        expect(evidence.findings.filter(f=>f.code==="runtime-unresolved-edge")).toHaveLength(1);
      }
    }
  }
  console.log("[DEBUG] computed imports retain exact active caller SHA and independent lexical callsite identity");
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
  const context={root:process.cwd(),cwd:directory,outDirectory:"",buildDirectory,cargoHome:resolve(process.env.CARGO_HOME??join(homedir(),".cargo")),digest:(path:string)=>{try{return createHash("sha256").update(readFileSync(path)).digest("hex")}catch{return undefined}},read:(path:string)=>{try{return readFileSync(path)}catch{return undefined}},directoryEntries:(path:string)=>readDirectoryMetadata(path)?.map(([name,kind,symlinkTarget])=>({path:join(path,name),kind,symlinkTarget}))};
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

 test("Trunk raw compiler query and actual mounted outputs bind exact bytes", async()=>{
  const {runtimeTrunkCompilationV1}=await import("../🔎️verification/🟦️.ts"),row=JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")).trunkOutputs;
  const hash=(bytes:string)=>new Bun.CryptoHasher("sha256").update(bytes).digest("hex"),rawSha256=hash(row.raw),outputs=Object.entries(row.outputs).map(([path,bytes])=>({path,sha256:hash(String(bytes))}));
  const receipt={rawSha256,querySha256:rawSha256,outputs},admit=new Ajv({strict:true}).addSchema(schema).getSchema(`${schema.$id}#/$defs/RuntimeTrunkCompilationV1`)!;
  expect(admit(receipt),JSON.stringify(admit.errors)).toBe(true);
  expect(runtimeTrunkCompilationV1(receipt,(path:string)=>row.outputs[path])).toEqual([]);
  expect(runtimeTrunkCompilationV1({...receipt,querySha256:"0".repeat(64)},(path:string)=>row.outputs[path])).toHaveLength(1);
  expect(runtimeTrunkCompilationV1(receipt,()=>undefined)).toHaveLength(2);
  expect(runtimeTrunkCompilationV1(receipt,(path:string)=>row.outputs[path]+" changed")).toHaveLength(2);
 });

test("actual Trunk compiled transformation retains current mounted bytes after temporary cleanup",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Caller-owned artifact directory required");
 const directory=join(artifacts,"actual-trunk-neutral"),packageRoot=join(directory,"owner"),build=join(directory,"compiler");mkdirSync(join(packageRoot,"src"),{recursive:true});
 const bindgenVersion=(Bun.TOML.parse(readFileSync(join(process.cwd(),"Cargo.lock"),"utf8")) as {package:{name:string;version:string}[]}).package.find(row=>row.name==="wasm-bindgen")!.version;
 writeFileSync(join(packageRoot,"Cargo.toml"),'[package]\nname="runtime-trunk-neutral"\nversion="0.1.0"\nedition="2021"\n[workspace]\n[lib]\ncrate-type=["cdylib"]\n[dependencies]\nwasm-bindgen="='+bindgenVersion+'"\n');
 writeFileSync(join(packageRoot,"src/lib.rs"),'#[wasm_bindgen::prelude::wasm_bindgen] pub fn value()->u32{7}\n');
 const root=process.cwd(),environment={...process.env,CARGO_BUILD_BUILD_DIR:build,CARGO_TARGET_DIR:join(directory,"target")},nativeOwner=join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🌐️wasm/📜️script.ts");
 const {cargoStreamingStatus}=await import("../../../🏃️process/🟦️.ts");expect(await cargoStreamingStatus(["generate-lockfile","--manifest-path",join(packageRoot,"Cargo.toml"),"--offline"],packageRoot,environment,30000)).toBe(0);
 const {buildTrunkRenderer}=await import(nativeOwner);await buildTrunkRenderer({rustPackageRoot:packageRoot,workspace:root,profile:"dev",toolWorkspace:root,stateRoot:join(directory,"trunk-state"),environment});
 const ledger=join(build,"semio-trunk-provenance"),path=readdirSync(ledger).map(name=>join(ledger,name,"trunk.json")).sort((a,b)=>JSON.parse(readFileSync(b,"utf8")).observedAtMs-JSON.parse(readFileSync(a,"utf8")).observedAtMs)[0]!,receipt=JSON.parse(readFileSync(path,"utf8"));
 const {runtimeTrunkObservationCurrentV1,runtimeCargoObservationCurrentV1}=await import("../🔎️verification/🟦️.ts"),digest=(path:string)=>{try{return createHash("sha256").update(readFileSync(path)).digest("hex")}catch{return undefined}},read=(path:string)=>{try{return readFileSync(resolve(root,path))}catch{return undefined}};
 const context={root,profile:"dev",manifest:join(packageRoot,"Cargo.toml"),outputDirectory:join(packageRoot,"dist/wasm-dev"),owner:nativeOwner,digest,read,compilerCurrent:observation=>runtimeCargoObservationCurrentV1(observation,{root,cwd:observation.cwd,outDirectory:"",buildDirectory:build,cargoHome:resolve(process.env.CARGO_HOME??join(homedir(),".cargo")),digest,read,directoryEntries:path=>readDirectoryMetadata(resolve(root,path))?.map(([name,kind,symlinkTarget])=>({path:join(resolve(root,path),name),kind,symlinkTarget}))})};
 const admitTrunk=new Ajv({strict:true}).addSchema(schema).getSchema(`${schema.$id}#/$defs/RuntimeTrunkObservationV1`)!;expect(admitTrunk(receipt),JSON.stringify(admitTrunk.errors)).toBe(true);
 expect(runtimeTrunkObservationCurrentV1({...receipt,builtAtMs:NaN},context)).toBeUndefined();expect(runtimeTrunkObservationCurrentV1({...receipt,observedAtMs:undefined},context)).toBeUndefined();
 expect(runtimeTrunkObservationCurrentV1(receipt,context)).toBeDefined();expect(runtimeTrunkObservationCurrentV1({...receipt,raw:{...receipt.raw,sha256:"0".repeat(64)}},context)).toBeUndefined();
 const wasm=receipt.outputs.find(output=>output.path.endsWith("_bg.wasm"))!,bytes=readFileSync(wasm.path),module=await WebAssembly.compile(bytes);expect(WebAssembly.Module.exports(module).some(entry=>entry.name==="value")).toBe(true);
 const bindings=await import(receipt.outputs.find(output=>output.path.endsWith(".js"))!.path);await bindings.default({module_or_path:bytes});expect(bindings.value()).toBe(7);
 const hash=new Bun.CryptoHasher("sha256").update(bytes).digest("hex");expect(hash).toBe(wasm.sha256);writeFileSync(wasm.path,Buffer.concat([bytes,Buffer.from("changed")]));expect(runtimeTrunkObservationCurrentV1(receipt,context)).toBeUndefined();writeFileSync(wasm.path,bytes);
 writeFileSync(join(directory,"verified-receipt.json"),JSON.stringify({path,receipt,actualValue:bindings.value(),independentWasmSha256:hash},null,2)+"\n");console.log("[DEBUG] actual Trunk compiled/query/staged digests agree after temporary cleanup; third-party WebAssembly executes value7; stale mounted bytes refused");
},120000);

 test("actual rustc consumed-byte checksums refuse edits before completed receipt observation",async()=>{
 const {blake3}=await import("@noble/hashes/blake3.js"),{runtimeCargoConsumedInputsV1,runtimeDepInfoChecksumsV1}=await import("../🔎️verification/🟦️.ts");
 const row=JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")).compilerChecksums,bytes=Buffer.from(row.source),hash=Buffer.from(blake3(bytes)).toString("hex"),text=`owner.rlib: ${row.path}\n# checksum:blake3=${hash} file_len:${bytes.length} ${row.path}\n`,unit={depInfo:[{text,baseDirectory:row.baseDirectory}],inputs:[{path:resolve(row.baseDirectory,row.path),kind:"file"}]};
 const admit=new Ajv({strict:true}).addSchema(schema).getSchema(`${schema.$id}#/$defs/RuntimeCompilerChecksumV1`)!;for(const value of runtimeDepInfoChecksumsV1(text,row.baseDirectory))expect(admit(value),JSON.stringify(admit.errors)).toBe(true);
 expect(runtimeCargoConsumedInputsV1(unit,()=>bytes)).toEqual([]);expect(runtimeCargoConsumedInputsV1(unit,()=>Buffer.from(row.changed))).toHaveLength(1);expect(runtimeCargoConsumedInputsV1({...unit,depInfo:[{...unit.depInfo[0],text:`owner.rlib: ${row.path}\n`}]},()=>bytes)).toHaveLength(1);
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR!;const source=join(artifacts,"compiler-consumed-owner.rs"),dep=join(artifacts,"compiler-consumed-owner.d");writeFileSync(source,row.source);const child=Bun.spawn(["rustc","-Z","checksum-hash-algorithm=blake3","--crate-name","compiler_consumed_owner","--crate-type","lib","--emit",`dep-info=${dep}`,source],{stdout:"pipe",stderr:"pipe"});expect(await child.exited,await new Response(child.stderr).text()).toBe(0);
 const actual={depInfo:[{text:readFileSync(dep,"utf8"),baseDirectory:artifacts}],inputs:[{path:source,kind:"file"}]};expect(runtimeCargoConsumedInputsV1(actual,path=>readFileSync(path))).toEqual([]);writeFileSync(source,row.changed);expect(runtimeCargoConsumedInputsV1(actual,path=>readFileSync(path))).toHaveLength(1);console.log("[DEBUG] actual rustc BLAKE3 consumed-byte checksums and independent noble agree; pre-observation source edits refused");
 });

test("actual proc-macro reads preserve original snapshots and tracked directory inputs",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR!,directory=join(artifacts,`actual-compiler-resources-${crypto.randomUUID()}`),resources=join(directory,"inputs"),captures=join(directory,"captures");mkdirSync(resources,{recursive:true});
 const row=JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")).compilerResourceProgram;for(const[path,value]of Object.entries(row.files)){mkdirSync(join(resources,path,".."),{recursive:true});writeFileSync(join(resources,path),value as string);}
 const helper=resolve(process.cwd(),"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️resources/🧮️compiler/🦀️.rs"),source=join(directory,"producer.rs"),caller=join(directory,"caller.rs"),library=join(directory,process.platform==="win32"?"compiler_resources.dll":process.platform==="darwin"?"libcompiler_resources.dylib":"libcompiler_resources.so"),binary=join(directory,process.platform==="win32"?"caller.exe":"caller"),dep=join(directory,"caller.d");
 writeFileSync(source,`#![feature(proc_macro_tracked_path,proc_macro_tracked_env)]\nextern crate proc_macro;#[path=${JSON.stringify(helper)}] mod resources;\n#[proc_macro]pub fn observed(_:proc_macro::TokenStream)->proc_macro::TokenStream{let root=proc_macro::tracked::env_var("SEMIO_COMPILER_RESOURCE_ROOT").ok();resources::with_compiler_resources_v1(${JSON.stringify(directory)},${JSON.stringify(source)},proc_macro::Span::call_site().local_file(),root,|path|proc_macro::tracked::path(path),||{let path=std::path::PathBuf::from(proc_macro::tracked::env_var("RESOURCE_INPUT").unwrap());let bytes=resources::read(path.join("input.txt")).unwrap();let entries=resources::read_dir(&path).unwrap().collect::<Result<Vec<_>,_>>().unwrap();assert!(!bytes.is_empty());assert_eq!(entries.len(),2);"const OBSERVED:u8=7;".parse().unwrap()})}\n`);
 writeFileSync(caller,`compiler_resources::observed!();fn main(){assert_eq!(OBSERVED,${row.observedValue});println!("[DEBUG] actual proc-macro payload consumed");}`);
 const environment={...process.env,SEMIO_COMPILER_RESOURCE_ROOT:captures,RESOURCE_INPUT:resources,CARGO_MANIFEST_DIR:directory,CARGO_CRATE_NAME:"caller"};
 const compile=Bun.spawn(["rustc","--edition","2024","--crate-name","compiler_resources","--crate-type","proc-macro",source,"-o",library],{env:environment,stdout:"pipe",stderr:"pipe"});expect(await compile.exited,await new Response(compile.stderr).text()).toBe(0);
 const primary=Bun.spawn(["rustc","--edition","2024","-Z","checksum-hash-algorithm=blake3","--crate-name","caller","--extern",`compiler_resources=${library}`,caller,"--emit",`link,dep-info=${dep}`,"-o",binary],{env:environment,stdout:"pipe",stderr:"pipe"});expect(await primary.exited,await new Response(primary.stderr).text()).toBe(0);const run=Bun.spawn([binary],{stdout:"pipe",stderr:"pipe"});expect(await run.exited,await new Response(run.stderr).text()).toBe(0);
 const glob=(await import("fast-glob")).default,observations=(await glob("**/observation.json",{cwd:captures,absolute:true})).map(path=>({path,value:JSON.parse(readFileSync(path,"utf8"))}));expect(observations).toHaveLength(1);const capture=observations[0]!,admit=new Ajv({strict:true}).addSchema(schema).getSchema(`${schema.$id}#/$defs/RuntimeCompilerResourceObservationV1`)!;expect(admit(capture.value),JSON.stringify(admit.errors)).toBe(true);expect(capture.value.caller.source).toBe(caller);const original=capture.value.resources.find((row:any)=>row.kind==="read"),listing=capture.value.resources.find((row:any)=>row.kind==="directory");expect(readFileSync(original.output,"utf8")).toBe(row.files["input.txt"]);expect(listing.entries.map((row:any)=>row.path).sort()).toEqual((await glob("*",{cwd:resources,absolute:true,onlyFiles:false,markDirectories:false})).sort());const compiler=readFileSync(dep,"utf8");for(const path of[original.path,original.output,listing.path,capture.path])expect(compiler.includes(path)).toBe(true);writeFileSync(original.path,"source changed");expect(readFileSync(original.output,"utf8")).toBe(row.files["input.txt"]);writeFileSync(join(directory,"verified.json"),JSON.stringify({observations,depInfo:compiler},null,2));console.log("[DEBUG] actual proc-macro read bytes and directory rosters retained; fast-glob agrees and rustc tracks snapshots/capture/original inputs");
},30000);

test("actual Cargo proc-macro receipts bind current original resources and exact caller units",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR!,root=process.cwd(),directory=join(artifacts,`compiler-resource-cargo-${crypto.randomUUID()}`),macro=join(directory,"macro"),caller=join(directory,"caller"),inputs=join(directory,"inputs"),build=join(directory,"build"),manifest=join(directory,"Cargo.toml"),helper=resolve(root,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️resources/🧮️compiler/🦀️.rs");
 for(const path of[macro,caller,inputs])mkdirSync(path,{recursive:true});writeFileSync(manifest,'[workspace]\nresolver="2"\nmembers=["macro","caller"]\n');writeFileSync(join(macro,"Cargo.toml"),'[package]\nname="compiler-observation-macro"\nversion="0.1.0"\nedition="2024"\n[lib]\nproc-macro=true\npath="lib.rs"\n');writeFileSync(join(caller,"Cargo.toml"),'[package]\nname="compiler-observation-caller"\nversion="0.1.0"\nedition="2024"\n[lib]\npath="lib.rs"\n[dependencies]\ncompiler-observation-macro={path="../macro"}\n');writeFileSync(join(inputs,"asset.txt"),"actual payload");
 writeFileSync(join(macro,"lib.rs"),`#![feature(proc_macro_tracked_path,proc_macro_tracked_env)]\nextern crate proc_macro;#[path=${JSON.stringify(helper)}]mod resources;#[proc_macro]pub fn observed(_:proc_macro::TokenStream)->proc_macro::TokenStream{resources::with_compiler_resources_v1(env!("CARGO_MANIFEST_DIR"),concat!(env!("CARGO_MANIFEST_DIR"),"/lib.rs"),proc_macro::Span::call_site().local_file(),proc_macro::tracked::env_var("SEMIO_COMPILER_RESOURCE_ROOT").ok(),|path|proc_macro::tracked::path(path),||{let root=std::path::PathBuf::from(proc_macro::tracked::env_var("RESOURCE_INPUT").unwrap());let bytes=resources::read(root.join("asset.txt")).unwrap();let _entries=resources::read_dir(&root).unwrap().collect::<Result<Vec<_>,_>>().unwrap();assert_eq!(bytes,b"actual payload");"pub const OBSERVED:u8=7;".parse().unwrap()})}`);writeFileSync(join(caller,"lib.rs"),"compiler_observation_macro::observed!();");
 const {cargoStreamingStatus}=await import("../../../🏃️process/🟦️.ts"),environment={...process.env,CARGO_TARGET_DIR:join(directory,"target"),CARGO_BUILD_BUILD_DIR:build,RESOURCE_INPUT:inputs,SEMIO_TEST_ARTIFACT_DIR:undefined};
 expect(await cargoStreamingStatus(["generate-lockfile","--manifest-path",manifest,"--offline"],directory,environment,10000)).toBe(0);expect(await cargoStreamingStatus(["check","--manifest-path",manifest,"--locked","-p","compiler-observation-caller","--message-format=json"],directory,environment,30000)).toBe(0);
 const ledger=join(build,"semio-cargo-provenance"),path=readdirSync(ledger).map(name=>join(ledger,name))[0]!,{runtimeCargoProvenanceV1,runtimeCompilerResourceInputsV1,runtimeCompilerResourceReadOwnersV1,runtimeCargoObservationCurrentV1}=await import("../🔎️verification/🟦️.ts"),{cargoInputDigestV1,cargoDirectoryEntriesV1}=await import("../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts"),[observation]=runtimeCargoProvenanceV1(readFileSync(path,"utf8"));expect(observation!.compilerResources).toHaveLength(1);
 const context={root,cwd:directory,outDirectory:"",buildDirectory:build,cargoHome:resolve(process.env.CARGO_HOME??join(homedir(),".cargo")),digest:(path:string)=>cargoInputDigestV1(path).sha256??undefined,read:(path:string)=>{try{return readFileSync(path)}catch{return undefined}},directoryEntries:(path:string)=>readDirectoryMetadata(path)?.map(([name,kind,symlinkTarget])=>({path:join(path,name),kind,symlinkTarget})),entries:(path:string)=>cargoDirectoryEntriesV1(path)??undefined},resource=observation!.compilerResources[0]!,evidence=runtimeCompilerResourceInputsV1(observation!,resource,context);
 expect(evidence.findings).toEqual([]);expect(evidence.verified).toBe(true);const mapped=runtimeCompilerResourceReadOwnersV1(observation!,context),readOwners=Object.values(mapped.owners).flat(),validateOwner=new Ajv({strict:true}).addSchema(schema).getSchema(`${schema.$id}#/$defs/RuntimeResourceReadOwnerV1`)!;expect(readOwners).toHaveLength(3);for(const value of readOwners)expect(validateOwner(value),JSON.stringify(validateOwner.errors)).toBe(true);expect(readOwners.flatMap(owner=>owner.inputs.map(input=>input.kind)).sort()).toEqual(["directory","file","file"]);expect(Object.values(mapped.directories)).toHaveLength(1);for(const[source,owners]of Object.entries(mapped.owners)){const value=readFileSync(resolve(root,source),"utf8");for(const entry of owners)expect(value.slice(entry.callsiteOffset).startsWith(entry.inputs[0]!.kind==="directory"?"read_dir":"read")).toBe(true);}expect(runtimeCargoObservationCurrentV1(observation!,context)).toBe(true);expect(runtimeCompilerResourceInputsV1(observation!,{...resource,callerUnit:null},context).verified).toBe(false);writeFileSync(join(inputs,"asset.txt"),"changed source");expect(runtimeCompilerResourceInputsV1(observation!,resource,context).verified).toBe(false);writeFileSync(join(inputs,"asset.txt"),"actual payload");writeFileSync(join(inputs,"new.txt"),"new metadata");expect(runtimeCompilerResourceInputsV1(observation!,resource,context).verified).toBe(false);
 const before=new Set(readdirSync(ledger));expect(await cargoStreamingStatus(["check","--manifest-path",manifest,"--locked","-p","compiler-observation-caller","--message-format=json"],directory,environment,30000)).toBe(0);const refreshedPath=readdirSync(ledger).find(name=>!before.has(name))!,[refreshed]=runtimeCargoProvenanceV1(readFileSync(join(ledger,refreshedPath),"utf8"));expect(runtimeCompilerResourceInputsV1(refreshed!,refreshed!.compilerResources[0]!,context).verified).toBe(true);expect(refreshed!.compilerResources[0]!.path).not.toBe(resource.path);
 writeFileSync(join(directory,"verified-receipt.json"),JSON.stringify({path,observation,evidence,refreshed},null,2));console.log("[DEBUG] real Cargo completed resource capture binds exact proc-macro/caller units; edited source/new directory entry/missing caller refused");
},60000);

 test("compiler resource groups preserve exact operation inputs across actual invocations",async()=>{
 const {runtimeMergeResourceReadOwnersV1}=await import("../🔎️verification/🟦️.ts");
 for(const row of JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")).compilerResourceOwnerGroups){const result=runtimeMergeResourceReadOwnersV1(row.groups);expect(result.findings.length>0).toBe(row.refused);if(!row.refused){const graph=new Graph({multi:true});for(const group of row.groups)for(const[source,owners]of Object.entries(group.owners)as any)for(const owner of owners)for(const input of owner.inputs){graph.mergeNode(source);graph.mergeNode(input.path);graph.addDirectedEdge(source,input.path);}expect(result.owners["helper.rs"][0].inputs.map((input:any)=>input.path).sort()).toEqual(row.paths);expect(graph.outNeighbors("helper.rs").sort()).toEqual(row.paths);const validate=new Ajv({strict:true}).addSchema(schema).getSchema(`${schema.$id}#/$defs/RuntimeResourceReadOwnerV1`)!;expect(validate(result.owners["helper.rs"][0]),JSON.stringify(validate.errors)).toBe(true);}}
 });

test("actual build reader records original typed directory metadata and exact internal operations",async()=>{
 const directory=join(process.env.SEMIO_TEST_ARTIFACT_DIR!,`build-resource-reader-${crypto.randomUUID()}`),inputs=join(directory,"inputs"),out=join(directory,"out");mkdirSync(inputs,{recursive:true});mkdirSync(out);writeFileSync(join(inputs,"asset.txt"),"actual build payload");const helper=resolve(process.cwd(),"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️resources/🦀️.rs"),source=join(directory,"main.rs"),binary=join(directory,process.platform==="win32"?"reader.exe":"reader");
 writeFileSync(source,`#[path=${JSON.stringify(helper)}]mod resources;fn main(){let mut observation=resources::ResourceObservationsV1::default();let input=std::path::Path::new(${JSON.stringify(inputs)});let out=std::path::Path::new(${JSON.stringify(out)});let entries=observation.read_dir(input).unwrap();assert_eq!(entries.len(),1);assert_eq!(observation.read(&input.join("asset.txt"),out).unwrap(),b"actual build payload");assert!(observation.read(&input.join("missing.txt"),out).is_err());observation.finish(out);}`);const compile=Bun.spawn(["rustc","--edition","2024",source,"-o",binary],{stdout:"pipe",stderr:"pipe"});expect(await compile.exited,await new Response(compile.stderr).text()).toBe(0);const run=Bun.spawn([binary],{stdout:"pipe",stderr:"pipe"});expect(await run.exited,await new Response(run.stderr).text()).toBe(0);
 const rows=readFileSync(join(out,"semio-runtime-resource-inputs.jsonl"),"utf8").trim().split("\n").map(line=>JSON.parse(line)),validate=new Ajv({strict:true}).addSchema(schema).getSchema(`${schema.$id}#/$defs/RuntimeBuildResourceObservationV1`)!;for(const row of rows)expect(validate(row),JSON.stringify(validate.errors)).toBe(true);const {cargoDirectoryEntriesV1}=await import("../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts"),listing=rows.find(row=>row.kind==="directory"),read=rows.find(row=>row.kind==="read");expect(listing.entries.map((entry:any)=>[entry.path.slice(entry.path.lastIndexOf("/")+1),entry.kind,entry.symlinkTarget])).toEqual(cargoDirectoryEntriesV1(inputs));const glob=(await import("fast-glob")).default;expect(listing.entries.map((entry:any)=>entry.path)).toEqual(await glob("*",{cwd:inputs,absolute:true}));expect(readFileSync(read.output,"utf8")).toBe("actual build payload");for(const row of rows)expect(readFileSync(row.operation.source,"utf8").split("\n")[row.operation.line-1]).toContain(`fs::${row.operation.name}`);expect(rows.some(row=>row.kind==="failed"&&row.path.endsWith("missing.txt"))).toBe(true);const {runtimeBuildResourceInputsV1}=await import("../🔎️verification/🟦️.ts");expect(runtimeBuildResourceInputsV1(rows.map(JSON.stringify).join("\n"),{root:process.cwd(),cwd:directory,outDirectory:out,read:path=>{try{return readFileSync(resolve(process.cwd(),path))}catch{return undefined}},directoryEntries:path=>readDirectoryMetadata(resolve(process.cwd(),path))?.map(([name,kind,symlinkTarget])=>({path:join(resolve(process.cwd(),path),name),kind,symlinkTarget}))}).findings.some(finding=>finding.code==="runtime-unresolved-edge")).toBe(true);
},30000);

test("actual guest Cargo build reads bind exact host operations and original metadata",async()=>{
 const root=process.cwd(),directory=join(process.env.SEMIO_TEST_ARTIFACT_DIR!,`build-read-cargo-${crypto.randomUUID()}`),inputs=join(directory,"inputs"),build=join(directory,"build"),manifest=join(directory,"Cargo.toml"),helper=resolve(root,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️resources/🦀️.rs");mkdirSync(inputs,{recursive:true});writeFileSync(join(inputs,"asset.txt"),"actual original bytes");writeFileSync(manifest,'[workspace]\n[package]\nname="build-read-oracle"\nversion="0.0.0"\nedition="2024"\nbuild="build.rs"\n[lib]\npath="lib.rs"\n');writeFileSync(join(directory,"build.rs"),`#[path=${JSON.stringify(helper)}]mod resources;fn main(){let mut observed=resources::ResourceObservationsV1::default();let source=std::path::Path::new(${JSON.stringify(inputs)});let out=std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());assert_eq!(observed.read_dir(source).unwrap().len(),1);let bytes=observed.read(&source.join("asset.txt"),&out).unwrap();assert_eq!(bytes,b"actual original bytes");observed.finish(&out);}`);writeFileSync(join(directory,"lib.rs"),'pub const VALUE:&str=include_str!(concat!(env!("OUT_DIR"),"/semio-runtime-resource-read-1.bin"));');
 const {cargoStreamingStatus}=await import("../../../🏃️process/🟦️.ts"),environment={...process.env,CARGO_TARGET_DIR:join(directory,"target"),CARGO_BUILD_BUILD_DIR:build,SEMIO_TEST_ARTIFACT_DIR:undefined};expect(await cargoStreamingStatus(["generate-lockfile","--manifest-path",manifest,"--offline"],directory,environment,10000)).toBe(0);expect(await cargoStreamingStatus(["check","--manifest-path",manifest,"--locked","--target","wasm32-unknown-unknown","--message-format=json"],directory,environment,30000)).toBe(0);
 const {runtimeCargoProvenanceV1,runtimeBuildResourceReadOwnersV1}=await import("../🔎️verification/🟦️.ts"),{cargoInputDigestV1}=await import("../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts"),ledger=join(build,"semio-cargo-provenance"),path=join(ledger,readdirSync(ledger)[0]!),[observation]=runtimeCargoProvenanceV1(readFileSync(path,"utf8")),resource=observation!.buildResources![0]!,validateReceipt=new Ajv({strict:true}).addSchema(schema).getSchema(`${schema.$id}#/$defs/RuntimeCargoBuildResourcesV1`)!,context={root,cwd:directory,outDirectory:resource.out_dir,read:(path:string)=>{try{return readFileSync(resolve(root,path))}catch{return undefined}},directoryEntries:(path:string)=>readDirectoryMetadata(resolve(root,path))?.map(([name,kind,symlinkTarget])=>({path:join(resolve(root,path),name),kind,symlinkTarget}))},mapped=runtimeBuildResourceReadOwnersV1(observation!,resource,context);expect(validateReceipt(resource),JSON.stringify(validateReceipt.errors)).toBe(true);expect(mapped.findings).toEqual([]);expect(mapped.verified).toBe(true);expect(Object.values(mapped.owners).flat()).toHaveLength(2);
 const helperPath=helper.slice(root.length+1),evidence=inspectRuntimeGraphV1([helperPath],{read:path=>{try{return readFileSync(resolve(root,path),"utf8")}catch{return undefined}},resourceReads:mapped.owners,directoryInputs:mapped.directories,resourceDigest:(path,kind)=>{const digest=cargoInputDigestV1(resolve(root,path));return digest.kind===kind?digest.sha256??undefined:undefined},fixtureCollections:["🧫️fixtures"]});expect(evidence.findings).toEqual([]);expect(evidence.nodes).toContain(inputs.slice(root.length+1));expect(evidence.nodes).toContain(join(inputs,"asset.txt").slice(root.length+1));const oracle=new Graph({multi:true});for(const edge of evidence.edges){oracle.mergeNode(edge.from);oracle.mergeNode(edge.to);oracle.addDirectedEdge(edge.from,edge.to);}expect(oracle.outNeighbors(helperPath).sort()).toEqual([inputs.slice(root.length+1),join(inputs,"asset.txt").slice(root.length+1)].sort());
 expect(runtimeBuildResourceReadOwnersV1({...observation!,units:observation!.units.filter(unit=>!unit.message.target.kind.includes("custom-build"))},resource,context).verified).toBe(false);writeFileSync(join(inputs,"new.txt"),"new directory metadata");expect(runtimeBuildResourceReadOwnersV1(observation!,resource,context).verified).toBe(false);writeFileSync(join(directory,"verified-receipt.json"),JSON.stringify({path,observation,mapped,evidence},null,2));
},60000);


test("original browser actor producer inputs require complete current owned policy and compiler bytes",async()=>{
 const factory=await import("../../../../../../💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts"),directory=join(process.env.SEMIO_TEST_ARTIFACT_DIR!,`actor-policy-${crypto.randomUUID()}`);mkdirSync(directory,{recursive:true});
 const program=JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")).actorProducerProgram,parse=Bun.spawn(["node","--input-type=module","--eval",'import {parse} from "@bytecodealliance/jco";process.stdout.write(Buffer.from(await parse(process.argv[1])).toString("hex"));',program.component],{stdout:"pipe",stderr:"pipe"});const [component,error,status]=await Promise.all([new Response(parse.stdout).text(),new Response(parse.stderr).text(),parse.exited]);expect(status,error).toBe(0);const actor=await factory.buildClosedBrowserActorArtifactV1(Buffer.from(component,"hex")),compiler=join(directory,"compiler.mjs");writeFileSync(compiler,actor.producer.compiler.bytes);
 const observation={policyCanonical:actor.policyCanonical,policySha256:actor.policySha256,runtime:actor.producer.runtime,compiler:{path:compiler,sha256:actor.producer.compiler.sha256,byteLength:actor.producer.compiler.byteLength},inputs:actor.producer.inputs};
 const document=JSON.parse(readFileSync(resolve(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧬️schema/🔣️.json"),"utf8")),admit=new Ajv({strict:true}).addSchema(document).getSchema(`${document.$id}#/$defs/BrowserActorProducerInputsV1`)!;expect(admit(observation),JSON.stringify(admit.errors)).toBe(true);
 for(const row of JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")).actorProducerChanges){let value=structuredClone(observation);if(row.name==="missing-input")value.inputs=value.inputs.slice(1);if(row.name==="foreign-input")value.inputs[0]={...value.inputs[0],path:compiler};if(row.name==="changed-policy")value.policySha256="0".repeat(64);if(row.name==="noncanonical-policy")value.policyCanonical=JSON.stringify(JSON.parse(value.policyCanonical),null,2);if(row.name==="changed-compiler")value.compiler.sha256="0".repeat(64);if(row.name==="changed-runtime")value.runtime.sha256="0".repeat(64);
 let accepted=false;try{const result=await factory.verifyBrowserActorProducerInputsV1(value);accepted=true;expect(result.inputs).toHaveLength(observation.inputs.length);expect(result.importInterfaces).toEqual(JSON.parse(value.policyCanonical).options.importInterfaces);}catch{}expect(accepted,row.name).toBe(row.accepted);}
 expect(new Bun.CryptoHasher("sha256").update(readFileSync(compiler)).digest("hex")).toBe(observation.compiler.sha256);expect(Buffer.from(await crypto.subtle.digest("SHA-256",new TextEncoder().encode(actor.policyCanonical))).toString("hex")).toBe(actor.policySha256);writeFileSync(join(directory,"original-producer.json"),JSON.stringify(observation));console.log("[DEBUG] actual actor factory original compiler/policy bytes survive; independent WebCrypto/Ajv refuse missing, foreign and changed inputs");
},120000);


test("selected actor ownership binds only one exact production child operation",async()=>{
 const {runtimeActorChildImportV1}=await import("../🔎️verification/🟦️.ts");for(const row of JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")).actorChildCalls){const files={"child.ts":row.source,"actor.mjs":"export const value=7;"},selection=runtimeActorChildImportV1(row.source,"child.ts","actor.mjs",createHash("sha256").update(files["actor.mjs"]).digest("hex"));expect(selection?.callsiteIndex??null).toBe(row.index);const transformed=new Bun.Transpiler({loader:"ts",define:{"import.meta.vitest":"undefined"},deadCodeElimination:true}).transformSync(row.source),parsed=ts.createSourceFile("child.ts",transformed,ts.ScriptTarget.Latest,true,ts.ScriptKind.TS),calls:ts.CallExpression[]=[];const visit=(node:ts.Node)=>{if(ts.isCallExpression(node)&&node.expression.kind===ts.SyntaxKind.ImportKeyword&&!ts.isStringLiteral(node.arguments[0]!))calls.push(node);ts.forEachChild(node,visit);};visit(parsed);if(selection){expect(calls[selection.callsiteIndex]!.arguments[0]!.getText(parsed)).toBe("moduleUrl");expect(selection.sourceSha256).toBe(new Bun.CryptoHasher("sha256").update(transformed).digest("hex"));}
 const evidence=inspectRuntimeGraphV1(["child.ts"],{read:path=>files[path],productionTests:"excluded",dynamicImports:selection?{"child.ts":[selection]}:{}});expect(evidence.findings.filter(row=>row.code==="runtime-unresolved-edge")).toHaveLength(row.unresolved);}
});


test("actor component custody requires the exact current package workspace invocation",async()=>{
 const {runtimeActorCargoUnitV1}=await import("../🔎️verification/🟦️.ts"),{selectedCargoArguments}=await import("../../../🗂️workspaces/🦀️cargo/🟦️.ts"),root=join(process.env.SEMIO_TEST_ARTIFACT_DIR!,`actor-workspace-${crypto.randomUUID()}`),data=JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")),layout=data.actorCargoWorkspace;
 const put=(path:string,text:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
 put("Cargo.toml",'[workspace]\nresolver="2"\nmembers=["host"]\nexclude=["hub"]\n[workspace.metadata.semio.repository]\nschema-version=1\nowner-manifests=["*/Cargo.toml"]\nmember-manifests=["host/Cargo.toml"]\nexclude-patterns=[]\n');put("host/Cargo.toml",'[package]\nname="host"\nversion="0.0.0"\n[lib]\npath="lib.rs"\n');put("host/lib.rs","");
 put("hub/Cargo.toml",'[workspace]\nresolver="2"\nmembers=["'+layout.member+'"]\n[workspace.metadata.semio.repository]\nschema-version=1\nmember-manifests=["'+layout.member+'/Cargo.toml"]\nexclude-patterns=[]\n');
 const manifest='[package]\nname="'+layout.package+'"\nversion="0.0.0"\n[lib]\npath="lib.rs"\n',packageManifest=join(root,layout.manifest);put(layout.manifest,manifest);put("hub/"+layout.member+"/lib.rs","");
 const selection={cargoPackage:layout.package,componentPath:join(root,"generation/component.wasm"),componentSha256:new Bun.CryptoHasher("sha256").update("actual component").digest("hex")},args=selectedCargoArguments(root,["rustc","-p",selection.cargoPackage,"--lib","--crate-type","cdylib","--target","wasm32-wasip2","--profile","wasm-release","--message-format=json"]),toml=(await import("@iarna/toml")).default;
 expect(toml.parse(manifest).package.name).toBe(selection.cargoPackage);expect(args[2]).toBe(packageManifest);const native=Bun.spawnSync(["cargo","metadata","--offline","--no-deps","--format-version","1","--manifest-path",packageManifest],{cwd:root,stdout:"pipe",stderr:"pipe"});expect(native.exitCode,native.stderr.toString()).toBe(0);const metadata=JSON.parse(native.stdout.toString());expect(metadata.workspace_root).toBe(join(root,"hub"));expect(metadata.packages[0].manifest_path).toBe(packageManifest);
 const admit=new Ajv({strict:true}).addSchema(schema).getSchema(`${schema.$id}#/$defs/RuntimeActorCargoSelectionV1`)!;expect(admit(selection),JSON.stringify(admit.errors)).toBe(true);
 for(const row of data.actorCargoChanges){const unit={message:{manifest_path:packageManifest,target:{kind:["lib"],src_path:join(root,"hub/"+layout.member+"/lib.rs"),name:"neutral_actor"},profile:{test:false},features:["default"]},artifacts:[{path:join(root,"original.wasm"),sha256:selection.componentSha256,stagedPath:selection.componentPath,stagedSha256:selection.componentSha256}]},observation={manifest:packageManifest,cwd:root,status:0,cancelled:false,args:[...args],units:[unit]};if(row.name==="wrong-args")observation.args.push("--all-features");if(row.name==="test-unit")unit.message.profile.test=true;if(row.name==="wrong-package")observation.args[observation.args.indexOf("-p")+1]="foreign";if(row.name==="wrong-stage")unit.artifacts[0].stagedPath=join(root,"foreign.wasm");if(row.name==="wrong-digest")unit.artifacts[0].sha256="0".repeat(64);if(row.name==="failed")observation.status=101;if(row.name==="testing-feature")unit.message.features.push("mutation-testing");if(row.name==="implicit-root"){observation.manifest=join(root,"Cargo.toml");observation.args.splice(1,2);}if(row.name==="foreign-unit-manifest")unit.message.manifest_path=join(root,"host/Cargo.toml");expect(!!runtimeActorCargoUnitV1(observation,selection,{root,read:path=>{try{return readFileSync(path,"utf8")}catch{return undefined}}}),row.name).toBe(row.accepted);}
});


test("ordinary selected Dev actor acquisition refuses an absent production ledger",async()=>{
 const {runtimeSelectedActorsV1}=await import("../🔎️verification/🟦️.ts"),root=process.cwd(),config=JSON.parse(readFileSync(join(owner,"🔣️.json"),"utf8")),directory=join(process.env.SEMIO_TEST_ARTIFACT_DIR!,`missing-actor-ledger-${crypto.randomUUID()}`),previous=process.env.OS_HUB_DATA;mkdirSync(join(directory,"trusted-catalog"),{recursive:true});process.env.OS_HUB_DATA=directory;
 try{await expect(runtimeSelectedActorsV1(root,config,{read:path=>{try{return readFileSync(resolve(root,path),"utf8")}catch{return undefined}},current:()=>false,fixtureCollections:["🧫️fixtures"],checkCancellation:()=>{}})).rejects.toThrow("current.json");const {LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES}=await import("../../../../../../../../🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts");expect(LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES.split(",")).toHaveLength(6);}finally{if(previous===undefined)delete process.env.OS_HUB_DATA;else process.env.OS_HUB_DATA=previous;}
});
