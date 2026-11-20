"""🛠️ Author digital-only component and equipment volumes in the catalog's measured local frames."""

import copy
import json
import re
from pathlib import Path

TICKET = Path(__file__).resolve().parent
CATALOG = next((TICKET.parents[6] / "🎓️teaching").glob("*architecture/*staffage/*catalog"))
LISTS = ("body", "thermal", "structure")


def by_id(items, name):
    return next(item for item in items if item["id"] == name)


class Volume:
    def __init__(self, source, scale):
        self.source = source
        self.value = {"status": "authored", "denominator": scale,
                      **{key: copy.deepcopy(source[key]) for key in LISTS}, "decisions": []}

    def part(self, name, group="body"):
        return by_id(self.value[group], name)

    def remove(self, names, group="body"):
        ids = names.split()
        self.value[group] = [item for item in self.value[group] if item["id"] not in ids]

    def replace(self, name, geometry, group="body"):
        part = self.part(name, group)
        for key in ("kind", "size", "cornerRadius", "transform", "plane", "profile", "from", "to",
                    "holes", "radii", "sections", "capStart", "capEnd", "axis", "origin", "repeat", "slender"):
            part.pop(key, None)
        part.update(geometry)

    def reference(self, name, body, group):
        old = self.part(name, group)
        attrs = {k: old[k] for k in ("id", "region", "member", "part", "anchor", "layer", "sectionFaces", "tags") if k in old}
        old.clear()
        old.update(attrs, of=body)

    def note(self, en, de):
        self.value["decisions"].append({"en": en, "de": de})

    def finish(self):
        ids = {part["id"] for part in self.value["body"]}
        for group in ("thermal", "structure"):
            for part in self.value[group]:
                target = part.get("of")
                if target and target not in ids:
                    del part["of"]
                    shape = by_id(self.source["body"], target)
                    part.update({k: copy.deepcopy(v) for k, v in shape.items() if k not in part})
        return self.value


def box(size, centre):
    return {"kind": "box", "size": size, "transform": {"translation": centre}}


def extrude(plane, profile, start, end, holes=None):
    solid = {"kind": "extrude", "plane": plane, "profile": profile, "from": start, "to": end}
    if holes:
        solid["holes"] = holes
    return solid


def station(part, indexes):
    part["profile"] = [part["profile"][i] for i in indexes]


def balustrade(v):
    v.part("handrail")["profile"] = [[1.1, .017], [1.085, .0265], [1.047, .017], [1.047, -.017], [1.085, -.0265], [1.1, -.017]]
    v.replace("support-base-clamp-block", extrude("xy", [[0,-.15],[0,.15],[.045,.122],[.045,-.122]], -1, 1), "structure")
    v.note("Minimal: three members describe the continuous shoe, laminated pane and faceted cap. The structural section shows the two bearing shoulders of the base extrusion.", "Minimal: Drei Körper zeigen das durchgehende Schuhprofil, die Verbundglasscheibe und den facettierten Handlauf. Im Tragwerk sind die beiden Auflagerschultern des Fußprofils sichtbar.")


def window(v):
    v.remove("sill")
    lining = v.part("lining")
    lining["profile"] = [[-.05,-.45],[1.274,-.45],[1.274,.45],[-.05,.45]]
    lining["holes"] = [[[0,-.4],[0,.4],[1.224,.4],[1.224,-.4]]]
    v.remove("j-hinge", "structure")
    v.reference("m-lining", "lining", "structure")
    v.reference("j-stay", "sash-frame", "structure")
    v.part("th-lining","thermal").pop("of")
    v.part("th-lining","thermal").update({k:x for k,x in by_id(v.source["body"],"lining").items() if k != "id"})
    v.note("Minimal: two glass surfaces, exposed sash timber, fixed lining timber and the projecting sill form five thermal surfaces.", "Minimal: Zwei Glasflächen, Flügelholz, Laibungsholz und die auskragende Sohlbank bilden fünf thermische Flächen.")
    v.note("The fixed sill and jambs form a single four-sided lining. The opening remains clear around the independently tilting glazed sash. The bottom hinge supports stay at the two actual pivots.", "Sohlbank und Laibung bilden einen umlaufenden Rahmen. Der verglaste Kippflügel bleibt unabhängig beweglich. Die beiden unteren Lager sitzen weiterhin an den Drehpunkten.")


