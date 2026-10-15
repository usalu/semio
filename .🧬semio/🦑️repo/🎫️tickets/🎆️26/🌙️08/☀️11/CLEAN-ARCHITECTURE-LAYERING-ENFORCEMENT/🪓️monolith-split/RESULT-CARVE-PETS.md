# Result: `semio-framework-pets` carved into seven producer crates plus a thin facade

Executor: Sonnet 5.5, 2026-10-15. Tools and scripts: `🪓️monolith-split/pets-carve-tools/` (`plan.py`, `carve1.py` to `carve5.py`, `deps.py`, `facade.py`, `oracle.py`, `verify.ts`, `census-test.ts`, `lossless.py`, `fmt.sh`). Logs: `🗑️generated/pets-census-after.log`, `pets-cargo-check-contract.log`, `pets-cargo-update.log`, `pets-ts-test.log`.
`P` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🐾️pets`.

## Status in one paragraph

The split is done and statically proven equivalent: all seven new crates and the facade pass a real, full type-check chain (`rustc --emit=metadata` with real serde and serde_json, one `--extern` rmeta per crate), natively and for the `--test` selection, with 0 errors and 0 warnings, exactly like the pre-split baseline (0 / 0). The sanctioned launcher `bun repo cargo` is down for an unrelated reason ("Finite original maintenance authority required", before any compile), so **no cargo check and no unit test has been run**; the unit tests are type-checked, not executed. Two things the launcher would have done are left for it: the root `Cargo.lock` refresh (a peer's partial refresh already contains the new packages, but without the dev-dependency and optional edges, expected diff below) and the real `check`/`test` runs.

## What the investigation found

- The "shared modules tree" is the product's own `🔨️modules/` (25 module folders plus `🧬️schema/`). Each module folder also holds its TypeScript twin and its `🧪️tests/🔬️unit/` pair, and stays where it is.
- **No other crate mounts or depends on these files.** Evidence: `git grep` over every `*.rs`, `*.toml`, `*.json`, `*.ts` outside the ticket folder for `semio-framework-pets`, `semio_framework_pets`, `pets::` and `🐾️pets/` found only the root `Cargo.toml` (member plus an unused `[workspace.dependencies]` row), `Cargo.lock`, the pets package itself, the owner registry, launch entries, README and the oracle registry text. Only the 23 Protocol v2 subject adapters (`🧪️tests/*/🦀️.rs`) use the crate, through `use pets::{…}` (lib name `pets`), linked as the single "subject package" by `🦑️repo/🔨️modules/🧪️test/🕸️dependencies/🟨️.mjs` (`rustSubjectPackage`). There is no TS or wasm binding of the Rust crate (no `cfg(target_*)` anywhere in the tree). So there was no inverted layering: every module file now has exactly one owning crate (`verify.ts`: 32 files checked, each owned once).
- Because the test host links exactly one package at `📦️packages/🦀️rust` as `pets`, the cleanest layout keeps `semio-framework-pets` as the thin facade instead of editing the 23 adapters and the oracle registry.

## Resulting layout

| Crate | Node (lib root `<node>/🦀️.rs`, package `<node>/📦️packages/🦀️rust`) | Modules (files stay in place, mounted with `#[path]`) | Depends on |
|---|---|---|---|
| `semio-framework-pets-contract` | `P/🔨️modules/📜️contract` | `🧬️schema`, `✅️validation`, plus `testkit` (feature) | - |
| `semio-framework-pets-kinematics` | `P/🔨️modules/⚙️kinematics` | trigonometry, randomness, rig, animation, terrain | contract |
| `semio-framework-pets-clearance` | `P/🔨️modules/🚧️clearance` (the module file is the lib root) | clearance | contract, kinematics |
| `semio-framework-pets-traversal` | `P/🔨️modules/🧭️traversal` | swing, climbing | contract, kinematics |
| `semio-framework-pets-interaction` | `P/🔨️modules/🖱️interaction` | gesture, effects, mischief | contract, kinematics, traversal |
| `semio-framework-pets-character` | `P/🔨️modules/🎭️character` | behavior, feeling | contract, kinematics; dev: interaction |
| `semio-framework-pets-staging` | `P/🔨️modules/🎬️staging` | draft, spacing, schedule, attention, locomotion, sociability, choice, population, clock, projection, stage (only `stage` is `pub`, as before) | contract, kinematics, character, interaction |
| `semio-framework-pets` (facade, lib `pets`) | `P/🦀️.rs` (lib root), `P/📦️packages/🦀️rust` | none; 16 explicit `pub use child::module::{…}` statements (480 names, exactly what the 23 adapters import) plus `pub use serde_json` | all seven |

The DAG points downward only (`verify.ts` DFS over normal and dev edges: no cycle; character's dev edge on interaction is acyclic because interaction does not depend on character). Cohesion: contract = the normative types and their validation; kinematics = pure numerics, rig, animation, ground; traversal = everything that hangs or climbs; interaction = learner, page and particles; character = behaviour and feeling in numbers; staging = the fold of events into frames. clearance is its own seam (no other module uses it, its proof suite is 2,258 test lines).

Why seven and not fewer: the test selection (`cfg(test)`) of each crate also has to stay inside the 4,000 / 32 / 262,144 gate, and the unit suites are as large as the sources (clearance 2,258 lines, climbing 1,228, terrain 835). The partition keeps every test selection under the gate too.

## Changes beyond wiring (all minimal, all compile-checked)

1. `terrain::larger` and `terrain::smaller`: `pub(crate)` to `pub`. Both are used by clearance, climbing, mischief, effects, gesture and the staging modules, now across crates. A negative control (copy of the tree in the scratchpad with `larger` private again) fails with E0603 in clearance and traversal, so the oracle does catch it. No other item, field or method needed widening (full type-check, so field and method privacy are covered).
2. Every `crate::<module>` that now lives in another crate became `semio_framework_pets_<crate>::<module>` (119 `use` lines, 3 inline test uses hoisted to imports). Module bodies are otherwise byte-identical: `lossless.py` (normalised line multiset against `git HEAD`) shows differences in 5 of 50 files, all of them these edits.
3. **Shared test kit.** The helpers every module suite shared (`fixture`, `typed`, `json`, `entries`, `number`, `bits`, `assert_same`) were `pub(crate)` inside the schema unit tests and are unreachable across crates. They moved to `P/🧬️schema/🧪️tests/🧰️testkit/🦀️.rs`, mounted by the contract root as `#[cfg(any(test, feature = "testkit"))] pub mod testkit`; feature `testkit = ["dep:serde_json"]` (the same pattern the peer quiz carve uses). Dependants enable it as a dev-dependency feature.
4. **Latent bug fixed on the way.** The old kit read fixtures from `CARGO_MANIFEST_DIR/../../🧫️fixtures` = `P/🧫️fixtures`, which today holds only empty case directories; the vectors live in `P/🧪️tests/🧫️fixtures` (24 cases, e.g. `⚗️chemistry-rules/🔣️.json` present). The kit now reads `…/../../../../🧪️tests/🧫️fixtures`. Not executed.
5. The schema unit test embedded the 111,034-byte normative `🔣️.json` with `include_str!`, which alone pushed the contract test closure over the byte gate (280,680). It now reads it at run time through `testkit::normative()` (CARVING-RULES 3). Contract test closure: 169,869 bytes.
6. Facade: `P/🦀️.rs` (formerly the "component" with glob re-exports) is now the lib root with explicit re-exports; `P/📦️packages/🦀️rust/🦀️.rs` (the `#[path]` glue) is deleted; manifest depends on the seven crates and `serde_json` (float_roundtrip kept), `serde` dropped. `📜️script.ts` `test` now runs all eight packages (bottom up), so `@semio-tech/pets-rs:test` and the launch entry `🧪️test🐾️pets🦀️` cover the whole Rust core; `build` still builds the facade manifest and therefore everything.
7. README (`P/README.md`): layout table rows for the seven crates, façade row, test command note.

## Wiring done (recipe steps)

- Manifests: seven new `Cargo.toml` (role `product`, id `pets-<x>`, exactly the three `rust-source-boundary` keys at the ceiling, `[lints] workspace`, `[lib] path = "../../🦀️.rs"`), plus facade manifest rewritten with the same boundary table.
- `📋️project.json` per new crate (`@semio-tech/pets-<x>-rs`, tags `lang:rust`, `role:product`, `family:pets` like the existing `pets-rs`; targets are inferred as in the recipe). The facade project keeps its explicit `build`/`test` targets.
- Root `Cargo.toml` `[workspace] members`: seven rows inserted, byte-sorted (the list was verified sorted before and after).
- `📋️build-owners.json`: seven rows (`root` = package dir, `manifest` = its `📋️project.json`, `packages: []`) inserted inside the existing `@semio-tech/pets*` block. The file is not globally sorted (a peer-added row breaks it near `framework-job-worker`), so the rows follow plain string order locally.
- Six launch routes per crate (42), identical in `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`, inserted right after `🧪️test🐾️pets🦀️`: names `🦀️ <crate> 🏗️build|🔎️check|🧪️test 🛠️prototype|🚀️release`, groups `4_build` / `4_gate` / `4_test`, orders `900.01401`-`900.01415`, `922.04401`-`922.04414`, `903.58401`-`903.58414`, all unused as numbers (one candidate, `900.01410`, equals the existing `900.0141` numerically and was moved to `900.01415` in both files). Both files parse; every command exists exactly once.
- Corpus row: **not applicable**. The boundary corpus (`🖱️ui/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json`) is UI-only and its test rejects forbidden UI dependencies; pets has no such area corpus.
- No consumer outside the pets package needed an edit (there is none). The 23 adapters and `🔮️oracles/🔣️.json` are unchanged and still resolve against the facade (check below).

## Census rows (canonical `readRustSourceClosureV1`, `🗑️generated/census.ts`, host cfg; limits 4000 lines / 32 files / 262,144 bytes)

| Crate | Before | After: files / bytes / active lines | Test selection (cfg test, testkit): files / bytes / lines |
|---|---|---|---|
| `semio-framework-pets` | 28 / 556,044 / 11,324 (ratio 2.831, over) | 1 / 8,468 / 65 | 1 / 8,468 / 65 |
| `-contract` | new | 3 / 112,087 / 2,685 | 6 / 169,869 / 3,570 |
| `-kinematics` | new | 6 / 50,929 / 1,090 | 11 / 164,649 / 3,352 |
| `-clearance` | new | 1 / 34,835 / 734 | 2 / 140,111 / 2,994 |
| `-traversal` | new | 3 / 76,343 / 1,355 | 5 / 189,599 / 3,151 |
| `-interaction` | new | 4 / 75,699 / 1,504 | 7 / 154,616 / 3,063 |
| `-character` | new | 3 / 84,282 / 1,422 | 5 / 141,562 / 2,518 |
| `-staging` | new | 12 / 123,965 / 2,524 | 21 / 179,901 / 3,481 |

All eight rows: `overLimit false`, `closureCompilable true`, `oversizedDeps 0`; the final workspace census run shows 140 compilable crates and 25 oversized (it was 125 and 31 at my first run; peers carved other crates in the meantime, so that change is not all mine; `pets` is no longer a carvable leaf). Every crate and every test selection is below 3,600 lines, 21 files and 190,000 bytes. The test column comes from `pets-carve-tools/census-test.ts` (same counter with cfg `test`, plus `feature="testkit"` for the contract).

## Verification (every command run, with its actual result)

1. Baseline before the first edit: census row above (28 / 556,044 / 11,324), and a pristine copy of the pets tree from `git archive HEAD` in the scratchpad.
2. **Full type-check oracle.** Instead of the `-o /dev/null` metadata probe (which stops at resolution errors because serde is missing), I used the real serde 1.0.228 and serde_json 1.0.149 metadata that exists in another checkout's build output (`/Users/ueli/Documents/semio-kinan-wip/target/debug/deps`, read-only, built by `nightly-2026-07-07`; the pair `libserde-2119fe5726a5eff0` plus `libserde_json-7081bf7b8f264137` shares one `serde_core`, found with `-Zls=root`). Each crate is compiled in topological order with `rustc +nightly-2026-07-07 --edition 2021 --crate-type lib --emit=metadata -C extra-filename=-pets --out-dir <scratchpad> … --extern <dep>=<scratchpad>/lib<dep>-pets.rmeta <lib root>` from its package directory with `CARGO_MANIFEST_DIR` set, and with `--test` for the unit selection (contract compiled with `--cfg 'feature="testkit"'` for the test chain). Outputs are small rmeta files in the session scratchpad only; no cargo, no `RUSTC` override, no scratch workspace, no private target dir, nothing written to the gated store. This deviates from `📓️RESOLUTION-ORACLE.md` in two honest ways: toolchain `nightly-2026-07-07` instead of `2026-07-20`, and real dependency metadata instead of a stubbed or unresolved one (so it type-checks, not only resolves).
   - Baseline (original crate, pristine copy): lib 0 errors / 0 warnings; `--test` 0 / 0.
   - Split, native: contract, kinematics, clearance, traversal, interaction, character, staging, facade: 0 errors, 0 warnings each.
   - Split, `--test`: all seven crates, lib and test targets, plus the facade: 0 errors, 0 warnings.
   - Negative control (scratchpad copy with `larger` back to `pub(crate)`): E0603 reported in clearance and traversal.
   - Adapter imports: the `use pets::…` statements of all 23 subject adapters, compiled against the facade metadata: exit 0, 0 errors.
   Limits: this checks types, borrowing and privacy, but nothing was code-generated or executed, and only the host target was checked (no wasm: the crate has no wasm consumer and no wasm serde artifacts were available).
3. Losslessness (`lossless.py`): against HEAD, 45 of 50 module and test files are identical after normalising `semio_framework_pets_<x>::` to `crate::`; the other five differ only by the intended edits above (terrain visibility, three import hoists, the moved test kit).
4. rustfmt (`fmt.sh check`, `rustfmt +nightly-2026-07-20 --edition 2021 --check --config skip_children=true,unstable_features=true`, repo `rustfmt.toml`): all touched files clean. Two test files were already unclean at HEAD (`🎪️stage` and `🧬️schema` unit tests, long lines); the stage file was formatted completely, the schema test only had its import lines edited and keeps its pre-existing wrapping debt.
5. `verify.ts` (`bun`): TOML of all eight manifests parses (`Bun.TOML`), package names match, `package.workspace` resolves to the repo root, path dependencies resolve to manifests of the expected name, boundary keys exact, workspace membership, DAG, every `#[path]` target exists and each module file is owned once, longest path in the pets tree is 126 bytes, `launch.json`, the seed, `📋️build-owners.json` and all `📋️project.json` parse, owner rows and all 84 launch commands present exactly once and order keys unique. Result: `ALL STATIC CHECKS PASSED`.
6. `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-pets-contract --message-format=short`: failed after the cache stage with `error: Finite original maintenance authority required` (log `🗑️generated/pets-cargo-check-contract.log`), before any compile. The same error for `-- update --workspace --offline` (`pets-cargo-update.log`). So no gate verdict and no real `check` or `test` exists for any of these crates.
7. `bun repo run @semio-tech/pets:test` (the TypeScript twin, untouched by this change): killed by my 290 s alarm while the launcher was still building its graph (`build:selected-compile-imports`), no result. No TypeScript file was changed. `bun ./📜️script.ts test` called directly is refused ("Original parent Script process invocation required").
8. `bun repo verify` was started once and stopped after it only printed lease-store progress; not a result.

## Lock file

Not refreshed by me (launcher down). The root `Cargo.lock` was partly refreshed by a peer around 11:49, when the new crates were still only path dependencies of the facade, so it lists the eight packages but without the edges Cargo adds for workspace members. When the launcher works, `update --workspace --offline` should change exactly these entries: contract gains `serde_json`; kinematics, traversal gain `serde_json`; interaction gains `serde_json`; character gains `semio-framework-pets-interaction` and `serde_json`; staging gains `serde_json`; clearance and the facade stay as they are. Until then a `--locked` resolve would report the lock as out of date.

## Open items for the next wave

- Run the real `check` and `test` for the eight packages once `bun repo cargo` works again (the unit suites have never been executed against the moved fixture path and the run-time schema read), then refresh the lock.
- `P/🧫️fixtures/` (untracked, ignored) still holds 24 empty case directories left over from the fixture move; harmless now, safe to delete. The `@see ../../../../🧫️fixtures/…` doc lines in the module unit tests still point at it.
- The hand-written `testkit` and a future `semio-framework-pets-…` consumer should depend on the child crates directly instead of the facade; the facade exists for the test host's single-package contract.
- Not done on purpose: renaming lib targets to short names (the quiz peer used `quiz_schema`); the crates keep the recipe default `semio_framework_pets_<x>`.
