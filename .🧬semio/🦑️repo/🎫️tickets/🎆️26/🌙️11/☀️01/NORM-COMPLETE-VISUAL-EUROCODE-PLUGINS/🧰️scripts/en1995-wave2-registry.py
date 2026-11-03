#!/usr/bin/env python3
"""EN 1995 wave 2 part 2: fixtures for the new leaves, hand-fix of every existing fixture to the new document shape, registries, test lists, workspace wiring, dsl assets."""
import json, os, re, glob, copy, sys
exec(open(os.path.dirname(os.path.abspath(__file__)) + "/en1995-wave2-leaves.py").read().split("if os.path.exists(MUT + L[0][\"dir\"])")[0])
def edit(p, f):
    s = rd(p); n = f(s); assert n != s, p; wr(p, n)
if os.path.exists(FIX + L[0]["dir"]): sys.exit("registry already applied")
E = {"removed": [], "inserted": [], "moved": [], "modified": []}
def empty_diff(): return {"annex": None, "members": copy.deepcopy(E), "connections": copy.deepcopy(E), "ulsEquation": None, "cases": copy.deepcopy(E)}
def case_patch(**kw):
    p = {"labelEn": None, "labelDe": None, "entryId": None, "numbers": copy.deepcopy(E), "texts": copy.deepcopy(E)}; p.update(kw); return p
def jdump(o): return json.dumps(o, indent=2, ensure_ascii=False)

def upgrade_snapshot(d):
    if "ulsEquation" not in d: d["ulsEquation"] = "6.10"; d["cases"] = [copy.deepcopy(CASE)]
    return d
def upgrade_diff(d):
    if "ulsEquation" not in d: d["ulsEquation"] = None; d["cases"] = copy.deepcopy(E)
    return d
n = 0
for p in glob.glob(FIX + "**/*.json", recursive=True):
    if "/📸️snapshot/" in p: f = upgrade_snapshot
    elif "/🔺️diff/" in p: f = upgrade_diff
    else: continue
    s = rd(p); d = json.loads(s); nl = s.endswith("\n"); wr(p, jdump(f(d)) + ("\n" if nl else "")); n += 1
print("upgraded", n)

base_p = FIX + "🔩️change-connection-action-fk/✅apply/🔣️.before.json"
BASE = json.loads(rd(base_p))
def mk(kind, payload, mutate, diff_cases=None, diff_uls=None):
    l = next(x for x in L if x["kind"] == kind)
    before = copy.deepcopy(BASE); after = copy.deepcopy(BASE); mutate(after)
    diff = empty_diff()
    if diff_cases is not None: diff["cases"] = diff_cases
    if diff_uls is not None: diff["ulsEquation"] = diff_uls
    d = FIX + l["dir"] + "/✅apply/"
    wr(d + "🔣️.before.json", jdump(before)); wr(d + "🔣️.after.json", jdump(after))
    wr(d + "🔣️.mutation.json", jdump({l["T"]: payload})); wr(d + "🔣️.outcome.json", jdump({"status": "applied"}))
    wr(d + "🔣️.diff.json", jdump(diff))
