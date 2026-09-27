/** ⏱️ F3 — phase timeline of `serve s react dev` up to the Vite spawn (same steps as ServeScript, Vite not started).
 * usage: bun f3-serve-phases.ts [variant] */
import { join } from "node:path";
const DEV = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const t0 = performance.now();
const mark = (label: string) => console.log(`[f3-phase] ${label} @ ${(performance.now() - t0).toFixed(0)} ms`);
const act = await import(`${DEV}/♻️activation/🟦️.ts`);
mark("import activation");
const plan = await import(`${DEV}/../🔌️plugin/🏗️build/📋️plan/🟦️.ts`);
mark("import plan");
const fresh = await import(`${DEV}/♻️activation/🔍️freshness/🟦️.ts`);
mark("import freshness");
const hubMod = await import(`${DEV}/🚀️local-hub/🏃️execution/🟦️.ts`);
mark("import local-hub");
const variant = process.argv[2] ?? "s";
const root = `${DEV}/📦️packages/🟦️typescript`;
const runtime = act.developmentRuntimeRoot(root, variant, "dev", "react");
const receipt = act.readActivationReceipt(join(runtime, "activation"));
mark(`receipt (${receipt.plugins.length} plugins)`);
const resolved = plan.resolvePlaygroundFilter(variant);
mark("resolvePlaygroundFilter");
const freshness = fresh.reportServeStagedModuleFreshness(variant, "react", "dev", runtime, receipt);
mark("freshness sync part");
const hub = await hubMod.ensureDevLocalHub("/Users/ueli/Documents/semio", { hubUrl: process.env.S_HUB_URL || undefined });
mark(`ensureDevLocalHub (${hub?.hubUrl ?? "none"})`);
await freshness;
mark("freshness settled");
