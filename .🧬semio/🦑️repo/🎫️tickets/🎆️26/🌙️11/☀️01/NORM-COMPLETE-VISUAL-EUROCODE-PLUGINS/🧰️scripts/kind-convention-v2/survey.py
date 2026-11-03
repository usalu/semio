"""Survey: for every edit rule of an artifact, which list rows addressed by index lack a stable `id` in the snapshot JSON schema."""
import json, re, sys, glob, os, collections
ART = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
SUB = "🏅️standards/🔖️1/🪆️subsets/✳️any/"
NAMES = {"en1991": "🏋️en1991", "en1994": "🧩️en1994", "en1996": "🪨️en1996", "en1998": "🫨️en1998"}

def load(a):
    base = os.path.join(ART, NAMES[a], SUB)
    s = json.load(open(base + "🧬️schema/📸️snapshot/🔣️.json"))
    rules = open(base + "🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs").read()
    return base, s, rules

def resolve(s, node):
    while True:
        if "$ref" in node:
            node = s["$defs"][node["$ref"].split("/")[-1]] if "$defs" in s else s["definitions"][node["$ref"].split("/")[-1]]
        elif "anyOf" in node:
            c = [x for x in node["anyOf"] if x.get("type") != "null"]
            node = c[0]
        else:
            return node

def rowof(s, path):
    node = resolve(s, s)
    for seg in path.split("."):
        name = seg.replace("[]", "")
        node = resolve(s, node["properties"][name])
        if node.get("type") == "array":
            node = resolve(s, node["items"])
    return node


def colls(kind, path):
    if kind != "SetFieldRule":
        return [path]
    parts = path.split("[]")
    out, acc = [], ""
    for p in parts[:-1]:
        acc += p
        out.append(acc)
        acc += "[]"
    return out


if __name__ == "__main__":
    for a in NAMES:
        base, s, rules = load(a)
        seen = {}
        cnt = collections.Counter()
        for m in re.finditer(r'(SetFieldRule|InsertItemRule|RemoveItemRule) \{ path: "([^"]+)", kind: "([^"]+)", selectors: &\[([^\]]*)\]', rules):
            kind, path, k, sel = m.groups()
            cnt[kind] += 1
            for c in colls(kind, path):
                cp = c
                try:
                    row = rowof(s, cp)
                    seen[c] = ("id" in row["properties"]) if row else None
                except Exception as e:
                    seen[c] = "ERR"
        print(a, dict(cnt))
        for c, v in seen.items():
            print("   ", c, v)
