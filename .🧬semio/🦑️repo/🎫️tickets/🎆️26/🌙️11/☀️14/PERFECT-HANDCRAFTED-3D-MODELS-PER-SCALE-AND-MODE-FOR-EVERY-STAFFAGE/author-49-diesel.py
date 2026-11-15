import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "author-36-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("diesel-generator")
CS = dict(material="container-steel", role="body")
CSD = dict(material="container-steel", role="detail")
ES = dict(material="exhaust-steel", role="body")
ESD = dict(material="exhaust-steel", role="detail")
GS = dict(material="galvanised-steel", role="body")
GSD = dict(material="galvanised-steel", role="detail")
AL = dict(material="aluminium", role="detail")
rect = lambda a0, a1, b0, b1: [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]
HOOD = [(1.9, 2.59), (3.03, 2.59), (3.03, 3.25), (2.2, 3.25), (1.9, 2.95)]
tilt = math.radians(30)
AXF = ((1, 0, 0), (0, math.cos(tilt), math.sin(tilt)), (0, -math.sin(tilt), math.cos(tilt)))
AXR = ((1, 0, 0), (0, math.cos(tilt), -math.sin(tilt)), (0, math.sin(tilt), math.cos(tilt)))
EPS = 1.03


def container(skid_h=0.2, roof_t=0.1, pt=0.0, sw=1.94):
    V.box("skid-tank", "container", [0, skid_h / 2, 0], [6.06, skid_h, sw], tone=0.3, region="skid-tank", **CS)
    if pt:
        V.box("wall", "container", [0, (skid_h + 2.59 - roof_t) / 2, 0], [6.06, 2.59 - roof_t - skid_h, 2.44 - 2 * pt], tone=0.55, region="enclosure-walls", layer="skin", **CS)
        holes = [rect(-2.8, -1.5, 0.55, 2.05), rect(-0.2, 1.1, 0.55, 2.05)]
        V.ext("wall-plate", "container", "side", rect(-3.03, 3.03, skid_h, 2.59 - roof_t), 1.22 - pt, 1.22, holes=holes, tone=0.55, region="enclosure-walls", layer="skin", rep=dict(count=2, step=[0, 0, -(2.44 - pt)]), **CS)
    else:
        V.box("wall", "container", [0, (skid_h + 2.59 - roof_t) / 2, 0], [6.06, 2.59 - roof_t - skid_h, 2.44], tone=0.55, region="enclosure-walls", layer="skin", **CS)
    V.box("roof-band", "container", [0, 2.59 - roof_t / 2, 0], [6.06, roof_t, 2.44], tone=0.4, region="roof", **CS)


def castings(t, h=0.2):
    V.box("corner-casting-a", "container", [-2.905, h / 2, -1.095], [0.25, h, 0.25], tone=0.2, region="skid-tank", rep=dict(count=2, step=[0, 0, 2.19]), **CSD)
    V.box("corner-casting-b", "container", [2.905, h / 2, -1.095], [0.25, h, 0.25], tone=0.2, region="skid-tank", rep=dict(count=2, step=[0, 0, 2.19]), **CSD)


def exhaust(cap_cr, t_cap, knuckle, bracket, clamps, flange):
    V.cap("silencer", "silencer", [-1.725, 2.865, -0.9], [-0.075, 2.865, -0.9], 0.275, tone=0.35, region="silencer", **ES)
    if flange:
        V.cyl("silencer-flange", "silencer", [0.0, 2.865, -0.9], [0.08, 2.865, -0.9], 0.285, tone=0.2, region="silencer", **ESD)
    if clamps:
        V.cyl("silencer-clamp-1", "silencer", [-1.6, 2.865, -0.9], [-1.48, 2.865, -0.9], 0.285, tone=0.2, region="silencer", **ESD)
        V.cyl("silencer-clamp-2", "silencer", [-0.5, 2.865, -0.9], [-0.38, 2.865, -0.9], 0.285, tone=0.2, region="silencer", **ESD)
    V.cyl("exhaust-stack", "stack", [-1.2, 3.1, -0.9], [-1.2, 3.47, -0.9], 0.1, tone=0.2, region="exhaust-stack", **ES)
    if bracket:
        V.box("stack-hinge-bracket", "stack", [-1.07, 3.43, -0.9], [0.07, 0.08, 0.1], tone=0.3, region="exhaust-stack", **ESD)
    V.box("rain-cap", "cap-flap", [-1.2, 3.47 + t_cap / 2, -0.9], [0.3, t_cap, 0.3], cr=min(cap_cr, t_cap * 0.45), tone=0.3, region="rain-cap", **GS)
    if knuckle:
        V.cyl("rain-cap-knuckle", "cap-flap", [-1.05, 3.47, -0.96], [-1.05, 3.47, -0.84], knuckle, tone=0.3, region="rain-cap", **GSD)


