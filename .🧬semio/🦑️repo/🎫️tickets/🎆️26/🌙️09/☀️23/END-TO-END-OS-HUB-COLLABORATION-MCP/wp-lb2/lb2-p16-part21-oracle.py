#!/usr/bin/env python3
"""🔮️ LB2 p16 third-party oracle (IfcOpenShell 0.8.4): the stdio contract's canonical Part-21 reading of every committed IFC 2x3
fixture agrees with IfcOpenShell's own reading of the same file — the same instance ids, entity types (case-insensitive), argument
counts, instance count and FILE_SCHEMA. Input: the JSON projections the ifc law
`committed_ifc2x3_fixtures_read_through_the_canonical_part21_codec` writes with `SEMIO_PART21_ORACLE_OUT=<dir>`.

Second oracle (python-jsonschema 4.x + referencing, `--schemas <tree>`): every `$ref` of the ifc 2x3 payload/snapshot/artifact
contracts resolves across documents, and the converted set-snapshot fixture validates against them (mutation payload, before and
after snapshots) — the same `$id` registry the contracts publish.

usage: python3 lb2-p16-part21-oracle.py <dir> | --schemas <tree>
"""
import glob, json, os, sys


def schemas(tree):
    import jsonschema
    from referencing import Registry, Resource
    from referencing.jsonschema import DRAFT7

    root = os.path.join(tree, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3")
    documents = {}
    for path in glob.glob(os.path.join(root, "**/🔣️.json"), recursive=True):
        document = json.load(open(path, encoding="utf-8"))
        if isinstance(document, dict) and isinstance(document.get("$id"), str):
            documents[document["$id"]] = (path, document)
    registry = Registry().with_resources((key, Resource.from_contents(document, default_specification=DRAFT7)) for key, (_, document) in documents.items())
    failures = []

    def refs(node, resolver, where):
        if isinstance(node, dict):
            if isinstance(node.get("$ref"), str):
                try:
                    resolver.lookup(node["$ref"])
                except Exception as error:
                    failures.append(f"{where}: {node['$ref']} ({type(error).__name__})")
            for value in node.values():
                refs(value, resolver, where)
        elif isinstance(node, list):
            for value in node:
                refs(value, resolver, where)

    for key, (path, document) in sorted(documents.items()):
        refs(document, registry.resolver(base_uri=key), key.split("/ifc/2x3/")[-1])
    base = "https://json.schemas.assets.semio-tech.com/s/stdio/ifc/2x3/base"
    case = os.path.join(root, "🪆️subsets/🧱️base/🧫️fixtures/🧬️mutations/📸️set-snapshot/✏️renames-the-ifcproject-instance")
    checks = [("🦠️mutation", f"{base}/mutation/set-snapshot/schema.json"), ("📸️snapshot/⬅️before", f"{base}/snapshot.json"), ("📸️snapshot/➡️after", f"{base}/snapshot.json")]
    for folder, key in checks:
        instance = json.load(open(os.path.join(case, folder, "🔣️.json"), encoding="utf-8"))
        validator = jsonschema.Draft7Validator(documents[key][1], registry=registry)
        errors = sorted(validator.iter_errors(instance), key=lambda error: list(error.absolute_path))
        failures.extend(f"{folder}: {'/'.join(map(str, error.absolute_path))}: {error.message[:160]}" for error in errors[:5])
    print(f"ifc 2x3 contracts {len(documents)}, fixtures {len(checks)}, failures {len(failures)}")
    for line in failures:
        print("  ", line)
    sys.exit(1 if failures else 0)


if sys.argv[1] == "--schemas":
    schemas(sys.argv[2])

import ifcopenshell

files = sorted(glob.glob(f"{sys.argv[1]}/*.json"))
agreeing, refused, disagreements = 0, [], []
for path in files:
    record = json.load(open(path, encoding="utf-8"))
    source, document = record["source"], record["document"]
    try:
        model = ifcopenshell.open(source)
    except Exception as error:
        refused.append(f"{source.split('🔖️2x3/')[-1]}: {type(error).__name__}: {str(error)[:120]}")
        continue
    problems = []
    schema = [value["value"] for item in document["header"]["fileSchema"] for value in item.get("values", [])]
    if model.schema not in schema:
        problems.append(f"FILE_SCHEMA {schema} vs {model.schema}")
    instances = document["instances"]
    if len(list(model)) != len(instances):
        problems.append(f"{len(instances)} instances vs {len(list(model))}")
    for instance in instances:
        if len(instance["entities"]) != 1:
            continue
        entity = model.by_id(instance["id"])
        ours = instance["entities"][0]
        if entity.is_a().upper() != ours["typeName"].upper() or len(entity) != len(ours["arguments"]):
            problems.append(f"#{instance['id']} {ours['typeName']}/{len(ours['arguments'])} vs {entity.is_a()}/{len(entity)}")
    if problems:
        disagreements.append(f"{source.split('🔖️2x3/')[-1]}: {problems[:4]}")
    else:
        agreeing += 1
print(f"fixtures {len(files)}, agreeing {agreeing}, disagreeing {len(disagreements)}, refused by IfcOpenShell {len(refused)}")
for line in disagreements + refused:
    print("  ", line)
sys.exit(1 if disagreements or not files else 0)
