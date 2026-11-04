import json, sys

def descend(tree, indices):
    node = tree
    for index in indices:
        node = node[int(index)]
    return node

def render(node):
    return "[" + ",".join(render(child) if isinstance(child, list) else child for child in node) + "]"

def after(case):
    tree = ["t", "p", ["t"]]
    if "refused" in case or case.get("expect") == "none":
        return None
    parts = case["path"].strip("/").split("/")
    walk = parts[3:]
    if case["op"] == "insert":
        slot = walk[-1]
        owner = descend(tree, [walk[i] for i in range(1, len(walk) - 1, 2)])
        owner.insert(len(owner) if slot == "-" else int(slot), "t")
    elif case["op"] == "remove":
        owner = descend(tree, [walk[i] for i in range(1, len(walk) - 2, 2)])
        del owner[int(walk[-1])]
    return render(tree)

def main(path, write):
    fixture = json.load(open(path, encoding="utf-8"))
    for case in fixture["cases"]:
        case["treeAfter"] = after(case)
    if write:
        json.dump(fixture, open(path, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
        open(path, "a").write("\n")
    print(len(fixture["cases"]), "cases computed by list operations")

if __name__ == "__main__":
    main(sys.argv[1], "--write" in sys.argv)
