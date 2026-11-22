/** 👋️ Generator of `waving-person`: derived from the rig, outlines and volumes of `standing-adult`, a distinct person who waves and hails with an open hand and wears a belt bag.
 *  bun build-waving-person.ts [--out file.json]
 * @see ../../📋️brief.md */
import { addShapes, ellipse, fillEmpty, measureDimensions, note, readAsset, reword, roundRect, scrub, t, writeAsset, type Bones, type Json } from "./lib.ts";

const a: Json = await readAsset("standing-adult");
const base = structuredClone(a.poses.find((p: Json) => p.id === "relaxed"));
const fingers = (side: "r" | "l", curl: number, spread: number): Bones => {
  const s = side === "r" ? 1 : -1, tw = side === "r" ? -90 : 90;
  const out: Bones = {
    [`thumb-metacarpal-${side}`]: { angleDeg: 6, abductionDeg: s * (10 + spread), twistDeg: tw },
    [`thumb-proximal-${side}`]: { angleDeg: 2 * curl, abductionDeg: s * 16 },
    [`thumb-distal-${side}`]: { angleDeg: 2 * curl, abductionDeg: s * 6 },
  };
  ["index", "middle", "ring", "little"].forEach((f, i) => {
    out[`${f}-proximal-${side}`] = { angleDeg: curl * (1 + i) - spread * (1.5 - i), abductionDeg: s * (3 + i), twistDeg: tw };
    out[`${f}-middle-${side}`] = { angleDeg: curl * (4 + i) };
    out[`${f}-distal-${side}`] = { angleDeg: curl * (2 + i * 0.5) };
  });
  return out;
};
const legs: Bones = { "thigh-l": { angleDeg: -2.5 }, "shank-l": { angleDeg: -2.5 }, "foot-l": { angleDeg: 5 }, "toe-l": { angleDeg: 0 }, "thigh-r": { angleDeg: -2.5 }, "shank-r": { angleDeg: -2.5 }, "foot-r": { angleDeg: 5 }, "toe-r": { angleDeg: 0 } };
const leftRelaxed: Bones = { "arm-l": { angleDeg: 0, abductionDeg: 6 }, "forearm-l": { angleDeg: 9 }, "hand-l": { angleDeg: 3 }, ...fingers("l", 4, 0) };
const waveArm = (sway: number): Bones => ({ "clavicle-r": { angleDeg: 8 }, "arm-r": { angleDeg: 12, abductionDeg: -66, twistDeg: 84 }, "forearm-r": { angleDeg: 108 - sway, twistDeg: 0 }, "hand-r": { angleDeg: -8 }, ...fingers("r", 0.5, 6) });
const pose = (id: string, title: Json, description: Json, bones: Bones, met: number, breathing: string, hoodie: number, extra: Json = {}): Json => ({
  ...structuredClone(base), id, title, description, bones: { ...legs, ...bones }, ...extra,
  thermal: { ...structuredClone(base.thermal), activityMet: met, breathing, regions: { ...structuredClone(base.thermal.regions), hoodie: { exposedFraction: hoodie, sunlitFraction: 0 }, "belt-bag": { exposedFraction: 0.75, sunlitFraction: 0 } } },
});

