import ast, sys
sys.stdout.reconfigure(encoding="utf-8")
p = "C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py"
src = open(p, encoding="utf-8").read()
tree = ast.parse(src)
fresh = {"_direction": "🧿", "_demand": "🧪", "_bar_values": "🔠", "_variant": "📋", "_conversion": "🪫"}
nodes = {n.name: n for n in ast.walk(tree) if isinstance(n, ast.FunctionDef)}
lines = src.split("\n")
for name, emoji in fresh.items():
    doc = nodes[name].body[0].value
    row = lines[doc.lineno - 1]
    at = row.index('"""') + 3
    first = row[at:].split(" ")[0]
    lines[doc.lineno - 1] = row[:at] + emoji + row[at + len(first):]
open(p + ".new", "w", encoding="utf-8").write("\n".join(lines))
