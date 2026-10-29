// [DEBUG] humans-a coordinate converter: hand-authored rest-world design (f = forward m, u = up m, ground u = 0) -> bone-local catalog JSON.
// usage: node build.mjs <asset-id>... ; writes the CategoryDocument of all existing assets in ORDER to OUT.
import { writeFileSync, existsSync } from "node:fs";
import { pathToFileURL } from "node:url";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
const here = dirname(fileURLToPath(import.meta.url));
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const OUT = process.env.OUT ?? `${S}/🗂️catalog/👤️humans/🔣️.json`;
const ORDER = ["walking-commuter", "seated-reader", "running-child", "wheelchair-user", "elderly-with-cane", "cyclist"];
const KEY = { "walking-commuter": ["loading-left", "passing-left", "contact-right", "loading-right"], "seated-reader": ["leaning-forward", "rising"], "running-child": ["toe-off", "landing", "flight-1", "flight-2", "flight-3"], "elderly-with-cane": ["loading-left", "contact-right", "loading-right", "passing-right"] };
const rad = (d) => (d * Math.PI) / 180;
const norm = (d) => { let a = ((d + 180) % 360 + 360) % 360 - 180; return a === -180 ? 180 : a; };
const r4 = (v) => { const n = Math.round(v * 1e4) / 1e4; return Object.is(n, -0) ? 0 : n; };
const world = (b) => ({ x: b.at[0], y: b.at[1], a: b.worldAngleDeg });
const toLocal = (b, p) => { const w = world(b); const dx = p[0] - w.x, dy = p[1] - w.y; const c = Math.cos(rad(w.a)), s = Math.sin(rad(w.a)); return [dx * c + dy * s, -dx * s + dy * c]; };
const dirLocal = (b, deg) => norm(deg - b.worldAngleDeg);
function convertPath(b, d) {
  const tok = d.match(/[MLCQZ]|-?\d*\.?\d+(?:e-?\d+)?/g); const out = []; let i = 0;
  while (i < tok.length) { const c = tok[i++]; if (c === "Z") { out.push("Z"); continue; }
    const n = c === "C" ? 3 : c === "Q" ? 2 : 1; const parts = [c];
    for (let k = 0; k < n; k++) { const p = toLocal(b, [Number(tok[i++]), Number(tok[i++])]); parts.push(`${r4(p[0])} ${r4(p[1])}`); }
    out.push(parts.join(" ")); }
  return out.join(" ");
}
export function convert(src) {
  const byId = Object.fromEntries(src.bones.map((b) => [b.id, b]));
  const bones = src.bones.map((b) => {
    const p = b.parent ? byId[b.parent] : null; const [x, y] = p ? toLocal(p, b.at) : b.at;
    const bone = { id: b.id, parent: b.parent ?? null, title: b.title, x: r4(x), y: r4(y), angleDeg: r4(p ? norm(b.worldAngleDeg - p.worldAngleDeg) : b.worldAngleDeg), lengthM: b.lengthM, joint: b.joint };
    if (b.mass) { const m = b.mass; const [cx, cy] = Array.isArray(m.com) ? toLocal(b, m.com) : m.comAt ? toLocal(b, m.comAt) : [m.com ?? b.lengthM / 2, m.comY ?? 0]; bone.mass = { kg: m.kg, comX: r4(cx), comY: r4(cy), distribution: m.distribution ?? "point", source: m.source ?? "literature", reference: m.reference }; if (!bone.mass.reference) delete bone.mass.reference; }
    if (b.section) bone.section = b.section;
    return bone; });
  const anchors = src.anchors.map((a) => { const [x, y] = toLocal(byId[a.bone], a.at); const { at, ...rest } = a; const { id, bone, ...tail } = rest; return { id, bone, x: r4(x), y: r4(y), ...tail }; });
  const shapes = src.shapes.map((s) => { const b = byId[s.bone]; const base = { id: s.id, bone: s.bone, role: s.role, lod: s.lod, z: s.z, filled: s.filled ?? true, tone: s.tone };
    let g;
    if (s.kind === "path" || s.d) g = { kind: "path", d: convertPath(b, s.d) };
    else if (s.kind === "poly") g = { kind: "poly", points: s.points.map((p) => toLocal(b, p).map(r4)), closed: s.closed ?? false };
    else if (s.kind === "capsule") { const p0 = toLocal(b, s.p0), p1 = toLocal(b, s.p1); g = { kind: "capsule", x0: r4(p0[0]), y0: r4(p0[1]), x1: r4(p1[0]), y1: r4(p1[1]), r0: s.r0, r1: s.r1 ?? s.r0 }; }
    else if (s.kind === "ellipse") { const c = toLocal(b, s.c); g = { kind: "ellipse", cx: r4(c[0]), cy: r4(c[1]), rx: s.rx, ry: s.ry, rotationDeg: r4(dirLocal(b, s.rotationDeg ?? 0)) }; if (!g.rotationDeg) delete g.rotationDeg; }
    else throw new Error(`shape ${s.id}: kind`);
    const out = { ...base, ...g }; if (s.region) out.region = s.region; if (s.material) out.material = s.material; return out; });
  const { bones: _b, anchors: _a, shapes: _s, rootAt, ...rest } = src;
  const asset = { id: rest.id, category: "humans", title: rest.title, summary: rest.summary, tags: rest.tags, provenance: rest.provenance, dimensions: rest.dimensions, materials: rest.materials,
    rig: { kind: rest.rigKind ?? "anatomical", root: src.bones[0].id, bones }, anchors, geometry: { elevation: shapes, ...(rest.plan ? { plan: rest.plan } : {}) }, poses: rest.poses.map((p) => ((KEY[rest.id] ?? []).includes(p.id) ? { ...p, key: true } : p)), clips: rest.clips, thermal: rest.thermal, structure: rest.structure, presentation: rest.presentation, exports: rest.exports, completion: "authored" };
  return asset;
}
function fmt(v, ind = 0) {
  const pad = "  ".repeat(ind);
  const flat = JSON.stringify(v);
  if (flat.length <= 150 || typeof v !== "object" || v === null) return flat;
  if (Array.isArray(v)) return "[\n" + v.map((x) => pad + "  " + fmt(x, ind + 1)).join(",\n") + "\n" + pad + "]";
  return "{\n" + Object.entries(v).map(([k, x]) => `${pad}  ${JSON.stringify(k)}: ${fmt(x, ind + 1)}`).join(",\n") + "\n" + pad + "}";
}
if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  const assets = [];
  for (const id of ORDER) { const f = join(here, `${id}.mjs`); if (!existsSync(f)) continue; const mod = await import(pathToFileURL(f).href + `?t=${Date.now()}`); assets.push(convert(mod.default)); }
  const doc = { $schema: "../../🧬️schema/🔣️.json#/$defs/CategoryDocument", version: "semio.staffage/v1", category: "humans", assets };
  writeFileSync(OUT, fmt(doc) + "\n");
  console.log(`wrote ${OUT}: ${assets.map((a) => a.id).join(", ")}`);
}
