/** 🧰️ Ticket tool of work package D3: (re)creates the private scratch crate the Rust twins of feeling, effects and mischief are compiled in before they are mounted in the real crate, and proves with deliberately broken twins that the bit-exactness check of `d3_check_bits.rs` notices.
 *
 * `prepare` writes `🗑️generated/d3/crate/📦️packages/🦀️rust/{Cargo.toml, 🦀️.rs, Cargo.lock, hosts/<case>.rs, copies/…,
 * standins/…}`: the crate mounts the schema, trigonometry, randomness and the three twins (with their unit suites) by
 * `#[path]`, and the siblings' twins they import (behaviour, gesture, terrain) as copies without their unit suites,
 * which their owners may be editing; it restates the workspace lint tables, links serde_json as the real crate does
 * (`float_roundtrip`), carries `d3_check_bits.rs` of this folder as the integration test `d3_bits` and one host
 * stand-in per case as an example (`required-features = ["sut"]`). `Toss` of `🧗️climbing` (work package D1) is a
 * stand-in of the TypeScript shape, or the copied climbing and swing twins with `D3_REAL_CLIMBING=1`. Everything below
 * `🗑️generated` may be deleted; this command brings it back.
 *
 * `mutants` copies the three twins, breaks one expression in each copy (a fused multiply-add, a reassociated sum or
 * product, a native `f64::min`, a sign, a shift), runs the bit check against each copy and fails unless every mutant
 * is caught. Run `d3_dump_bits.ts` before it.
 *
 * Run from the repository root:
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/d3_scratch.ts prepare
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/d3_scratch.ts mutants
 *
 * @see ./rust_scratch.sh — the runner of a scratch crate (work package K)
 * @see ./d3_dump_bits.ts, ./d3_check_bits.rs — the bit-exactness tool
 * @see ./motion_scratch.ts — the pattern (work package L)
 */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const ROOT = resolve(import.meta.dir, "../../../../../../..").replaceAll("\\", "/");
const PETS = "🧰️framework/🛍️products/🐾️pets";
const UP = "../../../../../../../../../../../..";
const SCRATCH = join(import.meta.dir, "🗑️generated", "d3");
const CRATE = join(SCRATCH, "crate", "📦️packages", "🦀️rust");
const CASES: readonly (readonly [string, string])[] = [
  ["feeling_dynamics", "💗️feeling-dynamics"],
  ["chemistry_rules", "⚗️chemistry-rules"],
  ["particle_motion", "✨️particle-motion"],
  ["mischief_choice", "🪄️mischief-choice"],
];
const TWINS = { feeling: "💗️feeling", effects: "✨️effects", mischief: "🪄️mischief" } as const;
type Twin = keyof typeof TWINS;
const TEST_ATTACH = '#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n';

