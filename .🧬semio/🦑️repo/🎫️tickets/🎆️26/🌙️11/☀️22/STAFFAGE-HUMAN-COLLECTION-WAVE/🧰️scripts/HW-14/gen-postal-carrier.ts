/** 📮️ Generator of `postal-carrier`: derived from the rig, body, garments and gait of `walking-commuter` (as `standing-adult` was derived from `standing-phone-user`) and turned into a letter carrier with a mail satchel, a letter in the right hand, a hand-over and an address-reading pose and a delivery walk.
 *  bun gen-postal-carrier.ts [--out file.json]   (default: the catalog folder) */
import { writeFileSync } from "node:fs";
import { T, type Json, solidExtent, measureDimensions, loadBase, frames, writeAsset, removeBones, renameRegion, renameMaterial, renameBone, renameIds, removeShapes, SCALES, VIEWS, round } from "./lib.ts";
import { rescaleBone, flatShape, addShape, poseTrueLimb, splitUpright, retrace, patch } from "./props.ts";

const a: Json = loadBase("walking-commuter");
const out = process.argv.includes("--out") ? process.argv[process.argv.indexOf("--out") + 1] : undefined;

//#region identity
a.id = "postal-carrier";
a.title = T("Postal Carrier", "Postzustellende Person");
a.summary = T(
  "A letter carrier of 1.78 m and 72 kg on the walking round in a knee-length weatherproof service coat and scarf, with a 7 kg canvas mail satchel at the right hip and the next letter in the right hand: hands the letter over, reads the address, and walks the round heel to toe with the satchel swinging on its strap.",
  "Eine Briefzustellerin oder ein Briefzusteller von 1,78 m und 72 kg auf dem Zustellgang, in knielangem Wetterschutzmantel der Dienstkleidung und Schal, mit einer 7 kg schweren Posttasche aus Segeltuch an der rechten Hüfte und dem nächsten Brief in der rechten Hand: übergibt den Brief, liest die Anschrift und geht die Runde von der Ferse zur Zehe, während die Tasche am Riemen schwingt.",
);
a.tags = ["human", "adult", "service", "postal", "letter", "satchel", "delivery", "gait"];
a.provenance.author = "Semio staffage human worker HW-14";
a.provenance.created = "2026-11-22";
a.provenance.references.push(
  { id: "ainsworth-2011", citation: "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine and Science in Sports and Exercise 43(8), 1575-1581 (occupation codes for walking with a light load and standing tasks)." },
  { id: "din-33402-2", citation: "DIN 33402-2:2020-12 Ergonomics, human body dimensions, part 2: values (stature and segment dimensions of German adults, 18 to 65 years)." },
  { id: "iso-269", citation: "ISO 269:1985 Correspondence envelopes, designation and sizes (DL 110 x 220 mm)." },
);
a.provenance.notes = T(
  "Derived by HW-14 from the rig, body, garments, gait and volumes of walking-commuter (a 1.78 m, 72 kg man, 7.5 heads; de Leva 1996 male segment fractions plus 2.7 kg of clothing, segment lengths after Drillis and Contini, stature near the DIN 33402-2 male median of 1.75 m). Changed: the outfit is read as a weatherproof postal service coat over a fleece midlayer with service trousers and a service scarf; the shoulder bag became a canvas mail satchel 20 % deeper, 25 % wider and 65 % thicker in every drawing and volume (rescaled about its top edge and its inner face, so the strap, lugs and the coat contact stay in place) and carries 7 kg of mail as an applied load; a DL envelope (110 x 220 mm, 30 g) is a new bone fixed in the right hand and drawn in all three views and four scales; new poses hand the letter over at arm's length and read the address at chest height; the hand-over clip and the delivery walk replace the watch glance. Generator: T/🧰️scripts/HW-14/gen-postal-carrier.ts.",
  "Von HW-14 aus Rig, Körper, Kleidung, Gang und Volumen von walking-commuter abgeleitet (ein Mann von 1,78 m und 72 kg, 7,5 Kopfhöhen; männliche Segmentanteile nach de Leva 1996 plus 2,7 kg Kleidung, Segmentlängen nach Drillis und Contini, Körperhöhe nahe dem männlichen Median von 1,75 m nach DIN 33402-2). Geändert: die Kleidung ist ein Wetterschutzmantel der Postdienstkleidung über einer Fleece-Zwischenschicht mit Diensthose und Dienstschal; die Umhängetasche wurde zur Posttasche aus Segeltuch, in jeder Zeichnung und jedem Volumen 20 % tiefer, 25 % breiter und 65 % dicker (um ihre Oberkante und ihre Innenseite skaliert, sodass Riemen, Laschen und der Kontakt zum Mantel bleiben), und trägt 7 kg Post als aufgebrachte Last; ein DL-Umschlag (110 x 220 mm, 30 g) ist ein neuer, in der rechten Hand fester Knochen, in allen drei Ansichten und vier Maßstäben gezeichnet; neue Posen reichen den Brief mit gestrecktem Arm hin und lesen die Anschrift auf Brusthöhe; der Übergabe-Clip und der Zustellgang ersetzen den Blick auf die Uhr. Generator: T/🧰️scripts/HW-14/gen-postal-carrier.ts.",
);
//#endregion

