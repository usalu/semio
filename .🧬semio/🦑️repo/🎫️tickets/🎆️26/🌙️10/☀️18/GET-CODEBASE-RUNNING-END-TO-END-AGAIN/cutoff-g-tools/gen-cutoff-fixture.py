"""Regenerates the fixture of the cutoff laws: hand written worlds plus 120 seeded random worlds; the expected outcome of every world is computed by the independent Python model of the component, never by the JavaScript."""
import importlib.util
import json
import random
import sys

G = sys.argv[1]
spec = importlib.util.spec_from_file_location("model", G + "/🧪️tests/🐍️oracle/🐍️.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
worlds = []


def add(id, owners, derived, stale, answers):
    w = {"id": id, "owners": owners, "derived": derived, "stale": stale, "answers": answers}
    w["expect"] = m.plan(w)
    worlds.append(w)


add("nothing stale", {"a": {"raw": ["r1"], "derived": ["q1"]}, "b": {"raw": [], "derived": ["q1"]}}, {"q1": {"answer": "d1", "support": ["s1", "s2"]}}, [], {})
add("support stale, same answer: cutoff", {"a": {"raw": ["r1"], "derived": ["q1"]}, "b": {"raw": [], "derived": ["q1"]}}, {"q1": {"answer": "d1", "support": ["s1", "s2"]}}, ["s1"], {"q1": "d1"})
add("support stale, other answer: all holders", {"a": {"raw": ["r1"], "derived": ["q1"]}, "b": {"raw": [], "derived": ["q1"]}, "c": {"raw": [], "derived": []}}, {"q1": {"answer": "d1", "support": ["s1", "s2"]}}, ["s2"], {"q1": "d2"})
add("computation fails: all holders", {"a": {"raw": [], "derived": ["q1"]}, "b": {"raw": [], "derived": ["q2"]}}, {"q1": {"answer": "d1", "support": ["s1"]}, "q2": {"answer": "e1", "support": ["s9"]}}, ["s1"], {"q1": "!error"})
add("direct raw row and held question", {"a": {"raw": ["r1"], "derived": ["q1"]}, "b": {"raw": ["r2"], "derived": ["q1"]}}, {"q1": {"answer": "d1", "support": ["s1", "r1"]}}, ["r1"], {"q1": "d1"})
add("shared support row staled two questions, one changes", {"a": {"raw": [], "derived": ["q1"]}, "b": {"raw": [], "derived": ["q2"]}, "c": {"raw": [], "derived": ["q1", "q2"]}}, {"q1": {"answer": "x", "support": ["s1", "s2"]}, "q2": {"answer": "y", "support": ["s1", "s3"]}}, ["s1"], {"q1": "x", "q2": "z"})
add("stale row outside every support", {"a": {"raw": ["r1"], "derived": ["q1"]}}, {"q1": {"answer": "d", "support": ["s1"]}}, ["zzz"], {"q1": "d"})
rnd = random.Random(20261026)
for n in range(120):
    names = ["p%d" % i for i in range(rnd.randint(1, 9))]
    qs = ["q%d" % i for i in range(rnd.randint(1, 6))]
    rows = ["x%d" % i for i in range(rnd.randint(3, 14))]
    derived = {q: {"answer": "d" + q, "support": rnd.sample(rows, rnd.randint(1, min(5, len(rows))))} for q in qs}
    owners = {p: {"raw": rnd.sample(rows, rnd.randint(0, 3)), "derived": rnd.sample(qs, rnd.randint(0, min(3, len(qs))))} for p in names}
    stale = rnd.sample(rows, rnd.randint(0, min(5, len(rows))))
    answers = {q: (derived[q]["answer"] if rnd.random() < 0.6 else rnd.choice(["other", "!error"])) for q in qs}
    add("random %d" % n, owners, derived, stale, answers)
json.dump({"version": 1, "worlds": worlds}, open(G + "/🧪️tests/🔣️.json", "w"), indent=None, separators=(",", ":"), ensure_ascii=False)
print(len(worlds), "worlds")
