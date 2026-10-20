#!/usr/bin/env python3
"""🧷️ Cuts staged/plan.json: every anchored edit of the pm-seam set, proven against the live tree and against the live tree with the round-5 lock-sync batch applied on top (counts must be identical or listed). Read only on the repository."""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..')))
import paths, overlay
from winpath import iter_logical
B, D = paths.BUILD, paths.DEPENDENCIES
CATALOG = "🔌️managers/📚️catalog/🟦️.ts"
edits = []
def edit(path, id, group, old, new, count=1):
    edits.append({"path": path, "id": id, "group": group, "old": old, "new": new, "count": count})
# ---- bootstrap (build library) ----
BOOT = B + "/🚀️bootstrap/🟦️.ts"
edit(BOOT, "bootstrap-import", "bootstrap", 'import {createHash} from "node:crypto";',
     'import {createHash} from "node:crypto";\nimport {bootstrapKeyV1,lockedVersionV1,type LockGraphV1} from "../../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🟦️.ts";\nimport {currentHostV1,selectPackageManagerV1} from "../../⚡️caching/🚀️bootstrap/📦️dependencies/' + CATALOG + '";')
edit(BOOT, "bootstrap-key", "bootstrap",
     'lock=bounded(join(root,"bun.lock"),8388608),key=createHash("sha256").update(JSON.stringify([manifest.dependencies,manifest.devDependencies,manifest.optionalDependencies,manifest.patchedDependencies,process.versions.bun,process.platform,process.arch])).update(lock).digest("hex");',
     'packageManager=selectPackageManagerV1("javascript"),packageHost=currentHostV1(),lock=bounded(join(root,packageManager.primaryLockName()),8388608),key=bootstrapKeyV1(packageManager,{manifest,host:packageHost,lock}),lockGraph=(()=>{let graph:LockGraphV1|undefined;return()=>graph??=packageManager.parseLock(lock);})();')
edit(BOOT, "bootstrap-recheck", "bootstrap", '!bounded(join(root,"bun.lock"),8388608).equals(lock)', '!bounded(join(root,packageManager.primaryLockName()),8388608).equals(lock)')
edit(BOOT, "bootstrap-locked-version", "bootstrap",
     'escaped=name.replace(/[.*+?^${}()|[\\]\\\\]/g,"\\\\$&"),locked=new RegExp(\'"\'+escaped+\'"\\\\s*:\\\\s*\\\\[\\\\s*"\'+escaped+\'@([^"\\\\s]+)"\').exec(lock.toString())?.[1];',
     'locked=lockedVersionV1(lockGraph(),name);')
edit(BOOT, "bootstrap-generation-directory", "bootstrap", 'join(store.root,"j",binding.generation.slice(0,20),"node_modules")', 'join(store.root,packageManager.generationDirectory(),binding.generation.slice(0,20),"node_modules")')
# ---- graph membership ----
edit(B + "/🔣️policy.json", "graph-policy", "graph", '      "go.mod",\n      "bun.lock"\n    ],', '      "go.mod"\n    ],\n    "lockEcosystems": [\n      "javascript"\n    ],')
GRAPH = B + "/🕸️graph/🟦️.ts"
edit(GRAPH, "graph-import", "graph", 'import{fileURLToPath}from"node:url";', 'import{fileURLToPath}from"node:url";\nimport{selectPackageManagerV1}from"../../⚡️caching/🚀️bootstrap/📦️dependencies/' + CATALOG + '";')
edit(GRAPH, "graph-members", "graph", 'const {basenames,suffixes}=graphDeclarationV1("../🔣️policy.json").graphMembers as {basenames:string[];suffixes:string[]},names=new Set(basenames);', 'const {basenames,suffixes,lockEcosystems}=graphDeclarationV1("../🔣️policy.json").graphMembers as {basenames:string[];suffixes:string[];lockEcosystems:string[]},names=new Set([...basenames,...lockEcosystems.flatMap(ecosystem=>selectPackageManagerV1(ecosystem).lockFileNames())]);')
OBS = B + "/👁️observation/🧪️tests/⚡️incremental/🟦️.ts"
edit(OBS, "graph-test-import", "graph", 'import {buildGraphMembersV1} from "../../../🕸️graph/🟦️.ts";', 'import {buildGraphMembersV1} from "../../../🕸️graph/🟦️.ts";\nimport {selectPackageManagerV1} from "../../../../⚡️caching/🚀️bootstrap/📦️dependencies/' + CATALOG + '";')
edit(OBS, "graph-test-oracle", "graph", 'const members=(policy as {graphMembers:{basenames:string[];suffixes:string[]}}).graphMembers,oracle=picomatch([`**/{${members.basenames.join(",")}}`,', 'const members=(policy as {graphMembers:{basenames:string[];suffixes:string[];lockEcosystems:string[]}}).graphMembers,oracle=picomatch([`**/{${[...members.basenames,...members.lockEcosystems.flatMap(ecosystem=>selectPackageManagerV1(ecosystem).lockFileNames())].join(",")}}`,')
# ---- javascript dependency storage ----
STORE = D + "/💾️storage/🟦️.ts"
edit(STORE, "storage-import", "storage", 'import {createHash} from "node:crypto";',
     'import {createHash} from "node:crypto";\nimport {currentHostV1,selectPackageManagerV1} from "../' + CATALOG + '";\nimport {parseJsoncV1} from "../🔌️managers/📖️formats/🟦️.ts";\n'
     'const packageManagerV1=selectPackageManagerV1("javascript"),lockNameV1=packageManagerV1.primaryLockName(),scopeConfigV1=packageManagerV1.configurationNames("scope")[0]!,generationDirectoryV1=packageManagerV1.generationDirectory(),packageManagerRuntimeV1=()=>packageManagerV1.runtimeIdentity(currentHostV1()),readLockDocumentV1=(text:string):any=>parseJsoncV1(text);\n'
     '/** 📁️ Names the store directory every generation of the selected JavaScript manager lives in. */\nexport const javascriptDependencyGenerationDirectoryV1=generationDirectoryV1;')
