"""🔬️ Lists the paths at which two JSON documents differ (lists of objects with ids are matched by id), with both values shortened. Usage: `python3 json-diff.py <a.json> <b.json> [max lines]`."""
import json
import sys


def walk(a, b, path, out):
    """🚶️ Collects the differing paths of two JSON values."""
    if isinstance(a, dict) and isinstance(b, dict):
        for k in sorted(set(a) | set(b)):
            if k not in a:
                out.append(f"{path}.{k}: only b {json.dumps(b[k], ensure_ascii=False)[:160]}")
            elif k not in b:
                out.append(f"{path}.{k}: only a {json.dumps(a[k], ensure_ascii=False)[:160]}")
            else:
                walk(a[k], b[k], f"{path}.{k}", out)
    elif isinstance(a, list) and isinstance(b, list):
        if all(isinstance(x, dict) and "id" in x for x in a + b) and a + b:
            ia, ib = {x["id"]: x for x in a}, {x["id"]: x for x in b}
            for k in list(dict.fromkeys(list(ia) + list(ib))):
                if k not in ia:
                    out.append(f"{path}[{k}]: only b {json.dumps(ib[k], ensure_ascii=False)[:160]}")
                elif k not in ib:
                    out.append(f"{path}[{k}]: only a {json.dumps(ia[k], ensure_ascii=False)[:160]}")
                else:
                    walk(ia[k], ib[k], f"{path}[{k}]", out)
            if [x["id"] for x in a if x["id"] in ib] != [x["id"] for x in b if x["id"] in ia]:
                out.append(f"{path}: order of ids differs")
        elif len(a) != len(b):
            out.append(f"{path}: length {len(a)} vs {len(b)}: {json.dumps(a, ensure_ascii=False)[:120]} | {json.dumps(b, ensure_ascii=False)[:120]}")
        else:
            for i, (x, y) in enumerate(zip(a, b)):
                walk(x, y, f"{path}[{i}]", out)
    elif a != b:
        out.append(f"{path}: {json.dumps(a, ensure_ascii=False)[:160]} | {json.dumps(b, ensure_ascii=False)[:160]}")


if __name__ == "__main__":
    lines = []
    walk(json.load(open(sys.argv[1])), json.load(open(sys.argv[2])), "", lines)
    limit = int(sys.argv[3]) if len(sys.argv) > 3 else 60
    print("\n".join(lines[:limit]))
    print(f"{len(lines)} differing paths")
