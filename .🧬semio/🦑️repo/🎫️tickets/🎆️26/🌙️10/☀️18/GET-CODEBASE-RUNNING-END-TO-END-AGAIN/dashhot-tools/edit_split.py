#!/usr/bin/env python3
"""🏺 Batch `split` (lane dashhot) as anchored edits on the CURRENT live bytes: `python3 -I edit_split.py <root>`.
`build` becomes a cacheable artifact producer (output: the per-mode artifact directory with the executable and its record), installation becomes a local, idempotent,
receipt-checked step (`installDashboardArtifactV1`) used by `install`, by the script route outside a task (the launcher's rebuild) and by the test routes.
Every anchor must match exactly once (else exit, nothing written); JSON files are edited as data and re-dumped in the repository style (indent 2, UTF-8)."""
import json, os, sys

L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/"
INSTALL = L + "📚️library/⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/"
PUBLIC = L + "📚️library/📦️packages/🟦️typescript/💾️storage/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts"
TEXTS = L + "📚️library/⚡️caching/🚀️bootstrap/📖️texts/🔣️.json"
ROUTES = L + "🎛️dashboard/🛤️task-routes/🟦️.ts"
PROJECT = L + "🎛️dashboard/📦️packages/🦀️rust/📋️project.json"
INTENT_TEST = L + "📚️library/🏗️build/🕸️graph/🦀️rust/📥️intent/🧪️tests/🟦️.ts"
EXEC = L + "🎛️dashboard/🧪️tests/🧊️execution/"
README = L + "🎛️dashboard/README.md"
ARTIFACT_OUTPUT = "{workspaceRoot}/.🧬semio/🦑️repo/⚡️cache/🎛️dashboard/{options.env.SEMIO_BUILD_MODE}/artifact"
BUILD_INPUTS = ["default", "^default", "reads", "cargoToolchain", "{workspaceRoot}/Cargo.toml", "{workspaceRoot}/Cargo.lock", "{workspaceRoot}/🧰️framework/Cargo.toml", "{workspaceRoot}/rust-toolchain.toml", "{workspaceRoot}/.cargo/config.toml", "{workspaceRoot}/🧰️framework/🔨️modules/🖱️ui/⌨️tui/**/*", "{workspaceRoot}/🧰️framework/🔨️modules/🖱️ui/🧱️elements/*/🎯️targets/⌨️tui/**/*", "{workspaceRoot}/🧰️framework/🔨️modules/🖱️ui/🎚️axes/🤖️generated/**/*", "{workspaceRoot}/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/**/*", {"dependentTasksOutputFiles": "**/*", "transitive": True}]


def replace(text, old, new, label):
    if text.count(old) != 1:
        sys.exit(f"anchor {label}: {text.count(old)} matches")
    return text.replace(old, new)


def edit_text(root, rel, edits, out):
    text = open(os.path.join(root, rel), encoding="utf8").read()
    for index, (old, new) in enumerate(edits):
        text = replace(text, old, new, f"{rel[-40:]}#{index}")
    out[rel] = text


def edit_json(root, rel, change, out):
    data = json.load(open(os.path.join(root, rel), encoding="utf8"))
    change(data)
    out[rel] = json.dumps(data, indent=2, ensure_ascii=False) + "\n"


def project(data):
    build = data["targets"]["build"]
    if build.get("cache") is not False:
        sys.exit("build target already changed")
    rebuilt = {}
    for key, value in build.items():
        if key == "cache":
            rebuilt["cache"] = True
            rebuilt["inputs"] = BUILD_INPUTS
        elif key == "outputs":
            rebuilt["outputs"] = [ARTIFACT_OUTPUT]
        else:
            rebuilt[key] = value
    rebuilt["metadata"]["semio"]["cache"] = {"version": 1, "class": "deterministic", "reason": "pure-function-of-inputs"}
    data["targets"]["build"] = rebuilt


def install_schema(data):
    if "ArtifactProtocol" in data["definitions"]:
        sys.exit("artifact protocol exists")
    data["description"] = "🏺️ One native dashboard executable record: the artifact a `build` leaves at `ArtifactProtocol` (its `path` names the executable inside the artifact directory, relative to the installation root) and the installation that pins it (its `path` names the immutable tools copy, relative to the workspace) share it; an installation copies the artifact record and only changes `path`."
    data["definitions"]["ArtifactProtocol"] = {"const": {"directory": "artifact", "record": "artifact.json"}}
    data["properties"]["version"] = {"const": 5}
    compiled = data["properties"]["compiled"]
    compiled["description"] = "🧾️ What the executable was compiled from: the files of its own Cargo dep-info plus every crate manifest, the workspace manifests and lock, the toolchain files and the package declaration (relative to the installation root), when that build started, and the sha256 each file held then (empty when it was already touched after the start). A file is changed when it is missing, or touched at or after `builtAtMs` and no longer holds those bytes."
    compiled["required"] = ["builtAtMs", "sources", "digests"]
    compiled["properties"]["digests"] = {"type": "array", "minItems": 1, "items": {"type": "string", "pattern": "^(?:[a-f0-9]{64})?$"}}


