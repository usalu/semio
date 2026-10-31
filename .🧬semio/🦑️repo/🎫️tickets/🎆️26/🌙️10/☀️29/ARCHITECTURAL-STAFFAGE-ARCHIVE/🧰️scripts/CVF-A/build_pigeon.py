from build_common import *
import pigeon_rep as R
w = R.W; pat = pattern
PATS = [
 pat("rest", T_("Resting, nostril breathing", "Ruhe, Nasenatmung"), 26, 0.005, (0.45, 0.02, 0.45, 0.08), "nose", "calder-king-1974"),
 pat("walk", T_("Walking, nostril breathing", "Gehen, Nasenatmung"), 40, 0.006, (0.45, 0.02, 0.45, 0.08), "nose", "calder-king-1974"),
 pat("flight", T_("Flapping flight, fast breathing", "Flügelschlagflug, schnelle Atmung"), 240, 0.010, (0.5, 0.0, 0.5, 0.0), "nose", "butler-1977"),
]
FE = lambda t: ("plumage", T_("Plumage layer", "Gefiederschicht"), "feather", "plumage", t, 0.04, round(t*60000), "scholander-1950")
regs = [
 region("plumage-body", T_("Body plumage", "Rumpfgefieder"), "feather", "plumage", 0.022, 0.12, "cylinder", 0.5, [FE(0.015)], 12000, "trunk", 0.66, "walsberg-king-1978"),
 region("plumage-neck-head", T_("Neck and head plumage", "Hals- und Kopfgefieder"), "feather", "plumage", 0.0035, 0.04, "cylinder", 0.6, [FE(0.008)], 6000, "head", 0.09, "walsberg-king-1978"),
 region("plumage-wings", T_("Wing plumage", "Flügelgefieder"), "feather", "plumage", 0.02, 0.1, "vertical-plate", 0.5, [FE(0.008)], 5000, "wing", 0.12, "walsberg-king-1978"),
 region("plumage-tail", T_("Tail plumage", "Schwanzgefieder"), "feather", "plumage", 0.0035, 0.07, "horizontal-plate-up", 0.5, [FE(0.006)], 3000, "tail", 0.02, "walsberg-king-1978"),
 region("legs-feet", T_("Legs and feet", "Beine und Füße"), "skin", "bare-skin", 0.002, 0.012, "cylinder", 0.3, [], 2500, "leg", 0.06, "ashrae-2021"),
 region("beak", T_("Beak", "Schnabel"), "skin", "beak-keratin", 0.0003, 0.01, "cylinder", 0.5, [], 1500, "head", 0.02, "ashrae-2021"),
 region("cere", T_("Cere", "Wachshaut"), "skin", "cere-wax", 0.0001, 0.008, "sphere", 0.5, [], 1000, "head", 0.01, "ashrae-2021"),
 region("eye", T_("Eye", "Auge"), "skin", "eye-tissue", 0.00015, 0.01, "sphere", 0.5, [], 1500, "head", 0.02, "ashrae-2021"),
]
SUN = {"plumage-body": 0.5, "plumage-neck-head": 0.5, "plumage-wings": 0.4, "plumage-tail": 0.3, "legs-feet": 0.2, "beak": 0.4, "cere": 0.4, "eye": 0.5}
def rule(p):
    i = p["id"]
    wing = 1.0
    if i in ("standing-alert", "pecking-strike", "pecking-reach") or i.startswith("walk"): wing = 0.45
    if i == "takeoff-crouch": wing = 0.7
    if i.startswith("walk"): return dict(met=2.0, breathing="walk", exposed={"plumage-wings": wing})
    if i.startswith("peck"): return dict(met=1.6, breathing="rest", exposed={"plumage-wings": wing})
    if i == "wings-half-open": return dict(met=1.5, breathing="rest", exposed={"plumage-wings": 1.0})
    if i == "takeoff-crouch": return dict(met=3.0, breathing="walk", exposed={"plumage-wings": wing})
    if i == "takeoff-wings-up": return dict(met=6.0, breathing="flight", exposed={"plumage-wings": 1.0})
    if i.startswith(("takeoff", "airborne")): return dict(met=15.0, breathing="flight", exposed={"plumage-wings": 1.0})
    return dict(met=1.3, breathing="rest", exposed={"plumage-wings": wing})
