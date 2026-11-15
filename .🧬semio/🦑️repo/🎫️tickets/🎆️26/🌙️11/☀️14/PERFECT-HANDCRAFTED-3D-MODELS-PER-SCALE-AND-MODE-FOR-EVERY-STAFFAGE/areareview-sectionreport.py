"""Builds the C25 sectionFaces table (appended to area-decisions.md by areareview-report.py) from the catalog and the removals of areareview-unsection.py."""
import glob, importlib.util, json, os, re
T = os.path.dirname(os.path.abspath(__file__))
S = os.path.abspath(os.path.join(T, *[".."] * 7, "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"))
spec = importlib.util.spec_from_file_location("u", os.path.join(T, "areareview-unsection.py")); u = importlib.util.module_from_spec(spec); spec.loader.exec_module(u)
FLOW = re.compile(r"plume|jet|steam|outlet|exhaust-air|supply-air|extract|slot-air|flue-gas|cold-jet|case-plume|rear-exhaust|spray|emitter")
VERDICT = {
 "etics-brick-wall-12": ("legitimate", "cut faces of a wall sample that continues (a)"), "etics-brick-wall-30": ("legitimate", "cut faces of a wall sample that continues (a)"), "solid-brick-wall-1960": ("legitimate", "cut faces of a wall sample that continues (a)"),
 "leaning-person": ("legitimate", "wall face against the person's mounting wall sample (a)"), "tied-canopy": ("legitimate", "wall render, beam and strut cut where they continue into the building; glass faces inside the laminated unit (a)(b)"),
 "tilt-window": ("legitimate", "pane edges and faces inside the sealed insulating unit (b)"), "wheelchair-ramp": ("legitimate", "skin undersides bonded to the poured core, run ends continue, kerb on the ground (a)(b)"),
 "chalkboard": ("legitimate", "enamel back bonded to the board core, frame back on the wall (b)"), "stair-flight": ("legitimate", "tread top layer bonded to the core, edges continue (a)(b)"),
 "steel-column": ("legitimate", "flange and web faces welded together, column ends continue (a)(b)"), "street-lamp": ("legitimate", "pole and arm ends continue into the ground and the luminaire (a)"),
 "retractable-bollard": ("legitimate", "cover frame cast into the ground slab (b)"), "sectional-garage-door": ("legitimate", "panel and strip faces stacked against each other, seal on the floor (b)"),
 "sliding-door": ("legitimate", "glass face inside the frame rebate, the other face is the conditioned interior (b)"), "bar-stool": ("legitimate", "plate core glued to the seat pad (b)"),
 "laptop": ("legitimate", "deck and underside plates joined to the case along their edges (b)"), "mvhr-unit": ("legitimate", "back of casing and window on the wall (b)"), "shower-head": ("legitimate", "brackets and valve on the wall, tube ends continue (a)(b)"),
 "market-parasol": ("legitimate", "top and underside are two regions, each region owns one face (face counted once in the other)"), "gas-hob": ("legitimate", "plate underside and knob back on the appliance, burner ring seated, flame rings are flow volumes (b)"),
 "split-ac-indoor-unit": ("legitimate", "grille and vane inner faces against the casing (b)"), "server-rack": ("legitimate", "closed cabinet: inner faces of the panels and the unit fronts belong to the sealed airway (b); outer faces count"),
 "tower-server": ("legitimate", "closed case: inner faces belong to the sealed airway (b)"), "toaster": ("legitimate", "slot air is a flow volume"), "charcoal-grill": ("legitimate", "envelope caps of hot-bowl and lid shells, charcoal bed seated, flows (b)"),
 "vintage-car-1929": ("legitimate", "closed body shells: inner faces look into the cabin or engine bay, which are regions of their own (b); to be re-examined if the coordinator wants closed shells counted two-sided"),
 "kitchen-extractor-hood": ("legitimate", "canopy inner face is the side the released heat enters (operating region, one face is the exchange face); extract is a flow volume"), "solar-thermal-collector": ("legitimate", "glass underside inside the sealed collector, absorber underside bonded to risers and insulation (b); the declared one-sided area is what the solver wants (faces = 1, plate-up)"),
 "hot-water-cylinder": ("legitimate", "plume is a flow volume"), "cooking-pot": ("legitimate", "steam is a flow volume"), "patio-heater": ("legitimate", "flow volumes"), "drinking-fountain": ("legitimate", "jet is a flow volume"), "tealight": ("legitimate", "flame plume is a flow volume"),
 "plate-heat-exchanger": ("legitimate", "outlets are flow volumes"), "sorption-wheel": ("legitimate", "outlets are flow volumes"), "thermostatic-radiator-valve": ("legitimate", "knob mounting faces (b)"),
}
rows = json.load(open(os.path.join(T, "🗑️generated", "areareview", "sectionfaces.json")))
out = ["## `sectionFaces` against C25", "",
       "Every use found in the catalog (`volumes` of all scales) and judged; illegitimate uses were removed from the asset file and from `T/volumes/<category>/<asset>.json` by `areareview-unsection.py` (only the member `sectionFaces` of the solid disappears, checked by JSON comparison). Flow volumes (plumes, jets, outlets) are not surfaces at all; they are kept and listed as such, the coordinator may prefer a dedicated flag.", "",
       "| asset | scales | solids (1:50 ids) | faces | verdict | why |", "|---|---|---|---|---|---|"]
