"""🔎️ Prints one JSON path of a catalog or candidate document, truncated, to study the contract section by section."""
import json
import sys

S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"


def main():
    source = sys.argv[1]
    path = sys.argv[2] if len(sys.argv) > 2 else ""
    limit = int(sys.argv[3]) if len(sys.argv) > 3 else 4000
    file = source if source.endswith(".json") else S + source + "/🔣️.json"
    node = json.load(open(file))
    for key in [k for k in path.split(".") if k]:
        node = node[int(key)] if isinstance(node, list) else node[key]
    if isinstance(node, dict) and limit < 0:
        for k, v in node.items():
            print(k, len(json.dumps(v)))
        return
    text = json.dumps(node, ensure_ascii=False, indent=None)
    print(text[: abs(limit)])
    print("[len]", len(text))


if __name__ == "__main__":
    main()