def hood(t, frame):
    V.ext("hood-side-left", "outlet-hood", "side", HOOD, 0.9 - t, 0.9, tone=0.4, region="discharge-hood", **CS)
    V.ext("hood-side-right", "outlet-hood", "side", HOOD, -0.9, -0.9 + t, tone=0.4, region="discharge-hood", **CS)
    V.box("hood-slope", "outlet-hood", [2.05, 3.1, 0], [0.4243 + (t - 0.04) * 0.5, t, 1.8 - 2 * t], rz=45, tone=0.4, region="discharge-hood", **CS)
    V.box("hood-top", "outlet-hood", [2.615, 3.25 - t / 2, 0], [0.83, t, 1.8 - 2 * t], tone=0.4, region="discharge-hood", **CS)
    V.box("hood-back", "outlet-hood", [1.9 + t / 2, 2.77, 0], [t, 0.36, 1.8 - 2 * t], tone=0.4, region="discharge-hood", **CS)
    if frame:
        V.ext("hood-outlet-frame", "outlet-hood", "front", rect(2.59, 3.25, -0.85, 0.85), 3.03 - frame, 3.03, holes=[rect(2.65, 3.2, -0.8, 0.8)], tone=0.15, region="discharge-hood", material="container-steel", role="detail")


def flap(t):
    V.box("outlet-flap", "outlet-flap", [3.03 - t / 2, 2.935, 0], [t, 0.63, 1.7], tone=0.3, region="outlet-flap", **GS)


def louvres(n, thick, pitch, wide, zc=1.2):
    for k, (x, name) in enumerate([(-2.15, "inlet-louvre-1"), (0.45, "inlet-louvre-2")]):
        for side, z in [("front", -1.2), ("rear", 1.2)]:
            a = AXF if z < 0 else AXR
            V.box(f"{name}-{side}-slat", "container", [x, 0.62, zc * (1 if z > 0 else -1)], [1.3, thick, wide], axes=a, tone=0.25, region="inlet-louvres", rep=dict(count=n, step=[0, pitch, 0]), **({"texture": True} if thick < 0.0075 else {}), **AL)


def emitters(f):
    V.to("thermal")
    t = max(f, 0.0075) * EPS
    V.cyl("th-exhaust-stack-jet", "stack", [-1.2, 3.46, -0.9], [-1.2, 3.46 + t, -0.9], 0.1, tone=0.2, region="exhaust-stack", tags=["exhaust-stack-jet"], **ESD)
    V.cyl("th-radiator-air", "outlet-hood", [3.025, 2.9, 0], [3.025 + t, 2.9, 0], 0.6, tone=0.4, region="discharge-hood", tags=["radiator-air"], **CSD)


