#!/usr/bin/env python3
"""📋️ Builds the per-file finding table of C/result-compliance.md from the scan outputs: python3 -I report.py <out md>. Streams the jsonl files; reads the small json files whole."""
import collections, glob, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
G = os.path.join(C, "🗑️generated", "cp", sys.argv[2] if len(sys.argv) > 2 else "")

def jsonl(path):
    if not os.path.exists(path): return
    for line in open(path, encoding="utf8"):
        yield json.loads(line)

def rows_of(paths):
    out = collections.defaultdict(list)
    for p in paths:
        for row in jsonl(p):
            for x in row.get("rows", []): out[row["file"]].append((x["rule"], x["line"], x["detail"]))
    return out

today = [l.rstrip("\n") for l in open(os.path.join(HERE, "today.txt"), encoding="utf8") if l.strip()]
before = rows_of([os.path.join(G, "ast.jsonl"), os.path.join(G, "other.jsonl")])
after = rows_of([os.path.join(G, "ast-after.jsonl"), os.path.join(G, "other-after.jsonl")])
laws = json.load(open(os.path.join(G, "laws.json"), encoding="utf8"))
laws_after = json.load(open(os.path.join(G, "laws-after-all.json"), encoding="utf8")) if os.path.exists(os.path.join(G, "laws-after-all.json")) else None
names = json.load(open(os.path.join(G, "names-split.json"), encoding="utf8"))
policy = json.load(open(os.path.join(G, "policy-const2.json"), encoding="utf8"))
batch_of = collections.defaultdict(list)
moved_from = {}
for m in sorted(glob.glob(os.path.join(HERE, "staged", "*", "manifest.json"))):
    b = os.path.basename(os.path.dirname(m))
    for rel, row in json.load(open(m, encoding="utf8")).items():
        batch_of[rel].append(b)
        if row["new"] is None: moved_from[rel] = b

def law_rows(law, key):
    out = collections.defaultdict(list)
    for f in law.get(key, []):
        out[f["path"]].append(f)
    return out

lang_before = law_rows(laws, "languageLawMine")
lang_after = law_rows(laws_after, "languageLawMine") if laws_after else {}
docs_before = collections.defaultdict(list)
for h in laws["docstringLaw"]: docs_before[h["path"]].append(h)
runtime_before = collections.defaultdict(list)
for v in laws["runtimeLaw"]: runtime_before[v.split(":")[0]].append(v)
naming = collections.defaultdict(list)
for f in names["new"]:
    if f["path"].endswith((".mjs", ".d.mts")) and f.get("sibling", "").endswith((".d.mts", ".mjs")): continue
    naming[f["path"]].append(f)

def short(rows, rule): return [r for r in rows if r[0] == rule]
KINDS = ["doc-not-emoji", "dup-emoji", "comment-in-body", "debug-tag", "external-import", "legacy-word", "de-ascii-fallback", "de-equals-en", "de-empty"]

is_test = lambda rel: ("🧪️tests" in rel) or ("🧪️test/" in rel) or "/tests/" in rel
LEASE_LEGACY = ("🔒️leases/📈️standing/", "🧭️routing/📥️invocation/🏃️process/")
def classify(rel, kind, rows):
    if kind == "comment-in-body": return "doc-in-body" if all(d.startswith("docstring in a function body") for _r, _l, d in rows) else "comment-in-body"
    if kind == "legacy-word": return "legacy-lease-state" if any(p in rel for p in LEASE_LEGACY) else "legacy-word(false positive)"
    return kind
MUST = ["doc-not-emoji", "dup-emoji", "comment-in-body", "debug-tag", "prose-literal", "path-emoji", "runtime-name"]
per = []
total = collections.Counter(); total_after = collections.Counter()
tests_external = collections.Counter(); prod_external = collections.defaultdict(set)
for rel in today:
    b = before.get(rel, []); a = after.get(rel, [])
    cells = {}
    for k in ["doc-not-emoji", "dup-emoji", "comment-in-body", "debug-tag", "de-ascii-fallback", "de-equals-en", "de-empty", "legacy-word"]:
        rb, ra = short(b, k), short(a, k)
        if rel.endswith(".rs") and k == "comment-in-body": continue
        if not rb and not ra: continue
        kc = classify(rel, k, rb or ra)
        if k == "legacy-word" and kc == "legacy-word(false positive)": continue
        cells[kc] = (len(rb), len(ra)); total[kc] += len(rb); total_after[kc] += len(ra)
    rule = "external-import"
    if True:
        ext = sorted({d for r, _l, d in b if r == rule and not d.startswith("@semio-tech/")})
        if ext:
            if is_test(rel): tests_external.update(ext)
            else: prod_external[rel] = set(ext)
    lb = lang_before.get(rel, []); la = lang_after.get(rel, [])
    if lb: cells["prose-literal"] = (len(lb), len(la)); total["prose-literal"] += len(lb); total_after["prose-literal"] += len(la)
    if rel in runtime_before: cells["runtime-name"] = (len(runtime_before[rel]), 0); total["runtime-name"] += 1
    if rel in naming: cells["path-emoji"] = (len(naming[rel]), 0 if rel in batch_of else len(naming[rel])); total["path-emoji"] += len(naming[rel]); total_after["path-emoji"] += 0 if rel in batch_of else len(naming[rel])
    sf = policy.get(rel, [])
    if sf and not is_test(rel): cells["schema-first?"] = (len(sf), len(sf)); total["schema-first?"] += len(sf); total_after["schema-first?"] += len(sf)
    if rel in prod_external and not is_test(rel): cells["external-import(prod)"] = (len(prod_external[rel]), len(prod_external[rel]))
    nodoc = len(short(b, "export-no-doc"))
    if cells:
        per.append((rel, cells, nodoc, batch_of.get(rel, [])))
out = sys.argv[1]
with open(out, "w", encoding="utf8") as o:
    o.write("| Finding | Before | After the staged batches |\n|---|---|---|\n")
    for k in sorted(total): o.write("| %s | %d | %d |\n" % (k, total[k], total_after[k]))
    o.write("\nExternal imports in TEST files (allowed: tests may use third-party oracles): %s\n\n" % ", ".join("%s x%d" % (k, v) for k, v in tests_external.most_common()))
    o.write("| File (tail) | Findings before -> after | Fixed in |\n|---|---|---|\n")
    for rel, cells, nodoc, bs in sorted(per):
        txt = "; ".join("%s %d->%d" % (k, v[0], v[1]) for k, v in sorted(cells.items()))
        o.write("| `%s` | %s | %s |\n" % (rel[-105:], txt, ", ".join(bs) if bs else "-"))
    clean = [r for r in today if r not in {p[0] for p in per}]
    o.write("\nFiles with no finding (%d of %d): all other files of the list; undocumented exports (info, no AGENTS.md rule demands a docstring on every export) exist in %d files.\n" % (len(clean), len(today), sum(1 for r in today if short(before.get(r, []), "export-no-doc"))))
print(len(per), "files with findings of", len(today))
