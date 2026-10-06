#!/usr/bin/env python3
"""⏪️ S5-STORE: the exact inverse of wave AA + the remote-merge renumber fix, hunk by hunk (never a whole-file copy, so a
peer's later edit of the same file stays). Every landed hunk must occur exactly once or nothing is written.

    python3 🧪️s5-store-restore-aa.py [--check]
"""
import importlib.util
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent


def load(name):
    """📦️ A wave script as a module, without running its `main`."""
    source = (TICKET / name).read_text(encoding="utf-8")
    module = type(sys)("wave")
    module.__file__ = str(TICKET / name)
    saved, sys.argv = sys.argv, [name, "--check"]
    try:
        exec(compile(source.rsplit("\nmain()\n", 1)[0], name, "exec"), module.__dict__)
    finally:
        sys.argv = saved
    return module


def main():
    absent = load("🧪️s5-store-archive-absent.py")
    renumber = load("🧪️s5-store-ingest-renumber-dirt.py")
    texts, pending = {}, []

    def text_of(path):
        if path not in texts:
            texts[path] = path.read_text(encoding="utf-8")
        return texts[path]

    def reverse(path, landed, original, name):
        text = text_of(path)
        if landed not in text:
            return
        if text.count(landed) != 1:
            raise SystemExit(f"{path.parent.name}: landed hunk `{name}` occurs {text.count(landed)} times (expected 1)")
        texts[path] = text.replace(landed, original)
        pending.append(f"{path.parent.name}:{name}")

    reverse(absent.UNIT_TEST, absent.RUST_LAW, "", "law")
    for path in absent.NAME_MAPS:
        text = text_of(path)
        arm = 'ArtifactEvent::DocumentArchiveAbsent => "documentArchiveAbsent",'
        lines = [line for line in text.split("\n") if line.strip() != arm]
        if len(lines) != len(text.split("\n")):
            texts[path] = "\n".join(lines)
            pending.append(f"{path.parent.name}:name-map")
    for path, edits in absent.EDITS.items():
        for name, old, new, _marker in reversed(edits):
            reverse(path, new, old, name)
    reverse(renumber.STORE, renumber.NEW, renumber.OLD, "renumber")
    removed = [absent.STORE / relative for relative in absent.NEW_FILES if (absent.STORE / relative).exists()]
    pending += [f"remove:{path.parent.name}" for path in removed]
    print("pending: " + (", ".join(pending) if pending else "none"))
    if "--check" in sys.argv[1:] or not pending:
        return
    for path, text in texts.items():
        if text != path.read_text(encoding="utf-8"):
            path.write_text(text, encoding="utf-8")
    for path in removed:
        path.unlink()
        path.parent.rmdir()
    print("restored")


main()