//#region materials and regions
renameMaterial(a, "wool-coat", "service-coat");
renameMaterial(a, "wool-scarf", "service-scarf");
renameMaterial(a, "wool-trouser", "service-trouser");
renameMaterial(a, "leather-bag", "satchel-canvas");
renameMaterial(a, "wool-suit", "fleece-midlayer");
const mat = (id: string) => a.materials.find((m: Json) => m.id === id);
Object.assign(mat("service-coat"), { title: T("Weatherproof service coat (polyester-cotton, coated)", "Wetterschutzmantel der Dienstkleidung (Polyester-Baumwolle, beschichtet)"), emissivity: 0.9, solarAbsorptivity: 0.85, source: "authored-estimate", reference: "ashrae-2021" });
Object.assign(mat("service-scarf"), { title: T("Knitted service scarf", "Gestrickter Dienstschal"), emissivity: 0.92, solarAbsorptivity: 0.7 });
Object.assign(mat("service-trouser"), { title: T("Service trousers (polyester-wool twill)", "Diensthose (Polyester-Woll-Köper)"), emissivity: 0.91, solarAbsorptivity: 0.8, source: "authored-estimate", reference: "ashrae-2021" });
Object.assign(mat("satchel-canvas"), { title: T("Mail satchel (waxed canvas with leather trim)", "Posttasche (gewachstes Segeltuch mit Lederbesatz)"), emissivity: 0.9, solarAbsorptivity: 0.45, source: "authored-estimate" });
Object.assign(mat("fleece-midlayer"), { title: T("Polyester fleece midlayer", "Fleece-Zwischenschicht aus Polyester"), emissivity: 0.92, solarAbsorptivity: 0.8 });
a.materials.push({ id: "paper", title: T("Envelope paper", "Umschlagpapier"), emissivity: 0.93, solarAbsorptivity: 0.25, densityKgM3: 800, specificHeatJKgK: 1340, source: "literature", reference: "ashrae-2021" });

