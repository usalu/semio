import { writeFileSync } from "node:fs";
import { sampleClip } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎞️animation/🟦️.ts";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const out = process.argv[2]!;
const T = (en: string, de: string) => ({ en, de });
const W = "winter-2009";

const bone = (id: string, parent: string | null, x: number, y: number, angleDeg: number, lengthM: number, joint: object, title: [string, string], mass?: [number, number, number]) => ({
  id, parent, title: T(...title), x, y, angleDeg, lengthM, joint,
  ...(mass ? { mass: { kg: mass[0], comX: mass[1], comY: mass[2], distribution: "point", source: "literature", reference: W } } : {}),
});
const rev = (minDeg: number, maxDeg: number) => ({ type: "revolute", minDeg, maxDeg });
const side = (s: "l" | "r") => (s === "l" ? ["Left", "Linker"] : ["Right", "Rechter"]) as [string, string];

const bones = [
  bone("pelvis", null, 0, 0, 90, 0.1, rev(-20, 20), ["Pelvis", "Becken"], [10, 0.05, 0]),
  bone("torso", "pelvis", 0.1, 0, 0, 0.45, rev(-30, 45), ["Torso", "Rumpf"], [27.3, 0.225, 0]),
  bone("head", "torso", 0.45, 0, 0, 0.22, rev(-45, 45), ["Head and neck", "Kopf und Hals"], [6.1, 0.11, 0]),
  ...(["l", "r"] as const).flatMap((s) => [
    bone(`arm-${s}`, "torso", 0.42, 0, -180, 0.6, rev(-60, 170), [`${side(s)[0]} arm`, `${side(s)[1]} Arm`], [3.75, 0.27, 0]),
    bone(`thigh-${s}`, "pelvis", 0, 0, -180, 0.45, rev(-30, 100), [`${side(s)[0]} thigh`, `${side(s)[1]} Oberschenkel`], [7.5, 0.1949, 0]),
    bone(`shank-${s}`, `thigh-${s}`, 0.45, 0, 0, 0.43, rev(-140, 0), [`${side(s)[0]} shank`, `${side(s)[1]} Unterschenkel`], [3.5, 0.1862, 0]),
    bone(`foot-${s}`, `shank-${s}`, 0.43, 0, 90, 0.22, rev(-50, 40), [`${side(s)[0]} foot`, `${side(s)[1]} Fuß`], [1.1, 0.11, -0.04]),
  ]),
];

const anchor = (id: string, b: string, x: number, y: number, kind: string, title: [string, string], extra: object = {}) => ({ id, bone: b, x, y, kind, title: T(...title), ...extra });
const anchors = [
  ...(["l", "r"] as const).map((s) => anchor(`contact-${s}`, `foot-${s}`, 0.19, -0.08, "contact", [`${side(s)[0]} sole`, `${side(s)[1]} Sohle`], { support: "contact", normalDeg: 90, footprintLengthM: 0.12, footprintWidthM: 0.09, frictionCoefficient: 0.6 })),
  anchor("mouth", "head", 0.07, -0.09, "emitter", ["Mouth and nose", "Mund und Nase"]),
  anchor("bag-strap", "torso", 0.4, 0.1, "load", ["Shoulder bag strap", "Riemen der Umhängetasche"]),
];

