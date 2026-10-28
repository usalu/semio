#!/usr/bin/env python3
"""🧊 Batch `fresh` (lane dashhot) as anchored edits: `python3 -I edit_fresh.py <root>` rewrites the batch files of <root> from their CURRENT content.
Every anchor must match exactly once (else exit 2, nothing written); JSON files are edited as data and re-dumped in the repository style (indent 2, UTF-8).
Used to (re)build `staged/fresh/files` on top of the live base whenever a peer lands an edit in one of these files."""
import json, os, sys

L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/"
INTENT = L + "📚️library/🏗️build/🕸️graph/🦀️rust/📥️intent/"
INSTALL = L + "📚️library/⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/"
PUBLIC = L + "📚️library/📦️packages/🟦️typescript/💾️storage/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts"
TEXTS = L + "📚️library/⚡️caching/🚀️bootstrap/📖️texts/🔣️.json"
ROUTES = L + "🎛️dashboard/🛤️task-routes/🟦️.ts"
PROJECT = L + "🎛️dashboard/📦️packages/🦀️rust/📋️project.json"
README = L + "🎛️dashboard/README.md"
EXEC = L + "🎛️dashboard/🧪️tests/🧊️execution/"
RECORD = "semio.sources.json"
NAME = "^[A-Za-z0-9_][A-Za-z0-9_-]*\\.sources\\.json$"


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


def intent_schema(data):
    if "sourcesRecord" in data["properties"]:
        sys.exit("intent schema already has sourcesRecord")
    properties = {}
    for key, value in data["properties"].items():
        properties[key] = value
        if key == "binary":
            properties["sourcesRecord"] = {"description": "🧾️ The file the build publishes beside the binary, in the same atomic publication: every source Cargo compiled it from (its own dep-info) and when that build started (`semio.cargo.binary-sources/v1`). Only a binary deliverable has one.", "type": "string", "minLength": 14, "maxLength": 160, "pattern": NAME[:-1] + "(?![\\s\\S])"}
    data["properties"] = properties
    branch = data["allOf"][0]
    branch["then"]["properties"]["sourcesRecord"] = {}
    branch["else"]["properties"]["sourcesRecord"] = False


def intent_fixture(data):
    data["accepted"].append({"intent": {"version": 1, "manifest": "crate/Cargo.toml", "package": "selected", "scope": "binary", "binary": "main", "sourcesRecord": "main.sources.json"}, "arguments": ["-p", "selected", "--bin", "main"]})
    data["rejected"] += [
        {"version": 1, "manifest": "crate/Cargo.toml", "package": "selected", "scope": "package", "sourcesRecord": "main.sources.json"},
        {"version": 1, "manifest": "crate/Cargo.toml", "package": "selected", "scope": "binary", "binary": "main", "sourcesRecord": "nested/main.sources.json"},
        {"version": 1, "manifest": "crate/Cargo.toml", "package": "selected", "scope": "binary", "binary": "main", "sourcesRecord": "main.json"},
        {"version": 1, "manifest": "crate/Cargo.toml", "package": "selected", "scope": "binary", "binary": "main", "sourcesRecord": True},
    ]
    for row in data["appRoutes"]:
        if row.get("binary") == "semio":
            row["sourcesRecord"] = RECORD


def project(data):
    intent = data["targets"]["compile"]["metadata"]["semio"]["rustBuildIntent"]
    rebuilt = {}
    for key, value in intent.items():
        rebuilt[key] = value
        if key == "binary":
            rebuilt["sourcesRecord"] = RECORD
    data["targets"]["compile"]["metadata"]["semio"]["rustBuildIntent"] = rebuilt
    entries = data["targets"]["test-source-freshness"]["metadata"]["semio"]["runtimeSourceEntries"]
    entries.append("{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧊️execution/🧾️compiled/🟦️.ts")


