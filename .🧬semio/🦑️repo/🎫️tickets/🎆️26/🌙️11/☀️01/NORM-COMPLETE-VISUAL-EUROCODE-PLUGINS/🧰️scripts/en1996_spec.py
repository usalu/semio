"""EN 1996 wave 2 entity spec shared by the generators (`en1996_gen_*.py` import this by exec). Input script, kept in the ticket."""
import copy, json, os, re

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996"
SUB = f"{ROOT}/🏅️standards/🔖️1/🪆️subsets/✳️any"
SCH = f"{SUB}/🧬️schema"
MUT = f"{SCH}/🧬️mutations"
FIX = f"{SUB}/🧪️tests/🧫️fixtures/🧬️mutations"
SNAP = f"{SCH}/📸️snapshot"
DIFF = f"{SCH}/🔺️diff"
IO = f"{SUB}/🚪️io"
OWNER_REL = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
SCHEMA_ID = "https://json.schemas.assets.semio-tech.com/s/norm/en1996"
SNAPSHOT_REF = f"{SCHEMA_ID}/snapshot.json"


def rd(p):
    return open(p, encoding="utf-8").read()


def wr(p, s):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf-8").write(s)


def jload(p):
    return json.loads(rd(p))


def jdump(p, d):
    wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def pascal(s):
    return "".join(p[:1].upper() + p[1:] for p in re.split(r"[-_]", s))


def snake(s):
    return s.replace("-", "_")


def camel(s):
    p = pascal(s)
    return p[:1].lower() + p[1:]


def en1996_name(s):
    return "En1996" + s


ENUMS = {
    "TieKind": dict(variants=["WallTie", "Anchor", "RingTie"], en=["Cavity wall tie", "Wall-floor anchor", "Ring tie"], de=["Drahtanker", "Wand-Decken-Anker", "Ringanker"], label_en="Tie type", label_de="Ankerart", doc="🔗️ Wall tie, wall-floor anchor or ring tie (EN 1996-1-1 §8.5)."),
    "BarKind": dict(variants=["Anchorage", "Shear", "DeepBeam"], en=["Anchorage bar", "Shear reinforcement", "Deep-beam bar"], de=["Verankerungsstab", "Querkraftbewehrung", "Wandträgerstab"], label_en="Bar role", label_de="Stabfunktion", doc="🪢️ Role of a reinforcing bar of the wall (EN 1996-1-1 §6.6, §6.7, §8.2)."),
    "FrostClass": dict(variants=["Exempt", "Moderate", "Severe"], en=["No frost exposure", "Moderate frost exposure", "Severe frost exposure"], de=["Keine Frostbeanspruchung", "Mäßige Frostbeanspruchung", "Starke Frostbeanspruchung"], label_en="Frost class", label_de="Frostklasse", doc="❄️ Frost exposure class of the masonry (EN 1996-2 Annex A)."),
}


def F(name, en, de, unit="m", step=0.05, prec=2, mn=0, mx=None, kind="f64", disp=None, factor=None, sample=1.0, default=0.0):
    return dict(name=name, camel=camel(name), en=en, de=de, unit=unit, step=step, prec=prec, min=mn, max=mx, kind=kind, disp=disp, factor=factor, sample=sample, default=default)


KN = dict(disp="kN", factor=0.001)
MM2 = dict(disp="mm²", factor=1e6)

