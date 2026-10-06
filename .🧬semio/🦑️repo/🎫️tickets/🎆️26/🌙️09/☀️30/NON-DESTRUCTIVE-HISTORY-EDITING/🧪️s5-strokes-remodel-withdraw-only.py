#!/usr/bin/env python3
"""🚪️ Declares the eight remodel result leaves withdraw-only (design §22.20, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

usage: python3 🧪️s5-strokes-remodel-withdraw-only.py [--check]   (from the repository root)

`replace-{dense,geo-products,mesh-result,qc,sparse,tracks,trajectory}` and `commit-reconstruction` carry whole pipeline
results (point clouds, meshes, reports): every input is `hidden`, so their editor would open with zero rows. Their leaf
descriptor gains `"editable": false` as its last key — the derive then answers `input_schema() -> None`, the history row
offers Withdraw/Restore only and the inputs gate does not judge them. Explicit file list; a descriptor that does not
re-print byte-identically, or whose payload shows an input, fails the run before anything is written.
"""
import collections
import glob
import json
import sys

ROOT = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
LEAVES = ["replace-dense", "replace-geo-products", "replace-mesh-result", "replace-qc", "replace-sparse", "replace-tracks", "replace-trajectory", "commit-reconstruction"]


def printed(document):
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"


def main():
    check = sys.argv[1:] == ["--check"]
    if sys.argv[1:] and not check:
        sys.exit(__doc__)
    plan, pending = [], 0
    for leaf in LEAVES:
        matches = [path for path in glob.glob(f"{ROOT}/*{leaf}") if not any(character.isascii() for character in path.rsplit("/", 1)[-1][: -len(leaf)])]
        if len(matches) != 1:
            sys.exit(f"{leaf}: expected one leaf directory, found {matches}")
        path = f"{matches[0]}/🔣️.json"
        raw = open(path, encoding="utf-8").read()
        descriptor = json.loads(raw, object_pairs_hook=collections.OrderedDict)
        if descriptor.get("semanticKind") != leaf:
            sys.exit(f"{path}: semanticKind is {descriptor.get('semanticKind')!r}")
        if printed(descriptor) != raw:
            sys.exit(f"{path} does not re-print byte-identically; nothing written")
        payload = json.load(open(f"{matches[0]}/{descriptor['payloadSchema']}", encoding="utf-8"))
        shown = [name for name, node in payload.get("properties", {}).items() if name != "mutation" and (node.get("x-semio-ui") or {}).get("widget") != "hidden"]
        if shown:
            sys.exit(f"{path}: the payload shows input(s) {shown}; an editable leaf is not declared withdraw-only")
        if descriptor.get("editable") is False:
            continue
        pending += 1
        descriptor.pop("editable", None)
        descriptor["editable"] = False
        plan.append((path, printed(descriptor)))
    if check:
        print(f"{pending} pending descriptor(s)")
        sys.exit(1 if pending else 0)
    for path, text in plan:
        open(path, "w", encoding="utf-8").write(text)
    print(f"declared {len(plan)} leaf descriptor(s) withdraw-only")


main()