edit(STORE, "storage-arguments", "storage",
     'return Object.freeze(["install",mode==="sync"?"--frozen-lockfile":"--lockfile-only","--"+recipe.lifecycle,"--linker="+recipe.linker,"--backend="+backend,"--cache-dir",cache]);}',
     'return Object.freeze(packageManagerV1.render(mode,{platform:process.platform,cache}).arguments);}')
edit(STORE, "storage-environment", "storage",
     'for(const candidate of[path.join(home,".bunfig.toml"),path.join(config,".bunfig.toml"),path.join(home,".npmrc")])javascriptDependencyProducerArgumentsV1("sync",candidate);Object.assign(output,{HOME:home,USERPROFILE:home,XDG_CONFIG_HOME:config,XDG_CACHE_HOME:cache,TMPDIR:temporary,TMP:temporary,TEMP:temporary,NODE_ENV:recipe.nodeEnvironment,BUN_INSTALL_CACHE_DIR:cache,BUN_INSTALL_GLOBAL_DIR:global,BUN_INSTALL_BIN:bin});',
     'for(const candidate of[...packageManagerV1.configurationNames("home").map(name=>path.join(home,name)),...packageManagerV1.configurationNames("config").map(name=>path.join(config,name))])javascriptDependencyProducerArgumentsV1("sync",candidate);Object.assign(output,packageManagerV1.environment(root,{},path===posix?"/":"\\\\"));')
edit(STORE, "storage-scope-configuration", "storage", 'ownConfig=join(sourceRoot,"bunfig.toml"),rootConfig=join(workspace,"bunfig.toml")', 'ownConfig=join(sourceRoot,scopeConfigV1),rootConfig=join(workspace,scopeConfigV1)')
edit(STORE, "storage-stage-configuration", "storage", 'files.push({path:join(directory,"bunfig.toml"),bytes:configuration})', 'files.push({path:join(directory,scopeConfigV1),bytes:configuration})')
edit(STORE, "storage-ambient-roles", "storage",
     '[[installSchema.$defs.ProducerRecipeProtocol.const.home,[".bunfig.toml",".npmrc"]],[installSchema.$defs.ProducerRecipeProtocol.const.config,[".bunfig.toml"]]]as const',
     '[[installSchema.$defs.ProducerRecipeProtocol.const.home,packageManagerV1.configurationNames("home")],[installSchema.$defs.ProducerRecipeProtocol.const.config,packageManagerV1.configurationNames("config")]]as const')
edit(STORE, "storage-lock-name", "storage", '"bun.lock"', 'lockNameV1', 17)
edit(STORE, "storage-runtime-identity", "storage", 'process.versions.bun', 'packageManagerRuntimeV1()', 2)
edit(STORE, "storage-lock-document", "storage", 'Bun.JSONC.parse(', 'readLockDocumentV1(', 3)
edit(STORE, "storage-generation-directory", "storage", '"j"', 'generationDirectoryV1', 36)
edit(STORE, "storage-generation-root", "storage", 'roots:["j/"+plan.generation.slice(0,20)]', 'roots:[generationDirectoryV1+"/"+plan.generation.slice(0,20)]')
edit(STORE, "storage-profiles", "storage", 'for(const profile of["bun-import","bun-require"]as const){', 'for(const profile of packageManagerV1.resolutionProfiles()){')
edit(D + "/🧾️generation/🧪️tests/🟦️.ts", "storage-pin-ambient-roles", "storage",
     '[[installSchema.$defs.ProducerRecipeProtocol.const.home,[".bunfig.toml",".npmrc"]],[installSchema.$defs.ProducerRecipeProtocol.const.config,[".bunfig.toml"]]]',
     '[[installSchema.$defs.ProducerRecipeProtocol.const.home,packageManagerV1.configurationNames("home")],[installSchema.$defs.ProducerRecipeProtocol.const.config,packageManagerV1.configurationNames("config")]]')