const shape = (id: string, b: string, role: string, lod: number, z: number, geometry: object, extra: object = {}) => ({ id, bone: b, role, lod, z, filled: true, tone: 0.5, ...geometry, ...extra });
const capsule = (x0: number, y0: number, x1: number, y1: number, r0: number, r1: number) => ({ kind: "capsule", x0, y0, x1, y1, r0, r1 });
const elevation = [
  shape("head", "head", "body", 0, 5, { kind: "ellipse", cx: 0.1, cy: 0, rx: 0.12, ry: 0.095 }, { region: "head-skin", material: "skin", tone: 0.85 }),
  shape("neck", "head", "body", 1, 4, capsule(-0.06, 0, 0.06, 0, 0.05, 0.05), { region: "head-skin", material: "skin", tone: 0.85 }),
  shape("hair", "head", "detail", 2, 6, { kind: "path", d: "M 0.03 0.09 C 0.10 0.15 0.215 0.10 0.215 0 C 0.215 0.06 0.12 0.08 0.03 0.09 Z" }, { tone: 0.2 }),
  shape("torso", "torso", "body", 0, 3, capsule(0, 0, 0.45, 0, 0.13, 0.16), { region: "torso-clothing", material: "cotton-clothing", tone: 0.45 }),
  shape("torso-weave", "torso", "texture", 3, 3.1, { kind: "poly", points: [[0.1, -0.05], [0.35, -0.05]], closed: false }, { region: "torso-clothing", material: "cotton-clothing", filled: false }),
  shape("pelvis", "pelvis", "body", 0, 3, capsule(0, 0, 0.1, 0, 0.14, 0.14), { region: "leg-clothing", material: "cotton-clothing", tone: 0.3 }),
  ...(["l", "r"] as const).flatMap((s) => {
    const z = s === "r" ? 2 : 1;
    const az = s === "r" ? 6 : 0.5;
    return [
      shape(`arm-${s}`, `arm-${s}`, "body", 0, az, capsule(0, 0, 0.6, 0, 0.045, 0.035), { region: "arm-clothing", material: "cotton-clothing", tone: 0.5 }),
      shape(`thigh-${s}`, `thigh-${s}`, "body", 0, z, capsule(0, 0, 0.45, 0, 0.085, 0.065), { region: "leg-clothing", material: "cotton-clothing", tone: 0.3 }),
      shape(`shank-${s}`, `shank-${s}`, "body", 0, z, capsule(0, 0, 0.43, 0, 0.06, 0.04), { region: "leg-clothing", material: "cotton-clothing", tone: 0.3 }),
      shape(`foot-${s}`, `foot-${s}`, "body", 0, z, { kind: "rect", x: -0.06, y: -0.08, w: 0.27, h: 0.07, r: 0.02 }, { region: "shoes", material: "leather-shoe", tone: 0.15 }),
    ];
  }),
];
const plan = [
  shape("plan-shoulders", "torso", "body", 0, 1, { kind: "ellipse", cx: 0, cy: 0, rx: 0.12, ry: 0.23 }, { tone: 0.45 }),
  shape("plan-head", "head", "body", 0, 2, { kind: "ellipse", cx: 0, cy: 0, rx: 0.1, ry: 0.09 }, { tone: 0.85 }),
];

