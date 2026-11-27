# Twins: feature-test gaps of the bim model subset

Scope: `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/{💰️costs,🧙️modify-bim-1,🛠️gestures-bim-1}`.

## Findings that changed the plan

- Runners are not registered in any Cargo.toml. The host materializes a throwaway crate per case from the taxonomy (`🖥️host/🏗️materialization`), compiles the committed `🦀️.rs` as `mod adapter`, and links the owner crate plus the registry's `subjectHostPackages` under `--features sut`. `repo test-file` accepts TypeScript and JavaScript only, so Python oracles run as `python 🐍️.py check|write <fixture>` in the locked env and as host oracle handlers through `adapter()`.
- `modify-bim-1` and `gestures-bim-1` could never be discovered: their `@mode-scenario` tag is not a mode (`differential|conformance|round-trip|property|error`), their Scenario Outlines had no kebab-case `id` column, and their oracle/capability tags (`bim-1-numpy-shapely-geometry`, `bim-1-author`, `bim-1-modify`) were not in the oracle registry. `💰️costs` had no runner, no oracle, no `@id`/`@level`/`@mode` tags.
- The bim subset registry declared no `subjectHostPackages`, yet all 36 existing runners `use` crates other than the owner crate (snapshot, tests-5, mut-any-190, ...). Without them no `--features sut` host compiles. Added (see below). This affects every bim case, not only mine.
- `💰️costs` had only a TypeScript implementation (`🧬️schema/💡️inferences/💰️costs/🟦️.ts`), no Rust crate. A Rust twin was written.

## Done

- `💰️costs`: `🧪️.feature` renamed to `🥒️.feature` (tags, 4 `@id`s, fixture URI, one doc string for the diagnostics scenario); new `🦀️.rs` runner (4 subject handlers); new `🐍️.py` oracle with `sympy` rationals (4 oracle handlers plus `check`).
- New crate `semio-s-artifact-bim-model-costs` (`🧬️schema/💡️inferences/💰️costs/🦀️.rs` and `📦️packages/🦀️rust/{Cargo.toml,🦀️.rs}`), a Rust twin of the TypeScript `inferCosts` with exact decimal arithmetic (own digit-vector decimal, half-up, no external crate). Added as a member of `🏢️model/Cargo.toml` (one line, before quantities).
- `🧙️modify-bim-1`: new `🦀️.rs` runner (8 subject handlers: maps, frames, loops, offsets, trims, splits, arrays, joins, all through the modify kernel and `compute_wall_layout`); oracle gets `adapter()` with the same 8 handlers (it already had numpy+shapely `check`/`write`, still passes: 36 cases agree). Feature: `id` column in every Examples table, joins scenario names the joins model fixture, `@mode-scenario` scenarios `modify-gestures` and `modify-hotkeys` removed (they need a window; they are proved by `✏️editor/🧵️gestures/🧪️tests/🔬️unit`).
- `🛠️gestures-bim-1`: new `🦀️.rs` runner (6 subject handlers: typed-point per row, cursor-point, component-fit, component-frame, component-turns, route-length, through `typed::parse/resolve`, the `Rig` tool session, `place::fit/Frame`, the route tool); oracle gets `adapter()` with the same 6 handlers. Feature: `id` column for typed-point, `component-place` replaced by differential `component-turns` (the numpy `turns` cases), `typed-wall`, `cursor-wall`, `component-host`, `route-keys` removed (window-bound, covered by the unit tests of the tools).
- Registry `✳️any/🔮️oracles/🔣️.json`: `bim-1-shapely-geometry` gains capability `bim-1-modify`; new oracles `bim-1-numpy-shapely-geometry` (`bim-1-author`) and `bim-1-sympy-costs` (`bim-1-infer`); new `subjectHostPackages` (23 crates: the 21 the runners import plus costs and the framework geometry kernel; `bim` with feature `testkit` for the `Rig` fixture). Plugin registry `🔮️oracles/🔣️.json`: `oracleHostPackages` gains numpy 2.5.0 and sympy 1.14.0.

## Verified

- Oracles (locked Python env): `💰️costs/🐍️.py check` agrees (sympy 1.14.0); `🧙️modify-bim-1/🐍️.py check` agrees (36 cases); the gestures oracle `check` is unchanged from before. The costs diagnostics scenario run by hand through the oracle yields 8 diagnostics (3 item-invalid, 2 link-invalid, 1 item-missing, 2 overflow) and unchanged totals.
- Features: `twins-features.test.ts` (this ticket folder, via `repo test-file`): all three features parse with zero errors, every scenario id is valid, and each feature's oracle id, capability and comparison profile resolve in the loaded registry (3 pass).

## Not verified, and why

- No Rust compile of the new crate, the three runners or the host. `repo cargo` and `repo run ...:test-contract` are queued behind the shared host capacity gate (needs 3.5 GB; other sessions hold the memory; `repo run test-contract` ran the full 540 s without starting because the replica never settles for its 900 s window). Native cargo and direct bun are refused by the compiler ingress / caller-authority gates. Log: `🗑️generated/twins-costs-check.log` (cargo check of the costs crate, still waiting at the time of writing). Rust code was desk-checked against the signatures of the crates it calls; remaining compile risk is the guessed names: `cursor_keys::*::DELTA` (inherent const vs trait), `Component` fields, `CutFlaw::message`, `create.mep.path`.
- Differential equality of subject and oracle (host parity run) has not been executed. Quantities that meet shapely only through sampled arcs (loops, offsets, splits) are rounded to a 1e-4 or 1e-6 grid on both sides, so a boundary flip is possible but unlikely.

