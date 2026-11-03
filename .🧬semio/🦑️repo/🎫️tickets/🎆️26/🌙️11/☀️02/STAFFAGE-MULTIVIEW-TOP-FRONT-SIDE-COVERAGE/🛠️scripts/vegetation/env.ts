import { readdirSync } from "node:fs";
import { load, CAT } from "./lib.ts";
for (const d of readdirSync(CAT)) { const { asset } = load(d); console.log(d, JSON.stringify(asset.thermal.environment), asset.poses.map((p: any) => p.id + ":" + Object.values(p.bones).some((b: any) => b.abductionDeg)).join(",")); }
