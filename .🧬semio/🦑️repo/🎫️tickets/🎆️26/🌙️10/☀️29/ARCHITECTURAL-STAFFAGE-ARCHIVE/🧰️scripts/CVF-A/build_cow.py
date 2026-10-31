from build_common import *
import cow_rep as R
w = R.W; pat = pattern
PATS = [
 pat("rest", T_("Standing, ruminating", "Stehen, Wiederkäuen"), 30, 3.5, (0.40, 0.04, 0.46, 0.10), "nose", "merck-resting-respiratory-rates"),
 pat("walk", T_("Walking", "Gehen"), 45, 4.0, (0.42, 0.02, 0.50, 0.06), "nose", "stahl-1967"),
 pat("graze", T_("Grazing", "Grasen"), 34, 3.5, (0.40, 0.04, 0.46, 0.10), "nose", "merck-resting-respiratory-rates"),
 pat("lying", T_("Lying, ruminating", "Liegen, Wiederkäuen"), 28, 3.5, (0.40, 0.04, 0.46, 0.10), "nose", "merck-resting-respiratory-rates"),
]
FUR = lambda t, m: ("coat", T_("Coat layer", "Haarkleid"), "fur", m, t, 0.045, round(t*91000), "cena-monteith-1975")
regs = [
 region("coat-trunk", T_("White trunk coat", "Weißes Rumpffell"), "fur", "white-hair", 1.17, 0.6, "cylinder", 0.5, [FUR(0.008, "white-hair")], 40000, "trunk", 0.22, "scholander-1950"),
 region("coat-patches", T_("Black coat patches", "Schwarze Fellflecken"), "fur", "black-hair", 1.30, 0.6, "cylinder", 0.5, [FUR(0.008, "black-hair")], 40000, "trunk", 0.25, "scholander-1950"),
 region("flank-rumen", T_("Rumen flank", "Pansenflanke"), "fur", "white-hair", 0.45, 0.4, "vertical-plate", 0.5, [FUR(0.008, "white-hair")], 40000, "trunk", 0.17, "cigr-2002"),
 region("coat-head-neck", T_("Head and neck coat", "Kopf- und Halsfell"), "fur", "black-hair", 0.45, 0.25, "cylinder", 0.6, [FUR(0.006, "black-hair")], 20000, "head", 0.07, "cena-monteith-1975"),
 region("coat-forelimbs", T_("Foreleg coat", "Fell der Vorderbeine"), "fur", "white-hair", 0.38, 0.12, "cylinder", 0.3, [FUR(0.005, "white-hair")], 16000, "arm", 0.06, "cena-monteith-1975"),
 region("coat-hindlimbs", T_("Hindleg coat", "Fell der Hinterbeine"), "fur", "white-hair", 0.57, 0.15, "cylinder", 0.3, [FUR(0.005, "white-hair")], 18000, "leg", 0.10, "cena-monteith-1975"),
 region("lower-legs", T_("Lower legs", "Unterschenkel"), "skin", "bare-skin", 0.30, 0.09, "cylinder", 0.2, [FUR(0.003, "white-hair")], 9000, "foot", 0.02, "cena-monteith-1975"),
 region("hooves", T_("Hooves", "Klauen"), "skin", "claw-horn", 0.05, 0.08, "horizontal-plate-down", 0.0, [], 12000, "foot", 0.005, "ashrae-2021"),
 region("udder", T_("Udder", "Euter"), "skin", "bare-skin", 0.45, 0.25, "sphere", 0.1, [FUR(0.002, "white-hair")], 22000, "trunk", 0.08, "kadzere-2002"),
 region("muzzle", T_("Muzzle", "Flotzmaul"), "skin", "bare-skin", 0.03, 0.1, "sphere", 0.5, [], 6000, "head", 0.01, "ashrae-2021"),
 region("eyes", T_("Eyes", "Augen"), "skin", "bare-skin", 0.002, 0.04, "sphere", 0.5, [], 3000, "head", 0.002, "ashrae-2021"),
 region("ears", T_("Ears", "Ohren"), "fur", "black-hair", 0.07, 0.15, "vertical-plate", 0.7, [FUR(0.003, "black-hair")], 3500, "head", 0.005, "cena-monteith-1975"),
 region("coat-tail", T_("Tail coat", "Schwanzfell"), "fur", "black-hair", 0.08, 0.06, "cylinder", 0.5, [FUR(0.008, "black-hair")], 8000, "tail", 0.008, "cena-monteith-1975"),
]
for r in regs:
    if r["id"] in ("lower-legs", "udder"):
        for l in r["layers"]: l["kind"] = "fur"
