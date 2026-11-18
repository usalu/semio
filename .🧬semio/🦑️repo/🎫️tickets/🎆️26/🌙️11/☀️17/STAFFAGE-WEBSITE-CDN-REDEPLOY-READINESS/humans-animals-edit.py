"""Byte-stable load/save of a catalog asset (keeps the authored number spelling) for the humans and animals repairs."""
import json
from json import encoder

CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"

class N(float):
    def __new__(cls, raw):
        o = float.__new__(cls, raw); o.raw = raw; return o

def load(slug):
    with open(CAT + slug + "/🔣️.json", encoding="utf8") as f:
        return json.loads(f.read(), parse_float=N)

INLINE = {"at", "radii", "rotation", "size", "tags", "translation", "p0", "p1", "origin", "axis", "scale", "axisX"}

def _scalar(v):
    if isinstance(v, N): return v.raw
    if isinstance(v, float): return repr(v)
    return json.dumps(v, ensure_ascii=False)

def _write(v, ind, key, out):
    if isinstance(v, dict):
        if not v: out.append("{}"); return
        out.append("{\n")
        items = list(v.items())
        for i, (k, x) in enumerate(items):
            out.append(ind + "  " + json.dumps(k, ensure_ascii=False) + ": ")
            _write(x, ind + "  ", k, out)
            out.append(",\n" if i < len(items) - 1 else "\n")
        out.append(ind + "}")
    elif isinstance(v, list):
        if not v: out.append("[]"); return
        if key in INLINE and not (key == "tags" and len(ind) <= 4) and all(not isinstance(x, (list, dict)) for x in v):
            out.append("[" + ", ".join(_scalar(x) for x in v) + "]"); return
        out.append("[\n")
        for i, x in enumerate(v):
            out.append(ind + "  ")
            _write(x, ind + "  ", None, out)
            out.append(",\n" if i < len(v) - 1 else "\n")
        out.append(ind + "]")
    else:
        out.append(_scalar(v))

def dumps(doc):
    out = []
    _write(doc, "", None, out)
    return "".join(out) + "\n"

def _blocks(text):
    lines = text.split("\n")
    heads = [i for i, l in enumerate(lines) if l.startswith('    "') and l.rstrip().endswith(("{", "[", ",")) and not l.startswith('     ') and '": ' in l]
    out = {}
    for n, i in enumerate(heads):
        end = heads[n + 1] if n + 1 < len(heads) else len(lines)
        out[lines[i].split('"')[1]] = "\n".join(lines[i:end])
    return out

def save(slug, doc, original):
    """Splices only the top-level asset members that changed into the original text, keeping its hand-formatted arrays elsewhere."""
    path = CAT + slug + "/🔣️.json"
    text = open(path, encoding="utf8").read()
    old, new = _blocks(dumps(original)), _blocks(dumps(doc))
    for key in new:
        if old.get(key) == new[key]: continue
        cur = _blocks(text)[key]
        text = text.replace(cur, new[key], 1)
    open(path, "w", encoding="utf8").write(text)

def loadpair(slug):
    return load(slug), load(slug)

def num(x):
    return N(repr(round(x, 5)))

if __name__ == "__main__":
    import sys
    for slug in sys.argv[1:]:
        s = open(CAT + slug + "/🔣️.json", encoding="utf8").read()
        print(slug, dumps(load(slug)) == s)
