#!/usr/bin/env bun
/** 🔍️ U5 — read-only live check of the dev hub owner's catalog verdict on real data roots. Usage: bun u5-catalog-freshness.ts <dataDir...> */
import { devHubCatalogFreshnessV1, devHubStatusTextV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts";
for (const dataDir of process.argv.slice(2)) {
  const verdict = devHubCatalogFreshnessV1(dataDir);
  console.log(JSON.stringify({ dataDir, verdict }));
  if (verdict.kind === "stale") for (const locale of ["en", "de"] as const) console.log(devHubStatusTextV1({ kind: "catalog-stale", dataDir, reason: verdict.reason }, locale));
}