a.poses = [
  pose("waving", t("Waving hello", "Zum Gruß winken"), t("Feet at hip width, the right upper arm raised sideways to shoulder height, the elbow bent so the forearm stands upright, the open palm turned forward beside the head; the left arm hangs, the head is level and turned a little toward the person greeted.", "Füße hüftbreit, der rechte Oberarm seitlich auf Schulterhöhe gehoben, der Ellbogen gebeugt, sodass der Unterarm aufrecht steht, die offene Handfläche neben dem Kopf nach vorn gedreht; der linke Arm hängt, der Kopf ist gerade und ein wenig zur gegrüßten Person gewandt."),
    { lumbar: { angleDeg: 0.4 }, torso: { angleDeg: 0.6 }, neck: { angleDeg: 0, twistDeg: 0 }, head: { angleDeg: 2 }, ...leftRelaxed, ...waveArm(0) }, 1.7, "quiet-standing", 0.92),
  pose("hailing", t("Hailing with the arm stretched up", "Mit hochgestrecktem Arm herbeirufen"), t("Weight on both feet, the right arm stretched straight up and a little forward above the head with the open hand high, as when hailing a taxi or a friend across a square; the shoulder is lifted, the head tilted slightly back to look ahead.", "Gewicht auf beiden Füßen, der rechte Arm gestreckt nach oben und etwas nach vorn über den Kopf, die offene Hand hoch, wie beim Herbeirufen eines Taxis oder einer Freundin über den Platz; die Schulter ist gehoben, der Kopf leicht zurückgeneigt, der Blick geradeaus."),
    { lumbar: { angleDeg: -1 }, torso: { angleDeg: -1.5 }, neck: { angleDeg: -4 }, head: { angleDeg: -3 }, ...leftRelaxed, "clavicle-r": { angleDeg: 20 }, "arm-r": { angleDeg: 166, abductionDeg: -16 }, "forearm-r": { angleDeg: 8 }, "hand-r": { angleDeg: -12 }, ...fingers("r", 0.5, 4) }, 1.8, "talking", 0.92),
  pose("waving-both-arms", t("Waving with both arms", "Mit beiden Armen winken"), t("Both arms raised overhead in a wide V with open hands, the elbows almost straight, to be seen from far away; feet at hip width, head tilted up.", "Beide Arme in einem weiten V über den Kopf gehoben, die Hände offen, die Ellbogen fast gestreckt, um von weit her gesehen zu werden; Füße hüftbreit, Kopf leicht gehoben."),
    { lumbar: { angleDeg: -1 }, torso: { angleDeg: -1 }, neck: { angleDeg: -3 }, head: { angleDeg: -4 }, "clavicle-r": { angleDeg: 22 }, "clavicle-l": { angleDeg: 22 }, "arm-r": { angleDeg: 20, abductionDeg: -140 }, "forearm-r": { angleDeg: 12 }, "hand-r": { angleDeg: -6 }, "arm-l": { angleDeg: 20, abductionDeg: 140 }, "forearm-l": { angleDeg: 12 }, "hand-l": { angleDeg: -6 }, ...fingers("r", 0.5, 6), ...fingers("l", 0.5, 6) }, 2.3, "talking", 0.9),
  pose("standing-relaxed", t("Standing relaxed", "Entspannt stehen"), t("Feet at hip width, arms hanging loosely, head level: the moment before and after the greeting.", "Füße hüftbreit, Arme locker hängend, Kopf gerade: der Moment vor und nach dem Gruß."),
    { torso: { angleDeg: 0 }, neck: { angleDeg: -1 }, head: { angleDeg: 1 }, ...leftRelaxed, "arm-r": { angleDeg: 1, abductionDeg: -6 }, "forearm-r": { angleDeg: 9 }, "hand-r": { angleDeg: 4 }, ...fingers("r", 4, 0) }, 1.2, "quiet-standing", 0.92),
];

