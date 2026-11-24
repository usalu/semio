"""🔦️ Prints every JSON path of an asset document at which a string value equals one of the given words, or an object has it as `id`, `anchor` or `of`, with the object (cut): where.py <asset id> <word>…"""
import glob, json, sys

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
a = json.load(open(glob.glob(f"{CATALOG}/*/{sys.argv[1]}/🔣️.json")[0]))["asset"]
words = set(sys.argv[2:])


def walk(node, path, parent):
    if isinstance(node, dict):
        for k, v in node.items():
            walk(v, f"{path}.{k}", node)
    elif isinstance(node, list):
        for i, v in enumerate(node):
            walk(v, f"{path}[{i}]", node)
    elif isinstance(node, str) and node in words:
        print(path, "=", node, "|", json.dumps(parent, ensure_ascii=False)[:int(sys.argv[0] and 420)])


walk(a, "asset", None)
