from build_common import *
import deer_rep as R
w = R.W; pat = pattern
PATS = [
 pat("rest", T_("Resting, nose breathing", "Ruhe, Nasenatmung"), 23, 0.22, (0.40, 0.05, 0.45, 0.10), "nose", "stahl-1967"),
 pat("walk", T_("Walking, nose breathing", "Gehen, Nasenatmung"), 40, 0.28, (0.42, 0.02, 0.50, 0.06), "nose", "stahl-1967"),
 pat("bound", T_("Bounding flight, deep fast breathing", "Fluchtsprünge, tiefe schnelle Atmung"), 120, 0.45, (0.5, 0.0, 0.5, 0.0), "nose", "stahl-1967"),
 pat("freeze", T_("Frozen alert, held and shallow", "Erstarrt, angehalten und flach"), 12, 0.15, (0.35, 0.20, 0.35, 0.10), "nose", "stahl-1967"),
]
FUR = lambda t, m="summer-coat": ("coat", T_("Coat layer", "Haarkleid"), "fur", m, t, 0.045, round(t*91000), "cena-monteith-1975")
regs = [
 region("coat-trunk", T_("Trunk and neck coat", "Rumpf- und Halsdecke"), "fur", "summer-coat", 0.59, 0.30, "cylinder", 0.5, [FUR(0.012)], 14000, "trunk", 0.66, "scholander-1950"),
 region("rump-patch", T_("White rump patch", "Weißer Spiegel"), "fur", "white-coat", 0.01, 0.12, "vertical-plate", 0.5, [FUR(0.012, "white-coat")], 14000, "trunk", 0.015, "scholander-1950"),
 region("coat-head", T_("Head coat", "Kopffell"), "fur", "summer-coat", 0.06, 0.12, "sphere", 0.6, [FUR(0.006)], 9000, "head", 0.07, "cena-monteith-1975"),
 region("coat-forelimbs", T_("Foreleg coat", "Fell der Vorderläufe"), "fur", "summer-coat", 0.12, 0.05, "cylinder", 0.3, [FUR(0.006)], 9000, "arm", 0.09, "cena-monteith-1975"),
 region("coat-hindlimbs", T_("Hindleg coat", "Fell der Hinterläufe"), "fur", "summer-coat", 0.15, 0.06, "cylinder", 0.3, [FUR(0.006)], 10000, "leg", 0.15, "cena-monteith-1975"),
 region("ears", T_("Ears", "Lauscher"), "fur", "summer-coat", 0.022, 0.08, "vertical-plate", 0.7, [FUR(0.003)], 3000, "head", 0.005, "cena-monteith-1975"),
 region("nose-skin", T_("Nose leather", "Nasenspiegel"), "skin", "bare-skin", 0.004, 0.03, "sphere", 0.5, [], 4000, "head", 0.003, "ashrae-2021"),
 region("eyes", T_("Eyes", "Augen"), "skin", "bare-skin", 0.001, 0.02, "sphere", 0.5, [], 2500, "head", 0.003, "ashrae-2021"),
 region("hooves", T_("Hooves", "Schalen"), "surface", "hoof-horn", 0.013, 0.04, "horizontal-plate-down", 0.0, [], 9000, "foot", 0.004, "ashrae-2021"),
]
SUN = {"coat-trunk": 0.5, "rump-patch": 0.5, "coat-head": 0.5, "coat-forelimbs": 0.25, "coat-hindlimbs": 0.25, "ears": 0.6, "nose-skin": 0.4, "eyes": 0.5, "hooves": 0.05}
def rule(p):
    i = p["id"]
    if i.startswith("walk"): return dict(met=3.4, breathing="walk")
    if i.startswith("bound"): return dict(met=6.5, breathing="bound")
    if i == "grazing": return dict(met=1.6, breathing="rest")
    if i == "alert-foreleg-raised": return dict(met=1.6, breathing="freeze")
    return dict(met=1.5, breathing="rest")