renameRegion(a, "coat", "service-coat");
renameRegion(a, "scarf", "service-scarf");
renameRegion(a, "trousers", "service-trousers");
renameRegion(a, "bag-leather", "satchel");
const reg = (id: string) => a.thermal.regions.find((r: Json) => r.id === id);
reg("service-coat").title = T("Service coat over fleece and shirt", "Dienstmantel über Fleece und Hemd");
const coatLayers = reg("service-coat").layers;
Object.assign(coatLayers.find((l: Json) => l.id === "shirt"), { title: T("Service shirt, long sleeve", "Diensthemd, langarm") });
Object.assign(coatLayers.find((l: Json) => l.id === "jacket"), { title: T("Fleece midlayer", "Fleece-Zwischenschicht"), resistanceM2KW: 0.0775, vapourResistanceM2PaW: 12.4, heatCapacityJM2K: 450 });
Object.assign(coatLayers.find((l: Json) => l.id === "overcoat"), { title: T("Weatherproof service coat (0.8 clo with its lining)", "Wetterschutz-Dienstmantel (0,8 clo mit Futter)"), resistanceM2KW: 0.124, vapourResistanceM2PaW: 32, heatCapacityJM2K: 900 });
reg("service-coat").heatCapacityJM2K = 13000 + 180 + 160 + 450 + 900;
reg("service-scarf").title = T("Service scarf over the neck", "Dienstschal über dem Hals");
reg("service-scarf").layers[0].title = T("Knitted service scarf", "Gestrickter Dienstschal");
reg("service-trousers").title = T("Service trousers over the legs", "Diensthose über den Beinen");
reg("service-trousers").layers.find((l: Json) => l.id === "trousers").title = T("Service trousers, straight, polyester-wool twill", "Diensthose, gerade, Polyester-Woll-Köper");
reg("shoes").title = T("Socks and leather service shoes", "Socken und lederne Dienstschuhe");
Object.assign(reg("satchel"), { title: T("Canvas mail satchel", "Posttasche aus Segeltuch"), heatCapacityJM2K: 1800 });
a.thermal.regions.push({ id: "letter-paper", title: T("Letter in the right hand", "Brief in der rechten Hand"), kind: "surface", model: "passive", material: "paper", areaM2: 0.0484, characteristicLengthM: 0.11, geometry: "vertical-plate", skyViewFactor: 0.4, layers: [], heatCapacityJM2K: 110, source: "authored-estimate", reference: "iso-269" });
a.thermal.assumptions = [
  a.thermal.assumptions[0],
  T("Clothing layers are garment values in clo after the ISO 9920 / ASHRAE tables (1 clo = 0.155 m2K/W, enclosed air included); the table digits are authored estimates: undershirt, service shirt, a fleece midlayer (0.5 clo) and the lined weatherproof coat (0.8 clo) on trunk and arms, service trousers over briefs on the legs; resistances are referred to the outer area (intrinsic value times f_cl = 1 + 1.97 I_cl); vapour resistance R/(0.0165 K/Pa x 0.38), the coated coat a little tighter.", "Kleidungsschichten sind Kleidungsstückwerte in clo nach den Tabellen von ISO 9920 / ASHRAE (1 clo = 0,155 m2K/W, eingeschlossene Luft enthalten); die Tabellenwerte sind geschätzt: Unterhemd, Diensthemd, eine Fleece-Zwischenschicht (0,5 clo) und der gefütterte Wetterschutzmantel (0,8 clo) an Rumpf und Armen, Diensthose über der Unterhose an den Beinen; Widerstände sind auf die Außenfläche bezogen (Eigenwert mal f_cl = 1 + 1,97 I_cl); Dampfwiderstand R/(0,0165 K/Pa x 0,38), der beschichtete Mantel etwas dichter."),
  a.thermal.assumptions[2],
  T("Activity after ISO 8996 and the Compendium of Physical Activities: standing and handing over 1.3 to 1.4 met, walking the round at 4 km/h with a 7 kg satchel about 3.0 met (level walking at 4 km/h is about 2.8 met, the carried load adds about 0.2 met); breathing patterns are authored estimates guided by quiet adult values (about 0.5 L, 12-15 per minute) and ICRP 66 light-exercise ventilation (about 25 L/min).", "Aktivität nach ISO 8996 und dem Compendium of Physical Activities: Stehen und Übergeben 1,3 bis 1,4 met, der Zustellgang mit 4 km/h und 7 kg Tasche etwa 3,0 met (Gehen in der Ebene mit 4 km/h etwa 2,8 met, die getragene Last fügt etwa 0,2 met hinzu); die Atemmuster sind geschätzt nach ruhigen Erwachsenenwerten (etwa 0,5 L, 12-15 je Minute) und der Lüftung bei leichter Belastung nach ICRP 66 (etwa 25 L/min)."),
  T("The mail satchel (waxed canvas, about 0.45 m2 with flap, lugs and strap) and the paper envelope (2 x 0.11 x 0.22 m) are passive surfaces without internal heat and are not part of the body model; neither carries a nameplate, so the total power is the metabolic heat of the body alone.", "Die Posttasche (gewachstes Segeltuch, etwa 0,45 m2 mit Klappe, Laschen und Riemen) und der Papierumschlag (2 x 0,11 x 0,22 m) sind passive Flächen ohne innere Wärme und nicht Teil des Körpermodells; keines trägt ein Typenschild, die Gesamtleistung ist also allein die Stoffwechselwärme des Körpers."),
];
//#endregion

