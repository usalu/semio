"""📝️ Fills the asset section of `silhouette-SO.md` from the measured lines before and after (`🗑️generated/silhouette/SO/before.txt`, `after.txt`) and the notes below: report.py"""
import os, re
HERE = os.path.dirname(os.path.abspath(__file__))
U = os.path.dirname(os.path.dirname(HERE))
GEN = f"{U}/🗑️generated/silhouette/SO"
NOTES = {
    "bicycle-stand": "1:500: the tube of the print floor is an oval of 75.2 mm in the plane of the hoop and 63 mm across (it was round, 75.2 mm, against bars of 63 mm in front and plan, which the lateral extent rule caps); the front also draws the far leg and the cross rail behind the near leg, so `volumes-feature` reads 63 mm for every bone; decisions texts of 1:500 say so.",
    "chalkboard": "1:50: the ruler is a 7.5 mm board as the elevation draws it and no `slender` strip (its stroke stood up to 22 mm outside the tray, requests.md). 1:100: the end view draws the leaf where its body is (3.5 mm towards the room, lower edge at 68 mm).",
    "charcoal-grill": "1:50: the cap of the front foot is 27 mm across on the 25 mm leg instead of 33 mm, in the volume and the three views; the body is 0.6064 m wide. No declared dimension changed.",
    "drinking-fountain": "1:100, 1:200, 1:500: the water jet is a `texture` solid, so its stroke in elevation and front is drawn in the `texture` role (at 1:200 the stroke closed the arch of the bubbler into a filled area).",
    "market-parasol": "1:50: the front draws the pin of the tilt knuckle. 1:100, 1:200: the rib rods run to the end of the rib (tip cap merged) in the volume and both elevations, body 2.591 m across.",
    "patio-heater": "1:50 side elevation: the wheel bracket of the volume is drawn between wheel and housing; the two visible reflector studs are drawn on the side where the volume has them.",
    "playground-swing": "1:50: the through bolts of the apex sleeves lie horizontal like the apex bolts (body 2.5072 m high instead of 2.5135 m), the hanger bands have the wall of the sleeves; the front draws the hook shank at 13 mm, starts the chain band at the lower eye and draws the two cheeks of each hanger fork apart.",
    "shopping-trolley": "Not repaired. Measured mismatches at 1:50: castor forks and axle bolts, tray lip and strut in the front, rim corner in the plan (several places of 20 to 50 mm each). The greedy repair `auto.ts` needed more than 15 minutes per step on this wire asset and found no accepted step.",
    "stepladder": "Not repaired (not reached within this session). Measured places: 1:50 front, hinge and rail caps (22 mm); 1:50 and 1:100 plan, rear rails and spreaders beside the platform (30 to 37 mm); 1:500 front, a tread (214 mm).",
    "street-lamp": "1:50: the six cooling fins of the head are a `texture` solid (4 mm plates), so their strokes in the three views are drawn in the `texture` role.",
    "tealight": "1:50: the wick is drawn as the filled 2 mm strand of the volume instead of a stroke of 6.5 mm; the wax line lies 1.75 mm lower so that its stroke ends at the cup rim.",
    "used-baby-grand-piano": "Not repaired (not reached within this session).",
    "used-basketball-hoop": "Not repaired (not reached within this session). Measured places: 1:50 front at the anchor nuts of the base plate (17 mm), 1:50 plan at ring mount and ring rod (71 mm), 1:100 side elevation at the ring bracket (47 mm).",
    "used-bathtub": "In the generator (`author-used-bathtub.py`), 1:50: eight wall bands instead of four in front and plan, and the lowest band, the lowest inside band and the floor step in at both ends like the rounded corners of the bottom sheet (`coved`); the plan passes (11.4 mm). The front stays red at the open waste hole, see below and requests.md.",
    "used-bicycle-shelter": "1:500: the front draws the three purlins as 1:200 does (the open strip between gutter and panels). 1:50 side elevation: tyre outline 0.5 mm smaller (an opening at the 256-cell limit, requests.md); the paper depot tag is drawn in the `texture` role.",
    "used-climbing-net-pyramid": "Not repaired (not reached within this session). Measured now: IoU 0.899 to 0.911 in all views of 1:50, 1:100 and 1:200 and 361 mm in the plan of 1:500 (the suite of 16:27 named only the latter).",
    "used-dog-waste-station": "1:50: the mounting plate between post and bin runs to the top of the bin (0.41 m) in the volume, the side elevation and the front.",
    "used-double-bass": "1:50, 1:100, 1:200 plan: the neck sleeve of the cover is drawn as lateral slices that follow the width of its loft instead of one block; 1:50 plan: the heel of the neck ends its full width where the cover band ends.",
    "used-drum-kit": "Not repaired (not reached within this session).",
    "used-football-goal": "1:50 side elevation: the sagging back mesh is drawn by the outline of its volume (26 points from the rows of the mesh polyhedron).",
    "used-marching-snare-and-bass-drum": "1:500 side elevation: a stroke of 0.13 mm along the centre line of each of the four J bars beside its filled band; IoU 0.964. 1:200 side elevation stays red by 0.5 mm, see below.",
    "used-recycling-bin-cluster": "1:50: the front draws the six trunnion bosses and pins between bins and posts.",
    "used-seesaw": "In the generator (`author-seesaw.py`): the hollow of the old seat was drawn 0.28 m towards the pivot instead of along the seat (a sign), which made an outline of 20 mm beside the beam in the plan of 1:50.",
    "used-slide-tower": "1:200: the two lifting eyes on the ridge are kept as lugs of 30 x 62 x 30 mm in the volume, its thermal list and the three views; body 4.417 m high.",
    "used-upright-piano": "1:50: the front draws the dark backs of the three pedal slots of the knee board.",
    "used-viola": "1:50 plan: the case handle is drawn as grip and two tabs with the finger slot open; the last 18 mm of the slit under the raised lid are drawn closed.",
    "used-water-play-table": "1:50: the plan draws the strap of the chute. 1:100: the two sluice gates are 15 mm plates instead of 3 mm sheets, the thickness their strokes draw.",
}


def failing(path):
    """📏️ Per asset the failing checks of a measured file."""
    out = {}
    if not os.path.exists(path):
        return out
    for line in open(path):
        m = re.match(r"^(\S+) (1:\d+) f=", line)
        if m:
            for t in re.findall(r"bbox \S+ \S+ \S+|(?:elevation|front|plan)(?:\(s\))? \S+", line):
                if "!" in t:
                    out.setdefault(m.group(1), []).append(f"{m.group(2)} {t}")
        m = re.match(r"^(\S+): (OK|\d+ failing checks)", line)
        if m:
            out.setdefault(m.group(1), [])
    return out


before, after = failing(f"{GEN}/before.txt"), failing(f"{GEN}/after.txt")
order = [l.split("\t")[1] for l in open(f"{U}/🧰️silhouette/SO.tsv")]
rows = []
for a in order:
    b, f = before.get(a, []), after.get(a)
    state = "not measured" if f is None else ("green" if not f else "RED: " + "; ".join(f))
    rows.append(f"### `{a}`\n\n- Before: {'; '.join(b) or 'none measured'}.\n- After: {state}.\n- Changed: {NOTES[a]}\n")
path = f"{U}/silhouette-SO.md"
text = open(path).read()
start, end = text.index("## Assets"), text.index("## Files")
open(path, "w").write(text[:start] + "## Assets\n\n" + "\n".join(rows) + "\n" + text[end:])
print(len(rows), "assets")
