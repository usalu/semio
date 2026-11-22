/** 📝️ The authored texts of `patient-with-iv-pole` (English first, German second): identity, assumptions, per-scale decisions, provenance. */
import { type J, t } from "./lib.ts";
import { IV_REFS, REFS } from "./refs.ts";

const D = (silhouette: J, retained: J, merged: J, omitted: J, thinElements: J, hatching: J, styles: J, overlays: J): J => ({ silhouette, retained, merged, omitted, thinElements, hatching, styles, overlays });

export const PATIENT_TEXT = {
  identity: {
    title: t("Patient with IV Pole", "Patient mit Infusionsständer"),
    summary: t(
      "A 71-year-old in-patient of 1.78 m in a knee-length cotton patient gown, bare-legged in felt slippers, walking slowly and slightly stooped along the ward corridor while the right hand pushes a five-castor IV stand of 2 m with a saline bag and a running infusion pump (15 W nameplate); a slow walk at 0.71 m/s and a rest at the stand.",
      "Ein 71-jähriger stationärer Patient von 1,78 m in knielangem Baumwoll-Patientenhemd, mit nackten Beinen in Filzpantoffeln, geht langsam und leicht gebeugt den Stationsflur entlang, während die rechte Hand einen 2 m hohen Infusionsständer auf fünf Lenkrollen mit Kochsalzbeutel und laufender Infusionspumpe (15 W Nennleistung) schiebt; langsames Gehen mit 0,71 m/s und eine Pause am Ständer.",
    ),
    tags: ["human", "adult", "elderly", "walker", "patient", "healthcare", "hospital", "gown", "slippers", "iv-stand", "infusion-pump", "gait"],
  },
  structure: [
    t("Segments are point masses at the de Leva (1996) centres of a 70 kg man plus 0.8 kg of gown and slippers; the IV stand (6.3 kg), its saline bag (1.05 kg) and pump (1.9 kg) roll on their own castors and are not carried: the hand only steers, a 6 N downward load at the handle stands for the hand resting on it.", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva (1996) eines 70 kg schweren Mannes plus 0,8 kg Hemd und Pantoffeln; der Infusionsständer (6,3 kg), sein Kochsalzbeutel (1,05 kg) und die Pumpe (1,9 kg) rollen auf ihren eigenen Lenkrollen und werden nicht getragen: Die Hand lenkt nur, eine Last von 6 N nach unten am Griff steht für die aufgelegte Hand."),
    t("Contacts are the heel and the ball of each foot following the gait spans; the stand's grip joint keeps the pole upright and its sliding joint keeps the castors on the floor in every pose and clip key. The slow walk is solved quasi-statically.", "Kontakte sind Ferse und Fußballen jedes Fußes nach den Gangspannen; das Griffgelenk des Ständers hält die Stange senkrecht und sein Schiebegelenk die Rollen in jeder Haltung und jedem Clip-Schlüssel auf dem Boden. Das langsame Gehen wird quasistatisch gelöst."),
  ],
  thermal: (skin: number): J[] => [
    t(`Skin area ${skin} m2 after DuBois and DuBois (1916) for 70 kg and 1.78 m; skin fractions 0.071 face, neck and neckline, 0.024 scalp, 0.05 hands, 0.49 trunk and arms under the gown, 0.295 legs, 0.07 feet (rule-of-nines proportions, authored).`, `Hautfläche ${skin.toFixed(3).replace(".", ",")} m2 nach DuBois und DuBois (1916) für 70 kg und 1,78 m; Hautanteile 0,071 Gesicht, Hals und Ausschnitt, 0,024 Kopfhaut, 0,05 Hände, 0,49 Rumpf und Arme unter dem Hemd, 0,295 Beine, 0,07 Füße (Neunerregel-Proportionen, geschätzt).`),
    t("The gown is one layer of about 0.3 clo (ISO 9920 garment tables as guidance), the slippers 0.05 clo; the legs are bare. Ward air 23 C at 40 % relative humidity.", "Das Hemd ist eine Schicht von etwa 0,3 clo (Kleidungstabellen der ISO 9920 als Richtwert), die Pantoffeln 0,05 clo; die Beine sind nackt. Stationsluft 23 C bei 40 % relativer Feuchte."),
    t("Activity: standing 1.2 met (ISO 7730 Annex B), slow walking at 2.2 km/h 2.0 met (Ainsworth 2011, code 17151); an older patient's basal metabolism of 66 W after Mifflin-St Jeor for 70 kg, 71 years, and skin blood flow limited to 70 L/(m2 h) (authored).", "Aktivität: Stehen 1,2 met (ISO 7730 Anhang B), langsames Gehen mit 2,2 km/h 2,0 met (Ainsworth 2011, Code 17151); Grundumsatz eines älteren Patienten von 66 W nach Mifflin-St Jeor für 70 kg und 71 Jahre, Hautdurchblutung auf 70 L/(m2 h) begrenzt (geschätzt)."),
    t("The infusion pump releases 4 W of its 15 W electrical nameplate as heat into its housing while it infuses; the stand and the bag at room temperature are passive.", "Die Infusionspumpe gibt beim Infundieren 4 W ihrer elektrischen Nennleistung von 15 W als Wärme an ihr Gehäuse ab; Ständer und Beutel bei Raumtemperatur sind passiv."),
  ],
  representation: {
    "500": D(
      t("One composed figure 3.6 mm tall beside a 4 mm stand: the gown as one mass from the collar to the knee, bare shins, slippers, the right forearm reaching to the handle, the pole as a single line, the base as a low disc and the pump as a small block.", "Eine zusammengesetzte Figur von 3,6 mm Höhe neben einem 4 mm hohen Ständer: das Hemd als eine Masse vom Kragen bis zum Knie, nackte Schienbeine, Pantoffeln, der rechte Unterarm zum Griff, die Stange als einzelne Linie, der Fuß als flache Scheibe und die Pumpe als kleiner Block."),
      t("The stand taller than the patient, the slight stoop and the handle joining hand and pole: the cues of a patient walking with an infusion.", "Der Ständer, höher als der Patient, die leichte Beugung und der Griff zwischen Hand und Stange: die Merkmale eines Patienten, der mit einer Infusion geht."),
      t("The five legs and castors are typified as one disc; collar and neck are one form.", "Die fünf Beine und Rollen sind zu einer Scheibe typisiert; Kragen und Hals sind eine Form."),
      t("Saline bag, crossbar, face and fingers are left out.", "Kochsalzbeutel, Querstange, Gesicht und Finger entfallen."),
      t("The 25 mm pole is a 0.13 mm line (65 mm on paper), the 75 mm handle is thickened to the feature.", "Die 25-mm-Stange ist eine 0,13-mm-Linie (65 mm auf dem Papier), der Griff auf 75 mm zum kleinsten Merkmal verdickt."),
      t("No hatching.", "Keine Schraffur."),
      t("Line views: one outline; solid views: pale gown, skin-toned shins, dark base.", "Linienansichten: ein Umriss; Flächenansichten: helles Hemd, hautfarbene Schienbeine, dunkler Fuß."),
      t("Overlays fill the merged masses; the pump block carries its warm operating value.", "Überlagerungen füllen die zusammengefassten Massen; der Pumpenblock trägt seinen warmen Betriebswert."),
    ),
    "200": D(
      t("Separate masses 9 mm tall: head and grey hair, gown with sleeves, hands, bare shins, slippers; the stand with pole, five legs and castors, saline bag at the top and pump at hand height.", "Getrennte Massen von 9 mm Höhe: Kopf und graues Haar, Hemd mit Ärmeln, Hände, nackte Schienbeine, Pantoffeln; der Ständer mit Stange, fünf Beinen und Rollen, Kochsalzbeutel oben und Pumpe in Handhöhe."),
      t("The base star, the bag and the pump as their own forms.", "Der Fußstern, der Beutel und die Pumpe als eigene Formen."),
      t("Hub, legs and castors are one base form.", "Nabe, Beine und Rollen sind eine Fußform."),
      t("Face, fingers, clamp and the drip line are left out.", "Gesicht, Finger, Klemme und die Infusionsleitung entfallen."),
      t("Pole and crossbar are lines; legs and handle are drawn at 40 mm, a little thicker than real, to print.", "Stange und Querstange sind Linien; Beine und Griff sind mit 40 mm gezeichnet, etwas dicker als echt, damit sie drucken."),
      t("No hatching.", "Keine Schraffur."),
      t("Body pen 0.18 mm; solid views separate gown, skin, slippers, steel and the white bag.", "Körperstift 0,18 mm; Flächenansichten trennen Hemd, Haut, Pantoffeln, Stahl und den weißen Beutel."),
      t("Overlays follow the separated masses; the stand and bag share one passive region.", "Überlagerungen folgen den getrennten Massen; Ständer und Beutel teilen eine passive Region."),
    ),
    "100": D(
      t("Articulated figure 18 mm tall: gown with collar and wrap-over neckline, sleeves, hands, bare shins and slippers; the stand drawn from its volume with pole, base star, castors, handle clamp, pump and bag.", "Gegliederte Figur von 18 mm Höhe: Hemd mit Kragen und überlappendem Ausschnitt, Ärmel, Hände, nackte Schienbeine und Pantoffeln; der Ständer aus seinem Volumen gezeichnet mit Stange, Fußstern, Rollen, Griffklemme, Pumpe und Beutel."),
      t("Ears, hair fringe, wrists, the neckline skin, the handle clamp.", "Ohren, Stirnhaar, Handgelenke, die Haut im Ausschnitt, die Griffklemme."),
      t("Hand and fingers one form per hand; hub, legs and castors one base form.", "Hand und Finger je eine Form; Nabe, Beine und Rollen eine Fußform."),
      t("Drip line and the gown ties at the back are left out.", "Infusionsleitung und die Bänder am Rücken entfallen."),
      t("The 25 mm pole is drawn at true width (0.25 mm); the 12 mm crossbar is a line.", "Die 25-mm-Stange ist in wahrer Breite gezeichnet (0,25 mm); die 12-mm-Querstange ist eine Linie."),
      t("No hatching: the cotton gown is drawn plain.", "Keine Schraffur: das Baumwollhemd bleibt glatt."),
      t("Body 0.2 mm, detail 0.1 mm; solid views give the gown a pale tone against the skin of the shins.", "Körper 0,2 mm, Detail 0,1 mm; Flächenansichten geben dem Hemd einen hellen Ton gegen die Haut der Schienbeine."),
      t("Overlays map one region per shape.", "Überlagerungen bilden eine Region pro Form ab."),
    ),
    "50": D(
      t("Full figure 36 mm tall: face in profile and front, ears, grey hair, gown with collar and overlapping front panels, back vent and side seam, sleeves with cuffs, fingered hands, bare shins and ankles, felt slippers with soles; the stand with pole, crossbar, bag, pump, handle and clamp, five legs and castors.", "Vollständige Figur von 36 mm Höhe: Gesicht im Profil und von vorn, Ohren, graues Haar, Hemd mit Kragen und überlappenden Vorderteilen, Rückenschlitz und Seitennaht, Ärmel mit Bündchen, Hände mit Fingern, nackte Schienbeine und Knöchel, Filzpantoffeln mit Sohlen; der Ständer mit Stange, Querstange, Beutel, Pumpe, Griff und Klemme, fünf Beinen und Rollen."),
      t("Every edge of the 1:50 body and of the stand.", "Jede Kante des 1:50-Körpers und des Ständers."),
      t("Nothing is merged apart from the fingers of the right hand closed on the handle.", "Nichts ist zusammengefasst außer den Fingern der am Griff geschlossenen rechten Hand."),
      t("The 4 mm drip line from the bag to the arm is left out: it is below the printable stroke and moves freely.", "Die 4-mm-Infusionsleitung vom Beutel zum Arm entfällt: Sie liegt unter dem druckbaren Strich und bewegt sich frei."),
      t("Pole 25 mm and crossbar 12 mm at true width.", "Stange 25 mm und Querstange 12 mm in wahrer Breite."),
      t("No hatching.", "Keine Schraffur."),
      t("Body 0.25 mm, detail 0.13 mm, texture 0.1 mm; solid views separate pale gown, skin, slippers, chrome, the white bag and the pump.", "Körper 0,25 mm, Detail 0,13 mm, Textur 0,1 mm; Flächenansichten trennen helles Hemd, Haut, Pantoffeln, Chrom, den weißen Beutel und die Pumpe."),
      t("Every shape carries one region; the thermal view shows warm bare shins against the gown and the warm pump on the cool chrome pole.", "Jede Form trägt eine Region; die Thermikansicht zeigt warme nackte Schienbeine gegen das Hemd und die warme Pumpe an der kühlen Chromstange."),
    ),
  } as Record<string, J>,
  volume: {
    "500": [t("1:500 body: the commuter's coarse figure as a patient: the coat lofts become the gown, the scarf ring the collar; the stand is a slender 25 mm pole, a typified base disc of 0.54 m, the pump box and a 75 mm handle to the right hand.", "Körper 1:500: die grobe Figur des Pendlers als Patient: die Mantel-Lofts werden zum Hemd, der Schalring zum Kragen; der Ständer ist eine schlanke 25-mm-Stange, eine typisierte Fußscheibe von 0,54 m, der Pumpenkasten und ein 75-mm-Griff zur rechten Hand.")],
    "200": [t("1:200 body: gown, bare legs and slippers as at 1:200 of the commuter without scarf and bag; the stand with pole, hub, five 40 mm legs and castors, 40 mm bag, pump and a 40 mm handle.", "Körper 1:200: Hemd, nackte Beine und Pantoffeln wie im Maßstab 1:200 des Pendlers ohne Schal und Tasche; der Ständer mit Stange, Nabe, fünf 40-mm-Beinen und Rollen, 40-mm-Beutel, Pumpe und einem 40-mm-Griff.")],
    "100": [t("1:100 body at a 15 mm feature: gown with lapel-like overlapping front panels and the bare neckline, hands with every phalanx; the stand with 30 mm legs, 50 mm castors, the slender crossbar, the 35 mm saline bag, the pump and a 22 mm handle with its clamp.", "Körper 1:100 bei 15 mm kleinstem Merkmal: Hemd mit reversartig überlappenden Vorderteilen und dem nackten Ausschnitt, Hände mit allen Fingergliedern; der Ständer mit 30-mm-Beinen, 50-mm-Rollen, der schlanken Querstange, dem 35-mm-Kochsalzbeutel, der Pumpe und einem 22-mm-Griff mit Klemme.")],
    "50": [t("1:50 body: the gown cut like the overcoat it derives from (body, skirt halves that swing with the thighs, centre strip, overlapping front panels, collar) as one cotton layer without buttons and pockets, bare shins and ankles, slippers with heel and sole; the stand as at 1:100 with every part at true section.", "Körper 1:50: das Hemd geschnitten wie der Mantel, von dem es abstammt (Oberteil, Rockhälften, die mit den Oberschenkeln schwingen, Mittelstreifen, überlappende Vorderteile, Kragen), als eine Baumwollschicht ohne Knöpfe und Taschen, nackte Schienbeine und Knöchel, Pantoffeln mit Ferse und Sohle; der Ständer wie bei 1:100 mit jedem Teil in wahrem Querschnitt.")],
  } as Record<string, J[]>,
  notes: t(
    "Derived by HW-03 from walking-commuter (rig with spine, shoulder girdle and fingers, de Leva male segment fractions, the planted-foot walk cycle, the knee-length coat cut and its traced outlines): the overcoat becomes a patient gown, trousers and shoes become bare legs and slippers, scarf, bag, buttons and pockets are removed, the walk is slowed to 1.7 s per cycle with a slight stoop and the right hand on the handle of a new IV stand rig (revolute grip, sliding pole) whose joints are solved for every pose and clip key so the pole stays upright and its castors on the floor; the stand is drawn at every scale from its own volume. Generator: ticket STAFFAGE-HUMAN-COLLECTION-WAVE, scripts HW-03/patient.ts.",
    "Von HW-03 aus walking-commuter abgeleitet (Skelett mit Wirbelsäule, Schultergürtel und Fingern, männliche Segmentanteile nach de Leva, der Gangzyklus mit aufgesetzten Füßen, der Schnitt des knielangen Mantels und seine nachgezeichneten Umrisse): Der Mantel wird zum Patientenhemd, Hose und Schuhe werden zu nackten Beinen und Pantoffeln, Schal, Tasche, Knöpfe und Taschen entfallen, der Gang ist auf 1,7 s je Zyklus verlangsamt, leicht gebeugt, die rechte Hand am Griff eines neuen Infusionsständer-Skeletts (Drehgriff, gleitende Stange), dessen Gelenke für jede Haltung und jeden Clip-Schlüssel gelöst sind, sodass die Stange senkrecht und die Rollen auf dem Boden bleiben; der Ständer ist in jedem Maßstab aus seinem eigenen Volumen gezeichnet. Generator: Ticket STAFFAGE-HUMAN-COLLECTION-WAVE, Skripte HW-03/patient.ts.",
  ),
  references: [REFS.incropera, REFS.iso7730b, REFS.ainsworth, REFS.hollman, IV_REFS.iec60601],
};
