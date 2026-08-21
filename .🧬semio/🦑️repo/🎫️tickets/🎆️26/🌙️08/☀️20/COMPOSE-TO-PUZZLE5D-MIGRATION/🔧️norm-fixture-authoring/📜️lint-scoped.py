#!/usr/bin/env python3
"""🧹️ The puzzle plugin's `fixtures lint` rules, transcribed verbatim from
`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts` and SCOPED to the four norm trees this lane
owns — the CLI truncates its repo-wide error list at 40 rows, so it cannot prove a specific tree is
clean. Also checks every `include_str!` target and every wired `#[path]`. Ticket scratch.
"""

import io
import json
import os
import re
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", "..", ".."))
NORM = os.path.join(REPO, "✏️s", "\U0001f50c️plugins", "\U0001f4d5️norm", "\U0001f5ff️artifacts")
TREES = ["\U0001f4d8️en1996", "\U0001f4d8️en1997", "\U0001f4d3️iso16757", "\U0001f4d8️en1995"]

NON_MUTATION_DIRS = {"\U0001f4be️binary", "\U0001f4dd️text"}
CORE_CASE_FILES = [
    "\U0001f9a0️mutation/\U0001f523️component.json",
    "\U0001f53a️diff/\U0001f523️component.json",
    "\U0001f3af️outcome/\U0001f523️component.json",
    "\U0001f980️component.rs",
]
DERIVED_CASE_FILES = [
    "\U0001f9a0️mutation/\U0001f527️component.op.semio",
    "\U0001f9a0️mutation/\U0001f4e1️component.spr.semio",
    "\U0001f53a️diff/\U0001fa79️component.patch.semio",
    "\U0001f53a️diff/\U0001f4e1️component.patch.spr.semio",
]
SNAPSHOT_CORE = "\U0001f523️component.json"
SNAPSHOT_DERIVED = ["\U0001f5e3️component.dsl.semio", "\U0001f3d2️component.pack.semio"]
SNAPSHOT_REF = "\U0001f517️component.ref.json"


def mutations_root(tree):
    return os.path.join(NORM, tree, "\U0001f3c5️standards", "\U0001f516️1", "\U0001fa86️subsets", "✳️any", "\U0001f9ec️schema", "\U0001f9ec️mutations")


def dirs_in(path):
    return sorted(e for e in os.listdir(path) if os.path.isdir(os.path.join(path, e))) if os.path.isdir(path) else []


def lint_case(case_dir, label):
    errors, warnings = [], []
    outcome_file = os.path.join(case_dir, "\U0001f3af️outcome/\U0001f523️component.json")
    rejected = False
    if os.path.isfile(outcome_file):
        try:
            outcome = json.load(io.open(outcome_file, encoding="utf-8"))
            rejected = outcome.get("status") == "rejected"
            if outcome.get("status") not in ("applied", "rejected"):
                errors.append("%s: outcome.status must be applied|rejected" % label)
            if rejected and not isinstance(outcome.get("code"), str):
                errors.append("%s: rejected outcome must carry a code" % label)
        except Exception as error:
            errors.append("%s: outcome is not valid JSON: %s" % (label, error))
    for relative in CORE_CASE_FILES:
        if rejected and relative.startswith("\U0001f53a️diff/"):
            continue
        if not os.path.isfile(os.path.join(case_dir, relative)):
            errors.append("%s: missing %s" % (label, relative))
    for relative in DERIVED_CASE_FILES:
        if rejected and relative.startswith("\U0001f53a️diff/"):
            continue
        if not os.path.isfile(os.path.join(case_dir, relative)):
            warnings.append("%s: missing derived %s" % (label, relative))
    if rejected and not os.path.isfile(os.path.join(case_dir, "\U0001f53a️diff/\U0001f6ab️component.absent")):
        errors.append("%s: rejected case must carry 🔺️diff/🚫️component.absent" % label)
    for side in ["⬅️before", "➡️after"]:
        side_dir = os.path.join(case_dir, "\U0001f4f8️snapshot", side)
        if not os.path.isdir(side_dir):
            errors.append("%s: missing 📸️snapshot/%s" % (label, side))
            continue
        if os.path.isfile(os.path.join(side_dir, SNAPSHOT_REF)):
            if os.path.isfile(os.path.join(side_dir, SNAPSHOT_CORE)):
                errors.append("%s: 📸️snapshot/%s has both a reference and inline encodings" % (label, side))
            continue
        if not os.path.isfile(os.path.join(side_dir, SNAPSHOT_CORE)):
            errors.append("%s: 📸️snapshot/%s is missing %s" % (label, side, SNAPSHOT_CORE))
        for name in SNAPSHOT_DERIVED:
            if not os.path.isfile(os.path.join(side_dir, name)):
                warnings.append("%s: missing derived 📸️snapshot/%s/%s" % (label, side, name))
    return errors, warnings