def delta(**kw): x = copy.deepcopy(E); x.update(kw); return x
def mod(cid, patch): return {"id": cid, "patch": patch}
C = "case-kser"
mk("change-uls-equation", {"newValue": "6.10ab"}, lambda s: s.__setitem__("ulsEquation", "6.10ab"), diff_uls="6.10ab")
mk("insert-case", {"index": 99, "case": NEWCASE}, lambda s: s["cases"].append(copy.deepcopy(NEWCASE)), delta(inserted=[{"index": 1, "row": NEWCASE}]))
mk("remove-case", {"index": 0}, lambda s: s["cases"].pop(0), delta(removed=[{"id": C, "index": 0}]))
mk("change-case-label-en", {"caseId": C, "newValue": "Slip modulus of the support dowels (revised)"}, lambda s: s["cases"][0].__setitem__("labelEn", "Slip modulus of the support dowels (revised)"), delta(modified=[mod(C, case_patch(labelEn="Slip modulus of the support dowels (revised)"))]))
mk("change-case-label-de", {"caseId": C, "newValue": "Verschiebungsmodul der Auflagerdübel (überarbeitet)"}, lambda s: s["cases"][0].__setitem__("labelDe", "Verschiebungsmodul der Auflagerdübel (überarbeitet)"), delta(modified=[mod(C, case_patch(labelDe="Verschiebungsmodul der Auflagerdübel (überarbeitet)"))]))
mk("change-case-entry-id", {"caseId": C, "newValue": "T-70"}, lambda s: s["cases"][0].__setitem__("entryId", "T-70"), delta(modified=[mod(C, case_patch(entryId="T-70"))]))
newn = {"id": "f_u", "value": 400.0}; newt = {"id": "class", "value": "C24"}
mk("insert-case-number", {"caseId": C, "index": 99, "number": newn}, lambda s: s["cases"][0]["numbers"].append(copy.deepcopy(newn)), delta(modified=[mod(C, case_patch(numbers=delta(inserted=[{"index": 2, "row": newn}])))]))
mk("remove-case-number", {"caseId": C, "index": 1}, lambda s: s["cases"][0]["numbers"].pop(1), delta(modified=[mod(C, case_patch(numbers=delta(removed=[{"id": "rho_m1", "index": 1}])))]))
mk("change-case-number-value", {"caseId": C, "numberId": "d", "newValue": 16.0}, lambda s: s["cases"][0]["numbers"][0].__setitem__("value", 16.0), delta(modified=[mod(C, case_patch(numbers=delta(modified=[mod("d", {"value": 16.0})])))]))
mk("insert-case-text", {"caseId": C, "index": 99, "text": newt}, lambda s: s["cases"][0]["texts"].append(copy.deepcopy(newt)), delta(modified=[mod(C, case_patch(texts=delta(inserted=[{"index": 1, "row": newt}])))]))
mk("remove-case-text", {"caseId": C, "index": 0}, lambda s: s["cases"][0]["texts"].pop(0), delta(modified=[mod(C, case_patch(texts=delta(removed=[{"id": "kind", "index": 0}])))]))
mk("change-case-text-value", {"caseId": C, "textId": "kind", "newValue": "nail"}, lambda s: s["cases"][0]["texts"][0].__setitem__("value", "nail"), delta(modified=[mod(C, case_patch(texts=delta(modified=[mod("kind", {"value": "nail"})])))]))

KIND_M = "change-member-m-crit"
p = ANY + "🔮️oracles/🔣️.json"; d = json.loads(rd(p)); cat = d["mutationCatalogs"][0]
cat["vectors"] = [v for v in cat["vectors"] if v["mutationId"] != KIND_M]; cat["kinds"] = [k for k in cat["kinds"] if k != KIND_M]
man = d["mutationManifests"][0]; man["mutations"] = [m for m in man["mutations"] if m["id"] != KIND_M]
tmpl_man = next(m for m in man["mutations"] if m["id"] == "change-connection-action-fk")
for l in L:
    cat["vectors"].append({"mutationId": l["kind"], "sourceMutationDirectoryName": l["dir"], "mutationDirectoryName": l["dir"], "scenarios": [{"id": l["kind"], "directoryName": "✅apply"}]})
    cat["kinds"].append(l["kind"])
    m = copy.deepcopy(tmpl_man); m["id"] = l["kind"]; m["productionDispatch"] = {"operation": l["kind"], "bridgeVersion": 1, "variant": l["T"]}; man["mutations"].append(m)