def compiled_fixture(data):
    data["mutations"] += [
        {"name": "an included file is rewritten with identical bytes", "kind": "rewrite-same", "path": "schema/🔣️.json", "digest": False, "compiled": False},
        {"name": "a crate source is rewritten with identical bytes", "kind": "rewrite-same", "path": "app/🎮️registry/🦀️.rs", "digest": True, "compiled": False},
    ]


def texts(data):
    data["messages"]["dashboard.artifact-required"] = {
        "en": "[dashboard] there is no {mode} build to install: run repo run @semio-tech/repo-dashboard-rs:build first",
        "de": "[dashboard] es gibt keinen {mode}-Build zum Installieren: zuerst repo run @semio-tech/repo-dashboard-rs:build ausführen",
    }
    data["messages"]["dashboard.artifact-behind-sources"] = {
        "en": "[dashboard] the {mode} build is behind its compiled source {path}: run repo run @semio-tech/repo-dashboard-rs:build --skip-cache",
        "de": "[dashboard] der {mode}-Build ist hinter seiner kompilierten Quelle {path} zurück: repo run @semio-tech/repo-dashboard-rs:build --skip-cache ausführen",
    }


INSTALL_EDITS = [
    ('import {CARGO_BINARY_SOURCES_SCHEMA_V1,cargoBinarySourcesFreshnessV1,parseCargoBinarySourcesV1} from', 'import {parseCargoBinarySourcesV1} from'),
    ('import {sayBootstrapV1} from "../../📖️texts/🟦️.ts";\n', 'import {refuseBootstrapV1,sayBootstrapV1} from "../../📖️texts/🟦️.ts";\n'),
    ('import { existsSync, readFileSync, writeFileSync, renameSync, mkdirSync, lstatSync, statSync, readdirSync } from "node:fs";',
     'import { existsSync, readFileSync, writeFileSync, renameSync, mkdirSync, lstatSync, statSync, readdirSync, copyFileSync, chmodSync, rmSync, constants as fsConstants } from "node:fs";'),
    ('const slashes=(path:string)=>path.replaceAll("\\\\","/");\n',
     'const slashes=(path:string)=>path.replaceAll("\\\\","/");\n'
     'const executableName=()=>process.platform==="win32"?"semio.exe":"semio";\n'
     'const artifactProtocol=installationSchema.definitions.ArtifactProtocol.const;\n'
     'const artifactDirectory=(root:string,mode:BuildMode)=>join(cache(root),"🎛️dashboard",readBuildMode(mode),artifactProtocol.directory);\n'
     'const readRecord=(file:string):any=>{try{return JSON.parse(readFileSync(file,"utf8"));}catch{return undefined;}};\n'
     'const writeRecord=(file:string,value:unknown)=>{const temporary=file+"."+process.pid+".tmp";mkdirSync(dirname(file),{recursive:true});writeFileSync(temporary,JSON.stringify(value)+"\\n");renameSync(temporary,file);};\n'
     'const sha256=(file:string)=>createHash("sha256").update(readFileSync(file)).digest("hex");\n'),
    ('/** 🗃️ What an installed executable was compiled from: the files of its own Cargo dep-info, relative to the installation root, and when that build started. */\n'
     'export type DashboardCompiledSources={readonly builtAtMs:number;readonly sources:readonly string[]};\n',
     '/** 🗃️ What an executable was compiled from: the files of its own Cargo dep-info and every Cargo manifest, lock, toolchain file and the package declaration (Cargo\'s whole freshness basis), relative to the installation root, when that build started, and the sha256 each file held then (empty when it was already touched after the start). */\n'
     'export type DashboardCompiledSources={readonly builtAtMs:number;readonly sources:readonly string[];readonly digests:readonly string[]};\n'),
    ('const skippedDirectories=new Set(',
     '/** 🏺️ Checks current host and mode and that the record names the executable inside this mode\'s artifact directory (relative to the installation root). */\n'
     'function validDashboardArtifactV1(root:string,value:unknown,mode:BuildMode):boolean{\n'
     ' if(validateJsonSchemaSubset(installationSchema,value).length)return false;\n'
     ' const record=value as {platform:string;mode:string;path:string};\n'
     ' return record.platform===process.platform&&record.mode===mode&&record.path===slashes(relative(root,join(artifactDirectory(root,mode),executableName())));\n'
     '}\n\n'
     'const skippedDirectories=new Set('),
    ('function sourceInputs(packageRoot:string,workspace:string):Pick<DashboardSources,"roots"|"manifests"> {\n'
     '  workspace=resolve(workspace);const cache=new Map<string,CargoTable>(),seen=new Set<string>(),pending=[resolve(packageRoot)],roots=new Set<string>(),manifests=new Set<string>();\n',
     'function sourceInputs(packageRoot:string,workspace:string):Pick<DashboardSources,"roots"|"manifests">&{readonly crates:readonly string[]} {\n'
     '  workspace=resolve(workspace);const cache=new Map<string,CargoTable>(),seen=new Set<string>(),pending=[resolve(packageRoot)],roots=new Set<string>(),manifests=new Set<string>(),crates=new Set<string>();\n'),
    ('    if(!existsSync(file))continue;\n    const manifest=read(directory),', '    if(!existsSync(file))continue;\n    crates.add(slashes(relative(workspace,file)));\n    const manifest=read(directory),'),
    ('  return{roots:ordered.map(root=>slashes(relative(workspace,root))).sort(),manifests:[...manifests].sort()};\n',
     '  return{roots:ordered.map(root=>slashes(relative(workspace,root))).sort(),manifests:[...manifests].sort(),crates:[...crates].sort()};\n'),
    ('  const sources=sourceInputs(packageRoot,workspace);\n  return {...sources,digest:await digestDashboardSources(workspace,sources)};\n',
     '  const {roots,manifests}=sourceInputs(packageRoot,workspace);\n  return {roots,manifests,digest:await digestDashboardSources(workspace,{roots,manifests})};\n'),
    ("/** 🕰️ The first compiled source that is missing or was written, replaced or re-linked at or after the build started, or nothing: Cargo's own freshness rule (`cargoBinarySourcesFreshnessV1`) over the whole dependency closure of the binary, read on the later of modification and inode change time so a replacement that keeps old timestamps counts. Only stats, no bytes. */\n"
     "export function changedDashboardSourceV1(root:string,compiled:DashboardCompiledSources):string|undefined{\n"
     " return cargoBinarySourcesFreshnessV1({schema:CARGO_BINARY_SOURCES_SCHEMA_V1,...compiled},path=>{try{const facts=statSync(resolve(root,path));return Math.max(facts.mtimeMs,facts.ctimeMs);}catch{return null;}}).changed??undefined;\n"
     "}\n\n"
     "/** 📥️ Admits the sources record Cargo published beside the executable as installation-root-relative paths; a missing or malformed record refuses the installation. */\n"
     "function compiledDashboardSourcesV1(root:string,file:string):DashboardCompiledSources{\n"
     " let text:string;try{text=readFileSync(file,\"utf8\");}catch{throw Error(\"Dashboard executable sources record required\");}\n"
     " const record=parseCargoBinarySourcesV1(text);if(record===null)throw Error(\"Dashboard executable sources record required\");\n"
     " return {builtAtMs:record.builtAtMs,sources:[...new Set(record.sources.map(source=>slashes(relative(root,resolve(root,source)))))].sort()};\n"
     "}\n",
     "/** 🕰️ The first compiled source that is missing, or that was written, replaced or re-linked at or after the build started (the later of modification and inode change time) and no longer holds the bytes it held then, or nothing: Cargo's own freshness rule over the whole freshness basis of the binary, made content-exact so an identical rewrite (a revert, a checkout, a restored cached artifact) stays current. Stats every file, reads only the touched ones. */\n"
     "export function changedDashboardSourceV1(root:string,compiled:DashboardCompiledSources):string|undefined{\n"
     " if(compiled.digests.length!==compiled.sources.length)return compiled.sources[0];\n"
     " return compiled.sources.find((path,index)=>{const file=resolve(root,path);try{const facts=statSync(file);return Math.max(facts.mtimeMs,facts.ctimeMs)>=compiled.builtAtMs&&(compiled.digests[index]===\"\"||sha256(file)!==compiled.digests[index]);}catch{return true;}});\n"
     "}\n\n"
     "/** 📥️ Admits the sources record Cargo published beside the executable and adds the rest of Cargo's freshness basis (every crate manifest, the workspace manifests and lock, the toolchain files, the package declaration), as installation-root-relative paths with the bytes each held at the build start; a missing or malformed record refuses the artifact. */\n"
     "function compiledDashboardSourcesV1(packageRoot:string,root:string,file:string):DashboardCompiledSources{\n"
     " let text:string;try{text=readFileSync(file,\"utf8\");}catch{throw Error(\"Dashboard executable sources record required\");}\n"
     " const record=parseCargoBinarySourcesV1(text);if(record===null)throw Error(\"Dashboard executable sources record required\");\n"
     " const {manifests,crates}=sourceInputs(packageRoot,root),basis=[...record.sources.map(source=>slashes(relative(root,resolve(root,source)))),...crates,...manifests,...workspaceFiles,slashes(relative(root,join(packageRoot,\"📋️project.json\")))];\n"
     " const sources=[...new Set(basis)].filter(path=>existsSync(resolve(root,path))).sort();\n"
     " return {builtAtMs:record.builtAtMs,sources,digests:sources.map(path=>{const file=resolve(root,path),facts=statSync(file);return Math.max(facts.mtimeMs,facts.ctimeMs)<record.builtAtMs?sha256(file):\"\";})};\n"
     "}\n"),
    ('/** ✅️ The installed executable and the number of files it was compiled from when this host and mode\'s installation records exactly these captured sources, none of its compiled sources changed since its build and its immutable bytes are still in place with their recorded size and permissions; nothing otherwise. Only stats, no bytes. */\n'
     'export function currentDashboardV1(root:string,mode:BuildMode,workspace:string,sources:DashboardSources):{readonly executable:string;readonly compiled:number}|undefined{\n'
     ' let record;try{record=JSON.parse(readFileSync(manifest(root,mode),"utf8"));}catch{return;}\n'
     ' if(!validDashboardRecord(record,mode)||record.sources.digest!==sources.digest||JSON.stringify(record.sources.roots)!==JSON.stringify(sources.roots)||JSON.stringify(record.sources.manifests)!==JSON.stringify(sources.manifests)||changedDashboardSourceV1(root,record.compiled)!==undefined)return;\n'
     ' const executable=resolve(workspace,record.path);\n'
     ' try{const facts=lstatSync(executable);return facts.isFile()&&facts.size===record.bytes&&(facts.mode&0o777)===record.permissions?{executable,compiled:record.compiled.sources.length}:undefined;}catch{return;}\n'
     '}\n',
     '/** 🧷️ Whether an executable is in place with the size and permissions its record names. Only stats, no bytes. */\n'
     'function executableInPlaceV1(executable:string,record:{bytes:number;permissions:number}):boolean{\n'
     ' try{const facts=lstatSync(executable);return facts.isFile()&&!facts.isSymbolicLink()&&facts.size===record.bytes&&(facts.mode&0o777)===record.permissions;}catch{return false;}\n'
     '}\n\n'
     '/** ✅️ This mode\'s built artifact and the number of files of its freshness basis when its record is valid for this host, none of those files changed since its build and its executable is in place; nothing otherwise. */\n'
     'export function currentDashboardArtifactV1(root:string,mode:BuildMode):{readonly executable:string;readonly compiled:number}|undefined{\n'
     ' const record=readRecord(join(artifactDirectory(root,mode),artifactProtocol.record));\n'
     ' if(!validDashboardArtifactV1(root,record,mode)||changedDashboardSourceV1(root,record.compiled)!==undefined)return;\n'
     ' const executable=resolve(root,record.path);\n'
     ' return executableInPlaceV1(executable,record)?{executable,compiled:record.compiled.sources.length}:undefined;\n'
     '}\n'),
    ('/** 🧊️ Publishes immutable executable bytes and one atomic native installation record. */\n'
     'export async function installDashboard(packageRoot:string,root:string,options:DashboardExecutionOptions,sources?:DashboardSources):Promise<string> {\n'
     ' const mode=readBuildMode(options.mode),control=options.control,name=process.platform==="win32"?"semio.exe":"semio",recorded=sources??await captureDashboardSources(packageRoot,root);compilerAdmissionBudgetV1(control);if(validateJsonSchemaSubset(installationSchema.properties.sources,recorded).length)throw Error("Dashboard installation source ownership refused");\n'
     ' const {pinExecutableArtifact,retireArtifactDirectoriesV1}=await import("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/📤️publication/🟦️.ts");\n'
     ' return withRepositoryToolingStoreV1(options.workspace,options.invocation,control,async tools=>withRepositoryPublishedCargoExecutableV1(join(packageRoot,"Cargo.toml"),name,options.workspace,mode,options.invocation,control,async (artifact,control)=>{\n'
     '  const compiled=compiledDashboardSourcesV1(root,join(dirname(artifact.path),installationSchema.definitions.CompiledSourceProtocol.const.record)),executable=await pinExecutableArtifact(artifact.path,join(tools.root,"dashboard-cli",mode),`dashboard-installation:${mode}`,{control,signal:control.signal,leaseDirectory:join(options.workspace,".cache/b/l/m/data"),onWait:async progress=>{compilerAdmissionBudgetV1(control);await control.publish(progress);await control.yieldContinuation();}});await tools.recheck();const path=slashes(relative(options.workspace,executable));if(isAbsolute(path)||path.startsWith("../"))throw Error("Dashboard installation must stay inside workspace");const target=manifest(root,mode),temporary=target+"."+process.pid+".tmp";mkdirSync(dirname(target),{recursive:true});writeFileSync(temporary,JSON.stringify({version:4,platform:process.platform,mode,path,sources:recorded,compiled,digest:artifact.receipt.digest,bytes:artifact.receipt.bytes,permissions:artifact.receipt.permissions})+"\\n");compilerAdmissionBudgetV1(control);renameSync(temporary,target);await retireOldDashboardExecutablesV1(join(tools.root,"dashboard-cli",mode),artifact.receipt.digest,options.workspace,control,retireArtifactDirectoriesV1);return executable;\n'
     ' }));\n'
     '}\n',
     '/** 📤️ Copies the published executable into this mode\'s artifact directory (the cacheable output of `build`) with a record of what it was compiled from, under the exclusive artifact lease: the executable first, then the record that names it, each by an atomic rename, so a reader never sees a record without its bytes. */\n'
     'export async function exportDashboardArtifactV1(packageRoot:string,root:string,options:DashboardExecutionOptions,sources:DashboardSources):Promise<string>{\n'
     ' const mode=readBuildMode(options.mode),control=options.control,name=executableName(),directory=artifactDirectory(root,mode);compilerAdmissionBudgetV1(control);if(validateJsonSchemaSubset(installationSchema.properties.sources,sources).length)throw Error("Dashboard installation source ownership refused");\n'
     ' return withRepositoryPublishedCargoExecutableV1(join(packageRoot,"Cargo.toml"),name,options.workspace,mode,options.invocation,control,async(artifact,control)=>withResourceLeases({directory:join(options.workspace,".cache/b/l/m/data"),control,signal:control.signal,resources:[{resource:"artifact:"+directory,mode:"exclusive"}]},async()=>{\n'
     '  const compiled=compiledDashboardSourcesV1(packageRoot,root,join(dirname(artifact.path),installationSchema.definitions.CompiledSourceProtocol.const.record)),executable=join(directory,name),staged=`${directory}.${process.pid}.bin`;\n'
     '  mkdirSync(directory,{recursive:true});rmSync(staged,{force:true});copyFileSync(artifact.path,staged,fsConstants.COPYFILE_FICLONE);chmodSync(staged,artifact.receipt.permissions);renameSync(staged,executable);compilerAdmissionBudgetV1(control);\n'
     '  writeRecord(join(directory,artifactProtocol.record),{version:5,platform:process.platform,mode,path:slashes(relative(root,executable)),sources,compiled,digest:artifact.receipt.digest,bytes:artifact.receipt.bytes,permissions:artifact.receipt.permissions});return executable;\n'
     ' }));\n'
     '}\n\n'
     '/** 🧊️ Installs this mode\'s built artifact: local, idempotent and receipt-checked. It refuses an artifact that is missing, foreign or behind a file of its freshness basis; when the installation already names the same bytes and its immutable copy is in place it only refreshes the record (no tools store); otherwise it pins the artifact executable into the tools store. The record is the artifact record with the pinned `path` and the sources captured before the check. */\n'
     'export async function installDashboardArtifactV1(packageRoot:string,root:string,options:DashboardExecutionOptions):Promise<string>{\n'
     ' const mode=readBuildMode(options.mode),control=options.control,artifact=readRecord(join(artifactDirectory(root,mode),artifactProtocol.record));compilerAdmissionBudgetV1(control);\n'
     ' if(!validDashboardArtifactV1(root,artifact,mode)||!executableInPlaceV1(resolve(root,artifact.path),artifact))throw refuseBootstrapV1("missing","dashboard.artifact-required",{mode});\n'
     ' const sources=await captureDashboardSources(packageRoot,root),changed=changedDashboardSourceV1(root,artifact.compiled);if(changed!==undefined)throw refuseBootstrapV1("stale","dashboard.artifact-behind-sources",{mode,path:changed});\n'
     ' const installed=readRecord(manifest(root,mode)),current=validDashboardRecord(installed,mode)&&installed.digest===artifact.digest?resolve(options.workspace,installed.path):undefined;\n'
     ' if(current&&executableInPlaceV1(current,installed)){if(JSON.stringify(installed.sources)!==JSON.stringify(sources)||JSON.stringify(installed.compiled)!==JSON.stringify(artifact.compiled))writeRecord(manifest(root,mode),{...artifact,sources,path:installed.path});return current;}\n'
     ' const {pinExecutableArtifact,retireArtifactDirectoriesV1}=await import("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/📤️publication/🟦️.ts");\n'
     ' return withRepositoryToolingStoreV1(options.workspace,options.invocation,control,async tools=>{\n'
     '  const executable=await pinExecutableArtifact(resolve(root,artifact.path),join(tools.root,"dashboard-cli",mode),`dashboard-installation:${mode}`,{control,signal:control.signal,leaseDirectory:join(options.workspace,".cache/b/l/m/data"),onWait:async progress=>{compilerAdmissionBudgetV1(control);await control.publish(progress);await control.yieldContinuation();}});await tools.recheck();\n'
     '  if(basename(dirname(executable))!==artifact.digest)throw Error("Dashboard artifact changed during installation");const path=slashes(relative(options.workspace,executable));if(isAbsolute(path)||path.startsWith("../"))throw Error("Dashboard installation must stay inside workspace");\n'
     '  compilerAdmissionBudgetV1(control);writeRecord(manifest(root,mode),{...artifact,sources,path});await retireOldDashboardExecutablesV1(join(tools.root,"dashboard-cli",mode),artifact.digest,options.workspace,control,retireArtifactDirectoriesV1);return executable;\n'
     ' });\n'
     '}\n\n'
     '/** 🧊️ Exports the published executable as this mode\'s artifact and installs it (the route of the store-backed installation laws and of callers holding a fresh compile). */\n'
     'export async function installDashboard(packageRoot:string,root:string,options:DashboardExecutionOptions,sources?:DashboardSources):Promise<string> {\n'
     ' await exportDashboardArtifactV1(packageRoot,root,options,sources??await captureDashboardSources(packageRoot,root));\n'
     ' return installDashboardArtifactV1(packageRoot,root,options);\n'
     '}\n'),
]

