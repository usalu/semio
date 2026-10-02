/** 🎭️ Ticket tool of work package M: holds the projections of the Rust adapters of the behaviour and stage cases to the TypeScript adapters' number for number, without a tolerance.
 *
 * The harness compares `🧠️behavior-choice` and `🤝️bond-dynamics` under the profile `pets-float-v1` (1e-9); this tool
 * compares the same projections exactly, which is what design §2.4 asks of the twins. `harness` (the default) reads
 * what the real Rust subject projected in the last parity run (`⚡️cache/tests/results/…-subject-rust`); `scratch` runs
 * the adapters through the scratch crate's host stand-ins (`hosts/<case>.rs`, written by `stage_scratch.ts prepare`)
 * on a copy of the plan the harness last wrote for the TypeScript subject. Either way every scenario's projection
 * file is compared with what the TypeScript adapter answers right now for the same fixture. Both sides pass through
 * JSON text, like in the harness, so `-0` and `0` are one number and a NaN would be `null`.
 *
 * Run from the repository root, after a parity run of the three cases:
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/rehearse_stage_adapters.ts [harness|scratch]
 *
 * @see ./rehearse_motion_adapters.ts — the precedent of work package L
 * @see ./rust_scratch.sh — the runner of the scratch crate
 */
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

type Scenario = { readonly subject?: (ctx: { fixtureBytes(uri: string): Uint8Array }) => { projection: unknown } | Promise<{ projection: unknown }> };
type Adapter = { readonly scenarios: Readonly<Record<string, Scenario>> };

const ROOT = resolve(import.meta.dir, "../../../../../../..");
const OWNER = "🧰️framework/🛍️products/🐾️pets";
const CACHE = join(ROOT, ".🧬semio/🦑️repo/⚡️cache/tests");
const CASES: readonly (readonly [string, string])[] = [["🧠️behavior-choice", "behavior_choice"], ["🤝️bond-dynamics", "bond_dynamics"], ["🎪️stage-trace", "stage_trace"]];
const MODE = process.argv[2] ?? "harness";
if (MODE !== "scratch" && MODE !== "harness") throw new Error(`unknown mode ${MODE}: harness or scratch`);

/** 🧮️ Counts the numbers of a projection and collects where two projections differ; numbers must be the same double. */
function differences(path: string, left: unknown, right: unknown, found: string[], tally: { numbers: number }): void {
  if (typeof left === "number" && typeof right === "number") {
    tally.numbers += 1;
    if (!Object.is(left, right)) found.push(`${path}: typescript ${left} rust ${right}`);
  } else if (Array.isArray(left) && Array.isArray(right)) {
    if (left.length !== right.length) found.push(`${path}: lengths ${left.length} and ${right.length}`);
    for (let index = 0; index < Math.min(left.length, right.length); index++) differences(`${path}[${index}]`, left[index], right[index], found, tally);
  } else if (left !== null && right !== null && typeof left === "object" && typeof right === "object" && !Array.isArray(left) && !Array.isArray(right)) {
    const keys = [...new Set([...Object.keys(left), ...Object.keys(right)])].sort();
    for (const key of keys) {
      if (!(key in left) || !(key in right)) found.push(`${path}.${key}: only on one side`);
      else differences(`${path}.${key}`, (left as Record<string, unknown>)[key], (right as Record<string, unknown>)[key], found, tally);
    }
  } else if (left !== right) found.push(`${path}: typescript ${JSON.stringify(left)} rust ${JSON.stringify(right)}`);
}

/** 🗂️ The harness directory of one case, role and implementation under `work` or `results`. */
function cached(kind: "work" | "results", name: string, suffix: string): string {
  const entry = readdirSync(join(CACHE, kind)).find((candidate) => candidate.endsWith(`-${name}-${suffix}`));
  if (entry === undefined) throw new Error(`the harness cache holds no ${kind} of ${name} ${suffix}: run its parity once`);
  return join(CACHE, kind, entry);
}

