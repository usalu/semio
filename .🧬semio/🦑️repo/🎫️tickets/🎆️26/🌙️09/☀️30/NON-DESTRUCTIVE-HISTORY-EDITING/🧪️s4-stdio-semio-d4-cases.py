#!/usr/bin/env python3
"""🧩️ D4 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING for the fixture-driven semio subsets whose mutate cases name a
committed payload per row (presentation, drawing, image, mesh): commits each subset's `🩹️patch-snapshot` payload fixture in
the aggregate's own wire tagging, lists it in the real-artifact mutate and inverse outlines, and arms the subset's
independent Python oracle — `patched_snapshot` over its own reading, undone by its own `set-snapshot` or by the exact inverse
patch; mesh's TypeScript oracle carries its own hand-written pointer arm, which is only required present here.
Idempotent; `--check` lists what is pending and exits 1 while anything is.

@see ../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py
"""
from __future__ import annotations

import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
SUBSETS = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"
IMPORT = "from semio_repo_test import Adapter, Context, Outcome, digest\n"
FIXTURE = "🩹️patch-snapshot"
CASES = {
    "📽️presentation": ("📽️mutate-semio-presentation", "internal", {"operation": "set", "path": "/masters/0/shapes/0/blocks/0/runs/0/text", "value": "Mastertitelformat gepatcht"}, ("fifteen", "sixteen")),
    "🖼️image": ("🖼️mutate-semio-image", "internal", {"operation": "set", "path": "/frames/0/delayMs", "value": 200}, ("thirteen", "fourteen")),
    "🖊️drawing": ("🖊️mutate-semio-drawing", "external", {"operation": "set", "path": "/layers/0/name", "value": "Introduction demonstration mouse, patched"}, ("seventeen", "eighteen")),
    "🔺️mesh": ("🔺️mutate-semio-mesh", "typescript", {"operation": "set", "path": "/materials/0/roughness", "value": 0.5}, ("seventeen", "eighteen")),
}


def armed(text: str, tagging: str, counts: tuple[str, str]) -> str:
    if '"patch-snapshot"' in text:
        return text
    assert text.count(IMPORT) == 1, "the host import anchor is missing"
    names = "patched_snapshot" if tagging == "internal" else "patched_snapshot, snapshot_patch_inverse"
    text = text.replace(IMPORT, f"from semio_repo_test import Adapter, Context, Outcome, digest, {names}\n")
    text = text.replace(f"this subset's {counts[0]} declared verbs", f"this subset's {counts[1]} declared verbs")
    if tagging == "internal":
        text = text.replace('    "setSnapshot": "set-snapshot",\n', '    "setSnapshot": "set-snapshot",\n    "patchSnapshot": "patch-snapshot",\n', 1)
        apply = re.search(r"\ndef apply_mutation\(.*?\n    kind = kind_of\(mutation\)\n    result = [^\n]+\n", text, re.S)
        undo = re.search(r"\ndef inverse_mutation\(.*?\n    kind = kind_of\(mutation\)\n", text, re.S)
        assert apply and undo, "the apply or inverse anchor is missing"
        text = text[: undo.end()] + '    if kind == "patch-snapshot":\n        return {"mutation": "setSnapshot", "snapshot": json.loads(json.dumps(document))}\n' + text[undo.end() :]
        return text[: apply.end()] + '    if kind == "patch-snapshot":\n        return patched_snapshot(document, mutation["patch"])\n' + text[apply.end() :]
    text = text.replace('    "CreateLayer": "create-layer",\n', '    "CreateLayer": "create-layer",\n    "PatchSnapshot": "patch-snapshot",\n', 1)
    apply = "    verb, argument = verb_of(mutation)\n    kind = VERBS[verb]\n    result = clone(document)\n"
    assert text.count(apply) == 1, "the apply anchor is missing"
    text = text.replace(apply, apply + '    if kind == "patch-snapshot":\n        return patched_snapshot(document, argument["patch"])\n')
    undo = "    verb, argument = verb_of(mutation)\n    kind = VERBS[verb]\n    if kind == \"create-layer\":\n"
    assert text.count(undo) == 1, "the inverse anchor is missing"
    return text.replace(undo, undo.replace('    if kind == "create-layer":\n', '    if kind == "patch-snapshot":\n        return [{"PatchSnapshot": {"patch": snapshot_patch_inverse(document, argument["patch"])}}]\n    if kind == "create-layer":\n'))


def with_rows(text: str) -> str:
    lines = text.split("\n")
    inserts = []
    for index, line in enumerate(lines):
        title = line.strip()
        if not (title.startswith("Scenario Outline: Apply <id> to the real derived") or title.startswith("Scenario Outline: Undoing <id> restores the real derived")):
            continue
        cursor = next(at for at in range(index + 1, len(lines)) if lines[at].strip().startswith("Examples:"))
        end = next((at for at in range(cursor + 2, len(lines)) if not lines[at].strip().startswith("|")), len(lines))
        if not any(row.split("|")[1].strip() == "patch-snapshot" for row in lines[cursor + 2:end]):
            inserts.append(end)
    for at in reversed(inserts):
        lines.insert(at, f"      | patch-snapshot | {FIXTURE} |")
    return "\n".join(lines)


def main() -> int:
    check = "--check" in sys.argv
    pending = 0
    for subset, (case, tagging, patch, counts) in CASES.items():
        feature = SUBSETS / subset / "🧪️tests" / case / "🥒️.feature"
        payload = SUBSETS / subset / "🧫️fixtures" / case / FIXTURE / "🦠️mutation" / "🔣️.json"
        wire = {"mutation": "patchSnapshot", "patch": patch} if tagging == "internal" else {"PatchSnapshot": {"patch": patch}}
        wanted = {feature: with_rows(feature.read_text(encoding="utf-8")), payload: json.dumps(wire, ensure_ascii=False, separators=(",", ":")) + "\n"}
        if tagging == "typescript":
            assert "function patchedSnapshot" in (SUBSETS / subset / "🧪️tests" / case / "🟦️.ts").read_text(encoding="utf-8"), f"{subset}: the TypeScript oracle carries no patch-snapshot arm"
        else:
            oracle = SUBSETS / subset / "🧪️tests" / case / "🐍️.py"
            wanted[oracle] = armed(oracle.read_text(encoding="utf-8"), tagging, counts)
        for path, content in wanted.items():
            current = path.read_text(encoding="utf-8") if path.exists() else None
            if current != content:
                pending += 1
                print(f"{'pending' if check else 'written'}: {path.relative_to(SUBSETS)}")
                if not check:
                    path.parent.mkdir(parents=True, exist_ok=True)
                    path.write_text(content, encoding="utf-8")
    print(f"{pending} file(s) {'pending' if check else 'written'}")
    return 1 if check and pending else 0


if __name__ == "__main__":
    sys.exit(main())
