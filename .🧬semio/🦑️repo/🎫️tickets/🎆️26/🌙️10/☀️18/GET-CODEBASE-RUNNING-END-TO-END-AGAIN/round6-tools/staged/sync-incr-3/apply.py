#!/usr/bin/env python3
"""Anchored, idempotent, all-or-nothing apply of the sync-incr round-3 set (supersedes staged2): install-equivalent generation adoption (see C/result-sync-incr.md section 5).

usage: apply.py check                      read only; rc 0 applied, rc 1 pending, rc 2 refused
       apply.py apply [--write|--dry-run]  dry run unless --write; refuses on drift or while a dependency sync/lock/show runs
       apply.py mirror DIR [--root R]      scratch tree: real copies along the patched paths, symlinks elsewhere, set applied
"""
import hashlib
import json
import os
import shutil
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")
DEFAULT_ROOT = os.path.abspath(os.path.join(HERE, *[".."] * 9))
LIBRARY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
DEPENDENCIES = LIBRARY + "/⚡️caching/🚀️bootstrap/📦️dependencies"
STORAGE = DEPENDENCIES + "/💾️storage/🟦️.ts"
ADOPT = DEPENDENCIES + "/🪞️adopt"
INSTALL = DEPENDENCIES + "/💾️storage/🧬️schema/📥️install.json"
STORAGE_TESTS = DEPENDENCIES + "/💾️storage/🧪️tests/🟦️.ts"
READING_TESTS = DEPENDENCIES + "/💾️storage/🧪️tests/👓️reading/🟦️.ts"
READING_FIXTURE = DEPENDENCIES + "/💾️storage/🧪️tests/👓️reading/🔣️.json"
CACHING = LIBRARY + "/⚡️caching"
SCRIPT_COMMANDS = CACHING + "/🧪️tests/🔬️script-commands/🟦️.ts"
CACHING_PROJECT = CACHING + "/📋️project.json"
NEW = {
    "core.ts": ADOPT + "/🟦️.ts",
    "glue.ts": ADOPT + "/💾️storage/🟦️.ts",
    "schema.json": ADOPT + "/🧬️schema/🔣️.json",
    "adopt.test.ts": ADOPT + "/🧪️tests/🟦️.ts",
    "adopt.fixture.json": ADOPT + "/🧪️tests/🔣️.json",
    "adopt.oracle.py": ADOPT + "/🧪️tests/🐍️oracle.py",
}
FULL = [LIBRARY, "🧰️framework/🔨️modules/🏃️process", "🧰️framework/🔨️modules/📁️filesystem", "🧰️framework/🔨️modules/🧬️schema"]
TMP = ".sync-incr3.tmp"


class Edit:
    def __init__(self, label, old, new, count=1, marker=None):
        self.label, self.old, self.new, self.count, self.marker = label, old, new, count, marker or new


def read(path):
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def staged(name):
    return read(os.path.join(STAGED, name))


