/** 📦️ Breakdown of the GLB of a default pose: bytes by node group (flow, body, glyph …), by kind, JSON by key and per node/mesh, with unique accessor accounting. glb.ts <asset id> [--pose <id>] */
import { writeFileSync } from "node:fs";
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { writeGlb } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";

const [target, ...rest] = process.argv.slice(2);
const asset = (await readStreamedAsset((await assetPathsFromFiles()).find((e) => e.folder === target)!)).asset;
const pose = rest.includes("--pose") ? rest[rest.indexOf("--pose") + 1]! : asset.presentation.defaultPose;
const t0 = Date.now();
const bytes = await writeGlb(asset, { pose });
if (process.env.OUT) writeFileSync(process.env.OUT, bytes);
const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
const jsonBytes = view.getUint32(12, true);
const json = JSON.parse(new TextDecoder().decode(bytes.subarray(20, 20 + jsonBytes)));
const size: Record<number, number> = { 5120: 1, 5121: 1, 5122: 2, 5123: 2, 5125: 4, 5126: 4 };
const width: Record<string, number> = { SCALAR: 1, VEC2: 2, VEC3: 3, VEC4: 4, MAT4: 16 };
const bytesOf = (i: number): number => { const a = json.accessors[i]; const v = json.bufferViews[a.bufferView]; return v.byteStride ? v.byteStride * a.count : a.count * size[a.componentType]! * width[a.type]!; };
console.log(`glb ${bytes.length} (${(bytes.length / 1048576).toFixed(2)} MiB) pose ${pose}, json ${jsonBytes}, bin ${bytes.length - jsonBytes - 28}, ${Date.now() - t0} ms; scenes ${json.scenes.length} nodes ${json.nodes.length} meshes ${json.meshes.length} accessors ${json.accessors.length}`);
console.log("json by key:", Object.entries(json).map(([k, v]) => [k, JSON.stringify(v).length] as const).sort((a, b) => b[1] - a[1]).slice(0, 8).map(([k, n]) => `${k} ${n}`).join(", "));
const nodesJson = json.nodes.map((n: any) => JSON.stringify(n).length) as number[];
console.log(`nodes: ${json.nodes.length}, avg ${(nodesJson.reduce((a, b) => a + b, 0) / json.nodes.length).toFixed(0)} B; with mesh ${json.nodes.filter((n: any) => n.mesh !== undefined).length}`);
const meshJson = JSON.stringify(json.meshes).length;
const extras = json.meshes.reduce((s: number, m: any) => s + m.primitives.reduce((t: number, p: any) => t + JSON.stringify(p.extras ?? {}).length, 0), 0);
console.log(`meshes json ${meshJson}, of it primitive extras.solids ${extras}`);
const group = (name: string): string => /^flow/.test(name) ? "flow" : /glyph|^g-/.test(name) ? "glyph" : "other";
const owner = new Map<number, string>();
const seen = new Set<number>();
const by: Record<string, { bytes: number; tri: number; prims: number; names: Set<string> }> = {};
const kinds: Record<string, number> = {};
const solidKinds: Record<string, { n: number; tri: number; bytes: number }> = {};
for (const scene of json.scenes) {
  const walk = (i: number): void => {
    const node = json.nodes[i];
    if (node.mesh !== undefined) for (const p of json.meshes[node.mesh].primitives) {
      for (const [name, a] of [...Object.entries(p.attributes as Record<string, number>), ["idx", p.indices as number] as const]) if (!seen.has(a)) {
        seen.add(a); kinds[name] = (kinds[name] ?? 0) + bytesOf(a);
      }
      for (const s of p.extras?.solids ?? []) {
        const kind = String(s.id).replace(/-?\d+(-\d+)*$/, "").replace(/^(flow-(stream|arrow|vapour|fog)).*/, "$1").replace(/[a-z0-9]+-[a-z0-9]+.*$/, (m: string) => m).slice(0, 14);
        const grp = /^flow-arrow/.test(s.id) ? "flow-arrow" : /^flow-vapour/.test(s.id) ? "flow-vapour" : /^flow-fog/.test(s.id) ? "flow-fog" : /^flow-stream/.test(s.id) ? "flow-stream-tube" : "other";
        const k = (solidKinds[scene.name.split("-")[1]! + ":" + grp] ??= { n: 0, tri: 0, bytes: 0 });
        k.n += 1; k.tri += s.count / 3;
      }
    }
    for (const c of node.children ?? []) walk(c);
  };
  for (const r of scene.nodes) walk(r);
}
console.log("bin by kind:", Object.entries(kinds).map(([k, v]) => `${k} ${v}`).join(", "));
const tot: Record<string, { n: number; tri: number }> = {};
for (const [k, v] of Object.entries(solidKinds)) { const g = k.split(":")[1]!; const t = (tot[g] ??= { n: 0, tri: 0 }); t.n += v.n; t.tri += v.tri; }
console.log("solids (all scenes, with repeats):", Object.entries(tot).map(([k, v]) => `${k} n=${v.n} tri=${v.tri}`).join("; "));
for (const scene of json.scenes) {
  const mode = scene.name;
  let tri = 0, flowTri = 0, ownBytes = 0, prims = 0;
  const walk = (i: number): void => {
    const node = json.nodes[i];
    if (node.mesh !== undefined) for (const p of json.meshes[node.mesh].primitives) {
      prims += 1;
      const t = p.indices !== undefined && (p.mode === undefined || p.mode === 4) ? json.accessors[p.indices].count / 3 : 0;
      tri += t;
      for (const s of p.extras?.solids ?? []) if (/^flow/.test(s.id)) flowTri += s.count / 3;
    }
    for (const c of node.children ?? []) walk(c);
  };
  for (const r of scene.nodes) walk(r);
  console.log(`  ${mode}: ${prims} prims, ${tri} triangles, flow ${flowTri}`);
}
