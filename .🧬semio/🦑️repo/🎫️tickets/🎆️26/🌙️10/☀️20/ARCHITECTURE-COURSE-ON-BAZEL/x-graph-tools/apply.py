#!/usr/bin/env python3
"""Two-phase anchored apply of the x-graph set (see T/handoff-graph-closure.md).

usage: apply.py --check | --live | mirror DIR [--bare]   [--root ROOT]
Phase 1 reads every target, verifies that every anchor occurs exactly once (or the edit is already present) and computes all new texts in memory.
Any refusal exits 2 with nothing written. --check stops after phase 1 (exit 1 when edits are pending, 0 when everything is applied). --live then writes each file through a temporary file and os.replace.
`mirror DIR` builds a scratch tree (real copies along the patched paths, symlinks elsewhere, byte copies of the library and its framework modules; never a hard link) and applies the set there.
"""
import os
import shutil
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")
DEFAULT_ROOT = os.path.abspath(os.path.join(HERE, *[".."] * 8))
LIBRARY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
BUILD = LIBRARY + "/🏗️build"
GRAPH = BUILD + "/🕸️graph/🟦️.ts"
ADMISSION = BUILD + "/🕸️graph/🚪️admission/🟦️.ts"
SCHEMA = BUILD + "/🧬️schema/🚪️admission.json"
TESTS = BUILD + "/🕸️graph/🚪️admission/🧪️tests/🗃️closure-cache"
NEW = {
    "closure-cache.test.ts": TESTS + "/🟦️.ts",
    "closure-cache.fixture.json": TESTS + "/🔣️.json",
    "closure-cache.schema.json": TESTS + "/🧬️schema/🔣️.json",
    "closure-cache.oracle.py": TESTS + "/🐍️oracle.py",
    "closure-cache.step.ts": TESTS + "/🏃️step/🟦️.ts",
}
FULL = [LIBRARY, "🧰️framework/🔨️modules/📚️compiler", "🧰️framework/🔨️modules/🏃️process", "🧰️framework/🔨️modules/📁️filesystem", "🧰️framework/🔨️modules/🧬️schema"]


class Refusal(Exception):
    pass


def read(path):
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def staged(name):
    return read(os.path.join(STAGED, name))


def replace(label, old, new):
    return ("replace", label, old, new)


def edits():
    validator = "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts"
    admission = [
        replace("admission imports the compiled validator", 'import {validateJsonSchemaSubset} from "%s";' % validator, 'import {compileJsonSchemaSubset,validateJsonSchemaSubset} from "%s";' % validator),
        replace("admission imports join", 'import {createHash} from "node:crypto";', 'import {createHash} from "node:crypto";\nimport {join} from "node:path";'),
        replace(
            "admission names the requester and annotates failures",
            ' const observe=async(owner:BuildOwnerRegistryV1["owners"][number],targets:readonly string[],inputs:readonly string[])=>{const value=await inspect(owner,targets,inputs);if(validateJsonSchemaSubset(',
            ' const requestedBy=new Map<string,BuildOwnerAdmissionOriginV1>();let origin:BuildOwnerAdmissionOriginV1=null;\n const observe=async(owner:BuildOwnerRegistryV1["owners"][number],targets:readonly string[],inputs:readonly string[])=>{let value:BuildOwnerAdmissionFactsV1;try{value=await inspect(owner,targets,inputs);}catch(error){throw annotateBuildOwnerAdmissionFailureV1(error,owner.name,requestedBy);}if(validateJsonSchemaSubset(',
        ),
        replace(
            "admission records the first requester",
            ' const admit=(name:string)=>{if(!byName.has(name))throw Error("Unknown canonical build owner: "+name);admitted.add(name);},task=',
            ' const admit=(name:string)=>{if(!byName.has(name))throw Error("Unknown canonical build owner: "+name);if(!admitted.has(name))requestedBy.set(name,origin);admitted.add(name);},task=',
        ),
        replace("admission origin owner", "   const owner=byName.get(name)!;if(!facts.has(name)){const base=await observe(owner,[],[]);", "   origin={owner:name};const owner=byName.get(name)!;if(!facts.has(name)){const base=await observe(owner,[],[]);"),
        replace("admission origin task", "   for(const target of active){const operation=value.project.targets[target];", "   for(const target of active){origin={owner:name,target};const operation=value.project.targets[target];"),
        replace("admission origin catalog", '  for(const provider of registry.catalogProviders)if(selection.kind==="all"||', '  origin="catalog";for(const provider of registry.catalogProviders)if(selection.kind==="all"||'),
        ("append", "admission cache vocabulary", None, staged("admission.append.ts"), "export function buildOwnerAdmissionRecordV1("),
    ]
    schema = [("insert", "schema cache defs", '"$defs": {\n    "sourceInputObservation": {', staged("admission.defs.txt"), '"admissionCache": {', '"$defs": {\n')]
    graph = [
        replace(
            "graph imports the cache vocabulary",
            'import {readBuildOwnerRegistryV1,readBuildGraphSelectionV1,closeBuildOwnerAdmissionV1,buildGraphScopeKeyV1,type BuildGraphSelectionV1,type BuildOwnerRegistryV1,type BuildOwnerAdmissionV1} from "./🚪️admission/🟦️.ts";',
            'import {readBuildOwnerRegistryV1,readBuildGraphSelectionV1,closeBuildOwnerAdmissionV1,buildGraphScopeKeyV1,observeBuildOwnerAdmissionV1,buildAdmissionReceiptPathV1,buildAdmissionReceiptPublishableV1,createBuildAdmissionReceiptV1,readBuildAdmissionReceiptV1,type BuildOwnerAdmissionPortsV1,type BuildGraphSelectionV1,type BuildOwnerRegistryV1,type BuildOwnerAdmissionV1} from "./🚪️admission/🟦️.ts";',
        ),
        replace(
            "graph imports the writer directory",
            'import {withRepositoryBuildGraphConsumerV1,withRepositoryBuildGraphFactStoreV1,withRepositoryBuildGraphSelectionV1,buildGraphWriterResourceV1} from "../📥️inputs/💾️storage/🟨️.mjs";',
            'import {withRepositoryBuildGraphConsumerV1,withRepositoryBuildGraphFactStoreV1,withRepositoryBuildGraphSelectionV1,buildGraphWriterResourceV1,repositoryBuildGraphWriterDirectoryV1} from "../📥️inputs/💾️storage/🟨️.mjs";',
        ),
        replace(
            "graph inventory reuses the admission closure",
            "  const inventory=async()=>{let started=performance.now();const captured=selected?await observation.captureObservedSourceInputsV1(()=>observation.settleObservedComputationV1(async()=>{",
            staged("graph.inventory.head.ts").rstrip("\n"),
        ),
        replace(
            "graph closure observes each owner through its persisted record",
            "return closeBuildOwnerAdmissionV1(options.selection,registry!,async(owner,targets,inputs)=>{if(invocation)checkScriptInvocation(invocation);return producer.discoverBuildOwnerAdmissionFactsV1(owner,targets,registry,{workspaceRoot:root,invocation,phaseControl,selection:options.selection,nativeCache,nativeClosures},inputs);});},{signal:invocation?.control.signal})):undefined,candidate=captured?.value;",
            staged("graph.port.txt").rstrip("\n"),
        ),
        replace(
            "graph remembers the closure it just derived",
            'timing("workspace-membership-hash",started);return {files,digest,admission,observations:captured?.observations??[]};};',
            'timing("workspace-membership-hash",started);if(admission&&captured&&buildAdmissionReceiptPublishableV1(captured.observations,startedAt))admissionMemo={admission,observations:captured.observations,startedAt};return {files,digest,admission,observations:captured?.observations??[],startedAt,persisted:false};};',
        ),
        replace(
            "graph persists the closure together with the graph it admitted",
            "await rename(receipt,authorityPath);}finally{await Promise.all([rm(temporary,{force:true}),rm(receipt,{force:true})]);}\n  return graph;",
            "await rename(receipt,authorityPath);}finally{await Promise.all([rm(temporary,{force:true}),rm(receipt,{force:true})]);}\n  if(selected&&initial.admission&&!initial.persisted)await publishAdmission(initial.admission,initial.observations,initial.startedAt);\n  return graph;",
        ),
        replace(
            "graph staging sweep removes unfinished admission receipts",
            '"(?:\\\\.authority)?\\\\.json\\\\.[a-f0-9-]{36}$"',
            '"(?:\\\\.authority|\\\\.admission)?\\\\.json\\\\.[a-f0-9-]{36}$"',
        ),
    ]
    return [(ADMISSION, admission), (SCHEMA, schema), (GRAPH, graph)]


