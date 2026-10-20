#!/usr/bin/env python3
"""Idempotent anchored apply of the x-ingress set (see T/handoff-ingress.md).

usage: apply.py check|apply|mirror DIR [--root ROOT] [--dry-run]
Every anchor must occur exactly once in an unpatched file; a patched file is recognised by its marker and left alone.
`mirror DIR` builds a scratch tree (real files along the patched paths, symlinks elsewhere) and applies the set there.
"""
import os
import shutil
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")
DEFAULT_ROOT = os.path.abspath(os.path.join(HERE, *[".."] * 8))
LIBRARY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
GENERATION = LIBRARY + "/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation"
CORE = GENERATION + "/🟦️.ts"
GLUE = GENERATION + "/💾️storage/🟦️.ts"
BOOT = LIBRARY + "/🏗️build/🚀️bootstrap/🟦️.ts"
TESTS = GENERATION + "/🧪️tests/🚪️ingress"
FULL = [LIBRARY, "🧰️framework/🔨️modules/📚️compiler", "🧰️framework/🔨️modules/🏃️process", "🧰️framework/🔨️modules/📁️filesystem", "🧰️framework/🔨️modules/🧬️schema"]
NEW = {
    "ingress.test.ts": TESTS + "/🟦️.ts",
    "ingress.fixture.json": TESTS + "/🔣️.json",
    "ingress.schema.json": TESTS + "/🧬️schema/🔣️.json",
    "ingress.oracle.py": TESTS + "/🐍️oracle.py",
}


def read(path):
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def staged(name):
    return read(os.path.join(STAGED, name))


class Edit:
    def __init__(self, label, old, new, marker=None, before=None, span=None):
        self.label, self.old, self.new, self.marker, self.before, self.span = label, old, new, marker or new, before, span


