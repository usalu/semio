/** 📷️ Generator of `photographer-with-camera`: derived from the rig, outlines and volumes of `tour-guide` (pointer and flag removed), a press photographer with a DSLR raised to the eye, standing and crouched; both hands placed on the camera by inverse kinematics.
 *  bun build-photographer-with-camera.ts [--out file.json]
 * @see ../../📋️brief.md */
import { addShapes, dropParts, measureDimensions, note, readAsset, roundRect, scrub, t, writeAsset, type Bones, type Json } from "./lib.ts";
import { at, compose, decompose, dir, frameOf, frames, relative, solve, sub, unit, type F, type Param, type V } from "./ik.ts";

const a: Json = await readAsset("tour-guide");
dropParts(a, ["pointer", "flag-pole", "flag"], undefined, ["pointer-metal", "pole-metal", "flag-cloth"], ["flag-textile"]);
const alu = a.materials.find((m: Json) => m.id === "pointer-aluminium"); alu.title = t("Anodised aluminium (badge clip)", "Eloxiertes Aluminium (Ausweisklammer)");
for (const s of ["l", "r"]) { const foot = a.volumes["500"].structure.find((x: Json) => x.member === `foot-${s}` && x.part === "member"); foot.members = [...new Set([...(foot.members ?? []), `toe-${s}`])]; }
for (const p of a.poses) { for (const id of ["pointer", "flag-pole", "flag"]) delete p.bones[id]; for (const id of ["pointer-metal", "pole-metal", "flag-cloth"]) delete p.thermal.regions[id]; }

/** 📷️ Camera-local geometry: origin on the lens axis in the plane of the back plate, u along the optical axis, v up, w to the photographer's left. */
const CAM = {
  body: { u: [0, 0.075], v: [-0.05, 0.045], w: [-0.085, 0.055], r: 0.01 },
  hump: { u: [0.008, 0.066], v: [0.04, 0.072], w: [-0.028, 0.028], r: 0.006 },
  eyepiece: { u: [-0.018, 0.002], v: [0.035, 0.068], w: [-0.02, 0.02], r: 0.005 },
  grip: { u: [0.02, 0.098], v: [-0.05, 0.035], w: [-0.092, -0.03], r: 0.012 },
  lens: { u: [0.075, 0.18], r: 0.037 },
  dial: { u: [0.02, 0.05], v: [0.045, 0.054], w: [0.03, 0.052], r: 0.004 },
};
const EYE_R: V = [0.055, -0.125, -0.032];
/** 🎯️ The camera frame for a head frame: eyepiece at the right eye, optical axis along the gaze. */
const cameraAtEye = (head: F): F => {
  const fwd = unit(dir(head, [0, -1, 0])), up = unit(dir(head, [1, 0, 0]));
  const eye = at(head, EYE_R);
  const f = frameOf([0, 0, 0], fwd, dir(head, [0, 0, 1]));
  return { ...f, o: sub(at({ ...f, o: eye }, [0.02, 0, 0]), [up[0] * 0.052, up[1] * 0.052, up[2] * 0.052]) as V };
};
/** ✋️ The right hand round the grip and the left hand under the lens, as frames in camera-local terms (palm of the right hand to the camera, palm of the left hand up). */
const GRIP_R = frameOf([0.01, -0.145, -0.1], sub([0.078, 0.03, -0.075], [0.01, -0.145, -0.1]), [0, 0, 1]);
const GRIP_L = frameOf([0.07, -0.085, 0.075], sub([0.14, -0.045, -0.095], [0.07, -0.085, 0.075]), [0, 1, 0]);

const arm = (s: "r" | "l"): Param[] => [
  { bone: `upper-arm-${s}`, field: "angleDeg" }, { bone: `upper-arm-${s}`, field: "abductionDeg", min: -120, max: 120 }, { bone: `upper-arm-${s}`, field: "twistDeg", min: -100, max: 100 },
  { bone: `forearm-${s}`, field: "angleDeg" }, { bone: `forearm-${s}`, field: "twistDeg", min: -95, max: 95 },
  { bone: `hand-${s}`, field: "angleDeg" }, { bone: `hand-${s}`, field: "abductionDeg", min: -35, max: 35 }, { bone: `hand-${s}`, field: "twistDeg", min: -45, max: 45 },
];
const frameError = (now: F, want: F, k = 0.12): number[] => [...sub(now.o, want.o), ...sub(now.u, want.u).map((e) => e * k), ...sub(now.w, want.w).map((e) => e * k)];
/** 🦾️ Places both hands on a camera frame given by a function of the solved head. */
function hold(start: Bones, camera: (f: Record<string, F>) => F): Bones {
  const best = (bones: Bones, s: "r" | "l", grip: F) => {
    let top = solve(a, bones, arm(s), (f) => frameError(f[`hand-${s}`]!, compose(camera(f), grip)), 60);
    const sg = s === "r" ? -1 : 1;
    for (const ang of [20, 50, 80]) for (const abd of [5, 35, 65]) for (const tw of [-50, 0, 50]) {
      const tryStart: Bones = { ...bones, [`upper-arm-${s}`]: { angleDeg: ang, abductionDeg: sg * abd, twistDeg: sg * tw }, [`forearm-${s}`]: { angleDeg: 110 }, [`hand-${s}`]: { angleDeg: 0 } };
      const c = solve(a, tryStart, arm(s), (f) => frameError(f[`hand-${s}`]!, compose(camera(f), grip)), 40);
      if (c.error < top.error) top = c;
    }
    return solve(a, top.bones, arm(s), (f) => frameError(f[`hand-${s}`]!, compose(camera(f), grip)), 150);
  };
  const r = best(start, "r", GRIP_R);
  const l = best(r.bones, "l", GRIP_L);
  const f = frames(a, l.bones);
  const show = (s: "r" | "l", g: F) => { const e = frameError(f[`hand-${s}`]!, compose(camera(f), g), 1); return `${s} pos ${Math.hypot(e[0]!, e[1]!, e[2]!).toFixed(3)} u ${Math.hypot(e[3]!, e[4]!, e[5]!).toFixed(2)} w ${Math.hypot(e[6]!, e[7]!, e[8]!).toFixed(2)}`; };
  console.log(`[DEBUG] hold ${show("r", GRIP_R)} ${show("l", GRIP_L)}`, JSON.stringify(Object.fromEntries(Object.entries(l.bones).filter(([k]) => /arm|hand/.test(k)))));
  return l.bones;
}

