from hbuild import *
import cyclist_geo

AID = 'cyclist'
c = Ctx(AID)
v1 = [dict(s) for s in c.a['geometry']['elevation']]
geo = cyclist_geo.build(c)
HAT = hatch
pens = {"500": {"body": .13, "detail": None, "texture": None, "glazing": None, "ground": .13},
        "200": {"body": .18, "detail": .13, "texture": .1, "glazing": None, "ground": .18},
        "100": {"body": .25, "detail": .15, "texture": .1, "glazing": None, "ground": .25},
        "50": {"body": .35, "detail": .25, "texture": .1, "glazing": .13, "ground": .35}}
s50 = []
for s in v1:
    s = {k: v for k, v in s.items() if k != "lod"}
    if s["id"] == "jacket": s["hatch"] = "softshell"
    s50.append(s)
D_ = lambda **k: {kk: tx(*vv) for kk, vv in k.items()}
reps = {
 "500": dict(shapes=geo["500"], pens=pens["500"], hatches=[], range=(350, 1000), decisions=D_(
    silhouette=("The bicycle is reduced to line work: two wheel circles, two frame triangles, fork with handlebar and a saddle tick; the rider is a leaning mass (helmet head, jacket with near arm, trousers, shoes) that overlaps the frame. Wheels and frame are drawn exactly at real extents (1.74 m long).", "Das Fahrrad ist auf Linien reduziert: zwei Radkreise, zwei Rahmendreiecke, Gabel mit Lenker und ein Sattelstrich; die Fahrerin ist eine Neigungsmasse (Helmkopf, Jacke mit nahem Arm, Hose, Schuhe), die den Rahmen überlappt. Räder und Rahmen sind in echter Ausdehnung gezeichnet (1,74 m lang)."),
    retained=("Both tyres, frame triangles, fork, handlebar, saddle, rider head, torso, near arm, both legs (they move with the cranks) and shoes.", "Beide Reifen, Rahmendreiecke, Gabel, Lenker, Sattel, Kopf, Rumpf, naher Arm, beide Beine (sie folgen den Kurbeln) und Schuhe."),
    merged=("Helmet, face and ponytail are one head piece; jacket, collar and neck one torso piece; hand and glove merge into the forearm piece; rim, tyre and fender are one wheel circle.", "Helm, Gesicht und Pferdeschwanz sind ein Kopfstück; Jacke, Kragen und Hals ein Rumpfstück; Hand und Handschuh verschmelzen mit dem Unterarmstück; Felge, Reifen und Schutzblech sind ein Radkreis."),
    omitted=("Spokes (2 mm), hubs, cranks, pedals, chain, chainring, fenders, headlamp, far arm, all rider details. Wheel rotation is therefore not visible at this scale.", "Speichen (2 mm), Naben, Kurbeln, Pedale, Kette, Kettenblatt, Schutzbleche, Scheinwerfer, ferner Arm, alle Fahrerdetails. Die Raddrehung ist deshalb in diesem Maßstab nicht sichtbar."),
    thinElements=("Frame tubes (30 mm) and tyres (40 mm) are single 0.13 mm lines (65 mm real, 1.6 to 2 times thickened) placed on the real tube centre lines; the line weight stands for the section, no double outline.", "Rahmenrohre (30 mm) und Reifen (40 mm) sind einzelne 0,13-mm-Linien (65 mm real, 1,6- bis 2-fach verbreitert) auf der realen Rohrachse; die Strichstärke steht für den Querschnitt, keine Doppelkontur."),
    hatching=("None.", "Keine."),
    styles=("All bicycle parts are open strokes of one pen; the rider is a closed mass in solid view (jacket mid grey, trousers dark, shoes black).", "Alle Fahrradteile sind offene Striche eines Stifts; die Fahrerin ist in der Flächenansicht eine geschlossene Masse (Jacke mittelgrau, Hose dunkel, Schuhe schwarz)."),
    overlays=("Thermal colours map to the rider pieces; the head piece merges face, hair and helmet, the forearm piece merges jacket and gloves; tyres, frame and saddle are coloured as strokes.", "Wärmefarben liegen auf den Fahrerstücken; das Kopfstück mischt Gesicht, Haar und Helm, das Unterarmstück Jacke und Handschuhe; Reifen, Rahmen und Sattel werden als Striche gefärbt."))),
 "200": dict(shapes=geo["200"], pens=pens["200"], hatches=[], range=(150, 350), decisions=D_(
    silhouette=("Wheels as single tyre circles (0.18 mm equals the 36 mm tyre), frame as individual tubes, crank, chainring and pedals; rider with separated head, hair, helmet, jacket, trousers, arm and leg segments and shoes.", "Räder als einzelne Reifenkreise (0,18 mm entspricht dem 36-mm-Reifen), Rahmen als einzelne Rohre, Kurbel, Kettenblatt und Pedale; Fahrerin mit getrennten Teilen für Kopf, Haar, Helm, Jacke, Hose, Arm- und Beinsegmente sowie Schuhe."),
    retained=("Hubs as dots, two spoke diameters per wheel as a rotation cue, cranks of both sides, reflective band line on the jacket.", "Naben als Punkte, zwei Speichendurchmesser je Rad als Drehhinweis, Kurbeln beider Seiten, Reflexstreifen als Linie auf der Jacke."),
    merged=("Hand and glove with the forearm; rim into the tyre circle (gap 0.09 mm would not print).", "Hand und Handschuh mit dem Unterarm; Felge im Reifenkreis (0,09 mm Abstand wären nicht druckbar)."),
    omitted=("Fenders (0.05 mm gap to the tyre), chain, headlamp, face features, fingers, zip, vent, 32 of 36 spokes.", "Schutzbleche (0,05 mm Abstand zum Reifen), Kette, Scheinwerfer, Gesichtszüge, Finger, Reißverschluss, Lüftung, 32 von 36 Speichen."),
    thinElements=("Seat, top and down tube 0.18 mm (36 mm real, 1.1 times); chain and seat stays, fork, handlebar 0.13 mm (26 mm real, 1.3 to 1.6 times); the four spokes are 0.1 mm texture strokes standing for the spoke set.", "Sitz-, Ober- und Unterrohr 0,18 mm (36 mm real, 1,1-fach); Kettenstreben, Sitzstreben, Gabel, Lenker 0,13 mm (26 mm real, 1,3- bis 1,6-fach); die vier Speichen sind 0,1-mm-Texturstriche für den Speichensatz."),
    hatching=("None.", "Keine."),
    styles=("Outline pen 0.18 mm, detail 0.13 mm, spokes 0.1 mm; tyres are lines in every view, rider masses carry solid tones.", "Konturstift 0,18 mm, Detail 0,13 mm, Speichen 0,1 mm; Reifen sind in jeder Ansicht Linien, Fahrermassen tragen Flächentöne."),
    overlays=("Rider regions have own pieces except the forearm pieces (jacket plus gloves); bicycle metal, tyres and saddle are separate strokes.", "Die Regionen der Fahrerin haben eigene Stücke außer den Unterarmstücken (Jacke plus Handschuhe); Fahrradmetall, Reifen und Sattel sind getrennte Striche."))),
 "100": dict(shapes=geo["100"], pens=pens["100"], hatches=[HAT("softshell", "Softshell fabric", "Softshellgewebe", "lines", 30, 1.2, 0.07), HAT("far-limb", "Far side limb", "Abgewandtes Glied", "lines", 60, 0.6, 0.07)], range=(75, 150), decisions=D_(
    silhouette=("Tyres as black bands (0.4 mm = 40 mm), fenders as arcs at a 0.45 mm gap, frame tubes as strokes whose pen equals the real tube diameter, chain, chainring, cranks and pedals; rider with ponytail, helmet with vent, jacket with zip and reflective band, mitten gloves with thumb, trouser clips, sneakers with soles.", "Reifen als schwarze Bänder (0,4 mm = 40 mm), Schutzbleche als Bögen mit 0,45 mm Abstand, Rahmenrohre als Striche, deren Stiftbreite dem realen Rohrdurchmesser entspricht, Kette, Kettenblatt, Kurbeln und Pedale; Fahrerin mit Pferdeschwanz, Helm mit Lüftung, Jacke mit Reißverschluss und Reflexstreifen, Fäustlingshandschuhen mit Daumen, Hosenklammern, Sneakern mit Sohlen."),
    retained=("Headlamp, saddle outline, hub, 12 spokes per wheel (every third of the 36 real spokes) as 0.1 mm lines, handlebar and grip.", "Scheinwerfer, Sattelumriss, Nabe, 12 Speichen je Rad (jede dritte der 36 realen) als 0,1-mm-Linien, Lenker und Griff."),
    merged=("Rim into the tyre band; handlebar tube and stem keep separate strokes.", "Felge im Reifenband; Lenkerrohr und Vorbau bleiben getrennte Striche."),
    omitted=("Eye, ear, finger separations, spoke nipples, valve, bell, lock, laces.", "Auge, Ohr, Fingertrennungen, Speichennippel, Ventil, Klingel, Schloss, Schnürsenkel."),
    thinElements=("Spokes are drawn as 0.1 mm lines at 30 degree pitch (2 mm real, thickened 5 times, spacing 1.5 mm at the rim); chain 0.1 mm (10 mm), stays 0.2 mm (20 mm): no outline pairs are used where the gap would be below 0.1 mm.", "Speichen sind 0,1-mm-Linien im 30-Grad-Raster (2 mm real, 5-fach verbreitert, 1,5 mm Abstand an der Felge); Kette 0,1 mm (10 mm), Streben 0,2 mm (20 mm): wo der Abstand unter 0,1 mm läge, werden keine Konturpaare verwendet."),
    hatching=("Softshell: 1.2 mm lines at 30 degrees on the jacket; far-side limbs 0.6 mm lines at 60 degrees.", "Softshell: 1,2-mm-Linien unter 30 Grad auf der Jacke; abgewandte Glieder 0,6-mm-Linien unter 60 Grad."),
    styles=("Body 0.25, detail 0.15, texture 0.1 mm; tyres filled near black, helmet dark, jacket mid tone.", "Körper 0,25, Detail 0,15, Textur 0,1 mm; Reifen fast schwarz gefüllt, Helm dunkel, Jacke mittlerer Ton."),
    overlays=("Every region has its own shape: face, hair, helmet, gloves, jacket, trousers, shoes, bicycle metal, tyres, saddle.", "Jede Region hat eine eigene Form: Gesicht, Haar, Helm, Handschuhe, Jacke, Hose, Schuhe, Fahrradmetall, Reifen, Sattel."))),
 "50": dict(shapes=s50, pens=pens["50"], hatches=[HAT("softshell", "Softshell fabric", "Softshellgewebe", "lines", 30, 1.0, 0.06)], range=(20, 75), decisions=D_(
    silhouette=("Full articulation from the original 1:50 study: rider with profile face, ponytail, helmet vent and strap, jacket with zip and fold, fingered gloves with thumb, trouser clips, sneakers; bicycle with 18 spoke diameters per wheel, rims, hubs, cog, fenders, tubes, chain, crank, pedals and headlamp.", "Volle Gliederung nach der ursprünglichen 1:50-Studie: Fahrerin mit Gesicht im Profil, Pferdeschwanz, Helmlüftung und -riemen, Jacke mit Reißverschluss und Falte, Fingerhandschuhe mit Daumen, Hosenklammern, Sneaker; Fahrrad mit 18 Speichendurchmessern je Rad, Felgen, Naben, Ritzel, Schutzblechen, Rohren, Kette, Kurbel, Pedalen und Scheinwerfer."),
    retained=("All detail of the original drawing.", "Alle Details der ursprünglichen Zeichnung."),
    merged=("Nothing merged.", "Nichts verschmolzen."),
    omitted=("Spoke nipples, valve, cables, bell.", "Speichennippel, Ventil, Züge, Klingel."),
    thinElements=("Spokes 0.1 mm lines at 10 degree pitch (2 mm real, 1.1 mm spacing at the rim), tubes as capsules of their real diameter (22 to 44 mm = 0.44 to 0.88 mm).", "Speichen 0,1-mm-Linien im 10-Grad-Raster (2 mm real, 1,1 mm Abstand an der Felge), Rohre als Kapseln mit ihrem realen Durchmesser (22 bis 44 mm = 0,44 bis 0,88 mm)."),
    hatching=("Softshell 1.0 mm lines at 30 degrees (pen 0.06) on the jacket.", "Softshell 1,0-mm-Linien unter 30 Grad (Stift 0,06) auf der Jacke."),
    styles=("Body 0.35, detail 0.25, texture 0.1 mm.", "Körper 0,35, Detail 0,25, Textur 0,1 mm."),
    overlays=("One shape per region and bone.", "Eine Form je Region und Knochen."))),
}

