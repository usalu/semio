from build_common import *
import dog_rep as R
w = R.W; A = R.ASSET
pat = pattern
PATS = [
 pat("rest", T_("Resting, nose breathing", "Ruhe, Nasenatmung"), 24, 0.30, (0.40, 0.05, 0.45, 0.10), "nose", "merck-resting-respiratory-rates"),
 pat("walk", T_("Walking, nose breathing", "Gehen, Nasenatmung"), 32, 0.40, (0.42, 0.02, 0.50, 0.06), "nose", "stahl-1967"),
 pat("pant", T_("Panting through the mouth", "Hecheln durch das Maul"), 270, 0.11, (0.5, 0.0, 0.5, 0.0), "mouth", "crawford-1962"),
 pat("sniff", T_("Sniffing bursts", "Schnüffelstöße"), 300, 0.03, (0.5, 0.0, 0.5, 0.0), "nose", "craven-2010"),
]
FUR = lambda t: ("fur", T_("Fur layer", "Fellschicht"), "fur", "short-fur", t, 0.045, round(t*91000), "cena-monteith-1975")
regs = [
 region("coat-trunk", T_("Trunk and neck coat", "Rumpf- und Halsfell"), "fur", "short-fur", 0.544, 0.28, "cylinder", 0.5, [FUR(0.012)], 22000, "trunk", 0.66, "scholander-1950"),
 region("coat-head", T_("Head coat", "Kopffell"), "fur", "short-fur", 0.07, 0.12, "sphere", 0.6, [FUR(0.006)], 12000, "head", 0.07, "cena-monteith-1975"),
 region("ears", T_("Ears", "Ohren"), "fur", "short-fur", 0.03, 0.09, "vertical-plate", 0.7, [FUR(0.003)], 3000, "head", 0.005, "cena-monteith-1975"),
 region("coat-forelimbs", T_("Foreleg coat", "Fell der Vorderbeine"), "fur", "short-fur", 0.16, 0.06, "cylinder", 0.3, [FUR(0.006)], 12000, "arm", 0.08, "cena-monteith-1975"),
 region("coat-hindlimbs", T_("Hindleg coat", "Fell der Hinterbeine"), "fur", "short-fur", 0.20, 0.08, "cylinder", 0.3, [FUR(0.008)], 14000, "leg", 0.15, "cena-monteith-1975"),
 region("coat-tail", T_("Tail coat", "Rutenfell"), "fur", "short-fur", 0.04, 0.06, "cylinder", 0.5, [FUR(0.010)], 8000, "tail", 0.01, "cena-monteith-1975"),
 region("nose-skin", T_("Nose leather", "Nasenspiegel"), "skin", "bare-skin", 0.004, 0.03, "sphere", 0.5, [], 4000, "head", 0.001, "ashrae-2021"),
 region("tongue", T_("Tongue", "Zunge"), "skin", "tongue-mucosa", 0.012, 0.04, "horizontal-plate-down", 0.4, [], 6000, "head", 0.014, "goldberg-1981"),
 region("pads", T_("Paw pads", "Pfotenballen"), "skin", "bare-skin", 0.02, 0.05, "horizontal-plate-down", 0.0, [], 10000, "foot", 0.01, "ashrae-2021"),
]
SUN = {"coat-trunk": 0.5, "coat-head": 0.5, "ears": 0.6, "coat-forelimbs": 0.25, "coat-hindlimbs": 0.25, "coat-tail": 0.4, "nose-skin": 0.4, "tongue": 0.4, "pads": 0.05}
def rule(p):
    i = p["id"]
    if i.startswith("walk"): return dict(met=3.4, breathing="walk", exposed={"tongue": 0.3})
    if i.startswith("trot"): return dict(met=6.7, breathing="pant", exposed={"tongue": 1.0})
    if i == "sniffing-head-down": return dict(met=1.7, breathing="sniff")
    if i == "sitting-waiting": return dict(met=1.2, breathing="rest")
    return dict(met=1.3, breathing="rest")
