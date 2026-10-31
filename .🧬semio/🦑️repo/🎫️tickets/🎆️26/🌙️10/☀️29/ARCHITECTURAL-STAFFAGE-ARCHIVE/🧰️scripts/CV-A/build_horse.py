from build_common import *
import horse_rep as R
w = R.W; pat = pattern
PATS = [
 pat("rest", T_("Resting, nose breathing", "Ruhe, Nasenatmung"), 12, 5.5, (0.38, 0.04, 0.45, 0.13), "nose", "merck-resting-respiratory-rates"),
 pat("walk", T_("Walking, nose breathing", "Gehen, Nasenatmung"), 22, 6.5, (0.42, 0.02, 0.50, 0.06), "nose", "hodgson-rose-1994"),
 pat("graze", T_("Grazing, nose breathing", "Grasen, Nasenatmung"), 14, 5.5, (0.40, 0.04, 0.46, 0.10), "nose", "merck-resting-respiratory-rates"),
 pat("snort", T_("Snort", "Schnauben"), 6, 8.0, (0.30, 0.10, 0.50, 0.10), "nose", "hodgson-rose-1994"),
]
FUR = lambda t: ("coat", T_("Coat layer", "Haarkleid"), "fur", "short-coat", t, 0.045, round(t*91000), "cena-monteith-1975")
regs = [
 region("coat-trunk", T_("Trunk and neck coat", "Rumpf- und Halsfell"), "fur", "short-coat", 4.05, 0.75, "cylinder", 0.5, [FUR(0.010)], 40000, "trunk", 0.7235, "scholander-1950"),
 region("coat-head", T_("Head coat", "Kopffell"), "fur", "short-coat", 0.30, 0.22, "cylinder", 0.6, [FUR(0.006)], 20000, "head", 0.0595, "cena-monteith-1975"),
 region("coat-forelimbs", T_("Foreleg coat", "Fell der Vorderbeine"), "fur", "short-coat", 0.55, 0.12, "cylinder", 0.3, [FUR(0.005)], 18000, "arm", 0.08, "cena-monteith-1975"),
 region("coat-hindlimbs", T_("Hindleg coat", "Fell der Hinterbeine"), "fur", "short-coat", 0.75, 0.16, "cylinder", 0.3, [FUR(0.005)], 20000, "leg", 0.12, "cena-monteith-1975"),
 passive("tail", T_("Tail hair", "Schweifhaar"), "hair", "mane-tail-hair", 0.12, 0.12, "cylinder", 0.5, 1500, "cena-monteith-1975"),
 passive("mane", T_("Mane hair", "Mähnenhaar"), "hair", "mane-tail-hair", 0.10, 0.08, "vertical-plate", 0.7, 1500, "cena-monteith-1975"),
 region("ears", T_("Ears", "Ohren"), "fur", "short-coat", 0.03, 0.10, "vertical-plate", 0.7, [FUR(0.003)], 3500, "head", 0.003, "cena-monteith-1975"),
 region("muzzle-skin", T_("Muzzle skin", "Maulhaut"), "skin", "bare-skin", 0.015, 0.08, "sphere", 0.5, [], 6000, "head", 0.01, "ashrae-2021"),
 region("eyes", T_("Eyes", "Augen"), "skin", "eye-surface", 0.003, 0.04, "sphere", 0.5, [], 3000, "head", 0.003, "ashrae-2021"),
 region("hooves", T_("Hooves", "Hufe"), "surface", "hoof-horn", 0.12, 0.12, "horizontal-plate-down", 0.0, [("horn", T_("Hoof horn", "Hufhorn"), "shell", "hoof-horn", 0.008, 0.3, 17600, "hoyt-taylor-1981")], 17600, "foot", 0.001, "hoyt-taylor-1981"),
]
for r in regs:
    if r["id"] == "hooves": r["layers"][0]["source"] = "illustrative"; r["layers"][0].pop("vapourResistanceM2PaW")