def edits():
    storage = [
        Edit("inert fields import", 'import type {InstalledLedgerSinkV1} from "../🧾️incremental/💾️storage/🟦️.ts";\n',
             'import type {InstalledLedgerSinkV1} from "../🧾️incremental/💾️storage/🟦️.ts";\n'
             'import {adoptProtocolV1} from "../🪞️adopt/🟦️.ts";\n'
             'const javascriptDependencyInertManifestFieldsV1:ReadonlySet<string>=new Set(adoptProtocolV1.adopt.inertManifestFields);\n', marker="javascriptDependencyInertManifestFieldsV1:ReadonlySet"),
        Edit("member manifests drop inert fields", 'canonical=Object.fromEntries(Object.entries(document).filter(([key])=>key!=="workspaces"))',
             'canonical=Object.fromEntries(Object.entries(document).filter(([key])=>key!=="workspaces"&&!javascriptDependencyInertManifestFieldsV1.has(key)))', marker="key!==\"workspaces\"&&!javascriptDependencyInertManifestFieldsV1"),
        Edit("root manifest drops inert fields", 'manifest=JSON.parse(capture(rootPath)!.toString()) as Record<string,any>;',
             'manifest=Object.fromEntries(Object.entries(JSON.parse(capture(rootPath)!.toString())as Record<string,any>).filter(([key])=>!javascriptDependencyInertManifestFieldsV1.has(key)))as Record<string,any>;', marker="manifest=Object.fromEntries(Object.entries(JSON.parse(capture(rootPath)"),
        Edit("ready interface v3", 'readonly version:2;readonly purpose:"current-dependency-source-owner-ready"', 'readonly version:3;readonly purpose:"current-dependency-source-owner-ready"'),
        Edit("ready interface without execution", 'readonly recipeSha256:string;readonly executionSha256:string;readonly members', 'readonly recipeSha256:string;readonly members'),
        Edit("ready fields without execution", '"recipeSha256","executionSha256","members"', '"recipeSha256","members"'),
        Edit("ready receipt v3", '{version:2,purpose:"current-dependency-source-owner-ready",scope,generation:selected.plan.generation,', '{version:3,purpose:"current-dependency-source-owner-ready",scope,generation:selected.plan.generation,'),
        Edit("ready receipt without execution", 'recipeSha256:projection.recipeSha256,executionSha256:fingerprint.sha256,members:', 'recipeSha256:projection.recipeSha256,members:'),
        Edit("ready identity without execution", '{scopes:[{scope,sha256:ready.value.executionSha256}]}', '{scopes:[{scope,sha256:"0".repeat(64)}]}'),
        Edit("adopter, runtime and seal",
             'const incremental=await import("../🧾️incremental/💾️storage/🟦️.ts"),gateContext=',
             'const incremental=await import("../🧾️incremental/💾️storage/🟦️.ts"),adopter=await import("../🪞️adopt/💾️storage/🟦️.ts"),\n'
             ' adoptRuntime={bun:process.versions.bun,platform:process.platform as "darwin"|"linux"|"win32",arch:process.arch,recipeSha256:adopter.adoptRecipeSha256V1(installSchema.$defs.ProducerRecipeProtocol.const)},\n'
             ' adoptSeal=async(row:{plan:{generation:string;directory:string;files:readonly{path:string;bytes:Buffer}[]}})=>{try{const installed=incremental.newestJavascriptDependencyInstalledReceiptV1(store.root,row.plan.generation);if(!installed)return;await adopter.sealJavascriptDependencyAdoptReceiptV1(store.root,adopter.adoptReceiptBodyV1(row.plan.generation,adopter.adoptProjectionOfV1(row.plan.directory,row.plan.files),adoptRuntime,installed),checkpoint,()=>control.yieldContinuation());}catch(error){if(control.signal.aborted)throw error;}},\n'
             ' gateContext=', marker="adoptSeal=async(row"),
        Edit("fresh generation adopts a verified donor instead of installing",
             'let terminal=await install();checkpoint();',
             'const adoptedFrom=await adopter.adoptJavascriptDependencyInstalledTreeV1({storeRoot:store.root,generation:row.plan.generation,directory:row.plan.directory,files:row.plan.files,members:row.plan.members.length,runtime:adoptRuntime,check:checkpoint,\n'
             '  verifyDonor:donor=>incremental.verifyJavascriptDependencyInstalledReceiptV1(store.root,requireNativeStoragePathV1(store.root,join(store.root,"j",donor.slice(0,20))),donor,{check:checkpoint,tick:async work=>{checkpoint();await control.publish({resource:"javascript-installed-paths:adopt-donor:"+work,mode:"shared",elapsedMs:performance.now()-started});await control.yieldContinuation();checkpoint();}}),\n'
             '  pin:donor=>pinJavascriptDependencyGenerationsV1(frame,[donor]),\n'
             '  run:async argv=>(await withRequestedIssuedResourceControlV1(control,0,phase=>captureOwnedCommandResultV1(argv[0]!,argv.slice(1),row.plan.directory,{control:phase,env:environment,output:{maximumBytes:limits.maximumOutputBytes,maximumLines:limits.maximumOutputLines}}))).status===0,\n'
             '  scan:()=>scanJavascriptDependencyInstalledPathsV1(requireNativeStoragePathV1(context.root,join(context.root,"j",row.plan.generation.slice(0,20))),control,()=>checkJavascriptDependencyReadContextV1(context,control)),\n'
             '  remove:async path=>{await removeJavascriptDependencyTreeV1(path,async()=>{checkpoint();});}});\n'
             ' if(adoptedFrom)await writeOwnedCommandOutputV1(process.stderr,"[DATA] Store dependency generation adopted "+JSON.stringify({version:1,purpose:"current-dependency-generation-adopted",scope:row.scope,donor:adoptedFrom})+"\\n",control);\n'
             ' let terminal=adoptedFrom?({status:0,signal:null}as unknown as Awaited<ReturnType<typeof install>>):await install();checkpoint();', marker="const adoptedFrom=await adopter."),
        Edit("fresh generation seals its adoption receipt",
             'await requireJavascriptDependencyInstalledPathsIncrementalV1(workspace,context,control,row.plan.generation,checkpoint);frame.built.set(row.scope,',
             'await requireJavascriptDependencyInstalledPathsIncrementalV1(workspace,context,control,row.plan.generation,checkpoint);await adoptSeal(row);frame.built.set(row.scope,', marker="await adoptSeal(row);frame.built.set("),
        Edit("existing generation seals its adoption receipt once",
             'throw Error("Existing current Store ready semantics changed");await requireJavascriptDependencyInstalledPathsIncrementalV1(workspace,context,control,row.plan.generation,checkpoint);}',
             'throw Error("Existing current Store ready semantics changed");await requireJavascriptDependencyInstalledPathsIncrementalV1(workspace,context,control,row.plan.generation,checkpoint);if(!adopter.hasJavascriptDependencyAdoptReceiptV1(store.root,row.plan.generation))await adoptSeal(row);}', marker="hasJavascriptDependencyAdoptReceiptV1(store.root"),
    ]
    install = [
        Edit("schema description names the inert fields", "Native fields other than scripts, nested semio, array ordering,", "Native fields other than scripts, nested semio, the install-inert manifest fields named by the adopt protocol, array ordering,"),
        Edit("ready schema drops execution from required", '"recipeSha256",\n        "executionSha256",\n        "members",', '"recipeSha256",\n        "members",', marker='"recipeSha256",\n        "members",\n        "bindings",'),
        Edit("ready schema drops execution property", '"executionSha256": {\n          "type": "string",\n          "pattern": "^[a-f0-9]{64}$"\n        },\n        "members"', '"members"', marker='"recipeSha256": {\n          "type": "string",\n          "pattern": "^[a-f0-9]{64}$"\n        },\n        "members"'),
        Edit("ready schema version 3", '"properties": {\n        "version": {\n          "const": 2\n        },\n        "purpose": {\n          "const": "current-dependency-source-owner-ready"',
             '"properties": {\n        "version": {\n          "const": 3\n        },\n        "purpose": {\n          "const": "current-dependency-source-owner-ready"'),
    ]
    ready_old = 'version:2,purpose:"current-dependency-source-owner-ready"'
    ready_new = 'version:3,purpose:"current-dependency-source-owner-ready"'
    tests = [
        (STORAGE_TESTS, [Edit("storage test ready v3", ready_old + ',scope:".",ownerRelationSha256:"b".repeat(64),groupSha256:"c".repeat(64),recipeSha256:"d".repeat(64),executionSha256:"e".repeat(64),ownerReferences',
                              ready_new + ',scope:".",ownerRelationSha256:"b".repeat(64),groupSha256:"c".repeat(64),recipeSha256:"d".repeat(64),ownerReferences')]),
        (READING_TESTS, [
            Edit("reading ready v3 (a)", ready_old + ',scope:".",generation,ownerRelationSha256:"b".repeat(64),groupSha256:"c".repeat(64),recipeSha256:"d".repeat(64),executionSha256:"e".repeat(64),members',
                 ready_new + ',scope:".",generation,ownerRelationSha256:"b".repeat(64),groupSha256:"c".repeat(64),recipeSha256:"d".repeat(64),members'),
            Edit("reading ready v3 (b)", ready_old + ',scope:"b",generation,ownerRelationSha256:digest("relation"),groupSha256:digest("group"),recipeSha256:digest("recipe"),executionSha256:digest("same-instance-proof"),',
                 ready_new + ',scope:"b",generation,ownerRelationSha256:digest("relation"),groupSha256:digest("group"),recipeSha256:digest("recipe"),'),
            Edit("reading ready mutation", 'if(vector.mode==="provider-missing")delete input.executionSha256;', 'if(vector.mode==="provider-missing")delete input.recipeSha256;'),
            Edit("reading ready derivation", 'expect(result.value.version).toBe(2);expect(result.value.executionSha256).toBe(execution.scopes[1]!.sha256);',
                 'expect(result.value.version).toBe(3);expect(Object.keys(result.value)).not.toContain("executionSha256");'),
            Edit("reading ready oracle version", "select json_extract(?1,'$.version')=2 and json_extract(?1,'$.scope')='b'", "select json_extract(?1,'$.version')=3 and json_extract(?1,'$.scope')='b'"),
            Edit("reading ready v3 (c)", ready_old + ',scope:"b",generation,ownerRelationSha256:scope.relationSha256,groupSha256:"c".repeat(64),recipeSha256,executionSha256:"d".repeat(64),members',
                 ready_new + ',scope:"b",generation,ownerRelationSha256:scope.relationSha256,groupSha256:"c".repeat(64),recipeSha256,members'),
        ]),
        (READING_FIXTURE, [
            Edit("reading fixture v3", '"expected": {\n  "version": 2,', '"expected": {\n  "version": 3,'),
            Edit("reading fixture without execution", '  "executionSha256": "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",\n', '', marker='dddd",\n  "members": []'),
        ]),
    ]
    registry = [Edit("suite registry", '"javascript-incremental-sync":"🚀️bootstrap/📦️dependencies/🧾️incremental/🧪️tests/🟦️.ts",',
                     '"javascript-incremental-sync":"🚀️bootstrap/📦️dependencies/🧾️incremental/🧪️tests/🟦️.ts","javascript-adopt":"🚀️bootstrap/📦️dependencies/🪞️adopt/🧪️tests/🟦️.ts",', marker='"javascript-adopt"')]
    row = lambda name: '    "test-javascript-' + name + '": {"executor":"repo:script","cache":false,"outputs":[],"dependsOn":[],"options":{"cwd":"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching","command":"bun ./📜️script.ts test javascript-' + name + '","forwardAllArgs":false},"metadata":{"semio":{"execution":{"version":1,"kind":"source"},"cache":{"version":1,"class":"live","reason":"exercises-host-resources"}}}},\n'
    target = [Edit("suite target", '    "test-javascript-incremental-sync": {"executor"', row("adopt") + '    "test-javascript-incremental-sync": {"executor"', marker='"test-javascript-adopt"')]
    return [(STORAGE, storage), (INSTALL, install), (SCRIPT_COMMANDS, registry), (CACHING_PROJECT, target)] + tests