wr(p, jdump(d) + "\n")
p = ROOT + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"; raw = rd(p); d = json.loads(raw)
rows = [r for r in d["rows"] if not (r.get("artifact") == "🪵️en1995" and r.get("kind") == KIND_M)]
idx = max(i for i, r in enumerate(rows) if r.get("artifact") == "🪵️en1995" and r.get("kind") == "change-connection-action-fk")
new = [{"aggregateVariant": l["T"], "artifact": "🪵️en1995", "entity": l["entity"], "kind": l["kind"], "module": l["dir"], "physicalLayout": "direct", "record": l["record"], "source": "🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/" + l["dir"] + "/🦀️.rs", "standard": "🔖️1", "subset": "✳️any", "type": l["T"], "verb": l["verb"]} for l in L]
d["rows"] = rows[:idx + 1] + new + rows[idx + 1:]
wr(p, jdump(d) + ("\n" if raw.endswith("\n") else ""))
MT = ANY + "🧪️tests/🪵️mutate-en1995-1/"
def f(s):
    s = re.sub(r'    \("change-member-m-crit", \(\n(?:        include_str!.*\n){4}    \)\),\n', "", s)
    s = s.replace('    "change-member-m-crit",\n', "")
    s = s.replace('    "change-connection-action-fk",\n];', '    "change-connection-action-fk",\n' + "".join(f'    "{l["kind"]}",\n' for l in L) + "];", 1)
    ent = "".join(f'    ("{l["kind"]}", (\n' + "".join(f'        include_str!("../🧫️fixtures/🧬️mutations/{l["dir"]}/✅apply/🔣️.{x}.json"),\n' for x in ["before", "mutation", "after", "outcome"]) + "    )),\n" for l in L)
    m = re.search(r'    \("change-connection-action-fk", \(\n(?:        include_str!.*\n){4}    \)\),\n', s)
    return s[:m.end()] + ent + s[m.end():]
edit(MT + "🦀️.rs", f)
def f(s):
    s = s.replace('    "change-member-m-crit",\n', "").replace('    "change-member-m-crit": ("⚠️change-member-m-crit", "✅apply"),\n', "")
    s = s.replace('    "change-connection-action-fk",\n', '    "change-connection-action-fk",\n' + "".join(f'    "{l["kind"]}",\n' for l in L), 1)
    return s.replace('    "change-connection-action-fk": ("🔩️change-connection-action-fk", "✅apply"),\n', '    "change-connection-action-fk": ("🔩️change-connection-action-fk", "✅apply"),\n' + "".join(f'    "{l["kind"]}": ("{l["dir"]}", "✅apply"),\n' for l in L), 1)
edit(MT + "🐍️.py", f)
def f(s):
    s = re.sub(r"      \| change-member-m-crit .*\n", "", s)
    row = "      | change-connection-action-fk      | 🔩️change-connection-action-fk      | ✅apply  |\n"
    s = s.replace(row, row + "".join(f"      | {l['kind']} | {l['dir']} | ✅apply  |\n" for l in L))
    return s.replace("The 66 `✅apply` vectors", f"The {66 - 1 + len(L)} `✅apply` vectors")
edit(MT + "🥒️.feature", f)

def val(l):
    c = 'base.cases[0].id.clone()'
    pk = "semio_s_artifact_norm_en1995_mut_cases::" + l["mod"] + "::" + l["T"]
    k = l["kind"]
    body = {
      "change-uls-equation": 'new_value: "6.10ab".into()',
      "insert-case": 'index: 99, case: semio_s_artifact_norm_en1995_model::CatalogueCase { id: "case-extra".into(), ..base.cases[0].clone() }',
      "remove-case": 'index: 0',
      "change-case-label-en": f'case_id: {c}, new_value: "Revised".into()',
      "change-case-label-de": f'case_id: {c}, new_value: "Überarbeitet".into()',
      "change-case-entry-id": f'case_id: {c}, new_value: "T-70".into()',
      "insert-case-number": f'case_id: {c}, index: 99, number: semio_s_artifact_norm_en1995_model::CaseNumber {{ id: "f_u".into(), value: 400.0 }}',
      "remove-case-number": f'case_id: {c}, index: 0',
      "change-case-number-value": f'case_id: {c}, number_id: base.cases[0].numbers[0].id.clone(), new_value: 16.0',
      "insert-case-text": f'case_id: {c}, index: 99, text: semio_s_artifact_norm_en1995_model::CaseText {{ id: "class".into(), value: "C24".into() }}',
      "remove-case-text": f'case_id: {c}, index: 0',
      "change-case-text-value": f'case_id: {c}, text_id: base.cases[0].texts[0].id.clone(), new_value: "nail".into()',
    }[k]
    return f"        En1995Mutation::{l['T']}({pk} {{ {body} }}),\n"
vals = "".join(val(l) for l in L)
for tp in [MUT + "🧪️tests/🔬️unit/🦀️.rs", ANY + "🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs", ANY + "🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️op-round-trip/🦀️.rs"]:
    def f(s):
        i = s.index("En1995Mutation::ChangeConnectionActionFK(")
        j = s.index("\n", i) + 1
        return s[:j] + vals + s[j:]
    edit(tp, f)