def install_schema(data):
    data["properties"]["version"] = {"const": 4}
    data["required"] = ["version", "platform", "mode", "path", "sources", "compiled", "digest", "bytes", "permissions"]
    properties = {}
    for key, value in data["properties"].items():
        properties[key] = value
        if key == "sources":
            properties["compiled"] = {
                "description": "🧾️ What the installed executable was compiled from: the files of its own Cargo dep-info (relative to the installation root) and when that build started. The installation is behind its sources as soon as one of them is missing or was written, replaced or re-linked at or after `builtAtMs`.",
                "type": "object", "additionalProperties": False, "required": ["builtAtMs", "sources"],
                "properties": {
                    "builtAtMs": {"type": "number", "minimum": 0},
                    "sources": {"type": "array", "minItems": 1, "uniqueItems": True, "items": {"type": "string", "minLength": 1}},
                },
            }
    data["properties"] = properties
    data["definitions"]["CompiledSourceProtocol"] = {"const": {"record": RECORD, "changedAt": ["mtimeMs", "ctimeMs"]}}


def texts(data):
    data["messages"]["dashboard.installation-current"] = {
        "en": "[dashboard] {mode} is current: its {files} source files and the {compiled} files its executable was compiled from are unchanged since its build, so nothing is built",
        "de": "[dashboard] {mode} ist aktuell: seine {files} Quelldateien und die {compiled} Dateien, aus denen sein Programm gebaut wurde, sind seit seinem Build unverändert, es wird nichts gebaut",
    }


