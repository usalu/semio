import { readFile } from "node:fs/promises";
import { join } from "node:path";

/** 🧮️ Journey-level memory budget of one PDF editor page: runs `probe-memory.ts` (boot, a title edit, Open File of the two-page fixture `opens` times, settle) in one headless browser and asserts the budget of `pdf-memory-budget.json` on its rows: one compilation of the core module for the whole page, and that core module the opened app's own component (`component`, the descriptor's component of the PDF 1.7 editor) and no other, one live linear memory once the opens settled, no renderer growth per further open, and the warden's resident measure and the renderer's peak footprint below the cap with headroom. Prints one verdict per budget and the peak resident tree the warden saw, and exits non-zero when a budget fails.
 * Usage: `bun pdf-memory-budget.ts [label]` from the workspace root with the host on 6260 (`sh serve-pdf-native.sh`).
 * @see ./🧮️pdf-browser-page-memory.md */

type Budget = { opens: number; wardenCapMB: number; component: string; coreModuleCompilations: number; liveLinearMemoriesSettled: number; perOpenRendererGrowthMB: number; settledTreeResidentMB: number; rendererPeakFootprintMB: number };
type Report = { largeCompiles?: number; liveMemories?: number; memoryMB?: number[]; largeModules?: string[] };
type Row = { stage: string; tree?: { totalMB: number }; peakMB?: number; renderer?: { footprintMB: number; peakFootprintMB: number }; main?: Report | string; workers?: { report: Report | string }[] };

const label = process.argv[2] ?? "budget";
const budget = JSON.parse(await readFile(join(import.meta.dir, "pdf-memory-budget.json"), "utf8")) as Budget;
const probe = Bun.spawn([process.execPath, join(import.meta.dir, "probe-memory.ts"), label], { cwd: process.cwd(), env: { ...process.env, PROBE_OPENS: String(budget.opens), PROBE_DUMPS: "0", PROBE_ABORT_MB: String(budget.wardenCapMB - 64) }, stdout: "inherit", stderr: "inherit" });
const exited = await probe.exited;
const rows = (await readFile(join(import.meta.dir, "🗑️generated", `memory-${label}.jsonl`), "utf8")).split("\n").filter(Boolean).map(line => JSON.parse(line) as Row);
const stage = (name: string) => rows.find(row => row.stage === name);
const reports = (row: Row | undefined) => [row?.main, ...(row?.workers ?? []).map(worker => worker.report)].filter((report): report is Report => typeof report === "object" && report !== null);
const settled = stage("settled");
const last = stage(`open-${budget.opens}`), before = stage(`open-${budget.opens - 1}`);
const aborted = rows.find(row => row.stage === "abort");
const verdicts = [
  { budget: "the journey ran to the end below the warden cap", pass: exited === 0 && settled !== undefined && aborted === undefined, measured: aborted ? `aborted at ${aborted.tree?.totalMB} MB` : `exit ${exited}` },
  { budget: `core module compilations = ${budget.coreModuleCompilations}`, pass: reports(settled).reduce((sum, report) => sum + (report.largeCompiles ?? 0), 0) === budget.coreModuleCompilations, measured: reports(settled).reduce((sum, report) => sum + (report.largeCompiles ?? 0), 0) },
  { budget: `every compiled core module is the opened app's component ${budget.component}`, pass: reports(settled).flatMap(report => report.largeModules ?? []).length > 0 && reports(settled).flatMap(report => report.largeModules ?? []).every(name => name.includes(`${budget.component}.core`)), measured: reports(settled).flatMap(report => report.largeModules ?? []) },
  { budget: `live linear memories after settling = ${budget.liveLinearMemoriesSettled}`, pass: reports(settled).reduce((sum, report) => sum + (report.liveMemories ?? 0), 0) === budget.liveLinearMemoriesSettled, measured: reports(settled).flatMap(report => report.memoryMB ?? []) },
  { budget: `renderer footprint growth per further open <= ${budget.perOpenRendererGrowthMB} MB`, pass: last?.renderer !== undefined && before?.renderer !== undefined && last.renderer.footprintMB - before.renderer.footprintMB <= budget.perOpenRendererGrowthMB, measured: last?.renderer && before?.renderer ? last.renderer.footprintMB - before.renderer.footprintMB : null },
  { budget: `settled tree resident <= ${budget.settledTreeResidentMB} MB`, pass: (settled?.tree?.totalMB ?? Infinity) <= budget.settledTreeResidentMB, measured: settled?.tree?.totalMB ?? null },
  { budget: `renderer peak footprint <= ${budget.rendererPeakFootprintMB} MB`, pass: (settled?.renderer?.peakFootprintMB ?? Infinity) <= budget.rendererPeakFootprintMB, measured: settled?.renderer?.peakFootprintMB ?? null },
];
for (const verdict of verdicts) console.log(`${verdict.pass ? "PASS" : "FAIL"} ${verdict.budget}: ${JSON.stringify(verdict.measured)}`);
console.log(`peak tree resident seen by the sampler: ${settled?.peakMB ?? aborted?.tree?.totalMB} MB (warden cap ${budget.wardenCapMB} MB)`);
process.exit(verdicts.every(verdict => verdict.pass) ? 0 : 1);