const legsStand: Bones = { "thigh-l": { angleDeg: 4 }, "shank-l": { angleDeg: -7.68 }, "foot-l": { angleDeg: 3.68 }, "thigh-r": { angleDeg: -2 }, "shank-r": { angleDeg: -4 }, "foot-r": { angleDeg: 6 }, "toe-l": { angleDeg: 0 }, "toe-r": { angleDeg: 0 } };
const shooting = hold({ ...legsStand, torso: { angleDeg: 1 }, neck: { angleDeg: -12 }, head: { angleDeg: 11 }, "clavicle-r": { angleDeg: -6 }, "clavicle-l": { angleDeg: 4 }, "upper-arm-r": { angleDeg: 45, abductionDeg: -60, twistDeg: 40 }, "forearm-r": { angleDeg: 120 }, "upper-arm-l": { angleDeg: 45, abductionDeg: 5 }, "forearm-l": { angleDeg: 110 } }, (f) => cameraAtEye(f.head!));

/** 🦵️ A deep squat: per leg the ankle under the knee at the given place, the foot flat. */
function squat(start: Bones): Bones {
  let bones = start;
  for (const [s, z] of [["r", -0.13], ["l", 0.13]] as const) {
    const params: Param[] = [{ bone: `thigh-${s}`, field: "angleDeg" }, { bone: `shank-${s}`, field: "angleDeg" }, { bone: `foot-${s}`, field: "angleDeg" }];
    bones = solve(a, bones, params, (f) => { const foot = f[`foot-${s}`]!; return [foot.o[0] - (s === "r" ? 0.06 : 0) - 0.15, foot.o[1] + 0.46, foot.u[1] * 0.5]; }).bones;
    void z;
  }
  return bones;
}
const crouchBase = squat({ pelvis: { angleDeg: -32 }, torso: { angleDeg: 6 }, neck: { angleDeg: 6 }, head: { angleDeg: 10 }, "thigh-r": { angleDeg: 110 }, "shank-r": { angleDeg: -125 }, "foot-r": { angleDeg: 30 }, "toe-r": { angleDeg: 0 }, "thigh-l": { angleDeg: 105 }, "shank-l": { angleDeg: -125 }, "foot-l": { angleDeg: 30 }, "toe-l": { angleDeg: 0 } });
const crouchHead = solve(a, crouchBase, [{ bone: "neck", field: "angleDeg" }, { bone: "head", field: "angleDeg" }], (f) => [f.head!.u[0], f.head!.u[2]]).bones;
const crouched = hold({ ...crouchHead, "clavicle-r": { angleDeg: -6 }, "upper-arm-r": { angleDeg: 45, abductionDeg: -60, twistDeg: 40 }, "forearm-r": { angleDeg: 120 }, "upper-arm-l": { angleDeg: 45, abductionDeg: 5 }, "forearm-l": { angleDeg: 110 } }, (f) => cameraAtEye(f.head!));

const screenAt = (f: Record<string, F>): F => {
  const torso = f.torso!;
  const o = at(torso, [0.3, -0.36, -0.01]);
  const fwd = unit([0.62, -0.78, 0]);
  return frameOf(o, fwd, [0, 0, 1]);
};
const checking = hold({ ...legsStand, torso: { angleDeg: -4 }, neck: { angleDeg: -8 }, head: { angleDeg: -6 }, "upper-arm-r": { angleDeg: 30, abductionDeg: -10 }, "forearm-r": { angleDeg: 90 }, "upper-arm-l": { angleDeg: 30, abductionDeg: 10 }, "forearm-l": { angleDeg: 90 } }, screenAt);

/** 🧷️ The camera bone, fixed to the right hand with the grip measured in the shooting pose. */
const F0 = frames(a, shooting);
const camWorld = compose(F0["hand-r"]!, relative(GRIP_R, { o: [0, 0, 0], u: [1, 0, 0], v: [0, 1, 0], w: [0, 0, 1] }));
const camRel = relative(F0["hand-r"]!, camWorld);
const rot = decompose(camRel);
const handIndex = a.rig.bones.findIndex((b: Json) => b.id === "hand-r");
a.rig.bones.splice(handIndex + 1, 0, { id: "camera", parent: "hand-r", title: t("Digital SLR camera with standard zoom", "Digitale Spiegelreflexkamera mit Standardzoom"), x: +camRel.o[0].toFixed(5), y: +camRel.o[1].toFixed(5), z: +camRel.o[2].toFixed(5), ...rot, lengthM: 0.18, joint: { type: "fixed" }, mass: { kg: 1.25, comX: 0.07, comY: -0.005, distribution: "point", source: "authored-estimate" } });
const restCam = frames(a, {}).camera!;
console.log("[DEBUG] camera rest", JSON.stringify(rot), "eye check", JSON.stringify(sub(frames(a, shooting).camera!.o, cameraAtEye(frames(a, shooting).head!).o)));

const restZ = restCam.o[2];

/** 🧢️ The cap turned backwards (so the brim clears the camera): every cap shape and solid turned 180° about the head axis through the middle of the skull. */
const C = -0.015;
const flipSolid = (s: Json): Json => {
  const tr = s.transform ?? {};
  const q = tr.rotation ?? [0, 0, 0, 1];
  const [x, y, z, w] = q;
  const t0 = tr.translation ?? [0, 0, 0];
  return { ...s, transform: { ...tr, rotation: [w, -z, y, -x].map((e: number) => Math.round(e * 1e6) / 1e6), translation: [t0[0], Math.round((2 * C - t0[1]) * 1e6) / 1e6, -t0[2]] } };
};
for (const rep of Object.values(a.representations) as Json[]) for (const view of ["elevation", "front", "plan"]) for (const sh of rep.geometry[view]) {
  if (sh.bone !== "head" || !/^cap/.test(sh.id)) continue;
  const sweep = sh.solid?.sweep;
  if (view === "elevation" || sweep === "u") sh.points = sh.points.map(([p, q]: number[]) => (view === "elevation" ? [p, +(2 * C - q).toFixed(5)] : [+(2 * C - p).toFixed(5), -q])).reverse();
  if (sweep === "v") { const [z0, z1] = [sh.solid.z0, sh.solid.z1]; sh.solid = { ...sh.solid, z0: +(2 * C - z1).toFixed(5), z1: +(2 * C - z0).toFixed(5) }; }
}
for (const vol of Object.values(a.volumes) as Json[]) for (const list of ["body", "thermal", "structure"]) vol[list] = vol[list].map((s: Json) => (s.bone === "head" && /^(th-)?cap/.test(s.id) && s.of === undefined ? flipSolid(s) : s));

