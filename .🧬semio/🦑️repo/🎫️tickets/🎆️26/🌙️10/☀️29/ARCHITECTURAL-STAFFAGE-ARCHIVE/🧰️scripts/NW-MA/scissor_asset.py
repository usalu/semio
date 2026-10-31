"""Assembles and writes scissor-lift."""
from scissor import *

EXPORTS = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"]
OCC_N = 90 * 9.80665
materials = [mat_paint("chassis-paint", "Painted chassis steel (orange)", "Lackierter Fahrgestellstahl (orange)", 0.5), mat_paint("scissor-steel", "Painted scissor arm steel", "Lackierter Scherenarmstahl", 0.5), material("deck-steel", "Anti-slip deck plate", "Rutschhemmendes Bühnenblech", 0.8, 0.6, 7850, 50, 460, "literature", "ashrae-2021"), MAT_GALV, MAT_RUBBER]
materials[3] = material("rail-steel", "Galvanised guard rail tube", "Verzinktes Geländerrohr", 0.3, 0.55, 7850, 50, 460, "literature", "ashrae-2021")
R = region
regions = [
    R("arms", "Scissor arms", "Scherenarme", "surface", "passive", "scissor-steel", 10, 1.7, "cylinder", 0.5, [steel_layer("web", 0.006, "scissor-steel")], 21600, ref="ashrae-2021"),
    R("chassis", "Chassis frame and covers", "Fahrgestellrahmen und Abdeckungen", "surface", "passive", "chassis-paint", 5, 0.9, "vertical-plate", 0.4, [steel_layer("plate", 0.004, "chassis-paint")], 14400, ref="ashrae-2021"),
    R("battery-box", "Battery compartment cover", "Batteriefachabdeckung", "engine", "operating", "chassis-paint", 0.6, 0.45, "vertical-plate", 0.4, [steel_layer("cover", 0.002, "chassis-paint")], 7200, op=("power-draw", 150)),
    R("motor-pump", "Motor and hydraulic pump cover", "Abdeckung von Motor und Hydraulikpumpe", "engine", "operating", "chassis-paint", 0.6, 0.4, "vertical-plate", 0.4, [steel_layer("cover", 0.002, "chassis-paint")], 7200, op=("lift-pump", 400)),
    R("deck", "Platform deck and extension", "Bühnenboden und Verlängerung", "surface", "passive", "deck-steel", 3.4, 1.3, "horizontal-plate-up", 1.0, [steel_layer("plate", 0.004, "deck-steel")], 14400, ref="ashrae-2021"),
    R("rails", "Guard rails and control box", "Geländer und Steuerkasten", "surface", "passive", "rail-steel", 2.4, 0.05, "cylinder", 0.9, [steel_layer("tube", 0.003, "rail-steel")], 10800, ref="ashrae-2021"),
    R("tyres", "Non-marking tyres", "Nichtmarkierende Reifen", "tyre", "passive", "rubber", 0.5, 0.54, "cylinder", 0.5, [layer("rubber", "Solid rubber 20 mm", "Vollgummi 20 mm", "polymer", "rubber", 0.02, 0.13, 37400, None, "literature", "incropera-2011")], 37400, ref="incropera-2011"),
]
RID = [r["id"] for r in regions]
DEF = {"arms": {"exposedFraction": 1, "sunlitFraction": 0.5}, "chassis": {"exposedFraction": 1, "sunlitFraction": 0.3}, "battery-box": {"exposedFraction": 0.6, "sunlitFraction": 0.2}, "motor-pump": {"exposedFraction": 0.6, "sunlitFraction": 0.2},
       "deck": {"exposedFraction": 1, "sunlitFraction": 0.9}, "rails": {"exposedFraction": 1, "sunlitFraction": 0.7}, "tyres": {"exposedFraction": 1, "sunlitFraction": 0.1, "contact": {"fraction": 0.1, "tempC": 30}}}
ops = [opkey("power-draw", "Battery current", "Batteriestrom", [(0, "Off", "Aus"), (0.3, "Standby and lowering", "Bereitschaft und Senken"), (0.6, "Driving", "Fahren"), (1, "Lifting at full load", "Heben mit Volllast")]),
       opkey("lift-pump", "Motor and pump", "Motor und Pumpe", [(0, "Stopped", "Stillstand"), (0.5, "Raising slowly", "Langsames Heben"), (1, "Raising at rated speed", "Heben mit Nenngeschwindigkeit")])]