//#region rig: satchel and letter
renameBone(a, "bag", "satchel");
renameIds(a, "bag", "satchel");
const satchel = a.rig.bones.find((b: Json) => b.id === "satchel");
satchel.title = T("Mail satchel on its strap", "Posttasche am Riemen");
for (const an of a.anchors) if (an.id === "bag-cog") { an.id = "satchel-cog"; an.title = T("Centre of gravity of the loaded satchel", "Schwerpunkt der beladenen Posttasche"); }
const rest = frames(a, null);
const SU = 1.2, SV = 1.25, SW = 1.65, U0 = 0.415, VC = -0.008, WI = -0.0605;
rescaleBone(a, "satchel", rest.satchel!, (u) => U0 + (u - U0) * SU, (v) => VC + (v - VC) * SV, (w) => 0.008 + (w >= -0.1455 ? WI + (w - WI) * SW : w + (WI + (-0.1455 - WI) * SW + 0.1455)), (id) => (id.includes("strap") || id.includes("joint") ? "" : id.includes("lug") ? "uw" : "uvw"));

const hand = a.rig.bones.find((b: Json) => b.id === "hand-r");
a.rig.bones.push({ id: "letter", parent: "hand-r", title: T("Letter (DL envelope)", "Brief (DL-Umschlag)"), x: 0.05, y: 0.004, angleDeg: -8, lengthM: 0.11, joint: { type: "fixed" }, mass: { kg: 0.03, comX: 0.02, comY: 0.08, distribution: "point", source: "authored-estimate", reference: "iso-269" } });
void hand;
const zLetter = rest["hand-r"]!.o.z;
const LETTER = [[-0.035, -0.03], [0.075, -0.03], [0.075, 0.19], [-0.035, 0.19]];
const DRAWN_LETTER = ["50", "100", "200", "500"];
for (const k of SCALES) {
  const vol = a.volumes[k];
  const sheet = { kind: "box", bone: "letter", material: "paper", size: [0.11, 0.22, 0.002], sheet: true, transform: { translation: [0.02, 0.08, 0] } };
  if (DRAWN_LETTER.includes(k)) {
    for (const v of ["elevation"]) addShape(a.representations[k].geometry[v], flatShape("letter", "letter", 9.5, 0.94, "letter-paper", "paper", LETTER, zLetter, 0.002));
    if (k === "50") a.representations[k].geometry.elevation.push({ id: "letter-flap-line", bone: "letter", role: "detail", z: 9.6, filled: false, tone: 0.2, region: "letter-paper", material: "paper", kind: "poly", points: [[-0.031, 0.186], [0.02, 0.14], [0.071, 0.186]], closed: false, solid: { kind: "extruded", sweep: "w", z0: round(zLetter - 0.001), z1: round(zLetter + 0.001) } });
    vol.body.push({ id: "letter", ...sheet, role: "body", tone: 0.94 });
    vol.thermal.push({ id: "th-letter", of: "letter", region: "letter-paper" });
    vol.structure.push({ id: "member-letter", of: "letter", member: "letter", part: "member" });
  } else {
    for (const v of VIEWS) {
      const list = a.representations[k].geometry[v];
      const host = list.find((x: Json) => x.bone === "hand-r") ?? list.find((x: Json) => x.bone === "forearm-r");
      host.regions = [...(host.regions ?? [host.region]), "letter-paper"];
      delete host.region;
    }
    vol.thermal.push({ id: "th-letter", ...sheet, role: "body", tone: 0.94, region: "letter-paper" });
    vol.structure.push({ id: "member-letter", ...sheet, role: "body", tone: 0.4, member: "letter", part: "member" });
  }
  if (k === "50") vol.structure.push({ id: "mass-letter", kind: "ellipsoid", bone: "letter", material: "paper", role: "detail", tone: 0.2, radii: [0.012, 0.012, 0.004], transform: { translation: [0.02, 0.08, 0] }, member: "letter", part: "mass" });
}
a.dimensions.massKg = round(a.rig.bones.reduce((s: number, b: Json) => s + (b.mass?.kg ?? 0), 0), 3);
a.anchors.push({ id: "letter-tip", bone: "letter", x: 0.02, y: 0.19, kind: "attachment", title: T("Far edge of the letter, where the recipient takes it", "Ferne Kante des Briefs, an der ihn die empfangende Person nimmt") });
//#endregion

