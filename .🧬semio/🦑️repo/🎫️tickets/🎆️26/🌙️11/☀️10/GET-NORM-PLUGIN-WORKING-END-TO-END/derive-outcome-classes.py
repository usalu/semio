import json, os, sys
subset = sys.argv[1]
mutations = os.path.join(subset, "🧬️schema", "🧬️mutations")
oracle_path = os.path.join(subset, "🔮️oracles", "🔣️.json")
derived = {}
changed = []
for name in sorted(os.listdir(mutations)):
    leaf = os.path.join(mutations, name, "🔣️.json")
    if not os.path.isfile(leaf):
        continue
    directory = os.path.dirname(leaf)
    diff = os.path.join(directory, "🔺️diff", "🦀️.rs")
    source = open(diff if os.path.isfile(diff) else os.path.join(directory, "🦀️.rs"), encoding="utf-8").read()
    text = open(leaf, encoding="utf-8").read()
    descriptor = json.loads(text)
    classes = ["applied"] + (["no-op"] if '"mutation.no-op"' in source else []) + (["rejected"] if "MutationOutcome::error(" in source or "MutationOutcome::fatal(" in source else [])
    derived[descriptor["semanticKind"]] = classes
    if descriptor["outcomeClasses"] != classes:
        changed.append((descriptor["semanticKind"], descriptor["outcomeClasses"], classes))
        descriptor["outcomeClasses"] = classes
        open(leaf, "w", encoding="utf-8", newline="\n").write(json.dumps(descriptor, indent=2, ensure_ascii=False) + "\n")
oracle = json.load(open(oracle_path, encoding="utf-8"))
moved = 0
def walk(node):
    global moved
    if isinstance(node, dict):
        if node.get("id") in derived and "outcomes" in node and "productionDispatch" in node and node["outcomes"] != derived[node["id"]]:
            node["outcomes"] = derived[node["id"]]
            moved += 1
        for value in node.values():
            walk(value)
    elif isinstance(node, list):
        for value in node:
            walk(value)
walk(oracle)
open(oracle_path, "w", encoding="utf-8", newline="\n").write(json.dumps(oracle, indent=2, ensure_ascii=False) + "\n")
print("descriptors changed", len(changed), changed[:5], "oracle entries changed", moved)
