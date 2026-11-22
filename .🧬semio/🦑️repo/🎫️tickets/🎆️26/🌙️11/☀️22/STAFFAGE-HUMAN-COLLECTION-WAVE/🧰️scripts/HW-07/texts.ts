/** 📝️ The bilingual decision texts of the HW-07 people. The body drawings and volumes of all three are derived from the catalog jogger (same athletic
 * figure, rescaled), so the per-scale body decisions are stated once here and completed per person by the kit (clothing names) and the prop
 * (racket, ball, mat) of each scale.
 * @see ./tennis-player.ts @see ./basketball-player.ts @see ./yoga-practitioner.ts */
import type { T2 } from "./lib.ts";

export type Kit = { top: T2; shorts: T2; feet: T2; band: T2 };
export type PropText = Partial<Record<"silhouette" | "retained" | "merged" | "omitted" | "thinElements" | "hatching" | "styles" | "overlays", T2>>;
const j = (a: T2, b?: T2): T2 => (b ? { en: `${a.en} ${b.en}`, de: `${a.de} ${b.de}` } : a);

/** 🖊️ The eight representation decisions of one scale: the inherited athletic figure in the person's kit, plus the person's prop. */
export function decisions(scale: string, kit: Kit, prop: PropText): Record<string, T2> {
  const k = kit;
  const base: Record<string, Record<string, T2>> = {
    "50": {
      silhouette: { en: `Full articulation: face profile with eye, brow, ear and mouth line, hair under the ${k.band.en}, ${k.top.en} with collar and hem, hands with finger lines and thumb, ${k.shorts.en} with hem, ${k.feet.en}. Front and plan are drawn in full detail as well: tapered clothing outlines, hands with a thumb form, hair mass, ears and face features; every limb keeps its bone binding, and every part on a bone that a pose turns out of its rest plane (trunk, neck, head, legs) is an extruded solid, so turned shoulders and spread legs read right in all three views.`, de: `Volle Gliederung: Gesichtsprofil mit Auge, Braue, Ohr und Mundlinie, Haar unter dem ${k.band.de}, ${k.top.de} mit Kragen und Saum, Hände mit Fingerlinien und Daumen, ${k.shorts.de} mit Saum, ${k.feet.de}. Vorderansicht und Grundriss sind ebenso detailliert: konisch zulaufende Kleidungsumrisse, Hände mit Daumenform, Haarmasse, Ohren und Gesichtszüge; jedes Glied bleibt an seinen Knochen gebunden, und jedes Teil auf einem Knochen, den eine Pose aus seiner Ruheebene dreht (Rumpf, Hals, Kopf, Beine), ist ein extrudierter Körper, damit gedrehte Schultern und gespreizte Beine in allen drei Ansichten stimmen.` },
      retained: { en: `Silhouettes 0.25 mm, seams and hems 0.18 mm, hair strands and texture 0.13 mm; every clothing layer keeps its own region.`, de: `Umrisse 0,25 mm, Nähte und Säume 0,18 mm, Haarsträhnen und Textur 0,13 mm; jede Kleidungsschicht behält ihre eigene Region.` },
      merged: { en: `Thigh skin and the ${k.shorts.en} overlap with a hem line instead of two outlines; only fabric texture and the finest stitching are merged.`, de: `Oberschenkelhaut und ${k.shorts.de} überlappen mit einer Saumlinie statt zwei Umrissen; nur Stofftextur und feinste Nähte sind zusammengefasst.` },
      omitted: { en: `Nostril, eyelashes, separate fingers and fabric folds below 0.1 mm of paper.`, de: `Nasenloch, Wimpern, einzelne Finger und Stofffalten unter 0,1 mm Papiermass.` },
      thinElements: { en: `Finger lines are 0.13 mm detail strokes; open detail strokes on turning bones are closed 2 mm ribbons so they turn with their solid.`, de: `Fingerlinien sind 0,13-mm-Detailstriche; offene Detailstriche auf drehenden Knochen sind geschlossene 2-mm-Bänder, damit sie mit ihrem Körper drehen.` },
      hatching: { en: `The knit of the ${k.top.en} keeps the fine line hatch of the figure.`, de: `Die Strickware des ${k.top.de} behält die feine Linienschraffur der Figur.` },
      styles: { en: `Silhouettes 0.25 mm, seams 0.18 mm, texture 0.13 mm; tones follow the kit.`, de: `Umrisse 0,25 mm, Nähte 0,18 mm, Textur 0,13 mm; die Tonwerte folgen der Kleidung.` },
      overlays: { en: `Every region is its own shape or a detail line on it, in all three views, so thermal, structural and transport overlays work from every viewpoint.`, de: `Jede Region ist eine eigene Form oder eine Detaillinie darauf, in allen drei Ansichten, damit Thermik-, Tragwerks- und Strömungsüberlagerungen aus jeder Blickrichtung funktionieren.` },
    },
    "100": {
      silhouette: { en: `Articulated clothing outlines: ${k.top.en} with short sleeves, ${k.shorts.en} over the thigh, bare lower leg, ${k.feet.en}, head with face profile and hair cap, hand as a mitten with thumb. Front and plan are articulated outlines for this scale; parts on bones that turn out of their rest plane are extruded solids.`, de: `Gegliederte Kleidungsumrisse: ${k.top.de} mit kurzen Ärmeln, ${k.shorts.de} über dem Oberschenkel, nackter Unterschenkel, ${k.feet.de}, Kopf mit Gesichtsprofil und Haarkappe, Hand als Fäustling mit Daumen. Vorderansicht und Grundriss sind gegliederte Umrisse für diesen Massstab; Teile auf Knochen, die aus ihrer Ruheebene drehen, sind extrudierte Körper.` },
      retained: { en: `Sleeve and bare arm, hem on the thigh, heel and toe of the foot, nose, chin and hair mass, the ${k.band.en}.`, de: `Ärmel und nackter Arm, Saum am Oberschenkel, Ferse und Zehen des Fusses, Nase, Kinn und Haarmasse, das ${k.band.de}.` },
      merged: { en: `Neck and head share one outline; each hand is one outline.`, de: `Hals und Kopf teilen einen Umriss; jede Hand ist ein Umriss.` },
      omitted: { en: `Eye, brow, ear, mouth line, fingers, seams and hair strands.`, de: `Auge, Braue, Ohr, Mundlinie, Finger, Nähte und Haarsträhnen.` },
      thinElements: { en: `The ${k.band.en} is one 0.13 mm detail line; the thumb stays a separate form (about 0.25 mm).`, de: `Das ${k.band.de} ist eine 0,13-mm-Detaillinie; der Daumen bleibt eine eigene Form (etwa 0,25 mm).` },
      hatching: { en: `No body hatching: tone fills carry the materials.`, de: `Keine Körperschraffur: Tonflächen tragen die Materialien.` },
      styles: { en: `0.2 mm outlines, 0.13 mm detail lines; tones follow the kit.`, de: `0,2-mm-Umrisse, 0,13-mm-Detaillinien; die Tonwerte folgen der Kleidung.` },
      overlays: { en: `Regions map one-to-one onto the shapes in all three views.`, de: `Die Regionen entsprechen in allen drei Ansichten eins zu eins den Formen.` },
    },
    "200": {
      silhouette: { en: `Main masses: head with hair cap, ${k.top.en} trunk, ${k.shorts.en}, two arm segments with the hand as a tapered tip, thigh and shank per side and a foot of two parts. Front and plan show the same masses designed for this scale.`, de: `Hauptmassen: Kopf mit Haarkappe, Rumpf im ${k.top.de}, ${k.shorts.de}, zwei Armsegmente mit der Hand als spitzem Ende, Oberschenkel und Unterschenkel je Seite und ein zweiteiliger Fuss. Vorderansicht und Grundriss zeigen dieselben Massen für diesen Massstab.` },
      retained: { en: `The separate masses head, hair, top, shorts, sleeve, bare arm, bare leg and foot with their tones; the shoulder and hip widths of the real figure.`, de: `Die getrennten Massen Kopf, Haar, Oberteil, Shorts, Ärmel, nackter Arm, nacktes Bein und Fuss mit ihren Tonwerten; Schulter- und Hüftbreite der realen Figur.` },
      merged: { en: `Hand and forearm form one tapered segment; neck and head one outline.`, de: `Hand und Unterarm bilden ein spitz zulaufendes Segment; Hals und Kopf einen Umriss.` },
      omitted: { en: `Thumb, fingers, face details, seams and every fold (below 0.13 mm of paper or without meaning at 1:200).`, de: `Daumen, Finger, Gesichtszüge, Nähte und alle Falten (unter 0,13 mm Papiermass oder ohne Bedeutung bei 1:200).` },
      thinElements: { en: `The wrist and the hand tip are a continuous taper of the arm; nothing of the body is thickened.`, de: `Handgelenk und Handspitze sind eine durchgehende Verjüngung des Arms; nichts am Körper ist verdickt.` },
      hatching: { en: `No hatching; tone alone separates the materials.`, de: `Keine Schraffur; der Tonwert allein trennt die Materialien.` },
      styles: { en: `0.18 mm body pen; tones follow the kit.`, de: `0,18-mm-Körperstift; die Tonwerte folgen der Kleidung.` },
      overlays: { en: `Each mass carries its own region, so overlays show separate values for skin and clothing.`, de: `Jede Masse trägt ihre eigene Region, sodass die Überlagerungen getrennte Werte für Haut und Kleidung zeigen.` },
    },
    "500": {
      silhouette: { en: `One solid silhouette in overlapping articulated forms so the clips still animate: head, trunk and pelvis, and per side an upper arm, a forearm with fist, a thigh, a shank and a foot. Front and plan are each one composed silhouette designed for the paper size.`, de: `Eine geschlossene Silhouette aus überlappenden, gegliederten Formen, damit die Clips weiter animieren: Kopf, Rumpf und Becken sowie je Seite Oberarm, Unterarm mit Faust, Oberschenkel, Unterschenkel und Fuss. Vorderansicht und Grundriss sind je eine für die Papiergrösse komponierte Silhouette.` },
      retained: { en: `The posture of the default pose and its true height, width and depth.`, de: `Die Haltung der Standardpose und ihre wahre Höhe, Breite und Tiefe.` },
      merged: { en: `Neck, head and hair, top, shorts and pelvis form one body; hand and fist merge into the forearm. Parts closer than the minimum gap are drawn as one.`, de: `Hals, Kopf und Haar, Oberteil, Shorts und Becken bilden einen Körper; Hand und Faust verschmelzen mit dem Unterarm. Teile, die näher als der Mindestabstand liegen, werden als eines gezeichnet.` },
      omitted: { en: `Face, ears, fingers, thumb, the ${k.band.en}, seams and the far-hand outline.`, de: `Gesicht, Ohren, Finger, Daumen, das ${k.band.de}, Nähte und der Umriss der fernen Hand.` },
      thinElements: { en: `Fingers fall far below the pen width (0.15 mm = 7.5 cm) and are dropped.`, de: `Finger liegen weit unter der Stiftbreite (0,15 mm = 7,5 cm) und entfallen.` },
      hatching: { en: `No hatching: the figure is solid.`, de: `Keine Schraffur: die Figur ist flächig.` },
      styles: { en: `Dark solid fill with a 0.15 mm outline for every part; line weight is the only hierarchy.`, de: `Dunkle Vollfläche mit 0,15-mm-Umriss für jedes Teil; die Strichstärke ist die einzige Hierarchie.` },
      overlays: { en: `Merged forms list all their regions and are painted area-weighted.`, de: `Zusammengefasste Formen nennen alle ihre Regionen und werden flächengewichtet eingefärbt.` },
    },
  };
  const b = base[scale]!;
  return Object.fromEntries(Object.entries(b).map(([f, t]) => [f, j(t, prop[f as keyof PropText])]));
}