def patch(text, edit):
    kind, label = edit[0], edit[1]
    if kind == "replace":
        old, new = edit[2], edit[3]
        if new in text:
            return text, "present"
        if text.count(old) != 1:
            raise Refusal("anchor of '%s' occurs %d times (needs exactly 1)" % (label, text.count(old)))
        return text.replace(old, new), "replace"
    anchor, new, marker = edit[2], edit[3], edit[4]
    if marker in text:
        return text, "present"
    if kind == "append":
        return text + new, "append"
    if text.count(anchor) != 1:
        raise Refusal("anchor of '%s' occurs %d times (needs exactly 1)" % (label, text.count(anchor)))
    return text.replace(anchor, edit[5] + new + anchor[len(edit[5]):]), "insert"


def plan(root):
    rows, writes = [], {}
    for relative, items in edits():
        path = os.path.join(root, relative)
        if not os.path.isfile(path):
            raise Refusal("target missing: " + relative)
        text = read(path)
        for edit in items:
            text, how = patch(text, edit)
            rows.append((relative.split("/")[-2] + "/" + relative.split("/")[-1], edit[1], how))
        writes[path] = text
    for key, relative in NEW.items():
        target = os.path.join(root, relative)
        data = staged(key)
        rows.append((relative.split("/")[-2], key, "present" if os.path.exists(target) and read(target) == data else "write"))
        writes[target] = data
    return rows, writes


def write_all(writes):
    for path, text in writes.items():
        if os.path.exists(path) and read(path) == text:
            continue
        os.makedirs(os.path.dirname(path), exist_ok=True)
        temporary = path + ".x-graph.tmp"
        with open(temporary, "w", encoding="utf-8") as handle:
            handle.write(text)
        os.replace(temporary, path)


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
                if os.path.isdir(os.path.join(root, child)):
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


def main():
    args = sys.argv[1:]
    root = os.path.abspath(args[args.index("--root") + 1]) if "--root" in args else DEFAULT_ROOT
    if not args or args[0] not in ("--check", "--live", "mirror"):
        raise SystemExit(__doc__)
    if args[0] == "mirror":
        destination = os.path.abspath(args[1])
        mirror(destination, root)
        if "--bare" not in args:
            rows, writes = plan(destination)
            write_all(writes)
        print("mirror", destination)
        return 0
    try:
        rows, writes = plan(root)
    except Refusal as refusal:
        print("REFUSED, nothing written: %s" % refusal)
        return 2
    for row in rows:
        print("%-34s %-62s %s" % row)
    pending = [row for row in rows if row[2] != "present"]
    print("%d operations, %d pending" % (len(rows), len(pending)))
    if args[0] == "--check":
        return 1 if pending else 0
    write_all(writes)
    print("applied")
    return 0


sys.exit(main())
