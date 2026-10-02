# 📓️ Work package K (Rust twin: package, schema, validation, kinematics) — report

Ticket `2026/10/02/QUIZ-PETS`, phase 2. `P` = `🧰️framework/🛍️products/🐾️pets`, `TK` = this ticket folder, `TEST` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`. Everything below was run on 2026-10-02 on the Windows host (cargo 1.99.0-nightly 2026-07-05, bun 1.4.2). Raw outputs: `TK/🗑️generated/wp-k/`.

## 1. What exists

| File | Content |
|---|---|
| `P/🧬️schema/🦀️.rs` | serde twin of all 57 `$defs` entries under their names (`deny_unknown_fields`, `kind`-tagged `Shape` and `StageEvent`, optional `$schema` as `json_schema`, members that may be `null` must be present), the constants `LANGUAGES`, `TICKS_PER_SECOND`, `PAINTS`, `CHANNELS`, `GAITS`, `ACTIVITIES`, `PET_MODES`, `MENAGERIE_SCHEMA`, `ENSEMBLE_SCHEMA`, and two accessors (`Activity::as_str`, `Repertoire::clips(activity)`) |
| `P/🧬️schema/🧪️tests/🔬️unit/🦀️.rs` | 8 tests and the shared test kit (`fixture`, `typed`, `json`, `entries`, `number`, `bits`, `assert_same`) |
| `P/🔨️modules/📐️trigonometry/🦀️.rs` (+ unit tests, 15) | `sin_turns`, `cos_turns`, `clamp`, `lerp`, `smoothstep` |
| `P/🔨️modules/🎲️randomness/🦀️.rs` (+ unit tests, 11) | `random_words`, `random_unit`, `random_between`, `random_pick` |
| `P/🔨️modules/🦴️rig/🦀️.rs` (+ unit tests, 12) | `Affine`, `IDENTITY`, `compose`, `invert`, `transform`, `BonePose`, `Pose`, `rest_pose`, `solve_rig`, `look_offset` |
| `P/🔨️modules/✅️validation/🦀️.rs` (+ unit tests, 11) | `Issue`, `IssueCode`, `menagerie_issues`, `species_issues`, `ensemble_issues`, `assemble_menagerie` |
| `P/🦀️.rs` | façade: `pub use crate::<module>::*` (L has added `animation` and `terrain`) |
| `P/📦️packages/🦀️rust/{Cargo.toml, 🦀️.rs, 📋️project.json, 📜️script.ts}` | crate `semio-framework-pets` (lib `pets`, `[lints] workspace = true`, feature `sut = ["dep:serde_json"]`), glue with `#[path]` mounts, nx project `@semio-tech/pets-rs` (`build`, `test`, `test-quick`, `test-long`, `test-exhaustive`), task router |
| root `Cargo.toml` | member `P/📦️packages/🦀️rust` after the quiz member; `[workspace.dependencies] semio-framework-pets` after the quiz line (two anchored edits). `Cargo.lock` gained the package entry when cargo first resolved it. |
| `P/🔮️oracles/🔣️.json` | `subjectFeatures: [{ implementation: "rust", features: ["sut"] }]` (anchored edit before `oracleHostPackages`) |
| `P/🧪️tests/{🧬️schema-conformance, 📐️turn-trigonometry, 🎲️counter-randomness, 🦴️rig-solving, 👀️gaze-tracking}/🦀️.rs` | Rust subject adapters, the projections of the `🟦️.ts` adapters (23 scenarios) |
| `P/README.md` | one paragraph under "Validation issues": what the Rust validators take and report, and the `float_roundtrip` requirement (anchored edit) |
| `🧰️framework/…/📚️library/🔣️schema-catalog.json` | regenerated (`schema generate`): the pets scope gained the `🦀️rust` format and its hash; three hashes of another scope were refreshed as a side effect |
| `TK/🗒️rust-scratch-notes.md` | how L and M compile before/after the package, the schema decisions, the kit, bit-exactness rules, adapter rules, pitfalls |
| `TK/rust_scratch.sh` | runner of a private scratch crate (own cargo build and target directory) |
| `TK/dump_kinematics_bits.ts`, `TK/compare_kinematics_bits.rs` | bit-pattern proof of the kinematics (TypeScript dumps, Rust compares) |
| `TK/fuzz_validation.ts`, `TK/compare_validation_findings.rs` | differential proof of the validator on 6 000 broken documents |
| `TK/rehearse_rust_adapters.ts` | rehearsal of the Rust adapters against the TypeScript adapters without the harness |

Module paths other Rust files rely on are as briefed: `crate::schema`, `crate::trigonometry`, `crate::randomness`, `crate::rig` (plus `crate::validation`).

## 2. Verification — commands and real results

From the repository root unless a `cd` is shown; `TK` and `P` as above.

