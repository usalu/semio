/** 🔭️ W3-F1 probe: runs the repo-wide tool-run policy predicates and prints every finding plus the ones under the lane's roots (reasoning, dag, trinity). */
import * as script from "/Users/ueli/Documents/semio/📜️script.ts";

const repoRoot = "/Users/ueli/Documents/semio";
const laneRoots = ["✏️s/🔌️plugins/💡️reasoning/", "✏️s/🔌️plugins/🕸️dag/", "✏️s/🔌️plugins/🔱️trinity/"];
const rows = script.INTERACTIVITY_TOOL_RUN_REQUIREMENTS;
const sources = script.interactivityToolRunPolicySources(repoRoot, rows);
console.log(`sources=${sources.length}`);
for (const [name, findings] of [
  ["amend", script.interactivityToolRunAmendFailures(sources, rows)],
  ["local-lifecycle", script.interactivityToolRunLocalLifecycleFailures(sources, rows)],
  ["legacy-trace", script.interactivityToolRunLegacyTraceFailures(sources)],
  ["declaration", script.interactivityToolRunDeclarationFailures(sources, rows)],
  ["reserved-action", script.interactivityToolRunReservedActionFailures(sources)],
] as const) {
  const lane = findings.filter((finding) => laneRoots.some((root) => finding.file.startsWith(root)) || laneRoots.some((root) => finding.text.includes(root)));
  console.log(`== ${name}: ${findings.length} total, ${lane.length} lane`);
  for (const finding of lane) console.log(`LANE ${finding.file}:${finding.line} ${finding.text}`);
}