def f(s):
    s = s.replace('    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡️joint-actions/📦️packages/🦀️rust",\n', '    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡️joint-actions/📦️packages/🦀️rust",\n    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📇️cases/📦️packages/🦀️rust",\n', 1)
    return s.replace("[workspace.dependencies.semio-s-artifact-norm-en1995-mut-connections]", "[workspace.dependencies.semio-s-artifact-norm-en1995-mut-cases]\npath = \"🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📇️cases/📦️packages/🦀️rust\"\n\n[workspace.dependencies.semio-s-artifact-norm-en1995-mut-connections]", 1)
edit(ART + "Cargo.toml", f)
for cp in [MUT + "📦️packages/🦀️rust/Cargo.toml", ANY + "🚪️io/📦️packages/🦀️rust/Cargo.toml", ANY + "🚪️io/📝️text/📦️packages/🦀️rust/Cargo.toml"]:
    edit(cp, lambda s: s.replace("semio-s-artifact-norm-en1995-mut-connections = { workspace = true }\n", "semio-s-artifact-norm-en1995-mut-cases = { workspace = true }\nsemio-s-artifact-norm-en1995-mut-connections = { workspace = true }\n", 1))
edit(ART + "📦️packages/🦀️rust/📜️script.ts", lambda s: s.replace('"semio-s-artifact-norm-en1995-mut-connections"', '"semio-s-artifact-norm-en1995-mut-cases","semio-s-artifact-norm-en1995-mut-connections"', 1))
def f(s):
    s = s.replace(' "semio-s-artifact-norm-en1995-mut-connections",\n', ' "semio-s-artifact-norm-en1995-mut-cases",\n "semio-s-artifact-norm-en1995-mut-connections",\n')
    blk = '[[package]]\nname = "semio-s-artifact-norm-en1995-mut-connections"'
    new = '[[package]]\nname = "semio-s-artifact-norm-en1995-mut-cases"\nversion = "0.1.0"\ndependencies = [\n "semio-framework-os-kernel",\n "semio-framework-pack-json",\n "semio-framework-schema",\n "semio-framework-ui-locale",\n "semio-framework-value",\n "semio-framework-value-derive",\n "semio-s-artifact-norm-en1995-diff",\n "semio-s-artifact-norm-en1995-model",\n "serde",\n]\n\n'
    assert blk in s
    return s.replace(blk, new + blk, 1)
edit(ART + "Cargo.lock", f)

def toks(line):
    out = []; i = 0
    while i < len(line):
        if line[i] == " ": i += 1; continue
        if line[i] == '"':
            j = i + 1
            while line[j] != '"': j += 1
            out.append(line[i:j + 1]); i = j + 1
        elif line[i] == "[": out.append(line[i:]); break
        else:
            j = line.index(" ", i) if " " in line[i:] else len(line); out.append(line[i:j]); i = j
    return out
def fix_dsl(p, with_case):
    lines = rd(p).split("\n"); out = []; mode = None
    for ln in lines:
        if ln.startswith("members ["):
            ln = ln.replace(" m-crit-nm:NUM", ""); mode = "members"
        elif ln.startswith("connections ["): mode = "connections"
        elif mode == "members" and ln.startswith("  ") and ln.strip() != "}":
            t = toks(ln.strip()); del t[17]; ln = "  " + " ".join(t)
        out.append(ln)
    s = "\n".join(out)
    hdr = "cases [id:TEXT label-en:TEXT label-de:TEXT entry-id:TEXT numbers:TABLE texts:TABLE] {\n"
    body = '  case-kser "Slip modulus of the support dowels" "Verschiebungsmodul der Auflagerdübel" T-83 [ {id=d value=12} {id=rho_m1 value=420} ] [ {id=kind value=dowel} ]\n' if with_case else ""
    wr(p, s.rstrip("\n") + '\nuls-equation="6.10"\n' + hdr + body + "}\n")
for a, wc in [("🏠️glulam-floor-beam", True), ("🌉️glulam-footbridge", False), ("❌️multi-fail-timber", False), ("⚠️overloaded-footbridge", False)]:
    fix_dsl(ANY + f"🖼️assets/{a}/{a}/🗣️.dsl.semio", wc)
print("registry done")
