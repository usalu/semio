/** 🧫️ Ticket tool of work package A1 (second round): the scratch in which the split stage is proven before it lands, away from the live tree that eight other agents compile at the same time.
 *
 * The scratch lives below `🗑️generated/a1`: `before/` and `after/` each hold a copy of the pets core at the depth it
 * has in the repository (`<side>/🧰️framework/🛍️products/🐾️pets/…`: schema, modules, cases, fixtures, glue), `before`
 * as committed and `after` with the output of `split_stage.ts split`.
 *
 * - `prepare` writes what the copies need to be compiled where they are: a stand-in for the adapter module of the
 *   test harness (the cases import it by a relative path), a `tsconfig.json` per side, and the private Rust crate
 *   `crate/📦️packages/🦀️rust` that mounts the twins of `after` by `#[path]` (run it with `a1_cargo.sh`).
 * - `traces <side>` replays the thirteen committed scripts of the stage-trace fixture through the TypeScript core of
 *   a side and compares every trace with the committed one.
 * - `replay <side> <sessions.json> <digests.json>` replays the sessions `long_trace_digest.ts` recorded through the
 *   TypeScript core of a side and writes its two digests per simulated second, the way that tool does.
 * - `compare <digests.json> <digests.json>` counts the checkpoints in which two recordings differ.
 *
 * Run from the repository root: `bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/a1_scratch.ts <command> …`.
 *
 * @see ./split_stage.ts — the split and its check
 * @see ./a1_cargo.sh — cargo in the private crate
 * @see ./long_trace_digest.ts, ./long_trace_digest.rs — the long-run proof of work package M
 */
import { copyFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";

const V = "️";

/** 🏷️ A path segment the way the taxonomy writes it: one emoji, the variation selector, the slug. */
function named(emoji: string, slug: string): string {
  return `${emoji.replaceAll(V, "")}${V}${slug}`;
}

const ROOT = resolve(import.meta.dir, "../../../../../../..");
const SCRATCH = join(import.meta.dir, named("🗑", "generated"), "a1");
const FRAMEWORK = named("🧰", "framework");
const PETS = join(FRAMEWORK, named("🛍", "products"), named("🐾", "pets"));
const MODULES = named("🔨", "modules");
const TS = named("🟦", ".ts");
const RS = named("🦀", ".rs");
const PACKAGES = named("📦", "packages");
const ADAPTER = join(FRAMEWORK, MODULES, named("🧪", "test"), named("🔌", "adapter"), TS);
const OLD: readonly (readonly [string, string])[] = [
  ["validation", named("✅", "validation")],
  ["trigonometry", named("📐", "trigonometry")],
  ["randomness", named("🎲", "randomness")],
  ["rig", named("🦴", "rig")],
  ["animation", named("🎞", "animation")],
  ["terrain", named("🏞", "terrain")],
  ["behavior", named("🧠", "behavior")],
];
const PARTS: readonly (readonly [string, string])[] = [
  ["draft", named("📝", "draft")],
  ["spacing", named("📏", "spacing")],
  ["schedule", named("🗓", "schedule")],
  ["attention", named("👀", "attention")],
  ["locomotion", named("🚶", "locomotion")],
  ["sociability", named("💞", "sociability")],
  ["choice", named("🎯", "choice")],
  ["population", named("👥", "population")],
  ["clock", named("🕰", "clock")],
  ["projection", named("🎥", "projection")],
];
const FNV_OFFSET_BASIS = 2166136261;
const FNV_PRIME = 16777619;
const BYTES = new DataView(new ArrayBuffer(8));

/** 🪢️ A path with forward slashes, as TOML, JSON and Rust attributes want it. */
function slashed(path: string): string {
  return path.replaceAll("\\", "/");
}

/** 📦️ The manifest of the private crate: the dependencies and the lint tables of the real one, a workspace of its own. */
function manifest(): string {
  return [
    "# 🧪️ Private scratch crate of work package A1, written by `a1_scratch.ts prepare`: compiles the split stage from the",
    "# scratch copy `after/` (mounted by `#[path]`) without touching the root workspace. Not a member of anything.",
    "[workspace]",
    "",
    "[package]",
    'name = "semio-framework-pets-scratch-a1"',
    'version = "0.0.0"',
    'edition = "2021"',
    "publish = false",
    "",
    "[lib]",
    'name = "pets"',
    `path = "${RS}"`,
    "",
    "[[example]]",
    'name = "long_trace_digest"',
    'path = "../../../../../long_trace_digest.rs"',
    'required-features = ["sut"]',
    "",
    "[features]",
    'sut = ["dep:serde_json"]',
    "",
    "[dependencies]",
    'serde = { version = "=1.0.228", features = ["derive"] }',
    'serde_json = { version = "=1.0.149", features = ["raw_value", "float_roundtrip"], optional = true }',
    "",
    "[dev-dependencies]",
    'serde_json = { version = "=1.0.149", features = ["raw_value", "float_roundtrip"] }',
    "",
    "[lints.rust]",
    'future_incompatible = { level = "warn", priority = -1 }',
    'rust_2018_idioms = { level = "warn", priority = -1 }',
    'unsafe_op_in_unsafe_fn = "warn"',
    'unused_lifetimes = "warn"',
    'unused_qualifications = "warn"',
    "",
    "[lints.clippy]",
    'all = { level = "warn", priority = -1 }',
    'cloned_instead_of_copied = "warn"',
    'inefficient_to_string = "warn"',
    'map_unwrap_or = "warn"',
    'needless_pass_by_value = "warn"',
    'semicolon_if_nothing_returned = "warn"',
    'unnecessary_wraps = "warn"',
    'redundant_clone = "warn"',
    "",
  ].join("\n");
}

/** 🔌️ The glue of the private crate: the mounts the real glue carries after the split, pointing into `after/`. */
function glue(): string {
  const pets = `../../../after/${slashed(PETS)}`;
  return [
    "//! 📦️ Scratch glue of work package A1, written by `a1_scratch.ts prepare` — the mounts of the real package glue after the split of the stage, pointing into the scratch copy.",
    "",
    `#[path = "${pets}/${named("🧬", "schema")}/${RS}"]`,
    "pub mod schema;",
    "",
    ...OLD.flatMap(([module, directory]) => [`#[path = "${pets}/${MODULES}/${directory}/${RS}"]`, `pub mod ${module};`, ""]),
    ...PARTS.flatMap(([module, directory]) => [`#[path = "${pets}/${MODULES}/${directory}/${RS}"]`, `mod ${module};`, ""]),
    `#[path = "${pets}/${MODULES}/${named("🎪", "stage")}/${RS}"]`,
    "pub mod stage;",
    "",
    `#[path = "${pets}/${RS}"]`,
    "mod component;",
    "pub use component::*;",
    "",
    '#[cfg(feature = "sut")]',
    "pub use serde_json;",
    "",
  ].join("\n");
}

/** 🏗️ `prepare`: the stand-ins, the compiler configurations and the private crate. */
function prepare(): void {
  for (const side of ["before", "after"]) {
    const stub = join(SCRATCH, side, ADAPTER);
    mkdirSync(dirname(stub), { recursive: true });
    writeFileSync(stub, `/** 🔌️ Stand-in of the scratch: the adapter module of the test harness, from the repository. */\nexport * from "${slashed(relative(dirname(stub), join(ROOT, ADAPTER)))}";\n`);
    const home = join(SCRATCH, side, PETS, PACKAGES, named("🟦", "typescript"));
    const config = {
      extends: slashed(relative(home, join(ROOT, "tsconfig.json"))),
      compilerOptions: { incremental: false, types: ["bun"], paths: { "@semio-tech/pets": [`./${TS}`] } },
      include: [`./${TS}`, `../../${TS}`, `../../${named("🧬", "schema")}/${TS}`, `../../${MODULES}/**/*.ts`, `../../${named("🧪", "tests")}/*/${TS}`],
      exclude: ["**/node_modules/**"],
    };
    writeFileSync(join(home, "tsconfig.json"), `${JSON.stringify(config, null, 2)}\n`);
  }
  const crate = join(SCRATCH, "crate", PACKAGES, named("🦀", "rust"));
  mkdirSync(crate, { recursive: true });
  writeFileSync(join(crate, "Cargo.toml"), manifest());
  writeFileSync(join(crate, RS), glue());
  copyFileSync(join(ROOT, "Cargo.lock"), join(crate, "Cargo.lock"));
  process.stdout.write(`scratch prepared below ${SCRATCH}\n`);
}

/** 📥️ A module of the pets core of a side. */
async function core<Module>(side: string, ...segments: string[]): Promise<Module> {
  return (await import(pathToFileURL(join(SCRATCH, side, PETS, ...segments)).href)) as Module;
}

/** 🎞️ `traces`: the committed scripts of the stage-trace fixture replayed through a side. */
async function traces(side: string): Promise<void> {
  const name = named("🎪", "stage-trace");
  const adapter = await core<{ traceOf: (menagerie: unknown, script: { id: string; ticks: number; expected: unknown }) => { trace: unknown } }>(side, named("🧪", "tests"), name, TS);
  const vectors = JSON.parse(readFileSync(join(SCRATCH, side, PETS, named("🧫", "fixtures"), name, named("🔣", ".json")), "utf8")) as { menagerie: unknown; scripts: { id: string; ticks: number; expected: unknown }[] };
  let same = 0;
  let ticks = 0;
  for (const script of vectors.scripts) {
    const trace = adapter.traceOf(vectors.menagerie, script).trace;
    const equal = Bun.deepEquals(trace, script.expected, true);
    process.stdout.write(`${script.id}: ${script.ticks} ticks, ${equal ? "the committed trace" : "ANOTHER TRACE"}\n`);
    if (equal) same++;
    ticks += script.ticks;
  }
  process.stdout.write(`${side}: ${same} of ${vectors.scripts.length} scripts replay into their committed trace (${ticks} ticks)\n`);
  process.exitCode = same === vectors.scripts.length ? 0 : 1;
}

/** 🔢️ FNV-1a over the eight bytes of a double, little-endian. */
function fold(hash: number, value: number): number {
  BYTES.setFloat64(0, value, true);
  let folded = hash;
  for (let index = 0; index < 8; index++) folded = Math.imul(folded ^ BYTES.getUint8(index), FNV_PRIME) >>> 0;
  return folded;
}

type Any = any;

/** 🎬️ `replay`: the recorded sessions of the long run played through a side; two digests per simulated second, as `long_trace_digest.ts` folds them. */
async function replay(side: string, sessions: string, out: string): Promise<void> {
  const schema = await core<{ ACTIVITIES: readonly string[]; PET_MODES: readonly string[]; TICKS_PER_SECOND: number }>(side, named("🧬", "schema"), TS);
  const stage = await core<{ advance: (menagerie: Any, stage: Any, events: Any[]) => Any; frameOf: (menagerie: Any, stage: Any) => Any; openStage: (seed: number) => Any }>(side, MODULES, named("🎪", "stage"), TS);
  const recorded = JSON.parse(readFileSync(resolve(sessions), "utf8")) as { menageries: Record<string, Any>; sessions: { id: string; menagerie: string; seed: number; ticks: number; steps: { at: number; events: Any[] }[] }[] };
  const foldFrame = (hash: number, menagerie: Any, frame: Any): number => {
    let folded = fold(fold(fold(fold(hash, frame.tick), frame.rate), frame.wake === null ? -1 : frame.wake), frame.actors.length);
    for (const actor of frame.actors) {
      folded = fold(folded, menagerie.species.findIndex((species: Any) => species.id === actor.species));
      folded = fold(fold(fold(fold(fold(folded, actor.x), actor.y), actor.facing), schema.ACTIVITIES.indexOf(actor.activity)), actor.opacity);
      for (const number of actor.bones) folded = fold(folded, number);
      for (const eye of actor.eyes) folded = fold(fold(fold(folded, eye.x), eye.y), eye.lid);
      folded = fold(folded, actor.mood);
    }
    return folded;
  };
  const foldStage = (menagerie: Any, state: Any): number => {
    const kind = (id: string): number => menagerie.species.findIndex((species: Any) => species.id === id);
    const surface = (id: string): number => state.surfaces.findIndex((candidate: Any) => candidate.id === id);
    const numbers: number[] = [state.seed, state.tick, schema.PET_MODES.indexOf(state.mode), state.quiet ? 1 : 0, state.width, state.height, state.pointer === null ? 0 : 1, state.pointer === null ? 0 : state.pointer.x, state.pointer === null ? 0 : state.pointer.y, state.pointed, state.glances.length];
    for (const glance of state.glances) numbers.push(glance.x, glance.y);
    numbers.push(state.surfaces.length, state.keepouts.length, state.perches.length);
    for (const perch of state.perches) numbers.push(surface(perch.surface), perch.x0, perch.x1, perch.y);
    numbers.push(state.wanted.length);
    for (const wanted of state.wanted) numbers.push(kind(wanted));
    numbers.push(state.actors.length);
    for (const actor of state.actors) {
      const species = menagerie.species[kind(actor.species)];
      numbers.push(kind(actor.species), actor.perch === null ? -1 : surface(actor.perch), actor.x, actor.y, actor.vx, actor.vy, actor.facing, actor.faced, schema.ACTIVITIES.indexOf(actor.activity), actor.since, actor.until, actor.goal);
      numbers.push(actor.partner === null ? -1 : kind(actor.partner), actor.clip === null || species === undefined ? -1 : species.clips.findIndex((clip: Any) => clip.id === actor.clip));
      numbers.push(actor.gaze.x, actor.gaze.y, actor.gaze.vx, actor.gaze.vy, actor.blink, actor.mood, actor.needs.energy, actor.needs.sociability, actor.needs.curiosity, actor.opacity, actor.leaving ? 1 : 0, actor.draws);
    }
    numbers.push(state.rapports.length);
    for (const rapport of state.rapports) numbers.push(kind(rapport.between[0]), kind(rapport.between[1]), rapport.drift);
    numbers.push(state.met, state.draws);
    let folded = FNV_OFFSET_BASIS;
    for (const number of numbers) folded = fold(folded, number);
    return folded;
  };
  const digests: Record<string, number[]> = {};
  let checkpoints = 0;
  let ticks = 0;
  for (const session of recorded.sessions) {
    const menagerie = recorded.menageries[session.menagerie];
    const steps = new Map(session.steps.map((step) => [step.at, step.events]));
    const folded: number[] = [];
    let state = stage.openStage(session.seed);
    let hash = FNV_OFFSET_BASIS;
    for (let tick = 0; tick < session.ticks; tick++) {
      const events = steps.get(tick);
      if (events !== undefined) state = stage.advance(menagerie, state, events);
      state = stage.advance(menagerie, state, [{ kind: "ticked", ticks: 1 }]);
      hash = foldFrame(hash, menagerie, stage.frameOf(menagerie, state));
      if ((tick + 1) % schema.TICKS_PER_SECOND === 0) folded.push(hash, foldStage(menagerie, state));
    }
    digests[session.id] = folded;
    checkpoints += folded.length / 2;
    ticks += session.ticks;
  }
  mkdirSync(dirname(resolve(out)), { recursive: true });
  writeFileSync(resolve(out), JSON.stringify(digests));
  process.stdout.write(`${side}: ${recorded.sessions.length} sessions, ${ticks} ticks, ${checkpoints} checkpoints (two digests each) written to ${out}\n`);
}

/** ⚖️ `compare`: the checkpoints in which two recordings of digests differ. */
function compare(left: string, right: string): void {
  const one = JSON.parse(readFileSync(resolve(left), "utf8")) as Record<string, number[]>;
  const other = JSON.parse(readFileSync(resolve(right), "utf8")) as Record<string, number[]>;
  let checkpoints = 0;
  let mismatches = 0;
  let first = "none";
  for (const id of new Set([...Object.keys(one), ...Object.keys(other)])) {
    const mine = one[id] ?? [];
    const theirs = other[id] ?? [];
    const length = Math.max(mine.length, theirs.length);
    for (let at = 0; at < length; at += 2) {
      checkpoints++;
      if (mine[at] === theirs[at] && mine[at + 1] === theirs[at + 1]) continue;
      mismatches++;
      if (first === "none") first = `${id} second ${at / 2 + 1}`;
    }
  }
  process.stdout.write(`${Object.keys(one).length} and ${Object.keys(other).length} sessions, ${checkpoints} checkpoints compared (two digests each), ${mismatches} mismatches; first mismatch: ${first}\n`);
  process.exitCode = mismatches === 0 && checkpoints > 0 ? 0 : 1;
}

const [command, ...rest] = process.argv.slice(2);
if (command === "prepare") prepare();
else if (command === "traces") await traces(rest[0] ?? "after");
else if (command === "replay") await replay(rest[0]!, rest[1]!, rest[2]!);
else if (command === "compare") compare(rest[0]!, rest[1]!);
else throw new Error("usage: a1_scratch.ts prepare | traces <side> | replay <side> <sessions.json> <digests.json> | compare <digests.json> <digests.json>");
