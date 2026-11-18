import json, sys

scenario = json.load(sys.stdin)
entries = scenario["entries"]
touched = set(scenario.get("touched", []))
distinct = set()
references = 0
for layers in entries:
    for layer in layers:
        for row in layer:
            distinct.add(row)
            references += 1
unique_layers = {tuple(layer) for layers in entries for layer in layers}
stale_entries = sum(1 for layers in entries if any(row in touched for layer in layers for row in layer))
stale_layers = len({tuple(layer) for layers in entries for layer in layers if any(row in touched for row in layer)})
print(json.dumps({"distinctRows": len(distinct), "rowReferences": references, "distinctLayers": len(unique_layers), "entries": len(entries), "staleEntries": stale_entries, "staleLayers": stale_layers}))
