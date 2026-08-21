#!/usr/bin/env python3
"""Re-runs `fixtures lint`'s own rules (transcribed from
`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts`) scoped to ONE agent's 24 trees, because the
repo-wide CLI truncates its error list at 40 rows. Also checks that every `include_str!` target
resolves and every self-wired `#[path]` module file exists.

usage: scoped-lint-stdio-formats-b.py <repo-root>
"""
import json
import os
import re
import sys

CORE_CASE_FILES = ["🦠️mutation/🔣️component.json", "🔺️diff/🔣️component.json", "🎯️outcome/🔣️component.json", "🦀️component.rs"]
DERIVED_CASE_FILES = ["🦠️mutation/🔧️component.op.semio", "🦠️mutation/📡️component.spr.semio", "🔺️diff/🩹️component.patch.semio", "🔺️diff/📡️component.patch.spr.semio"]
SNAPSHOT_CORE = "🔣️component.json"
SNAPSHOT_DERIVED = ["🗣️component.dsl.semio", "🎒️component.pack.semio"]
SNAPSHOT_REF = "🔗️component.ref.json"
NON_MUTATION_DIRS = {"💾️binary", "📝️text"}

A = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
S = "🧿️semio/🏅️standards/🔖️v1/🪆️subsets"
TREES = [
    f"{A}/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    f"{A}/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    f"{A}/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    f"{A}/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    f"{A}/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    f"{A}/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    f"{A}/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    f"{A}/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    f"{A}/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    f"{A}/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    f"{A}/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    f"{A}/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    f"{A}/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    f"{A}/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
] + [f"{A}/{S}/{name}/🧬️schema/🧬️mutations" for name in ["✳️any", "✳️value", "✳️video", "✳️audio", "✳️flow", "✳️model", "✳️animation", "✳️document", "✳️cad", "✳️presentation"]]


def dirs_in(path):
    return sorted(entry for entry in os.listdir(path) if os.path.isdir(os.path.join(path, entry))) if os.path.isdir(path) else []


def lint_case(case_dir, label):
    errors, warnings = [], []
    outcome_file = os.path.join(case_dir, "🎯️outcome/🔣️component.json")
    rejected = False
    if os.path.exists(outcome_file):
        try:
            outcome = json.load(open(outcome_file, encoding="utf8"))
            rejected = outcome.get("status") == "rejected"
            if outcome.get("status") not in ("applied", "rejected"):
                errors.append(f"{label}: 🎯️outcome.status must be applied or rejected")
            if rejected and not isinstance(outcome.get("code"), str):
                errors.append(f"{label}: rejected outcome must carry a machine-readable code")
        except Exception as error:
            errors.append(f"{label}: 🎯️outcome is not valid JSON: {error}")
    for relative in CORE_CASE_FILES:
        if rejected and relative.startswith("🔺️diff/"):
            continue
        if not os.path.exists(os.path.join(case_dir, relative)):
            errors.append(f"{label}: missing {relative}")
    for relative in DERIVED_CASE_FILES:
        if rejected and relative.startswith("🔺️diff/"):
            continue
        if not os.path.exists(os.path.join(case_dir, relative)):
            warnings.append(f"{label}: missing derived {relative}")
    if rejected and not os.path.exists(os.path.join(case_dir, "🔺️diff/🚫️component.absent")):
        errors.append(f"{label}: rejected case must carry 🔺️diff/🚫️component.absent")
    for side in ["⬅️before", "➡️after"]:
        side_dir = os.path.join(case_dir, "📸️snapshot", side)
        if not os.path.isdir(side_dir):
            errors.append(f"{label}: missing 📸️snapshot/{side}")
            continue
        if os.path.exists(os.path.join(side_dir, SNAPSHOT_REF)):
            if os.path.exists(os.path.join(side_dir, SNAPSHOT_CORE)):
                errors.append(f"{label}: 📸️snapshot/{side} has both a reference and inline encodings")
            continue
        if not os.path.exists(os.path.join(side_dir, SNAPSHOT_CORE)):
            errors.append(f"{label}: 📸️snapshot/{side} is missing {SNAPSHOT_CORE}")
        for name in SNAPSHOT_DERIVED:
            if not os.path.exists(os.path.join(side_dir, name)):
                warnings.append(f"{label}: missing derived 📸️snapshot/{side}/{name}")
    return errors, warnings


def check_wiring(root, tree, case_dir, label):
    errors = []
    test_file = os.path.join(case_dir, "🦀️component.rs")
    if not os.path.exists(test_file):
        return errors
    source = open(test_file, encoding="utf8").read()
    for target in re.findall(r'include_str!\("([^"]+)"\)', source):
        if not os.path.exists(os.path.join(case_dir, target)):
            errors.append(f"{label}: include_str! target does not resolve: {target}")
    root_rs = os.path.join(tree, "🦀️component.rs")
    wired = re.findall(r'#\[path = "([^"]+)"\]', open(root_rs, encoding="utf8").read())
    relative = os.path.relpath(test_file, tree)
    if relative not in wired:
        errors.append(f"{label}: not wired from the tree's own mutations-root 🦀️component.rs")
    for target in wired:
        if not os.path.exists(os.path.join(tree, target)):
            errors.append(f"{tree}: #[path] does not resolve: {target}")
    return errors


def main(root):
    os.chdir(root)
    total_errors, total_warnings, rows = [], 0, []
    for tree in TREES:
        leaves = [entry for entry in dirs_in(tree) if entry not in NON_MUTATION_DIRS and os.path.exists(os.path.join(tree, entry, "🦠️mutation/🦀️component.rs"))]
        covered, tree_errors, tree_warnings = 0, [], 0
        for leaf in leaves:
            cases = dirs_in(os.path.join(tree, leaf, "🧪️tests"))
            if not cases:
                tree_errors.append(f"{tree}/{leaf}: no 🧪️tests cases")
                continue
            covered += 1
            for case in cases:
                case_dir = os.path.join(tree, leaf, "🧪️tests", case)
                label = f"{tree}/{leaf}/{case}"
                errors, warnings = lint_case(case_dir, label)
                tree_errors += errors + check_wiring(root, tree, case_dir, label)
                tree_warnings += len(warnings)
        rows.append((len(leaves) - covered, len(leaves), len(tree_errors), tree_warnings, tree))
        total_errors += tree_errors
        total_warnings += tree_warnings
    for uncovered, leaves, errors, warnings, tree in rows:
        mark = "✅️" if errors == 0 and uncovered == 0 else "❌️"
        print(f"{mark} {leaves - uncovered}/{leaves} covered · {errors} error(s) · {warnings} derived warning(s)  {tree}")
    print()
    for error in total_errors:
        print(f"❌️ {error}")
    print(f"\n{'✅️' if not total_errors else '❌️'} {len(total_errors)} error(s), {total_warnings} derived-encoding warning(s) across {len(TREES)} trees")
    return 1 if total_errors else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1]))