/** 🎬️ Runs the scratch host of one case on a copy of the TypeScript subject's plan and answers where it projected to. */
function rehearsed(name: string, example: string, work: string, lines: string[]): string {
  const directory = join(work, name);
  rmSync(directory, { recursive: true, force: true });
  mkdirSync(join(directory, "out"), { recursive: true });
  const plan = JSON.parse(readFileSync(join(cached("work", name, "subject-typescript"), "📋️plan.json"), "utf8")) as Record<string, unknown>;
  const planPath = join(directory, "plan.json");
  const outPath = join(directory, "results.jsonl");
  writeFileSync(planPath, JSON.stringify({ ...plan, implementation: "rust", role: "subject", workDir: join(directory, "work"), outputDir: join(directory, "out"), artifactDir: join(directory, "artifacts"), resultsPath: outPath }, null, 2));
  const run = Bun.spawnSync(["bash", join(import.meta.dir, "rust_scratch.sh"), "wp-m", "run", "--quiet", "--offline", "--features", "sut", "--example", example, "--", "--plan", planPath, "--out", outPath], { cwd: ROOT, stdout: "pipe", stderr: "pipe" });
  if (run.exitCode !== 0) lines.push(`${name}: the Rust host exited with ${run.exitCode}: ${run.stderr.toString().trim().split("\n").slice(-6).join(" | ")}`);
  return join(directory, "out");
}

const work = join(import.meta.dir, "🗑️generated", "wp-m", "rehearsal");
mkdirSync(work, { recursive: true });
const lines: string[] = [];
let failed = false;
let numbers = 0;
let scenarios = 0;
for (const [name, example] of CASES) {
  const projections = MODE === "scratch" ? rehearsed(name, example, work, lines) : cached("results", name, "subject-rust");
  const plan = JSON.parse(readFileSync(join(cached("work", name, "subject-typescript"), "📋️plan.json"), "utf8")) as { scenarios: { id: string }[] };
  const adapter = ((await import(join(ROOT, OWNER, "🧪️tests", name, "🟦️.ts"))) as { default: Adapter }).default;
  const ctx = {
    fixtureBytes(uri: string): Uint8Array {
      if (!uri.startsWith("shared://")) throw new Error(`unexpected fixture ${uri}`);
      return readFileSync(join(ROOT, OWNER, "🧫️fixtures", uri.slice("shared://".length)));
    },
  };
  for (const scenario of plan.scenarios) {
    const subject = adapter.scenarios[scenario.id]?.subject;
    if (subject === undefined) throw new Error(`${name}: the TypeScript adapter has no subject for ${scenario.id}`);
    const file = join(projections, `${scenario.id}.subject.projection.json`);
    const found: string[] = [];
    const tally = { numbers: 0 };
    try {
      const expected = JSON.parse(JSON.stringify((await subject(ctx)).projection)) as unknown;
      if (!existsSync(file)) found.push("the Rust adapter projected nothing");
      else differences("$", expected, JSON.parse(readFileSync(file, "utf8")), found, tally);
    } catch (error) {
      found.push(`the TypeScript adapter threw: ${error instanceof Error ? error.message : String(error)}`);
    }
    if (found.length > 0) failed = true;
    numbers += tally.numbers;
    scenarios += 1;
    lines.push(`${name}/${scenario.id}: ${tally.numbers} numbers, ${found.length} differences${found.length > 0 ? `\n  ${found.slice(0, 8).join("\n  ")}` : ""}`);
  }
}
lines.push(`${MODE}: ${CASES.length} cases, ${scenarios} scenarios, ${numbers} numbers, ${failed ? "DIFFERENCES" : "0 differences"}`);
writeFileSync(join(work, `rehearsal-${MODE}.txt`), `${lines.join("\n")}\n`);
console.log(lines.map((line) => `[DEBUG] ${line}`).join("\n"));
if (failed || lines.some((line) => line.includes("exited with"))) process.exit(1);
