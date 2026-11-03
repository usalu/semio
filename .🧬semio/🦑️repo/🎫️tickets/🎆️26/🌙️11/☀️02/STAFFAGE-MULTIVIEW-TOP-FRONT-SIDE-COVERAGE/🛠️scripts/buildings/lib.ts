import { readFileSync, writeFileSync } from "node:fs";
import { path } from "./sy.ts";
export type Doc = { asset: any; [k: string]: any };
export const load = (id: string): Doc => JSON.parse(readFileSync(path(id), "utf8"));
export const save = (id: string, doc: Doc): void => writeFileSync(path(id), JSON.stringify(doc, null, 2) + "\n");
export const clone = <T>(v: T): T => JSON.parse(JSON.stringify(v));
export const shapes = (a: any, scale: number, view: string): any[] => clone(a.representations[String(scale)].geometry[view]);
export const text = (en: string, de: string) => ({ en, de });
export function sign(list: any[], opts: { drop?: RegExp; unfill?: RegExp } = {}): any[] {
  return list.filter((s) => s.role !== "ground" && s.role !== "texture" && !(opts.drop && opts.drop.test(s.id))).map((s) => {
    const o = { ...s };
    if (opts.unfill && opts.unfill.test(s.id)) o.filled = false;
    delete o.solid;
    return o;
  });
}
export const poly = (id: string, bone: string, role: string, points: number[][], closed = false, extra: object = {}) => ({ id, bone, role, z: 1, filled: false, tone: 0.5, kind: "poly", points, closed, ...extra });
export const rect = (id: string, bone: string, role: string, x: number, y: number, w: number, h: number, filled = false, extra: object = {}) => ({ id, bone, role, z: 1, filled, tone: 0.5, kind: "rect", x, y, w, h, ...extra });
const R = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${R}/🔨️modules/🧮️evaluation/🟦️.ts`);
export function worldOf(a: any, view: string, list: any[]): any[] {
  const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: 200 } });
  const root = a.rig.root;
  return list.map((s) => {
    if (view !== "elevation") return { ...s, bone: root };
    const [m0, m1, m2, m3, m4, m5] = st.bones[s.bone].world as number[];
    const P = (x: number, y: number): number[] => [Math.round((m0 * x + m2 * y + m4) * 1e6) / 1e6, Math.round((m1 * x + m3 * y + m5) * 1e6) / 1e6];
    const { bone, ...rest } = s;
    if (s.kind === "poly") return { ...rest, bone: root, points: s.points.map((p: number[]) => P(p[0], p[1])) };
    if (s.kind === "rect") return { ...rest, bone: root, kind: "poly", closed: true, points: [P(s.x, s.y), P(s.x + s.w, s.y), P(s.x + s.w, s.y + s.h), P(s.x, s.y + s.h)], x: undefined, y: undefined, w: undefined, h: undefined };
    if (s.kind === "ellipse") {
      const c = P(s.cx, s.cy);
      const swap = Math.abs(m0) < 1e-9;
      return { ...rest, bone: root, cx: c[0], cy: c[1], rx: swap ? s.ry : s.rx, ry: swap ? s.rx : s.ry };
    }
    throw new Error(`shape kind ${s.kind} of ${s.id}`);
  }).map((s) => JSON.parse(JSON.stringify(s)));
}