ROWS = [
    dict(key="pier", struct="WallPier", coll="piers", pre="Pier", emoji="🏛️", crate="piers-lintels", noun_en="Pier", noun_de="Pfeiler",
         doc="🏛️ Pier between openings with its position along the wall — SI base units (EN 1996-1-1 §5.5.1, §6.1).",
         fields=[F("position_m", "Pier position x", "Pfeilerlage x", sample=3.0), F("width_m", "Pier width", "Pfeilerbreite", sample=0.9)],
         slug={"position_m": "position", "width_m": "width"}, new_id="pier-2"),
    dict(key="lintel", struct="WallLintel", coll="lintels", pre="Lintel", emoji="🛤️", crate="piers-lintels", noun_en="Lintel", noun_de="Sturz",
         doc="🛤️ Lintel above an opening with position, span, depth and bearing — SI base units (EN 1996-1-1 §5.5.1.4, §6.1.3).",
         fields=[F("position_m", "Lintel position x", "Sturzlage x", sample=0.6), F("span_m", "Lintel clear span", "Sturz-Lichtweite", sample=1.4), F("depth_m", "Lintel depth", "Sturzhöhe", sample=0.25), F("bearing_length_m", "Lintel bearing length", "Sturz-Auflagerlänge", sample=0.25)],
         slug={"position_m": "position", "span_m": "span", "depth_m": "depth", "bearing_length_m": "bearing-length"}, new_id="lintel-2"),
    dict(key="tie", struct="WallTie", coll="ties", pre="Tie", emoji="🔗️", crate="ties", noun_en="Tie or anchor", noun_de="Anker",
         doc="🔗️ Wall tie, wall-floor anchor or ring tie with position and design resistance — SI base units (EN 1996-1-1 §8.5).",
         fields=[F("kind", "Tie type", "Ankerart", kind="TieKind", sample="Anchor"), F("position_m", "Tie position x", "Ankerlage x", sample=1.0), F("level_m", "Tie level y", "Ankerhöhe y", sample=2.5), F("capacity_n", "Tie design resistance", "Bemessungswiderstand Anker", unit="N", step=100, prec=0, sample=12000.0, **KN)],
         slug={"kind": "kind", "position_m": "position", "level_m": "level", "capacity_n": "capacity"}, new_id="tie-2"),
    dict(key="prestress", struct="WallPrestress", coll="prestress", pre="Prestress", emoji="🎻", crate="prestress", noun_en="Tendon", noun_de="Spannglied",
         doc="🎻 Post-tensioned tendon of a prestressed masonry wall — SI base units (EN 1996-1-1 §3.4, §6.8).",
         fields=[F("force_n", "Initial prestressing force P0", "Anfängliche Vorspannkraft P0", unit="N", step=1000, prec=0, sample=90000.0, **KN),
                 F("area_m2", "Tendon area A_p", "Spannstahlfläche A_p", unit="m2", step=1e-5, prec=6, sample=2.0e-4, **MM2),
                 F("f_pk_pa", "Characteristic tendon strength f_pk", "Charakteristische Spannstahlfestigkeit f_pk", unit="Pa", step=5e7, prec=0, sample=1.77e9, disp="N/mm²", factor=1e-6),
                 F("position_m", "Tendon position x", "Spanngliedlage x", sample=2.5),
                 F("eccentricity_m", "Tendon eccentricity", "Spanngliedausmitte", mn=None, sample=0.05),
                 F("loss_ratio", "Prestress loss ratio", "Spannkraftverlust", unit="1", step=0.01, prec=2, mx=1, sample=0.2)],
         slug={"force_n": "force", "area_m2": "area", "f_pk_pa": "strength", "position_m": "position", "eccentricity_m": "eccentricity", "loss_ratio": "loss"}, new_id="tendon-2"),
    dict(key="confined", struct="ConfinedElement", coll="confined", pre="Confined", emoji="🧰️", crate="confined", noun_en="Confining element", noun_de="Einfassungselement",
         doc="🧰️ Tie column of confined masonry with position, section and reinforcement — SI base units (EN 1996-1-1 §8.7).",
         fields=[F("position_m", "Column position x", "Stützenlage x", sample=2.5), F("width_m", "Column width", "Stützenbreite", sample=0.25), F("depth_m", "Column depth", "Stützentiefe", sample=0.24), F("as_m2", "Column reinforcement A_s", "Stützenbewehrung A_s", unit="m2", step=1e-5, prec=6, sample=6.0e-4, **MM2)],
         slug={"position_m": "position", "width_m": "width", "depth_m": "depth", "as_m2": "steel"}, new_id="column-2"),
    dict(key="flange", struct="ShearFlange", coll="flanges", pre="Flange", emoji="🔲", crate="flanges", noun_en="Flange wall", noun_de="Flanschwand",
         doc="🔲 Intersecting flange wall of a flanged shear wall — SI base units (EN 1996-1-1 §5.5.3, §6.2).",
         fields=[F("length_m", "Flange length", "Flanschlänge", sample=1.2), F("thickness_m", "Flange thickness", "Flanschdicke", sample=0.24), F("position_m", "Flange position x", "Flanschlage x", sample=0.0)],
         slug={"length_m": "length", "thickness_m": "thickness", "position_m": "position"}, new_id="flange-2"),
    dict(key="bar", struct="WallBar", coll="bars", pre="Bar", emoji="🪢️", crate="bars", noun_en="Reinforcing bar", noun_de="Bewehrungsstab",
         doc="🪢️ Anchorage, shear or deep-beam bar of reinforced masonry — SI base units (EN 1996-1-1 §6.6, §6.7, §8.2).",
         fields=[F("kind", "Bar role", "Stabfunktion", kind="BarKind", sample="Shear"), F("diameter_m", "Bar diameter", "Stabdurchmesser", step=0.002, prec=3, sample=0.012), F("length_m", "Provided bar length", "Vorhandene Stablänge", sample=0.6), F("spacing_m", "Bar spacing", "Stababstand", sample=0.4)],
         slug={"kind": "kind", "diameter_m": "diameter", "length_m": "length", "spacing_m": "spacing"}, new_id="bar-2"),
]