def edits():
    core = [Edit("core append", None, staged("core.append.ts"), marker="export async function confirmGenerationWalkV1")]
    glue = [
        Edit("glue import lease", 'import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";',
             'import {acquireResourceLease,checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";'),
        Edit("glue import lease directory", 'import type {RepositoryCacheStoreV1} from "../../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";',
             'import type {RepositoryCacheStoreV1} from "../../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";\nimport {repositoryLeaseDirectoryV1} from "../../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";'),
        Edit("glue import core names", "import {captureGenerationReceiptV1,generationLocationV1,", "import {captureGenerationReceiptV1,confirmGenerationWalkV1,generationLocationV1,"),
        Edit("glue import core more", "parseGenerationReceiptV1,verifyGenerationReceiptV1,type GenerationControlV1,",
             "parseGenerationReceiptV1,publishGenerationBestEffortV1,runGenerationWriterV1,verifyGenerationReceiptV1,type GenerationControlV1,type GenerationPublishOutcomeV1,type GenerationReplayResultV1,"),
        Edit("glue capture accepts an implementation", "invocation:ScriptInvocation,subject:JavascriptDependencyGenerationSubjectV1=javascriptDependencyGenerationSubjectV1()){",
             "invocation:ScriptInvocation,subject:JavascriptDependencyGenerationSubjectV1=javascriptDependencyGenerationSubjectV1(),implementation?:Awaited<ReturnType<typeof captureJavascriptDependencyGenerationImplementationV1>>){"),
        Edit("glue capture uses it", "implementation:await captureJavascriptDependencyGenerationImplementationV1(invocation,subject)", "implementation:implementation??await captureJavascriptDependencyGenerationImplementationV1(invocation,subject)"),
        Edit("glue block", None, staged("glue.insert.ts"), marker="export async function writeJavascriptDependencyGenerationFilesV1", before="/** 📮️ Publishes one receipt"),
        Edit("glue publish signature", "subject:JavascriptDependencyGenerationSubjectV1=javascriptDependencyGenerationSubjectV1()):Promise<\"published\"|\"present\"|\"deferred\"|\"stale\"|\"failed\"> {",
             "subject:JavascriptDependencyGenerationSubjectV1=javascriptDependencyGenerationSubjectV1(),graceMilliseconds:number=0):Promise<\"published\"|\"present\"|\"deferred\"|\"stale\"|\"failed\"> {"),
        Edit("glue publish call", "try{return await writeJavascriptDependencyGenerationReceiptV1(workspace,store,control,checkpoint,walk,startedAtMs,observed,subject);}",
             "try{return await writeJavascriptDependencyGenerationReceiptV1(workspace,store,control,checkpoint,walk,startedAtMs,observed,subject,graceMilliseconds);}"),
        Edit("glue write signature", "observed:BunWorkspaceObservedOwnershipV1|undefined,subject:JavascriptDependencyGenerationSubjectV1):Promise<\"published\"|\"present\"|\"deferred\"|\"stale\"> {",
             "observed:BunWorkspaceObservedOwnershipV1|undefined,subject:JavascriptDependencyGenerationSubjectV1,graceMilliseconds:number):Promise<\"published\"|\"present\"|\"deferred\"|\"stale\"> {"),
        Edit("glue write files", None,
             " await writeJavascriptDependencyGenerationFilesV1(store.root,all,checkpoint,()=>control.yieldContinuation());\n retireJavascriptDependencyGenerationFilesV1(store.root,scope,graceMilliseconds);\n",
             marker="await writeJavascriptDependencyGenerationFilesV1(store.root,all,checkpoint", span=(" await withArtifactRelativeDirectoryV1(store.root,checkpoint,async base=>{", " return \"published\";")),
        Edit("glue retire signature", "export function retireJavascriptDependencyGenerationFilesV1(storeRoot:string,scope:string):void{\n const directory=join(storeRoot,protocol.directory);",
             "export function retireJavascriptDependencyGenerationFilesV1(storeRoot:string,scope:string,graceMilliseconds:number=0):void{\n const directory=join(storeRoot,protocol.directory),aged=(name:string)=>graceMilliseconds<=0||Date.now()-lstatSync(join(directory,name)).mtimeMs>=graceMilliseconds;"),
        Edit("glue retire receipts", "if(name.startsWith(prefix)&&name!==newest){unlinkSync(join(directory,name));continue;}", "if(name.startsWith(prefix)&&name!==newest&&aged(name)){unlinkSync(join(directory,name));continue;}"),
        Edit("glue retire evidence", "for(const name of names)if(evidenceName.test(name)&&!keep.has(name))unlinkSync(join(directory,name));", "for(const name of names)if(evidenceName.test(name)&&!keep.has(name)&&aged(name))unlinkSync(join(directory,name));"),
    ]
    boot = [
        Edit("bootstrap confirmWalk", 'return "material-required";}\n    const witness=():BuildParserReceiptV1=>{',
             'return "material-required";}\n    const confirmWalk=()=>generation.confirmGenerationWalkV1({replay:()=>generation.replayJavascriptDependencyGenerationWalkV1(workspace,context,actualControl,check,walk!,walkStartedAt),walk:()=>currentOwner()});\n    const witness=():BuildParserReceiptV1=>{'),
        Edit("bootstrap adoption falls back to a replay", "if(!adoptedCandidateV1(sealed)){cached=undefined;walk=await currentOwner();owner=walk;current=witness();}}",
             "if(!adoptedCandidateV1(sealed)){cached=undefined;const again=await confirmWalk();if(again.via===\"walk\"){walk=again.walk;owner=walk;current=witness();}}}"),
        Edit("bootstrap shared pass decides readiness", 'if(!publishReceipt&&walk){candidate={walk,startedAt:walkStartedAt};return "receipt-required";}await store.recheck();check();const confirmed=walk&&!cached?await currentOwner():await generation.verify',
             'if(!publishReceipt&&walk)candidate={walk,startedAt:walkStartedAt};await store.recheck();check();const confirmed=walk&&!cached?await confirmWalk():await generation.verify'),
        Edit("bootstrap best effort publish", None,
             '    if(!publishReceipt&&walk&&lstatSync(path,{throwIfNoEntry:false})&&buildParserReceiptMatchesV1(JSON.parse(bounded(path,bootstrap.maximumReceiptBytes).toString()),current))await generation.publishGenerationBestEffortV1({attempt:()=>generation.publishJavascriptDependencyGenerationReceiptSharedV1(workspace,store,actualControl,check,walk!,walkStartedAt),aborted:()=>actualControl.signal.aborted,report:generation.reportJavascriptDependencyGenerationPublishV1(walk.evidence.scope)});\n',
             marker="generation.publishGenerationBestEffortV1(", before='    if(!publishReceipt){if(!lstatSync(path,{throwIfNoEntry:false}))return "receipt-required";'),
        Edit("bootstrap tail confirms by replay", "const settled=walk&&!cached?await currentOwner():await generation.verify", "const settled=walk&&!cached?await confirmWalk():await generation.verify"),
    ]
    return [(CORE, core), (GLUE, glue), (BOOT, boot)]


