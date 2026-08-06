import { join } from "node:path";
import { writeFileSync } from "node:fs";

const root = process.cwd();
const ticket = process.argv[2]!;
const withFlag = process.env.SEMIO_OS_STATE_AUTHORITY === "1";

try {
  // Import the module — policy is the only export we need; call helpers via a small eval of the same file internals by re-running defineLint with flag.
  const mod = await import(join(root, "📜️script.ts"));
  const TechnologyLinter = (await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts"))).TechnologyLinter;
  const linter = new TechnologyLinter("probe", root);
  console.log(`[DEBUG] invoking policy (SEMIO_OS_STATE_AUTHORITY=${process.env.SEMIO_OS_STATE_AUTHORITY ?? ""})`);
  const breaches = await mod.policy(linter);
  const kinds = new Map<string, number>();
  for (const b of breaches) kinds.set(b.kind, (kinds.get(b.kind) ?? 0) + 1);
  const osKinds = [...kinds.entries()].filter(([k]) => k.startsWith("os-state-authority"));
  const summary = {
    flag: withFlag,
    total: breaches.length,
    high: breaches.filter((b) => b.priority === "high").length,
    osStateAuthority: breaches.filter((b) => String(b.kind).startsWith("os-state-authority")).length,
    byKind: Object.fromEntries([...kinds.entries()].sort((a, b) => b[1] - a[1])),
    osByKind: Object.fromEntries(osKinds),
    osSamples: breaches.filter((b) => String(b.kind).startsWith("os-state-authority")).slice(0, 40).map((b) => ({ id: b.id, kind: b.kind, scope: b.scope, line: b.line, summary: b.summary })),
  };
  writeFileSync(join(ticket, withFlag ? "🧪w3-policy-flag-on-summary.json" : "🧪w3-policy-default-summary.json"), JSON.stringify(summary, null, 2));
  console.log(JSON.stringify({ total: summary.total, high: summary.high, osStateAuthority: summary.osStateAuthority, osByKind: summary.osByKind }, null, 2));
} catch (e) {
  console.error("[DEBUG] policy probe failed:", e);
  writeFileSync(join(ticket, "🧪w3-policy-probe-error.txt"), String(e?.stack ?? e));
  process.exit(1);
}
