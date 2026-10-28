#!/usr/bin/env python3
"""📓️ docs5 doc batch: one anchored, idempotent edit list for one repository document (the build guide or the README), applied and reverted as a unit.

usage: python3 -I stage.py check|status|apply|revert [--root DIR] [--strict-base]

The batch directory holds this script and `edits.json` (the guide path, the sha256 of the guide at staging, and the edits).
Edit kinds:
  replace: `old` -> `new`; pending when `old` occurs once and `new` does not occur, applied when `new` occurs once.
  insert:  `block` placed on its own line(s) after (`where` "after") or before ("before") the unique line `anchor`;
           pending when the block is absent and the anchor is unique, applied when the block occurs once (anywhere), so
           two batches may insert at the same anchor in any order.
`prereq` on an edit names the code batch whose text it builds on: a missing `old` is then reported as PREREQ (exit 3) instead of DRIFT.
Exit codes: 0 clean (all pending, all applied, or partly applied and now fixed), 2 drift (nothing written), 3 prerequisite missing (nothing written),
4 internal check failed after the edit (nothing written). `--strict-base` also refuses (exit 2) when the guide differs from the staged sha256 and nothing of the batch is applied yet.
Never run this against the live guide except from the landing gate; verify on a mirror (`--root`)."""
import hashlib
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SPEC = json.load(open(os.path.join(HERE, "edits.json"), encoding="utf8"))


def find_root(argv):
    if "--root" in argv:
        return os.path.abspath(argv[argv.index("--root") + 1])
    here = HERE
    while True:
        if os.path.isfile(os.path.join(here, SPEC["guide"])) and os.path.isfile(os.path.join(here, "AGENTS.md")):
            return here
        parent = os.path.dirname(here)
        if parent == here:
            sys.exit("no checkout root found above %s; pass --root DIR" % HERE)
        here = parent


def read(path):
    with open(path, "rb") as handle:
        return handle.read().decode("utf8")


def at_edge(text, edit):
    start = text.index(edit["anchor"])
    end = start + len(edit["anchor"])
    return text[end:end + 1] == "\n" if edit["where"] == "after" else start == 0 or text[start - 1] == "\n"


def state(text, edit):
    if edit["kind"] == "replace":
        if text.count(edit["new"]) == 1:
            return "applied"
        if text.count(edit["old"]) == 1 and text.count(edit["new"]) == 0:
            return "pending"
        return "prereq" if edit.get("prereq") and text.count(edit["old"]) == 0 else "drift"
    block = edit["block"]
    if text.count(block) == 1:
        return "applied"
    if text.count(block) == 0 and text.count(edit["anchor"]) == 1 and at_edge(text, edit):
        return "pending"
    return "prereq" if edit.get("prereq") and text.count(edit["anchor"]) == 0 else "drift"


def do(text, edit):
    if edit["kind"] == "replace":
        return text.replace(edit["old"], edit["new"], 1)
    if edit["where"] == "after":
        return text.replace(edit["anchor"], edit["anchor"] + "\n" + edit["block"], 1)
    return text.replace(edit["anchor"], edit["block"] + "\n" + edit["anchor"], 1)


def undo(text, edit):
    if edit["kind"] == "replace":
        return text.replace(edit["new"], edit["old"], 1)
    glue = ("\n" + edit["block"]) if edit["where"] == "after" else (edit["block"] + "\n")
    return text.replace(glue, "", 1) if text.count(glue) == 1 else text


def write(path, text):
    temporary = path + ".docs5.tmp"
    with open(temporary, "wb") as handle:
        handle.write(text.encode("utf8"))
    try:
        os.chmod(temporary, os.stat(path).st_mode & 0o7777)
    except OSError:
        pass
    os.replace(temporary, path)


def main(argv):
    if len(argv) < 2 or argv[1] not in ("check", "status", "apply", "revert"):
        sys.exit(__doc__)
    mode = argv[1]
    root = find_root(argv)
    path = os.path.join(root, SPEC["guide"])
    text = read(path)
    sha = hashlib.sha256(text.encode("utf8")).hexdigest()
    edits = SPEC["edits"]
    states = [state(text, edit) for edit in edits]
    print("batch %s: guide sha256 %s (staged base %s: %s)" % (SPEC["name"], sha[:12], SPEC["baseSha256"][:12], "same" if sha == SPEC["baseSha256"] else "moved, anchors decide"))
    for edit, current in zip(edits, states):
        print("  %-8s %s [%s] %s" % (current.upper(), edit["id"], edit["lang"], (edit.get("anchor") or edit.get("old") or "")[:70].replace("\n", " ")))
    if mode in ("check", "status"):
        bad = [s for s in states if s in ("drift", "prereq")]
        print("result: %s" % ("applied" if all(s == "applied" for s in states) else "pending" if all(s == "pending" for s in states) else "partial" if not bad else "drift" if "drift" in bad else "prereq"))
        return 2 if "drift" in states else 3 if "prereq" in states else 0
    if "drift" in states:
        print("refused: an anchor moved or a text was edited since staging; nothing written")
        return 2
    if "prereq" in states:
        print("refused: the code batch named in `prereq` (%s) is not applied yet; nothing written" % SPEC.get("code", "?"))
        return 3
    if mode == "apply":
        if "--strict-base" in argv and sha != SPEC["baseSha256"] and all(s == "pending" for s in states):
            print("refused: the guide differs from the staged base and --strict-base is set; nothing written")
            return 2
        for edit, current in zip(edits, states):
            if current == "pending":
                text = do(text, edit)
        want = "applied"
    else:
        for edit, current in reversed(list(zip(edits, states))):
            if current == "applied":
                text = undo(text, edit)
        want = "pending"
    after = [state(text, edit) for edit in edits]
    if any(s != want for s in after):
        print("internal check failed (%s); nothing written" % ",".join("%s=%s" % (e["id"], s) for e, s in zip(edits, after) if s != want))
        return 4
    write(path, text)
    print("%s: %d edit(s) written to %s" % (mode, sum(1 for s in states if s != want), path))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