SUN = {"coat-trunk": 0.5, "coat-head": 0.5, "coat-forelimbs": 0.25, "coat-hindlimbs": 0.25, "tail": 0.3, "mane": 0.6, "ears": 0.6, "muzzle-skin": 0.4, "eyes": 0.5, "hooves": 0.05}
SWEAT = {"coat-trunk": 0.08, "coat-forelimbs": 0.08, "coat-hindlimbs": 0.08, "coat-head": 0.05}
def rule(p):
    i = p["id"]
    if i.startswith("walk"): return dict(met=3.9, breathing="walk", wet=SWEAT)
    if i == "grazing-head-down": return dict(met=1.8, breathing="graze")
    if i == "standing-rest": return dict(met=1.35, breathing="rest", exposed={"eyes": 0.55})
    return dict(met=1.5, breathing="rest")
cfg = dict(world=w, regions=regs, sun=SUN, exposed={}, ground_tempC=18, emit_default={"plume-back": 1.0}, rule=rule, base_met=1.5,
  contact_map={"contact-hn": ("hooves", 0.2), "contact-hf": ("hooves", 0.2), "contact-fn": ("hooves", 0.2), "contact-ff": ("hooves", 0.2)},
  breath_ids=["breath-nose", "snort"], reset_met=True, drop_refs=["kleiber-1947"], keep_refs=["spector-1956"],
  body=dict(species="horse", massKg=550, skinAreaM2=6.04, basalMetabolismW=384, metWM2=63.6, coreSetPointC=37.8, skinSetPointC=33.0, sweating=True, panting=False,
            maxSkinBloodFlowLM2H=70, minSkinBloodFlowLM2H=5, source="authored-estimate", reference="kleiber-1932"),
  respiration=dict(noseEmitter="breath-nose", exhaledTempC=30, exhaledRelativeHumidity=0.95, patterns=PATS,
                   motion=[{"bone": "thorax", "angleDeg": 0.5, "dy": 0.012}], source="authored-estimate", reference="schmidt-nielsen-1970"),
  emitters=[
   {"id": "breath-nose", "kind": "breath", "title": T_("Nostrils", "Nüstern"), "anchor": "nose", "directionDeg": -25, "diameterM": 0.04, "source": "authored-estimate", "reference": "schmidt-nielsen-1970"},
   {"id": "plume-back", "kind": "plume", "title": T_("Warm-air plume above the back", "Warmluftfahne über dem Rücken"), "anchor": "back-plume", "directionDeg": 90, "diameterM": 0.8, "region": "coat-trunk", "source": "authored-estimate", "reference": "cena-monteith-1975"}],
  new_anchors=[emitter_anchor(w, "back-plume", "thorax", 0.85, 1.58, T_("Back above the trunk (plume source)", "Rücken über dem Rumpf (Fahnenquelle)"))],
  clip_tracks={
   "walk-cycle": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 3.9}, {"t": 0.475, "v": 4.0}, {"t": 0.95, "v": 3.9}]}],
   "graze-and-swish": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 2}, {"t": 2.9, "v": 3}, {"t": 3.3, "v": 2}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.8}, {"t": 3.2, "v": 1.9}, {"t": 6.4, "v": 1.8}]}]},
  assumptions=[
   T_("Skin area follows Meeh's law with k = 9.0: 6.04 m² for 550 kg (Spector 1956); the ten regions sum to this area, mane and tail hair are passive surfaces.", "Die Hautfläche folgt dem Meeh-Gesetz mit k = 9,0: 6,04 m² für 550 kg (Spector 1956); die zehn Regionen summieren sich zu dieser Fläche, Mähnen- und Schweifhaar sind passive Flächen."),
   T_("Basal metabolism is Kleiber's 3.39 W per kg^0.75 = 384 W (verified coefficient); 1 met is that rate per skin area (63.6 W/m²). Standing 1.5 (about 0.98 W/kg), grazing 1.8 and walking 3.9 met (about 1.5 kW at 1.3 m/s, cost of walking about 1.8 J per kg and m after Hoyt and Taylor 1981) are authored estimates.", "Der Grundumsatz ist Kleibers 3,39 W je kg^0,75 = 384 W (verifizierter Koeffizient); 1 Met ist diese Rate je Hautfläche (63,6 W/m²). Stehen 1,5 (etwa 0,98 W/kg), Grasen 1,8 und Gehen 3,9 Met (etwa 1,5 kW bei 1,3 m/s, Gehkosten etwa 1,8 J je kg und m nach Hoyt und Taylor 1981) sind Schätzwerte."),
   T_("Coat layers are thickness over an effective conductivity of 0.045 W/(m K): a short summer coat of 10 mm on the trunk, 5-6 mm on head and legs, 3 mm on the ears; muzzle and eyes are bare, hooves are horn shells. Authored estimates guided by Scholander et al. (1950) and Cena and Monteith (1975). Walking raises wettedness to 0.08 (sweating horse).", "Haarkleidschichten sind Dicke durch eine effektive Leitfähigkeit von 0,045 W/(m K): kurzes Sommerfell von 10 mm am Rumpf, 5-6 mm an Kopf und Beinen, 3 mm an den Ohren; Maul und Augen sind nackt, Hufe sind Hornschalen. Schätzwerte nach Scholander et al. (1950) und Cena und Monteith (1975). Beim Gehen steigt die Benetzung auf 0,08 (schwitzendes Pferd)."),
   T_("The resting rate of 12 per minute lies inside the verified range for horses (10-14, Merck Veterinary Manual after Reece 2004); tidal volumes of 5.5-6.5 L, the walking, grazing and snort patterns are authored estimates (Hodgson and Rose 1994 as guidance). Locomotor-respiratory coupling (Bramble and Carrier 1983) appears only at the gallop and is not modelled here.", "Die Ruhefrequenz von 12 pro Minute liegt im verifizierten Bereich für Pferde (10-14, Merck Veterinary Manual nach Reece 2004); Atemzugvolumina von 5,5-6,5 L sowie die Muster für Gehen, Grasen und Schnauben sind Schätzwerte (Hodgson und Rose 1994 als Anhalt). Die Bewegungs-Atem-Kopplung (Bramble und Carrier 1983) tritt erst im Galopp auf und wird hier nicht modelliert."),
  ],
  notes=T_("Side elevation of the near side of a horse walking to the right, head carried high in the default pose. Four preset scales are individually drawn.", "Seitenansicht der nahen Seite eines nach rechts gehenden Pferdes, in der Standardpose mit hoch getragenem Kopf. Vier Maßstäbe sind einzeln gezeichnet."))
