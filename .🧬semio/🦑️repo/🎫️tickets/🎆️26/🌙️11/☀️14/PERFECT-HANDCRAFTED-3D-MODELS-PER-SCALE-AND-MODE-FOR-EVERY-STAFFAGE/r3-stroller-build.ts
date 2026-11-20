/** 🏗️ Assembles the round-3 volumes source of `humans/parent-with-stroller` from the hand-stated scale modules and writes `volumes/humans/parent-with-stroller.json`.
 * Usage: `bun r3-stroller-build.ts [--only 50,100]`
 * @see ./r3-stroller-kit.ts */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { scale50 } from "./r3-stroller-s50.ts";

const file = join(import.meta.dir, "volumes", "humans", "parent-with-stroller.json");
const src = JSON.parse(readFileSync(file, "utf8"));
const builders: Record<string, () => { body: any[]; thermal: any[]; structure: any[]; decisions: any[] }> = { "50": scale50 };
try { const m = await import("./r3-stroller-coarse.ts"); Object.assign(builders, { "100": m.scale100, "200": m.scale200, "500": m.scale500 }); } catch (e) { if (!String(e).includes("Cannot find module")) throw e; }
for (const [k, f] of Object.entries(builders)) {
  const v = f();
  src.volumes[k] = { status: "authored", denominator: Number(k), ...v };
  console.log(k, "body", v.body.length, "thermal", v.thermal.length, "structure", v.structure.length);
}
writeFileSync(file, JSON.stringify(src));
