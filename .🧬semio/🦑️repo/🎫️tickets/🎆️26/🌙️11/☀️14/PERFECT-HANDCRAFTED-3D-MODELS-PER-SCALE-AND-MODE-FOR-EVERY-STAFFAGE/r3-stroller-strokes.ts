/** 🖋️ The hand-stated strokes over the redrawn outlines of `humans/parent-with-stroller` (quilting seams, hem band, trouser hem, zip) as bone-local 3D points, and the bilingual decisions of each redrawn representation.
 * A seam is a ring around a garment: seen from the side it is the straight line from front to back at its station, seen from the front the line from side to side.
 * @see ./r3-stroller-draw2d.ts */
export type Stroke = { id: string; bone: string; parent: string; role: "detail" | "texture"; tone: number; region: string; material: string; views: string[]; pts: [number, number, number][] };
const seam = (bone: string, parent: string, i: number, u: number, rx: number, ry: number, cv: number, region: string, role: "detail" | "texture" = "texture"): Stroke[] => [
  { id: `${parent}-seam-${i}`, bone, parent, role, tone: 0.25, region, material: "cotton-clothing", views: ["elevation"], pts: [[u, cv - ry, 0], [u, cv + ry, 0]] },
  { id: `${parent}-seam-${i}`, bone, parent, role, tone: 0.25, region, material: "cotton-clothing", views: ["front"], pts: [[u, cv, -rx], [u, cv, rx]] },
];
const limb = (name: string, rows: [number, number, number, number][], region: string, role: "detail" | "texture" = "texture"): Stroke[] => ["l", "r"].flatMap((s) => rows.flatMap(([u, rx, ry, cv], i) => seam(`${name}-${s}`, `${name}-${s}`, i + 1, u, rx, ry, cv, region, role)));
const ZIP: Stroke = { id: "jacket-zip", bone: "torso", parent: "torso", role: "detail", tone: 0.2, region: "torso-clothing", material: "cotton-clothing", views: ["front"], pts: [[-0.02, -0.134, 0], [0.15, -0.143, 0], [0.26, -0.146, 0], [0.39, -0.114, 0], [0.47, -0.062, 0]] };
export const STROKES: Record<string, Stroke[]> = {
  "50": [
    ...([[0.03, 0.17, 0.125, -0.005], [0.105, 0.172, 0.127, -0.007], [0.18, 0.177, 0.13, -0.008], [0.255, 0.181, 0.132, -0.008], [0.33, 0.186, 0.124, -0.004]] as const).flatMap(([u, rx, ry, cv], i) => seam("torso", "torso", i + 1, u, rx, ry, cv, "torso-clothing")),
    ...seam("pelvis", "jacket-skirt", 2, -0.05, 0.172, 0.126, 0.006, "torso-clothing", "detail"),
    ...limb("upper-arm", [[0.09, 0.06, 0.064, 0.001], [0.165, 0.055, 0.058, 0.002], [0.24, 0.052, 0.054, 0.001]], "arm-clothing"),
    ...limb("forearm", [[0.075, 0.05, 0.05, 0.001], [0.15, 0.044, 0.043, 0]], "arm-clothing"),
    ...limb("shank", [[0.414, 0.053, 0.056, -0.001]], "leg-clothing", "detail"), ZIP,
  ],
  "100": [...seam("pelvis", "jacket-skirt", 1, -0.05, 0.172, 0.126, 0.006, "torso-clothing", "detail"), ZIP],
  "200": [], "500": [],
};
const d = (en: string, de: string): { en: string; de: string } => ({ en, de });
const overlays = d("All twelve thermal regions are drawn, each bound to its own outline; outlines follow the bones of the 3D body, so every pose and clip carries them.", "Alle zwölf thermischen Regionen sind gezeichnet, jede an ihren eigenen Umriss gebunden; die Umrisse folgen den Knochen des 3D-Körpers, sodass jede Pose und jeder Clip sie mitführt.");
export const REP_DECISIONS: Record<string, Record<string, { en: string; de: string }>> = {
  "50": {
    silhouette: d("The three views are the orthographic views of the 1:50 body in the pushing pose (ruling E1), one outline per part and bone: head with nose and ear, hair, scarf, collar, quilted jacket on the trunk and its skirt on the pelvis, sleeves with cuffs, hands closed round the push bar, jeans, sneakers with soles; the toddler with snowsuit, boots, hands, face and hat; the stroller with its chassis tubes per side, joint hubs and caster forks, seat sling, bumper bar, footrest, basket, two canopy halves and spoked wheels.", "Die drei Ansichten sind die orthografischen Ansichten des 1:50-Körpers in der Schiebepose (Regel E1), ein Umriss je Teil und Knochen: Kopf mit Nase und Ohr, Haar, Schal, Kragen, Steppjacke am Rumpf und ihr Schoß am Becken, Ärmel mit Bündchen, um den Schiebebügel geschlossene Hände, Jeans, Sneaker mit Sohlen; das Kleinkind mit Schneeanzug, Stiefeln, Händen, Gesicht und Mütze; der Kinderwagen mit Gestellrohren je Seite, Gelenknaben und Lenkrollengabeln, Sitzbespannung, Schutzbügel, Fußstütze, Korb, zwei Verdeckhälften und Speichenrädern."),
    retained: d("Quilting seams of jacket and sleeves, hem band, zip, trouser hem, eyes and lips, six spokes per rear wheel and five per front wheel.", "Steppnähte von Jacke und Ärmeln, Saumbund, Reißverschluss, Hosensaum, Augen und Lippen, sechs Speichen je Hinterrad und fünf je Vorderrad."),
    merged: d("Fingers and thumb of a hand are one outline; the tubes of one chassis side are one outline; rim, spokes and hub of a wheel are one outline inside the tyre ring.", "Finger und Daumen einer Hand sind ein Umriss; die Rohre einer Gestellseite sind ein Umriss; Felge, Speichen und Nabe eines Rads sind ein Umriss im Reifenring."),
    omitted: d("Harness straps, brake cable, laces, basket mesh.", "Gurtbänder, Bremszug, Schnürsenkel, Korbnetz."),
    thinElements: d("Tubes are drawn at their real diameters (18 to 30 mm = 0.36 to 0.6 mm on paper); the canopy bows (12 mm) are part of the canopy outline.", "Rohre sind mit realem Durchmesser gezeichnet (18 bis 30 mm = 0,36 bis 0,6 mm auf dem Papier); die Verdeckbügel (12 mm) gehören zum Verdeckumriss."),
    hatching: d("Knit hatch on the jacket, weave hatch on the jeans.", "Strickschraffur auf der Jacke, Webschraffur auf den Jeans."),
    styles: d("Body 0.35, detail 0.25, texture 0.13 mm; tyres near black, jeans dark, jacket mid tone, shoes and skin light.", "Körper 0,35, Detail 0,25, Textur 0,13 mm; Reifen fast schwarz, Jeans dunkel, Jacke mittel, Schuhe und Haut hell."),
    overlays,
  },
  "100": {
    silhouette: d("Orthographic views of the 1:100 body: head with nose and ear, hair mass, scarf, collar, smooth jacket and skirt, sleeves, fists with one finger slab and thumb, jeans, shoes; the toddler as trunk, legs, arms, hands, head and hat; the stroller with chassis tubes, hubs, casters, seat sling, bumper bar, footrest, basket, canopy halves and wheels with web discs.", "Orthografische Ansichten des 1:100-Körpers: Kopf mit Nase und Ohr, Haarmasse, Schal, Kragen, glatte Jacke mit Schoß, Ärmel, Fäuste mit einer Fingerplatte und Daumen, Jeans, Schuhe; das Kleinkind als Rumpf, Beine, Arme, Hände, Kopf und Mütze; der Kinderwagen mit Gestellrohren, Naben, Lenkrollen, Sitzbespannung, Schutzbügel, Fußstütze, Korb, Verdeckhälften und Rädern mit Scheiben."),
    retained: d("Hem band of the jacket and the zip.", "Saumbund der Jacke und Reißverschluss."),
    merged: d("The four fingers are one slab; shoe and sole are one last; tyre is a ring around one web disc with hub; cuffs are part of the sleeve.", "Die vier Finger sind eine Platte; Schuh und Sohle sind ein Leisten; der Reifen ist ein Ring um eine Scheibe mit Nabe; Bündchen gehören zum Ärmel."),
    omitted: d("Quilting (6 mm), eyes, lips, spokes, canopy bows, seat wings, clamps, fork prongs.", "Steppung (6 mm), Augen, Lippen, Speichen, Verdeckbügel, Sitzwangen, Klemmen, Gabelzinken."),
    thinElements: d("Tubes keep their real diameters (18 to 25 mm = 0.18 to 0.25 mm on paper).", "Rohre behalten ihre realen Durchmesser (18 bis 25 mm = 0,18 bis 0,25 mm auf dem Papier)."),
    hatching: d("None.", "Keine."),
    styles: d("Body 0.25, detail 0.15 mm; tyres near black, jeans dark, jacket mid tone.", "Körper 0,25, Detail 0,15 mm; Reifen fast schwarz, Jeans dunkel, Jacke mittel."),
    overlays,
  },
  "200": {
    silhouette: d("Orthographic views of the 1:200 body: head with neck, hair, scarf, jacket, pelvis, one outline per limb segment, fists, shoes; the toddler as trunk, legs, arms, head and hat; the stroller as push bar, legs and axle of 30 mm tubes, hubs, seat sling, basket block, canopy halves and disc wheels.", "Orthografische Ansichten des 1:200-Körpers: Kopf mit Hals, Haar, Schal, Jacke, Becken, ein Umriss je Gliedabschnitt, Fäuste, Schuhe; das Kleinkind als Rumpf, Beine, Arme, Kopf und Mütze; der Kinderwagen als Schiebebügel, Streben und Achse aus 30-mm-Rohren, Naben, Sitzbespannung, Korbblock, Verdeckhälften und Scheibenräder."),
    retained: d("Both canopy halves, the joint hubs, the toddler's hat.", "Beide Verdeckhälften, die Gelenknaben, die Mütze des Kleinkinds."),
    merged: d("Hand is one fist; nose, ears and collar merge into head and jacket; seat rails, casters, footrest and bumper bar merge into legs and sling; wheels are discs.", "Die Hand ist eine Faust; Nase, Ohren und Kragen gehen in Kopf und Jacke auf; Sitzrohre, Lenkrollen, Fußstütze und Schutzbügel gehen in Streben und Bespannung auf; Räder sind Scheiben."),
    omitted: d("All strokes inside the outlines.", "Alle Striche innerhalb der Umrisse."),
    thinElements: d("Tubes are thickened from 25 to 30 mm (0.15 mm on paper).", "Rohre sind von 25 auf 30 mm verdickt (0,15 mm auf dem Papier)."),
    hatching: d("None.", "Keine."),
    styles: d("One body pen; tones as at 1:100.", "Ein Körperstift; Töne wie bei 1:100."),
    overlays,
  },
  "500": {
    silhouette: d("Orthographic views of the 1:500 body: head, hair, jacket, pelvis, one mass per limb segment, fists and shoes; the toddler as one body mass and a head; the stroller as seat, canopy halves, basket, disc wheels and the chassis as centre-line strokes.", "Orthografische Ansichten des 1:500-Körpers: Kopf, Haar, Jacke, Becken, eine Masse je Gliedabschnitt, Fäuste und Schuhe; das Kleinkind als eine Körpermasse und Kopf; der Kinderwagen als Sitz, Verdeckhälften, Korb, Scheibenräder und das Gestell als Mittellinienstriche."),
    retained: d("Stride of the legs, both arms to the bar, canopy and wheels.", "Schrittstellung der Beine, beide Arme zum Bügel, Verdeck und Räder."),
    merged: d("Shoe and forefoot are one mass on the foot; scarf and collar merge into the jacket; the toddler's limbs merge into its body mass.", "Schuh und Vorfuß sind eine Masse am Fuß; Schal und Kragen gehen in der Jacke auf; die Glieder des Kleinkinds gehen in seiner Körpermasse auf."),
    omitted: d("Hubs, footrest, bumper bar, hat, every stroke inside the outlines.", "Naben, Fußstütze, Schutzbügel, Mütze, alle Striche innerhalb der Umrisse."),
    thinElements: d("The chassis tubes (25 mm = 0.05 mm on paper) are open strokes of the body pen on the real tube axes: one run from the rear axle over the joint and the push bar to the other side, and the two front legs.", "Die Gestellrohre (25 mm = 0,05 mm auf dem Papier) sind offene Striche des Körperstifts auf den realen Rohrachsen: ein Zug von der Hinterachse über das Gelenk und den Schiebebügel zur anderen Seite sowie die beiden Vorderstreben."),
    hatching: d("None.", "Keine."),
    styles: d("One 0.13 mm pen; jacket mid tone, jeans dark, canopy mid grey, wheels near black.", "Ein 0,13-mm-Stift; Jacke mittel, Jeans dunkel, Verdeck mittelgrau, Räder fast schwarz."),
    overlays,
  },
};
