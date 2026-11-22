/** 📝️ The authored texts of `nurse-walking` (English first, German second): identity, assumptions, per-scale decisions, provenance. */
import { type J, t } from "./lib.ts";
import { REFS } from "./refs.ts";

const D = (silhouette: J, retained: J, merged: J, omitted: J, thinElements: J, hatching: J, styles: J, overlays: J): J => ({ silhouette, retained, merged, omitted, thinElements, hatching, styles, overlays });

export const NURSE_TEXT = {
  identity: {
    title: t("Nurse Walking", "Gehende Pflegekraft"),
    summary: t(
      "A ward nurse of 1.78 m in a short-sleeved V-neck scrub tunic and drawstring scrub trousers, compression socks and closed clogs, bare below the elbows, walking briskly down the corridor at 1.2 m/s and 120 steps per minute with both arms swinging; a brisk walk cycle and a pause at the nurses' station.",
      "Eine Pflegekraft von 1,78 m in kurzärmeligem Kasack mit V-Ausschnitt und Bereichshose mit Kordelzug, Kompressionsstrümpfen und geschlossenen Clogs, unterhalb der Ellbogen frei, geht zügig mit 1,2 m/s und 120 Schritten je Minute den Flur entlang, beide Arme schwingen mit; ein zügiger Gangzyklus und eine Pause am Stationstresen.",
    ),
    tags: ["human", "adult", "walker", "nurse", "healthcare", "hospital", "scrubs", "clogs", "gait", "brisk"],
  },
  structure: [
    t("Segments are point masses at the de Leva (1996) centres of a 76 kg man plus 1.5 kg of scrubs, socks and clogs; nothing is carried.", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva (1996) eines 76 kg schweren Mannes plus 1,5 kg Bereichskleidung, Strümpfe und Clogs; getragen wird nichts."),
    t("Contacts are the heel and the ball of each foot following the gait spans with overlapping double support; the brisk walk is solved with inertia from the smooth keys at 30 per second.", "Kontakte sind Ferse und Fußballen jedes Fußes nach den Gangspannen mit überlappender Doppelstandphase; der zügige Gang wird mit Trägheit aus den glatten Schlüsseln mit 30 je Sekunde gelöst."),
  ],
  thermal: (skin: number): J[] => [
    t(`Skin area ${skin} m2 after DuBois and DuBois (1916) for 76 kg and 1.78 m; skin fractions 0.071 face, neck and V-neck, 0.024 scalp, 0.12 bare forearms and hands, 0.42 trunk and upper arms, 0.295 legs, 0.07 feet (rule-of-nines proportions, authored).`, `Hautfläche ${skin.toFixed(3).replace(".", ",")} m2 nach DuBois und DuBois (1916) für 76 kg und 1,78 m; Hautanteile 0,071 Gesicht, Hals und V-Ausschnitt, 0,024 Kopfhaut, 0,12 nackte Unterarme und Hände, 0,42 Rumpf und Oberarme, 0,295 Beine, 0,07 Füße (Neunerregel-Proportionen, geschätzt).`),
    t("Garments after the ISO 9920 tables as guidance: undershirt 0.08, short-sleeved scrub tunic 0.17, scrub trousers 0.15 clo with briefs, compression socks and clogs; bare below the elbows after the epic3 infection-control guidance. Ward air 22 C, 40 % relative humidity.", "Kleidung nach den Tabellen der ISO 9920 als Richtwert: Unterhemd 0,08, kurzärmeliger Kasack 0,17, Bereichshose 0,15 clo mit Unterhose, Kompressionsstrümpfen und Clogs; unterhalb der Ellbogen frei nach den Hygienerichtlinien epic3. Stationsluft 22 C, 40 % relative Feuchte."),
    t("Activity: brisk walking at 4.3 km/h and 120 steps per minute 3.5 met (Ainsworth 2011, code 17190; a cadence of 100 or more steps per minute is moderate intensity after Tudor-Locke 2018), standing 1.4 met; breathing 22 per minute at 1.3 L while walking (authored after ICRP 66).", "Aktivität: zügiges Gehen mit 4,3 km/h und 120 Schritten je Minute 3,5 met (Ainsworth 2011, Code 17190; ab 100 Schritten je Minute mäßige Intensität nach Tudor-Locke 2018), Stehen 1,4 met; Atmung beim Gehen 22 je Minute mit 1,3 L (geschätzt nach ICRP 66)."),
  ],
  representation: {
    "500": D(
      t("One composed figure 3.6 mm tall in mid-stride: head, the tunic as one mass to the upper thigh, short sleeves, bare forearms swinging, trousers and clogs.", "Eine zusammengesetzte Figur von 3,6 mm Höhe im Schritt: Kopf, der Kasack als eine Masse bis zum Oberschenkel, kurze Ärmel, schwingende nackte Unterarme, Hose und Clogs."),
      t("The open stride and the counter-swinging arms: a person in a hurry.", "Der offene Schritt und die gegenläufig schwingenden Arme: eine Person in Eile."),
      t("Collar and neck one form; hand and forearm one form.", "Kragen und Hals eine Form; Hand und Unterarm eine Form."),
      t("Face, fingers and pockets are left out.", "Gesicht, Finger und Taschen entfallen."),
      t("Nothing is below the printable feature apart from the fingers.", "Nichts liegt unter dem druckbaren Merkmal außer den Fingern."),
      t("No hatching.", "Keine Schraffur."),
      t("Line views: one outline; solid views: mid-tone scrubs, skin-toned forearms, pale clogs.", "Linienansichten: ein Umriss; Flächenansichten: mitteltonige Bereichskleidung, hautfarbene Unterarme, helle Clogs."),
      t("Overlays fill the merged masses; the bare forearms read warm against the tunic.", "Überlagerungen füllen die zusammengefassten Massen; die nackten Unterarme sind wärmer als der Kasack."),
    ),
    "200": D(
      t("Separate masses 9 mm tall: head and hair, neck, tunic, short sleeves, bare forearms and hands, legs and clogs split into heel and toe.", "Getrennte Massen von 9 mm Höhe: Kopf und Haar, Hals, Kasack, kurze Ärmel, nackte Unterarme und Hände, Beine und Clogs in Ferse und Spitze geteilt."),
      t("The tunic hem above mid-thigh, the sleeve ending above the elbow.", "Der Kasacksaum oberhalb der Oberschenkelmitte, der Ärmel endet über dem Ellbogen."),
      t("Hands one form each.", "Hände je eine Form."),
      t("Face, fingers and pockets are left out.", "Gesicht, Finger und Taschen entfallen."),
      t("None.", "Keine."),
      t("No hatching.", "Keine Schraffur."),
      t("Body pen 0.18 mm, detail 0.1 mm.", "Körperstift 0,18 mm, Detail 0,1 mm."),
      t("Overlays follow the separated masses.", "Überlagerungen folgen den getrennten Massen."),
    ),
    "100": D(
      t("Articulated figure 18 mm tall: V-neck with its skin, collar band, tunic with hip pockets, short sleeves, bare forearms with wrists, hands, scrub trousers and clogs.", "Gegliederte Figur von 18 mm Höhe: V-Ausschnitt mit seiner Haut, Kragenband, Kasack mit Hüfttaschen, kurze Ärmel, nackte Unterarme mit Handgelenken, Hände, Bereichshose und Clogs."),
      t("Ears, hair fringe, the V-neck, the tunic hem traced from the trimmed volume.", "Ohren, Stirnhaar, der V-Ausschnitt, der aus dem gekürzten Volumen nachgezeichnete Kasacksaum."),
      t("Hand and fingers one form per hand.", "Hand und Finger je eine Form."),
      t("Buttons and lapels of the base figure are removed, the pockets are below the feature.", "Knöpfe und Revers der Ausgangsfigur sind entfernt, die Taschen liegen unter dem Merkmal."),
      t("None.", "Keine."),
      t("No hatching: the scrub twill is drawn plain.", "Keine Schraffur: der Köper bleibt glatt."),
      t("Body 0.2 mm, detail 0.1 mm.", "Körper 0,2 mm, Detail 0,1 mm."),
      t("Overlays map one region per shape.", "Überlagerungen bilden eine Region pro Form ab."),
    ),
    "50": D(
      t("Full figure 36 mm tall: face in profile and front, ears, hair, V-neck tunic with collar band and two hip pockets, short sleeves, bare forearms and fingered hands, drawstring trousers, compression socks at the ankle, clogs with heel and sole.", "Vollständige Figur von 36 mm Höhe: Gesicht im Profil und von vorn, Ohren, Haar, Kasack mit V-Ausschnitt, Kragenband und zwei Hüfttaschen, kurze Ärmel, nackte Unterarme und Hände mit Fingern, Hose mit Kordelzug, Kompressionsstrümpfe am Knöchel, Clogs mit Ferse und Sohle."),
      t("Every garment edge of the 1:50 body.", "Jede Kleidungskante des 1:50-Körpers."),
      t("Nothing is merged.", "Nichts ist zusammengefasst."),
      t("Fob watch and name badge are not modelled.", "Uhr und Namensschild sind nicht modelliert."),
      t("None.", "Keine."),
      t("No hatching.", "Keine Schraffur."),
      t("Body 0.25 mm, detail 0.13 mm, texture 0.1 mm; solid views separate scrubs, skin, socks and clogs.", "Körper 0,25 mm, Detail 0,13 mm, Textur 0,1 mm; Flächenansichten trennen Bereichskleidung, Haut, Strümpfe und Clogs."),
      t("Every shape carries one region; the thermal view shows the warm bare forearms swinging beside the tunic.", "Jede Form trägt eine Region; die Thermikansicht zeigt die warmen nackten Unterarme neben dem Kasack."),
    ),
  } as Record<string, J>,
  volume: {
    "500": [t("1:500 body: the commuter's coarse figure with its coat lofts cut at the 0.765 m hem to a tunic, the forearm lofts thinned to bare arms (72 %), the scarf ring as collar and neck; no bag.", "Körper 1:500: die grobe Figur des Pendlers, die Mantel-Lofts am Saum auf 0,765 m zum Kasack gekürzt, die Unterarm-Lofts zu nackten Armen verschlankt (72 %), der Schalring als Kragen und Hals; keine Tasche.")],
    "200": [t("1:200 body: tunic cut at the hem, bare thinned forearms, trousers and split clogs as at 1:200 of the commuter, without scarf and bag.", "Körper 1:200: Kasack am Saum gekürzt, nackte verschlankte Unterarme, Hose und geteilte Clogs wie im Maßstab 1:200 des Pendlers, ohne Schal und Tasche.")],
    "100": [t("1:100 body at a 15 mm feature: tunic body, waist and the stubs of the skirt halves above the hem, V-neck skin in the opening, collar band, bare forearms and hands with every phalanx; lapels, buttons and the back vent removed.", "Körper 1:100 bei 15 mm kleinstem Merkmal: Kasack-Oberteil, Taille und die Stümpfe der Rockhälften oberhalb des Saums, Haut im V-Ausschnitt, Kragenband, nackte Unterarme und Hände mit allen Fingergliedern; Revers, Knöpfe und Rückenschlitz entfernt.")],
    "50": [t("1:50 body: the tunic as two layer shells (undershirt, tunic) cut at the hem, with hip pockets; bare forearms without shirt cuffs; trousers over briefs, compression socks and clogs with heel block and sole.", "Körper 1:50: der Kasack als zwei Schichtschalen (Unterhemd, Kasack), am Saum gekürzt, mit Hüfttaschen; nackte Unterarme ohne Hemdmanschetten; Hose über Unterhose, Kompressionsstrümpfe und Clogs mit Absatzblock und Sohle.")],
  } as Record<string, J[]>,
  notes: t(
    "Derived by HW-03 from walking-commuter (rig with spine, shoulder girdle and fingers, de Leva male segment fractions, the planted-foot walk cycle and its key poses): the overcoat lofts are cut at the hem to a scrub tunic and every outline of the trunk, arms and legs is re-traced from the trimmed volume in the default pose, the forearms are thinned to bare arms, scarf, bag, lapels, buttons and cuffs are removed, the right arm swings in counter-phase (the left arm's swing half a cycle later) and the cycle is shortened from 1.2 to 1.0 s. Generator: ticket STAFFAGE-HUMAN-COLLECTION-WAVE, scripts HW-03/nurse.ts.",
    "Von HW-03 aus walking-commuter abgeleitet (Skelett mit Wirbelsäule, Schultergürtel und Fingern, männliche Segmentanteile nach de Leva, der Gangzyklus mit aufgesetzten Füßen und seine Schlüsselhaltungen): Die Mantel-Lofts sind am Saum zum Kasack gekürzt und jeder Umriss von Rumpf, Armen und Beinen ist aus dem gekürzten Volumen in der Standardhaltung neu nachgezeichnet, die Unterarme sind zu nackten Armen verschlankt, Schal, Tasche, Revers, Knöpfe und Manschetten entfernt, der rechte Arm schwingt gegengleich (der Schwung des linken Arms einen halben Zyklus später) und der Zyklus ist von 1,2 auf 1,0 s verkürzt. Generator: Ticket STAFFAGE-HUMAN-COLLECTION-WAVE, Skripte HW-03/nurse.ts.",
  ),
  references: [REFS.ainsworth, REFS.tudorLocke, REFS.loveday, REFS.iso7730b],
};