def structure(scale, f, extra_members=()):
    V.to("structure")
    ids = {s["id"]: s for s in V.lists["body"]}
    for sid, bone in [("skid-tank", "container"), ("wall", "container"), ("roof-band", "container"), ("silencer", "silencer"), ("exhaust-stack", "stack"), ("rain-cap", "cap-flap"), ("outlet-flap", "outlet-flap"), ("hood-side-left", "outlet-hood"), ("hood-side-right", "outlet-hood"), ("hood-slope", "outlet-hood"), ("hood-top", "outlet-hood"), ("hood-back", "outlet-hood"), ("corner-casting-a", "container"), ("corner-casting-b", "container")]:
        if sid in ids:
            V.member(sid, bone)
    if "stack-hinge-bracket" in ids:
        V.member("stack-hinge-bracket", "stack", "joint")
    else:
        V.box("joint-stack-bracket", "stack", [-1.2, 3.4, -0.9], [max(0.07, f * 1.3), max(0.08, f * 1.1), max(0.1, f * 1.3)], tone=0.3, member="stack", part="joint", **ESD)
    if "rain-cap-knuckle" in ids:
        V.member("rain-cap-knuckle", "cap-flap", "joint")
    elif scale < 200:
        V.cyl("joint-cap-knuckle", "cap-flap", [-1.05, 3.47, -0.96], [-1.05, 3.47, -0.84], max(0.025, f * 0.7), tone=0.3, member="cap-flap", part="joint", **GSD)
    V.cyl("joint-flap-hinge", "outlet-flap", [3.0, 3.2, -0.8], [3.0, 3.2, 0.8], max(0.03, f * 0.7), tone=0.3, member="outlet-flap", part="joint", **GSD)
    for tag, x, z in [("a-l", -2.9, 1.13), ("a-r", -2.9, -1.13), ("b-l", 2.9, 1.13), ("b-r", 2.9, -1.13)]:
        V.box(f"support-contact-{tag}", "container", [x, f / 2, z], [0.25, f, 0.25], tone=0.2, member="container", part="support", anchor=f"contact-{tag}", **CSD)
    V.ell("mass-container", "container", [0, 1.1, 0], [1.6, 0.6, 0.7], tone=0.2, member="container", part="mass", **CS)
    if f < 0.01:
        V.ell("mass-fuel", "container", [0, 0.1, 0], [1.8, 0.09 if f < 0.01 else 0.09, 0.8], tone=0.2, member="container", part="mass", **CS)


