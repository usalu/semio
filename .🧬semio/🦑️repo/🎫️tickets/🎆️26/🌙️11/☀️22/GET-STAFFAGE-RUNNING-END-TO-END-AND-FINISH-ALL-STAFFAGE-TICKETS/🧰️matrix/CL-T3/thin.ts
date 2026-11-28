/** 🧮️ What the thinning of lane CL-T3 would drop, without writing: per scale and list the stations and the stations dropped at the share `SHARE` of the smallest feature (default the one of the repair). thin.ts <asset id | file.json> */
import { readDoc } from "../MR/lib.ts";
import { SHARE, log, thin } from "./repairs.ts";

const { doc } = readDoc(process.argv[2]!);
for (const scale of [50, 100, 200, 500]) for (const mode of ["body", "thermal", "structure"] as const) {
  const lofts = doc.asset.volumes[String(scale)][mode].filter((s: { kind?: string }) => s.kind === "loft");
  const before = lofts.reduce((n: number, s: { sections: unknown[] }) => n + s.sections.length, 0);
  console.log(`1:${scale} ${mode}: ${lofts.length} lofts, ${before} stations, dropped ${thin(doc.asset, scale, [mode], Number(process.env.SHARE ?? SHARE))}`);
}
if (process.env.SHOW !== "0") for (const line of log) console.log(`   ${line}`);