MATS = [mat("still-air", "Still air", "Ruhende Luft", 0.9), mat("base-layer", "Synthetic base layer", "Synthetisches Unterhemd", 0.9, 0.6), mat("fleece", "Thin fleece", "Dünnes Fleece", 0.92, 0.7),
        mat("tights", "Stretch tights", "Stretch-Unterhose", 0.9, 0.7), mat("sock", "Sports socks", "Sportsocken", 0.9, 0.6), mat("hair-layer", "Hair and air", "Haar und Luft", 0.98, 0.85),
        mat("eps-foam", "Expanded polystyrene liner", "Polystyrol-Schaum", 0.9, 0.5), mat("glove-lining", "Glove lining", "Handschuhfutter", 0.9, 0.6)]
regions = [
 dict(id="face-skin", en="Face and neck skin", de="Haut von Gesicht und Hals", kind="skin", model="body", material="skin", area=0.098, L=0.2, geom="sphere", svf=0.6, layers=[], seg="head", ms=0.07, sf=0.055, ex=1.0, su=0.5, ref="dubois-1916"),
 dict(id="hair", en="Ponytail and hair at the nape", de="Pferdeschwanz und Haar im Nacken", kind="hair", model="body", material="hair", area=0.027, L=0.15, geom="cylinder", svf=0.5, layers=[layer("hair-pelt", "hair", "hair-layer", 0.04, 300, "Hair and trapped air", "Haar und eingeschlossene Luft", 0.01)], seg="head", ms=0.01, sf=0.015, ex=1.0, su=0.4),
 dict(id="helmet", en="Helmet over the scalp", de="Helm über der Kopfhaut", kind="clothing", model="body", material="eps-helmet", area=0.07, L=0.2, geom="sphere", svf=0.85,
      layers=[layer("hm-hair", "hair", "hair-layer", 0.03, 250, "Hair under the helmet", "Haar unter dem Helm", 0.008), layer("hm-air", "air-gap", "still-air", 0.03, 0, "Air gap in the helmet", "Luftspalt im Helm", 0.012), layer("hm-eps", "insulation", "eps-foam", 0.2, 600, "EPS liner, vented (effective)", "EPS-Schale, belüftet (effektiv)", 0.025), layer("hm-shell", "shell", "eps-helmet", 0.004, 300, "Thin polycarbonate shell", "Dünne Polycarbonat-Schale", 0.001)], seg="head", ms=0.04, sf=0.03, ex=1.0, su=0.65),
 dict(id="gloves", en="Leather cycling gloves", de="Leder-Radhandschuhe", kind="clothing", model="body", material="leather-glove", area=0.098, L=0.05, geom="cylinder", svf=0.4,
      layers=clothed("gl", [("lining", "clothing", "glove-lining", 0.06, 250, "Glove lining", "Handschuhfutter", 0.001), ("leather", "shell", "leather-glove", 0.05, 600, "Leather palm and mesh back", "Lederhandfläche und Netzrücken", 0.001)]), seg="hand", ms=0.02, sf=0.05, ex=1.0, su=0.35),
 dict(id="jacket", en="Base layer, fleece and softshell jacket on trunk and arms", de="Unterhemd, Fleece und Softshelljacke an Rumpf und Armen", kind="clothing", model="body", material="softshell", area=0.921, L=0.3, geom="cylinder", svf=0.5,
      layers=clothed("jk", [("base", "clothing", "base-layer", 0.15, 300, "Long-sleeve base layer", "Langarm-Unterhemd", 0.001), ("fleece", "clothing", "fleece", 0.2, 500, "Thin fleece", "Dünnes Fleece", 0.004), ("softshell", "shell", "softshell", 0.12, 600, "Softshell jacket", "Softshelljacke", 0.002)]), seg="trunk", ms=0.39, sf=0.45, ex=0.92, su=0.42),
 dict(id="trousers", en="Tights and stretch trousers on thighs and shanks", de="Unterhose und Stretchhose an Ober- und Unterschenkeln", kind="clothing", model="body", material="stretch-trouser", area=0.64, L=0.14, geom="cylinder", svf=0.45,
      layers=clothed("tr", [("tights", "clothing", "tights", 0.1, 250, "Tights", "Stretch-Unterhose", 0.001), ("stretch", "clothing", "stretch-trouser", 0.15, 450, "Stretch trousers", "Stretchhose", 0.002)]), seg="leg", ms=0.44, sf=0.33, ex=0.9, su=0.42),
 dict(id="shoes", en="Socks and canvas sneakers", de="Socken und Stoffsneaker", kind="clothing", model="body", material="canvas-sneaker", area=0.125, L=0.1, geom="cylinder", svf=0.25,
      layers=[layer("sh-sock", "clothing", "sock", 0.012, 300, "Sports socks", "Sportsocken", 0.002), layer("sh-air", "air-gap", "still-air", 0.01, 0, "Air in the shoe", "Luft im Schuh", 0.004), layer("sh-canvas", "shell", "canvas-sneaker", 0.02, 900, "Canvas upper and rubber sole", "Stoffschaft und Gummisohle", 0.004)], seg="foot", ms=0.03, sf=0.07, ex=1.0, su=0.3),
 dict(id="bike-metal", en="Frame, rims and drivetrain", de="Rahmen, Felgen und Antrieb", kind="surface", model="passive", material="steel", area=0.9, L=0.03, geom="cylinder", svf=0.5, layers=[], cap=2500, ex=1.0, su=0.5, ref="ashrae-2021"),
 dict(id="tyres", en="Tyres", de="Reifen", kind="tyre", model="passive", material="rubber", area=0.6, L=0.04, geom="cylinder", svf=0.4, layers=[], cap=3000, ex=1.0, su=0.4, ref="ashrae-2021"),
 dict(id="saddle", en="Saddle", de="Sattel", kind="surface", model="passive", material="leather-saddle", area=0.07, L=0.15, geom="horizontal-plate-up", svf=0.9, layers=[], cap=2000, ex=1.0, su=0.5),
]
for r in regions: r.setdefault("ref", "iso-9920-2007")
env = {"airTempC": 14, "meanRadiantTempC": 15, "skyTempC": 8, "groundTempC": 15, "relativeHumidity": 0.65, "windMS": 4.3, "windDirectionDeg": 180, "solarWM2": 200, "diffuseFraction": 0.6, "pressurePa": 101325}
body = {"species": "human", "massKg": 66, "skinAreaM2": 1.78, "basalMetabolismW": 68, "metWM2": 58.15, "coreSetPointC": 36.8, "skinSetPointC": 34.1, "sweating": True, "panting": False, "maxSkinBloodFlowLM2H": 90, "minSkinBloodFlowLM2H": 6.3, "source": "authored-estimate", "reference": "gagge-1971"}
PAT = [("recovery", "Recovery after stopping", "Erholung nach dem Anhalten", 17, 0.78, (0.38, 0.04, 0.45, 0.13), "nose"),
       ("easy", "Easy riding or coasting", "Lockeres Fahren oder Rollen", 18, 0.9, (0.40, 0.02, 0.50, 0.08), "nose"),
       ("riding", "Steady city riding", "Gleichmäßiges Stadtfahren", 24, 1.25, (0.44, 0.0, 0.53, 0.03), "nose-and-mouth"),
       ("hard", "Standing climb effort", "Anstrengung im Wiegetritt", 32, 1.75, (0.46, 0.0, 0.54, 0.0), "nose-and-mouth")]
