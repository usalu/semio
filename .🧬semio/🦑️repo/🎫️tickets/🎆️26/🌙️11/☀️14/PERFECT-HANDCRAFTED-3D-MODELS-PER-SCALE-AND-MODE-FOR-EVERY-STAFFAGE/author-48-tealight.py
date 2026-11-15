from author_37_lib import *
import math, json, glob, re

CAT = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*objects/tealight/*.json")[0]
ASSET = json.load(open(CAT, encoding="utf-8"))["asset"]


def half_profile(scale, shape, dy=0.0):
    """Right half of a drawn teardrop path as a revolve profile (axial, radius)."""
    d = [s for s in ASSET["representations"][str(scale)]["geometry"]["elevation"] if s["id"] == shape][0]["d"]
    nums = [float(t) for t in re.findall(r"-?\d+\.?\d*(?:e-?\d+)?", d)]
    p0 = (nums[0], nums[1])
    pts = [p0]
    k = 2
    while k + 5 < len(nums):
        c1, c2, p3 = (nums[k], nums[k + 1]), (nums[k + 2], nums[k + 3]), (nums[k + 4], nums[k + 5])
        for i in range(1, 7):
            t = i / 6
            pts.append(tuple((1 - t) ** 3 * p0[j] + 3 * (1 - t) ** 2 * t * c1[j] + 3 * (1 - t) * t ** 2 * c2[j] + t ** 3 * p3[j] for j in range(2)))
        p0 = p3
        if p3[0] <= 1e-9:
            break
        k += 6
    return [[round(p[1], 6), max(p[0], 0.0)] for p in pts]


CUP = Ctx("cup", "aluminium-cup", "cup", "body", 0.5)
WAX = Ctx("wax", "paraffin", "wax", "body", 0.85)
WIK = Ctx("wick", "wick-cotton", "wick", "body", 0.1)
FLB = Ctx("flame-body", "flame", "flame", "body", 0.9)
FLT = Ctx("flame-tip", "flame", "flame", "body", 0.97)
ST = Ctx("cup", "aluminium-cup", None, "detail", 0.15)
TOP, WALL = 0.016, 0.0001


def rout(y):
    return 0.0155 + 0.21875 * y


def cup_profile50():
    ri = lambda y: 0.0154 + 0.2315 * (y - 0.0011)
    return [[0.0010, 0.0], [0.0010, 0.0140], [0.0, 0.0145], [0.0, 0.0155], [0.0140, rout(0.0140)], [0.0157, rout(0.0157)], [0.0160, 0.01915], [0.0160, 0.01885], [0.0140, 0.018393], [0.0011, 0.0154], [0.0011, 0.0]]


def wax_profile(y0, rin, ytop=0.0140, dome=0.0006):
    """Wax fill in wax-local y (bone origin 1.4 mm): floor y0 (global), against the inner wall plus 0.05 mm, domed free surface."""
    pts = [[y0 - 0.0014, 0.0], [y0 - 0.0014, rin(y0) + 0.00005]]
    ye = ytop
    pts.append([ye - 0.0014, rin(ye) + 0.00005])
    redge = rin(ye) + 0.00005
    for f in (0.9, 0.75, 0.5, 0.25, 0.0):
        r = f * redge
        pts.append([ytop + dome * (1 - f * f) - 0.0014, r])
    return pts


def disc(sid, ctx, y0, y1, r, tags, layer):
    c = ctx(layer=layer)
    s = ext(sid, c, ngon(r, 24), "zx", y0, y1)
    s["sectionFaces"] = ["+u", "-u", "+v", "-v", "+w", "-w"]
    s["tags"] = [tags]
    return s


def pad(r, h, y0=0.0):
    return tag(cyl("support-base-contact", ST(member="cup", part="support", tone=0.1), "v", (0, y0 + h / 2, 0), h, r), tags=["base-contact"])


def stm_all(ids):
    return [stm("member-" + i, of, m, "member") for i, of, m in ids]


# ---------------------------------------------------------------- 1:50
ri50 = lambda y: 0.0154 + 0.2315 * (y - 0.0011)
body50 = [
    rev("cup-shell", CUP, cup_profile50(), "v"),
    rev("wax", WAX, wax_profile(0.00105, ri50), "v"),
    rev("wick", WIK, [[-0.0134, 0.0], [-0.0134, 0.0040], [-0.0129, 0.0040], [-0.0129, 0.0010], [0.0001, 0.0010], [0.0053, 0.0010], [0.0053, 0.0]], "v"),
    rev("flame", FLB, half_profile(50, "flame"), "v"),
    rev("flame-tongue", FLT, half_profile(50, "tongue"), "v"),
]
th50 = [
    th("th-cup-shell", "cup-shell", "cup", "foil"),
    th("th-wax", "wax", "wax", "wax"),
    th("th-wick", "wick", "wick", "braid"),
    th("th-flame", "flame", "flame", "envelope"),
    th("th-flame-tongue", "flame-tongue", "flame", "envelope"),
    disc("th-flame-plume", FLT(region="flame"), 0.0135, 0.0210, 0.010, "flame-plume", "envelope"),
]

st50 = [
    stm("member-cup", "cup-shell", "cup", "member"),
    stm("member-wax", "wax", "wax", "member"),
    stm("member-wick", "wick", "wick", "member"),
    stm("member-flame", "flame", "flame", "member"),
    stm("member-flame-body", "flame", "flame-body", "member"),
    stm("member-flame-tip", "flame-tongue", "flame-tip", "member"),
    stm("joint-wick", "wick", "wick", "joint"),
    stm("joint-flame", "flame", "flame", "joint"),
    stm("joint-flame-body", "flame", "flame-body", "joint"),
    tag(ell("mass-wax", ST(member="wax", part="mass", bone="wax"), (0, 0.0065, 0), (0.011, 0.005, 0.011))),
    pad(0.0155, 0.0075),
]
D50 = [
    text("Own 1:50 model of the 38 mm tealight against the drawings (lit pose): the cup is a true 0.1 mm foil shell (one revolve) with the conical wall 31 to 38 mm, a rolled rim lip, a 0.1 mm floor and the pressed-in underside with its 1.5 mm standing ring; the paraffin fill is a separate revolve that lies 0.05 mm into the foil, 14 mm deep with the drawn 0.6 mm dome on top; the wick is one revolve of the 2 mm braid with the 8 mm sustainer tab at the floor (the tab keeps the part above the print feature and is hidden in the wax); the flame and its tongue are revolves of the drawn teardrop outlines on their own bones. The wax surface of the drawing was 1.2 mm higher: it is lowered to the 14 mm fill the notes state (a surface line on the very rim made the drawn silhouette protrude over the cup).",
         "Eigenes 1:50-Modell des 38-mm-Teelichts nach den Zeichnungen (Pose brennend): das Schälchen ist eine echte 0,1-mm-Folienschale (eine Drehfläche) mit der konischen Wand von 31 auf 38 mm, einer gerollten Randlippe, einem 0,1-mm-Boden und der eingedrückten Unterseite mit ihrem 1,5 mm breiten Standring; die Paraffinfüllung ist eine eigene Drehfläche, die 0,05 mm in die Folie reicht, 14 mm tief mit der gezeichneten Wölbung von 0,6 mm oben; der Docht ist eine Drehfläche aus dem 2-mm-Geflecht mit dem 8-mm-Dochtfuß am Boden (der Fuß hält das Teil über dem Druckmerkmal und liegt im Wachs verborgen); Flamme und Zunge sind Drehflächen der gezeichneten Tropfenumrisse an ihren eigenen Knochen. Die Wachsoberfläche der Zeichnung lag 1,2 mm höher: sie ist auf die in den Notizen genannte Füllung von 14 mm abgesenkt (eine Oberflächenlinie genau am Rand liess die gezeichnete Silhouette über das Schälchen ragen)."),
    text("Thermal: cup, wax, wick and flame/tongue reference their body solids (the cup region exposes outer cone and the 2 mm of inner wall above the wax; floor and wax sides are buried); the flame region is the operating region (heater, key burn) and its emitter flame-plume is a 20 mm disc of 7.5 mm at the flame-top anchor (tag flame-plume, all faces section faces: a virtual outlet of the rising plume). The unlit pose lowers the flame bones into the cup, so no hidden region is needed.",
         "Thermik: Schälchen, Wachs, Docht und Flamme/Zunge verweisen auf ihre Körper (die Schälchenregion zeigt den Aussenkegel und die 2 mm Innenwand über dem Wachs; Boden und Wachsflanken sind verdeckt); die Flammenregion ist die Betriebsregion (Heizer, Schlüssel burn) und ihr Emitter flame-plume eine 20-mm-Scheibe von 7,5 mm am Anker flame-top (Markierung flame-plume, alle Flächen Schnittflächen: ein virtueller Auslass der aufsteigenden Fahne). Die Pose unbrennend senkt die Flammenknochen ins Schälchen, daher braucht es keine verborgene Region."),
    text("Structure: the cup shell, the wax, the wick and the flame bones are members (referencing their solids), wick/flame/flame-body the joints, a wax mass ellipsoid at the 12.8 g centre of mass and the 31 mm contact disc under the base as the support of the anchor base-contact.",
         "Tragwerk: Schälchenschale, Wachs, Docht und die Flammenknochen sind Stäbe (mit Verweis auf ihre Körper), Docht/Flamme/Flammenkörper die Gelenke, ein Wachs-Massenellipsoid im Schwerpunkt der 12,8 g und die 31-mm-Kontaktscheibe unter dem Boden als Auflager des Ankers base-contact."),
]

# ---------------------------------------------------------------- 1:100
rin100 = lambda y: 0.0154 + 0.25 * y


def cup_profile100():
    return [[0.0001, 0.0], [0.0, 0.0], [0.0, 0.0155], [0.014, 0.019], [TOP, 0.0195], [TOP, 0.0194], [0.014, 0.0189], [0.0001, 0.0154]]


def plug(ytop, rin, wick_top):
    pts = [[0.00008, 0.0], [0.00008, rin(0.00008) + 0.00005], [ytop, rin(ytop) + 0.00005]]
    redge = rin(ytop) + 0.00005
    for f in (0.75, 0.5, 0.25):
        pts.append([ytop + 0.0006 * (1 - f * f), f * redge])
    pts.append([ytop + 0.0006, 0.001])
    pts.append([wick_top, 0.001])
    pts.append([wick_top, 0.0])
    return pts


body100 = [
    rev("cup-shell", CUP, cup_profile100(), "v"),
    rev("wax-wick", WIK(region=None, regions=["wax", "wick"], material="paraffin", tone=0.85), [[y - 0.0145, r] for y, r in plug(0.014, rin100, 0.0198)], "v"),
    rev("flame", FLB, half_profile(100, "flame"), "v"),
    rev("flame-tongue", FLT, half_profile(100, "tongue"), "v"),
]
th100 = [
    th("th-cup-shell", "cup-shell", "cup", "foil"),
    tag(th("th-wax", "wax-wick", "wax", "wax"), sectionFaces=["+u", "-u", "+w", "-w", "-v"]),
    tag(rev("th-wick", WIK(layer="braid", role="detail"), [[-0.0145 + 0.0140, 0.0], [-0.0145 + 0.0140, 0.001], [0.0053, 0.001], [0.0053, 0.0]], "v")),
    th("th-flame", "flame", "flame", "envelope"),
    th("th-flame-tongue", "flame-tongue", "flame", "envelope"),
    disc("th-flame-plume", FLT(region="flame"), 0.0135, 0.0285, 0.010, "flame-plume", "envelope"),
]
st100 = [
    stm("member-cup", "cup-shell", "cup", "member"),
    stm("member-wax", "wax-wick", "wax", "member"),
    stm("member-wick", "wax-wick", "wick", "member"),
    stm("member-flame", "flame", "flame", "member"),
    stm("member-flame-body", "flame", "flame-body", "member"),
    stm("member-flame-tip", "flame-tongue", "flame-tip", "member"),
    tag(ell("mass-wax", ST(member="wax", part="mass", bone="wax"), (0, 0.0068 - 0.0014, 0), (0.012, 0.0078, 0.012))),
    pad(0.0155, 0.015, -0.0075),
]
D100 = [
    text("Own 1:100 model: the foil cup as a 0.1 mm shell with a flat floor and the drawn trapezoid (no lip, no standing ring); wax and wick are one revolve on the wick bone (the drawing merges them): the fill to 14 mm with its dome and the 2 mm wick stub up to 19.8 mm, which also keeps the part above the 15 mm print feature; flame and tongue are revolves of the drawn outlines.",
         "Eigenes 1:100-Modell: das Folienschälchen als 0,1-mm-Schale mit flachem Boden und dem gezeichneten Trapez (keine Lippe, kein Standring); Wachs und Docht sind eine Drehfläche am Dochtknochen (die Zeichnung fasst sie zusammen): die Füllung bis 14 mm mit ihrer Wölbung und dem 2-mm-Dochtstummel bis 19,8 mm, was das Teil auch über dem Druckmerkmal von 15 mm hält; Flamme und Zunge sind Drehflächen der gezeichneten Umrisse."),
    text("Thermal: cup, wax and flame/tongue by reference (the wax reference declares its sides and underside, the interface to the foil, as section faces so that only the free top counts), the wick as its own 2 mm braid solid (layer braid, true thickness, buried in the plug above the wax), emitter disc of 15 mm at flame-top. The wax region is not compared per region here (below 6 x feature^2).",
         "Thermik: Schälchen, Wachs und Flamme/Zunge als Verweis (der Wachsverweis erklärt Flanken und Unterseite, die Grenze zur Folie, als Schnittflächen, damit nur die freie Oberseite zählt), der Docht als eigener 2-mm-Geflechtkörper (Schicht braid, wahre Dicke, über dem Wachs im Stopfen verdeckt), Emitterscheibe von 15 mm bei flame-top. Die Wachsregion wird hier nicht je Region verglichen (unter 6 x Merkmal^2)."),
    text("Structure: six members (cup, wax, wick, flame, flame-body, flame-tip), the wax mass ellipsoid (15 mm, the smallest the feature allows, so it ends 1 mm below the floor) and the 31 mm contact disc, which stands under the base (-7.5 to 7.5 mm) so that it does not bury the wax top; the joints of 1:50 are dropped.",
         "Tragwerk: sechs Stäbe (Schälchen, Wachs, Docht, Flamme, Flammenkörper, Flammenspitze), das Wachs-Massenellipsoid (15 mm, das Kleinste, was das Merkmal erlaubt, daher endet es 1 mm unter dem Boden) und die 31-mm-Kontaktscheibe, die unter dem Boden steht (-7,5 bis 7,5 mm), damit sie die Wachsoberseite nicht verdeckt; die Gelenke von 1:50 entfallen."),
]

# ---------------------------------------------------------------- 1:200
rin200 = lambda y: 0.0164 + 0.3125 * y


def cup_profile200():
    return [[0.0001, 0.0], [0.0, 0.0], [0.0, 0.0165], [0.014, 0.020875], [TOP, 0.0215], [TOP, 0.0214], [0.014, 0.020775], [0.0001, 0.0164]]


body200 = [
    rev("cup-shell", CUP, cup_profile200(), "v"),
    rev("wax-wick", CUP(region=None, regions=["wax", "wick"], material="paraffin", tone=0.85), [list(q) for q in plug(0.014, rin200, 0.0198)], "v"),
    tag(rev("wax-top", WIK(region=None, regions=["wax", "wick"], material="paraffin", tone=0.85, role="detail"), [[-0.0010, 0.0], [-0.0010, 0.0185], [-0.0005, 0.0185], [-0.0005, 0.0]], "v"), texture=True),
    rev("flame", FLB, half_profile(200, "flame"), "v"),
]
th200 = [
    th("th-cup-shell", "cup-shell", "cup", "foil"),
    tag(th("th-wax", "wax-wick", "wax", "wax"), sectionFaces=["+u", "-u", "+w", "-w", "-v"]),
    tag(rev("th-wick", WIK(layer="braid", role="detail"), [[-0.0145 + 0.0140, 0.0], [-0.0145 + 0.0140, 0.001], [0.0053, 0.001], [0.0053, 0.0]], "v")),
    th("th-flame", "flame", "flame", "envelope"),
    disc("th-flame-plume", FLT(region="flame"), 0.0135, 0.0435, 0.010, "flame-plume", "envelope"),
]
st200 = [
    stm("member-cup", "cup-shell", "cup", "member"),
    stm("member-wax", "wax-wick", "wax", "member"),
    stm("member-wick", "wax-top", "wick", "member"),
    stm("member-flame", "flame", "flame", "member"),
    stm("member-flame-body", "flame", "flame-body", "member"),
    stm("member-flame-tip", "flame", "flame-tip", "member"),
    tag(ell("mass-wax", ST(member="wax", part="mass", bone="wax"), (0, -0.0004 - 0.0014, 0), (0.0165, 0.015, 0.0165))),
    pad(0.0155, 0.03, -0.0165),
]
D200 = [
    text("Own 1:200 model: the foil cup shell with the drawn thickened trapezoid (33 to 43 mm), the merged wax-and-wick plug on the cup bone (its fill reaches 19.8 mm, above the 16 mm drawn extent of the cup), the one drawn flame drop of 22 mm as a revolve of its outline; the wick bone is drawn only as the wax-top disc of the plan, modelled as the 0.5 mm wax top skin flagged texture (a surface sheet below every print feature, left out of silhouette and feature rules).",
         "Eigenes 1:200-Modell: die Folienschale mit dem gezeichneten verdickten Trapez (33 bis 43 mm), der zusammengefasste Wachs-Docht-Stopfen am Schälchenknochen (seine Füllung reicht bis 19,8 mm, über die gezeichnete Ausdehnung des Schälchens von 16 mm), der eine gezeichnete Flammentropfen von 22 mm als Drehfläche seines Umrisses; der Dochtknochen ist nur als Wachsscheibe der Draufsicht gezeichnet, modelliert als 0,5 mm dicke Wachshaut mit Kennzeichen texture (eine Oberflächenschicht unter jedem Druckmerkmal, aus Silhouetten- und Merkmalregeln ausgenommen)."),
    text("Thermal: cup and wax by reference, flame as the one drop, the wick as its own 2 mm braid solid, emitter disc of 30 mm at flame-top; only the total area is compared at this scale.",
         "Thermik: Schälchen und Wachs als Verweis, Flamme als der eine Tropfen, der Docht als eigener 2-mm-Geflechtkörper, Emitterscheibe von 30 mm bei flame-top; in diesem Maßstab wird nur die Gesamtfläche verglichen."),
    text("Structure: six members referencing the solids, a 30 mm wax mass ellipsoid (the smallest the feature allows, centred at the ground so that it ends below the wax top) and the 30 mm contact disc beneath the base (-16.5 to 13.5 mm); no joints.",
         "Tragwerk: sechs Stäbe mit Verweis auf die Körper, ein 30-mm-Wachs-Massenellipsoid (das Kleinste, was das Merkmal erlaubt, am Boden zentriert, damit es unter der Wachsoberseite endet) und die 30-mm-Kontaktscheibe unter dem Boden (-16,5 bis 13,5 mm); keine Gelenke."),
]
if __name__ == "__main__":
    write("tealight", {50: dict(body=body50, thermal=th50, structure=st50, decisions=D50),
                       100: dict(body=body100, thermal=th100, structure=st100, decisions=D100),
                       200: dict(body=body200, thermal=th200, structure=st200, decisions=D200)}, category="objects")