by = {}
for a, sc, lst, sid, faces, reg, of in rows:
    by.setdefault(a, []).append((sc, lst, sid, faces))
for a in sorted(by):
    items = by[a]
    flows = [i for i in items if FLOW.search(i[2])]
    rest = [i for i in items if not FLOW.search(i[2])]
    scales = ",".join(sorted({i[0] for i in items}, key=int))
    if rest:
        v, why = VERDICT.get(a, ("legitimate", "see ruling"))
        ids = ", ".join(sorted({i[2] for i in rest if i[0] == "50"}))[:300] or "(coarser scales only)"
        fc = " ".join(sorted({"/".join(i[3]) for i in rest if i[0] == "50"})[:3])
        out.append(f"| {a} | {scales} | {ids} | {fc} | {v} | {why} |")
    if flows:
        ids = ", ".join(sorted({i[2] for i in flows if i[0] == "50"}))[:300] or "(coarser scales only)"
        out.append(f"| {a} | {scales} | {ids} | all | flow volume, not a surface | emitter/plume/outlet solid, no material face |")
out += ["", "Removed as illegitimate (faces in free air), with the resulting `areaM2` decision in the table above:", "",
        "| asset | scales | solids | face | why illegitimate | area decision |", "|---|---|---|---|---|---|",
        "| city-car | 50,100,200,500 | th-bp-lower(-paint) | -v | vehicle underside in free air | SET 13.0 (underbody 4.8 m2 added) |",
        "| delivery-van | 50,100,200,500 | th-shell(-paint) | -v | vehicle underside in free air | SET 22 (committed; underbody 8.2 m2) |",
        "| city-bus | 50,100,200,500 | th-shell(-insulation) | -v | vehicle underside in free air | SET 62 (underbody 26 m2 added) |",
        "| height-adjustable-desk | 50,100,200,500 | th-top-board(-laminate) | -v | underside of a free-standing board | SET 2.68 (both faces; solver faces = 1 so both faces give the right Watts, geometry plate-up is a note) |",
        "| dining-table | 50,100,200,500 | th-table-top-top-l/r (slab-l/r) | -v | underside of a free-standing top | SET 3.0 (both faces) |",
        "| person-with-umbrella | 50,100,200,500 | canopy (body), th-canopy | -u | underside of an open canopy | SET 2.1 (both faces of the dome) |", ""]
open(os.path.join(T, "🗑️generated", "areareview", "sections.md"), "w").write("\n".join(out))