emitters = [emitter("motor-vent", "vent", "Motor cooling air at the chassis end", "Motorkühlluft am Fahrgestellende", "motor-outlet", 0, 0.1, "lift-pump", 0.03, 25, 0.0),
            {"id": "battery-plume", "kind": "plume", "title": tx("Warm air rising above the battery compartment", "Warme Luft über dem Batteriefach"), "anchor": "battery-top", "directionDeg": 90, "diameterM": 0.3, "region": "battery-box", "source": "authored-estimate"}]
anchors = [anchor("contact-f", "chassis", 0.55, 0.0, "contact", "Front tyre contact", "Vorderreifen-Aufstandspunkt", "contact", 0.08, 0.18, 0.6, 90), anchor("contact-r", "chassis", -0.55, 0.0, "contact", "Rear tyre contact", "Hinterreifen-Aufstandspunkt", "contact", 0.08, 0.18, 0.6, 90),
           anchor("motor-outlet", "chassis", 0.925, 0.3, "emitter", "Motor air outlet", "Motorluftaustritt"), anchor("battery-top", "chassis", -0.4, 0.45, "emitter", "Top of the battery cover", "Oberseite der Batterieabdeckung"),
           anchor("occupant-1", "platform", 0.45, 0.1, "load", "First occupant standing point", "Standpunkt der ersten Person"), anchor("occupant-2", "platform", 1.15, 0.1, "load", "Second occupant standing point", "Standpunkt der zweiten Person"),
           anchor("tool-point", "platform", 0.8, 0.1, "load", "Tool box position", "Position des Werkzeugkastens")]
CONTACTS = ["contact-f", "contact-r"]


def th(op, em, over=None):
    return tbind(RID, DEF, over or {}, {"motor-vent": em, "battery-plume": em}, op)


def loads(f):
    if f <= 0:
        return []
    return [{"id": "occupant-a", "kind": "occupant", "title": tx("First person, 90 kg", "Erste Person, 90 kg"), "anchor": "occupant-1", "fxN": 0, "fyN": round(-OCC_N * f, 1), "source": "authored-estimate"},
            {"id": "occupant-b", "kind": "occupant", "title": tx("Second person, 90 kg", "Zweite Person, 90 kg"), "anchor": "occupant-2", "fxN": 0, "fyN": round(-OCC_N * f, 1), "source": "authored-estimate"},
            {"id": "tool-box", "kind": "equipment", "title": tx("Tool box, 40 kg (rated platform load 230 kg)", "Werkzeugkasten, 40 kg (Nennlast der Bühne 230 kg)"), "anchor": "tool-point", "fxN": 0, "fyN": round(-40 * 9.80665 * f, 1), "source": "authored-estimate"}]


poses = [
    pose("stowed", "Stowed, ready to drive", "Eingefahren, fahrbereit", "Scissors folded to a 0.55 m stack, platform at 1.1 m with its rails at 2.2 m, battery on standby.", "Schere zu einem 0,55 m hohen Paket zusammengefaltet, Bühne bei 1,1 m mit Geländer bei 2,2 m, Batterie in Bereitschaft.", bones_for(TH0), CONTACTS, th({"power-draw": 0, "lift-pump": 0}, {"motor-vent": 0, "battery-plume": 0}), []),
    pose("mid-raise", "Half raised with two occupants", "Halb gehoben mit zwei Personen", "Arm angle 22 degrees: platform at about 3.4 m, motor and pump at half speed under 220 kg.", "Armwinkel 22 Grad: Bühne bei etwa 3,4 m, Motor und Pumpe mit halber Drehzahl unter 220 kg.", bones_for(22.0), CONTACTS, th({"power-draw": 1, "lift-pump": 0.5}, {"motor-vent": 1, "battery-plume": 1}), loads(1)),
    pose("raised-working", "Fully raised, deck extended", "Voll gehoben, Bühne ausgeschoben", "Arm angle 38 degrees: platform at 5.8 m, the deck extended by 0.9 m, two occupants and a tool box on board.", "Armwinkel 38 Grad: Bühne bei 5,8 m, Bühne um 0,9 m ausgeschoben, zwei Personen und ein Werkzeugkasten an Bord.", bones_for(TH_MAX, 0.9), CONTACTS, th({"power-draw": 0.3, "lift-pump": 0}, {"motor-vent": 0, "battery-plume": 1}, {"deck": {"exposedFraction": 1, "sunlitFraction": 1.0}, "rails": {"exposedFraction": 1, "sunlitFraction": 0.9}}), loads(1)),
    pose("lowering", "Lowering, platform half down", "Absenken, Bühne halb unten", "Key pose between raised and stowed: arm angle 22 degrees, deck retracted.", "Schlüsselpose zwischen gehoben und eingefahren: Armwinkel 22 Grad, Bühne eingefahren.", bones_for(22.0), CONTACTS, th({"power-draw": 0.3, "lift-pump": 0}, {"motor-vent": 0, "battery-plume": 1}), loads(1), key=True),
]
steps = [(0, TH0, 0), (3, TH0, 0), (9, TH_MAX, 0), (11, TH_MAX, 0.9), (13, TH_MAX, 0.9), (14.5, TH_MAX, 0), (20, TH0, 0), (24, TH0, 0)]
kl = []
for t, th_, ex in steps:
    pn = "stowed" if th_ < 5 else "raised-working"
    kl.append(key(t, pn, "smooth", bones_for(th_, ex)))
