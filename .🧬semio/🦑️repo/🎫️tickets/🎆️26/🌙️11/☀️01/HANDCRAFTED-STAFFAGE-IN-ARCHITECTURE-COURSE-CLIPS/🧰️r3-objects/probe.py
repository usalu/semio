import json, glob, sys

A = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"


def load(cat, aid):
    return json.load(open(f"{A}/{cat}/{aid}/🔣️.json", encoding="utf8"))["asset"]


def keys(o, depth=0, maxd=2):
    for k, v in o.items():
        t = type(v).__name__
        n = len(v) if hasattr(v, "__len__") else ""
        print("  " * depth + f"{k}: {t} {n}")
        if isinstance(v, dict) and depth < maxd:
            keys(v, depth + 1, maxd)


if __name__ == "__main__":
    a = load(sys.argv[1], sys.argv[2])
    keys(a, 0, int(sys.argv[3]) if len(sys.argv) > 3 else 1)