resp = {"noseEmitter": "breath-nose", "mouthEmitter": "breath-mouth", "exhaledTempC": 33, "exhaledRelativeHumidity": 0.95, "patterns": [pattern(*p) for p in PAT],
        "motion": [{"bone": "torso", "angleDeg": 1.0, "dy": 0.005}], "source": "authored-estimate", "reference": "gupta-2010"}
RULE = {"riding-upright": dict(met=4, work=62, pat="riding", plume=0.2, wet=0.2, grip=True, saddle=True),
        "standing-on-pedals": dict(met=7, work=150, pat="hard", plume=0.2, wet=0.35, grip=True, saddle=False),
        "coasting": dict(met=2.2, work=0, pat="easy", plume=0.25, wet=0.1, grip=True, saddle=True),
        "dismounted-astride": dict(met=1.8, work=0, pat="recovery", plume=0.7, wet=0.15, grip=True, saddle=False)}
def pose_rule(p):
    r = RULE[p["id"]]; ct = {}
    if r["grip"]: ct["gloves"] = {"fraction": 0.35, "tempC": 14}
    if p["id"] == "dismounted-astride": ct["shoes"] = {"fraction": 0.35, "tempC": 15}
    else: ct["shoes"] = {"fraction": 0.25, "tempC": 14}
    if r["saddle"]: ct["trousers"] = {"fraction": 0.05, "tempC": 15}
    ex = {"jacket": 0.88, "trousers": 0.85} if p["id"] != "dismounted-astride" else {}
    return {"met": r["met"], "work": r["work"], "pat": r["pat"], "contact": ct, "wet": {"jacket": r["wet"], "trousers": r["wet"]*0.7, "face-skin": r["wet"]*0.8}, "ex": ex, "em": {"plume": r["plume"]}}
