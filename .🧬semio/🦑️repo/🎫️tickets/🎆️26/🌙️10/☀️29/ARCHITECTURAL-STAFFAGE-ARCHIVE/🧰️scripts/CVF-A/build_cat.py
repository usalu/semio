from build_common import *
import cat_rep as R
w = R.W
pat = pattern
PATS = [
 pat("rest", T_("Resting, nose breathing", "Ruhe, Nasenatmung"), 26, 0.042, (0.40, 0.05, 0.45, 0.10), "nose", "merck-resting-respiratory-rates"),
 pat("walk", T_("Walking, nose breathing", "Gehen, Nasenatmung"), 40, 0.045, (0.42, 0.02, 0.50, 0.06), "nose", "stahl-1967"),
 pat("stalk", T_("Stalking, slow and quiet", "Anschleichen, langsam und ruhig"), 30, 0.040, (0.42, 0.04, 0.46, 0.08), "nose", "stahl-1967"),
 pat("yawn", T_("Yawn through the mouth", "Gähnen durch das Maul"), 4, 0.12, (0.55, 0.10, 0.30, 0.05), "mouth", "stahl-1967"),
]
FUR = lambda t: ("fur", T_("Fur layer", "Fellschicht"), "fur", "short-fur", t, 0.045, round(t*91000), "cena-monteith-1975")
regs = [
 region("coat-trunk", T_("Trunk and neck coat", "Rumpf- und Halsfell"), "fur", "short-fur", 0.124, 0.13, "cylinder", 0.5, [FUR(0.012)], 16000, "trunk", 0.69, "scholander-1950"),
 region("coat-head", T_("Head coat", "Kopffell"), "fur", "short-fur", 0.013, 0.07, "sphere", 0.6, [FUR(0.006)], 9000, "head", 0.075, "cena-monteith-1975"),
 region("ears", T_("Ears", "Ohren"), "fur", "short-fur", 0.005, 0.045, "vertical-plate", 0.7, [FUR(0.002)], 2500, "head", 0.003, "cena-monteith-1975"),
 region("coat-forelimbs", T_("Foreleg coat", "Fell der Vorderbeine"), "fur", "short-fur", 0.030, 0.035, "cylinder", 0.3, [FUR(0.007)], 9000, "arm", 0.065, "cena-monteith-1975"),
 region("coat-hindlimbs", T_("Hindleg coat", "Fell der Hinterbeine"), "fur", "short-fur", 0.047, 0.045, "cylinder", 0.3, [FUR(0.008)], 10000, "leg", 0.13, "cena-monteith-1975"),
 region("coat-tail", T_("Tail coat", "Schwanzfell"), "fur", "short-fur", 0.030, 0.03, "cylinder", 0.5, [FUR(0.014)], 6000, "tail", 0.012, "cena-monteith-1975"),
 region("nose-skin", T_("Nose leather", "Nasenspiegel"), "skin", "bare-skin", 0.0005, 0.015, "sphere", 0.5, [], 3000, "head", 0.001, "ashrae-2021"),
 region("eyes", T_("Eyes", "Augen"), "skin", "cornea", 0.0005, 0.012, "sphere", 0.5, [], 2500, "head", 0.004, "ashrae-2021"),
 region("tongue", T_("Tongue", "Zunge"), "skin", "tongue-mucosa", 0.0015, 0.025, "horizontal-plate-down", 0.4, [], 5000, "head", 0.01, "ashrae-2021"),
 region("pads", T_("Paw pads", "Pfotenballen"), "skin", "bare-skin", 0.0085, 0.03, "horizontal-plate-down", 0.0, [], 8000, "foot", 0.01, "ashrae-2021"),
]
SUN = {"coat-trunk": 0.5, "coat-head": 0.5, "ears": 0.6, "coat-forelimbs": 0.25, "coat-hindlimbs": 0.25, "coat-tail": 0.4, "nose-skin": 0.4, "eyes": 0.5, "tongue": 0.4, "pads": 0.05}
def rule(p):
    i = p["id"]
    if i.startswith("walk"): return dict(met=2.0, breathing="walk")
    if i.startswith("stalk"): return dict(met=1.8, breathing="stalk")
    if i == "stretch-deep": return dict(met=1.5, breathing="yawn", exposed={"tongue": 0.6})
    if i == "play-bow": return dict(met=2.5, breathing="walk")
    if i == "sitting-tail-wrapped": return dict(met=1.1, breathing="rest")
    return dict(met=1.3, breathing="rest")
