#!/usr/bin/env python3
"""🔮️ LB2 triage oracle (python-jsonschema 4.x, third-party): does every stdio editor's NEW document (its initial snapshot, dumped
by the scratch-only `lb2_dump_initial_snapshots` probe) conform to its own snapshot schema?

The snapshot schema of a document is found schema-first: the document's `schema` field names the artifact schema id
(`s.<schema>`), which the `#[artifact_schema(id = "…")]` derive beside each `🧬️schema/📸️snapshot/🦀️.rs` owns; the sibling
`🔣️.json` is that snapshot contract. Every `$id` document under the stdio artifacts and the framework is loaded into one
`referencing` registry, so cross-document `$ref`s resolve exactly as far as the documents exist.

usage: python3 lb2-initial-snapshot-oracle.py <dump-dir> [<tree>] [--refs]   (--refs: every `$ref` of every snapshot contract resolves)
"""
import glob, json, os, re, sys

import jsonschema
from referencing import Registry, Resource
from referencing.jsonschema import DRAFT7

DUMP = sys.argv[1]
TREE = sys.argv[2] if len(sys.argv) > 2 and not sys.argv[2].startswith("--") else "/Users/ueli/Documents/semio"
ART = os.path.join(TREE, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts")

owners = {}
for rust in glob.glob(os.path.join(ART, "**/🧬️schema/📸️snapshot/🦀️.rs"), recursive=True):
    found = re.search(r'#\[artifact_schema\(id = "([^"]+)"\)\]', open(rust, encoding="utf-8").read())
    sibling = os.path.join(os.path.dirname(rust), "🔣️.json")
    if found and os.path.isfile(sibling):
        owners.setdefault(found.group(1), sibling)

resources = []
for root in (ART, os.path.join(TREE, "🧰️framework")):
    for path in glob.glob(os.path.join(root, "**/🔣️.json"), recursive=True):
        if "node_modules" in path:
            continue
        try:
            document = json.load(open(path, encoding="utf-8"))
        except (OSError, ValueError):
            continue
        if isinstance(document, dict) and isinstance(document.get("$id"), str):
            resources.append((document["$id"], Resource.from_contents(document, default_specification=DRAFT7)))
registry = Registry().with_resources(resources)

def unresolved_refs(contract_path):
    document = json.load(open(contract_path, encoding="utf-8"))
    resolver = registry.resolver(base_uri=document.get("$id", ""))
    missing = []

    def walk(node):
        if isinstance(node, dict):
            reference = node.get("$ref")
            if isinstance(reference, str):
                try:
                    resolver.lookup(reference)
                except Exception as error:
                    missing.append(f"{reference} ({type(error).__name__})")
            for value in node.values():
                walk(value)
        elif isinstance(node, list):
            for value in node:
                walk(value)

    walk(document)
    return missing


if "--refs" in sys.argv:
    broken = {schema_id: refs for schema_id, contract in sorted(owners.items()) if (refs := unresolved_refs(contract))}
    print(f"snapshot contracts {len(owners)}, with unresolved refs {len(broken)}")
    for schema_id, refs in broken.items():
        print(f"  {schema_id}: {sorted(set(refs))[:4]}")
    sys.exit(0)

report = {"conforming": [], "violations": {}, "unowned": []}
for path in sorted(glob.glob(os.path.join(DUMP, "*.json"))):
    name = os.path.basename(path)[:-5]
    snapshot = json.load(open(path, encoding="utf-8"))
    schema_id = snapshot.get("schema", "")
    schema_id = schema_id if schema_id.startswith("s.") else f"s.{schema_id}"
    contract = owners.get(schema_id)
    if contract is None:
        report["unowned"].append(f"{name}: {schema_id}")
        continue
    validator = jsonschema.Draft7Validator(json.load(open(contract, encoding="utf-8")), registry=registry)
    try:
        errors = sorted(validator.iter_errors(snapshot), key=lambda error: list(error.absolute_path))
        messages = [f"/{'/'.join(map(str, error.absolute_path))}: {error.message[:160]}" for error in errors]
    except Exception as error:
        messages = [f"unresolvable contract: {error}"[:220]]
    if messages:
        report["violations"][name] = messages[:6]
    else:
        report["conforming"].append(name)

print(f"conforming {len(report['conforming'])}, violating {len(report['violations'])}, unowned {len(report['unowned'])}")
for name, messages in report["violations"].items():
    print(f"  {name}: {messages}")
for line in report["unowned"]:
    print(f"  UNOWNED {line}")
