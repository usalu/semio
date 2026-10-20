#!/usr/bin/env python3
"""Two-phase anchored apply of the x-ingress follow-up (see T/handoff-ingress-followup.md).

usage: apply.py --check | --live | mirror DIR   [--root ROOT]
Phase 1 reads every target, verifies the x-ingress base set is present and that every anchor occurs exactly once (or the edit is already present), and computes all new texts in memory.
Any refusal exits 2 with nothing written. --check stops after phase 1 (exit 1 when edits are pending, 0 when everything is applied). --live then writes each file through a temporary file and os.replace.
"""
import hashlib
import os
import shutil
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")
DEFAULT_ROOT = os.path.abspath(os.path.join(HERE, *[".."] * 9))
LIBRARY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
GENERATION = LIBRARY + "/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation"
CORE = GENERATION + "/🟦️.ts"
GLUE = GENERATION + "/💾️storage/🟦️.ts"
SCHEMA = GENERATION + "/🧬️schema/🔣️.json"
BOOT = LIBRARY + "/🏗️build/🚀️bootstrap/🟦️.ts"
TESTS = GENERATION + "/🧪️tests/🛡️followup"
NEW = {
    "followup.test.ts": TESTS + "/🟦️.ts",
    "followup.fixture.json": TESTS + "/🔣️.json",
    "followup.schema.json": TESTS + "/🧬️schema/🔣️.json",
    "followup.oracle.py": TESTS + "/🐍️oracle.py",
}
FULL = [LIBRARY, "🧰️framework/🔨️modules/📚️compiler", "🧰️framework/🔨️modules/🏃️process", "🧰️framework/🔨️modules/📁️filesystem", "🧰️framework/🔨️modules/🧬️schema"]
BASE = {
    CORE: "export async function confirmGenerationWalkV1",
    GLUE: "export async function writeJavascriptDependencyGenerationFilesV1(",
    BOOT: "const confirmWalk=()=>generation.confirmGenerationWalkV1(",
}


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
    core = [("append", "core guard and origin", None, staged("core.append.ts"), "export async function guardGenerationReplayV1")]
    glue = [
        replace("glue import core names", "parseGenerationReceiptV1,publishGenerationBestEffortV1,", "guardGenerationReplayV1,generationWalkOriginV1,parseGenerationReceiptV1,publishGenerationBestEffortV1,"),
        replace("glue replay becomes the unguarded inner function", "export async function replayJavascriptDependencyGenerationWalkV1(", "async function replayJavascriptDependencyGenerationWalkUnguardedV1("),
        ("insert", "glue guarded replay", "/** 🤝️ Publishes one receipt under the caller's SHARED", staged("glue.guard.ts"), "export function replayJavascriptDependencyGenerationWalkV1("),
        ("span", "glue files writer removes its unfinished leaf through a fresh descriptor scope", ("export async function writeJavascriptDependencyGenerationFilesV1(", "/** 🧫️ Replays a completed walk's"), staged("glue.files.ts"), "async function removeUnfinishedGenerationLeafV1("),
        replace("glue re-export", "export {confirmGenerationWalkV1,publishGenerationBestEffortV1};", "export {confirmGenerationWalkV1,generationWalkOriginV1,publishGenerationBestEffortV1};"),
    ]
    boot = [
        replace("bootstrap walk origin and confirm", "    const confirmWalk=()=>generation.confirmGenerationWalkV1({replay:()=>generation.replayJavascriptDependencyGenerationWalkV1(workspace,context,actualControl,check,walk!,walkStartedAt),walk:()=>currentOwner()});",
                "    const walkOriginAt=generation.generationWalkOriginV1(cached?.startedAt,walkStartedAt);\n    const confirmWalk=()=>generation.confirmGenerationWalkV1({replay:()=>generation.replayJavascriptDependencyGenerationWalkV1(workspace,context,actualControl,check,walk!,walkOriginAt),walk:()=>currentOwner()});"),
        replace("bootstrap tail publish", "if(walk&&!cached)await generation.publishJavascriptDependencyGenerationReceiptV1(workspace,store,actualControl,check,walk,walkStartedAt);return \"ready\";",
                "if(walk&&!cached)await generation.publishJavascriptDependencyGenerationReceiptV1(workspace,store,actualControl,check,walk,walkOriginAt);return \"ready\";"),
    ]
    schema = [replace("evidence cap equals the relative-create file cap", '"maximumEvidenceBytes": 134217728,', '"maximumEvidenceBytes": 67108864,')]
    return [(CORE, core), (GLUE, glue), (BOOT, boot), (SCHEMA, schema)]


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
    if kind == "span":
        start, end = anchor
        if text.count(start) != 1 or text.count(end) != 1 or text.index(start) > text.index(end):
            raise Refusal("span anchors of '%s' are not unique and ordered" % label)
        return text[: text.index(start)] + new + "\n" + text[text.index(end):], "span"
    if kind == "append":
        return text + new, "append"
    if text.count(anchor) != 1:
        raise Refusal("anchor of '%s' occurs %d times (needs exactly 1)" % (label, text.count(anchor)))
    return text.replace(anchor, new + anchor), "insert"


def plan(root):
    rows, writes = [], {}
    for relative, items in edits():
        path = os.path.join(root, relative)
        if not os.path.isfile(path):
            raise Refusal("target missing: " + relative)
        text = read(path)
        if relative in BASE and BASE[relative] not in text:
            raise Refusal("x-ingress base set is not applied in " + relative)
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
        temporary = path + ".x-ingress-followup.tmp"
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
        print("%-34s %-52s %s" % row)
    pending = [row for row in rows if row[2] != "present"]
    print("%d operations, %d pending" % (len(rows), len(pending)))
    if args[0] == "--check":
        return 1 if pending else 0
    write_all(writes)
    print("applied")
    return 0


sys.exit(main())
