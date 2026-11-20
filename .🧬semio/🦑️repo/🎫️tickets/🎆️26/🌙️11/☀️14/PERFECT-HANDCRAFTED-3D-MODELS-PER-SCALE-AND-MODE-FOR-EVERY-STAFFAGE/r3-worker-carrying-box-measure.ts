/** 📏️ Runs the product's `measureVolumes` and `checkVolumes` on the candidate document of `humans/worker-carrying-box` (draft rig, draft 2D and the authoring volumes in one file), so the round-3 volumes can be measured before the root fixes are in the catalog.
 * Usage: `bun r3-worker-carrying-box-measure.ts [--scale N] [--rules a,b] [--all]` prints failing measurements and the thermal areas; `--all` prints every measurement. */
import { readFileSync } from "node:fs";
import { join } from "node:path";

import * as volumes from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { GENERATED, type Json } from "./volumes-lib.ts";

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(name) < 0 ? undefined : args[args.indexOf(name) + 1]);
const asset = JSON.parse(readFileSync(join(GENERATED, "r3-worker-carrying-box", "🔣️.json"), "utf8")).asset as Json;
const options = { ...(flag("--scale") === undefined ? {} : { scale: Number(flag("--scale")) }), ...(flag("--rules") === undefined ? {} : { rules: flag("--rules")!.split(",") }) };
const list = (volumes as Json).measureVolumes(asset, options) as Json[];
let bad = 0;
for (const m of list) {
  const info = String(m.name).startsWith("info:");
  if (!m.ok && !info) bad++;
  if (args.includes("--all") || (!m.ok && !info) || /^(info:outerAreaM2|areaDeviation)/.test(m.name)) console.log(`${m.ok || info ? "  " : "! "}${m.rule} 1:${m.scale} ${m.list ?? ""} ${m.name}=${typeof m.value === "number" ? Math.round(m.value * 1e4) / 1e4 : m.value}${m.limit === undefined || info ? "" : ` (${m.bound} ${Math.round(m.limit * 1e4) / 1e4})`}`);
}
for (const f of (volumes as Json).checkVolumes(asset, options) as Json[]) console.log(`F ${f.rule} 1:${f.scale} ${f.list ?? ""} ${f.code}: ${String(f.message.en).slice(0, 300)}`);
console.log(bad === 0 ? "no failing measurement" : `${bad} failing measurement(s)`);
