/** 🧪️ Probe: the jogger with twisted chest and spread right leg, its flat shapes on those bones solidified, validated. */
import { join } from "node:path";
import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { GENERATED, clone, loadHuman, solidify } from "./lib.ts";
const doc = await loadHuman("jogger");
const a = doc.asset;
const base = a.poses.find((p: any) => p.id === "stretch-stand");
const mk = (id: string, bones: any) => ({ ...clone(base), id, bones });
a.poses.push(mk("probe-chest", { torso: { twistDeg: -20 }, chest: { twistDeg: -30 }, neck: { twistDeg: 25 }, head: { twistDeg: 20 } }));
a.poses.push(mk("probe-thigh", { "thigh-r": { angleDeg: 60, abductionDeg: -40 } }));
const t0 = performance.now();
console.log("changed", solidify(a, ["torso", "chest", "neck", "head", "thigh-r", "shank-r", "foot-r", "toe-r"]), ((performance.now() - t0) / 1000).toFixed(1), "s");
await Bun.write(join(GENERATED, "probe-solidify.json"), JSON.stringify(doc));
const issues = validateAsset(a);
console.log(issues.length);
for (const i of issues.slice(0, 40)) console.log(`  ${i.code}: ${i.message.en}`.slice(0, 500));