BIND = {"IDX": {"rest": 0, "walk": 1, "pant": 2, "sniff": 3}}
cfg = dict(world=w, regions=regs, sun=SUN, exposed={"tongue": 0.05}, ground_tempC=18, emit_default={"plume-back": 1.0}, rule=rule, base_met=1.3,
  contact_map={"contact-hn": ("pads", 0.15), "contact-hf": ("pads", 0.15), "contact-fn": ("pads", 0.15), "contact-ff": ("pads", 0.15), "contact-rump": ("coat-hindlimbs", 0.06)},
  breath_ids=["breath-nose", "sniff", "pant-mouth"], reset_met=True,
  drop_refs=["kleiber-1947"], keep_refs=["taylor-1970", "spector-1956"],
  body=dict(species="dog", massKg=30, skinAreaM2=1.08, basalMetabolismW=43.5, metWM2=40.3, coreSetPointC=38.5, skinSetPointC=35.0, sweating=False, panting=True,
            maxSkinBloodFlowLM2H=50, minSkinBloodFlowLM2H=4, source="authored-estimate", reference="kleiber-1932"),
  respiration=dict(noseEmitter="breath-nose", mouthEmitter="breath-mouth", exhaledTempC=32, exhaledRelativeHumidity=0.95, patterns=PATS,
                   motion=[{"bone": "thorax", "angleDeg": 0.8, "dy": 0.004}], source="authored-estimate", reference="schmidt-nielsen-1970"),
  emitters=[
   {"id": "breath-nose", "kind": "breath", "title": T_("Nostrils", "Nasenlöcher"), "anchor": "nose", "directionDeg": -50, "diameterM": 0.012, "source": "authored-estimate", "reference": "craven-2010"},
   {"id": "breath-mouth", "kind": "breath", "title": T_("Open mouth (panting)", "Offenes Maul (Hecheln)"), "anchor": "mouth", "directionDeg": -15, "diameterM": 0.03, "source": "authored-estimate", "reference": "goldberg-1981"},
   {"id": "plume-back", "kind": "plume", "title": T_("Warm-air plume above the back", "Warmluftfahne über dem Rücken"), "anchor": "back-plume", "directionDeg": 90, "diameterM": 0.3, "region": "coat-trunk", "source": "authored-estimate", "reference": "cena-monteith-1975"}],
  new_anchors=[emitter_anchor(w, "back-plume", "thorax", 0.43, 0.60, T_("Back above the trunk (plume source)", "Rücken über dem Rumpf (Fahnenquelle)"))],
  clip_tracks={
   "sniff-ground": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 1.2, "v": 3}, {"t": 2.8, "v": 0}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.3}, {"t": 1.5, "v": 1.7}, {"t": 4, "v": 1.3}]}],
   "sit-and-pant": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 3.4, "v": 2}, {"t": 5.4, "v": 0}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.3}, {"t": 1.5, "v": 1.2}, {"t": 3.6, "v": 1.7}, {"t": 5, "v": 1.7}, {"t": 6, "v": 1.3}]}],
   "walk-cycle": [], "trot-cycle": []},
  assumptions=[
   T_("Skin area follows Meeh's law A = 0.112·M^(2/3) = 1.08 m² for 30 kg (Spector 1956); the nine regions sum to this area.", "Die Hautfläche folgt dem Meeh-Gesetz A = 0,112·M^(2/3) = 1,08 m² für 30 kg (Spector 1956); die neun Regionen summieren sich zu dieser Fläche."),
   T_("Basal metabolism is Kleiber's 3.39 W per kg^0.75 = 43.5 W (verified literature coefficient); one met of this asset is that basal rate per skin area (40.3 W/m²). Standing 1.3, walking 3.4 and trotting 6.7 met follow the cost of running 0.53·M^-0.316 ml O2 per g and km after Taylor et al. (1970) at 1.2 and 2.5 m/s; these multiples are authored estimates.", "Der Grundumsatz ist Kleibers 3,39 W pro kg^0,75 = 43,5 W (verifizierter Literaturkoeffizient); ein Met dieses Assets ist dieser Grundumsatz je Hautfläche (40,3 W/m²). Stehen 1,3, Gehen 3,4 und Traben 6,7 Met folgen dem Laufkostenansatz 0,53·M^-0,316 ml O2 je g und km nach Taylor et al. (1970) bei 1,2 und 2,5 m/s; diese Vielfachen sind Schätzwerte."),
   T_("Fur layers are thickness over an effective conductivity of 0.045 W/(m K): 12 mm on the trunk, 6-10 mm on head, limbs and tail, 3 mm on the ears; legs, ears and bare skin are thin, the trunk is thick. Thicknesses are authored estimates guided by Scholander et al. (1950) and Cena and Monteith (1975); no numbers of those papers are used.", "Fellschichten sind Dicke durch eine effektive Leitfähigkeit von 0,045 W/(m K): 12 mm am Rumpf, 6-10 mm an Kopf, Läufen und Rute, 3 mm an den Ohren; Läufe, Ohren und nackte Haut sind dünn, der Rumpf ist dick. Die Dicken sind Schätzwerte nach Scholander et al. (1950) und Cena und Monteith (1975); Zahlenwerte dieser Arbeiten werden nicht übernommen."),
   T_("Resting rate 24 per minute lies inside the verified resting range of dogs (18-34, Merck Veterinary Manual after Reece 2004); tidal volumes, panting (270 per minute, 0.11 L) and sniffing (5 Hz) are authored estimates guided by Crawford (1962) and Craven et al. (2010). Nose breath leaves at 32 °C because the nasal passages recover heat (Schmidt-Nielsen et al. 1970); one exhaled state serves all patterns.", "Die Ruhefrequenz von 24 pro Minute liegt im verifizierten Ruhebereich von Hunden (18-34, Merck Veterinary Manual nach Reece 2004); Atemzugvolumina, Hecheln (270 pro Minute, 0,11 L) und Schnüffeln (5 Hz) sind Schätzwerte nach Crawford (1962) und Craven et al. (2010). Die Nasenluft verlässt den Körper mit 32 °C, weil die Nasengänge Wärme zurückgewinnen (Schmidt-Nielsen et al. 1970); ein Ausatemzustand gilt für alle Muster."),
   T_("Panting heat loss is carried by the moist tongue, exposed 5 % with a closed mouth, 30 % when walking and fully when trotting; the tongue draws its heat from the body model like any region.", "Der Hecheln-Wärmeverlust läuft über die feuchte Zunge, die bei geschlossenem Maul zu 5 %, beim Gehen zu 30 % und im Trab vollständig frei liegt; die Zunge bezieht ihre Wärme wie jede Region aus dem Körpermodell."),
  ],
  notes=T_("Side elevation of the near (left) side of a dog walking to the right; far-side limbs, ear and tail root are separate chains drawn darker and behind. Four preset scales are individually drawn (see the representations); the default pose bounds include the raised head. Thermal and respiratory data follow the verified Kleiber coefficient and Merck resting rates; all other physiological numbers are authored estimates.", "Seitenansicht der nahen (linken) Seite eines nach rechts gehenden Hundes; Gliedmaßen, Ohr und Rutenansatz der fernen Seite sind eigene Ketten, dunkler und dahinter gezeichnet. Vier Maßstäbe sind einzeln gezeichnet (siehe Darstellungen); die Maße der Standardpose enthalten den erhobenen Kopf. Wärme- und Atemdaten folgen dem verifizierten Kleiber-Koeffizienten und den Merck-Ruhefrequenzen; alle übrigen physiologischen Zahlen sind Schätzwerte."))

