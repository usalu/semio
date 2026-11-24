/** 🎨️ Gives every outline or prism the repair drew in place of authored shapes the look of what it replaced: per group of ops (the plates or prisms of one bone in one view) each added shape takes tone, hatch, pen and drawing order `z` of the removed shape with its id, else of the removed shape of its material that it overlaps most in the default pose, else of the removed shape it overlaps most (a shape that an earlier group of the repair drew counts with the look it was given); the volume only gives the tone of a part that was not drawn before. Rewrites the shapes inside ops/<slug>.json (idempotent) and prints what changed.
 *  bun restyle.ts <backup dir> <slug>… */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { applyOps, readOps, writeOps } from "./ops.ts";
import { opsFile } from "./people.ts";

type Box = { x0: number; x1: number; y0: number; y1: number };
const [backupDir, ...slugs] = process.argv.slice(2) as string[];
const boxes = (asset: any, scale: string, view: string): Map<string, Box> => {
  const out = new Map<string, Box>();
  try {
    const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scale) } });
    for (const s of state.shapes[view] as any[]) { const p = s.rings.flat(); if (p.length > 0) out.set(s.shape.id, { x0: Math.min(...p.map((q: any) => q.x)), x1: Math.max(...p.map((q: any) => q.x)), y0: Math.min(...p.map((q: any) => q.y)), y1: Math.max(...p.map((q: any) => q.y)) }); }
  } catch { return out; }
  return out;
};
const overlap = (a: Box | undefined, b: Box | undefined): number => (a === undefined || b === undefined ? 0 : Math.max(0, Math.min(a.x1, b.x1) - Math.max(a.x0, b.x0)) * Math.max(0, Math.min(a.y1, b.y1) - Math.max(a.y0, b.y0)));
for (const slug of slugs) {
  const before = (await Bun.file(`${backupDir}/${slug}.json`).json()).asset;
  const ops = await readOps(opsFile(slug));
  const after: any = structuredClone(before);
  applyOps(after, ops);
  const groups = new Map<string, any[]>();
  for (const o of ops) if (o.scale !== undefined && typeof (o as any).group === "string" && /^(plate|prism)\|/u.test((o as any).group)) { const k = `${o.scale}|${(o as any).group}`; groups.set(k, [...(groups.get(k) ?? []), o]); }
  let changed = 0;
  const cache = new Map<string, [Map<string, Box>, Map<string, Box>]>();
  for (const [key, list] of groups) {
    const [scale, , view] = key.split("|") as [string, string, string];
    const removedIds = list.filter((o) => o.op === "shape-remove").map((o) => o.id as string);
    const authored = (before.representations[scale].geometry[view] as any[]).filter((s) => removedIds.includes(s.id));
    const earlier = ops.filter((x) => x.scale === scale && x.view === view && x.op === "shape-add" && removedIds.includes(x.shape.id) && !authored.some((s) => s.id === x.shape.id) && !list.includes(x)).map((x) => x.shape);
    const removed = [...authored, ...earlier];
    if (removed.length === 0) continue;
    if (!cache.has(`${scale}|${view}`)) cache.set(`${scale}|${view}`, [boxes(before, scale, view), boxes(after, scale, view)]);
    const [was, now] = cache.get(`${scale}|${view}`)!;
    for (const o of list.filter((x) => x.op === "shape-add")) {
      const id = o.shape.id as string;
      const best = (pool: any[]): any | undefined => pool.map((s) => ({ s, a: overlap(was.get(s.id), now.get(id)) })).sort((p, q) => q.a - p.a)[0]?.s;
      const match = removed.find((s) => s.id === id || s.id === id.replace(/(-p)+$/u, "")) ?? best(removed.filter((s) => s.material !== undefined && s.material === o.shape.material)) ?? best(removed);
      if (match === undefined) continue;
      const style: Record<string, unknown> = { tone: match.tone, z: match.z };
      for (const k of ["hatch", "penMm"]) if (match[k] !== undefined) style[k] = match[k];
      for (const target of ops.filter((x) => x.scale === scale && x.view === view && (x.op === "shape-add" || x.op === "shape") && x.shape?.id === id)) {
        const s = target.shape;
        if (Object.entries(style).every(([k, v]) => s[k] === v) && (match.hatch !== undefined || s.hatch === undefined)) continue;
        Object.assign(s, style);
        if (match.hatch === undefined) delete s.hatch;
        changed++;
      }
    }
  }
  await writeOps(opsFile(slug), ops);
  console.log(`${slug}: ${changed} shapes restyled in ${groups.size} groups`);
}
