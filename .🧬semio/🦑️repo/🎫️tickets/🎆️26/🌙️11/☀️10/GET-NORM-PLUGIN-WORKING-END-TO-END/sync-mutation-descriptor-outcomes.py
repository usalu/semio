import json, os, re, sys
log, mutations, oracle = sys.argv[1:4]
reached = {}
for line in open(log, encoding="utf-8", errors="replace"):
    m = re.match(r'^([a-z0-9-]+): the descriptor declares \{[^}]*\}, production dispatch reaches \{([^}]*)\} from the committed vectors', line)
    if m:
        reached[m.group(1)] = [c.strip().strip('"') for c in m.group(2).split(",")]
order = ["applied", "no-op", "rejected"]
leaves = {}
for name in os.listdir(mutations):
    path = os.path.join(mutations, name, "🔣️.json")
    if os.path.isfile(path):
        leaves[json.load(open(path, encoding="utf-8"))["semanticKind"]] = path
changed = 0
for kind, classes in reached.items():
    classes = sorted(classes, key=order.index)
    path = leaves[kind]
    text = open(path, encoding="utf-8").read()
    doc = json.loads(text)
    doc["outcomeClasses"] = classes
    open(path, "w", encoding="utf-8", newline="\n").write(json.dumps(doc, indent=2, ensure_ascii=False) + "\n")
    changed += 1
print("descriptors", changed)
if oracle != "-":
    doc = json.load(open(oracle, encoding="utf-8"))
    n = 0
    def walk(o):
        global n
        if isinstance(o, dict):
            if o.get("id") in reached and "outcomes" in o and "productionDispatch" in o:
                want = sorted(reached[o["id"]], key=order.index)
                if o["outcomes"] != want:
                    o["outcomes"] = want; n += 1
            for v in o.values(): walk(v)
        elif isinstance(o, list):
            for v in o: walk(v)
    walk(doc)
    print("oracle outcomes changed", n)
    open(oracle, "w", encoding="utf-8", newline="\n").write(json.dumps(doc, indent=2, ensure_ascii=False) + "\n")
