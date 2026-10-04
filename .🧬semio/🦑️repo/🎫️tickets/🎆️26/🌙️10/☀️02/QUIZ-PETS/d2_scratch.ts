/** 🧰️ Ticket tool of work package D2: (re)creates the private scratch crate the Rust twins of clearance and gesture are compiled in before they are mounted in the real crate, and proves with deliberately broken twins that the bit-exactness checks of `d2_check_bits.rs` and `d2_check_world.rs` notice.
 *
 * `prepare` writes `🗑️generated/d2/crate/📦️packages/🦀️rust/{Cargo.toml, 🦀️.rs, Cargo.lock, hosts/<case>.rs}`: the crate
 * mounts the repository's twins by `#[path]` (schema, trigonometry, randomness, terrain, clearance, gesture — the
 * gesture twin from `🗑️generated/d2/draft/🦀️.rs` while that draft exists, so the mounted repository file is
 * replaced only once the draft is green), restates the workspace lint tables, links serde_json as the real crate does
 * (`float_roundtrip`), carries `d2_check_bits.rs` as the integration test `d2_bits`, `d2_check_world.rs` as a unit
 * test module of the library (it reaches the reference world of the clearance unit suite), and one host stand-in per
 * case as an example (`required-features = ["sut"]`). Everything below `🗑️generated` may be deleted; this command
 * brings it back.
 *
 * `mutants` copies the two twins, breaks one expression in each copy (a fused multiply-add, a reassociated product or
 * quotient, a native `f64::min`/`f64::max`, a comparison that is strict where it was not, a tie broken the other way, a
 * constant one unit in the last place off), runs both checks against each copy (the copy of clearance keeps the
 * repository's unit suite attached, whose reference world the world check replays) and fails unless every mutant is
 * caught. Run `d2_dump_bits.ts` before it.
 *
 * Run from the repository root (`mutants <name>…` runs only the named mutants):
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/d2_scratch.ts prepare
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/d2_scratch.ts mutants
 *
 * @see ./rust_scratch.sh — the runner of a scratch crate (work package K)
 * @see ./d2_dump_bits.ts, ./d2_check_bits.rs, ./d2_check_world.rs — the bit-exactness tools
 * @see ./motion_scratch.ts — the pattern (work package L)
 */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const ROOT = resolve(import.meta.dir, "../../../../../../..").replaceAll("\\", "/");
const PETS = "🧰️framework/🛍️products/🐾️pets";
const UP = "../../../../../../../../../../../..";
const SCRATCH = join(import.meta.dir, "🗑️generated", "d2");
const CRATE = join(SCRATCH, "crate", "📦️packages", "🦀️rust");
const DRAFT = join(SCRATCH, "draft", "🦀️.rs");
const CASES: readonly (readonly [string, string])[] = [
  ["clearance_proof", "🚧️clearance-proof"],
  ["gesture_recognition", "👆️gesture-recognition"],
];
const TWINS = { clearance: "🚧️clearance", gesture: "👆️gesture" } as const;
type Twin = keyof typeof TWINS;
const TEST_ATTACH: Record<Twin, string> = { clearance: '#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\npub(crate) mod tests;\n', gesture: '#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n' };