const BAG = { x0: 0.095, x1: 0.155, y0: 0.878, y1: 0.996, z: 0.115, r: 0.024, strapY0: 0.947, strapY1: 0.971 };
/** 👝️ The belt bag worn in front of the hips: a bone of its own on the pelvis (fixed), a rounded box in every view and scale, the waist strap at 1:50 and 1:100 (2D) and 1:50 (3D). */
function beltBag(asset: Json): void {
  const pelvis = asset.rig.bones.find((b: Json) => b.id === "pelvis");
  const cx = (BAG.x0 + BAG.x1) / 2, depth = BAG.x1 - BAG.x0, height = BAG.y1 - BAG.y0;
  asset.rig.bones.splice(asset.rig.bones.indexOf(pelvis) + 1, 0, { id: "belt-bag", parent: "pelvis", title: t("Belt bag worn in front of the hips", "Vor der Hüfte getragene Gürteltasche"), x: BAG.y0 - pelvis.y, y: -cx, angleDeg: 0, lengthM: height, joint: { type: "fixed" }, mass: { kg: 0.45, comX: height / 2, comY: 0, distribution: "point", source: "authored-estimate" } });
  asset.materials.push({ id: "nylon-belt-bag", title: t("Nylon belt bag with polyester webbing strap", "Gürteltasche aus Nylon mit Polyester-Gurtband"), emissivity: 0.9, solarAbsorptivity: 0.75, specificHeatJKgK: 1700, source: "authored-estimate" });
  asset.thermal.regions.push({ id: "belt-bag", title: t("Belt bag", "Gürteltasche"), kind: "surface", model: "passive", material: "nylon-belt-bag", areaM2: 0.085, characteristicLengthM: 0.23, geometry: "vertical-plate", skyViewFactor: 0.4, layers: [], heatCapacityJM2K: 7600, source: "authored-estimate" });
  const side = roundRect(0, height, -depth / 2, depth / 2, BAG.r);
  const front = roundRect(-BAG.z, BAG.z, BAG.y0, BAG.y1, BAG.r);
  const plan = roundRect(BAG.x0, BAG.x1, -BAG.z, BAG.z, BAG.r);
  const body = (id: string, points: number[][], z: number) => ({ id, bone: "belt-bag", role: "body", z, filled: true, tone: 0.28, region: "belt-bag", material: "nylon-belt-bag", kind: "poly", points, closed: true });
  const line = (id: string, points: number[][], z: number) => ({ id, bone: "belt-bag", role: "detail", z, filled: false, tone: 0.1, region: "belt-bag", kind: "poly", points, closed: false });
  for (const scale of [50, 100, 200]) {
    addShapes(asset, scale, "elevation", [body("belt-bag", side, 4.3)], "head");
    addShapes(asset, scale, "front", [body("belt-bag", front, 4.3)], "sleeve-l");
    addShapes(asset, scale, "plan", [body("belt-bag", plan, 4.3)], "hoodie");
  }
  for (const scale of [50, 100]) {
    addShapes(asset, scale, "front", [line("belt-bag-zip", [[-BAG.z + 0.02, BAG.y1 - 0.022], [0, BAG.y1 - 0.016], [BAG.z - 0.02, BAG.y1 - 0.022]], 4.31)], "sleeve-l");
  }
  const strapFront = roundRect(-0.18, 0.18, BAG.strapY0, BAG.strapY1, 0.004, 1);
  const strapSide = roundRect(BAG.strapY0 - BAG.y0, BAG.strapY1 - BAG.y0, cx - 0.115, cx + 0.135, 0.004, 1);
  for (const scale of [50]) {
    addShapes(asset, scale, "front", [{ ...body("belt-strap", strapFront, 4.25), tone: 0.2 }], "belt-bag");
    addShapes(asset, scale, "elevation", [{ ...body("belt-strap", strapSide, 4.25), tone: 0.2 }], "belt-bag");
  }
  const box = { id: "belt-bag", kind: "box", bone: "belt-bag", material: "nylon-belt-bag", role: "body", tone: 0.28, size: [height, depth, 2 * BAG.z], cornerRadius: BAG.r, transform: { translation: [height / 2, 0, 0] } };
  const strap = { id: "belt-strap", kind: "extrude", bone: "belt-bag", material: "nylon-belt-bag", role: "body", tone: 0.2, plane: "yz", profile: ellipse(cx + 0.01, 0, 0.125, 0.18, 32), holes: [ellipse(cx + 0.01, 0, 0.115, 0.17, 32)], from: BAG.strapY0 - BAG.y0, to: BAG.strapY1 - BAG.y0 };
  const link = { id: "belt-bag-link", kind: "box", bone: "belt-bag", material: "nylon-belt-bag", role: "body", tone: 0.85, size: [0.024, 0.075, 0.05], transform: { translation: [(BAG.strapY0 + BAG.strapY1) / 2 - BAG.y0, 0.065, 0] }, member: "belt-bag", part: "member" };
  for (const scale of ["50", "100", "200"]) {
    const vol = asset.volumes[scale];
    vol.body.push(box);
    vol.thermal.push({ id: "th-belt-bag", of: "belt-bag", region: "belt-bag" });
    vol.structure.push({ id: "belt-bag-frame", of: "belt-bag", member: "belt-bag", part: "member" }, scale === "200" ? { ...link, size: [0.03, 0.075, 0.05] } : link);
  }
  asset.volumes["50"].body.push(strap);
  asset.volumes["50"].thermal.push({ id: "th-belt-strap", of: "belt-strap", region: "belt-bag" });
  const floor = { ...box, id: "th-belt-bag", size: [height, 0.075, 2 * BAG.z], cornerRadius: 0.03, transform: { translation: [height / 2, -0.0075, 0] }, region: "belt-bag" };
  asset.volumes["500"].thermal.push(floor);
  asset.volumes["500"].structure.push({ ...floor, id: "belt-bag-frame", region: undefined, member: "belt-bag", part: "member", tone: 0.85 }, { ...link, size: [0.075, 0.09, 0.075], transform: { translation: [height / 2, 0.07, 0] } });
  for (const view of ["elevation", "front", "plan"]) for (const s of asset.representations["500"].geometry[view]) if (s.id === "sil-body") { s.regions = [s.region ?? "hoodie", "belt-bag"]; delete s.region; }
}
beltBag(a);

