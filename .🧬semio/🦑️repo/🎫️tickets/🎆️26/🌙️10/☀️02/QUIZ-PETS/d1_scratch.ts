/** 🧰️ Ticket tool of work package D1 (round 2): (re)creates the private scratch crate the gear twins (swing, climbing, the terrain additions) compile and run in, and proves with deliberately broken twins that the bit-exactness tool notices.
 *
 * `prepare` writes `🗑️generated/d1/crate/📦️packages/🦀️rust/{Cargo.toml, 🦀️.rs, hosts/<case>.rs, Cargo.lock}`: the crate
 * mounts the repository's twins by `#[path]` (schema, trigonometry, randomness, rig, animation, terrain, swing, climbing),
 * restates the workspace lint tables, links serde_json as the real crate does (`float_roundtrip`), carries
 * `d1_check_gear_bits.rs` of this folder as the integration test `gear_bits` and one host stand-in per case as an example
 * (the two lines the harness generates). Everything below `🗑️generated` may be deleted; this command brings it back.
 *
 * `mutants` copies the swing, climbing and terrain twins, breaks one expression in each copy (a fused multiply-add, a
 * reassociated sum or quotient, the native `f64::max`/`f64::min`, a flipped comparison), runs `gear_bits` against each
 * copy and fails unless every mutant is caught — but for the ones marked equivalent (with the reason), which must
 * survive. Run `d1_dump_gear_bits.ts` before it.
 *
 * Run from the repository root:
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/d1_scratch.ts prepare
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/d1_scratch.ts mutants
 *
 * @see ./rust_scratch.sh — the runner of a scratch crate (work package K)
 * @see ./d1_dump_gear_bits.ts, ./d1_check_gear_bits.rs — the bit-exactness tool
 * @see ./📓️report2-d1.md — the recorded results
 */
import { copyFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const ROOT = resolve(import.meta.dir, "../../../../../../..").replaceAll("\\", "/");
const PETS = "🧰️framework/🛍️products/🐾️pets";
const UP = "../../../../../../../../../../../..";
const SCRATCH = join(import.meta.dir, "🗑️generated", "d1");
const CRATE = join(SCRATCH, "crate", "📦️packages", "🦀️rust");
const CASES: readonly (readonly [string, string])[] = [
  ["swing_dynamics", "🪢️swing-dynamics"],
  ["parachute_descent", "🪂️parachute-descent"],
  ["wall_climbing", "🧗️wall-climbing"],
  ["ladder_geometry", "🪜️ladder-geometry"],
  ["grapple_reach", "🎣️grapple-reach"],
  ["turn_trigonometry", "📐️turn-trigonometry"],
  ["terrain_walking", "🏞️terrain-walking"],
  ["hop_ballistics", "🦘️hop-ballistics"],
];
const MODULES: readonly (readonly [string, string, boolean])[] = [
  ["schema", "🧬️schema", false],
  ["trigonometry", "🔨️modules/📐️trigonometry", false],
  ["randomness", "🔨️modules/🎲️randomness", false],
  ["rig", "🔨️modules/🦴️rig", false],
  ["animation", "🔨️modules/🎞️animation", false],
  ["terrain", "🔨️modules/🏞️terrain", true],
  ["swing", "🔨️modules/🪢️swing", true],
  ["climbing", "🔨️modules/🧗️climbing", true],
];
const TEST_ATTACH = '#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n';
const EXTREME = (comparison: string, sign: string): string => `    if left.is_nan() || right.is_nan() {\n        f64::NAN\n    } else if left ${comparison} right || (left == right && left.${sign}()) {\n        left\n    } else {\n        right\n    }\n`;

type Mutant = { readonly name: string; readonly module: "terrain" | "swing" | "climbing"; readonly edits: readonly (readonly [string, string])[]; readonly equivalent?: string };
const MUTANTS: readonly Mutant[] = [
  { name: "terrain_native_extremes", module: "terrain", edits: [[EXTREME(">", "is_sign_positive"), "    left.max(right)\n"], [EXTREME("<", "is_sign_negative"), "    left.min(right)\n"]] },
  { name: "terrain_fused_wall_distance", module: "terrain", edits: [["let dy = larger(larger(pitch.y0 - y, 0.0), y - pitch.y1);\n        let distance = dx * dx + dy * dy;", "let dy = larger(larger(pitch.y0 - y, 0.0), y - pitch.y1);\n        let distance = dx.mul_add(dx, dy * dy);"]] },
  { name: "terrain_reassociated_grown_box", module: "terrain", edits: [["let right = rect.x + rect.width + margin;", "let right = rect.x + (rect.width + margin);"]] },
  { name: "terrain_inclusive_lip", module: "terrain", edits: [["keepout.height > 0.0 && larger(low, keepout.x) < smaller(high, keepout.x + keepout.width)", "keepout.height > 0.0 && larger(low, keepout.x) <= smaller(high, keepout.x + keepout.width)"]] },
  { name: "swing_fused_free_fall", module: "swing", edits: [["let free_y = bob.y + (bob.y - previous.y) * damping + gravity / 4096.0;", "let free_y = (bob.y - previous.y).mul_add(damping, bob.y) + gravity / 4096.0;"]] },
  { name: "swing_reassociated_pull", module: "swing", edits: [["let root = along * along - radius * (reach - length * length);", "let root = along * along - radius * reach + radius * (length * length);"]] },
  { name: "swing_native_root_floor", module: "swing", edits: [["(if root > 0.0 { root } else { 0.0 }).sqrt()", "root.max(0.0).sqrt()"]], equivalent: "f64::max ignores a NaN root exactly like the comparison does, and the two differ only for a root of −0 under an `along` of −0" },
  { name: "swing_reassociated_release", module: "swing", edits: [["Point { x: (x * 64.0) / RELEASE_DIVISOR, y: (y * 64.0) / RELEASE_DIVISOR }", "Point { x: x * (64.0 / RELEASE_DIVISOR), y: y * (64.0 / RELEASE_DIVISOR) }"]] },
  { name: "swing_fused_chute_drift", module: "swing", edits: [["let vx = canopy.vx + (wanted - canopy.vx) * CHUTE_STEER_EASE;", "let vx = (wanted - canopy.vx).mul_add(CHUTE_STEER_EASE, canopy.vx);"]] },
  { name: "swing_reassociated_wind", module: "swing", edits: [["CHUTE_WIND * sin_turns((CHUTE_WIND_RATE * ticks as f64) / 64.0 + phase)", "CHUTE_WIND * sin_turns(CHUTE_WIND_RATE * (ticks as f64 / 64.0) + phase)"]], equivalent: "a division by 64 is an exact scaling, so it commutes with the rounding of the product" },
  { name: "swing_wind_phase_inside", module: "swing", edits: [["CHUTE_WIND * sin_turns((CHUTE_WIND_RATE * ticks as f64) / 64.0 + phase)", "CHUTE_WIND * sin_turns((CHUTE_WIND_RATE * ticks as f64 + phase * 64.0) / 64.0)"]], equivalent: "scaling the sum by 64 and back is exact, so both forms round the same real sum once" },
  { name: "swing_reassociated_reel_ramp", module: "swing", edits: [["(REEL_SPEED * (age + 1) as f64) / REEL_RAMP as f64", "REEL_SPEED * ((age + 1) as f64 / REEL_RAMP as f64)"]], equivalent: "60 × (k ÷ 10) rounds to 6k for every k of the ramp (−4 … 9)" },
  { name: "swing_fused_reel_span", module: "swing", edits: [["let span = (x - anchor.x) * (x - anchor.x) + (y - anchor.y) * (y - anchor.y);", "let span = (x - anchor.x).mul_add(x - anchor.x, (y - anchor.y) * (y - anchor.y));"]] },
  { name: "climbing_reassociated_cling", module: "climbing", edits: [["pitch.x + (pitch.side.sign() * size.width) / 2.0", "pitch.x + pitch.side.sign() * (size.width / 2.0)"]], equivalent: "a factor ±1 and a halving are exact, in either order" },
  { name: "climbing_reassociated_foot", module: "climbing", edits: [["pitch.y1 - GRIP_BITE + HAND_HEIGHT * size.height", "pitch.y1 + (HAND_HEIGHT * size.height - GRIP_BITE)"]] },
  { name: "climbing_reassociated_hoist", module: "climbing", edits: [["y: from.y + (to.y - hump - from.y) * rise + hump * settle", "y: from.y + (to.y - (hump + from.y)) * rise + hump * settle"]] },
  { name: "climbing_fused_span", module: "climbing", edits: [["    (dx * dx + dy * dy).sqrt()\n}", "    dx.mul_add(dx, dy * dy).sqrt()\n}"]] },
  { name: "climbing_reassociated_hook", module: "climbing", edits: [["let share = (speed * ticks as f64) / RATE / length;", "let share = (speed * ticks as f64) / (RATE * length);"]], equivalent: "both forms round the same real quotient once, the division by 64 being exact" },
  { name: "climbing_hook_share_first", module: "climbing", edits: [["let share = (speed * ticks as f64) / RATE / length;", "let share = speed * (ticks as f64 / RATE / length);"]] },
  { name: "climbing_native_slide", module: "climbing", edits: [["let speed = smaller(larger(vy, SLIDE_START) + SLIDE_GAIN / RATE, SLIDE_SPEED);", "let speed = (vy.max(SLIDE_START) + SLIDE_GAIN / RATE).min(SLIDE_SPEED);"]] },
  { name: "climbing_reassociated_gathered", module: "climbing", edits: [["speed * smoothstep((ticks + 1) as f64 / ramp as f64)", "speed * smoothstep(ticks as f64 / ramp as f64 + 1.0 / ramp as f64)"]] },
  { name: "climbing_strict_lean", module: "climbing", edits: [["LADDER_STEEP * rise <= lean && lean <= LADDER_FLAT * rise", "LADDER_STEEP * rise < lean && lean <= LADDER_FLAT * rise"]] },
  { name: "climbing_strict_reach", module: "climbing", edits: [["lower.y0 - upper.y1 + GRIP_BITE <= CROSS_REACH * size.height", "lower.y0 - upper.y1 + GRIP_BITE < CROSS_REACH * size.height"]] },
  { name: "climbing_last_among_equals", module: "climbing", edits: [["above.is_none_or(|held| next.y1 > held.y1)", "above.is_none_or(|held| next.y1 >= held.y1)"]] },
  { name: "climbing_blended_grab", module: "climbing", edits: [["x + (hold - x) * smoothstep(tick as f64 / WALL_GRAB_TICKS as f64)", "x * (1.0 - smoothstep(tick as f64 / WALL_GRAB_TICKS as f64)) + hold * smoothstep(tick as f64 / WALL_GRAB_TICKS as f64)"]] },
];

/** 📦️ The manifest of the scratch crate. */
function manifest(): string {
  const lints = ["[lints.rust]", 'future_incompatible = { level = "warn", priority = -1 }', 'rust_2018_idioms = { level = "warn", priority = -1 }', 'unsafe_op_in_unsafe_fn = "warn"', 'unused_lifetimes = "warn"', 'unused_qualifications = "warn"', "", "[lints.clippy]", 'all = { level = "warn", priority = -1 }', 'cloned_instead_of_copied = "warn"', 'inefficient_to_string = "warn"', 'map_unwrap_or = "warn"', 'needless_pass_by_value = "warn"', 'semicolon_if_nothing_returned = "warn"', 'unnecessary_wraps = "warn"', 'redundant_clone = "warn"'];
  return [
    "# 🧪️ Private scratch crate of work package D1, written by `d1_scratch.ts prepare`: compiles the pets Rust twins from the",
    "# repository files (mounted by `#[path]`) without touching the root workspace. Not a member of anything.",
    "[workspace]",
    "",
    "[package]",
    'name = "semio-framework-pets-scratch-d1"',
    'version = "0.0.0"',
    'edition = "2021"',
    "publish = false",
    "",
    "[lib]",
    'name = "pets"',
    'path = "🦀️.rs"',
    "",
    "[[test]]",
    'name = "gear_bits"',
    'path = "../../../../../d1_check_gear_bits.rs"',
    "",
    ...CASES.flatMap(([example]) => ["[[example]]", `name = "${example}"`, `path = "hosts/${example}.rs"`, 'required-features = ["sut"]', ""]),
    "[features]",
    'sut = ["dep:serde_json"]',
    ...MODULES.filter(([, , swappable]) => swappable).map(([module]) => `mutant-${module} = []`),
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

/** 🔌️ The glue of the scratch crate: the mounts of the real glue, with a mutant in place of a twin when its feature is on, and the flat names of the façade. */
function glue(): string {
  const mount = ([module, path, swappable]: readonly [string, string, boolean]): string =>
    swappable ? `#[cfg_attr(not(feature = "mutant-${module}"), path = "${UP}/${PETS}/${path}/🦀️.rs")]\n#[cfg_attr(feature = "mutant-${module}", path = "mutants/current_${module}.rs")]\npub mod ${module};\n` : `#[path = "${UP}/${PETS}/${path}/🦀️.rs"]\npub mod ${module};\n`;
  return [
    "//! 📦️ Scratch glue of work package D1, written by `d1_scratch.ts prepare` — the mounts the real package glue carries for the gear twins and what they import, re-exported flat like the façade. The `mutant-…` features swap in a deliberately broken copy.\n",
    ...MODULES.map(mount),
    MODULES.map(([module]) => `pub use ${module}::*;`).join("\n") + "\n",
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
  for (const [module, path, swappable] of MODULES) if (swappable) writeFileSync(join(CRATE, "mutants", `current_${module}.rs`), readFileSync(join(ROOT, PETS, path, "🦀️.rs"), "utf8").replace(TEST_ATTACH, () => ""));
  console.log(`[DEBUG] scratch crate written to ${CRATE}`);
}

/** 🧟️ Writes every mutant, runs the bit-exactness check against it and answers how many cases caught it. */
function mutants(): void {
  prepare();
  const lines: string[] = [];
  let survived = 0;
  let surprises = 0;
  for (const mutant of MUTANTS) {
    const path = MODULES.find(([module]) => module === mutant.module)![1];
    let source = readFileSync(join(ROOT, PETS, path, "🦀️.rs"), "utf8");
    for (const [from, to] of [[TEST_ATTACH, ""] as const, ...mutant.edits]) {
      if (source.split(from).length !== 2) throw new Error(`${mutant.name}: the twin does not contain exactly once: ${from}`);
      source = source.replace(from, () => to);
    }
    writeFileSync(join(CRATE, "mutants", `${mutant.name}.rs`), source);
    writeFileSync(join(CRATE, "mutants", `current_${mutant.module}.rs`), source);
    const run = Bun.spawnSync(["bash", join(import.meta.dir, "rust_scratch.sh"), "d1", "test", "--offline", "--features", `mutant-${mutant.module}`, "--test", "gear_bits", "--", "--nocapture"], { cwd: ROOT, stdout: "pipe", stderr: "pipe" });
    const output = run.stdout.toString();
    const total = /^total: (\d+) cases, \d+ result numbers, \d+ throwing, (\d+) mismatches$/m.exec(output);
    const caught = total === null ? -1 : Number(total[2]);
    if (caught <= 0) survived += 1;
    if (caught <= 0 && mutant.equivalent === undefined) surprises += 1;
    if (caught > 0 && mutant.equivalent !== undefined) surprises += 1;
    const where = [...output.matchAll(/^(\w+): \d+ cases, \d+ result numbers, \d+ throwing, ([1-9]\d*) mismatches$/gm)].filter((match) => match[1] !== "total").map((match) => `${match[1]} ${match[2]}`);
    lines.push(`${mutant.name}: ${caught < 0 ? `the check did not run (${run.stderr.toString().trim().split("\n").slice(-3).join(" | ")})` : `${caught} of ${total![1]} cases differ`}${where.length > 0 ? ` (${where.join(", ")})` : ""}${mutant.equivalent === undefined ? "" : ` — equivalent: ${mutant.equivalent}`}`);
    writeFileSync(join(CRATE, "mutants", `current_${mutant.module}.rs`), readFileSync(join(ROOT, PETS, path, "🦀️.rs"), "utf8").replace(TEST_ATTACH, () => ""));
  }
  const expected = MUTANTS.filter((mutant) => mutant.equivalent !== undefined).length;
  lines.push(`${MUTANTS.length} mutants, ${survived} survived (${expected} expected to), ${surprises} surprises`);
  writeFileSync(join(SCRATCH, "mutants.txt"), `${lines.join("\n")}\n`);
  console.log(lines.map((line) => `[DEBUG] ${line}`).join("\n"));
  if (surprises > 0) process.exit(1);
}

const command = process.argv[2];
if (command === "prepare") prepare();
else if (command === "mutants") mutants();
else throw new Error("usage: d1_scratch.ts prepare|mutants");
