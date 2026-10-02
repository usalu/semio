/** 🧰️ Ticket tool of work package L: (re)creates the private scratch crate the motion tools run in, and proves with deliberately broken twins that the bit-exactness tool notices.
 *
 * `prepare` writes `🗑️generated/wp-l/crate/📦️packages/🦀️rust/{Cargo.toml, 🦀️.rs, hosts/<case>.rs, Cargo.lock}`: the
 * crate mounts the repository's twins by `#[path]`, restates the workspace lint tables, links serde_json as the real
 * crate does (`float_roundtrip`), carries `check_motion_bits.rs` and `probe_json_number_parsing.rs` of this folder
 * as integration tests and one host stand-in per motion case as an example. Everything below `🗑️generated` may be
 * deleted; this command brings it back.
 *
 * `mutants` copies the animation and terrain twins, breaks one expression in each copy (a fused multiply-add, a
 * reassociated sum, the native `f64::max`/`f64::min`), runs `check_motion_bits.rs` against each copy and fails unless
 * every mutant is caught. Run `dump_motion_bits.ts` before it.
 *
 * Run from the repository root:
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/motion_scratch.ts prepare
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/motion_scratch.ts mutants
 *
 * @see ./rust_scratch.sh — the runner of a scratch crate (work package K)
 * @see ./dump_motion_bits.ts, ./check_motion_bits.rs — the bit-exactness tool
 * @see ./rehearse_motion_adapters.ts — the adapter rehearsal
 */
import { copyFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const ROOT = resolve(import.meta.dir, "../../../../../../..").replaceAll("\\", "/");
const PETS = "🧰️framework/🛍️products/🐾️pets";
const UP = "../../../../../../../../../../../..";
const SCRATCH = join(import.meta.dir, "🗑️generated", "wp-l");
const CRATE = join(SCRATCH, "crate", "📦️packages", "🦀️rust");
const CASES: readonly (readonly [string, string])[] = [["animation_sampling", "🎞️animation-sampling"], ["spring_settling", "🪀️spring-settling"], ["terrain_walking", "🏞️terrain-walking"], ["hop_ballistics", "🦘️hop-ballistics"]];
const TEST_ATTACH = '#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n';
const EXTREME = (comparison: string, sign: string): string => `    if left.is_nan() || right.is_nan() {\n        f64::NAN\n    } else if left ${comparison} right || (left == right && left.${sign}()) {\n        left\n    } else {\n        right\n    }\n`;

type Mutant = { readonly name: string; readonly module: "animation" | "terrain"; readonly edits: readonly (readonly [string, string])[] };
const MUTANTS: readonly Mutant[] = [
  { name: "animation_fused_bezier_abscissa", module: "animation", edits: [["if ((ax * middle + bx) * middle + cx) * middle < amount {", "if ax.mul_add(middle, bx).mul_add(middle, cx) * middle < amount {"]] },
  { name: "animation_fused_bezier_height", module: "animation", edits: [["((ay * solved + by) * solved + cy) * solved\n", "ay.mul_add(solved, by).mul_add(solved, cy) * solved\n"]] },
  {
    name: "animation_fused_spring",
    module: "animation",
    edits: [
      ["let quickened = velocity + (stiffness * (target - position) - damping * velocity) * TICK_SECONDS;", "let quickened = (stiffness * (target - position) - damping * velocity).mul_add(TICK_SECONDS, velocity);"],
      ["Spring { position: position + quickened * TICK_SECONDS, velocity: quickened }", "Spring { position: quickened.mul_add(TICK_SECONDS, position), velocity: quickened }"],
    ],
  },
  { name: "animation_reassociated_spring", module: "animation", edits: [["let quickened = velocity + (stiffness * (target - position) - damping * velocity) * TICK_SECONDS;", "let quickened = velocity + stiffness * (target - position) * TICK_SECONDS - damping * velocity * TICK_SECONDS;"]] },
  { name: "terrain_native_extremes", module: "terrain", edits: [[EXTREME(">", "is_sign_positive"), "    left.max(right)\n"], [EXTREME("<", "is_sign_negative"), "    left.min(right)\n"]] },
  { name: "terrain_fused_distance", module: "terrain", edits: [["let distance = dx * dx + dy * dy;", "let distance = dx.mul_add(dx, dy * dy);"]] },
  { name: "terrain_reassociated_hop", module: "terrain", edits: [["let vy = ((dy - (GRAVITY * ticks * (ticks + 1.0)) / (2.0 * RATE * RATE)) * RATE) / ticks;", "let vy = (dy - (GRAVITY * ticks * (ticks + 1.0)) / (2.0 * RATE * RATE)) * (RATE / ticks);"]] },
];

/** 📦️ The manifest of the scratch crate. */
function manifest(): string {
  const lints = ["[lints.rust]", "future_incompatible = { level = \"warn\", priority = -1 }", "rust_2018_idioms = { level = \"warn\", priority = -1 }", "unsafe_op_in_unsafe_fn = \"warn\"", "unused_lifetimes = \"warn\"", "unused_qualifications = \"warn\"", "", "[lints.clippy]", "all = { level = \"warn\", priority = -1 }", "cloned_instead_of_copied = \"warn\"", "inefficient_to_string = \"warn\"", "map_unwrap_or = \"warn\"", "needless_pass_by_value = \"warn\"", "semicolon_if_nothing_returned = \"warn\"", "unnecessary_wraps = \"warn\"", "redundant_clone = \"warn\""];
  return [
    "# 🧪️ Private scratch crate of work package L, written by `motion_scratch.ts prepare`: compiles the pets Rust twins from the",
    "# repository files (mounted by `#[path]`) without touching the root workspace. Not a member of anything.",
    "[workspace]",
    "",
    "[package]",
    'name = "semio-framework-pets-scratch-l"',
    'version = "0.0.0"',
    'edition = "2021"',
    "publish = false",
    "",
    "[lib]",
    'name = "pets"',
    'path = "🦀️.rs"',
    "",
    "[[test]]",
    'name = "motion_bits"',
    'path = "../../../../../check_motion_bits.rs"',
    "",
    "[[test]]",
    'name = "json_number_parsing"',
    'path = "../../../../../probe_json_number_parsing.rs"',
    "",
    ...CASES.flatMap(([example]) => ["[[example]]", `name = "${example}"`, `path = "hosts/${example}.rs"`, 'required-features = ["sut"]', ""]),
    "[features]",
    'sut = ["dep:serde_json"]',
    "mutant-animation = []",
    "mutant-terrain = []",
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

/** 🔌️ The glue of the scratch crate: the mounts of the real glue, with a mutant in place of a twin when its feature is on. */
function glue(): string {
  const mount = (module: string, path: string): string => `#[path = "${UP}/${PETS}/${path}/🦀️.rs"]\npub mod ${module};\n`;
  const swappable = (module: string, path: string): string => `#[cfg_attr(not(feature = "mutant-${module}"), path = "${UP}/${PETS}/${path}/🦀️.rs")]\n#[cfg_attr(feature = "mutant-${module}", path = "mutants/current_${module}.rs")]\npub mod ${module};\n`;
  return [
    "//! 📦️ Scratch glue of work package L, written by `motion_scratch.ts prepare` — the mounts the real package glue carries for animation and terrain and what they import. The `mutant-…` features swap in a deliberately broken copy.\n",
    mount("schema", "🧬️schema"),
    mount("trigonometry", "🔨️modules/📐️trigonometry"),
    mount("randomness", "🔨️modules/🎲️randomness"),
    mount("rig", "🔨️modules/🦴️rig"),
    swappable("animation", "🔨️modules/🎞️animation"),
    swappable("terrain", "🔨️modules/🏞️terrain"),
    "pub use animation::*;\npub use rig::*;\npub use schema::*;\npub use terrain::*;\npub use trigonometry::*;\n",
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

/** 🧟️ Writes every mutant, runs the bit-exactness check against it and answers how many cases caught it. */
function mutants(): void {
  prepare();
  const lines: string[] = [];
  let survived = 0;
  for (const mutant of MUTANTS) {
    let source = readFileSync(join(ROOT, PETS, "🔨️modules", mutant.module === "animation" ? "🎞️animation" : "🏞️terrain", "🦀️.rs"), "utf8");
    for (const [from, to] of [[TEST_ATTACH, ""] as const, ...mutant.edits]) {
      if (source.split(from).length !== 2) throw new Error(`${mutant.name}: the twin does not contain exactly once: ${from}`);
      source = source.replace(from, () => to);
    }
    writeFileSync(join(CRATE, "mutants", `${mutant.name}.rs`), source);
    writeFileSync(join(CRATE, "mutants", `current_${mutant.module}.rs`), source);
    const run = Bun.spawnSync(["bash", join(import.meta.dir, "rust_scratch.sh"), "wp-l", "test", "--offline", "--features", `mutant-${mutant.module}`, "--test", "motion_bits", "--", "--nocapture"], { cwd: ROOT, stdout: "pipe", stderr: "pipe" });
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
else throw new Error("usage: motion_scratch.ts prepare|mutants");