## Open

- `🎲️infer-bim-1-solids-three`, `🧊️export-bim-1-gltf`, `🏔️solids-bim-1-wall-depth-three`: TypeScript (three.js) oracles only; no `🐍️.py`. Left as is (a `three` oracle is third-party and registered); decide whether a Python twin is wanted.
- Missing `bim-1-bcf-xml`, `bim-1-jsonschema-numpy-energy` oracle registrations for the bcf and energy features (not mine).
- `🔣️taxonomy.json` `members-of-tests` lists only 14 of the 40 bim case names; `💰️costs` and `🧙️modify-bim-1` are not in it (as 24 others are not); regenerate it.
- The new costs crate `Cargo.toml` and `🦀️.rs` mount were written by hand in the shape of the carved crates; if the carve generator owns them it must learn the costs component. `Cargo.lock` of the model workspace changes by one package once cargo runs.
- Window-bound scenarios removed from the modify/gestures features need a headless window harness to return as host scenarios.

## Files

Created: `🧬️schema/💡️inferences/💰️costs/🦀️.rs`, `.../💰️costs/📦️packages/🦀️rust/{Cargo.toml,🦀️.rs}`, `🧪️tests/💰️costs/{🦀️.rs,🐍️.py}`, `🧪️tests/🧙️modify-bim-1/🦀️.rs`, `🧪️tests/🛠️gestures-bim-1/🦀️.rs`, `$T/twins-features.test.ts`, `$T/twins.md`.
Renamed: `🧪️tests/💰️costs/🧪️.feature` to `🥒️.feature` (rewritten).
Edited: `🧪️tests/🧙️modify-bim-1/{🥒️.feature,🐍️.py}`, `🧪️tests/🛠️gestures-bim-1/{🥒️.feature,🐍️.py}`, `✳️any/🔮️oracles/🔣️.json`, `✏️s/🔌️plugins/🏙️bim/🔮️oracles/🔣️.json`, `🏢️model/Cargo.toml` (one member line).

## Handoff (compile verification needed, nothing was compiled)

The `repo cargo check` of the costs crate was cancelled at the coordinator's request (my PIDs 95939 and 95940, no children left). No build of mine is running. All Rust below is desk-checked only.

Guessed names, in `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/`:

- `🧬️schema/💡️inferences/💰️costs/🦀️.rs` (new crate source): uses `std::iter::repeat_n` (needs Rust 1.82+); workspace lints (`[lints] workspace = true`) may flag missing docs or similar. The crate is the one new `🏢️model/Cargo.toml` member (`...💰️costs/📦️packages/🦀️rust`); its `Cargo.toml` and `🦀️.rs` mount were hand-written in the carve shape (mount path `"../../🦀️.rs"`).
- `🧪️tests/💰️costs/🦀️.rs`: `Json` and `Outcome` re-exports from `semio_repo_test_host`; destructuring assignment into `ModelCosts` fields is in the crate, not here.
- `🧪️tests/🧙️modify-bim-1/🦀️.rs`: `Component` has exactly the 9 fields copied from the unit test (storey, family, position, elevation, rotation, mirrored, host, system, name); `Wall { .., ..template }` fields; `EndJoin`/`LocationLine` under `schema::snapshot::values`; `CutFlaw::message()` and `TrimFlaw::message()` exist (seen in source, not compiled); `offset_axis` returns `Option`; `compute_wall_layout` returns `BTreeMap<String, WallLayout>` with `.footprint[..].point`; `loops::centroid` returns a type with `.x/.y`; `semio_framework_geometry` re-exports `loops` and `Point`.
- `🧪️tests/🛠️gestures-bim-1/🦀️.rs`: `cursor_keys::{CursorLeft, ...}::DELTA` as an inherent associated const (the unit test imports `cursor_keys::*`, so it may be a trait const needing a trait import); `create.mep.path` (Vec of `Point3` with x,y,z) on `ModelMutation::CreateMepElement`; `create.component.rotation` on `CreateComponent`; `Rig` public fields `session` and `preview` and methods `plan/nudge/typed/key/down/mv/finish`; `Step.refused` and `Step.mutations` Debug; `GestureKey::Turn(f64)`; `place::{UTILITY, fit, Frame}` from `semio_s_artifact_bim_model_move_storey`; `route::UTILITY` from `..._gestures`; `typed::{parse, resolve}` from `..._coordination`; `plane::point2` from `..._storey_levels`.
- `🔮️oracles/🔣️.json` `subjectHostPackages` (23 entries, `bim` with feature `testkit`): features were not checked for the other crates (`tests-5`, `ifc`, `gestures`, `kit` not included). Paths came from the member list of `🏢️model/Cargo.toml`. A package named in a runner but missing from the list fails the host build with an unresolved import. `semio-framework-geometry` path was taken from the root `Cargo.toml`.
- Host-level: row ids (`translate-0`, `case-3`, `0`) are parsed by taking the part after the last `-`; the contract checks (`test-contract`) and a parity run (`test-parity -- quick --case <slug>`) were never executed.
