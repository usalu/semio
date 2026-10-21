#!/usr/bin/env python3
"""Applies the polyglot compilation-unit module. Dry run by default.
group 1: creates `L/🏗️build/🧩️units/**` from `staged/🧩️units/**` (create-only; an existing different file refuses the whole set).
group 2: one anchored edit of the live TypeScript units engine: its private Tarjan implementation becomes a thin wrapper over the shared one (the wrapper keeps the UTF-16 member order the TypeScript plans were built with).
  --apply     write (temp + rename, all or nothing, rollback on error; the edited engine is backed up under C/🗑️generated/polyglot-units/)
  --undo      remove created files that still equal the staged bytes and restore the engine if it still equals the edited bytes
  group 3 (default): a dependent TypeScript unit keys the declaration CLOSURE of its dependencies instead of their own declaration digest (sound when a re-exported type changes two units away); --no-ts-closure skips it
  group 4 (default): wires `repo units`: the operation in the catalogue and its schema, the router branch and the regenerated help snapshot (wiring.py, wiring/help-fixture.json); --no-wiring skips it
  --no-dedupe skip group 2
  --root DIR  operate on a mirror of the repository (verification overlay)"""
import hashlib, os, shutil, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import wiring

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged", "🧩️units")
ROOT = os.path.abspath(os.path.join(HERE, *[".."] * 8))
if "--root" in sys.argv:
    ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1])
