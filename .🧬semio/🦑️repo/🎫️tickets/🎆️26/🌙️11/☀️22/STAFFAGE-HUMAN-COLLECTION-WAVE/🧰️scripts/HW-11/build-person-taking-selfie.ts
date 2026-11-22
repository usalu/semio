/** 🤳️ Generator of `person-taking-selfie`: derived from the rig, outlines and volumes of `standing-phone-user` (headphones removed, a daypack added), a tourist who holds the phone at arm's length and poses; the phone placed by inverse kinematics so that its screen faces the eyes.
 *  bun build-person-taking-selfie.ts [--out file.json]
 * @see ../../📋️brief.md */
import { addShapes, dropParts, fillEmpty, measureDimensions, note, readAsset, roundRect, scrub, t, writeAsset, type Bones, type Json } from "./lib.ts";
import { add, at, compose, cross, dir, frameOf, frames, mul, relative, solve, sub, unit, type F, type Param, type V } from "./ik.ts";

const a: Json = await readAsset("standing-phone-user");
dropParts(a, [], /^(th-)?headphone/, ["headphones"], ["headphone-polymer"]);
for (const p of a.poses) delete p.thermal.regions.headphones;
for (const view of ["elevation", "front", "plan"]) for (const s of a.representations["500"].geometry[view]) if (s.regions?.includes("headphones")) { const rest = s.regions.filter((r: string) => r !== "headphones"); if (rest.length === 1) { s.region = rest[0]; delete s.regions; } else s.regions = rest; }

/** 🎒️ The daypack: its own fixed bone on the torso, a rounded box on the back in every scale, shoulder straps over the chest at 1:50 and 1:100. */
const PACK = { u0: 0.02, u1: 0.31, v0: 0.088, depth: 0.12, w: 0.13, r: 0.03 };
{
  const torso = a.rig.bones.find((b: Json) => b.id === "torso");
  a.rig.bones.splice(a.rig.bones.indexOf(torso) + 1, 0, { id: "daypack", parent: "torso", title: t("Daypack of 15 litres", "Tagesrucksack mit 15 Litern"), x: PACK.u0, y: PACK.v0, angleDeg: 0, lengthM: PACK.u1 - PACK.u0, joint: { type: "fixed" }, mass: { kg: 1.6, comX: (PACK.u1 - PACK.u0) / 2, comY: PACK.depth * 0.45, distribution: "point", source: "authored-estimate" } });
  a.materials.push({ id: "pack-nylon", title: t("Ripstop nylon daypack with padded polyester straps", "Tagesrucksack aus Ripstop-Nylon mit gepolsterten Polyestergurten"), emissivity: 0.9, solarAbsorptivity: 0.7, specificHeatJKgK: 1700, source: "authored-estimate" });
  a.thermal.regions.push({ id: "daypack", title: t("Daypack and shoulder straps", "Tagesrucksack und Schultergurte"), kind: "surface", model: "passive", material: "pack-nylon", areaM2: 0.21, characteristicLengthM: 0.29, geometry: "vertical-plate", skyViewFactor: 0.4, layers: [], heatCapacityJM2K: 9500, source: "authored-estimate" });
}
const restF = frames(a, {});
const packRest = restF.daypack!;
const H = PACK.u1 - PACK.u0;
const packShape = (view: string, sc: number): Json[] => {
  const side = { id: "daypack", bone: "daypack", role: "body", z: 3.85, filled: true, tone: 0.35, region: "daypack", material: "pack-nylon", kind: "poly", points: roundRect(0, H, 0, PACK.depth, PACK.r), closed: true, solid: { kind: "extruded", sweep: "w", z0: +(packRest.o[2] - PACK.w).toFixed(5), z1: +(packRest.o[2] + PACK.w).toFixed(5) } };
  if (view !== "elevation") return [side];
  const out = [side];
  if (sc <= 100) out.push({ id: "daypack-pocket", bone: "daypack", role: "detail", z: 3.86, filled: false, tone: 0.1, region: "daypack", kind: "poly", points: [[0.03, PACK.depth - 0.002], [0.03, PACK.depth + 0.012], [0.15, PACK.depth + 0.012], [0.15, PACK.depth - 0.002]], closed: false, solid: side.solid });
  return out;
};
const strapFront = (s: 1 | -1): number[][] => {
  const top = at(restF.torso!, [0.45, -0.02, s * 0.075]), low = at(restF.torso!, [0.2, -0.12, s * 0.105]);
  const half = 0.021;
  return [[top[2]! - half, top[1]!], [top[2]! + half, top[1]!], [low[2]! + half, low[1]!], [low[2]! - half, low[1]!]].map(([x, y]) => [+x!.toFixed(4), +y!.toFixed(4)]);
};
for (const sc of [50, 100, 200, 500]) {
  for (const view of ["elevation", "front", "plan"]) addShapes(a, sc, view, packShape(view, sc), sc === 500 ? "sil-body" : view === "elevation" ? "hood" : view === "plan" ? "hoodie" : "hood");
  if (sc <= 100) addShapes(a, sc, "front", [1, -1].map((s) => ({ id: `daypack-strap-${s > 0 ? "l" : "r"}`, bone: "torso", role: "body", z: 4.2, filled: true, tone: 0.3, region: "daypack", material: "pack-nylon", kind: "poly", points: strapFront(s as 1 | -1), closed: true })), "collar-rib");
  const vol = a.volumes[`${sc}`];
  const box = { id: "daypack", kind: "box", bone: "daypack", material: "pack-nylon", role: "body", tone: 0.35, size: [H, PACK.depth, 2 * PACK.w], cornerRadius: PACK.r, transform: { translation: [H / 2, PACK.depth / 2, 0] } };
  const link = { id: "daypack-link", kind: "box", bone: "daypack", material: "pack-nylon", role: "body", tone: 0.85, size: sc === 500 ? [0.075, 0.12, 0.075] : sc === 200 ? [0.03, 0.12, 0.04] : [0.03, 0.12, 0.03], transform: { translation: [H * 0.7, -0.04, 0] }, member: "daypack", part: "member" };
  vol.body.push(box);
  vol.thermal.push({ id: "th-daypack", of: "daypack", region: "daypack" });
  vol.structure.push({ id: "daypack-frame", of: "daypack", member: "daypack", part: "member" }, link);
  if (sc === 50) for (const s of [1, -1]) {
    const top = [0.45, -0.075] as const, low = [0.2, -0.152] as const;
    const len = Math.hypot(top[0] - low[0], top[1] - low[1]), ang = Math.atan2(top[1] - low[1], top[0] - low[0]);
    const strap = { id: `daypack-strap-${s > 0 ? "l" : "r"}`, kind: "box", bone: "torso", material: "pack-nylon", role: "body", tone: 0.3, size: [len, 0.009, 0.042], cornerRadius: 0.003, transform: { rotation: [0, 0, Math.sin(ang / 2), Math.cos(ang / 2)], translation: [(top[0] + low[0]) / 2, (top[1] + low[1]) / 2, s * 0.09] } };
    vol.body.push(strap);
    vol.thermal.push({ id: `th-${strap.id}`, of: strap.id, region: "daypack" });
  }
}

