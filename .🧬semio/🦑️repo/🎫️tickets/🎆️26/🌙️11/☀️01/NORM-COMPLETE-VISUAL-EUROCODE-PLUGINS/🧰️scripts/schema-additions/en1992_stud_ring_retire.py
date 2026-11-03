"""Removes the five retired `*-stud-ring-*` kinds from the en1992 layers the additive leaf-kinds merge cannot shrink (mutation root json/rs, edit rules, text and binary codec, harness, oracle, taxonomy, feature). Idempotent; twins are regenerated afterwards."""
import glob, json, re, sys
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/leaf-kinds")
import os
A = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/"
KINDS = ["change-stud-ring-radius", "change-stud-ring-count", "change-stud-ring-diameter", "insert-stud-ring", "remove-stud-ring"]
RECS = ["".join(w.capitalize() for w in k.split("-")) for k in KINDS]
MODS = [k.replace("-", "_") for k in KINDS]
def rw(path, fn):
    t = open(path, encoding="utf8").read(); u = fn(t)
    if u != t:
        open(path, "w", encoding="utf8").write(u); print("edited", path[len(A):])
def drop(t, pat): return re.sub(pat, "", t, flags=re.M)
ks, rs, ms = "|".join(KINDS), "|".join(RECS), "|".join(MODS)
M = A + "🧬️schema/🧬️mutations/"
def root(t):
    t = drop(t, r"^use [\w:]+::(%s);\n" % ms)
    t = drop(t, r"^    (%s)\([^\n]*\n" % rs)
    return drop(t, r'^    "(%s)",\n' % ks)
rw(M + "🦀️.rs", root)
def js(t):
    d = json.loads(t)
    d["oneOf"] = [x for x in d["oneOf"] if not any("/mutation/%s/" % k in json.dumps(x) for k in KINDS)]
    return json.dumps(d, indent=2, ensure_ascii=False) + "\n"
rw(M + "🔣️.json", js)
rw(M + "🧭️edit-rules/🦀️.rs", lambda t: drop(t, r"^        (SetFieldRule|InsertItemRule|RemoveItemRule) \{ path: \"members\[\]\.studRings[^\n]*\n"))
def text(t):
    t = drop(t, r"^use [\w:]+::(%s);\n" % ms)
    t = drop(t, r"^        En1992Mutation::(%s)\([^\n]*\n" % rs)
    return drop(t, r'^        "(%s)" => Ok[^\n]*\n' % ks)
rw(A + "🚪️io/📝️text/🧬️mutations/🦀️.rs", text)
p = A + "🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
rw(p, lambda t: re.sub(r"^record (%s) tag=\d+\nfield payload bytes\n" % ks, "", t, flags=re.M))
H = A + "🧪️tests/🏛️mutate-en1992-1/"
def hrs(t):
    for k in KINDS:
        t = drop(t, r'^    "%s",\n' % k)
        t = re.sub(r'^        "%s" => \(\n(?:            [^\n]*\n)+        \),\n' % k, "", t, flags=re.M)
    return t
rw(H + "🦀️.rs", hrs)
def hpy(t):
    for k in KINDS:
        t = drop(t, r'^    "%s",\n' % k); t = drop(t, r'^    "%s": \([^\n]*\n' % k)
    return t
rw(H + "🐍️.py", hpy)
rw(H + "🥒️.feature", lambda t: drop(t, r"^      \| (%s) +\|[^\n]*\n" % ks))
def orc(t):
    o = json.loads(t); c, m = o["mutationCatalogs"][0], o["mutationManifests"][0]
    c["vectors"] = [v for v in c["vectors"] if v["mutationId"] not in KINDS]; c["kinds"] = [k for k in c["kinds"] if k not in KINDS]
    m["mutations"] = [x for x in m["mutations"] if x["id"] not in KINDS]
    return json.dumps(o, indent=2, ensure_ascii=False) + "\n"
rw(A + "🔮️oracles/🔣️.json", orc)
from common import TAXONOMY, dumps
rw(TAXONOMY, lambda t: dumps({**json.loads(t), "rows": [r for r in json.loads(t)["rows"] if not (r["artifact"] == "en1992" and r["kind"] in KINDS)]}))
