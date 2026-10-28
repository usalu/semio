import ast, sys, collections
sys.stdout.reconfigure(encoding="utf-8")
p = "C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py"
src = open(p, encoding="utf-8").read()
tree = ast.parse(src)
fresh = {"_price_row": "🪙", "_phase_formulate": "🛋️", "_swap": "🔀"}
nodes = {n.name: n for n in ast.walk(tree) if isinstance(n, ast.FunctionDef)}
lines = src.split("\n")
for name, emoji in fresh.items():
    node = nodes[name]
    doc = node.body[0].value
    row = lines[doc.lineno - 1]
    at = row.index('"""') + 3
    first = row[at:].split(" ")[0]
    lines[doc.lineno - 1] = row[:at] + emoji + row[at + len(first):]
open(p + ".new", "w", encoding="utf-8").write("\n".join(lines))
print("[DEBUG] written")