/** 📷️ The camera in every view and scale. */
type Part = { id: string; u: number[]; v: number[]; w: number[]; r: number; role?: string; tone?: number };
const box = (id: string, p: { u: number[]; v: number[]; w: number[]; r: number }): Part => ({ id, ...p });
const lensPart: Part = { id: "camera-lens", u: CAM.lens.u, v: [-CAM.lens.r, CAM.lens.r], w: [-CAM.lens.r, CAM.lens.r], r: 0.006, tone: 0.18 };
const parts: Record<string, Part[]> = {
  "50": [box("camera-body", CAM.body), box("camera-prism", CAM.hump), box("camera-eyepiece", CAM.eyepiece), box("camera-grip", CAM.grip), lensPart, box("camera-dial", CAM.dial)],
  "100": [box("camera-body", CAM.body), box("camera-prism", CAM.hump), box("camera-eyepiece", CAM.eyepiece), box("camera-grip", CAM.grip), lensPart],
  "200": [box("camera-body", { u: [-0.005, 0.098], v: [-0.05, 0.065], w: [-0.092, 0.055], r: 0.012 }), { ...lensPart, v: [-0.05, 0.037], w: [-0.05, 0.037], r: 0.012 }],
  "500": [box("camera", { u: [-0.01, 0.18], v: [-0.05, 0.065], w: [-0.092, 0.055], r: 0.015 })],
};
const camShape = (p: Part) => ({ id: p.id, bone: "camera", role: p.role ?? "body", z: 7.5, filled: true, tone: p.tone ?? 0.22, region: "camera", material: "camera-body", kind: "poly", points: roundRect(p.u[0]!, p.u[1]!, p.v[0]!, p.v[1]!, Math.min(p.r, (p.v[1]! - p.v[0]!) / 2.01, (p.u[1]! - p.u[0]!) / 2.01)), closed: true, solid: { kind: "extruded", sweep: "w", z0: +(restZ + p.w[0]!).toFixed(5), z1: +(restZ + p.w[1]!).toFixed(5) } });
const camSolid = (p: Part, sc: string): Json => p.id === "camera-lens" && sc !== "500" && sc !== "200"
  ? { id: p.id, kind: "loft", bone: "camera", material: "camera-body", role: "body", tone: 0.18, sections: [[0.075, 0.033], [0.085, 0.036], [0.172, 0.038], [0.18, 0.036]].map(([u, r]) => ({ at: [u, 0, 0], rx: r, ry: r, exponent: 2, rotation: [0.5, 0.5, 0.5, 0.5] })), capStart: true, capEnd: true }
  : { id: p.id, kind: "box", bone: "camera", material: "camera-body", role: "body", tone: p.tone ?? 0.22, size: [p.u[1]! - p.u[0]!, p.v[1]! - p.v[0]!, p.w[1]! - p.w[0]!], cornerRadius: Math.min(p.r, (p.v[1]! - p.v[0]!) / 2.01, (p.u[1]! - p.u[0]!) / 2.01, (p.w[1]! - p.w[0]!) / 2.01), transform: { translation: [(p.u[0]! + p.u[1]!) / 2, (p.v[0]! + p.v[1]!) / 2, (p.w[0]! + p.w[1]!) / 2] } };
for (const sc of ["50", "100", "200", "500"]) {
  const ps = parts[sc]!;
  for (const view of ["elevation", "front", "plan"]) addShapes(a, +sc, view, ps.map(camShape));
  if (sc === "50" || sc === "100") {
    const ring = (u: number) => ({ id: `camera-lens-ring-${Math.round(u * 1000)}`, bone: "camera", role: "detail", z: 7.6, filled: false, tone: 0.1, region: "camera", kind: "poly", points: [[u, -0.036], [u, 0.036]], closed: false, solid: { kind: "extruded", sweep: "w", z0: +(restZ - 0.036).toFixed(5), z1: +(restZ + 0.036).toFixed(5) } });
    addShapes(a, +sc, "elevation", sc === "50" ? [ring(0.12), ring(0.155)] : [ring(0.14)]);
  }
  const vol = a.volumes[sc];
  if (sc === "500") {
    const block = camSolid(ps[0]!, sc);
    vol.body.push(block);
    vol.thermal.push({ id: "th-camera", of: "camera", region: "camera" });
    vol.structure.push({ id: "camera-frame", of: "camera", member: "camera", part: "member" }, { id: "camera-grip-link", kind: "box", bone: "camera", material: "camera-body", role: "body", tone: 0.85, size: [0.075, 0.075, 0.075], transform: { translation: [0.04, -0.06, -0.1] }, member: "camera", part: "member" });
    continue;
  }
  vol.body.push(...ps.map((p) => camSolid(p, sc)));
  vol.thermal.push(...ps.map((p) => ({ id: `th-${p.id}`, of: p.id, region: "camera" })));
  vol.structure.push({ id: "camera-frame", of: "camera-body", member: "camera", part: "member" }, { id: "camera-grip-link", kind: "box", bone: "camera", material: "camera-body", role: "body", tone: 0.85, size: sc === "200" ? [0.04, 0.05, 0.04] : [0.03, 0.05, 0.03], transform: { translation: [0.04, -0.06, -0.1] }, member: "camera", part: "member" });
}
a.materials.push({ id: "camera-body", title: t("Polycarbonate and magnesium camera body, rubberised grip, glass lens", "Kameragehäuse aus Polycarbonat und Magnesium, gummierter Griff, Glasobjektiv"), emissivity: 0.9, solarAbsorptivity: 0.85, specificHeatJKgK: 1100, source: "authored-estimate" });
a.thermal.regions.push({ id: "camera", title: t("Digital SLR camera, warmed by sensor, processor and battery", "Digitale Spiegelreflexkamera, von Sensor, Prozessor und Akku erwärmt"), kind: "surface", model: "operating", material: "camera-body", areaM2: 0.09, characteristicLengthM: 0.14, geometry: "vertical-plate", skyViewFactor: 0.4, layers: [], heatCapacityJM2K: 14000, operating: { key: "camera", heatW: 2.5 }, ratedPowerW: 4.5, ratedPowerBasis: "electrical-input", source: "authored-estimate" });
a.thermal.operating = [...(a.thermal.operating ?? []), { id: "camera", title: t("Camera state", "Zustand der Kamera"), levels: [{ v: 0, title: t("Off", "Aus") }, { v: 0.3, title: t("Standby, metering", "Bereitschaft, Belichtungsmessung") }, { v: 0.6, title: t("Reviewing on the rear screen", "Bildkontrolle auf dem Rückdisplay") }, { v: 1, title: t("Shooting a burst", "Serienaufnahme") }] }];
const lanyard = a.anchors.find((x: Json) => x.id === "grip-r");
void lanyard;

