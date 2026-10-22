#!/usr/bin/env python3
"""Anchored edits of the short-path set, derived from the CURRENT live text (every anchor must occur exactly once). `build()` returns {live path: new text}."""
import json, os, sys
sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.environ.get("SEMIO_ROOT", "/Users/ueli/Documents/semio")
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = L + "/🏗️build"
BZ = B + "/🔧️tools/🏗️bazel"
PLAN = BZ + "/📐️path-plan"
ADAPTER = BZ + "/📜️script.ts"
EXEC = BZ + "/🏃️execution/🟦️.ts"
EXEC_SCHEMA = BZ + "/🏃️execution/🧬️schema/🔣️.json"
RUNNER = BZ + "/🏃️execution/🏭️runner/" + ("🍞️bun/" if os.path.exists(os.path.join(ROOT, BZ, "🏃️execution/🏭️runner/🍞️bun/🟦️.ts")) else "") + "🟦️.ts"
STORAGE = BZ + "/💾️storage/🟦️.ts"
PATHS_TEST = BZ + "/🧪️tests/📥️paths/🟦️.ts"
LAW_TEST = BZ + "/🏃️execution/🧪️tests/🔒️hermetic/🟦️.ts"
CLASS_LAW = BZ + "/🏃️execution/🧪️tests/🔒️classes/🟦️.ts"
NEW = {
    "schema.json": PLAN + "/🧬️schema/🔣️.json",
    "module.ts": PLAN + "/🟦️.ts",
    "test.ts": PLAN + "/🧪️tests/🟦️.ts",
    "fixture.json": PLAN + "/🧪️tests/🧫️fixtures/🔣️.json",
    "law.ts": PLAN + "/🧪️tests/🔒️real-bazel/🟦️.ts",
}
EXISTING = [ADAPTER, EXEC, EXEC_SCHEMA, RUNNER, STORAGE, PATHS_TEST, LAW_TEST, CLASS_LAW]


def read(path):
    with open(path, encoding="utf8") as handle:
        return handle.read()


def staged(name):
    return read(os.path.join(HERE, name))


def once(text, old, new, label):
    count = text.count(old)
    if count != 1:
        raise ValueError(f"{label}: anchor found {count} times: {old[:100]!r}")
    return text.replace(old, new)


def runner(text):
    t = text
    t = once(t, 'import{createHash}from"node:crypto";', 'import{createHash,randomBytes}from"node:crypto";', "runner crypto")
    t = once(t, 'import{dirname,isAbsolute,join,relative,resolve,sep}from"node:path";\nimport{pathToFileURL}from"node:url";', 'import{dirname,isAbsolute,join,parse,relative,resolve,sep}from"node:path";\nimport{pathToFileURL}from"node:url";', "runner path")
    t = once(t, ',scratch=resolve(".hermetic",sha(outputArgument).slice(0,16)),declared=manifest.inputs??[];',
             r',declared=manifest.inputs??[],staged=manifest.aliases!==undefined,rule=staged?manifest.scratch[{win32:"win32",darwin:"darwin"}[process.platform]??"linux"]:undefined,scratch=staged?resolve(rule.base.replace("{drive}",parse(process.cwd()).root.replace(/[\\/]$/,"")),randomBytes(rule.workDigits/2).toString("hex")):resolve(".hermetic",sha(outputArgument).slice(0,16));', "runner scratch")
    t = once(t, 'const source=resolve("s"),linked=declared.length>0&&(lstatSync(source).isSymbolicLink()||lstatSync(join(source,declared[0])).isSymbolicLink()),workspace=linked?join(scratch,"root","s"):source;\ntry{if(!linked&&lstatSync(workspace).isSymbolicLink())',
             'const source=resolve("s"),workspace=staged?join(scratch,rule.workspace):source;\nif(staged)rmSync(scratch,{recursive:true,force:true});\ntry{if(!staged&&lstatSync(workspace).isSymbolicLink())', "runner workspace")
    t = once(t, 'if(linked){rmSync(join(scratch,"root"),{recursive:true,force:true});for(const path of declared){const target=inside(path);mkdirSync(dirname(target),{recursive:true});clone(join(source,path),target);}}',
             'if(staged)for(const path of declared){const cut=path.lastIndexOf("/"),alias=manifest.aliases[cut<0?"":path.slice(0,cut)];if(alias===undefined)fail("an input directory without a declared alias: "+path);const target=inside(path);mkdirSync(dirname(target),{recursive:true});clone(resolve(alias,path.slice(cut+1)),target);}', "runner materialize")
    t = once(t, "It builds its own isolated workspace from the declared input list when Bazel did not isolate one (a sandbox symlinks the declared files back into the checkout, a local strategy links the whole checkout as the root), so an undeclared read fails on every host and strategy;",
             "It reads declared inputs by their short aliased labels `a/<alias>/<name>` (the manifest's `aliases` map each repository directory to its alias) and builds its own isolated workspace in a short private root of the manifest's `scratch` policy (`<base>/<work>/r`, so the root plus the longest repository path stays within the host's path budget), so an undeclared read fails on every host and strategy; without `aliases` the caller's `s` is already the isolated root;", "runner doc")
    return t


