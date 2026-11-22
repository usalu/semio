"""🗒️ The finisher's review notes of 2026-11-22 per started asset, written as `review-notes.json` for `report.py` and the ledger `wave.md`: state before, what was changed, measured reactions, which contact sheets were opened and what stays imperfect.

Usage: python3 review-notes.py
Every statement here was measured or seen in the finishing session, or is marked as taken from the interrupted wave's ledger.
"""
import json
import os

K = os.path.dirname(os.path.abspath(__file__))
ROWS = {l.split("|")[2].strip(): l.split("|")[1].strip() for l in open(os.path.join(K, "assign.md")) if l.startswith("| ") and l[2:4].isdigit()}
STARTED = sorted(os.listdir(os.path.join(K, "assets")))
POWER = {"used-park-lantern": 30, "used-bollard-light": 15, "used-bubbler-stone-fountain": 80}
OPENED_2D = {
    "used-backrest-park-bench", "used-basketball-hoop", "used-beer-garden-set", "used-bicycle-shelter", "used-bollard-light", "used-bubbler-stone-fountain", "used-chess-table", "used-climbing-net-pyramid",
    "used-concrete-bench-block", "used-concrete-table-tennis", "used-dog-waste-station", "used-double-swing",
}
GEAR = "Lifting gear, workers and users are not drawn: the schema has no per-pose presence of solids (request filed in the umbrella `requests.md`); they exist as loads and in the texts only."
WEAR = "Wear below the smallest feature (scratches, stickers, moss, graffiti, gum) is 2D strokes only, by the rule of the brief; the 3D body shows only the wear above the feature."
REFERENCE = "No author's reference table exists; the model was not measured against outside sources in the finishing session."
LIFT = "Lifting pose corrected: the {what} is an applied force of the statics of the lifted part alone instead of a support that shared the weight of the whole rig ({before} before, {after} after; `lift_statics.py`, `reactions.ts`)."

CHANGED = {
    "used-bicycle-shelter": [
        "Published for the first time (manifest row inserted, collection now 29 assets); the author had stopped after writing `distinction.json`.",
        LIFT.format(what="hold of the two workers on the slid-out sheet", before="147 N per hand", after="17.7 N per hand, a quarter of the 7.2 kg sheet each, its rear edge on the glazing bars"),
    ],
    "used-chess-table": [LIFT.format(what="pair of round slings", before="3269.7 N on each of two slings and two soil contacts", after="1222.4 N per sling, soil 2 x 5316.9 N") + " Key pose `slings-taut` added to the clip `relevel`; lifting anchors are `load` anchors; reference EN 1492-2 added."],
    "used-spring-rocker": [
        LIFT.format(what="four-leg sling", before="510 to 546 N per leg", after="50.1 N per handle leg and 82.5 N per rear leg, 265.2 N in all, footing 2424.8 N"),
        "Real defect found by the statics: both sling pairs (handle ends x 0.27 m, footrest ends x 0.13 m) lay ahead of the centre of mass of the lifted part (x 0.027 m), so the rocker would have tipped backwards on the hook. The rear pair is now choked round the rear spacer under the seat (x -0.1205 m); anchors `lift-foot-*` renamed `lift-rear-*`; key pose `slings-taut` added.",
    ],
    "used-pump-drinking-fountain": [LIFT.format(what="crane hook on the pump head", before="5263.5 N on the hook", after="399.6 N on the hook, slab 10590.5 N") + " Key pose `sling-taut` added to the clip `maintenance`."],
    "used-wayfinding-signpost": [LIFT.format(what="pair of hands under finger 2", before="840.5 N and 807.6 N on the hands", after="6.6 N per hand, footing 2543.4 N") + " The hands were moved to lie either side of the finger's centre of mass (local x 0.20 and 0.413 m instead of 0.30 and 0.52 m, which put 97 % on one hand); key pose `hands-under` added."],
    "used-outdoor-fitness-station": [LIFT.format(what="pair of workers' hands under the high bar", before="2926.2 N and 3092.5 N on the hands", after="14.0 N per worker") + " Key pose `hands-on` added to the clip `bar-change`."],
    "used-sandpit-digger": [LIFT.format(what="crane hook in the eye nut", before="4132.6 N on the hook", after="276.1 N on the hook") + " Key pose `hook-taut` added to the clip `maintenance-lift`."],
    "used-seesaw": [LIFT.format(what="pair of round slings on the beam", before="233.0 N on each of five supports", after="380.6 N per sling, 134.6 N on each of footing and tyres")],
    "used-tree-surround-bench": [LIFT.format(what="pair of hands at the outer slat ends of segment 1", before="179 N per hand", after="63.6 N per hand") + " Key pose `hands-on` added to the clip `segment-lift-out`."],
    "used-double-swing": [LIFT.format(what="resting point of the unshackled flat seat", before="293.2 N under the seat", after="38.7 N under the seat, the weight of the seat")],
    "used-slide-tower": [LIFT.format(what="four tie beam ends of the dismantled roof (crane sling, then lawn)", before="1155 to 1344 N per rest", after="230.4 N per rest") + " Key pose `roof-on-crane` added: in the clip `dismantle-roof` the crane now carries the roof from 1 s on instead of the tower supports."],
    "used-park-lantern": [LIFT.format(what="lawn mat under the eave of the lowered column", before="224.7 N on the mat", after="197.4 N on the mat by moments about the hinge pin, foundation 607.4 N")],
    "used-dog-waste-station": [
        "Bag slot lowered from 1.16 m to 1.05 m (bag tab gripped at 1.01 m) so that it lies inside the reach range of 0.85 to 1.05 m of DIN 18040-1 that the asset itself cites; the whole dispenser (housing, mount, clamps, lid, roll, tab, lock, strokes, anchors, bones) moved down by 0.11 m at all four scales.",
        "Consequence: the rain hood of the bin can open only 45 degrees under the dispenser (20 mm clear) instead of 85; joint limit, the poses `maintenance` and `lids-open` and their texts changed accordingly.",
    ],
    "used-concrete-table-tennis": [
        "Footing treatment made the same as rows 01 and 05: the two strip footings are massless bones `footing-a`, `footing-b` with body, thermal (`footings` region, 7.52 m2) and structure solids at all four scales and filled cut shapes in elevation and front (before: thin ground outlines in 2D only at 1:50 and 1:100, nothing in 3D); they stay behind in `maintenance`. A straight ground line was added at 1:200 and 1:500; the front hollow line that would cross the footing was removed.",
        "Footing depth set to the frost-free 0.80 m (author: 0.40 m). `dimensions.heightM` is now 1.7125 m (0.9125 m above ground plus 0.80 m of footing), as the product's extent rules demand; the true-extent symbol and `distinction.json` follow.",
        "With 0.40 m depth the full proof reported `volumes-silhouette` 1:50 front p95M 48.8 mm; the finding is an artefact of the banded raster (it disappears at 0.30, 0.50, 0.60 and 0.80 m with nothing else changed; request filed).",
    ],
}

