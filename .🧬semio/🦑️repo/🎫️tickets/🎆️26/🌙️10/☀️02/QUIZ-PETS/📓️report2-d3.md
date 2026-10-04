# 📓️ Work package D3 (second round): Rust twins of feeling, effects and mischief — report

Ticket `2026/10/02/QUIZ-PETS`, second round, phase D. `P` = `🧰️framework/🛍️products/🐾️pets`, `TK` = this ticket folder, `TEST` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`. Everything below was run on 2026-10-03 between 13:55 and 15:40 (WEST) on the Windows host (bun 1.4.2, `rustc 1.99.0-nightly`, cargo nightly 2026-07-07), while D1, D2 and other packages edited the same tree. Raw outputs: `TK/🗑️generated/d3/` (3 MB of text results, the scratch crate sources, `parity/`, `ported-from/`; the 727 MB of compiler output and the 65 MB bit file were removed at the end — `bun TK/d3_scratch.ts prepare` and `bun TK/d3_dump_bits.ts` bring back what a rerun needs).

## 1. What exists

| File | Content |
|---|---|
| `P/🔨️modules/💗️feeling/🦀️.rs` | complete twin of `🟦️.ts` (R0's `at_rest`, `shown_mood`, `settled`, `settles_at`, `valence_of`, `spirits_of` kept term for term; added everything else): moods (`MOOD_PRIORITIES`, `MOOD_OVERRIDE`, `impulse`, `calms_at`), face (`MOOD_LIDS/SLANTS/DROPS`, `Countenance`, `face_of`), appraisal (`Occasion`, `OCCASIONS`, `Stir`, `APPRAISALS`, `Character` with `From<&Species>`, `PRONENESS`, `PRONE_GAIN`, `proneness_of`, `stirred`, `appraised`, `TRICK_AMOUNT`, `performed`, `DROWSY_ENERGY`, `drowsed`), wishes (`MOOD_WEIGHTS`, `mood_weights`, `MOOD_AFFINITIES`, `MOOD_SHARES`, `Leaning`, `encounter_bias`, `swayed_shares`), contagion (`MOOD_SPREADS`, `CONTAGION_BEAT`, `CONTAGION_REACH`, `caught`), states (`Standing`, `lasting_ticks`, `state_at`, `state_ends`, `ladder_of`, `rungs_of`, `step_rung`, `step_state`, `state_after_trick`), tricks (`WILLING_MOODS`, `tricks_for`, `click_trick`, `WHIM_FAVOR`, `whim_trick`, `show_trick`), chemistry (`Sighting`, `CHEMISTRY_BEAT`, `EFFECT_AMOUNT`, `LEAN_TICKS`, `nearby`, `seen`, `held_ticks`, `matches`, `Trial`, `Consequence`, `Chemistry`, `every_ticks`, `trials_of`, `draws_of`, `reactions_of`) |
| `P/🔨️modules/💗️feeling/🧪️tests/🔬️unit/🦀️.rs` | 18 tests (R0's four kept, the face test extended to the whole face): every committed vector of `💗️feeling-dynamics` and `⚗️chemistry-rules` (tables, contest, stories, decays, cuts, faces, characters, wishes, leanings, crowds, states, ladders, tricks, relations, stages, stories of the chemistry), laws (impulses, calming, cuts of time, broken intensities, a beat of chemistry), source scan |
| `P/🔨️modules/✨️effects/🦀️.rs` | twin of `🟦️.ts`: the constants, `lowbias32`, `mix`, `unit` (re-export of `unit_of`), `scattered`, `emitter_key`, `Emission` (with `From<&Emitter>`), `Particle`, `life_ticks`, `swarm_of`, `period_of`, `born_at`, the five motions (private), `particles_of`, `capped`, `emitter_ends` |
| `P/🔨️modules/✨️effects/🧪️tests/🔬️unit/🦀️.rs` | 18 tests: the published words, every committed hash, sum and histogram, birth, motion (1e-9, ages exact), cap and end; laws of slots, counted births, counts, mirrors, bursts, orbits, caps, ends; constants and source scan |
| `P/🔨️modules/🪄️mischief/🦀️.rs` | twin of `🟦️.ts`: the constants, `Keyed` (trait; `Fixture` implements it), `Circumstances`, `Station`, `Lift`, `NO_LIFT`, `fits`, `fixture_for`, `chosen_fixture`, `patience_of`, `cooldown_of`, `allowed_from`, `allowed`, `station_for`, `lift_at`, `lift_ends`, `lift_wake`, `thrown_off` |
| `P/🔨️modules/🪄️mischief/🧪️tests/🔬️unit/🦀️.rs` | 17 tests: every committed match, candidate list, choice, leak permutation, gate, station (1e-9 for `y`/`room`), lift (1e-9) and throw; the pick reads each key once and nothing of a candidate (counting `Keyed`); laws of gates, stations, lifts, wakes, throws; constants and source scan |
| `P/🧪️tests/💗️feeling-dynamics/🦀️.rs` | Rust subject adapter, the ten scenarios of `🟦️.ts` (`tables` … `leanings`) with the same projections (`show` projected back to `-1/0/1`) |
| `P/🧪️tests/⚗️chemistry-rules/🦀️.rs` | Rust subject adapter, the eight scenarios (`relations` … `sample`; `reach`, `staged`/`placed`, `cast` and `rehearsal` restated) |
| `P/🧪️tests/✨️particle-motion/🦀️.rs` | Rust subject adapter, the six scenarios (`hashes` … `ends`) |
| `P/🧪️tests/🪄️mischief-choice/🦀️.rs` | Rust subject adapter, the eight scenarios (`matches` … `throws`); a host description is an adapter-local item that implements `pets::Keyed` |
| `P/📦️packages/🦀️rust/🦀️.rs` | two mounts after `gesture`/`clearance`: `pub mod effects;`, `pub mod mischief;` (anchored edits, file re-read before each; `feeling` was mounted by R0) |
| `P/🦀️.rs` | `pub use crate::effects::*;`, `pub use crate::mischief::*;`; one clause in the module docstring ("what it feels …, its particles, its mischief with the page") |
| `TK/d3_scratch.ts` | `prepare` (scratch crate `TK/🗑️generated/d3/crate`: the schema, trigonometry, randomness and the three twins mounted by `#[path]`, the siblings' twins they import — behaviour, gesture, terrain, and with `D3_REAL_CLIMBING=1` rig, animation, swing, climbing — as test-free copies, a `Toss` stand-in otherwise; the four adapters as host examples; the bit check as integration test), `mutants` (15 broken twins against the bit check) |
| `TK/d3_dump_bits.ts`, `TK/d3_check_bits.rs` | the bit-exactness tool pair (§3) |
| `TK/d3_rehearse_adapters.ts` | the four Rust adapters against the four TypeScript adapters, number for number without tolerance (`scratch`: through the scratch host; `harness`: the last parity run's Rust projections) |
| `TK/d3_code_rules.mjs` | docstring emojis (U+FE0F, unique per file, never `@`), console/`[DEBUG]`/print macros in product files, comments inside definitions, CRLF, banned stems |

No TypeScript file, fixture, oracle, feature or registry was touched; no sibling module was edited.

## 2. The API as landed — decisions

- **Signatures follow the twins in snake_case**; `Copy` values travel by value (`Feeling`, `Character`, `Leaning`, `Emission`, `Circumstances`), documents by reference. Tricks are answered as references into the species (`Option<&Trick>`, `Vec<&Trick>`), states and rungs as owned `Slug`s, candidates of the pick as references into the list given.
- **`null` / `-1` is `None`**: `state_ends`, `click_trick`/`whim_trick`/`show_trick`, `Leaning.show` (`Option<usize>`, the twin's `-1/0/1`), `emitter_ends`, `chosen_fixture`, `cooldown_of`, `allowed_from`, `station_for`, `lift_wake`; adapters restore `-1` where the TypeScript adapter projects it.
- **Types that say what the twin floors**: a click index and a step direction are `i64`; `capped` takes `cap: usize` (the twin's `Math.floor` and `room > 0` are the adapter's: a cap below 1 keeps nothing); `Emission.count` is the schema's `u32`; ticks `i64`; sides and facings `Facing`; `Station.footing` is the schema's `Footing` (only `Perch` or `Wall` occur).
- **`Character`** is a struct `{ mood, temperament }` with `From<&Species>` (the twin's `Pick<Species, …>`); **`Emission`** likewise with `From<&Emitter>`; **`Keyed`** is a public trait (the twin's private `Keyed` type) so a caller's own items can be picked — the type system proves that the pick reads keys only.
- **Hash**: `u32` arithmetic, `wrapping_mul` ≙ `Math.imul`, `>>` ≙ `>>>`, `wrapping_add` ≙ `+ … >>> 0`; an index or a tick enters by its low 32 bits (`as u32`), which is what the twin's `(b + 1) | 0` reads, negative ticks included (proved: `emitter_key` with `since = −1`, indices beyond 2³²).
- **`Math.min`/`Math.max` on floats are terrain's `larger`/`smaller`** (NaN and ±0 as JavaScript answers them); integer minima and maxima (`period_of`, `emitter_ends`, `allowed_from`, window bounds) are `Ord::max`. `TURN_RADIANS` and `HALF_TURN_RADIANS` are `std::f64::consts::{TAU, PI}`, bit-identical to the twin's literals (unit tests pin `0x401921fb54442d18`/`0x400921fb54442d18`; clippy's `approx_constant` forbids the literals).
- **Negated comparisons**: kept negated (with `#[allow(clippy::neg_cmp_op_on_partial_ord)]` on the function) wherever nothing in front guards against NaN — `impulse` (`!(amount > 0)`), `caught` (`!(… > MOOD_REST)`, `!(gain > 0)`), `reactions_of` (`!(unit < chance)`), `allowed_from` (`!(width >= MISCHIEF_WIDTH)`), `station_for`; written as the complement inside the loops of `fades_at`/`settles_at`/`calms_at`, whose guards have already refused a NaN intensity (R0's convention, kept). Clippy's `manual_range_contains` rewrote two integer tests in `lift_at`/`lift_wake` into `Range::contains` (same truth table on integers).
- **Repeated rounding of seconds into ticks** (`lasting_ticks`, `held_ticks`, `every_ticks`) is one private `whole_ticks`; the motions of a particle take a private `Source` (the twin's eight-argument functions would trip clippy's `too_many_arguments`). Same expressions, same order.
- **`Pitch` comes from the schema** (`crate::schema::Pitch`): while this package ran, the schema owner moved `Pitch` into the contract (TypeScript 14:14, Rust 14:22) and the TypeScript mischief changed its import at 14:20 — no logic changed, so the twin followed the import only.

## 3. Bit-exactness

`bun TK/d3_dump_bits.ts` evaluates **every exported function** of the three TypeScript modules (plus every exported constant) on generated inputs and writes inputs and results as IEEE-754 bit patterns (`nan`, `null` tokens; indices for moods, activities, cues, occasions, motions, modes, places, species, states, tricks, reactions) to `TK/🗑️generated/d3/d3-bits.json`; `TK/d3_check_bits.rs` (integration test `d3_bits` of the scratch crate) recomputes every case with the Rust twins. Inputs: lattice and arbitrary doubles, ±0, NaN, ±∞, subnormals, values on every threshold (`MOOD_FAINT ± 2⁻⁵⁰`, `MOOD_REST + 2⁻⁵²`, hold boundaries, lift phase boundaries), stories of 60 settle/impulse steps whose inputs are the previous outputs, 80 synthetic species (no states, lasting and circling states, states giving way to themselves or to unknown ids, tricks with `from`/`to`/`mood` incl. unknown ids), the committed chemistry menagerie and two variants (every trait without species; chances on every other reaction, `every` 0.001 s), drawn crowds of 2–6 sightings with coolings, rapports and too few units, emitters with counts beyond the cap and lives of one tick, the `nearby` boundary (reaches one to two units in the last place around the gap), words at the hash's edges, Unicode grounds.

Final run (`bun TK/d3_dump_bits.ts`, then `bash TK/rust_scratch.sh d3 test --offline --release --test d3_bits -- --nocapture` with `D3_REAL_CLIMBING=1`, ~15:12, on the files after the last sibling edit of §5):

| Function | Cases | Result numbers | Mismatches |
|---|---|---|---|
| `at_rest` / `shown_mood` / `impulse` / `settled` | 400 / 600 / 8 400 / 10 400 | 1 200 / 600 / 25 200 / 31 200 | 0 |
| `settles_at` / `calms_at` | 4 000 / 4 000 | 4 000 / 4 000 | 0 |
| `valence_of` / `spirits_of` / `face_of` | 9 / 2 000 / 2 000 | 9 / 2 000 / 8 000 | 0 |
| `proneness_of` / `stirred` / `appraised` / `performed` / `drowsed` | 1 500 / 3 000 / 4 000 / 2 000 / 2 000 | 1 500 / 9 000 / 12 000 / 6 000 / 6 000 | 0 |
| `mood_weights` / `encounter_bias` / `swayed_shares` / `caught` | 900 / 3 000 / 1 500 / 5 000 | 23 400 / 15 000 / 4 500 / 15 000 | 0 |
| `lasting_ticks` / `held_ticks` / `every_ticks` | 600 each | 600 each | 0 |
| `state_at` / `state_ends` | 8 000 / 8 000 | 16 000 / 8 000 | 0 |
| `ladder_of` / `rungs_of` / `step_rung` / `step_state` / `state_after_trick` | 80 / 207 / 4 220 / 1 540 / 844 | 228 / 549 / 4 220 / 1 540 / 844 | 0 |
| `tricks_for` / `click_trick` / `whim_trick` / `show_trick` | 6 000 each | 3 566 / 6 000 / 6 000 / 6 000 | 0 |
| `nearby` / `seen` / `matches` | 12 000 / 6 000 / 6 000 | 12 000 / 6 000 / 6 000 | 0 |
| `trials_of` (with `draws_of`) / `reactions_of` | 5 000 / 5 000 | 137 584 / 276 346 | 0 |
| `lowbias32` / `mix` / `unit` / `scattered` / `emitter_key` | 3 000 / 7 000 / 3 000 / 3 000 / 3 000 | same | 0 |
| `life_ticks` / `swarm_of` / `period_of` / `born_at` / `emitter_ends` | 1 500 each | 1 500 each | 0 |
| `particles_of` / `capped` | 3 882 / 1 500 | 162 366 / 100 621 | 0 |
| `fits` / `fixture_for` / `chosen_fixture` | 289 / 1 500 / 3 000 | 289 / 1 461 / 3 000 | 0 |
| `patience_of` / `cooldown_of` / `allowed_from` / `allowed` | 2 / 3 / 4 000 / 4 000 | same | 0 |
| `station_for` / `lift_at` / `lift_ends` / `lift_wake` / `thrown_off` | 5 000 / 8 000 / 8 000 / 8 000 / 2 000 | 22 580 / 32 000 / 8 000 / 8 000 / 4 000 | 0 |
| every exported constant | 1 | 539 | 0 |
| **total** | **213 077** | **1 038 647** | **0** (0 throwing) |

The same result on every intermediate run (207 071 cases at 14:29; 213 077 at ~14:41, ~15:01 and ~15:12 — the occasions of the sibling edits included each time, the constants growing from 527 to 536 to 539 numbers).

**That the check can fail** — `bun TK/d3_scratch.ts mutants` (each mutant compiled into the scratch crate in place of its twin, `--release`, the full check run against it), final run: `15 mutants, 0 survived`.

| Mutant | Cases that differ (of 213 077) |
|---|---|
| `drowsed`: `1 − energy ÷ 0.6` instead of `(0.6 − energy) ÷ 0.6` | 188 |
| face blended as `calm·(1 − i) + mood·i` | 1 670 (`face_of` 997, `spirits_of` 673) |
| encounter shares expanded instead of multiplied | 215 |
| contagion gain reassociated | 94 |
| fused multiply-add in the gap of `nearby` | 76 (only the boundary inputs find it) |
| `amount <= 0` instead of `!(amount > 0)` (NaN amount) | 162 |
| `calms_at` without the resting mood's half rate | 895 |
| `lowbias32` shifting by 16 instead of 15 | 18 723 |
| fused multiply-add in the fall's `y` | 229 |
| burst reach reassociated `pace·(cos·reach)` | 41 |
| orbit radius `· (1 ÷ 2π)` instead of `÷ 2π` | 36 |
| `rotation` of a fall mirrored as `DOWN − heading` (signed zero) | 13 |
| `f64::min` instead of `smaller` in the perch station | 1 |
| `toward` as `−value` (signed zero) | 641 |
| lift travel distributed over the sum | 1 660 |

The counts are those of the final run (15:00–15:10); in its last seven mutants the `constants` case differed as well (one case each, not counted above), because the `bonked` edit of §5 changed the feeling tables under the run — the dump was regenerated afterwards and the unmutated check is clean (§3 table). Four mutants of the first list survived and were replaced, because they are **equivalent on every input the twins can see**: a fused multiply-add in `proneness_of` (the `PRONENESS` factors are 0 and ±1, so the product is exact), `(0.5·G·s)·s` vs `0.5·G·(s·s)` in a burst (`s` is a dyadic tick fraction, every product exact), `LIFT_TILT.min(…)` in `lift_at` (no NaN or −0 reaches it behind `span > 0`), and — until the boundary inputs were added — a fused multiply-add in `nearby` (lattice boxes give exact squares). They are recorded because they show where the vectors cannot discriminate.

## 4. Verification — exact commands and real results

Repository root unless a `cd` is shown.

| # | Command | Result |
|---|---|---|
| 1 | `bun TK/d3_scratch.ts prepare` then `bash TK/rust_scratch.sh d3 test --offline --lib` | `test result: ok. 102 passed; 0 failed` (schema, trigonometry, randomness and the three twins; 18 + 18 + 17 of them mine) |
| 2 | `D3_REAL_CLIMBING=1 bun TK/d3_scratch.ts prepare` then `bash TK/rust_scratch.sh d3 clippy --offline --all-targets --features sut -- -D warnings` | exit 0, no finding (twins, unit tests, four adapters as examples, the bit check); `… test --offline --lib mischief::` → `17 passed` against D1's real climbing |
| 3 | `bash TK/rust_scratch.sh d3 test --offline --release --test d3_bits -- --nocapture` | `total: 213077 cases, 1038647 result numbers, 0 throwing, 0 mismatches` (§3) |
| 4 | `bun TK/d3_scratch.ts mutants` | `15 mutants, 0 survived` (§3) |
| 5 | `bun TK/d3_rehearse_adapters.ts scratch` | `scratch: 4 cases, 32 scenarios, 40529 numbers, 0 differences` |
| 6 | `RUSTC_WRAPPER="" cargo check -p semio-framework-pets --all-targets --features sut --offline` (after each mount) | `Finished` without a warning (effects mounted 14:12, mischief 15:17 — right after D1 mounted swing and climbing) |
| 7 | `RUSTC_WRAPPER="" cargo clippy -p semio-framework-pets --all-targets --features sut --offline -- -D warnings` | exit 0, `Finished` without a finding |
| 8 | `RUSTC_WRAPPER="" cargo test -p semio-framework-pets --offline --lib -- mischief:: effects:: feeling::` | `test result: ok. 53 passed; 0 failed` |
| 9 | `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @semio-tech/pets-rs:test --skip-nx-cache` | first two runs (14:47, 15:05): the 15 s budget of the fundamental level killed cargo while it compiled on the loaded host (`cargo exceeded its 15000ms budget`); after `cargo test -p semio-framework-pets --no-run`: `test result: FAILED. 425 passed; 2 failed` — `stage::tests::the_calm_home_replays_into_its_committed_trace` (the stage replay, ported later — not mine) and `climbing::tests::committed_wall_climbing_holds_surveys_climbs_grips_slides_mantles_and_routes_are_reproduced` (D1's twin, in progress at 15:18); every feeling, effects and mischief test passed |
| 10 | `bun ./📜️script.ts verify rust-warnings --target native -p semio-framework-pets`, `--target wasm32-unknown-unknown`, `--target wasm32-wasip2` | exit 1 three times before any crate is looked at: the command's own scope self-check fails (`error: [verify rust-warnings] native misses semio-hub-dag.`) — a repository-wide state of the hub packages, not pets. In its place: |
| 10a | `RUSTC_WRAPPER="" cargo clippy -p semio-framework-pets --lib --features sut --target wasm32-unknown-unknown --offline -- -D warnings`, the same with `--target wasm32-wasip2` (native: row 7) | exit 0 both, `Finished` without a finding |
| 11 | in `TEST`: `bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "💗️feeling-dynamics"` | `[test] level=exhaustive cases=1 executed=30 passed=30 failed=0 errored=0 parity=30/30` |
| 12 | same, `--case "⚗️chemistry-rules"` | `executed=24 passed=24 failed=0 errored=0 parity=24/24` |
| 13 | same, `--case "✨️particle-motion"` | `executed=18 passed=18 failed=0 errored=0 parity=18/18` |
| 14 | same, `--case "🪄️mischief-choice"` | `executed=24 passed=24 failed=0 errored=0 parity=24/24` |
| 15 | same, `--case "🧠️behavior-choice"` | `executed=24 passed=24 failed=0 errored=0 parity=24/24` |
| 16 | same, `--case "🤝️bond-dynamics"` | `executed=18 passed=18 failed=0 errored=0 parity=18/18` |
| 17 | `bun TK/d3_rehearse_adapters.ts harness` (what the real Rust subject projected in runs 11–14, against the TypeScript adapters, no tolerance) | `harness: 4 cases, 32 scenarios, 40773 numbers, 0 differences` |
| 18 | `rustfmt --check --edition 2021 --config-path rustfmt.toml` over the three twins (with their unit suites), the four adapters and `TK/d3_check_bits.rs` | exit 0 (the first pass's 27 hunks were applied by hand with Edit) |
| 19 | `node TK/d3_code_rules.mjs` | `problems: 0` over 16 files |
| 20 | `.venv/Scripts/python.exe -X utf8 <QUIZ-PRODUCT-AND-TEACHING-PROCTOR>/domain_docstring_emojis.py` over 15 files | `0 finding(s) in 15 file(s)` |
| 21 | `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/🐾️pets"` | `clean=true errors=0 warnings=0` (a first run at 15:00 reported `reference-preimage-unreadable` for `🚶️locomotion/🟦️.ts`, which a sibling changed during the scan) |

`executed` counts oracle + TypeScript + Rust per scenario; `parity` counts the three pairwise comparisons per scenario. The six parity runs ran from 15:20 to 15:27, after the last sibling edit of the feeling module (15:08) and after mischief was mounted.

## 5. Concurrent changes this package followed

- **`Pitch` moved into the schema** (§2) — mischief imports it from `crate::schema`; the scratch glue stopped re-exporting terrain while the terrain twin still declared its own (an ambiguous glob for a few minutes; gone by 14:45).
- **New occasions in the feeling module, by another package**: `climbed` and `missed` (TypeScript 14:47, my Rust twin 14:51, fixture `💗️feeling-dynamics` regenerated 14:53) and `bonked` (TypeScript 15:08, Rust 15:08). Both edits were made by their author in both twins, term for term (enum variant, `OCCASIONS`, `APPRAISALS` rows, docstring). The second mutant run caught the first edit mid-way (the scratch crate compiled a half-edited `OCCASIONS`, E0308, and the constants differed for the mutants after it); the dump was regenerated after each edit and the full check, the unit suites and the parity runs above are on the edited files. `TK/🗑️generated/d3/ported-from/` holds the TypeScript the twins were last proved against.
- **D1 mounted `swing` and `climbing`** at ~15:15; mischief was mounted immediately after, once the scratch crate had compiled it against D1's real climbing.

## 6. Deviations and open items

- **Domain where the twins part** (outside every valid document, recorded, not reconciled, like design §13.9): a NaN `life` or lasting/held/every seconds beyond 1.4·10¹⁷ (the twin's `Math.max(1, NaN)` is NaN and its huge doubles are not `i64`); fractional ticks and counts (not expressible in Rust). The bit tool stays inside the shared domain.
- **Unit-test oracles** inside the crate: the committed vectors of numpy/scipy (as in round one); no Rust third-party crate was added.
- **`P/📦️packages/🦀️rust/Cargo.toml` `description`** still lists only the first round's modules (open for whoever closes phase D; not touched to avoid a rebuild during the parity runs of siblings).
- **The stage** does not call the new feeling, effects or mischief functions in Rust yet (B2/B3/B5's TypeScript integration, phase D's stage port); `Character::from(&Species)`, `Emission::from(&Emitter)`, `Keyed for Fixture` are ready for it.
- No ticket bookkeeping from this package: the repo MCP server was unreachable in this session (`CONNECTION_CLOSED`). Nothing was committed; no e2e gate, no deploy-check.
