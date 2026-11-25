import glob, json, os, re, sys
root, log = sys.argv[1:3]
failing = set(re.findall(r"^([a-z0-9-]+): the descriptor declares", open(log, encoding="utf-8", errors="replace").read(), re.M))
changed = 0
for leaf in sorted(glob.glob(os.path.join(root, "*", "🔣️.json"))):
    directory = os.path.dirname(leaf)
    text = open(leaf, encoding="utf-8").read()
    descriptor = json.loads(text)
    if descriptor.get("semanticKind") not in failing:
        continue
    assert json.dumps(descriptor, indent=2, ensure_ascii=False) + "\n" == text, leaf
    source = open(os.path.join(directory, "🔺️diff", "🦀️.rs"), encoding="utf-8").read()
    classes = ["applied"] + (["no-op"] if '"mutation.no-op"' in source else []) + (["rejected"] if "MutationOutcome::error(" in source or "MutationOutcome::fatal(" in source else [])
    descriptor["outcomeClasses"] = classes
    open(leaf, "w", encoding="utf-8", newline="\n").write(json.dumps(descriptor, indent=2, ensure_ascii=False) + "\n")
    changed += 1
print(changed, len(failing))