const walkOffsets = (frontLeft: boolean, passing: boolean) => {
  const lead = frontLeft ? "l" : "r";
  const back = frontLeft ? "r" : "l";
  const o: Record<string, object> = {};
  if (!passing) {
    o[`thigh-${lead}`] = { angleDeg: 22 }; o[`shank-${lead}`] = { angleDeg: -3 }; o[`foot-${lead}`] = { angleDeg: -19 };
    o[`thigh-${back}`] = { angleDeg: -18 }; o[`shank-${back}`] = { angleDeg: -15 }; o[`foot-${back}`] = { angleDeg: -2 };
    o[`arm-${back}`] = { angleDeg: 25 }; o[`arm-${lead}`] = { angleDeg: -25 };
  } else {
    o[`thigh-${lead}`] = { angleDeg: 4 }; o[`shank-${lead}`] = { angleDeg: -10 }; o[`foot-${lead}`] = { angleDeg: 6 };
    o[`thigh-${back}`] = { angleDeg: 20 }; o[`shank-${back}`] = { angleDeg: -55 }; o[`foot-${back}`] = { angleDeg: 20 };
    o[`arm-${back}`] = { angleDeg: 5 }; o[`arm-${lead}`] = { angleDeg: -5 };
  }
  return o;
};
const pose = (id: string, title: [string, string], description: [string, string], bones: object, contacts: string[], met: number, breath: number, extra: object = {}) => ({
  id, title: T(...title), description: T(...description), bones, ground: "auto", contacts, thermal: { activityMet: met, emitters: { breath } }, loads: [], ...extra,
});
const bag = { id: "shoulder-bag", kind: "carried", title: T("Shoulder bag", "Umhängetasche"), anchor: "bag-strap", fxN: 0, fyN: -49.03, source: "authored-estimate" };
const poses = [
  pose("stand", ["Standing", "Stehend"], ["Relaxed upright standing on both feet.", "Entspanntes aufrechtes Stehen auf beiden Füßen."], {}, ["contact-l", "contact-r"], 1.2, 0.5),
  pose("stride-left-forward", ["Stride, left foot forward", "Schritt, linker Fuß vorn"], ["Left heel strike, right foot at toe-off, contralateral arm swing.", "Linker Fersenkontakt, rechter Fuß beim Abdruck, gegenläufiger Armschwung."], walkOffsets(true, false), ["contact-l"], 2.4, 1),
  pose("passing-left-stance", ["Passing, left stance", "Durchschwingen, linkes Standbein"], ["Left foot carries, right leg swings through.", "Linker Fuß trägt, rechtes Bein schwingt durch."], walkOffsets(true, true), ["contact-l"], 2.4, 1),
  pose("stride-right-forward", ["Stride, right foot forward", "Schritt, rechter Fuß vorn"], ["Right heel strike, left foot at toe-off.", "Rechter Fersenkontakt, linker Fuß beim Abdruck."], walkOffsets(false, false), ["contact-r"], 2.4, 1),
  pose("passing-right-stance", ["Passing, right stance", "Durchschwingen, rechtes Standbein"], ["Right foot carries, left leg swings through.", "Rechter Fuß trägt, linkes Bein schwingt durch."], walkOffsets(false, true), ["contact-r"], 2.4, 1),
  pose("carry-bag", ["Standing with shoulder bag", "Stehend mit Umhängetasche"], ["Upright with a 5 kg bag hanging from the left shoulder.", "Aufrecht mit einer 5-kg-Tasche an der linken Schulter."], { pelvis: { angleDeg: 2 }, torso: { angleDeg: -3 }, "arm-l": { angleDeg: 6 } }, ["contact-l", "contact-r"], 1.4, 0.6, { loads: [bag] }),
];