type Mutant = { readonly name: string; readonly twin: Twin; readonly edits: readonly (readonly [string, string])[] };
const MUTANTS: readonly Mutant[] = [
  { name: "feeling_drowsy_rewritten", twin: "feeling", edits: [["let tired = clamp((DROWSY_ENERGY - energy) / DROWSY_ENERGY, 0.0, 1.0);", "let tired = clamp(1.0 - energy / DROWSY_ENERGY, 0.0, 1.0);"]] },
  { name: "feeling_lerp_face", twin: "feeling", edits: [["calm + (table[mood_index(feeling.mood)] - calm) * feeling.intensity", "calm * (1.0 - feeling.intensity) + table[mood_index(feeling.mood)] * feeling.intensity"]] },
  { name: "feeling_reassociated_shares", twin: "feeling", edits: [["(1.0 + (MOOD_SHARES[one][index] - 1.0) * first.intensity) * (1.0 + (MOOD_SHARES[other][index] - 1.0) * second.intensity)", "1.0 + (MOOD_SHARES[one][index] - 1.0) * first.intensity + (MOOD_SHARES[other][index] - 1.0) * second.intensity + (MOOD_SHARES[one][index] - 1.0) * first.intensity * ((MOOD_SHARES[other][index] - 1.0) * second.intensity)"]] },
  { name: "feeling_gain_reassociated", twin: "feeling", edits: [["let gain = MOOD_SPREADS[mood_index(theirs.mood)] * sociability * (0.5 + 0.5 * affinity);", "let gain = MOOD_SPREADS[mood_index(theirs.mood)] * (sociability * (0.5 + 0.5 * affinity));"]] },
  { name: "feeling_gap_fused", twin: "feeling", edits: [["wide * wide + tall * tall <= reach * reach", "wide.mul_add(wide, tall * tall) <= reach * reach"]] },
  { name: "feeling_nan_amount", twin: "feeling", edits: [["    if !(amount > 0.0) {\n        return feeling;\n    }\n    if mood == feeling.mood {", "    if amount <= 0.0 {\n        return feeling;\n    }\n    if mood == feeling.mood {"]] },
  { name: "feeling_calm_wait", twin: "feeling", edits: [["let rate = if feeling.mood == resting { MOOD_DECAYS[mood_index(feeling.mood)] * PRONE_DECAY } else { MOOD_DECAYS[mood_index(feeling.mood)] };", "let rate = MOOD_DECAYS[mood_index(feeling.mood)];"]] },
  { name: "effects_shift_15_16", twin: "effects", edits: [["let second = (first ^ (first >> 15)).wrapping_mul(0x846c_a68b);", "let second = (first ^ (first >> 16)).wrapping_mul(0x846c_a68b);"]] },
  { name: "effects_fused_fall", twin: "effects", edits: [["y: source.origin.y + pace * sin_turns(heading) * seconds,\n        scale: 0.8", "y: (pace * sin_turns(heading)).mul_add(seconds, source.origin.y),\n        scale: 0.8"]] },
  { name: "effects_reassociated_burst_reach", twin: "effects", edits: [["x: source.origin.x + source.facing * (pace * cos_turns(heading) * reach),", "x: source.origin.x + source.facing * (pace * (cos_turns(heading) * reach)),"]] },
  { name: "effects_orbit_radius", twin: "effects", edits: [["let radius = (source.emission.speed * (source.life as f64 / TICKS_PER_SECOND as f64)) / TURN_RADIANS;", "let radius = (source.emission.speed * (source.life as f64 / TICKS_PER_SECOND as f64)) * (1.0 / TURN_RADIANS);"]] },
  { name: "effects_negative_zero", twin: "effects", edits: [["rotation: source.facing * (heading - DOWN),", "rotation: if source.facing > 0.0 { heading - DOWN } else { DOWN - heading },"]] },
  { name: "mischief_native_minimum", twin: "mischief", edits: [["if perch.x0 < left && left - smaller(perch.x1, left) <= STATION_GAP {", "if perch.x0 < left && left - perch.x1.min(left) <= STATION_GAP {"]] },
  { name: "mischief_toward_negation", twin: "mischief", edits: [["Facing::Left => 0.0 - value,", "Facing::Left => -value,"]] },
  { name: "mischief_reassociated_travel", twin: "mischief", edits: [["let travel = larger(0.0, room) * (LIFT_LEAST + (1.0 - LIFT_LEAST) * unit);", "let travel = larger(0.0, room) * LIFT_LEAST + larger(0.0, room) * (1.0 - LIFT_LEAST) * unit;"]] },
];

/** 📦️ The manifest of the scratch crate. */
function manifest(): string {
  const lints = ["[lints.rust]", 'future_incompatible = { level = "warn", priority = -1 }', 'rust_2018_idioms = { level = "warn", priority = -1 }', 'unsafe_op_in_unsafe_fn = "warn"', 'unused_lifetimes = "warn"', 'unused_qualifications = "warn"', "", "[lints.clippy]", 'all = { level = "warn", priority = -1 }', 'cloned_instead_of_copied = "warn"', 'inefficient_to_string = "warn"', 'map_unwrap_or = "warn"', 'needless_pass_by_value = "warn"', 'semicolon_if_nothing_returned = "warn"', 'unnecessary_wraps = "warn"', 'redundant_clone = "warn"'];
  return [
    "# 🧪️ Private scratch crate of work package D3, written by `d3_scratch.ts prepare`: compiles the pets Rust twins from the",
    "# repository files (mounted by `#[path]`) without touching the root workspace. Not a member of anything.",
    "[workspace]",
    "",
    "[package]",
    'name = "semio-framework-pets-scratch-d3"',
    'version = "0.0.0"',
    'edition = "2021"',
    "publish = false",
    "",
    "[lib]",
    'name = "pets"',
    'path = "🦀️.rs"',
    "",
    ...(existsSync(join(import.meta.dir, "d3_check_bits.rs")) ? ["[[test]]", 'name = "d3_bits"', 'path = "../../../../../d3_check_bits.rs"', ""] : []),
    ...written().flatMap(([example]) => ["[[example]]", `name = "${example}"`, `path = "hosts/${example}.rs"`, 'required-features = ["sut"]', ""]),
    "[features]",
    'sut = ["dep:serde_json"]',
    ...Object.keys(TWINS).map((twin) => `mutant-${twin} = []`),
    "",
    "[dependencies]",
    'serde = { version = "=1.0.228", features = ["derive"] }',
    'serde_json = { version = "=1.0.149", features = ["raw_value", "float_roundtrip"], optional = true }',
    "",
    "[dev-dependencies]",
    'serde_json = { version = "=1.0.149", features = ["raw_value", "float_roundtrip"] }',
    `semio-repo-test-host = { path = "${UP}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust" }`,
    "",
    ...lints,
    "",
  ].join("\n");
}