const EYES: V = [0.03, -0.085, 0];
type Aim = { fwd: number; up: number; right: number };
/** 🎯️ The phone frame for a head frame: its centre placed relative to the eyes, the screen (w) facing the eyes, the long axis (u) up and leaning a little toward the face. */
const phoneAt = (head: F, aim: Aim): F => {
  const eyes = at(head, EYES);
  const fwd = unit([-head.v[0], 0, -head.v[2]]), up: V = [0, 1, 0], right = cross(fwd, up);
  const c = add(eyes, add(mul(fwd, aim.fwd), add(mul(up, aim.up), mul(right, aim.right))));
  const toEyes = unit(sub(eyes, c));
  const u = unit(sub(up, mul(toEyes, 0.25)));
  const f = frameOf([0, 0, 0], u, toEyes);
  return { ...f, o: sub(c, mul(f.u, 0.0735)) };
};
const armR: Param[] = [
  { bone: "arm-r", field: "angleDeg" }, { bone: "arm-r", field: "abductionDeg", min: -120, max: 60 }, { bone: "arm-r", field: "twistDeg", min: -100, max: 100 },
  { bone: "forearm-r", field: "angleDeg" }, { bone: "forearm-r", field: "twistDeg", min: -95, max: 95 },
  { bone: "hand-r", field: "angleDeg" }, { bone: "hand-r", field: "abductionDeg", min: -35, max: 35 }, { bone: "hand-r", field: "twistDeg", min: -45, max: 45 }, { bone: "phone", field: "angleDeg" },
];
const armL: Param[] = [
  { bone: "arm-l", field: "angleDeg" }, { bone: "arm-l", field: "abductionDeg", min: -60, max: 120 }, { bone: "arm-l", field: "twistDeg", min: -100, max: 100 },
  { bone: "forearm-l", field: "angleDeg" }, { bone: "forearm-l", field: "twistDeg", min: -95, max: 95 }, { bone: "hand-l", field: "angleDeg" }, { bone: "hand-l", field: "abductionDeg", min: -35, max: 35 },
];
const frameError = (now: F, want: F, k = 0.12): number[] => [...sub(now.o, want.o), ...sub(now.u, want.u).map((e) => e * k), ...sub(now.w, want.w).map((e) => e * k)];
/** 🤳️ The right arm that holds the phone at an aim, tried from several starts. */
function selfieArm(start: Bones, aim: Aim): Bones {
  const res = (f: Record<string, F>) => frameError(f.phone!, phoneAt(f.head!, aim));
  let top = solve(a, start, armR, res, 60);
  for (const ang of [40, 80, 120]) for (const abd of [-10, -40, -70]) for (const tw of [-60, 0, 60]) {
    const c = solve(a, { ...start, "arm-r": { angleDeg: ang, abductionDeg: abd, twistDeg: tw }, "forearm-r": { angleDeg: 20 }, "hand-r": { angleDeg: 0 } }, armR, res, 40);
    if (c.error < top.error) top = c;
  }
  const r = solve(a, top.bones, armR, res, 150);
  const e = res(frames(a, r.bones));
  console.log(`[DEBUG] selfie arm pos ${Math.hypot(e[0]!, e[1]!, e[2]!).toFixed(3)} u ${(Math.hypot(e[3]!, e[4]!, e[5]!) / 0.12).toFixed(2)} w ${(Math.hypot(e[6]!, e[7]!, e[8]!) / 0.12).toFixed(2)}`, JSON.stringify(Object.fromEntries(Object.entries(r.bones).filter(([k]) => /-r$|phone/.test(k)))));
  return r.bones;
}
/** 🫴️ The left hand on the hip, fingers forward over the iliac crest. */
function akimbo(start: Bones): Bones {
  const res = (f: Record<string, F>) => { const p = f.pelvis!, h = f["hand-l"]!; return [...sub(h.o, at(p, [0.16, 0.03, 0.2])), ...sub(at(h, [0.09, 0, 0]), at(p, [0.09, -0.06, 0.19]))]; };
  let top = solve(a, start, armL, res, 60);
  for (const abd of [20, 45, 70]) for (const tw of [-60, 0, 60]) {
    const c = solve(a, { ...start, "arm-l": { angleDeg: -15, abductionDeg: abd, twistDeg: tw }, "forearm-l": { angleDeg: 90 }, "hand-l": { angleDeg: 0 } }, armL, res, 40);
    if (c.error < top.error) top = c;
  }
  const r = solve(a, top.bones, armL, res, 150);
  console.log(`[DEBUG] akimbo error ${r.error.toFixed(4)}`);
  return r.bones;
}
const baseTexting = a.poses.find((p: Json) => p.id === "texting");
const legs: Bones = Object.fromEntries(Object.entries(baseTexting.bones).filter(([k]) => /thigh|shank|foot|toe/.test(k))) as Bones;
const leftHang: Bones = { "arm-l": { angleDeg: 2, abductionDeg: 7 }, "forearm-l": { angleDeg: 14 }, "hand-l": { angleDeg: 3 } };
const armOut = selfieArm(akimbo({ ...legs, torso: { angleDeg: -1 }, neck: { angleDeg: -2 }, head: { angleDeg: 6 }, ponytail: { angleDeg: 3 }, "arm-l": { angleDeg: -10, abductionDeg: 40 }, "forearm-l": { angleDeg: 100 } }), { fwd: 0.4, up: -0.03, right: 0.1 });
const highAngle = selfieArm({ ...legs, ...leftHang, torso: { angleDeg: 1 }, neck: { angleDeg: -6 }, head: { angleDeg: -10 }, ponytail: { angleDeg: -8 } }, { fwd: 0.3, up: 0.3, right: 0.1 });
const backdrop = selfieArm(akimbo({ ...legs, torso: { angleDeg: 0 }, neck: { angleDeg: -1 }, head: { angleDeg: 3 }, ponytail: { angleDeg: 4 }, "arm-l": { angleDeg: -10, abductionDeg: 40 }, "forearm-l": { angleDeg: 100 } }), { fwd: 0.32, up: 0.02, right: 0.32 });
const checking: Bones = structuredClone(baseTexting.bones);