//#region poses
const satchelLoad = () => [{ id: "satchel-load", kind: "carried", title: T("Mail satchel with 7 kg of letters and small parcels", "Posttasche mit 7 kg Briefen und Kleinsendungen"), anchor: "satchel-cog", fxN: 0, fyN: -68.65, source: "authored-estimate" }];
for (const p of a.poses) {
  p.loads = satchelLoad();
  p.thermal.regions["letter-paper"] = { exposedFraction: 0.85, sunlitFraction: 0 };
  p.thermal.regions.satchel.exposedFraction = 0.85;
}
const base = (id: string) => a.poses.find((p: Json) => p.id === id);
const standing = base("standing");
standing.title = T("Standing with the next letter", "Stehen mit dem nächsten Brief");
standing.description = T("Upright stance at a front door, the right fist on the satchel strap holding the next letter upright, the left arm hanging, both feet flat.", "Aufrechter Stand an einer Haustür, die rechte Faust am Riemen der Posttasche hält den nächsten Brief aufrecht, der linke Arm hängt, beide Füße flach.");
standing.bones["hand-r"] = { angleDeg: -25 };
standing.thermal.activityMet = 1.3;

const watch = base("check-watch");
a.poses = a.poses.filter((p: Json) => p !== watch);
const handOver = JSON.parse(JSON.stringify(standing));
Object.assign(handOver, {
  id: "handing-over-letter",
  title: T("Handing over a letter", "Einen Brief übergeben"),
  description: T("The right arm reaches forward and a little down to a recipient in the doorway, the elbow almost straight, the envelope held flat between thumb and fingers at its near edge; trunk and head lean slightly towards the recipient, the satchel hangs at the hip.", "Der rechte Arm reicht nach vorn und etwas nach unten zu einer Person in der Tür, der Ellbogen fast gestreckt, der Umschlag flach zwischen Daumen und Fingern an seiner nahen Kante gehalten; Rumpf und Kopf neigen sich leicht der empfangenden Person zu, die Tasche hängt an der Hüfte."),
  bones: { ...standing.bones, "arm-r": { angleDeg: 48 }, "forearm-r": { angleDeg: 22 }, "hand-r": { angleDeg: -20 }, neck: { angleDeg: 1 }, head: { angleDeg: -3 }, "clavicle-r": { angleDeg: 4 } },
});
handOver.thermal = { ...JSON.parse(JSON.stringify(standing.thermal)), activityMet: 1.4 };
const reading = JSON.parse(JSON.stringify(standing));
Object.assign(reading, {
  id: "reading-address",
  title: T("Reading the address", "Die Anschrift lesen"),
  description: T("The right forearm lifts the envelope to chest height in front of the body, its face turned up to the eyes; head and neck bow to read the address and house number before the letter goes into the slot or the hand.", "Der rechte Unterarm hebt den Umschlag vor dem Körper auf Brusthöhe, die Vorderseite den Augen zugewandt; Kopf und Hals neigen sich, um Anschrift und Hausnummer zu lesen, bevor der Brief in den Schlitz oder die Hand geht."),
  bones: { ...standing.bones, "arm-r": { angleDeg: 18 }, "forearm-r": { angleDeg: 95 }, "hand-r": { angleDeg: -30 }, torso: { angleDeg: -1.8 }, lumbar: { angleDeg: -1 }, neck: { angleDeg: -9 }, head: { angleDeg: -14 } },
});
reading.thermal = { ...JSON.parse(JSON.stringify(standing.thermal)), activityMet: 1.3 };
a.poses.splice(1, 0, handOver, reading);
for (const p of a.poses) if (p.thermal.breathing === "walking") p.thermal.activityMet = 3.0;
//#endregion

