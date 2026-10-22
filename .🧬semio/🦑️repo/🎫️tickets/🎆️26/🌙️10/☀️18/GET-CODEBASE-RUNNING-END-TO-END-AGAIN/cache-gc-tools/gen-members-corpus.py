#!/usr/bin/env python3
"""Generates the language-agnostic member index corpus with an implementation that shares no code with the engine: sealed rows (sha256 of the canonical form), index texts and the expected proof outcome."""
import hashlib, json, os

HERE = os.path.dirname(os.path.abspath(__file__))
GROUP = "selections/" + "a" * 64
OTHER = "contexts/" + "b" * 64


def canon(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def seal(row):
    bare = {key: value for key, value in row.items() if key != "c"}
    return dict(row, c=hashlib.sha256(canon(bare).encode("utf-8")).hexdigest()[:16])


def hx(char, length=64):
    return (char * length)[:length]


def light(name, kind, size, stamp, digit):
    return seal({"n": name, "b": size, "h": hx(digit), "k": kind, "s": stamp})


def full(name, kind, size, stamp, digit, refs, links, graph=None):
    return seal({"n": name, "b": size, "h": hx(digit), "k": kind, "s": stamp, "x": size * 4, "g": graph, "r": sorted(refs), "l": links})


def header(group=GROUP):
    return json.dumps({"group": group, "semio": "semio-member-index", "version": 1}, sort_keys=True, separators=(",", ":"))


def text(rows, group=GROUP, tail=""):
    return header(group) + "\n" + "".join(json.dumps(row, sort_keys=True, separators=(",", ":")) + "\n" for row in rows) + tail


def sel(kind, identity, rest, label):
    return ("inference/%s/%s/%s" % (kind, identity, rest), label, "%s/%s" % (kind, identity), "inference/%s/%s" % (kind, identity), rest)


names = [
    sel("selections", "a" * 64, "np/%s.json" % hx("1", 24), "inference"),
    sel("selections", "a" * 64, "ip/%s.json" % hx("2", 24), "precision"),
    sel("contexts", "b" * 64, "rr/%s.json" % hx("3", 24), "cargo-receipt"),
    sel("selections", "a" * 64, "ob/%s.br" % hx("4"), "observation"),
    sel("selections", "a" * 64, "np/%s.json.records/mvhfzt80-16334-%s.pack" % (hx("1", 24), hx("5", 16)), "pack"),
    sel("selections", "a" * 64, "np/%s.json.records/mvhfzt80-16334-%s.idx" % (hx("1", 24), hx("5", 16)), "index"),
    sel("selections", "a" * 64, "test-cases.json", "inference"),
    sel("selections", "a" * 64, "emoji-project-json.receipt.json", "receipt"),
    sel("selections", "a" * 64, "emoji-project-json.publication.json", "publication"),
    sel("selections", "a" * 64, "members.jsonl", "member-index"),
    sel("contexts", "b" * 64, "members.jsonl", "member-index"),
    ("inference/np/%s.json" % hx("6", 24), "inference", "canonical", "inference", "np/%s.json" % hx("6", 24)),
    ("inference/ob/%s.br" % hx("7"), "observation", "canonical", "inference", "ob/%s.br" % hx("7")),
    ("inference/inference/emoji-project-json.publication.json", "publication", "canonical", "inference", "inference/emoji-project-json.publication.json"),
    ("inference/inference/p-57987/emoji-project-json.publication.json", "publication", "canonical", "inference", "inference/p-57987/emoji-project-json.publication.json"),
    ("inference/test-cases.json.records/mvhfzt80-16334-%s.pack" % hx("5", 16), "pack", "canonical", "inference", "test-cases.json.records/mvhfzt80-16334-%s.pack" % hx("5", 16)),
    ("inference/members.jsonl", "member-index", "canonical", "inference", "members.jsonl"),
    ("generations/o/%s.json.br" % hx("8"), "generation-fragment", "generations", "generations", "o/%s.json.br" % hx("8")),
    ("generations/x/%s.json.br" % hx("8"), "generation-index", "generations", "generations", "x/%s.json.br" % hx("8")),
    ("generations/f/%s.json.br" % hx("8"), "generation-frontier", "generations", "generations", "f/%s.json.br" % hx("8")),
    ("generations/m/%s.json" % hx("8"), "generation-manifest", "generations", "generations", "m/%s.json" % hx("8")),
    ("generations/members.jsonl", "member-index", "generations", "generations", "members.jsonl"),
    ("generations/current.json", "generation-pointer", None, None, None),
    ("generations/o/%s.json" % hx("8"), None, "generations", "generations", "o/%s.json" % hx("8")),
    ("inference/selections/%s/np/short.json" % ("a" * 64), None, "selections/" + "a" * 64, "inference/selections/" + "a" * 64, "np/short.json"),
    ("inference/selections/%s/other.txt" % ("a" * 64), None, "selections/" + "a" * 64, "inference/selections/" + "a" * 64, "other.txt"),
    ("inference/selections/%s" % ("a" * 63) + "/np/%s.json" % hx("1", 24), "inference", None, None, None),
    ("inference/selections/%s.json" % hx("1", 24), None, None, None, None),
    ("build-input-closures/%s.json" % hx("9"), None, None, None, None),
    ("selected/%s.json" % hx("9"), None, None, None, None),
]
kinds = [{"name": n, "kind": k, "group": g, "directory": d, "member": m} for n, k, g, d, m in names]

A = lambda digit: "source:" + hx(digit)
rows = {
    "np": light("np/%s.json" % hx("1", 24), "inference", 1200, "10:1200:5000", "a"),
    "ip": light("ip/%s.json" % hx("2", 24), "precision", 900, "11:900:5001", "b"),
    "ob": full("ob/%s.br" % hx("4"), "observation", 300, "12:300:5002", "c", [A("d")], [], None),
    "pk": full("np/x.json.records/aaaa-1-%s.pack" % hx("5", 16), "pack", 5000, "13:5000:5003", "d", [A("e"), "lock:" + hx("f")], [{"p": "ob/%s.br" % hx("4"), "m": "block"}], None),
}
valid_rows = [{"id": key, "line": json.dumps(row, sort_keys=True, separators=(",", ":")), "valid": True} for key, row in rows.items()]
bad = []
def bad_case(id, row):
    bad.append({"id": id, "line": json.dumps(row, sort_keys=True, separators=(",", ":")), "valid": False})
base = dict(rows["np"])
bad_case("tampered size", dict(base, b=base["b"] + 1))
bad_case("tampered check", dict(base, c=hx("0", 16)))
bad_case("unknown key", seal(dict({k: v for k, v in base.items() if k != "c"}, z=1)))
bad_case("unknown kind", seal({"n": "np/x.json", "b": 1, "h": hx("a"), "k": "spreadsheet", "s": "1:1:1"}))
bad_case("short hash", seal({"n": "np/x.json", "b": 1, "h": "abc", "k": "inference", "s": "1:1:1"}))
bad_case("parent segment", seal({"n": "np/../x.json", "b": 1, "h": hx("a"), "k": "inference", "s": "1:1:1"}))
bad_case("absolute name", seal({"n": "/np/x.json", "b": 1, "h": hx("a"), "k": "inference", "s": "1:1:1"}))
bad_case("backslash name", seal({"n": "np\\x.json", "b": 1, "h": hx("a"), "k": "inference", "s": "1:1:1"}))
bad_case("bad stamp", seal({"n": "np/x.json", "b": 1, "h": hx("a"), "k": "inference", "s": "1:1"}))
bad_case("negative size", seal({"n": "np/x.json", "b": -1, "h": hx("a"), "k": "inference", "s": "1:1:1"}))
bad_case("half a full row", seal({"n": "np/x.json", "b": 1, "h": hx("a"), "k": "inference", "s": "1:1:1", "x": 4}))
bad_case("bad reference", seal({"n": "np/x.json", "b": 1, "h": hx("a"), "k": "inference", "s": "1:1:1", "x": 4, "g": None, "r": ["source:xyz"], "l": []}))
bad_case("bad link", seal({"n": "np/x.json", "b": 1, "h": hx("a"), "k": "inference", "s": "1:1:1", "x": 4, "g": None, "r": [], "l": [{"p": "ob/x.br", "m": "file"}]}))
row_cases = valid_rows + bad + [{"id": "not json", "line": "{\"n\":", "valid": False}, {"id": "an array", "line": "[]", "valid": False}]

newer = light("np/%s.json" % hx("1", 24), "inference", 1300, "10:1300:5100", "9")
index_cases = [
    {"id": "empty text", "group": GROUP, "text": "", "valid": False, "names": [], "rejected": 0, "tornTail": False},
    {"id": "header only", "group": GROUP, "text": header() + "\n", "valid": True, "names": [], "rejected": 0, "tornTail": False},
    {"id": "foreign group", "group": GROUP, "text": text([rows["np"]], OTHER), "valid": False, "names": [], "rejected": 0, "tornTail": False},
    {"id": "two rows", "group": GROUP, "text": text([rows["np"], rows["ip"]]), "valid": True, "names": sorted([rows["np"]["n"], rows["ip"]["n"]]), "rejected": 0, "tornTail": False},
    {"id": "last row wins", "group": GROUP, "text": text([rows["np"], newer]), "valid": True, "names": [newer["n"]], "rejected": 0, "tornTail": False, "winner": {newer["n"]: newer["h"]}},
    {"id": "torn tail ignored", "group": GROUP, "text": text([rows["np"]], tail=json.dumps(rows["ip"], sort_keys=True, separators=(",", ":"))[:40]), "valid": True, "names": [rows["np"]["n"]], "rejected": 0, "tornTail": True},
    {"id": "merged line after a crash", "group": GROUP, "text": header() + "\n" + json.dumps(rows["np"], sort_keys=True, separators=(",", ":"))[:40] + json.dumps(rows["ip"], sort_keys=True, separators=(",", ":")) + "\n" + json.dumps(rows["ob"], sort_keys=True, separators=(",", ":")) + "\n", "valid": True, "names": [rows["ob"]["n"]], "rejected": 1, "tornTail": False},
    {"id": "damaged middle line", "group": GROUP, "text": header() + "\n" + json.dumps(rows["np"], sort_keys=True, separators=(",", ":")) + "\nnot json\n" + json.dumps(rows["ip"], sort_keys=True, separators=(",", ":")) + "\n", "valid": True, "names": sorted([rows["np"]["n"], rows["ip"]["n"]]), "rejected": 1, "tornTail": False},
]

leaf = lambda name, kind, size, stamp: {"name": name, "kind": kind, "bytes": size, "stamp": stamp}
proof_cases = [
    {
        "id": "every leaf indexed, one with references",
        "group": GROUP,
        "text": text([rows["np"], rows["ip"], rows["ob"], rows["pk"]]),
        "physical": [leaf(rows["np"]["n"], "inference", 1200, "10:1200:5000"), leaf(rows["ip"]["n"], "precision", 900, "11:900:5001"), leaf(rows["ob"]["n"], "observation", 300, "12:300:5002"), leaf(rows["pk"]["n"], "pack", 5000, "13:5000:5003"), leaf("members.jsonl", "member-index", 999, "1:999:1")],
        "expect": {"full": 2, "light": 2, "stale": 0, "unindexed": 0, "complete": True, "referencesComplete": False, "references": sorted([A("d"), A("e"), "lock:" + hx("f")])},
    },
    {
        "id": "all full leaves complete the references",
        "group": GROUP,
        "text": text([rows["ob"], rows["pk"]]),
        "physical": [leaf(rows["ob"]["n"], "observation", 300, "12:300:5002"), leaf(rows["pk"]["n"], "pack", 5000, "13:5000:5003")],
        "expect": {"full": 2, "light": 0, "stale": 0, "unindexed": 0, "complete": True, "referencesComplete": True, "references": sorted([A("d"), A("e"), "lock:" + hx("f")])},
    },
    {
        "id": "replaced leaf is stale",
        "group": GROUP,
        "text": text([rows["np"]]),
        "physical": [leaf(rows["np"]["n"], "inference", 1200, "10:1200:9999")],
        "expect": {"full": 0, "light": 0, "stale": 1, "unindexed": 0, "complete": False, "referencesComplete": False, "references": []},
    },
    {
        "id": "same stamp but another size is stale",
        "group": GROUP,
        "text": text([rows["np"]]),
        "physical": [leaf(rows["np"]["n"], "inference", 1201, "10:1200:5000")],
        "expect": {"full": 0, "light": 0, "stale": 1, "unindexed": 0, "complete": False, "referencesComplete": False, "references": []},
    },
    {
        "id": "same name under another kind is stale",
        "group": GROUP,
        "text": text([rows["np"]]),
        "physical": [leaf(rows["np"]["n"], "precision", 1200, "10:1200:5000")],
        "expect": {"full": 0, "light": 0, "stale": 1, "unindexed": 0, "complete": False, "referencesComplete": False, "references": []},
    },
    {
        "id": "leaf without a row is unindexed",
        "group": GROUP,
        "text": text([rows["np"]]),
        "physical": [leaf(rows["np"]["n"], "inference", 1200, "10:1200:5000"), leaf(rows["ip"]["n"], "precision", 900, "11:900:5001")],
        "expect": {"full": 0, "light": 1, "stale": 0, "unindexed": 1, "complete": False, "referencesComplete": False, "references": []},
    },
    {
        "id": "damaged index leaves everything unindexed",
        "group": GROUP,
        "text": "garbage\n",
        "physical": [leaf(rows["np"]["n"], "inference", 1200, "10:1200:5000")],
        "expect": {"full": 0, "light": 0, "stale": 0, "unindexed": 1, "complete": False, "referencesComplete": False, "references": []},
    },
    {
        "id": "dead rows are ignored",
        "group": GROUP,
        "text": text([rows["np"], rows["ip"]]),
        "physical": [leaf(rows["np"]["n"], "inference", 1200, "10:1200:5000")],
        "expect": {"full": 0, "light": 1, "stale": 0, "unindexed": 0, "complete": True, "referencesComplete": False, "references": []},
    },
    {
        "id": "empty group",
        "group": GROUP,
        "text": header() + "\n",
        "physical": [],
        "expect": {"full": 0, "light": 0, "stale": 0, "unindexed": 0, "complete": True, "referencesComplete": True, "references": []},
    },
]
corpus = {"version": 1, "group": GROUP, "kinds": kinds, "rows": row_cases, "indexes": index_cases, "proofs": proof_cases}
out = os.path.join(HERE, "staged4", "members-cases.json")
with open(out, "w", encoding="utf-8") as handle:
    json.dump(corpus, handle, indent=1, ensure_ascii=False)
    handle.write("\n")
print("wrote", out, len(kinds), "names", len(row_cases), "rows", len(index_cases), "indexes", len(proof_cases), "proofs")