const pose = (id: string, title: Json, description: Json, bones: Bones, met: number, breathing: string, phone: number, hoodie: number): Json => ({
  ...structuredClone(baseTexting), id, title, description, bones,
  thermal: { ...structuredClone(baseTexting.thermal), activityMet: met, breathing, regions: { ...structuredClone(baseTexting.thermal.regions), hoodie: { exposedFraction: hoodie, sunlitFraction: 0 }, daypack: { exposedFraction: 0.85, sunlitFraction: 0 } }, operating: { phone } },
});
a.poses = [
  pose("selfie-arm-out", t("Selfie at arm's length", "Selfie mit ausgestrecktem Arm"), t("Feet at hip width, the right arm stretched forward and a little up so the phone is 40 cm in front of the eyes at their height, the screen turned to the face; the chin lifted a little for the picture, the left hand on the hip.", "Füße hüftbreit, der rechte Arm nach vorn und etwas nach oben gestreckt, sodass das Telefon 40 cm vor den Augen auf ihrer Höhe ist, der Bildschirm zum Gesicht gedreht; das Kinn für das Bild etwas gehoben, die linke Hand an der Hüfte."), armOut, 1.4, "quiet-standing", 1, 0.84),
  pose("selfie-high-angle", t("Selfie from above", "Selfie von oben"), t("The phone held high in front of the forehead, 30 cm ahead and 30 cm above the eyes, the screen tilted down to the face; the chin is raised a little, the left arm hangs.", "Das Telefon hoch vor der Stirn gehalten, 30 cm vor und 30 cm über den Augen, der Bildschirm zum Gesicht geneigt; das Kinn ist etwas gehoben, der linke Arm hängt."), highAngle, 1.4, "quiet-standing", 1, 0.86),
  pose("selfie-with-backdrop", t("Selfie with the sight behind", "Selfie mit der Sehenswürdigkeit dahinter"), t("The phone held out to the right at eye height, 32 cm beside and 32 cm in front of the eyes, so that the sight behind the left shoulder is in the picture; the eyes turned to the screen, the left hand on the hip.", "Das Telefon auf Augenhöhe nach rechts hinausgehalten, 32 cm neben und 32 cm vor den Augen, damit die Sehenswürdigkeit hinter der linken Schulter im Bild ist; der Blick zum Bildschirm, die linke Hand an der Hüfte."), backdrop, 1.4, "quiet-standing", 1, 0.84),
  pose("checking-photo", t("Checking the photo", "Das Foto ansehen"), t("Head bowed over the phone held in both hands in front of the lower chest, the thumbs swiping through the pictures just taken.", "Kopf über das mit beiden Händen vor der unteren Brust gehaltene Telefon gesenkt, die Daumen wischen durch die eben gemachten Bilder."), checking, 1.2, "typing", 0.6, 0.86),
];
const feet = (d: number) => ["heel-l", "ball-l", "heel-r", "ball-r"].map((anchor) => ({ anchor, fromS: 0, toS: d }));
const key = (tS: number, p: string, bones?: Bones) => ({ t: tS, pose: p, ease: "smooth", ...(bones ? { bones } : {}) });
const tilt = (sway: number, ang: number): Bones => ({ head: { angleDeg: ang }, ponytail: { angleDeg: 3 + sway } });
a.clips = [
  { id: "selfie-posing", title: t("Posing for selfies", "Für Selfies posieren"), description: t("A 3-second loop at arm's length: the head nods into a new pose between shots and the ponytail swings, the phone takes a picture at each pose; arm and feet stay still.", "Eine Schleife von 3 Sekunden mit ausgestrecktem Arm: Der Kopf nickt zwischen den Aufnahmen in eine neue Pose, der Pferdeschwanz schwingt, das Telefon nimmt bei jeder Pose ein Bild auf; Arm und Füße bleiben ruhig."),
    durationS: 3, fps: 30, loop: true, rootMotion: "none",
    keys: [key(0, "selfie-arm-out"), key(0.75, "selfie-arm-out", tilt(0, 6)), key(1.5, "selfie-arm-out", tilt(-8, 3)), key(2.25, "selfie-arm-out", tilt(-2, 7)), key(3, "selfie-arm-out")],
    contacts: feet(3), phases: [{ id: "pose-a", fromS: 0, toS: 1.5, title: t("Chin up", "Kinn gehoben") }, { id: "pose-b", fromS: 1.5, toS: 3, title: t("Chin down, smiling", "Kinn gesenkt, lächelnd") }],
    events: [{ t: 0.1, kind: "operating", target: "phone", title: t("Shutter, first pose", "Auslösen, erste Pose") }, { t: 1.6, kind: "operating", target: "phone", title: t("Shutter, second pose", "Auslösen, zweite Pose") }],
    tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 3, v: 0 }] }] },
  { id: "take-and-check", title: t("Taking a selfie and checking it", "Selfie aufnehmen und ansehen"), description: t("A 5-second sequence: from looking at the phone the arm stretches out, the photo is taken from above and at arm's length, then the phone comes back to the chest to check the result.", "Eine Abfolge von 5 Sekunden: Vom Blick aufs Telefon streckt sich der Arm aus, das Foto wird von oben und mit ausgestrecktem Arm aufgenommen, dann kommt das Telefon zur Brust zurück, um das Ergebnis anzusehen."),
    durationS: 5, fps: 30, loop: false, rootMotion: "none",
    keys: [key(0, "checking-photo"), key(1.2, "selfie-high-angle"), key(2.0, "selfie-high-angle"), key(2.8, "selfie-arm-out"), key(3.6, "selfie-arm-out"), key(5, "checking-photo")],
    contacts: feet(5), phases: [{ id: "raise", fromS: 0, toS: 1.2, title: t("Arm out", "Arm hinaus") }, { id: "shoot", fromS: 1.2, toS: 3.6, title: t("Two angles", "Zwei Blickwinkel") }, { id: "check", fromS: 3.6, toS: 5, title: t("Checking the picture", "Bild ansehen") }],
    events: [{ t: 1.6, kind: "operating", target: "phone", title: t("Shutter from above", "Auslösen von oben") }, { t: 3.2, kind: "operating", target: "phone", title: t("Shutter at arm's length", "Auslösen mit ausgestrecktem Arm") }],
    tracks: [{ target: "operating:phone", ease: "smooth", keys: [{ t: 0, v: 0.6 }, { t: 1.2, v: 1 }, { t: 3.6, v: 1 }, { t: 5, v: 0.6 }] }, { target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 4.2, v: 1 }] }] },
  { id: "idle-breathing", title: t("Idle breathing over the phone", "Ruhiges Atmen über dem Telefon"), description: t("A five-second loop looking at the photos: the chest rises and falls, the head dips a little, the thumbs swipe; the feet stay planted.", "Eine Schleife von fünf Sekunden beim Ansehen der Fotos: Die Brust hebt und senkt sich, der Kopf senkt sich ein wenig, die Daumen wischen; die Füße bleiben stehen."),
    durationS: 5, fps: 30, loop: true, rootMotion: "none",
    keys: [key(0, "checking-photo"), key(1.25, "checking-photo", { head: { angleDeg: -24 }, torso: { angleDeg: -3.4 } }), key(2.5, "checking-photo", { "hand-l": { ...checking["hand-l"], angleDeg: -11 } }), key(3.75, "checking-photo", { head: { angleDeg: -20 }, torso: { angleDeg: -2.6 } }), key(5, "checking-photo")],
    contacts: feet(5), phases: [{ id: "idle", fromS: 0, toS: 5, title: t("Looking at the photos", "Fotos ansehen") }], events: [], tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 1 }, { t: 5, v: 1 }] }] },
];
a.presentation = { defaultPose: "selfie-arm-out", defaultClip: "selfie-posing", defaultScale: 100 };
a.id = "person-taking-selfie";
a.title = t("Person Taking a Selfie", "Person beim Selfie");
a.tags = ["human", "adult", "woman", "standing", "smartphone", "selfie", "tourist", "daypack", "posing"];
a.summary = t("A 24-year-old woman of 1.69 m (58 kg) in a hooded fleece sweatshirt, slim jeans, canvas sneakers and a 15-litre daypack, sightseeing: she holds her smartphone at arm's length with the screen turned to her face, lifts her chin and props the other hand on her hip. Poses at arm's length, from above, with the sight behind her and checking the photo; a posing loop, a take-and-check sequence and idle breathing.", "Eine 24-jährige Frau von 1,69 m (58 kg) in Fleece-Kapuzenpullover, enger Jeans, Stoffsneakern und einem 15-Liter-Tagesrucksack beim Sightseeing: Sie hält ihr Smartphone mit ausgestrecktem Arm, den Bildschirm zum Gesicht gedreht, hebt das Kinn und stützt die andere Hand in die Hüfte. Posen mit ausgestrecktem Arm, von oben, mit der Sehenswürdigkeit dahinter und beim Ansehen des Fotos; eine Posierschleife, eine Abfolge aus Aufnehmen und Ansehen sowie ruhiges Atmen.");
const refs = a.provenance.references.filter((r: Json) => !["iso-8996-2021", "ainsworth-2011"].includes(r.id));
refs.push({ id: "iso-8996-2021", citation: "ISO 8996:2021 Ergonomics of the thermal environment, determination of metabolic rate (activity classes and met values)." });
a.provenance = { ...a.provenance, author: "Semio staffage human worker HW-11", created: "2026-11-22", references: refs, notes: t("Skeleton, outlines and volumes derived by script from the archive asset standing-phone-user (1.69 m, de Leva female segment fractions); the headphones removed, a 15-litre daypack (290 by 120 by 260 mm, 1.6 kg) handcrafted on its own fixed bone on the back with its shoulder straps; four poses and three clips authored for selfies. The phone of every selfie pose is placed by inverse kinematics on the rig: its centre at a stated offset from the eyes and its screen turned to them; the left hand on the hip is solved the same way. Activity 1.4 met for posing with the arm held out (ISO 8996 low metabolic rate, static arm work) and 1.2 met for looking at the phone.", "Skelett, Umrisse und Volumen per Skript aus dem Archivobjekt standing-phone-user abgeleitet (1,69 m, Segmentanteile nach de Leva, Frauentabelle); die Kopfhörer entfernt, ein 15-Liter-Tagesrucksack (290 auf 120 auf 260 mm, 1,6 kg) von Hand auf einem eigenen festen Knochen am Rücken mit seinen Schultergurten gebaut; vier Posen und drei Clips für Selfies gestaltet. Das Telefon jeder Selfie-Pose ist durch inverse Kinematik am Rig gesetzt: sein Mittelpunkt in einem festgelegten Abstand von den Augen und sein Bildschirm ihnen zugewandt; die linke Hand an der Hüfte ist ebenso gelöst. Aktivität 1,4 met beim Posieren mit gehaltenem Arm (ISO 8996, niedriger Umsatz, statische Armarbeit) und 1,2 met beim Blick aufs Telefon.") };
a.structure.assumptions = [...a.structure.assumptions.filter((x: Json) => !/both feet stand flat/.test(x.en)), { en: "The 1.6 kg daypack hangs on the trunk through its straps, idealised as one short link from the pack to the spine.", de: "Der 1,6 kg schwere Tagesrucksack hängt über seine Gurte am Rumpf, idealisiert als ein kurzes Glied vom Rucksack zur Wirbelsäule." }, { en: "Contacts are heel and ball of each foot; both feet stand flat at hip-joint width in every named pose.", de: "Kontakte sind Ferse und Ballen jedes Fußes; beide Füße stehen in jeder benannten Pose flach in Hüftgelenkbreite." }];
scrub(a, /headphone|texting|bowed|clasped|between the hands|inside the hands/i, /Kopfhörer|Tipp|gesenkt|zusammengelegt|zwischen den Händen|in den Händen/i);
const left = fillEmpty(a, {
  "50": { silhouette: ["Every part is the base figure's orthographic outline of the 1:50 body: head with face, ear, hair cap, hair tie and ponytail, neck, sweatshirt with hood, hem, collar and pocket, sleeves, forearms, hands with fingers and thumb, the phone, jeans, sneakers.", "Jedes Teil ist der orthogonale Umriss des 1:50-Körpers der Grundfigur: Kopf mit Gesicht, Ohr, Haarkappe, Haargummi und Pferdeschwanz, Hals, Sweatshirt mit Kapuze, Saum, Kragen und Tasche, Ärmel, Unterarme, Hände mit Fingern und Daumen, das Telefon, Jeans, Sneaker."], merged: ["The four fingers of a hand are one shape; eyelets merge into the lace strokes; lips, chin and brows merge into the head outline.", "Die vier Finger einer Hand sind eine Form; Ösen gehen in den Schnürstrichen auf; Lippen, Kinn und Brauen gehen im Kopfumriss auf."], thinElements: ["Laces and seams are 0.10 mm texture lines; the 7.8 mm phone is drawn at its true thickness, with the screen as a 0.13 mm outline in the plan.", "Schnürsenkel und Nähte sind 0,10-mm-Texturlinien; das 7,8 mm dicke Telefon ist in wahrer Dicke gezeichnet, der Bildschirm als 0,13-mm-Umriss im Grundriss."], retained: ["Eye, brow, mouth, the hair tie, the fingers as their own shape, socks, toe caps, laces and the screen outline in the plan.", "Auge, Braue, Mund, das Haargummi, die Finger als eigene Form, Socken, Zehenkappen, Schnürung und der Bildschirmumriss im Grundriss."] },
  "100": { silhouette: ["Every part is the base figure's outline of the 1:100 body: head with nose and ear under the hair cap, ponytail, neck, sweatshirt with hood, sleeves, forearms, hands with finger slab and thumb, the phone, jeans, shoes.", "Jedes Teil ist der Umriss des 1:100-Körpers der Grundfigur: Kopf mit Nase und Ohr unter der Haarkappe, Pferdeschwanz, Hals, Sweatshirt mit Kapuze, Ärmel, Unterarme, Hände mit Fingerplatte und Daumen, das Telefon, Jeans, Schuhe."], retained: ["Hair cap with ponytail and ear, face profile with nose and chin, hands with finger slab and thumb, the phone as a slab of its true size.", "Haarkappe mit Pferdeschwanz und Ohr, Gesichtsprofil mit Nase und Kinn, Hände mit Fingerplatte und Daumen, das Telefon als Platte in wahrer Größe."], merged: ["Eye, brow and mouth are not drawn; laces and eyelets merge into the sneaker; the four fingers form one slab; cuffs merge into the forearms.", "Auge, Braue und Mund werden nicht gezeichnet; Schnürung und Ösen gehen im Sneaker auf; die vier Finger bilden eine Platte; Bündchen gehen in den Unterarmen auf."], thinElements: ["The phone is drawn at its true thickness of 7.8 mm (0.08 mm on paper), never thickened.", "Das Telefon ist in seiner wahren Dicke von 7,8 mm (0,08 mm auf dem Papier) gezeichnet, nie verdickt."], overlays: ["Hood and hoodie share the hoodie value, the daypack carries its own value; details inherit the region of the garment they sit on.", "Kapuze und Pullover teilen den Wert des Pullovers, der Rucksack trägt seinen eigenen Wert; Details erben die Region des Kleidungsstücks, auf dem sie sitzen."] },
  "200": { silhouette: ["One mass per part of the 1:200 body: head, hair cap with ponytail, neck, sweatshirt with the hood, sleeves, forearms, hands, the phone slab, jeans seat, legs, shoes, the daypack.", "Eine Masse je Teil des 1:200-Körpers: Kopf, Haarkappe mit Pferdeschwanz, Hals, Sweatshirt mit Kapuze, Ärmel, Unterarme, Hände, die Telefonplatte, Jeansgesäß, Beine, Schuhe, der Tagesrucksack."], retained: ["Elbows and knees as separate masses so that the pose changes read, the hood on the upper back, the ponytail, the kangaroo pocket as the only inner line.", "Ellbogen und Knie als eigene Massen, damit die Posenwechsel lesbar sind, die Kapuze auf dem oberen Rücken, der Pferdeschwanz, die Kängurutasche als einzige Innenlinie."], omitted: ["Ear, nose, eye, cuffs, laces, seams, knee folds.", "Ohr, Nase, Auge, Bündchen, Schnürung, Nähte, Kniefalten."], thinElements: ["The phone is drawn at its true thickness of 7.8 mm and never thickened.", "Das Telefon ist in seiner wahren Dicke von 7,8 mm gezeichnet und nie verdickt."], styles: ["Body pen 0.18 mm, detail pen 0.10 mm for the pocket; light tones for skin and shoes, mid for hoodie, dark for jeans, hair and daypack.", "Körperstift 0,18 mm, Detailstift 0,10 mm für die Tasche; helle Töne für Haut und Schuhe, mittlere für den Pullover, dunkle für Jeans, Haar und Rucksack."], overlays: ["One region per mass; both sleeves share the sweatshirt value, ponytail and hair cap the hair value, the phone slab carries the phone value, the daypack its own.", "Eine Region je Masse; beide Ärmel teilen den Wert des Sweatshirts, Pferdeschwanz und Haarkappe den Haarwert, die Telefonplatte trägt den Telefonwert, der Rucksack seinen eigenen."] },
  "500": { silhouette: ["The fewest masses of the 1:500 body: head with hair, neck, sweatshirt, daypack, jeans seat and per side sleeve, forearm, hand, thigh, shank, shoe and toe box.", "Die wenigsten Massen des 1:500-Körpers: Kopf mit Haar, Hals, Sweatshirt, Tagesrucksack, Jeansgesäß und je Seite Ärmel, Unterarm, Hand, Oberschenkel, Unterschenkel, Schuh und Zehenkappe."], retained: ["The raised arm that carries the pose and the exact stature and extent.", "Der gehobene Arm, der die Pose trägt, sowie genaue Körpergröße und Ausdehnung."], merged: ["Face and hair; ponytail into the hair; hood into the sweatshirt shape; the phone into the right hand, which lists both regions.", "Gesicht und Haar; Pferdeschwanz ins Haar; Kapuze in die Sweatshirtform; das Telefon in die rechte Hand, die beide Regionen führt."], omitted: ["Sleeve cuffs, hood fold, laces, sole line, thumbs and fingers, ponytail and phone as shapes of their own.", "Ärmelbündchen, Kapuzenfalte, Schnürung, Sohlenlinie, Daumen und Finger, Pferdeschwanz und Telefon als eigene Formen."], thinElements: ["The 7.8 mm phone and the 50 mm ponytail are not drawn on their own and never thickened; the hands are drawn 43 mm thick so that they print.", "Das 7,8 mm dicke Telefon und der 50 mm starke Pferdeschwanz werden nicht eigens gezeichnet und nie verdickt; die Hände sind 43 mm dick gezeichnet, damit sie drucken."], overlays: ["Thermal and structural overlays paint the sweatshirt shape with the sweatshirt value and the right hand with the area-weighted value of hand and phone (labelled merged); the daypack keeps its own value.", "Thermische und statische Überlagerungen färben die Sweatshirtform mit dem Wert des Sweatshirts und die rechte Hand mit dem flächengewichteten Wert von Hand und Telefon (als zusammengefasst beschriftet); der Rucksack behält seinen eigenen Wert."] },
}, {
  "50": [["Body at 1:50: every form is a loft through hand-stated cross-sections, taken over from the base figure: head with jaw, nose, lips, brows, eyes and ears, the hair cap with tie and ponytail, neck, hooded sweatshirt with hem and collar rib, pocket and drawstrings, sleeves, forearms, hands with every phalanx, the phone, jeans and sneakers.", "Körper bei 1:50: Jede Form ist ein Loft durch von Hand festgelegte Querschnitte, von der Grundfigur übernommen: Kopf mit Kiefer, Nase, Lippen, Brauen, Augen und Ohren, die Haarkappe mit Haargummi und Pferdeschwanz, Hals, Kapuzenpullover mit Saum- und Kragenbund, Tasche und Kordeln, Ärmel, Unterarme, Hände mit allen Fingergliedern, das Telefon, Jeans und Sneaker."], ["Thermal at 1:50: the outer garment surfaces, hair, hands and phone are the body solids themselves; inside them lie the real inner layers of T-shirt, briefs and socks.", "Thermik bei 1:50: Die äußeren Kleidungsflächen, Haar, Hände und Telefon sind die Körper selbst; darin liegen die echten inneren Schichten aus T-Shirt, Slip und Socken."]],
  "100": [["Body at 1:100, with fewer and longer sections: head with nose and ears under the hair cap, the ponytail, neck, the sweatshirt as one form with the hood on the upper back, sleeves, forearms, hands, the phone, jeans and shoes.", "Körper bei 1:100 mit weniger und längeren Querschnitten: Kopf mit Nase und Ohren unter der Haarkappe, der Pferdeschwanz, Hals, das Sweatshirt als eine Form mit der Kapuze auf dem oberen Rücken, Ärmel, Unterarme, Hände, das Telefon, Jeans und Schuhe."], ["Thermal at 1:100: every region lies on the body forms of this scale; the layers are merged into the outer garment surface.", "Thermik bei 1:100: Jede Region liegt auf den Körperformen dieses Maßstabs; die Schichten sind in die äußere Kleidungsfläche zusammengefasst."]],
  "200": [["Body at 1:200, one mass per part: head, hair cap, ponytail, neck, sweatshirt with the hood, jeans seat, sleeves, forearms, hands, the phone plate, thighs, shanks, shoes.", "Körper bei 1:200, eine Masse je Teil: Kopf, Haarkappe, Pferdeschwanz, Hals, Sweatshirt mit Kapuze, Jeansgesäß, Ärmel, Unterarme, Hände, die Telefonplatte, Oberschenkel, Unterschenkel, Schuhe."], ["Thermal at 1:200: every region lies on the masses of this scale, layers merged; the plate carries the phone.", "Thermik bei 1:200: Jede Region liegt auf den Massen dieses Maßstabs, Schichten zusammengefasst; die Platte trägt das Telefon."]],
  "500": [["Body at 1:500, the fewest masses that read as a standing person: head, hair cap with the ponytail merged into it, neck, sweatshirt with the hood merged into the upper back, jeans seat, and per side sleeve, forearm, hand, thigh, shank, shoe and toe box.", "Körper bei 1:500, die wenigsten Massen, die als stehende Person lesbar sind: Kopf, Haarkappe mit darin aufgegangenem Pferdeschwanz, Hals, Sweatshirt mit im oberen Rücken aufgegangener Kapuze, Jeansgesäß und je Seite Ärmel, Unterarm, Hand, Oberschenkel, Unterschenkel, Schuh und Zehenkappe."], ["Thermal at 1:500: every region lies on the masses of this scale, layers merged; the phone is smaller than the smallest feature and keeps a block at the size floor of 75 mm at its place so that its region stays in the model.", "Thermik bei 1:500: Jede Region liegt auf den Massen dieses Maßstabs, Schichten zusammengefasst; das Telefon ist kleiner als das kleinste Merkmal und behält an seinem Ort einen Block in der Mindestgröße von 75 mm, damit seine Region im Modell bleibt."]],
});
if (left.length) console.log("[DEBUG] still empty", left.join(" "));
const HM: Record<number, string> = { 50: "35", 100: "17.5", 200: "8.8", 500: "3.5" };
for (const sc of [50, 100, 200, 500]) note(a, sc, "silhouette", `Default pose "selfie-arm-out" (${HM[sc]} mm tall at 1:${sc}): the right arm stretched forward with the phone, the left hand on the hip, the chin lifted; the daypack on the back; the body is the base figure's own drawing carried by the rig, headphones removed.`, `Standardpose „Selfie mit ausgestrecktem Arm" (${HM[sc]!.replace(".", ",")} mm hoch bei 1:${sc}): der rechte Arm mit dem Telefon nach vorn gestreckt, die linke Hand an der Hüfte, das Kinn gehoben; der Tagesrucksack auf dem Rücken; der Körper ist die eigene Zeichnung der Grundfigur, vom Rig getragen, Kopfhörer entfernt.`, true);
note(a, 50, "retained", "The daypack as a rounded box with its front pocket line, both shoulder straps over the chest in the front view.", "Der Tagesrucksack als gerundeter Kasten mit der Linie der Fronttasche, beide Schultergurte über der Brust in der Vorderansicht.");
note(a, 100, "retained", "The daypack as a rounded box with the pocket line and the shoulder straps in the front view.", "Der Tagesrucksack als gerundeter Kasten mit Taschenlinie und den Schultergurten in der Vorderansicht.");
note(a, 200, "retained", "The daypack as one mass on the back, 120 mm deep.", "Der Tagesrucksack als eine Masse auf dem Rücken, 120 mm tief.");
note(a, 200, "omitted", "Shoulder straps (42 mm wide, 9 mm thick) and the pocket line of the daypack.", "Schultergurte (42 mm breit, 9 mm dick) und Taschenlinie des Tagesrucksacks.");
note(a, 500, "retained", "The daypack (120 mm deep, 0.24 mm) as its own mass on the back; the headphones of the base figure are gone from the trunk silhouette.", "Der Tagesrucksack (120 mm tief, 0,24 mm) als eigene Masse auf dem Rücken; die Kopfhörer der Grundfigur sind aus der Rumpfsilhouette entfernt.");
const packVol: Record<string, [string, string]> = {
  "50": ["Daypack at 1:50: a rounded box 290 by 120 by 260 mm (radius 30 mm) on its own fixed bone against the back below the hood, and two padded shoulder straps 42 by 9 mm over the chest; thermal region daypack; in the structure one short link joins the pack to the spine.", "Tagesrucksack bei 1:50: ein gerundeter Kasten 290 auf 120 auf 260 mm (Radius 30 mm) auf einem eigenen festen Knochen am Rücken unter der Kapuze und zwei gepolsterte Schultergurte 42 auf 9 mm über der Brust; thermische Region daypack; im Tragwerk verbindet ein kurzes Glied den Rucksack mit der Wirbelsäule."],
  "100": ["Daypack at 1:100: the rounded box; the 9 mm straps are below the 15 mm feature and drawn only in the front view.", "Tagesrucksack bei 1:100: der gerundete Kasten; die 9 mm dicken Gurte liegen unter dem Merkmal von 15 mm und sind nur in der Vorderansicht gezeichnet."],
  "200": ["Daypack at 1:200: the rounded box as one mass.", "Tagesrucksack bei 1:200: der gerundete Kasten als eine Masse."],
  "500": ["Daypack at 1:500: the box (120 mm deep) as its own mass on the back.", "Tagesrucksack bei 1:500: der Kasten (120 mm tief) als eigene Masse auf dem Rücken."],
};
for (const [sc, [en, de]] of Object.entries(packVol)) a.volumes[sc].decisions.push({ en, de });
const e = measureDimensions(a);
console.log("[DEBUG] extents", JSON.stringify(e));
if (process.argv.includes("--out")) await Bun.write(process.argv[process.argv.indexOf("--out") + 1]!, JSON.stringify({ asset: a }));
else console.log(await writeAsset(a));
