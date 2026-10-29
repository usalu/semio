"""Line-preserving editor for the hand-formatted animals catalog (one object per line)."""
import json, re
F = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/🔣️.json"

def inline(v, spaced):
    if isinstance(v, dict):
        if not v: return "{}"
        body = ", ".join(json.dumps(k, ensure_ascii=False) + ": " + inline(x, spaced) for k, x in v.items())
        return "{ " + body + " }" if spaced else "{" + body + "}"
    if isinstance(v, list): return "[" + ", ".join(inline(x, spaced) for x in v) + "]"
    return json.dumps(v, ensure_ascii=False)

class Doc:
    def __init__(self):
        self.lines = open(F).read().split("\n")
    def save(self):
        open(F, "w").write("\n".join(self.lines))
        json.loads("\n".join(self.lines))
    def items(self):
        asset = section = None
        for i, l in enumerate(self.lines):
            m = re.match(r'^      "id": "([^"]+)",$', l)
            if m: asset = m.group(1)
            m = re.match(r'^      "(\w+)": [\[{]', l)
            if m: section = m.group(1)
            s = l.strip()
            if s.startswith("{") and (s.endswith("},") or s.endswith("}")) and len(l) - len(l.lstrip()) >= 8:
                try: obj = json.loads(s.rstrip(","))
                except Exception: continue
                yield i, asset, section, obj
    def edit(self, asset, section, pred, fn):
        n = 0; out = []
        for i, a, sec, obj in list(self.items()):
            if a != asset or (section and sec != section) or not pred(obj): continue
            l = self.lines[i]; ind = l[: len(l) - len(l.lstrip())]; comma = "," if l.rstrip().endswith(",") else ""
            spaced = l.lstrip().startswith("{ ")
            res = fn(obj)
            if res is None: self.lines[i] = None
            elif isinstance(res, list): self.lines[i] = "\n".join(ind + inline(o, spaced) + ("," if j < len(res) - 1 or comma else "") for j, o in enumerate(res))
            else: self.lines[i] = ind + inline(res, spaced) + comma
            n += 1
        self.lines = [x for x in self.lines if x is not None]
        # flatten multi-line replacements
        self.lines = "\n".join(self.lines).split("\n")
        return n
    def get(self, asset, section, pred):
        return [o for i, a, sec, o in self.items() if a == asset and (not section or sec == section) and pred(o)]

def set_clip_keys(doc, asset, clip_id, extra, big=12):
    """Replace the override keys (more than `big` bones) of a clip by `extra`, keeping all other keys; handles one-line and multi-line clips."""
    L = doc.lines; cur = None; sec = None
    for i, l in enumerate(L):
        m = re.match(r'^      "id": "([^"]+)",$', l)
        if m: cur = m.group(1)
        m = re.match(r'^      "(\w+)": [\[{]', l)
        if m: sec = m.group(1)
        if cur != asset or sec != "clips": continue
        s = l.strip()
        if s.startswith("{") and f'"id": "{clip_id}"' in s[:40] and (s.endswith("},") or s.endswith("}")):
            comma = "," if l.rstrip().endswith(",") else ""; ind = l[: len(l) - len(l.lstrip())]
            c = json.loads(s.rstrip(","))
            c["keys"] = sorted([k for k in c["keys"] if not (k.get("bones") and len(k["bones"]) > big)] + extra, key=lambda k: k["t"])
            L[i] = ind + inline(c, s.startswith("{ ")) + comma
            return True
        if s == f'"id": "{clip_id}",' and L[i - 1].strip() == "{":
            j = i
            while L[j].strip() != '"keys": [': j += 1
            k0 = j + 1; k1 = k0
            while L[k1].strip() not in ("]", "],"): k1 += 1
            ind = L[k0][: len(L[k0]) - len(L[k0].lstrip())]; spaced = L[k0].lstrip().startswith("{ ")
            keys = []
            for t in L[k0:k1]: keys.append(json.loads(t.strip().rstrip(",")))
            keys = sorted([k for k in keys if not (k.get("bones") and len(k["bones"]) > big)] + extra, key=lambda k: k["t"])
            L[k0:k1] = [ind + inline(k, spaced) + ("," if n < len(keys) - 1 else "") for n, k in enumerate(keys)]
            return True
    return False
