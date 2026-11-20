/** ✏️ Ruling E1 for `humans/parent-with-stroller`: redraws the three viewpoints of every scale as the orthographic views of that scale's body in the default pose. Each drawn shape is one stated group of solids of one bone (the group table below is the drawing's line hierarchy: which parts read as one outline, their role, tone and region); its outline is the union of the projected solids, simplified to a quarter of the scale's smallest feature and carried into the frame of the 2D shape by the product's own placement (`inverse` of `r3-project2d.ts`). Slender tube runs become open strokes along their centre line; hand-stated seam, hem and sole strokes are added from the same stations the solids use.
 * Output: `r3-stroller-2d.json` (`representations.<scale>.geometry` and `.decisions`), read by `r3-stroller-roots.py`.
 * @see ./round3-brief.md */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { area, entryOf, featureM, loadSource, merged, poseState, projectWorld, scaleMeshes, simplifyRing, type Json, type P2, type View } from "./volumes-lib.ts";
import { contours, inverse, projectMask } from "./r3-project2d.ts";
import { REP_DECISIONS, STROKES } from "./r3-stroller-strokes.ts";

type G = { id: string; role?: string; hatch?: string };
const side = (id: string): string => (/-l(-|$)/.test(id) ? "-l" : /-r(-|$)/.test(id) ? "-r" : "");
/** 🗂️ The drawn group of a body solid: shape id, role and hatch; `null` leaves the solid undrawn. */
function groupOf(s: Json): G | null {
  const id: string = s.id, b: string = s.bone, sd = side(id);
  if (b === "head") return /^eye-/.test(id) ? { id, role: "detail" } : id === "lips" ? { id, role: "detail" } : /^hair/.test(id) ? { id: "hair" } : id === "scarf" ? { id: "scarf" } : { id: "head" };
  if (b === "torso") return id === "jacket-collar" ? { id } : { id: "torso", hatch: "knit" };
  if (b === "pelvis") return s.tone > 0.35 ? { id: "jacket-skirt", hatch: "knit" } : { id: "pelvis", hatch: "weave" };
  if (/^upper-arm/.test(b)) return { id: b, hatch: "knit" };
  if (/^forearm/.test(b)) return /^cuff/.test(id) ? { id } : { id: b, hatch: "knit" };
  if (/^hand/.test(b)) return { id: b };
  if (/^(thigh|shank)/.test(b)) return { id: b, hatch: "weave" };
  if (/^foot/.test(b)) return /^sole/.test(id) ? { id } : { id: `shoe${sd}` };
  if (/^toe/.test(b)) return /^sole/.test(id) ? { id } : { id: `shoe-toe${sd}` };
  if (b === "child") return /eye/.test(id) ? { id, role: "detail" } : /boot/.test(id) ? { id: "child-boots" } : /hand/.test(id) ? { id: "child-hands" } : /head|face|nose/.test(id) ? { id: "child-head" } : /hat|pompom/.test(id) ? { id: "child-hat" } : { id: "child-body" };
  if (b === "stroller-hood") return { id: "hood" };
  if (/^wheel/.test(b)) return /^(tyre|wheel)/.test(id) ? { id: `tyre-${b.slice(6)}${sd || "-l"}` } : { id: `wheel-${b.slice(6)}${sd || "-l"}` };
  if (id === "handle-grip" || id === "footrest" || id === "bumper-bar") return { id };
  if (/^(seat-sling|seat-wing|seat$)/.test(id)) return { id: "seat-fabric" };
  if (/^basket/.test(id)) return { id: "basket" };
  if (id === "canopy-rear") return { id: "hood-rear" };
  if (/^(joint-hub|swivel|fork|caster|canopy-hinge|handle-clamp|seat-mount)/.test(id)) return { id: `fittings${sd}` };
  return { id: sd === "" ? "chassis-cross" : `chassis${sd}` };
}
const DEPTH: Record<View, (p: number[]) => number> = { elevation: (p) => -p[2]!, front: (p) => p[0]!, plan: (p) => p[1]! };

