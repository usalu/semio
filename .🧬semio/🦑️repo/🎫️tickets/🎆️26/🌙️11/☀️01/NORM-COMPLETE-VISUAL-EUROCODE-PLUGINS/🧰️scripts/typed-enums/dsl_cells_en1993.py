#!/usr/bin/env python3
"""🧹 Typed enums v6: rewrites the enum cells of the en1993 `.dsl.semio` assets to the kebab tag of the schema variant (`BeamColumn` -> `beam-column`).
usage: dsl_cells_en1993.py [--apply]"""
import glob, json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import convert as C
apply = "--apply" in sys.argv
spec = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "specs", "en1993.json"), encoding="utf-8"))
norm = lambda s: re.sub(r"[^a-z0-9]", "", s.lower())
kebab = lambda n: "-".join(w.lower() for w in re.findall(r"[A-Z][a-z0-9]*|[a-z0-9]+", n))
TABLED = {("materials", "kind"): "MaterialKind", ("sections", "kind"): "SectionKind", ("load-cases", "kind"): "ActionKind", ("joints", "kind"): "JointKind", ("joints", "category"): "JointCategory", ("built-up-members", "kind"): "BuiltUpKind", ("parametric-sections", "shape"): "ParametricShape"}
enums = {c["enum"]: c for c in spec["conversions"]}
cols = {}
for c in spec["conversions"]:
    for h in c["hosts"]:
        key = re.sub(r"_", "-", h.split(".")[1])
        if key not in ("kind", "category", "shape"):
            cols.setdefault(key, set()).add(c["enum"])
TOK = re.compile(r'"(?:[^"\\]|\\.)*"|\[[^\]]*\]|\{[^}]*\}|\S+')
unknown = {}
changed = 0
for p in glob.glob(C.artifact_root("en1993") + "/🖼️assets/**/*.dsl.semio", recursive=True):
    out, head = [], None
    for line in open(p, encoding="utf-8").read().split("\n"):
        m = re.match(r"^(\S+) \[(.*)\] \{$", line)
        if m:
            head = (m.group(1), [x.split(":")[0] for x in m.group(2).split(" ")])
            out.append(line)
            continue
        if line.startswith("  ") and head and not line.startswith("  ["):
            toks = list(TOK.finditer(line))
            if len(toks) == len(head[1]):
                for t, col in zip(reversed(toks), reversed(head[1])):
                    e = TABLED.get((head[0], col)) or (next(iter(cols[col])) if len(cols.get(col, ())) == 1 else None)
                    if not e:
                        continue
                    raw = t.group(0).strip('"')
                    variants = [v[0] for v in enums[e]["variants"]]
                    hit = next((v for v in variants if norm(v) == norm(raw)), None)
                    if hit is None and raw == "" and "Unset" in variants:
                        hit = "Unset"
                    if hit is None:
                        hit = next((v[0] for v in enums[e]["variants"] if norm(raw) in [norm(x) for x in v[1]]), None)
                    if hit is None:
                        unknown[(head[0], col, raw)] = unknown.get((head[0], col, raw), 0) + 1
                        continue
                    new = kebab(hit)
                    if new != t.group(0):
                        line = line[: t.start()] + new + line[t.end():]
                        changed += 1
            elif toks:
                unknown[("arity", head[0], len(toks), len(head[1]))] = 1
        out.append(line)
    if apply:
        open(p, "w", encoding="utf-8", newline="").write("\n".join(out))
print(f"{changed} cells", "written" if apply else "would change")
for k, v in sorted(unknown.items(), key=str):
    print("UNKNOWN", k, v)