def radiator(v):
    v.part("radiator").pop("cornerRadius")
    v.remove("bracket bracket-far pipes")
    v.replace("connection-block", box([.11,.036,.1],[-.055,-.05,-.44]))
    v.remove("th-pipes","thermal")
    v.remove("joint-nipple-left joint-nipple-right","structure")
    v.replace("mass-radiator", box([.45,.06,.8],[.3,-.05,0]), "structure")
    v.note("A single flat panel shows the full heated face. The stepped return and its tails read as one connection below the valve side. The thermal bands and the four wall contacts retain their positions.", "Eine ebene Platte zeigt die gesamte Heizfläche. Rücklaufblock und Anschlussrohre bilden einen Körper unter der Ventilseite. Temperaturbänder und vier Wandauflager behalten ihre Positionen.")


def humidifier(v):
    v.remove("trough-walls bulkhead end-frame-outlet")
    v.part("end-frame-inlet")["repeat"] = {"count":2,"translation":[.47,0,0]}
    v.replace("trough-floor", box([.24,.06,.398],[.12,.03,0]))
    v.remove("th-end-frame-outlet th-trough-walls", "thermal")
    v.remove("st-end-frame-outlet", "structure")
    v.reference("st-pad-seat","pad","structure")
    v.replace("mass-water",box([.34,.032,.3],[.22,.02,0]),"structure")
    v.part("th-outlet-air","thermal")["profile"] = [[-.01, .15],[0,.15]]
    v.note("The two open duct mouths share one frame section. The distributor is a single tray above the absorbent pad, with the sump below. The thermal view retains the wet sheet and the discharge face.", "Beide offenen Kanalanschlüsse zeigen denselben Rahmenquerschnitt. Über der Verdunstermatte liegt die zusammengefasste Verteilwanne, darunter der Sumpf. Die Thermik zeigt Nassfläche und Ausblasquerschnitt.")


def fincoil(v):
    v.remove("casing-top end-plate-left end-plate-right")
    v.replace("casing-bottom", extrude("zx", [[-.23944,0],[.23,0],[.23,.52],[-.23944,.52]], -.05,.05,
                                        [[[-.2,.03],[-.2,.49],[.2,.49],[.2,.03]]]))
    v.reference("member-casing-bottom","casing-bottom","structure")
    v.remove("member-casing-top member-end-plate-left member-end-plate-right","structure")
    v.reference("member-fin-plates","coil-block","structure")
    v.remove("th-casing-top th-end-plate-left th-end-plate-right","thermal")
    v.replace("th-tube-bank",box([.044,.46,.4],[0,0,0]),"thermal")
    v.part("th-tube-bank","thermal")["transform"]=copy.deepcopy(v.part("coil-block")["transform"])
    v.part("member-tube-bars","structure")["repeat"] = {"count":2,"translation":[.36,0,0]}
    v.note("Four perimeter strips become a folded rectangular frame around the fin face. The collector and both projecting connections retain their separate positions. The two outer tube courses locate the simplified load path.", "Vier Randbleche bilden einen umlaufenden Rahmen um die Lamellenfläche. Sammler und beide vorstehenden Anschlüsse bleiben getrennt. Die beiden äußeren Rohrreihen zeigen den vereinfachten Lastabtrag.")