def execution(text):
    t = text
    t = once(t, 'import{hermeticDependenciesV1,type HermeticDependenciesV1,type HermeticModulesRowV1}from"../../../🏃️execution/🔒️hermetic/📦️dependencies/🟦️.ts";\n',
             'import{hermeticDependenciesV1,type HermeticDependenciesV1,type HermeticModulesRowV1}from"../../../🏃️execution/🔒️hermetic/📦️dependencies/🟦️.ts";\nimport{BAZEL_PATH_PLAN_V1}from"../📐️path-plan/🟦️.ts";\n', "exec import")
    t = once(t, 'readonly providers?:Readonly<Record<string,string>>;readonly rows?:readonly HermeticModulesRowV1[]}', 'readonly providers?:Readonly<Record<string,string>>;readonly rows?:readonly HermeticModulesRowV1[];readonly aliases?:Readonly<Record<string,string>>}', "exec details")
    t = once(t, 'inputs:[...details.inputs],', 'inputs:[...details.inputs],...(details.aliases===undefined?{}:{aliases:{...details.aliases},scratch:BAZEL_PATH_PLAN_V1.hermeticRoot}),', "exec manifest")
    t = once(t, "What the generator adds to one hermetic action's manifest beside its task: the declared input files and,", "What the generator adds to one hermetic action's manifest beside its task: the declared input files, the alias of each input directory with the private root policy (the runner reads inputs through the aliases) and,", "exec doc")
    return t


def exec_schema(text):
    d = json.loads(text)
    m = d["definitions"]["Manifest"]["properties"]
    m["aliases"] = {"description": "Repository directory to the alias `a/<hex>` its declared inputs are staged under; present when the runner builds its own private root.", "type": "object", "additionalProperties": {"type": "string", "pattern": "^a/[a-f0-9]{1,64}$"}}
    m["scratch"] = {"description": "The private root policy by platform: base, work name width, workspace directory and separator.", "type": "object", "additionalProperties": False, "required": ["darwin", "linux", "win32"], "properties": {p: {"type": "object", "additionalProperties": False, "required": ["base", "workDigits", "workspace", "separator"], "properties": {"base": {"type": "string", "minLength": 1, "maxLength": 128}, "workDigits": {"type": "integer", "minimum": 4, "maximum": 32}, "workspace": {"type": "string", "pattern": "^[a-z]$"}, "separator": {"enum": ["/", "\\"]}}} for p in ("darwin", "linux", "win32")}}
    return json.dumps(d, ensure_ascii=False, indent=2) + "\n"


