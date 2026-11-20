"""🌱 Round 3, appliances/desktop-monitor: the root corrections outside `volumes` and the 2D shapes: anchors in the corrected bone frames, the plume direction, the reference product, notes and the structural assumption.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def _load(name):
    spec = importlib.util.spec_from_file_location("r3_desktop_monitor_" + name, os.path.join(HERE, f"r3-desktop-monitor-{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


K = _load("kit")
B = _load("body")

DELL = {
    "id": "dell-se2422h-2021",
    "citation": "Dell Technologies (2021). Dell 24 Monitor SE2422H User's Guide, Specifications: physical characteristics (with stand 420.25 x 552.64 x 178.81 mm, without stand 331.61 x 552.64 x 49.69 mm, stand 173.93 x 240.42 x 178.81 mm; 3.77 kg with stand and cables, 3.18 kg without stand, stand 0.33 kg; tilt -5 to 21 degrees). https://www.dell.com/support/manuals/en-us/dell-se2422h-monitor/se2422h_ug/",
}


def anchors(old):
    """⚓ The base contact at the centre of the plate in the base frame that now starts at the rear edge; the plume anchor on the vent slots of the hump."""
    (um, vm), _, n, _ = B.vent_band(B.HOUSING_50, -0.0225, -0.0130)
    spec = {"base-contact": (0.106, 0.0), "housing-top": (K.r6(um + n[0] * 0.0003), K.r6(vm + n[1] * 0.0003))}
    return [dict(a, x=spec[a["id"]][0], y=spec[a["id"]][1]) for a in old]


def text(en, de):
    """🌐 A bilingual text."""
    return {"en": en, "de": de}


def apply(asset):
    """🛠 Writes every root correction into the asset in memory."""
    asset["anchors"] = anchors(asset["anchors"])
    for anchor in asset["anchors"]:
        if anchor["id"] == "housing-top":
            anchor["title"] = text("Vent slots on the upper slope of the rear housing", "Lüftungsschlitze am oberen Hang des Rückgehäuses")
    for emitter in asset["thermal"]["emitters"]:
        if emitter["id"] == "rear-plume":
            emitter["directionDeg"] = -5
    refs = asset["provenance"]["references"]
    if all(r["id"] != DELL["id"] for r in refs):
        refs.append(DELL)
    asset["provenance"]["notes"] = text(
        "Side elevation of a 23.8 to 24-inch (0.61 m diagonal) 16:9 IPS LED monitor on a height-fixed tilt stand of the Dell SE2422H class: panel 0.54 m wide and 0.323 m high with 6.5 mm bezel at the sides and top and a 19 mm chin around the 527 x 296.5 mm active area, panel edge 9.5 mm, the rear cover sloping to a 36 mm deep electronics hump with vent slots on its upper slope and the VESA 100 field with the quick-release bracket of the stand; riser 62 x 22 mm on a 0.25 x 0.21 m base plate 10 mm thick on four rubber pads; tilt hinge at the head of the riser 0.255 m above the desk, tilt -5 to +20 degrees; mass 3.6 kg (panel 3.0 kg, stand 0.6 kg). The sizes are rounded from this product class (Dell SE2422H: 552.6 x 420.3 x 178.8 mm, panel 49.7 mm deep, stand base 240 x 179 mm, 3.77 kg); the asset keeps its narrower 0.54 m panel and deeper 0.21 m base as a class estimate, no single product. The 25 W at level 1 are an authored estimate within the 15 to 35 W class of the measured LCD displays of Wilkins and Hosni 2011, standby 0.25 W follows the 0.5 W ceiling of the ENERGY STAR displays specification. All electrical power ends as heat: 9 W at the panel front (LED backlight and panel), 16 W at the rear housing (power supply, driver boards, backlight driver).",
        "Seitenansicht eines 23,8- bis 24-Zoll-16:9-IPS-LED-Monitors (0,61 m Diagonale) auf einem höhenfesten Neigefuß der Klasse Dell SE2422H: Panel 0,54 m breit und 0,323 m hoch mit 6,5 mm Rahmen seitlich und oben und 19 mm Kinn um die aktive Fläche 527 x 296,5 mm, Panelrand 9,5 mm, die Rückwand fällt zu einem 36 mm tiefen Elektronikbuckel mit Lüftungsschlitzen am oberen Hang und dem VESA-100-Feld mit dem Schnellwechselhalter des Fußes ab; Hals 62 x 22 mm auf einer Fußplatte 0,25 x 0,21 m, 10 mm dick, auf vier Gummifüßen; Neigegelenk am Kopf des Halses 0,255 m über dem Tisch, Neigung -5 bis +20 Grad; Masse 3,6 kg (Panel 3,0 kg, Fuß 0,6 kg). Die Maße sind aus dieser Produktklasse gerundet (Dell SE2422H: 552,6 x 420,3 x 178,8 mm, Panel 49,7 mm tief, Fußplatte 240 x 179 mm, 3,77 kg); das Asset behält sein schmaleres 0,54-m-Panel und die tiefere 0,21-m-Fußplatte als Klassenschätzung, kein einzelnes Produkt. Die 25 W auf Stufe 1 sind ein Schätzwert innerhalb der Klasse von 15 bis 35 W gemessener LCD-Displays von Wilkins und Hosni 2011, der Bereitschaftsbetrieb mit 0,25 W folgt der Obergrenze von 0,5 W der ENERGY-STAR-Anforderungen für Displays. Alle elektrische Leistung endet als Wärme: 9 W an der Panelfront (LED-Hintergrundbeleuchtung und Panel), 16 W am Rückgehäuse (Netzteil, Treiberplatinen, Treiber der Hintergrundbeleuchtung).",
    )
    decisions(asset)
    asset["structure"]["assumptions"] = [text(
        "A rigid body on four rubber pads: the 3.0 kg panel hangs on the tilt hinge at the head of the riser, leans back by 5 degrees and its centre of mass lies 0.025 m in front of the hinge and 0.044 m behind the centre of the base plate, inside the footprint; the riser carries it as a cantilever into the steel-cored base plate.",
        "Ein starrer Körper auf vier Gummifüßen: das 3,0 kg schwere Panel hängt am Neigegelenk am Kopf des Halses, neigt sich um 5 Grad nach hinten und sein Schwerpunkt liegt 0,025 m vor dem Gelenk und 0,044 m hinter der Mitte der Fußplatte, innerhalb der Standfläche; der Hals trägt es als Kragarm in die Fußplatte mit Stahlkern.",
    )]


REPRESENTATION = {
    "50": {
        "silhouette": text("A 0.41 m side view, 8.2 mm tall on paper: the base plate with its bevelled top on two rubber pads, the upright riser with its flared foot, the tilt hinge at its head with the quick-release bracket, and the panel tilted back by 5 degrees, its 9.5 mm edge and the rear cover sloping back to the flat 36 mm deep hump. Front, 10.8 mm by 8.2 mm: the panel with its thin bezel, the dark glass of the active area and the power light in the 19 mm chin, the riser between the chin and the base plate, the plate on its two pads. Plan, 4.2 mm by 10.8 mm: the rounded base plate, the riser foot and the hinge behind the hump, the bracket, the rear cover narrowing backwards with its four vent slots, the top edge of the panel and the glass seen at a slant because the panel leans back.",
                           "Eine Seitenansicht von 0,41 m, auf Papier 8,2 mm hoch: die Fußplatte mit abgeschrägter Oberkante auf zwei Gummifüßen, der aufrechte Hals mit verbreitertem Fuß, das Neigegelenk an seinem Kopf mit dem Schnellwechselhalter und das um 5 Grad nach hinten geneigte Panel, sein 9,5-mm-Rand und die Rückwand, die zum flachen, 36 mm tiefen Buckel abfällt. Vorderansicht, 10,8 mm auf 8,2 mm: das Panel mit schmalem Rahmen, dem dunklen Glas der aktiven Fläche und der Betriebsleuchte im 19-mm-Kinn, der Hals zwischen Kinn und Fußplatte, die Platte auf ihren zwei Füßen. Grundriss, 4,2 mm auf 10,8 mm: die gerundete Fußplatte, Halsfuß und Gelenk hinter dem Buckel, der Halter, die sich nach hinten verjüngende Rückwand mit ihren vier Lüftungsschlitzen, die Oberkante des Panels und das Glas in schräger Sicht, weil das Panel nach hinten geneigt ist."),
        "retained": text("Overall height 0.411 m, base depth 0.21 m and width 0.25 m, the 5 degree tilt, panel 0.54 x 0.323 m with 6.5 mm bezel and 19 mm chin, the 9.5 mm edge, the rear cover sections, the riser 62 x 22 mm, the hinge, the bracket, the pads and the power light.",
                         "Gesamthöhe 0,411 m, Fußtiefe 0,21 m und -breite 0,25 m, die Neigung von 5 Grad, Panel 0,54 x 0,323 m mit 6,5 mm Rahmen und 19 mm Kinn, der 9,5-mm-Rand, die Schnitte der Rückwand, der Hals 62 x 22 mm, das Gelenk, der Halter, die Füße und die Betriebsleuchte."),
        "merged": text("Bezel and LCD module are one frame shape; the rear cover is one shape from the panel edge to the back of the hump.", "Rahmen und LCD-Modul sind eine Rahmenform; die Rückwand ist eine Form vom Panelrand bis zur Rückseite des Buckels."),
        "omitted": text("Ports under the hump, the menu buttons, the power cable and the VESA screws, which are hidden or below the scale. The recessed glass is hidden by the bezel in the side view; hinge, bracket and housing are hidden behind the panel in the front view.",
                        "Anschlüsse unter dem Buckel, die Menütasten, das Netzkabel und die VESA-Schrauben, die verdeckt oder unter dem Maßstab liegen. Das vertiefte Glas ist in der Seitenansicht vom Rahmen verdeckt; Gelenk, Halter und Gehäuse liegen in der Vorderansicht hinter dem Panel."),
        "thinElements": text("The 2 mm rubber pads are drawn at their true height (0.04 mm on paper); the vent slots are four detail strokes in the plan; the power light is a 6 mm detail dot.", "Die 2 mm hohen Gummifüße sind in wahrer Höhe gezeichnet (0,04 mm auf Papier); die Lüftungsschlitze sind vier Detailstriche im Grundriss; die Betriebsleuchte ist ein 6-mm-Detailpunkt."),
        "hatching": text("None.", "Keine."),
        "styles": text("Base 0.5, riser 0.45, hinge 0.35, housing 0.55, bracket 0.42, bezel 0.3, glass 0.1, pads 0.15; body pen 0.18 mm, detail and glazing 0.13 mm.", "Fuß 0,5, Hals 0,45, Gelenk 0,35, Gehäuse 0,55, Halter 0,42, Rahmen 0,3, Glas 0,1, Füße 0,15; Körperstift 0,18 mm, Detail und Glas 0,13 mm."),
        "overlays": text("Panel front, rear housing and stand are three regions: the thermal view shows the warm rear housing with the vent slots, the cooler front and the stand at room temperature.", "Panelfront, Rückgehäuse und Standfuß sind drei Regionen: die Thermalansicht zeigt das warme Rückgehäuse mit den Lüftungsschlitzen, die kühlere Front und den Fuß auf Raumtemperatur."),
    },
    "100": {
        "silhouette": text("A tall flag sign 4.1 mm high on paper: the base plate on the desk, the riser with its rounded head and the back-leaning panel with its edge and the sloping rear hump. Front, 5.4 mm by 4.1 mm: base plate, riser and panel with its glass inside a bezel widened to one feature. Plan, 2.1 mm by 5.4 mm: base plate, riser foot, the rear cover, the panel edge and the slanted glass strip.",
                           "Ein hohes Fahnenzeichen von 4,1 mm Höhe auf Papier: die Fußplatte auf dem Tisch, der Hals mit gerundetem Kopf und das nach hinten geneigte Panel mit Rand und abfallendem Rückenbuckel. Vorderansicht, 5,4 mm auf 4,1 mm: Fußplatte, Hals und Panel mit Glas in einem auf ein Merkmal verbreiterten Rahmen. Grundriss, 2,1 mm auf 5,4 mm: Fußplatte, Halsfuß, Rückwand, Panelrand und der schräge Glasstreifen."),
        "retained": text("Height 0.411 m, base 0.25 x 0.21 m, the 5 degree tilt, the panel size, the sloping rear cover and the riser.", "Höhe 0,411 m, Fuß 0,25 x 0,21 m, die Neigung von 5 Grad, die Panelgröße, die abfallende Rückwand und der Hals."),
        "merged": text("The pads join the base plate; hinge and bracket join the head of the riser.", "Die Füße gehen in der Fußplatte auf; Gelenk und Halter im Kopf des Halses."),
        "omitted": text("Pads, hinge barrel, bracket, vent slots, power light and connectors.", "Füße, Gelenkwalze, Halter, Lüftungsschlitze, Betriebsleuchte und Anschlüsse."),
        "thinElements": text("The bezel around the glass is widened from 6.5 to 15 mm at the sides and top and the chin to 22 mm so that the glass edge reads at one feature; the panel edge is 10.5 mm.", "Der Rahmen um das Glas ist seitlich und oben von 6,5 auf 15 mm und das Kinn auf 22 mm verbreitert, damit die Glaskante mit einem Merkmal lesbar ist; der Panelrand ist 10,5 mm."),
        "hatching": text("None.", "Keine."),
        "styles": text("Base 0.5, riser 0.45, housing 0.55, bezel 0.3, glass 0.1; body pen 0.18 mm, glazing 0.13 mm.", "Fuß 0,5, Hals 0,45, Gehäuse 0,55, Rahmen 0,3, Glas 0,1; Körperstift 0,18 mm, Glas 0,13 mm."),
        "overlays": text("The three regions keep their own shapes, so the rear housing reads warmer than the panel face and the stand.", "Die drei Regionen behalten ihre eigenen Formen, sodass das Rückgehäuse wärmer als Panelfront und Standfuß lesbar ist."),
    },
    "200": {
        "silhouette": text("A flag 2.05 mm high on paper: a thin base plate, a riser bar and a back-leaning panel bar with its rear hump. Front, 2.7 mm by 2 mm: a T of base plate and riser under one panel mass. Plan, 1 mm by 2.7 mm: the base plate, the riser foot and one panel shape narrowing backwards.",
                           "Eine Fahne von 2,05 mm Höhe auf Papier: eine dünne Fußplatte, ein Halsbalken und ein nach hinten geneigter Panelbalken mit Rückenbuckel. Vorderansicht, 2,7 mm auf 2 mm: ein T aus Fußplatte und Hals unter einer Panelmasse. Grundriss, 1 mm auf 2,7 mm: die Fußplatte, der Halsfuß und eine sich nach hinten verjüngende Panelform."),
        "retained": text("Height 0.411 m, base 0.25 x 0.21 m, the 5 degree lean and the rear hump.", "Höhe 0,411 m, Fuß 0,25 x 0,21 m, die Neigung von 5 Grad und der Rückenbuckel."),
        "merged": text("Bezel, glass, module and rear cover form one panel shape; hinge and bracket join the riser.", "Rahmen, Glas, Modul und Rückwand bilden eine Panelform; Gelenk und Halter gehen im Hals auf."),
        "omitted": text("Pads, glass line, vents, light and connectors.", "Füße, Glaslinie, Lüftung, Leuchte und Anschlüsse."),
        "thinElements": text("The riser is widened from 22 to 34 mm deep (0.17 mm) to reach the feature; the panel edge stays 12.5 mm inside the panel mass.", "Der Hals ist von 22 auf 34 mm Tiefe (0,17 mm) verbreitert, um das Merkmal zu erreichen; der Panelrand bleibt 12,5 mm innerhalb der Panelmasse."),
        "hatching": text("None.", "Keine."),
        "styles": text("Base 0.5, riser 0.45, panel 0.4; one body pen of 0.18 mm.", "Fuß 0,5, Hals 0,45, Panel 0,4; ein Körperstift von 0,18 mm."),
        "overlays": text("The panel shape lists panel front and housing as merged regions; the stand is its own region.", "Die Panelform führt Panelfront und Gehäuse als zusammengeführte Regionen; der Fuß ist eine eigene Region."),
    },
}


def decisions(asset):
    """📝 The representation decisions of the redrawn scales, in the member order of the file."""
    for scale, values in REPRESENTATION.items():
        old = asset["representations"][scale]["decisions"]
        asset["representations"][scale]["decisions"] = {key: values.get(key, old[key]) for key in old}