const key = (t: number, p: string, ease = "linear", bones?: object) => ({ t, pose: p, ease, ...(bones ? { bones } : {}) });
const ev = (t: number, kind: string, title: [string, string], target?: string) => ({ t, kind, title: T(...title), ...(target ? { target } : {}) });
const KEYS = 96;
const r4 = (v: number) => Number(v.toFixed(4));
const L1 = 0.45, L2 = 0.43, SPEED = Number(process.env.WALK_SPEED ?? 1), STANCE = Number(process.env.WALK_SPEED ?? 1) * 0.25 + Number(process.env.WALK_OFF ?? 0.19), ANKLE_Y = 0.08;
const smooth = (w: number) => w * w * (3 - 2 * w);
const BALL = { x: 0.19, y: -0.08 };
const footPath = (tau: number) => {
  if (tau <= 0.55) return { x: STANCE, y: 0, abs: -35 * smooth(Math.min(1, Math.max(0, (tau - 0.3) / 0.25))) };
  const w = (tau - 0.55) / 0.4;
  return { x: STANCE + SPEED * smooth(w), y: 0.06 * Math.sin(Math.PI * w) ** 2, abs: -35 * (1 - smooth(w)) };
};
const footAt = (u: number, right: boolean) => {
  let tau = right ? u - 0.5 : u;
  let shift = right ? SPEED / 2 : 0;
  if (tau < -0.05) { tau += 1; shift -= SPEED; }
  if (tau > 0.95) { tau -= 1; shift += SPEED; }
  const f = footPath(tau);
  const c = Math.cos((f.abs * Math.PI) / 180), n = Math.sin((f.abs * Math.PI) / 180);
  return { x: f.x + shift - (BALL.x * c - BALL.y * n), y: f.y - (BALL.x * n + BALL.y * c), abs: f.abs };
};
const ik = (hip: { x: number; y: number }, foot: { x: number; y: number; abs: number }) => {
  const dx = foot.x - hip.x, dy = foot.y - hip.y;
  const d = Math.hypot(dx, dy);
  if (d > L1 + L2) throw new Error("leg out of reach");
  const beta = Math.acos((L1 * L1 + d * d - L2 * L2) / (2 * L1 * d));
  const thigh = Math.atan2(dy, dx) + beta;
  const knee = { x: hip.x + L1 * Math.cos(thigh), y: hip.y + L1 * Math.sin(thigh) };
  const shank = Math.atan2(foot.y - knee.y, foot.x - knee.x);
  const deg = (r: number) => (r * 180) / Math.PI;
  return { thigh: deg(thigh) + 90, shank: deg(shank - thigh), foot: foot.abs - deg(shank) - 90 };
};
const gaitBones = (u: number) => {
  const hip = { x: SPEED * u, y: 0.855 + 0.01 * Math.cos(4 * Math.PI * (u - 0.25)) };
  const left = ik(hip, footAt(u, false));
  const right = ik(hip, footAt(u, true));
  const arm = (phase: number) => -25 * Math.cos(phase) - 5 * Math.sin(phase);
  return {
    "thigh-l": { angleDeg: r4(left.thigh) }, "shank-l": { angleDeg: r4(left.shank) }, "foot-l": { angleDeg: r4(left.foot) },
    "thigh-r": { angleDeg: r4(right.thigh) }, "shank-r": { angleDeg: r4(right.shank) }, "foot-r": { angleDeg: r4(right.foot) },
    "arm-l": { angleDeg: r4(arm(2 * Math.PI * u)) }, "arm-r": { angleDeg: r4(arm(2 * Math.PI * u + Math.PI)) },
  };
};
const walk = (advanceM: number) => ({
  id: "walk", title: T("Walking", "Gehen"), description: T("Level walking at about 1.0 m/s, one stride per second, feet planted without sliding; joint angles from two-link inverse kinematics of smooth foot paths, sampled at 96 keys.", "Gehen in der Ebene mit etwa 1,0 m/s, ein Doppelschritt pro Sekunde, Füße gleiten nicht; Gelenkwinkel aus zweigliedriger inverser Kinematik glatter Fußbahnen in 96 Schlüsseln."),
  durationS: 1, fps: 30, loop: true, rootMotion: "plant", advanceM,
  keys: Array.from({ length: KEYS + 1 }, (_, k) => key(k / KEYS, "stride-left-forward", "linear", gaitBones(k / KEYS))),
  contacts: [{ anchor: "contact-l", fromS: 0, toS: 0.55 }, { anchor: "contact-r", fromS: 0, toS: 0.05 }, { anchor: "contact-r", fromS: 0.45, toS: 1 }, { anchor: "contact-l", fromS: 0.95, toS: 1 }],
  phases: [{ id: "left-stance", fromS: 0, toS: 0.5, title: T("Left stance", "Linkes Standbein") }, { id: "right-stance", fromS: 0.5, toS: 1, title: T("Right stance", "Rechtes Standbein") }],
  events: [ev(0, "contact-on", ["Left heel strike", "Linker Fersenkontakt"], "contact-l"), ev(0.05, "contact-off", ["Right toe-off", "Rechter Abdruck"], "contact-r"), ev(0.45, "contact-on", ["Right heel strike", "Rechter Fersenkontakt"], "contact-r"), ev(0.55, "contact-off", ["Left toe-off", "Linker Abdruck"], "contact-l")],
  tracks: [{ target: "emitter:breath", ease: "smooth", keys: [{ t: 0, v: 0.7 }, { t: 0.5, v: 1 }, { t: 1, v: 0.7 }] }],
});
const idle = {
  id: "idle-breathing", title: T("Breathing at rest", "Ruhiges Atmen"), description: T("Standing still with a 4 s breathing cycle and a slight chest and head motion.", "Ruhiges Stehen mit 4-s-Atemzyklus und leichter Brust- und Kopfbewegung."),
  durationS: 4, fps: 30, loop: true, rootMotion: "none",
  keys: [key(0, "stand", "smooth"), key(2, "stand", "smooth", { torso: { angleDeg: 1 }, head: { angleDeg: -1 } }), key(4, "stand", "smooth")],
  contacts: [{ anchor: "contact-l", fromS: 0, toS: 4 }, { anchor: "contact-r", fromS: 0, toS: 4 }],
  phases: [{ id: "inhale", fromS: 0, toS: 2, title: T("Inhale", "Einatmen") }, { id: "exhale", fromS: 2, toS: 4, title: T("Exhale", "Ausatmen") }],
  events: [ev(0, "inhale", ["Start of inhalation", "Beginn des Einatmens"]), ev(2, "exhale", ["Start of exhalation", "Beginn des Ausatmens"])],
  tracks: [{ target: "emitter:breath", ease: "smooth", keys: [{ t: 0, v: 0 }, { t: 2, v: 0 }, { t: 2.4, v: 1 }, { t: 3.6, v: 0.4 }, { t: 4, v: 0 }] }],
};
const swing = {
  id: "bag-swing", title: T("Swinging bag", "Schwingende Tasche"), description: T("Standing with the shoulder bag; the bag load rises by 30 % at the swing maximum.", "Stehen mit Umhängetasche; die Taschenlast steigt am Schwingungsmaximum um 30 %."),
  durationS: 2, fps: 30, loop: true, rootMotion: "none",
  keys: [key(0, "carry-bag", "smooth"), key(1, "carry-bag", "smooth", { torso: { angleDeg: -5 } }), key(2, "carry-bag", "smooth")],
  contacts: [{ anchor: "contact-l", fromS: 0, toS: 2 }, { anchor: "contact-r", fromS: 0, toS: 2 }],
  phases: [{ id: "swing", fromS: 0, toS: 2, title: T("Swing", "Schwingen") }],
  events: [ev(1, "motion", ["Swing maximum", "Schwingungsmaximum"])],
  tracks: [{ target: "load:shoulder-bag", ease: "smooth", keys: [{ t: 0, v: 1 }, { t: 1, v: 1.3 }, { t: 2, v: 1 }] }],
};