SUN = {"coat-trunk": 0.5, "coat-patches": 0.5, "flank-rumen": 0.5, "coat-head-neck": 0.5, "coat-forelimbs": 0.25, "coat-hindlimbs": 0.25, "lower-legs": 0.2, "hooves": 0.05, "udder": 0.0, "muzzle": 0.4, "eyes": 0.5, "ears": 0.6, "coat-tail": 0.3}
def rule(p):
    i = p["id"]
    if i.startswith("walk"): return dict(met=4.0, breathing="walk")
    if i == "grazing-head-down": return dict(met=2.4, breathing="graze")
    if i == "lying-resting": return dict(met=1.8, breathing="lying", exposed={"udder": 0.4}, sun=0.9)
    if i == "kneeling-front": return dict(met=2.3, breathing="rest")
    if i == "lowering-hindquarters": return dict(met=2.3, breathing="rest")
    return dict(met=2.1, breathing="rest")
cfg = dict(world=w, regions=regs, sun=SUN, exposed={}, ground_tempC=18, emit_default={"plume-back": 1.0}, rule=rule, base_met=2.1,
  contact_map={"contact-hn": ("hooves", 0.2), "contact-hf": ("hooves", 0.2), "contact-fn": ("hooves", 0.2), "contact-ff": ("hooves", 0.2),
               "contact-sternum": ("coat-trunk", 0.12), "contact-carpus-fn": ("coat-forelimbs", 0.08), "contact-carpus-ff": ("coat-forelimbs", 0.08),
               "contact-hock-hn": ("coat-hindlimbs", 0.08), "contact-hock-hf": ("coat-hindlimbs", 0.08)},
  breath_ids=["breath-nose"], reset_met=True, drop_refs=["kleiber-1947", "gebremedhin-2001"], keep_refs=[],
  body=dict(species="cow", massKg=650, skinAreaM2=5.30, basalMetabolismW=437, metWM2=82.5, coreSetPointC=38.6, skinSetPointC=33.5, sweating=True, panting=True,
            maxSkinBloodFlowLM2H=55, minSkinBloodFlowLM2H=4, source="authored-estimate", reference="kleiber-1932"),
  respiration=dict(noseEmitter="breath-nose", exhaledTempC=32, exhaledRelativeHumidity=0.95, patterns=PATS,
                   motion=[{"bone": "thorax", "angleDeg": 0.5, "dy": 0.012}], source="authored-estimate", reference="schmidt-nielsen-1970"),
  emitters=[
   {"id": "breath-nose", "kind": "breath", "title": T_("Nostrils", "Nüstern"), "anchor": "nose", "directionDeg": -30, "diameterM": 0.025, "source": "authored-estimate", "reference": "schmidt-nielsen-1970"},
   {"id": "plume-back", "kind": "plume", "title": T_("Warm-air plume above the back", "Warmluftfahne über dem Rücken"), "anchor": "back-plume", "directionDeg": 90, "diameterM": 0.8, "region": "coat-trunk", "source": "authored-estimate", "reference": "cigr-2002"}],
  new_anchors=[emitter_anchor(w, "back-plume", "thorax", 0.3, 0.33, T_("Back above the trunk (plume source)", "Rücken über dem Rumpf (Fahnenquelle)"))],
  clip_tracks={
   "walk-cycle": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 4.0}, {"t": 0.7, "v": 4.1}, {"t": 1.4, "v": 4.0}]}],
   "cud-chewing": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 2.1}, {"t": 5, "v": 2.1}]}],
   "lie-down": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 9, "v": 3}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 2.1}, {"t": 3.6, "v": 2.55}, {"t": 6.4, "v": 2.4}, {"t": 9, "v": 1.8}, {"t": 11, "v": 1.8}]}],
   "graze-and-chew": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 2.2, "v": 2}, {"t": 10, "v": 0}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 2.1}, {"t": 2.2, "v": 2.4}, {"t": 8, "v": 2.4}, {"t": 12, "v": 2.1}]}]},
  assumptions=[
   T_("Skin area follows Meeh's law with an authored k = 0.0706 (Meeh 1879; Berman 2003 for dairy cattle): 5.30 m² for 650 kg; the thirteen regions sum to this area.", "Die Hautfläche folgt dem Meeh-Gesetz mit einem gesetzten k = 0,0706 (Meeh 1879; Berman 2003 für Milchrinder): 5,30 m² für 650 kg; die dreizehn Regionen summieren sich zu dieser Fläche."),
   T_("Basal metabolism is Kleiber's 3.39 W per kg^0.75 = 437 W (verified coefficient); 1 met = 82.5 W/m². A lactating cow produces about twice this at rest (standing and ruminating 2.1 met = 910 W, guided by CIGR 2002 and NRC 2001), walking 4.0 met; all multiples are authored estimates.", "Der Grundumsatz ist Kleibers 3,39 W je kg^0,75 = 437 W (verifizierter Koeffizient); 1 Met = 82,5 W/m². Eine laktierende Kuh erzeugt in Ruhe etwa das Doppelte (Stehen und Wiederkäuen 2,1 Met = 910 W, Anhalt CIGR 2002 und NRC 2001), Gehen 4,0 Met; alle Vielfachen sind Schätzwerte."),
   T_("Coat layers are thickness over an effective conductivity of 0.045 W/(m K): 8 mm on trunk, rumen flank and tail, 5-6 mm on head and legs, 2-3 mm on lower legs, udder and ears; muzzle, eyes and hooves are bare. White and black patches differ only through their materials (solar absorptivity). Authored estimates guided by Scholander et al. (1950) and Cena and Monteith (1975). The udder is shaded and carries the largest unshielded area.", "Haarkleidschichten sind Dicke durch eine effektive Leitfähigkeit von 0,045 W/(m K): 8 mm an Rumpf, Pansenflanke und Schwanz, 5-6 mm an Kopf und Beinen, 2-3 mm an Unterschenkeln, Euter und Ohren; Flotzmaul, Augen und Klauen sind nackt. Weiße und schwarze Flecken unterscheiden sich nur über ihre Materialien (solare Absorption). Schätzwerte nach Scholander et al. (1950) und Cena und Monteith (1975). Das Euter liegt im Schatten und trägt die größte ungeschützte Fläche."),
   T_("Resting rate 26-50 per minute is the verified range for dairy cows (Merck Veterinary Manual after Reece 2004): 30 standing, 34 grazing, 28 lying per minute are inside it; tidal volume 3.5-4 L and the walking pattern are authored estimates (Kadzere et al. 2002 gives heat-stress rates but is not used for values). Cattle sweat and pant under heat load; both are enabled for the body model.", "Die Ruhefrequenz von 26-50 pro Minute ist der verifizierte Bereich für Milchkühe (Merck Veterinary Manual nach Reece 2004): 30 im Stehen, 34 beim Grasen, 28 im Liegen liegen darin; Atemzugvolumen 3,5-4 L und das Gehmuster sind Schätzwerte (Kadzere et al. 2002 nennt Hitzestressfrequenzen, liefert aber keine verwendeten Werte). Rinder schwitzen und hecheln unter Hitzelast; beides ist für das Körpermodell aktiviert."),
  ],
  notes=T_("Side elevation of the near side of a Holstein-Friesian cow walking to the right, head lowered in the default pose; far-side legs are separate chains drawn behind. Four preset scales are individually drawn.", "Seitenansicht der nahen Seite einer nach rechts gehenden Holstein-Friesian-Kuh, in der Standardpose mit gesenktem Kopf; die Beine der fernen Seite sind eigene Ketten dahinter. Vier Maßstäbe sind einzeln gezeichnet."))
