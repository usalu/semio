/** 🪤️ Scans the `volumes` of every catalog asset and of every source under ./volumes for the silent geometry traps of the kernel and writes ./kernel-traps.md (and 🗑️generated/kernel2/traps.json). Usage: bun kernel2-scan.ts [label]  (label names the run in the report, e.g. start or end). */
import { existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { loftFault, meshIntegrity, revolveClosure, taperFault, tessellateSolid, validateModel } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";

const TICKET = dirname(fileURLToPath(import.meta.url));
const CATALOG = resolve(TICKET, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const QUALITY = { chordM: 0.001, minSegments: 16, maxSegments: 48 };
const LISTS = ["body", "thermal", "structure"] as const;
type Row = { asset: string; origin: string; scale: string; list: string; solid: string; trap: string; detail: string };
const rows: Row[] = [];
const label = process.argv[2] ?? "run";

const sources: { asset: string; origin: string; volumes: any }[] = [];
for (const folder of readdirSync(CATALOG).sort()) {
  const dir = join(CATALOG, folder);
  let slugs: string[];
  try { slugs = readdirSync(dir).sort(); } catch { continue; }
  for (const slug of slugs) {
    const path = join(dir, slug, "🔣️.json");
    if (!existsSync(path)) continue;
    const parsed = JSON.parse(readFileSync(path, "utf8"));
    const asset = parsed.asset ?? parsed;
    if (asset.volumes) sources.push({ asset: `${folder.replace(/^[^a-z]+/u, "")}/${slug}`, origin: "catalog", volumes: asset.volumes });
  }
}
const SOURCES = join(TICKET, "volumes");
for (const category of readdirSync(SOURCES).sort()) {
  for (const file of readdirSync(join(SOURCES, category)).filter((f) => f.endsWith(".json")).sort()) {
    const parsed = JSON.parse(readFileSync(join(SOURCES, category, file), "utf8"));
    if (parsed.volumes) sources.push({ asset: `${category}/${file.replace(/\.json$/u, "")}`, origin: "source", volumes: parsed.volumes });
  }
}

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

const merged = new Map<string, Row & { origins: Set<string> }>();
for (const r of rows) {
  const key = [r.asset, r.scale, r.list, r.solid, r.trap].join("|");
  const hit = merged.get(key);
  if (hit) hit.origins.add(r.origin);
  else merged.set(key, { ...r, origins: new Set([r.origin]) });
}
const list = [...merged.values()].sort((a, b) => a.trap.localeCompare(b.trap) || a.asset.localeCompare(b.asset) || a.scale.localeCompare(b.scale) || a.list.localeCompare(b.list) || a.solid.localeCompare(b.solid));
const counts = new Map<string, number>();
for (const r of list) counts.set(r.trap, (counts.get(r.trap) ?? 0) + 1);
mkdirSync(join(TICKET, "🗑️generated", "kernel2"), { recursive: true });
writeFileSync(join(TICKET, "🗑️generated", "kernel2", `traps-${label}.json`), JSON.stringify(list.map(({ origins, ...r }) => ({ ...r, origin: [...origins].join("+") })), null, 1));
const assetsWith = new Set(list.map((r) => r.asset));
const out = [
  `# Kernel traps in the catalog volumes (${label})`,
  "",
  `Generated by \`kernel2-scan.ts ${label}\` over ${sources.filter((s) => s.origin === "catalog").length} catalog assets with \`volumes\` and ${sources.filter((s) => s.origin === "source").length} sources under \`volumes/\`; ${solids} own solids (references \`of\` are judged through their body solid). Tessellated at chord 1 mm, 16..48 segments in the bone frame with the solid's own \`transform\`; \`repeat\` is not expanded.`,
  "",
  `${list.length} findings in ${assetsWith.size} assets. Origin: \`catalog\` (the asset file), \`source\` (the file under \`volumes/\`), \`catalog+source\` (both).`,
  "",
  "## Traps",
  "",
  "| trap | meaning | how to repair | findings |",
  "|---|---|---|---|",
  `| negative-signed-volume | a closed solid is inside-out (its faces look inward) | since the loft fix this cannot come from a loft; check mirrored transforms (negative scale), reversed profile order of an extrude is handled | ${counts.get("negative-signed-volume") ?? 0} |`,
  `| open-solid-where-closed-expected | a solid that is not a closed surface although no cap was switched off | check capStart/capEnd, a revolve profile with a zero-length segment, an extrude whose profile touches itself | ${counts.get("open-solid-where-closed-expected") ?? 0} |`,
  `| non-manifold | two parts of the solid touch in an edge or a point (pinch) | separate or overlap them properly | ${counts.get("non-manifold") ?? 0} |`,
  `| inconsistent-winding | adjacent faces disagree on the side | report to the kernel owner | ${counts.get("inconsistent-winding") ?? 0} |`,
  `| degenerate-faces | triangles of zero area or a solid without triangles | enlarge the feature, remove the collapsed section | ${counts.get("degenerate-faces") ?? 0} |`,
  `| self-intersecting-loft-sections | consecutive sections overlap (path turns back or bends tighter than the section is wide) | add sections along the bend, enlarge the bend radius or split the loft | ${counts.get("self-intersecting-loft-sections") ?? 0} |`,
  `| revolve-implied-closure | an open revolve profile off the axis at both ends is closed through the axis and gives a filled body | repeat the first point as the last point to revolve the enclosed loop (shell, wall, bead) | ${counts.get("revolve-implied-closure") ?? 0} |`,
  `| taper-uniform | taperTo has equal factors: no taper, only a scaled prism (taperTo is [scale at from, scale at to]) | write [1, k] for a taper, or scale the profile | ${counts.get("taper-uniform") ?? 0} |`,
  `| taper-degenerate | taperTo collapses an end (both 0, or a factor below 0.001) | use exactly 0 for an apex only at one end | ${counts.get("taper-degenerate") ?? 0} |`,
  `| kernel-refuses-solid | validateModel reports an error for the solid (self-intersecting profile, a hole touching the outline, a revolve profile whose end points share an axial coordinate so that the two closing discs coincide, bad numbers): tessellate would throw for the whole model | fix the profile; for a revolve shell write it as a loop | ${counts.get("kernel-refuses-solid") ?? 0} |`,
  `| loft-caps-missing | a loft without capStart or capEnd (schema-required) | add the booleans | ${counts.get("loft-caps-missing") ?? 0} |`,
  `| tessellation-failed | the kernel refused the solid | fix the numbers | ${counts.get("tessellation-failed") ?? 0} |`,
  "",
  "## Findings",
  "",
  "| asset | origin | scale | list | solid | trap | detail |",
  "|---|---|---|---|---|---|---|",
  ...list.map((r) => `| ${r.asset} | ${[...r.origins].sort().join("+")} | 1:${r.scale} | ${r.list} | ${r.solid} | ${r.trap} | ${r.detail} |`),
  "",
];
if (list.length === 0) out.splice(out.indexOf("## Findings"), 0, "No finding.", "");
writeFileSync(join(TICKET, label === "end" || label === "run" ? "kernel-traps.md" : `🗑️generated/kernel2/kernel-traps-${label}.md`), out.join("\n"));
console.log(`${label}: ${list.length} findings in ${assetsWith.size} assets from ${solids} solids`, Object.fromEntries(counts));
