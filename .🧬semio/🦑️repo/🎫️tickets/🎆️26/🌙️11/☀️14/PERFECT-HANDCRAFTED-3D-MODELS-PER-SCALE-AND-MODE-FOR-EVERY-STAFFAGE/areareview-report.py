"""Writes area-decisions.md and model-findings.md (reviewer section) and marks the decided rows of area-review.md from 🗑️generated/areareview/state.json."""
import json, os, re

T = os.path.dirname(os.path.abspath(__file__))
GEN = os.path.join(T, "🗑️generated", "areareview")
state = json.load(open(os.path.join(GEN, "state.json")))
rows = [v for k, v in state.items() if not k.startswith("_")]
extra = os.path.join(T, "areareview-consequences.md")
LABEL = {"A": "ACCEPT", "RX": "RESTORE + exchange", "RF": "RESTORE + model defect", "S": "SET", "SX": "SET + exchange", "X": "exchange only", "RB": "RESTORE (C18 body)", "N": "KEEP", "DEFER": "DEFER (no 1:50 model yet)"}
g = lambda x: f"{x:g}" if isinstance(x, (int, float)) else str(x)

def line(v):
    dec = v["final"] if v["act"] != "DEFER" else v["author"]
    return f"| {v['cat'][-12:].strip()} {v['slug']} | {v['region']} | {v['kind']} | {v['model']} | {g(v['committed'])} | {g(v['author'])} | {g(dec)}{' (exchange)' if v['basis'] else ''} | {LABEL[v['act']]}{' (author withdrew)' if v['act'] == 'A' and v['final'] == v['committed'] and v['author'] != v['committed'] else ''} | {v['reason'].replace('|', '/')} |"

rows.sort(key=lambda v: (v["cat"], v["slug"], v["region"]))
changed = [v for v in rows if v["act"] != "DEFER" and (v["final"] != v["committed"] or v["basis"])]
by = {k: sum(1 for v in rows if v["act"] == k) for k in LABEL}

flux = []
for v in rows:
    if v["act"] in ("A", "S", "SX") and v["heatW"] not in (None, 0) and v["final"] != v["committed"]:
        q0, q1 = v["heatW"] / v["committed"], v["heatW"] / v["final"]
        if not 0.67 <= q1 / q0 <= 1.5:
            flux.append((v, q0, q1))

