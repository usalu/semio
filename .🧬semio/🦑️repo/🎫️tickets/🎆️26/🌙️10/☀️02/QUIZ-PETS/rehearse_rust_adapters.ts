#!/usr/bin/env bun
/** 🎭️ Ticket tool of work package K: rehearses the Rust subject adapters of the five K cases before the Rust package exists.
 *
 * It writes a Protocol v2 plan per case, runs the scratch host (`🗑️generated/wp-k/host`, built by
 * `SCRATCH_CRATE=host bash rust_scratch.sh wp-k build --offline --features sut`) against it exactly as the harness
 * runs a generated host (`host --plan … --out …`), calls the committed TypeScript adapter of the same case in this
 * process, and compares the two projections scenario by scenario: structure, strings, booleans and every number
 * exactly (`0` and `-0` count as equal, as in JSON). From the repository root:
 *
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/rehearse_rust_adapters.ts [adapters directory]
 *
 * The adapters directory defaults to the drafts (`🗑️generated/wp-k/adapters`); the host binary is the one built last.
 */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const HERE = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(HERE, "../../../../../../..");
const OWNER = "🧰️framework/🛍️products/🐾️pets";
const CASES = ["📐️turn-trigonometry", "🎲️counter-randomness", "🦴️rig-solving", "👀️gaze-tracking", "🧬️schema-conformance"];
const HOST = resolve(HERE, "🗑️generated/wp-k/target/debug/host.exe");

type Scenario = { readonly subject: (ctx: { fixtureBytes(uri: string): Uint8Array }) => { readonly projection: unknown } | Promise<{ readonly projection: unknown }> };
type TestAdapter = { readonly implementation: string; readonly scenarios: Readonly<Record<string, Scenario>> };

/** 🧫️ The bytes of a `shared://` fixture of the owner. */
function fixtureBytes(uri: string): Uint8Array {
  return readFileSync(resolve(ROOT, OWNER, "🧫️fixtures", uri.replace("shared://", "")));
}

/** 🔍️ The first place two projections differ, or `undefined` when they are the same. */
function difference(left: unknown, right: unknown, path: string): string | undefined {
  if (typeof left === "number" && typeof right === "number") return left === right ? undefined : `${path}: ${left} ≠ ${right}`;
  if (Array.isArray(left) && Array.isArray(right)) {
    if (left.length !== right.length) return `${path}: ${left.length} ≠ ${right.length} entries`;
    for (let index = 0; index < left.length; index++) {
      const found = difference(left[index], right[index], `${path}/${index}`);
      if (found !== undefined) return found;
    }
    return undefined;
  }
  if (left !== null && right !== null && typeof left === "object" && typeof right === "object" && !Array.isArray(left) && !Array.isArray(right)) {
    const keys = Object.keys(left).sort();
    const others = Object.keys(right).sort();
    if (keys.join("\u0000") !== others.join("\u0000")) return `${path}: keys ${keys.length} ≠ ${others.length}`;
    for (const key of keys) {
      const found = difference((left as Record<string, unknown>)[key], (right as Record<string, unknown>)[key], `${path}/${key}`);
      if (found !== undefined) return found;
    }
    return undefined;
  }
  return left === right ? undefined : `${path}: ${JSON.stringify(left)} ≠ ${JSON.stringify(right)}`;
}

/** 🔢️ How many leaves (numbers, strings, booleans, nulls) a projection holds. */
function leaves(value: unknown): number {
  if (Array.isArray(value)) return value.reduce<number>((sum, entry) => sum + leaves(entry), 0);
  if (value !== null && typeof value === "object") return Object.values(value).reduce<number>((sum, entry) => sum + leaves(entry), 0);
  return 1;
}

if (!existsSync(HOST)) throw new Error(`no rehearsal host at ${HOST}; build it first`);
let scenarios = 0;
let different = 0;
let compared = 0;
for (const name of CASES) {
  const slug = [...name].slice(2).join("");
  const adapter = ((await import(pathToFileURL(resolve(ROOT, OWNER, "🧪️tests", name, "🟦️.ts")).href)) as { readonly default: TestAdapter }).default;
  const out = resolve(HERE, "🗑️generated/wp-k/rehearsal", slug);
  mkdirSync(resolve(out, "results"), { recursive: true });
  const plan = {
    schemaVersion: 2,
    owner: OWNER,
    case: name,
    role: "subject",
    implementation: "rust",
    platform: "win32-x64",
    level: "exhaustive",
    workDir: resolve(out, "work"),
    outputDir: resolve(out, "results"),
    artifactDir: resolve(out, "results", "📦️artifacts"),
    fixtures: [{ uri: `shared://${name}/🔣️.json`, scope: "shared", name: `${name}/🔣️.json`, path: `${OWNER}/🧫️fixtures/${name}/🔣️.json`, digest: "" }],
    scenarios: Object.keys(adapter.scenarios).map((id) => ({ id, name: id, level: "fundamental", mode: "differential", seed: "", outlineOf: "", steps: [{ keyword: "Given", text: `the committed vectors shared://${name}/🔣️.json` }] })),
  };
  writeFileSync(resolve(out, "📋️plan.json"), JSON.stringify(plan, null, 1));
  const run = spawnSync(HOST, ["--plan", resolve(out, "📋️plan.json"), "--out", resolve(out, "results", "📤️results.jsonl")], { env: { ...process.env, PETS_CASE: slug }, encoding: "utf8" });
  if (run.status !== 0) process.stdout.write(`${name}: the host exited with ${run.status} ${run.stderr.trim()}\n`);
  const records = readFileSync(resolve(out, "results", "📤️results.jsonl"), "utf8")
    .split("\n")
    .filter((line) => line.length > 0)
    .map((line) => JSON.parse(line) as { readonly scenario: string; readonly status: string; readonly output: { readonly projection: unknown }; readonly diagnostics: readonly { readonly message: string }[] });
  for (const [id, scenario] of Object.entries(adapter.scenarios)) {
    scenarios += 1;
    const record = records.find((candidate) => candidate.scenario === id);
    const expected = JSON.parse(JSON.stringify((await scenario.subject({ fixtureBytes })).projection)) as unknown;
    const found = record === undefined ? "no result" : record.status !== "passed" ? `${record.status}: ${record.diagnostics.map((entry) => entry.message).join("; ")}` : difference(record.output.projection, expected, "");
    if (found !== undefined) different += 1;
    else compared += leaves(expected);
    process.stdout.write(`${name} ${id}: ${found === undefined ? `equal (${leaves(expected)} values)` : `DIFFERENT ${found}`}\n`);
  }
}
process.stdout.write(`[rehearsal] cases=${CASES.length} scenarios=${scenarios} different=${different} values=${compared}\n`);
process.exit(different === 0 ? 0 : 1);
