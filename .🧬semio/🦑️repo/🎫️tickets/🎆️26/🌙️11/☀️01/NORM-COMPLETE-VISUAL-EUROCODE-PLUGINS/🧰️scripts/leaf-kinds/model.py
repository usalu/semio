"""Reads an id-keyed artifact snapshot schema into entities and leaves (schema-first)."""
import json, re
from common import *


LIMIT = [255]
ABBREV = [("bolts-bolts", "bolts"), ("edge-stiffener", "stiffener"), ("transverse-weld", "t-weld"), ("reinforcement", "rebar"), ("thickness", "thk")]


def kind_limit():
    return LIMIT[0]


def shorten(kind, limit):
    for a, b in ABBREV:
        if len(kind) <= limit:
            break
        kind = kind.replace(a, b)
    assert len(kind) <= limit, "kind name too long: " + kind
    return kind


class Ent:
    def __init__(self, name, emoji, chain, en, de, slug=None):
        self.name, self.emoji, self.chain, self.en, self.de = name, emoji, chain, en, de
        self.slug = slug or name
        self.group_prefix = name

    @property
    def sels(self):
        return [c[2] for c in self.chain]

    @property
    def patch(self):
        return "".join(pascal(c[0]) for c in self.chain)

    @property
    def parent(self):
        return self.chain[:-1]


class Leaf:
    def __init__(self, ent, path, ty, ui, covered):
        self.ent, self.path, self.ty, self.ui, self.covered = ent, path, ty, ui, covered
        self.field = "_".join(path)
        self.access = ".".join(path)
        self.kind = shorten("change-%s-%s" % (ent.slug, kebab(self.field)), kind_limit())
        self.module = self.kind.replace("-", "_")
        self.record = pascal(self.kind)
        self.dirname = emoji_dir(ent.emoji, self.kind)
        self.new = "new_" + self.field
        self.newc = camel(self.new)

    @property
    def label(self):
        lab = (self.ui or {}).get("label") or {}
        if not lab.get("en") or not lab.get("de"):
            raise ValueError("schema field %s.%s has no EN + DE x-semio-ui.label; add both labels to the schema before generating its mutation kind" % (self.ent.name, self.field))
        return lab["en"], lab["de"]

    @property
    def rust_ty(self):
        return self.ty if not self.ty.startswith("enum:") else self.ty[5:]

    @property
    def copy(self):
        return self.ty in ("f64", "u32", "bool") or self.ty.startswith("enum:")


def leaves_of(defs, defname, skip=("id",)):
    out = []
    for name, p in defs[defname]["properties"].items():
        if name in skip or p.get("type") == "array":
            continue
        ref = p.get("$ref", "").split("/")[-1]
        if ref and "enum" in defs[ref]:
            out.append(([snake(name)], "enum:" + ref, p.get("x-semio-ui") or {}, defname))
        elif ref:
            for n2, p2 in defs[ref]["properties"].items():
                out.append(([snake(name), snake(n2)], scalar(p2), p2.get("x-semio-ui") or {}, ref))
        else:
            out.append(([snake(name)], scalar(p), p.get("x-semio-ui") or {}, defname))
    return out


def scalar(p):
    return {"number": "f64", "integer": "u32", "boolean": "bool", "string": "String"}[p["type"]]


def read_entities(art_schema_json, ents, covered):
    schema = json.loads(rd(art_schema_json))
    defs = schema["$defs"]
    leaves = []
    for e in ents:
        row_def = e.chain[-1][1]
        for path, ty, ui, owner in leaves_of(defs, row_def):
            leaves.append(Leaf(e, path, ty, ui, (e.name, "_".join(path)) in covered))
            leaves[-1].owner_def = owner
    return leaves, defs