const base = structuredClone(a.poses.find((p: Json) => p.id === "holding-flag"));
const sunny = (exposure: Record<string, number> = {}) => Object.fromEntries(Object.entries(base.thermal.regions).map(([id, r]: [string, Json]) => [id, { ...r, ...(exposure[id] === undefined ? {} : { exposedFraction: exposure[id] }) }]));
const pose = (id: string, title: Json, description: Json, bones: Bones, met: number, breathing: string, camera: number, regions: Record<string, number> = {}, shoes?: number, contacts = ["contact-l", "heel-l", "contact-r", "heel-r"]): Json => {
  const r = sunny(regions);
  if (shoes !== undefined) r.shoes = { ...r.shoes, contact: { ...r.shoes.contact, fraction: shoes } };
  return { ...structuredClone(base), id, title, description, bones, ground: "auto", contacts, loads: [],
    thermal: { ...structuredClone(base.thermal), activityMet: met, breathing, regions: { ...r, camera: { exposedFraction: 0.8, sunlitFraction: 0.5 } }, operating: { camera } } };
};
const hanging: Bones = { ...legsStand, torso: { angleDeg: 0 }, neck: { angleDeg: -2 }, head: { angleDeg: 1 }, "upper-arm-r": { angleDeg: 10, abductionDeg: -4 }, "forearm-r": { angleDeg: 32 }, "hand-r": { angleDeg: 8 }, "upper-arm-l": { angleDeg: 4, abductionDeg: 4 }, "forearm-l": { angleDeg: 12 }, "hand-l": { angleDeg: 4 } };
a.poses = [
  pose("shooting-standing", t("Standing, camera at the eye", "Stehend, Kamera am Auge"), t("Feet a little apart with the left foot ahead, trunk upright and the head pushed slightly forward into the camera: the right eye at the viewfinder, the right hand round the grip with the index finger on the shutter, the left palm cradling the zoom ring from below, both elbows close to the chest for a steady hold.", "Füße etwas auseinander, der linke Fuß vorn, Rumpf aufrecht und der Kopf leicht in die Kamera geschoben: das rechte Auge am Sucher, die rechte Hand um den Griff mit dem Zeigefinger am Auslöser, die linke Handfläche stützt den Zoomring von unten, beide Ellbogen nah am Brustkorb für einen ruhigen Halt."), shooting, 1.5, "rest", 1),
  pose("shooting-crouched", t("Crouched low, camera at the eye", "Tief in der Hocke, Kamera am Auge"), t("A deep squat on flat feet with the trunk leaning forward and the knees ahead of the toes, to shoot from a low eye height of about 1.2 m; the head is level, the camera held as when standing.", "Eine tiefe Hocke auf flachen Füßen mit nach vorn geneigtem Rumpf und den Knien vor den Zehen, um aus einer niedrigen Augenhöhe von etwa 1,2 m zu fotografieren; der Kopf ist waagerecht, die Kamera wird wie im Stehen gehalten."), crouched, 1.8, "rest", 1, { "leg-clothing": 0.85, "torso-clothing": 0.95 }, 0.55),
  pose("checking-screen", t("Checking the shot on the screen", "Aufnahme auf dem Display prüfen"), t("Standing, the camera lowered to chest height in both hands and tilted toward the face, the head bowed to look at the rear screen.", "Stehend, die Kamera mit beiden Händen auf Brusthöhe gesenkt und zum Gesicht geneigt, der Kopf gesenkt, um auf das Rückdisplay zu sehen."), checking, 1.3, "rest", 0.6),
  pose("standing-ready", t("Standing ready, camera in hand", "Bereit stehend, Kamera in der Hand"), t("Relaxed stance between shots, the camera carried by the grip in the right hand at hip height, the left arm hanging.", "Entspannter Stand zwischen den Aufnahmen, die Kamera am Griff in der rechten Hand auf Hüfthöhe getragen, der linke Arm hängt."), hanging, 1.3, "rest", 0.3),
];
const feet = (d: number) => ["contact-l", "heel-l", "contact-r", "heel-r"].map((anchor) => ({ anchor, fromS: 0, toS: d }));
const key = (tS: number, p: string, bones?: Bones) => ({ t: tS, pose: p, ease: "smooth", ...(bones ? { bones } : {}) });
const zoom = (d: number): Bones => ({ "forearm-l": { ...shooting["forearm-l"], twistDeg: (shooting["forearm-l"]!.twistDeg ?? 0) + d }, torso: { angleDeg: 1 - d / 20 } });
a.clips = [
  { id: "shutter-burst", title: t("Framing and shooting a burst", "Ausrichten und Serienaufnahme"), description: t("A 2.4-second loop at the eye: the left hand turns the zoom ring a little, the breath is held while the shutter fires a burst of three, then the trunk settles and the ring turns back.", "Eine Schleife von 2,4 Sekunden am Auge: Die linke Hand dreht den Zoomring ein wenig, der Atem wird angehalten, während der Auslöser eine Serie von drei Bildern auslöst, dann setzt sich der Rumpf und der Ring dreht zurück."),
    durationS: 2.4, fps: 30, loop: true, rootMotion: "none",
    keys: [key(0, "shooting-standing"), key(0.6, "shooting-standing", zoom(8)), key(1.2, "shooting-standing", zoom(8)), key(1.8, "shooting-standing", zoom(-4)), key(2.4, "shooting-standing")],
    contacts: feet(2.4), phases: [{ id: "framing", fromS: 0, toS: 0.6, title: t("Zooming and framing", "Zoomen und Ausrichten") }, { id: "burst", fromS: 0.6, toS: 1.2, title: t("Breath held, burst of three", "Atem angehalten, Serie von drei Bildern") }, { id: "settle", fromS: 1.2, toS: 2.4, title: t("Settling and reframing", "Nachsetzen und neu ausrichten") }],
    events: [0.7, 0.85, 1.0].map((tS, i) => ({ t: tS, kind: "operating", target: "camera", title: t(`Shutter release ${i + 1} of 3`, `Auslösung ${i + 1} von 3`) })),
    tracks: [{ target: "operating:camera", ease: "smooth", keys: [{ t: 0, v: 0.6 }, { t: 0.6, v: 1 }, { t: 1.2, v: 1 }, { t: 2.4, v: 0.6 }] }, { target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 2.4, v: 0 }] }] },
  { id: "crouch-and-shoot", title: t("Crouching down to shoot", "In die Hocke gehen und fotografieren"), description: t("A 6-second sequence: from standing with the camera in hand the photographer raises it to the eye while sinking into a deep squat, shoots, rises again and checks the picture on the screen.", "Eine Abfolge von 6 Sekunden: Aus dem Stand mit der Kamera in der Hand hebt die Person sie ans Auge, während sie in eine tiefe Hocke sinkt, löst aus, richtet sich wieder auf und prüft das Bild auf dem Display."),
    durationS: 6, fps: 30, loop: false, rootMotion: "none",
    keys: [key(0, "standing-ready"), key(0.8, "shooting-standing"), key(2.0, "shooting-crouched"), key(3.2, "shooting-crouched"), key(4.4, "shooting-standing"), key(6, "checking-screen")],
    contacts: feet(6), phases: [{ id: "raise", fromS: 0, toS: 0.8, title: t("Camera to the eye", "Kamera ans Auge") }, { id: "sink", fromS: 0.8, toS: 2, title: t("Sinking into the squat", "In die Hocke sinken") }, { id: "shoot", fromS: 2, toS: 3.2, title: t("Shooting low", "Von unten fotografieren") }, { id: "rise", fromS: 3.2, toS: 4.4, title: t("Rising", "Aufrichten") }, { id: "review", fromS: 4.4, toS: 6, title: t("Checking the screen", "Display prüfen") }],
    events: [{ t: 2.6, kind: "operating", target: "camera", title: t("Shutter release", "Auslösung") }],
    tracks: [{ target: "operating:camera", ease: "smooth", keys: [{ t: 0, v: 0.3 }, { t: 0.8, v: 1 }, { t: 4.4, v: 1 }, { t: 6, v: 0.6 }] }, { target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 1.3 }, { t: 2, v: 2.4 }, { t: 3.2, v: 1.8 }, { t: 4.4, v: 2.4 }, { t: 6, v: 1.3 }] }, { target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 6, v: 0 }] }] },
  { id: "idle-breathing", title: t("Idle breathing between shots", "Ruhiges Atmen zwischen den Aufnahmen"), description: t("A five-second loop standing ready: the chest rises and falls, the camera hand sways slightly; the feet stay planted.", "Eine Schleife von fünf Sekunden in Bereitschaft: Die Brust hebt und senkt sich, die Kamerahand pendelt leicht; die Füße bleiben stehen."),
    durationS: 5, fps: 30, loop: true, rootMotion: "none",
    keys: [key(0, "standing-ready"), key(1.25, "standing-ready", { torso: { angleDeg: -0.4 }, head: { angleDeg: 2 }, "forearm-r": { angleDeg: 35 } }), key(2.5, "standing-ready", { torso: { angleDeg: 0.3 }, head: { angleDeg: 0.5 } }), key(3.75, "standing-ready", { torso: { angleDeg: -0.3 }, head: { angleDeg: 1.6 }, "forearm-r": { angleDeg: 29 } }), key(5, "standing-ready")],
    contacts: feet(5), phases: [{ id: "idle", fromS: 0, toS: 5, title: t("Waiting for the moment", "Auf den Moment warten") }], events: [], tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 5, v: 0 }] }] },
];
a.presentation = { defaultPose: "shooting-standing", defaultClip: "shutter-burst", defaultScale: 100 };
a.id = "photographer-with-camera";
a.title = t("Photographer with Camera", "Fotografierende Person mit Kamera");
a.tags = ["human", "adult", "photographer", "camera", "dslr", "press", "crouching", "standing"];
a.summary = t("A 1.75 m press photographer (78 kg) in a softshell jacket, cotton trousers and a cap turned backwards, a press pass on a lanyard, shooting with a 1.25 kg digital SLR and standard zoom: the right eye at the viewfinder, the right hand round the grip, the left palm under the lens. Poses standing and in a deep crouch at the eye, checking the screen and standing ready; a burst loop, a crouch-and-shoot sequence and idle breathing; the camera counts 4.5 W of electrical input.", "Eine 1,75 m große Pressefotografin bzw. ein Pressefotograf (78 kg) in Softshelljacke, Baumwollhose und verkehrt herum getragener Mütze, mit Presseausweis am Umhängeband, fotografiert mit einer 1,25 kg schweren digitalen Spiegelreflexkamera mit Standardzoom: das rechte Auge am Sucher, die rechte Hand um den Griff, die linke Handfläche unter dem Objektiv. Posen stehend und in tiefer Hocke am Auge, beim Prüfen des Displays und bereit stehend; eine Serienaufnahme als Schleife, eine Abfolge aus Hocke und Aufnahme sowie ruhiges Atmen; die Kamera zählt 4,5 W elektrische Leistungsaufnahme.");
const refs = a.provenance.references.filter((r: Json) => !["iso-8996-2021", "ainsworth-2011", "cipa-dc-002"].includes(r.id));
refs.push({ id: "iso-8996-2021", citation: "ISO 8996:2021 Ergonomics of the thermal environment, determination of metabolic rate (activity classes and met values; metabolic rate for body postures)." });
refs.push({ id: "ainsworth-2011", citation: "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine and Science in Sports and Exercise 43(8), 1575-1581 (activity classes; MET values are mapped to thermal met as an estimate)." });
a.provenance = { ...a.provenance, author: "Semio staffage human worker HW-11", created: "2026-11-22", references: refs, notes: t("Skeleton, outlines and volumes derived by script from the archive asset tour-guide (1.75 m, de Leva segment masses); pointer, flag pole and flag removed, the cap turned backwards (every cap shape and solid rotated 180 degrees about the head axis) so that the brim clears the camera. The camera is handcrafted: a DSLR body 140 by 95 by 75 mm with pentaprism hump, eyepiece, grip and mode dial and a standard zoom of 74 mm diameter and 105 mm length (typical of an APS-C body with an 18-135 mm lens, 1.25 kg with battery), on its own fixed bone in the right hand; the grip of both hands is a stated hand frame relative to the camera, and the arm angles of every pose are solved by inverse kinematics on the rig so that the eyepiece sits at the right eye. Electrical input 4.5 W while shooting (catalogue-typical battery drain, authored estimate). Activity: 1.5 met standing and 1.8 met squatting with the camera raised (ISO 8996 low metabolic rate with static arm work and posture), 1.3 met reviewing or waiting.", "Skelett, Umrisse und Volumen per Skript aus dem Archivobjekt tour-guide abgeleitet (1,75 m, Segmentmassen nach de Leva); Zeiger, Fahnenstange und Fahne entfernt, die Mütze verkehrt herum gedreht (jede Mützenform und jeder Mützenkörper um 180 Grad um die Kopfachse gedreht), damit der Schirm die Kamera freigibt. Die Kamera ist von Hand gebaut: ein Spiegelreflexgehäuse 140 auf 95 auf 75 mm mit Prismenbuckel, Okular, Griff und Moduswahlrad und ein Standardzoom von 74 mm Durchmesser und 105 mm Länge (typisch für ein APS-C-Gehäuse mit 18-135-mm-Objektiv, 1,25 kg mit Akku), auf einem eigenen festen Knochen in der rechten Hand; der Griff beider Hände ist ein festgelegter Handrahmen relativ zur Kamera, und die Armwinkel jeder Pose sind durch inverse Kinematik am Rig gelöst, sodass das Okular am rechten Auge liegt. Elektrische Leistungsaufnahme 4,5 W beim Fotografieren (katalogtypischer Akkuverbrauch, Schätzung). Aktivität: 1,5 met stehend und 1,8 met in der Hocke mit erhobener Kamera (ISO 8996, niedriger Umsatz mit statischer Armarbeit und Körperhaltung), 1,3 met beim Prüfen oder Warten.") };