LIB = os.path.join(ROOT, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📚️library")
TARGET = os.path.join(LIB, "🏗️build", "🧩️units")
ENGINE = os.path.join(LIB, "🏗️build", "🟦️typescript", "🧩️units", "🟦️.ts")
BACKUP = os.path.normpath(os.path.join(HERE, "..", "🗑️generated", "polyglot-units"))
START = "/** 🔁️ Tarjan strongly connected components of a graph given as adjacency sets, iterative so deep import chains cannot overflow the stack. */"
IMPORT_ANCHOR = 'import schema from "./🧬️schema/🔣️.json" with {type: "json"};\n'
IMPORT_NEW = IMPORT_ANCHOR + 'import {stronglyConnectedV1 as sharedStronglyConnectedV1} from "../../🧩️units/🕸️graph/🟦️.ts";\n'
CLOSURE_OLD = "    return {key, declarations: receipt.declarations, view: viewPath};\n"
CLOSURE_NEW = "    return {key, declarations: sha(\"closure\", receipt.declarations, ...dependencies.map(row => row.declarations)), view: viewPath};\n"
WRAPPER = ("/** 🔁️ Tarjan strongly connected components: the one implementation lives in the neutral unit graph (`🏗️build/🧩️units/🕸️graph`); this wrapper keeps the UTF-16 member order the TypeScript plans were built with. */\n"
           "export const stronglyConnectedV1 = (nodes: readonly string[], adjacent: ReadonlyMap<string, ReadonlySet<string>>): string[][] => sharedStronglyConnectedV1(nodes, adjacent, (a, b) => a < b ? -1 : a > b ? 1 : 0);\n")


def sha(data):
    return hashlib.sha256(data).hexdigest()


def staged_files():
    for base, _, names in os.walk(STAGED):
        for name in sorted(names):
            yield os.path.relpath(os.path.join(base, name), STAGED)


def edited_engine(text, dedupe=True, closure=False):
    """Returns the edited engine text (idempotent), or raises SystemExit on drift."""
    if dedupe and WRAPPER not in text:
        if text.count(START) != 1 or text.count(IMPORT_ANCHOR) != 1:
            raise SystemExit("DRIFT: the Tarjan anchor of the TypeScript engine is not unique")
        begin = text.index(START)
        end = text.index("\n}\n", begin) + 3
        if "return out;" not in text[begin:end]:
            raise SystemExit("DRIFT: the Tarjan function does not end where expected")
        text = (text[:begin] + WRAPPER + text[end:]).replace(IMPORT_ANCHOR, IMPORT_NEW, 1)
    if closure and CLOSURE_NEW not in text:
        if text.count(CLOSURE_OLD) != 1:
            raise SystemExit("DRIFT: the declaration return anchor of the TypeScript engine is not unique")
        text = text.replace(CLOSURE_OLD, CLOSURE_NEW, 1)
    return text


WIRING = [("🏗️build/⚙️operations/🔣️catalog.json", wiring.catalog_edit), ("🏗️build/⚙️operations/🧬️schema/🔣️.json", wiring.schema_edit), ("🏗️build/📜️script.ts", wiring.router_edit)]
HELP = "🏗️build/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json"


def plan(dedupe, closure, wire):
    ops, refused = [], []
    for rel in sorted(staged_files()):
        source, destination = os.path.join(STAGED, rel), os.path.join(TARGET, rel)
        data = open(source, "rb").read()
        if os.path.exists(destination):
            if open(destination, "rb").read() == data:
                ops.append(("same", destination, None))
            else:
                refused.append(destination)
        else:
            ops.append(("create", destination, data))
    if dedupe or closure:
        if not os.path.exists(ENGINE):
            refused.append(ENGINE)
        else:
            before = open(ENGINE, "rb").read()
            try:
                after = edited_engine(before.decode("utf8"), dedupe, closure).encode("utf8")
                ops.append(("same" if after == before else "edit", ENGINE, after))
            except SystemExit as error:
                print(error)
                refused.append(ENGINE)
    if wire:
        for rel, fn in WIRING:
            path = os.path.join(LIB, rel)
            if not os.path.exists(path):
                refused.append(path)
                continue
            before = open(path, "rb").read()
            try:
                after = fn(before.decode("utf8")).encode("utf8")
                ops.append(("same" if after == before else "edit", path, after))
            except SystemExit as error:
                print(error)
                refused.append(path)
    return ops, refused


def write_atomic(path, data):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    temporary = path + ".tmp"
    with open(temporary, "wb") as handle:
        handle.write(data)
    if os.path.exists(path):
        shutil.copymode(path, temporary)
    os.replace(temporary, path)


def main():
    args = sys.argv[1:]
    dedupe, closure, wire = "--no-dedupe" not in args, "--no-ts-closure" not in args, "--no-wiring" not in args
    if not os.path.isdir(LIB):
        print("library not found:", LIB)
        return 2
    if "--undo" in args:
        removed = 0
        for rel in sorted(staged_files(), reverse=True):
            destination = os.path.join(TARGET, rel)
            if os.path.exists(destination) and open(destination, "rb").read() == open(os.path.join(STAGED, rel), "rb").read():
                os.remove(destination)
                removed += 1
        for base, _, _ in sorted(os.walk(TARGET), reverse=True):
            if os.path.isdir(base) and not os.listdir(base):
                os.rmdir(base)
        if os.path.isdir(BACKUP):
            for name in sorted(os.listdir(BACKUP)):
                if not name.endswith(".path"):
                    continue
                target = open(os.path.join(BACKUP, name), encoding="utf8").read()
                original = open(os.path.join(BACKUP, name[:-5] + ".orig"), "rb").read()
                if not os.path.exists(target):
                    continue
                current = open(target, "rb").read()
                if current != original and (target == ENGINE and any(edited_engine(original.decode("utf8"), d, c).encode("utf8") == current for d in (True, False) for c in (True, False)) or target != ENGINE and any(fn(original.decode("utf8")).encode("utf8") == current for rel, fn in WIRING if target == os.path.join(LIB, rel)) or target == os.path.join(LIB, HELP) and current == open(os.path.join(HERE, "wiring", "help-fixture.json"), "rb").read()):
                    write_atomic(target, original)
                    print("restored", os.path.relpath(target, ROOT))
        print("undone", removed, "files")
        return 0
    ops, refused = plan(dedupe, closure, wire)
    for kind, path, _ in ops:
        print(kind, os.path.relpath(path, ROOT))
    for path in refused:
        print("REFUSED (exists and differs, or drift):", os.path.relpath(path, ROOT))
    print(sum(1 for o in ops if o[0] == "create"), "to create,", sum(1 for o in ops if o[0] == "edit"), "to edit,", sum(1 for o in ops if o[0] == "same"), "already applied,", len(refused), "refused")
    if refused:
        return 1
    if "--apply" not in args:
        print("dry run: pass --apply to write")
        return 0
    done = []
    try:
        for kind, path, data in ops:
            if kind == "create":
                write_atomic(path, data)
                done.append((path, None))
            elif kind == "edit":
                original = open(path, "rb").read()
                os.makedirs(BACKUP, exist_ok=True)
                write_atomic(os.path.join(BACKUP, sha(path.encode("utf8"))[:16] + ".orig"), original)
                open(os.path.join(BACKUP, sha(path.encode("utf8"))[:16] + ".path"), "w", encoding="utf8").write(path)
                write_atomic(path, data)
                done.append((path, original))
    except Exception as error:
        for path, original in reversed(done):
            if original is None:
                os.remove(path)
            else:
                write_atomic(path, original)
        print("rolled back:", error)
        return 1
    print("applied", len(done), "operations")
    return 0


if __name__ == "__main__":
    sys.exit(main())
