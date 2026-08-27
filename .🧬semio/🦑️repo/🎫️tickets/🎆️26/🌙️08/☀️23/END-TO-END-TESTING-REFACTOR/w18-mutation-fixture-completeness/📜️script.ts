/** 🦠️ Audit: does every declared mutation kind have a fixture-backed mutate and inverse scenario? */
import { readdirSync, readFileSync, statSync, existsSync } from "node:fs";
import { join, relative } from "node:path";

const repoRoot = process.cwd();

const dirs = (p: string): string[] => {
  try { return readdirSync(p, { withFileTypes: true }).filter((e) => e.isDirectory()).map((e) => e.name); } catch { return []; }
};

const manifests: string[] = [];
const walk = (p: string, depth: number): void => {
  if (depth > 12) return;
  let entries;
  try { entries = readdirSync(p, { withFileTypes: true }); } catch { return; }
  for (const e of entries) {
    if (e.name === "node_modules" || e.name === "target" || e.name === ".git" || e.name === "temp") continue;
    const full = join(p, e.name);
    if (e.isDirectory()) walk(full, depth + 1);
    else if (e.name.endsWith("component.json") && full.includes("oracle")) manifests.push(full);
  }
};
walk(repoRoot, 0);

type Row = {
  manifest: string; owner: string; catalog: string; capability: string;
  kinds: string[]; deferred: string[]; vectors: { mutationId: string; src: string; proj: string; scenarios: string[] }[];
};
const rows: Row[] = [];
for (const m of manifests) {
  let parsed: any;
  try { parsed = JSON.parse(readFileSync(m, "utf8")); } catch { continue; }
  const cats = parsed.mutationCatalogs;
  if (!Array.isArray(cats)) continue;
  const owner = relative(repoRoot, m).split("/").slice(0, -2).join("/");
  for (const c of cats) {
    rows.push({
      manifest: relative(repoRoot, m), owner, catalog: c.id, capability: c.capability,
      kinds: c.kinds ?? [], deferred: c.deferredKinds ?? [],
      vectors: (c.vectors ?? []).map((v: any) => ({ mutationId: v.mutationId, src: v.sourceMutationDirectoryName, proj: v.mutationDirectoryName, scenarios: (v.scenarios ?? []).map((s: any) => s.id) })),
    });
  }
}

// feature files claiming catalogs
const features: { path: string; catalog: string; scenarioIds: string[] }[] = [];
const walkFeat = (p: string, depth: number): void => {
  if (depth > 14) return;
  let entries; try { entries = readdirSync(p, { withFileTypes: true }); } catch { return; }
  for (const e of entries) {
    if (e.name === "node_modules" || e.name === "target" || e.name === ".git" || e.name === "temp") continue;
    const full = join(p, e.name);
    if (e.isDirectory()) walkFeat(full, depth + 1);
    else if (e.name.endsWith(".feature")) {
      const text = readFileSync(full, "utf8");
      const tag = text.match(/@mutations-([a-z0-9-]+)/);
      if (!tag) continue;
      const ids: string[] = [];
      for (const line of text.split("\n")) {
        const t = line.match(/@id-([a-z0-9-]+)/);
        if (t) ids.push(t[1]);
      }
      // also Examples rows produce scenario ids; capture kind columns
      features.push({ path: relative(repoRoot, full), catalog: tag[1], scenarioIds: ids });
    }
  }
};
walkFeat(repoRoot, 0);

const totalKinds = rows.reduce((a, r) => a + r.kinds.length, 0);
const totalDeferred = rows.reduce((a, r) => a + r.deferred.length, 0);
const totalVectors = rows.reduce((a, r) => a + r.vectors.length, 0);
const kindsWithVector = rows.reduce((a, r) => a + r.kinds.filter((k) => r.vectors.some((v) => v.mutationId === k)).length, 0);

console.log(JSON.stringify({
  manifests: manifests.length, catalogs: rows.length, totalKinds, totalDeferred, totalVectors, kindsWithVector,
  featureFilesClaimingCatalog: features.length,
  catalogsWithZeroVectors: rows.filter((r) => r.vectors.length === 0).length,
  catalogsWithDeferred: rows.filter((r) => r.deferred.length > 0).length,
}, null, 2));
