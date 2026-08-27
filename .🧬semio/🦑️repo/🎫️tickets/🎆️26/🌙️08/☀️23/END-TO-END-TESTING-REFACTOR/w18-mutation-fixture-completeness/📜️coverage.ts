/** 🦠️ Repo-native mutation-coverage audit: uses the platform's own feature parser. */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
import { parseFeature } from "/Users/ueli/Documents/semio/./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";

const repoRoot = process.cwd();
const skip = new Set(["node_modules", "target", ".git", "temp", "storybook-static"]);
const features: string[] = [];
const manifests: string[] = [];
const walk = (p: string, d: number): void => {
  if (d > 14) return;
  let es; try { es = readdirSync(p, { withFileTypes: true }); } catch { return; }
  for (const e of es) {
    if (skip.has(e.name)) continue;
    const f = join(p, e.name);
    if (e.isDirectory()) { if (!f.includes("⚡️cache")) walk(f, d + 1); }
    else if (e.name.endsWith(".feature")) features.push(f);
    else if (e.name.endsWith("component.json") && f.includes("🧪️oracle")) manifests.push(f);
  }
};
walk(repoRoot, 0);

type Cat = { id: string; capability: string; kinds: string[]; deferred: string[]; owner: string; manifest: string };
const catalogs: Cat[] = [];
for (const m of manifests) {
  let j: any; try { j = JSON.parse(readFileSync(m, "utf8")); } catch { continue; }
  for (const c of j.mutationCatalogs ?? []) catalogs.push({ id: c.id, capability: c.capability, kinds: c.kinds ?? [], deferred: c.deferredKinds ?? [], owner: relative(repoRoot, m).split("/").slice(0, -2).join("/"), manifest: relative(repoRoot, m) });
}

const claimed = new Map<string, { path: string; ids: Set<string>; capability: string | null }>();
for (const f of features) {
  const parsed = parseFeature(readFileSync(f, "utf8"));
  if (parsed.mutationCatalog === null) continue;
  claimed.set(parsed.mutationCatalog, { path: relative(repoRoot, f), ids: new Set(parsed.scenarios.map((s) => s.id)), capability: parsed.capability });
}

const report: any[] = [];
let totalKinds = 0, coveredMutate = 0, coveredInverse = 0, orphanCatalogs = 0;
for (const c of catalogs) {
  totalKinds += c.kinds.length;
  const claim = claimed.get(c.id);
  if (!claim) { orphanCatalogs++; report.push({ catalog: c.id, owner: c.owner, kinds: c.kinds.length, problem: "no-feature-claims-catalog", missingMutate: c.kinds, missingInverse: c.kinds }); continue; }
  const missingMutate = c.kinds.filter((k) => !claim.ids.has(`mutate-${k}`));
  const missingInverse = c.kinds.filter((k) => !claim.ids.has(`inverse-${k}`));
  coveredMutate += c.kinds.length - missingMutate.length;
  coveredInverse += c.kinds.length - missingInverse.length;
  if (missingMutate.length || missingInverse.length || c.deferred.length) report.push({ catalog: c.id, owner: c.owner, feature: claim.path, kinds: c.kinds.length, missingMutate, missingInverse, deferred: c.deferred });
}
const strayFeatures = [...claimed.keys()].filter((id) => !catalogs.some((c) => c.id === id));
console.log(JSON.stringify({ catalogs: catalogs.length, featuresClaiming: claimed.size, totalKinds, coveredMutate, coveredInverse, orphanCatalogs, strayFeatures, catalogsWithGaps: report.length }, null, 2));
writeFileSync(process.argv[2], JSON.stringify(report, null, 1));
