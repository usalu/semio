"""🌱 Root values of the round 3 city-car outside `volumes` and the drawings: texts of the representations and thermal areas, applied to the parsed asset; and the member replacement that keeps the file's formatting."""
import json
import re

MEMBERS = ["provenance", "representations", "thermal"]
AREAS = {"body-panels": 15.4, "roof": 2.5, "tailgate": 0.9}


def member(text, key):
    found = re.search(r'\n  "' + key + r'"\s*:\s*', text)
    depth, index, quoted = 0, found.end(), False
    while True:
        char = text[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return found.end(), index + 1
        index += 1


def dumped(value):
    lines = json.dumps(value, indent=1, ensure_ascii=False).split("\n")
    return "\n".join(line if i == 0 else "  " + line for i, line in enumerate(lines))


def replace(text, key, value):
    start, end = member(text, key)
    return text[:start] + dumped(value) + text[end:]


REFERENCE = {"id": "b-segment-hatchbacks", "citation": "Manufacturer technical data of B-segment five-door hatchbacks: Volkswagen Polo (6R, 2009): length 3970 mm, width 1682 mm without and 1901 mm with door mirrors, height 1462 mm, wheelbase 2470 mm, track 1463 / 1456 mm; Suzuki Swift (AZG, 2010): 3850 x 1695 x 1510 mm, wheelbase 2430 mm, track 1480 / 1485 mm; Honda Jazz (GE, 2008): 3900 x 1695 x 1525 mm, wheelbase 2500 mm. The asset is a generic car of this class: 3.90 m long, 1.68 m body width, 1.90 m over the mirrors, 1.50 m high, wheelbase 2.45 m, front overhang 0.80 m, rear overhang 0.65 m, track 1.44 m."}
TEXTS = {
    "50": {
        "silhouette": ("True orthographic views of the 1:50 body. Side: hull with both wheel arches as circles of 0.365 m around the 0.30 m tyres, sills, bonnet plane, raked windscreen, roof with its crown, tailgate line and bumper ledge. Front: sills, body side to the belt at 0.84 m half width, tumblehome of the glasshouse to the 0.62 m roof, both door mirrors to the overall width of 1.90 m. Plan: body outline, cowl, windscreen with its plan sweep, sloping side glass beside the roof, bonnet and tailgate.", "Wahre orthogonale Ansichten des Körpers 1:50. Seite: Rumpf mit beiden Radhäusern als Kreise von 0,365 m um die 0,30-m-Reifen, Schweller, Haubenebene, geneigte Windschutzscheibe, Dach mit Wölbung, Heckklappenlinie und Stoßfängerabsatz. Vorn: Schweller, Karosserieflanke bis zur Brüstung mit 0,84 m halber Breite, Einzug des Glashauses zum 0,62-m-Dach, beide Außenspiegel bis zur Gesamtbreite von 1,90 m. Grundriss: Aufbaukontur, Windlauf, Windschutzscheibe mit Pfeilung, geneigte Seitenscheiben neben dem Dach, Haube und Heckklappe."),
        "retained": ("Everything the model has: seat backs, headrests and steering wheel behind the glass, dashboard binnacle in plan, five-spoke wheels with discs and calipers, suspension struts in the arches, tread marks, panel joints, handles, fuel flap, lamps, wipers, number plate, exhaust run.", "Alles, was das Modell hat: Sitzlehnen, Kopfstützen und Lenkrad hinter dem Glas, Instrumentenhutze im Grundriss, Fünfspeichenräder mit Scheiben und Sätteln, Federbeine in den Radhäusern, Profilmarken, Fugen, Griffe, Tankklappe, Leuchten, Wischer, Kennzeichen, Abgasstrang."),
        "merged": ("The A, B and C pillars are the body fill between the glass panes; the fuel flap is an outline.", "A-, B- und C-Säule sind die Karosseriefläche zwischen den Scheiben; die Tankklappe ist ein Umriss."),
        "omitted": ("Door seals, badges, the rear number plate and the wipers in the side view.", "Türdichtungen, Embleme, das hintere Kennzeichen und die Wischer in der Seitenansicht."),
        "overlays": ("All regions are drawn at their real extent, including calipers and the exhaust run. Bonnet and tailgate are their true section profiles on their hinge bones, swept across the car in front and plan, so the open poses read in every view; the width of 1.90 m includes the mirrors.", "Alle Regionen sind in ihrer wahren Ausdehnung gezeichnet, einschließlich Bremssätteln und Abgasstrang. Haube und Heckklappe sind ihre wahren Schnittprofile an ihren Scharnierknochen, in Vorderansicht und Grundriss quer über das Auto gezogen, sodass die offenen Posen in jeder Ansicht lesbar sind; die Breite von 1,90 m schließt die Spiegel ein."),
    },
    "100": {
        "silhouette": ("Hatchback profile with circular wheel arches 0.065 m above each tyre (0.65 mm), sills, separate roof band, two side panes between the pillars, bonnet and tailgate as their real section profiles. Front: body side, tumblehome, roof crown and the door mirrors (1.90 m overall). Plan: body outline, cowl, windscreen, side glass, roof, bonnet, tailgate with its glass, mirrors and lamps.", "Schrägheckprofil mit kreisförmigen Radhäusern 0,065 m über jedem Reifen (0,65 mm), Schwellern, eigenem Dachband, zwei Seitenscheiben zwischen den Säulen, Haube und Heckklappe als wahre Schnittprofile. Vorn: Karosserieflanke, Einzug, Dachwölbung und die Außenspiegel (1,90 m gesamt). Grundriss: Aufbaukontur, Windlauf, Windschutzscheibe, Seitenscheiben, Dach, Haube, Heckklappe mit Scheibe, Spiegel und Leuchten."),
        "retained": ("Door joints (front, B, rear), headlamp, tail lamp, mirror, exhaust pipe, silencer and tail pipe, tyres with rim, disc, five spokes and hub, engine-bay slot, grille; steering wheel (0.34 m = 3.4 mm), front seat backs with headrests and the rear bench behind the glass.", "Türfugen (vorn, B, hinten), Scheinwerfer, Rückleuchte, Spiegel, Abgasrohr, Schalldämpfer und Endrohr, Reifen mit Felge, Scheibe, fünf Speichen und Nabe, Motorraumschlitz, Grill; Lenkrad (0,34 m = 3,4 mm), Vordersitzlehnen mit Kopfstützen und die Rückbank hinter dem Glas."),
        "merged": ("Belt crease, sill line and bumper lines merge into the body outline; the caliper merges into the disc; grille and intake are one opening.", "Brüstungssicke, Schwellerlinie und Stoßfängerlinien gehen in der Aufbaukontur auf; der Sattel geht in der Scheibe auf; Grill und Lufteinlass sind eine Öffnung."),
        "omitted": ("Door handles, fuel flap, number plates, wipers, rear headrests, sidewall and tread (below 0.04 m = 0.4 mm).", "Türgriffe, Tankklappe, Kennzeichen, Wischer, hintere Kopfstützen, Flanke und Profil (unter 0,04 m = 0,4 mm)."),
        "thinElements": ("Spokes are 0.05 m wide (0.5 mm), bonnet and tailgate 0.04 to 0.06 m thick (0.4 to 0.6 mm), all drawn at real size.", "Speichen sind 0,05 m breit (0,5 mm), Haube und Heckklappe 0,04 bis 0,06 m dick (0,4 bis 0,6 mm), alles in wahrer Größe gezeichnet."),
    },
    "200": {
        "silhouette": ("Hatchback profile with open wheel arches, a lighter roof band, two side panes separated by the B-pillar and the dark engine-bay slot under the bonnet; each tyre keeps 0.065 m (0.33 mm) to its arch. Front: body, tumblehome, roof, windscreen, headlamps and the door mirrors, which give the overall width of 1.90 m. Plan: body outline, bonnet, roof, tailgate with its glass, windscreen, side glass, mirrors and lamps.", "Schrägheckprofil mit offenen Radhäusern, hellerem Dachband, zwei durch die B-Säule getrennten Seitenscheiben und dem dunklen Motorraumschlitz unter der Haube; jeder Reifen hält 0,065 m (0,33 mm) Abstand zum Radhaus. Vorn: Aufbau, Einzug, Dach, Windschutzscheibe, Scheinwerfer und die Außenspiegel, die die Gesamtbreite von 1,90 m ergeben. Grundriss: Aufbaukontur, Haube, Dach, Heckklappe mit Scheibe, Windschutzscheibe, Seitenscheiben, Spiegel und Leuchten."),
        "retained": ("Body, roof band, front and rear side glass, bonnet and tailgate, tailgate glass, silencer with tail pipe, tyres with wheel discs and hubs, mirrors (0.2 m = 1 mm), headlamps and tail lamps, two door joints.", "Aufbau, Dachband, vordere und hintere Seitenscheibe, Haube und Heckklappe, Heckscheibe, Schalldämpfer mit Endrohr, Reifen mit Radscheiben und Naben, Spiegel (0,2 m = 1 mm), Scheinwerfer und Rückleuchten, zwei Türfugen."),
        "merged": ("Rim and brake disc are one lighter wheel disc (brakes region); the sills are part of the body outline; grille, bumpers and the exhaust pipe under the floor are part of the body shape.", "Felge und Bremsscheibe sind eine hellere Radscheibe (Region Bremsen); die Schweller gehören zur Aufbaukontur; Grill, Stoßfänger und das Abgasrohr unter dem Boden gehören zur Aufbaufläche."),
        "omitted": ("Spokes, calipers, handles, belt crease, seats, steering wheel, cowl, tread (all below 0.1 m or hidden behind the glass tone).", "Speichen, Sättel, Griffe, Brüstungssicke, Sitze, Lenkrad, Windlauf, Profil (alle unter 0,1 m oder hinter dem Glaston verborgen)."),
        "thinElements": ("Bonnet (0.04 m) and tailgate (0.05 m) are drawn at their real thickness of 0.2 to 0.25 mm; the tail pipe is 0.054 m (0.27 mm).", "Haube (0,04 m) und Heckklappe (0,05 m) sind in ihrer wahren Dicke von 0,2 bis 0,25 mm gezeichnet; das Endrohr misst 0,054 m (0,27 mm)."),
        "overlays": ("Every drawn thermal region has its own shape: panels, roof band, bonnet, tailgate, glass, engine-bay slot, exhaust, tyres, wheel discs for the brakes; the engine bay shows when the bonnet is raised.", "Jede gezeichnete thermische Region hat ihre eigene Form: Bleche, Dachband, Haube, Heckklappe, Glas, Motorraumschlitz, Abgasanlage, Reifen, Radscheiben für die Bremsen; der Motorraum zeigt sich bei geöffneter Haube."),
    },
    "500": {
        "silhouette": ("One body silhouette (hatchback profile) with both wheel arches open by 0.065 m (0.13 mm) over two solid wheel discs, one glass band and the two door mirrors (0.14 m = 0.28 mm) that give the overall width of 1.90 m. Front: body, tumblehome, glass and mirrors. Plan: body outline, roof, windscreen, the sloping side glass, bonnet and tailgate.", "Eine Aufbausilhouette (Schrägheckprofil) mit beiden Radhäusern, 0,065 m (0,13 mm) offen über zwei vollen Radscheiben, einem Glasband und den beiden Außenspiegeln (0,14 m = 0,28 mm), die die Gesamtbreite von 1,90 m ergeben. Vorn: Aufbau, Einzug, Glas und Spiegel. Grundriss: Aufbaukontur, Dach, Windschutzscheibe, die geneigten Seitenscheiben, Haube und Heckklappe."),
        "retained": ("Body, wheel wells, one glazing band, roof, bonnet and tailgate (so both still open on their hinge bones), both tyres and the mirrors.", "Aufbau, Radhäuser, ein Verglasungsband, Dach, Haube und Heckklappe (beide öffnen weiter an ihren Gelenken), beide Reifen und die Spiegel."),
        "omitted": ("Door joints, handles, lamps, bumper lines, seats, tread and every wheel detail; exhaust (0.08 m = 0.16 mm); steering wheel and headrests behind the glass tone.", "Türfugen, Griffe, Leuchten, Stoßfängerlinien, Sitze, Profil und jedes Raddetail; Abgasanlage (0,08 m = 0,16 mm); Lenkrad und Kopfstützen hinter dem Glaston."),
        "thinElements": ("Bonnet and tailgate are drawn 0.076 m thick (0.15 mm, real 0.05 m), the smallest printable feature; they lie inside the body outline, so the closed outline equals the real one.", "Haube und Heckklappe sind 0,076 m dick gezeichnet (0,15 mm, real 0,05 m), das kleinste druckbare Merkmal; sie liegen innerhalb der Aufbaukontur, sodass der geschlossene Umriss dem wahren entspricht."),
    },
}
HIDDEN_INTERIOR = ("The interior fittings are drawn behind the glass at 1:50 and 1:100 only; at 1:200 and coarser they are too small to print and the glazing and body panels carry the cabin, while the model keeps the cabin as real solids inside the body.", "Die Innenausstattung ist nur in 1:50 und 1:100 hinter dem Glas gezeichnet; ab 1:200 ist sie zu klein zum Drucken, Verglasung und Karosseriebleche tragen die Kabine, während das Modell die Kabine als echte Körper im Aufbau behält.")


def apply(asset):
    references = asset["provenance"]["references"]
    if all(reference["id"] != REFERENCE["id"] for reference in references):
        references.insert(0, REFERENCE)
    for scale, texts in TEXTS.items():
        for key, (en, de) in texts.items():
            asset["representations"][scale]["decisions"][key] = {"en": en, "de": de}
    for region in asset["thermal"]["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
        if region["id"] == "interior":
            region["hidden"] = {"en": HIDDEN_INTERIOR[0], "de": HIDDEN_INTERIOR[1]}
    return asset