DEC = {
 500: dict(
  silhouette=("One composed side silhouette of 5.8 × 4.8 mm: hindquarters and chest masses, a two-part neck with the mane merged in, one head mass with a single ear mass, two tail masses, thigh and upper-arm masses; lower legs as strokes. Bounds equal the standing-alert pose.", "Eine zusammengesetzte Seitensilhouette von 5,8 × 4,8 mm: Hinterhand- und Brustmasse, ein zweiteiliger Hals mit eingeschmolzener Mähne, eine Kopfmasse mit einer Ohrmasse, zwei Schweifmassen, Oberschenkel- und Oberarmmassen; Unterläufe als Striche. Die Grenzen entsprechen der Standpose."),
  retained=("The long neck and raised head, the heavy hindquarters, the hanging tail and the leg angulation (hock, carpus).", "Der lange Hals und der erhobene Kopf, die schwere Hinterhand, der hängende Schweif und die Beinwinkelung (Sprung-, Vorderfußwurzelgelenk)."),
  merged=("Mane into the neck mass, muzzle and eye into the head mass, gaskin and cannon into one stroke, pastern and hoof into the stroke end. Every region is listed on the merged shapes.", "Mähne in die Halsmasse, Maul und Auge in die Kopfmasse, Unterschenkel und Röhrbein zu einem Strich, Fesseln und Huf in das Strichende. Jede Region ist an den verschmolzenen Formen aufgeführt."),
  omitted=("Jaw as a separate mass, forelock, the far ear, hoof outlines, eye and all construction lines.", "Unterkiefer als eigene Masse, Stirnschopf, das ferne Ohr, Hufumrisse, Auge und alle Konstruktionslinien."),
  thinElements=("Cannon and pastern (0.085-0.13 m = 0.17-0.26 mm) are 0.3 mm strokes = 0.15 m wide, up to 0.065 m wider than real, close to the 0.065 m a 0.13 mm outline adds per side.", "Röhrbein und Fessel (0,085-0,13 m = 0,17-0,26 mm) sind 0,3-mm-Striche = 0,15 m breit, bis zu 0,065 m breiter als real, nahe den 0,065 m, die eine 0,13-mm-Kontur je Seite zufügt."),
  hatching=("None.", "Keine."),
  styles=("Mid-grey masses with a 0.13 mm outline, far legs lighter and drawn first.", "Mittelgraue Massen mit 0,13-mm-Kontur, ferne Beine heller und zuerst gezeichnet."),
  overlays=("Merged masses carry the area-weighted value of their listed regions; the neck carries trunk coat and mane together, the head carries head, muzzle and eye.", "Verschmolzene Massen tragen den flächengewichteten Wert ihrer aufgeführten Regionen; der Hals trägt Rumpffell und Mähne gemeinsam, der Kopf Kopf, Maul und Auge.")),
 200: dict(
  silhouette=("Main masses separated at 11.6 × 4.7 mm... real proportions: hindquarters, chest, three neck pieces, head with jaw, ears, mane band, four tail pieces, every limb in its segments (thigh, gaskin, cannon, pastern, hoof; scapula, humerus, radius, cannon, pastern, hoof).", "Hauptmassen getrennt bei 11,6 × 4,7 mm in echten Proportionen: Hinterhand, Brust, drei Halsstücke, Kopf mit Unterkiefer, Ohren, Mähnenband, vier Schweifstücke, jede Gliedmaße in ihren Segmenten (Oberschenkel, Unterschenkel, Röhrbein, Fessel, Huf; Schulterblatt, Oberarm, Speiche, Röhrbein, Fessel, Huf)."),
  retained=("Hooves as separate masses, the mane band, the eye dot, both ears, the jaw.", "Hufe als eigene Massen, das Mähnenband, der Augenpunkt, beide Ohren, der Unterkiefer."),
  merged=("Muzzle skin into the head mass; seams between same-coat segments are hidden.", "Maulhaut in die Kopfmasse; Nähte zwischen Segmenten gleichen Fells sind verborgen."),
  omitted=("Forelock, nostril, brow, mouth line, hoof lines, coat strokes and all construction lines.", "Stirnschopf, Nüster, Braue, Maullinie, Hufstriche, Fellstriche und alle Konstruktionslinien."),
  thinElements=("Cannon and pastern widened by 15 % (0.12-0.15 m = 0.6-0.75 mm printed against 0.1-0.13 m real) to stay above the print profile's feature size.", "Röhrbein und Fessel um 15 % verbreitert (0,12-0,15 m = 0,6-0,75 mm gedruckt statt 0,1-0,13 m real), damit sie über der Merkmalsgröße des Druckprofils bleiben."),
  hatching=("None.", "Keine."),
  styles=("0.18 mm body outline, 0.1 mm eye; far side lighter.", "0,18-mm-Körperkontur, 0,1-mm-Auge; ferne Seite heller."),
  overlays=("One region per shape; mane and tail hair are separate passive regions.", "Eine Region je Form; Mähnen- und Schweifhaar sind eigene passive Regionen.")),
 100: dict(
  silhouette=("Articulated outline 23 × 9.5 mm with all segments at true width, mane in three pieces, forelock, ears, jaw.", "Gegliederter Umriss 23 × 9,5 mm mit allen Segmenten in wahrer Breite, Mähne in drei Stücken, Stirnschopf, Ohren, Unterkiefer."),
  retained=("Hoof shapes, eye, muzzle, forelock, hip and shoulder lines.", "Hufformen, Auge, Maul, Stirnschopf, Hüft- und Schulterlinie."),
  merged=("Neck coat with trunk coat (same region, seams hidden).", "Halsfell mit Rumpffell (gleiche Region, Nähte verborgen)."),
  omitted=("Nostril, brow, mouth line, hoof lines and coat strokes (0.1 mm and below).", "Nüster, Braue, Maullinie, Hufstriche und Fellstriche (0,1 mm und kleiner)."),
  thinElements=("Cannon 0.085-0.105 m = 0.85-1.05 mm and pastern 0.075-0.085 m at true width.", "Röhrbein 0,085-0,105 m = 0,85-1,05 mm und Fessel 0,075-0,085 m in wahrer Breite."),
  hatching=("None.", "Keine."),
  styles=("0.25 mm body, 0.13 mm detail; mane and tail lighter than the coat.", "0,25 mm Körper, 0,13 mm Detail; Mähne und Schweif heller als das Fell."),
  overlays=("One region per shape.", "Eine Region je Form.")),
 50: dict(
  silhouette=("Full articulation at 46 × 19 mm: every segment, jaw, ears, mane in three pieces with forelock, four tail pieces with hair strokes, hooves with sole lines.", "Volle Gliederung bei 46 × 19 mm: jedes Segment, Unterkiefer, Ohren, Mähne in drei Stücken mit Stirnschopf, vier Schweifstücke mit Haarstrichen, Hufe mit Sohlenlinien."),
  retained=("Nostril, eye, brow, mouth line, stifle, knee and hock lines, hoof lines, three coat strokes and tail hair strokes.", "Nüster, Auge, Braue, Maullinie, Kniescheiben-, Vorderfußwurzel- und Sprunggelenkslinien, Hufstriche, drei Fellstriche und Schweifhaarstriche."),
  merged=("Neck coat with trunk coat (seams hidden by the engine).", "Halsfell mit Rumpffell (Nähte vom System verborgen)."),
  omitted=("Individual hairs, hoof wall rings, veins.", "Einzelne Haare, Hufwandringe, Venen."),
  thinElements=("Pastern 0.075-0.085 m = 1.5-1.7 mm, nostril line 0.015 m = 0.3 mm at true size.", "Fessel 0,075-0,085 m = 1,5-1,7 mm, Nüsternlinie 0,015 m = 0,3 mm in wahrer Größe."),
  hatching=("None; texture strokes of 0.1 mm show the lie of the coat.", "Keine; 0,1-mm-Texturstriche zeigen die Haarrichtung."),
  styles=("0.35 mm body, 0.18 mm detail, 0.1 mm texture.", "0,35 mm Körper, 0,18 mm Detail, 0,1 mm Textur."),
  overlays=("One region per shape; texture and detail strokes carry none.", "Eine Region je Form; Textur- und Detailstriche tragen keine.")),
}
DEC[200]["silhouette"] = (DEC[200]["silhouette"][0].replace("at 11.6 × 4.7 mm...", "at 11.6 × 4.7 mm with"), DEC[200]["silhouette"][1])
VR = {500: (350, 800), 200: (150, 350), 100: (75, 150), 50: (20, 75)}
def build_horse():
    reps = {str(k): rep_auth(k, VR[k], R.PENS[k], R.rep(k), DEC[k]) for k in (500, 200, 100, 50)}
    return build("horse", cfg, reps, 100, {"heightM": 2.37, "widthM": 2.91, "depthM": 0.62, "massKg": 550})
if __name__ == "__main__":
    a = build_horse(); save(a, "horse"); print("saved", len(json.dumps(a)))
