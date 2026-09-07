#!/usr/bin/env python3
"""🔍️ W12 — audit every committed fem3d vector against the HARDENED rule set.

Reports, per vector, whether the committed `🎯️outcome` and `📸️snapshot/➡️after` still describe
what the hardened diff builders will do. Anything printed as `CONFLICT` is a fixture W12 has to
re-author (or a rule W12 has to reconsider).

Usage: uv run python 🔨️w12-audit-fem3d.py
"""

import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import importlib.util

RULES = importlib.util.spec_from_file_location("w12rules", os.path.join(os.path.dirname(os.path.abspath(__file__)), "🔨️w12-fem3d-rules.py"))
rules = importlib.util.module_from_spec(RULES)
RULES.loader.exec_module(rules)

REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *([".."] * 7)))
SUBSETS = os.path.join(REPO, "✏️s", "🔌️plugins", "🏗️fem", "🗿️artifacts", "🧊️3d", "🏅️standards", "🔖️1", "🪆️subsets")
SUBSET_NAMES = ("🕸️mesh", "🧱️material", "🛡️boundary", "🏋️load", "📈️analysis")


def read(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def vectors():
    for subset in SUBSET_NAMES:
        mutations = os.path.join(SUBSETS, subset, "🧬️schema", "🧬️mutations")
        for kind_directory in sorted(os.listdir(mutations)):
            tests = os.path.join(mutations, kind_directory, "🧪️tests")
            if not os.path.isdir(tests):
                continue
            for case in sorted(os.listdir(tests)):
                yield subset, kind_directory, case, os.path.join(tests, case)


def main():
    conflicts = []
    agree = 0
    for subset, kind_directory, case, root in vectors():
        before = read(os.path.join(root, "📸️snapshot", "⬅️before", "🔣️.json"))
        after = read(os.path.join(root, "📸️snapshot", "➡️after", "🔣️.json"))
        mutation = read(os.path.join(root, "🦠️mutation", "🔣️.json"))
        committed = read(os.path.join(root, "🎯️outcome", "🔣️.json"))
        produced, outcome = rules.apply_mutation(before, mutation)
        notes = []
        if outcome["status"] != committed.get("status"):
            notes.append("status %s -> %s" % (committed.get("status"), outcome["status"]))
        if rules.rejected(outcome) and outcome["code"] != committed.get("code"):
            notes.append("code %s -> %s" % (committed.get("code"), outcome["code"]))
        if rules.rejected(outcome) and outcome["path"] != committed.get("path", []):
            notes.append("path %s -> %s" % (committed.get("path"), outcome["path"]))
        committed_messages = [(entry["level"], entry["code"]) for entry in committed.get("messages", [])]
        produced_messages = [(entry["level"], entry["code"]) for entry in outcome["messages"]]
        if committed_messages != produced_messages:
            notes.append("messages %s -> %s" % (committed_messages, produced_messages))
        if produced != after:
            moved = [name for name in rules.MEMBERS if produced[name] != after[name]]
            notes.append("after differs in %s" % moved)
        if notes:
            conflicts.append((kind_directory, case, notes, outcome.get("message", "")))
        else:
            agree += 1
    print("vectors audited: %d" % (agree + len(conflicts)))
    print("  agree     %d" % agree)
    print("  CONFLICT  %d" % len(conflicts))
    for kind_directory, case, notes, message in conflicts:
        print("")
        print("  %s / %s" % (kind_directory, case))
        for note in notes:
            print("      %s" % note)
        if message:
            print("      would say: %s" % message)
    return 0


if __name__ == "__main__":
    sys.exit(main())
