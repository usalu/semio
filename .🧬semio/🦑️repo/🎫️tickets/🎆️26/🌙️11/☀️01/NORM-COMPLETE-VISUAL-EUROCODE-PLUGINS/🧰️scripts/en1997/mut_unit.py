import sys
sys.path.insert(0, ".")
from spec import *
from rows import *
import sys as _s
for p in [MUT + "/🧪️tests/🔬️unit/🦀️.rs", S + "/🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs", S + "/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]:
    s = open(p, encoding="utf8").read()
    if "InsertWall" not in s:
        lets = "".join(f"    let {r.item}_row = demo.{r.coll}[0].clone();\n" for r in ROWS if r.item not in ("layer", "footing", "pile"))
        s = s.replace("    let pile = demo.piles[0].clone();\n", "    let pile = demo.piles[0].clone();\n" + lets, 1)
        def entry(r, v):
            st, m = struct(r, v), mod(r, v)
            var = {"layer": "layer", "footing": "footing", "pile": "pile"}.get(r.item, f"{r.item}_row")
            if v == "insert": return f"        En1997Mutation::{st}({m}::{st} {{ index: 0, {r.item}: {var}.clone() }}),\n"
            if v == "remove": return f"        En1997Mutation::{st}({m}::{st} {{ index: 0 }}),\n"
            return f"        En1997Mutation::{st}({m}::{st} {{ {r.item}: {var}.clone() }}),\n"
        # existing entries consume layer/footing/pile by move; clone for the new ones
        s = s.replace("InsertLayer(insert_layer::InsertLayer { index: 0, layer })", "InsertLayer(insert_layer::InsertLayer { index: 0, layer: layer.clone() })")
        s = s.replace("InsertFooting(insert_footing::InsertFooting { index: 0, footing })", "InsertFooting(insert_footing::InsertFooting { index: 0, footing: footing.clone() })")
        s = s.replace("InsertPile(insert_pile::InsertPile { index: 0, pile })", "InsertPile(insert_pile::InsertPile { index: 0, pile: pile.clone() })")
        s = s.replace("        En1997Mutation::RemovePile(remove_pile::RemovePile { index: 0 }),\n    ]", "        En1997Mutation::RemovePile(remove_pile::RemovePile { index: 0 }),\n" + "".join(entry(r, v) for r, v in newkinds()) + "    ]", 1)
        open(p, "w", encoding="utf8").write(s)
    print("ok")
