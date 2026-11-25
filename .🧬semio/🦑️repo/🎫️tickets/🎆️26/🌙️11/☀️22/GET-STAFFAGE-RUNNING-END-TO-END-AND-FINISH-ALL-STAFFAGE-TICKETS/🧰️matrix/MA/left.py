"""🪶️ Takes up the parts `loft.ts` left as they are: left.py <id>… reads the `LEFT` lines of `$GEN/loft/<id>.build1.log` and, on the loft op of each in `loft/<id>.json`, names the one body solid that carries the id of the shape (when the bone has several) and raises `max` to MAX (default 1.5), so the part becomes the body of its own solid in the three views."""
import json, os, re, sys, pathlib
W = pathlib.Path(__file__).resolve().parent
GEN = W.parent.parent / "🗑️generated" / "matrix" / "MA"
for a in sys.argv[1:]:
    path = W / "loft" / f"{a}.json"
    ops = json.load(open(path))
    n = 0
    for scale, bone, shape, solids in re.findall(r"^1:(\d+) (\S+) (\S+): LEFT, .*; solids (\S*)$", (GEN / "loft" / f"{a}.build1.log").read_text(), re.M):
        for op in ops:
            if op["op"] == "loft" and op["scale"] == scale and op["bone"] == bone and op["id"] == shape:
                if shape in solids.split("+") and "+" in solids:
                    op["solids"] = [shape]
                op["max"] = float(os.environ.get("MAX", "1.5"))
                n += 1
    path.write_text("[\n" + ",\n".join(" " + json.dumps(op, ensure_ascii=False) for op in ops) + "\n]\n")
    print(a, n, "ops taken up")