clips = [
    clip("raise-cycle", "Raise, work and lower", "Heben, Arbeiten und Senken", "A 24 s loop: the platform rises to 5.8 m in 6 s, the deck slides out and back, then the stack folds down again; motor, pump and battery follow the lift.",
         "Eine Schleife von 24 s: Die Bühne steigt in 6 s auf 5,8 m, die Verlängerung fährt aus und wieder ein, dann faltet sich das Paket wieder zusammen; Motor, Pumpe und Batterie folgen dem Hub.", 24, 30, True, kl,
         [{"anchor": a, "fromS": 0, "toS": 24} for a in CONTACTS], [phase("raise", 3, 9, "Raising", "Heben"), phase("work", 9, 14.5, "Working with deck out", "Arbeiten mit ausgeschobener Bühne"), phase("lower", 14.5, 20, "Lowering", "Senken")],
         [event(3, "operating", "Lift starts", "Heben beginnt", "lift-pump"), event(11, "motion", "Deck extension slides out", "Bühnenverlängerung fährt aus", "deck-ext"), event(14.5, "motion", "Deck extension retracts", "Bühnenverlängerung fährt ein", "deck-ext")],
         [track("operating:lift-pump", [(0, 0), (3, 0), (4, 1), (8, 1), (9, 0), (14.5, 0), (24, 0)]), track("operating:power-draw", [(0, 0), (3, 0.3), (4, 1), (8, 1), (9, 0.3), (14.5, 0.3), (15, 0.3), (20, 0.3), (24, 0)]), track("emitter:motor-vent", [(0, 0), (3, 1), (9.5, 0)], "step"), track("emitter:battery-plume", [(0, 0), (3, 1), (20.5, 0)], "step"), track("load:occupant-a", [(0, 1)], "linear"), track("load:occupant-b", [(0, 1)], "linear"), track("load:tool-box", [(0, 1)], "linear")]),
    clip("start-lift-stop", "Start, lift and shut down", "Anlassen, Heben und Abstellen", "A 22 s sequence: key on at 1 s, two occupants step on, the platform rises, the deck extends, then everything lowers and the key is turned off at 21 s.",
         "Eine Folge von 22 s: Schlüssel an bei 1 s, zwei Personen steigen auf, die Bühne steigt, die Verlängerung fährt aus, dann senkt sich alles und der Schlüssel wird bei 21 s abgeschaltet.", 22, 30, False,
         [key(0, "stowed", "smooth", bones_for(TH0)), key(3, "stowed", "smooth", bones_for(TH0)), key(9, "raised-working", "smooth", bones_for(TH_MAX)), key(11, "raised-working", "smooth", bones_for(TH_MAX, 0.9)), key(14, "raised-working", "smooth", bones_for(TH_MAX, 0.9)), key(15.5, "raised-working", "smooth", bones_for(TH_MAX)), key(21, "stowed", "smooth", bones_for(TH0)), key(22, "stowed", "smooth", bones_for(TH0))],
         [{"anchor": a, "fromS": 0, "toS": 22} for a in CONTACTS], [phase("board", 0, 3, "Key on and boarding", "Zündung und Einsteigen"), phase("lift", 3, 11, "Lifting", "Heben"), phase("work", 11, 15.5, "Working", "Arbeiten"), phase("lower", 15.5, 22, "Lowering and shutdown", "Senken und Abstellen")],
         [event(1.0, "ignition", "Key on", "Schlüssel an", "motor-vent"), event(2.0, "load-on", "Occupants step on the platform", "Personen betreten die Bühne", "occupant-a"), event(21.0, "shutdown", "Key off", "Schlüssel aus", "motor-vent")],
         [track("load:occupant-a", [(0, 0), (1.8, 0), (2.4, 1), (21, 1), (22, 1)]), track("load:occupant-b", [(0, 0), (2.0, 0), (2.6, 1), (21, 1), (22, 1)]), track("load:tool-box", [(0, 0), (2.2, 0), (2.8, 1), (21, 1), (22, 1)]),
          track("operating:power-draw", [(0, 0), (1, 0.3), (3, 0.3), (4, 1), (8, 1), (9, 0.3), (15.5, 0.3), (21, 0.3), (21.5, 0)]), track("operating:lift-pump", [(0, 0), (3, 0), (4, 1), (8, 1), (9, 0), (22, 0)]),
          track("emitter:motor-vent", [(0, 0), (3, 1), (9.5, 0)], "step"), track("emitter:battery-plume", [(0, 0), (1, 1), (21.5, 0)], "step")]),
]