cfg = dict(world=w, regions=regs, sun=SUN, exposed={"tongue": 0.02}, ground_tempC=18, emit_default={"plume-back": 1.0}, rule=rule, base_met=1.3,
  contact_map={"contact-hn": ("pads", 0.15), "contact-hf": ("pads", 0.15), "contact-fn": ("pads", 0.15), "contact-ff": ("pads", 0.15), "contact-rump": ("coat-hindlimbs", 0.05)},
  breath_ids=["breath-nose", "yawn-mouth"], reset_met=True, drop_refs=["kleiber-1947"], keep_refs=["spector-1956"],
  body=dict(species="cat", massKg=4.2, skinAreaM2=0.26, basalMetabolismW=9.9, metWM2=38.1, coreSetPointC=38.6, skinSetPointC=35.0, sweating=False, panting=True,
            maxSkinBloodFlowLM2H=40, minSkinBloodFlowLM2H=4, source="authored-estimate", reference="kleiber-1932"),
  respiration=dict(noseEmitter="breath-nose", mouthEmitter="breath-mouth", exhaledTempC=31, exhaledRelativeHumidity=0.95, patterns=PATS,
                   motion=[{"bone": "thorax-b", "angleDeg": 1.0, "dy": 0.0015}], source="authored-estimate", reference="schmidt-nielsen-1970"),
  emitters=[
   {"id": "breath-nose", "kind": "breath", "title": T_("Nostrils", "Nasenlöcher"), "anchor": "nose", "directionDeg": -25, "diameterM": 0.007, "source": "authored-estimate", "reference": "schmidt-nielsen-1970"},
   {"id": "breath-mouth", "kind": "breath", "title": T_("Open mouth (yawn, panting)", "Offenes Maul (Gähnen, Hecheln)"), "anchor": "mouth", "directionDeg": -20, "diameterM": 0.02, "source": "authored-estimate", "reference": "ashrae-2021"},
   {"id": "plume-back", "kind": "plume", "title": T_("Warm-air plume above the back", "Warmluftfahne über dem Rücken"), "anchor": "back-plume", "directionDeg": 90, "diameterM": 0.12, "region": "coat-trunk", "source": "authored-estimate", "reference": "cena-monteith-1975"}],
  new_anchors=[emitter_anchor(w, "back-plume", "thorax-a", 0.2, 0.275, T_("Back above the trunk (plume source)", "Rücken über dem Rumpf (Fahnenquelle)"))],
  clip_tracks={
   "walk-cycle": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 2.0}, {"t": 0.4, "v": 2.1}, {"t": 0.8, "v": 2.0}]}],
   "stalk-cycle": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.8}, {"t": 1.2, "v": 1.9}, {"t": 2.4, "v": 1.8}]}],
   "stretch": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 4.2, "v": 3}, {"t": 6.0, "v": 0}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.3}, {"t": 4.2, "v": 1.5}, {"t": 8, "v": 1.3}]}]},
  assumptions=[
   T_("Skin area follows Meeh's law A = 0.1·M^(2/3) = 0.26 m² for 4.2 kg (k = 10 cm² per g^(2/3), Spector 1956); the ten regions sum to this area.", "Die Hautfläche folgt dem Meeh-Gesetz A = 0,1·M^(2/3) = 0,26 m² für 4,2 kg (k = 10 cm² je g^(2/3), Spector 1956); die zehn Regionen summieren sich zu dieser Fläche."),
   T_("Basal metabolism is Kleiber's 3.39 W per kg^0.75 = 9.9 W (verified literature coefficient); 1 met of this asset is that rate per skin area (38.1 W/m²). Standing 1.3, walking 2.0, stalking 1.8 and play 2.5 met are authored estimates (cost of walking after the running relation of Taylor et al. 1970 gives about 1.8).", "Der Grundumsatz ist Kleibers 3,39 W je kg^0,75 = 9,9 W (verifizierter Literaturkoeffizient); 1 Met dieses Assets ist diese Rate je Hautfläche (38,1 W/m²). Stehen 1,3, Gehen 2,0, Anschleichen 1,8 und Spiel 2,5 Met sind Schätzwerte (die Gehkosten nach der Laufbeziehung von Taylor et al. 1970 ergeben etwa 1,8)."),
   T_("Fur layers are thickness over an effective conductivity of 0.045 W/(m K): 12 mm trunk, 14 mm tail, 6-8 mm head and limbs, 2 mm ears; eyes, nose, tongue and pads are bare. Thicknesses are authored estimates guided by Scholander et al. (1950) and Cena and Monteith (1975).", "Fellschichten sind Dicke durch eine effektive Leitfähigkeit von 0,045 W/(m K): 12 mm Rumpf, 14 mm Schwanz, 6-8 mm Kopf und Gliedmaßen, 2 mm Ohren; Augen, Nase, Zunge und Ballen sind nackt. Die Dicken sind Schätzwerte nach Scholander et al. (1950) und Cena und Monteith (1975)."),
   T_("The resting rate of 26 per minute lies inside the verified range for cats (16-40, Merck Veterinary Manual after Reece 2004); tidal volumes of about 10 mL per kg, the walking, stalking and yawn patterns are authored estimates guided by allometric scaling (Stahl 1967). Cats pant only under extreme heat; no pose uses panting.", "Die Ruhefrequenz von 26 pro Minute liegt im verifizierten Bereich für Katzen (16-40, Merck Veterinary Manual nach Reece 2004); Atemzugvolumina von etwa 10 mL je kg sowie die Muster für Gehen, Anschleichen und Gähnen sind Schätzwerte nach allometrischer Skalierung (Stahl 1967). Katzen hecheln nur bei extremer Hitze; keine Pose nutzt Hecheln."),
  ],
  notes=T_("Side elevation of the near (left) side of a cat walking to the right; the tail is carried raised. Four preset scales are decided individually: 1:500 is unavailable (the cat would be 0.7 mm tall), 1:200 to 1:50 are drawn.", "Seitenansicht der nahen (linken) Seite einer nach rechts gehenden Katze; der Schwanz wird erhoben getragen. Vier Maßstäbe sind einzeln entschieden: 1:500 ist nicht verfügbar (die Katze wäre 0,7 mm hoch), 1:200 bis 1:50 sind gezeichnet."))