const region = (id: string, title: [string, string], kind: string, material: string, model: string, areaM2: number, characteristicLengthM: number, share: number, extra: object, ref: string, source = "literature") => ({
  id, title: T(...title), kind, material, model, areaM2, characteristicLengthM, ...extra, metabolicShare: share, source, reference: ref,
});
const regions = [
  region("head-skin", ["Head and neck skin", "Haut von Kopf und Hals"], "skin", "skin", "core-resistance", 0.1, 0.2, 0.1, { coreTempC: 36.8, insulationM2KW: 0.05 }, "dubois-1916"),
  region("torso-clothing", ["Torso clothing", "Rumpfbekleidung"], "clothing", "cotton-clothing", "core-resistance", 0.58, 0.3, 0.4, { coreTempC: 36.8, insulationM2KW: 0.14 }, "iso-9920-2007"),
  region("arm-clothing", ["Arm clothing", "Armbekleidung"], "clothing", "cotton-clothing", "core-resistance", 0.28, 0.08, 0.15, { coreTempC: 36.8, insulationM2KW: 0.12 }, "iso-9920-2007"),
  region("leg-clothing", ["Leg clothing", "Beinbekleidung"], "clothing", "cotton-clothing", "core-resistance", 0.82, 0.13, 0.35, { coreTempC: 36.8, insulationM2KW: 0.14 }, "iso-9920-2007"),
  region("shoes", ["Shoes", "Schuhe"], "surface", "leather-shoe", "passive", 0.12, 0.1, 0, {}, "iso-9920-2007", "authored-estimate"),
];
const material = (id: string, title: [string, string], emissivity: number, solarAbsorptivity: number, source: string, reference?: string) => ({ id, title: T(...title), emissivity, solarAbsorptivity, source, ...(reference ? { reference } : {}) });
const ref = (id: string, citation: string) => ({ id, citation });

