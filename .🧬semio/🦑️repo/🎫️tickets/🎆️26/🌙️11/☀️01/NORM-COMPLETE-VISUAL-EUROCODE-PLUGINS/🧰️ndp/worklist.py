import json, re, sys, collections
reg = json.load(open(sys.argv[1]))["entries"]
cf = json.load(open(sys.argv[2]))
byline = {}
for e in reg: byline.setdefault(e["specRef"], []).append(e["id"])
conflicts = cf["conflicts"]
spec = {"E92": "en1992-target.md", "E95": "en1995-en1996-target.md", "E96": "en1995-en1996-target.md"}
cmark = {}
for c in conflicts:
    ref = f"{spec[c[0][:3]]}:{re.match(r'\d+', c[1]).group(0)}"
    for i in byline[ref]: cmark.setdefault(i, []).append(c[0])
def esc(x): return x.replace("|", "\\|").replace("\n", " ")
o = ["# NDP Verification Worklist", "",
"Consolidated list of every value tagged `[V]`, `[T]`, `[verify]` or `NA-DE ?` in `📐️specs/*-target.md`, for human verification against the printed DIN EN standards and national annexes. Registry: `⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json` (status `unverified` until a human sets `verified` and records the print edition). Until then every check citing the id yields `UnverifiedBasis`, never a compliant verdict.", "",
f"Totals: {len(reg)} registry entries, {sum(e['status']=='verified' for e in reg)} verified, {len(conflicts)} repo-vs-spec conflicts (section 1, verify first).", "",
"Extraction note: entries are line-segment granular and were extracted mechanically; `Value used` quotes the spec wording (the value the software would use today). `Symbol` or `Clause` of `?` means the segment names none, read the spec line. Typed numeric recommended/NA-DE values are filled in the JSON only where the spec states `EC x / DE y`. Spec lines: `file:line` in `📐️specs/`.", "",
"How to verify: open the printed standard/NA at the cited clause, compare the `Value used`, then edit the entry in `🔣️.json` (`status: verified`, `source.edition`, `source.locator`, numeric values) and regenerate `🗃️table.rs` with `🧰️ndp/codegen.py` in the ticket; the Rust and Ajv tests keep both in sync.", "",
"## 1. Conflicts first (repo and spec disagree)", "",
"Source: `🔍️audits/ndp-crosscheck-en1992-en1995-en1996.md` (aliases such as `H92:78` are defined there). Candidate A is the spec value, candidate B the value currently encoded in the repo (file:line).", "",
"| Marker | Cross-check id | Spec line | Item | Candidate A (spec) | Candidate B (repo, file:line) | Registry ids |", "|---|---|---|---|---|---|---|"]
for c in conflicts:
    ref = f"{spec[c[0][:3]]}:{re.match(r'\d+', c[1]).group(0)}"
    o.append(f"| conflict | {c[0]} | {esc(c[1])} | {esc(c[2])} | {esc(c[3])} | {esc(c[4])} | {', '.join('`'+i+'`' for i in byline[ref])} |")
o += ["", "## 2. All unverified values by code", ""]
bycode = collections.defaultdict(list)
for e in reg: bycode[e["code"]].append(e)
for code in sorted(bycode):
    o += [f"### {code} ({len(bycode[code])})", "", "| Registry id | Code | Clause | Symbol | Value used (spec wording) | Source to check | Spec line | Flag | Public evidence (non-authoritative) |", "|---|---|---|---|---|---|---|---|---|"]
    for e in bycode[code]:
        s = e["source"]
        o.append(f"| `{e['id']}` | {e['code']} | {esc(e['clause'])} | {esc(e['symbol'])} | {esc(e['asWritten'])} | {esc(s['document'])}, {esc(s['locator'])} | {e['specRef']} | {'conflict ' + ','.join(cmark[e['id']]) if e['id'] in cmark else ''} | {esc(e.get('evidence',''))} |")
    o.append("")
open(sys.argv[3], "w").write("\n".join(o))
print(len(o))