out = ["# Area decisions (reviewer of thermal areas)", "",
       "Reviewer pass over every committed-versus-working-tree `areaM2` difference (`area-audit.md`) and every row of `area-review.md`. Rulings: C1, C15, C16, C18, C19 in `design.md`. Scripts: `areareview-decisions.py` (the decisions, one reason each), `areareview-apply.py` (applies them: only the number token of `areaM2` or an inserted `areaBasis` line, checked by JSON comparison, written atomically, skipped when the file changed meanwhile), `areareview-report.py` (this file). Findings for the model authors are appended to `model-findings.md`.", "",
       "## Method (short)", "", "`areaM2` weights only for passive regions (temperature per m2 is independent of it) but sets the flux density `q = heatW / areaM2` of operating regions, the draw of coupled regions and the segment factor `f = areaM2 / skinAreaM2` of body regions. Per changed value decide ACCEPT (hand calculation of the real object from `dimensions`, the drawing and standard sections agrees within ~30 %), RESTORE + `areaBasis: exchange` (fins, plates, pads, matrices, aggregates), RESTORE + model finding (poor model, named in `model-findings.md`), or SET (hand value). Write the decision in `areareview-decisions.py` as `d(slug, region, ACT, value, reason, finding)` and run `python3 areareview-apply.py` (audit + surgical write + state), then `python3 areareview-report.py` (this file, `model-findings.md`, marks in `area-review.md`); the second reviewer appends below the line `## Reviewer 2`, which the report script preserves. Ruling C29 (areaM2 = TOTAL outer area of the region's own solids, covering by riders, hands, seats, ground, walls or other regions NOT excluded; exposure is the pose's `exposedFraction`/`contact.fraction`; `heatW` stays the physical datum; body totals stay tied to BodyPhysiology): decide every row at the total by hand calculation, reasons start with C29 total. Removal of illegitimate `sectionFaces`: `areareview-unsection.py`; pose fractions: `areareview-pose.py`; layer order: `areareview-layerfix.py`.", "",
       "## What `areaM2` does in the solver (from the code)", "",
       "Read from `🔨️modules/🌡️thermal` (`🟦️.ts`, `🧱️balance`, `🫀️physiology`, `🛋️comfort`, `🍃️canopy`) and the `ThermalRegion` schema.", "",
       "- **Passive surface, glazing, tyre, wall, water (no `operating`, no `hydronic`, no `coupled`)**: the surface temperature is the root of one balance per m² (`solveSteady`: inflow + absorbed solar = radiation + convection + latent), with `characteristicLengthM` and `geometry` choosing the natural-convection correlation, `skyViewFactor` and `geometry` the radiant environment, the layers the series resistance towards the interior or heat source. **`areaM2` does not enter the temperature.** It only weights: the Watt totals `r.areaM2 × exposed` (`netRadiativeW`, `sensibleW`, `latentW`, phase change), the area-weighted merge colour of small-scale shapes (`Σ areaM2·exposedFraction·value`), and the totals shown in the legend.",
       "- **Operating regions**: the heat released inside enters as `q = heatW × level / areaM2` (`inflowOf`). **A change of `areaM2` rescales the heat flux density, hence the surface temperature, one to one.** A halved area doubles `q`; this is why operating regions are only accepted where the area is the honest outer surface of the heated part (the heat leaves through the outer face), and why changes beyond a factor 1.5 in `q` are listed in the follow-up table below (the `heatW` that keeps the committed flux density).",
       "- **Coupled and hydronic regions**: the draw of a coupled region on its source (`areaM2_dep × inflow / areaM2_source`) scales with the ratio of the two areas.",
       "- **Conditioned**: `(T_interior − T_s)/(ΣR + R_film)` per m², area only weights.",
       "- **Leaf regions**: one-sided leaf area, both faces exchange (`faces = 2`), transpiration through one; `leafAreaIndex` converts to ground area. Area changes the Watt totals only, never the leaf temperature.",
       "- **Body regions**: `f = areaM2 / skinAreaM2` distributes the Gagge two-node core over the segments and `fcl = areaM2 / skinArea` is the clothing area factor in the comfort index (`🛋️comfort`): body areas are physiological and tied to `BodyPhysiology` (C18); changing them moves the whole-body balance.",
       "- `exposedFraction`, `contact.fraction` and `sunlitFraction` are per-frame inputs, so `areaM2` is the *total* outer area and the exposure is applied on top (C1 asks for the exposed surface of the model: buried and contact faces are therefore the validator's, not the solver's, notion; where a region is covered by a rider or a hand the author's exposed value is the honest drawn pose and accepted).", "",
       "## Principles of the decisions", "",
       "1. Hand calculations use `dimensions`, the drawing and the real object (profile perimeters, tube lengths, standard sections), not the 3D model; the model value is accepted where the hand value agrees within ~30 %.",
       "2. Thin sheets standing free in the air (slats, shelves, louvres, canvas, flags, plates) exchange through both faces: both faces count. Enclosure panels whose inner face looks into a cavity, and heated parts whose heat leaves through the outer face (hood, casing, lid, plate), count the outer face only, because `q = heatW/area` is a flux through that face.",
       "3. Exchange areas the envelope cannot show (fruit aggregates, fin packs, plate packs, wetted pads, honeycomb matrices, heat-transfer contact discs) keep or get a hand-derived value and `areaBasis: \"exchange\"` (C19).",
       "4. Where the model is poor (buried or counted twice, detail solids on top of a surface, solids of the wrong shape, missing parts) the committed value is restored and the defect is named in `model-findings.md`.",
       "5. Body-model regions are never changed from the model (C18); the deviation is a model finding.",
       "6. A region whose asset has no authored 1:50 volumes in the file is deferred (C15: corrections only from the 1:50 model) and reviewed again once they exist.", "",
       "## Totals", "",
       f"{len(rows)} decided rows: " + ", ".join(f"{LABEL[k]} {n}" for k, n in by.items() if n) + f". {len(changed)} rows differ from the committed catalog after the decisions (changed value or exchange marker).", "",
       "## Decisions", "",
       "| asset | region | kind | model | committed m2 | author m2 | decided m2 | decision | reason |", "|---|---|---|---|---|---|---|---|---|"] + [line(v) for v in rows] + [""]

out += ["## Operating regions whose heat flux density changes by more than a factor 1.5", "",
        "`q = heatW / areaM2`. The decided area is the honest outer surface; `heatW` is a physical power in most assets (accepted as is, the surface temperature follows) and a tuned input in some (coils, valve stubs). The last column is the `heatW` that keeps the committed flux density; whether to apply it is the thermal author's decision, it is not part of this review.", "",
        "| asset | region | committed m2 | decided m2 | heatW | q committed W/m2 | q decided W/m2 | heatW keeping the old q |", "|---|---|---|---|---|---|---|---|"]
for v, q0, q1 in flux:
    out.append(f"| {v['slug']} | {v['region']} | {g(v['committed'])} | {g(v['final'])} | {g(v['heatW'])} | {q0:.0f} | {q1:.0f} | {v['heatW'] * v['final'] / v['committed']:.3g} |")
out.append("")