/** 📝️ The cases whose Rust adapter is written. */
function written(): (readonly [string, string])[] {
  return CASES.filter(([, name]) => existsSync(join(ROOT, PETS, "🧪️tests", name, "🦀️.rs")));
}

/** 📄️ Whether the twin of a module is written. */
function twinWritten(twin: Twin): boolean {
  return existsSync(join(ROOT, PETS, "🔨️modules", TWINS[twin], "🦀️.rs"));
}

/** 🔍️ Whether a repository twin exists and declares a type. */
function declares(module: string, type: string): boolean {
  const path = join(ROOT, PETS, "🔨️modules", module, "🦀️.rs");
  return existsSync(path) && new RegExp(`pub struct ${type}\\b`).test(readFileSync(path, "utf8"));
}

/** 📑️ A sibling's twin as it stands now, without its unit suite (which may be mid-edit by its owner): copied to `copies/<module>.rs` and mounted from there. */
function copied(module: string, path: string): string {
  const source = readFileSync(join(ROOT, PETS, "🔨️modules", path, "🦀️.rs"), "utf8");
  writeFileSync(join(CRATE, "copies", `${module}.rs`), source.split(TEST_ATTACH).join(""));
  return `#[path = "copies/${module}.rs"]\npub mod ${module};\n`;
}

/** 🧩️ The siblings' twins the three twins import (as test-free copies), and a stand-in of what a sibling has not written yet. */
function standins(): string[] {
  const mounts: string[] = [copied("behavior", "🧠️behavior"), copied("gesture", "👆️gesture"), copied("terrain", "🏞️terrain")];
  if (process.env.D3_REAL_CLIMBING === "1" && declares("🧗️climbing", "Toss")) {
    for (const [module, path] of [["rig", "🦴️rig"], ["animation", "🎞️animation"], ["swing", "🪢️swing"], ["climbing", "🧗️climbing"]] as const) mounts.push(copied(module, path));
  } else {
    writeFileSync(
      join(CRATE, "standins", "climbing.rs"),
      [
        "//! 🧗️ Scratch stand-in of D3: `Toss` in the shape of the TypeScript twin, until work package D1 writes the climbing twin.",
        "use serde::{Deserialize, Serialize};",
        "",
        "/// 🏌️ A velocity with which an actor is thrown into the air (stand-in).",
        "#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]",
        "#[serde(deny_unknown_fields)]",
        "pub struct Toss {",
        "    pub vx: f64,",
        "    pub vy: f64,",
        "}",
        "",
      ].join("\n"),
    );
    mounts.push('#[path = "standins/climbing.rs"]\npub mod climbing;\n');
  }
  return mounts;
}

/** 🔌️ The glue of the scratch crate: the mounts the three twins need, with a mutant in place of a twin when its feature is on. */
function glue(): string {
  const mount = (module: string, path: string): string => `#[path = "${UP}/${PETS}/${path}/🦀️.rs"]\npub mod ${module};\n`;
  const swappable = (module: Twin): string => `#[cfg_attr(not(feature = "mutant-${module}"), path = "${UP}/${PETS}/🔨️modules/${TWINS[module]}/🦀️.rs")]\n#[cfg_attr(feature = "mutant-${module}", path = "mutants/current_${module}.rs")]\npub mod ${module};\n`;
  return [
    "//! 📦️ Scratch glue of work package D3, written by `d3_scratch.ts prepare` — the mounts the real package glue carries for the feeling, effects and mischief twins and what they and their unit suites import. The `mutant-…` features swap in a deliberately broken copy.\n",
    mount("schema", "🧬️schema"),
    mount("trigonometry", "🔨️modules/📐️trigonometry"),
    mount("randomness", "🔨️modules/🎲️randomness"),
    ...standins(),
    ...(Object.keys(TWINS) as Twin[]).filter(twinWritten).map(swappable),
    ["behavior", "climbing", ...(Object.keys(TWINS) as Twin[]).filter(twinWritten), "gesture", "randomness", "schema", "trigonometry"].map((module) => `pub use ${module}::*;\n`).join(""),
    '#[cfg(feature = "sut")]\npub use serde_json;\n',
  ].join("\n");
}