const feet = (d: number) => ["heel-l", "ball-l", "heel-r", "ball-r"].map((anchor) => ({ anchor, fromS: 0, toS: d }));
const key = (tS: number, pose: string, bones?: Bones) => ({ t: tS, pose, ease: "smooth", ...(bones ? { bones } : {}) });
const sway = (s: number): Bones => ({ "forearm-r": { angleDeg: 108 - s, twistDeg: 0 }, "hand-r": { angleDeg: -8 - s * 0.5 } });
a.clips = [
  { id: "wave", title: t("Waving hello", "Zum Gruß winken"), description: t("A 1.2-second loop of the greeting: the upright forearm swings out and in about the elbow twice, the open hand following a little behind; trunk and feet stay still.", "Eine Schleife von 1,2 Sekunden: Der aufrechte Unterarm schwingt zweimal um den Ellbogen nach außen und zurück, die offene Hand folgt etwas verzögert; Rumpf und Füße bleiben ruhig."),
    durationS: 1.2, fps: 30, loop: true, rootMotion: "none",
    keys: [key(0, "waving"), key(0.3, "waving", sway(22)), key(0.6, "waving", sway(-6)), key(0.9, "waving", sway(22)), key(1.2, "waving")],
    contacts: feet(1.2), phases: [{ id: "out", fromS: 0, toS: 0.3, title: t("Hand swings out", "Hand schwingt hinaus") }, { id: "in", fromS: 0.3, toS: 0.6, title: t("Hand swings back", "Hand schwingt zurück") }, { id: "out-again", fromS: 0.6, toS: 0.9, title: t("Hand swings out again", "Hand schwingt erneut hinaus") }, { id: "back", fromS: 0.9, toS: 1.2, title: t("Hand returns upright", "Hand kehrt aufrecht zurück") }],
    events: [], tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 1.2, v: 0 }] }] },
  { id: "hail-and-wave", title: t("Hailing, then waving", "Herbeirufen, dann winken"), description: t("A 3.6-second gesture from standing: the right arm shoots up to hail, the raised hand waves twice above the head, then the arm comes down again.", "Eine Geste von 3,6 Sekunden aus dem Stand: Der rechte Arm schnellt zum Herbeirufen hoch, die erhobene Hand winkt zweimal über dem Kopf, dann sinkt der Arm wieder."),
    durationS: 3.6, fps: 30, loop: false, rootMotion: "none",
    keys: [key(0, "standing-relaxed"), key(0.7, "hailing"), key(1.05, "hailing", { "forearm-r": { angleDeg: 28 }, "hand-r": { angleDeg: -22 } }), key(1.4, "hailing"), key(1.75, "hailing", { "forearm-r": { angleDeg: 28 }, "hand-r": { angleDeg: -22 } }), key(2.1, "hailing"), key(2.6, "waving"), key(3.6, "standing-relaxed")],
    contacts: feet(3.6), phases: [{ id: "raise", fromS: 0, toS: 0.7, title: t("Arm shoots up", "Arm schnellt hoch") }, { id: "wave-overhead", fromS: 0.7, toS: 2.1, title: t("Waving overhead", "Winken über dem Kopf") }, { id: "lower", fromS: 2.1, toS: 3.6, title: t("Arm comes down", "Arm sinkt") }],
    events: [{ t: 0.7, kind: "motion", title: t("Hand at its highest", "Hand am höchsten Punkt") }],
    tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 0.7, v: 1 }, { t: 2.6, v: 0 }] }, { target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 1.2 }, { t: 0.7, v: 1.8 }, { t: 2.6, v: 1.7 }, { t: 3.6, v: 1.2 }] }] },
  { id: "idle-breathing", title: t("Idle breathing", "Ruhiges Atmen"), description: t("A five-second loop at about 1.2 met between two greetings: the chest rises and falls with quiet breathing, head and arms sway slightly; the feet stay planted.", "Eine Schleife von fünf Sekunden bei etwa 1,2 met zwischen zwei Grüßen: Die Brust hebt und senkt sich mit dem ruhigen Atmen, Kopf und Arme wiegen sich leicht; die Füße bleiben stehen."),
    durationS: 5, fps: 30, loop: true, rootMotion: "none",
    keys: [key(0, "standing-relaxed"), key(1.25, "standing-relaxed", { torso: { angleDeg: -0.36 }, head: { angleDeg: 1.8 }, "arm-r": { angleDeg: 2.5, abductionDeg: -6 }, "arm-l": { angleDeg: 1, abductionDeg: 6 }, lumbar: { angleDeg: -0.24 } }), key(2.5, "standing-relaxed", { torso: { angleDeg: 0.24 }, head: { angleDeg: 0.4 }, "forearm-r": { angleDeg: 10.5 }, lumbar: { angleDeg: 0.16 } }), key(3.75, "standing-relaxed", { torso: { angleDeg: -0.3 }, head: { angleDeg: 1.5 }, "arm-r": { angleDeg: 2, abductionDeg: -6 }, "arm-l": { angleDeg: -1, abductionDeg: 6 }, lumbar: { angleDeg: -0.2 } }), key(5, "standing-relaxed")],
    contacts: feet(5), phases: [{ id: "idle", fromS: 0, toS: 5, title: t("Quiet standing", "Ruhiges Stehen") }], events: [], tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 5, v: 0 }] }] },
];
a.presentation = { defaultPose: "waving", defaultClip: "wave", defaultScale: 100 };

