import { existsSync, readFileSync, readdirSync } from "node:fs";
import { mkdir, writeFile } from "node:fs/promises";
import { join } from "node:path";

/** ✅️ Acceptance command of the PDF 1.7 artifact editor: gates (repo check/build/test of `@semio-tech/stdio-pdf`, the editor's Rust suites), every browser suite against the real app on port 6260, the third-party oracle over every saved or exported PDF, and the feature coverage check of `acceptance-features.json` (every inventoried feature needs at least one verdict and every verdict of it must pass).
 * Usage from the repo root: `repo <ticket>/acceptance.ts [gates] [e2e]` (both by default); exits 0 only when every stage ran and passed. Report: `🗑️generated/acceptance/report.json`.
 * @see ./🗺️feature-inventory.md */

const here = import.meta.dir;
const root = process.cwd();
const bun = process.execPath;
const generated = join(here, "🗑️generated", "acceptance");
const stages = process.argv.slice(2).length > 0 ? process.argv.slice(2) : ["gates", "e2e"];
const suites = [["journey", "pdf-pane-journey.ts"], ["content", "sweep-content.ts"], ["document", "sweep-document.ts"], ["annotations", "sweep-annotations.ts"], ["views", "sweep-views-history.ts"]] as const;
const rust = ["semio-s-artifact-stdio-pdf-page-editor-tests", "semio-s-artifact-stdio-pdf-page-editor-authoring-tests", "semio-s-artifact-stdio-pdf-editor-kit", "semio-s-artifact-stdio-pdf-editor-document", "semio-s-artifact-stdio-pdf-image-route", "semio-s-artifact-stdio-pdf-save-cause", "semio-s-artifact-stdio-pdf-native-save"];
type Check = { stage: string; name: string; command: string; exit: number; seconds: number; log: string };
const checks: Check[] = [];
await mkdir(generated, { recursive: true });

/** 🏃️ Runs one command to completion with its output in a log and records its exit status. */
const run = (stage: string, name: string, command: string[], env: Record<string, string> = {}) => {
  const log = join(generated, `${name.replace(/[^\w.-]+/gu, "_")}.log`);
  const started = Date.now();
  const child = Bun.spawnSync(command, { cwd: root, env: { ...process.env, ...env }, stdout: Bun.file(log), stderr: "pipe" });
  const exit = child.exitCode ?? 1;
  if (child.stderr.length > 0) Bun.write(`${log}.err`, child.stderr);
  checks.push({ stage, name, command: command.join(" "), exit, seconds: Math.round((Date.now() - started) / 1000), log });
  console.log(`${exit === 0 ? "PASS" : "FAIL"} ${stage} ${name} (exit ${exit})`);
  return exit;
};

/** 🌐️ Ensures the editor host answers on 6260, starting `serve-pdf-native.sh` when it does not; returns the started process. */
async function host() {
  const up = async () => fetch("http://127.0.0.1:6260/").then(response => response.ok, () => false);
  if (await up()) return undefined;
  Bun.spawnSync(["sh", join(here, "mount-products.sh")]);
  const started = Bun.spawn(["sh", join(here, "serve-pdf-native.sh")], { stdout: Bun.file(join(generated, "host.log")), stderr: "inherit" });
  for (const deadline = Date.now() + 300_000; !(await up()); await Bun.sleep(2000)) if (Date.now() > deadline) throw new Error("the PDF editor host did not answer on 6260 within 300 s");
  return started;
}

if (stages.includes("gates")) {
  for (const target of ["check", "build", "test"]) run("gates", `repo stdio-pdf:${target}`, ["repo", "run", `@semio-tech/stdio-pdf:${target}`, "--lang=en"], { SEMIO_LANGUAGE: "en" });
  run("gates", "typecheck harness", ["repo", "typecheck", ...readdirSync(here).filter(file => file.endsWith(".ts")).map(file => join(here, file))], { SEMIO_LANGUAGE: "en" });
  for (const crate of rust) run("gates", `cargo ${crate}`, ["sh", join(here, "native-cargo.sh"), "test", "-p", crate, ...(crate.endsWith("native-save") ? ["--lib"] : [])]);
}

type Verdict = { step: string; pass: boolean };
const verdicts: (Verdict & { suite: string })[] = [];
if (stages.includes("e2e")) {
  const started = await host();
  try {
    for (const [suite, script] of suites) {
      const out = `acceptance-${suite}`;
      const folder = join(here, "🗑️generated", out);
      Bun.spawnSync(["rm", "-rf", folder]);
      run("e2e", `suite ${suite}`, [bun, join(here, script)], { PDF_JOURNEY_OUT: out });
      const file = join(folder, "journey.json");
      if (existsSync(file)) for (const verdict of JSON.parse(readFileSync(file, "utf8")) as Verdict[]) verdicts.push({ suite, step: verdict.step, pass: verdict.pass });
      else verdicts.push({ suite, step: `${suite}: journey.json written`, pass: false });
      if (existsSync(folder) && readdirSync(folder).some(name => name.endsWith(".pdf"))) run("e2e", `oracle ${suite}`, ["uv", "run", "--offline", "--with", "pymupdf==1.26.7", "--with", "pypdf", "python", "-I", join(here, "journey-oracle.py"), "all", folder]);
    }
  } finally {
    started?.kill();
  }
}

const manifest = JSON.parse(readFileSync(join(here, "acceptance-features.json"), "utf8")) as { features: { id: string; feature: string; steps: string[] }[] };
const coverage = stages.includes("e2e") ? manifest.features.map(entry => {
  const matched = verdicts.filter(verdict => entry.steps.some(pattern => new RegExp(pattern, "u").test(`${verdict.suite}: ${verdict.step}`)));
  return { id: entry.id, feature: entry.feature, verdicts: matched.length, failing: matched.filter(verdict => !verdict.pass).map(verdict => `${verdict.suite}: ${verdict.step}`), pass: matched.length > 0 && matched.every(verdict => verdict.pass) };
}) : [];
const failedVerdicts = verdicts.filter(verdict => !verdict.pass);
const complete = ["gates", "e2e"].every(stage => stages.includes(stage));
const pass = complete && checks.every(check => check.exit === 0) && failedVerdicts.length === 0 && coverage.every(entry => entry.pass);
await writeFile(join(generated, "report.json"), `${JSON.stringify({ at: new Date().toISOString(), stages, complete, pass, checks, verdicts: { total: verdicts.length, failed: failedVerdicts }, coverage }, null, 1)}\n`);
for (const entry of coverage.filter(item => !item.pass)) console.log(`UNCOVERED ${entry.id} ${entry.feature}: ${entry.verdicts === 0 ? "no verdict" : entry.failing.join("; ")}`);
console.log(`${pass ? "ACCEPTED" : "REJECTED"}: ${checks.filter(check => check.exit === 0).length}/${checks.length} checks, ${verdicts.length - failedVerdicts.length}/${verdicts.length} verdicts, ${coverage.filter(entry => entry.pass).length}/${coverage.length} features${complete ? "" : ", incomplete stages"}`);
process.exitCode = pass ? 0 : 1;