def platecoil(v):
    for end in ("rear", "front"):
        part = v.part(f"casing-end-{end}")
        part["holes"] = [loop[::6] for loop in part["holes"]]
    v.remove("base-rail-front")
    v.part("base-rail-rear")["repeat"] = {"count":2, "translation":[.38,0,0]}
    v.remove("th-base-rail-front", "thermal")
    v.remove("st-base-rail-front st-collar-lower-left st-collar-lower-right", "structure")
    v.reference("st-core-corner-bar","plate-pack","structure")
    v.replace("mass-plates",box([.22,.25,.34],[.225,0,0]),"structure")
    v.part("th-supply-air-outlet","thermal")["profile"] = [[-.006,.099],[0,.099]]
    v.note("Four octagonal duct openings retain the positions of both air circuits. The two base runners share a section. The diamond plate core, bypass flap and port collars remain separate from the cabinet.", "Vier achteckige Kanalöffnungen zeigen die Positionen beider Luftkreise. Die beiden Fußschienen haben einen gemeinsamen Querschnitt. Rautenförmiger Plattenblock, Bypassklappe und Stutzen bleiben vom Gehäuse getrennt.")


def valve200(v):
    v.part("capillary")["sections"] = [v.part("capillary")["sections"][i] for i in [0,3,6,9,10]]
    station(v.part("inlet"), [0,1,2,4,6])
    station(v.part("head"), [0,1,3,4,6,7])
    v.replace("joint-body", box([.018,.018,.018],[0,0,0]),"structure")
    v.note("A five-station sensing line leaves the diaphragm and meets the bulb. The nut and cap share a tapered transition with the valve stem. The three pipe connections retain their bearing points.", "Eine Fühlerleitung aus fünf Querschnitten verbindet Membran und Fühler. Überwurf und Kappe gehen konisch in den Ventilschaft über. Die drei Rohranschlüsse behalten ihre Lagerpunkte.")


def valve500(v):
    v.part("capillary")["sections"] = [v.part("capillary")["sections"][i] for i in [0,2,4]]
    station(v.part("bulb"),[0,2,3,5])
    v.part("inlet")["profile"]=[[.022,0],[.022,.0075],[.07,.0105],[.07,0]]
    v.part("capillary-coil")["profile"]=[[.017,.03],[.035,.03]]
    v.remove("stem")
    v.remove("joint-body", "structure")
    v.part("member-column","structure")["profile"]=[[.022,.0075],[.07,.0105],[.108,.009]]
    v.replace("mass-valve",box([.015,.015,.015],[0,-.003,0]),"structure")
    v.note("The brass valve reads as a tee under the diaphragm. The capillary coil is one short drum with a three-station tail to the sensing bulb. The suction pipe stays parallel to the bulb.", "Der Messingkörper bildet ein T-Stück unter der Membran. Die Kapillarwicklung ist eine kurze Trommel mit dreiteiligem Übergang zum Fühler. Die Saugleitung verläuft parallel zum Fühler.")


def compressor(v):
    v.replace("feet",box([.22,.0295,.22],[0,.01475,0]))
    station(v.part("top-cap"),[0,1,2,4,6])
    v.remove("window")
    plume=v.part("th-dome-plume-outlet","thermal")
    station(plume, [0,3,6,7,10,13,14])
    v.part("foot-frame","structure")["holes"] = [v.part("foot-frame","structure")["holes"][0][::4]]
    v.remove("crankshaft","structure")
    v.note("The hermetic vessel has a cylindrical barrel and a two-slope crown. Separate suction and discharge tubes retain their directions. A square mounting footprint replaces the four small lugs.", "Die hermetische Kapsel besteht aus Zylindermantel und zweifach geneigtem Deckel. Saug- und Druckstutzen behalten ihre Austrittsrichtungen. Eine quadratische Standfläche fasst die vier kleinen Füße zusammen.")


