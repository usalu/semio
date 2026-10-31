from hbuild import *
import worker_geo

AID = 'worker-carrying-box'
c = Ctx(AID)
v1 = [dict(s) for s in c.a['geometry']['elevation']]
geo = worker_geo.build(c)
HAT = hatch
pens = {"500": {"body": .13, "detail": None, "texture": None, "glazing": None, "ground": .13},
        "200": {"body": .18, "detail": .13, "texture": None, "glazing": None, "ground": .18},
        "100": {"body": .25, "detail": .15, "texture": .1, "glazing": None, "ground": .25},
        "50": {"body": .35, "detail": .25, "texture": .13, "glazing": .13, "ground": .35}}
s50 = [{k: v for k, v in s.items() if k != "lod"} for s in v1]
D_ = lambda **k: {kk: tx(*vv) for kk, vv in k.items()}
reps = {
 "500": dict(shapes=geo["500"], pens=pens["500"], hatches=[], range=(350, 1000), decisions=D_(
    silhouette=("One upright figure with a box: helmet head, torso mass with the near arm reaching forward to the box, hips, legs with boots, and the 0.4 x 0.3 m box as a closed rectangle; the box in front of the body is the identifying feature.", "Eine aufrechte Figur mit Karton: Helmkopf, Rumpfmasse mit dem nahen, zum Karton greifenden Arm, Hüfte, Beine mit Stiefeln und der 0,4 x 0,3 m große Karton als geschlossenes Rechteck; der Karton vor dem Körper ist das Erkennungsmerkmal."),
    retained=("Helmet with peak, both legs and boots, near arm, box.", "Helm mit Schirm, beide Beine und Stiefel, naher Arm, Karton."),
    merged=("Helmet, hair and face are one head piece; shirt, vest and its bands one torso mass; hand and glove merge with the forearm.", "Helm, Haar und Gesicht sind ein Kopfstück; Hemd, Weste und Streifen eine Rumpfmasse; Hand und Handschuh verschmelzen mit dem Unterarm."),
    omitted=("Far arm (coincides with the near arm), reflective bands (0.25 mm wide, would not separate from the vest), knee pad, pocket, tape, handle, label, face details.", "Ferner Arm (deckt sich mit dem nahen), Reflexstreifen (0,25 mm breit, würden sich nicht von der Weste abheben), Knieschoner, Tasche, Klebeband, Griff, Etikett, Gesichtszüge."),
    thinElements=("No element is thinner than the outline pen; the forearm (0.07 m, 0.14 mm) is kept because it is the load path to the box and overlaps the torso mass.", "Kein Element ist dünner als der Konturstift; der Unterarm (0,07 m, 0,14 mm) bleibt, weil er der Lastpfad zum Karton ist und die Rumpfmasse überlappt."),
    hatching=("None.", "Keine."), styles=("One 0.13 mm outline; solid view: vest light, trousers and boots dark, box mid tone.", "Eine 0,13-mm-Kontur; Flächenansicht: Weste hell, Hose und Stiefel dunkel, Karton mittlerer Ton."),
    overlays=("Thermal colours attach to pieces; the torso piece merges shirt and vest (labelled merged), the head piece face, hair and helmet; the box is its own piece.", "Wärmefarben liegen auf den Stücken; das Rumpfstück mischt Hemd und Weste (als gemischt gekennzeichnet), das Kopfstück Gesicht, Haar und Helm; der Karton ist ein eigenes Stück."))),
 "200": dict(shapes=geo["200"], pens=pens["200"], hatches=[], range=(150, 350), decisions=D_(
    silhouette=("Separated masses: head with hair, helmet, shirt torso, hi-vis vest with two reflective band lines, hips, upper arm and forearm-with-glove per side, thigh and shank per side, boots with toe pieces, box with tape line.", "Getrennte Massen: Kopf mit Haar, Helm, Hemdrumpf, Warnweste mit zwei Reflexstreifen-Linien, Hüfte, Ober- und Unterarm mit Handschuh je Seite, Ober- und Unterschenkel je Seite, Stiefel mit Spitzenstück, Karton mit Klebebandlinie."),
    retained=("Both arms, both legs, helmet peak, vest bands as lines.", "Beide Arme, beide Beine, Helmschirm, Westenstreifen als Linien."),
    merged=("Hair into the head piece; glove into the forearm piece.", "Haar im Kopfstück; Handschuh im Unterarmstück."),
    omitted=("Ear, ridge, strap, knee pad, pocket, fingers, handle, label, hem lines.", "Ohr, Grat, Kinnriemen, Knieschoner, Tasche, Finger, Griff, Etikett, Saumlinien."),
    thinElements=("The reflective bands (50 mm) are 0.13 mm lines (26 mm line weight for 50 mm real: drawn as single strokes across the vest, not as outlines).", "Die Reflexstreifen (50 mm) sind 0,13-mm-Linien (einzelne Striche über der Weste, keine Konturen)."),
    hatching=("None.", "Keine."), styles=("Outlines 0.18 mm, bands and tape 0.13 mm; vest light tone so it separates from the shirt.", "Konturen 0,18 mm, Streifen und Klebeband 0,13 mm; Weste heller Ton, damit sie sich vom Hemd absetzt."),
    overlays=("Every region except the merged hair has its own piece (hair shares the head piece).", "Jede Region außer dem verschmolzenen Haar hat ein eigenes Stück (das Haar teilt das Kopfstück)."))),
 "100": dict(shapes=geo["100"], pens=pens["100"], hatches=[HAT("far-limb", "Far side limb", "Abgewandtes Glied", "lines", 60, 0.6, 0.07)], range=(75, 150), decisions=D_(
    silhouette=("Articulated work clothing: helmet with ridge, hair and ear, shirt torso, vest with two reflective bands (closed strips) and shoulder strap line, sleeves with cuffs, mitten gloves with thumb, trousers with hems, boots with sole lines, box with tape strip and hand hole.", "Gegliederte Arbeitskleidung: Helm mit Grat, Haar und Ohr, Hemdrumpf, Weste mit zwei Reflexstreifen (geschlossene Bänder) und Schulterträgerlinie, Ärmel mit Manschetten, Fäustlingshandschuhe mit Daumen, Hose mit Säumen, Stiefel mit Sohlenlinien, Karton mit Klebestreifen und Griffloch."),
    retained=("Both arms (the far arm hatched), both legs.", "Beide Arme (der ferne schraffiert), beide Beine."),
    merged=("Fingers into one mitten outline.", "Finger in einer Fäustlingskontur."),
    omitted=("Chin strap, eye, finger separations, knee pad, trouser pocket, label, flap lines, laces.", "Kinnriemen, Auge, Fingertrennungen, Knieschoner, Hosentasche, Etikett, Klappenlinien, Schnürsenkel."),
    thinElements=("Reflective bands are 0.4 mm wide closed strips (40 mm real vs 50 mm), cuffs and hems 0.15 mm lines.", "Reflexstreifen sind 0,4 mm breite geschlossene Bänder (40 mm real statt 50 mm), Manschetten und Säume 0,15-mm-Linien."),
    hatching=("Far-side arm and leg: 0.6 mm lines at 60 degrees.", "Abgewandter Arm und abgewandtes Bein: 0,6-mm-Linien unter 60 Grad."),
    styles=("Body 0.25, detail 0.15, texture 0.1 mm; vest light, bands dark, boots black.", "Körper 0,25, Detail 0,15, Textur 0,1 mm; Weste hell, Streifen dunkel, Stiefel schwarz."),
    overlays=("All nine regions are drawn: head skin, hair, helmet, shirt, vest, sleeves, gloves, trousers, boots plus the box.", "Alle neun Regionen sind gezeichnet: Kopfhaut, Haar, Helm, Hemd, Weste, Ärmel, Handschuhe, Hose, Stiefel sowie der Karton."))),
 "50": dict(shapes=s50, pens=pens["50"], hatches=[], range=(20, 75), decisions=D_(
    silhouette=("Full drawing from the original 1:50 study: profile face, helmet with ridge and strap, vest with bands, strap and hem, cuffs, fingered gloves with thumbs, knee pad, pocket, boots with toe caps and sole seams, box with tape, handle, label and flap.", "Volle Zeichnung nach der ursprünglichen 1:50-Studie: Gesicht im Profil, Helm mit Grat und Kinnriemen, Weste mit Streifen, Träger und Saum, Manschetten, Fingerhandschuhe mit Daumen, Knieschoner, Tasche, Stiefel mit Kappen und Sohlennähten, Karton mit Klebeband, Griff, Etikett und Klappe."),
    retained=("All detail of the original drawing.", "Alle Details der ursprünglichen Zeichnung."), merged=("Nothing merged.", "Nichts verschmolzen."),
    omitted=("Laces, stitching of the vest.", "Schnürsenkel, Nähte der Weste."),
    thinElements=("Finger separations 0.13 mm at 6 mm finger pitch; box flap line 0.13 mm.", "Fingertrennungen 0,13 mm bei 6 mm Fingerteilung; Kartonklappenlinie 0,13 mm."),
    hatching=("None.", "Keine."), styles=("Body 0.35, detail 0.25, texture 0.13 mm.", "Körper 0,35, Detail 0,25, Textur 0,13 mm."), overlays=("One shape per region and bone.", "Eine Form je Region und Knochen."))),
}

