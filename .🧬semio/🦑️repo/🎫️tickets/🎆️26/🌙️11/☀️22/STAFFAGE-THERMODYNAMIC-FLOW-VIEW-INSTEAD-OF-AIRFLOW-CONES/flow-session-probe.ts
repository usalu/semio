/** 🔬️ Probes the flow solver protocol outside the browser: develops the reference walker through `FlowServer` and prints every reply kind with its time. Usage: bun flow-session-probe.ts [pose|clip] */
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
const S = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const transport = await import(join(S, "🔨️modules/💨️transport/🟦️.ts"));
const walker = JSON.parse(readFileSync(join(S, "🧫️fixtures/🚶️reference-walker/🔣️.json"), "utf8"));
const query = process.argv[2] === "clip" ? { kind: "clip", clip: "walk", timeS: 0 } : { kind: "pose", pose: "stand" };
const t0 = performance.now();
let slices = 0;
try {
  const field = await transport.developTransport(walker, { ...query, timeS: 14.5 }, walker.thermal.environment, { maxAgeS: 14, ...(query.kind === "clip" ? { periodS: 1 } : {}), flow: true, pathlines: false, progress: (d: number, t: number) => { slices = d; if (d % 8 === 0) console.log(`slice ${d}/${t} at ${((performance.now() - t0) / 1000).toFixed(1)} s`); } });
  console.log("done", ((performance.now() - t0) / 1000).toFixed(1), "s", slices, "slices", field.parcels.length, "parcels", field.flow.streams.length, "streams");
} catch (error) {
  console.log("error after", slices, "slices:", error);
}