ROUTES_EDITS = [
    ("import { captureDashboardSources, currentDashboardV1, sayDashboardCurrentV1, installedDashboard, installDashboard,",
     "import { captureDashboardSources, currentDashboardArtifactV1, exportDashboardArtifactV1, installDashboardArtifactV1, sayDashboardCurrentV1, installedDashboard,"),
    ('    if(process.env.SEMIO_TASK_PROJECT!=="@semio-tech/repo-dashboard-rs"||process.env.SEMIO_TASK_TARGET!=="build"){await runOwnedBuildChildInvocationV1(this.repoRoot,["run","@semio-tech/repo-dashboard-rs:build",...segments],this.invocation,createIssuedResourceControlV1(this.invocation,"dashboard:build",0));return;}\n'
     '    await buildAndInstallDashboard(this.root,this.repoRoot,dashboardOptions(this.invocation,this.repoRoot));\n',
     '    if(process.env.SEMIO_TASK_PROJECT!=="@semio-tech/repo-dashboard-rs"||process.env.SEMIO_TASK_TARGET!=="build"){if(!currentDashboardArtifactV1(this.repoRoot,readBuildMode(process.env.SEMIO_BUILD_MODE)))await runOwnedBuildChildInvocationV1(this.repoRoot,["run","@semio-tech/repo-dashboard-rs:build",...segments],this.invocation,createIssuedResourceControlV1(this.invocation,"dashboard:build",0));await installDashboardArtifactV1(this.root,this.repoRoot,dashboardOptions(this.invocation,this.repoRoot));return;}\n'
     '    await buildDashboardArtifactV1(this.root,this.repoRoot,dashboardOptions(this.invocation,this.repoRoot));\n'),
    ("  async run(): Promise<void> { console.log(`[dashboard] Installed ${await installDashboard(this.root, this.repoRoot,dashboardOptions(this.invocation,this.repoRoot))}`); }",
     "  async run(): Promise<void> { console.log(`[dashboard] Installed ${await installDashboardArtifactV1(this.root,this.repoRoot,dashboardOptions(this.invocation,this.repoRoot))}`); }"),
    ("/** 🔨️ Builds the selected executable and installs it, unless the installation of this mode is current for the captured sources and everything its executable was compiled from (`currentDashboardV1`), then no nested build runs at all; the sources are captured first so edits made during the build stay detectable, and Ctrl+C cancels the build. */\n"
     "export async function buildAndInstallDashboard(packageRoot: string, workspace: string, options:DashboardExecutionOptions): Promise<string> {\n"
     "  compilerAdmissionBudgetV1(options.control);const mode=readBuildMode(options.mode);\n"
     "  const sources = await captureDashboardSources(packageRoot, workspace);compilerAdmissionBudgetV1(options.control);\n"
     "  const current=currentDashboardV1(workspace,mode,options.workspace,sources);\n"
     "  if(current){sayDashboardCurrentV1(mode,sources,current.compiled);return current.executable;}\n"
     "  await runDeclaredRustBuildV1(workspace,\"@semio-tech/repo-dashboard-rs\",packageRoot,options.invocation,[],\"compile\");\n"
     "  return await installDashboard(packageRoot, workspace, options,sources);\n"
     "}\n",
     "/** 🏺️ Builds this mode's artifact, the cacheable output of `build`, unless the artifact is current for the captured sources and everything its executable was compiled from (`currentDashboardArtifactV1`), then no nested build runs at all; the sources are captured first so edits made during the build stay detectable, and Ctrl+C cancels the build. */\n"
     "export async function buildDashboardArtifactV1(packageRoot: string, workspace: string, options:DashboardExecutionOptions): Promise<string> {\n"
     "  compilerAdmissionBudgetV1(options.control);const mode=readBuildMode(options.mode);\n"
     "  const sources = await captureDashboardSources(packageRoot, workspace);compilerAdmissionBudgetV1(options.control);\n"
     "  const current=currentDashboardArtifactV1(workspace,mode);\n"
     "  if(current){sayDashboardCurrentV1(mode,sources,current.compiled);return current.executable;}\n"
     "  await runDeclaredRustBuildV1(workspace,\"@semio-tech/repo-dashboard-rs\",packageRoot,options.invocation,[],\"compile\");\n"
     "  return await exportDashboardArtifactV1(packageRoot, workspace, options,sources);\n"
     "}\n"
     "/** 🔨️ Builds this mode's artifact and installs it (`installDashboardArtifactV1`). */\n"
     "export async function buildAndInstallDashboard(packageRoot: string, workspace: string, options:DashboardExecutionOptions): Promise<string> {\n"
     "  await buildDashboardArtifactV1(packageRoot,workspace,options);\n"
     "  return await installDashboardArtifactV1(packageRoot,workspace,options);\n"
     "}\n"),
]