cfg = dict(world=w, regions=regs, sun=SUN, exposed={}, ground_tempC=18, emit_default={"plume-back": 1.0}, rule=rule, base_met=1.3,
  contact_map={"contact-n": ("legs-feet", 0.15), "contact-f": ("legs-feet", 0.15)},
  breath_ids=["breath-nostril", "breath-exertion"], reset_met=True, drop_refs=[], keep_refs=[],
  body=dict(species="bird", massKg=0.35, skinAreaM2=0.0517, basalMetabolismW=2.0, metWM2=38.7, coreSetPointC=41.5, skinSetPointC=39.0, sweating=False, panting=True,
            maxSkinBloodFlowLM2H=60, minSkinBloodFlowLM2H=6, source="authored-estimate", reference="lasiewski-dawson-1967"),
  respiration=dict(noseEmitter="breath-nostril", exhaledTempC=32, exhaledRelativeHumidity=0.9, patterns=PATS,
                   motion=[{"bone": "trunk", "angleDeg": 1.5, "dy": 0.002}], source="authored-estimate", reference="schmidt-nielsen-1970"),
  emitters=[
   {"id": "breath-nostril", "kind": "breath", "title": T_("Nostrils in the cere", "Nasenlöcher in der Wachshaut"), "anchor": "nostril", "directionDeg": -20, "diameterM": 0.002, "source": "authored-estimate", "reference": "calder-king-1974"},
   {"id": "plume-back", "kind": "plume", "title": T_("Warm-air plume above the back", "Warmluftfahne über dem Rücken"), "anchor": "back-plume", "directionDeg": 90, "diameterM": 0.1, "region": "plumage-body", "source": "authored-estimate", "reference": "scholander-1950"}],
  new_anchors=[emitter_anchor(w, "back-plume", "trunk", 0.09, 0.0, T_("Back above the trunk (plume source)", "Rücken über dem Rumpf (Fahnenquelle)"))],
  clip_tracks={
   "walk-cycle": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 2.0}, {"t": 0.25, "v": 2.1}, {"t": 0.5, "v": 2.0}]}],
   "peck-cycle": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.3}, {"t": 0.4, "v": 1.6}, {"t": 0.84, "v": 1.6}, {"t": 1.3, "v": 1.3}, {"t": 2.0, "v": 1.3}]}],
   "take-off": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 0.22, "v": 2}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 3}, {"t": 0.22, "v": 6}, {"t": 0.3, "v": 15}, {"t": 0.72, "v": 15}]}]},
  assumptions=[
   T_("Surface area about 0.052 m² for 0.35 kg (Walsberg and King 1978: area proportional to mass^0.667); the eight regions sum to this area, plumage covers 98 %.", "Oberfläche etwa 0,052 m² für 0,35 kg (Walsberg und King 1978: Fläche proportional zu Masse^0,667); die acht Regionen summieren sich zu dieser Fläche, Gefieder bedeckt 98 %."),
   T_("Basal metabolism 2.0 W is an authored estimate guided by the non-passerine allometry of Lasiewski and Dawson (1967) and Aschoff and Pohl (1970); 1 met = 38.7 W/m². Standing 1.3, walking 2.0, take-off 3-6 and flapping flight 15 met (about 30 W) are authored estimates; the body temperature of 41.5 °C is typical of columbiform birds.", "Der Grundumsatz von 2,0 W ist ein Schätzwert nach der Nicht-Singvogel-Allometrie von Lasiewski und Dawson (1967) und Aschoff und Pohl (1970); 1 Met = 38,7 W/m². Stehen 1,3, Gehen 2,0, Abflug 3-6 und Flügelschlagflug 15 Met (etwa 30 W) sind Schätzwerte; die Körpertemperatur von 41,5 °C ist für Taubenvögel typisch."),
   T_("Plumage layers are thickness over an effective conductivity of 0.04 W/(m K): 15 mm on the body (contour feathers over down), 6-8 mm on neck, wings and tail; legs, feet, beak and cere are bare. Authored estimates guided by Scholander et al. (1950). Wings tucked against the flank are exposed to 45 %.", "Gefiederschichten sind Dicke durch eine effektive Leitfähigkeit von 0,04 W/(m K): 15 mm am Rumpf (Deckfedern über Daunen), 6-8 mm an Hals, Flügeln und Schwanz; Beine, Füße, Schnabel und Wachshaut sind nackt. Schätzwerte nach Scholander et al. (1950). Am Rumpf angelegte Flügel sind zu 45 % exponiert."),
   T_("Respiration rates and volumes of the pigeon are authored estimates (rest 26 per minute, 5 mL; flight 240 per minute, 10 mL) guided by Calder and King (1974) and Butler et al. (1977); no verified avian values were available. The exhaled air leaves at 32 °C.", "Atemfrequenzen und -volumina der Taube sind Schätzwerte (Ruhe 26 pro Minute, 5 mL; Flug 240 pro Minute, 10 mL) nach Calder und King (1974) und Butler et al. (1977); verifizierte Vogelwerte lagen nicht vor. Die Ausatemluft verlässt den Körper mit 32 °C."),
  ],
  notes=T_("Side elevation of a feral pigeon (Columba livia) walking to the right; folded wing on the near side, far wing and tail drawn behind. 1:500 is unavailable (0.5 × 0.7 mm); 1:200, 1:100 and 1:50 are drawn.", "Seitenansicht einer Stadttaube (Columba livia), die nach rechts geht; angelegter Flügel der nahen Seite, ferner Flügel und Schwanz dahinter. 1:500 ist nicht verfügbar (0,5 × 0,7 mm); 1:200, 1:100 und 1:50 sind gezeichnet."))
