#!/usr/bin/env python3
"""🧪️ Ticket tool: judges a surveyed corpus with variants of the gesture module, to show what every threshold is for.

Run from the repository root, after a survey of ``generate_gesture_vectors.py`` left its corpus in the scratch folder:

``.venv/Scripts/python.exe <this file> <corpus.json> <name>:<CONSTANT>=<value>[,<CONSTANT>=<value>…] …``

Per variant it copies ``🔨️modules/👆️gesture/🟦️.ts`` into ``🗑️generated/a6/variants/<name>/`` with the named exported
constants replaced (every one must be found exactly once), copies the case adapter and the judge beside it with their
imports pointed at the copy, lets ``bun`` judge the corpus and prints the tally of ``generate_gesture_vectors.py``:
per kind of ordinary travel the cues it set off, per group of deliberate traces how many were recognised. The
variant ``as-is:`` (no replacement) is the module as it stands. Nothing in the repository is touched.
"""

import importlib.util
import json
import os
import re
import subprocess
import sys

sys.dont_write_bytecode = True
TICKET = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(TICKET, "..", "..", "..", "..", "..", "..", ".."))
PETS = os.path.join(ROOT, "🧰️framework", "🛍️products", "🐾️pets")
MODULE = os.path.join(PETS, "🔨️modules", "👆️gesture", "🟦️.ts")
ADAPTER = os.path.join(PETS, "🧪️tests", "👆️gesture-recognition", "🟦️.ts")
JUDGE = os.path.join(TICKET, "judge_gesture_traces.ts")
SCRATCH = os.path.join(TICKET, "🗑️generated", "a6", "variants")


def load(path, name):
    """📦️ Loads one Python file as a module."""
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def slashed(path):
    """➗️ A path as an import specifier."""
    return path.replace(os.sep, "/")


def repointed(source, targets):
    """🔀️ A source with every import whose specifier ends in one of some suffixes pointed at another file."""
    for suffix, target in targets.items():
        source, count = re.subn(r'from "[^"]*' + re.escape(suffix) + '"', lambda _match: 'from "%s"' % slashed(target), source)
        if count == 0:
            raise AssertionError("no import ends in %s" % suffix)
    return source


def variant(name, changes):
    """🧬️ The judge of one variant: the module with its constants replaced, the adapter and the judge beside it."""
    folder = os.path.join(SCRATCH, name)
    os.makedirs(folder, exist_ok=True)
    with open(MODULE, encoding="utf-8") as handle:
        module = handle.read()
    for constant, value in changes.items():
        module, count = re.subn(r"(export const %s = )[^;]+;" % re.escape(constant), lambda match: match.group(1) + value + ";", module)
        if count != 1:
            raise AssertionError("%s: the constant %s was found %d times" % (name, constant, count))
    module = repointed(module, {"🧬️schema/🟦️.ts": os.path.join(PETS, "🧬️schema", "🟦️.ts")})
    with open(ADAPTER, encoding="utf-8") as handle:
        adapter = repointed(handle.read(), {"🔌️adapter/🟦️.ts": os.path.join(ROOT, "🧰️framework", "🔨️modules", "🧪️test", "🔌️adapter", "🟦️.ts"), "👆️gesture/🟦️.ts": os.path.join(folder, "gesture.ts")})
    with open(JUDGE, encoding="utf-8") as handle:
        judge = repointed(handle.read(), {"👆️gesture-recognition/🟦️.ts": os.path.join(folder, "adapter.ts"), "👆️gesture/🟦️.ts": os.path.join(folder, "gesture.ts")})
    for filename, source in (("gesture.ts", module), ("adapter.ts", adapter), ("judge.ts", judge)):
        with open(os.path.join(folder, filename), "w", encoding="utf-8", newline="\n") as handle:
            handle.write(source)
    return os.path.join(folder, "judge.ts"), os.path.join(folder, "verdicts.json")


if __name__ == "__main__":
    generator = load(os.path.join(TICKET, "generate_gesture_vectors.py"), "generate_gesture_vectors")
    with open(sys.argv[1], encoding="utf-8") as handle:
        corpus = json.load(handle)
    for argument in sys.argv[2:]:
        name, _, listed = argument.partition(":")
        changes = dict(change.split("=") for change in listed.split(",") if change)
        judge, outputs = variant(name, changes)
        subprocess.run(["bun", judge, sys.argv[1], outputs], check=True, cwd=ROOT)
        with open(outputs, encoding="utf-8") as handle:
            report = generator.tallied(corpus, json.load(handle))
        ordinary = ", ".join("%s %s" % (kind, json.dumps(entry["cues"]) if entry["cues"] else "0") for kind, entry in report["ordinary"].items())
        deliberate = ", ".join("%s %d/%d%s" % (group, entry["detected"], entry["traces"], " (%d wrong)" % entry["wrong"] if entry["wrong"] else "") for group, entry in report["deliberate"].items())
        sys.stdout.buffer.write(("%s %s\n  ordinary: %s\n  deliberate: %s\n" % (name, json.dumps(changes), ordinary, deliberate)).encode("utf-8"))
