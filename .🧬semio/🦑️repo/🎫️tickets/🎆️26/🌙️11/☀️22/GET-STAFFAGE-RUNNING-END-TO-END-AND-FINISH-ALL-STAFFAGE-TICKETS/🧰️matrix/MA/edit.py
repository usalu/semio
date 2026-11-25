"""✏️ Sets fields on the loft ops of an asset: edit.py <id> '<scale regex>' '<bone regex>' '<shape id regex>' '<json fields>'  (a null value deletes the field; the fields {"drop": true} remove the ops)."""
import json, re, sys, pathlib
path = pathlib.Path(__file__).resolve().parent / "loft" / f"{sys.argv[1]}.json"
scale, bone, shape, fields = sys.argv[2], sys.argv[3], sys.argv[4], json.loads(sys.argv[5])
ops, out, n = json.load(open(path)), [], 0
for op in ops:
    if op.get("bone") is not None and re.fullmatch(scale, str(op.get("scale"))) and re.fullmatch(bone, op["bone"]) and re.fullmatch(shape, op["id"]):
        n += 1
        if fields.get("drop"):
            continue
        for key, value in fields.items():
            if value is None:
                op.pop(key, None)
            else:
                op[key] = value
    out.append(op)
path.write_text("[\n" + ",\n".join(" " + json.dumps(op, ensure_ascii=False) for op in out) + "\n]\n")
print(sys.argv[1], n, "ops")
