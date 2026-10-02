# 🗒️ Rust scratch crate — how the pets twins are compiled before the real package exists

Written by work package K for L (animation, terrain) and M (behaviour, stage). `P` = `🧰️framework/🛍️products/🐾️pets`, `TK` = this ticket folder.

## Status (K, 2026-10-02 06:20)

`TK/📓️report-wp-e.md` exists, so **the real package exists now**: `P/📦️packages/🦀️rust/{Cargo.toml, 🦀️.rs, 📋️project.json, 📜️script.ts}`, the façade `P/🦀️.rs`, the root `Cargo.toml` member and `[workspace.dependencies] semio-framework-pets`, `subjectFeatures` (`sut`) in `P/🔮️oracles/🔣️.json`, and K's five adapters. From here on the harness dispatches a Rust subject for every pets case. Add your module with one `#[path] pub mod <name>;` pair in the glue and one `pub use crate::<name>::*;` line in the façade (L's `animation` and `terrain` are already there). `RUSTC_WRAPPER="" cargo check -p semio-framework-pets --features sut --offline` from the repository root checks the real crate (shared build directory); the scratch crates stay useful for quick private runs. If cargo answers `failed to write Cargo.lock … os error 1224`, another cargo holds the lock file mapped — run it again.

## Why a scratch crate

The moment `P/📦️packages/🦀️rust` exists, the Protocol v2 harness demands a Rust adapter for every pets case, and package E is still running parity with the TypeScript subject only. So the module files `P/🔨️modules/<m>/🦀️.rs` and their unit tests are written into the repository, but they are compiled from a private crate under `TK/🗑️generated/<work package>/` until `TK/📓️report-wp-e.md` exists. K creates the real package, the root `Cargo.toml` registration and the `subjectFeatures` entry then.

## Layout (K's, copy it for your own folder)

```
TK/rust_scratch.sh                                             runner: syncs fixtures, sets the private cargo dirs, runs cargo
TK/🗑️generated/wp-k/crate/📦️packages/🦀️rust/Cargo.toml        own `[workspace]` table, package `semio-framework-pets-scratch-k`, lib name `pets`
TK/🗑️generated/wp-k/crate/📦️packages/🦀️rust/🦀️.rs            glue: `#[path]` mounts of the repository files (twelve `../` up to the root)
TK/🗑️generated/wp-k/crate/📦️packages/🦀️rust/Cargo.lock        a copy of the root `Cargo.lock` (cargo trims it; pins the same serde versions)
TK/🗑️generated/wp-k/crate/🧫️fixtures/                         copy of `P/🧫️fixtures`, refreshed by the runner before every run
TK/🗑️generated/wp-k/{build,target}/                           private cargo build and target directories
```

To get your own: `mkdir -p TK/🗑️generated/wp-l/crate/📦️packages/🦀️rust`, copy K's `Cargo.toml` (rename the package to `…-scratch-l`) and `🦀️.rs` into it, `cp Cargo.lock` from the repository root next to them, add your `#[path]` mounts to the glue. Then, from the repository root:

```bash
TK=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS"
bash "$TK/rust_scratch.sh" wp-l test --offline
bash "$TK/rust_scratch.sh" wp-l test --offline animation::          # one module's tests
bash "$TK/rust_scratch.sh" wp-l clippy --offline --all-targets --features sut -- -D warnings
```

First build: 9 s (serde, serde_json, syn); afterwards about 2 s. `--offline` works: serde 1.0.228 and serde_json 1.0.149 are in the local registry cache.

## Mounts and module names (fixed; the real glue will carry exactly these)