SCALARS = [
    dict(slug="frost-class", crate="site", emoji="❄️", **F("frost_class", "Frost class", "Frostklasse", kind="FrostClass", sample="Severe", default="Exempt"), noun_en="frost class", noun_de="Frostklasse"),
    dict(slug="cover", crate="site", emoji="🛡️", **F("cover_m", "Cover to reinforcement", "Betondeckung bzw. Überdeckung der Bewehrung", step=0.005, prec=3, sample=0.035, default=0.03), noun_en="cover", noun_de="Überdeckung"),
    dict(slug="deflection-limit", crate="site", emoji="📉", **F("deflection_limit", "Deflection limit span/x", "Verformungsgrenze Stützweite/x", unit="1", step=10, prec=0, sample=300.0, default=250.0), noun_en="deflection limit", noun_de="Verformungsgrenze"),
    dict(slug="soil-height", crate="site", emoji="🏞️", **F("soil_height_m", "Retained soil height", "Höhe der Anschüttung", step=0.05, prec=2, sample=1.8), noun_en="soil height", noun_de="Anschütthöhe"),
    dict(slug="soil-density", crate="site", emoji="🪨", **F("soil_density_kg_m3", "Soil density", "Wichte des Bodens", unit="kg/m3", step=50, prec=0, sample=1900.0, default=1800.0), noun_en="soil density", noun_de="Bodenwichte"),
    dict(slug="isotherm-depth", crate="site", emoji="🔥", **F("isotherm_depth_m", "Isotherm depth a_θ", "Isothermentiefe a_θ", step=0.005, prec=3, sample=0.04), noun_en="isotherm depth", noun_de="Isothermentiefe"),
    dict(slug="plan-x", crate="plan", emoji="🧭", **F("plan_x_m", "Plan position x", "Grundrissposition x", mn=None, step=0.1, prec=2, sample=2.0), noun_en="plan x", noun_de="Grundriss x"),
    dict(slug="plan-y", crate="plan", emoji="🧭", **F("plan_y_m", "Plan position y", "Grundrissposition y", mn=None, step=0.1, prec=2, sample=3.0), noun_en="plan y", noun_de="Grundriss y"),
    dict(slug="plan-angle", crate="plan", emoji="📐️", **F("plan_angle_deg", "Plan angle", "Grundrisswinkel", unit="deg", step=5, prec=1, mn=-180, mx=180, sample=90.0), noun_en="plan angle", noun_de="Grundrisswinkel"),
    dict(slug="in-plane-shear", crate="plan", emoji="↔️", **F("in_plane_shear_n", "In-plane shear V_Ed", "Scheibenschub V_Ed", unit="N", mn=None, step=1000, prec=0, sample=15000.0, **KN), noun_en="in-plane shear", noun_de="Scheibenschub"),
    dict(slug="in-plane-moment", crate="plan", emoji="🔄", **F("in_plane_moment_nm", "In-plane moment M_Ed", "Scheibenmoment M_Ed", unit="Nm", mn=None, step=1000, prec=0, sample=20000.0, disp="kNm", factor=0.001), noun_en="in-plane moment", noun_de="Scheibenmoment"),
]

