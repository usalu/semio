/** 🧰️ Ticket tool of work package M: (re)creates the private scratch crate the behaviour and stage twins are compiled and tested in, away from the root workspace and its shared build directory, and proves with deliberately broken twins that the long-run comparison notices.
 *
 * `prepare` writes `🗑️generated/wp-m/crate/📦️packages/🦀️rust/{Cargo.toml, 🦀️.rs, hosts/<case>.rs, Cargo.lock}`: the
 * crate mounts the repository's twins by `#[path]` under the module names of the real glue, restates the workspace
 * lint tables, links serde_json as the real crate does (`float_roundtrip`), carries `long_trace_digest.rs` of this
 * folder as an example (the Rust half of the long-run bit-exactness proof) and one host stand-in per case of this
 * work package. Everything below `🗑️generated` may be deleted; this command brings it back.
 *
 * `mutants` copies the behaviour and the stage twin, breaks one expression in each copy (a fused multiply-add, a
 * reassociated product or quotient, a constant one unit in the last place off, two words of a draw swapped, a span
 * one tick long, a rule of the pointer or of spreading out bent), replays the recorded sessions of `long_trace_digest.ts` through each copy and fails unless every
 * mutant yields other digests than the TypeScript twin — except the mutants marked `survives`, which record what the
 * long run cannot see (an equivalent mutant, and quantities that only weigh a draw) and must survive. Run
 * `long_trace_digest.ts` before it.
 *
 * Run from the repository root:
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/stage_scratch.ts prepare
 *   bash .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/rust_scratch.sh wp-m test --offline
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/stage_scratch.ts mutants
 *
 * @see ./rust_scratch.sh — the runner of a scratch crate (work package K)
 * @see ./long_trace_digest.ts, ./long_trace_digest.rs — the long-run bit-exactness proof
 * @see ./motion_scratch.ts — the precedent of work package L
 */
import { copyFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const ROOT = resolve(import.meta.dir, "../../../../../../..").replaceAll("\\", "/");
const PETS = "🧰️framework/🛍️products/🐾️pets";
const UP = "../../../../../../../../../../../..";
const SCRATCH = join(import.meta.dir, "🗑️generated", "wp-m");
const CRATE = join(SCRATCH, "crate", "📦️packages", "🦀️rust");
const CASES: readonly (readonly [string, string])[] = [["behavior_choice", "🧠️behavior-choice"], ["bond_dynamics", "🤝️bond-dynamics"], ["stage_trace", "🎪️stage-trace"]];
const MODULES: readonly (readonly [string, string])[] = [["schema", "🧬️schema"], ["trigonometry", "🔨️modules/📐️trigonometry"], ["randomness", "🔨️modules/🎲️randomness"], ["rig", "🔨️modules/🦴️rig"], ["animation", "🔨️modules/🎞️animation"], ["terrain", "🔨️modules/🏞️terrain"], ["behavior", "🔨️modules/🧠️behavior"], ["stage", "🔨️modules/🎪️stage"]];
const MUTABLE = ["behavior", "stage"];
const TEST_ATTACH = '#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n';

type Mutant = { readonly name: string; readonly module: "behavior" | "stage"; readonly edits: readonly (readonly [string, string])[]; readonly survives?: string };
const MUTANTS: readonly Mutant[] = [
  { name: "stage_fused_mood", module: "stage", edits: [["let next = body.mood + (want - body.mood) * MOOD_EASE;", "let next = (want - body.mood).mul_add(MOOD_EASE, body.mood);"]], survives: "an equivalent mutant: MOOD_EASE is 2⁻⁵, the product is exact, so fusing it rounds nothing differently" },
  { name: "stage_fused_goal", module: "stage", edits: [["let mut goal = clamp(low + (high - low) * place, at - reach, at + reach);", "let mut goal = clamp((high - low).mul_add(place, low), at - reach, at + reach);"]] },
  { name: "stage_reassociated_goal", module: "stage", edits: [["let mut goal = clamp(low + (high - low) * place, at - reach, at + reach);", "let mut goal = clamp(low * (1.0 - place) + high * place, at - reach, at + reach);"]] },
  { name: "stage_swapped_blink_words", module: "stage", edits: [["if unit_of(words[1]) * 6.0 < 1.0 { now + BLINK_AGAIN } else { blink_at(now, unit_of(words[0])) }", "if unit_of(words[0]) * 6.0 < 1.0 { now + BLINK_AGAIN } else { blink_at(now, unit_of(words[1])) }"]] },
  { name: "stage_eye_height_one_ulp_off", module: "stage", edits: [["const EYE_HEIGHT: f64 = 0.6;", "const EYE_HEIGHT: f64 = 0.600_000_000_000_000_1;"]] },
  { name: "stage_reassociated_glide", module: "stage", edits: [["launches.push(Launch { x, vx: (dx * RATE) / ticks, vy: (dy * RATE) / ticks, ticks: ticks as Ticks });", "launches.push(Launch { x, vx: dx * (RATE / ticks), vy: dy * (RATE / ticks), ticks: ticks as Ticks });"]] },
  { name: "stage_eager_one_tick_longer", module: "stage", edits: [["let eager = needs_after(one.needs, Activity::Idle, now - one.since, draft.kinds[first].temperament).sociability;", "let eager = needs_after(one.needs, Activity::Idle, now - one.since + 1, draft.kinds[first].temperament).sociability;"]], survives: "a blind spot: the sociability of the moment only weighs a pairing draw, and 3e-5 more of it flips no draw in these sessions" },
  { name: "stage_fused_eye", module: "stage", edits: [["let eye = Point { x: actor.x, y: actor.y - kinds[index].size.height * EYE_HEIGHT };", "let eye = Point { x: actor.x, y: (0.0 - kinds[index].size.height).mul_add(EYE_HEIGHT, actor.y) };"]] },
  { name: "stage_turn_rest_one_tick_longer", module: "stage", edits: [["const TURN_REST: Ticks = 56;", "const TURN_REST: Ticks = 57;"]] },
  { name: "stage_perk_one_tick_later", module: "stage", edits: [["const PERK_LINGER: Ticks = 32;", "const PERK_LINGER: Ticks = 33;"]] },
  { name: "stage_perk_cost_one_ulp_off", module: "stage", edits: [["const PERK_COST: f64 = 0.6;", "const PERK_COST: f64 = 0.600_000_000_000_000_1;"]] },
  { name: "stage_fused_lean", module: "stage", edits: [["x: bone.x + LEAN_REACH * across,", "x: LEAN_REACH.mul_add(across, bone.x),"]] },
  { name: "stage_ground_as_good_as_a_shelf", module: "stage", edits: [["let raised = |room: &Room| draft.stage.perches[room.perch].y < ground;", "let raised = |room: &Room| draft.stage.perches[room.perch].y <= ground;"]] },
  { name: "stage_every_survey_brings_new_ground", module: "stage", edits: [["draft.stage.surfaces.iter().any(|surface| surface_of(before, &surface.id).is_none())", "!draft.stage.surfaces.is_empty()"]] },
  { name: "stage_falls_from_vanished_ground", module: "stage", edits: [["if uprooted || draft.stage.mode == PetMode::Still {", "if draft.stage.mode == PetMode::Still {"]] },
  { name: "behavior_fused_energy", module: "behavior", edits: [["energy: clamp(needs.energy + (if energy < 0.0 { energy * (1.5 - temperament.energy) } else { energy }) * seconds, 0.0, 1.0),", "energy: clamp((if energy < 0.0 { energy * (1.5 - temperament.energy) } else { energy }).mul_add(seconds, needs.energy), 0.0, 1.0),"]] },
  { name: "behavior_reassociated_fidget", module: "behavior", edits: [["{ limits.fidget * drive * (0.5 + curiosity) }", "{ limits.fidget * (drive * (0.5 + curiosity)) }"]], survives: "a blind spot of the long run: a weight one unit in the last place off flips no pick; the weights themselves are held to the TypeScript twin bit for bit by rehearse_stage_adapters.ts" },
  { name: "behavior_reassociated_fade", module: "behavior", edits: [["let step = (RAPPORT_FADE_STEP * ticks as f64) / RAPPORT_FADE_TICKS;", "let step = RAPPORT_FADE_STEP * (ticks as f64 / RAPPORT_FADE_TICKS);"]] },
];

/** 📦️ The manifest of the scratch crate. */
function manifest(): string {
  return [
    "# 🧪️ Private scratch crate of work package M, written by `stage_scratch.ts prepare`: compiles the pets Rust twins from the",
    "# repository files (mounted by `#[path]`) without touching the root workspace. Not a member of anything.",
    "[workspace]",
    "",
    "[package]",
    'name = "semio-framework-pets-scratch-m"',
    'version = "0.0.0"',
    'edition = "2021"',
    "publish = false",
    "",
    "[lib]",
    'name = "pets"',
    'path = "🦀️.rs"',
    "",
    "[[example]]",
    'name = "long_trace_digest"',
    'path = "../../../../../long_trace_digest.rs"',
    'required-features = ["sut"]',
    "",
    ...CASES.flatMap(([example]) => ["[[example]]", `name = "${example}"`, `path = "hosts/${example}.rs"`, 'required-features = ["sut"]', ""]),
    "[features]",
    'sut = ["dep:serde_json"]',
    ...MUTABLE.map((module) => `mutant-${module} = []`),
    "",
    "[dependencies]",
    'serde = { version = "=1.0.228", features = ["derive"] }',
    'serde_json = { version = "=1.0.149", features = ["raw_value", "float_roundtrip"], optional = true }',
    "",
    "[dev-dependencies]",
    'serde_json = { version = "=1.0.149", features = ["raw_value", "float_roundtrip"] }',
    `semio-repo-test-host = { path = "${UP}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust" }`,
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

/** 🔌️ The glue of the scratch crate: the mounts of the real glue (without validation) and the flat names of the façade, with a mutant in place of a twin when its feature is on. */
function glue(): string {
  const mount = ([module, path]: readonly [string, string]): string =>
    MUTABLE.includes(module) ? `#[cfg_attr(not(feature = "mutant-${module}"), path = "${UP}/${PETS}/${path}/🦀️.rs")]\n#[cfg_attr(feature = "mutant-${module}", path = "mutants/current_${module}.rs")]\npub mod ${module};\n` : `#[path = "${UP}/${PETS}/${path}/🦀️.rs"]\npub mod ${module};\n`;
  return [
    "//! 📦️ Scratch glue of work package M, written by `stage_scratch.ts prepare` — the mounts the real package glue carries for the simulation and what it imports. The `mutant-…` features swap in a deliberately broken copy.\n",
    ...MODULES.map(mount),
    `${MODULES.map(([module]) => `pub use ${module}::*;`).join("\n")}\n`,
    '#[cfg(feature = "sut")]\npub use serde_json;\n',
  ].join("\n");
}

/** 🏗️ Writes the scratch crate. */
function prepare(): void {
  mkdirSync(join(CRATE, "hosts"), { recursive: true });
  mkdirSync(join(CRATE, "mutants"), { recursive: true });
  writeFileSync(join(CRATE, "Cargo.toml"), manifest());
  writeFileSync(join(CRATE, "🦀️.rs"), glue());
  copyFileSync(join(ROOT, "Cargo.lock"), join(CRATE, "Cargo.lock"));
  for (const [example, name] of CASES) writeFileSync(join(CRATE, "hosts", `${example}.rs`), `// Stand-in of the entrypoint the harness generates for a Rust host.\n#[path = "${ROOT}/${PETS}/🧪️tests/${name}/🦀️.rs"]\nmod adapter;\n\nfn main() -> std::process::ExitCode {\n    semio_repo_test_host::run_main(adapter::adapter())\n}\n`);
  console.log(`[DEBUG] scratch crate written to ${CRATE}`);
}

/** 🧟️ Writes every mutant, replays the recorded sessions through it and answers how many checkpoints caught it. */
function mutants(): void {
  prepare();
  const lines: string[] = [];
  let survived = 0;
  let surprises = 0;
  for (const mutant of MUTANTS) {
    let source = readFileSync(join(ROOT, PETS, "🔨️modules", mutant.module === "behavior" ? "🧠️behavior" : "🎪️stage", "🦀️.rs"), "utf8");
    for (const [from, to] of [[TEST_ATTACH, ""] as const, ...mutant.edits]) {
      if (source.split(from).length !== 2) throw new Error(`${mutant.name}: the twin does not contain exactly once: ${from}`);
      source = source.replace(from, () => to);
    }
    writeFileSync(join(CRATE, "mutants", `${mutant.name}.rs`), source);
    writeFileSync(join(CRATE, "mutants", `current_${mutant.module}.rs`), source);
    const run = Bun.spawnSync(["bash", join(import.meta.dir, "rust_scratch.sh"), "wp-m", "run", "--release", "--offline", "--features", `sut,mutant-${mutant.module}`, "--example", "long_trace_digest", "--", "--no-record"], { cwd: ROOT, stdout: "pipe", stderr: "pipe" });
    const total = /rust against typescript: (\d+) sessions, (\d+) checkpoints compared \(two digests each\), (\d+) mismatches; first mismatch: (.*)$/m.exec(run.stdout.toString());
    const caught = total === null ? -1 : Number(total[3]);
    const expected = mutant.survives === undefined ? caught > 0 : caught === 0;
    if (caught === 0) survived += 1;
    if (!expected) surprises += 1;
    lines.push(caught < 0 ? `${mutant.name}: the run failed: ${run.stderr.toString().trim().split("\n").slice(-4).join(" | ")}` : `${mutant.name}: ${caught} of ${total![2]} checkpoints differ${mutant.survives === undefined ? "" : ` (expected to survive — ${mutant.survives})`}; first: ${total![4]}`);
    console.log(`[DEBUG] ${lines[lines.length - 1]}`);
  }
  lines.push(`${MUTANTS.length} mutants, ${survived} survived (${MUTANTS.filter((mutant) => mutant.survives !== undefined).length} expected to), ${surprises} surprises`);
  writeFileSync(join(SCRATCH, "mutants.txt"), `${lines.join("\n")}\n`);
  console.log(`[DEBUG] ${lines[lines.length - 1]}`);
  if (surprises > 0) process.exit(1);
}

const command = process.argv[2] ?? "prepare";
if (command === "prepare") prepare();
else if (command === "mutants") mutants();
else throw new Error(`unknown command ${command}: prepare or mutants`);