MATS = [mat("still-air", "Still air", "Ruhende Luft", 0.9), mat("tee-cotton", "Cotton T-shirt", "Baumwoll-T-Shirt", 0.9, 0.5), mat("briefs", "Cotton briefs", "Baumwollunterhose", 0.9, 0.5), mat("work-sock", "Work socks", "Arbeitssocken", 0.9, 0.6),
        mat("hair-layer", "Hair and air", "Haar und Luft", 0.98, 0.85), mat("harness-air", "Helmet air gap", "Helm-Luftspalt", 0.9)]
regions = [
 dict(id="head-skin", en="Face and neck skin", de="Haut von Gesicht und Hals", kind="skin", model="body", material="skin", area=0.111, L=0.2, geom="sphere", svf=0.6, layers=[], seg="head", ms=0.05, sf=0.055, ex=1.0, su=0.0, ref="dubois-1916"),
 dict(id="hair", en="Hair at nape and temples", de="Haar an Nacken und Schläfen", kind="hair", model="body", material="hair", area=0.02, L=0.15, geom="cylinder", svf=0.5, layers=[layer("hair-pelt", "hair", "hair-layer", 0.04, 300, "Hair and trapped air", "Haar und eingeschlossene Luft", 0.01)], seg="head", ms=0.005, sf=0.01, ex=0.9, su=0.0),
 dict(id="helmet", en="Safety helmet over the scalp", de="Schutzhelm über der Kopfhaut", kind="clothing", model="body", material="hdpe-shell", area=0.067, L=0.2, geom="sphere", svf=0.85,
      layers=[layer("hm-hair", "hair", "hair-layer", 0.03, 250, "Hair under the helmet", "Haar unter dem Helm", 0.008), layer("hm-air", "air-gap", "harness-air", 0.1, 0, "Air gap and harness (effective)", "Luftspalt und Kopfband (effektiv)", 0.03), layer("hm-shell", "shell", "hdpe-shell", 0.007, 700, "HDPE shell", "HDPE-Schale", 0.003)], seg="head", ms=0.015, sf=0.03, ex=1.0, su=0.0),
 dict(id="torso-clothing", en="T-shirt and work shirt on the trunk below and beside the vest", de="T-Shirt und Arbeitshemd am Rumpf unter und neben der Weste", kind="clothing", model="body", material="cotton-clothing", area=0.232, L=0.3, geom="cylinder", svf=0.5,
      layers=clothed("ts", [("tee", "clothing", "tee-cotton", 0.08, 250, "Cotton T-shirt", "Baumwoll-T-Shirt", 0.0008), ("shirt", "clothing", "cotton-clothing", 0.25, 500, "Long-sleeve work shirt", "Langarm-Arbeitshemd", 0.0012)]), seg="trunk", ms=0.12, sf=0.105, ex=0.9, su=0.0),
 dict(id="vest", en="High-visibility vest over shirt", de="Warnweste über dem Hemd", kind="clothing", model="body", material="hi-vis-polyester", area=0.533, L=0.3, geom="cylinder", svf=0.5,
      layers=clothed("vs", [("tee", "clothing", "tee-cotton", 0.08, 250, "Cotton T-shirt", "Baumwoll-T-Shirt", 0.0008), ("shirt", "clothing", "cotton-clothing", 0.25, 500, "Long-sleeve work shirt", "Langarm-Arbeitshemd", 0.0012), ("vest", "clothing", "hi-vis-polyester", 0.10, 300, "Polyester mesh vest with bands", "Polyester-Netzweste mit Streifen", 0.001)]), seg="trunk", ms=0.25, sf=0.22, ex=0.9, su=0.0),
 dict(id="arm-clothing", en="Long work-shirt sleeves", de="Lange Ärmel des Arbeitshemds", kind="clothing", model="body", material="cotton-clothing", area=0.317, L=0.09, geom="cylinder", svf=0.45,
      layers=clothed("am", [("shirt", "clothing", "cotton-clothing", 0.25, 500, "Long-sleeve work shirt", "Langarm-Arbeitshemd", 0.0012)]), seg="arm", ms=0.12, sf=0.14, ex=0.9, su=0.0),
 dict(id="gloves", en="Leather work gloves", de="Leder-Arbeitshandschuhe", kind="clothing", model="body", material="glove", area=0.111, L=0.05, geom="cylinder", svf=0.4,
      layers=clothed("gl", [("lining", "clothing", "cotton-clothing", 0.06, 250, "Cotton back", "Baumwollrücken", 0.001), ("leather", "shell", "glove", 0.05, 600, "Leather palm", "Lederhandfläche", 0.001)]), seg="hand", ms=0.03, sf=0.05, ex=1.0, su=0.0),
 dict(id="leg-clothing", en="Briefs and cotton work trousers", de="Unterhose und Baumwoll-Arbeitshose", kind="clothing", model="body", material="cotton-clothing", area=0.73, L=0.15, geom="cylinder", svf=0.45,
      layers=clothed("lg", [("briefs", "clothing", "briefs", 0.04, 150, "Cotton briefs", "Baumwollunterhose", 0.0008), ("trousers", "clothing", "cotton-clothing", 0.28, 800, "Thick cotton work trousers", "Dicke Baumwoll-Arbeitshose", 0.002)]), seg="leg", ms=0.38, sf=0.32, ex=0.88, su=0.0),
 dict(id="shoes", en="Socks and leather safety boots", de="Socken und Lederschutzstiefel", kind="clothing", model="body", material="leather-shoe", area=0.141, L=0.1, geom="cylinder", svf=0.25,
      layers=[layer("sh-sock", "clothing", "work-sock", 0.02, 400, "Thick work socks", "Dicke Arbeitssocken", 0.003), layer("sh-air", "air-gap", "still-air", 0.015, 0, "Air in the boot", "Luft im Stiefel", 0.005), layer("sh-leather", "shell", "leather-shoe", 0.05, 2000, "Leather upper, steel toe cap and sole", "Lederschaft, Stahlkappe und Sohle", 0.006)], seg="foot", ms=0.03, sf=0.07, ex=1.0, su=0.0),
 dict(id="box-surface", en="Cardboard box surface", de="Kartonoberfläche", kind="surface", model="passive", material="cardboard", area=0.66, L=0.3, geom="vertical-plate", svf=0.5, layers=[], cap=1500, ex=1.0, su=0.0, ref="ashrae-2021"),
]
for r in regions: r.setdefault("ref", "iso-9920-2007")
env = {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 20, "groundTempC": 19, "relativeHumidity": 0.5, "windMS": 0.2, "windDirectionDeg": 180, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325}
body = {"species": "human", "massKg": 84, "skinAreaM2": 2.02, "basalMetabolismW": 86, "metWM2": 58.15, "coreSetPointC": 36.8, "skinSetPointC": 34.1, "sweating": True, "panting": False, "maxSkinBloodFlowLM2H": 90, "minSkinBloodFlowLM2H": 6.3, "source": "authored-estimate", "reference": "gagge-1971"}
PAT = [("carrying", "Walking with the box", "Gehen mit Karton", 20, 1.45, (0.42, 0.02, 0.52, 0.04), "nose"),
       ("carrying-chest", "Carrying the box at the chest", "Karton vor der Brust tragen", 22, 1.5, (0.43, 0.01, 0.52, 0.04), "nose-and-mouth"),
       ("bracing", "Bracing in the squat", "Anspannen in der Hocke", 24, 1.6, (0.44, 0.0, 0.5, 0.06), "nose-and-mouth"),
       ("heavy-lift", "Lifting burst", "Hebestoß", 34, 1.9, (0.46, 0.0, 0.54, 0.0), "nose-and-mouth"),
       ("lowering", "Lowering the box", "Karton absetzen", 23, 1.55, (0.44, 0.01, 0.51, 0.04), "nose-and-mouth")]