def build_reps():
    r = {}
    nb = tx
    r["500"] = unavailable(500, nb("Seen from the side the stowed scissor stack is five stages of 0.11 m (0.22 mm at 1:500) that merge into one block, while the raised scissors need ten crossing arms of 0.12 m (0.24 mm) and a guard rail of 0.04 to 0.05 m tubes (0.1 mm); at 1:500 neither state can be drawn as the same articulated rig without arms closer than the 0.15 mm gap limit.", "Von der Seite ist das eingefahrene Scherenpaket fünf Stufen von je 0,11 m (0,22 mm bei 1:500), die zu einem Block verschmelzen, während die gehobene Schere zehn kreuzende Arme von 0,12 m (0,24 mm) und ein Geländer aus Rohren von 0,04 bis 0,05 m (0,1 mm) braucht; bei 1:500 lässt sich keiner der beiden Zustände mit demselben gegliederten Rig zeichnen, ohne dass Arme näher als die Grenzlücke von 0,15 mm liegen."),
                      nb("Scissor stage pitch of the stowed stack and the 0.05 m guard rail tubes", "Stufenabstand des eingefahrenen Scherenpakets und die 0,05-m-Geländerrohre"), 0.11,
                      [{"kind": "scale", "denominator": 200}, {"kind": "symbol", "symbol": "site-plan-footprint"}])
    r["200"] = rep(200, 150, 350, pens(0.18, 0.13, None, 0.13, 0.25), [], shapes(200), [
        nb("Ten scissor arms as 0.12 m bars (0.6 mm) that stack to a block when stowed and cross when raised, a chassis block with two cover fields, two tyres, the deck with its extension and a rail frame of top rail, mid rail, toe board and two posts.", "Zehn Scherenarme als Leisten von 0,12 m (0,6 mm), die sich eingefahren zum Block stapeln und gehoben kreuzen, ein Fahrgestellblock mit zwei Abdeckfeldern, zwei Reifen, der Boden mit Verlängerung und ein Geländerrahmen aus Handlauf, Knieleiste, Fußleiste und zwei Pfosten."),
        nb("All ten arms individually (they are the moving mechanism), battery and pump covers, the rail frame windows.", "Alle zehn Arme einzeln (sie sind der Bewegungsmechanismus), Batterie- und Pumpenabdeckung, die Fenster des Geländerrahmens."),
        nb("The extension deck hides under the deck when retracted; no pins or rollers are drawn.", "Die Verlängerung verschwindet eingefahren unter dem Boden; Bolzen und Rollen sind nicht gezeichnet."),
        nb("Pins, rollers, mesh infill, hydraulic cylinders, hubs.", "Bolzen, Rollen, Gitterfüllung, Hydraulikzylinder, Naben."),
        nb("Rails 0.07 m (0.35 mm), arms 0.12 m (0.6 mm): at real size.", "Geländer 0,07 m (0,35 mm), Arme 0,12 m (0,6 mm): in Realgröße."),
        nb("None.", "Keine."),
        nb("Arms dark, chassis mid-tone, deck mid-dark, rails dark; arms at 0.18 mm outline.", "Arme dunkel, Fahrgestell mittlerer Ton, Boden mitteldunkel, Geländer dunkel; Arme mit 0,18-mm-Kontur."),
        nb("Every region has its own form.", "Jede Region hat eine eigene Form."),
    ])
    r["100"] = rep(100, 75, 150, pens(0.25, 0.18, None, 0.13, 0.35), [], shapes(100), [
        nb("As 1:200 with hubs on the tyres, pivot pins on every arm and a second mid rail.", "Wie 1:200 mit Naben an den Reifen, Gelenkbolzen an jedem Arm und einer zweiten Knieleiste."),
        nb("Ten pivot pins at the arm centres, hubs.", "Zehn Gelenkbolzen in den Armmitten, Naben."),
        nb("Rail tubes 0.05 m (0.5 mm) are drawn at real size.", "Geländerrohre von 0,05 m (0,5 mm) sind in Realgröße gezeichnet."),
        nb("End pins, rollers, cylinders.", "Endbolzen, Rollen, Zylinder."),
        nb("Pins 0.09 m (0.9 mm).", "Bolzen 0,09 m (0,9 mm)."),
        nb("None.", "Keine."),
        nb("As 1:200, detail pen 0.18 mm.", "Wie 1:200, Detailstift 0,18 mm."),
        nb("As 1:200; hubs and pins carry the tyres and arms regions.", "Wie 1:200; Naben und Bolzen tragen die Reifen- und Armregionen."),
    ])
    r["50"] = rep(50, 20, 75, pens(0.35, 0.25, 0.13, 0.18, 0.5), [], shapes(50), [
        nb("All 1:100 parts plus a centre post and the control box on the rail frame.", "Alle 1:100-Teile plus ein Mittelpfosten und der Steuerkasten am Geländerrahmen."),
        nb("Pins with 0.11 m diameter, control box, centre post.", "Bolzen mit 0,11 m Durchmesser, Steuerkasten, Mittelpfosten."),
        nb("Nothing merged.", "Nichts zusammengefasst."),
        nb("Rollers, hydraulic cylinders, mesh infill, wheel studs.", "Rollen, Hydraulikzylinder, Gitterfüllung, Radbolzen."),
        nb("Rail tubes 0.05 m (1 mm).", "Geländerrohre 0,05 m (1 mm)."),
        nb("None.", "Keine."),
        nb("Heavier pens 0.35 / 0.25 mm.", "Stärkere Stifte 0,35 / 0,25 mm."),
        nb("As 1:100.", "Wie 1:100."),
    ])
    return r


