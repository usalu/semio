/** 📐️ Author 68: 2D repair at the root from world-coordinate outlines, for any shape kind (poly, ellipse, capsule, ...): `repair(slug, ops)` turns every op into a poly in the shape's own local frame (affine fitted from the drawn ring, two passes for non-poly shapes), writes the text patch through `fauna-shape.py` and applies it to the asset.
 * @example repair("domestic-cat", [{ scale: 50, view: "plan", id: "head", world: [[0.34, 0], ...] }])
 * @see ./fauna-shape.py */
import { openCtx } from "./kit-lib.ts";
import { polyMap, type P } from "./fauna-affine.ts";
import { writeFileSync } from "node:fs";
import { globSync } from "node:fs";

export type Op = { scale: number; view: "elevation" | "front" | "plan"; id: string; world: P[]; set?: Record<string, unknown>; del?: string[]; clone?: string };
const KEEP = new Set(["id", "bone", "role", "z", "filled", "tone", "region", "regions", "layer", "kind", "closed", "points", "penMm"]);
const sh = (args: string[]): string => { const r = Bun.spawnSync(args); if (r.exitCode !== 0) throw new Error(`${args.join(" ")}: ${r.stderr.toString()}`); return r.stdout.toString(); };
const assetOf = (slug: string): string => (globSync(`/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/${slug}/🔣️.json`) as string[])[0]!;
const apply = (slug: string, patches: unknown[], tag: string): void => {
  const f = `🗑️generated/author-68/patch-${slug}-${tag}.json`;
  writeFileSync(f, JSON.stringify(patches));
  sh(["python3", "fauna-shape.py", assetOf(slug), f, "--apply"]);
};

export function repair(slug: string, ops: Op[]): void {
  const spec = `animals/${slug}`;
  const pass1: unknown[] = [];
  const later: Op[] = [];
  for (const op of ops) {
    const c = openCtx(spec, op.scale);
    const geo: any = (c.m as any).representations[String(op.scale)].geometry[op.view];
    const shape = geo.find((s: any) => s.id === (op.clone ?? op.id));
    if (shape === undefined) throw new Error(`no ${op.scale} ${op.view} ${op.id}`);
    if (shape.kind === "poly" && op.clone === undefined) {
      const m = polyMap(c, op.view, op.id, shape.points);
      if (m.err > 1e-4) throw new Error(`${op.scale} ${op.view} ${op.id}: affine error ${m.err}`);
      pass1.push({ scale: String(op.scale), view: op.view, id: op.id, set: { points: m.toLocal(op.world), ...(op.set ?? {}) }, ...(op.del ? { del: op.del } : {}) });
    } else {
      const del = Object.keys(shape).filter((k) => !KEEP.has(k));
      pass1.push({ ...(op.clone ? { clone: op.clone } : {}), scale: String(op.scale), view: op.view, id: op.id, set: { kind: "poly", closed: true, points: op.world, ...(op.set ?? {}) }, del: [...del, ...(op.del ?? [])] });
      later.push(op);
    }
  }
  apply(slug, pass1, "a");
  if (later.length > 0) {
    const pass2: unknown[] = [];
    for (const op of later) {
      const c = openCtx(spec, op.scale);
      const m = polyMap(c, op.view, op.id, op.world);
      if (m.err > 1e-4) throw new Error(`${op.scale} ${op.view} ${op.id}: affine error ${m.err} (pass 2)`);
      pass2.push({ scale: String(op.scale), view: op.view, id: op.id, set: { points: m.toLocal(op.world) } });
    }
    apply(slug, pass2, "b");
  }
}
