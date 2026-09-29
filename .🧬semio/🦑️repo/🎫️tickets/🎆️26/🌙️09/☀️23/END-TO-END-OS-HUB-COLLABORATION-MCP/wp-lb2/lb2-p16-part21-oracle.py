#!/usr/bin/env python3
"""🔮️ LB2 p16 third-party oracle (IfcOpenShell 0.8.4): the stdio contract's canonical Part-21 reading of every committed IFC 2x3
fixture agrees with IfcOpenShell's own reading of the same file — the same instance ids, entity types (case-insensitive), argument
counts, instance count and FILE_SCHEMA. Input: the JSON projections the ifc law
`committed_ifc2x3_fixtures_read_through_the_canonical_part21_codec` writes with `SEMIO_PART21_ORACLE_OUT=<dir>`.

usage: python3 lb2-p16-part21-oracle.py <dir>
"""
import glob, json, sys

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
