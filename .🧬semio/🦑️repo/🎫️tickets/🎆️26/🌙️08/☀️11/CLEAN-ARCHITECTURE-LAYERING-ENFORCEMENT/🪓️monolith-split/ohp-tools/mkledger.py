"""📒 Writes F/ORACLE-HARVEST-PLUGINS.md from ohp-tools/ledger.tsv and the first/last pos result of every family (resolution+privacy rows before/after, new rows)."""
import json, glob, os, re, collections
F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
RES = "E0425 E0432 E0433 E0412 E0603 E0616 E0624 E0107 E0422 E0423 E0405 E0407 E0437 E0438".split()
WANT = "E0425 E0432 E0433 E0412 E0599 E0061 E0308 E0603 E0616 E0624 E0107 E0282".split()
def rows(path, codes):
    r = json.load(open(path, encoding="utf-8"))
    out = {}
    for cfg, e in r["summary"].items():
        c = collections.Counter()
        for d in e["new_list"]:
            if d["stub"] or d.get("inherited") or d["code"] not in codes: continue
            f = re.sub(r"^__m_[A-Za-z0-9_]+/", "", d["file"]); f = "/".join(f.split("/")[-4:])
            c[(d["code"], f, re.sub(r"__m_[A-Za-z0-9_]+", "M", d["msg"])[:130])] += 1
        out[cfg] = c
    return out
fams = collections.defaultdict(list)
for p in glob.glob(F + "/🗑️generated/ohp-*-result.json"):
    m = re.match(r".*/ohp-(.+?)-(r\d+|d\d+|e\d+)-result\.json$", p)
    if m: fams[m.group(1)].append((m.group(2), p))
def key(l): return (l[0][0], int(l[0][1:]))
out = ["# Oracle harvest, plugin families: ledger", "", "Machine ledger: `ohp-tools/ledger.tsv`. Section 1 lists each real or artifact class with its fix; section 2 gives the proof per family (non-stub rows of the resolution and privacy codes, first run against the last run, `new` = rows that appear only in the last run).", "", "## 1. Rows by class", "", "| family | cfg | row | class | fix |", "|---|---|---|---|---|"]
for l in open(F + "/ohp-tools/ledger.tsv", encoding="utf-8"):
    c = l.rstrip("\n").split("\t")
    if len(c) == 5: out.append("| " + " | ".join(x.replace("|", "/") for x in c) + " |")
out += ["", "## 2. Proof per family (resolution + privacy codes, non-stub)", "", "| family | runs | cfg | before | after | gone | new | all wanted codes before / after |", "|---|---|---|---|---|---|---|---|"]
for fam in sorted(fams):
    runs = sorted(fams[fam], key=key)
    if len(runs) < 2: 
        a = rows(runs[0][1], RES); w = rows(runs[0][1], WANT)
        for cfg in a: out.append(f"| {fam} | {runs[0][0]} only | {cfg} | {sum(a[cfg].values())} | - | - | - | {sum(w[cfg].values())} / - |")
        continue
    f0, f1 = runs[0], runs[-1]
    a, b = rows(f0[1], RES), rows(f1[1], RES)
    wa, wb = rows(f0[1], WANT), rows(f1[1], WANT)
    for cfg in a:
        gone = a[cfg] - b.get(cfg, collections.Counter()); new = b.get(cfg, collections.Counter()) - a[cfg]
        out.append(f"| {fam} | {f0[0]} to {f1[0]} | {cfg} | {sum(a[cfg].values())} | {sum(b.get(cfg, {}).values())} | {sum(gone.values())} | {sum(new.values())} | {sum(wa[cfg].values())} / {sum(wb.get(cfg, {}).values())} |")
open(F + "/ORACLE-HARVEST-PLUGINS.md", "w", encoding="utf-8").write("\n".join(out) + "\n")
print(len(out), "lines")
