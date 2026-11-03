import { readFileSync } from "node:fs";
import { apply, solveRig } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { capsule, ellipse, line, pathShape, poly, rect, type Pens, type Shape } from "./plan-lib.ts";

type Asset = { rig: { bones: { id: string; lengthM: number }[] }; thermal: { regions: { id: string }[] }; representations: Record<string, { pens: Pens; geometry: { elevation: { region?: string; tone: number; material?: string }[] } }>; materials: { id: string }[] };

export type Garment = {
  readonly torso: string;
  readonly sleeve: string;
  readonly legs: string;
  readonly shoes: string;
  readonly hair: string;
  readonly face: string;
  readonly hand: string;
  readonly neckwear?: string;
  readonly bag?: string;
};

export type HumanSpec = {
  readonly seated: boolean;
  readonly armZ: number;
  readonly legZ: number;
  readonly garment: Garment;
  readonly shoulderY: number;
  readonly sleeveR: number;
  readonly forearmR: number;
  readonly shoulderRx: number;
  readonly shoulderRy: number;
  readonly waistRx: number;
  readonly waistRy: number;
  readonly connectorR: number;
  readonly headRx: number;
  readonly headRy: number;
  readonly legY: number;
  readonly thighR: readonly [number, number];
  readonly shankR: readonly [number, number];
  readonly heel: number;
  readonly shoeHeelR: number;
  readonly shoeBallR: number;
  readonly neckwear?: { readonly rx: number; readonly ry: number };
  readonly bag?: { readonly x: number; readonly y: number; readonly w: number; readonly h: number };
};

