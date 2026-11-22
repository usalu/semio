/** 📝️ The authored texts of `doctor-with-clipboard` (English first, German second): identity, assumptions, per-scale decisions, symbol, provenance. */
import { type J, t } from "./lib.ts";
import { REFS } from "./refs.ts";

const D = (silhouette: J, retained: J, merged: J, omitted: J, thinElements: J, hatching: J, styles: J, overlays: J): J => ({ silhouette, retained, merged, omitted, thinElements, hatching, styles, overlays });

export const DOCTOR_TEXT = {
  identity: {
    title: t("Doctor with Clipboard", "Arzt mit Klemmbrett"),
    summary: t(
      "A ward doctor of 1.78 m in a knee-length white lab coat over shirt and pullover, chinos and leather shoes, a stethoscope round the neck, reading the chart on an A4 clipboard held in the right hand at chest height, the pen ready in the left; poses for reading, writing a note and talking to a patient at about 1.3 to 1.5 met.",
      "Ein Stationsarzt von 1,78 m in knielangem weißem Laborkittel über Hemd und Pullover, Chinohose und Lederschuhen, ein Stethoskop um den Hals, liest das Krankenblatt auf einem A4-Klemmbrett, das er mit der rechten Hand in Brusthöhe hält, den Stift bereit in der linken; Haltungen für Lesen, Notieren und das Gespräch mit einer Patientin oder einem Patienten bei etwa 1,3 bis 1,5 met.",
    ),
    tags: ["human", "adult", "standing", "doctor", "healthcare", "hospital", "lab-coat", "stethoscope", "clipboard", "reading", "writing"],
  },
  structure: [
    t("Segments are point masses at the de Leva (1996) centres of a 75 kg man plus 2.6 kg of clothing and the 0.17 kg stethoscope; the 0.42 kg clipboard is the mass of its own fixed bone on the right hand.", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva (1996) eines 75 kg schweren Mannes plus 2,6 kg Kleidung und dem 0,17 kg schweren Stethoskop; das 0,42 kg schwere Klemmbrett ist die Masse seines eigenen festen Knochens an der rechten Hand."),
    t("Contacts are the heel and the ball of each foot in every pose and clip; the motions are slow, so the structure is solved quasi-statically.", "Kontakte sind Ferse und Fußballen jedes Fußes in jeder Haltung und jedem Clip; die Bewegungen sind langsam, daher wird das Tragwerk quasistatisch gelöst."),
  ],
  thermal: (skin: number): J[] => [
    t(`Skin area ${skin} m2 after DuBois and DuBois (1916) for 75 kg and 1.78 m; skin fractions 0.071 face and neck, 0.024 scalp, 0.05 hands, 0.49 trunk and arms, 0.295 legs, 0.07 feet (rule-of-nines proportions, authored).`, `Hautfläche ${skin.toFixed(3).replace(".", ",")} m2 nach DuBois und DuBois (1916) für 75 kg und 1,78 m; Hautanteile 0,071 Gesicht und Hals, 0,024 Kopfhaut, 0,05 Hände, 0,49 Rumpf und Arme, 0,295 Beine, 0,07 Füße (Neunerregel-Proportionen, geschätzt).`),
    t("Garment insulation in clo after the ISO 9920 garment tables as guidance (undershirt 0.08, long-sleeve shirt 0.25, thin pullover 0.25, knee-length lab coat 0.3, chinos 0.24, briefs, socks and shoes), referred to the outer area with f_cl of about 1.15; vapour resistance R/(0.0165 K/Pa x 0.38). The digits are authored estimates.", "Kleidungsisolation in clo nach den Kleidungstabellen der ISO 9920 als Richtwert (Unterhemd 0,08, langärmeliges Hemd 0,25, dünner Pullover 0,25, knielanger Laborkittel 0,3, Chino 0,24, Unterhose, Socken und Schuhe), auf die Außenfläche bezogen mit f_cl von etwa 1,15; Dampfwiderstand R/(0,0165 K/Pa x 0,38). Die Zahlen sind Schätzungen."),
    t("Activity after ISO 8996 and ISO 7730 Annex B: standing relaxed and reading 1.3 met, writing standing 1.5 met, standing and talking 1.4 met; ward air at 22 C and 40 % relative humidity.", "Aktivität nach ISO 8996 und ISO 7730 Anhang B: entspanntes Stehen und Lesen 1,3 met, Schreiben im Stehen 1,5 met, Stehen und Sprechen 1,4 met; Stationsluft bei 22 C und 40 % relativer Feuchte."),
    t("Stethoscope (PVC tube, steel chest piece) and clipboard (hardboard, paper, steel clip, plastic pen) are passive surfaces without power; their heat capacities are those of their materials over their areas.", "Stethoskop (PVC-Schlauch, Bruststück aus Stahl) und Klemmbrett (Hartfaser, Papier, Stahlklemme, Kunststoffstift) sind passive Flächen ohne Leistung; ihre Wärmekapazitäten sind die ihrer Werkstoffe über ihrer Fläche."),
  ],
  representation: {
    "500": D(
      t("One composed figure 3.6 mm tall: head with hair, the white coat as one mass from collar to knee with the collar and neck merged, legs, shoes, both arms raised in front of the chest and the clipboard as a 0.2 mm board ahead of the chest.", "Eine zusammengesetzte Figur von 3,6 mm Höhe: Kopf mit Haar, der weiße Kittel als eine Masse vom Kragen bis zum Knie, Kragen und Hals zusammengefasst, Beine, Schuhe, beide Arme vor der Brust erhoben und das Klemmbrett als 0,2-mm-Brett vor der Brust."),
      t("Coat length to the knee, the forward reach of the clipboard and the bowed head: the cues of a doctor reading a chart.", "Kittellänge bis zum Knie, die Vorlage des Klemmbretts und der geneigte Kopf: die Merkmale eines Arztes, der ein Krankenblatt liest."),
      t("Collar and neck are one form; the stethoscope is merged into the coat silhouette (the coat shape lists both regions).", "Kragen und Hals sind eine Form; das Stethoskop ist in die Kittelsilhouette eingerechnet (die Kittelform führt beide Regionen)."),
      t("Fingers, face, pockets, buttons, paper, clip and pen are left out.", "Finger, Gesicht, Taschen, Knöpfe, Papier, Klemme und Stift entfallen."),
      t("The board is drawn at 4 mm true thickness edge-on as a thin outline; the stethoscope tube (10 mm) is below the feature and merged.", "Das Brett ist in wahrer Dicke von 4 mm als dünne Kante gezeichnet; der Stethoskopschlauch (10 mm) liegt unter dem kleinsten Merkmal und ist eingerechnet."),
      t("No hatching.", "Keine Schraffur."),
      t("Line views: one body outline at 0.13 mm; solid views: light coat, dark chinos and shoes, mid-grey board.", "Linienansichten: ein Körperumriss mit 0,13 mm; Flächenansichten: heller Kittel, dunkle Hose und Schuhe, mittelgraues Brett."),
      t("Thermal and structural overlays fill the merged masses; the coat silhouette carries the area-weighted value of coat and stethoscope.", "Thermische und statische Überlagerungen füllen die zusammengefassten Massen; die Kittelsilhouette trägt den flächengewichteten Wert von Kittel und Stethoskop."),
    ),
    "200": D(
      t("Separate masses 9 mm tall: head and hair, neck, coat, sleeves and hands, legs and shoes; the clipboard reaches ahead of the chest in the right hand.", "Getrennte Massen von 9 mm Höhe: Kopf und Haar, Hals, Kittel, Ärmel und Hände, Beine und Schuhe; das Klemmbrett ragt in der rechten Hand vor die Brust."),
      t("Coat outline with sleeves, shoe split into rear and toe, the board in plan and front as a light plate.", "Kittelumriss mit Ärmeln, Schuh in Ferse und Zehe geteilt, das Brett in Draufsicht und Vorderansicht als helle Platte."),
      t("The stethoscope is merged into the coat mass; paper and clip into the board.", "Das Stethoskop ist in die Kittelmasse eingerechnet; Papier und Klemme in das Brett."),
      t("Face, fingers, buttons, pockets and the pen are left out.", "Gesicht, Finger, Knöpfe, Taschen und der Stift entfallen."),
      t("The 4 mm board is a thin outline in the side view; the tube is below the feature.", "Das 4-mm-Brett ist in der Seitenansicht ein dünner Umriss; der Schlauch liegt unter dem kleinsten Merkmal."),
      t("No hatching.", "Keine Schraffur."),
      t("Body pen 0.18 mm, detail 0.1 mm; solid views separate light coat, skin, dark chinos and the board.", "Körperstift 0,18 mm, Detail 0,1 mm; Flächenansichten trennen hellen Kittel, Haut, dunkle Hose und das Brett."),
      t("Overlays follow the separated masses; coat and stethoscope share one value.", "Überlagerungen folgen den getrennten Massen; Kittel und Stethoskop teilen einen Wert."),
    ),
    "100": D(
      t("Articulated figure 18 mm tall: collar and lapels over the shirt front, coat with sleeves to the wrist, hands, the board with its paper, the stethoscope tube over the lapels.", "Gegliederte Figur von 18 mm Höhe: Kragen und Revers über der Hemdbrust, Kittel mit Ärmeln bis zum Handgelenk, Hände, das Brett mit seinem Papier, der Stethoskopschlauch über den Revers."),
      t("Shirt front in the lapel opening, ears, hair fringe, wrists, the paper on the board in front and plan, the tube as a 0.1 mm line in every view.", "Hemdbrust in der Reversöffnung, Ohren, Stirnhaar, Handgelenke, das Papier auf dem Brett in Vorder- und Draufsicht, der Schlauch als 0,1-mm-Linie in jeder Ansicht."),
      t("Hand and fingers are one mitten form per hand; clip merged into the board.", "Hand und Finger sind je eine Fäustlingsform; die Klemme ist in das Brett eingerechnet."),
      t("Buttons, pockets, chest piece and the pen are left out.", "Knöpfe, Taschen, Bruststück und Stift entfallen."),
      t("The stethoscope tube is an open line at its true 10 mm width (0.1 mm pen); the board is drawn at 4 mm.", "Der Stethoskopschlauch ist eine offene Linie in wahrer Breite von 10 mm (0,1-mm-Stift); das Brett ist mit 4 mm gezeichnet."),
      t("No hatching: the white coat is drawn plain.", "Keine Schraffur: der weiße Kittel bleibt glatt."),
      t("Body 0.2 mm, detail 0.1 mm; solid views show the white coat against the white shirt front by tone.", "Körper 0,2 mm, Detail 0,1 mm; Flächenansichten zeigen den weißen Kittel gegen die weiße Hemdbrust durch den Ton."),
      t("Overlays map one region per shape; the tube line carries the stethoscope region.", "Überlagerungen bilden eine Region pro Form ab; die Schlauchlinie trägt die Stethoskopregion."),
    ),
    "50": D(
      t("Full figure 36 mm tall: face in profile and front, ears, hair, lab coat with collar, lapels, three buttons, two flap pockets, back vent and side seam, shirt front, sleeves with shirt cuffs, fingered hands, the stethoscope round the neck with its chest piece, the A4 clipboard with paper and clip, the pen in the left hand.", "Vollständige Figur von 36 mm Höhe: Gesicht im Profil und von vorn, Ohren, Haar, Laborkittel mit Kragen, Revers, drei Knöpfen, zwei Pattentaschen, Rückenschlitz und Seitennaht, Hemdbrust, Ärmel mit Hemdmanschetten, Hände mit Fingern, das Stethoskop um den Hals mit seinem Bruststück, das A4-Klemmbrett mit Papier und Klemme, der Stift in der linken Hand."),
      t("Every garment edge the 1:50 body has, the steel clip and the chest piece.", "Jede Kleidungskante des 1:50-Körpers, die Stahlklemme und das Bruststück."),
      t("Nothing is merged apart from the fingers of the closed right hand round the board edge.", "Nichts ist zusammengefasst außer den Fingern der um die Brettkante geschlossenen rechten Hand."),
      t("Earpieces and the tube behind the neck are hidden by the collar in the side and front views.", "Ohrbügel und der Schlauch hinter dem Nacken sind in Seiten- und Vorderansicht vom Kragen verdeckt."),
      t("The tube is a 10 mm (0.2 mm) band, the board 3 mm with a 1 mm paper layer as thin outlines, the pen a 9 mm bar.", "Der Schlauch ist ein Band von 10 mm (0,2 mm), das Brett 3 mm mit 1 mm Papier als dünne Umrisse, der Stift ein Stab von 9 mm."),
      t("No hatching: white cotton twill is drawn plain.", "Keine Schraffur: weißer Baumwollköper bleibt glatt."),
      t("Body 0.25 mm, detail 0.13 mm, texture 0.1 mm; solid views separate white coat, shirt, skin, dark chinos, steel and the dark tube.", "Körper 0,25 mm, Detail 0,13 mm, Textur 0,1 mm; Flächenansichten trennen weißen Kittel, Hemd, Haut, dunkle Hose, Stahl und den dunklen Schlauch."),
      t("Every shape carries one region; the thermal view shows the warm face and hands against the insulated coat, the passive board at air temperature.", "Jede Form trägt eine Region; die Thermikansicht zeigt warmes Gesicht und warme Hände gegen den gedämmten Kittel, das passive Brett bei Lufttemperatur."),
    ),
  } as Record<string, J>,
  volume: {
    "500": [t("1:500 body: the commuter's coarse figure re-dressed: the coat lofts become the white lab coat, the scarf ring becomes the merged collar and neck; the clipboard is one 4 mm sheet box in the right hand and the stethoscope one slender tube loft over the shoulders (true 10 mm section, declared slender); no bag.", "Körper 1:500: die grobe Figur des Pendlers neu eingekleidet: die Mantel-Lofts werden zum weißen Laborkittel, der Schalring zum zusammengefassten Kragen mit Hals; das Klemmbrett ist ein 4-mm-Plattenquader in der rechten Hand und das Stethoskop ein schlanker Schlauch-Loft über den Schultern (wahrer Querschnitt 10 mm, als schlank erklärt); keine Tasche.")],
    "200": [t("1:200 body: coat, neck, head and hair as at 1:200 of the commuter without scarf and bag; the clipboard a 4 mm sheet, the stethoscope a slender 10 mm tube laid 2 mm into the coat surface of this scale.", "Körper 1:200: Kittel, Hals, Kopf und Haar wie im Maßstab 1:200 des Pendlers ohne Schal und Tasche; das Klemmbrett eine 4-mm-Platte, das Stethoskop ein schlanker 10-mm-Schlauch, 2 mm in die Kitteloberfläche dieses Maßstabs gelegt.")],
    "100": [t("1:100 body at a 15 mm feature: lab coat with lapels and the shirt front in the opening (the scarf front piece re-cut as shirt), collar, hands with every phalanx; the clipboard one 4 mm sheet (paper merged, clip below the feature), the stethoscope tube slender; chest piece and pen are below the feature and left out.", "Körper 1:100 bei 15 mm kleinstem Merkmal: Laborkittel mit Revers und der Hemdbrust in der Öffnung (das Schalvorderteil als Hemd neu geschnitten), Kragen, Hände mit allen Fingergliedern; das Klemmbrett eine 4-mm-Platte (Papier eingerechnet, Klemme unter dem Merkmal), der Stethoskopschlauch schlank; Bruststück und Stift liegen unter dem Merkmal und entfallen.")],
    "50": [t("1:50 body: the lab coat cut like the overcoat it derives from (body, skirt halves that swing with the thighs, centre strip, lapels, collar, buttons, flap pockets) in four layer shells (undershirt, shirt, pullover, lab coat); the 3 mm hardboard with a 1 mm paper sheet and a 10 mm steel clip, the 9 mm pen across the left hand, the 10 mm stethoscope tube from the right chest over both shoulders and behind the neck to the left chest with a 45 mm steel chest piece.", "Körper 1:50: der Laborkittel geschnitten wie der Mantel, von dem er abstammt (Oberteil, Rockhälften, die mit den Oberschenkeln schwingen, Mittelstreifen, Revers, Kragen, Knöpfe, Pattentaschen), in vier Schichtschalen (Unterhemd, Hemd, Pullover, Laborkittel); die 3-mm-Hartfaserplatte mit 1 mm Papier und einer 10-mm-Stahlklemme, der 9-mm-Stift quer durch die linke Hand, der 10-mm-Stethoskopschlauch von der rechten Brust über beide Schultern und hinter dem Nacken zur linken Brust mit einem 45-mm-Bruststück aus Stahl.")],
  } as Record<string, J[]>,
  symbols: [
    {
      id: "standing-person-marker", title: t("Standing person marker", "Stehende Person als Marke"), description: t("Position dot with a short facing tick for ward plans at 1:500 and smaller.", "Positionspunkt mit kurzem Blickrichtungsstrich für Stationspläne ab 1:500."),
      semantics: "true-position", units: "paper-mm", denominators: [500], pens: { body: 0.15, detail: 0.15, texture: null, glazing: null, ground: 0.18 }, hatches: [],
      geometry: {
        elevation: [{ id: "dot", bone: "pelvis", role: "body", z: 1, filled: true, tone: 0.9, kind: "ellipse", cx: 0, cy: 0, rx: 0.45, ry: 0.45 }],
        front: [{ id: "front-dot", bone: "pelvis", role: "body", z: 1, filled: true, tone: 0.9, kind: "ellipse", cx: 0, cy: 0, rx: 0.45, ry: 0.45 }],
        plan: [{ id: "plan-dot", bone: "pelvis", role: "body", z: 1, filled: true, tone: 0.9, kind: "ellipse", cx: 0, cy: 0, rx: 0.45, ry: 0.45 }, { id: "plan-tick", bone: "pelvis", role: "detail", z: 2, filled: false, tone: 0, kind: "poly", points: [[0.45, 0], [1.3, 0]], closed: false }],
      },
      dimensionalNote: t("A conventional sign in paper millimetres, not to scale: the dot marks where the person stands, the tick the direction the person faces.", "Ein konventionelles Zeichen in Papiermillimetern, nicht maßstäblich: Der Punkt markiert den Standort, der Strich die Blickrichtung."),
    },
  ],
  notes: t(
    "Derived by HW-03 from walking-commuter (rig with spine, shoulder girdle and fifteen finger bones per hand, de Leva male segment fractions, the knee-length coat cut and its traced outlines at four scales): the overcoat becomes a white lab coat over shirt and pullover, scarf and bag are removed, a stethoscope and a clipboard with paper, clip and pen are added in every scale and volume, the poses are solved on the rig so the board centre and the pen tip meet authored targets. Generator: ticket STAFFAGE-HUMAN-COLLECTION-WAVE, scripts HW-03/doctor.ts.",
    "Von HW-03 aus walking-commuter abgeleitet (Skelett mit Wirbelsäule, Schultergürtel und fünfzehn Fingerknochen je Hand, männliche Segmentanteile nach de Leva, der Schnitt des knielangen Mantels und seine nachgezeichneten Umrisse in vier Maßstäben): Der Mantel wird zum weißen Laborkittel über Hemd und Pullover, Schal und Tasche entfallen, Stethoskop und Klemmbrett mit Papier, Klemme und Stift kommen in jedem Maßstab und Volumen dazu, die Haltungen sind am Skelett gelöst, sodass Brettmitte und Stiftspitze vorgegebene Ziele treffen. Generator: Ticket STAFFAGE-HUMAN-COLLECTION-WAVE, Skripte HW-03/doctor.ts.",
  ),
  references: [REFS.incropera, REFS.iso216, REFS.iso7730b, REFS.ainsworth],
};