/** 🏗️ Writes the scratch crate. */
function prepare(): void {
  for (const folder of ["hosts", "mutants", "standins", "copies"]) mkdirSync(join(CRATE, folder), { recursive: true });
  writeFileSync(join(CRATE, "Cargo.toml"), manifest());
  writeFileSync(join(CRATE, "🦀️.rs"), glue());
  writeFileSync(join(CRATE, "Cargo.lock"), readFileSync(join(ROOT, "Cargo.lock")));
  for (const [twin] of Object.entries(TWINS)) if (!existsSync(join(CRATE, "mutants", `current_${twin}.rs`))) writeFileSync(join(CRATE, "mutants", `current_${twin}.rs`), "");
  for (const [example, name] of written()) writeFileSync(join(CRATE, "hosts", `${example}.rs`), `// Stand-in of the entrypoint the harness generates for a Rust host.\n#[path = "${ROOT}/${PETS}/🧪️tests/${name}/🦀️.rs"]\nmod adapter;\n\nfn main() -> std::process::ExitCode {\n    semio_repo_test_host::run_main(adapter::adapter())\n}\n`);
  console.log(`[DEBUG] scratch crate written to ${CRATE} (climbing ${process.env.D3_REAL_CLIMBING === "1" && declares("🧗️climbing", "Toss") ? "copied" : "stand-in"})`);
}

/** 🧟️ Writes every mutant, runs the bit check against it and answers how many cases caught it. */
function mutants(): void {
  prepare();
  const lines: string[] = [];
  let survived = 0;
  for (const mutant of MUTANTS) {
    let source = readFileSync(join(ROOT, PETS, "🔨️modules", TWINS[mutant.twin], "🦀️.rs"), "utf8");
    for (const [from, to] of [[TEST_ATTACH, ""] as const, ...mutant.edits]) {
      if (source.split(from).length !== 2) throw new Error(`${mutant.name}: the twin does not contain exactly once: ${from}`);
      source = source.replace(from, () => to);
    }
    writeFileSync(join(CRATE, "mutants", `${mutant.name}.rs`), source);
    writeFileSync(join(CRATE, "mutants", `current_${mutant.twin}.rs`), source);
    const run = Bun.spawnSync(["bash", join(import.meta.dir, "rust_scratch.sh"), "d3", "test", "--offline", "--release", "--features", `mutant-${mutant.twin}`, "--test", "d3_bits", "--", "--nocapture"], { cwd: ROOT, stdout: "pipe", stderr: "pipe" });
    const output = run.stdout.toString();
    const total = /^total: (\d+) cases, \d+ result numbers, \d+ throwing, (\d+) mismatches$/m.exec(output);
    const caught = total === null ? -1 : Number(total[2]);
    if (caught <= 0) survived += 1;
    const where = [...output.matchAll(/^(\w+): \d+ cases, \d+ result numbers, \d+ throwing, ([1-9]\d*) mismatches$/gm)].filter((match) => match[1] !== "total").map((match) => `${match[1]} ${match[2]}`);
    lines.push(`${mutant.name}: ${caught < 0 ? `the check did not run (${run.stderr.toString().trim().split("\n").slice(-3).join(" | ")})` : `${caught} of ${total![1]} cases differ`}${where.length > 0 ? ` (${where.join(", ")})` : ""}`);
  }
  lines.push(`${MUTANTS.length} mutants, ${survived} survived`);
  writeFileSync(join(SCRATCH, "mutants.txt"), `${lines.join("\n")}\n`);
  console.log(lines.map((line) => `[DEBUG] ${line}`).join("\n"));
  if (survived > 0) process.exit(1);
}

const command = process.argv[2];
if (command === "prepare") prepare();
else if (command === "mutants") mutants();
else {
  console.error("usage: bun d3_scratch.ts prepare | mutants");
  process.exit(2);
}