def s50():
    container(0.2, 0.1, 0.06)
    castings(0)
    V.box("fuel-fill", "container", [-2.7, 0.1, -1.0], [0.4, 0.1, 0.04], tone=0.2, region="skid-tank", **CSD)
    V.box("roof-edge", "container", [0, 2.5975, 0], [5.86, 0.015, 2.24], tone=0.4, region="roof", **CSD)
    V.ext("lifting-eye", "container", "side", rect(2.5, 2.8, 2.59, 2.66), 0.985, 1.015, holes=[[(2.65 + 0.018 * math.cos(2 * math.pi * k / 12), 2.625 + 0.018 * math.sin(2 * math.pi * k / 12)) for k in range(12)]], tone=0.2, region="roof", rep=dict(count=2, step=[0, 0, -2.0]), **CSD)
    louvres(15, 0.002, 0.1, 0.035, 1.19)
    V.box("door-line", "container", [1.875, 1.3, -1.2275], [0.75, 1.7, 0.015], tone=0.5, region="enclosure-walls", layer="skin", **CSD)
    V.box("door-handle", "container", [1.58, 1.35, -1.23], [0.06, 0.3, 0.02], tone=0.2, region="enclosure-walls", layer="skin", **CSD)
    V.box("end-door", "container", [3.0375, 1.3, 0], [0.015, 1.7, 1.0], tone=0.5, region="enclosure-walls", layer="skin", **CSD)
    exhaust(0.02, 0.1, 0.025, True, True, True)
    hood(0.04, 0.05)
    flap(0.06)
    V.auto_thermal()
    V.to("thermal")
    V.box("th-wall-wool", "container", [0, (0.2 + 2.49) / 2, 0], [6.056, 2.286, 2.316], tone=0.8, region="enclosure-walls", layer="wool", **CS)
    emitters(0.0075)
    structure(50, 0.0077)
    V.decide("The container is a plain box on a skid tank with four corner castings (the one 2.44 m wide block the drawings agree on), louvres are 15 tilted real 2 mm aluminium slats per panel (texture sheets: the louvre is drawn as a grille, and the 2 mm sheets give the honest 5.8 m2 of the hand calculation), two panels on each long side (the elevation draws one side), door and handle are raised panels on the drawn side and the end door a raised panel on the +x end; the roof carries a 15 mm edge plate and two lifting eyes with real 36 mm holes.",
             "Der Container ist ein glatter Kasten auf einem Sockeltank mit vier Eckbeschlägen (der eine 2,44 m breite Block, auf den die Zeichnungen sich einigen), die Lamellen sind 15 geneigte echte 2-mm-Aluminiumlamellen je Feld (Texturbleche: die Lamelle wird als Gitter gezeichnet, und die 2-mm-Bleche ergeben die ehrlichen 5,8 m2 der Handrechnung), zwei Felder auf jeder Längsseite (die Ansicht zeichnet eine Seite), Tür und Griff sind aufgesetzte Felder auf der gezeichneten Seite und die Stirntür ein aufgesetztes Feld am +x-Ende; das Dach trägt eine 15 mm dicke Randplatte und zwei Kranösen mit echten 36-mm-Löchern.")
    V.decide("The silencer is a capsule along x at z -0.9 (0.55 m diameter as in the front ellipse) with flange and two clamp rings, the stack stands vertical at x -1.2 with its hinge bracket, the rain cap is one rounded plate with a knuckle on its own bone; the discharge hood is a shell (two side plates of the elevation pentagon, sloped front, top and back panel and an outlet frame with the 1.6 m by 0.55 m mouth) and the outlet flap one plate on its own bone covering the mouth.",
             "Der Schalldämpfer ist eine Kapsel entlang x bei z -0,9 (0,55 m Durchmesser wie die Ellipse der Vorderansicht) mit Flansch und zwei Klemmringen, das Abgasrohr steht senkrecht bei x -1,2 mit seinem Scharnierwinkel, die Regenhaube ist eine gerundete Platte mit Knoten auf eigenem Knochen; die Ausblashaube ist eine Hülle (zwei Seitenbleche des Fünfecks der Ansicht, schräge Front-, Deck- und Rückplatte und ein Auslassrahmen mit der Mündung 1,6 m mal 0,55 m) und die Auslassklappe eine Platte auf eigenem Knochen vor der Mündung.")
    V.decide("Thermal: the enclosure walls are two nested shells, the 2 mm skin (the body wall box with its doors and handle) over a 50 mm wool shell that is the same box shrunk by the skin; all other regions reference their body solids, the exhaust jet is a 200 mm nozzle disc on the stack mouth and the radiator air a 1.2 m disc in front of the outlet flap. The authored region areas were corrected to the exposed surfaces of these solids.",
             "Thermik: Die Gehäusewände sind zwei verschachtelte Schalen, die 2 mm dicke Haut (der Wandkasten mit Türen und Griff) über einer 50-mm-Wollschale, die derselbe um die Haut verkleinerte Kasten ist; alle übrigen Regionen verweisen auf ihre Körper, der Abgasstrahl ist eine 200-mm-Düsenscheibe an der Rohrmündung und die Kühlerabluft eine 1,2-m-Scheibe vor der Auslassklappe. Die angegebenen Regionsflächen wurden auf die freien Flächen dieser Körper korrigiert.")
    V.decide("Structure: skid tank, corner castings, wall box and roof carry the load to four 250 mm supports at the declared corner anchors; silencer, stack, rain cap, hood panels and flap are members, bracket, knuckle and a flap hinge rod are joints, and two masses carry the machine in the container and the fuel in the skid tank.",
             "Tragwerk: Sockeltank, Eckbeschläge, Wandkasten und Dach tragen die Last in vier 250-mm-Auflager an den angegebenen Eckankern; Schalldämpfer, Abgasrohr, Regenhaube, Haubenbleche und Klappe sind Stäbe, Winkel, Knoten und eine Klappenscharnierstange sind Gelenke, und zwei Massen tragen die Maschine im Container und den Kraftstoff im Sockeltank.")
    V.finish(50)


