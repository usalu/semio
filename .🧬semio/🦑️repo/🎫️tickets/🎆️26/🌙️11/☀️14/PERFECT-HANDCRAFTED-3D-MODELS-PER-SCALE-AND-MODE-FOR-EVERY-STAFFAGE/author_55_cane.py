"""Author 55: person-with-white-cane. Run: python3 author_55_cane.py [build]  (build = figure-build first)."""
import sys; sys.path.insert(0, ".")
from author_55_lib import *
SLUG = "person-with-white-cane"; SKIN = "skin-medium"
AL, GRIP, NYL = "cane-alu-white", "grip-eva", "nylon-roller"
def cane(n):
    b, t, st = [], [], []
    if n == 50:
        b += [cap("cane-shaft", "cane", AL, [0.09, 0, 0], [1.226, 0, 0], 0.0085, 0.0058, tone=0.95)]
        b += [cap("cane-band", "cane", "cane-alu-white", [0.97, 0, 0], [1.20, 0, 0], 0.0067, 0.0063, role="detail", tone=0.35)]
        b += [lofts("cane-grip", "cane", GRIP, [(-0.115, 0.004, 0.004), (-0.110, 0.0125, 0.0125), (-0.100, 0.0165, 0.0165), (-0.050, 0.0175, 0.0175), (0.030, 0.0165, 0.0165), (0.090, 0.0140, 0.0140), (0.110, 0.0105, 0.0105), (0.116, 0.0090, 0.0090)], tone=0.12)]
        loop = [(-0.105, 0), (-0.125, 0.026), (-0.170, 0.030), (-0.200, 0.0), (-0.170, -0.030), (-0.125, -0.026), (-0.105, 0)]
        for i, (p0, p1) in enumerate(zip(loop, loop[1:]), 1):
            b.append(cap(f"cane-strap-{i}", "cane", GRIP, [p0[0], p0[1], 0], [p1[0], p1[1], 0], 0.0042, role="detail", tone=0.3))
        b += [cap("cane-ferrule", "cane", NYL, [1.215, 0, 0], [1.268, 0, 0], 0.0072, 0.0085, tone=0.6)]
        b += [ell("cane-roller", "cane", NYL, [1.2845, 0, 0], [0.0155, 0.0155, 0.0155], tone=0.8)]
        reg = {"cane-shaft": ["cane-shaft", "cane-band", "cane-ferrule"], "cane-grip": ["cane-grip"] + [f"cane-strap-{i}" for i in range(1, 7)], "cane-tip": ["cane-roller"]}
    if n == 100:
        b += [cap("cane-shaft", "cane", AL, [0.09, 0, 0], [1.23, 0, 0], 0.0090, 0.0065, tone=0.95, slender=True)]
        b += [lofts("cane-grip", "cane", GRIP, [(-0.1, 0.008, 0.008), (-0.092, 0.0140, 0.0140), (0.03, 0.0150, 0.0150), (0.09, 0.0150, 0.0150), (0.1, 0.0100, 0.0100)], tone=0.12)]
        b += [ell("cane-tip", "cane", NYL, [1.2855, 0, 0], [0.0145, 0.0145, 0.0145], tone=0.8)]
        b += [cap("cane-strap-1", "cane", GRIP, [-0.1, 0, 0], [-0.165, 0.028, 0], 0.00825, role="detail", tone=0.3), cap("cane-strap-2", "cane", GRIP, [-0.165, 0.028, 0], [-0.2, -0.005, 0], 0.00825, role="detail", tone=0.3)]
        reg = {"cane-shaft": ["cane-shaft"], "cane-grip": ["cane-grip", "cane-strap-1", "cane-strap-2"], "cane-tip": ["cane-tip"]}
    if n == 200:
        b += [cap("cane-shaft", "cane", AL, [0.08, 0, 0], [1.25, 0, 0], 0.0085, 0.0060, tone=0.95, slender=True)]
        b += [cap("cane-grip", "cane", GRIP, [-0.075, 0, 0], [0.075, 0, 0], 0.0165, tone=0.12)]
        b += [ell("cane-tip", "cane", NYL, [1.282, 0, 0], [0.0165, 0.0165, 0.0165], tone=0.8)]
        reg = {"cane-shaft": ["cane-shaft"], "cane-grip": ["cane-grip"], "cane-tip": ["cane-tip"]}
    if n == 500:
        b += [cap("cane-shaft", "cane", AL, [0.09, 0, 0], [1.26, 0, 0], 0.0080, 0.0060, tone=0.95, slender=True)]
        b += [cap("cane-grip", "cane", GRIP, [-0.02, 0, 0], [0.02, 0, 0], 0.0265, tone=0.12)]
        b += [ell("cane-tip", "cane", NYL, [1.272, 0, 0], [0.0265, 0.0265, 0.0265], tone=0.8)]
        reg = {"cane-shaft": ["cane-shaft"], "cane-grip": ["cane-grip"], "cane-tip": ["cane-tip"]}
    for r, ids in reg.items():
        t += [th(i, r) for i in ids]
    rc = {50: 0.0052, 100: 0.0055, 200: 0.0060, 500: 0.0060}[n]
    u0 = {50: -0.11, 100: -0.095, 200: -0.07, 500: 0.0}[n]
    st += [{**cap("member-cane", "cane", AL, [u0, 0, 0], [1.226 if n == 50 else 1.23 if n < 500 else 1.26, 0, 0], rc, rc * 0.78, tone=0.9, slender=True), "member": "cane", "part": "member"}]
    rs = {50: 0.0095, 100: 0.0145, 200: 0.0165, 500: 0.0265}[n]
    st += [{**ell("support-cane-tip", "cane", NYL, [1.3 - rs, 0, 0] if n >= 100 else [1.3 - 0.0095, 0, 0], [rs, rs, rs], role="detail", tone=0.2), "member": "cane", "part": "support", "anchor": "cane-tip"}]
    return b, t, st