for (const line of scrub(a, /flag|pointer|pole|guide|ferrule|finial|hoist/i, /Fahne|Zeiger|Stange|Führ|Gummikappe|Knauf|Hohlsaum/i)) console.log("[DEBUG] left", line.slice(0, 300));
const fill = (x: Json, en: string, de: string) => { if (x.en.trim() === "") { x.en = en; x.de = de; } };
const R = a.representations;
fill(R["50"].decisions.merged, "Cap, brim and button are one shape, hand and fingers one shape per hand, cuff and forearm one shape, sole and lacing belong to the shoe.", "Mütze, Schirm und Knopf sind eine Form, Hand und Finger eine Form je Hand, Bündchen und Unterarm eine Form, Sohle und Schnürung gehören zum Schuh.");
fill(R["50"].decisions.omitted, "Fabric folds, seams, laces as single strands and the camera's neck strap.", "Stofffalten, Nähte, Schnürsenkel als einzelne Stränge und der Tragegurt der Kamera.");
fill(R["50"].decisions.thinElements, "The lanyard (7 mm) is a stroke, the two lens rings are strokes; nothing is thickened.", "Das Umhängeband (7 mm) ist ein Strich, die beiden Objektivringe sind Striche; nichts wird verdickt.");
fill(R["50"].decisions.styles, "0.25 mm outlines, 0.13 mm detail strokes; jacket dark, chinos mid, skin light, the camera near black.", "0,25-mm-Umrisse, 0,13-mm-Detailstriche; Jacke dunkel, Chinohose mittel, Haut hell, die Kamera fast schwarz.");
fill(R["50"].decisions.overlays, "Each shape carries the thermal region of the garment, body part or device it shows (jacket, sleeves, wrist, hands, face, hair and cap, chinos, shoes, badge, camera), in all three views.", "Jede Form trägt die thermische Region des Kleidungsstücks, Körperteils oder Geräts, das sie zeigt (Jacke, Ärmel, Handgelenk, Hände, Gesicht, Haar und Mütze, Chinohose, Schuhe, Ausweis, Kamera), in allen drei Ansichten.");
fill(R["100"].decisions.omitted, "Face details, zips, pockets, lacing.", "Gesichtsdetails, Reißverschlüsse, Taschen, Schnürung.");
fill(R["200"].decisions.omitted, "Ears, lanyard, zips, pockets, the camera's dial and eyepiece as separate forms.", "Ohren, Umhängeband, Reißverschlüsse, Taschen, Wahlrad und Okular der Kamera als eigene Formen.");
fill(R["500"].decisions.omitted, "Face, ears, badge, lanyard, fingers and the camera's details.", "Gesicht, Ohren, Ausweis, Umhängeband, Finger und die Einzelheiten der Kamera.");
const SIL: Record<string, [string, string]> = {
  "50": ["Every filled shape is the true orthographic outline of the 1:50 model, per bone: head with nose, lips, ears and eyes under the cap, neck and collar, softshell jacket with skirt, sleeves and cuffs, hands, chinos, shoes with sole and toe, the camera parts. Inner lines: zips, pockets, lanyard, mouth and lens rings.", "Jede gefüllte Form ist der wahre orthogonale Umriss des 1:50-Modells, je Knochen: Kopf mit Nase, Lippen, Ohren und Augen unter der Mütze, Hals und Kragen, Softshelljacke mit Schoß, Ärmel und Bündchen, Hände, Chinohose, Schuhe mit Sohle und Spitze, die Kamerateile. Innenlinien: Reißverschlüsse, Taschen, Umhängeband, Mund und Objektivringe."],
  "100": ["Every filled shape is the true outline of the 1:100 model: head with nose and ears, cap with brim, hair at the nape, neck and collar, jacket and skirt, sleeves, forearms, hands, trousers, shoes, the camera with prism, eyepiece, grip and lens.", "Jede gefüllte Form ist der wahre Umriss des 1:100-Modells: Kopf mit Nase und Ohren, Mütze mit Schirm, Haar im Nacken, Hals und Kragen, Jacke und Schoß, Ärmel, Unterarme, Hände, Hose, Schuhe, die Kamera mit Prisma, Okular, Griff und Objektiv."],
  "200": ["Every filled shape is the true outline of the 1:200 model: head with nose, cap with brim, hair, neck and collar, jacket and skirt, sleeves, forearms, hands, trousers, shoes, camera body and lens.", "Jede gefüllte Form ist der wahre Umriss des 1:200-Modells: Kopf mit Nase, Mütze mit Schirm, Haar, Hals und Kragen, Jacke und Schoß, Ärmel, Unterarme, Hände, Hose, Schuhe, Kameragehäuse und Objektiv."],
  "500": ["Every filled shape is the true outline of the 1:500 model: head, cap with brim, neck, the jacket as one form, sleeves, forearms, hands, seat, legs, shoes and the camera block.", "Jede gefüllte Form ist der wahre Umriss des 1:500-Modells: Kopf, Mütze mit Schirm, Hals, die Jacke als eine Form, Ärmel, Unterarme, Hände, Gesäß, Beine, Schuhe und der Kamerablock."],
};
for (const [sc, [en, de]] of Object.entries(SIL)) fill(R[sc].decisions.silhouette, en, de);
fill(R["100"].decisions.thinElements, "The lanyard is a 0.13 mm stroke; nothing is thickened.", "Das Umhängeband ist ein 0,13-mm-Strich; nichts wird verdickt.");
fill(R["200"].decisions.retained, "The badge on the chest and the camera with its lens at the eye, so the figure reads as a press photographer.", "Der Ausweis auf der Brust und die Kamera mit dem Objektiv am Auge, damit die Figur als Pressefotograf lesbar ist.");
fill(R["200"].decisions.thinElements, "None; lens (74 mm) and camera body (103 mm) at their true outline.", "Keine; Objektiv (74 mm) und Kameragehäuse (103 mm) in ihrem wahren Umriss.");
fill(R["500"].decisions.retained, "The cap brim at the back, the raised forearms and the camera block at the eye, so the photographer reads at 3.6 mm height.", "Der Mützenschirm hinten, die gehobenen Unterarme und der Kamerablock am Auge, damit die fotografierende Person bei 3,6 mm Höhe lesbar ist.");
fill(R["500"].decisions.thinElements, "None.", "Keine.");
const th = a.thermal.assumptions;
fill(th[1], "Standing photographer at 1.3 to 1.8 met (ISO 8996 low metabolic rate with static arm work), 16 degrees Celsius, 55 % humidity, 1.5 m/s wind and 250 W/m2 direct sun: sunlit areas receive the full irradiance, shaded areas the diffuse share.", "Stehende fotografierende Person mit 1,3 bis 1,8 met (ISO 8996, niedriger Umsatz mit statischer Armarbeit), 16 Grad Celsius, 55 % Feuchte, 1,5 m/s Wind und 250 W/m2 direkter Sonne: Besonnte Flächen erhalten die volle Einstrahlung, beschattete den diffusen Anteil.");
fill(th[3], "The camera is an operating surface: 2.5 W of its 4.5 W electrical input are released at the housing while shooting a burst (level 1), proportionally less when reviewing or on standby.", "Die Kamera ist eine Betriebsfläche: 2,5 W ihrer 4,5 W elektrischen Leistungsaufnahme werden bei Serienaufnahmen (Stufe 1) am Gehäuse frei, anteilig weniger bei Bildkontrolle oder in Bereitschaft.");
fill(a.structure.assumptions[0], "The 1.25 kg camera is carried by the right hand through the grip and steadied by the left palm under the lens; with both forearms raised, the shoulder moments are the weights of arm, hand and camera times their lever arms.", "Die 1,25 kg schwere Kamera wird von der rechten Hand am Griff getragen und von der linken Handfläche unter dem Objektiv gestützt; bei beiden gehobenen Unterarmen sind die Schultermomente die Gewichte von Arm, Hand und Kamera mal ihren Hebelarmen.");
th[0].en = th[0].en.replace("78.59 kg", "78.0 kg (without the camera)"); th[0].de = th[0].de.replace("78,59 kg", "78,0 kg (ohne Kamera)").replace("78.59 kg", "78,0 kg (ohne Kamera)");
const volText: Record<string, [string, string]> = {
  "50": ["1:50 body: every part is a loft through hand-stated sections, taken over from the base figure: head with jaw, nose, lips, ears and eyes under a six-panel cap (worn backwards here), short hair at temples and nape; the neck in the stand collar of a hip-length softshell jacket with front zip, zipped pockets, chest pocket and hem band; the lanyard with the press pass; sleeves, forearms with cuffs, the hands with thumb and finger phalanges; chinos and leather shoes with sole and lacing.", "Körper bei 1:50: Jedes Teil ist ein Loft durch von Hand festgelegte Querschnitte, von der Grundfigur übernommen: Kopf mit Kiefer, Nase, Lippen, Ohren und Augen unter einer Sechs-Panel-Mütze (hier verkehrt herum), kurzes Haar an Schläfen und Nacken; der Hals im Stehkragen einer hüftlangen Softshelljacke mit Frontreißverschluss, Reißverschlusstaschen, Brusttasche und Saumbund; das Umhängeband mit dem Presseausweis; Ärmel, Unterarme mit Bündchen, die Hände mit Daumen und Fingergliedern; Chinohose und Lederschuhe mit Sohle und Schnürung."],
  "100": ["1:100 body (smallest feature 15 mm): the same figure in fewer sections: head with nose and ears, the cap with its brim a feature thick, hair at temples and nape, neck in the stand collar; the jacket as upper part and skirt with the hem band, the badge on its lanyard; sleeves, forearms, hands as palm with finger band and thumb; trousers and shoes.", "Körper bei 1:100 (kleinstes Merkmal 15 mm): dieselbe Figur mit weniger Querschnitten: Kopf mit Nase und Ohren, die Mütze mit einem merkmalsdicken Schirm, Haar an Schläfen und Nacken, Hals im Stehkragen; die Jacke als Oberteil und Schoß mit Saumbund, der Ausweis am Umhängeband; Ärmel, Unterarme, Hände als Handfläche mit Fingerband und Daumen; Hose und Schuhe."],
  "200": ["1:200 body (smallest feature 30 mm): head with the nose as one form, hair at the nape, the cap with a 30 mm brim, neck and collar; jacket upper part and skirt, the badge; sleeves, forearms and hands as single forms with the thumb; trousers and shoes.", "Körper bei 1:200 (kleinstes Merkmal 30 mm): Kopf mit der Nase als eine Form, Haar im Nacken, die Mütze mit 30 mm Schirm, Hals und Kragen; Jackenoberteil und Schoß, der Ausweis; Ärmel, Unterarme und Hände als einzelne Formen mit Daumen; Hose und Schuhe."],
  "500": ["1:500 body (smallest feature 75 mm): head, the cap as crown and brim over it, neck; the jacket as one form from hem to shoulders, sleeves and forearms, the hands as single forms; seat, trouser legs in two sections, one shoe per foot.", "Körper bei 1:500 (kleinstes Merkmal 75 mm): Kopf, die Mütze als Kopfteil und Schirm darüber, Hals; die Jacke als eine Form vom Saum bis zu den Schultern, Ärmel und Unterarme, die Hände als einzelne Formen; Gesäß, Hosenbeine in zwei Abschnitten, ein Schuh je Fuß."],
};
for (const [sc, vol] of Object.entries(a.volumes) as [string, Json][]) vol.decisions.forEach((d: Json, i: number) => fill(d, ...(i === 0 ? volText[sc]! : [`1:${sc} ${i === 1 ? "thermal: every region lies on the solids of this scale, garment layers merged into the outer surface." : "structure: the skeleton of the base figure as the load path."}`, `${i === 1 ? "Thermik" : "Tragwerk"} bei 1:${sc}: ${i === 1 ? "Jede Region liegt auf den Körpern dieses Maßstabs, Kleidungsschichten in die Außenfläche zusammengefasst." : "Das Skelett der Grundfigur als Lastpfad."}`] as [string, string])));
for (const [k, rep] of Object.entries(R) as [string, Json][]) for (const [f, v] of Object.entries(rep.decisions) as [string, Json][]) if (v.en.trim() === "") console.log("[DEBUG] empty", k, f);
const HM: Record<number, string> = { 50: "35.7", 100: "17.9", 200: "8.9", 500: "3.6" };
for (const sc of [50, 100, 200, 500]) note(a, sc, "silhouette", `Default pose "shooting-standing" (${HM[sc]} mm tall at 1:${sc}): the camera at the right eye, both forearms raised in front of the chest, the cap turned backwards; the body is the base figure's own drawing carried by the rig, pointer, flag pole and flag removed.`, `Standardpose „Stehend fotografieren" (${HM[sc]!.replace(".", ",")} mm hoch bei 1:${sc}): die Kamera am rechten Auge, beide Unterarme vor der Brust gehoben, die Mütze verkehrt herum; der Körper ist die eigene Zeichnung der Grundfigur, vom Rig getragen, Zeiger, Fahnenstange und Fahne entfernt.`, true);
note(a, 50, "retained", "The camera as body, pentaprism hump, eyepiece, grip, mode dial and lens, two lens ring lines; every part an extruded solid in all three views, so it turns correctly with the hands.", "Die Kamera als Gehäuse, Prismenbuckel, Okular, Griff, Moduswahlrad und Objektiv, zwei Objektivringlinien; jedes Teil ein extrudierter Körper in allen drei Ansichten, damit es sich mit den Händen richtig dreht.");
note(a, 100, "retained", "The camera as body, prism hump, eyepiece, grip and lens with one ring line.", "Die Kamera als Gehäuse, Prismenbuckel, Okular, Griff und Objektiv mit einer Ringlinie.");
note(a, 100, "omitted", "The mode dial of the camera (22 mm).", "Das Moduswahlrad der Kamera (22 mm).");
note(a, 200, "merged", "Prism hump, eyepiece and grip merge into one camera body mass; the lens stays a separate mass.", "Prismenbuckel, Okular und Griff verschmelzen zu einer Gehäusemasse; das Objektiv bleibt eine eigene Masse.");
note(a, 500, "merged", "The camera is one block from eyepiece to lens front (190 by 115 by 147 mm), the smallest mass that prints.", "Die Kamera ist ein Block vom Okular bis zur Objektivfront (190 auf 115 auf 147 mm), die kleinste Masse, die druckt.");
const camVol: Record<string, [string, string]> = {
  "50": ["Camera at 1:50: rounded boxes for body, prism hump, eyepiece, grip and mode dial and a lofted cylinder for the zoom (74 mm, the focus ring 76 mm); thermal region camera on every part; in the structure the body is the member of the camera bone, joined to the bones of the right hand by a short grip link.", "Kamera bei 1:50: gerundete Kästen für Gehäuse, Prismenbuckel, Okular, Griff und Moduswahlrad und ein geloftetes Zylinderobjektiv (74 mm, Fokusring 76 mm); thermische Region camera auf jedem Teil; im Tragwerk ist das Gehäuse der Stab des Kameraknochens, über ein kurzes Griffglied mit den Knochen der rechten Hand verbunden."],
  "100": ["Camera at 1:100: body, prism hump, eyepiece, grip and lens; the 8 mm dial is below the 15 mm feature and left out.", "Kamera bei 1:100: Gehäuse, Prismenbuckel, Okular, Griff und Objektiv; das 8 mm hohe Wahlrad liegt unter dem Merkmal von 15 mm und entfällt."],
  "200": ["Camera at 1:200: one body mass with hump and grip, and the lens.", "Kamera bei 1:200: eine Gehäusemasse mit Buckel und Griff sowie das Objektiv."],
  "500": ["Camera at 1:500: one block of 190 by 115 by 147 mm, the lens merged in, so that it reaches the 75 mm feature.", "Kamera bei 1:500: ein Block von 190 auf 115 auf 147 mm mit darin aufgegangenem Objektiv, damit er das Merkmal von 75 mm erreicht."],
};
for (const [sc, [en, de]] of Object.entries(camVol)) a.volumes[sc].decisions.push({ en, de }, { en: "The cap is worn backwards: its solids are the base cap turned 180 degrees about the head axis.", de: "Die Mütze sitzt verkehrt herum: Ihre Körper sind die Mütze der Grundfigur, um 180 Grad um die Kopfachse gedreht." });
const bodyKg = a.rig.bones.filter((b: Json) => b.id !== "camera").reduce((s: number, b: Json) => s + (b.mass?.kg ?? 0), 0);
a.thermal.body = { ...a.thermal.body, massKg: Math.round(bodyKg * 100) / 100, skinAreaM2: Math.round(0.007184 * Math.pow(bodyKg, 0.425) * Math.pow(175, 0.725) * 1000) / 1000 };
const e = measureDimensions(a);
console.log("[DEBUG] extents", JSON.stringify(e));
if (process.argv.includes("--out")) await Bun.write(process.argv[process.argv.indexOf("--out") + 1]!, JSON.stringify({ asset: a }));
else console.log(await writeAsset(a));
console.log("[DEBUG] crouch legs", JSON.stringify(Object.fromEntries(Object.entries(crouched).filter(([k]) => /thigh|shank|foot|toe|pelvis|neck|head|torso/.test(k)))));