cfg = dict(world=w, regions=regs, sun=SUN, exposed={}, ground_tempC=18, emit_default={"plume-back": 1.0}, rule=rule, base_met=1.5,
  contact_map={"contact-hn": ("hooves", 0.2), "contact-hf": ("hooves", 0.2), "contact-fn": ("hooves", 0.2), "contact-ff": ("hooves", 0.2)},
  breath_ids=["breath-nose", "breath-heavy"], reset_met=True, drop_refs=["kleiber-1947", "gebremedhin-2001"], keep_refs=["spector-1956", "taylor-1970"],
  body=dict(species="deer", massKg=25, skinAreaM2=0.97, basalMetabolismW=37.9, metWM2=39.1, coreSetPointC=38.5, skinSetPointC=33.5, sweating=False, panting=True,
            maxSkinBloodFlowLM2H=45, minSkinBloodFlowLM2H=4, source="authored-estimate", reference="kleiber-1932"),
  respiration=dict(noseEmitter="breath-nose", exhaledTempC=31, exhaledRelativeHumidity=0.95, patterns=PATS,
                   motion=[{"bone": "thorax", "angleDeg": 0.7, "dy": 0.005}], source="authored-estimate", reference="schmidt-nielsen-1970"),
  emitters=[
   {"id": "breath-nose", "kind": "breath", "title": T_("Nostrils", "Nasenlöcher"), "anchor": "nose", "directionDeg": -25, "diameterM": 0.014, "source": "authored-estimate", "reference": "schmidt-nielsen-1970"},
   {"id": "plume-back", "kind": "plume", "title": T_("Warm-air plume above the back", "Warmluftfahne über dem Rücken"), "anchor": "back-plume", "directionDeg": 90, "diameterM": 0.3, "region": "coat-trunk", "source": "authored-estimate", "reference": "cena-monteith-1975"}],
  new_anchors=[emitter_anchor(w, "back-plume", "thorax", 0.3, 0.1, T_("Back above the trunk (plume source)", "Rücken über dem Rumpf (Fahnenquelle)"))],
  materials=None,
  clip_tracks={
   "walk-cycle": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 3.4}, {"t": 0.5, "v": 3.5}, {"t": 1.0, "v": 3.4}]}],
   "bound-cycle": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 6.2}, {"t": 0.45, "v": 6.8}, {"t": 0.9, "v": 6.2}]}],
   "alarm-freeze": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 2.4, "v": 3}, {"t": 4.4, "v": 0}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.6}, {"t": 2.1, "v": 1.6}, {"t": 2.6, "v": 2.0}, {"t": 7, "v": 1.6}]}]},
  assumptions=[
   T_("Skin area follows Meeh's law with k = 0.1135: 0.97 m² for 25 kg (Spector 1956); the nine regions sum to this area.", "Die Hautfläche folgt dem Meeh-Gesetz mit k = 0,1135: 0,97 m² für 25 kg (Spector 1956); die neun Regionen summieren sich zu dieser Fläche."),
   T_("Basal metabolism is Kleiber's 3.39 W per kg^0.75 = 37.9 W (verified coefficient); 1 met = 39.1 W/m². Standing 1.5, walking 3.4 and bounding 6.5 met follow the running relation of Taylor et al. (1970) with authored speeds (1.2 and 3 m/s); the multiples are authored estimates.", "Der Grundumsatz ist Kleibers 3,39 W je kg^0,75 = 37,9 W (verifizierter Koeffizient); 1 Met = 39,1 W/m². Stehen 1,5, Gehen 3,4 und Fluchtsprünge 6,5 Met folgen der Laufbeziehung von Taylor et al. (1970) mit angenommenen Geschwindigkeiten (1,2 und 3 m/s); die Vielfachen sind Schätzwerte."),
   T_("The summer coat is 12 mm on the trunk (effective conductivity 0.045 W/(m K)), 6 mm on head and legs, 3 mm on the ears; nose, eyes and hooves are bare. Authored estimates guided by Scholander et al. (1950) and Cena and Monteith (1975); the white rump patch is a separate region because its solar absorptivity differs.", "Das Sommerhaar ist am Rumpf 12 mm dick (effektive Leitfähigkeit 0,045 W/(m K)), an Kopf und Läufen 6 mm, an den Lauschern 3 mm; Nase, Augen und Schalen sind nackt. Schätzwerte nach Scholander et al. (1950) und Cena und Monteith (1975); der weiße Spiegel ist eine eigene Region, weil seine solare Absorption abweicht."),
   T_("Respiration of the roe deer is not covered by a verified source here: rate 23 per minute and tidal volume 0.22 L at rest follow the allometric relations of Stahl (1967) for 25 kg, bounding (120 per minute, 0.45 L) and the held-breath pattern are authored estimates.", "Für die Atmung des Rehs liegt hier keine verifizierte Quelle vor: Frequenz 23 pro Minute und Atemzugvolumen 0,22 L in Ruhe folgen den allometrischen Beziehungen von Stahl (1967) für 25 kg, Fluchtsprünge (120 pro Minute, 0,45 L) und das angehaltene Atemmuster sind Schätzwerte."),
  ],
  notes=T_("Side elevation of the near side of a roe doe walking to the right, head raised in the default pose; the far-side legs are separate chains drawn darker and behind.", "Seitenansicht der nahen Seite einer nach rechts gehenden Ricke, in der Standardpose mit erhobenem Kopf; die Läufe der fernen Seite sind eigene Ketten, dunkler und dahinter gezeichnet."))