D = {50: ("White long cane as one object: tapered aluminium tube 9.5 to 7 mm with a red-white sleeve band, EVA grip with flare and wrist-strap loop, nylon ferrule and rolling ball tip; thermal regions on the same solids, structure member is the tube core and the support the ball contact.", "Weißer Langstock als ein Gegenstand: konisches Aluminiumrohr 9,5 bis 7 mm mit Bandmanschette, EVA-Griff mit Kehle und Handschlaufe, Nylon-Zwinge und Rollkugelspitze; Thermik auf denselben Solids, Tragelement ist der Rohrkern, Auflager der Kugelkontakt."),
     100: ("Cane at 1:100: slender tapered tube at true size, grip and rolling tip as one loft each; strap and band merged away.", "Stock 1:100: schlankes konisches Rohr in wahrer Größe, Griff und Rollspitze je ein Loft; Schlaufe und Band entfallen."),
     200: ("Cane at 1:200: slender tube, grip capsule and tip ball as the drawing strokes them.", "Stock 1:200: schlankes Rohr, Griffkapsel und Spitzenkugel wie in der Zeichnung gestrichelt."),
     500: ("Cane at 1:500: slender tube at true size; grip and tip are compact blocks at the drawn pen width.", "Stock 1:500: schlankes Rohr in wahrer Größe; Griff und Spitze als kompakte Körper in der gezeichneten Stiftbreite.")}
def post(d):
    v = d["volumes"]["50"]
    for x in v["body"] + v["thermal"]:
        if "sections" in x and (x["id"].startswith("forearm-r-cuff-s-r") or x["id"].startswith("th-forearm-r-cuff-s-r")):
            x["sections"][0]["at"][0] = 0.1955; x["sections"][1]["at"][0] = 0.2055
    for sc, sid, dw in (("200", "member-toe-l", 0.02), ("500", "member-arm-l", -0.06)):
        for x in d["volumes"][sc]["structure"]:
            if x["id"] == sid:
                for q in x["sections"]: q["at"][2] += dw
    v = d["volumes"]["500"]
    for i, x in enumerate(v["thermal"]):
        if x["id"] == "th-hair":
            v["thermal"][i] = {**ell("th-hair", "head", "hair-brown", [0.170, 0.012, 0], [0.0400, 0.0400, 0.0400], tone=0.4), "region": "hair"}
if __name__ == "__main__":
    if "build" in sys.argv: print(run_build(SLUG))
    d = load(SLUG); restore_bones(SLUG, d, {"neck"})
    for n in (50, 100, 200, 500):
        v = d["volumes"][str(n)]; skinfix(v, SKIN)
        b, t, st = cane(n)
        replace_props(v, {"cane"}, {"cane-shaft", "cane-grip", "cane-tip"}, b, t, st)
        add_decisions(v, *D[n])
    post(d)
    save(SLUG, d)
