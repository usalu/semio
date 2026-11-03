import json, sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf8")
p = sys.argv[1]
d = json.load(open(p, encoding="utf8"))
a = d["asset"]


def sh(o, depth=0, maxd=2):
    if isinstance(o, dict):
        for k, v in o.items():
            if isinstance(v, (dict, list)) and depth < maxd:
                print("  " * depth + k + (" [%d]" % len(v) if isinstance(v, list) else ""))
                sh(v, depth + 1, maxd)
            else:
                print("  " * depth + k + ": " + json.dumps(v, ensure_ascii=False)[:110])
    elif isinstance(o, list) and o and depth < maxd:
        sh(o[0], depth + 1, maxd)


if len(sys.argv) > 2:
    for part in sys.argv[2].split("."):
        a = a[part]
    sh(a, 0, int(sys.argv[3]) if len(sys.argv) > 3 else 2)
else:
    sh(a, 0, 2)
