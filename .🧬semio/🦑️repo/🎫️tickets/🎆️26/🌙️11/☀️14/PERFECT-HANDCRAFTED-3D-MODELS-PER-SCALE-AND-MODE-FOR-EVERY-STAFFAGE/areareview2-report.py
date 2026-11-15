"""Writes the 'Reviewer 2' section of area-decisions.md (replacing only that section), the findings in model-findings.md and the pointer block in area-review.md."""
import glob, importlib.util, json, os, subprocess, re
T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *[".."] * 7))
ROOT = os.path.join(REPO, "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
s = importlib.util.spec_from_file_location("dec2", os.path.join(T, "areareview2-decisions.py")); dec = importlib.util.module_from_spec(s); s.loader.exec_module(dec)

RULE = """## Reviewer 2 (machinery, vehicles, components, objects; ruling C29)

Scripts: `areareview2-decisions.py` (decisions with one reason each), `areareview2-apply.py` (surgical: the `areaM2` number token, an inserted `areaBasis` line, existing number tokens of the default pose; JSON-compared, atomic, re-read before writing), `areareview2-unsection.py` (C25), `areareview2-dump.py` / `areareview2-brief.py` (hand-calculation aids: closed analytic surface per solid), `areareview2-report.py` (this section). The rows of the first reviewer for these four categories are superseded by the table below (its rows above stay as written).

### Rule derived from the solver (C29)

Read in `🌡️thermal/🧱️balance`, `🔬️physics` (`viewFactors`, `SOLAR_GEOMETRY_FACTOR`, `naturalNusselt`) and `solveSteady`:

- **R-A, passive, hydronic, coupled and water regions:** the surface temperature is one balance per m2; `areaM2` only weights the Watt totals, the merge colour and the coupled draw. The pose then applies `exposedFraction` and `contact.fraction` on top, so `areaM2` is the TOTAL outer area: every face of the union of the region's own solids, including faces covered by other regions, the ground, a wall or a rider, and both faces of a free sheet; the pose carries the covering. Excluded only: `sectionFaces` (C25), faces between solids of the same region, and the bore of a sealed hollow section or the inside of an enclosed casing (a void, not a face towards the environment).
- **R-B, operating regions:** `q = heatW x level / areaM2` sets the temperature, so `areaM2` is the surface the heat crosses. `heatW` is the datum and stays; the flux density follows the honest outer area (outer faces of a heated shell: the inside of a hood or casing is the source side). The value is the TOTAL outer area of the part otherwise.
- **R-C, conditioned regions:** `(interiorTempC - Ts) / (sum R layers + interior film)` is per m2 of the plate: the region is a layered plate with ONE interior side (the conditioned boundary, a C25 b face) and the outdoor side. `areaM2` is the outward faces (outer face plus the exposed edges), NOT the total: the interior-side face of the layered solid must be a `sectionFaces` face or the model exceeds the value. This is what the sibling regions of the catalog already do (curtain-wall glass 2.60 = one face, wall-face 4.87, sliding-door leaf-glass 1.62). The committed one-face values of the sliding door (0.28, 0.3, 0.42) were the right idea but counted one face only and omitted the edges.
- **R-D, `geometry`:** `horizontal-plate-up/-down` only selects the sky/ground view, the solar factor and the convection correlation per m2; it does not make the area one-sided, the opposite face of a horizontal plate is counted (market-parasol has two regions only because the two faces face different environments). A one-sided value is right only where the other face is a C25 face or the plate is conditioned (R-C).
- **Exchange areas** (fins, honeycomb, flame, plume, spray) keep `areaBasis: "exchange"` and a hand value (C19).
- Pose fractions: edited only where the decided total makes the committed fraction wrong (list at the end of this section).

"""

def head(slug, region):
    p = glob.glob(f"{ROOT}/*/{slug}/🔣️.json")[0]
    rel = os.path.relpath(p, REPO)
    try:
        doc = json.loads(subprocess.check_output(["git", "-C", REPO, "show", "HEAD:" + rel], stderr=subprocess.DEVNULL))["asset"]
    except Exception:
        return None, None
    now = json.load(open(p))["asset"]
    c = next((r for r in doc["thermal"]["regions"] if r["id"] == region), None)
    n = next((r for r in now["thermal"]["regions"] if r["id"] == region), None)
    return (c or {}).get("areaM2"), n
rows = []
cnt = {}
for (slug, region), (act, val, why) in sorted(dec.R.items()):
    com, n = head(slug, region)
    cat = os.path.basename(os.path.dirname(os.path.dirname(glob.glob(f"{ROOT}/*/{slug}/🔣️.json")[0])))
    cur = n["areaM2"] if n else None
    name = {"A": "ACCEPT", "S": "SET", "X": "exchange only", "SX": "SET + exchange", "D": "DEFER (no model in the file)"}[act]
    cnt[name] = cnt.get(name, 0) + 1
    rows.append(f"| {cat}/{slug} | {region} | {n['kind'] if n else ''} | {n['model'] if n else ''} | {com} | {cur} | {name} | {why} |")
out = RULE + f"### Totals\n\n{len(dec.R)} decided rows: " + ", ".join(f"{k} {v}" for k, v in sorted(cnt.items())) + ".\n\n### Decisions (committed = HEAD; value = in the file after this review)\n\n| asset | region | kind | model | committed m2 | value m2 | decision | reason (independent hand calculation of the real object) |\n|---|---|---|---|---|---|---|---|\n" + "\n".join(rows) + "\n\n" + open(os.path.join(T, "areareview2-crosscheck.md"), encoding="utf8").read() + "### C25 `sectionFaces`\n\n- solar-thermal-collector: `th-glass` and `th-absorber-plate` (`th-absorber` at 1:200/1:500) lose `-v` at all four scales (asset and source, `areareview2-unsection.py`); the air gap between glass and absorber is free air that the collector physics is about. Areas decided: glazing 3.74, absorber 4.1 (heatW -620 W stays: -151 W/m2), housing 5.0 (the cavity faces count now).\n- city-car `th-tailgate +v` at 1:100/200/500: judged LEGITIMATE (the inner side of a closed tailgate faces the enclosed luggage space, C25 b); the 1:50 tailgate must carry the same marker, 0.75 is the outward panel.\n- city-car, delivery-van, city-bus shell undersides: already removed by reviewer 1 (verified: no `sectionFaces` left on them); tram `sectionFaces` are the interior faces of conditioned shells (R-C, legitimate).\n\n### Pose fractions edited (default pose, existing members only)\n\n" + "\n".join(f"- {a}/{b} `{c}` -> {d}: {e}" for a, b, c, d, e in dec.POSES) + "\n"
p = os.path.join(T, "area-decisions.md")
t = open(p, encoding="utf8").read()
i = t.find("## Reviewer 2 (")
j = t.find("\n## ", i + 5) if i >= 0 else -1
if i >= 0:
    t = t[:i] + out + (t[j:] if j > 0 else "")
else:
    t = t.rstrip("\n") + "\n\n" + out
open(p, "w", encoding="utf8").write(t)
print("rows", len(rows))
