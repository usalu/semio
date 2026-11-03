import { readFileSync, writeFileSync } from "node:fs";
export const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
export const CAT = `${L}/🗂️catalog/🌳️vegetation`;
export type Aff = { sx: number; tx: number; sy: number; ty: number };
export const ID: Aff = { sx: 1, tx: 0, sy: 1, ty: 0 };
const n = (v: number) => Math.round(v * 10000) / 10000;

export function transformPath(d: string, a: Aff): string {
  const tokens = d.match(/[A-Za-z]|-?\d*\.?\d+(?:e-?\d+)?/g) ?? [];
  const out: string[] = []; let i = 0; let cmd = "";
  const num = () => Number(tokens[i++]);
  while (i < tokens.length) {
    if (/[A-Za-z]/.test(tokens[i]!)) cmd = tokens[i++]!;
    const rel = cmd === cmd.toLowerCase(); const C = cmd.toUpperCase();
    const X = (v: number) => n(rel ? v * a.sx : v * a.sx + a.tx);
    const Y = (v: number) => n(rel ? v * a.sy : v * a.sy + a.ty);
    if (C === "Z") { out.push(cmd); continue; }
    if (C === "M" || C === "L" || C === "T") { out.push(`${cmd}${X(num())} ${Y(num())}`); if (C === "M") cmd = rel ? "l" : "L"; continue; }
    if (C === "H") { out.push(`${cmd}${X(num())}`); continue; }
    if (C === "V") { out.push(`${cmd}${Y(num())}`); continue; }
    if (C === "C") { const p = [0, 1, 2].map(() => `${X(num())} ${Y(num())}`); out.push(`${cmd}${p.join(" ")}`); continue; }
    if (C === "Q" || C === "S") { const p = [0, 1].map(() => `${X(num())} ${Y(num())}`); out.push(`${cmd}${p.join(" ")}`); continue; }
    if (C === "A") { const rx = num(), ry = num(), rot = num(), la = num(), sw = num(); out.push(`${cmd}${n(rx * Math.abs(a.sx))} ${n(ry * Math.abs(a.sy))} ${rot} ${la} ${sw} ${X(num())} ${Y(num())}`); continue; }
    throw new Error(`path command ${cmd}`);
  }
  return out.join("");
}

export function transformShape(s: any, a: Aff): void {
  const X = (v: number) => n(v * a.sx + a.tx), Y = (v: number) => n(v * a.sy + a.ty);
  const rs = Math.sqrt(Math.abs(a.sx * a.sy));
  switch (s.kind) {
    case "path": s.d = transformPath(s.d, a); break;
    case "poly": s.points = s.points.map(([x, y]: number[]) => [X(x), Y(y)]); break;
    case "capsule": s.x0 = X(s.x0); s.x1 = X(s.x1); s.y0 = Y(s.y0); s.y1 = Y(s.y1); s.r0 = n(s.r0 * rs); s.r1 = n(s.r1 * rs); break;
    case "ellipse": s.cx = X(s.cx); s.cy = Y(s.cy); s.rx = n(s.rx * Math.abs(a.sx)); s.ry = n(s.ry * Math.abs(a.sy)); break;
    case "rect": { const x1 = X(s.x + s.w), y1 = Y(s.y + s.h), x0 = X(s.x), y0 = Y(s.y); s.x = Math.min(x0, x1); s.y = Math.min(y0, y1); s.w = n(Math.abs(x1 - x0)); s.h = n(Math.abs(y1 - y0)); break; }
    default: throw new Error(`shape ${s.kind}`);
  }
}

export function load(id: string): { doc: any; asset: any; indent: string; file: string } {
  const file = `${CAT}/${id}/🔣️.json`; const t = readFileSync(file, "utf8");
  const indent = t.split("\n")[1]!.match(/^ */)![0];
  const doc = JSON.parse(t);
  return { doc, asset: doc.asset, indent, file };
}
export function save(x: { doc: any; indent: string; file: string }): void { writeFileSync(x.file, JSON.stringify(x.doc, null, x.indent.length) + "\n"); }

export const evaluateMod = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
export const three = await import(`${L}/🧪️tests/🧊️three-views/🧰️.ts`);
export const stateAt = (asset: any, den: number, pose?: string) => evaluateMod.evaluate(structuredClone(asset), { kind: "pose", pose: pose ?? asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: den } });
export function boxes(st: any) {
  const out: any = { elevation: {}, front: {}, plan: {} };
  for (const v of ["elevation", "front", "plan"]) for (const s of st.shapes[v]) {
    if (s.shape.role === "ground") continue; const b = out[v][s.shape.bone] ??= [1e9, -1e9, 1e9, -1e9];
    for (const p of s.rings.flat()) { b[0] = Math.min(b[0], p.x); b[1] = Math.max(b[1], p.x); b[2] = Math.min(b[2], p.y); b[3] = Math.max(b[3], p.y); }
  }
  return out;
}
export const bands = (asset: any): number[] => three.authoredScales(asset);