a.id = "waving-person";
a.title = t("Waving Person", "Winkende Person");
a.tags = ["human", "adult", "standing", "gesture", "waving", "greeting", "hailing", "belt-bag"];
const refs = a.provenance.references.filter((r: Json) => r.id !== "iso-8996-2021");
refs.push({ id: "iso-8996-2021", citation: "ISO 8996:2021 Ergonomics of the thermal environment, determination of metabolic rate. (activity classes and met values)." });
refs.push({ id: "ainsworth-2011", citation: "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine and Science in Sports and Exercise 43(8), 1575-1581 (activity classes; MET values are mapped to thermal met as an estimate)." });
a.provenance = { ...a.provenance, author: "Semio staffage human worker HW-11", created: "2026-11-22", references: refs };

reword(a, ["in the relaxed pose", "in the waving pose"], ["in der entspannten Pose", "in der Winkpose"]);
scrub(a, /relaxed|arms hanging|generic scale figure/i, /entspannt|Arme.{0,20}hängend|generische Maßstabsfigur/i);
const leftover = fillEmpty(a, { "500": { retained: ["The raised forearm with the open hand beside the head, which carries the greeting, the hanging left arm, the stance of the feet and the exact stature and extent.", "Der gehobene Unterarm mit der offenen Hand neben dem Kopf, der den Gruß trägt, der hängende linke Arm, die Fußstellung sowie genaue Körpergröße und Ausdehnung."] } });
if (leftover.length) console.log("[DEBUG] still empty", leftover.join(" "));
const H = { 50: "35", 100: "17.6", 200: "8.8", 500: "3.5" } as Record<number, string>;
for (const sc of [50, 100, 200, 500]) note(a, sc, "silhouette", `Default pose "waving" (${H[sc]} mm tall at 1:${sc}): the right upper arm is raised sideways, the forearm stands upright and the open palm faces forward beside the head; every part is the base figure's own shape carried by the rig.`, `Standardpose „Winken" (${H[sc]!.replace(".", ",")} mm hoch bei 1:${sc}): Der rechte Oberarm ist seitlich gehoben, der Unterarm steht aufrecht und die offene Handfläche zeigt neben dem Kopf nach vorn; jedes Teil ist die eigene Form der Grundfigur, vom Rig getragen.`, true);
note(a, 50, "retained", "The belt bag as a rounded box with its zip line in the front view and the 25 mm webbing strap around the hips in the side and front views.", "Die Gürteltasche als gerundeter Kasten mit Reißverschlusslinie in der Vorderansicht und dem 25 mm breiten Gurtband um die Hüfte in Seiten- und Vorderansicht.");
note(a, 100, "retained", "The belt bag as a rounded box with its zip line in the front view.", "Die Gürteltasche als gerundeter Kasten mit Reißverschlusslinie in der Vorderansicht.");
note(a, 100, "omitted", "The 25 mm waist strap of the belt bag (0.25 mm, hidden by the arms in most poses).", "Das 25 mm breite Hüftgurtband der Gürteltasche (0,25 mm, in den meisten Posen von den Armen verdeckt).");
note(a, 200, "retained", "The belt bag as one rounded mass in front of the hips, 60 mm deep (0.3 mm).", "Die Gürteltasche als eine gerundete Masse vor der Hüfte, 60 mm tief (0,3 mm).");
note(a, 200, "omitted", "The zip and the waist strap of the belt bag.", "Reißverschluss und Hüftgurt der Gürteltasche.");
note(a, 500, "merged", "The belt bag (60 mm, below the 75 mm feature) is part of the trunk mass; the trunk silhouette lists its region.", "Die Gürteltasche (60 mm, unter dem Merkmal von 75 mm) gehört zur Rumpfmasse; die Rumpfsilhouette führt ihre Region.");
const bagVol: Record<string, [string, string]> = {
  "50": ["Belt bag at 1:50: a rounded box 118 by 60 by 230 mm (radius 24 mm) on its own fixed bone in front of the hips, the webbing strap an elliptic band 25 mm high and 10 mm thick round the hips; thermal region belt-bag; in the structure the bag hangs on the pelvic girdle through one short link that stands for the strap.", "Gürteltasche bei 1:50: ein gerundeter Kasten 118 auf 60 auf 230 mm (Radius 24 mm) auf einem eigenen festen Knochen vor der Hüfte, das Gurtband ein elliptisches Band 25 mm hoch und 10 mm dick um die Hüfte; thermische Region belt-bag; im Tragwerk hängt die Tasche über ein kurzes Glied, das für den Gurt steht, am Beckengürtel."],
  "100": ["Belt bag at 1:100: the same rounded box without the strap (10 mm, below the 15 mm feature); the structure link stands for the strap.", "Gürteltasche bei 1:100: derselbe gerundete Kasten ohne Gurt (10 mm, unter dem Merkmal von 15 mm); das Tragwerksglied steht für den Gurt."],
  "200": ["Belt bag at 1:200: the rounded box as one mass, 60 mm deep, twice the feature.", "Gürteltasche bei 1:200: der gerundete Kasten als eine Masse, 60 mm tief, das Doppelte des Merkmals."],
  "500": ["Belt bag at 1:500: below the 75 mm feature it is not part of the body; the thermal and structural lists keep one block at the size floor of 75 mm so that its region and its mass stay in the model.", "Gürteltasche bei 1:500: unter dem Merkmal von 75 mm gehört sie nicht zum Körper; Thermik- und Tragwerksliste behalten einen Block in der Mindestgröße von 75 mm, damit Region und Masse im Modell bleiben."],
};
for (const [sc, [en, de]] of Object.entries(bagVol)) a.volumes[sc].decisions.push({ en, de });
a.summary = t("A slim adult of 1.69 m (59 kg) in a fleece hooded sweatshirt, jeans, canvas sneakers and a nylon belt bag worn in front of the hips, greeting someone: the right forearm stands upright beside the head with the open palm forward. Poses for waving, hailing with the arm stretched up, waving with both arms and standing relaxed; loops for the wave and idle breathing and a hail-and-wave gesture, at 1.2 to 2.3 met.", "Eine schlanke erwachsene Person von 1,69 m (59 kg) in Fleece-Kapuzenpullover, Jeans, Stoffsneakern und einer vor der Hüfte getragenen Nylon-Gürteltasche, die jemanden grüßt: Der rechte Unterarm steht aufrecht neben dem Kopf, die offene Handfläche zeigt nach vorn. Posen für Winken, Herbeirufen mit hochgestrecktem Arm, Winken mit beiden Armen und entspanntes Stehen; Schleifen für das Winken und ruhiges Atmen sowie eine Geste aus Herbeirufen und Winken, bei 1,2 bis 2,3 met.");
a.provenance.notes = t("Skeleton, outlines and volumes derived by script from the archive asset standing-adult (stature 1.69 m barefoot, de Leva segment fractions, ANSUR II and DIN 33402-2 breadths for a slim 59 kg adult) and handcrafted on: the belt bag (own fixed bone, rounded box in every view and scale, strap at 1:50, thermal region and structural link), four poses and three clips authored by hand for greeting gestures. Activity: waving 1.7 met and hailing 1.8 met (ISO 8996 low metabolic rate class with arm work), waving with both arms 2.3 met (Ainsworth compendium, light-to-moderate standing arm work, mapped as an estimate), relaxed standing 1.2 met (ISO 7730 Annex A).", "Skelett, Umrisse und Volumen per Skript aus dem Archivobjekt standing-adult abgeleitet (Körperhöhe barfuß 1,69 m, Segmentanteile nach de Leva, Breiten nach ANSUR II und DIN 33402-2 für eine schlanke Person von 59 kg) und von Hand weitergearbeitet: die Gürteltasche (eigener fester Knochen, gerundeter Kasten in jeder Ansicht und jedem Maßstab, Gurt bei 1:50, thermische Region und Tragwerksglied), vier Posen und drei Clips von Hand für Grußgesten. Aktivität: Winken 1,7 met und Herbeirufen 1,8 met (ISO 8996, Klasse niedriger Umsatz mit Armarbeit), Winken mit beiden Armen 2,3 met (Ainsworth-Kompendium, leichte bis mäßige Armarbeit im Stehen, als Schätzung übertragen), entspanntes Stehen 1,2 met (ISO 7730 Anhang A).");
const e = measureDimensions(a);
console.log("[DEBUG] extents", JSON.stringify(e));
if (process.argv.includes("--out")) await Bun.write(process.argv[process.argv.indexOf("--out") + 1]!, JSON.stringify({ asset: a }));
else console.log(await writeAsset(a));