OPENING_POS = dict(slug="opening-position", crate="positions", emoji="🪟", level="opening", **F("position_m", "Opening position x", "Öffnungslage x", sample=2.0), noun_en="opening position", noun_de="Öffnungslage")
CONC_A1 = dict(slug="concentrated-distance-a1", crate="positions", emoji="📏", level="conc", **F("distance_a1_m", "Distance a1 to wall end", "Abstand a1 vom Wandende", step=0.05, prec=2, sample=0.5), noun_en="distance a1", noun_de="Abstand a1")
CONC_POS = dict(slug="concentrated-position", crate="positions", emoji="📍️", level="conc", **F("position_m", "Load position x", "Lastlage x", sample=1.5), noun_en="load position", noun_de="Lastlage")

CRATES = {
    "piers-lintels": "piers and lintels", "ties": "wall ties, anchors and ring ties", "prestress": "prestressing tendons", "confined": "confined masonry elements",
    "flanges": "flange walls", "bars": "reinforcing bars", "site": "frost, cover, deflection, soil and fire isotherm", "plan": "bracing layout and in-plane actions", "positions": "opening and load positions and the distance a1",
}
CRATE_EMOJI = {"piers-lintels": "🏛️", "ties": "🔗️", "prestress": "🎻", "confined": "🧰️", "flanges": "🔲", "bars": "🪢️", "site": "🏞️", "plan": "🧭", "positions": "📍️"}


def crate_lib(c):
    return "semio_s_artifact_norm_en1996_mut_" + c.replace("-", "_")


def crate_pkg(c):
    return "semio-s-artifact-norm-en1996-mut-" + c


def crate_dir(c):
    return f"{CRATE_EMOJI[c]}{c}"


def leaves():
    out = []
    for r in ROWS:
        for f in r["fields"]:
            kind = f"change-{r['key']}-{r['slug'][f['name']]}"
            out.append(dict(kind=kind, op="field", row=r, field=f, crate=r["crate"], emoji=r["emoji"]))
        out.append(dict(kind=f"insert-{r['key']}", op="insert", row=r, crate=r["crate"], emoji="➕️"))
        out.append(dict(kind=f"remove-{r['key']}", op="remove", row=r, crate=r["crate"], emoji="➖️"))
    for s in SCALARS:
        out.append(dict(kind=f"change-{s['slug']}", op="scalar", field=s, crate=s["crate"], emoji=s["emoji"]))
    for s in (OPENING_POS, CONC_POS, CONC_A1):
        out.append(dict(kind=f"change-{s['slug']}", op=s["level"], field=s, crate=s["crate"], emoji=s["emoji"]))
    return out


def dirname(leaf):
    return f"{leaf['emoji']}{leaf['kind']}"


def extra_wall_set():
    return [s for s in SCALARS]
