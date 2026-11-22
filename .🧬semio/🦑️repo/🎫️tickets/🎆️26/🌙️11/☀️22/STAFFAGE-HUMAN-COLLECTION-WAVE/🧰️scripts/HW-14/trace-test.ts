import { readFileSync } from "node:fs";
import { traceOutline } from "./lib.ts";
const a = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
const t0 = performance.now();
const r = traceOutline(a, "50", "front", (s) => s.material === "service-coat" && ["torso", "lumbar", "pelvis", "thigh-l", "thigh-r"].includes(s.bone));
console.log(r.length, r.map((x) => x.length), ((performance.now() - t0) / 1000).toFixed(1));
console.log(JSON.stringify(r[0]).slice(0, 300));
