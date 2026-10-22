import { runToolSpawnLawV1 } from "../../🧰️direct-tool-spawn/⌨️command/🟦️.ts";
import { judgeProcessStartsV1, listProcessSpawnFilesV1, processAllowlistProblemsV1, scanRepositoryProcessStartsV1, PROCESS_ALLOWLIST_V1, type ProcessAllowlistV1 } from "../🟦️.ts";

/** 📋️ The machine-readable result of the raw-process-spawn law: scanned files, production sites per disposition, findings and stale allowlist rows. */
export type ProcessSpawnReportV1 = Readonly<{ scanned: number; sites: number; dispositions: Readonly<Record<string, number>>; findings: ReturnType<typeof judgeProcessStartsV1>["findings"]; problems: ReturnType<typeof processAllowlistProblemsV1>; stale: readonly string[] }>;

/** 🧮️ Scans the repository and judges every production process start against the tool registry and the allowlist. */
export function processSpawnReportV1(root: string, allowlist: ProcessAllowlistV1 = PROCESS_ALLOWLIST_V1): Readonly<{ report: ProcessSpawnReportV1; rows: ReturnType<typeof judgeProcessStartsV1>["rows"] }> {
  const files = listProcessSpawnFilesV1(root, allowlist), scan = scanRepositoryProcessStartsV1(root, files, undefined, allowlist), verdict = judgeProcessStartsV1(scan.sites, undefined, allowlist), dispositions: Record<string, number> = {};
  for (const row of verdict.rows) dispositions[row.disposition] = (dispositions[row.disposition] ?? 0) + 1;
  return { report: { scanned: scan.scanned, sites: verdict.rows.length, dispositions, findings: verdict.findings, problems: processAllowlistProblemsV1(new Set(files), allowlist), stale: verdict.stale }, rows: verdict.rows };
}

/** 🚪️ Runs the raw-process-spawn law: `[--report] [--json] [--strict] [--sites]`. Findings are process starts outside the interface modules and literal programs the registry does not know, each without an allowlist row; stale rows only warn unless `--strict`. */
export function runProcessSpawnLawV1(root: string, args: readonly string[], write: (line: string) => void = line => console.log(line), allowlist: ProcessAllowlistV1 = PROCESS_ALLOWLIST_V1): number {
  const { report, rows } = processSpawnReportV1(root, allowlist), failed = report.findings.length > 0 || report.problems.length > 0 || args.includes("--strict") && report.stale.length > 0;
  if (args.includes("--json")) { write(JSON.stringify(report, null, 1)); return failed ? 1 : 0; }
  write(`[verify raw-process-spawn] scanned ${report.scanned} files; ${report.sites} production process starts.`);
  for (const [name, count] of Object.entries(report.dispositions).sort((a, b) => b[1] - a[1])) write(`  ${String(count).padStart(5)} ${name}`);
  if (args.includes("--sites")) for (const row of rows) write(`${row.disposition}\t${row.path}\t${row.line}\t${row.callee}\t${row.form}\t${row.program}`);
  if (args.includes("--report")) return 0;
  for (const finding of [...report.findings, ...report.problems]) console.error(`raw-process-spawn\t${finding.rule}\t${finding.path}${finding.line > 0 ? ":" + finding.line : ""}\t${finding.program}\t${finding.detail}`);
  for (const row of report.stale.slice(0, 20)) write(`[verify raw-process-spawn] stale allowlist row ${row}`);
  if (report.stale.length > 0) write(`[verify raw-process-spawn] ${report.stale.length} stale allowlist row(s): remove the rows the code no longer needs.`);
  if (report.findings.length + report.problems.length > 0) { write(`[verify raw-process-spawn] ${report.findings.length + report.problems.length} finding(s).`); return 1; }
  if (args.includes("--strict") && report.stale.length > 0) return 1;
  write("[verify raw-process-spawn] passed.");
  return 0;
}

/** 🧰️ Runs both spawn laws behind `repo verify tool-spawns`: registered tools started outside their adapter (with the test ledger) and every other process start outside the interface modules. With `--json` the two results are one document. */
export function runToolSpawnLawsV1(root: string, args: readonly string[], write: (line: string) => void = line => console.log(line)): number {
  if (args.includes("--prune")) return runToolSpawnLawV1(root, args, write);
  if (!args.includes("--json")) { const first = runToolSpawnLawV1(root, args, write), second = runProcessSpawnLawV1(root, args, write); return Math.max(first, second); }
  const captured: string[] = [], first = runToolSpawnLawV1(root, args, line => { captured.push(line); }), { report } = processSpawnReportV1(root), second = report.findings.length > 0 || report.problems.length > 0 || args.includes("--strict") && report.stale.length > 0 ? 1 : 0;
  write(JSON.stringify({ ...JSON.parse(captured.join("\n")), rawProcessSpawn: report }, null, 1));
  return Math.max(first, second);
}
