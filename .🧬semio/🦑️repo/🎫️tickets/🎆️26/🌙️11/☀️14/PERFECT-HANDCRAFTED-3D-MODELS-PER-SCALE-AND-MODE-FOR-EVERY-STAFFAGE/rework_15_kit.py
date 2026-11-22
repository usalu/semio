"""🧰 Rework 15 shared helpers: load a catalog asset with its volumes source, write the candidate document, and write the changed root members and the source back (re-reading the catalog file right before the write).

Usage from a `rework-15-<slug>-*.py` script: `kit = Kit("objects/<slug>")`, edit `kit.asset` and `kit.volumes`, then `kit.finish(sys.argv)`: without flags it writes the candidate `🗑️generated/rework-15-<slug>/🔣️.json`, `--apply` also writes the source and the catalog root members.
"""
import glob
import json
import math
import os
import re

T = os.path.dirname(os.path.abspath(__file__))
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
SCALES = ("50", "100", "200", "500")
VIEWS = ("elevation", "front", "plan")


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(i) for i in value) or (all(isinstance(i, list) and all(scalar(x) for x in i) for i in value) and len(json.dumps(value)) <= 160)
    return False


def dump(value, depth):
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(k, ensure_ascii=False) + ": " + dump(v, depth + 1) for k, v in value.items()) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(v, depth + 1) for v in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def member_range(text, key):
    found = re.search(r'\n    "' + key + r'"\s*:\s*', text)
    depth, i, quoted = 0, found.end(), False
    while True:
        c = text[i]
        if quoted:
            if c == "\\":
                i += 1
            elif c == '"':
                quoted = False
        elif c == '"':
            quoted = True
        elif c in "{[":
            depth += 1
        elif c in "}]":
            depth -= 1
            if depth == 0:
                return found.end(), i + 1
        elif depth == 0 and c in ",\n":
            return found.end(), i
        i += 1


def rnd(value, digits=5):
    if isinstance(value, float):
        return round(value + 0.0, digits)
    if isinstance(value, list):
        return [rnd(v, digits) for v in value]
    if isinstance(value, dict):
        return {k: rnd(v, digits) for k, v in value.items()}
    return value


def text(en, de):
    return {"en": en, "de": de}


class Kit:
    def __init__(self, spec):
        self.category, self.slug = spec.split("/")
        self.catalog = glob.glob("%s/🗂️catalog/*%s/%s/🔣️.json" % (S, self.category, self.slug))[0]
        self.source = os.path.join(T, "volumes", self.category, self.slug + ".json")
        self.text = open(self.catalog, encoding="utf-8").read()
        self.document = json.loads(self.text)
        self.asset = self.document["asset"]
        self.before = json.loads(self.text)["asset"]
        self.volumes = json.load(open(self.source, encoding="utf-8"))["volumes"]
        self.asset["volumes"] = self.volumes

    def bone(self, id):
        return next(b for b in self.asset["rig"]["bones"] if b["id"] == id)

    def add_bone(self, after, bone):
        bones = self.asset["rig"]["bones"]
        bones[:] = [b for b in bones if b["id"] != bone["id"]]
        bones.insert(next(i for i, b in enumerate(bones) if b["id"] == after) + 1, bone)

    def solids(self, scale, list="body"):
        return self.volumes[str(scale)][list]

    def solid(self, scale, id, list="body"):
        return next((s for s in self.solids(scale, list) if s["id"] == id), None)

    def drop(self, scale, ids, lists=("body", "thermal", "structure")):
        """🗑️ Removes solids by id from the body and every reference to them from the other lists."""
        gone = set(ids)
        v = self.volumes[str(scale)]
        for name in lists:
            v[name][:] = [s for s in v[name] if s["id"] not in gone and s.get("of") not in gone]

    def put(self, scale, solid, region=None, member=None, part="member", after=None, layer=None):
        """🧱 Adds or replaces a body solid and its thermal and structural references (`th-<id>`, `st-<id>`)."""
        v = self.volumes[str(scale)]
        solid = rnd(solid)
        old = next((i for i, s in enumerate(v["body"]) if s["id"] == solid["id"]), None)
        if old is not None:
            v["body"][old] = solid
        elif after and any(s["id"] == after for s in v["body"]):
            v["body"].insert(next(i for i, s in enumerate(v["body"]) if s["id"] == after) + 1, solid)
        else:
            v["body"].append(solid)
        if region and not any(s.get("of") == solid["id"] for s in v["thermal"]):
            ref = {"id": "th-" + solid["id"], "of": solid["id"], "region": region}
            if layer:
                ref["layer"] = layer
            v["thermal"].append(ref)
        if member and not any(s.get("of") == solid["id"] for s in v["structure"]):
            v["structure"].append({"id": "st-" + solid["id"], "of": solid["id"], "member": member, "part": part})
        return solid

    def shapes(self, scale, view):
        return self.asset["representations"][str(scale)]["geometry"][view]

    def shape(self, scale, view, id):
        return next((s for s in self.shapes(scale, view) if s["id"] == id), None)

    def put_shape(self, scale, view, shape, after=None):
        shapes = self.shapes(scale, view)
        shape = rnd(shape)
        old = next((i for i, s in enumerate(shapes) if s["id"] == shape["id"]), None)
        if old is not None:
            shapes[old] = shape
        elif after and any(s["id"] == after for s in shapes):
            shapes.insert(next(i for i, s in enumerate(shapes) if s["id"] == after) + 1, shape)
        else:
            shapes.append(shape)

    def drop_shapes(self, scale, view, ids):
        shapes = self.shapes(scale, view)
        shapes[:] = [s for s in shapes if s["id"] not in set(ids)]

    def decide(self, scale, en, de):
        """📝 Appends one bilingual decision to the volume of a scale (once)."""
        decisions = self.volumes[str(scale)]["decisions"]
        if not any(d["en"] == en for d in decisions):
            decisions.append(text(en, de))

    def finish(self, argv):
        out = os.path.join(T, "🗑️generated", "rework-15-" + self.slug)
        os.makedirs(out, exist_ok=True)
        with open(os.path.join(out, "🔣️.json"), "w", encoding="utf-8") as f:
            f.write(dump(self.document, 0) + "\n")
        print("candidate", os.path.join(out, "🔣️.json"))
        if "--apply" not in argv:
            return
        with open(self.source, "w", encoding="utf-8") as f:
            f.write(dump({"volumes": self.volumes}, 0) + "\n")
        fresh = open(self.catalog, encoding="utf-8").read()
        if fresh != self.text:
            raise SystemExit("catalog changed while building; rerun")
        changed = [k for k in self.asset if k != "volumes" and self.asset[k] != self.before.get(k)]
        result = fresh
        for key in changed:
            a, b = member_range(result, key)
            result = result[:a] + dump(self.asset[key], 2) + result[b:]
        check = json.loads(result)["asset"]
        for key in check:
            if key != "volumes":
                assert check[key] == self.asset[key], key
        open(self.catalog, "w", encoding="utf-8").write(result)
        print("source written; catalog members written:", ", ".join(changed) or "none")
