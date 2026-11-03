"""Fork-local: the full-catalogue demo document (one seeded case per catalogue entry), mirroring `design_case::seed` for number and text rows."""
import json, re
from lib import *


def entries(rust_text):
    out = []
    for m in re.finditer(r'Entry \{ id: "([^"]+)", (?:part|standard): "([^"]*)", clause: "([^"]*)", en: "((?:[^"\\]|\\.)*)", de: "((?:[^"\\]|\\.)*)", inputs: &\[([^\]]*)\]', rust_text):
        out.append({"id": m.group(1), "en": json.loads('"%s"' % m.group(4)), "de": json.loads('"%s"' % m.group(5)), "inputs": re.findall(r'"([^"]+)"', m.group(6))})
    return out


def spec(vocab, entry_id, key):
    rows = [r for r in vocab["inputs"] if r["key"] == key]
    scoped = [r for r in rows if entry_id in [s.strip() for s in r.get("scope", "").split(",") if s.strip()]]
    return (scoped or [r for r in rows if not r.get("scope")] or rows or [None])[0]


def seed_cases(vocab, ents, id_fn):
    cases = []
    for e in ents:
        numbers, texts = [], []
        for key in e["inputs"]:
            r = spec(vocab, e["id"], key)
            if not r or "example" not in r:
                continue
            if r["kind"] == "number" and isinstance(r["example"], (int, float)):
                numbers.append({"id": key, "value": float(r["example"])})
            elif r["kind"] != "number" and isinstance(r["example"], str):
                texts.append({"id": key, "value": r["example"]})
        cases.append({"id": id_fn(e["id"]), "labelEn": e["en"], "labelDe": e["de"], "entryId": e["id"], "numbers": numbers, "texts": texts})
    return cases


def demo_id(entry_id):
    return "case-" + "".join(c if c.isascii() and c.isalnum() else "-" for c in entry_id.lower())