PENS = {"200": dict(body=0.18, detail=0.1, texture=None, glazing=None, ground=0.25), "100": dict(body=0.25, detail=0.13, texture=None, glazing=None, ground=0.35), "50": dict(body=0.35, detail=0.18, texture=0.1, glazing=None, ground=0.5)}
DEC = {
 "200": dict(
  silhouette=("One composed profile 4.0 × 1.8 mm: hindquarters, chest, neck, head with a separate jaw, two pointed ears, tail as two strokes, thighs and upper arms as masses, lower legs as strokes. Real proportions, bounds equal the standing-alert pose.", "Ein zusammengesetztes Profil von 4,0 × 1,8 mm: Hinterhand, Brust, Hals, Kopf mit eigenem Unterkiefer, zwei spitze Ohren, Schwanz als zwei Striche, Oberschenkel und Oberarme als Massen, Unterläufe als Striche. Echte Proportionen, die Grenzen entsprechen der Standpose."),
  retained=("Pointed ears and the raised tail (the identity of the cat), the eye dot, the jaw as its own mass for yawning, thighs and shoulders.", "Spitze Ohren und der erhobene Schwanz (die Identität der Katze), der Augenpunkt, der Unterkiefer als eigene Masse fürs Gähnen, Oberschenkel und Schultern."),
  merged=("Three trunk masses into two (hindbody, chest), nose leather and eye into the head mass, tongue into the jaw, shank/hock/paw into one stroke per limb section, five tail bones into two strokes. All regions are listed on the merged shapes.", "Drei Rumpfmassen zu zwei (Hinterhand, Brust), Nasenspiegel und Auge in die Kopfmasse, Zunge in den Unterkiefer, Unterschenkel/Sprunggelenk/Pfote zu einem Strich je Beinabschnitt, fünf Schwanzknochen zu zwei Strichen. Alle Regionen sind an den verschmolzenen Formen aufgeführt."),
  omitted=("Pads, toes, brow, mouth line, stripes and construction lines; the nose is 0.01 m = 0.05 mm.", "Ballen, Zehen, Braue, Maullinie, Streifen und Konstruktionslinien; die Nase misst 0,01 m = 0,05 mm."),
  thinElements=("Lower legs (0.022-0.04 m = 0.11-0.2 mm) are drawn as 0.2 mm strokes = 0.04 m, up to 0.018 m wider than real; the tail (0.03 m = 0.15 mm) as a 0.2 mm stroke = 0.04 m.", "Unterläufe (0,022-0,04 m = 0,11-0,2 mm) sind als 0,2-mm-Striche = 0,04 m gezeichnet, bis zu 0,018 m breiter als real; der Schwanz (0,03 m = 0,15 mm) als 0,2-mm-Strich = 0,04 m."),
  hatching=("None.", "Keine."),
  styles=("Mid-grey masses and strokes with a 0.18 mm body outline; far-side limbs one tone lighter.", "Mittelgraue Massen und Striche mit 0,18-mm-Körperkontur; Gliedmaßen der fernen Seite einen Ton heller."),
  overlays=("Limb strokes carry the limb coat and the pads; head and jaw carry head coat, nose, eyes and tongue together (merged, area-weighted).", "Beinstriche tragen Gliedmaßenfell und Ballen; Kopf und Unterkiefer tragen Kopffell, Nase, Augen und Zunge gemeinsam (verschmolzen, flächengewichtet).")),
 "100": dict(
  silhouette=("Articulated outline 8.0 × 3.6 mm: three trunk masses, neck, head, jaw, two ears, five tail pieces, each limb in its anatomical segments with paw outline.", "Gegliederter Umriss 8,0 × 3,6 mm: drei Rumpfmassen, Hals, Kopf, Unterkiefer, zwei Ohren, fünf Schwanzstücke, jede Gliedmaße in ihren anatomischen Segmenten mit Pfotenumriss."),
  retained=("Eye, nose leather, tongue, pad marks, shoulder line, both ears.", "Auge, Nasenspiegel, Zunge, Ballenmarken, Schulterlinie, beide Ohren."),
  merged=("Neck coat with the trunk coat; toes into the paw outline.", "Halsfell mit dem Rumpffell; Zehen in den Pfotenumriss."),
  omitted=("Toes, claws, whiskers, stripes and fur strokes (below 0.1 mm at this scale).", "Zehen, Krallen, Tasthaare, Streifen und Fellstriche (unter 0,1 mm in diesem Maßstab)."),
  thinElements=("Metacarpus and hock (0.022-0.026 m = 0.22-0.26 mm) drawn at true width; tail 0.02-0.034 m = 0.2-0.34 mm at true width.", "Mittelhand und Sprunggelenk (0,022-0,026 m = 0,22-0,26 mm) in wahrer Breite; Schwanz 0,02-0,034 m = 0,2-0,34 mm in wahrer Breite."),
  hatching=("None: fur is shown by tone.", "Keine: das Fell wird durch Ton gezeigt."),
  styles=("0.25 mm body outline, 0.13 mm details; far-side parts lighter and drawn first.", "0,25-mm-Körperkontur, 0,13-mm-Details; Teile der fernen Seite heller und zuerst gezeichnet."),
  overlays=("One region per shape: trunk, head, ears, fore and hind coat, tail, nose, eye, tongue, pads.", "Eine Region je Form: Rumpf, Kopf, Ohren, Vorder- und Hinterfell, Schwanz, Nase, Auge, Zunge, Ballen.")),
 "50": dict(
  silhouette=("Full articulation at 15.4 × 7.2 mm: three trunk masses, neck, head with brow and mouth line, inner ear, five tail rings, limbs in all segments with pads and toe lines.", "Volle Gliederung bei 15,4 × 7,2 mm: drei Rumpfmassen, Hals, Kopf mit Braue und Maullinie, Innenohr, fünf Schwanzringe, Gliedmaßen in allen Segmenten mit Ballen und Zehenlinien."),
  retained=("Tabby stripes, tail ring, inner ear, brow, mouth line, toes and the shoulder and hip lines.", "Tabby-Streifen, Schwanzring, Innenohr, Braue, Maullinie, Zehen sowie Schulter- und Hüftlinien."),
  merged=("Neck coat with trunk coat (seams hidden by the engine).", "Halsfell mit Rumpffell (Nähte vom System verborgen)."),
  omitted=("Whiskers (they would exceed the nominal width), claws, individual hairs.", "Tasthaare (sie würden die Nennbreite überschreiten), Krallen, einzelne Haare."),
  thinElements=("Tail 0.02-0.034 m = 0.4-0.68 mm and metacarpus 0.022 m = 0.44 mm at true width.", "Schwanz 0,02-0,034 m = 0,4-0,68 mm und Mittelhand 0,022 m = 0,44 mm in wahrer Breite."),
  hatching=("None; stripes are 0.1 mm texture strokes.", "Keine; Streifen sind 0,1-mm-Texturstriche."),
  styles=("Body 0.35 mm, detail 0.18 mm, texture 0.1 mm; far side tone 0.55, near 0.72.", "Körper 0,35 mm, Detail 0,18 mm, Textur 0,1 mm; ferne Seite Ton 0,55, nahe 0,72."),
  overlays=("One region per shape; stripes and detail strokes carry none.", "Eine Region je Form; Streifen und Detailstriche tragen keine.")),
}
VR = {"200": (150, 350), "100": (75, 150), "50": (20, 75)}
def build_cat():
    fn = {"200": R.rep200, "100": R.rep100, "50": R.rep50}
    reps = {"500": rep_unavail(500, ("A cat is 0.77 m × 0.36 m: 1.5 × 0.7 mm on paper at 1:500, trunk 0.11 m deep = 0.22 mm, legs 0.025-0.04 m = 0.05-0.08 mm. No composed silhouette separates head, legs and tail below the print profile's feature size without drawing a blob.", "Eine Katze misst 0,77 m × 0,36 m: auf dem Papier 1,5 × 0,7 mm bei 1:500, der Rumpf 0,11 m hoch = 0,22 mm, die Läufe 0,025-0,04 m = 0,05-0,08 mm. Keine zusammengesetzte Silhouette trennt Kopf, Läufe und Schwanz oberhalb der Merkmalsgröße des Druckprofils, ohne einen Klecks zu zeichnen."), ("Legs of the cat (real width 0.025 m = 0.05 mm)", "Läufe der Katze (reale Breite 0,025 m = 0,05 mm)"), 0.025, [{"kind": "scale", "denominator": 200}])}
    for k in ("200", "100", "50"): reps[k] = rep_auth(int(k), VR[k], PENS[k], fn[k](), DEC[k])
    return build("domestic-cat", cfg, reps, 50, {"heightM": 0.36, "widthM": 0.77, "depthM": 0.13, "massKg": 4.2})
if __name__ == "__main__":
    a = build_cat(); save(a, "domestic-cat"); print("saved", len(json.dumps(a)))
