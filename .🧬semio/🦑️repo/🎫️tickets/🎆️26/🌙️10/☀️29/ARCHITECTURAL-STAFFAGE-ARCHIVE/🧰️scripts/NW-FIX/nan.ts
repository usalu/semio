/** [DEBUG] Locate non-finite thermal fields. */
import { readFileSync } from "node:fs";
import { readdirSync } from "node:fs";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveThermalAt } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";

const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
function locate(id: string): any {
  for (const cat of readdirSync(root)) {
    try {
      const files = readdirSync(`${root}/${cat}/${id}`);
      const doc = files.find((f) => f.endsWith(".json"));
      if (doc) return JSON.parse(readFileSync(`${root}/${cat}/${id}/${doc}`, "utf8")).asset;
    } catch { /* next */ }
  }
  throw new Error(id);
}
const cases: [string, string, number][] = [["delivery-van", "stop-and-unload", 6], ["motor-scooter", "stop-and-shut-down", 3.5]];
for (const [id, clip, timeS] of cases) {
  const asset = locate(id);
  const state = evaluate(asset, { kind: "clip", clip, timeS });
  const field = solveThermalAt(asset, { kind: "clip", clip, timeS });
  console.log("\n==", id, timeS);
  const inputs = (state as { thermal?: { regions?: Record<string, unknown> } }).thermal?.regions ?? {};
  for (const [rid, region] of Object.entries(field.regions)) {
    const bad = Object.entries(region).filter(([, v]) => typeof v === "number" && !Number.isFinite(v));
    if (bad.length) console.log(" region", rid, bad.map(([k]) => k).join(","), "input", JSON.stringify(inputs[rid]));
  }
  const envBad = Object.entries(field.environment).filter(([, v]) => typeof v === "number" && !Number.isFinite(v));
  if (envBad.length) console.log(" env", envBad);
}