//#region clips
a.clips = a.clips.filter((c: Json) => c.id !== "glance-at-watch");
const walk = a.clips.find((c: Json) => c.id === "walk-cycle");
Object.assign(walk, {
  id: "delivery-walk",
  title: T("Walking the delivery round", "Den Zustellgang gehen"),
  description: T("Level walking between front doors at about 1.1 m/s (4 km/h), 100 steps per minute; feet planted without sliding, heel strike to toe-off with double support, the left arm swinging, the right fist holding the strap and the next letter, the loaded satchel swaying on its strap.", "Gehen in der Ebene zwischen Haustüren mit etwa 1,1 m/s (4 km/h), 100 Schritte pro Minute; Füße gleiten nicht, Fersenkontakt bis Abdruck mit Doppelstandphase, der linke Arm schwingt, die rechte Faust hält Riemen und nächsten Brief, die beladene Tasche pendelt am Riemen."),
});
for (const t of walk.tracks ?? []) if (t.target === "activityMet") t.keys = t.keys.map((k: Json) => ({ ...k, v: 3.0 }));
const idle = a.clips.find((c: Json) => c.id === "idle-breathing");
idle.title = T("Waiting at the door", "An der Tür warten");
idle.description = T("Standing at a front door after ringing, with the resting breathing pattern (13 per minute) and slight chest motion derived from the pattern; the letter stays ready in the right hand.", "Stehen an einer Haustür nach dem Klingeln, mit dem Ruheatemmuster (13 je Minute) und leichter, aus dem Muster abgeleiteter Brustbewegung; der Brief bleibt in der rechten Hand bereit.");
idle.phases = [{ id: "waiting", fromS: 0, toS: 4, title: T("Waiting for the door to open", "Warten, bis die Tür aufgeht") }];
const feet = ["heel-l", "ball-l", "heel-r", "ball-r"].map((anchor) => ({ anchor, fromS: 0, toS: 5 }));
a.clips.splice(1, 0, {
  id: "hand-over-letter",
  title: T("Reading the address and handing the letter over", "Anschrift lesen und Brief übergeben"),
  description: T("Standing at the door: the carrier lifts the letter to read the address, reaches out and holds it while the recipient takes it, then returns the hand to the satchel strap; both feet stay flat.", "Stehend an der Tür: die zustellende Person hebt den Brief, liest die Anschrift, reicht ihn hin und hält ihn, bis die empfangende Person ihn nimmt, dann kehrt die Hand an den Taschenriemen zurück; beide Füße bleiben flach."),
  durationS: 5, fps: 30, loop: false, rootMotion: "none",
  keys: [
    { t: 0, pose: "standing", ease: "smooth" },
    { t: 1, pose: "reading-address", ease: "smooth" },
    { t: 1.8, pose: "reading-address", ease: "smooth" },
    { t: 2.8, pose: "handing-over-letter", ease: "smooth" },
    { t: 3.8, pose: "handing-over-letter", ease: "smooth" },
    { t: 5, pose: "standing", ease: "smooth" },
  ],
  contacts: feet,
  phases: [
    { id: "read", fromS: 0, toS: 1.8, title: T("Lifting the letter and reading the address", "Brief heben und Anschrift lesen") },
    { id: "reach", fromS: 1.8, toS: 2.8, title: T("Reaching out", "Hinreichen") },
    { id: "hand-over", fromS: 2.8, toS: 3.8, title: T("Holding out until it is taken", "Hinhalten, bis er genommen wird") },
    { id: "return", fromS: 3.8, toS: 5, title: T("Back to the strap", "Zurück zum Riemen") },
  ],
  events: [
    { t: 1, kind: "motion", title: T("Address at reading distance", "Anschrift in Leseabstand") },
    { t: 2.8, kind: "motion", title: T("Letter held out at arm's length", "Brief mit gestrecktem Arm hingehalten") },
    { t: 3.8, kind: "load-off", title: T("The recipient takes the letter", "Die empfangende Person nimmt den Brief") },
  ],
  tracks: [
    { target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 1.3 }, { t: 2.8, v: 1.4 }, { t: 5, v: 1.3 }] },
    { target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 5, v: 0 }] },
  ],
});
a.presentation = { defaultPose: "standing", defaultClip: "delivery-walk", defaultScale: 100 };
//#endregion