const build = (advanceM: number, dimensions: object) => ({
  id: "reference-walker", category: "humans", title: T("Reference walker", "Referenz-Fußgänger"),
  summary: T("A two-legged adult walker with planted-foot root motion, used as the engine reference fixture.", "Ein zweibeiniger erwachsener Fußgänger mit Standfuß-Wurzelbewegung als Referenz-Fixture der Engine."),
  tags: ["human", "walker", "fixture"],
  provenance: {
    author: "Semio staffage core engine", method: "handcrafted", created: "2026-10-29",
    references: [
      ref(W, "Winter, D. A. (2009). Biomechanics and Motor Control of Human Movement, 4th ed., Wiley. Table 4.1, segment masses and centres of mass, scaled to 75 kg."),
      ref("ashrae-2021", "ASHRAE Handbook Fundamentals (2021), Chapter 9 Thermal Comfort."),
      ref("iso-9920-2007", "ISO 9920:2007 Ergonomics of the thermal environment, estimation of thermal insulation and water vapour resistance of a clothing ensemble."),
      ref("dubois-1916", "DuBois, D. and DuBois, E. F. (1916). A formula to estimate the approximate surface area if height and weight be known. Archives of Internal Medicine 17, 863-871."),
      ref("iso-8996-2021", "ISO 8996:2021 Ergonomics of the thermal environment, determination of metabolic rate."),
    ],
    notes: T("A minimal but complete fixture; poses are authored estimates of gait key frames.", "Eine minimale, aber vollständige Fixture; die Posen sind Schätzungen von Gang-Schlüsselbildern."),
  },
  dimensions,
  materials: [
    material("skin", ["Skin", "Haut"], 0.98, 0.6, "literature", "ashrae-2021"),
    material("cotton-clothing", ["Cotton clothing", "Baumwollkleidung"], 0.95, 0.5, "literature", "ashrae-2021"),
    material("leather-shoe", ["Leather shoe", "Lederschuh"], 0.9, 0.7, "authored-estimate"),
  ],
  rig: { kind: "anatomical", root: "pelvis", bones },
  anchors,
  geometry: { elevation, plan },
  poses,
  clips: [walk(advanceM), idle, swing],
  thermal: {
    environment: { airTempC: 20, meanRadiantTempC: 20, relativeHumidity: 0.5, windMS: 0.2, windDirectionDeg: 180, solarWM2: 0, pressurePa: 101325 },
    baseMet: 1.2, regions,
    emitters: [{ id: "breath", kind: "breath", title: T("Exhaled breath", "Ausgeatmete Luft"), anchor: "mouth", directionDeg: 0, carries: "vapour", temperatureC: 34, velocityMS: 1.5, diameterM: 0.03, vapourKgPerKg: 0.034, periodS: 4, dutyFrom: 0.5, dutyTo: 1, source: "authored-estimate" }],
    assumptions: [T("Metabolic shares follow the surface-area weighting of the clothed regions.", "Die Stoffwechselanteile folgen der flächengewichteten Verteilung der bekleideten Regionen."), T("Insulation values run from core to outer surface.", "Die Dämmwerte gelten vom Körperkern bis zur Außenoberfläche.")],
  },
  structure: { gravityMS2: 9.80665, dynamics: "inertial", supports: [], assumptions: [T("Segments are point masses at the centres of Winter's anthropometric table.", "Segmente sind Punktmassen in den Schwerpunkten der Anthropometrie-Tabelle von Winter.")], clinical: false },
  presentation: { defaultPose: "stand", defaultClip: "walk", viewpoints: ["elevation", "plan"] },
  exports: ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"],
  completion: "authored",
});

let asset: any = build(0, { heightM: 1.7, widthM: 0.3, depthM: 0.46, massKg: 75.1 });
const walkClip = asset.clips[0];
const advance = sampleClip(asset, walkClip, 1 - 1e-12).root.x - sampleClip(asset, walkClip, 0).root.x;
asset = build(Number(advance.toFixed(6)), asset.dimensions);
const bounds = evaluate(asset, { kind: "pose", pose: "stand" }).bounds;
const mass = asset.rig.bones.reduce((sum: number, b: any) => sum + (b.mass?.kg ?? 0), 0);
asset = build(Number(advance.toFixed(6)), { heightM: Number((bounds.maxY - bounds.minY).toFixed(3)), widthM: Number((bounds.maxX - bounds.minX).toFixed(3)), depthM: 0.46, massKg: Number(mass.toFixed(1)) });
writeFileSync(out, JSON.stringify(asset, null, 2) + "\n");
console.log(JSON.stringify({ advance, dimensions: asset.dimensions }));
