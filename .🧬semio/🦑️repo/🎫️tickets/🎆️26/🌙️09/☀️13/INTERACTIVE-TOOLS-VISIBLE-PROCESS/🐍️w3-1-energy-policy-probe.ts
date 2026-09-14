/** 🔋️ W3-1 probe: runs the W1-D tool-run policy predicates against the energy requirement row only and prints every finding. */
import * as script from "/Users/ueli/Documents/semio/📜️script.ts";

const repoRoot = "/Users/ueli/Documents/semio";
const rows = script.INTERACTIVITY_TOOL_RUN_REQUIREMENTS.filter((row) => row.toolId === "energySimulation");
const sources = script.interactivityToolRunPolicySources(repoRoot, script.INTERACTIVITY_TOOL_RUN_REQUIREMENTS);
const energy = (findings: readonly { file: string; line: number; text: string }[]) => findings.filter((finding) => finding.file.includes("🔋️energy"));
let total = 0;
for (const [name, findings] of [
  ["amend", energy(script.interactivityToolRunAmendFailures(sources, rows))],
  ["local-lifecycle", energy(script.interactivityToolRunLocalLifecycleFailures(sources, rows))],
  ["legacy-trace", energy(script.interactivityToolRunLegacyTraceFailures(sources))],
  ["declaration", script.interactivityToolRunDeclarationFailures(sources, rows)],
  ["reserved-action", energy(script.interactivityToolRunReservedActionFailures(sources))],
] as const) {
  total += findings.length;
  console.log(`== ${name}: ${findings.length}`);
  for (const finding of findings) console.log(`${finding.file}:${finding.line} ${finding.text}`);
}
console.log(`energy findings=${total}`);
process.exit(total === 0 ? 0 : 1);