def s100():
    container(0.2, 0.1, 0.06)
    castings(0)
    V.box("roof-edge", "container", [0, 2.6, 0], [5.86, 0.03, 2.24], tone=0.4, region="roof", **CSD)
    louvres(8, 0.04, 0.19, 0.05, 1.19)
    V.box("door-line", "container", [1.875, 1.3, -1.2325], [0.75, 1.7, 0.03], tone=0.5, region="enclosure-walls", layer="skin", **CSD)
    V.box("end-door", "container", [3.045, 1.3, 0], [0.03, 1.7, 1.0], tone=0.5, region="enclosure-walls", layer="skin", **CSD)
    exhaust(0.03, 0.1, 0.04, True, True, False)
    hood(0.04, 0.03)
    flap(0.06)
    V.auto_thermal()
    V.to("thermal")
    V.box("th-wall-wool", "container", [0, (0.2 + 2.49) / 2, 0], [6.056, 2.286, 2.316], tone=0.8, region="enclosure-walls", layer="wool", **CS)
    emitters(0.0155)
    structure(100, 0.0155)
    V.decide("At 1:100 the louvres are eight 40 mm slats per panel at 190 mm pitch, door and end door are 30 mm panels, fuel fill, handle and lifting eyes are dropped, the roof edge plate is 30 mm, the rain-cap knuckle 40 mm and the silencer flange is merged into the capsule; hood walls stay 40 mm, the outlet frame becomes a 30 mm plate.",
             "Bei 1:100 sind die Lamellen acht 40 mm dicke Lamellen je Feld im Abstand von 190 mm, Tür und Stirntür 30-mm-Felder, Tankstutzen, Griff und Kranösen entfallen, die Dachrandplatte ist 30 mm dick, der Haubenknoten 40 mm, und der Schalldämpferflansch geht in der Kapsel auf; die Haubenwände bleiben 40 mm, der Auslassrahmen wird zu einer 30-mm-Platte.")
    V.decide("Thermal: wall skin (door panels included) and wool shell as at 1:50; all other regions reference their body solids; emitters are 15 mm discs.",
             "Thermik: Wandhaut (mit den Türfeldern) und Wollschale wie bei 1:50; alle übrigen Regionen verweisen auf ihre Körper; Emitter sind 15 mm dicke Scheiben.")
    V.decide("Structure: the same load path as at 1:50 with 15 mm supports and a single mass in the container (the fuel is included in it).",
             "Tragwerk: derselbe Lastpfad wie bei 1:50 mit 15-mm-Auflagern und einer einzigen Masse im Container (der Kraftstoff ist darin enthalten).")
    V.finish(100)


def s200():
    container(0.2, 0.1, 0.031)
    castings(0)
    for k, x in enumerate([-2.15, 0.45]):
        for side, z in [("front", -1.2), ("rear", 1.2)]:
            V.box(f"inlet-louvre-{k + 1}-{side}-panel", "container", [x, 1.3, (1.22 - 0.0155) * (1 if z > 0 else -1)], [1.3, 1.5, 0.031], tone=0.25, region="inlet-louvres", **AL)
    V.box("door-line", "container", [1.875, 1.3, -1.2355], [0.75, 1.7, 0.031], tone=0.5, region="enclosure-walls", layer="skin", **CSD)
    exhaust(0.04, 0.07, 0.0, False, True, False)
    hood(0.04, 0.0)
    flap(0.06)
    V.auto_thermal()
    V.to("thermal")
    V.box("th-wall-wool", "container", [0, (0.2 + 2.49) / 2, 0], [6.056, 2.286, 2.374], tone=0.8, region="enclosure-walls", layer="wool", **CS)
    emitters(0.031)
    structure(200, 0.031)
    V.decide("At 1:200 the louvre slats become four flat 31 mm aluminium panels (two per long side), the end door, roof edge, castings' separate hole details, handle, fuel fill and eyes vanish, the rain cap is a 70 mm plate without knuckle, the silencer keeps its two 120 mm clamp rings, and the outlet frame disappears into the hood.",
             "Bei 1:200 werden die Lamellen vier flache 31 mm dicke Aluminiumfelder (zwei je Längsseite), Stirntür, Dachrand, Handgriff, Tankstutzen und Ösen entfallen, die Regenhaube ist eine 70 mm dicke Platte ohne Knoten, der Schalldämpfer behält seine zwei 120-mm-Klemmringe, und der Auslassrahmen geht in der Haube auf.")
    V.decide("Thermal: skin and wool as before, the louvre region is the four panels, emitters are 31 mm discs.",
             "Thermik: Haut und Wolle wie zuvor, die Lamellenregion sind die vier Felder, Emitter sind 31 mm dicke Scheiben.")
    V.decide("Structure: as at 1:100 with 31 mm supports and joints sized to the feature, one mass; the rain-cap knuckle is not a separate joint any more.",
             "Tragwerk: wie bei 1:100 mit 31-mm-Auflagern und auf die Mindestgröße bemessenen Gelenken, eine Masse; der Haubenknoten ist kein eigenes Gelenk mehr.")
    V.finish(200)


