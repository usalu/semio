"""Grip coverage audit of the six design-case catalogues (FORMS-V3): declared roles of every geometry row against the grips the shared forms offer, before and after v3, plus the geometric inputs (lengths, angles, times) a catalogue entry declares that no role draws."""
import json, collections
exec(open("audit.py").read().split('if __name__')[0])

ALL = ("width", "length", "height")
GRIPS = {
    "Member": {"width", "length", "height", "eff_height", "notch_length", "taper", "apex_angle", "apex_radius", "plies"},
    "Plate": {"width", "length", "height", "plies"},
    "Column": set(ALL), "Block": set(ALL), "Wall": set(ALL), "Connection": set(ALL),
    "Cylinder": {"width", "height"}, "Gauge": set(), "Load": set(ALL),
    "Fastener": {"width", "length", "height", "thickness2", "diameter", "spacing", "spacing2"},
    "Weld": {"width", "length", "height", "throat"},
    "Anchor": {"width", "length", "height", "embed", "diameter"},
    "Opening": {"width", "length", "height", "open_w", "open_l"},
    "Symbol": set(),
    "Footing": {"width", "length", "height", "embed", "water"},
    "Pile": {"width", "height", "spacing", "water"},
    "RetainingWall": {"width", "length", "height", "base", "base_t", "toe", "water"},
    "SheetWall": {"width", "length", "height", "embed", "prop", "water"},
    "Slope": {"width", "length", "height", "soft", "level"},
    "SoilBlock": {"width", "length", "height", "water", "rod"},
    "GroundAnchor": {"width", "length", "free"},
    "Seepage": {"width", "length", "height", "embed", "layer"},
    "Well": {"width", "height", "radius", "level"},
}
NEW = {"Fastener": {"count", "rows"}, "Pile": {"rows", "columns", "layers"}, "Footing": {"layers"}, "SoilBlock": {"layers"}, "Slope": {"angle", "ratio"}, "GroundAnchor": {"angle"}}
APPEARANCE = {"grain", "ply_dir"}
EXTRA_DRAWN = {"T-90": {"a1", "i1", "s1", "a2", "i2", "s2", "a3", "i3", "s3", "h1", "h2", "h3"}, "T-93": {"n_parts", "area", "l1", "lef"}, "T-94": {"s", "n_panels", "h", "b_i", "l_dia"}, "T-101": {"t_req", "face_left", "face_right", "face_top", "face_bottom"}, "T-102": {"t_req", "face_left", "face_right", "face_top", "face_bottom"}, "T-106": {"t_req", "face_left", "face_right", "face_top", "face_bottom"}, "T-107": {"t_req", "n_layers"}, "T-120": {"crowd_density"}, "T-121": {"n1", "n2", "n3", "n4", "n5", "n6", "years", "ds1"}, "T-122": {"bar_spacing", "plate_a", "deck_depth"}}
GEO_UNITS = {"Metre", "Millimetre", "Degree", "Radian", "Minute"}

def roles_of(row):
    pairs = [(r, row[r]) for r in ALL if row[r]] + list(row["aux"].items())
    if row["load"]:
        pairs.append(("load", row["load"]))
    return pairs

summary = []
for code, (name, rel) in CASES.items():
    directory = f"{ROOT}/{code}/{rel}/📐️geometry"
    rows = json.load(open(f"{directory}/🔣️.json"))["entries"]
    entries, vocab = extract_entries(code), extract_vocabulary(code)
    declared = before = after = 0
    gaps_before, gaps_after, undrawn = [], [], collections.defaultdict(list)
    for eid, row in rows.items():
        form = row["form"]
        keys_drawn = set()
        for role, key in roles_of(row):
            if role in APPEARANCE or not key:
                continue
            keys_drawn.add(key)
            declared += 1
            base = role == "load" or role in GRIPS.get(form, set())
            new = base or role in NEW.get(form, set())
            before += base
            after += new
            if not base:
                gaps_before.append(f"{eid}:{role}={key}")
            if not new and not (eid in EXTRA_DRAWN and key in EXTRA_DRAWN[eid]):
                gaps_after.append(f"{eid}:{role}={key}")
        extra = EXTRA_DRAWN.get(eid, set())
        for k in entries.get(eid, ("", []))[1]:
            s = vocab_spec(vocab, eid, k)
            if s and s["kind"] == "number" and s["unit"] in GEO_UNITS and k not in keys_drawn and k not in extra:
                undrawn[eid].append(f"{k} [{s['unit']}]")
    summary.append((code, len(rows), declared, before, after, gaps_before, gaps_after, undrawn))

for code, n, declared, before, after, gb, ga, und in summary:
    print(f"## {code}: {n} entries, {declared} declared grip roles; grips before v3 {before}, after v3 {after}")
    print("  gaps after (declared role without a grip):", ga if ga else "none")
    fixed = [g for g in gb if g not in ga]
    print("  closed by v3 (generic forms):", fixed if fixed else "none")
    print(f"  entries with geometric-unit inputs that no role draws: {len(und)}; inputs: {sum(len(v) for v in und.values())}")
    for eid, keys in list(und.items())[:400]:
        print("   ", eid, ", ".join(keys))