def patch_text(text, edit):
    before, span = edit.before, edit.span
    if edit.marker in text and (edit.old is None or edit.old not in text or edit.new in text):
        return text, "present"
    if edit.old is None and before is None and span is None:
        return text + edit.new, "append"
    if before is not None:
        if text.count(before) != 1:
            raise SystemExit("anchor '%s' must occur once, found %d (%s)" % (before[:60], text.count(before), edit.label))
        return text.replace(before, edit.new + before), "insert"
    if span is not None:
        start, end = span
        if text.count(start) != 1 or text.count(end) < 1:
            raise SystemExit("span anchors of '%s' not unique" % edit.label)
        i = text.index(start)
        j = text.index(end, i)
        return text[:i] + edit.new + text[j:], "span"
    if text.count(edit.old) != 1:
        raise SystemExit("anchor of '%s' must occur once, found %d" % (edit.label, text.count(edit.old)))
    return text.replace(edit.old, edit.new), "replace"


def run(root, dry):
    results = []
    for relative, items in edits():
        path = os.path.join(root, relative)
        text = read(path)
        for edit in items:
            text, how = patch_text(text, edit)
            results.append((os.path.basename(os.path.dirname(relative)) + "/" + os.path.basename(relative), edit.label, how))
        if not dry:
            temporary = path + ".x-ingress.tmp"
            with open(temporary, "w", encoding="utf-8") as handle:
                handle.write(text)
            os.replace(temporary, path)
    for key, relative in NEW.items():
        target = os.path.join(root, relative)
        data = staged(key)
        state = "present" if os.path.exists(target) and read(target) == data else "write"
        results.append((relative.split("/")[-2], key, state))
        if state == "write" and not dry:
            os.makedirs(os.path.dirname(target), exist_ok=True)
            with open(target, "w", encoding="utf-8") as handle:
                handle.write(data)
    for row in results:
        print("%-22s %-48s %s" % row)
    pending = [row for row in results if row[2] not in ("present",)]
    print("%d operations, %d pending%s" % (len(results), len(pending), " (dry run)" if dry else ""))
    return len(pending)


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
        parent = os.path.dirname(place)
        probe = destination
        for part in os.path.relpath(parent, destination).split(os.sep):
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
    for relative in NEW.values():
        os.makedirs(os.path.dirname(os.path.join(destination, relative)), exist_ok=True)
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
        sys.exit(1 if run(root, True) else 0)
    if args[0] == "apply":
        run(root, "--dry-run" in args)
        return
    destination = os.path.abspath(args[1])
    mirror(destination, root)
    if "--bare" not in args:
        run(destination, False)
    print("mirror", destination)


main()
