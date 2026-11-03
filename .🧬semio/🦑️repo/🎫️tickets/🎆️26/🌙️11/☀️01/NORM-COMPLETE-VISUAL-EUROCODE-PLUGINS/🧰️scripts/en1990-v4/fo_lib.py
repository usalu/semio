"""Shared helpers of the EN 1990 v4 factor-override cascade (ticket-local input script; ROOT override with FO_ROOT for scratch runs)."""
import difflib, json, os, re, sys

SEMIO = "/home/deck/semio"
ARTS = os.environ.get("FO_ROOT", SEMIO + "/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
SUB = ARTS + "/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any"
SCH = SUB + "/🧬️schema"
SNAP = SCH + "/📸️snapshot"
DIFF = SCH + "/🔺️diff"
SQ = SUB + "/🚪️io/🪶️sqlite/📸️snapshot"
INF = SCH + "/💡️inferences"

ENTITY = "FactorOverride"
COLL = "factor_overrides"
CAMEL = "factorOverrides"
TABLE = "en1990_factor_override"
ROW = "factor_override"
FIELDS = [
    ("factor", "String", "Factor kind", "Faktorart"),
    ("target", "String", "Target action or group", "Zielwirkung oder -gruppe"),
    ("value", "f64", "Override value", "Überschreibungswert"),
    ("justification", "String", "Justification", "Begründung"),
]
FACTORS = ["gamma", "psi0", "psi1", "psi2", "xi", "k_fi"]


def camel(s):
    parts = s.split("_")
    return parts[0] + "".join(p.capitalize() for p in parts[1:])


def rd(p):
    return open(p, encoding="utf8").read()


def wr(p, t):
    assert len(p[len(SEMIO) + 1:].encode()) < 256 if p.startswith(SEMIO) else True, "path too long " + p
    open(p, "w", encoding="utf8").write(t)


def edit(p, fn):
    t = rd(p)
    n = fn(t)
    if n != t:
        wr(p, n)
        print("  edited", p.split("/✳️any/")[-1])
    return n


def sub1(t, pattern, repl, flags=0):
    out, n = re.subn(pattern, repl, t, count=1, flags=flags)
    assert n == 1, "no unique match: " + pattern[:120]
    return out


def put(t, old, new, guard=None):
    if (guard or new) in t:
        return t
    assert t.count(old) == 1, ("anchor", old[:100], t.count(old))
    return t.replace(old, new)