def main():
    root = os.path.abspath(sys.argv[1])
    out = {}
    edit_json(root, INTENT + "🧬️schema/🔣️.json", intent_schema, out)
    edit_json(root, INTENT + "🧪️tests/🔣️.json", intent_fixture, out)
    edit_text(root, INTENT + "🟦️.ts", [
        (" binary?:string;\n", " binary?:string;\n sourcesRecord?:string;\n"),
        ('allowed=new Set(["version","manifest","package","scope","binary","features","defaultFeatures","target","fallback"])', 'allowed=new Set(["version","manifest","package","scope","binary","sourcesRecord","features","defaultFeatures","target","fallback"])'),
        (' if(row.scope==="binary"?!name(row.binary):Object.hasOwn(row,"binary"))return fail();\n', ' if(row.scope==="binary"?!name(row.binary):Object.hasOwn(row,"binary"))return fail();\n if(Object.hasOwn(row,"sourcesRecord")&&(row.scope!=="binary"||typeof row.sourcesRecord!=="string"||row.sourcesRecord.length>160||!/' + NAME + '/u.test(row.sourcesRecord)))return fail();\n'),
        ('...(Object.hasOwn(row,"binary")?{binary:row.binary as string}:{}),', '...(Object.hasOwn(row,"binary")?{binary:row.binary as string}:{}),...(Object.hasOwn(row,"sourcesRecord")?{sourcesRecord:row.sourcesRecord as string}:{}),'),
    ], out)
    edit_text(root, INTENT + "🧪️tests/🟦️.ts", [
        ('expect(intent.binary).toBe("binary" in row?row.binary:undefined);', 'expect(intent.binary).toBe("binary" in row?row.binary:undefined);expect(intent.sourcesRecord).toBe("sourcesRecord" in row?row.sourcesRecord:undefined);'),
    ], out)
    edit_text(root, INTENT + "🏃️routing/🟦️.ts", [
        ("{producer:\"artifact\",invocation:invocation,consume:verifyCapturedCargoArtifactsV1,network:repositoryCargoNetworkV1(invocation,workspace),control:phaseControl,environment:process.env}", "{producer:\"artifact\",invocation:invocation,consume:verifyCapturedCargoArtifactsV1,network:repositoryCargoNetworkV1(invocation,workspace),control:phaseControl,environment:process.env,...(intent.sourcesRecord?{sourcesRecord:intent.sourcesRecord}:{})}"),
    ], out)
    edit_json(root, PROJECT, project, out)
    edit_json(root, INSTALL + "🧬️schema/🔣️.json", install_schema, out)
    edit_json(root, TEXTS, texts, out)
    edit_text(root, INSTALL + "🟦️.ts", [
        ('import {type CargoDependencyNetworkV1} from "../../../../📦️packages/🟦️typescript/🦀️cargo/📥️dependencies/📋️contract/🟦️.ts";\n',
         'import {type CargoDependencyNetworkV1} from "../../../../📦️packages/🟦️typescript/🦀️cargo/📥️dependencies/📋️contract/🟦️.ts";\nimport {CARGO_BINARY_SOURCES_SCHEMA_V1,cargoBinarySourcesFreshnessV1,parseCargoBinarySourcesV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";\nimport {sayBootstrapV1} from "../../📖️texts/🟦️.ts";\n'),
        ('import { existsSync, readFileSync, writeFileSync, renameSync, mkdirSync, lstatSync } from "node:fs";', 'import { existsSync, readFileSync, writeFileSync, renameSync, mkdirSync, lstatSync, statSync } from "node:fs";'),
        ('export type DashboardSources={readonly roots:readonly string[];readonly manifests:readonly string[];readonly digest:string};\n',
         'export type DashboardSources={readonly roots:readonly string[];readonly manifests:readonly string[];readonly digest:string};\n\n'
         '/** 🗃️ What an installed executable was compiled from: the files of its own Cargo dep-info, relative to the installation root, and when that build started. */\n'
         'export type DashboardCompiledSources={readonly builtAtMs:number;readonly sources:readonly string[]};\n'),
        ('/** ⏳️ Why the installed executable no longer matches the sources, or nothing while it is current. */\n',
         '/** 🕰️ The first compiled source that is missing or was written, replaced or re-linked at or after the build started, or nothing: Cargo\'s own freshness rule (`cargoBinarySourcesFreshnessV1`) over the whole dependency closure of the binary, read on the later of modification and inode change time so a replacement that keeps old timestamps counts. Only stats, no bytes. */\n'
         'export function changedDashboardSourceV1(root:string,compiled:DashboardCompiledSources):string|undefined{\n'
         ' return cargoBinarySourcesFreshnessV1({schema:CARGO_BINARY_SOURCES_SCHEMA_V1,...compiled},path=>{try{const facts=statSync(resolve(root,path));return Math.max(facts.mtimeMs,facts.ctimeMs);}catch{return null;}}).changed??undefined;\n'
         '}\n\n'
         '/** 📥️ Admits the sources record Cargo published beside the executable as installation-root-relative paths; a missing or malformed record refuses the installation. */\n'
         'function compiledDashboardSourcesV1(root:string,file:string):DashboardCompiledSources{\n'
         ' let text:string;try{text=readFileSync(file,"utf8");}catch{throw Error("Dashboard executable sources record required");}\n'
         ' const record=parseCargoBinarySourcesV1(text);if(record===null)throw Error("Dashboard executable sources record required");\n'
         ' return {builtAtMs:record.builtAtMs,sources:[...new Set(record.sources.map(source=>slashes(relative(root,resolve(root,source)))))].sort()};\n'
         '}\n\n'
         '/** ✅️ The installed executable and the number of files it was compiled from when this host and mode\'s installation records exactly these captured sources, none of its compiled sources changed since its build and its immutable bytes are still in place with their recorded size and permissions; nothing otherwise. Only stats, no bytes. */\n'
         'export function currentDashboardV1(root:string,mode:BuildMode,workspace:string,sources:DashboardSources):{readonly executable:string;readonly compiled:number}|undefined{\n'
         ' let record;try{record=JSON.parse(readFileSync(manifest(root,mode),"utf8"));}catch{return;}\n'
         ' if(!validDashboardRecord(record,mode)||record.sources.digest!==sources.digest||JSON.stringify(record.sources.roots)!==JSON.stringify(sources.roots)||JSON.stringify(record.sources.manifests)!==JSON.stringify(sources.manifests)||changedDashboardSourceV1(root,record.compiled)!==undefined)return;\n'
         ' const executable=resolve(workspace,record.path);\n'
         ' try{const facts=lstatSync(executable);return facts.isFile()&&facts.size===record.bytes&&(facts.mode&0o777)===record.permissions?{executable,compiled:record.compiled.sources.length}:undefined;}catch{return;}\n'
         '}\n\n'
         '/** ⏭️ Says that a build of this mode has nothing to do, in the language of the command. */\n'
         'export function sayDashboardCurrentV1(mode:BuildMode,sources:DashboardSources,compiled:number):void{sayBootstrapV1("dashboard.installation-current",{mode,files:Number.parseInt(sources.digest),compiled});}\n\n'
         '/** ⏳️ Why the installed executable no longer matches the sources, or nothing while it is current. */\n'),
        ('  const current=await digestDashboardSources(root,sources);\n  if(current===sources.digest)return;\n  const before=Number.parseInt(sources.digest),after=Number.parseInt(current);\n  return `its sources changed since installation (${before===after?`${after} files`:`${before} -> ${after} files`})`;\n',
         '  const current=await digestDashboardSources(root,sources);\n  if(current!==sources.digest){const before=Number.parseInt(sources.digest),after=Number.parseInt(current);return `its sources changed since installation (${before===after?`${after} files`:`${before} -> ${after} files`})`;}\n  const changed=changedDashboardSourceV1(root,record.compiled);\n  return changed===undefined?undefined:`its compiled source ${changed} changed after the build that produced it`;\n'),
        ('const executable=await pinExecutableArtifact(artifact.path,', 'const compiled=compiledDashboardSourcesV1(root,join(dirname(artifact.path),installationSchema.definitions.CompiledSourceProtocol.const.record)),executable=await pinExecutableArtifact(artifact.path,'),
        ('JSON.stringify({version:3,platform:process.platform,mode,path,sources:recorded,digest:', 'JSON.stringify({version:4,platform:process.platform,mode,path,sources:recorded,compiled,digest:'),
    ], out)
    edit_text(root, PUBLIC, [
        ("export { captureDashboardSources, dashboardBuildDescendantsV1,", "export { captureDashboardSources, changedDashboardSourceV1, currentDashboardV1, dashboardBuildDescendantsV1,"),
        ("launchDashboard, mayRunStale, staleDashboard, withInstalledDashboardV1 }", "launchDashboard, mayRunStale, sayDashboardCurrentV1, staleDashboard, withInstalledDashboardV1 }"),
        ("export type { DashboardExecutionOptions }", "export type { DashboardCompiledSources, DashboardExecutionOptions }"),
    ], out)
    edit_text(root, ROUTES, [
        ("import { captureDashboardSources, installedDashboard,", "import { captureDashboardSources, currentDashboardV1, sayDashboardCurrentV1, installedDashboard,"),
        ("/** 🔨️ Builds the selected executable and installs it; the sources are captured first so edits made during the build stay detectable, and Ctrl+C cancels the build. */\n",
         "/** 🔨️ Builds the selected executable and installs it, unless the installation of this mode is current for the captured sources and everything its executable was compiled from (`currentDashboardV1`), then no nested build runs at all; the sources are captured first so edits made during the build stay detectable, and Ctrl+C cancels the build. */\n"),
        ("  const sources = await captureDashboardSources(packageRoot, workspace);compilerAdmissionBudgetV1(options.control);\n",
         "  const sources = await captureDashboardSources(packageRoot, workspace);compilerAdmissionBudgetV1(options.control);\n  const current=currentDashboardV1(workspace,mode,options.workspace,sources);\n  if(current){sayDashboardCurrentV1(mode,sources,current.compiled);return current.executable;}\n"),
        ('await this.bun([join(this.root,"../../🧪️tests/🧊️execution/🟦️.ts"),"--timeout","120000","--test-name-pattern","source digest"],"dashboard:source-freshness",options,{SEMIO_TEST_ARTIFACT_DIR:artifacts});return;',
         'await this.bun([join(this.root,"../../🧪️tests/🧊️execution/🟦️.ts"),"--timeout","120000","--test-name-pattern","source digest"],"dashboard:source-freshness",options,{SEMIO_TEST_ARTIFACT_DIR:artifacts});await this.bun([join(this.root,"../../🧪️tests/🧊️execution/🧾️compiled/🟦️.ts"),"--timeout","120000"],"dashboard:compiled-freshness",options,{SEMIO_TEST_ARTIFACT_DIR:artifacts});return;'),
        ('await this.bun([join(this.root,"../../🧪️tests/🧊️execution/🟦️.ts")],"dashboard:execution-test",options,{SEMIO_TEST_ARTIFACT_DIR:artifacts});return;',
         'await this.bun([join(this.root,"../../🧪️tests/🧊️execution/🟦️.ts"),join(this.root,"../../🧪️tests/🧊️execution/🧾️compiled/🟦️.ts")],"dashboard:execution-test",options,{SEMIO_TEST_ARTIFACT_DIR:artifacts});return;'),
    ], out)
    edit_text(root, EXEC + "🟦️.ts", [
        ('record:any={version:3,platform:process.platform,mode:selected,path:`.cache/b/t/m/dashboard-cli/${selected}/${digest}/${name}`,sources:await captureDashboardSources(packageRoot,root),digest,bytes:1,permissions:0o755}',
         'record:any={version:4,platform:process.platform,mode:selected,path:`.cache/b/t/m/dashboard-cli/${selected}/${digest}/${name}`,sources:await captureDashboardSources(packageRoot,root),compiled:{builtAtMs:Date.now()+1,sources:["mod/📦️packages/🦀️rust/🦀️.rs"]},digest,bytes:1,permissions:0o755}'),
        ('publish=async(bytes:string|Buffer)=>{writeFileSync(source,bytes);chmodSync(source,0o755);await stageArtifacts(published,relative(process.cwd(),manifest).replaceAll("\\\\","/"),new Map([[name,source]]),',
         'publish=async(bytes:string|Buffer)=>{writeFileSync(source,bytes);chmodSync(source,0o755);const sourcesRecord=source+".sources.json";writeFileSync(sourcesRecord,JSON.stringify({schema:"semio.cargo.binary-sources/v1",builtAtMs:Date.now(),sources:[manifest]}));await stageArtifacts(published,relative(process.cwd(),manifest).replaceAll("\\\\","/"),new Map([[name,source],[installationSchema.definitions.CompiledSourceProtocol.const.record,sourcesRecord]]),'),
        ('expect(record.permissions).toBe(0o755);', 'expect(record.permissions).toBe(0o755);expect(record.compiled.sources).toEqual(["p/Cargo.toml"]);'),
    ], out)
    edit_text(root, EXEC + "🥒️.feature", [
        ("    And replacing an independent dependency lock remains outside the captured scope\n",
         "    And replacing an independent dependency lock remains outside the captured scope\n\n"
         "  Scenario: Files the executable includes from outside its crate roots make the installation stale\n"
         "    Given an installation records the files of its executable's own Cargo dep-info and when that build started\n"
         "    When a schema outside every crate root or a TypeScript file that the binary includes is edited, replaced with its old timestamps or removed\n"
         "    Then the installation is stale although the source digest is unchanged, and the reason names that file\n"
         "    And reading an included file or editing an unrelated file keeps the installation current\n\n"
         "  Scenario: A build of a current installation runs no nested build\n"
         "    Given the installation of the selected mode records exactly the captured sources and none of its compiled files changed\n"
         "    And its immutable executable is in place with the recorded size and permissions\n"
         "    When the dashboard build task runs\n"
         "    Then it answers that executable without starting the nested compile and says so in one line in the language of the command\n"
         "    And a removed, resized or re-permissioned executable, a changed source or a changed compiled file sends it on to build\n"),
    ], out)
    edit_text(root, README, [
        ("files that must and must not change it): choose `@semio-tech/repo-dashboard-rs:test-execution` in New Task.\n",
         "files that must and must not change it): choose `@semio-tech/repo-dashboard-rs:test-execution` in New Task.\n\n"
         "An installation also records what its executable was compiled from: the `compile` intent declares `sourcesRecord`, so Cargo's own\n"
         "dep-info of the binary (every file of the dependency closure, including `include_str!` files outside the crate roots) and the build\n"
         "start are published beside it and copied into `installed.json` (`compiled`). `repo-dashboard-rs:build` asks first whether the\n"
         "installation of the mode is current (`currentDashboardV1`: the same captured sources, no compiled source written, replaced or re-linked\n"
         "at or after the build start, the immutable executable in place) and then runs no nested `compile` at all; it says so in one line. The\n"
         "same rule makes `repo dashboard` rebuild when a compiled file outside the source digest changes. A change of the build tool's own code\n"
         "does not make a current installation stale; `repo run @semio-tech/repo-dashboard-rs:compile` then `:install` forces a rebuild.\n"),
    ], out)
    for rel, text in out.items():
        path = os.path.join(root, rel)
        with open(path, "w", encoding="utf8") as handle:
            handle.write(text)
        print("edited", rel[-80:])


main()