D2 = lambda *a: a
DEC = {
 500: dict(
  silhouette=("One composed side silhouette of 5.1 × 3.0 mm: hindquarters and chest masses, a neck mass, one head mass, the udder as a small mass, thigh and upper-arm masses; lower legs and tail as strokes. Black-and-white patches are not drawn.", "Eine zusammengesetzte Seitensilhouette von 5,1 × 3,0 mm: Hinterhand- und Brustmasse, eine Halsmasse, eine Kopfmasse, das Euter als kleine Masse, Oberschenkel- und Oberarmmassen; Unterläufe und Schwanz als Striche. Schwarz-weiße Flecken werden nicht gezeichnet."),
  retained=("The deep barrel, the udder, the lowered head and the angular hind leg.", "Der tiefe Rumpf, das Euter, der gesenkte Kopf und das gewinkelte Hinterbein."),
  merged=("Coat, patches and rumen flank into the trunk masses (three regions listed); ears, muzzle and eye into the head mass; gaskin, cannon and hoof into one stroke per leg.", "Fell, Flecken und Pansenflanke in die Rumpfmassen (drei Regionen geführt); Ohren, Flotzmaul und Auge in die Kopfmasse; Unterschenkel, Röhrbein und Klaue zu einem Strich je Bein."),
  omitted=("Patches, teats, switch, jaw, hoof outlines, all detail lines.", "Flecken, Zitzen, Schwanzquaste, Unterkiefer, Klauenumrisse, alle Detaillinien."),
  thinElements=("Cannons (0.08-0.10 m = 0.16-0.2 mm) are 0.25 mm strokes = 0.125 m wide, up to 0.045 m wider than real; the tail (0.05 m) is a 0.2 mm stroke = 0.1 m.", "Röhrbeine (0,08-0,10 m = 0,16-0,2 mm) sind 0,25-mm-Striche = 0,125 m breit, bis zu 0,045 m breiter als real; der Schwanz (0,05 m) ist ein 0,2-mm-Strich = 0,1 m."),
  hatching=("None.", "Keine."),
  styles=("Light-grey masses (the white coat dominates) with a 0.13 mm outline; head and far legs darker.", "Hellgraue Massen (das weiße Fell überwiegt) mit 0,13-mm-Kontur; Kopf und ferne Beine dunkler."),
  overlays=("Trunk masses carry the area-weighted value of trunk coat, black patches and rumen flank; udder, tail and head are separate.", "Rumpfmassen tragen den flächengewichteten Wert von Rumpffell, schwarzen Flecken und Pansenflanke; Euter, Schwanz und Kopf sind getrennt.")),
 200: dict(
  silhouette=("Main masses separated at 10.2 × 3.0 mm... ", "Hauptmassen getrennt bei 10,2 × 3,0 mm... "),
  retained=("Three dark patch masses, the udder with the hind leg, switch, ears, hooves.", "Drei dunkle Fleckenmassen, das Euter mit dem Hinterbein, Quaste, Ohren, Klauen."),
  merged=("Neck with the head-neck coat; seams of same-coat segments hidden.", "Hals mit dem Kopf-Hals-Fell; Nähte gleicher Fellsegmente verborgen."),
  omitted=("Teats, blaze, brow, mouth line, hoof clefts and coat strokes.", "Zitzen, Blesse, Braue, Maullinie, Klauenspalt und Fellstriche."),
  thinElements=("Cannons printed at true width 0.08-0.10 m = 0.4-0.5 mm; tail 0.04-0.07 m = 0.2-0.35 mm at true width.", "Röhrbeine in wahrer Breite 0,08-0,10 m = 0,4-0,5 mm; Schwanz 0,04-0,07 m = 0,2-0,35 mm in wahrer Breite."),
  hatching=("None; patches are solid dark masses.", "Keine; Flecken sind massive dunkle Flächen."),
  styles=("White coat as light tone, patches and head dark, 0.18 mm outline.", "Weißes Fell als heller Ton, Flecken und Kopf dunkel, 0,18-mm-Kontur."),
  overlays=("Patches map to the coat-patches region, the pale trunk to coat-trunk, the rumen disc to flank-rumen.", "Flecken bilden die Region der schwarzen Flecken ab, der helle Rumpf das weiße Rumpffell, die Pansenscheibe die Pansenflanke.")),
 100: dict(
  silhouette=("Articulated outline 25.5 × 15 mm: all segments at true width, patches, teats, rumen disc, blaze, both ears, jaw and hooves.", "Gegliederter Umriss 25,5 × 15 mm: alle Segmente in wahrer Breite, Flecken, Zitzen, Pansenscheibe, Blesse, beide Ohren, Unterkiefer und Klauen."),
  retained=("Teats, blaze, rumen disc, hip line.", "Zitzen, Blesse, Pansenscheibe, Hüftlinie."),
  merged=("Neck with the head-neck coat (same region).", "Hals mit dem Kopf-Hals-Fell (gleiche Region)."),
  omitted=("Nostril, mouth line, hoof clefts, hair strokes.", "Nüster, Maullinie, Klauenspalt, Haarstriche."),
  thinElements=("Teats 0.026-0.034 m wide = 0.26-0.34 mm at true width; tail 0.04 m = 0.4 mm.", "Zitzen 0,026-0,034 m breit = 0,26-0,34 mm in wahrer Breite; Schwanz 0,04 m = 0,4 mm."),
  hatching=("None.", "Keine."),
  styles=("0.25 mm body, 0.13 mm detail; patches dark.", "0,25 mm Körper, 0,13 mm Detail; Flecken dunkel."),
  overlays=("One region per shape.", "Eine Region je Form.")),
 50: dict(
  silhouette=("Full articulation at 51 × 30 mm: all of 1:100 plus nostril, mouth line, hoof clefts, knee line and hair strokes.", "Volle Gliederung bei 51 × 30 mm: alles von 1:100 plus Nüster, Maullinie, Klauenspalt, Karpallinie und Haarstriche."),
  retained=("Hoof clefts, knee line, nostril, mouth line, neck fold and hair strokes.", "Klauenspalt, Karpallinie, Nüster, Maullinie, Halsfalte und Haarstriche."),
  merged=("Neck with the head-neck coat (same region).", "Hals mit dem Kopf-Hals-Fell (gleiche Region)."),
  omitted=("Individual hairs, udder veins, ear tags.", "Einzelne Haare, Euteradern, Ohrmarken."),
  thinElements=("Teats 0.026 m = 0.52 mm and nostril line 0.02 m at true size.", "Zitzen 0,026 m = 0,52 mm und Nüsternlinie 0,02 m in wahrer Größe."),
  hatching=("None; 0.1 mm texture strokes show the hair direction.", "Keine; 0,1-mm-Texturstriche zeigen die Haarrichtung."),
  styles=("0.35 mm body, 0.18 mm detail, 0.1 mm texture.", "0,35 mm Körper, 0,18 mm Detail, 0,1 mm Textur."),
  overlays=("One region per shape; strokes carry none.", "Eine Region je Form; Striche tragen keine.")),
}
DEC[200]["silhouette"] = ("Main masses separated at 10.2 × 3.0 mm: hindquarters, chest, neck, head with jaw, both ears, three patches, rumen disc, udder, tail in three pieces with switch, every leg in true-width segments with hooves.", "Hauptmassen getrennt bei 10,2 × 3,0 mm: Hinterhand, Brust, Hals, Kopf mit Unterkiefer, beide Ohren, drei Flecken, Pansenscheibe, Euter, Schwanz in drei Stücken mit Quaste, jedes Bein in wahrbreiten Segmenten mit Klauen.")
DEC[200]["silhouette"] = (DEC[200]["silhouette"][0].replace("10.2 × 3.0", "10.2 × 7.5"), DEC[200]["silhouette"][1].replace("10,2 × 3,0", "10,2 × 7,5"))
DEC[500]["silhouette"] = (DEC[500]["silhouette"][0].replace("5.1 × 3.0", "5.1 × 3.0"), DEC[500]["silhouette"][1])
VR = {500: (350, 800), 200: (150, 350), 100: (75, 150), 50: (20, 75)}
def build_cow():
    HAT = {50: [{"id": "hair", "title": T_("Switch hair", "Quastenhaar"), "pattern": "lines", "angleDeg": 82, "spacingMm": 0.45, "penMm": 0.1}]}
    reps = {str(k): rep_auth(k, VR[k], R.PENS[k], R.rep(k), DEC[k], HAT.get(k, ())) for k in (500, 200, 100, 50)}
    return build("dairy-cow", cfg, reps, 100, {"heightM": 1.49, "widthM": 2.55, "depthM": 0.72, "massKg": 650})
if __name__ == "__main__":
    a = build_cow(); save(a, "dairy-cow"); print("saved", len(json.dumps(a)))
