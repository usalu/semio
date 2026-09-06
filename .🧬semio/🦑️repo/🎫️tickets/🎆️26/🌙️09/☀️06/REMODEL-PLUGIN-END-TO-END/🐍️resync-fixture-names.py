#!/usr/bin/env python3
"""🧷 Repoints stale references to a remodeling mutation fixture-case directory at the name that
directory actually carries on disk after the 2026-09-05 repo-wide path-shortening pass (commit
`3a6a9d6bfc`). On-disk names are the truth; nothing is renamed back.

Each mutation kind owns exactly one case directory, so the mapping is `<kind> -> <the one dir>` and
needs no hash derivation.

`--plan` prints the kind -> (stale, current, stale module ident, current module ident) table the
`📦️packages/🦀️rust/🦀️.rs` mounts need; that file is edited by hand with targeted edits because a
concurrent implementer owns another region of it.

`--oracle` rewrites `🔮️oracle/🔣️.json` in place at the TEXT level — only the
`mutationCatalogs[].vectors[].scenarios[].directoryName` values and their `id` siblings
(`🟦️.ts:769` requires `leadingEmojiIdentity(directoryName).rest === id`) — so no other byte of that
file's formatting moves. Add `--apply` to write.
"""
import json
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
OWNER = os.path.join(REPO, "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any")
MUTATIONS = os.path.join(OWNER, "🧬️schema/🧬️mutations")
WIRING = os.path.join(REPO, "✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/🦀️.rs")
ORACLE = os.path.join(OWNER, "🔮️oracle/🔣️.json")
APPLY = "--apply" in sys.argv


def case_directories():
    """🗺️ kind directory name -> the single fixture-case directory it owns on disk."""
    found = {}
    for kind in sorted(os.listdir(MUTATIONS)):
        tests = os.path.join(MUTATIONS, kind, "🧪️tests")
        if not os.path.isdir(tests):
            continue
        cases = sorted(name for name in os.listdir(tests) if os.path.isdir(os.path.join(tests, name)))
        if len(cases) != 1:
            raise SystemExit(f"[resync] {kind} owns {len(cases)} case directories, not one: {cases}")
        found[kind] = cases[0]
    return found


def leading_emoji_rest(name):
    """🔤️ The identity a directory name renders once its leading emoji identity is stripped."""
    index = 0
    while index < len(name) and not (name[index].isascii() and (name[index].isalnum() or name[index] == "-")):
        index += 1
    return name[index:]


def module_ident(case):
    """🦀️ `🎥️adds-stream-c-458900` -> `tests_adds_stream_c_458900`."""
    return "tests_" + re.sub(r"[^a-z0-9]+", "_", leading_emoji_rest(case).lower()).strip("_")


def plan(cases):
    text = open(WIRING, encoding="utf-8").read()
    rows = 0
    for kind, case in cases.items():
        for match in re.finditer(r'🧬️mutations/' + re.escape(kind) + r'/🧪️tests/([^/"]+)/🦀️\.rs"', text):
            stale = match.group(1)
            status = "OK" if stale == case else "STALE"
            print(f"{status}\t{kind}\t{stale}\t{case}\t{module_ident(stale)}\t{module_ident(case)}")
            rows += 1 if status == "STALE" else 0
    print(f"[plan] {rows} stale mount(s) in {WIRING}")
    return rows


def oracle(cases):
    raw = open(ORACLE, encoding="utf-8").read()
    document = json.loads(raw)
    replacements, changed = [], 0
    for catalog in document.get("mutationCatalogs", []):
        for vector in catalog.get("vectors", []):
            kind = vector["mutationDirectoryName"]
            case = cases.get(kind)
            if case is None:
                raise SystemExit(f"[oracle] vector names {kind}, which owns no fixture directory")
            for scenario in vector.get("scenarios", []):
                stale = scenario["directoryName"]
                if stale == case:
                    continue
                replacements.append((json.dumps(stale, ensure_ascii=False), json.dumps(case, ensure_ascii=False)))
                replacements.append((json.dumps(scenario["id"], ensure_ascii=False), json.dumps(leading_emoji_rest(case), ensure_ascii=False)))
                print(f"[oracle] {kind}: {stale} -> {case}")
                changed += 1
    text = raw
    for stale, fresh in replacements:
        if text.count(stale) != 1:
            raise SystemExit(f"[oracle] {stale} occurs {text.count(stale)} times; refusing an ambiguous text edit")
        text = text.replace(stale, fresh)
    if APPLY and text != raw:
        open(ORACLE, "w", encoding="utf-8").write(text)
    print(f"[oracle] scenarios repointed: {changed}; apply={APPLY}")
    return changed


def main():
    cases = case_directories()
    print(f"[resync] {len(cases)} kinds own a fixture case directory")
    if "--plan" in sys.argv:
        plan(cases)
    if "--oracle" in sys.argv:
        oracle(cases)
    return 0


if __name__ == "__main__":
    sys.exit(main())