DEC = {
 500: dict(
  silhouette=("One composed side silhouette of 2.9 × 2.2 mm: hindquarters and chest masses, a neck wedge, one head mass with a single ear mass, thigh and upper-arm masses; the four slender legs as 0.2 mm strokes.", "Eine zusammengesetzte Seitensilhouette von 2,9 × 2,2 mm: Hinterhand- und Brustmasse, ein Halskeil, eine Kopfmasse mit einer Lauschermasse, Oberschenkel- und Oberarmmassen; die vier schlanken Läufe als 0,2-mm-Striche."),
  retained=("The slender build: deep chest, long neck, long legs, raised head with the large ear.", "Der schlanke Körperbau: tiefe Brust, langer Hals, lange Läufe, erhobener Kopf mit dem großen Lauscher."),
  merged=("Shank, cannon and hoof into one stroke per leg; eye and nose leather into the head; the white rump patch into the hindquarter mass (listed as region).", "Unterschenkel, Röhrbein und Schale zu einem Strich je Lauf; Auge und Nasenspiegel in den Kopf; der weiße Spiegel in die Hinterhandmasse (als Region geführt)."),
  omitted=("Far ear, jaw, hoof outlines, neck/shoulder seams, all detail lines.", "Fernes Ohr, Unterkiefer, Schalenumrisse, Hals-/Schulternähte, alle Detaillinien."),
  thinElements=("Cannons (real 0.04-0.05 m = 0.08-0.1 mm) are 0.2 mm strokes = 0.1 m, i.e. twice the real width; this exceeds what the outline pen alone would add and is accepted because otherwise the legs vanish; the check reports them against the print profile.", "Röhrbeine (real 0,04-0,05 m = 0,08-0,1 mm) sind 0,2-mm-Striche = 0,1 m, also doppelt so breit wie real; das übersteigt, was der Konturstift allein zufügt, und wird akzeptiert, weil die Läufe sonst verschwinden; die Prüfung meldet sie gegen das Druckprofil."),
  hatching=("None.", "Keine."),
  styles=("Mid-grey masses with 0.13 mm outline; far legs lighter.", "Mittelgraue Massen mit 0,13-mm-Kontur; ferne Läufe heller."),
  overlays=("Merged masses carry the area-weighted values of their listed regions.", "Verschmolzene Massen tragen die flächengewichteten Werte ihrer aufgeführten Regionen.")),
 200: dict(
  silhouette=("Main masses separated at 7.3 × 5.4 mm: hindquarters, chest, neck, head with jaw, both ears, white rump patch, all legs in true-width segments with small hooves.", "Hauptmassen getrennt bei 7,3 × 5,4 mm: Hinterhand, Brust, Hals, Kopf mit Unterkiefer, beide Ohren, weißer Spiegel, alle Läufe in wahrbreiten Segmenten mit kleinen Schalen."),
  retained=("Rump patch, both ears, hooves, eye dot.", "Spiegel, beide Ohren, Schalen, Augenpunkt."),
  merged=("Nose leather into the head mass; thigh seams hidden.", "Nasenspiegel in die Kopfmasse; Oberschenkelnähte verborgen."),
  omitted=("Brow, mouth line, hoof clefts, coat strokes, construction lines.", "Braue, Maullinie, Schalenspalt, Fellstriche, Konstruktionslinien."),
  thinElements=("Cannons printed at true width 0.04-0.05 m = 0.2-0.25 mm, at the profile minimum; they are the first to check.", "Röhrbeine in wahrer Breite 0,04-0,05 m = 0,2-0,25 mm, am Profilminimum; sie sind als Erste zu prüfen."),
  hatching=("None.", "Keine."),
  styles=("0.18 mm body outline, 0.1 mm eye dot.", "0,18-mm-Körperkontur, 0,1-mm-Augenpunkt."),
  overlays=("One region per shape; nose leather is carried by the head mass without its own colour.", "Eine Region je Form; der Nasenspiegel wird von der Kopfmasse getragen.")),
 100: dict(
  silhouette=("Articulated outline 14.7 × 10.8 mm: every segment, the white rump patch, ears, jaw and hooves at true width.", "Gegliederter Umriss 14,7 × 10,8 mm: jedes Segment, der weiße Spiegel, Ohren, Unterkiefer und Schalen in wahrer Breite."),
  retained=("Eye, nose leather, shoulder and hip lines.", "Auge, Nasenspiegel, Schulter- und Hüftlinie."),
  merged=("Neck with trunk coat (same region).", "Hals mit Rumpfdecke (gleiche Region)."),
  omitted=("Hoof clefts, brow, mouth line, coat strokes.", "Schalenspalt, Braue, Maullinie, Fellstriche."),
  thinElements=("Cannons 0.038-0.05 m = 0.38-0.5 mm at true width.", "Röhrbeine 0,038-0,05 m = 0,38-0,5 mm in wahrer Breite."),
  hatching=("None.", "Keine."),
  styles=("0.25 mm body, 0.13 mm detail; rump patch light.", "0,25 mm Körper, 0,13 mm Detail; Spiegel hell."),
  overlays=("One region per shape.", "Eine Region je Form.")),
 50: dict(
  silhouette=("Full articulation at 29 × 22 mm: all of 1:100 plus brow, mouth line, inner ear, hoof clefts and coat strokes.", "Volle Gliederung bei 29 × 22 mm: alles von 1:100 plus Braue, Maullinie, Innenohr, Schalenspalt und Fellstriche."),
  retained=("Hoof clefts, inner ear, brow, mouth line, three coat strokes.", "Schalenspalt, Innenohr, Braue, Maullinie, drei Fellstriche."),
  merged=("Neck with trunk coat (same region).", "Hals mit Rumpfdecke (gleiche Region)."),
  omitted=("Individual hairs, dew claws, lashes.", "Einzelne Haare, Afterklauen, Wimpern."),
  thinElements=("Cannons 0.038 m = 0.76 mm at true width.", "Röhrbeine 0,038 m = 0,76 mm in wahrer Breite."),
  hatching=("None; 0.1 mm texture strokes show the coat.", "Keine; 0,1-mm-Texturstriche zeigen das Haarkleid."),
  styles=("0.35 mm body, 0.18 mm detail, 0.1 mm texture.", "0,35 mm Körper, 0,18 mm Detail, 0,1 mm Textur."),
  overlays=("One region per shape; strokes carry none.", "Eine Region je Form; Striche tragen keine.")),
}
VR = {500: (350, 800), 200: (150, 350), 100: (75, 150), 50: (20, 75)}
def build_deer():
    reps = {str(k): rep_auth(k, VR[k], R.PENS[k], R.rep(k), DEC[k]) for k in (500, 200, 100, 50)}
    return build("roe-deer", cfg, reps, 100, {"heightM": 1.08, "widthM": 1.46, "depthM": 0.2, "massKg": 25})
if __name__ == "__main__":
    a = build_deer(); save(a, "roe-deer"); print("saved", len(json.dumps(a)))
