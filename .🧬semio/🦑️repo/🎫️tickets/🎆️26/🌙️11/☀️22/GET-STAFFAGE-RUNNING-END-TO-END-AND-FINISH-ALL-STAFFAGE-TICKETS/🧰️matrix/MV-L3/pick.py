"""✂️ A part of saved loft ops: `python3 pick.py <ops.json> <out.json> <item>…` with items `<scale>[/<view>]:<bone,bone|*|-bone,bone>` writes the ops of the named scales (and views; without a view all three) with the named bones only (`*` every bone, a leading `-` every bone but the named ones); what no item names is left out. The ops of `../MV-L1/loft.ts --save` are lists of shapes per scale and view and the guard reads every view by itself, so a part of them is applied by the same `--ops`."""
import json, sys
ops = json.load(open(sys.argv[1]))
out = {}
for item in sys.argv[3:]:
    where, names = item.split(":", 1)
    scale, _, only = where.partition("/")
    drop = names.startswith("-")
    wanted = set(names.lstrip("-").split(","))
    for view, shapes in ops[scale].items():
        if only in ("", view):
            out.setdefault(scale, {})[view] = [shape for shape in shapes if names == "*" or ((shape["bone"] in wanted) != drop)]
json.dump(out, open(sys.argv[2], "w"), ensure_ascii=False, separators=(",", ":"))
print("; ".join(f"{scale} {view}: {len(shapes)} lofts" for scale, views in out.items() for view, shapes in views.items()))