type Mutant = { readonly name: string; readonly twin: Twin; readonly edits: readonly (readonly [string, string])[] };
const MUTANTS: readonly Mutant[] = [
  { name: "clearance_native_extremes_united", twin: "clearance", edits: [["Extent { x0: smaller(one.x0, other.x0), y0: smaller(one.y0, other.y0), x1: larger(one.x1, other.x1), y1: larger(one.y1, other.y1) }", "Extent { x0: one.x0.min(other.x0), y0: one.y0.min(other.y0), x1: one.x1.max(other.x1), y1: one.y1.max(other.y1) }"]] },
  { name: "clearance_native_extremes_slot", twin: "clearance", edits: [["let place = smaller(larger(x, x0), x1);", "let place = x.max(x0).min(x1);"]] },
  { name: "clearance_touching_meets", twin: "clearance", edits: [["one.x0 < other.x1 && other.x0 < one.x1 && one.y0 < other.y1 && other.y0 < one.y1", "one.x0 <= other.x1 && other.x0 < one.x1 && one.y0 < other.y1 && other.y0 < one.y1"]] },
  { name: "clearance_reassociated_carve", twin: "clearance", edits: [["x + (obstacle.x0 - extent.x1 - SEAM), x + (obstacle.x1 - extent.x0 + SEAM)", "x + obstacle.x0 - extent.x1 - SEAM, x + obstacle.x1 - extent.x0 + SEAM"]] },
  { name: "clearance_seat_mean_by_reciprocal", twin: "clearance", edits: [["smaller(larger(sum / count as f64, low), high)", "smaller(larger(sum * (1.0 / count as f64), low), high)"]] },
  { name: "clearance_reassociated_seat_mean", twin: "clearance", edits: [["shift: mean - (bodies[member].extent.x0 - offsets[at])", "shift: mean - bodies[member].extent.x0 + offsets[at]"]] },
  { name: "clearance_reassociated_scoot", twin: "clearance", edits: [["from + (to - from) * fraction", "from * (1.0 - fraction) + to * fraction"]] },
  { name: "clearance_push_ties_right_first", twin: "clearance", edits: [["            if least == left {\n                dx -= left;\n            } else if least == right {\n                dx += right;\n", "            if least == right {\n                dx += right;\n            } else if least == left {\n                dx -= left;\n"]] },
  { name: "clearance_rest_strict", twin: "clearance", edits: [["host.y0 - rider.y1 <= 2.0 * SEAM", "host.y0 - rider.y1 < 2.0 * SEAM"]] },
  { name: "clearance_guard_negation", twin: "clearance", edits: [["        return 0.0 - reach;", "        return -reach;"]] },
  { name: "clearance_lift_reassociated", twin: "clearance", edits: [["host.y0 - SEAM - extent.y1", "host.y0 - (SEAM + extent.y1)"]] },
  { name: "gesture_fused_radius", twin: "gesture", edits: [["    let radius = rx * rx + ry * ry;\n    let reach = larger(body.width, body.height);", "    let radius = rx.mul_add(rx, ry * ry);\n    let reach = larger(body.width, body.height);"]] },
  { name: "gesture_fused_cross", twin: "gesture", edits: [["let cross = circling.px * ry - circling.py * rx;", "let cross = circling.px.mul_add(ry, -(circling.py * rx));"]] },
  { name: "gesture_reassociated_round", twin: "gesture", edits: [["let round = far <= CIRCLE_ROUND * CIRCLE_ROUND * near;", "let round = far <= CIRCLE_ROUND * (CIRCLE_ROUND * near);"]] },
  { name: "gesture_reassociated_speed", twin: "gesture", edits: [["let speed = (length * TICKS_PER_SECOND as f64) / max(stroking.reached - stroking.began, 1) as f64;", "let speed = length * (TICKS_PER_SECOND as f64 / max(stroking.reached - stroking.began, 1) as f64);"]] },
  { name: "gesture_reassociated_outer", twin: "gesture", edits: [["let outer = reach * CIRCLE_REACH;", "let outer = reach * 17.0 / 5.0;"]] },
  { name: "gesture_reassociated_hysteresis", twin: "gesture", edits: [["let hysteresis = STROKE_HYSTERESIS * body.width;", "let hysteresis = body.width * 3.0 / 20.0;"]] },
  { name: "gesture_heat_leak_one_ulp_off", twin: "gesture", edits: [["larger(heat - (max(tick - since, 0) as f64 * HEAT_LEAK) / TICKS_PER_SECOND as f64, 0.0)", "larger(heat - (max(tick - since, 0) as f64 * 0.5000000000000001) / TICKS_PER_SECOND as f64, 0.0)"]] },
  { name: "gesture_fused_shake_span", twin: "gesture", edits: [["    let span = sx * sx + sy * sy;", "    let span = sx.mul_add(sx, sy * sy);"]] },
  { name: "gesture_fused_slop", twin: "gesture", edits: [["return if dx * dx + dy * dy >= press.slop * press.slop {", "return if dx.mul_add(dx, dy * dy) >= press.slop * press.slop {"]] },
  { name: "gesture_native_extremes_stroke", twin: "gesture", edits: [["    let top_since = smaller(stroking.top_since, ry);\n    let bottom_since = larger(stroking.bottom_since, ry);", "    let top_since = stroking.top_since.min(ry);\n    let bottom_since = stroking.bottom_since.max(ry);"]] },
];

