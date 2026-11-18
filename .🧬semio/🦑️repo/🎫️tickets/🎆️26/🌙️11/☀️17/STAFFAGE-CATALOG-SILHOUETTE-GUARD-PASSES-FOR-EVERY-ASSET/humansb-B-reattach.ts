/** 🔗 Moves every flat shape flagged `lateral-ambiguous` by the validator (a flat shape on a bone that spreads or twists in a pose or clip) onto the nearest ancestor bone that never spreads, keeping its outline in the default pose (rest coordinates solved by damped Newton through the exact evaluator). Writes ops for humansb-B-apply.py.
 *  bun humansb-B-reattach.ts <asset> <out.json> [patch.json] */
import { readFileSync, writeFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const [name, out, patch] = process.argv.slice(2) as string[];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const find = (list: any[], id: string) => list.find((x) => x.id === id);
const pre: any[] = patch ? JSON.parse(readFileSync(patch, "utf8")) : [];
for (const o of pre) {
  if (o.op.startsWith("solid")) { Object.assign(find((asset as any).volumes[o.scale].body, o.id), o.set); continue; }
  const list: any[] = (asset as any).representations[o.scale].geometry[o.view];
  if (o.op === "shape-add") list.splice(list.indexOf(find(list, o.after)) + 1, 0, o.shape);
  else if (o.op === "shape-set") Object.assign(find(list, o.id), o.set);
  else list[list.indexOf(find(list, o.id))] = o.shape;
}
const a: any = asset;
const own = new Set<string>();
const scan = (offsets: any) => { for (const [id, off] of Object.entries(offsets ?? {}) as [string, any][]) if ((off.abductionDeg ?? 0) !== 0 || (off.twistDeg ?? 0) !== 0) own.add(id); };
for (const p of a.poses) scan(p.bones);
for (const c of a.clips) for (const k of c.keys) scan(k.bones);
const byId = new Map<string, any>(a.rig.bones.map((b: any) => [b.id, b]));
const spread = new Set<string>();
for (const b of a.rig.bones) for (let c: any = b; c !== undefined; c = c.parent === null ? undefined : byId.get(c.parent)) if (own.has(c.id)) { spread.add(b.id); break; }
const target = (bone: string): string | undefined => { for (let c: any = byId.get(bone); c !== undefined; c = c.parent === null ? undefined : byId.get(c.parent)) if (!spread.has(c.id)) return c.id; return undefined; };
const issues = validateAsset(asset).filter((i: any) => i.code === "lateral-ambiguous");
const flagged = new Map<string, Set<string>>();
for (const i of issues) for (const id of (i as any).data.shapes) { if (!flagged.has((i as any).data.bone)) flagged.set((i as any).data.bone, new Set()); flagged.get((i as any).data.bone)!.add(id); }
console.log(`[DEBUG] spread bones ${[...spread].join(" ")}; flagged ${[...flagged].map(([b, s]) => `${b}:${s.size}`).join(" ")}`);
const ops: any[] = [];
for (const scale of ["500", "200", "100", "50"]) {
  for (const view of ["front", "plan", "elevation"] as const) {
    const moves = new Map<string, string>();
    for (const s of a.representations[scale].geometry[view]) {
      const ids = flagged.get(s.bone);
      if (ids === undefined || !ids.has(s.id) || s.solid !== undefined || s.kind !== "poly" || s.role === "ground") continue;
      const t = target(s.bone);
      if (t !== undefined) moves.set(s.id, t);
    }
    if (moves.size === 0) continue;
    const evalA = (x: any) => evaluate(x, { kind: "pose", pose: x.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scale) } } as never) as any;
    const ringsOf = (x: any) => new Map<string, { x: number; y: number }[]>(evalA(x).shapes[view].map((r: any) => [r.shape.id, r.rings[0]]));
    const goal = ringsOf(a);
    const mk = (pts: Map<string, number[][]>): any => {
      const c = structuredClone(a);
      c.representations[scale].geometry[view] = c.representations[scale].geometry[view].map((s: any) => (moves.has(s.id) ? { ...s, bone: moves.get(s.id), followTip: undefined, points: pts.get(s.id) } : s));
      for (const s of c.representations[scale].geometry[view]) if (s.followTip === undefined) delete s.followTip;
      return c;
    };
    const boneProbe = (bone: string) => {
      const c = structuredClone(a);
      c.representations[scale].geometry[view].push({ kind: "poly", points: [[0, 0], [1, 0], [0, 1]], closed: true, id: "__probe", bone, role: "body", z: 9, filled: true, tone: 0.5 });
      const r = ringsOf(c).get("__probe")!;
      return { a: r[1]!.x - r[0]!.x, c: r[1]!.y - r[0]!.y, b: r[2]!.x - r[0]!.x, d: r[2]!.y - r[0]!.y, tx: r[0]!.x, ty: r[0]!.y };
    };
    let cur = new Map<string, number[][]>();
    for (const [id, bone] of moves) {
      const m = boneProbe(bone), det = m.a * m.d - m.b * m.c;
      cur.set(id, goal.get(id)!.map((p) => [(m.d * (p.x - m.tx) - m.b * (p.y - m.ty)) / det, (-m.c * (p.x - m.tx) + m.a * (p.y - m.ty)) / det]));
    }
    for (let it = 0; it < 14; it++) {
      const r0 = ringsOf(mk(cur));
      const shift = (dx: number, dy: number) => ringsOf(mk(new Map([...cur].map(([id, p]) => [id, p.map((q) => [q[0]! + dx, q[1]! + dy])]))));
      const rx = shift(1e-3, 0), ry = shift(0, 1e-3);
      let worst = 0;
      const next = new Map<string, number[][]>();
      for (const [id, pts] of cur) {
        next.set(id, pts.map((p, k) => {
          const g = goal.get(id)![k]!, c0 = r0.get(id)![k]!;
          const dx = g.x - c0.x, dy = g.y - c0.y;
          worst = Math.max(worst, Math.hypot(dx, dy));
          const aa = (rx.get(id)![k]!.x - c0.x) / 1e-3, cc = (rx.get(id)![k]!.y - c0.y) / 1e-3, bb = (ry.get(id)![k]!.x - c0.x) / 1e-3, dd = (ry.get(id)![k]!.y - c0.y) / 1e-3;
          const det = aa * dd - bb * cc;
          if (Math.abs(det) < 0.05) return p;
          const damp = it < 3 ? 0.5 : 0.7;
          return [p[0]! + (damp * (dd * dx - bb * dy)) / det, p[1]! + (damp * (-cc * dx + aa * dy)) / det];
        }));
      }
      cur = next;
      if (worst < 3e-4) break;
      if (it === 13) console.log(`[DEBUG] 1:${scale} ${view} residual ${(worst * 1000).toFixed(2)}mm`);
    }
    const r5 = (v: number) => Math.round(v * 1e5) / 1e5;
    for (const s of a.representations[scale].geometry[view]) if (moves.has(s.id)) {
      const ns = { ...s, bone: moves.get(s.id), points: cur.get(s.id)!.map((p) => [r5(p[0]!), r5(p[1]!)]) };
      delete ns.followTip;
      ops.push({ op: "shape", scale, view, id: s.id, shape: ns });
    }
    console.log(`[DEBUG] 1:${scale} ${view}: ${moves.size} shapes moved`);
  }
}
writeFileSync(out!, JSON.stringify(ops, null, 1));