IMPERFECT = {
    "used-backrest-park-bench": ["Reactions of the lifting pose were already exact (416.5 N per sling): the footings are massless, so the ledger's proposal to add a hook force did not apply."],
    "used-basketball-hoop": ["The chain net is rigid and tilts with the ring (ledger); the lowering pose was already statically separate (hinge pin 463.2 N, sling 858.8 N)."],
    "used-beer-garden-set": ["Open plan strokes of the table top show mirrored in the plan of the turned-over table (request carried over).", "In the stacked and maintenance poses the set is solved as one rigid body (solver request)."],
    "used-bicycle-shelter": ["The post footings are not modelled (stated in the structure assumptions); bicycle, stands and frames are solved as one rigid body (solver request).", "The dimension of the height reads 2.60 to 2.63 m between the scales (coarser gutter and sheets)."],
    "used-bollard-light": ["Lifting pose already exact (75.5 N per grip)."],
    "used-bubbler-stone-fountain": ["The boulder is smooth; limescale, moss and algae are strokes only."],
    "used-chess-table": ["In `maintenance` the table hangs 60 mm above its bed with no sling drawn."],
    "used-climbing-net-pyramid": ["The net faces are rigid; at 1:500 the light-line drawing of the net is dense (ropes close to the print floor).", "Mass markers lie up to 0.1 m off the member centres (ledger, not re-measured)."],
    "used-concrete-bench-block": ["Lifting pose already exact (5061.2 N per socket)."],
    "used-concrete-table-tennis": ["`dimensions.heightM` 1.7125 m includes the footings below ground; the height above ground (0.9125 m) is in the texts only."],
    "used-dog-waste-station": ["The hood opens only 45 degrees; emptying is through the front under the hood."],
    "used-double-swing": ["The post foundations below the gravel are not modelled; the gravel sample (about 1.6 t) is not in `massKg`; the gravel mesh is large (ledger: 1.8 MB)."],
    "used-football-goal": ["The mesh net is opaque in the 3D sheets (ledger)."],
    "used-litter-bin-post": ["Ledger points not re-worked: the tipped liner is approximate in the front view, the liner slides on its axis, the dent is stepped."],
    "used-outdoor-fitness-station": ["At 1:500 the bench legs are slender (ledger)."],
    "used-park-armchair": ["Perforation is a hatch; the slender-tube request stays open. The ledger's point about oversized rust discs at 1:500 is stale: the document has no such shapes."],
    "used-park-lantern": ["While the column is lowered, the fading mat force stands for the rope; struts are approximate in the front view of the tilted pose and the rear door shows in the front view (ledger, not re-worked)."],
    "used-park-noticeboard": ["At 1:200 and 1:500 glass and pinboard are thicker in the thermal list (ledger); the reading clip barely moves."],
    "used-playground-roundabout": ["At 1:50 the rim is a 60 x 60 mm envelope (ledger)."],
    "used-pump-drinking-fountain": ["The head's centre of mass lies 32 mm beside the single lifting point; the jet is three slices that jump (ledger)."],
    "used-recycling-bin-cluster": ["The lid held open in `maintenance` is a load of the worker's hand (already the applied-force pattern)."],
    "used-sandpit-digger": ["The eye nut stands 0.12 m behind the centre of mass of the folded digger (stated in the assumptions); in service the post socket, sand and edging are solved as one rigid body on seven supports (solver request); no lever linkage (ledger)."],
    "used-seesaw": ["Lawn hollows are text only (ledger)."],
    "used-slide-tower": ["The sliding child is a moving load only (ledger)."],
    "used-spring-rocker": ["1:500 is unavailable (2.4 mm on paper); the 1 degree rest lean of the spring is ignored (ledger)."],
    "used-sun-lounger-bench": ["Lifting pose already exact (226.0 and 200.1 N per sling); the worn lawn is not drawn (ledger)."],
    "used-tree-surround-bench": ["No tree trunk is drawn; the 1:50 plan is dense (ledger)."],
    "used-water-play-table": ["No moving water stream; the screw flights are static in 2D; five permanent supports share by equal stiffness (ledger, a true indeterminate case)."],
    "used-wayfinding-signpost": ["The swap slides the finger only 85 mm; at 1:500 the fingers are 75 mm (ledger)."],
}
after = {l.split(" ")[0]: l.strip() for l in open(os.path.join(K, "reactions-measured.txt"))}
notes = {}
for slug in STARTED:
    changed = CHANGED.get(slug, [])
    shelter = slug == "used-bicycle-shelter"
    before = "catalog file present and equal to the candidate, but no manifest row and no membership of `used-park-objects` (ledger `running`: interrupted at publish); full proof VALID, check CLI ✓." if shelter else "published; full proof VALID (schema 0, validateAsset 0, checkVolumes 0), check CLI ✓, catalog copy equal to the candidate, manifest row equal and member of `used-park-objects`."
    done = "corrected (see below), full proof VALID, check CLI ✓, re-published through `publish.ts`, catalog copy VALID with `--catalog-only`." if changed else "unchanged; proven as above, catalog copy VALID with `--catalog-only`."
    sheets = "Opened by the finisher as a contact sheet of the final document: the three-quarter view of every authored scale and list (body, thermal, structure), of every non-default pose and of two instants of every clip at 1:50 (tiles of 320 px; the six-view sheets at full resolution were not opened one by one). " + ("The contact sheet of the product's 2D drawings (line and solid rows; elevation, front, plan) of every authored scale was opened too." if slug in OPENED_2D else "The contact sheet of the 2D drawings was rendered but NOT opened by the finisher; for the 2D the ledger of the interrupted wave is the only record of what its author saw.")
    notes[slug] = {
        "row": ROWS[slug], "before": before, "after": done, "changed": changed, "powerW": POWER.get(slug, 0),
        "reactions": [after[slug].replace(";", "\n").strip()] if slug in after else [],
        "verify": "ok (candidate = catalog copy, manifest row = recomputed row, member of the collection)",
        "sheets": sheets, "opened": slug in OPENED_2D,
        "imperfect": IMPERFECT.get(slug, []) + [GEAR, WEAR, REFERENCE],
    }
with open(os.path.join(K, "review-notes.json"), "w", encoding="utf8") as f:
    json.dump(notes, f, ensure_ascii=False, indent=1)
    f.write("\n")
print(f"[DEBUG] {len(notes)} notes, {sum(n['opened'] for n in notes.values())} with opened 2D sheets, {sum(bool(n['changed']) for n in notes.values())} changed")
