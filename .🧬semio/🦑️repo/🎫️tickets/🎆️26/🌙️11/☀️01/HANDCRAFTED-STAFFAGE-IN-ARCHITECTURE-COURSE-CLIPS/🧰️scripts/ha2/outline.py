import json, sys

def walk(v, depth, path, out, maxdepth):
    pad = "  " * depth
    if isinstance(v, dict):
        for k, x in v.items():
            if isinstance(x, (dict, list)):
                n = len(x)
                out.append(f"{pad}{k}: {type(x).__name__}[{n}]")
                if depth < maxdepth:
                    walk(x, depth + 1, path + [k], out, maxdepth)
            else:
                s = json.dumps(x, ensure_ascii=False)
                if len(s) > 120:
                    s = s[:117] + "..."
                out.append(f"{pad}{k}: {s}")
    elif isinstance(v, list):
        for i, x in enumerate(v):
            if isinstance(x, (dict, list)):
                label = ""
                if isinstance(x, dict):
                    for key in ("id", "name", "kind", "role", "scale"):
                        if key in x and not isinstance(x[key], (dict, list)):
                            label += f" {key}={x[key]}"
                out.append(f"{pad}[{i}]{label}")
                if depth < maxdepth:
                    walk(x, depth + 1, path + [i], out, maxdepth)
            else:
                out.append(f"{pad}- {json.dumps(x, ensure_ascii=False)}")

f = sys.argv[1]
maxdepth = int(sys.argv[2]) if len(sys.argv) > 2 else 3
with open(f, encoding="utf-8") as fh:
    d = json.load(fh)
out = []
walk(d, 0, [], out, maxdepth)
print("\n".join(out))