resp = {"noseEmitter": "breath-nose", "mouthEmitter": "breath-mouth", "exhaledTempC": 34, "exhaledRelativeHumidity": 0.95, "patterns": [pattern(*p) for p in PAT],
        "motion": [{"bone": "torso", "angleDeg": 1.0, "dy": 0.005}], "source": "authored-estimate", "reference": "gupta-2010"}
RULE = {"squat-grip": dict(met=4.5, work=0, pat="bracing", wet=0.2, ct={"gloves": {"fraction": 0.5, "tempC": 20}, "box-surface": {"fraction": 0.1, "tempC": 19}}),
        "lift-mid": dict(met=7, work=150, pat="heavy-lift", wet=0.35, ct={"gloves": {"fraction": 0.5, "tempC": 20}}),
        "carrying-box-low": dict(met=3.4, work=0, pat="carrying", wet=0.15, ct={"gloves": {"fraction": 0.5, "tempC": 20}, "vest": {"fraction": 0.05, "tempC": 20}}),
        "carrying-box-chest": dict(met=3.8, work=0, pat="carrying-chest", wet=0.2, ct={"gloves": {"fraction": 0.5, "tempC": 20}, "vest": {"fraction": 0.2, "tempC": 20}}, ex={"vest": 0.7, "torso-clothing": 0.75}),
        "setting-down": dict(met=4.2, work=0, pat="lowering", wet=0.25, ct={"gloves": {"fraction": 0.5, "tempC": 20}})}