DEC = {
 200: dict(
  silhouette=("One composed profile of 1.7 × 1.3 mm: plump body, neck wedge, head with beak, tail wedge, near wing as coverts and primaries, far wing primaries; legs as two 0.15 mm strokes.", "Ein zusammengesetztes Profil von 1,7 × 1,3 mm: rundlicher Rumpf, Halskeil, Kopf mit Schnabel, Schwanzkeil, naher Flügel als Decken und Schwingen, ferner Flügel nur Schwingen; Beine als zwei 0,15-mm-Striche."),
  retained=("Round breast, the small head, the long folded primaries and the wedge tail.", "Runde Brust, der kleine Kopf, die langen angelegten Schwingen und der Keilschwanz."),
  merged=("Beak, cere and eye into the head mass; three neck bones into one wedge; thigh into the body; shank, tarsus and toes into two strokes; shoulder into the covert mass.", "Schnabel, Wachshaut und Auge in die Kopfmasse; drei Halsknochen zu einem Keil; Oberschenkel in den Rumpf; Unterschenkel, Lauf und Zehen zu zwei Strichen; Schulter in die Deckfedermasse."),
  omitted=("Jaw, eye ring, feather structure, toes and every detail line; the eye (0.009 m = 0.045 mm) cannot print.", "Unterschnabel, Augenring, Federstruktur, Zehen und alle Detaillinien; das Auge (0,009 m = 0,045 mm) ist nicht druckbar."),
  thinElements=("Tarsus and shank (0.014-0.02 m = 0.07-0.1 mm) are 0.15 mm strokes = 0.03 m, up to 0.016 m wider than real, below the 0.03 m the 0.15 mm outline already adds.", "Lauf und Unterschenkel (0,014-0,02 m = 0,07-0,1 mm) sind 0,15-mm-Striche = 0,03 m, bis zu 0,016 m breiter als real, unterhalb der 0,03 m, die die 0,15-mm-Kontur ohnehin zufügt."),
  hatching=("None.", "Keine."),
  styles=("Grey solids with a 0.15 mm outline; far wing and tail lighter.", "Graue Flächen mit 0,15-mm-Kontur; ferner Flügel und Schwanz heller."),
  overlays=("Head mass carries neck/head plumage, beak, cere and eye (merged, area-weighted); leg strokes carry legs-feet.", "Die Kopfmasse trägt Hals-/Kopfgefieder, Schnabel, Wachshaut und Auge (verschmolzen, flächengewichtet); Beinstriche tragen Beine und Füße.")),
 100: dict(
  silhouette=("Articulated outline 3.4 × 2.5 mm: body, three neck pieces, head, beak, jaw, two tails, both wings in shoulder, coverts and primaries, both legs in thigh, shank, tarsus and toes.", "Gegliederter Umriss 3,4 × 2,5 mm: Rumpf, drei Halsstücke, Kopf, Schnabel, Unterschnabel, zwei Schwänze, beide Flügel in Schulter, Decken und Schwingen, beide Beine in Oberschenkel, Unterschenkel, Lauf und Zehen."),
  retained=("Eye dot, cere, beak in a darker tone, thighs inside the plumage.", "Augenpunkt, Wachshaut, Schnabel in dunklerem Ton, Oberschenkel im Gefieder."),
  merged=("Neck pieces share the plumage region (seams hidden); toes merge into one foot outline.", "Halsstücke teilen die Gefiederregion (Nähte verborgen); Zehen bilden einen Fußumriss."),
  omitted=("Individual toes, claws, eye ring, feather texture.", "Einzelne Zehen, Krallen, Augenring, Federstruktur."),
  thinElements=("Tarsus 0.014-0.016 m = 0.14-0.16 mm and shank 0.016-0.024 m drawn at true width; they sit at the profile minimum and are the first features to check.", "Lauf 0,014-0,016 m = 0,14-0,16 mm und Unterschenkel 0,016-0,024 m in wahrer Breite; sie liegen am Profilminimum und sind die ersten zu prüfenden Merkmale."),
  hatching=("None.", "Keine."),
  styles=("0.2 mm body outline, 0.13 mm details; beak darker.", "0,2-mm-Körperkontur, 0,13-mm-Details; Schnabel dunkler."),
  overlays=("One region per shape.", "Eine Region je Form.")),
 50: dict(
  silhouette=("Full articulation at 6.7 × 5 mm: all pieces of the 1:100 drawing plus eye ring, neck sheen, wing bar, three primary strokes, breast feather scallops and a tail band.", "Volle Gliederung bei 6,7 × 5 mm: alle Stücke der 1:100-Zeichnung plus Augenring, Halsglanz, Flügelbinde, drei Schwingenstriche, Brustfederschuppen und ein Schwanzband."),
  retained=("Neck sheen, double wing bar, primaries, breast feathers, tail band.", "Halsglanz, doppelte Flügelbinde, Schwingen, Brustfedern, Schwanzband."),
  merged=("Same-plumage neighbours share their region (seams hidden).", "Benachbarte Teile desselben Gefieders teilen ihre Region (Nähte verborgen)."),
  omitted=("Individual feather barbs, claws, nostril slit.", "Einzelne Federäste, Krallen, Nasenschlitz."),
  thinElements=("Tarsus 0.014 m = 0.28 mm and toes 0.014 m at true width.", "Lauf 0,014 m = 0,28 mm und Zehen 0,014 m in wahrer Breite."),
  hatching=("None; 0.1 mm texture strokes follow the primaries.", "Keine; 0,1-mm-Texturstriche folgen den Schwingen."),
  styles=("0.3 mm body, 0.15 mm detail, 0.1 mm texture.", "0,3 mm Körper, 0,15 mm Detail, 0,1 mm Textur."),
  overlays=("One region per shape; strokes carry none.", "Eine Region je Form; Striche tragen keine.")),
}
VR = {200: (150, 350), 100: (75, 150), 50: (20, 75)}
def build_pigeon():
    reps = {"500": rep_unavail(500, ("A pigeon is 0.335 m × 0.254 m: 0.7 × 0.5 mm on paper at 1:500, body 0.14 m deep = 0.28 mm, legs 0.014 m = 0.03 mm. It would print as a speck with no head, wing or tail.", "Eine Taube misst 0,335 m × 0,254 m: auf dem Papier 0,7 × 0,5 mm bei 1:500, der Rumpf 0,14 m tief = 0,28 mm, die Beine 0,014 m = 0,03 mm. Sie würde als Fleck ohne Kopf, Flügel und Schwanz drucken."), ("Whole bird (body depth 0.14 m = 0.28 mm)", "Ganzer Vogel (Rumpftiefe 0,14 m = 0,28 mm)"), 0.14, [{"kind": "scale", "denominator": 200}])}
    for k in (200, 100, 50): reps[str(k)] = rep_auth(k, VR[k], R.PENS[k], R.rep(k), DEC[k])
    return build("pigeon", cfg, reps, 50, {"heightM": 0.25, "widthM": 0.33, "depthM": 0.15, "massKg": 0.35})
if __name__ == "__main__":
    a = build_pigeon(); save(a, "pigeon"); print("saved", len(json.dumps(a)))