READING = D + "/💾️storage/🧪️tests/👓️reading/🟦️.ts"
edit(READING, "storage-pin-generation-binding", "storage",
     'posix:path.posix,win32:path.win32,sep:path.sep},api=new Function(...Object.keys(ports),ts.transpileModule(nodes.map(node=>node.getText(tree).replace(/^export\\s/u,"")).join("\\n")+";return {read:readJavascriptDependencySourceOwnerExecutionV1,publish:publishJavascriptDependencyBorrowedProvidersDescriptorV1};"',
     'generationDirectoryV1:"j",posix:path.posix,win32:path.win32,sep:path.sep},api=new Function(...Object.keys(ports),ts.transpileModule(nodes.map(node=>node.getText(tree).replace(/^export\\s/u,"")).join("\\n")+";return {read:readJavascriptDependencySourceOwnerExecutionV1,publish:publishJavascriptDependencyBorrowedProvidersDescriptorV1};"')
edit(D + "/🧪️tests/🟦️.ts", "storage-bundle-prelude", "storage",
     'await build({ stdin: { contents: "const installSchema="+JSON.stringify(dependencyInstallSchema)+";"+operation.getText(source), loader: "ts" }, outfile: module,',
     'await build({ bundle: true, stdin: { contents: "import {selectPackageManagerV1} from "+JSON.stringify(join(owner, "🔌️managers/📚️catalog/🟦️.ts"))+";const packageManagerV1=selectPackageManagerV1(\\"javascript\\");const installSchema="+JSON.stringify(dependencyInstallSchema)+";"+operation.getText(source), loader: "ts", resolveDir: owner }, outfile: module,')
INC = D + "/🧾️incremental/💾️storage/🟦️.ts"
edit(INC, "incremental-import", "storage", 'import {javascriptDependencySourceOwnerRuntimeV1,', 'import {javascriptDependencyGenerationDirectoryV1,javascriptDependencySourceOwnerRuntimeV1,')
edit(INC, "incremental-generation-directory", "storage", '"j"', 'javascriptDependencyGenerationDirectoryV1', 2)
# ---- suite registration (same shape as the sibling dependency suites) ----
CACHING = paths.LIBRARY + "/⚡️caching"
target = (
 '    "test-package-manager-seam": {\n      "executor": "repo:script",\n      "cache": false,\n      "outputs": [],\n      "dependsOn": [],\n      "options": {\n'
 '        "cwd": "' + paths.LIBRARY + '/⚡️caching",\n        "command": "bun ./📜️script.ts test package-manager-seam",\n        "forwardAllArgs": false\n      },\n'
 '      "metadata": {\n        "semio": {\n          "execution": {\n            "version": 1,\n            "kind": "source"\n          },\n          "cache": {\n            "version": 1,\n            "class": "live",\n            "reason": "exercises-host-resources"\n          }\n        }\n      }\n    },\n')
edit(CACHING + "/📋️project.json", "register-target", "register", '    "test-javascript-installed-paths": {\n', target + '    "test-javascript-installed-paths": {\n')
edit(CACHING + "/🧪️tests/🔬️script-commands/🟦️.ts", "register-suite", "register", '"javascript-incremental-sync":"🚀️bootstrap/📦️dependencies/🧾️incremental/🧪️tests/🟦️.ts",', '"package-manager-seam":"🚀️bootstrap/📦️dependencies/🔌️managers/🧪️tests/🟦️.ts","javascript-incremental-sync":"🚀️bootstrap/📦️dependencies/🧾️incremental/🧪️tests/🟦️.ts",')
def prove():
    by_path, problems = {}, []
    variants = {}
    for item in edits:
        if item["path"] not in variants:
            variants[item["path"]] = {"live": overlay.live(item["path"]), "after": overlay.after_lock_sync(item["path"])}
    for item in edits:
        seen = set()
        for name, text in variants[item["path"]].items():
            seen.add(text.count(item["old"]))
        if seen != {item["count"]}:
            problems.append("%s [%s] anchor counts %s, expected %d" % (item["path"].split("/")[-2], item["id"], sorted(seen), item["count"]))
        by_path.setdefault(item["path"], []).append({"id": item["id"], "group": item["group"], "old": item["old"], "new": item["new"], "counts": [item["count"]]})
    return by_path, problems
def main():
    by_path, problems = prove()
    if problems:
        print("\n".join(problems)); sys.exit(1)
    new_files = []
    for relative in iter_logical(paths.NEW):
        new_files.append({"path": relative, "group": "ratchet" if "⚖️ratchet" in relative else "seam"})
    new_files.sort(key=lambda row: row["path"])
    plan = {"version": 1, "files": [{"path": path, "edits": rows} for path, rows in by_path.items()], "newFiles": new_files}
    out = os.path.join(paths.HERE, "staged", "plan.json")
    json.dump(plan, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("edits", sum(len(rows) for rows in by_path.values()), "files", len(by_path), "new", len(new_files))
main()