//#region structure
a.structure.assumptions = [
  T("Segments are point masses at the de Leva centres; the 7 kg loaded satchel is an applied load at its centre of gravity that the strap carries to the near shoulder, the 30 g letter is a point mass of its fixed bone in the right hand.", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva; die 7 kg schwere beladene Posttasche ist eine Last in ihrem Schwerpunkt, die der Riemen auf die nahe Schulter bringt, der 30 g schwere Brief ist eine Punktmasse seines festen Knochens in der rechten Hand."),
  a.structure.assumptions[1],
  T("Handing over at arm's length raises the moment at the right shoulder to roughly the arm weight (about 3.6 kg with hand and letter) times its 0.3 m lever, about 11 N m; the satchel on the opposite strap side loads the right shoulder with about 69 N.", "Die Übergabe mit gestrecktem Arm erhöht das Moment an der rechten Schulter auf etwa das Armgewicht (rund 3,6 kg mit Hand und Brief) mal seinem Hebel von 0,3 m, etwa 11 N m; die Tasche belastet über den Riemen die rechte Schulter mit etwa 69 N."),
];
//#endregion

//#region drawings made pose-true for the presentation pose
const fingersR = a.rig.bones.filter((b: Json) => b.id.endsWith("-r") && /thumb|index|middle|ring|little/.test(b.id)).map((b: Json) => b.id);
const HAND_OUT = Number(process.env.HAND_OUT ?? 0);
const handSpan: Record<string, [number, number]> = {};
for (const k of SCALES) { const e = solidExtent(a, k, ["hand-r", ...fingersR]); handSpan[k] = [round(e.min.z - HAND_OUT, 4), round(e.max.z, 4)]; }
const [LIMB_DZ, LIMB_M, LIMB_DX] = [Number(process.env.LIMB_DZ ?? -0.006), Number(process.env.LIMB_M ?? 0.9), Number(process.env.LIMB_DX ?? 0)];
const limbAxes: Record<string, Record<string, [number, number, number]>> = {};
for (const b of ["arm-r", "forearm-r"]) { limbAxes[b] = {}; for (const k of SCALES) { const e = solidExtent(a, k, [b]); const arm = b === "arm-r";
limbAxes[b]![k] = [round((e.min.x + e.max.x) / 2 - rest[b]!.o.x + (arm ? LIMB_DX : 0), 4), round((e.min.z + e.max.z) / 2 - rest[b]!.o.z + LIMB_DZ, 4), round((arm ? LIMB_M : 1) * (e.max.z - e.min.z) / (e.max.x - e.min.x), 3)]; } }
poseTrueLimb(a, SCALES, ["arm-r", "forearm-r"], { "hand-r": handSpan }, limbAxes, { "arm-r": 0.015, "forearm-r": 0.06 });
const spineY = { torso: rest.torso!.o.y, lumbar: rest.lumbar!.o.y, pelvis: rest.pelvis!.o.y };
const coatTrunk = (s: Json) => s.material === "service-coat" && ["torso", "lumbar", "pelvis", "thigh-l", "thigh-r"].includes(s.bone);
for (const k of SCALES) { retrace(a, k, "front", "coat", coatTrunk); retrace(a, k, "plan", "coat", coatTrunk); }
for (const k of SCALES) for (const v of ["elevation", "front"] as const) if (!(k === "500" && v === "elevation")) splitUpright(a.representations[k].geometry[v], "coat", v, spineY, [{ y: rest.torso!.o.y, bone: "lumbar" }, { y: 0.97, bone: "pelvis" }]);
for (const side of ["r", "l"]) patch(a, "50", "elevation", `heel-fill-${side}`, `foot-${side}`, `shoe-rear-${side}`, [[-0.05, 0.106], [0.004, 0.097], [-0.05, 0.085]]);
patch(a, "50", "front", "coat-hand-gap-l", "pelvis", "coat-skirt", [[0.196, 0.835], [0.214, 0.835], [0.214, 0.758], [0.198, 0.758]]);
a.volumes["50"].body.push({ id: "satchel-strap-grip", kind: "box", bone: "hand-r", material: "satchel-canvas", role: "body", tone: 0.36, size: [0.036, 0.03, 0.032], cornerRadius: 0.004, transform: { translation: [0.065, 0, 0.058] } });
a.volumes["50"].thermal.push({ id: "th-satchel-strap-grip", of: "satchel-strap-grip", region: "satchel" });
for (const k of ["50", "100"]) {
  a.volumes[k].body.push({ id: "satchel-back-pad", kind: "box", bone: "satchel", material: "satchel-canvas", role: "body", tone: 0.5, size: [0.2, 0.3, 0.03], cornerRadius: 0.005, transform: { translation: [0.52, -0.008, -0.04] } });
  a.volumes[k].thermal.push({ id: "th-satchel-back-pad", of: "satchel-back-pad", region: "satchel" });
}
patch(a, "50", "front", "coat-arm-gap-r", "lumbar", "coat-waist", [[-0.227, 1.112], [-0.176, 1.107], [-0.18, 1.058], [-0.216, 1.058]]);
patch(a, "50", "front", "satchel-strap-front", "satchel", "satchel-body", [[-0.2562, 1.1437], [-0.25, 1.14], [-0.2469, 1.1294], [-0.2581, 1.0794], [-0.2606, 1.0794], [-0.2669, 1.0844], [-0.265, 1.0956], [-0.2694, 1.1062], [-0.2644, 1.11], [-0.2606, 1.1181], [-0.2587, 1.1287], [-0.2594, 1.1387]]);
const satchelBody = (x: Json) => x.bone === "satchel" && !x.id.includes("strap");
for (const k of SCALES) for (const v of VIEWS) retrace(a, k, v, "satchel-body", satchelBody);
for (const k of SCALES) for (const v of ["plan"] as const) retrace(a, k, v, "satchel-strap", (x: Json) => x.id === "satchel-strap");
patch(a, "100", "elevation", "strap-gap", "lumbar", "coat-waist", [[-0.1525, 1.10], [-0.1425, 1.10], [-0.125, 1.0925], [-0.1275, 1.0575], [-0.155, 1.06]]);
//#endregion

measureDimensions(a);
if (out) writeFileSync(out, JSON.stringify({ $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset: a }, null, 2) + "\n");
else console.log(writeAsset(a));
void removeBones; void removeShapes;