/** 📦️ The manifest of the scratch crate. */
function manifest(): string {
  const lints = ["[lints.rust]", 'future_incompatible = { level = "warn", priority = -1 }', 'rust_2018_idioms = { level = "warn", priority = -1 }', 'unsafe_op_in_unsafe_fn = "warn"', 'unused_lifetimes = "warn"', 'unused_qualifications = "warn"', "", "[lints.clippy]", 'all = { level = "warn", priority = -1 }', 'cloned_instead_of_copied = "warn"', 'inefficient_to_string = "warn"', 'map_unwrap_or = "warn"', 'needless_pass_by_value = "warn"', 'semicolon_if_nothing_returned = "warn"', 'unnecessary_wraps = "warn"', 'redundant_clone = "warn"'];
  return [
    "# 🧪️ Private scratch crate of work package D2, written by `d2_scratch.ts prepare`: compiles the pets Rust twins from the",
    "# repository files (mounted by `#[path]`) without touching the root workspace. Not a member of anything.",
    "[workspace]",
    "",
    "[package]",
    'name = "semio-framework-pets-scratch-d2"',
    'version = "0.0.0"',
    'edition = "2021"',
    "publish = false",
    "",
    "[lib]",
    'name = "pets"',
    'path = "🦀️.rs"',
    "",
    ...(existsSync(join(import.meta.dir, "d2_check_bits.rs")) ? ["[[test]]", 'name = "d2_bits"', 'path = "../../../../../d2_check_bits.rs"', ""] : []),
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

/** 🗺️ Where the scratch crate mounts a twin from: the draft of the gesture twin while it exists, else the repository file. */
function source(twin: Twin): string {
  return twin === "gesture" && existsSync(DRAFT) ? "../../../draft/🦀️.rs" : `${UP}/${PETS}/🔨️modules/${TWINS[twin]}/🦀️.rs`;
}

/** 🔌️ The glue of the scratch crate: the mounts the two twins and their unit suites need, with a mutant in place of a twin when its feature is on. */
function glue(): string {
  const mount = (module: string, path: string): string => `#[path = "${UP}/${PETS}/${path}/🦀️.rs"]\npub mod ${module};\n`;
  const swappable = (module: Twin): string => `#[cfg_attr(not(feature = "mutant-${module}"), path = "${source(module)}")]\n#[cfg_attr(feature = "mutant-${module}", path = "mutants/current_${module}.rs")]\npub mod ${module};\n`;
  return [
    "//! 📦️ Scratch glue of work package D2, written by `d2_scratch.ts prepare` — the mounts the real package glue carries for the clearance and gesture twins and what they and their unit suites import, and the world check as a unit test module. The `mutant-…` features swap in a deliberately broken copy.\n",
    mount("schema", "🧬️schema"),
    mount("trigonometry", "🔨️modules/📐️trigonometry"),
    mount("randomness", "🔨️modules/🎲️randomness"),
    mount("terrain", "🔨️modules/🏞️terrain"),
    swappable("clearance"),
    swappable("gesture"),
    ...(existsSync(join(import.meta.dir, "d2_check_world.rs")) ? ['#[cfg(test)]\n#[path = "../../../../../d2_check_world.rs"]\nmod world_check;\n'] : []),
    ["clearance", "gesture", "schema"].map((module) => `pub use ${module}::*;\n`).join(""),
    '#[cfg(feature = "sut")]\npub use serde_json;\n',
  ].join("\n");
}

/** 🏗️ Writes the scratch crate. */
function prepare(): void {
  for (const folder of ["hosts", "mutants"]) mkdirSync(join(CRATE, folder), { recursive: true });
  writeFileSync(join(CRATE, "Cargo.toml"), manifest());
  writeFileSync(join(CRATE, "🦀️.rs"), glue());
  writeFileSync(join(CRATE, "Cargo.lock"), readFileSync(join(ROOT, "Cargo.lock")));
  for (const twin of Object.keys(TWINS) as Twin[]) if (!existsSync(join(CRATE, "mutants", `current_${twin}.rs`))) writeFileSync(join(CRATE, "mutants", `current_${twin}.rs`), "");
  for (const [example, name] of written()) writeFileSync(join(CRATE, "hosts", `${example}.rs`), `// Stand-in of the entrypoint the harness generates for a Rust host.\n#[path = "${ROOT}/${PETS}/🧪️tests/${name}/🦀️.rs"]\nmod adapter;\n\nfn main() -> std::process::ExitCode {\n    semio_repo_test_host::run_main(adapter::adapter())\n}\n`);
  console.log(`[DEBUG] scratch crate written to ${CRATE} (gesture from ${source("gesture")})`);
}

/** 🔎️ The counts of one run of the checks: the bit check's total line and the world check's line. */
function caught(output: string): { bits: number; world: number } {
  const bits = /^total: \d+ cases, \d+ result numbers, (\d+) mismatches$/m.exec(output);
  const world = /^world: \d+ runs, \d+ ticks, (\d+) mismatches$/m.exec(output);
  return { bits: bits === null ? -1 : Number(bits[1]), world: world === null ? -1 : Number(world[1]) };
}

/** 🧟️ Writes every mutant, runs both checks against it and answers how many cases caught it. */
function mutants(): void {
  prepare();
  const lines: string[] = [];
  let survived = 0;
  const only = process.argv.slice(3);
  const chosen = only.length > 0 ? MUTANTS.filter((mutant) => only.includes(mutant.name)) : MUTANTS;
  for (const mutant of chosen) {
    const path = mutant.twin === "gesture" && existsSync(DRAFT) ? DRAFT : join(ROOT, PETS, "🔨️modules", TWINS[mutant.twin], "🦀️.rs");
    let text = readFileSync(path, "utf8");
    const suite = mutant.twin === "clearance" ? `#[cfg(test)]\n#[path = "${ROOT}/${PETS}/🔨️modules/🚧️clearance/🧪️tests/🔬️unit/🦀️.rs"]\npub(crate) mod tests;\n` : "";
    for (const [from, to] of [[TEST_ATTACH[mutant.twin], suite] as const, ...mutant.edits]) {
      if (text.split(from).length !== 2) throw new Error(`${mutant.name}: the twin does not contain exactly once: ${from}`);
      text = text.replace(from, () => to);
    }
    writeFileSync(join(CRATE, "mutants", `${mutant.name}.rs`), text);
    writeFileSync(join(CRATE, "mutants", `current_${mutant.twin}.rs`), text);
    const bits = Bun.spawnSync(["bash", join(import.meta.dir, "rust_scratch.sh"), "d2", "test", "--offline", "--release", "--features", `mutant-${mutant.twin}`, "--test", "d2_bits", "--", "--nocapture"], { cwd: ROOT, stdout: "pipe", stderr: "pipe" });
    const world = Bun.spawnSync(["bash", join(import.meta.dir, "rust_scratch.sh"), "d2", "test", "--offline", "--release", "--features", `mutant-${mutant.twin}`, "--lib", "world_check", "--", "--nocapture"], { cwd: ROOT, stdout: "pipe", stderr: "pipe" });
    const found = { bits: caught(bits.stdout.toString()).bits, world: caught(world.stdout.toString()).world };
    if (found.bits <= 0 && found.world <= 0) survived += 1;
    const where = [...bits.stdout.toString().matchAll(/^(\w+): \d+ cases, \d+ result numbers, ([1-9]\d*) mismatches$/gm)].filter((match) => match[1] !== "total").map((match) => `${match[1]} ${match[2]}`);
    const failure = found.bits < 0 ? ` (bit check did not run: ${bits.stderr.toString().trim().split("\n").slice(-3).join(" | ")})` : "";
    lines.push(`${mutant.name}: bits ${found.bits} cases differ${where.length > 0 ? ` (${where.join(", ")})` : ""}, world ${found.world} runs differ${failure}`);
  }
  lines.push(`${chosen.length} mutants, ${survived} survived`);
  writeFileSync(join(SCRATCH, "mutants.txt"), `${lines.join("\n")}\n`);
  console.log(lines.map((line) => `[DEBUG] ${line}`).join("\n"));
  if (survived > 0) process.exit(1);
}

const command = process.argv[2];
if (command === "prepare") prepare();
else if (command === "mutants") mutants();
else throw new Error("usage: d2_scratch.ts prepare|mutants");