import glob, collections
CAT = os.path.join(T, *[".."] * 7, "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
expo = []
cache = {}
for v in rows:
    if v["act"] not in ("A", "S", "SX") or v["final"] == v["committed"]:
        continue
    if v["slug"] not in cache:
        a = json.load(open(glob.glob(f"{CAT}/*/{v['slug']}/🔣️.json")[0]))["asset"]
        m = collections.defaultdict(lambda: [1.0, 0.0])
        for p in a.get("poses", []):
            for rid, r in ((p.get("thermal") or {}).get("regions") or {}).items():
                m[rid][0] = min(m[rid][0], r.get("exposedFraction", 1))
                m[rid][1] = max(m[rid][1], (r.get("contact") or {}).get("fraction", 0))
        cache[v["slug"]] = m
    e, c = cache[v["slug"]].get(v["region"], [1.0, 0.0])
    if e < 0.999 or c > 0:
        expo.append((v, e, c))
out += ["## Exposed-only areas where the poses also reduce the exposure (C1 against the solver)", "",
        "`areaM2` is the exposed surface of the model (C1, enforced by `volumes-thermal-coverage`), while the poses of these regions additionally carry `exposedFraction` below 1 and/or a `contact` share, which the solver applies on top of the area (`outflow = max(e, 0.05) × exchange`, `contact × h_ct`). For passive regions this changes the Watt totals and merge weights only; for the operating ones (heatW column) it also decides how hot the region gets, because `q = heatW / areaM2` is spread over the exposed share only. Superseded by C29 (areaM2 is the total outer area, exposure stays with the pose): the rows of the humans, animals, vegetation, appliances and furniture categories decided at an exposed-only value were re-decided to the hand-calculated total (reasons starting 'C29 total'); the table lists what the poses carry. Original note: the coordinator should either set the pose `exposedFraction` of these regions to 1 where the author already removed the covered faces from the area, or accept totals as area.", "",
        "| asset | region | decided m2 | pose min exposedFraction | pose max contact | heatW |", "|---|---|---|---|---|---|"]
for v, e, c in expo:
    out.append(f"| {v['slug']} | {v['region']} | {g(v['final'])} | {e:g} | {c:g} | {g(v['heatW']) if v['heatW'] is not None else ''} |")
out.append("")

secp = os.path.join(GEN, "sections.md")
if os.path.exists(secp):
    out += [open(secp, encoding="utf8").read().rstrip(), ""]
c27 = os.path.join(T, "areareview-c27c28.md")
if os.path.exists(c27):
    out += [open(c27, encoding="utf8").read().rstrip(), ""]
lo = os.path.join(T, "areareview-layerorder.md")
if os.path.exists(lo):
    out += [open(lo, encoding="utf8").read().rstrip(), ""]
if os.path.exists(extra):
    out += [open(extra, encoding="utf8").read().rstrip(), ""]

pend = state.get("_pending", {"undecided": [], "rereview": []})
out += ["## Open at the last pass", "",
        "Deferred (no 1:50 volumes in the file at review): " + (", ".join(sorted({f"{v['slug']}/{v['region']}" for v in rows if v["act"] == "DEFER"})) or "none") + ".", "",
        "Without decision yet (new or changed rows, decided at the next pass): " + (", ".join(f"{a}/{b}" for a, b, *_ in pend["undecided"]) or "none") + ".", "",
        "Changed again by the author since the decision (re-review): " + (", ".join(f"{a}/{b} ({c} then {d} now {e})" for a, b, c, d, e in pend["rereview"]) or "none") + ".", "",
        "## Passes", "", "| time | audit rows | applied | undecided | re-review |", "|---|---|---|---|---|"]
for p in state.get("_passes", []):
    out.append(f"| {p['time']} | {p['rows']} | {p['applied']} | {p['undecided']} | {p['rereview']} |")
target = os.path.join(T, "area-decisions.md")
keep = ""
if os.path.exists(target):
    old = open(target, encoding="utf8").read()
    k = old.find("\n## Reviewer 2")
    if k >= 0:
        keep = old[k:].rstrip("\n") + "\n"
open(target, "w", encoding="utf8").write("\n".join(out) + "\n" + keep)

mf = os.path.join(T, "model-findings.md")
text = open(mf, encoding="utf8").read()
text = re.sub(r"\n## Area review \(reviewer\)\n.*?(?=\n## |\Z)", "", text, flags=re.S).rstrip("\n") + "\n"
text += "\n## Area review (reviewer)\n" + "".join(f"- {v['finding']}\n" for v in rows if v["finding"])
open(mf, "w", encoding="utf8").write(text)

rv = os.path.join(T, "area-review.md")
idx = {(f"{v['slug']}", v["region"]): v for v in rows}

def mark(raw):
    lines = raw.split("\n")
    for i, l in enumerate(lines):
        if not l.startswith("|") or "---" in l:
            continue
        c = [x.strip() for x in l.strip().strip("|").split("|")]
        if len(c) < 5 or "asset" == c[0]:
            continue
        v = idx.get((c[0].split("/")[-1], c[1]))
        if v is None or "decided:" in l:
            continue
        dec = f"decided: {LABEL[v['act']]} {g(v['final'] if v['act'] != 'DEFER' else v['author'])} (area-decisions.md)"
        if len(c) == 8:
            c[7] = dec
            lines[i] = "| " + " | ".join(c) + " |"
        else:
            lines[i] = l.rstrip().rstrip("|").rstrip() + f" **{dec}** |"
    return "\n".join(lines)

for _ in range(6):
    raw = open(rv, encoding="utf8").read()
    new = mark(raw)
    if new == raw:
        break
    if open(rv, encoding="utf8").read() == raw:
        open(rv, "w", encoding="utf8").write(new)
        break
print("report written:", len(rows), "rows")