def desiccant(v):
    for part, vertices in (("foot-left",[0,3,6,7,8]),("foot-right",[0,1,2,5,8]),("belt-guard",[0,6,13,21,22,23])):
        station(v.part(part),vertices)
    for side in ("ahead","behind","top","bottom"):
        part=v.part("th-band-"+side,"thermal")
        n=len(part["profile"])//2
        station(part, [0,n//4,n//2,3*n//4,n-1,n,n+n//4,n+n//2,n+3*n//4,2*n-1])
    v.part("st-spoke","structure")["repeat"]={"count":2,"rotationAxis":[0,0,1],"rotationRad":3.141592654,"origin":[0,0,0]}
    v.remove("st-sector-frame-far","structure")
    v.note("The desiccant rotor keeps its open annular housing and the partition between the two air streams. Three chords locate the belt cover beside the motor, and the two support feet follow the lower arc.", "Der Sorptionsrotor behält das ringförmige Gehäuse und die Trennung beider Luftströme. Drei Sehnen beschreiben den Riemenschutz neben dem Motor. Die beiden Füße folgen dem unteren Kreisbogen.")


def lift(v):
    v.part("tray")["size"]=[.94,.325,.165]
    v.part("tray")["transform"]["translation"]=[0,.2225,.3225]
    v.part("tray")["repeat"]["translation"]=[0,0,-.645]
    v.remove("battery-cover pump-cover controller-cover ladder-stile axle-f axle-r")
    for stage in range(1,5):
        v.remove(f"tube-b{stage}-root tube-b{stage}-tip")
    v.remove("th-ladder-stile th-axle-f th-axle-r","thermal")
    v.remove("frame-cross-2 member-deck-end-front","structure")
    v.part("frame-cross-1","structure")["repeat"]={"count":2,"translation":[1.6,0,0]}
    v.note("The closed power compartments become two flush side pods. Each moving scissor stage keeps its crossed arms and centre pin. The extension and end gates continue to travel with their own platform frames.", "Die geschlossenen Aggregatefächer bilden zwei bündige Seitenkästen. Jede bewegliche Scherenstufe behält ihre gekreuzten Arme und den Mittelbolzen. Ausschub und Endgeländer bleiben ihren Plattformrahmen zugeordnet.")


def bicycle(v):
    for suffix in ("f","r"):
        wheel=v.part("wheel-"+suffix)
        wheel["profile"]=[[-.0185,.297],[.0185,.297],[.0185,.335],[0,.345],[-.0185,.335],[-.0185,.297]]
        v.part("spoke-"+suffix)["repeat"].update(count=4,rotationRad=1.570796327)
    v.part("loop-tube")["sections"]=[v.part("loop-tube")["sections"][i] for i in [0,1,3,4]]
    v.part("saddle")["sections"]=[v.part("saddle")["sections"][i] for i in [0,2,3]]
    v.remove("rack-leg-r crank-axle")
    v.remove("handlebar-l-1 handlebar-l-2 handlebar-l-3 handlebar-l-4 handlebar-l-5 handlebar-r-1 handlebar-r-2 handlebar-r-3 handlebar-r-4 handlebar-r-5","structure")
    for suffix in ("l","r"):
        v.value["structure"].append({"id":"swept-bar-"+suffix,"of":"handlebar-"+suffix,"member":"steer","part":"member"})
    v.note("Four spoke directions carry each open wheel. The swept step-through tube and the wide rear of the saddle retain their profiles. The steering member follows the complete handlebar sweep on both sides.", "Vier Speichenrichtungen tragen jedes offene Rad. Der gebogene Tiefeinstieg und das breite Sattelhinterteil behalten ihr Profil. Der Lenkstab folgt dem gesamten Lenkerbogen auf beiden Seiten.")


def scooter(v):
    v.part("deck")["profile"]=[[-.075,.266],[.075,.266],[.06,-.452],[.03,-.452],[.03,-.316],[-.03,-.316],[-.03,-.452],[-.06,-.452]]
    v.part("neck")["sections"]=[v.part("neck")["sections"][i] for i in [0,2]]
    for name in ("wheel-f","wheel-r"):
        v.part(name)["profile"]=[[-.025,.104],[0,.108],[.025,.104]]
    v.part("stem")["profile"]=[[0,.027],[.737,.02]]
    v.reference("deck-girder","deck","structure")
    v.remove("arm-l arm-r","structure")
    v.replace("mass-battery",box([.41,.032,.095],[-.04,.122,0]),"structure")
    hub = v.part("th-hub-f","thermal")
    hub["profile"] = [[-.023,.065],[.023,.065]]
    v.note("The deck has a flat nose and a split tail around the rear tyre. A single inclined neck joins the steering head. The tapered stem, handlebar and two individually rotating wheels remain distinct.", "Das Trittbrett hat eine gerade Vorderkante und ein geteiltes Heck um den Hinterreifen. Ein geneigter Hals verbindet es mit dem Lenkkopf. Konische Stange, Lenker und beide einzeln drehenden Räder bleiben getrennt.")


AUTHORS = {
    "glass-balustrade": [(500, balustrade)], "tilt-window": [(500, window)],
    "panel-radiator": [(500, radiator)], "evaporative-humidifier": [(500, humidifier)],
    "finned-heat-exchanger": [(500, fincoil)], "plate-heat-exchanger": [(500, platecoil)],
    "expansion-valve": [(200, valve200), (500, valve500)], "scroll-compressor": [(500, compressor)],
    "sorption-wheel": [(500, desiccant)], "scissor-lift": [(500, lift)],
    "city-bicycle": [(500, bicycle)], "electric-kick-scooter": [(500, scooter)]
}


def write_scale(path, scale, value):
    text = path.read_text()
    marker = re.compile(r'(?m)^([ ]*)"'+str(scale)+r'": \{\s*"status": "unavailable",\s*"denominator": '+str(scale)+r'\s*\}')
    match = marker.search(text, text.index('"volumes"'))
    if not match:
        existing = json.loads(text)
        current = existing.get("asset", existing)["volumes"][str(scale)]
        if current["status"] == "unavailable":
            token = f'"{scale}": ' + json.dumps(current)
            assert text.count(token) == 1
            path.write_text(text.replace(token, f'"{scale}": ' + json.dumps(value, ensure_ascii=False)))
            return
        if current != value:
            if not set(d["en"] for d in current["decisions"]).intersection(d["en"] for d in value["decisions"]):
                raise ValueError(f"Concurrent amendment of {path} 1:{scale}")
            key = re.search(r'"' + str(scale) + r'":\s*(?=\{)',text[text.index('"volumes"'):])
            start = text.index('"volumes"') + key.end()
            end = start + json.JSONDecoder().raw_decode(text[start:])[1]
            old = text[start:end]
            if "\n" in old:
                indent = re.search(r'\n( *)\}$',old).group(1)
                step = len(re.search(r'\n( +)"',old).group(1))-len(indent)
                new = ("\n"+indent).join(json.dumps(value,ensure_ascii=False,indent=step).splitlines())
            else:
                new = json.dumps(value,ensure_ascii=False)
            path.write_text(text[:start]+new+text[end:])
        return
    indent = match.group(1)
    step = len(re.search(r'\n( +)"status"',match.group()).group(1))-len(indent)
    lines = json.dumps(value, ensure_ascii=False,indent=step).splitlines()
    replacement = indent + f'"{scale}": ' + ("\n"+indent).join(lines)
    path.write_text(text[:match.start()]+replacement+text[match.end():])


if __name__ == "__main__":
    for slug, scales in AUTHORS.items():
        path = next(CATALOG.glob("*/"+slug+"/*.json"))
        category = json.loads(path.read_text())["asset"]["category"]
        source_path = TICKET / "volumes" / category / (slug+".json")
        for scale, author in scales:
            volumes = json.loads(path.read_text())["asset"]["volumes"]
            fine = max(int(k) for k in volumes if int(k)<scale and volumes[k]["status"]=="authored")
            v = Volume(volumes[str(fine)],scale)
            author(v)
            result = v.finish()
            write_scale(path,scale,result)
            write_scale(source_path,scale,result)
            print(slug,scale,*(len(result[k]) for k in LISTS))