def storage(text):
    t = text
    t = once(t, 'import {requireBazelSlotV1} from "../🧵️sessions/🟦️.ts";\n', 'import {requireBazelSlotV1} from "../🧵️sessions/🟦️.ts";\nimport {bazelOutputRootV1} from "../📐️path-plan/🟦️.ts";\n', "storage import")
    t = once(t, 'options:Readonly<{environment?:Readonly<Record<string,string|undefined>>;skipCache?:boolean;skipRemoteCache?:boolean}>={}', 'options:Readonly<{environment?:Readonly<Record<string,string|undefined>>;skipCache?:boolean;skipRemoteCache?:boolean;checkout?:string}>={}', "storage options")
    t = once(t, 'startup:Object.freeze([`--output_base=${bazelServerBaseV1(layout,slot)}`])', 'startup:Object.freeze((()=>{const short=options.checkout===undefined?undefined:bazelOutputRootV1(process.platform,options.checkout);return short?[`--output_user_root=${short.root}`,`--output_base=${short.base(requireBazelSlotV1(slot))}`]:[`--output_base=${bazelServerBaseV1(layout,slot)}`];})())', "storage startup")
    return t


def adapter(text):
    t = text
    t = once(t, 'import {HERMETIC_TABLE_V1} from "../../🏃️execution/🔒️hermetic/🟦️.ts";\n', 'import {HERMETIC_TABLE_V1} from "../../🏃️execution/🔒️hermetic/🟦️.ts";\nimport {bazelOutputRootV1,bazelPathLawV1,linkBazelOutputRootV1} from "./📐️path-plan/🟦️.ts";\n', "adapter import")
    t = once(t, 'bazelCacheFlagsV1(storage.paths,hold.session.slot,{environment:process.env})', 'bazelCacheFlagsV1(storage.paths,hold.session.slot,{environment:process.env,checkout:root})', "adapter flags a")
    t = once(t, 'bazelCacheFlagsV1(storage.paths,session.slot,{environment:process.env,skipCache:request.skipCache,skipRemoteCache:request.skipRemoteCache})', 'bazelCacheFlagsV1(storage.paths,session.slot,{environment:process.env,skipCache:request.skipCache,skipRemoteCache:request.skipRemoteCache,checkout:root})', "adapter flags b")
    t = once(t, 'profile=requireBazelPlatformProfileV1(cache,bazelPathClosureV1([],[])),startup=bazelStartupOptionsV1(root,storage,cache,profile,hold.servers)', 'profile=requireBazelPlatformProfileV1(cache,bazelPathClosureV1([],[])),short=bazelOutputRootV1(process.platform,root),link=short?linkBazelOutputRootV1(short.root,storage.paths.server):undefined,startup=bazelStartupOptionsV1(root,storage,cache,profile,hold.servers)', "adapter link a")
    # demotion by the path law
    t = once(t, 'environment:process.env,check:()=>checkScriptInvocation(invocation)});if(derived)hermeticPlans.set(action.id,derived);else demoted.add(action.id);}',
             'environment:process.env,check:()=>checkScriptInvocation(invocation)}),sources=[...index.files.filter((_path,position)=>marks[position]),...extra],law=bazelPathLawV1({platform:process.platform as "darwin",arch:process.arch as "arm64",longPaths:process.platform==="win32"?(readNativeWindowsLongPathsEnabledV1()?"enabled":"disabled"):"unprobed",...(bazelOutputRootV1(process.platform,root)?{outputBase:bazelOutputRootV1(process.platform,root)!.base("s0")}:{}),checkout:root},[{id:action.id,kind:"hermetic",sources}],path=>bazelSourceAliasLabelV1(path));if(derived&&law.fits)hermeticPlans.set(action.id,derived);else{demoted.add(action.id);if(derived)console.warn(`[bazel] hermetic task kept on the host, its longest path is ${law.rows[0]?.worst?.units} units (${law.rows[0]?.worst?.name}) beyond ${law.limit}: ${action.id}`);}}', "adapter law")
    t = once(t, 'view(action.kind==="hermetic"?"real":"alias")', 'view("alias")', "adapter alias view")
    t = once(t, ',repositoryLabels=new Set<string>();', ';', "adapter repositoryLabels decl")
    i = t.index(' for(const action of declared)if(action.kind==="hermetic"){const{marks,extra}=index.selectAll(inputs.get(action.id)!);for(let position=0;position<marks.length;position++)if(marks[position]&&declarable.has(index.files[position]!))repositoryLabels.add(')
    j = t.index("\n", i)
    t = t[:i] + t[j + 1:]
    t = once(t, 'bazelPathClosureV1(universe.map(path=>bazelSourceAliasLabelV1(path)),repositoryLabels)', 'bazelPathClosureV1(universe.map(path=>bazelSourceAliasLabelV1(path)),[])', "adapter closure")
    t = once(t, 'await writeChanged(join(workspace,task.label+".json"),JSON.stringify(bazelHermeticManifestV1(task,buildExecutionArgumentsV1({...task.definition.execution,forwardAllArgs:true},task.args),{inputs:paths,',
             'await writeChanged(join(workspace,task.label+".json"),JSON.stringify(bazelHermeticManifestV1(task,buildExecutionArgumentsV1({...task.definition.execution,forwardAllArgs:true},task.args),{inputs:paths,aliases:Object.fromEntries([...new Set(paths.map(path=>path.includes("/")?path.slice(0,path.lastIndexOf("/")):""))].sort().map(directory=>[directory,aliasRule.link+"/"+bazelSourceAliasV1(directory)])),', "adapter manifest")
    return t