```rust
#[path = "<up>/🧰️framework/🛍️products/🐾️pets/🧬️schema/🦀️.rs"]
pub mod schema;
#[path = "<up>/🧰️framework/🛍️products/🐾️pets/🔨️modules/📐️trigonometry/🦀️.rs"]
pub mod trigonometry;
#[path = "<up>/🧰️framework/🛍️products/🐾️pets/🔨️modules/🎲️randomness/🦀️.rs"]
pub mod randomness;
#[path = "<up>/🧰️framework/🛍️products/🐾️pets/🔨️modules/🦴️rig/🦀️.rs"]
pub mod rig;
#[path = "<up>/🧰️framework/🛍️products/🐾️pets/🔨️modules/✅️validation/🦀️.rs"]
pub mod validation;
// L: pub mod animation; pub mod terrain;      M: pub mod behavior; pub mod stage;
```

`<up>` = `../../../../../../../../../../../..` (twelve) from `TK/🗑️generated/<wp>/crate/📦️packages/🦀️rust/🦀️.rs`; in the real glue it is `../..` + `🧬️schema/🦀️.rs` or `🔨️modules/<m>/🦀️.rs`, exactly as in the quiz crate.

Inside module files import with `use crate::schema::{…};`, `use crate::trigonometry::{…};`, `use crate::randomness::{…};`, `use crate::rig::{…};`. Do not import through the crate root (`crate::Species`): the façade `P/🦀️.rs` re-exports everything flat in the real crate, but **a scratch glue must not mount the façade** — it names modules that your scratch crate may not mount (`pub use crate::animation::*` breaks K's crate, `crate::validation` breaks yours if you leave it out). Write `pub use schema::*;` lines in your scratch glue if you want flat names there.

Each module file ends with

```rust
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
```

## The schema twin `crate::schema` (decisions you port against)

| TypeScript | Rust | Note |
|---|---|---|
| `number` | `f64` | every schema `number` |
| `Ticks` | `i64` | `since`, `until`, `blink`, `pointed`, `met`, `tick`, `Ticked.ticks`, `Frame.wake: Option<Ticks>`; `TICKS_PER_SECOND: Ticks = 64` — write `TICKS_PER_SECOND as f64` in float expressions (exact), `x.floor() as Ticks` for `Math.floor(x)` |
| `seed`, `draws` | `u32` | words of a randomness key; `(counter + 1) >>> 0` is `counter.wrapping_add(1)` |
| `facing: 1 \| -1` | `i8` | `f64::from(actor.facing)` in arithmetic |
| `rate: 0 \| 16 \| 32 \| 64` | `u8` | |
| `x: T \| null` (required) | `Option<T>` | `perch`, `partner`, `clip`, `pointer`, `wake`; serialised as `null`, absence is refused |
| `x?: T` | `Option<T>` | `Bone.parent`, `Bone.rotation`, `RectShape.radius`, `Part.stroke_width`, `Face.mouth`, `Face.above`, `Key.ease`, `Locomotion.hover`, `$schema` → `json_schema` |
| `Clip.loop` | `Clip.looping` | `loop` is a keyword; the wire name stays `loop` |
| `Part.strokeWidth` | `Part.stroke_width` | wire name stays camelCase |
| `Ease` | `[f64; 4]` | `Key` is `Copy` |
| `Bond.between`, `Rapport.between` | `[Slug; 2]` | |
| `Shape`, `StageEvent` | enums tagged by `kind` | `Shape::Ellipse(EllipseShape { … })`, `StageEvent::Ticked(Ticked { ticks })`, `StageEvent::Unpointed(Unpointed {})` |
| `Paint`, `Channel`, `Gait`, `Activity`, `PetMode` | `Copy` enums | `Channel::ScaleX` ↔ `"scaleX"`; consts `PAINTS`, `CHANNELS`, `GAITS`, `ACTIVITIES`, `PET_MODES` in contract order |
| `species.repertoire[activity]` | `species.repertoire.clips(activity) -> Option<&[Slug]>` | `Repertoire` is a struct of eleven `Option<Vec<Slug>>` |
| `ACTIVITIES.indexOf(a)` | `ACTIVITIES.iter().position(…)` | `Activity::as_str()` gives the wire string |
| `Slug`, `Color`, `Surface.id`, `Perch.surface` | `String` | |

`Point`, `Rect`, `Size`, `Gaze`, `Needs`, `Temperament`, `EyeFrame`, `Key`, `Locomotion`, the shapes and the scalar events are `Copy`. Every struct derives `Clone, Debug, PartialEq, Serialize, Deserialize` with `deny_unknown_fields`.

## Kinematics (K's modules, names of the TypeScript exports in snake_case)

- `crate::trigonometry::{sin_turns, cos_turns, clamp, lerp, smoothstep}` — all `f64 → f64`.
- `crate::randomness::{random_words(key: &[u32], count: usize) -> Vec<u32>, random_unit(key: &[u32]) -> f64, random_between(key, low, high) -> f64, random_pick(key, weights: &[f64]) -> Option<usize>}` — `None` is the TypeScript twin's `-1`.
- `crate::rig::{Affine = [f64; 6], IDENTITY, compose, invert, transform, BonePose, Pose = Vec<BonePose>, rest_pose, solve_rig(species: &Species, pose: &[BonePose]) -> Vec<f64>, look_offset(eye: Point, target: Point, reach: f64) -> Point}`; `BonePose { x, y, rotation, scale_x, scale_y }` is `Copy` and serialises `scaleX`, `scaleY`.
- `crate::validation::{Issue, IssueCode, menagerie_issues, species_issues, ensemble_issues, assemble_menagerie}`.

## Test kit (reuse it instead of writing a second one)

`crate::schema::tests::{fixture, typed, json, entries, number, bits, assert_same}` (the schema module's test module is `pub(crate)`):

- `fixture("animation-sampling")` reads `CARGO_MANIFEST_DIR/../../🧫️fixtures/<directory ending with that name>/🔣️.json` as a `serde_json::Value`.
- `typed::<T>(&value)`, `json(&typed)`, `entries(&value)` (array or empty), `number(&value)` (panics unless a number).
- `bits(x)` = sixteen hex digits of `x.to_bits()` — the form of the `bits` fields the fixtures carry.
- `assert_same(context, &produced, &expected)` compares two JSON values exactly (numbers by `f64 ==`, so `40` equals `40.0`).

Levels: tests outside a submodule are `fundamental`; slower ones go into `mod quick { use super::super::*; … }`, `mod long`, `mod exhaustive` inside the test file (the budgeted runner skips those by name).

## Bit-exactness rules (design §2.4) as they look in Rust

- Write `0.0 - x` where the TypeScript writes `0 - x`; `-x` would turn `0` into `-0.0`.
- `a * b + c * d` stays as written; never `mul_add`, never `hypot`, `powi`/`powf`, `sin`, `cos`, `exp`, `ln`. `sqrt`, `abs`, `floor`, `min`, `max` are exact — but `f64::min`/`max` differ from `Math.min`/`max` for NaN and for `±0`; where the TypeScript uses a comparison (`a < b ? a : b`), write the same comparison.
- `x === 0` is `x == 0.0`; `!(a > b)` stays `!(a > b)` (NaN keeps the same branch); clippy's `neg_cmp_op_on_partial_ord` may complain — allow it on that function rather than rewriting the comparison.
- `Math.imul(a, b) >>> 0` is `a.wrapping_mul(b)` on `u32`; `>>> 16` is `>> 16` on `u32`.
- Integer → float conversions (`ticks as f64`) are exact below 2⁵³; float → integer after `floor()` with `as i64`.
- serde_json parses the committed decimals to the same doubles as `JSON.parse` for the short literals the fixtures use; compare against the `bits` fields where a fixture has them, otherwise exact `f64` equality is what the cross-subject parity will demand (TypeScript and Rust adapters print through shortest-round-trip formatting on both sides).

## Adapters (`P/🧪️tests/<case>/🦀️.rs`) before the package exists

The harness compiles an adapter into a generated host crate that depends on **two crates only**: `semio-repo-test-host` and the product crate with feature `sut`. Consequences:

- Everything comes through `pets::…`: `use pets::serde_json::{self, json, Map, Value};` and the product's own names (`use pets::{sin_turns, Species, …};` — flat, through the façade).
- `serde` itself is not nameable in an adapter: a helper generic over `serde::de::DeserializeOwned` does not compile there. Decode with concrete types (`serde_json::from_value::<Pose>(value.clone())`).
- A projection is built as a `serde_json::Value` and handed over with `Outcome::projection(parse_json(&value.to_string())?)`; object key order does not matter, array order does. Project `-1` where the TypeScript adapter does (`random_pick(…).map_or(-1, |index| index as i64)`), `null` where it projects `null`.
- Bit patterns: `format!("{:016x}", value.to_bits())` equals the TypeScript adapters' `bits()`.

K rehearses its five adapters without the package: `TK/🗑️generated/wp-k/host/{Cargo.toml, main.rs}` is a stand-in for the generated host (it mounts the draft adapters by `#[path]`, picks one by `PETS_CASE` and calls `semio_repo_test_host::run_main`), built with `SCRATCH_CRATE=host bash TK/rust_scratch.sh wp-k build --offline --features sut`. `bun TK/rehearse_rust_adapters.ts` then writes a plan per case, runs the host as the harness would (`host --plan … --out …`), calls the committed TypeScript adapter of the same case in-process and compares both projections value by value, exactly. Result for K: 5 cases, 23 scenarios, 7 085 values, 0 differences. Copy the two host files and the tool (its `CASES` list and `PETS_CASE` match arms) to rehearse yours.

## Pitfalls hit so far

0. **serde_json parses long decimals one unit in the last place off unless its `float_roundtrip` feature is on.** Measured: the committed unit `0.9421396472025663` came back as `0.9421396472025664` (a decimal whose digits exceed 2⁵³ leaves serde_json's exact path). JavaScript's `JSON.parse` and Python are correctly rounded, so a Rust adapter or unit test that reads such an input would feed its subject a different double than the TypeScript adapter does — `🎞️animation-sampling` has 26 such inputs (`ease` values), `🧠️behavior-choice` 72 (`units`). The scratch `Cargo.toml` therefore asks for `serde_json = { version = "=1.0.149", features = ["raw_value", "float_roundtrip"] }` (dependency under `sut` and dev-dependency), and the real crate will carry `features = ["float_roundtrip"]` on both lines. With it, every fixture number is the double the other languages read, and exact comparisons against `expected` and `bits` hold (K: 50 of 50 tests, all committed bit patterns). The harness host's own `parse_json` is `str::parse::<f64>` (exact) and prints shortest round-trip, so projections travel unharmed.
1. `.cargo/config.toml` of the repository applies to every cargo run below the root, also to a scratch crate: its shared `build-dir` and the unstable new layout. The runner overrides them (`CARGO_BUILD_BUILD_DIR`, `CARGO_TARGET_DIR`, `CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false`, `CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false`) so nothing waits on other teams' locks, and sets `RUSTC_WRAPPER=""` (sccache is configured on this host but not installed).
2. `CARGO_MANIFEST_DIR` of a scratch crate is not the real package directory, so the fixtures are copied beside the crate (`crate/🧫️fixtures`), two levels above the manifest, exactly where the real package finds them. Do not replace the copy by a junction: a later `rm -rf 🗑️generated` could follow it into the product.
3. Read files that sit beside the test with `include_str!("../../🔣️.json")` (relative to the test file) — that needs no path arithmetic and works in both crates.
4. Create and change repository files with the Write/Edit tools only; heredocs on this host eat backslashes.
5. The docstring gate wants an emoji as the first token of every `//!` and `///` run; unit-test functions carry no docstring.
6. Clippy with the workspace lint set flags `needless_pass_by_value` (take `&T` or make the type `Copy`) and `unnecessary_wraps`.
