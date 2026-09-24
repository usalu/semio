import { readFileSync, writeFileSync } from "node:fs";
import { discoverTestContributions, loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const registry = loadOracleRegistry(root);
const engineOf = new Map(registry.oracles.map((o: any) => [o.id, o.engine?.family]));
const PROFILES: Record<string, string[]> = {
  "csv-rfc4180-equation-1-mutate": ["semantic-equation-csv-v1"],
  "csv-rfc4180-reader": ["unordered-json-v1"],
  "three-fem2d-mesh-reader": ["semantic-fem-mesh-manifold-v1"],
  "manifold-fem2d-mesh-measure": ["semantic-fem-mesh-manifold-v1"],
  "three-fem3d-mesh-reader": ["semantic-fem-mesh-manifold-v1"],
  "manifold-fem3d-mesh-measure": ["semantic-fem-mesh-manifold-v1"],
  "quick-xml-drawing-1-mutate": ["semantic-drawing-svg-v1"],
  "three-carrier-reader": ["semantic-mesh-manifold-v1"],
  "manifold-mesh-measure": ["semantic-mesh-manifold-v1"],
  "manifold3d-three": ["semantic-mesh-manifold-v1"],
  "quick-xml-drawing-svg-reader": ["xml-element-tree"],
  "ixmilia-dxf-drawing-reader": ["semantic-drawing-carrier-v1"],
  "lopdf-drawing-pdf-reader": ["semantic-drawing-carrier-v1"],
};
const OUTCOME: Record<string, string> = { fatal: "rejected", error: "rejected", warning: "applied", info: "applied" };
const log: string[] = [];
for (const c of discoverTestContributions(root)) {
  const path = `${root}/${c.manifestPath}`;
  const text = readFileSync(path, "utf8");
  const raw = JSON.parse(text);
  const note = (rule: string) => log.push(`${rule} ${c.manifestPath}`);
  for (const f of raw.fixtureManifests ?? []) {
    if (f.generator?.platform === "darwin") { f.generator.platform = "darwin-arm64"; note("platform"); }
    if (f.provenance?.source === "handcrafted") { f.provenance.source = "authored"; note("source"); }
    if (f.units) {
      for (const k of ["length", "angle"]) if (f.units[k] === "none") { f.units[k] = "unitless"; note("units"); }
      for (const k of ["up", "handedness"]) if (f.units[k] === "none") { delete f.units[k]; note("units"); }
    }
    if (typeof f.family === "string" && f.family.includes("+")) { f.family = f.family.replaceAll("+", "-"); note("family"); }
    const family = f.generator ? engineOf.get(f.generator.oracle) : undefined;
    if (f.generator && family && f.generator.engineFamily !== family && !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(f.generator.engineFamily)) { f.generator.engineFamily = family; note("engineFamily"); }
  }
  for (const p of raw.comparisonProfiles ?? []) if (p.text === true) { p.text = "utf8"; note("text"); }
  for (const h of raw.oracleHostPackages ?? []) if ("_comment" in h) { const { _comment, ...rest } = h; Object.keys(h).forEach((k) => delete h[k]); Object.assign(h, { ...rest, rationale: _comment }); note("host-rationale"); }
  for (const m of raw.mutationManifests ?? []) for (const mu of m.mutations ?? []) {
    if (Array.isArray(mu.outcomes) && mu.outcomes.some((o: string) => o in OUTCOME)) { mu.outcomes = [...new Set(mu.outcomes.map((o: string) => OUTCOME[o] ?? o))]; note("outcomes"); }
  }
  for (const o of raw.oracles ?? []) {
    if (!Array.isArray(o.comparisonProfiles) && PROFILES[o.id]) { o.comparisonProfiles = PROFILES[o.id]; note("oracle-profiles"); }
    if (o.ecosystem === "typescript") { o.ecosystem = "javascript"; note("ecosystem"); }
    if (o.engine?.family === "Pillow") { o.engine.family = "pillow"; note("engine-family"); }
  }
  for (const d of raw.noOracleDecisions ?? []) {
    if (typeof d.capability === "string" && d.capabilities === undefined) {
      const { capability, blockers, notes, ...rest } = d;
      const rationale = [d.rationale, ...(blockers ?? []), notes].filter((x) => typeof x === "string" && x.length > 0).join("\n\n");
      Object.keys(d).forEach((k) => delete d[k]);
      Object.assign(d, { id: rest.id, capabilities: [capability], rationale, ...Object.fromEntries(Object.entries(rest).filter(([k]) => !["id", "rationale"].includes(k))) });
      note("decision-shape");
    }
  }
  const next = JSON.stringify(raw, null, 2) + "\n";
  if (next !== text && process.argv[2] === "apply") writeFileSync(path, next);
}
const counts: Record<string, number> = {};
for (const l of log) { const k = l.split(" ")[0]; counts[k] = (counts[k] ?? 0) + 1; }
console.log(JSON.stringify(counts));
console.log([...new Set(log.map((l) => l.split(" ").slice(1).join(" ")))].join("\n"));
