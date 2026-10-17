// 🪣️ Prints the puzzle-fill interactivity predicates (run-job, P4e) as the audit reads the sources. usage: bun fillprobe.ts <out.json>
import {writeFileSync} from "node:fs";
import {interactivityPuzzleFillSources, interactivityPuzzleFillRunJobFailures, interactivityPuzzleFillP4eFailures} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🪣️tool-run-and-history/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const sources = interactivityPuzzleFillSources(root);
const out = {
  sizes: Object.fromEntries(Object.entries(sources).map(([k, v]) => [k, v.length])),
  runJob: interactivityPuzzleFillRunJobFailures(sources),
  p4e: interactivityPuzzleFillP4eFailures(sources.precompute, sources.fill, sources.geometry),
};
writeFileSync(process.argv[2]!, JSON.stringify(out, null, 1));
console.log(JSON.stringify({sizes: out.sizes, runJob: out.runJob.length, p4e: out.p4e.length}));