def main():
    root = os.path.abspath(sys.argv[1])
    out = {}
    edit_text(root, INSTALL + "🟦️.ts", INSTALL_EDITS, out)
    edit_json(root, INSTALL + "🧬️schema/🔣️.json", install_schema, out)
    edit_text(root, PUBLIC, [
        ("export { captureDashboardSources, changedDashboardSourceV1, currentDashboardV1, dashboardBuildDescendantsV1,", "export { captureDashboardSources, changedDashboardSourceV1, currentDashboardArtifactV1, dashboardBuildDescendantsV1,"),
        ("digestDashboardSources, installDashboard, installedDashboard,", "digestDashboardSources, exportDashboardArtifactV1, installDashboard, installDashboardArtifactV1, installedDashboard,"),
    ], out)
    edit_json(root, TEXTS, texts, out)
    edit_text(root, ROUTES, ROUTES_EDITS, out)
    edit_json(root, PROJECT, project, out)
    edit_text(root, INTENT_TEST, [
        ('expect(dashboard.targets.build.cache).toBe(false);', 'expect(dashboard.targets.build.cache).toBe(true);expect(dashboard.targets.build.metadata.semio.cache).toEqual({version:1,class:"deterministic",reason:"pure-function-of-inputs"});'),
    ], out)
    edit_json(root, L + "🎛️dashboard/🧪️tests/🧫️fixtures/🧊️execution/🧾️compiled.json", compiled_fixture, out)
    edit_text(root, EXEC + "🟦️.ts", [
        ('record:any={version:4,platform:process.platform,mode:selected,path:`.cache/b/t/m/dashboard-cli/${selected}/${digest}/${name}`,sources:await captureDashboardSources(packageRoot,root),compiled:{builtAtMs:Date.now()+1,sources:["mod/📦️packages/🦀️rust/🦀️.rs"]},digest,bytes:1,permissions:0o755}',
         'record:any={version:5,platform:process.platform,mode:selected,path:`.cache/b/t/m/dashboard-cli/${selected}/${digest}/${name}`,sources:await captureDashboardSources(packageRoot,root),compiled:{builtAtMs:Date.now()+1,sources:["mod/📦️packages/🦀️rust/🦀️.rs"],digests:[""]},digest,bytes:1,permissions:0o755}'),
    ], out)
    edit_text(root, EXEC + "🥒️.feature", [
        ("  Scenario: A build of a current installation runs no nested build\n"
         "    Given the installation of the selected mode records exactly the captured sources and none of its compiled files changed\n"
         "    And its immutable executable is in place with the recorded size and permissions\n"
         "    When the dashboard build task runs\n"
         "    Then it answers that executable without starting the nested compile and says so in one line in the language of the command\n"
         "    And a removed, resized or re-permissioned executable, a changed source or a changed compiled file sends it on to build\n",
         "  Scenario: A build of a current artifact runs no nested build\n"
         "    Given the artifact of the selected mode records exactly the captured sources and none of its compiled files changed\n"
         "    And its executable is in place with the recorded size and permissions\n"
         "    When the dashboard build task runs\n"
         "    Then it answers that executable without starting the nested compile and says so in one line in the language of the command\n"
         "    And a removed, resized or re-permissioned executable, a changed source or a changed compiled file sends it on to build\n\n"
         "  Scenario: The build is a cacheable artifact and installation is local, idempotent and receipt-checked\n"
         "    Given the build task declares the per-mode artifact directory as its only output and is replayed from the action cache\n"
         "    When the artifact is installed\n"
         "    Then an installation that already names the same bytes answers at once without opening the tools store, refreshing only its record\n"
         "    And a missing, foreign or stale artifact is refused in the language of the command instead of being installed\n"
         "    And an artifact restored from the action cache after a revert stays current because a touched file only counts when its bytes changed\n"
         "    And a build outside the build tool runs the build only when the artifact is not current, then installs it\n"),
    ], out)
    edit_text(root, README, [
        ("start are published beside it and copied into `installed.json` (`compiled`). `repo-dashboard-rs:build` asks first whether the\n"
         "installation of the mode is current (`currentDashboardV1`: the same captured sources, no compiled source written, replaced or re-linked\n"
         "at or after the build start, the immutable executable in place) and then runs no nested `compile` at all; it says so in one line. The\n"
         "same rule makes `repo dashboard` rebuild when a compiled file outside the source digest changes. A change of the build tool's own code\n"
         "does not make a current installation stale; `repo run @semio-tech/repo-dashboard-rs:compile` then `:install` forces a rebuild.\n",
         "start are published beside it and copied, together with every Cargo manifest, the lock, the toolchain files and the package declaration\n"
         "(Cargo's whole freshness basis) and the sha256 each file held at the build start, into the artifact record and `installed.json`\n"
         "(`compiled`). A file counts as changed when it is missing, or when it was touched at or after the build start and no longer holds those\n"
         "bytes, so a revert or checkout of identical bytes keeps it current. The same rule makes `repo dashboard` rebuild when a compiled file\n"
         "outside the source digest changes.\n\n"
         "`build` is a cacheable artifact producer: its only output is `.🧬semio/🦑️repo/⚡️cache/🎛️dashboard/<mode>/artifact` (the executable and\n"
         "`artifact.json`, the same record shape as `installed.json`), so a no-op `repo run @semio-tech/repo-dashboard-rs:build` is an action-cache\n"
         "hit that starts no process. When its action does run it first asks whether the artifact is current (`currentDashboardArtifactV1`: valid\n"
         "for this host and mode, no file of its freshness basis changed, the executable in place) and then runs\n"
         "no nested `compile` at all; it says so in one line. Installation is a separate local step (`installDashboardArtifactV1`): it refuses a\n"
         "missing, foreign or stale artifact, answers at once (refreshing only the record) when `installed.json` already names the same bytes, and\n"
         "otherwise pins the executable into the tools store. `install`, the launcher's rebuild (`📜️script.ts build` outside the build tool, which\n"
         "runs the build only when the artifact is not current, then installs) and the test routes install that way. A change of the\n"
         "build tool's own code does not make a current artifact stale; removing that artifact directory and running `build --skip-cache` forces\n"
         "a rebuild.\n"),
    ], out)
    for rel, text in out.items():
        with open(os.path.join(root, rel), "w", encoding="utf8") as handle:
            handle.write(text)
        print("edited", rel[-80:])


main()