def patch(text, edit):
    if edit.marker in text:
        return text, "present"
    found = text.count(edit.old)
    if found != edit.count:
        raise Refused("anchor of '%s' must occur %d time(s), found %d" % (edit.label, edit.count, found))
    return text.replace(edit.old, edit.new), "replace"


class Refused(Exception):
    pass


def busy():
    out = subprocess.run(["ps", "-axo", "command"], capture_output=True, text=True).stdout.splitlines()
    return [line for line in out if ("dependencies sync" in line or "dependencies lock" in line or "repo show projects" in line) and "apply.py" not in line and "grep" not in line]


def plan(root):
    texts, rows = {}, []
    for relative, items in edits():
        text = read(os.path.join(root, relative))
        for edit in items:
            text, how = patch(text, edit)
            rows.append((os.path.basename(os.path.dirname(relative)) + "/" + os.path.basename(relative), edit.label, how))
        texts[relative] = text
    for key, relative in NEW.items():
        data = staged(key)
        target = os.path.join(root, relative)
        rows.append((relative.split("/")[-2] + "/" + key, "new file", "present" if os.path.exists(target) and read(target) == data else "write"))
    return texts, rows


def run(root, write, guard=True):
    try:
        texts, rows = plan(root)
    except Refused as error:
        print("REFUSED, nothing written: %s" % error)
        return 2
    for row in rows:
        print("%-34s %-44s %s" % row)
    pending = [row for row in rows if row[2] != "present"]
    print("%d operations, %d pending%s" % (len(rows), len(pending), "" if write else " (dry run)"))
    if write and pending:
        running = busy() if guard else []
        if running:
            print("REFUSED, a dependency sync, lock or show projects runs: %s" % running[0][:160])
            return 2
        for relative, text in texts.items():
            path = os.path.join(root, relative)
            if read(path) != text:
                with open(path + TMP, "w", encoding="utf-8") as handle:
                    handle.write(text)
                os.replace(path + TMP, path)
        for key, relative in NEW.items():
            target = os.path.join(root, relative)
            os.makedirs(os.path.dirname(target), exist_ok=True)
            with open(target + TMP, "w", encoding="utf-8") as handle:
                handle.write(staged(key))
            os.replace(target + TMP, target)
    return 1 if pending else 0