const { entry, asset } = entryOf("humans/parent-with-stroller");
const m: Json = merged(asset, loadSource(entry));
const out: Json = { representations: {} };
for (const scale of [50, 100, 200, 500]) {
  const feature = featureM(scale, m), cell = feature / 8, tol = feature / 4;
  const vol = m.volumes[String(scale)], sm = scaleMeshes(m, scale, vol);
  const region = new Map<string, string>(vol.thermal.filter((t: Json) => t.of).map((t: Json) => [t.of, t.region]));
  const regionOf = (s: Json): string => region.get(s.id) ?? (s.material === "skin" ? (s.bone === "child" ? "child-skin" : "head-skin") : "torso-clothing");
  const frames = poseState(m, scale).frames;
  let work: Json = m;
  const geometry: Json = {};
  for (const view of ["elevation", "front", "plan"] as View[]) {
    const groups = new Map<string, { g: G; first: Json; meshes: any[]; depth: number; n: number; order: number }>();
    const lines: { meta: Json; target: P2[]; depth: number }[] = [];
    sm.solids.body.forEach((s: Json, i: number) => {
      const mesh = sm.solidMeshes.body[i];
      if (mesh === undefined || s.texture === true) return;
      let d = 0;
      for (let k = 0; k < mesh.world.length; k += 3) d += DEPTH[view]([mesh.world[k]!, mesh.world[k + 1]!, mesh.world[k + 2]!]);
      d /= mesh.world.length / 3;
      if (s.slender === true) {
        const f = frames[s.bone]!;
        const pts: P2[] = s.sections.map((q: Json) => projectWorld(view, [0, 1, 2].map((k) => f.o[k]! + q.at[0] * f.u[k]! + q.at[1] * f.v[k]! + q.at[2] * f.w[k]!) as [number, number, number]));
        lines.push({ meta: { id: s.id === "push-frame" ? "chassis" : s.id, bone: s.bone, role: "body", z: 0, filled: false, tone: 0.3, region: regionOf(s), material: s.material }, target: pts, depth: d });
        return;
      }
      const g = groupOf(s);
      if (g === null || (view === "plan" && g.role === "detail")) return;
      const key = `${s.bone}|${g.id}`;
      const e = groups.get(key) ?? groups.set(key, { g, first: s, meshes: [], depth: 0, n: 0, order: groups.size }).get(key)!;
      e.meshes.push(mesh); e.depth += d; e.n++;
    });
    const jobs: { meta: Json; target: P2[]; key: string }[] = [];
    const ranked = [...groups.values()].map((e) => ({ ...e, depth: e.depth / e.n + e.order * 1e-5 })).sort((a, b) => a.depth - b.depth);
    ranked.forEach((e, rank) => {
      const mask = projectMask(view, e.meshes, cell, 4);
      if (mask === null) return;
      const rings = contours(mask.cells, mask.width, mask.height, mask.box, mask.cell).map((r) => simplifyRing(r, tol)).filter((r) => r.length >= 3 && Math.abs(area(r)) >= feature * feature * 0.5);
      const hatch = e.g.hatch !== undefined && (m.representations[String(scale)].hatches as Json[]).some((h) => h.id === e.g.hatch) ? { hatch: e.g.hatch } : {};
      const meta = { id: e.g.id, bone: e.first.bone, role: e.g.role ?? "body", z: Math.round((1 + rank * 0.1) * 100) / 100, filled: true, tone: e.first.tone, region: regionOf(e.first), material: e.first.material, ...hatch };
      for (const r of rings) jobs.push({ meta, target: r, key: `${e.first.bone}|${e.g.id}` });
    });
    const REV = /^(upper-arm|forearm|hand|thigh|shank)-[lr]$/, EXT = scale === 500 ? /^wheel-/ : /^(wheel-|foot-|toe-)/;
    const special = (key: string): boolean => view !== "elevation" && (REV.test(key.split("|")[0]!) || EXT.test(key.split("|")[0]!));
    const flat = jobs.filter((j) => !special(j.key));
    const solved = inverse(work, scale, view, flat.map((j) => ({ meta: j.meta, target: j.target })));
    const worst = Math.max(0, ...solved.map((x) => x.residualM));
    if (process.argv.includes("--debug")) solved.forEach((x, i) => { if (x.residualM > 0.001) console.log("[DEBUG] ", view, flat[i]!.key, (x.residualM * 1000).toFixed(1), x.edgeOn); });
    const shapes: Json[] = [];
    const byKey = new Map<string, { meta: Json; rings: P2[][] }>();
    flat.forEach((j, i) => { const e = byKey.get(j.key) ?? byKey.set(j.key, { meta: j.meta, rings: [] }).get(j.key)!; e.rings.push(solved[i]!.points); });
    const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
    for (const e of byKey.values()) {
      if (e.rings.length === 1) shapes.push({ ...e.meta, kind: "poly", points: e.rings[0]!.map((p) => [r4(p.x), r4(p.y)]), closed: true });
      else shapes.push({ ...e.meta, kind: "path", d: e.rings.map((r) => r.map((p, i) => `${i === 0 ? "M" : "L"} ${r4(p.x)} ${r4(p.y)}`).join(" ") + " Z").join(" ") });
    }
    if (view !== "elevation") for (const e of ranked) {
      const bone: string = e.first.bone, f = frames[bone]!, meta = jobs.find((j) => j.key === `${bone}|${e.g.id}`)?.meta;
      if (meta === undefined || !(REV.test(bone) || EXT.test(bone))) continue;
      const local: number[][] = [], edges: [number, number][] = [];
      for (const mesh of e.meshes) {
        const base = local.length;
        for (let k = 0; k < mesh.world.length; k += 3) { const d = [mesh.world[k]! - f.o[0]!, mesh.world[k + 1]! - f.o[1]!, mesh.world[k + 2]! - f.o[2]!]; local.push([d[0]! * f.u[0]! + d[1]! * f.u[1]! + d[2]! * f.u[2]!, d[0]! * f.v[0]! + d[1]! * f.v[1]! + d[2]! * f.v[2]!, d[0]! * f.w[0]! + d[1]! * f.w[1]! + d[2]! * f.w[2]!]); }
        for (let t = 0; t < mesh.indices.length; t += 3) for (const [i, j] of [[0, 1], [1, 2], [2, 0]] as const) edges.push([base + mesh.indices[t + i]!, base + mesh.indices[t + j]!]);
      }
      if (REV.test(bone)) {
        const lo = Math.min(...local.map((p) => p[0]!)), hi = Math.max(...local.map((p) => p[0]!)), n = Math.max(4, Math.round((hi - lo) / (feature / 2))), rad = new Array(n + 1).fill(0);
        for (const [i, j] of edges) { const a = local[i]!, c = local[j]!, steps = Math.max(1, Math.ceil((Math.abs(c[0]! - a[0]!) / (hi - lo)) * n * 2)); for (let q = 0; q <= steps; q++) { const k = q / steps, t = Math.round((((a[0]! + (c[0]! - a[0]!) * k) - lo) / (hi - lo)) * n); rad[t] = Math.max(rad[t], Math.abs(a[2]! + (c[2]! - a[2]!) * k)); } }
        const ring: P2[] = [{ x: lo, y: 0 }, ...rad.map((r, i) => ({ x: lo + ((hi - lo) * i) / n, y: r })), { x: hi, y: 0 }];
        const pts = simplifyRing(ring, tol / 2);
        shapes.push({ ...meta, kind: "poly", points: pts.map((p) => [r4(p.x), r4(Math.max(0, p.y))]), closed: true, solid: { kind: "revolved", axisBone: bone, axis: "u", axisX: 0, mirror: true } });
      } else {
        const twin = (work.representations[String(scale)].geometry.elevation as Json[]).find((x) => x.id === e.g.id && x.bone === bone)!;
        const outer = twin.kind === "poly" ? twin.points : (twin.d as string).split("Z")[0]!.trim().split(/[ML]/).filter((t: string) => t.trim() !== "").map((t: string) => t.trim().split(/\s+/).map(Number));
        const zs = e.meshes.flatMap((mesh: any) => { const o: number[] = []; for (let k = 2; k < mesh.world.length; k += 3) o.push(mesh.world[k]!); return o; });
        shapes.push({ ...meta, kind: "poly", points: outer, closed: true, solid: { kind: "extruded", sweep: "w", z0: r4(Math.min(...zs)), z1: r4(Math.max(...zs)) } });
      }
    }
    if (lines.length > 0) {
      const sl = inverse(work, scale, view, lines.map((l) => ({ meta: l.meta, target: l.target })));
      lines.forEach((l, i) => shapes.push({ ...l.meta, z: 0.5 + i * 0.01, kind: "poly", points: sl[i]!.points.map((p) => [r4(p.x), r4(p.y)]), closed: false }));
    }
    const mine = STROKES[String(scale)]!.filter((k) => k.views.includes(view) && shapes.some((x) => x.id === k.parent && x.bone === k.bone));
    if (mine.length > 0) {
      const metas = mine.map((k) => ({ id: k.id, bone: k.bone, role: k.role, z: Math.round((shapes.find((x) => x.id === k.parent && x.bone === k.bone)!.z + 0.005) * 1000) / 1000, filled: false, tone: k.tone, region: k.region, material: k.material }));
      const sv = inverse(work, scale, view, mine.map((k, i) => ({ meta: metas[i]!, target: k.pts.map((q) => { const f = frames[k.bone]!; return projectWorld(view, [0, 1, 2].map((c) => f.o[c]! + q[0] * f.u[c]! + q[1] * f.v[c]! + q[2] * f.w[c]!) as [number, number, number]); }) })));
      mine.forEach((_, i) => shapes.push({ ...metas[i]!, kind: "poly", points: sv[i]!.points.map((p) => [r4(p.x), r4(p.y)]), closed: false }));
    }
    if (view === "elevation") for (const x of shapes) {
      if (!/-r$/.test(x.bone) || x.filled !== true) continue;
      const twin = shapes.find((y) => y.bone === x.bone.replace(/-r$/, "-l") && y.id === x.id.replace(/-r(-|$)/, "-l$1") && y.filled === true);
      if (twin === undefined) continue;
      delete x.points; delete x.d; delete x.closed;
      Object.assign(x, twin.kind === "poly" ? { kind: "poly", points: twin.points, closed: true } : { kind: "path", d: twin.d });
    }
    geometry[view] = shapes;
    console.log(`1:${scale} ${view}: ${shapes.length} shapes, ${shapes.reduce((n: number, s: Json) => n + (s.points?.length ?? (s.d?.match(/[ML]/g)?.length ?? 0)), 0)} points, residual ${(worst * 1000).toFixed(3)} mm`);
    if (view === "elevation") work = { ...m, representations: { ...m.representations, [String(scale)]: { ...m.representations[String(scale)], geometry: { ...m.representations[String(scale)].geometry, elevation: shapes } } } };
  }
  out.representations[String(scale)] = { geometry, decisions: REP_DECISIONS[String(scale)] };
}
out.areas = { "stroller-frame": [0.5, 0.73] };
writeFileSync(join(import.meta.dir, "r3-stroller-2d.json"), JSON.stringify(out));
