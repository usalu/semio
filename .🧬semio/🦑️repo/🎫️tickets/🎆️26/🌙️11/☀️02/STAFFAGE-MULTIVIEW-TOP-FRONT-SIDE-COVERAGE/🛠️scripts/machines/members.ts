import { readFileSync, writeFileSync } from "node:fs";
import { fileOf } from "./lib.ts";
import { catalog, sweep, consistency, authoredScales, extentOf } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { validateAsset } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const r3 = (n: number) => Math.round(n * 1000) / 1000;
const REMOVE = new Set(["tower-head-legs", "tower-head-leg-l", "tower-head-leg-r", "tower-head-diagonals", "tower-head-frame", "tower-head-cross-1", "tower-head-cross-2", "slings", "sling-l", "sling-r"]);
const BASE_Y = 1.2, TOP_Y = 8.6, BASE_X = 1.2, BASE_Z = 0.9, TOP_X = 0, TOP_Z = 0.12;
const face = (y: number): { x: number; z: number } => { const t = (y - BASE_Y) / (TOP_Y - BASE_Y); return { x: BASE_X + (TOP_X - BASE_X) * t, z: BASE_Z + (TOP_Z - BASE_Z) * t }; };
type P = [number, number, number];
const seg = (a: P, b: P) => ({ from: { x: r3(a[0]), y: r3(a[1]), z: r3(a[2]) }, to: { x: r3(b[0]), y: r3(b[1]), z: r3(b[2]) } });

const legs = (): ReturnType<typeof seg>[] => [[1, 1], [1, -1], [-1, 1], [-1, -1]].map(([sx, sz]) => seg([sx * BASE_X, BASE_Y, sz * BASE_Z], [sx * TOP_X, TOP_Y, sz * TOP_Z]));
const diagonals = (): ReturnType<typeof seg>[] => {
  const ys = [1.2, 2.6, 4.0, 5.4, 6.8];
  const out: ReturnType<typeof seg>[] = [];
  for (let k = 0; k < ys.length - 1; k++) {
    const a = face(ys[k]), b = face(ys[k + 1]), s = k % 2 === 0 ? 1 : -1;
    for (const f of [1, -1]) {
      out.push(seg([f * a.x, ys[k], -s * a.z], [f * b.x, ys[k + 1], s * b.z]));
      out.push(seg([-s * a.x, ys[k], f * a.z], [s * b.x, ys[k + 1], f * b.z]));
    }
  }
  return out;
};
const baseFrame = (): ReturnType<typeof seg>[] => [
  seg([-BASE_X, BASE_Y, -BASE_Z], [BASE_X, BASE_Y, -BASE_Z]), seg([BASE_X, BASE_Y, -BASE_Z], [BASE_X, BASE_Y, BASE_Z]),
  seg([BASE_X, BASE_Y, BASE_Z], [-BASE_X, BASE_Y, BASE_Z]), seg([-BASE_X, BASE_Y, BASE_Z], [-BASE_X, BASE_Y, -BASE_Z]),
];
const baseCross = (): ReturnType<typeof seg>[] => [seg([-BASE_X, BASE_Y, -BASE_Z], [BASE_X, BASE_Y, BASE_Z]), seg([BASE_X, BASE_Y, -BASE_Z], [-BASE_X, BASE_Y, BASE_Z])];
const slings = (): ReturnType<typeof seg>[] => [[1, 1], [1, -1], [-1, 1], [-1, -1]].map(([sx, sz]) => seg([0, 1, sz * 0.12], [sx * 0.7, 0, sz * 0.7]));

export function apply(asset: any): string[] {
  const log: string[] = [];
  for (const rep of Object.values<any>(asset.representations)) {
    if (rep.status !== "authored") continue;
    const all = ["elevation", "front", "plan"].flatMap((v) => rep.geometry[v] as any[]);
    const tpl = (...ids: string[]) => all.find((s) => ids.includes(s.id));
    const meta = (t: any, bone: string, id: string) => ({ id, bone, role: t.role, z: t.z, filled: false, tone: t.tone, ...(t.region ? { region: t.region } : {}), ...(t.regions ? { regions: t.regions } : {}), material: t.material, ...(t.penMm ? { penMm: t.penMm } : {}) });
    const specs: { tpl: any; bone: string; prefix: string; list: ReturnType<typeof seg>[]; d: number }[] = [];
    const headLeg = tpl("tower-head-legs", "tower-head-leg-l"), headDiag = tpl("tower-head-diagonals"), headFrame = tpl("tower-head-frame"), headCross = tpl("tower-head-cross-1"), sl = tpl("slings", "sling-l");
    if (headLeg) specs.push({ tpl: headLeg, bone: "slewing", prefix: "tower-head-leg", list: legs(), d: 0.1 });
    if (headDiag || headLeg) specs.push({ tpl: headDiag ?? headLeg, bone: "slewing", prefix: "tower-head-diagonal", list: diagonals(), d: 0.06 });
    if (headFrame) specs.push({ tpl: headFrame, bone: "slewing", prefix: "tower-head-frame", list: baseFrame(), d: 0.08 });
    if (headCross) specs.push({ tpl: headCross, bone: "slewing", prefix: "tower-head-cross", list: baseCross(), d: 0.06 });
    if (sl) specs.push({ tpl: sl, bone: "skip", prefix: "sling", list: slings(), d: 0.02 });
    for (const view of ["elevation", "front", "plan"]) {
      rep.geometry[view] = rep.geometry[view].filter((s: any) => !REMOVE.has(s.id));
      for (const sp of specs) sp.list.forEach((m, i) => rep.geometry[view].push({ kind: "member", ...m, section: { kind: "circle", d: sp.d }, ...meta(sp.tpl, sp.bone, `${sp.prefix}-${i + 1}`) }));
    }
    log.push(`1:${rep.denominator}: ${specs.map((s) => `${s.prefix}x${s.list.length}`).join(" ")}`);
  }
  return log;
}

if (import.meta.main) {
  const write = process.argv.includes("--write");
  const file = fileOf("tower-crane");
  const doc = JSON.parse(readFileSync(file, "utf8"));
  const before = catalog().find((a) => a.id === "tower-crane")!;
  console.log(apply(doc.asset).join("\n"));
  const issues = validateAsset(doc.asset, { assets: new Set(catalog().map((a) => a.id)) });
  console.log("issues", issues.length); for (const i of issues.slice(0, 8)) console.log("  ", i.code, i.path, i.message.en.slice(0, 200));
  const f: string[] = [];
  for (const s of authoredScales(doc.asset)) for (const x of [...sweep(doc.asset, s), ...consistency(doc.asset, s)]) f.push(`[${x.check}] ${x.message}`);
  console.log("guard", f.length); for (const x of f.slice(0, 6)) console.log("  ", x.slice(0, 220));
  const ext = (a: any, s: number, v: string) => { const st: any = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: s } } as any); const e = extentOf(st.shapes[v]); return e ? [e.minX, e.maxX, e.minY, e.maxY].map((n) => +n.toFixed(2)) : null; };
  for (const s of authoredScales(doc.asset)) for (const v of ["elevation", "front", "plan"]) { const a = ext(before, s, v), b = ext(doc.asset, s, v); if (JSON.stringify(a) !== JSON.stringify(b)) console.log(`  extent 1:${s} ${v}: ${JSON.stringify(a)} -> ${JSON.stringify(b)}`); }
  if (write) { writeFileSync(file, JSON.stringify(doc, null, 1) + "\n"); console.log("written"); }
}
