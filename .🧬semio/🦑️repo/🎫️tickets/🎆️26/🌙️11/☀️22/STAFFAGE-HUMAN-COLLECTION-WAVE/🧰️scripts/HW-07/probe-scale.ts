/** 🧪️ Probe: the jogger scaled by a factor, validated. bun probe-scale.ts <factor> */
import { join } from "node:path";
import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { GENERATED, loadHuman, scaleAsset } from "./lib.ts";
const doc = await loadHuman("jogger");
scaleAsset(doc.asset, Number(process.argv[2] ?? 1.088));
await Bun.write(join(GENERATED, "probe-scale.json"), JSON.stringify(doc));
const issues = validateAsset(doc.asset);
console.log(issues.length);
for (const i of issues.slice(0, 40)) console.log(`  ${i.code}: ${i.message.en}`.slice(0, 400));
