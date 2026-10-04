import {readFileSync,writeFileSync,mkdirSync,rmSync} from "node:fs";
import {resolve,join} from "node:path";
import ts from "typescript";
import {createHash} from "node:crypto";
const root=resolve(import.meta.dir,"../../../../../../../.."),ticket=resolve(import.meta.dir,".."),inputs=import.meta.dir;
const general="🧰️framework/📦️packages/🦀️rust",repo="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript",os="🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",plugin="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust";
if(process.argv[2]==="cleanup-generated"){rmSync(join(ticket,"🗑️generated","general-neutral-execution"),{recursive:true,force:true});console.log("[DEBUG] General source lane generated outputs removed; authored inputs and reports retained");process.exit(0);}
if(process.argv[2]==="progress-proof"){
 const neutral="🧰️framework/🔨️modules/🏃️process/🎛️owned-execution",path=`${neutral}/🟦️.ts`,after=readFileSync(join(root,path),"utf8"),before=after.replace('; onProgress?: (line: string) => void','').replace('startNativeProgress(label, 10_000, options.onProgress)','startNativeProgress(label)'),generated=join(ticket,"🗑️generated","general-neutral-execution"),digest=(source:string)=>createHash("sha256").update(source).digest("hex");mkdirSync(generated,{recursive:true});
 const sources=[{path,before,after,beforeSha256:digest(before),afterSha256:digest(after)},...["🧬️schema/📤️progress/🔣️.json","🧫️fixtures/📤️progress/🔣️.json","🧪️tests/📤️progress/🟦️.ts"].map(leaf=>{const path=`${neutral}/${leaf}`,after=readFileSync(join(root,path),"utf8");return {path,before:null,after,beforeSha256:null,afterSha256:digest(after)};})];
 writeFileSync(join(inputs,"progress-source-pairs.json"),JSON.stringify({sources,inverse:sources.map(source=>({path:source.path,before:source.after,after:source.before}))},null,2));
 const test=join(root,`${neutral}/🧪️tests/📤️progress/🟦️.ts`);await Promise.all([true,false].map(async before=>{const result=Bun.spawn([process.execPath,"test",test],{cwd:root,env:{...process.env,...(before?{SEMIO_OWNED_PROGRESS_SOURCE_PAIRS:join(inputs,"progress-source-pairs.json")}: {})},stdout:"pipe",stderr:"pipe"}),stdout=new Response(result.stdout).text(),stderr=new Response(result.stderr).text(),status=await result.exited,output=(await stdout)+(await stderr);writeFileSync(join(generated,before?"progress-before-red.txt":"progress-channel.txt"),output);console.log(`[DEBUG] owned progress ${before?"before-red":"current-green"} status=${status}`);if(status!==(before?1:0))process.exit(1);if(before&&(!output.includes("running elapsedMs=")||!output.includes("Expected")))process.exit(1);}));
 const options:ts.CompilerOptions={noEmit:true,strict:true,skipLibCheck:true,allowImportingTsExtensions:true,esModuleInterop:true,target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,types:["bun"],typeRoots:[join(root,"node_modules/@types")]},diagnostics=ts.getPreEmitDiagnostics(ts.createProgram([test,join(root,`${general}/📜️script.ts`)],options));writeFileSync(join(generated,"progress-strict.txt"),ts.formatDiagnosticsWithColorAndContext(diagnostics,{getCurrentDirectory:()=>root,getCanonicalFileName:value=>value,getNewLine:()=>"\n"}));console.log(`[DEBUG] actual progress and General strict diagnostics=${diagnostics.length}`);if(diagnostics.length)process.exit(1);process.exit(0);
}
if(process.argv[2]==="receipt"){
 const published=JSON.parse(readFileSync(join(inputs,"published-pairs.json"),"utf8")) as {pairs:{path:string;before:string;after:string;beforeSha256:string;afterSha256:string}[]},digest=(source:string)=>createHash("sha256").update(source).digest("hex"),nativeOwner="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration";
 const updated=published.pairs.map(pair=>{const current=readFileSync(join(root,pair.path),"utf8");return {...pair,publishedAfter:pair.after,publishedAfterSha256:pair.afterSha256,after:current,afterSha256:digest(current)};});
 const created=["🧬️schema/🗺️owner-test-manifests/🔣️.json","🧫️fixtures/🗺️owner-test-manifests/🔣️.json","🗺️owner-test-manifests/🟦️.ts","🧪️tests/🗺️owner-test-manifests/🟦️.ts"].map(leaf=>{const path=`${nativeOwner}/${leaf}`,after=readFileSync(join(root,path),"utf8");return {path,before:null,after,beforeSha256:null,afterSha256:digest(after)};});
 let progressSources:{path:string;before:string|null;after:string;beforeSha256:string|null;afterSha256:string}[]=[];try{progressSources=JSON.parse(readFileSync(join(inputs,"progress-source-pairs.json"),"utf8")).sources;}catch{}
 const rows=[...updated,...created,...progressSources];writeFileSync(join(inputs,"final-source-pairs.json"),JSON.stringify({rows,inverse:rows.map(row=>({path:row.path,before:row.after,after:row.before}))},null,2));
 const phase="🗑️generated/general-neutral-execution",generated=join(ticket,phase),logs=["current-red","proposed-green","neutral-cargo-port","neutral-capture","test-artifact-kind-source","test-fixture-ownership-source","test-command-ingress-consumer","test-general-execution-ownership","progress-before-red","progress-channel"].map(name=>{let source="";try{source=readFileSync(join(generated,`${name}.txt`),"utf8");}catch{}return {name,sha256:digest(source),tail:source.replace(/\u001b\[[0-9;]*m/gu,"").split("\n").filter(line=>/\d+ pass|\d+ fail|\d+ expect|successfully ran|\[DEBUG\]/u.test(line)).slice(-6).join("; ")};});
 const report=`# General Native Task Ownership Refactor\n\n[Full current source pairs, publication hashes and inverse](general-neutral-execution-inputs/final-source-pairs.json) cover ${rows.length} production closure files. New admission fixtures declare seven language-agnostic request vectors; the independent Ajv oracle checks closed request shape, and TypeScript parses the actual command/import roster.\n\nGeneral now calls neutral Cargo, Vitest, budgeted execution and asynchronous capture APIs under explicitly supplied owner policies. The native repository wrapper prepares each explicitly selected package manifest and injects distinct assertion policies. Core/deflate/SQLite package filters retain their exact manifest selections; async descriptor/typegen routes retain exit statuses and capture cancellation/progress.\n\nThe complete mixed fixture native roster is retained in Repo orchestration. OS artifact-kind and native SQLite admission belong OS; plugin snapshot laws belong Plugin. General retains the neutral surface witness and portable artifact grammar. Project/package/seed/generated launch routes call these actual owners, with no retired forwarding commands.\n\nThe initial Nx registration attempt refused before source execution because graph composition embedded the workspaceRoot cwd token. Localized General option repairs use the repository-standard dot cwd. All original command/manifest arguments are retained.\n\n| Evidence | Recorded Result | Output Hash |\n| --- | --- | --- |\n${logs.map(row=>`| ${row.name} | ${row.tail||"Not Yet Recorded"} | ${row.sha256} |`).join("\n")}\n\nProposed General/policy test strict diagnostics were zero. The Cargo-port check filtered out its compiler-bearing empty-selection law: no Cargo/rustc or native generator ran in this source lane. Native execution confirmation remains delegated to the sole native execution owner.\n`;
 writeFileSync(join(ticket,"GENERAL-NEUTRAL-EXECUTION-OWNERSHIP.md"),report+"\nCanonical preview stdout is preserved by an explicit neutral owned-process progress callback. The native Repo command owner selects stderr; ordinary process defaults remain unchanged. The retained before image fails the actual 10.2s structured-output process law after its stdout progress tick; the current implementation matches the independent Node process and JSON5 oracle, with Ajv and first-party schema admission.\n");console.log(`[DEBUG] General full closure receipt=${rows.length}`);process.exit(0);
}
if(process.argv[2]==="registered-source"||process.argv[2]==="registered-progress"){
 const generated=join(ticket,"🗑️generated","general-neutral-execution");mkdirSync(generated,{recursive:true});
 for(const target of process.argv[2]==="registered-progress"?["@semio-tech/repo-lib:test-general-execution-ownership"]:["@semio-tech/framework-rs:test-artifact-kind-source","@semio-tech/framework-rs:test-fixture-ownership-source","@semio-tech/framework-rs:test-command-ingress-consumer","@semio-tech/repo-lib:test-general-execution-ownership"]){
 const child=Bun.spawn([process.execPath,"nx","run",target,"--skip-nx-cache","--excludeTaskDependencies"],{cwd:root,env:{...process.env,NX_DAEMON:"false",NX_ISOLATE_PLUGINS:"false",SEMIO_TEST_ARTIFACT_DIR:generated},stdout:"pipe",stderr:"pipe"});
 const stdout=new Response(child.stdout).text(),stderr=new Response(child.stderr).text(),status=await child.exited,output=(await stdout)+(await stderr);writeFileSync(join(generated,`${target.split(":")[1]}.txt`),output);console.log(`[DEBUG] registered ${target} status=${status}`);if(status)process.exit(1);
 }
 const files=[join(root,`${general}/📜️script.ts`),join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/🗺️owner-test-manifests/🟦️.ts"),join(root,"🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🧪️tests/📤️progress/🟦️.ts")],options:ts.CompilerOptions={noEmit:true,strict:true,skipLibCheck:true,allowImportingTsExtensions:true,esModuleInterop:true,target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,types:["bun"],typeRoots:[join(root,"node_modules/@types")]},diagnostics=ts.getPreEmitDiagnostics(ts.createProgram(files,options));writeFileSync(join(generated,"final-strict.txt"),ts.formatDiagnosticsWithColorAndContext(diagnostics,{getCurrentDirectory:()=>root,getCanonicalFileName:value=>value,getNewLine:()=>"\n"}));console.log(`[DEBUG] final actual General and both policy laws strict diagnostics=${diagnostics.length}`);if(diagnostics.length)process.exit(1);
 process.exit(0);
}
const hash=(source:string)=>createHash("sha256").update(source).digest("hex");
const read=(path:string)=>readFileSync(join(root,path),"utf8");
type Pair={path:string;before:string;after:string;beforeSha256:string;afterSha256:string};
const pairs:Pair[]=[];
function change(path:string,edit:(source:string)=>string){const previous=pairs.find(pair=>pair.path===path),before=previous?.after??read(path),after=edit(before);if(before===after)return;if(previous){previous.after=after;previous.afterSha256=hash(after);}else pairs.push({path,before,after,beforeSha256:hash(before),afterSha256:hash(after)});}
function splice(source:string,before:string,after:string){if(!source.includes(before))throw Error(`Missing owned splice: ${before.slice(0,100)}`);return source.replace(before,after);}
const current=read(`${general}/📜️script.ts`),fixtureClass=current.slice(current.indexOf("/** 🧱️ Verifies portable ownership witnesses"),current.indexOf("/** 🪪️ Runs the portable artifact grammar"));
const artifactClass=current.slice(current.indexOf("/** 🪪️ Runs the portable artifact grammar"),current.indexOf("class TestScript"));
const policies=`/** 🔐️ Selects a policy only for an explicitly declared owning package manifest. */
function cargoPolicy(root:string,manifest:string){
 const path=resolve(root,manifest),policies=JSON.parse(process.env.SEMIO_CARGO_TEST_POLICIES??"[]") as unknown[];
 if(!Array.isArray(policies)||policies.length>64)throw Error("Invalid exact Cargo policy inventory");
 const policy=policies.map(value=>readCargoTestPolicyV1({...process.env,SEMIO_CARGO_TEST_POLICY:JSON.stringify(value)})).find(value=>value.manifestPath===path)??readCargoTestPolicyV1(process.env);
 if(policy.manifestPath!==path)throw Error(\`Missing exact Cargo policy: \${path}\`);
 return policy;
}
/** 🦀️ Runs a declared package under its supplied manifest policy and assertion budgets. */
async function testCargo(root:string,manifest:string,packages:string[],args:string[]){
 const policy=cargoPolicy(root,manifest);
 await runCargoTestsV1({manifestPath:policy.manifestPath,packages,cwd:root,extraArgs:args,environment:{...process.env,CARGO_TARGET_DIR:policy.targetDirectory}},policy);
}
/** 📥️ Captures one owned compiler operation with bounded evidence, progress and cancellation. */
async function captureCargo(root:string,args:string[],environment:Readonly<Record<string,string|undefined>>=process.env,emit=true){
 const policy=readCargoTestPolicyV1(environment);mkdirSync(policy.artifactDirectory,{recursive:true});
 const output=mkdtempSync(join(policy.artifactDirectory,"general-cargo-")),stopProgress=startNativeProgress("framework:cargo",10_000,line=>process.stderr.write(\`\${line}\\n\`));let cancelled=false;
 const stop=()=>{cancelled=true;};process.once("SIGINT",stop);process.once("SIGTERM",stop);
 const boundary=args.indexOf("--"),invocation=boundary<0?[...args,"--manifest-path",policy.manifestPath]:[...args.slice(0,boundary),"--manifest-path",policy.manifestPath,...args.slice(boundary)];
 try{const result=await captureOwnedProcess("cargo",invocation,{cwd:root,env:{...environment,CARGO_TARGET_DIR:policy.targetDirectory},budgetMs:policy.buildBudgetMs,maxOutputBytes:536870912,stdoutPath:join(output,"stdout.txt"),stderrPath:join(output,"stderr.txt"),cancelled:()=>cancelled});
 if(emit)process.stdout.write(result.stdout);process.stderr.write(result.stderr);
 if(result.reason!=="exit"||result.signal)throw Error(\`Framework Cargo stopped: \${result.reason} \${result.signal??""}\`);
 return result.status??1;
 }finally{stopProgress();process.off("SIGINT",stop);process.off("SIGTERM",stop);if(!policy.retainArtifacts)rmSync(output,{recursive:true,force:true});}
}
`;
change(`${general}/📜️script.ts`,source=>{
 source=splice(source,'import { resolveTestLevel }','import { testLevelBudgetMs, resolveTestLevel }');
 source=splice(source,'import { runRepositoryExactCargoLaws, runCargoLint, runRepositoryCargoTests, runCmdStatus, runRepositoryTestCommand, runVitest } from "../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";',`import { runCargoTestsV1, runCargoLintV1, readCargoTestPolicyV1 } from "../../🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {runBudgetedTestCommand} from "../../🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import {runVitestV1,readVitestPolicyV1} from "../../🔨️modules/🏃️process/🧪️testing/🧪️vitest/🟦️.ts";
import {captureOwnedProcess} from "../../🔨️modules/🏃️process/📥️capture/🟦️.ts";
import {startNativeProgress} from "../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";`);
 source=splice(source,fixtureClass,`/** 🧱️ Verifies the portable surface ownership witness against its hierarchy oracle. */
class FixtureOwnershipSourceScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
 if(segments.length)throw Error("test-fixture-ownership-source accepts no arguments");
 await runBudgetedTestCommand(process.execPath,["test",join(this.repoRoot,"🧰️framework/🧪️tests/🧱️fixture-ownership/🟦️.ts")],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
 }
}

`);
 source=splice(source,artifactClass,`/** 🪪️ Runs the portable artifact grammar oracle. */
class ArtifactKindSourceScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
 if(segments.length)throw Error("test-artifact-kind-source accepts no arguments");
 await runBudgetedTestCommand(process.execPath,["test",join(this.repoRoot,"🧰️framework/🔨️modules/🚪️io/🧪️tests/🪪️artifact-kind/🟦️.ts")],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
 }
}

`);
 source=source.replaceAll('runRepositoryTestCommand(', 'runBudgetedTestCommand(').replaceAll('{ cwd: this.repoRoot }','{ cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), throwOnFailure:true }').replaceAll('{ cwd: this.repoRoot, budgetMs: buildBudgetMs() }','{ cwd: this.repoRoot, budgetMs: buildBudgetMs(), throwOnFailure:true }');
 source=source.replaceAll('runRepositoryCargoTests(["semio-framework"], this.repoRoot,','testCargo(this.repoRoot, "🧰️framework/📦️packages/🦀️rust/Cargo.toml", ["semio-framework"],');
 source=splice(source,'await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");','await runVitestV1(readVitestPolicyV1(process.env,this.root),rest,"../../🧪️tests/🎚️config/🟦️.ts",process.env);');
 source=splice(source,'await runRepositoryCargoTests(["semio-framework-hash", "semio-framework-pixels", "semio-framework-intrinsic-size", "semio-framework-mesh-engine"], this.repoRoot, rest.length ? rest : ["--lib"]);',`for(const [module,name] of [["🔏️hash","hash"],["🔲️pixels","pixels"],["📏️intrinsic-size","intrinsic-size"],["🏗️mesh-engine","mesh-engine"]])await testCargo(this.repoRoot,\`🧰️framework/🔨️modules/\${module}/📦️packages/🦀️rust/Cargo.toml\`,[\`semio-framework-\${name}\`],rest.length?rest:["--lib"]);`);
 source=splice(source,'await runRepositoryCargoTests(["semio-framework-deflate"],this.repoRoot,rest.length?rest:["--lib","deflate_controlled_"]);','await testCargo(this.repoRoot,"🧰️framework/🔨️modules/🗜️deflate/📦️packages/🦀️rust/Cargo.toml",["semio-framework-deflate"],rest.length?rest:["--lib","deflate_controlled_"]);');
 const io=source.slice(source.indexOf('    if (segments[0] === "io")'),source.indexOf('    if (segments[0] !== undefined'));
 source=splice(source,io,'');
 source=splice(source,'await runRepositoryCargoTests(["semio-framework-io-sqlite-snapshot"], this.repoRoot, ["--lib", "--no-fail-fast"]);','await testCargo(this.repoRoot,"🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/📦️packages/🦀️rust/Cargo.toml",["semio-framework-io-sqlite-snapshot"],["--lib","--no-fail-fast"]);');
 source=splice(source,'["build", "-p", "semio-framework-io-sqlite-snapshot", "--bin", "semio-io-sqlite-snapshot-oracle"]','["build", "--manifest-path",cargoPolicy(this.repoRoot,"🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/📦️packages/🦀️rust/Cargo.toml").manifestPath,"-p", "semio-framework-io-sqlite-snapshot", "--bin", "semio-io-sqlite-snapshot-oracle"]');
 source=source.replaceAll('  run(): void {\n    const status = runCmdStatus(', '  async run(): Promise<void> {\n    const status = await captureCargo(');
 const descriptorStart=source.indexOf('    const status = await captureCargo('),descriptorEnd=source.indexOf('    if (status !== 0)',descriptorStart);
 source=source.slice(0,descriptorStart)+`    const status = await captureCargo(this.repoRoot,["test","-p","semio-framework","--lib","manifest::package_descriptor_value_codec_tests::package_descriptor_first_party_codec_preserves_serde_wire_and_required_fields","--","--exact"]);\n`+source.slice(descriptorEnd);
 source=splice(source,'  run(segments: string[]): void {\n    runCargoLint(["semio-framework"], this.root, segments);\n  }','  async run(segments:string[]):Promise<void>{\n    const policy=cargoPolicy(this.repoRoot,"🧰️framework/📦️packages/🦀️rust/Cargo.toml");\n    await runCargoLintV1({manifestPath:policy.manifestPath,packages:["semio-framework"],cwd:this.root,extraArgs:segments,environment:{...process.env,CARGO_TARGET_DIR:policy.targetDirectory}},policy);\n  }');
 source=splice(source,'function runTypegenExportTest(root: string, outPath?: string): void {','async function runTypegenExportTest(root: string, outPath?: string): Promise<void> {');
 const start=source.indexOf('  const status = runCmdStatus('),end=source.indexOf('  if (status !== 0)',start);
 source=source.slice(0,start)+`  const status = await captureCargo(root,["test","--features","typegen",TYPEGEN_TEST_FILTER],env);\n`+source.slice(end);
 source=source.replaceAll('  run(_segments: string[]): void {','  async run(_segments: string[]): Promise<void> {').replaceAll('    runTypegenExportTest(', '    await runTypegenExportTest(');
 source=splice(source,'const result = Bun.spawnSync(["cargo", "test", "--locked", "--features", "typegen", TYPEGEN_TEST_FILTER], { cwd: this.root, env: { ...process.env, CARGO_TARGET_DIR: join(temp, "target"), SEMIO_TYPEGEN_OUT: outPath }, stderr: "pipe", stdout: "pipe" });\n      if (result.exitCode !== 0) throw new Error(`framework preview export failed: ${result.stderr.toString()}`);',`const policy=readCargoTestPolicyV1(process.env),status=await captureCargo(this.root,["test","--locked","--features","typegen",TYPEGEN_TEST_FILTER],{...process.env,SEMIO_CARGO_TEST_POLICY:JSON.stringify({...policy,targetDirectory:join(temp,"target")}),SEMIO_TYPEGEN_OUT:outPath},false);\n      if(status!==0)throw Error(\`framework preview export failed: \${status}\`);`);
 source=source.replaceAll('const stopProgress=startNativeProgress("framework:cargo")','const stopProgress=startNativeProgress("framework:cargo",10_000,line=>process.stderr.write(`${line}\\n`))');
 source=splice(source,'import { tmpdir } from "node:os";\n','');
 source=splice(source,'    const temp = mkdtempSync(join(tmpdir(), "semio-framework-typegen-"));','    const artifacts=readCargoTestPolicyV1(process.env).artifactDirectory;mkdirSync(artifacts,{recursive:true});\n    const temp = mkdtempSync(join(artifacts, "semio-framework-typegen-"));');
 source=source.replaceAll('.register("test-artifact-kind", ArtifactKindTestScript)','.register("test-artifact-kind-source", ArtifactKindSourceScript)').replaceAll('.register("test-fixture-ownership", FixtureOwnershipTestScript)','.register("test-fixture-ownership-source", FixtureOwnershipSourceScript)');
 return splice(source,'//#region 🧹️WireRetirement',policies+'\n//#region 🧹️WireRetirement');
});
const native="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts";
change(native,source=>{
 source=splice(source,'import { buildBudgetMs }','import {nativeOwnerTestManifestRequestV1} from "./🗺️owner-test-manifests/🟦️.ts";\nimport { buildBudgetMs }');
 const before=source.slice(source.indexOf('      const cwdIndex=args.indexOf("--cwd")'),source.indexOf('      prepareCargoWorkspaceInvocation',source.indexOf('if(tool==="owner-command")')));
 source=splice(source,before,`      const request=nativeOwnerTestManifestRequestV1(args.slice(1)),path=resolve(this.repoRoot,request.manifest),cwd=resolve(this.repoRoot,request.cwd);\n`);
 source=splice(source,'      if (cargo.package?.name) Object.assign(env, { SEMIO_CARGO_TEST_POLICY: JSON.stringify(repositoryCargoTestPolicyV1(path,cwd)) });',`      if (cargo.package?.name) Object.assign(env, { SEMIO_CARGO_TEST_POLICY: JSON.stringify(repositoryCargoTestPolicyV1(path,cwd)) });
      const policies=request.testManifests.map(manifest=>{
        const selected=resolve(this.repoRoot,manifest);
        prepareCargoWorkspaceInvocation(this.repoRoot,["test","--manifest-path",selected],cwd);
        return repositoryCargoTestPolicyV1(selected,cwd);
      });
      if(new Set(policies.map(policy=>policy.manifestPath)).size!==policies.length)throw Error("Duplicate native test manifest authority");
      env.SEMIO_CARGO_TEST_POLICIES=JSON.stringify(policies);`);
 return splice(source,'await runOwnedCommand(command,args.slice(separator+2),cwd,"native:owner-command",0,{env});','await runOwnedCommand(request.command,[...request.args],cwd,"native:owner-command",0,{env});');
});
change(`${repo}/📜️script.ts`,source=>{
 source=splice(source,'repositoryVitestPolicyV1, runRepositoryTestCommand','repositoryVitestPolicyV1, runRepositoryExactCargoLaws, runRepositoryTestCommand');
 source=splice(source,'import { TEST_LEVEL_BUDGET_MS','import {buildBudgetMs} from "../../../../../../🔨️modules/🏃️process/⏱️budget/🟦️.ts";\nimport { TEST_LEVEL_BUDGET_MS');
 const owned=fixtureClass.replace('if (segments.length > 1 || (segments.length === 1 && segments[0] !== "source")) throw new Error("test-fixture-ownership accepts only source");','if(segments.length)throw Error("test-fixture-ownership accepts no arguments");').replace('    if (segments[0] === "source") return;\n','');
 const test=`/** 🗺️ Verifies exact manifest-policy admission and the General command ownership closure. */
class GeneralExecutionOwnershipScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
 if(segments.length)throw Error("test-general-execution-ownership accepts no arguments");
 await runRepositoryTestCommand(process.execPath,["test",join(this.repoRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/🗺️owner-test-manifests/🟦️.ts")],{cwd:this.repoRoot,env:repoTestArtifactEnvironment(this.repoRoot,"general-execution-ownership"),budgetMs:60000});
 }
}
`;
 source=splice(source,'const router = new ScriptRouter(import.meta.dir)',owned+'\n'+test+'\nconst router = new ScriptRouter(import.meta.dir)\n  .register("test-fixture-ownership",FixtureOwnershipTestScript).register("test-general-execution-ownership",GeneralExecutionOwnershipScript)');
 return source;
});
change(`${os}/📜️script.ts`,source=>{
 const start=artifactClass.indexOf('    const artifactDir'),nativeBody=artifactClass.slice(start,artifactClass.lastIndexOf('  }'));
 const owned=`/** 🪪️ Verifies the OS native artifact-kind admission against the neutral grammar. */
class ArtifactKindNativeScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
 if(segments.length)throw Error("test-artifact-kind accepts no arguments");
 ${nativeBody}
 }
}
/** 🪶️ Verifies native OS SQLite snapshot admission under its exact owner. */
class SnapshotSqliteAdmissionScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
 if(segments.length)throw Error("test-snapshot-sqlite-admission accepts no arguments");
 await runRepositoryCargoTests(["semio-framework-os-kernel"],this.repoRoot,["--test","sqlite_snapshot_native_admission"]);
 }
}
router.register("test-artifact-kind",ArtifactKindNativeScript).register("test-snapshot-sqlite-admission",SnapshotSqliteAdmissionScript);
`;
 source=splice(source,'import { runCargo,','import {buildBudgetMs} from "../../../../🔨️modules/🏃️process/⏱️budget/🟦️.ts";\nimport { runCargo,');
 return splice(source,'await runScriptMain(router, { defaultCommand: "check" });',owned+'\nawait runScriptMain(router, { defaultCommand: "check" });');
});
change(`${plugin}/📜️script.ts`,source=>{
 const owned=`/** 🪶️ Verifies the plugin-owned native SQLite snapshot laws. */
class SnapshotSqliteAdmissionScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
 if(segments.length)throw Error("test-snapshot-sqlite-admission accepts no arguments");
 await runRepositoryCargoTests(["semio-framework-plugin"],this.repoRoot,["--lib","sqlite_snapshot_"]);
 }
}
`;
 return splice(source,'const router = new ScriptRouter(import.meta.dir)',owned+'\nconst router = new ScriptRouter(import.meta.dir).register("test-snapshot-sqlite-admission",SnapshotSqliteAdmissionScript)');
});
const cargoScript="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts",repoScript=`${repo}/📜️script.ts`;
change(`${general}/📋️project.json`,source=>{
 const doc=JSON.parse(source);delete doc.targets["test-fixture-ownership"];delete doc.targets["test-artifact-kind"];delete doc.targets["test-snapshot-sqlite-io"];
 for(const [name,target] of Object.entries(doc.targets) as [string,any][]){
  if(name==="test-fixture-ownership-source"||name==="test-artifact-kind-source")target.options.command=`bun ./${"📜️script.ts"} ${name}`;
  const route=target.options.command.replace(/^bun \.\/📜️script.ts /u,""),sourceOnly=name.endsWith("-source")||name==="test-host-effect-invocation"||name==="test-command-ingress-consumer";
  target.options.cwd=".";
  if(sourceOnly)target.options.command=`bun ./${repoScript} owner-command --cwd ${general} -- bun ./📜️script.ts ${route}`;
  else{
   const selected=name==="test-core-modules"?["🔏️hash","🔲️pixels","📏️intrinsic-size","🏗️mesh-engine"].map(module=>`🧰️framework/🔨️modules/${module}/📦️packages/🦀️rust/Cargo.toml`):name==="test-deflate-encoding"?["🧰️framework/🔨️modules/🗜️deflate/📦️packages/🦀️rust/Cargo.toml"]:name.startsWith("test-snapshot-sqlite")?["🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/📦️packages/🦀️rust/Cargo.toml"]:[];
   target.options.command=`bun ./${cargoScript} native owner-command --manifest ${general}/Cargo.toml --cwd ${general}${selected.map(manifest=>` --test-manifest ${manifest}`).join("")} -- bun ./📜️script.ts ${route}`;
  }
 }
 return JSON.stringify(doc,null,2)+"\n";
});
for(const [owner,name] of [[repo,"test-fixture-ownership"],[repo,"test-general-execution-ownership"],[os,"test-artifact-kind"],[os,"test-snapshot-sqlite-admission"],[plugin,"test-snapshot-sqlite-admission"]]){
 change(`${owner}/📋️project.json`,source=>{
 const line=`    "${name}": {"executor":"nx:run-commands","cache":false,"options":{"cwd":"${owner}","command":"bun ./📜️script.ts ${name}"}},`;
 return splice(source,'  "targets": {','  "targets": {\n'+line);
 });
 change(`${owner}/package.json`,source=>{const doc=JSON.parse(source);doc.scripts??={};doc.scripts[name]=`nx run ${JSON.parse(read(`${owner}/📋️project.json`)).name}:${name}`;return JSON.stringify(doc,null,2)+"\n";});
}
for(const path of [".vscode/🧩️launch.seed.jsonc",".vscode/launch.json"]){
 change(path,source=>{
 source=source.replaceAll("@semio-tech/framework-rs:test-artifact-kind\"","@semio-tech/framework-os-kernel:test-artifact-kind\"").replaceAll("@semio-tech/framework-rs:test-fixture-ownership\"","@semio-tech/repo-lib:test-fixture-ownership\"").replaceAll("@semio-tech/framework-rs:test-snapshot-sqlite-io\"","@semio-tech/framework-os-kernel:test-snapshot-sqlite-admission\"").replaceAll("⚖️test-artifact-kind🧰️framework🦀️","⚖️test-artifact-kind🧰️os🦀️").replaceAll("⚖️test-fixture-ownership🧰️framework🦀️","⚖️test-fixture-ownership🦑️repo🟦️").replaceAll("⚖️test-snapshot-sqlite-io🧰️framework🦀️","⚖️test-snapshot-sqlite-admission🧰️os🦀️");
 const commands=[["@semio-tech/framework-plugin:test-snapshot-sqlite-admission","⚖️test-snapshot-sqlite-admission🧰️os🔌️plugin🦀️"],["@semio-tech/repo-lib:test-general-execution-ownership","⚖️test-general-execution-ownership🦑️repo🟦️"],["@semio-tech/repo-lib:test-fixture-ownership","⚖️test-fixture-ownership🦑️repo🟦️"],["@semio-tech/framework-os-kernel:test-snapshot-sqlite-admission","⚖️test-snapshot-sqlite-admission🧰️os🦀️"]].filter(([command])=>!source.includes(`bun nx run ${command}"`));
 const objects=commands.map(([command,name],index)=>JSON.stringify({name,type:"node-terminal",request:"launch",command:`bun nx run ${command}`,cwd:"${workspaceFolder}",presentation:{group:"4_gate",order:900.06101+index/100000}},null,2).split("\n").map(line=>`    ${line}`).join("\n")+",");
 return objects.length?splice(source,'  "configurations": [','  "configurations": [\n'+objects.join("\n")):source;
 });
}
rmSync(join(inputs,"proposed"),{recursive:true,force:true});
writeFileSync(join(inputs,"source-pairs.json"),JSON.stringify({pairs,inverse:pairs.map(({path,before,after})=>({path,before:after,after:before}))},null,2));
if(process.argv[2]==="verify"||process.argv[2]==="publish"){
 const generated=join(ticket,"🗑️generated","general-neutral-execution");mkdirSync(generated,{recursive:true});
 const source=join(root,`${general}/📜️script.ts`),proposal=pairs.find(pair=>pair.path===`${general}/📜️script.ts`)!,test=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/🗺️owner-test-manifests/🟦️.ts");
 for(const [phase,args,proposed] of [["current-red",["--test-name-pattern","General task ownership"],false],["proposed-green",[],true]] as const){const result=Bun.spawnSync([process.execPath,"test",test,...args],{cwd:root,env:{...process.env,SEMIO_TEST_ARTIFACT_DIR:generated,...(proposed?{SEMIO_GENERAL_SOURCE_PAIRS:join(inputs,"source-pairs.json")}:{}),SEMIO_COVERAGE:"0"},stdout:"pipe",stderr:"pipe"});writeFileSync(join(generated,`${phase}.txt`),result.stdout.toString()+result.stderr.toString());console.log(`[DEBUG] ${phase} status=${result.exitCode}`);if(result.exitCode!==(proposed?0:1))process.exit(1);}
 const options:ts.CompilerOptions={noEmit:true,strict:true,skipLibCheck:true,allowImportingTsExtensions:true,esModuleInterop:true,target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,types:["bun"],typeRoots:[join(root,"node_modules/@types")]},host=ts.createCompilerHost(options),original=host.getSourceFile;
 host.getSourceFile=(path,language,onError,create)=>path===source?ts.createSourceFile(path,proposal.after,language):original(path,language,onError,create);
 const diagnostics=ts.getPreEmitDiagnostics(ts.createProgram([source,test],options,host));writeFileSync(join(generated,"strict.txt"),ts.formatDiagnosticsWithColorAndContext(diagnostics,{getCurrentDirectory:()=>root,getCanonicalFileName:value=>value,getNewLine:()=>"\n"}));console.log(`[DEBUG] proposed General and policy law strict diagnostics=${diagnostics.length}`);if(diagnostics.length)process.exit(1);
 for(const [phase,testPath,args] of [["neutral-cargo-port","🧪️testing/🦀️cargo/🧪️tests/🟦️.ts",["--test-name-pattern","preserves exact owner"]],["neutral-capture","📥️capture/🧪️tests/🟦️.ts",[]]] as const){const result=Bun.spawnSync([process.execPath,"test",join(root,"🧰️framework/🔨️modules/🏃️process",testPath),...args],{cwd:root,env:{...process.env,SEMIO_TEST_ARTIFACT_DIR:generated},stdout:"pipe",stderr:"pipe"});writeFileSync(join(generated,`${phase}.txt`),result.stdout.toString()+result.stderr.toString());console.log(`[DEBUG] ${phase} status=${result.exitCode}`);if(result.exitCode)process.exit(1);}
}
if(process.argv[2]==="publish"){for(const pair of pairs)if(read(pair.path)!==pair.before)throw Error(`Changed before publication: ${pair.path}`);for(const pair of pairs)writeFileSync(join(root,pair.path),pair.after);writeFileSync(join(inputs,"published-pairs.json"),JSON.stringify({pairs,inverse:pairs.map(({path,before,after})=>({path,before:after,after:before}))},null,2));}
console.log(`[DEBUG] general direct neutral proposal=${pairs.length} published=${process.argv[2]==="publish"}`);
