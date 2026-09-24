#!/usr/bin/env bun
import { existsSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const G = join(REPO, ".tmp-ticket/wp-o2b/generated");
const activation = await import(`${REPO}/.tmp-ticket/wp-o2b/links/activation.ts`);
const catalog = await import(`${REPO}/.tmp-ticket/wp-o2b/links/catalog-view.ts`);
const deployment = await import(`${REPO}/.tmp-ticket/wp-o2b/links/deployment.ts`);

const moduleRoot = activation.pluginModulesRoot("dev");
const entries = catalog.readGeneratedCatalogProjection().entries;
let wrote = 0, skipped = 0, missing = 0;
const t0 = Date.now();
for (const entry of entries) {
  const dir = deployment.moduleDirectoryName(entry.pluginId);
  const moduleDirectory = join(moduleRoot, dir);
  if (!existsSync(moduleDirectory)) { missing++; continue; }
  const sourceRoot = join(REPO, entry.cratePath, "..", "..");
  if (!existsSync(sourceRoot)) { missing++; continue; }
  activation.writeStagedSourceFreshness(moduleDirectory, sourceRoot);
  wrote++;
}
const out = { wrote, skipped, missing, ms: Date.now() - t0, total: entries.length };
writeFileSync(join(G, "backfill-stat-index.json"), JSON.stringify(out, null, 2));
console.log(JSON.stringify(out));