def s500():
    container(0.2, 0.09, 0.0, 2.44)
    V.rev("silencer", "silencer", [-2.0, 2.855, -0.9], [1, 0, 0], [[0, 0], [0, 0.055], [0.0295, 0.165], [0.11, 0.2455], [0.22, 0.275], [1.98, 0.275], [2.09, 0.2455], [2.1705, 0.165], [2.2, 0.055], [2.2, 0]], tone=0.35, region="silencer", **ES)
    V.cyl("exhaust-stack", "stack", [-1.2, 3.1, -0.9], [-1.2, 3.47, -0.9], 0.1, tone=0.2, region="exhaust-stack", **ES)
    V.box("rain-cap", "cap-flap", [-1.2, 3.52, -0.9], [0.3, 0.1, 0.3], tone=0.3, region="rain-cap", **GS)
    hood(0.08, 0.0)
    flap(0.1)
    V.auto_thermal()
    V.to("thermal")
    V.box("th-wall-wool", "container", [0, (0.2 + 2.5) / 2, 0], [6.056, 2.3, 2.436], tone=0.8, region="enclosure-walls", layer="wool", **CS)
    for side, z in (("front", -1.2555), ("rear", 1.2555)):
        for k, x in enumerate([-2.15, 0.45]):
            V.box(f"th-louvre-{k + 1}-{side}", "container", [x, 1.3, z], [1.3, 1.5, 0.0755], tone=0.25, region="inlet-louvres", **AL)
    V.box("th-wall-skin", "container", [0, 1.35, 0], [6.06, 2.3, 2.44], tone=0.55, region="enclosure-walls", layer="skin", **CS) if False else None
    emitters(0.0755)
    structure(500, 0.0755)
    V.decide("At 1:500 the container is three slabs (a full-width 200 mm skid tank as the 1:500 front draws it, wall box, 90 mm roof band) without castings, doors, louvres or edge plates, the silencer is one revolved body with the drawn rounded-corner outline (a flat 110 mm end and 220 mm corner radii, not a hemispherical capsule: the 1:500 elevation draws that outline and a capsule left 0.28 m wedges against the roof), the stack a 200 mm cylinder, the rain cap a 100 mm plate, the hood walls are 80 mm plates and the flap a 100 mm plate; clamps, flange, brackets, knuckles and the outlet frame are gone.",
             "Bei 1:500 ist der Container aus drei Platten gebaut (vollbreiter 200-mm-Sockeltank wie in der 1:500-Vorderansicht, Wandkasten, 90 mm dickes Dachband) ohne Beschläge, Türen, Lamellen oder Randplatten, der Schalldämpfer ist ein Drehkörper mit der gezeichneten Kontur mit gerundeten Ecken (110 mm flaches Ende und 220 mm Eckenradius, keine halbkugelige Kapsel: die 1:500-Ansicht zeichnet diese Kontur, und eine Kapsel ließ 0,28 m große Keile gegen das Dach), das Abgasrohr ein 200-mm-Zylinder, die Regenhaube eine 100 mm dicke Platte, die Haubenwände sind 80 mm dicke Platten und die Klappe eine 100 mm dicke Platte; Klemmen, Flansch, Winkel, Knoten und Auslassrahmen entfallen.")
    V.decide("Thermal: louvres are four 75 mm panels standing 37.5 mm proud of the walls (thermal only, the 1:500 drawing merges them into the container silhouette); wall skin is the wall box and the wool shell is nested in it; emitters are 75 mm discs.",
             "Thermik: Die Lamellen sind vier 75 mm dicke Felder, 37,5 mm vor den Wänden stehend (nur Thermik, die 1:500-Zeichnung fasst sie in der Containersilhouette zusammen); die Wandhaut ist der Wandkasten, die Wollschale liegt in ihm; Emitter sind 75 mm dicke Scheiben.")
    V.decide("Structure: three slabs, silencer, stack, cap, hood plates and flap as members, joints and supports sized to 75 mm, one mass.",
             "Tragwerk: drei Platten, Schalldämpfer, Abgasrohr, Haube, Haubenplatten und Klappe als Stäbe, Gelenke und Auflager auf 75 mm bemessen, eine Masse.")
    V.finish(500)


sel = os.environ.get('SC', '500').split(',')
for k in sel:
    {'50': s50, '100': s100, '200': s200, '500': s500}[k]()
V.write([int(k) for k in sel])