def paths_test(text):
    t = text
    t = once(t, 'test.skipIf(!python)("the generated workspace names aliased sources for host-state actions and repository paths for hermetic ones, links the farm, records the closure and every label resolves"', 'test.skipIf(!python)("the generated workspace names aliased sources for host-state and hermetic actions alike, links the farm, records the closure and every label resolves"', "paths test title")
    t = once(t, 'expect(labels).toContain(aliasLabel);expect(labels).toContain(api.bazelSourceLabelV1("lib/src/a.txt"));', 'expect(labels).toContain(aliasLabel);expect(labels).toContain(api.bazelSourceAliasLabelV1("lib/src/a.txt"));', "paths test a")
    t = once(t, 'expect(labels.filter(label=>label.startsWith("s/")).every(label=>label.startsWith("s/lib/"))).toBe(true);', 'expect(labels.filter(label=>label.startsWith("s/"))).toEqual([]);', "paths test b")
    t = once(t, 'const aliasLabels=labels.filter(label=>label.startsWith("a/")),repositoryLabels=labels.filter(label=>label.startsWith("s/lib/"));', 'const aliasLabels=labels.filter(label=>label.startsWith("a/")),repositoryLabels:string[]=[];', "paths test c")
    return t


def law_test(text):
    t = text
    t = once(t, 'const binary=process.env.SEMIO_TEST_BAZEL_BINARY,', 'const adapter=await import("../../../📜️script.ts");\nconst binary=process.env.SEMIO_TEST_BAZEL_BINARY,', "law adapter")
    t = once(t, '[...(declared as Record<string,string[]>)[id]!.map(path=>"s/"+path),', '[...(declared as Record<string,string[]>)[id]!.map(path=>adapter.bazelSourceAliasLabelV1(path)),', "law inputs")
    return t


def class_law(text):
    t = text
    t = once(t, 'const binary=process.env.SEMIO_TEST_BAZEL_BINARY,', 'const adapter=await import("../../../📜️script.ts");\nconst binary=process.env.SEMIO_TEST_BAZEL_BINARY,', "class law adapter")
    t = once(t, 'manifest.inputs.map((path:string)=>"s/"+path)', 'manifest.inputs.map((path:string)=>adapter.bazelSourceAliasLabelV1(path))', "class law inputs")
    return t


def build():
    out = {}
    for path, fn in ((RUNNER, runner), (EXEC, execution), (EXEC_SCHEMA, exec_schema), (STORAGE, storage), (ADAPTER, adapter), (PATHS_TEST, paths_test), (LAW_TEST, law_test), (CLASS_LAW, class_law)):
        out[path] = fn(read(os.path.join(ROOT, path)))
    return out


if __name__ == "__main__":
    for path, text in build().items():
        print(path, len(text))