| # | Command | Result |
|---|---|---|
| 1 | `bash TK/rust_scratch.sh wp-k test --offline -- --nocapture` (scratch crate, K's five modules + the two ticket proofs) | `test result: ok. 63 passed; 0 failed` (61 unit tests + 2 proofs) |
| 2 | same with `--release` | `62 passed; 0 failed` (before the validator proof was added), bit proof again 0 mismatches |
| 3 | `bash TK/rust_scratch.sh wp-k clippy --offline --all-targets --features sut -- -D warnings`; the same with `--target wasm32-unknown-unknown` and `--target wasm32-wasip2` (library) | exit 0 three times, no warning (workspace lint set restated in the scratch manifest) |
| 4 | `bun TK/dump_kinematics_bits.ts` then `bash TK/rust_scratch.sh wp-k test --offline proof -- --nocapture` | `[proof] functions=14 compared=24638 mismatches=0` (435 angles each for sine and cosine, 300+ inputs for every other function, 300 poses → 15 300 matrix entries). Negative control: one flipped hex digit in the dump → `mismatches=1`, test fails. |
| 5 | `bun TK/fuzz_validation.ts` then `bash TK/rust_scratch.sh wp-k test --offline findings -- --nocapture` | `[findings] mutants=6000 decoded=4171 refused=1829 valid=2007 findings=3490 disagreements=0`: every mutant the twin decodes yields exactly the TypeScript findings; every mutant the twin refuses carries a type-level finding in TypeScript |
| 6 | `SCRATCH_CRATE=host bash TK/rust_scratch.sh wp-k build --offline --features sut` then `bun TK/rehearse_rust_adapters.ts` (before the package existed) | `[rehearsal] cases=5 scenarios=23 different=0 values=7085` — the Rust host's projections equal the TypeScript adapters' value by value; the committed adapters are byte-identical to the rehearsed drafts except for three line breaks rustfmt asked for afterwards |
| 7 | `RUSTC_WRAPPER="" cargo check -p semio-framework-pets --features sut --offline` (root workspace) | `Finished` (first attempt failed with `failed to write Cargo.lock … os error 1224` because another cargo had the lock file mapped; the retry passed) |
| 8 | `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @semio-tech/pets-rs:test --skip-nx-cache` | exit 0, `111 passed; 0 failed; 4 filtered out` — K's 57 (schema 8, trigonometry 15, randomness 11, rig 12, validation 11) and L's 54 (animation 23, terrain 31). A first run without `--skip-nx-cache` was answered from the nx cache (someone had just run it); the numbers here are from the uncached run. |
| 9 | `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @semio-tech/pets-rs:test-quick --skip-nx-cache` | exit 0, `115 passed; 0 failed` (adds K's four `quick` tests) |
| 10 | `bun ./📜️script.ts verify rust-warnings --target native -p semio-framework-pets`, `--target wasm32-unknown-unknown`, `--target wasm32-wasip2` | `native clean.`, `wasm32-unknown-unknown clean.`, `wasm32-wasip2 clean.` |
| 11 | `RUSTC_WRAPPER="" cargo clippy -p semio-framework-pets --all-targets --features sut --offline -- -D warnings` | exit 0 (tests of K and L included) |
| 12 | `cd TEST && bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "🧬️schema-conformance"` | `cases=1 executed=9 passed=9 failed=0 errored=0 parity=9/9` |
| 13 | same, `--case "📐️turn-trigonometry"` | `executed=18 passed=18 failed=0 errored=0 parity=18/18` |
| 14 | same, `--case "🎲️counter-randomness"` | `executed=15 passed=15 failed=0 errored=0 parity=15/15` |
| 15 | same, `--case "🦴️rig-solving"` | `executed=18 passed=18 failed=0 errored=0 parity=18/18` |
| 16 | same, `--case "👀️gaze-tracking"` | `executed=9 passed=9 failed=0 errored=0 parity=9/9` |
| 17 | `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/🐾️pets"` | `clean=true errors=0 warnings=0` (run three times: with the module files only, with the package, at the end) |
| 18 | `cd TEST && bun ./📜️script.ts contract --owner "…/🐾️pets"` | exit 1 from the repository-wide backlog (`5865 high-priority breach(es) across 4 rule(s)`, the number A and B reported); 0 lines and 0 entries of `breaches/testing.json` name pets |
| 19 | `bun ./📜️script.ts schema check` before / after `schema generate` | before: `findings=9345` with `schema-catalog-stale=1`; after: `findings=9344`, no stale catalog; no finding names pets either time |
| 20 | `rustfmt --check --edition 2021 --config-path rustfmt.toml <each of K's Rust files>` | no diff (diffs of the first pass were applied by hand; no formatter wrote a repository file) |
| 21 | `.venv/Scripts/python.exe -X utf8 <QUIZ-PRODUCT-AND-TEACHING-PROCTOR>/domain_docstring_emojis.py <16 Rust files of K>` | `0 finding(s) in 16 file(s)` |

Parity runs 12–16 were repeated after the last adapter edit (line breaks); the lines above are the last results. `executed` counts oracle + TypeScript + Rust per scenario, `parity` counts oracle↔TypeScript, oracle↔Rust and TypeScript↔Rust. The `bit-patterns` scenarios of three cases compare sixteen-digit hex strings exactly across all three.

## 3. Decisions and deviations

1. **`serde_json` with `float_roundtrip`.** Without that feature serde_json read the committed `0.9421396472025663` as `0.9421396472025664` (one unit in the last place; found by the first unit-test run). JSON.parse and Python round correctly, so a Rust adapter would feed its subject other doubles than the TypeScript adapter for long decimals (26 such inputs in `🎞️animation-sampling`, 72 in `🧠️behavior-choice`). The crate's optional `serde_json` (feature `sut`) and its dev-dependency both carry `features = ["float_roundtrip"]`. Cargo unifies features per build, so a workspace-wide build that includes this crate's tests turns the feature on for every crate in that build (more exact parsing, about twice as slow for floats). Any future Rust host that decodes pets documents must enable it as well; the README says so.
2. **The validators take typed twins**, not untyped JSON (quiz precedent, and `serde_json` is not a runtime dependency): `species_issues(&Species)` and so on. serde refuses what the type-level structure forbids; the validator keeps `type-invalid` for a number that is not finite, `value-invalid` for the `schema` identifier, `slug-invalid`, `length-invalid`, `items-too-few` and the thirteen rule codes. `IssueCode` is an enum of all twenty codes (kebab-case on the wire) so committed findings of either core decode.
3. **`random_pick` returns `Option<usize>`**; `None` is the twin's `-1`, and the adapter projects `-1`.
4. **Number types of the twin:** `Ticks = i64` everywhere (also `Ticked.ticks`, `Stage.tick`, `Frame.tick`, although the schema gives those a minimum of 0 — one type keeps tick arithmetic free of casts; nothing validates a stage at run time); `seed` and `draws` are `u32` (key words; the TypeScript counters wrap with `>>> 0`); `facing` is `i8`, `rate` is `u8`. `Clip.loop` is the field `looping` (`loop` is a keyword; wire name unchanged).
5. **Members that may be `null` are required** (`perch`, `partner`, `clip`, `pointer`, `wake`): a private `deserialize_with` helper refuses their absence, which plain `Option` would accept.
6. **Scratch layout one level deeper than briefed** (`TK/🗑️generated/wp-k/crate/📦️packages/🦀️rust/`), so that `CARGO_MANIFEST_DIR/../../🧫️fixtures` resolves to a private copy of the fixtures the runner refreshes; a junction was rejected because deleting `🗑️generated` could follow it into the product.
7. **Bit vectors stay in `🗑️generated`** as briefed; the committed bit-exactness checks are the `bits` fields of B's fixtures (unit tests and `bit-patterns` scenarios) and cross-subject parity.
8. **The trigonometry module allows two clippy lints for the whole file** (`excessive_precision`, `approx_constant`): the constants are fdlibm's published literals and `TAU` as in the twin; a unit test pins all thirteen bit patterns.
9. **The schema catalog was regenerated** although it is not in the brief: the new Rust format of the pets scope made it stale (`schema-catalog-stale`).
10. **The adapters went into the product only after `TK/📓️report-wp-e.md` appeared (06:14)**; no 30-minute fallback was needed. L had placed its adapters and module files earlier and mounted `animation` and `terrain` in the glue and façade within seconds of the package's creation.
11. **`P/README.md` (A's file) got one paragraph** by anchored edit; nothing else of A's text was touched.
12. **Scratch compiler output was removed** (`🗑️generated/wp-k/{build,target}`, 683 MB); logs, the two proof inputs, the scratch crate, the rehearsal host and the adapter drafts are still there for the ticket's cleanup.

## 4. Open

- **M's cases have no Rust adapter yet** (`🧠️behavior-choice`, `🤝️bond-dynamics`, `🎪️stage-trace`), and L's were not run by me: since the package exists, an owner-wide `parity` run dispatches a Rust subject for every case and fails for the ones without a working adapter. Only K's five cases were run. `pets-stage-trace` still lacks `independent-implementations` (M adds it with the stage adapter).
- **The façade's module doc** lists K's modules only; L and M may extend the sentence when they add their lines.
- **`serde_json/float_roundtrip` is local to this crate.** Whether the workspace dependency should carry it for everyone is a workspace decision; see decision 1 for the unification effect.
- **`Cargo.lock`** carries the new package entry beside unrelated additions of other tickets (`governor`, `futures-timer`, …) that were already in the working tree.
- **No Rust-side third-party oracle crate** was added: the unit tests use numpy's committed answers, the committed bit patterns and the platform's own `sin`/`cos` (std) as references.
- **Performance was not benchmarked** (`random_unit` avoids the vector allocation of `random_words`; `solve_rig` reduces each angle twice like the twin).
- The repo MCP server was unreachable (`CONNECTION_CLOSED`), so no ticket bookkeeping was done from this work package. Nothing was committed; no e2e, deploy or other owner's parity run was started.
