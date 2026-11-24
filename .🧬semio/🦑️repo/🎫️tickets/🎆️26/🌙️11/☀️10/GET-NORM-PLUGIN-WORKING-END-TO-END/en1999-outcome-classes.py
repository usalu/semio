import json, glob, os, sys
root = sys.argv[1]
changed = 0
for leaf in sorted(glob.glob(os.path.join(root, "*", "🔣️.json"))):
    directory = os.path.dirname(leaf)
    source = open(os.path.join(directory, "🔺️diff", "🦀️.rs"), encoding="utf-8").read()
    text = open(leaf, encoding="utf-8").read()
    descriptor = json.loads(text)
    assert json.dumps(descriptor, indent=2, ensure_ascii=False) + "\n" == text, leaf
    classes = ["applied"] + (["no-op"] if '"mutation.no-op"' in source else []) + (["rejected"] if "MutationOutcome::error(" in source or "MutationOutcome::fatal(" in source else [])
    if descriptor["outcomeClasses"] != classes:
        descriptor["outcomeClasses"] = classes
        open(leaf, "w", encoding="utf-8", newline="\n").write(json.dumps(descriptor, indent=2, ensure_ascii=False) + "\n")
        changed += 1
print(changed)