def symbols():
    sh = [G.rect("footprint", "chassis", "body", 1, -0.925, -0.405, 1.85, 0.81, tone=0.6), G.rect("platform-outline", "chassis", "detail", 2, -0.925, -0.405, 1.85, 0.81, tone=0.3, filled=False), G.poly("extension-outline", "chassis", "detail", 3, [(0.925, -0.405), (1.825, -0.405), (1.825, 0.405), (0.925, 0.405)], closed=True, filled=False)]
    return [{"id": "site-plan-footprint", "title": tx("Site-plan footprint with deck extension", "Baustellenplan-Grundriss mit Bühnenverlängerung"), "description": tx("Plan symbol of the 1.85 x 0.81 m lift with the 0.9 m deck extension outlined.", "Planzeichen der Hubbühne von 1,85 x 0,81 m mit umrissener Bühnenverlängerung von 0,9 m."),
             "semantics": "true-extent", "units": "model-m", "denominators": [500, 200], "pens": pens(0.25, 0.18, None, None, 0.25), "hatches": [], "geometry": {"elevation": [], "plan": sh},
             "dimensionalNote": tx("True extent in model metres: footprint 1.85 x 0.81 m, extension outline 0.9 m beyond the front; real size at both listed scales.", "Wahre Ausdehnung in Modellmetern: Grundriss 1,85 x 0,81 m, Verlängerungsumriss 0,9 m über der Front; in beiden gelisteten Maßstäben in Realgröße.")}]


