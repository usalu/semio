"""Fixtures of the case collection: `cases: []` in every snapshot document, the empty case delta in every diff document and the apply vectors of the seven case kinds."""
import json, os, re
from lib import *
import lib2
import jsonspan as J

EMPTY_DELTA = {"removed": [], "inserted": [], "moved": [], "modified": []}
SKIP = ("target", "node_modules", "📦️packages", "🧬️schema", "🗑️generated", "__pycache__")


def walk_json(art):
    for dp, dn, fn in os.walk(art.any):
        dn[:] = [x for x in dn if x not in SKIP and x != "🖼️assets"]
        for f in fn:
            if f.endswith(".json"):
                yield os.path.join(dp, f)


def root_members(t):
    _, node = J.parse(t)
    return node if node["kind"] == "object" else None


def append_root_member(t, key, value):
    node = root_members(t)
    last = node["members"][node["order"][-1]]
    pos = last["span"][1]
    multiline = "\n" in t[node["span"][0]:node["span"][1]]
    if multiline:
        ls = t.rfind("\n", 0, t.index(json.dumps(node["order"][-1]), node["span"][0])) + 1
        ind = re.match(r"[ \t]*", t[ls:]).group(0)
        body = json.dumps(value, indent=2, ensure_ascii=False).replace("\n", "\n" + ind) if not (isinstance(value, dict) and not value) and value != [] else json.dumps(value)
        return t[:pos] + ",\n" + ind + json.dumps(key) + ": " + body + t[pos:]
    sep = ", " if ", " in t[node["span"][0]:node["span"][0] + 200] else ","
    colon = ": " if sep == ", " else ":"
    return t[:pos] + sep + json.dumps(key) + colon + json.dumps(value, ensure_ascii=False, separators=(",", ":") if sep == "," else None) + t[pos:]


def classify(t, snapshot_marker):
    try:
        node = root_members(t)
    except Exception:
        return None
    if node is None:
        return None
    keys = set(node["order"])
    if snapshot_marker in keys:
        return "cases" if "cases" in keys else "snapshot"
    return None


def pad_snapshots(art, marker, dry=False):
    n = 0
    for p in walk_json(art):
        if "🔺️diff" in p:
            continue
        t = rd(p)
        if ("\"%s\"" % marker) not in t:
            continue
        c = classify(t, marker)
        if c == "snapshot":
            n += 1
            if not dry:
                wr(p, append_root_member(t, "cases", []))
    return n


def pad_diffs(art, marker, dry=False):
    """Diff documents are objects under a `🔺️diff` directory whose lanes include `marker`."""
    n = 0
    for p in walk_json(art):
        if "🔺️diff" not in p:
            continue
        t = rd(p)
        node = root_members(t)
        if node is None or marker not in node["order"] or "cases" in node["order"]:
            continue
        n += 1
        if not dry:
            wr(p, append_root_member(t, "cases", EMPTY_DELTA))
    return n


def case_row(id_, label_en, label_de, entry, numbers, texts):
    return {"id": id_, "labelEn": label_en, "labelDe": label_de, "entryId": entry, "numbers": [{"id": k, "value": v} for k, v in numbers], "texts": [{"id": k, "value": v} for k, v in texts]}


def patch(label_en=None, label_de=None, entry=None, numbers=None, texts=None):
    return {"labelEn": label_en, "labelDe": label_de, "entryId": entry, "numbers": numbers or EMPTY_DELTA, "texts": texts or EMPTY_DELTA}


def delta(removed=(), inserted=(), modified=()):
    return {"removed": list(removed), "inserted": list(inserted), "moved": [], "modified": list(modified)}


def write_json(p, obj):
    wr(p, json.dumps(obj, indent=2, ensure_ascii=False) + "\n")


def kind_fixtures(art, external, base_kind_dir, case_a, case_b, number_key, number_value, text_key, text_value, entry_b):
    """Apply vectors of the seven case kinds on top of the padded before/diff of `base_kind_dir` (an existing kind dir name)."""
    root = art.fixd + "/" + base_kind_dir + "/✅apply"
    base = json.loads(rd(root + "/🔣️.before.json"))
    dtempl = json.loads(rd(root + "/🔣️.diff.json"))
    lane = {k: (None if not isinstance(v, dict) or any(v.get(x) for x in ("removed", "inserted", "moved", "modified")) else EMPTY_DELTA) for k, v in dtempl.items()}

    def mut(kind, payload):
        name = RECORD[kind]
        return {name: payload} if external else dict([("mutation", name[:1].lower() + name[1:])] + list(payload.items()))

    def docs(kind, before_cases, after_cases, payload, cases_delta):
        b = dict(base); b["cases"] = before_cases
        a = dict(base); a["cases"] = after_cases
        d = dict(lane); d["cases"] = cases_delta
        v = art.fixd + "/" + LEAF_DIRS[kind] + "/✅apply"
        write_json(v + "/🔣️.before.json", b)
        write_json(v + "/🔣️.after.json", a)
        write_json(v + "/🔣️.mutation.json", mut(kind, payload))
        write_json(v + "/🔣️.outcome.json", {"status": "applied"})
        write_json(v + "/🔣️.diff.json", d)

    ca, cb = case_a, case_b
    docs("insert-case", [ca], [ca, cb], {"index": 99, "case": cb}, delta(inserted=[{"index": 1, "row": cb}]))
    docs("remove-case", [ca, cb], [cb], {"index": 0}, delta(removed=[{"id": ca["id"], "index": 0}]))
    new_a = json.loads(json.dumps(ca)); [n.update(value=number_value) for n in new_a["numbers"] if n["id"] == number_key]
    docs("change-case-number", [ca], [new_a], {"caseId": ca["id"], "key": number_key, "value": number_value}, delta(modified=[{"id": ca["id"], "patch": patch(numbers=delta(modified=[{"id": number_key, "patch": {"value": number_value}}]))}]))
    new_a = json.loads(json.dumps(ca)); [t.update(value=text_value) for t in new_a["texts"] if t["id"] == text_key]
    docs("change-case-text", [ca], [new_a], {"caseId": ca["id"], "key": text_key, "value": text_value}, delta(modified=[{"id": ca["id"], "patch": patch(texts=delta(modified=[{"id": text_key, "patch": {"value": text_value}}]))}]))
    for kind, field, key in (("change-case-label-en", "labelEn", "label_en"), ("change-case-label-de", "labelDe", "label_de"), ("change-case-entry-id", "entryId", "entry")):
        value = {"labelEn": ca["labelEn"] + " (revised)", "labelDe": ca["labelDe"] + " (überarbeitet)", "entryId": entry_b}[field]
        new_a = dict(ca); new_a[field] = value
        docs(kind, [ca], [new_a], {"caseId": ca["id"], "newValue": value}, delta(modified=[{"id": ca["id"], "patch": patch(**{key: value})}]))
