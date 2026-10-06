import { readFileSync, writeFileSync, mkdirSync, renameSync, existsSync, symlinkSync, realpathSync, readdirSync } from "node:fs";
import { resolve, join, basename, relative } from "node:path";
import assert from "node:assert/strict";

const workspace=resolve(import.meta.dir,"../../../../../../../..");
const ticket=resolve(import.meta.dir,".."),generated=join(ticket,"🗑️generated");
mkdirSync(generated,{recursive:true});
const aliases:Record<string,string>={dashboard:"🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard",cli:"🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli",bootstrap:"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap"};
const path=(value:string)=>resolve(workspace,value.replace(/^(dashboard|cli|bootstrap)(?=\/)/,name=>aliases[name]!));
function edit(file:string,change:(text:string)=>string):void {
 const target=path(file),before=readFileSync(target,"utf8"),after=change(before.replace(/\r\n/g,"\n"));
 assert.notEqual(after,before,`No change: ${file}`);
 const staged=join(generated,`stage-${Date.now()}-${basename(target)}`);writeFileSync(staged,after);
 assert.equal(readFileSync(target,"utf8"),before,`Concurrent change: ${file}`);renameSync(staged,target);
}
if(process.argv[2]==="red")edit("dashboard/🖥️terminal/🧪️tests/🔬️unit/🦀️.rs",source=>source.replace('assert!(matches!(dashboard.windows[0].body, WindowBody::Language { .. }));','assert!(matches!(dashboard.windows[0].body, WindowBody::Wizard { .. }), "ordinary startup must not ask for language");'));
if(process.argv[2]==="apply"){
 const edits=JSON.parse(readFileSync(join(import.meta.dir,process.argv[3]!),"utf8"));
 for(const row of edits){if(row.source){const destination=path(row.path);mkdirSync(resolve(destination,".."),{recursive:true});const after=readFileSync(join(import.meta.dir,row.source),"utf8");try{edit(row.path,()=>after);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;writeFileSync(destination,after);}}else edit(row.path,source=>{for(const pair of row.replace){const before=pair.fromFile?readFileSync(join(import.meta.dir,pair.fromFile),"utf8"):pair.from,after=pair.toFile?readFileSync(join(import.meta.dir,pair.toFile),"utf8"):pair.to;assert.ok(source.includes(before),`Missing anchor: ${row.path}: ${before.slice(0,90)}`);source=pair.all?source.replaceAll(before,after):source.replace(before,after);}return source;});}
}
if(process.argv[2]==="tail"){
 const name=process.argv[3]!;assert.equal(name,basename(name));const value=readFileSync(join(generated,name),"utf8").replace(/\0/g,"").replace(/\x1b\[[0-?]*[ -/]*[@-~]/g,"");console.log(value.slice(-6000));
}
if(process.argv[2]==="tree")edit("dashboard/🌳️command-tree/🦀️.rs",source=>{
 source=source.replaceAll('#[derive(Debug, Clone, PartialEq, Eq)]','#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]');
 source=source.replace('pub fn discover(root: &Path) -> CommandNode {','pub fn discover(root: &Path) -> CommandNode { discover_cancellable(root, &std::sync::atomic::AtomicBool::new(false)) }\n\n/// ⏳️ Discovers current commands with cooperative cancellation.\npub fn discover_cancellable(root: &Path, cancelled: &std::sync::atomic::AtomicBool) -> CommandNode {');
 source=source.replace('collect_project_targets(root, root, &mut trie);','collect_project_targets(root, root, &mut trie, cancelled);').replace('collect_inferred_targets(root, &mut trie);','collect_inferred_targets(root, &mut trie, cancelled);');
 source=source.replace('fn collect_project_targets(root: &Path, dir: &Path, trie: &mut TrieNode) {','fn collect_project_targets(root: &Path, dir: &Path, trie: &mut TrieNode, cancelled: &std::sync::atomic::AtomicBool) {\n    if cancelled.load(std::sync::atomic::Ordering::Relaxed) { return; }').replace('collect_project_targets(root, &path, trie);','collect_project_targets(root, &path, trie, cancelled);');
 source=source.replace('fn collect_inferred_targets(root: &Path, trie: &mut TrieNode) {','fn collect_inferred_targets(root: &Path, trie: &mut TrieNode, cancelled: &std::sync::atomic::AtomicBool) {');
 source=source.replace('let file = match fs::File::open(&path)', 'if cancelled.load(std::sync::atomic::Ordering::Relaxed) { return; }\n        let file = match fs::File::open(&path)');
 source=source.replace('let reader = std::io::BufReader::new(file);','let reader = std::io::BufReader::with_capacity(65536, CancellationReader { file, cancelled });');
 const reader='struct CancellationReader<\'a> { file: fs::File, cancelled: &\'a std::sync::atomic::AtomicBool }\nimpl std::io::Read for CancellationReader<\'_> {\n    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {\n        if self.cancelled.load(std::sync::atomic::Ordering::Relaxed) { return Err(std::io::Error::new(std::io::ErrorKind::Interrupted, "command discovery cancelled")); }\n        std::io::Read::read(&mut self.file, buffer)\n    }\n}\n\n';
 source=source.replace('#[derive(serde::Deserialize)]\nstruct NxCommandGraph',reader+'#[derive(serde::Deserialize)]\nstruct NxCommandGraph');
 const first=source.indexOf('fn inject_playground_dev('),last=source.indexOf('fn verb_rank(',first);assert.ok(first>0&&last>first);source=source.slice(0,first)+readFileSync(join(import.meta.dir,"📓️playgrounds.rs.md"),"utf8")+source.slice(last);
 const seed='/// 🌱️ Immediate known launch commands without a recursive source walk or Nx graph.\npub fn seed(root: &Path) -> CommandNode {\n    let mut trie = TrieNode::default();\n    collect_workspace_scripts(root, &mut trie);\n    inject_playground_dev(root, &mut trie);\n    let mut tree = trie.into_command_node("root", "semio"); sort_tree(&mut tree, 0); tree\n}\n';
 source=source.replace('// #endregion 🔖️Discover',seed+'// #endregion 🔖️Discover');return source;
});
if(process.argv[2]==="native"){
 edit("dashboard/🌳️command-tree/🦀️.rs",source=>source.replace('#[derive(serde::Deserialize)]\nstruct NxCommandGraph',`struct CancellationReader<'a> { file: fs::File, cancelled: &'a std::sync::atomic::AtomicBool }
impl std::io::Read for CancellationReader<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        if self.cancelled.load(std::sync::atomic::Ordering::Relaxed) { return Err(std::io::Error::new(std::io::ErrorKind::Interrupted, "command discovery cancelled")); }
        std::io::Read::read(&mut self.file, buffer)
    }
}

#[derive(serde::Deserialize)]
struct NxCommandGraph`));
 edit("dashboard/📦️packages/🦀️rust/🦀️.rs",source=>source.replace('#[path = "../../🖥️terminal/🦀️.rs"]','#[path = "../../⚙️preferences/🦀️.rs"]\npub mod preferences;\n\n#[path = "../../📚️inventory/🦀️.rs"]\npub mod inventory;\n\n#[path = "../../🖥️terminal/🦀️.rs"]'));
 edit("cli/🦀️.rs",source=>source.replace('if argv.is_empty() {\n        return terminal::run(&root);\n    }','if argv.is_empty() || argv.first().is_some_and(|argument| argument.starts_with("--")) {\n        return terminal::run_with(&root, &args::parse(&std::iter::once("dashboard".into()).chain(argv.iter().cloned()).collect::<Vec<_>>()));\n    }').replace('"daemon" => daemon::run', '"dashboard" => terminal::run_with(&root, &parsed),\n        "preferences" => semio_framework_repo_dashboard::preferences::run(&root, &parsed),\n        "repo-view" => command_tree::run_action(&root, &parsed),\n        "daemon" => daemon::run'));
 edit("dashboard/🌳️command-tree/🦀️.rs",source=>source.replace('// #region 🔖️Command\n','// #region 🔖️Command\n/// 🦀️ Executes an owned repo action in a managed process so expensive queries remain cancellable.\npub fn run_action(root: &Path, parsed: &crate::args::ParsedArgs) -> i32 {\n    match parsed.flag("action").and_then(|value| serde_json::from_str::<RepoAction>(value).ok()) {\n        Some(action) => { println!("{}", action.execute(root)); 0 },\n        None => { eprintln!("[dashboard] invalid repo action"); 2 }\n    }\n}\n\n'));
 edit("dashboard/🖥️terminal/🦀️.rs",source=>{
  source=source.replace('Language { widget: NodeId },\n    Wizard { widget: NodeId, cursor: Vec<usize> },','Overview { widget: NodeId },\n    Launcher { widget: NodeId },\n    Settings { widget: NodeId },');
  const first=source.indexOf('// #region 🔖️TreeNav'),last=source.indexOf('// #region 🔖️Keys',first);source=source.slice(0,first)+source.slice(last);
  source=source.replace('locale: Option<Locale>,','locale: Locale,').replace('Some(Locale::English) => en.into(), Some(Locale::German) => de.into(), None => format!("{en} / {de}")','Locale::English => en.into(), Locale::German => de.into()');
  source=source.replace('tree: CommandNode,','tree: CommandNode,\n    commands: Vec<(String, CommandLeaf)>,\n    inventory: Option<crate::inventory::Job>,\n    inventory_status: String,\n    preferences: crate::preferences::Preferences,\n    preference_path: PathBuf,\n    shared_preferences: bool,\n    saving_preferences: Option<std::sync::mpsc::Receiver<(crate::preferences::Change, std::io::Result<u64>)>>,');
  function section(start:string,end:string,file:string){const first=source.indexOf(start),last=source.indexOf(end,first);assert.ok(first>=0&&last>first,`terminal section ${start}`);source=source.slice(0,first)+readFileSync(join(import.meta.dir,file),"utf8")+source.slice(last);}
  section('    fn refresh_wizard_for(', '    fn open_output(', '📓️terminal-methods.rs.md');
  section('    fn attach_wizard(', '    fn add_wizard_window(', '📓️terminal-views.rs.md');
  section('    fn handle_wizard_signal(', '    fn poll_sessions(', '📓️terminal-selection.rs.md');
  section('/// 🎛️ Interactive semio dashboard:', '    loop {\n        let output_changed', '📓️terminal-start.rs.md');
  source=source.replaceAll('WindowBody::Wizard { widget, .. } | WindowBody::Language { widget }','WindowBody::Launcher { widget } | WindowBody::Overview { widget } | WindowBody::Settings { widget }');
  source=source.replace('std::thread::spawn(move || { let _ = sender.send(Connection::connect(&root)); });',`std::thread::spawn(move || {
                let result = Connection::connect(&root).or_else(|_| {
                    let executable = std::env::current_exe()?;
                    if crate::daemon::supervisor::start_detached(&root, &executable) != 0 { return Err(std::io::Error::other("daemon startup failed")); }
                    Connection::connect(&root)
                });
                let _ = sender.send(result);
            });`);
  source=source.replace('let output_changed = dash.poll_sessions(&mut tui);','let output_changed = dash.poll_sessions(&mut tui) | dash.poll_inventory(&mut tui);');
  source=source.replace('for session in sessions { self.update_session(tui, session); }','for session in sessions { self.update_session(tui, session); }\n                    self.refresh_views(tui);');
  source=source.replace('self.remount(tui);\n    }\n\n    fn remount(', 'self.remount(tui);\n        self.refresh_views(tui);\n    }\n\n    fn remount(');
  section('    fn show_repo_output(', '    fn send(', '📓️repo-output.rs.md');
  source=source.replace('dash.light = !dash.light;\n                                tui.set_appearance(if dash.light { AppearanceName::Light } else { AppearanceName::Dark });', 'dash.queue_preference(crate::preferences::Change { appearance: Some(if dash.light { "dark" } else { "light" }.into()), ..Default::default() });');
  section("                            Key::Char('l') => {", "                            Key::Char('r') |", '📓️settings-key.rs.md');
  source=source.replace('if k.key == Key::Char(\'q\') && !dash.terminal_input {','if k.key == Key::Char(\'q\') && !dash.terminal_input && !dash.focused_window().is_some_and(|window| matches!(window.body, WindowBody::Launcher { .. })) {');
  const old='let key_ev = match (k.key, k.mods) {',begin=source.indexOf(old),end=source.indexOf('                            if let Some(widget_state)',begin);assert.ok(begin>0&&end>begin);source=source.slice(0,begin)+'let key_ev = *k;\n'+source.slice(end);
  source=source.replace('("jk/arrows", "move", "bewegen")','("arrows", "move", "bewegen")');
  source=source.replace('("l", "language", "Sprache")]', '("l", "language", "Sprache"), ("h", "tasks", "Aufgaben"), ("p", "settings", "Einstellungen"), ("f", "refresh", "aktualisieren"), ("e", "cancel discovery", "Suche abbrechen")]');
  source=source.replace('f.status = format!("{running} · {} · {}", dash.windows.len(), dash.connection_status);','let progress = dash.inventory.as_ref().map(|job| format!(" · discovering {}s · C-Space e cancels", job.started.elapsed().as_secs())).unwrap_or_else(|| format!(" · {} commands · {}", dash.commands.len(), dash.inventory_status));\n                f.status = format!("{running} · {}{}", dash.connection_status, progress);');
  source=source.replaceAll('"wizard"','"Commands"');
  return source;
 });
}

if(process.argv[2]==="installation"){
 edit("cli/📦️packages/🦀️rust/📜️script.ts",source=>{
  source=source.replace('import { pinExecutableArtifact } from "../../../../../../🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts";','import { installedDashboard, installDashboard } from "../../📦️installation/🟦️.ts";');
  source=source.replace('await buildRepositoryCargoArtifacts(join(this.root, "Cargo.toml"), ["--release", "--bin", "semio"], this.repoRoot);','await buildRepositoryCargoArtifacts(join(this.root, "Cargo.toml"), ["--release", "--bin", "semio"], this.repoRoot);\n    await installDashboard(this.root, this.repoRoot);');
  const first=source.indexOf('  const controller=new AbortController()',source.indexOf('export async function dashboardExecutable')),last=source.indexOf('\n}\n',first);assert.ok(first>0&&last>first);source=source.slice(0,first)+'  return installedDashboard(workspace);'+source.slice(last);
  source=source.replace('class TestScript extends BundleScript {','class InstallScript extends BundleScript {\n  async run(): Promise<void> { console.log(`[dashboard] Installed ${await installDashboard(this.root, this.repoRoot)}`); }\n}\n\nclass PreferencesScript extends BundleScript {\n  async run(segments: string[]): Promise<void> { process.exit(runCmdStatus(installedDashboard(this.repoRoot), ["preferences", ...segments], { cwd: this.repoRoot, env: devToolingEnv() })); }\n}\n\nclass TestScript extends BundleScript {');
  source=source.replace('.register("build", BuildScript)', '.register("build", BuildScript).register("install", InstallScript).register("preferences", PreferencesScript)');return source;
 });
 edit("cli/📦️packages/🦀️rust/📋️project.json",source=>{const project=JSON.parse(source);for(const name of ["run","daemon","workflow"])delete project.targets[name].dependsOn;const root=project.targets.run.options.cwd;project.targets.install={executor:"nx:run-commands",options:{cwd:root,command:"bun ./📜️script.ts install"},cache:false,dependsOn:["build"]};project.targets.preferences={executor:"nx:run-commands",options:{cwd:root,command:"bun ./📜️script.ts preferences",forwardAllArgs:true},cache:false};return JSON.stringify(project,null,2)+"\n";});
 edit("bootstrap/📜️script.ts",source=>source.replace('  publishBootstrapSources(WORKSPACE_ROOT);\n  await new ScriptRouter(WORKSPACE_ROOT).register("nx", NxScript).run(process.argv.slice(2));',`  const args=process.argv.slice(2);
  const native=await import("../../../⌨️cli/📦️installation/🟦️.ts"),invocation=args[0]==="nx"?native.dashboardInvocation(args.slice(1)):undefined;
  if(invocation){try{process.exitCode=await native.launchDashboard(WORKSPACE_ROOT,invocation);}catch(error){console.error(error instanceof Error?error.message:String(error));process.exitCode=1;}}
  else{publishBootstrapSources(WORKSPACE_ROOT);await new ScriptRouter(WORKSPACE_ROOT).register("nx", NxScript).run(args);}`));
 edit("package.json",source=>{const data=JSON.parse(source);data.scripts["dashboard:install"]="bun nx run @semio-tech/repo-cli-rs:install --outputStyle=stream";data.scripts["dashboard:preferences"]="bun nx run @semio-tech/repo-cli-rs:preferences --outputStyle=stream";return JSON.stringify(data,null,2)+"\n";});
 edit("cli/🧪️tests/🧊️execution/🟦️.ts",source=>source.replace('import { dashboardExecutable }','import { installDashboard, dashboardInvocation } from "../../📦️installation/🟦️.ts";\nimport { dashboardExecutable }').replace('const a = await dashboardExecutable(packageRoot, root);','const a = await installDashboard(packageRoot, root);').replace('const b = await dashboardExecutable(packageRoot, root);','expect(await dashboardExecutable(packageRoot, root)).toBe(a);\n  const b = await installDashboard(packageRoot, root);'));
}
if(process.argv[2]==="launch")edit(".vscode/launch.json",source=>{
 const data=JSON.parse(source),index=data.configurations.findIndex((entry:{name:string})=>entry.name==="🛠️dev🎛️dashboard🌳️command-tree");assert.ok(index>=0);
 data.configurations.splice(index+1,0,...[{name:"🛠️dev🎛️dashboard📦️install",command:"bun nx run @semio-tech/repo-cli-rs:install --outputStyle=stream",order:1.5},{name:"🛠️dev🎛️dashboard⚙️preferences",command:"bun nx run @semio-tech/repo-cli-rs:preferences --outputStyle=stream -- show",order:1.6}].map(({name,command,order})=>({name,type:"node-terminal",request:"launch",command,cwd:"${workspaceFolder}",presentation:{group:"3_dev",order}})));
 return JSON.stringify(data,null,2)+"\n";
});
if(process.argv[2]==="verify"){
 const fixture=join(import.meta.dir,"🧰️nx"),modules=join(fixture,"node_modules"),installed=realpathSync(join(workspace,".nx/installation/node_modules"));
 if(!existsSync(modules))symlinkSync(installed,modules,process.platform==="win32"?"junction":"dir");
 const nx=join(installed,"nx/dist/bin/nx.js"),operation=process.argv[3]!;
 assert.ok(["install","unit","native","execution","ui"].includes(operation));
 const child=Bun.spawn(["node",nx,"run","dashboard-ticket-verification:verify",`--args=${operation}`,"--outputStyle=stream"],{cwd:fixture,env:{...process.env,NX_DAEMON:"false",NX_TUI:"false",NX_ISOLATE_PLUGINS:"false",NX_WORKSPACE_DATA_DIRECTORY:join(generated,"nx-workspace"),SEMIO_TEST_ARTIFACT_DIR:generated},stdout:"inherit",stderr:"inherit",stdin:"inherit"});
 process.exitCode=await child.exited;
}
if(process.argv[2]==="owner"){
 const operation=process.argv[3]!,cli=path("cli/📦️packages/🦀️rust/📜️script.ts"),dashboard=path("dashboard/📦️packages/🦀️rust/📜️script.ts");
 const env={...process.env,NX_WORKSPACE_ROOT:workspace,REPO_ROOT:workspace,SEMIO_TEST_ARTIFACT_DIR:generated};
 const args=operation==="ui"?[path("🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📜️script.ts"),"test","quick","backend::native_windows::tests"]:operation==="install"?[cli,"build"]:operation==="execution"?[cli,"test","execution"]:[dashboard,"test","quick",...(operation==="native"?["daemon::tests::quick::actual_cli","--","--include-ignored","--nocapture","--test-threads=1"]:[])];
 if(operation==="ui"){process.env.NX_WORKSPACE_ROOT=workspace;const library=await import(path("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts"));env.SEMIO_CARGO_TEST_POLICY=JSON.stringify(library.repositoryCargoTestPolicyV1(path("🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml"),workspace,env));}
 if(operation==="native"){const installation=await import(path("cli/📦️installation/🟦️.ts"));env.SEMIO_TEST_CLI=installation.installedDashboard(workspace);}
 const child=Bun.spawn(["bun",...args],{cwd:workspace,env,stdin:"inherit",stdout:"inherit",stderr:"inherit"});process.exitCode=await child.exited;
}

if(process.argv[2]==="seed")edit(".vscode/🧩️launch.seed.jsonc",source=>{
 const launch=JSON.parse(readFileSync(path(".vscode/launch.json"),"utf8"));
 const entries=["🛠️dev🎛️dashboard📦️install","🛠️dev🎛️dashboard⚙️preferences"].map(name=>{const entry=launch.configurations.find((value:{name:string})=>value.name===name);assert.ok(entry);assert.ok(!source.includes(`"name": "${name}"`));return entry;});
 const start=source.indexOf('"name": "🛠️dev🎛️dashboard🌳️command-tree"'),anchor="\n    },",end=source.indexOf(anchor,start)+anchor.length;assert.ok(start>=0&&end>start);
 const inserted=entries.map(entry=>JSON.stringify(entry,null,2).split("\n").map(line=>"    "+line).join("\n")+",").join("\n");
 return source.slice(0,end)+"\n"+inserted+source.slice(end);
});

if(process.argv[2]==="review"){
 const normalized=(name:string)=>readFileSync(join(generated,name),"utf8").replace(/\s+/g," ");
 assert.ok(normalized("startup-pending-unit.log").includes("53 passed; 0 failed; 3 ignored"));
 const native=normalized("startup-pending-native.log");assert.ok(native.includes("3 passed; 0 failed; 0 ignored"));
 assert.ok(native.includes("native searchable launcher via_nx=false completed"));assert.ok(native.includes("native searchable launcher via_nx=true completed"));
 assert.ok(normalized("startup-ui-ownership-final.log").includes("4 passed; 0 failed"));
 for(const name of ["startup-pending-install.log","startup-execution-final.log"])assert.ok(normalized(name).includes("Successfully ran target"));
 const trials=[...native.matchAll(/first frame trial=(\d+) wrapper=(true|false) actual_workspace=(true|false) native_us=(\d+) host_ms=(\d+)/g)];assert.equal(trials.length,5);
 const benchmark="\n\n### Final Actual First-Frame Measurements\n\n| Trial | Entry | Workspace | Native frame | Host to frame |\n| --- | --- | --- | --- | --- |\n"+trials.map(row=>`| ${row[1]} | ${row[2]==="true"?"Bun package wrapper":"Native executable"} | ${row[3]==="true"?"Actual repository":"1,000-project fixture"} | ${Number(row[4])/1000} ms | ${row[5]} ms |`).join("\n")+"\n\nInstalled immutable executable: `"+JSON.parse(readFileSync(path(".🧬semio/🦑️repo/⚡️cache/🎛️dashboard/installed.json"),"utf8")).path+"`.\n";
 const reportPath=relative(workspace,join(ticket,"📓️native-startup.md"));edit(reportPath,source=>source+"\n\n"+readFileSync(join(import.meta.dir,"📓️final-acceptance.md"),"utf8")+benchmark);
 const ticketPath=relative(workspace,ticket).replaceAll("\\","/"),closeFile=join(ticket,"🧪️repo/🔣️close.json"),previous=JSON.parse(readFileSync(closeFile,"utf8")),files=new Set<string>(previous.files);
 for(const name of readdirSync(import.meta.dir).filter(name=>name.endsWith(".json"))){const rows=JSON.parse(readFileSync(join(import.meta.dir,name),"utf8"));if(Array.isArray(rows))for(const row of rows)if(row.path)files.add(relative(workspace,path(row.path)).replaceAll("\\","/"));}
 function collect(directory:string):void{for(const entry of readdirSync(directory,{withFileTypes:true})){if(["🗑️generated","node_modules",".nx"].includes(entry.name)||entry.isSymbolicLink())continue;const item=join(directory,entry.name);if(entry.isDirectory())collect(item);else files.add(relative(workspace,item).replaceAll("\\","/"));}}
 collect(ticket);files.add(`${ticketPath}/📓️files.md`);files.add(".vscode/🧩️launch.seed.jsonc");
 const inventory=[...files].filter(file=>!file.includes("/🗑️generated/")&&!file.endsWith("AGENTS.md")).sort();
 const checked=Bun.spawnSync(["git","diff","--check","--",...inventory],{cwd:workspace,stdout:"pipe",stderr:"pipe"});assert.equal(checked.exitCode,0,checked.stdout.toString()+checked.stderr.toString());
 const measured=trials[4]!,summary=`Completed the native developer dashboard and final startup/usability goal. Installed startup uses a clean Rust executable without provisioning, graph construction, catalog walking or rebuilding before its usable frame. Defaults ask no language or terminology questions; durable local/shared settings, searchable direct commands, layouts, appearance and renderer preferences are available through Ctrl+B controls. One daemon retains full task lifecycle/output, cancellation, descendant termination and detach/reattach. Starts selected before connection remain in a bounded cancellable queue. Final native PTY checks passed across five starts, including the actual workspace: native frame ${Number(measured[4])/1000}ms and wrapper-to-frame ${measured[5]}ms. The actual searchable launcher ran Nx builds and rendered output through both native wrapper paths. 53 Rust dashboard tests, 6 independent Bun oracles, 3 installed-binary runtime tests, 4 Windows terminal ownership tests and 2 installation/launch tests passed. Earlier real renderer/lifecycle evidence remains in this ticket. Current acceptance, exact scope and platform limits are in 📓️native-startup.md. Authored investigation inputs/reports are retained and generated outputs are removed. No modifying Git commands, worktrees, AGENTS.md edits or repository goal-management changes were used.`;
 edit(relative(workspace,closeFile),()=>JSON.stringify({...previous,summary,files:inventory},null,2)+"\n");
 edit(`${ticketPath}/📓️files.md`,()=>"# Dashboard Ticket File Inventory\n\nThe repository MCP closing inventory retains the preceding control-plane work and adds the native startup/preferences/terminal changes. Paths include authored ticket inputs and reports. Concurrent unrelated changes were preserved. Generated outputs, junctions and AGENTS.md are excluded. The scoped Git whitespace check passed.\n\n"+inventory.map(file=>`- ${file}`).join("\n")+"\n");
 console.log(`[DEBUG] final acceptance logs verified; ${inventory.length} authored/source paths reviewed`);
}
