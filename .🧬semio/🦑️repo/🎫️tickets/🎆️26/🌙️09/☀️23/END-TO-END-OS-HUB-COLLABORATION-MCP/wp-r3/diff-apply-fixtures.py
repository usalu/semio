import json, os, sys
ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧫️fixtures"
def walk(a, b, path, out):
    if type(a) != type(b):
        out.append((path, "type", a, b)); return
    if isinstance(a, dict):
        for k in sorted(set(a) | set(b)):
            if k not in a: out.append((path + "." + k, "added", None, b[k]))
            elif k not in b: out.append((path + "." + k, "removed", a[k], None))
            else: walk(a[k], b[k], path + "." + k, out)
    elif isinstance(a, list):
        if len(a) != len(b):
            out.append((path, f"len {len(a)}->{len(b)}", None, None))
            for i, (x, y) in enumerate(zip(a, b)):
                if x != y: out.append((path + f"[{i}]", "first-diff", x, y)); break
            if len(b) > len(a): out.append((path + f"[{len(a)}:]", "tail", None, b[len(a):]))
            if len(a) > len(b): out.append((path + f"[{len(b)}:]", "tail-removed", a[len(b):], None))
        else:
            for i, (x, y) in enumerate(zip(a, b)): walk(x, y, path + f"[{i}]", out)
    elif a != b:
        out.append((path, "changed", a, b))
only = sys.argv[1] if len(sys.argv) > 1 else None
for d in sorted(os.listdir(ROOT)):
    p = os.path.join(ROOT, d)
    if not os.path.exists(os.path.join(p, "⬅️before.json")) or (only and only not in d): continue
    b = json.load(open(os.path.join(p, "⬅️before.json"))); a = json.load(open(os.path.join(p, "➡️after.json")))
    out = []; walk(b, a, "", out)
    print("=====", d)
    for row in out[:6]: print("  ", row[0], row[1], json.dumps(row[2])[:300] if row[2] is not None else "", "=>", json.dumps(row[3])[:600] if row[3] is not None else "")