def mirror(destination, root):
    targets = [relative for relative, _ in edits()] + list(NEW.values())
    chain = set()
    for relative in targets:
        parts = relative.split("/")
        for depth in range(1, len(parts)):
            chain.add("/".join(parts[:depth]))
    shutil.rmtree(destination, ignore_errors=True)
    os.makedirs(destination)

    def build(relative_dir):
        source = os.path.join(root, relative_dir) if relative_dir else root
        target = os.path.join(destination, relative_dir) if relative_dir else destination
        os.makedirs(target, exist_ok=True)
        for name in sorted(os.listdir(source)):
            child = (relative_dir + "/" + name) if relative_dir else name
            if child in chain:
                build(child)
            elif child in targets:
                shutil.copyfile(os.path.join(root, child), os.path.join(destination, child))
            elif not os.path.exists(os.path.join(destination, child)):
                os.symlink(os.path.join(root, child), os.path.join(destination, child))

    build("")
    skip = shutil.ignore_patterns("node_modules", "dist", "target", ".cache", "*.dmg")
    for relative in FULL:
        place = os.path.join(destination, relative)
        probe = destination
        for part in os.path.relpath(os.path.dirname(place), destination).split(os.sep):
            probe = os.path.join(probe, part)
            if os.path.islink(probe):
                os.unlink(probe)
                os.makedirs(probe)
            elif not os.path.exists(probe):
                os.makedirs(probe)
        if os.path.islink(place):
            os.unlink(place)
        elif os.path.isdir(place):
            shutil.rmtree(place)
        shutil.copytree(os.path.join(root, relative), place, symlinks=True, ignore=skip)
    for name in os.listdir(root):
        if name == "node_modules" and not os.path.lexists(os.path.join(destination, name)):
            os.symlink(os.path.join(root, name), os.path.join(destination, name))
    return destination


def main():
    args = sys.argv[1:]
    root = DEFAULT_ROOT
    if "--root" in args:
        root = os.path.abspath(args[args.index("--root") + 1])
    if not args or args[0] not in ("check", "apply", "mirror"):
        raise SystemExit(__doc__)
    if args[0] == "check":
        sys.exit(run(root, False))
    if args[0] == "apply":
        sys.exit(run(root, "--write" in args and "--dry-run" not in args))
    mirror(os.path.abspath(args[1]), root)
    sys.exit(run(os.path.abspath(args[1]), True, False) if "--apply" in args else 0)


main()
