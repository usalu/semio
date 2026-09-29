#!/usr/bin/env bun
/**
 * 🪟️ R10 window-3 landing tool — one kernel-derive input per invocation, strictly serial (preamble 14, item 1). Every
 * step is a dry run unless `--apply`; every step is idempotent and re-derived from the live tree at run time.
 *   taxonomy   register the unresolved directories (kinds probes of every pass) in `🔣️taxonomy.json` via
 *              `taxonomy-register.py`, validated by discovery's `validateTaxonomy` before the live file is written
 *   discovery  R9's registry catalog content-input patch (`wp-r9/launch-manifest-inputs.discovery.patch`) + the law that
 *              every launch-projected manifest is a registry catalog content input (re-derived: R9's law patch no
 *              longer applies because its type hunk already landed)
 *   targets    the nx targets slices handed over (`window3-spec.json` `targets`, with `configurations`) into their
 *              `📋️project.json`, inserted textually after the last target so each file keeps its own formatting, plus the
 *              exact-once command edits (`projectJsonEdits`)
 *   seed       curated launch rows (`seedRows`), row edits (`seedRowEdits`, `seedTextEdits`) in `.vscode/🧩️launch.seed.jsonc`
 *   render     `.vscode/launch.json` rendered by the registry's own generator (never hand-spliced)
 *   plan       goal-plan checks (`planChecks`) into the acceptance plan
 *   st2-r10    ST2's per-family stdio set, R10's part (`wp-st2/st2-apply.py --part r10`: taxonomy rows, project manifests,
 *              launch seed/json, `🧩️composition` → `🏘️composition`): every edited file backed up, the new files and the move
 *              recorded in the backup's `.r10-manifest.json` so `revert st2-r10` deletes and moves them back
 *   revert     `revert <step> --apply` restores the files the newest applied run of that step changed
 * Every applied write keeps the file's previous bytes under `<STATE>/window3-backups/<step>-<time>/`; every input and output
 * (kinds probes, candidate, previews) lives in STATE = `.🧬semio/🌐hub/s14-r10-state/` (gitignored ticket dirs are swept).
 * Usage: bun window3-apply.ts <step> [--apply]
 */
