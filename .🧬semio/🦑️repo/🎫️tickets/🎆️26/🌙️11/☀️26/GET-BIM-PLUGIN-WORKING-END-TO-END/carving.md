# Carving the over-budget BIM model crates

## Pattern found
- The warden message (`unit {key} peaked at … not on the carving list: carve it along the domain taxonomy or declare it`) comes from the admission policy (`carving` list and families, schema under `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission`). The list "may only shrink", so declaring is the last resort.
- Carving = the repo `new crates <artifact-package-dir>` command (`🏗️authoring/📦️crate-tree`, budgets in source files/bytes/lines). It places every taxonomy folder's `🦀️.rs` into a crate at `<folder>/📦️packages/🦀️rust` that mounts the unchanged files with `#[path]` (path-preserving, nested `#[path = "."]` inline modules), plus `extern crate … as dsl|protocol|store|schema|value_derive` aliases, a manifest cloned from the artifact (paths rebased), `testkit` feature forwarding, workspace member, `📋️build-owners.json` row and the artifact root manifest/script rows.
- Its budget is source size, so these three crates (small sources, derive/macro heavy) were not split by it; the same layout was applied by hand. Inherent impls stay in the crate of their type (the `*-values` impl files), and consumer paths stay stable through explicit `pub use` facades.

## Result
| Crate | Before | After |
|---|---|---|
| `semio-s-artifact-bim-model-schema` | descriptor + 8 snapshot value files (82 KB derives) + issues/thermal/codec mounts | facade: descriptor, codec, issues, thermal; value mounts replaced by `pub use` of the four value crates |
| new `…-geometry-values` | | points, vertices, profiles, solid shapes, anchors (+ `🧲️anchors-values` impls) |
| new `…-properties-values` | | property and schedule values (+ phases, schedule-kit, property-kit impls) |
| new `…-issues-values` | | element classes, rules, issues, camera (+ `⚙️coordination-values` impls); uses geometry, properties |
| new `…-values` | | remaining value types (`💠️values`) (+ views, sheets impls); re-exports all upstream values so `snapshot::values` is unchanged |
| `semio-s-artifact-bim-model-kit` | component + utilities + 5 windows | facade re-exporting `kit`, `utilities`, `modes::edit::windows::{plan,schedule,section,sheet,world}` |
| new `…-kit-core` | | `🧰️kit/🦀️.rs` (helpers, id mint; `window_config` re-export now `pub`) |
| new `…-utilities` | | utility registry; owns `window_kinds::{PLAN,SECTION,SHEET,WORLD}` (breaks the utilities <-> windows cycle; plan/section/sheet `WINDOW_KIND_ID` point there) |
| new `…-window-plan|world|section|schedule|sheet` | | one crate per window folder, depend on kit-core (+ utilities) |
| `semio-s-artifact-bim-model-interaction` | component + vocabulary + 2 entity test mounts | component + vocabulary + own test |
| new `…-entities-tests`, `…-entities-families-tests` | | the two entity table test files, mounted `#[cfg(test)]` at their original module path |

Crate dirs: new values crates sit beside the snapshot folders (`📐️geometry-values`, `🔍️properties-values`, `🚨️issues-values`, `💠️values`), kit crates in `✏️editor/🧰️kit/🧱️core`, `✏️editor/🪛️utilities`, `✏️editor/🎭️modes/✏️edit/🪟️windows/<w>`, tests in their `🧪️tests/🔬️unit` folders.

## Registrations
- Model workspace `Cargo.toml` members (13); artifact root `📦️packages/🦀️rust/Cargo.toml` deps, dev-deps and `testkit`; its `📜️script.ts` package list.
- `📋️build-owners.json` rows; `.cargo/config.toml` per-package opt-level/codegen-units overrides moved from `kit`/`schema` to the five window crates and four value crates.
- bim `🔮️oracles/🔣️.json` `subjectHostPackages`: the four value crates (the list enumerates crates, `schema` stays listed).
- `Cargo.lock` files are left to cargo (no `--locked` configured).

## Files
Generator and log inputs: `carve-bim.py` (this folder, run once). Edited sources: `🧰️kit/🦀️.rs` (`pub use window_config`), `🪛️utilities/🦀️.rs` (`window_kinds`), `WINDOW_KIND_ID` in plan, section, sheet window files, `🕹️interaction/🦀️.rs`, `🧬️schema/📦️packages/🦀️rust/🦀️.rs`, `💠️values/🦀️.rs` (split).

## Verification
See the end of this file (filled after `repo cargo check`).