def pose_rule(p):
    r = RULE.get(p["id"], dict(met=3.4, work=0, pat="carrying", wet=0.15, ct={"gloves": {"fraction": 0.5, "tempC": 20}}))
    ct = dict(r["ct"]); ct["shoes"] = {"fraction": 0.3, "tempC": 19}
    w = r["wet"]
    return {"met": r["met"], "work": r["work"], "pat": r["pat"], "contact": ct, "ex": r.get("ex", {}), "wet": {"head-skin": w*0.8, "vest": w, "torso-clothing": w, "arm-clothing": w*0.7, "leg-clothing": w*0.6}, "em": {"plume": 0.5 if r["met"] < 4 else 0.8}}
spec = dict(materials=MATS, refs=[GAGGE, GUPTA], new_anchors=[dict(id="nose", bone="head", wx=0.073, wy=1.657, en="Nostrils", de="Nasenlöcher"), dict(id="crown", bone="head", wx=-0.032, wy=1.825, en="Crown of the head (helmet top)", de="Scheitel (Helmoberseite)")],
    env=env, body=body, respiration=resp, emitters=std_emitters(plume_region="vest"), regions=regions, baseMet=1.5, emit_default={"plume": 0.6}, pose_rule=pose_rule,
    clip_tracks={"squat-lift": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 4.5}, {"t": 0.6, "v": 5}, {"t": 1.1, "v": 7}, {"t": 1.6, "v": 7}, {"t": 2.6, "v": 3.4}, {"t": 3.2, "v": 3.4}]},
                                {"target": "externalWorkW", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 0.7, "v": 0}, {"t": 1.0, "v": 150}, {"t": 2.0, "v": 150}, {"t": 2.6, "v": 0}]},
                                {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 2}, {"t": 1.0, "v": 3}, {"t": 2.6, "v": 0}]}, {"target": "emitter:plume", "ease": "smooth", "keys": [{"t": 0, "v": 0.6}, {"t": 1.6, "v": 0.85}, {"t": 3.2, "v": 0.6}]}],
                 "carry-walk": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}]}, {"target": "emitter:plume", "ease": "smooth", "keys": [{"t": 0, "v": 0.5}, {"t": 1.1, "v": 0.5}]}],
                 "set-down": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 1.0, "v": 4}, {"t": 2.2, "v": 2}]}, {"target": "emitter:plume", "ease": "smooth", "keys": [{"t": 0, "v": 0.5}, {"t": 3, "v": 0.8}]}]},
    assumptions=[
     tx("Body regions sum to the DuBois area of 2.02 m2 (84 kg, 178 cm); clothed regions carry the area factor 1 + 0.28 clo. The vest overlays the shirt on most of the trunk, so trunk surface is split into a vest region (22 % of skin) and a shirt region (10.5 %).", "Die Körperregionen ergeben die DuBois-Fläche von 2,02 m2 (84 kg, 178 cm); bekleidete Regionen tragen den Flächenfaktor 1 + 0,28 clo. Die Weste liegt auf dem größten Teil des Rumpfs über dem Hemd, daher ist die Rumpffläche in eine Westenregion (22 % der Haut) und eine Hemdregion (10,5 %) geteilt."),
     tx("Clothing is layered per region (T-shirt, work shirt, mesh vest; briefs, thick work trousers; socks, leather boot; hair, air gap and HDPE shell in the helmet) with garment values guided by ISO 9920 (garment sum x 0.835 plus 0.161 clo enclosed air).", "Die Kleidung ist je Region geschichtet (T-Shirt, Arbeitshemd, Netzweste; Unterhose, dicke Arbeitshose; Socken, Lederstiefel; Haar, Luftspalt und HDPE-Schale im Helm) mit Werten nach Anhalt von ISO 9920 (Summe x 0,835 plus 0,161 clo eingeschlossene Luft)."),
     tx("Lifting the 20 kg box about 0.94 m takes 184 J; together with part of the body lift this is taken as 150 W of external work during the 2 s lifting burst (7 met peak, mechanical efficiency about 18 %). Carrying on level ground and holding the box do no external work; lowering returns the energy as heat.", "Das Anheben des 20-kg-Kartons um etwa 0,94 m erfordert 184 J; zusammen mit einem Teil des Körperhubs werden während des 2 s dauernden Hebestoßes 150 W äußere Arbeit angesetzt (Spitze 7 met, mechanischer Wirkungsgrad etwa 18 %). Tragen auf ebenem Boden und Halten leisten keine äußere Arbeit; Absetzen gibt die Energie als Wärme zurück."),
     tx("Metabolic rates for carrying (3.4 met), chest carry (3.8), squat (4.5) and lowering (4.2) follow the Compendium of Physical Activities classes for lifting and carrying; breathing patterns follow from the metabolic rate (ventilation 25 to 27 times the oxygen uptake): 29 to 64 L/min.", "Stoffwechselraten für Tragen (3,4 met), Brusttragen (3,8), Hocke (4,5) und Absetzen (4,2) folgen den Klassen des Compendium of Physical Activities für Heben und Tragen; die Atemmuster folgen aus dem Stoffwechsel (Ventilation das 25- bis 27-Fache der Sauerstoffaufnahme): 29 bis 64 L/min."),
     tx("The reference case is an indoor loading hall at 20 C without sun; the box is a passive cardboard surface at ambient temperature that exchanges heat only through the gloves and the pressed vest.", "Der Referenzfall ist eine Ladehalle bei 20 C ohne Sonne; der Karton ist eine passive Kartonfläche mit Umgebungstemperatur, die nur über die Handschuhe und die angedrückte Weste Wärme austauscht.")])
doc = finish(c, spec, reps)
doc["asset"]["summary"] = c.a["summary"]
print(write(doc, AID))