import { copyFileSync, existsSync, mkdirSync, readdirSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const HERE = import.meta.dir;
const TAXONOMY = join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
const SEED = join(ROOT, ".vscode/🧩️launch.seed.jsonc");
const LAUNCH = join(ROOT, ".vscode/launch.json");
const PLAN = join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/🎚️config/🔣️.json");
const LAW = join(ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🚀️launch/🟦️.ts");
const DISCOVERY_PATCH = join(ROOT, ".tmp-ticket/wp-r9/launch-manifest-inputs.discovery.patch");
const STATE = join(ROOT, ".🧬semio/🌐hub/s14-r10-state");
mkdirSync(STATE, { recursive: true });
const KINDS = (): string[] => readdirSync(STATE).filter((name) => /^tax-kinds-.*\.json$/u.test(name) && !name.endsWith(".live.json")).sort().map((name) => join(STATE, name));

type Spec = {
  targets: { hold?: string; project: string; projectJson: string; name: string; command: string; forwardAllArgs: boolean; cache: boolean; dependsOn?: string[]; configurations?: Record<string, { args: string }> }[];
  projectJsonEdits?: { projectJson: string; target: string; before: string; after: string }[];
  seedTextEdits?: { row: string; before: string; after: string }[];
  seedRows: { after: string; row: { name: string } & Record<string, unknown> }[];
  seedRowEdits: { name: string; command: string }[];
  seedInputEdits: { id: string; default: string }[];
  planChecks: { step: string; landed?: string; hold?: string; check: { id: string } & Record<string, unknown> }[];
  planSteps: { after: string; step: { id: string } & Record<string, unknown> }[];
};
const spec = JSON.parse(readFileSync(join(HERE, "window3-spec.json"), "utf8")) as Spec;
const [step, revertStep] = process.argv.slice(2);
const apply = process.argv.includes("--apply");
const BACKUPS = join(STATE, "window3-backups");
const backupDir = join(BACKUPS, `${step}-${new Date().toISOString().replaceAll(":", "-")}`);

/** 💾️ Writes a live file, first keeping its current bytes under this run's backup directory (`revert <step>` restores
 * the newest backup of that step). */
function save(path: string, text: string): void {
  const kept = join(backupDir, path.slice(ROOT.length + 1));
  if (existsSync(path) && !existsSync(kept)) {
    mkdirSync(dirname(kept), { recursive: true });
    copyFileSync(path, kept);
  }
  writeFileSync(path, text);
}

function stepRevert(): void {
  const newest = readdirSync(BACKUPS).filter((name) => name.startsWith(`${revertStep}-`)).sort().at(-1);
  if (!newest) throw new Error(`no backup of step ${revertStep}`);
  const walk = (dir: string): string[] => readdirSync(dir, { withFileTypes: true }).flatMap((entry) => (entry.isDirectory() ? walk(join(dir, entry.name)) : [join(dir, entry.name)]));
  const manifestPath = join(BACKUPS, newest, ".r10-manifest.json");
  const manifest = existsSync(manifestPath) ? (JSON.parse(readFileSync(manifestPath, "utf8")) as { created: string[]; moves: [string, string][] }) : { created: [], moves: [] };
  for (const [from, to] of manifest.moves) {
    console.log(`move back ${to} → ${from}`);
    if (apply && existsSync(join(ROOT, to))) renameSync(join(ROOT, to), join(ROOT, from));
  }
  for (const created of manifest.created) {
    console.log(`remove ${created}`);
    if (apply) rmSync(join(ROOT, created), { force: true });
  }
  for (const kept of walk(join(BACKUPS, newest)).filter((path) => path !== manifestPath)) {
    const live = join(ROOT, kept.slice(join(BACKUPS, newest).length + 1));
    console.log(`restore ${live.slice(ROOT.length + 1)}`);
    if (apply) copyFileSync(kept, live);
  }
}
const run = (command: string[], cwd = ROOT) => {
  const result = Bun.spawnSync(command, { cwd, stdout: "pipe", stderr: "pipe" });
  return { code: result.exitCode, out: result.stdout.toString() + result.stderr.toString() };
};

/** 🔍️ Index just past the `}` that closes the JSON object opening at `open` (string-aware). */
function objectEnd(text: string, open: number): number {
  let depth = 0;
  let inString = false;
  for (let index = open; index < text.length; index += 1) {
    const char = text[index];
    if (inString) {
      if (char === "\\") index += 1;
      else if (char === '"') inString = false;
    } else if (char === '"') inString = true;
    else if (char === "{") depth += 1;
    else if (char === "}" && --depth === 0) return index + 1;
  }
  throw new Error("unbalanced JSON object");
}

function stepTaxonomy(): void {
  const candidate = join(STATE, "taxonomy.window3.json");
  const excluded: string[] = [];
  for (let attempt = 0; ; attempt += 1) {
    const register = run(["python3", join(HERE, "taxonomy-register.py"), ...KINDS(), ...excluded.flatMap((name) => ["--exclude", name]), "--base", TAXONOMY, "--out", candidate, "--report", join(STATE, "tax-register-window3.json")]);
    console.log(register.out.trim());
    if (register.code !== 0) throw new Error("taxonomy registration failed");
    const validate = run(["bun", join(HERE, "taxonomy-validate.ts"), candidate]);
    console.log(validate.out.trim());
    if (validate.code === 0) break;
    const refused = [...validate.out.matchAll(/has invalid exact member "([^"]+)"/gu)].map((match) => match[1]!);
    if (!refused.length || attempt >= 3) throw new Error("candidate taxonomy is invalid — live file untouched");
    excluded.push(...refused);
    console.log(`refused member names (rename, not registration): ${refused.join(", ")}`);
  }
  console.log(run(["git", "diff", "--no-index", "--stat", TAXONOMY, candidate]).out.trim());
  if (apply) save(TAXONOMY, readFileSync(candidate, "utf8"));
}

const LAW_ANCHOR = `  it("stays loadable by an independent JSONC reader and is byte-identical to the committed launch.json", async () => {`;
const LAW_BLOCK = `  it("declares every launch-projected project manifest as a registry catalog content input", async () => {
    const { projects } = await renderLaunch();
    const { loadCatalogTaxonomy, registryCatalogInputPaths, registryCatalogInputView } = await import("../../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts");
    const root = getWorkspaceRoot(), taxonomy = loadCatalogTaxonomy(), base = registryCatalogInputView(root, taxonomy);
    const manifests = projects.map((project) => (project.path ? \`\${project.path}/📋️project.json\` : "📋️project.json"));
    const reachable = new Set(manifests.flatMap((manifest) => manifest.split("/").map((_, index, segments) => segments.slice(0, index + 1).join("/"))));
    const inputs = new Set(registryCatalogInputPaths(root, taxonomy, {
      kind: (path) => base.kind(path),
      readText: (path) => base.readText(path),
      entries: (path) => base.entries(path).filter((entry) => reachable.has(path ? \`\${path}/\${entry.name}\` : entry.name)),
    }));
    expect(manifests.length).toBeGreaterThan(100);
    expect(manifests.filter((manifest) => !inputs.has(manifest))).toEqual([]);
  });

`;

function stepDiscovery(): void {
  const check = run(["git", "apply", "--check", DISCOVERY_PATCH]);
  const applied = run(["git", "apply", "--check", "--reverse", DISCOVERY_PATCH]).code === 0;
  console.log(`discovery patch: ${applied ? "already applied" : check.code === 0 ? "applies cleanly" : `does NOT apply:\n${check.out}`}`);
  const law = readFileSync(LAW, "utf8");
  const lawPresent = law.includes(`it("declares every launch-projected project manifest as a registry catalog content input"`);
  if (!lawPresent && law.split(LAW_ANCHOR).length !== 2) throw new Error("law anchor is not unique");
  console.log(`law: ${lawPresent ? "present" : "anchor found once"}`);
  if (!applied && check.code !== 0) throw new Error("re-derive the discovery patch before applying");
  if (!apply) return;
  const discoveryFile = join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts");
  if (!applied) {
    save(discoveryFile, readFileSync(discoveryFile, "utf8"));
    if (run(["git", "apply", DISCOVERY_PATCH]).code !== 0) throw new Error("git apply failed");
  }
  if (!lawPresent) save(LAW, law.replace(LAW_ANCHOR, `${LAW_BLOCK}${LAW_ANCHOR}`));
}

function stepTargets(): void {
  const byFile = new Map<string, Spec["targets"]>();
  for (const held of spec.targets.filter((target) => target.hold)) console.log(`target ${held.project}:${held.name}: held — ${held.hold}`);
  for (const target of spec.targets.filter((candidate) => !candidate.hold)) byFile.set(target.projectJson, [...(byFile.get(target.projectJson) ?? []), target]);
  for (const [relative, targets] of byFile) {
    const path = join(ROOT, relative);
    let text = readFileSync(path, "utf8");
    const parsed = JSON.parse(text) as { name: string; targets: Record<string, unknown> };
    const missing = targets.filter((target) => !(target.name in parsed.targets));
    for (const target of targets) if (target.project !== parsed.name) throw new Error(`${relative} is ${parsed.name}, spec says ${target.project}`);
    console.log(`${parsed.name}: ${missing.length} to add (${missing.map((target) => target.name).join(", ") || "none"}), ${targets.length - missing.length} present`);
    if (!missing.length) continue;
    const open = text.indexOf("{", text.indexOf('"targets"'));
    const end = objectEnd(text, open) - 1;
    const lastBrace = text.lastIndexOf("}", end - 1);
    const block = missing.map((target) => {
      const body = { executor: "nx:run-commands", cache: target.cache, ...(target.dependsOn ? { dependsOn: target.dependsOn } : {}), options: { cwd: dirname(relative), command: target.command, ...(target.forwardAllArgs ? { forwardAllArgs: true } : {}) }, ...(target.configurations ? { configurations: target.configurations } : {}) };
      return `    ${JSON.stringify(target.name)}: ${JSON.stringify(body, null, 2).replaceAll("\n", "\n    ")}`;
    }).join(",\n");
    text = `${text.slice(0, lastBrace + 1)},\n${block}${text.slice(lastBrace + 1)}`;
    const reparsed = JSON.parse(text) as { targets: Record<string, unknown> };
    for (const target of missing) if (!(target.name in reparsed.targets)) throw new Error(`insertion of ${target.name} failed`);
    if (apply) save(path, text);
    else writeFileSync(join(STATE, `preview-${parsed.name.replaceAll("/", "_")}.json`), text);
  }
}

/** ✏️ Exact-once textual replacements (`before` → `after`) in one file; an edit already applied is reported, a missing or
 * ambiguous `before` refuses the whole step. */
function replaceExactlyOnce(text: string, edits: { label: string; before: string; after: string }[]): string {
  for (const edit of edits) {
    if (text.split(edit.after).length === 2 && !text.includes(edit.before)) {
      console.log(`${edit.label}: already applied`);
      continue;
    }
    if (text.split(edit.before).length !== 2) throw new Error(`${edit.label}: before-text found ${text.split(edit.before).length - 1}× (need exactly 1)`);
    text = text.replace(edit.before, edit.after);
    console.log(`${edit.label}: replaced`);
  }
  return text;
}

function stepProjectEdits(): void {
  const byFile = new Map<string, NonNullable<Spec["projectJsonEdits"]>>();
  for (const edit of spec.projectJsonEdits ?? []) byFile.set(edit.projectJson, [...(byFile.get(edit.projectJson) ?? []), edit]);
  for (const [relative, edits] of byFile) {
    const path = join(ROOT, relative);
    const text = replaceExactlyOnce(readFileSync(path, "utf8"), edits.map((edit) => ({ label: `${relative.split("/").at(-4)}:${edit.target}`, before: edit.before, after: edit.after })));
    JSON.parse(text);
    if (apply) save(path, text);
  }
}

function renderRow(row: Record<string, unknown>): string {
  return JSON.stringify(row, null, 2).split("\n").map((line) => `    ${line}`).join("\n");
}

function stepSeed(): void {
  let text = readFileSync(SEED, "utf8");
  for (const { after, row } of spec.seedRows) {
    if (text.includes(`"name": ${JSON.stringify(row.name)},`)) {
      console.log(`seed row ${row.name}: present`);
      continue;
    }
    const marker = `      "name": ${JSON.stringify(after)},\n`;
    if (text.split(marker).length !== 2) throw new Error(`seed anchor ${after} not unique`);
    const start = text.indexOf(marker);
    const close = text.indexOf("\n    },\n", start) + "\n    },\n".length;
    text = `${text.slice(0, close)}${renderRow(row)},\n${text.slice(close)}`;
    console.log(`seed row ${row.name}: inserted after ${after}`);
  }
  for (const edit of spec.seedRowEdits) {
    const marker = `      "name": ${JSON.stringify(edit.name)},\n`;
    if (text.split(marker).length !== 2) throw new Error(`seed row ${edit.name} not unique`);
    const start = text.indexOf(marker);
    const commandAt = text.indexOf('      "command": ', start);
    const lineEnd = text.indexOf("\n", commandAt);
    text = `${text.slice(0, commandAt)}      "command": ${JSON.stringify(edit.command)},${text.slice(lineEnd)}`;
    console.log(`seed row ${edit.name}: command → ${edit.command}`);
  }
  for (const edit of spec.seedInputEdits) {
    const marker = `      "id": ${JSON.stringify(edit.id)},\n`;
    if (text.split(marker).length !== 2) throw new Error(`seed input ${edit.id} not unique`);
    const start = text.indexOf(marker);
    const defaultAt = text.indexOf('      "default": ', start);
    const lineEnd = text.indexOf("\n", defaultAt);
    const trailing = text.slice(defaultAt, lineEnd).trimEnd().endsWith(",") ? "," : "";
    text = `${text.slice(0, defaultAt)}      "default": ${JSON.stringify(edit.default)}${trailing}${text.slice(lineEnd)}`;
    console.log(`seed input ${edit.id}: default → ${edit.default}`);
  }
  text = replaceExactlyOnce(text, (spec.seedTextEdits ?? []).map((edit) => ({ label: `seed row ${edit.row}`, before: edit.before, after: edit.after })));
  Bun.JSONC.parse(text);
  if (apply) save(SEED, text);
  else writeFileSync(join(STATE, "preview-launch.seed.jsonc"), text);
}

async function stepRender(): Promise<void> {
  const { renderCatalogFiles } = await import(`${ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts`);
  const { generateLaunchJson, declaredProjectTargets } = await import(`${ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts`);
  const { playgrounds } = renderCatalogFiles(ROOT);
  const text: string = generateLaunchJson(ROOT, playgrounds, declaredProjectTargets(ROOT));
  const current = readFileSync(LAUNCH, "utf8");
  const parsed = Bun.JSONC.parse(text) as { configurations: { name: string }[] };
  console.log(JSON.stringify({ configurations: parsed.configurations.length, identical: text === current, lines: text.split("\n").length }));
  if (apply && text !== current) save(LAUNCH, text);
}

async function stepPlan(): Promise<void> {
  let text = readFileSync(PLAN, "utf8");
  for (const { after, step: planStep } of spec.planSteps ?? []) {
    const current = JSON.parse(text) as { steps: { id: string }[] };
    if (current.steps.some((existing) => existing.id === planStep.id)) {
      console.log(`plan step ${planStep.id}: present`);
      continue;
    }
    const afterAt = text.indexOf(`"id": ${JSON.stringify(after)},`);
    if (afterAt < 0) throw new Error(`plan step ${after} not found`);
    const stepOpen = text.lastIndexOf("\n    {", afterAt);
    const stepClose = objectEnd(text, text.indexOf("{", stepOpen));
    const checks = (planStep.checks as unknown[]).map((check) => `        ${JSON.stringify(check)}`).join(",\n");
    const { checks: _checks, ...head } = planStep;
    const rendered = `{ ${Object.entries(head).map(([key, value]) => `${JSON.stringify(key)}: ${JSON.stringify(value)}`).join(", ")}, "checks": [\n${checks}\n      ]\n    }`;
    text = `${text.slice(0, stepClose)},\n    ${rendered}${text.slice(stepClose)}`;
    console.log(`plan step ${planStep.id}: added after ${after}`);
  }
  const plan = JSON.parse(text) as { steps: { id: string; checks: { id: string }[] }[] };
  for (const { step: stepId, check, landed, hold } of spec.planChecks) {
    if (landed) continue;
    if (hold) {
      console.log(`plan check ${check.id}: held — ${hold}`);
      continue;
    }
    if (plan.steps.some((candidate) => candidate.checks.some((existing) => existing.id === check.id))) {
      console.log(`plan check ${check.id}: present`);
      continue;
    }
    const stepAt = text.indexOf(`"id": ${JSON.stringify(stepId)},`);
    if (stepAt < 0) throw new Error(`plan step ${stepId} not found`);
    const checksAt = text.indexOf('"checks": [', stepAt);
    const close = text.indexOf("\n      ]", checksAt);
    text = `${text.slice(0, close)},\n        ${JSON.stringify(check)}${text.slice(close)}`;
    console.log(`plan check ${check.id}: added to ${stepId}`);
  }
  const preview = join(STATE, "preview-plan.json");
  writeFileSync(preview, text);
  const { readGoalPlan } = await import(`${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts`);
  const valid = readGoalPlan(ROOT, preview) as { steps: { checks: unknown[] }[] };
  console.log(`plan valid under the acceptance schema: ${valid.steps.length} steps, ${valid.steps.reduce((sum, step) => sum + step.checks.length, 0)} checks`);
  if (apply) save(PLAN, text);
}

const ST2_APPLY = join(ROOT, ".tmp-ticket/wp-st2/st2-apply.py");

function stepSt2R10(): void {
  const dry = run(["python3", ST2_APPLY, "--dry-run", "--part", "r10"]);
  console.log(dry.out.trim());
  if (dry.code !== 0 || !/dry run — nothing written/u.test(dry.out)) throw new Error("st2-apply --part r10 dry run is not clean");
  const edits = [...dry.out.matchAll(/^edit\s+(.+?)\s+\+\d+ -\d+$/gmu)].map((match) => match[1]!);
  const created = [...dry.out.matchAll(/^new\s+(.+?)\s+\d+ lines$/gmu)].map((match) => match[1]!);
  const moves = [...dry.out.matchAll(/^move\s+(.+?) → (.+?)\s+\(\d+ files\)$/gmu)].map((match) => [match[1]!, match[2]!] as [string, string]);
  console.log(`st2-r10: ${edits.length} edits, ${created.length} new files, ${moves.length} moves`);
  if (!apply) return;
  for (const edit of edits) save(join(ROOT, edit), readFileSync(join(ROOT, edit), "utf8"));
  mkdirSync(backupDir, { recursive: true });
  writeFileSync(join(backupDir, ".r10-manifest.json"), JSON.stringify({ created, moves }, null, 1));
  const write = run(["python3", ST2_APPLY, "--write", "--part", "r10"]);
  console.log(write.out.trim());
  if (write.code !== 0) throw new Error("st2-apply --write --part r10 failed — run `revert st2-r10 --apply`");
}

const steps: Record<string, () => void | Promise<void>> = { "st2-r10": stepSt2R10, taxonomy: stepTaxonomy, discovery: stepDiscovery, targets: () => { stepTargets(); stepProjectEdits(); }, seed: stepSeed, render: stepRender, plan: stepPlan, revert: stepRevert };
if (!step || !steps[step]) throw new Error(`usage: bun window3-apply.ts <${Object.keys(steps).join("|")}> [--apply]`);
console.log(`[window3] ${step} ${apply ? "APPLY" : "dry run"}`);
await steps[step]();
