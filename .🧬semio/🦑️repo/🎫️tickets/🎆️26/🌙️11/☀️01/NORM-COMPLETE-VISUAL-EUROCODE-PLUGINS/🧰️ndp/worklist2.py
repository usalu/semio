import json, re, sys, collections, pathlib
REG = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json")
OUT = pathlib.Path(sys.argv[1])
old = OUT.read_text()
conflicts = old[old.index("## 1. Conflicts first"):min(i for i in (old.find("## 1b."), old.index("## 2.")) if i >= 0)] if "## 1. Conflicts first" in old else ""
conflicts = re.sub(r"ndp\.en1995\.t-01\.", "ndp.en1995.strength.", conflicts).rstrip() + "\n"
entries = json.load(open(REG))["entries"]
open_entries = [e for e in entries if e["status"] != "verified"]
done = [e for e in entries if e["status"] == "verified"]
def esc(x): return str(x).replace("|", "\\|").replace("\n", " ").strip()
def val(x):
    if x is None: return "-"
    return f"{x:g}" if isinstance(x, (int, float)) else esc(x)[:60]
def nat(clause):
    return [int(t) if t.isdigit() else t for t in re.split(r"(\d+)", clause)]
o = ["# NDP Verification Worklist", "",
"Every registry entry that is still `unverified`, grouped by standard, part and clause, so a human can walk the printed standard from front to back. Registry: `⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json`. Until a human sets `verified`, every check citing the id yields `UnverifiedBasis`, never a compliant verdict.", "",
f"Totals: {len(entries)} registry entries, {len(open_entries)} unverified (listed below), {len(done)} marked verified (section 3, re-check the flagged ones).", "",
"How to verify: open the printed standard or national annex at the clause heading, compare `EN` (recommended value) and `DE` (NA-DE value) with the print, then edit the entry in `🔣️.json` (`status: verified`, `source.edition`, `source.locator`, values) and regenerate the Rust rows with `python3 -I TICKET/🧰️ndp/codegen.py 🔣️.json 🗃️table.rs` (the Rust, Ajv and Python adoption tests keep both in sync). `-` means the annex gives no number (table or formula text, see the registry `asWritten`). Rows whose note reads `as used by the artifact` carry the number the code uses today; the spec line (`file:line` in `📐️specs/`) is given where one exists.", "",
conflicts, "## 1b. Policy placeholders (no normative value exists in the repo spec)", "", "These ids are modelling choices of an artifact owner, not values of any standard. Replace them with the printed wording or remove the feature.", "", *[f"- `{e['id']}` (EN {val(e['recommended'])}, DE {val(e['naDe'])}): {esc(e['specRef'])[:200]}" for e in open_entries if e.get("evidence", "").startswith("POLICY PLACEHOLDER")], "", "## 2. All unverified entries by standard, part and clause", ""]
by_code = collections.defaultdict(list)
for e in open_entries: by_code[e["code"]].append(e)
for code in sorted(by_code):
    es = by_code[code]
    o += [f"### {code} ({len(es)})", ""]
    by_part = collections.defaultdict(list)
    for e in es: by_part[e.get("part") or "general"].append(e)
    for part in sorted(by_part):
        o += [f"#### {code}, part {part} ({len(by_part[part])})" if part != "general" else f"#### {code}, part not stated ({len(by_part[part])})", ""]
        by_clause = collections.defaultdict(list)
        for e in by_part[part]: by_clause[e["clause"] or "?"].append(e)
        for clause in sorted(by_clause, key=nat):
            group = by_clause[clause]
            docs = sorted({e["source"]["document"] for e in group})
            o += [f"##### Clause {esc(clause)} ({len(group)}; {esc(', '.join(docs))})", "", "| Registry id | Symbol | Unit | EN | DE | Locator | Spec / note |", "|---|---|---|---|---|---|---|"]
            for e in sorted(group, key=lambda e: e["id"]):
                note = e["asWritten"] if not e["specRef"].startswith("wave2 ndp") else ""
                ref = e["specRef"] if not e["specRef"].startswith("wave2 ndp") else ""
                text = (ref + (": " if ref and note else "") + note)[:150]
                flag = " **conflict**" if "conflict" in e.get("evidence", "")[:12] else ""
                o.append(f"| `{e['id'].split('.', 2)[2]}` | {esc(e['symbol'])[:40]} | {esc(e['unit'])} | {val(e['recommended'])} | {val(e['naDe'])} | {esc(e['source']['locator'])[:50]} | {esc(text)}{flag} |")
            o.append("")
o += ["## 3. Marked verified in the registry (re-check)", "",
"These rows were flagged verified in the code before the registry existed (EN 1995 strength, k_mod and k_def rows) or by a later reviewer; they gate nothing wrongly only while the numbers match the print. `review-en1995-en1996.md` doubts the C16 and C30 numbers, check those first.", ""]
by = collections.defaultdict(list)
for e in done: by[e["id"].rsplit(".", 1)[0] if e["id"].count(".") > 3 else e["id"]].append(e)
for key in sorted(by): o.append(f"- `{key}` ({len(by[key])} cells)")
OUT.write_text("\n".join(o) + "\n")
print(len(open_entries), len(done), OUT.stat().st_size)
