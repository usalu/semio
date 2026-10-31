/** [DEBUG] Scale operating heat so steady surfaces stay realistic, and shrink the japanese-maple 1:500 drawing onto its dimensions. */
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveThermal, solveThermalAt } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";

const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";

function locate(id: string): string {
  for (const cat of readdirSync(root)) {
    try {
      const doc = readdirSync(`${root}/${cat}/${id}`).find((f) => f.endsWith(".json"));
      if (doc) return `${root}/${cat}/${id}/${doc}`;
    } catch { /* next category */ }
  }
  throw new Error(id);
}

function limitOf(id: string): number | null {
  if (/brake|resistor/.test(id)) return 340;
  if (/bms|hydraulic|drum-drive|controller/.test(id)) return 80;
  if (/engine|exhaust|silencer|e-axle|traction|hub-motor|compressor/.test(id)) return 240;
  return null;
}

function queries(asset: any): any[] {
  const out: any[] = asset.poses.filter((p: any) => !p.key).map((p: any) => ({ kind: "pose", pose: p.id }));
  for (const clip of asset.clips) out.push({ kind: "clip", clip: clip.id, timeS: clip.durationS * 0.37 }, { kind: "clip", clip: clip.id, timeS: 0 });
  return out;
}

function temps(asset: any, regionId: string): number[] {
  const found: number[] = [];
  for (const query of queries(asset)) {
    const field = query.kind === "pose" ? solveThermal(evaluate(asset, query)) : solveThermalAt(asset, query);
    const region = field.regions[regionId];
    if (region) found.push(region.surfaceTempC);
  }
  return found;
}

const HOT = ["city-car", "city-bus", "city-bicycle", "forklift", "delivery-van", "motor-scooter", "electric-kick-scooter", "tram", "wheel-loader", "concrete-mixer-truck", "excavator", "diesel-generator", "retractable-bollard", "heat-pump-outdoor-unit"];

for (const id of HOT) {
  const path = locate(id);
  const doc = JSON.parse(readFileSync(path, "utf8"));
  const asset = doc.asset;
  let changed = false;
  for (const region of asset.thermal.regions) {
    const op = region.operating;
    if (!op || typeof op.heatW !== "number") continue;
    if (region.id === "coil-guard" || region.id === "base-tray") continue;
    const limit = limitOf(region.id);
    if (limit === null) continue;
    const before = Math.max(...temps(asset, region.id));
    if (before <= limit) {
      console.log(`[DEBUG] ${id} ${region.id} ${before.toFixed(1)} C ok heat ${op.heatW}`);
      continue;
    }
    let lo = 0;
    let hi = op.heatW;
    for (let i = 0; i < 8; i++) {
      op.heatW = (lo + hi) / 2;
      const t = Math.max(...temps(asset, region.id));
      if (t > limit) hi = op.heatW;
      else lo = op.heatW;
    }
    op.heatW = Math.round(lo * 10) / 10;
    const after = Math.max(...temps(asset, region.id));
    console.log(`[DEBUG] ${id} ${region.id} ${before.toFixed(1)} -> ${after.toFixed(1)} C heat ${op.heatW}`);
    changed = true;
  }
  if (id === "heat-pump-outdoor-unit") {
    const tray = asset.thermal.regions.find((r: any) => r.id === "base-tray");
    let lo = 0;
    let hi = tray.operating.heatW;
    for (let i = 0; i < 8; i++) {
      tray.operating.heatW = (lo + hi) / 2;
      const t = Math.max(...temps(asset, "base-tray"));
      if (t > 36) hi = tray.operating.heatW;
      else lo = tray.operating.heatW;
    }
    tray.operating.heatW = Math.round(lo * 10) / 10;
    const coil = Math.min(...temps(asset, "coil-guard"));
    const trayT = Math.max(...temps(asset, "base-tray"));
    console.log(`[DEBUG] heat-pump coil min ${coil.toFixed(1)} tray max ${trayT.toFixed(1)} heat ${tray.operating.heatW}`);
    changed = true;
  }
  if (changed) writeFileSync(path, JSON.stringify(doc, null, 1) + "\n");
}

function bounds(asset: any, denominator: number): { w: number; h: number } {
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator } });
  return { w: state.bounds.maxX - state.bounds.minX, h: state.bounds.maxY - state.bounds.minY };
}

function scaleNums(shape: any, s: number): void {
  for (const key of ["x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "x", "y", "w", "h", "r"]) if (typeof shape[key] === "number") shape[key] *= s;
  if (shape.kind === "poly") shape.points = shape.points.map(([x, y]: number[]) => [x * s, y * s]);
  if (shape.kind === "path") {
    const tok = shape.d.match(/[A-Za-z]|-?\d*\.?\d+(?:e-?\d+)?/gu)!;
    const out: string[] = [];
    let i = 0;
    let cmd = "";
    const num = (): number => Number(tok[i++]);
    const put = (v: number): void => void out.push(String(Math.round(v * 1e5) / 1e5));
    while (i < tok.length) {
      if (/[A-Za-z]/u.test(tok[i]!)) {
        cmd = tok[i++]!;
        out.push(cmd);
        if (cmd === "Z" || cmd === "z") continue;
      }
      const c = cmd.toUpperCase();
      if (c === "H" || c === "V") put(num() * s);
      else if (c === "A") { put(num() * s); put(num() * s); put(num()); put(num()); put(num()); put(num() * s); put(num() * s); }
      else if (c === "C") { for (let k = 0; k < 6; k++) put(num() * s); }
      else if (c === "Q") { for (let k = 0; k < 4; k++) put(num() * s); }
      else if (c === "S") { for (let k = 0; k < 4; k++) put(num() * s); }
      else { put(num() * s); put(num() * s); }
    }
    shape.d = out.join(" ");
  }
}

const maplePath = locate("japanese-maple");
const maple = JSON.parse(readFileSync(maplePath, "utf8"));
const targetW = maple.asset.dimensions.widthM;
const targetH = maple.asset.dimensions.heightM;
let measured = bounds(maple.asset, 500);
console.log(`[DEBUG] maple before ${measured.w.toFixed(3)} x ${measured.h.toFixed(3)}`);
const factor = Math.min(targetW / measured.w, targetH / measured.h) * 0.995;
for (const shape of maple.asset.representations["500"].geometry.elevation) scaleNums(shape, factor);
for (const shape of maple.asset.representations["500"].geometry.plan ?? []) scaleNums(shape, factor);
measured = bounds(maple.asset, 500);
console.log(`[DEBUG] maple after ${measured.w.toFixed(3)} x ${measured.h.toFixed(3)} factor ${factor.toFixed(4)}`);
writeFileSync(maplePath, JSON.stringify(maple, null, 1) + "\n");