spec = dict(materials=MATS, refs=[GAGGE, GUPTA], new_anchors=[dict(id="nose", bone="head", wx=0.623, wy=1.600, en="Nostrils", de="Nasenlöcher"), dict(id="crown", bone="head", wx=0.558, wy=1.79, en="Crown of the head (helmet top)", de="Scheitel (Helmoberseite)")],
    env=env, body=body, respiration=resp, emitters=std_emitters(), regions=regions, baseMet=1.2, emit_default={"plume": 0.3}, pose_rule=pose_rule,
    clip_tracks={"pedal-cycle": [{"target": "externalWorkW", "ease": "smooth", "keys": [{"t": 0, "v": 58}, {"t": 4, "v": 62}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 2}]}, {"target": "emitter:plume", "ease": "smooth", "keys": [{"t": 0, "v": 0.2}, {"t": 4, "v": 0.2}]}],
                 "brake-and-stop": [{"target": "externalWorkW", "ease": "smooth", "keys": [{"t": 0, "v": 62}, {"t": 0.6, "v": 0}, {"t": 1.6, "v": 0}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 2}, {"t": 0.8, "v": 1}, {"t": 1.4, "v": 0}]}, {"target": "emitter:plume", "ease": "smooth", "keys": [{"t": 0, "v": 0.2}, {"t": 1.6, "v": 0.6}]}],
                 "dismount": [{"target": "externalWorkW", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 1, "v": 150}, {"t": 1.5, "v": 0}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 1, "v": 3}, {"t": 1.5, "v": 0}]}, {"target": "emitter:plume", "ease": "smooth", "keys": [{"t": 0, "v": 0.25}, {"t": 2.4, "v": 0.7}]}]},
    assumptions=[
     tx("Body regions sum to the DuBois area of 1.78 m2 (66 kg, 172 cm); clothed regions carry the area factor 1 + 0.28 clo. The bicycle surfaces (0.9 m2 frame and drivetrain, 0.6 m2 tyres, 0.07 m2 saddle) are additional passive areas.", "Die Körperregionen ergeben die DuBois-Fläche von 1,78 m2 (66 kg, 172 cm); bekleidete Regionen tragen den Flächenfaktor 1 + 0,28 clo. Die Fahrradflächen (0,9 m2 Rahmen und Antrieb, 0,6 m2 Reifen, 0,07 m2 Sattel) sind zusätzliche passive Flächen."),
     tx("Clothing is layered per region (base layer, fleece, softshell; tights, stretch trousers; socks, canvas shoe; helmet liner over hair; glove lining and leather) with garment values guided by ISO 9920 (garment sum x 0.835 plus 0.161 clo enclosed air). The vented EPS liner is an effective resistance of 0.2 m2K/W.", "Die Kleidung ist je Region geschichtet (Unterhemd, Fleece, Softshell; Unterhose, Stretchhose; Socken, Stoffschuh; Helmschale über Haar; Handschuhfutter und Leder) mit Werten nach Anhalt von ISO 9920 (Summe x 0,835 plus 0,161 clo eingeschlossene Luft). Die belüftete EPS-Schale ist ein effektiver Widerstand von 0,2 m2K/W."),
     tx("Riding at 15 km/h is 4 met (Compendium class for leisure cycling) and delivers about 62 W at the pedals (rolling resistance 0.008, drag area 0.6 m2, drive train 97 %), standing climbing 7 met and 150 W; the external work leaves the body as mechanical power and is not released as heat.", "Fahren mit 15 km/h entspricht 4 met (Compendium-Klasse für Freizeitradeln) und liefert etwa 62 W an den Pedalen (Rollwiderstand 0,008, Luftwiderstandsfläche 0,6 m2, Antrieb 97 %), Wiegetritt am Berg 7 met und 150 W; die äußere Arbeit verlässt den Körper als mechanische Leistung und wird nicht als Wärme frei."),
     tx("Breathing patterns follow from the metabolic rate (ventilation about 25 to 27 times the oxygen uptake): 13 L/min recovering, 16 easy, 30 steady riding, 56 hard; above about 30 L/min flow goes through nose and mouth (authored estimate).", "Atemmuster folgen aus dem Stoffwechsel (Ventilation etwa das 25- bis 27-Fache der Sauerstoffaufnahme): 13 L/min in der Erholung, 16 locker, 30 gleichmäßig, 56 hart; oberhalb von etwa 30 L/min strömt die Luft durch Nase und Mund (Autorenschätzung)."),
     tx("The relative wind of 4.3 m/s is the riding speed; it blows the thermal plume away (low plume strength while riding).", "Der relative Wind von 4,3 m/s ist die Fahrgeschwindigkeit; er bläst die thermische Fahne weg (geringe Fahnenstärke beim Fahren).")])
doc = finish(c, spec, reps)
doc["asset"]["summary"] = c.a["summary"]
print(write(doc, AID))
