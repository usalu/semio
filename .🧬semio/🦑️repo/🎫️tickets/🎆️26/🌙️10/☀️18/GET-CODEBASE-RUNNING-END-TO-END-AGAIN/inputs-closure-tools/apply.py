#!/usr/bin/env python3
"""Staged inputs-closure patch: anchored edits (each anchor must occur exactly once, idempotent) plus new files.

usage: apply.py check [--bazel-script PATH]
       apply.py apply [--dry-run] [--ignore-sync] [--bazel-script PATH]
       apply.py mirror DIR            copy the touched library directories to DIR with the patch applied, everything else symlinked
Targets: the plugin's script-inputs component and the Bazel adapter script (default: the checkout; --bazel-script composes onto another copy, e.g. bzl-stage-tools/staged/script.ts)."""
import hashlib, os, shutil, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")


def E(code):
    return chr(code) + "️"


def find_root():
    path = HERE
    while path != os.path.dirname(path):
        if os.path.isfile(os.path.join(path, "bunfig.toml")) and os.path.isfile(os.path.join(path, "bun.lock")):
            return path
        path = os.path.dirname(path)
    raise SystemExit("repository root not found")


ROOT = find_root()
LIBRARY = os.path.join(E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
BUILD = os.path.join(LIBRARY, E(0x1F3D7) + "build")
BAZEL = os.path.join(BUILD, E(0x1F527) + "tools", E(0x1F3D7) + "bazel")
INFERENCE = os.path.join(BUILD, E(0x1F578) + "graph", E(0x1F4E5) + "inference")
TS = E(0x1F7E6) + ".ts"
JSON_ = E(0x1F523) + ".json"
SCHEMA = E(0x1F9EC) + "schema"
TESTS = E(0x1F9EA) + "tests"
CLOSURE = E(0x1F517) + "closure"
WORKSPACE_CLOSURE = E(0x1F517) + "workspace-closure"
ADAPTER_SCRIPT = os.path.join(BAZEL, E(0x1F4DC) + "script.ts")
SCRIPT_INPUTS = os.path.join(INFERENCE, E(0x1F9ED) + "script-inputs", E(0x1F7E8) + ".mjs")
SOURCE_ANALYSES = os.path.join(INFERENCE, E(0x1F9EC) + "source-analyses", E(0x1F7E8) + ".mjs")
BOOTSTRAP = os.path.join(INFERENCE, E(0x1F501) + "bootstrap", E(0x1F7E8) + ".mjs")
PARSER = os.path.join(LIBRARY, E(0x26A1) + "caching", E(0x1F4E5) + "inference", E(0x1F7E8) + ".mjs")

NEW = {
    "adapter/closure/module.ts": os.path.join(BAZEL, CLOSURE, TS),
    "adapter/closure/schema.json": os.path.join(BAZEL, CLOSURE, SCHEMA, JSON_),
    "adapter/tests/module.ts": os.path.join(BAZEL, TESTS, E(0x1F4E5) + "closure", TS),
    "adapter/tests/fixture.json": os.path.join(BAZEL, TESTS, E(0x1F4E5) + "closure", JSON_),
    "adapter/tests/schema.json": os.path.join(BAZEL, TESTS, E(0x1F4E5) + "closure", SCHEMA + ".json"),
    "plugin/tests/module.ts": os.path.join(INFERENCE, TESTS, WORKSPACE_CLOSURE, TS),
    "plugin/tests/fixture.json": os.path.join(INFERENCE, TESTS, WORKSPACE_CLOSURE, JSON_),
    "plugin/tests/schema.json": os.path.join(INFERENCE, TESTS, WORKSPACE_CLOSURE, SCHEMA, JSON_),
}

def snippet(name):
    with open(os.path.join(STAGED, "snippets", name), encoding="utf8") as handle:
        return handle.read()


PLUGIN_HELPERS = '''/** \U0001F4E6️ Resolves a package specifier to the workspace module it names, so imports through `@scope/package` subpaths join a command closure like relative ones; installed registry packages stay covered by their locked external dependency. */
function workspacePackageModuleV1(containing, specifier, workspaceRoot) {
  if (specifier.startsWith(".") || isAbsolute(specifier) || isBuiltin(specifier) || /^[a-z][a-z0-9+.-]*:/u.test(specifier)) return undefined;
  let resolved;
  try { resolved = observedRequireResolve(containing, specifier); }
  catch (error) { if (error?.code === "MODULE_NOT_FOUND" || error?.code === "ERR_PACKAGE_PATH_NOT_EXPORTED") return undefined; throw error; }
  if (!isAbsolute(resolved)) return undefined;
  const local = relative(workspaceRoot, resolved);
  return local === "" || local.startsWith("..") || isAbsolute(local) || local.split(sep).includes("node_modules") ? undefined : resolved;
}

/** \U0001F9EA️ Names the literal module Source a target declares as an input (a test file it runs) so that file's import closure joins the target's closure. */
function declaredModuleInputV1(input, workspaceRoot, projectRoot) {
  if (typeof input !== "string" || input.startsWith("!") || /[*?{}[\\]]/u.test(input.replace(/^\\{(?:projectRoot|workspaceRoot)\\}/u, "")) || !/\\.[cm]?[jt]sx?$/u.test(input) || !/^\\{(?:projectRoot|workspaceRoot)\\}\\//u.test(input)) return undefined;
  const path = resolve(workspaceRoot, input.replace("{projectRoot}", projectRoot).replace("{workspaceRoot}", "."));
  return !relative(workspaceRoot, path).startsWith("..") && existsSync(path) ? path : undefined;
}

'''
PLUGIN_HELPERS = PLUGIN_HELPERS + snippet("url-target.mjs")

PLUGIN_OPS = [
    ('import { dirname, isAbsolute, join, relative, resolve } from "node:path";',
     'import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";'),
    ('const { existsSync } = observedFs;',
     'const { existsSync, statSync } = observedFs;'),
    ('/** \U0001F9ED️ Tracks literal relative command imports through the existing TypeScript tooling boundary. */',
     PLUGIN_HELPERS + '/** \U0001F9ED️ Tracks literal relative command imports through the existing TypeScript tooling boundary. */'),
    ('        imports.push({path:resolved,source:resource.source});\n      }\n      return imports;',
     '        imports.push({path:resolved,source:resource.source});\n      }\n'
     '      const references = observedSourceAnalysisV1(prepareSourceAnalysesV1().references,path);\n'
     '      for (const specifier of references.runtime) {\n'
     '        await checkpoint();\n'
     '        const member = workspacePackageModuleV1(path, specifier, workspaceRoot);\n'
     '        if (member) imports.push({path:member,source:true});\n'
     '      }\n'
     '      for (const literal of references.urls) {\n'
     '        await checkpoint();\n'
     '        const target = workspaceUrlTargetV1(path, literal, workspaceRoot);\n'
     '        if (target) imports.push(target);\n'
     '      }\n'
     '      return imports;'),
    ('    if(requiredSources)readPhysicalSource(path);\n    else if (!existsSync(path)) return;',
     '    if(requiredSources){const admitted=cache.physical??=new Set();if(!admitted.has(path)){readPhysicalSource(path);admitted.add(path);}}\n    else if (!existsSync(path)) return;'),
    (' const closure=entries.size?await relativeScriptInputs([...entries],workspaceRoot,scripts,true):undefined,',
     ' for(const input of target.inputs??[]){const declared=declaredModuleInputV1(input,workspaceRoot,projectRoot);if(declared)entries.add(declared);}\n'
     ' const closure=entries.size?await relativeScriptInputs([...entries],workspaceRoot,scripts,true):undefined,'),
]

ANALYSES_OPS = [
    ('const commandParser=loaded.commandSourceImports,runtimeParser=',
     'const commandParser=loaded.commandSourceImports,urlParser=loaded.commandUrlReferences,runtimeParser='),
    ('      command:register({id:identity("command-imports"),revision,analyze:(path,bytes)=>commandParser(path,bytes.toString("utf8"))}),',
     '      command:register({id:identity("command-imports"),revision,analyze:(path,bytes)=>commandParser(path,bytes.toString("utf8"))}),\n'
     '      references:register({id:identity("command-references"),revision,analyze:(path,bytes)=>{const text=bytes.toString("utf8");return {runtime:runtimeParser(path,text),urls:urlParser(path,text)};}}),'),
]

BOOTSTRAP_OPS = [
    ('  runtimeModuleSourceImports: (path, source) => currentCommandInputs(loadedCommandInputs).runtimeModuleSourceImports(path, source),',
     '  runtimeModuleSourceImports: (path, source) => currentCommandInputs(loadedCommandInputs).runtimeModuleSourceImports(path, source),\n'
     '  commandUrlReferences: (path, source) => currentCommandInputs(loadedCommandInputs).commandUrlReferences(path, source),'),
    ('    loaded.runtimeModuleSourceImports = admitted.runtimeModuleSourceImports;',
     '    loaded.runtimeModuleSourceImports = admitted.runtimeModuleSourceImports;\n'
     '    loaded.commandUrlReferences = admitted.commandUrlReferences;'),
]

PARSER_OPS = [
    ('/** \U0001F4CA\ufe0f Reports bounded immutable parser fact storage without source authority. */',
     snippet("parser-url-references.mjs") + '/** \U0001F4CA\ufe0f Reports bounded immutable parser fact storage without source authority. */'),
]

SIGNATURE = 'plan:BazelTaskPlanV1,workspace:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,implementation:BazelImplementationV1,recheck:()=>void)'
ADAPTER_OPS = [
    ('import {validateJsonSchemaSubset} from "../../../../../../../\U0001F528️modules/\U0001F9EC️schema/✅️validator/\U0001F4E6️packages/\U0001F7E6️typescript/\U0001F7E6️.ts";',
     'import {validateJsonSchemaSubset} from "../../../../../../../\U0001F528️modules/\U0001F9EC️schema/✅️validator/\U0001F4E6️packages/\U0001F7E6️typescript/\U0001F7E6️.ts";\n'
     'import {bazelClosureMarkerV1,bazelClosureMarkersForPlanV1,loadBazelClosuresV1,managedBazelClosureFetchV1,workspaceBazelLockFilesV1,type BazelClosureFetchV1,type BazelClosuresV1} from "./' + CLOSURE + '/' + TS + '";'),
    ('export function bazelTaskInputsV1(graph:BuildGraphV1,task:BazelTaskV1):unknown[]{',
     'export function bazelTaskInputsV1(graph:BuildGraphV1,task:BazelTaskV1,exactClosures?:BazelClosuresV1|"ignore"):unknown[]{'),
    ('  if(input&&typeof input==="object"&&"input"in input){const row=input as{input:string;projects?:string|string[];dependencies?:boolean};',
     '  const closure=bazelClosureMarkerV1(input);if(closure){if(exactClosures==="ignore")return[];if(!exactClosures)throw Error(`Bazel build input closure ${closure.kind} ${closure.key} requires its loaded manifest`);return[...exactClosures.files(closure)];}\n'
     '  if(input&&typeof input==="object"&&"input"in input){const row=input as{input:string;projects?:string|string[];dependencies?:boolean};'),
    ('const transitive=(task:BazelTaskV1)=>bazelTaskInputsV1(graph,task).some(',
     'const transitive=(task:BazelTaskV1)=>bazelTaskInputsV1(graph,task,"ignore").some('),
    ('cached:boolean,implementation:string):Promise<string>{',
     'cached:boolean,implementation:string,exactClosures:BazelClosuresV1):Promise<string>{'),
    ('?bazelTaskInputsV1(graph,task):[],digest=createHash("sha256");',
     '?bazelTaskInputsV1(graph,task,exactClosures):[],digest=createHash("sha256");'),
    (SIGNATURE,
     SIGNATURE[:-1] + ',closureFetch?:BazelClosureFetchV1)'),
    ('const emitted=bazelActionsV1(graph,plan,request.skipCache===true),declared=',
     'const emitted=bazelActionsV1(graph,plan,request.skipCache===true),exactClosures=await loadBazelClosuresV1(bazelClosureMarkersForPlanV1(graph,Object.values(plan.tasks)),closureFetch??managedBazelClosureFetchV1(root,phaseControl),workspaceBazelLockFilesV1(root)),declared='),
    ('bazelTaskInputsV1(graph,plan.tasks[action.id]!)])',
     'bazelTaskInputsV1(graph,plan.tasks[action.id]!,exactClosures)])'),
    ('action.cache.cache,implementation.sha256+implementation.loaderSha256);',
     'action.cache.cache,implementation.sha256+implementation.loaderSha256,exactClosures);'),
]


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    with open(path, "rb") as handle:
        return handle.read()


def absolute(relative):
    return os.path.join(ROOT, relative)


def option(args, name):
    return args[args.index(name) + 1] if name in args else None


def apply_ops(text, ops, label):
    applied, missing = [], []
    for index, (anchor, replacement) in enumerate(ops):
        if replacement in text:
            applied.append(index)
            continue
        count = text.count(anchor)
        if count != 1:
            missing.append((index, count))
            continue
        text = text.replace(anchor, replacement)
    if missing:
        raise SystemExit("%s: anchors not found exactly once: %s" % (label, ", ".join("op %d (%d occurrences)" % item for item in missing)))
    return text, applied


def targets(args):
    return {
        "plugin": (absolute(SCRIPT_INPUTS), PLUGIN_OPS),
        "analyses": (absolute(SOURCE_ANALYSES), ANALYSES_OPS),
        "bootstrap": (absolute(BOOTSTRAP), BOOTSTRAP_OPS),
        "parser": (absolute(PARSER), PARSER_OPS),
        "adapter": (os.path.abspath(option(args, "--bazel-script")) if option(args, "--bazel-script") else absolute(ADAPTER_SCRIPT), ADAPTER_OPS),
    }


def patched(args):
    result = {}
    for name, (path, ops) in targets(args).items():
        text = read(path).decode("utf8")
        new, applied = apply_ops(text, ops, name)
        result[name] = (path, text, new, applied)
    return result


def sync_running():
    rows = subprocess.run(["ps", "-axo", "pid=,command="], capture_output=True, text=True).stdout.splitlines()
    return [row.strip()[:120] for row in rows if "dependencies sync" in row and "bun" in row and "apply.py" not in row]


def cmd_check(args):
    for name, (path, text, new, applied) in patched(args).items():
        print("%s: %s; %d of %d edits already present; %s" % (name, path[-60:], len(applied), len(targets(args)[name][1]), "changes" if new != text else "no change"))
    for key, target in NEW.items():
        source = os.path.join(STAGED, key)
        current = os.path.exists(absolute(target))
        print("new %s -> %s (%s)" % (key, target[-70:], "identical" if current and read(absolute(target)) == read(source) else "exists, differs" if current else "absent"))


def cmd_apply(args):
    running = sync_running()
    if running and "--ignore-sync" not in args:
        print("refusing: a dependency sync is running (the freeze is not lifted):")
        for row in running:
            print("  " + row)
        raise SystemExit(2)
    rows = patched(args)
    for name, (path, text, new, applied) in rows.items():
        print("%s: %s" % (name, "would write" if "--dry-run" in args else "write") if new != text else "%s: already applied" % name)
        if new != text and "--dry-run" not in args:
            with open(path, "w", encoding="utf8") as handle:
                handle.write(new)
    for key, target in NEW.items():
        destination = absolute(target)
        data = read(os.path.join(STAGED, key))
        if os.path.exists(destination) and read(destination) == data:
            continue
        print("%s %s" % ("would write" if "--dry-run" in args else "write", target[-80:]))
        if "--dry-run" not in args:
            os.makedirs(os.path.dirname(destination), exist_ok=True)
            with open(destination, "wb") as handle:
                handle.write(data)


def cmd_mirror(args):
    directory = os.path.abspath(args[0])
    if os.path.exists(directory):
        shutil.rmtree(directory)
    os.makedirs(directory)
    base = os.path.realpath(directory)
    rows = patched(args[1:])
    copies = [LIBRARY]
    needed = set()
    for relative in copies:
        parts = relative.split(os.sep)
        for depth in range(len(parts)):
            needed.add(os.sep.join(parts[:depth]))
    needed.add("")

    def safe(path):
        if os.path.realpath(path) != base and not os.path.realpath(path).startswith(base + os.sep):
            raise SystemExit("refusing to write outside the mirror: " + path)
        return path

    for relative in sorted(needed, key=len):
        real, mirror = os.path.join(ROOT, relative), os.path.join(directory, relative)
        os.makedirs(safe(mirror), exist_ok=True)
        for entry in os.listdir(real):
            child = os.path.join(relative, entry) if relative else entry
            if child in needed or child in copies:
                continue
            destination = os.path.join(mirror, entry)
            if not os.path.lexists(destination):
                os.symlink(os.path.join(real, entry), safe(destination))
    for relative in copies:
        shutil.copytree(os.path.join(ROOT, relative), safe(os.path.join(directory, relative)), symlinks=True)
    for name, (path, text, new, applied) in rows.items():
        if "--control" in args:
            continue
        relative = {"plugin": SCRIPT_INPUTS, "analyses": SOURCE_ANALYSES, "bootstrap": BOOTSTRAP, "parser": PARSER, "adapter": ADAPTER_SCRIPT}[name]
        with open(safe(os.path.join(directory, relative)), "w", encoding="utf8") as handle:
            handle.write(new)
    for key, target in ({} if "--control" in args else NEW).items():
        destination = safe(os.path.join(directory, target))
        os.makedirs(os.path.dirname(destination), exist_ok=True)
        shutil.copyfile(os.path.join(STAGED, key), destination)
    for name in ("node_modules", "bunfig.toml", "package.json", "tsconfig.json"):
        if os.path.lexists(os.path.join(ROOT, name)) and not os.path.lexists(os.path.join(directory, name)):
            os.symlink(os.path.join(ROOT, name), safe(os.path.join(directory, name)))
    print("mirror written to " + directory)


if __name__ == "__main__":
    command, *rest = sys.argv[1:] or ["check"]
    if command == "check":
        cmd_check(rest)
    elif command == "apply":
        cmd_apply(rest)
    elif command == "mirror":
        cmd_mirror(rest)
    elif command == "dump-ops":
        import json
        print(json.dumps({"plugin": PLUGIN_OPS, "analyses": ANALYSES_OPS, "bootstrap": BOOTSTRAP_OPS, "parser": PARSER_OPS, "adapter": ADAPTER_OPS}, ensure_ascii=False))
    else:
        raise SystemExit(__doc__)
