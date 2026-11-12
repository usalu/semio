import { readFileSync } from "node:fs";
const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { modelOf } = await import(`${S}/🔨️modules/🧊️volume/🟦️.ts`);
const { qualityOf } = await import(`${S}/🔨️modules/🧊️volume/⏱️job/🟦️.ts`);
const { tessellate } = await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts");
const { encodeFbxDetailed } = await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🎬️fbx/🟦️.ts");
const [cat, slug, clip] = process.argv.slice(2);
const a = JSON.parse(readFileSync(`${S}/🗂️catalog/${cat}/${slug}/🔣️.json`, "utf8")).asset;
const d = await modelOf(a, { scale: 50, at: { kind: "clip", clip, timeS: 0 }, poses: [], clips: [clip], views: ["light-solid"] });
const r = await tessellate(d, qualityOf(a));
const prims = r.meshes.flatMap((m: any) => m.primitives.map((p: any) => ({ mesh: m.id, mat: p.material, tri: p.indices.length / 3, skinned: !!p.joints })));
console.log(prims.reduce((n: number, p: any) => n + p.tri, 0), prims.slice(0, 12));
const res = encodeFbxDetailed(r, { animation: clip, variant: "light-solid", format: "binary" }, { report() {}, cancelled: () => false });
console.log(res.notices.slice(0, 10));
console.log("nodes with mesh", r.nodes.filter((n: any) => n.mesh).map((n: any) => [n.id, n.mesh]).slice(0, 10), r.nodes.filter((n: any) => n.mesh).length);

const { readFbx, childrenNamed, child } = await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🎬️fbx/📖️reader/🟦️.ts");
const f = readFbx(res.bytes);
const objects = child(f.nodes.find((n: any) => n.name === "Objects"), "Geometry") ? childrenNamed(f.nodes.find((n: any) => n.name === "Objects"), "Geometry") : [];
let polys = 0; for (const g of objects) { const pv = child(g, "PolygonVertexIndex")!.properties[0]!.value as ArrayLike<number>; let n = 0; for (let i = 0; i < pv.length; i++) if (pv[i]! < 0) n++; polys += n; }
console.log("fbx geometries", objects.length, "polygons", polys);