export function humanPlan(asset: Asset, spec: HumanSpec, scale: "50" | "100" | "200" | "500"): Shape[] {
  const world = solveRig(asset.rig as never, {}, { x: 0, y: 0, angleDeg: 0 });
  const length = (id: string): number => asset.rig.bones.find((b) => b.id === id)!.lengthM;
  const o = (id: string): number => apply(world[id]!, 0, 0).x;
  const t = (id: string): number => apply(world[id]!, length(id), 0).x;
  const regions = new Set(asset.thermal.regions.map((r) => r.id));
  const materials = new Set(asset.materials.map((m) => m.id));
  const ref = asset.representations["100"]!.geometry.elevation;
  const toneOf = (region: string, fallback: number): number => ref.find((s) => s.region === region)?.tone ?? fallback;
  const materialOf = (region: string): string | undefined => {
    const id = ref.find((s) => s.region === region)?.material;
    return id !== undefined && materials.has(id) ? id : undefined;
  };
  const g = spec.garment;
  const only = (region: string): { region?: string; material?: string } => ({ ...(regions.has(region) ? { region } : {}), ...(materialOf(region) ? { material: materialOf(region) } : {}) });
  const merged = (list: readonly string[]): { regions?: string[] } => {
    const known = list.filter((r) => regions.has(r));
    return known.length > 1 ? { regions: known } : {};
  };
  const pens = asset.representations[scale]!.pens;
  const detail = pens.detail !== null;
  const out: Shape[] = [];
  const toneTorso = toneOf(g.torso, 0.5);
  const toneSleeve = toneOf(g.sleeve, toneTorso);
  const toneLegs = toneOf(g.legs, 0.4);
  const toneShoes = toneOf(g.shoes, 0.15);
  const toneHair = toneOf(g.hair, 0.2);
  const toneFace = toneOf(g.face, 0.8);
  const toneHand = toneOf(g.hand, 0.8);
  const sides = [
    { s: "l", sign: 1 },
    { s: "r", sign: -1 },
  ] as const;
  const headX = o("head") - 0.005;

  if (scale === "500") {
    const bodyRegions = [g.torso, g.hair, g.face].filter((r) => regions.has(r));
    out.push(ellipse("shoulders", "neck", "body", 4, toneTorso, o("neck"), 0, spec.shoulderRx, spec.shoulderRy + spec.sleeveR, { ...merged(bodyRegions) }));
    out.push(ellipse("head", "head", "body", 6, toneHair, headX, 0, spec.headRx, spec.headRy, only(g.hair)));
    return out;
  }

  if (scale === "200") {
    if (!spec.seated) {
      for (const { s, sign } of sides) {
        out.push(capsule(`foot-${s}`, "foot-" + s, "body", 1, toneShoes, o("foot-" + s) - spec.heel, sign * spec.legY, t("foot-" + s) + 0.06, sign * spec.legY, spec.shoeHeelR, spec.shoeBallR, { followTip: true, ...only(g.shoes) }));
      }
    }
    for (const { s, sign } of sides) {
      out.push(capsule(`arm-${s}`, "arm-" + s, "body", spec.armZ, toneSleeve, o("arm-" + s), sign * spec.shoulderY, t("arm-" + s), sign * spec.shoulderY, spec.sleeveR, spec.forearmR, { followTip: true, ...only(g.sleeve) }));
      out.push(capsule(`forearm-${s}`, "forearm-" + s, "body", spec.armZ, toneSleeve, o("forearm-" + s), sign * spec.shoulderY, t("forearm-" + s), sign * spec.shoulderY, spec.forearmR, spec.forearmR * 0.75, { followTip: true, ...only(g.sleeve) }));
    }
    out.push(capsule("trunk", "pelvis", "body", 4, toneTorso, o("pelvis"), 0, t("torso"), 0, spec.waistRy * 0.8, spec.shoulderRy, { followTip: true, ...only(g.torso) }));
    out.push(ellipse("shoulders", "neck", "body", 4.1, toneTorso, o("neck"), 0, spec.shoulderRx, spec.shoulderRy, only(g.torso)));
    out.push(ellipse("head", "head", "body", 6, toneHair, headX, 0, spec.headRx, spec.headRy, only(g.hair)));
    return out;
  }

  const full = scale === "50";
  for (const { s, sign } of sides) {
    const foot = "foot-" + s;
    const toe = "toe-" + s;
    if (!spec.seated) out.push(capsule(`shoe-heel-${s}`, foot, "body", 1.2, toneShoes, o(foot) - spec.heel, sign * spec.legY, t(foot), sign * spec.legY, spec.shoeHeelR, spec.shoeBallR, { followTip: true, ...only(g.shoes) }));
    if (!spec.seated) out.push(capsule(`shoe-toe-${s}`, toe, "body", 1.2, toneShoes, o(toe), sign * spec.legY, t(toe) + 0.02, sign * spec.legY, spec.shoeBallR, spec.shoeBallR * 0.88, { followTip: true, ...only(g.shoes) }));
    if (full && detail && !spec.seated) out.push(line(`shoe-laces-${s}`, foot, "detail", 1.4, 0.6, [[o(foot) + 0.04, sign * spec.legY - 0.025], [o(foot) + 0.12, sign * spec.legY - 0.02]], {}));
    if (!spec.seated) {
      out.push(capsule(`shank-${s}`, "shank-" + s, "body", spec.legZ + 0.05, toneLegs, o("shank-" + s), sign * spec.legY, t("shank-" + s), sign * spec.legY, spec.shankR[0], spec.shankR[1], { followTip: true, ...only(g.legs) }));
    }
    out.push(capsule(`thigh-${s}`, "thigh-" + s, "body", spec.legZ, toneLegs, o("thigh-" + s), sign * spec.legY, t("thigh-" + s), sign * spec.legY, spec.thighR[0], spec.thighR[1], { followTip: true, ...only(g.legs) }));
    out.push(capsule(`upper-arm-${s}`, "arm-" + s, "body", spec.armZ, toneSleeve, o("arm-" + s), sign * spec.shoulderY, t("arm-" + s), sign * spec.shoulderY, spec.sleeveR, spec.forearmR + 0.004, { followTip: true, ...only(g.sleeve) }));
    out.push(capsule(`forearm-${s}`, "forearm-" + s, "body", spec.armZ + 0.05, toneSleeve, o("forearm-" + s), sign * spec.shoulderY, t("forearm-" + s), sign * spec.shoulderY, spec.forearmR, spec.forearmR * 0.78, { followTip: true, ...only(g.sleeve) }));
    out.push(capsule(`hand-${s}`, "hand-" + s, "body", spec.armZ + 0.1, toneHand, o("hand-" + s), sign * spec.shoulderY, t("hand-" + s) + 0.035, sign * spec.shoulderY, 0.026, 0.021, { followTip: true, ...only(g.hand) }));
  }

  if (spec.bag) {
    const b = spec.bag;
    out.push(rect("bag", "bag", "body", 2.5, toneOf(g.bag ?? "", 0.3), b.x, b.y, b.w, b.h, 0.02, only(g.bag ?? "")));
    if (full && detail) {
      out.push(line("bag-flap", "bag", "detail", 2.6, 0.5, [[b.x + b.w * 0.1, b.y + b.h * 0.5], [b.x + b.w * 0.9, b.y + b.h * 0.5]], {}));
      out.push(line("bag-strap", "torso", "detail", 4.6, 0.5, [[o("torso") - 0.01, -spec.shoulderY + 0.03], [o("torso") + 0.1, -0.03], [o("torso") + 0.07, 0.13], [o("torso"), b.y]], only(g.bag ?? "")));
    }
  }

  out.push(ellipse("waist", "pelvis", "body", 3.9, toneTorso, o("pelvis"), 0, spec.waistRx, spec.waistRy, only(g.torso)));
  out.push(capsule("trunk", "pelvis", "body", 3.95, toneTorso, o("pelvis"), 0, t("torso"), 0, spec.connectorR, spec.connectorR, { followTip: true, ...only(g.torso) }));
  out.push(ellipse("shoulders", "neck", "body", 4, toneTorso, o("neck"), 0, spec.shoulderRx, spec.shoulderRy, only(g.torso)));
  if (full && detail) {
    for (const { s, sign } of sides) out.push(line(`shoulder-seam-${s}`, "neck", "detail", 4.2, 0.55, [[o("neck") + 0.05, sign * 0.07], [o("neck") - 0.005, sign * (spec.shoulderRy - 0.035)]], {}));
  }
  if (spec.neckwear && g.neckwear) out.push(ellipse("neckwear", "neck", "body", 5, toneOf(g.neckwear, 0.45), o("neck") + 0.005, 0, spec.neckwear.rx, spec.neckwear.ry, only(g.neckwear)));

  out.push(ellipse("head", "head", "body", 6, toneHair, headX, 0, spec.headRx, spec.headRy, only(g.hair)));
  out.push(ellipse("face", "head", "body", 6.1, toneFace, headX + spec.headRx * 0.62, 0, spec.headRx * 0.36, spec.headRy * 0.72, only(g.face)));
  if (full && detail) {
    out.push(capsule("nose", "head", "detail", 6.3, toneFace, headX + spec.headRx * 0.93, 0, headX + spec.headRx * 1.12, 0, 0.011, 0.008, { ...only(g.face) }));
    for (const { s, sign } of sides) out.push(ellipse(`ear-${s}`, "head", "detail", 6.2, toneFace, headX - 0.005, sign * (spec.headRy + 0.003), 0.012, 0.007, { ...only(g.face) }));
    out.push(line("hair-parting", "head", "detail", 6.3, 0.5, [[headX - spec.headRx * 0.2, -spec.headRy * 0.45], [headX - spec.headRx * 0.55, spec.headRy * 0.1], [headX - spec.headRx * 0.75, spec.headRy * 0.35]], {}));
  }
  void poly;
  void pathShape;
  void toneHand;
  return out;
}

export function loadAsset(path: string): Asset {
  return JSON.parse(readFileSync(path, "utf8")).asset;
}
