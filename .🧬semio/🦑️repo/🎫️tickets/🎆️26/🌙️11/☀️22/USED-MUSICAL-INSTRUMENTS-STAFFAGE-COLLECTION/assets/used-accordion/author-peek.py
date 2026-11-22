"""🔍️ Prints a compact outline of a catalog asset document: section sizes, rig, poses, clips and a chosen sub-path."""
import json, sys, os
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
def find(slug):
    for c in os.listdir(S):
        p = os.path.join(S, c, slug, "🔣️.json")
        if os.path.exists(p):
            return p
    return slug
def main():
    doc = json.load(open(find(sys.argv[1]), encoding="utf-8"))
    a = doc["asset"]
    if len(sys.argv) == 2:
        print(doc.get("$schema"), doc.get("version"))
        for k, v in a.items():
            print(k, len(json.dumps(v, ensure_ascii=False)))
        return
    node = a
    for part in sys.argv[2].split("/"):
        node = node[int(part)] if isinstance(node, list) else node[part]
    limit = int(sys.argv[3]) if len(sys.argv) > 3 else 4000
    print(json.dumps(node, ensure_ascii=False, indent=1)[:limit])
main()
