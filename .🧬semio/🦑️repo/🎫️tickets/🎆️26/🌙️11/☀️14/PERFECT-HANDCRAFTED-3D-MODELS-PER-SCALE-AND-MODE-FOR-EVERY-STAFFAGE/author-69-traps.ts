/** 🪤️ Kernel traps of the SOURCES of the given human slugs only: bun author-69-traps.ts slug [slug...] (prints rows; nothing written). */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { loftFault, meshIntegrity, revolveClosure, taperFault, tessellateSolid, validateModel } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
const TICKET = dirname(fileURLToPath(import.meta.url));
const QUALITY = { chordM: 0.001, minSegments: 16, maxSegments: 48 };
const LISTS = ["body", "thermal", "structure"] as const;
type Row = { asset: string; origin: string; scale: string; list: string; solid: string; trap: string; detail: string };
const rows: Row[] = [];
const sources = process.argv.slice(2).map((slug) => ({ asset: `humans/${slug}`, origin: "source", volumes: JSON.parse(readFileSync(join(TICKET, "volumes", "humans", `${slug}.json`), "utf8")).volumes }));
const GEOMETRY_ONLY = new Set(["kind", "size", "cornerRadius", "radii", "p0", "p1", "r0", "r1", "lateralScale", "profile", "axis", "origin", "sweepRad", "holes", "plane", "from", "to", "taperTo", "sections", "capStart", "capEnd", "transform"]);
let solids = 0;
for (const { asset, origin, volumes } of sources) {
  for (const scale of Object.keys(volumes)) {
    const volume = volumes[scale];
    if (volume?.status !== "authored") continue;
    for (const list of LISTS) {
      for (const s of volume[list] ?? []) {
        if (s.of !== undefined) continue;
        solids++;
        const add = (trap: string, detail: string) => rows.push({ asset, origin, scale, list, solid: s.id, trap, detail });
        const geometry = Object.fromEntries(Object.entries(s).filter(([k]) => GEOMETRY_ONLY.has(k)));
        if (s.kind === "revolve") {
          const closure = revolveClosure(s.profile);
          if (closure.kind === "shell") add("revolve-implied-closure", `open profile off the axis at both ends is filled (${closure.filledAreaM2.toExponential(2)} m2 instead of the loop ${closure.loopAreaM2.toExponential(2)} m2)`);
        }
        if (s.kind === "extrude") {
          const fault = taperFault(s.taperTo);
          if (fault) add(fault === "uniform" ? "taper-uniform" : "taper-degenerate", `taperTo ${JSON.stringify(s.taperTo)}`);
        }
        if (s.kind === "loft") {
          const fold = loftFault(s.sections);
          if (fold) add("self-intersecting-loft-sections", `sections ${fold[0]} and ${fold[1]} overlap`);
        }
        if (s.kind === "loft" && (typeof s.capStart !== "boolean" || typeof s.capEnd !== "boolean")) {
          add("loft-caps-missing", "capStart and capEnd are required by the schema; the kernel treats a missing cap as off");
          continue;
        }
        if (s.kind === "revolve" || s.kind === "extrude" || s.kind === "loft") {
          const doc: any = { version: "semio.model/v1", id: "d", title: { en: "d", de: "d" }, nodes: [{ id: "n", parent: null, translation: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1] }], solids: [{ id: "s", node: "n", material: "m", ...geometry }], meshes: [], skins: [], materials: [{ id: "m", title: { en: "m", de: "m" }, baseColor: [1, 1, 1, 1], metallic: 0, roughness: 1, emissive: [0, 0, 0], alphaMode: "opaque", doubleSided: false, unlit: false }], variants: [], poses: [], animations: [] };
          const errors = validateModel(doc).filter((i) => i.severity === "error");
          if (errors.length > 0) {
            add("kernel-refuses-solid", `${[...new Set(errors.map((i) => i.code))].join(", ")} (the kernel validator would stop tessellate for the whole model)`);
            continue;
          }
        }
        let m;
        try {
          const g = tessellateSolid({ id: "s", node: "n", material: "m", ...geometry } as never, QUALITY);
          m = meshIntegrity(g.positions, g.indices);
        } catch (e) {
          add("tessellation-failed", String((e as Error).message).slice(0, 120));
          continue;
        }
        const capsOff = s.kind === "loft" && (s.capStart === false || s.capEnd === false);
        const openByDesign = capsOff || s.open === true;
        if (m.closed && m.signedVolume < 0) add("negative-signed-volume", `signed volume ${m.signedVolume.toExponential(3)} m3: the solid is inside-out`);
        if (!m.closed && !openByDesign) add("open-solid-where-closed-expected", `${m.openEdges} open edges, ${m.nonManifoldEdges} non-manifold edges`);
        if (m.closed && !m.manifold) add("non-manifold", `${m.pinchVertices} pinch vertices`);
        if (m.nonManifoldEdges > 0 && !m.closed) add("non-manifold", `${m.nonManifoldEdges} edges used by more than two triangles`);
        if (m.closed && !m.oriented) add("inconsistent-winding", `${m.inconsistentEdges} edges walked the same way twice`);
        if (m.degenerate > 0) add("degenerate-faces", `${m.degenerate} of ${m.triangles} triangles have no area`);
        if (m.triangles === 0) add("degenerate-faces", "no triangles at all");
      }
    }
  }
}

for (const r of rows) console.log(`${r.asset} 1:${r.scale} ${r.list} ${r.solid} ${r.trap} ${r.detail}`);
console.log(`${rows.length} findings`);
