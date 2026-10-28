from pathlib import Path
ticket=Path(__file__).parent
path=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean/🟦️.ts"
source=path.read_text(encoding="utf8")
a="const pending=[{node:this,polygons,depth:0}]"
b="const pending:{node:Node;polygons:Polygon[];depth:number}[]=[{node:this,polygons,depth:0}]"
assert source.count(a)==1
source=source.replace(a,b)
a="const pending=[{node:this,depth:0}]"
b="const pending:{node:Node;depth:number}[]=[{node:this,depth:0}]"
assert source.count(a)==3
source=source.replace(a,b)
source=source.replace("/** 🧊️ Apply a regularized Boolean operation through the shared private step evaluator. */","/** 🧮️ Apply a regularized Boolean operation through the shared private step evaluator. */")
path.write_text(source,encoding="utf8")
print("[DEBUG] explicit owned Node frame types retain real strict TypeScript validation")

