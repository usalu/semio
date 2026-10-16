"""📐️ Plans the split of the energy mutation Python adapter by kind groups and prints the module sizes (dry run, writes nothing)."""
import ast, re, sys, json, collections

F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
O = F + "/🗑️generated/pfb-orig/energy-test/"
src = open(O + "🐍️.py", encoding="utf-8").read()
lines = src.split("\n")
rust = open(O + "🦀️.rs", encoding="utf-8").read().split("\n")
tree = ast.parse(src)

order, ident_kind = [], {}
for index, line in enumerate(rust):
    match = re.match(r'\s+id: "([a-z0-9-]+)",', line)
    if match:
        kind = re.match(r'\s+kind: "([a-z0-9-]+)",', rust[index + 1]).group(1)
        ident_kind[match.group(1)] = kind
        if kind not in order:
            order.append(kind)
cuts = [int(x) for x in sys.argv[1:]]
def group_of(kind):
    return sum(1 for c in cuts if order.index(kind) >= c)
snake = {k.replace("-", "_"): k for k in order}

nodes = []
for i, node in enumerate(tree.body):
    nodes.append(dict(node=node, start=node.lineno, end=node.end_lineno))
for entry in nodes:
    s = entry["start"]
    while s > 1 and lines[s - 2].startswith("#") and not re.match(r"#\s?(end)?region", lines[s - 2]):
        s -= 1
    entry["start"] = s
# attach docstring expression nodes to the previous node
merged = []
for entry in nodes:
    n = entry["node"]
    if isinstance(n, ast.Expr) and isinstance(getattr(n, "value", None), ast.Constant) and isinstance(n.value.value, str) and merged and entry["start"] > 1:
        merged[-1]["end"] = entry["end"]
        continue
    merged.append(entry)
nodes = merged

def names_of(n):
    if isinstance(n, (ast.FunctionDef, ast.ClassDef)):
        return [n.name]
    if isinstance(n, ast.Assign):
        return [ast.unparse(t) for t in n.targets]
    return []

defs = {}
for entry in nodes:
    for name in names_of(entry["node"]):
        defs[name] = entry
print("nodes", len(nodes), "defs", len(defs))

group_nodes = collections.defaultdict(list)
helpers = []
for entry in nodes:
    n = entry["node"]
    names = names_of(n)
    if not names or not isinstance(n, ast.FunctionDef):
        continue
    base = names[0][len("_invert_"):] if names[0].startswith("_invert_") else names[0]
    if base in snake:
        entry["group"] = group_of(snake[base])
        group_nodes[entry["group"]].append(entry)
    else:
        helpers.append(entry)
sizes = {g: sum(e["end"] - e["start"] + 2 for e in v) for g, v in sorted(group_nodes.items())}
rows_vocab = collections.Counter()
for g in sizes:
    rows_vocab[g] = sum(1 for k in order if group_of(k) == g)
print("groups (kind counts):", {g: rows_vocab[g] for g in sorted(sizes)})
print("function lines per group:", sizes)
print("helpers:", [(names_of(h["node"])[0], h["start"], h["end"]) for h in helpers])
