/** 🧭️ Retains exact Pixels task source ownership and exercises a product-free source projection. */
import { existsSync, mkdirSync, readFileSync, rmSync, statSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { createHash } from "node:crypto";
import ts from "typescript";
import assert from "node:assert/strict";

const root = process.cwd(), inputs = import.meta.dir, ticket = dirname(inputs);
const generated = join(ticket,"🗑️generated/pixels-task-owner-execution");
const pixel = "🧰️framework/🔨️modules/🔲️pixels";
const descriptor = JSON.parse(readFileSync(join(root,pixel,"🧪️testing/🧭️ownership/🔣️.json"),"utf8"));
const hash = (source: string) => createHash("sha256").update(source).digest("hex");
const put = (path: string,value: unknown) => writeFileSync(path,JSON.stringify(value,null,2)+"\n");
const [command] = process.argv.slice(2);
const removeOwnedProperty = (source: string,keys: string[]) => {
  const document = ts.parseJsonText("input.json",source);
  let object = document.statements[0]!.expression as ts.ObjectLiteralExpression;
  let property: ts.PropertyAssignment | undefined;
  for (const key of keys) {
    property = object.properties.find(node=>ts.isPropertyAssignment(node) && (node.name as ts.StringLiteral).text===key) as ts.PropertyAssignment | undefined;
    if (!property) throw Error("Owned property missing: "+keys.join("/"));
    object = property.initializer as ts.ObjectLiteralExpression;
  }
  const start = source.lastIndexOf("\n",property!.getStart(document))+1,end = source.indexOf("\n",property!.end)+1;
  return source.slice(0,start)+source.slice(end);
};

if (command === "deletion") {
  const files = new Set<string>();
  const visit = (path: string) => {
    if (files.has(path)) return;
    if (!path.startsWith(root+"/") || path.includes("/node_modules/")) return;
    if (path.includes("/🛍️products/")) throw Error("Product source reached from neutral task: "+path);
    files.add(path);
    if (!/\.[cm]?tsx?$/.test(path)) return;
    const source = readFileSync(path,"utf8");
    const tree = ts.createSourceFile(path,source,ts.ScriptTarget.Latest,true);
    const literal = (node: ts.Node) => {
      if (ts.isStringLiteral(node) && node.text.startsWith(".")) {
        const child = resolve(dirname(path),node.text);
        if (existsSync(child) && statSync(child).isFile()) visit(child);
      }
      ts.forEachChild(node,literal);
    };
    literal(tree);
    for (const imported of ts.preProcessFile(source,true,true).importedFiles) {
      if (!imported.fileName.startsWith(".")) continue;
      const child = ts.resolveModuleName(imported.fileName,path,{moduleResolution:ts.ModuleResolutionKind.Bundler,resolveJsonModule:true,allowJs:true},ts.sys).resolvedModule?.resolvedFileName;
      if (child) visit(child);
    }
  };
  for (const path of [pixel+"/📜️script.ts",...descriptor.strictRoots,...descriptor.testRoots]) visit(join(root,path));
  for (const path of ["nx.json","package.json",pixel+"/📋️project.json",...descriptor.inputs.filter((path: string)=>path.endsWith(".ttf")||path.endsWith(".otf")).map((path: string)=>path.replace("{workspaceRoot}/",""))]) files.add(join(root,path));
  const snapshot = join(generated,"without-products");
  rmSync(snapshot,{recursive:true,force:true});
  mkdirSync(snapshot,{recursive:true});
  const rows = [...files].sort().map(path=>{
    const bytes = readFileSync(path),binary = /\.(?:ttf|otf)$/.test(path);
    return {path:relative(root,path),current:bytes.toString(binary?"base64":"utf8"),encoding:binary?"base64":"utf8",sha256:createHash("sha256").update(bytes).digest("hex")};
  });
  for (const path of files) {
    const target = join(snapshot,relative(root,path));
    mkdirSync(dirname(target),{recursive:true});
    writeFileSync(target,readFileSync(path));
  }
  if (!existsSync(join(snapshot,"node_modules"))) symlinkSync(join(root,"node_modules"),join(snapshot,"node_modules"),process.platform==="win32"?"junction":"dir");
  put(join(inputs,"deletion-source-closure.json"),{rows,productsPresent:existsSync(join(snapshot,"🧰️framework/🛍️products")),rustExecuted:false});
  const env = {...process.env,SEMIO_TEST_ARTIFACT_DIR:generated,SEMIO_TEST_ARTIFACTS_DIR:generated};
  delete env.SEMIO_CARGO_TEST_POLICY;
  delete env.SEMIO_CARGO_TEST_POLICIES;
  const child = Bun.spawn([process.execPath,join(snapshot,pixel,"📜️script.ts"),"test","typescript"],{cwd:snapshot,stdout:Bun.file(join(generated,"deletion.log")),stderr:Bun.file(join(generated,"deletion-stderr.log")),env});
  const status = await child.exited;
  put(join(inputs,"deletion-result.json"),{status,productsPresent:existsSync(join(snapshot,"🧰️framework/🛍️products")),sourceFiles:files.size,rustExecuted:false});
  console.log(`[DEBUG] Pixels product-free retained workload status=${status}; sourceFiles=${files.size}`);
  process.exitCode = status;
} else if (command === "capture") {
  const before = JSON.parse(readFileSync(join(inputs,"full-pre-current-inverse.json"),"utf8"));
  const paths = new Set<string>(before.rows.map((row: {path:string})=>row.path));
  for (const path of ["🟦️.ts","🔣️.json","🧬️schema/🔣️.json","🧫️fixtures/🔣️.json","🧪️tests/🟦️.ts"]) paths.add(pixel+"/🧪️testing/🧭️ownership/"+path);
  for (const path of ["📜️script.ts","📋️project.json","package.json"]) paths.add(pixel+"/📦️packages/🦀️rust/"+path);
  const host = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing";
  for (const path of ["📜️script.ts","🧪️tests/🧭️ownership/🟦️.ts","🧪️tests/🧭️ownership/🧬️schema/🔣️.json","🧪️tests/🧭️ownership/🧫️fixtures/🔣️.json"]) paths.add(host+"/"+path);
  const rows = [...paths].map(path=>{
    const current = readFileSync(join(root,path),"utf8"),old = before.rows.find((row: {path:string})=>row.path===path);
    let inverse = old?.current??null;
    if (path.endsWith("⚛️react/📦️packages/🟦️typescript/📋️project.json")) inverse = removeOwnedProperty(current,["targets","paint2d-editing-check"]);
    else if (path.endsWith("⚛️react/📦️packages/🟦️typescript/package.json")) inverse = removeOwnedProperty(current,["scripts","paint2d-editing-check"]);
    else if (path.endsWith("⚛️react/📦️packages/🟦️typescript/📜️script.ts")) inverse = current;
    else if (path.startsWith(".vscode/")) {
      const document = ts.parseJsonText("launch.json",current),object = document.statements[0]!.expression as ts.ObjectLiteralExpression;
      const configurations = (object.properties.find(node=>ts.isPropertyAssignment(node) && (node.name as ts.StringLiteral).text==="configurations") as ts.PropertyAssignment).initializer as ts.ArrayLiteralExpression;
      const own = configurations.elements.find(node=>ts.isObjectLiteralExpression(node) && node.properties.some(property=>ts.isPropertyAssignment(property) && (property.name as ts.StringLiteral).text==="name" && (property.initializer as ts.StringLiteral).text==="🧪️test🖌️Paint2dHost✍️editing🟦️"));
      if (!own) throw Error("Owned launch object missing from current authority: "+path);
      const start = current.lastIndexOf("\n",own.getStart(document))+1,end = current.indexOf("\n",own.end)+1;
      inverse = (current.slice(0,start)+current.slice(end)).replaceAll("semio-framework-pixels:test","@semio-tech/pixels:test-rust");
    }
    return {path,current,sha256:hash(current),inverse,preSha256:old?.sha256??null};
  });
  put(join(inputs,"full-current-inverse.json"),{rows,rustExecuted:false});
  console.log(`[DEBUG] Pixels current source closure retained ${rows.length} full bodies`);
} else if (command === "native-observation") {
  const before = JSON.parse(readFileSync(join(inputs,"full-pre-current-inverse.json"),"utf8")).rows.find((row: {path:string})=>row.path===pixel+"/📜️script.ts").current as string;
  const cargo = join(root,"🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts"),entrypoint = join(root,"🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts");
  const original = before.replace(/from "(\.[^"]+)"/g,(_,path: string)=>"from "+JSON.stringify(resolve(root,pixel,path))).replaceAll("import.meta.dir",JSON.stringify(join(root,pixel)));
  const preDirectory = join(generated,"native-observation-original");
  mkdirSync(preDirectory,{recursive:true});
  writeFileSync(join(preDirectory,"📜️script.ts"),original);
  const observe = async (path: string,args: string[]) => {
    const source = `import {mock} from "bun:test";const calls=[];const cargo=await import(${JSON.stringify(cargo)});mock.module(${JSON.stringify(cargo)},()=>({...cargo,readCargoTestPolicyV1:()=>({source:"unchanged-policy"}),runCargoTestsV1:async(request,policy)=>calls.push({request,policy})}));mock.module(${JSON.stringify(entrypoint)},()=>({runScriptMain:async(router)=>router.run(${JSON.stringify(args)})}));await import(${JSON.stringify(path)});console.log(JSON.stringify(calls));`;
    const child = Bun.spawn([process.execPath,"-e",source],{cwd:root,env:process.env,stdout:"pipe",stderr:"pipe"});
    const [stdout,stderr,status] = await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
    assert.equal(status,0,stderr);
    return JSON.parse(stdout);
  };
  const cases = [];
  for (const args of [[],["--lib","affine_sampling","--","--nocapture"],["--lib","png_decode","--","--nocapture"],["--lib","image_sources","--","--nocapture"],["--offline","--features","retained-feature","--no-default-features"]]) {
    const originalCalls = await observe(join(preDirectory,"📜️script.ts"),["test","rust",...args]),currentCalls = await observe(join(root,pixel,"📦️packages/🦀️rust/📜️script.ts"),["test",...args]);
    assert.deepEqual(currentCalls,originalCalls);
    cases.push({args,originalCalls,currentCalls,equal:true});
  }
  put(join(inputs,"native-task-observation.json"),{cases,oracle:"node:assert/strict request equality on actual old/current script dispatch",cargoExecuted:false,rustExecuted:false});
  console.log(`[DEBUG] Pixels canonical native owner preserved all ${cases.length} original request/feature vectors without running Cargo`);
} else if (command === "missing-owner") {
  const host = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing";
  const closure = JSON.parse(readFileSync(join(inputs,"deletion-source-closure.json"),"utf8"));
  const cases = [];
  for (const missing of ["🟦️.ts","🧪️tests/🟦️.ts"]) {
    const snapshot = join(generated,"missing-owner",missing==="🟦️.ts"?"source":"tests");
    rmSync(snapshot,{recursive:true,force:true});
    for (const row of closure.rows) {
      const target = join(snapshot,row.path);
      mkdirSync(dirname(target),{recursive:true});
      writeFileSync(target,Buffer.from(row.current,row.encoding));
    }
    for (const path of ["📜️script.ts","🟦️.ts","🧪️tests/🟦️.ts","🧪️tests/🧭️ownership/🟦️.ts"]) {
      if (path===missing) continue;
      const target = join(snapshot,host,path);
      mkdirSync(dirname(target),{recursive:true});
      writeFileSync(target,readFileSync(join(root,host,path)));
    }
    symlinkSync(join(root,"node_modules"),join(snapshot,"node_modules"),process.platform==="win32"?"junction":"dir");
    const child = Bun.spawn([process.execPath,join(snapshot,host,"📜️script.ts"),"test"],{cwd:snapshot,env:{...process.env,SEMIO_TEST_ARTIFACT_DIR:generated,SEMIO_TEST_ARTIFACTS_DIR:generated},stdout:"pipe",stderr:"pipe"});
    const [stdout,stderr,status] = await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
    assert.notEqual(status,0);
    assert.match(stderr,/ENOENT/);
    assert.ok(stderr.includes(join(snapshot,host,missing)));
    assert.equal(existsSync(join(snapshot,host,missing)),false);
    cases.push({missing,status,source:readFileSync(join(root,host,"📜️script.ts"),"utf8"),stderrHash:hash(stderr),stdoutHash:hash(stdout),ownedFilePresent:false,compilerStarted:false});
  }
  put(join(inputs,"missing-owned-file-result.json"),{cases,oracle:"node:fs actual absent owned file and actual unchanged script refusal before compilation",cargoExecuted:false,rustExecuted:false});
  console.log(`[DEBUG] Pixels OS source owner refused both real absent owned files before starting compilation`);
} else if (command === "registered-graph") {
  const source = readFileSync(join(generated,"registered-project.json"),"utf8"),project = JSON.parse(source),target = project.targets["test-typescript"];
  assert.equal(target.options.command,"bun ./📜️script.ts test typescript");
  assert.equal(target.options.cwd,pixel);
  assert.equal(project.metadata?.nativeRoot,undefined);
  assert.equal(target.dependsOn,undefined);
  assert.deepEqual(Object.keys(project.targets),["test-typescript"]);
  assert.equal(target.inputs.some((input: unknown)=>typeof input==="string" && /native|🛍️products/.test(input)),false);
  const commands = target.inputs.filter((input: unknown)=>typeof input==="string" && input.startsWith("commandSources")).flatMap((input: string)=>project.namedInputs[input]);
  assert.ok(commands.length);
  assert.equal(commands.some((input: string)=>/🛍️products|🦀️cargo/.test(input)),false);
  put(join(inputs,"registered-task-ownership-law.json"),{command:target.options.command,cwd:target.options.cwd,inputPaths:target.inputs,commandSourcePaths:commands,nativeWrapperPresent:false,nativePreparationPresent:false,oracle:"actual Nx projected project plus node:assert exact source ownership law",projectSha256:hash(source),cargoExecuted:false});
  console.log(`[DEBUG] Pixels registered neutral task has only source-owned command/input paths and no native preparation`);
} else throw Error("Expected deletion, capture, native-observation, missing-owner or registered-graph");