/** 🧊️ The three volume decisions (body, thermal, structure) of one scale, completed by the person's own sentences for each list. */
export function volumeDecisions(scale: string, who: T2, extra: { body?: T2; thermal?: T2; structure?: T2 }): T2[] {
  const feature: Record<string, string> = { "50": "7.5", "100": "15", "200": "30", "500": "75" };
  const f = feature[scale]!;
  const body: Record<string, T2> = {
    "50": { en: `Body 1:50: ${who.en}, rebuilt from anthropometric stations (DIN 33402-2, ANSUR II girths) as lofts of superelliptic sections; head with jaw, nose, ears and eyes, hands with palm, four two-segment fingers and a two-segment thumb; every limb end is rounded into its neighbour so no joint opens in any pose, and the top is cut at T12/L1 and L5/S1 so the trunk folds on its vertebral joints.`, de: `Körper 1:50: ${who.de}, aus anthropometrischen Stationen (DIN 33402-2, ANSUR-II-Umfänge) als Lofts superelliptischer Schnitte aufgebaut; Kopf mit Kiefer, Nase, Ohren und Augen, Hände mit Handteller, vier zweigliedrigen Fingern und zweigliedrigem Daumen; jedes Gliedende ist in sein Nachbarglied gerundet, damit sich in keiner Pose ein Gelenk öffnet, und das Oberteil ist bei T12/L1 und L5/S1 geteilt, damit sich der Rumpf in seinen Wirbelgelenken faltet.` },
    "100": { en: `Body 1:100: the same person for a ${f} mm feature: the same girths through fewer stations, the fingers as one curled blade beside a separate thumb, the head with nose but without ears and eyes.`, de: `Körper 1:100: dieselbe Person für ein Merkmal von ${f} mm: dieselben Umfänge mit weniger Stationen, die Finger als eine gekrümmte Platte neben einem eigenen Daumen, der Kopf mit Nase, aber ohne Ohren und Augen.` },
    "200": { en: `Body 1:200: masses of at least ${f} mm: one top, one sleeve per arm, the hand a single mitten, the head a skull with jaw under the hair cap; girths and joint positions as at 1:50.`, de: `Körper 1:200: Massen von mindestens ${f} mm: ein Oberteil, ein Ärmel je Arm, die Hand ein einzelner Fäustling, der Kopf ein Schädel mit Kiefer unter der Haarkappe; Umfänge und Gelenklagen wie bei 1:50.` },
    "500": { en: `Body 1:500: the figure in its fewest masses, each through two to four stations; the real girths are kept so the person reads the same at every scale.`, de: `Körper 1:500: die Figur in ihren wenigsten Massen mit je zwei bis vier Stationen; die realen Umfänge bleiben, damit die Person in jedem Massstab gleich gelesen wird.` },
  };
  const thermal: T2 = scale === "50"
    ? { en: `Thermal 1:50: every clothing region is its fabric over an enclosed air shell, the bare skin regions are the body solids themselves, the breath outlets and the plume are small solids at their anchors.`, de: `Thermik 1:50: jede Kleidungsregion ist ihr Stoff über einer eingeschlossenen Luftschale, die Hautregionen sind die Körperteile selbst, Atemöffnungen und Fahne sind kleine Körper an ihren Ankern.` }
    : { en: `Thermal 1:${scale}: one surface per region on the body solids of this scale; the breath outlets and the plume are small solids at their anchors.`, de: `Thermik 1:${scale}: eine Oberfläche je Region auf den Körperteilen dieses Massstabs; Atemöffnungen und Fahne sind kleine Körper an ihren Ankern.` };
  const structure: T2 = scale === "50"
    ? { en: `Structure 1:50: the skeleton as one connected load path from the skull over the column, ribcage, shoulder girdle and pelvis to the long bones, hands and feet; every member overlaps its neighbour at the joint and the feet stand on their contact anchors.`, de: `Tragwerk 1:50: das Skelett als ein zusammenhängender Lastpfad vom Schädel über Wirbelsäule, Brustkorb, Schultergürtel und Becken zu den Röhrenknochen, Händen und Füssen; jedes Glied überlappt sein Nachbarglied im Gelenk, und die Füsse stehen auf ihren Kontaktankern.` }
    : { en: `Structure 1:${scale}: the load path for a smallest feature of ${f} mm: what is smaller is merged into its neighbour, the long bones keep their measured shafts and are declared slender instead of being thickened.`, de: `Tragwerk 1:${scale}: der Lastpfad für ein kleinstes Merkmal von ${f} mm: Kleineres ist mit seinem Nachbarn zusammengefasst, die Röhrenknochen behalten ihre gemessenen Schäfte und sind als schlank deklariert statt verdickt.` };
  return [j(body[scale]!, extra.body), j(thermal, extra.thermal), j(structure, extra.structure)];
}
