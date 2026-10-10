#!/usr/bin/env bun
/** 🧪️ Discovers canonical editor acceptance owners and runs their existing native laws through Nx. */
import { existsSync, readFileSync, readdirSync, statSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

let repository = import.meta.dir;
while (!existsSync(join(repository, "nx.json"))) {
  const parent = dirname(repository);
  if (parent === repository) throw Error("Repository root is unavailable");
  repository = parent;
}
const {receiveScriptProcessInvocation}=await import(join(repository,"🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts"));
await receiveScriptProcessInvocation(process.env,async original=>{
const control=original.control;
if(process.argv[2]==="source-oracle"){
  const source=process.argv[3]==="sqlite"?"🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🏛️ownership/🟦️.ts":process.argv[3]==="puzzle"?"✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts":undefined;
  const selectedSource=process.argv[3]==="step-history"?"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🧪️tests/🎛️history-inputs/🟦️.ts":source;
  const sources=process.argv[3]==="drawing-cold"?["✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️edit-selection/🧪️tests/🔬️unit/🟦️.ts", "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️canvas-tool/🟦️.ts", "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🧪️tests/🔬️unit/🟦️.ts"]:process.argv[3]==="step-references"?["✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔗️references/🧪️tests/🟦️.ts"]:process.argv[3]==="acceptance-arrays"?["🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🧪️tests/🔎️array-controls/🟦️.ts"]:process.argv[3]==="tool"?["🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️conformance/🟦️.ts","🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️node-graph-row-ownership/🟦️.ts"]:selectedSource?[selectedSource]:[];
  if(!sources.length)throw Error("source-oracle requires sqlite, puzzle, step-history, tool, drawing-cold, step-references or acceptance-arrays");
  const {runBudgetedTestCommand}=await import(join(repository,"🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
  await runBudgetedTestCommand(process.execPath,["test",...sources.map(source=>join(repository,source))],{cwd:repository,budgetMs:600000,throwOnFailure:true});
  process.exit(0);
}
const { runRepositoryCargoTests, runRepositoryTestCommand } = await import(join(repository, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts"));
const { runRepositoryCommand } = await import(join(repository,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts"));
if(process.argv[2]==="snapshot-clone-native"){
  await runRepositoryCargoTests(["semio-framework-os-kernel"],repository,control,["--lib",...process.argv.slice(3)],{...process.env,NEXTEST_SUCCESS_OUTPUT:"immediate",SEMIO_TEST_LEVEL:"long",SEMIO_TEST_ARTIFACT_DIR:join(import.meta.dir,"../🗑️generated/tools-execution")});
  process.exit(0);
}
if(process.argv[2]==="schema-retirement-native"){
  await runRepositoryCargoTests(["semio-framework-schema-validator"],repository,control,["--lib","original_compiled_validator_retires_recursive_pattern_fields_under_full_grants","--status-level","pass","--final-status-level","all","--","--nocapture"],{...process.env,NEXTEST_SUCCESS_OUTPUT:"immediate",SEMIO_TEST_LEVEL:"long",SEMIO_TEST_ARTIFACT_DIR:join(import.meta.dir,"../🗑️generated/core-execution")});
  process.exit(0);
}
if(process.argv[2]==="puzzle-domain-native"){
  const {runArtifactRustTests}=await import(join(repository,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts"));
  await runArtifactRustTests("semio-s-artifact-puzzle-2d",repository,process.argv.slice(3),["component-app-assembly"]);
  process.exit(0);
}
if(process.argv[2]==="step-history-oracle"){
  await runRepositoryCargoTests(["semio-s-artifact-stdio-step-test-oracle"],repository,control,["--lib","--features","oracles","committed_history_intents_match_independent_part21_reader_and_class_edits","--status-level","pass","--final-status-level","all","--","--nocapture"],{...process.env,NEXTEST_SUCCESS_OUTPUT:"immediate",SEMIO_TEST_LEVEL:"long",SEMIO_TEST_ARTIFACT_DIR:join(import.meta.dir,"../🗑️generated/tools-execution")});
  process.exit(0);
}
if(process.argv[2]==="architect-role-native"){
  await runRepositoryCargoTests(["semio-s-artifact-architect-program"],repository,control,["--lib","architect_native_sparse_diff_preserves_declared_roles_and_exact_text","--status-level","pass","--final-status-level","all","--","--nocapture"],{...process.env,NEXTEST_SUCCESS_OUTPUT:"immediate",SEMIO_TEST_LEVEL:"long",SEMIO_TEST_ARTIFACT_DIR:join(import.meta.dir,"../🗑️generated/tools-execution")});
  process.exit(0);
}
if(process.argv[2]==="sqlite-paged-native"){
  await runRepositoryCargoTests(["semio-framework-io-sqlite-snapshot"],repository,control,["--lib","history_edit_sqlite_paged_text_cells_keep_borrowed_measurement_allocation_free_and_cancel_owned_copy","--status-level","pass","--final-status-level","all","--","--nocapture"],{...process.env,NEXTEST_SUCCESS_OUTPUT:"immediate",SEMIO_TEST_LEVEL:"long",SEMIO_TEST_ARTIFACT_DIR:join(import.meta.dir,"../🗑️generated/tools-execution")});
  process.exit(0);
}
const roots = join(repository, "✏️s/🔌️plugins");
const files = (root: string): string[] => readdirSync(root, {withFileTypes:true}).flatMap(entry => {
  const path = join(root, entry.name);
  return entry.isDirectory() ? ["node_modules", "target", "dist", "🤖️generated", "🗑️generated"].includes(entry.name) ? [] : files(path) : entry.isFile() && entry.name.endsWith(".rs") ? [path] : [];
});
if (process.argv[2] === "rust-syntax") {
  const paths = process.argv.slice(3);
  if (!paths.length) throw Error("rust-syntax requires explicit source files or directories");
  const sources = paths.flatMap(path => { const source=resolve(repository,path);return statSync(source).isDirectory()?files(source):[source]; });
  for (const source of sources) {
    const result=Bun.spawnSync(["rustfmt","--edition","2021","--emit","stdout","--config","skip_children=true"],{stdin:readFileSync(source),stdout:"ignore",stderr:"pipe"});
    if(result.exitCode !== 0) {
      const diagnostic=new TextDecoder().decode(result.stderr), errors=diagnostic.split("\n").filter(line=>line.startsWith("error"));
      if(!errors.length || errors.some(line=>line !== "error[internal]: left behind trailing whitespace")) throw Error(`${source}: ${diagnostic}`);
      console.log(`Rustfmt parsed ${source}; formatter reported existing trailing whitespace`);
    }
  }
  console.log(`Rustfmt syntax parsed ${sources.length} explicit Rust owners`);
  process.exit(0);
}
const owners = readdirSync(roots).flatMap(plugin => {
  const artifacts = join(roots, plugin, "🗿️artifacts");
  if (!existsSync(artifacts)) return [];
  return readdirSync(artifacts).flatMap(artifact => {
    const root = join(artifacts, artifact), standards = join(root, "🏅️standards"), packageRoot = join(root, "📦️packages/🦀️rust");
    if (!existsSync(standards) || !existsSync(join(packageRoot, "Cargo.toml"))) return [];
    const registrations = files(standards).filter(path => ["history_edit_acceptance_law!","composed_child_history_law!"].some(name=>readFileSync(path,"utf8").includes(name)));
    if (!registrations.length) return [];
    const cargo = Bun.TOML.parse(readFileSync(join(packageRoot,"Cargo.toml"),"utf8")) as {package:{name:string};features?:Record<string,unknown>};
    const projectPath = ["package.json","📋️project.json"].map(name=>join(packageRoot,name)).find(existsSync);
    if (!projectPath) throw Error(`Missing Nx owner: ${root}`);
    const project = JSON.parse(readFileSync(projectPath,"utf8")).name as string;
    return [{root,project,crate:cargo.package.name,assembly:Object.hasOwn(cargo.features??{},"component-app-assembly"),registrations}];
  });
}).sort((a,b)=>a.root.localeCompare(b.root));
const [command, selection = "all", featureSelection = "all"] = process.argv.slice(2);
if (!["all","ungated","assembly"].includes(featureSelection)) throw Error("Feature selection must be all, ungated or assembly");
const crateSelection=selection.split(",");
if (!["all","stdio","non-stdio"].includes(selection) && (new Set(crateSelection).size!==crateSelection.length || crateSelection.some(crate=>!owners.some(owner=>owner.crate===crate)))) throw Error("Selection must name all, stdio, non-stdio or distinct authored native crates");
const selected = owners.filter(owner=>(selection === "all" || (selection === "stdio" || selection === "non-stdio" ? owner.root.includes("🗄️stdio") === (selection === "stdio") : crateSelection.includes(owner.crate))) && (featureSelection === "all" || owner.assembly === (featureSelection === "assembly")));
if(!selected.length)throw Error("Selected native assertion cohort has no authored registration");
const suffixes=new Set(["history_edits_end_to_end","history_edit_inputs_resolve","conflict_history_edits_end_to_end","child_history_edits_end_to_end"]);
console.log(`[DEBUG] history acceptance owners=${selected.length} registrations=${selected.reduce((sum,owner)=>sum+owner.registrations.length,0)}`);
if (command === "genesis-oracle") {
  await runRepositoryTestCommand(process.execPath,["test",join(import.meta.dir,"../deferred-genesis/🟦️.ts")],{cwd:repository});
} else if (command === "tool-oracle") {
  await runRepositoryTestCommand(process.execPath,["test",join(repository,"🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️conformance/🟦️.ts"),join(repository,"🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️node-graph-row-ownership/🟦️.ts")],{cwd:repository});
} else if (command === "census") {
  for(const owner of selected) console.log(`${owner.project}	${owner.crate}	assembly=${owner.assembly}	registrations=${owner.registrations.length}`);
} else if (command === "execute-group") {
  const scope=process.argv[5]??"history";
  if(!["history","named"].includes(scope))throw Error("Execution scope must be history or named");
  const filter=scope==="named"?["-E",`test(/(^|::)(${[...suffixes].join("|")})$/)`]:["history_edit"];
  console.log(`[DEBUG] history acceptance runtime scope=${scope}`);
  await runRepositoryCargoTests(selected.map(owner=>owner.crate),repository,control,["--lib",...(featureSelection==="assembly"?["--features","component-app-assembly"]:[]),...filter,"--no-fail-fast","--status-level","pass","--final-status-level","all","--","--nocapture"],{...process.env,NEXTEST_SUCCESS_OUTPUT:"immediate",SEMIO_TEST_LEVEL:"long",SEMIO_TEST_ARTIFACT_DIR:join(import.meta.dir,"../🗑️generated/tools-execution")});
} else if (command === "run") {
  let failed=false;
  for(const assembly of [false,true]){
    const group=selected.filter(owner=>owner.assembly===assembly);
    if(!group.length)continue;
    const expected=group.map(owner=>({crate:owner.crate,registrations:owner.registrations.flatMap(path=>[...readFileSync(path,"utf8").matchAll(/(?:history_edit_acceptance_law|composed_child_history_law)!\s*\(\s*"([^"\n]+)"\s*,\s*([^,]+),/gu)].map(match=>({kind:match[0].startsWith("history_edit")?"direct":"child",plugin:match[1],editor:match[2]?.trim(),source:path}))),direct:owner.registrations.reduce((sum,path)=>sum+[...readFileSync(path,"utf8").matchAll(/history_edit_acceptance_law!\s*\(/gu)].length,0),child:owner.registrations.reduce((sum,path)=>sum+[...readFileSync(path,"utf8").matchAll(/composed_child_history_law!\s*\(/gu)].length,0)}));
    let complete=false;
    const passes=new Map<string,Set<string>>(),refused=new Set<string>();
    const receipt=join(import.meta.dir,`../🗑️generated/tools-execution/assertions-${crateSelection.length>1?`selected-${Bun.hash(selection).toString(16)}`:selection}-${assembly?"assembly":"ungated"}-${Date.now()}.json`);
    console.log(`[DEBUG] history acceptance expected named assertions=${expected.reduce((sum,row)=>sum+row.direct*3+row.child,0)} crates=${group.length}`);
    try{
      await runRepositoryCommand(process.execPath,[import.meta.filename,"execute-group",selection,assembly?"assembly":"ungated","named"],repository,"history-native-named-assertions",Math.min(1800000,control.remainingMilliseconds()??1800000),{signal:control.signal,onLine:line=>{
        const plain=line.replace(/\x1b\[[0-9;]*m/gu,""),match=plain.match(/\b(PASS|FAIL|SKIP)\s+\[[^\]]+\]\s+(?:\([^)]*\)\s+)?(\S+)\s+(\S+)\s*$/u);
        if(!match||!suffixes.has(match[3]!.split("::").at(-1)!))return;
        const [,status,crate,name]=match;if(status!=="PASS"){refused.add(`${crate}::${name}`);return;}
        let names=passes.get(crate!);if(!names){names=new Set();passes.set(crate!,names);}names.add(name!);
      }});
      if(refused.size)throw Error(`Generic editor assertions refused or skipped: ${[...refused].join(", ")}`);
      for(const row of expected){
        const names=passes.get(row.crate)??new Set<string>(),directModules=new Map<string,Set<string>>(),children=new Set<string>();
        for(const name of names){const suffix=name.split("::").at(-1)!,module=name.slice(0,-suffix.length-2);if(suffix==="child_history_edits_end_to_end")children.add(module);else{let laws=directModules.get(module);if(!laws){laws=new Set();directModules.set(module,laws);}laws.add(suffix);}}
        if(directModules.size!==row.direct||children.size!==row.child||[...directModules.values()].some(laws=>laws.size!==3)||names.size!==row.direct*3+row.child)throw Error(`Incomplete named native assertions for ${row.crate}: expected direct=${row.direct} child=${row.child} laws=${row.direct*3+row.child}; actual direct=${directModules.size} child=${children.size} laws=${names.size}`);
      }
      const unselected=[...passes.keys()].filter(crate=>!expected.some(row=>row.crate===crate));if(unselected.length)throw Error(`Unselected generic assertion crates: ${unselected.join(", ")}`);
      complete=true;
      console.log(`[DEBUG] history acceptance actual named PASS assertions=${[...passes.values()].reduce((sum,names)=>sum+names.size,0)} matched every authored registration`);
    }catch(error){failed=true;console.error(error);}finally{mkdirSync(dirname(receipt),{recursive:true});writeFileSync(receipt,JSON.stringify({selection,assembly,expected,actual:[...passes].map(([crate,names])=>({crate,names:[...names].sort()})),refused:[...refused],complete},null,2)+"\n");console.log(`[DEBUG] Named assertion receipt ${receipt}`);}
  }
  if(failed)process.exitCode=1;
} else throw Error("Expected census or run");

});