PENS = {"500": dict(body=0.13, detail=None, texture=None, glazing=None, ground=0.18), "200": dict(body=0.18, detail=0.1, texture=None, glazing=None, ground=0.25),
        "100": dict(body=0.25, detail=0.13, texture=None, glazing=None, ground=0.35), "50": dict(body=0.35, detail=0.18, texture=0.1, glazing=None, ground=0.5)}
DEC = {
 "500": dict(
  silhouette=("One composed side silhouette of 2.9 × 1.4 mm: trunk in two overlapping masses (hindquarters on the lumbar bone, chest on the thorax bone), a neck wedge, one head mass with muzzle, a tail stroke; the four legs are drawn as thick strokes along the limb bones. Bounds equal the standing-alert pose.", "Eine zusammengesetzte Seitensilhouette von 2,9 × 1,4 mm: Rumpf aus zwei überlappenden Massen (Hinterhand am Lendenknochen, Brust am Brustkorbknochen), ein Halskeil, eine Kopfmasse mit Fang, ein Schwanzstrich; die vier Läufe sind als dicke Striche entlang der Gliedmaßenknochen gezeichnet. Die Umrissgrenzen entsprechen der Standpose."),
  retained=("Body proportions, the long otter tail, hind-limb angulation and the raised head; near and far legs stay separate bone chains so gait remains legible.", "Körperproportionen, die lange Otterrute, die Hinterhandwinkelung und der erhobene Kopf; nahe und ferne Beine bleiben getrennte Knochenketten, damit der Gang lesbar bleibt."),
  merged=("Shoulder blade and upper arm, neck skin and coat, ears into the head mass, shank and hock of the hind leg into one stroke; paw and metatarsus into one foot stroke. All coat regions are listed on the merged masses.", "Schulterblatt und Oberarm, Halshaut und Fell, Ohren in die Kopfmasse, Unterschenkel und Sprunggelenk des Hinterbeins zu einem Strich; Pfote und Mittelfuß zu einem Fußstrich. Alle Fellregionen sind an den verschmolzenen Massen aufgeführt."),
  omitted=("Eye, nose leather, mouth line, tongue, pads, toes, claws, fur strokes and all construction lines: the head is 0.23 m = 0.46 mm long and cannot carry them.", "Auge, Nasenspiegel, Maullinie, Zunge, Ballen, Zehen, Krallen, Fellstriche und alle Konstruktionslinien: der Kopf ist 0,23 m = 0,46 mm lang und kann sie nicht tragen."),
  thinElements=("Lower legs (real width 0.04-0.05 m = 0.1 mm) are drawn as 0.2-0.25 mm strokes, i.e. 0.1 m wide, 0.05 m wider than real, which is inside the 0.065 m a 0.13 mm outline already adds per side; the tail (0.05-0.09 m) is a 0.2 mm stroke.", "Unterläufe (reale Breite 0,04-0,05 m = 0,1 mm) sind als Striche von 0,2-0,25 mm gezeichnet, also 0,1 m breit und 0,05 m über der Wirklichkeit, innerhalb der 0,065 m, die eine 0,13-mm-Kontur je Seite ohnehin zufügt; die Rute (0,05-0,09 m) ist ein 0,2-mm-Strich."),
  hatching=("None.", "Keine."),
  styles=("Solid mid-grey masses with a 0.13 mm outline; far legs use thinner strokes and a lighter tone, so the dark/light line contrast carries depth.", "Mittelgraue Massen mit 0,13-mm-Kontur; ferne Läufe nutzen dünnere Striche und hellere Töne, der Hell-Dunkel-Kontrast trägt die Tiefe."),
  overlays=("Thermal overlays colour the merged masses by the area-weighted value of the listed regions (labelled as merged); head ears, nose, tongue and pads cannot be told apart, the legs show only the limb coat.", "Thermische Überlagerungen färben die verschmolzenen Massen mit dem flächengewichteten Wert der aufgeführten Regionen (als verschmolzen gekennzeichnet); Ohren, Nase, Zunge und Ballen am Kopf sind nicht unterscheidbar, die Läufe zeigen nur das Gliedmaßenfell.")),
 "200": dict(
  silhouette=("Main masses separated: hindquarters, chest, neck, head with jaw, tail as one tapering mass, near and far legs in three pieces each (thigh, shank-to-hock, foot); 7.4 × 3.6 mm with the real proportions.", "Hauptmassen getrennt: Hinterhand, Brust, Hals, Kopf mit Unterkiefer, Rute als eine zulaufende Masse, nahe und ferne Beine in je drei Teilen (Oberschenkel, Unterschenkel bis Sprunggelenk, Fuß); 7,4 × 3,6 mm mit den echten Proportionen."),
  retained=("Neck and head carriage, hanging ear, eye dot, the paw pads as one dark mark per foot pair, the jaw as its own mass so panting and sniffing read.", "Hals- und Kopfhaltung, Hängeohr, Augenpunkt, die Pfotenballen als eine dunkle Marke je Fußpaar, der Unterkiefer als eigene Masse, damit Hecheln und Schnüffeln lesbar bleiben."),
  merged=("Shoulder blade with upper arm and forearm joint masses, three tail bones into one mass, hock and paw into one lower-leg piece, head and nose leather.", "Schulterblatt mit Oberarm und Unterarmgelenk, drei Schwanzknochen zu einer Masse, Sprunggelenk und Pfote zu einem Unterlaufstück, Kopf und Nasenspiegel."),
  omitted=("Toes, claws, brow, mouth line, tongue (listed on the jaw/head masses), fur strokes and shoulder/hip construction lines.", "Zehen, Krallen, Braue, Maullinie, Zunge (an Unterkiefer und Kopfmasse geführt), Fellstriche und Schulter-/Hüftkonstruktionslinien."),
  thinElements=("Metacarpus and hock (0.04-0.05 m = 0.2-0.25 mm) are widened to 0.07 m inside the lower-leg pieces; the tail is a tapering mass 0.04-0.09 m wide (0.2-0.45 mm).", "Mittelhand und Sprunggelenk (0,04-0,05 m = 0,2-0,25 mm) sind in den Unterlaufstücken auf 0,07 m verbreitert; die Rute ist eine zulaufende Masse von 0,04-0,09 m Breite (0,2-0,45 mm)."),
  hatching=("None; on a 7 mm figure no hatch spacing at or above the profile minimum fits.", "Keine; auf einer 7 mm großen Figur passt kein Schraffurabstand ab dem Profilminimum."),
  styles=("Mid-grey solids with a 0.18 mm outline; detail strokes at 0.1 mm only for the eye and pads; far side lighter.", "Mittelgraue Flächen mit 0,18-mm-Kontur; Detailstriche mit 0,1 mm nur für Auge und Ballen; ferne Seite heller."),
  overlays=("Each mass carries its own region (trunk coat, limb coats, tail, ears); head and jaw carry the head, nose and tongue regions together.", "Jede Masse trägt ihre eigene Region (Rumpffell, Gliedmaßenfell, Rute, Ohren); Kopf und Unterkiefer tragen Kopf-, Nasen- und Zungenregion gemeinsam.")),
 "100": dict(
  silhouette=("Articulated outline, 14.4 × 7.2 mm: hindquarters, chest, neck, head, jaw, ear, tail in three pieces, every limb in its anatomical segments (thigh, shank, hock, paw; shoulder, upper arm, forearm, wrist, paw).", "Gegliederter Umriss, 14,4 × 7,2 mm: Hinterhand, Brust, Hals, Kopf, Unterkiefer, Ohr, Rute in drei Teilen, jede Gliedmaße in ihren anatomischen Segmenten (Oberschenkel, Unterschenkel, Sprunggelenk, Pfote; Schulter, Oberarm, Unterarm, Handwurzel, Pfote)."),
  retained=("Eye, nose leather, tongue on the jaw, ear, pad marks, shoulder and hip construction lines as 0.13 mm details.", "Auge, Nasenspiegel, Zunge am Unterkiefer, Ohr, Ballenmarken, Schulter- und Hüftkonstruktionslinien als 0,13-mm-Details."),
  merged=("Neck coat with trunk coat (same region, seams hidden); toes and claws into the paw outline.", "Halsfell mit Rumpffell (gleiche Region, Nähte verborgen); Zehen und Krallen in den Pfotenumriss."),
  omitted=("Toes, claws, brow, mouth line and fur strokes: their size (0.01-0.02 m = 0.1-0.2 mm) is at the profile minimum.", "Zehen, Krallen, Braue, Maullinie und Fellstriche: ihre Größe (0,01-0,02 m = 0,1-0,2 mm) liegt am Profilminimum."),
  thinElements=("Metacarpus 0.04-0.05 m = 0.4-0.5 mm printed at true width; tail tip 0.016 m = 0.16 mm widened to 0.03 m at its end.", "Mittelhand 0,04-0,05 m = 0,4-0,5 mm in wahrer Breite; die Schwanzspitze 0,016 m = 0,16 mm am Ende auf 0,03 m verbreitert."),
  hatching=("None: the short coat is shown by tone, not texture.", "Keine: das kurze Fell wird durch Ton, nicht durch Textur gezeigt."),
  styles=("Solid grey masses 0.25 mm outline, details 0.13 mm; far-side parts one tone lighter and drawn first.", "Graue Flächen mit 0,25-mm-Kontur, Details 0,13 mm; Teile der fernen Seite einen Ton heller und zuerst gezeichnet."),
  overlays=("Segments map one-to-one onto the nine coat/skin regions; the tongue region shows when the jaw opens.", "Segmente bilden die neun Fell-/Hautregionen eins zu eins ab; die Zungenregion erscheint, wenn der Kiefer sich öffnet.")),
 "50": dict(
  silhouette=("Full articulation at 28.8 × 14.4 mm: all coat masses, jaw, ear pair, tail in three tapering pieces, every limb in five segments with pads, toes and claws.", "Volle Gliederung bei 28,8 × 14,4 mm: alle Fellmassen, Unterkiefer, Ohrenpaar, Rute in drei zulaufenden Stücken, jede Gliedmaße in fünf Segmenten mit Ballen, Zehen und Krallen."),
  retained=("Eye, brow, mouth line, nose leather, tongue, pads, toe lines, shoulder/hip/elbow lines and four fur strokes.", "Auge, Braue, Maullinie, Nasenspiegel, Zunge, Ballen, Zehenlinien, Schulter-/Hüft-/Ellbogenlinien und vier Fellstriche."),
  merged=("Neck with trunk coat (same region, seams hidden by the engine).", "Hals mit Rumpffell (gleiche Region, Nähte vom System verborgen)."),
  omitted=("Individual hairs, whiskers, nail detail and nostril slit: below 0.1 mm.", "Einzelne Haare, Tasthaare, Nageldetails und Nüsternschlitz: unter 0,1 mm."),
  thinElements=("Claws 0.017 m long = 0.34 mm, drawn as small solid wedges; tail tip 0.016 m = 0.3 mm true width.", "Krallen 0,017 m lang = 0,34 mm, als kleine Keile gezeichnet; Schwanzspitze 0,016 m = 0,3 mm in wahrer Breite."),
  hatching=("None; fur is indicated by 0.1 mm texture strokes in the direction of growth.", "Keine; das Fell wird durch 0,1-mm-Texturstriche in Wuchsrichtung angedeutet."),
  styles=("Body 0.35 mm, detail 0.18 mm, fur texture 0.1 mm; far side tone 0.55, near 0.72, pads dark.", "Körper 0,35 mm, Detail 0,18 mm, Fellstruktur 0,1 mm; ferne Seite Ton 0,55, nahe 0,72, Ballen dunkel."),
  overlays=("One region per shape; texture and detail strokes carry no region.", "Eine Region je Form; Textur- und Detailstriche tragen keine Region.")),
}
VR = {"500": (350, 800), "200": (150, 350), "100": (75, 150), "50": (20, 75)}
def build_dog():
    reps = {k: rep_auth(int(k), VR[k], R.PENS[int(k)], R.rep(int(k)), DEC[k]) for k in ("500", "200", "100", "50")}
    reps = {k: reps[k] for k in ("500", "200", "100", "50")}
    a = build("labrador-dog", cfg, reps, 100, {"heightM": 0.72, "widthM": 1.44, "depthM": 0.28, "massKg": 30})
    return a
if __name__ == "__main__":
    a = build_dog(); save(a, "labrador-dog")
    print("saved", len(json.dumps(a)))