def main():
    total_errors = 0
    for tree in TREES:
        root = mutations_root(tree)
        aggregate = io.open(os.path.join(root, "\U0001f980️component.rs"), encoding="utf-8").read()
        body = re.search(r"pub enum \w*Mutation\w* \{([\s\S]*?)\n\}", aggregate)
        variants = re.findall(r"^\s+([A-Z][A-Za-z0-9]*)\(", body.group(1), re.M) if body else []
        leaves = [e for e in dirs_in(root) if e not in NON_MUTATION_DIRS and os.path.isfile(os.path.join(root, e, "\U0001f9a0️mutation/\U0001f980️component.rs"))]
        by_struct = {}
        for leaf in leaves:
            source = io.open(os.path.join(root, leaf, "\U0001f9a0️mutation/\U0001f980️component.rs"), encoding="utf-8").read()
            found = re.search(r"^pub struct ([A-Za-z0-9]+)", source, re.M)
            if found:
                by_struct[found.group(1)] = leaf

        errors, warnings, covered = [], [], 0
        for variant in variants:
            if variant not in by_struct:
                errors.append("%s: enum variant %s has no mutation directory" % (tree, variant))
        for leaf in leaves:
            cases = dirs_in(os.path.join(root, leaf, "\U0001f9ea️tests"))
            if not cases:
                errors.append("%s/%s: no 🧪️tests cases" % (tree, leaf))
                continue
            covered += 1
            for case in cases:
                case_errors, case_warnings = lint_case(os.path.join(root, leaf, "\U0001f9ea️tests", case), "%s/%s/%s" % (tree, leaf, case))
                errors += case_errors
                warnings += case_warnings

        # include_str! targets + wired #[path] modules
        wired = re.findall(r'#\[path = "([^"]+)"\]\n\s*mod (tests_\w+);', aggregate)
        for relative, _module in wired:
            if not os.path.isfile(os.path.join(root, relative)):
                errors.append("%s: wired #[path] does not resolve: %s" % (tree, relative))
        for leaf in leaves:
            for case in dirs_in(os.path.join(root, leaf, "\U0001f9ea️tests")):
                case_dir = os.path.join(root, leaf, "\U0001f9ea️tests", case)
                test_rs = os.path.join(case_dir, "\U0001f980️component.rs")
                if not os.path.isfile(test_rs):
                    continue
                for target in re.findall(r'include_str!\("([^"]+)"\)', io.open(test_rs, encoding="utf-8").read()):
                    if not os.path.isfile(os.path.join(case_dir, target)):
                        errors.append("%s/%s/%s: include_str! target missing: %s" % (tree, leaf, case, target))
        wired_cases = len(wired)
        expected_cases = sum(len(dirs_in(os.path.join(root, leaf, "\U0001f9ea️tests"))) for leaf in leaves)
        if wired_cases != expected_cases:
            errors.append("%s: %d cases on disk but %d wired modules" % (tree, expected_cases, wired_cases))

        total_errors += len(errors)
        print("%-14s %d/%d uncovered · %d case(s) · %d wired · %d error(s) · %d derived-encoding warning(s)"
              % (tree, len(leaves) - covered, len(leaves), expected_cases, wired_cases, len(errors), len(warnings)))
        for error in errors:
            print("   ❌️ " + error)
    print("SCOPED TOTAL ERRORS: %d" % total_errors)
    return 1 if total_errors else 0


if __name__ == "__main__":
    sys.exit(main())
