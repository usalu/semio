"""🔍 Peek into a staffage asset document section by section.

See `S/README.md` for the contract.
"""
import json, sys

def main():
    path, *keys = sys.argv[1:]
    doc = json.load(open(path))
    node = doc
    for k in keys:
        node = node[int(k)] if isinstance(node, list) else node[k]
    if isinstance(node, dict):
        for k, v in node.items():
            s = json.dumps(v, ensure_ascii=False)
            print(f"{k}: {len(s)} {s[:300]}")
    elif isinstance(node, list):
        print(f"list {len(node)}")
        for i, v in enumerate(node[:400]):
            s = json.dumps(v, ensure_ascii=False)
            print(f"[{i}] {s[:400]}")
    else:
        print(node)

if __name__ == "__main__":
    main()
