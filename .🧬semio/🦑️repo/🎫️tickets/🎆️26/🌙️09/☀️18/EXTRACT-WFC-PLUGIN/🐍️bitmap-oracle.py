"""🐍️ A1 bitmap — the ticket-local gate that runs every non-cargo check this artifact owns, in one
pass, from the repository root:

    python3 ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/EXTRACT-WFC-PLUGIN/🐍️bitmap-oracle.py"

It runs (1) the subset's cross-language mutation oracle, (2) the mount-contract reader, (3) a
structural audit of the committed fixture tree against the oracle manifest's own vector list, and
(4) a JSON well-formedness pass over every schema leaf the crate `include_str!`s, because a leaf
that stops parsing is a descriptor the host rejects at registration rather than at build.
"""

import json
import os
import subprocess
import sys

TICKET = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(TICKET, os.pardir, os.pardir, os.pardir, os.pardir, os.pardir, os.pardir, os.pardir))
SUBSET = os.path.join(ROOT, "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any")
QUINTET = [
    os.path.join("📸️snapshot", "⬅️before", "🔣️.json"),
    os.path.join("📸️snapshot", "➡️after", "🔣️.json"),
    os.path.join("🦠️mutation", "🔣️.json"),
    os.path.join("🔺️diff", "🔣️.json"),
    os.path.join("🎯️outcome", "🔣️.json"),
]


def run(label, script):
    print("== " + label)
    return subprocess.run([sys.executable, os.path.join(SUBSET, script)], check=False).returncode


def mutation_kinds_on_disk():
    root = os.path.join(SUBSET, "🧬️schema", "🧬️mutations")
    kinds = set()
    for entry in os.listdir(root):
        manifest = os.path.join(root, entry, "🔣️.json")
        if os.path.isfile(manifest):
            with open(manifest, encoding="utf-8") as handle:
                kinds.add(json.load(handle)["semanticKind"])
    return kinds


def audit_fixture_tree():
    print("== fixture tree vs oracle manifest")
    problems = []
    with open(os.path.join(SUBSET, "🔮️oracles", "🔣️.json"), encoding="utf-8") as handle:
        manifest = json.load(handle)
    declared = {}
    for vector in manifest["mutationCatalogs"][0]["vectors"]:
        for scenario in vector["scenarios"]:
            declared[(vector["mutationDirectoryName"], scenario["directoryName"])] = vector["mutationId"]
    root = os.path.join(SUBSET, "🧫️fixtures", "🧬️mutations")
    on_disk = set()
    for kind in sorted(os.listdir(root)):
        kind_path = os.path.join(root, kind)
        if not os.path.isdir(kind_path):
            continue
        for case in sorted(os.listdir(kind_path)):
            on_disk.add((kind, case))
            for leaf in QUINTET:
                path = os.path.join(kind_path, case, leaf)
                if not os.path.isfile(path):
                    problems.append(kind + "/" + case + " is missing " + leaf)
                    continue
                try:
                    with open(path, encoding="utf-8") as handle:
                        json.load(handle)
                except ValueError as error:
                    problems.append(path + " is not valid JSON: " + str(error))
    for key in sorted(set(declared) - on_disk):
        problems.append("the oracle manifest declares " + key[0] + "/" + key[1] + " but the fixture tree has no such case")
    for key in sorted(on_disk - set(declared)):
        problems.append("the fixture tree carries " + key[0] + "/" + key[1] + " but the oracle manifest declares no vector for it")
    for kind in sorted(mutation_kinds_on_disk() - set(declared.values())):
        problems.append("mutation kind " + kind + " has no committed vector")
    for problem in problems:
        print("FAIL " + problem)
    print("audited " + str(len(on_disk)) + " fixture cases, " + str(len(problems)) + " problems")
    return 1 if problems else 0


def audit_schema_leaves():
    print("== schema leaf well-formedness")
    problems = []
    checked = 0
    for directory, _, files in os.walk(os.path.join(SUBSET, "🧬️schema")):
        for name in files:
            if not name.endswith(".json"):
                continue
            path = os.path.join(directory, name)
            checked += 1
            try:
                with open(path, encoding="utf-8") as handle:
                    body = json.load(handle)
            except ValueError as error:
                problems.append(path + " is not valid JSON: " + str(error))
                continue
            if "$schema" in body and "json-schema.org" in str(body["$schema"]) and "$id" not in body:
                problems.append(path + " is a JSON Schema with no $id")
            # 🚧️ `contentEncoding` sits outside the owned validator's keyword allowlist; a base64
            # field says `"format": "base64"` instead.
            if "contentEncoding" in json.dumps(body):
                problems.append(path + " uses contentEncoding, which the owned validator refuses")
    for problem in problems:
        print("FAIL " + problem)
    print("checked " + str(checked) + " JSON leaves, " + str(len(problems)) + " problems")
    return 1 if problems else 0


def main():
    status = 0
    status |= run("mutation oracle (python second implementation)", os.path.join("🧪️tests", "🧩️mutate-bitmap-1", "🐍️.py"))
    status |= run("mount contract", os.path.join("🧪️tests", "🧩️mount-contract", "🐍️.py"))
    status |= audit_fixture_tree()
    status |= audit_schema_leaves()
    print("== bitmap oracle gate " + ("PASSED" if status == 0 else "FAILED"))
    return status


if __name__ == "__main__":
    sys.exit(main())
