/** 🧟️ Layered-overview mutants (overview-like-play revision): copies the layered overview's geometry module, element, fixture, schema and their two test
 * files into `🗑️generated/overview-like-play/mutants/` (imports of everything else point back at the repository), then
 * breaks the copy one way at a time and runs the copied tests: every mutant must fail them. Nothing in the repository's
 * own sources is touched, so other sessions and running dev servers never see a mutant.
 * `bun overview_camera_mutants.ts` prints one line per mutant and writes `mutants.json` beside the copies. */
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const ui = join(repoRoot, "🧰️framework", "🔨️modules", "🖱️ui");
const out = join(import.meta.dir, "🗑️generated", "overview-moves");
const work = join(out, "mutants");
const GEOMETRY = "🔨️modules/🥞️layered-overview-geometry/🟦️.ts";
const ELEMENT = "🧱️elements/🥞️LayeredOverview/🟦️.tsx";
const COPIED = [GEOMETRY, "🔨️modules/🥞️layered-overview-geometry/🧪️tests/🔬️unit/🟦️.ts", "🧫️fixtures/🥞️layered-overview/🔣️.json", "🧬️schema/🥞️layered-overview/🔣️.json", ELEMENT, "🧱️elements/🥞️LayeredOverview/🧪️tests/🧩️component/🟦️.tsx"] as const;

interface Mutant {
  readonly name: string;
  readonly file: string;
  readonly edits: readonly (readonly [string, string])[];
}

const MUTANTS: readonly Mutant[] = [
  { name: "by default a device that reports reduced motion freezes the overview", file: ELEMENT, edits: [['reducedMotion = "never" } = props;', 'reducedMotion = "auto" } = props;']] },
  { name: "the follow closes 12 % per frame, however long the frame took", file: GEOMETRY, edits: [["** (Math.max(0, elapsedMs) / LAYERED_FOLLOW_FRAME_MS)", "** 1"]] },
  { name: "the element steps its follow per frame instead of per elapsed time", file: ELEMENT, edits: [["followStep(offsetRef.current, targetRef.current, elapsed)", "followStep(offsetRef.current, targetRef.current)"]] },
  { name: "reduced motion still pans", file: ELEMENT, edits: [['const panning = mode === "strip" && pan === "pointer" && !reduced;', 'const panning = mode === "strip" && pan === "pointer";']] },
  { name: '`pan="none"` still pans', file: ELEMENT, edits: [['const panning = mode === "strip" && pan === "pointer" && !reduced;', 'const panning = mode === "strip" && !reduced;']] },
  { name: "a touch pointer pans the strip", file: ELEMENT, edits: [['if (event.pointerType !== "mouse" || openedRef.current !== null || revealedRef.current !== null) return;', "if (openedRef.current !== null || revealedRef.current !== null) return;"]] },
  { name: "the mouse pans the strip away from a revealed page", file: ELEMENT, edits: [['if (event.pointerType !== "mouse" || openedRef.current !== null || revealedRef.current !== null) return;', 'if (event.pointerType !== "mouse" || openedRef.current !== null) return;']] },
  { name: "the root always says that the mouse pans", file: ELEMENT, edits: [['data-pan={panning ? "pointer" : "none"}', 'data-pan="pointer"']] },
  { name: "a pace of zero still waits for an idle moment", file: ELEMENT, edits: [["if (delay <= 0) return warm(step.id);", "if (delay < 0) return warm(step.id);"]] },
  { name: "the glass has no hole for a page that is only partly on screen", file: GEOMETRY, edits: [['return { kind: "hole", left: horizontal.start, top: vertical.start, right: horizontal.end, bottom: vertical.end };', 'return { kind: "whole" };']] },
  { name: "a cell's bounds ignore the pan", file: GEOMETRY, edits: [["const start = Math.max(0, index - offset);", "const start = Math.max(0, index);"]] },
];

const slashed = (path: string): string => path.replaceAll("\\", "/");
const copied = new Set(COPIED.map((file) => slashed(join(ui, file))));

/** 📦️ A source of the repository as its copy needs it: LF line ends, and every relative import that leaves the copied
 * files pointing at the repository's own file. */
function forCopy(file: string): string {
  const source = readFileSync(join(ui, file), "utf8").replaceAll("\r\n", "\n");
  return source.replace(/(from\s+")(\.\.?\/[^"]+)(")/gu, (whole, before: string, specifier: string, after: string) => {
    const target = slashed(resolve(dirname(join(ui, file)), specifier));
    return copied.has(target) ? whole : `${before}${target}${after}`;
  });
}

rmSync(work, { recursive: true, force: true });
const pristine = new Map<string, string>();
for (const file of COPIED) {
  mkdirSync(dirname(join(work, file)), { recursive: true });
  pristine.set(file, forCopy(file));
  writeFileSync(join(work, file), pristine.get(file)!);
}
const config = join(work, "vitest.config.ts");
writeFileSync(
  config,
  `import { defineConfig } from "vitest/config";\nexport default defineConfig({ root: ${JSON.stringify(slashed(work))}, cacheDir: ${JSON.stringify(slashed(join(work, ".vite")))}, esbuild: { jsx: "automatic" }, server: { fs: { strict: false } }, test: { environment: "jsdom", include: ["**/🧪️tests/**/🟦️.{ts,tsx}"], setupFiles: [${JSON.stringify(slashed(join(ui, "🧪️tests", "🧹️react-environment", "🟦️.ts")))}], passWithNoTests: false } });\n`,
);

function runTests(): { readonly passed: boolean; readonly summary: string } {
  const result = spawnSync(process.execPath, [join(repoRoot, "node_modules", "vitest", "vitest.mjs"), "run", "--config", config], { cwd: repoRoot, encoding: "utf8", windowsHide: true, timeout: 300_000 });
  const lines = `${result.stdout}\n${result.stderr}`.replace(/\u001b\[[0-9;]*m/gu, "").split(/\r?\n/u);
  return { passed: result.status === 0, summary: lines.filter((line) => /^\s*(Test Files|Tests)\s/u.test(line)).map((line) => line.trim().replace(/\s+/gu, " ")).join("; ") };
}

const baseline = runTests();
console.log(`[DEBUG] baseline: ${baseline.passed ? "green" : "RED"} (${baseline.summary})`);
if (!baseline.passed) throw new Error("the copied tests fail without any mutant; nothing can be concluded");
const report: { name: string; file: string; killed: boolean; summary: string }[] = [];
for (const mutant of MUTANTS) {
  let text = pristine.get(mutant.file)!;
  for (const [needle, replacement] of mutant.edits) {
    if (text.split(needle).length !== 2) throw new Error(`mutant "${mutant.name}": ${JSON.stringify(needle)} occurs ${text.split(needle).length - 1} times in ${relative(repoRoot, join(ui, mutant.file))}`);
    text = text.replace(needle, replacement);
  }
  writeFileSync(join(work, mutant.file), text);
  const outcome = runTests();
  writeFileSync(join(work, mutant.file), pristine.get(mutant.file)!);
  report.push({ name: mutant.name, file: mutant.file, killed: !outcome.passed, summary: outcome.summary });
  console.log(`[DEBUG] ${outcome.passed ? "SURVIVED" : "killed"}: ${mutant.name} (${outcome.summary})`);
}
writeFileSync(join(out, "mutants.json"), `${JSON.stringify({ baseline, mutants: report }, null, 2)}\n`);
console.log(`[DEBUG] ${report.filter((entry) => entry.killed).length}/${report.length} mutants killed`);
