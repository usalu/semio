"""Reads one v3 catalogue and checks entry inputs against the vocabulary and the evaluator accessors. Usage: python3 -I check_catalogue.py CATALOGUE_DIR"""
import json, pathlib, re, sys
d = pathlib.Path(sys.argv[1])
vocab = {(r["key"], r["scope"]) for r in json.loads((d / "📚️inputs/🔣️.json").read_text(encoding="utf-8"))["inputs"]}
keys = {k for k, _ in vocab}
cases = (d / "🦀️.rs").read_text(encoding="utf-8")
ev = (d / "🧮️evaluators/🦀️.rs").read_text(encoding="utf-8")
bodies = {m.group(1): m.group(2) for m in re.finditer(r"(?:pub )?fn (\w+)\(q: &Inputs[^)]*\) -> [\w<>:, ]+ \{(.*?)\n\}\n", ev, re.S)}
used, bad = set(), 0
for m in re.finditer(r'Entry \{ id: "([^"]+)".*?inputs: &\[(.*?)\], eval: evaluators::(\w+) \}', cases):
    eid, ins, fn = m.group(1), re.findall(r'"([^"]+)"', m.group(2)), m.group(3)
    body = bodies[fn] + "".join(bodies.get(h, "") for h in re.findall(r'(\w+)\(q[,)]', bodies[fn]))
    read = set(re.findall(r'q\.[nt]\("([^"]+)"\)', body))
    for k in ins:
        if k not in keys: print("NO ROW", eid, k); bad += 1
    if set(ins) != read: print("MISMATCH", eid, "declared-not-read", sorted(set(ins) - read), "read-not-declared", sorted(read - set(ins))); bad += 1
    used |= set(ins)
print("unused rows", sorted(keys - used)); print("entries", len(re.findall(r'Entry \{ id:', cases)), "bad", bad)