def asset():
    return {
        "id": "scissor-lift", "category": "machinery", "title": tx("Electric scissor lift, 5.8 m platform height", "Elektro-Scherenarbeitsbühne, 5,8 m Plattformhöhe"),
        "summary": tx("A battery-electric 19 ft scissor lift in side elevation: five scissor stages built from ten arms that fold to a 0.55 m stack or rise to a 5.8 m platform, a sliding deck extension, guard rails, and two occupants with a tool box as loads.", "Eine batterieelektrische 19-ft-Scherenarbeitsbühne in der Seitenansicht: fünf Scherenstufen aus zehn Armen, die sich zu einem 0,55 m hohen Paket falten oder zu einer 5,8 m hohen Plattform heben, eine ausschiebbare Bühnenverlängerung, Geländer sowie zwei Personen mit Werkzeugkasten als Lasten."),
        "tags": ["machine", "scissor-lift", "mewp", "electric", "battery", "access-platform", "construction"],
        "provenance": {"author": "Semio staffage content worker NW-MA", "method": "handcrafted", "created": "2026-10-29",
                       "references": references(["en-280", "incropera-2011", "ashrae-2021", "class-data-note"]),
                       "notes": tx("Class data of 19 ft electric scissor lifts (about 1.85 x 0.81 m, 1.75 t, 5.8 m platform height, 230 kg platform load, 1.1 m guard rail per EN 280). The scissor stack is a closed linkage built as a tree: arms A_k hang from the end of B_(k-1) and B_k from the end of A_(k-1), the rolling foot is a prismatic bone and the platform is kept level by the pose offsets. All numbers are authored estimates of that class; 1:500 is unavailable.", "Klassendaten von 19-ft-Elektro-Scherenbühnen (etwa 1,85 x 0,81 m, 1,75 t, 5,8 m Plattformhöhe, 230 kg Plattformlast, 1,1 m Geländer nach EN 280). Das Scherenpaket ist ein geschlossenes Gestänge, als Baum aufgebaut: Arme A_k hängen am Ende von B_(k-1) und B_k am Ende von A_(k-1), der Rollenfuß ist ein Schubknochen und die Bühne bleibt durch die Posen-Offsets waagerecht. Alle Zahlen sind Schätzungen dieser Klasse; 1:500 ist nicht verfügbar.")},
        "dimensions": {"heightM": 2.2, "widthM": 1.85, "depthM": 0.81, "massKg": 1750},
        "materials": materials, "rig": rig.doc(), "anchors": anchors, "representations": build_reps(), "symbols": symbols(),
        "poses": poses, "clips": clips,
        "thermal": {"environment": DEFAULT_ENV, "passengers": [], "operating": ops, "regions": regions, "emitters": emitters,
                    "assumptions": [tx("24 V traction battery, 225 Ah (5.4 kWh): at full lift current (about 150 A) the internal resistance of 15 mOhm dissipates 340 W in the pack, of which 150 W leave through the visible cover; the motor and pump (2.2 kW, efficiency 82 percent) heat their cover with 400 W and a cooling fan of 0.03 m3/s carries away 25 K at full lift. Authored estimates.", "Traktionsbatterie 24 V, 225 Ah (5,4 kWh): Beim vollen Hubstrom (etwa 150 A) verbrennt der Innenwiderstand von 15 mOhm 340 W im Block, davon verlassen 150 W die sichtbare Abdeckung; Motor und Pumpe (2,2 kW, Wirkungsgrad 82 Prozent) heizen ihre Abdeckung mit 400 W und ein Lüfter mit 0,03 m3/s führt beim vollen Hub 25 K ab. Schätzungen."),
                                    tx("There is no exhaust: the lift is battery-electric. The plume emitter marks the warm air above the battery compartment.", "Es gibt kein Abgas: Die Bühne ist batterieelektrisch. Der Plume-Emitter markiert die warme Luft über dem Batteriefach.")]},
        "structure": structure("Two tyre contacts carry the lift as unilateral supports; the occupants (2 x 90 kg) and the 40 kg tool box are point loads at their standing points; quasi-static. The tipping line about the front axle shows with the deck extended.", "Zwei Reifenkontakte tragen die Bühne als einseitige Auflager; die Personen (2 x 90 kg) und der 40-kg-Werkzeugkasten sind Einzellasten an ihren Standpunkten; quasi-statisch. Die Kipplinie um die Vorderachse zeigt sich bei ausgeschobener Bühne."),
        "presentation": {"defaultPose": "stowed", "defaultClip": "raise-cycle", "defaultScale": 100, "viewpoints": ["elevation"]},
        "exports": EXPORTS, "completion": "authored",
    }


if __name__ == "__main__":
    a = asset()
    for p in selfcheck(a):
        print("PROBLEM", p)
    for d in (200, 100, 50):
        print(d, [round(v, 3) for v in bbox(a, rig, d, poses[0]["bones"])])
    print(write(a, "scissor-lift"))
