# 📓️ Report — work package M (Rust twins of the pets core)

Ticket `2026/10/02/QUIZ-PETS`. `P` = `🧰️framework/🛍️products/🐾️pets`, `TK` = this folder, `TEST` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`. Host: Windows, bun 1.4.2, `rustc 1.99.0-nightly (c4af71034 2026-07-06)`.

**State: final.** The package began as "behaviour and stage" and ended as the owner of every pets Rust twin. All Rust twins mirror the TypeScript core as it stands after the tuning pass (N) and the follow-up package (P, `📓️report-wp-p.md` §2, "None so far" under §2.9 at 11:22). Every command of §4 and the long run of §5 were run between 10:42 and 11:22 on 2026-10-02 on exactly these files: stage `🟦️.ts` of 10:38:02, behaviour 10:37:17, validation 10:36:34, trigonometry 10:37:12, terrain 10:38:12, rig 10:04:00, randomness 09:35:40, schema 09:36:52; vectors `🎪️stage-trace` 10:27:54, `🧠️behavior-choice` 10:00:31, `🧬️schema-conformance` 10:36:57. `TK/🗑️generated/wp-m/ported-from/` holds a copy of each of these TypeScript files; `cmp` finds every copy identical to the repository file.

## 1. What exists

| File | Content |
|---|---|
| `P/🔨️modules/🧠️behavior/🦀️.rs` (341 lines) | twin of `🟦️.ts`: `Limits`, `ModeLimits`, `MODE_LIMITS`, `Situation`, `activity_weights`, `dwell_of`, `mood_of`, `followers_of`, `ENCOUNTERS`, `Encounter`, `encounter_shares`, `encounter_of`, `AFFINITY_FLOOR`, `RAPPORT_SPAN`, `affinity_of`, `rapport_after`, `rapport_faded`, `needs_of`, `needs_after`, `cast_of` (with the private `turns_of`) |
| `P/🔨️modules/🎪️stage/🦀️.rs` (2 215 lines) | twin of `🟦️.ts` (1 728 lines): `open_stage`, `advance`, `frame_of`; the same private functions under the same names in snake_case, in the same regions and order |
| `P/🔨️modules/{🧠️behavior, 🎪️stage}/🧪️tests/🔬️unit/🦀️.rs` | 39 and 31 tests |
| `P/🧪️tests/{🧠️behavior-choice, 🤝️bond-dynamics, 🎪️stage-trace}/🦀️.rs` | Rust subject adapters, scenario for scenario the `🟦️.ts` adapters (9 + 6 + 3) |
| `P/📦️packages/🦀️rust/🦀️.rs`, `P/🦀️.rs` | mounts and re-exports of `behavior` and `stage`; the façade docstring and the crate `description` now name all nine modules |
| `P/🔮️oracles/🔣️.json`, `P/🧪️tests/🎪️stage-trace/🥒️.feature` | decision `pets-stage-trace` in the cross-implementation form: substitutes `specification-vectors`, `metamorphic-laws`, `independent-implementations`; scenario `traces` is `@mode-differential` |
| `TK/long_trace_digest.ts`, `TK/long_trace_digest.rs` | the long-run bit-exactness proof (§5), with `--probe` |
| `TK/stage_scratch.ts` | `prepare` (scratch crate `TK/🗑️generated/wp-m/crate`), `mutants` (18 deliberately broken twins against the long run) |
| `TK/rehearse_stage_adapters.ts` | compares the projections of the three Rust adapters with the TypeScript adapters number for number, without the 1e-9 of the harness profile |

Files of K and L changed after the hand-over (all Rust; every edit anchored, right after re-reading the file):

| File | Change |
|---|---|
| `P/🧬️schema/🦀️.rs` + its unit tests | `Actor.faced: Ticks`, `ActorFrame.activity: Activity` (P's report says "the Rust twin already carries both fields" — this package added them at 10:00); closed sets as types: `enum Facing { Right, Left }` (wire `1`/`-1`), `enum Rate { Rest, Quarter, Half, Full }` (wire `0`/`16`/`32`/`64`), `Ticked.ticks: u64`, `Stage.tick` and `Frame.tick` refuse a negative number (`elapsed`) |
| `P/🔨️modules/🎲️randomness/🦀️.rs` + tests | `STAGE_STREAM`, `CAST_STREAM`, `ROTATION_STREAM`, `unit_of`, `weighted_index` (moved in from behaviour); `random_unit`, `random_pick` are defined through them |
| `P/🔨️modules/🦴️rig/🦀️.rs` + tests | `pupil_reach(&Eye)` |
| `P/🔨️modules/✅️validation/🦀️.rs` + tests | `loop-seam` on the rotation channel judged within `SEAM_SLACK = 1e-9` of a turn (`apart = revolutions − floor(revolutions + 0.5)`); the counts of the regenerated vectors (21 accepted, 63 rules) |
| `P/🔨️modules/📐️trigonometry/🦀️.rs` | `sin_turns(turns: Turns)`, `cos_turns(turns: Turns)` (type only) |
| `P/🔨️modules/{🏞️terrain, 🎞️animation}/🦀️.rs` | nothing: their TypeScript changed in comments only |

No TypeScript file, no fixture and no file of the quiz product was touched.

## 2. The API as landed

```rust
// crate::schema (closed sets)
pub enum Facing { Right, Left }   // serde: 1 | -1;  FACINGS, sign() -> f64, reversed()
pub enum Rate { Rest, Quarter, Half, Full }   // serde: 0 | 16 | 32 | 64, Ord;  RATES
// crate::randomness
pub const STAGE_STREAM: u32 = 0xffff_ffff;  pub const CAST_STREAM: u32 = 0xffff_fffe;  pub const ROTATION_STREAM: u32 = 0xffff_fffd;
pub fn unit_of(word: u32) -> f64;  pub fn weighted_index(weights: &[f64], unit: f64) -> Option<usize>;   // None ≙ −1
// crate::rig
pub fn pupil_reach(eye: &Eye) -> f64;
// crate::behavior
pub struct Limits { movers: usize, fidgeters: usize, idle_low: Ticks, idle_high: Ticks, fidget, walk, hop, sleep, stroll: f64, encounter_gap: Ticks, encounter_rate: f64 }
pub struct ModeLimits { still, calm, lively: Limits }   // Index<PetMode>
pub const MODE_LIMITS: ModeLimits;
pub struct Situation { mode, quiet, movers: usize, fidgeters: usize, roam, hops, crowd: usize, watched }
pub fn activity_weights(actor: &Actor, species: &Species, situation: Situation) -> [f64; 11];
pub fn dwell_of(activity: Activity, mode: PetMode, unit: f64) -> Ticks;  pub fn mood_of(activity: Activity) -> f64;
pub fn followers_of(activity: Activity) -> &'static [Activity];
pub const ENCOUNTERS: [Activity; 3];  pub type Encounter = Activity;
pub fn encounter_shares(affinity: f64) -> [f64; 3];  pub fn encounter_of(affinity: f64, unit: f64) -> Encounter;
pub fn affinity_of(menagerie: &Menagerie, rapports: &[Rapport], a: &str, b: &str) -> f64;
pub fn rapport_after(drift: f64, activity: Activity) -> f64;  pub fn rapport_faded(drift: f64, ticks: Ticks) -> f64;
pub fn needs_of(temperament: Temperament) -> Needs;
pub fn needs_after(needs: Needs, activity: Activity, ticks: Ticks, temperament: Temperament) -> Needs;
pub fn cast_of(cast: &Cast, capacity: usize, epoch: i64, seed: u32) -> Vec<Slug>;
// crate::stage
pub fn open_stage(seed: u32) -> Stage;
pub fn advance(menagerie: &Menagerie, stage: Stage, events: &[StageEvent]) -> Stage;
pub fn frame_of(menagerie: &Menagerie, stage: &Stage) -> Frame;
```

How the TypeScript semantics are kept:

- Tables are read at `activity as usize`. Every product and sum is written in the twin's order; no `mul_add`, no `powi`, no libm. `Math.max`/`Math.min` on numbers are L's `larger`/`smaller`; minima of whole ticks are `Ticks::min`. Unit tests scan both sources for `.max(`, `.min(`, `f64::max`, `f64::min`, `.mul_add(`, the transcendentals, `static `, `thread_local!`, maps and print macros.
- Keys are `[seed, stream, counter]` as `u32` words; counters wrap; the arrival counter is `tick as u32`. `floor(unit × n)` becomes an index or ticks by one cast after `floor()`.
- `−1` sentinels: `weighted_index`, `index_of`, `heading_of` and `carry` answer `Option`; `pairing_tick` and `arrival_tick` keep the twin's `−1`.
- `x >= y ? 1 : -1` is `facing_to` (the TypeScript has the same helper since P), so NaN takes the twin's branch.
- The frame order is an insertion sort with the twin's comparator (`y`, then the species id by UTF-16 code units).
- A still-stage removal by index in `spread` guards the index like `splice` does (out of range removes nothing).

## 3. Decisions and deviations

1. **`advance` takes the stage by value**: a fold owns what it folds; without events the stage comes back untouched.
2. **`cast_of(cast, capacity: usize, epoch: i64, seed: u32)`**: the twin floors a fractional capacity or epoch and treats a negative capacity as 0; here the types say it. `spare` is the twin's signed difference cut at 0, which chooses the same `open` in every case.
3. **Closed sets are types** (audit F4), the way the quiz twin models them: `Facing` and `Rate` are enums on their wire numbers (`try_from`/`into`), so no other number decodes and none can be constructed; `Ticked.ticks` is `u64`; the tick of a stage and of a frame refuses a negative number when decoded. These are the three members the contract bounds (`minimum: 0`). `since`, `until`, `blink`, `faced`, `pointed` and `met` stay `Ticks` (`i64`): the contract gives them `$defs/Ticks`, which has no lower bound (the committed vectors carry `met: -640`), and the stage subtracts them freely. Two unit tests hold the enums to the inline `enum`s of the normative JSON.
4. **`MODE_LIMITS` is a struct indexed by `PetMode`**; weights and shares are arrays; stretches are `[f64; 2]` pairs; the room of an arrival names its perch by index; `quarters` filters the rooms it is given (`retain`) instead of building two lists — the same rooms in the same order.
5. **Where the twins cannot agree**, all outside every valid document: a menagerie with two species of one id; NaN coordinates in a glide; `ticked` with a fractional or negative count (not expressible).
6. **`tick + 1 <= pointed + PERK_LINGER` is written `tick < pointed + PERK_LINGER`** in the wake of `frame_of` (clippy `int_plus_one`; whole ticks, the same truth table).
7. **Formatting**: `rustfmt --check` with the repository's `rustfmt.toml` is clean; its findings were applied by hand with Edit.

## 4. Verification — exact commands and real results

| Command | Result |
|---|---|
| `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @semio-tech/pets-rs:test` | exit 0, `test result: ok. 184 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out` |
| `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @semio-tech/pets-rs:test-quick` | exit 0, `test result: ok. 189 passed; 0 failed` (behaviour 39, stage 31, terrain 31, animation 23, trigonometry 16, randomness 15, rig 14, validation 11, schema 9) |
| `RUSTC_WRAPPER="" cargo clippy -p semio-framework-pets --all-targets --features sut --offline -- -D warnings` | exit 0, no finding |
| `bun ./📜️script.ts verify rust-warnings --target native -p semio-framework-pets`, `--target wasm32-unknown-unknown`, `--target wasm32-wasip2` | `native clean.`, `wasm32-unknown-unknown clean.`, `wasm32-wasip2 clean.` |
| `bash TK/rust_scratch.sh wp-m test --offline --lib` (private scratch crate, validation not mounted) | `test result: ok. 178 passed; 0 failed` |
| `bash TK/rust_scratch.sh wp-m clippy --offline --all-targets --features sut -- -D warnings` (twins, tests, the three adapters, the ticket tool) | exit 0 |
| `rustfmt --check --edition 2021 --config-path rustfmt.toml <18 Rust files>` | exit 0 |
| `…/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/domain_docstring_emojis.py` over the 18 Rust files and three ticket tools | `0 finding(s) in 21 file(s)`; no `[DEBUG]`, no print macro in product code |
| `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/🐾️pets"` | `clean=true errors=0 warnings=0` |
| in `TEST`: `bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/🐾️pets"` | exit 1 with the repository-wide backlog (`5865 high-priority breach(es) across 4 rule(s)`); no line names pets |

Parity, in `TEST`: `bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "<name>"`, every run exit 0 with `failed=0 errored=0`:

| Case | Result |
|---|---|
| `🧬️schema-conformance` | `executed=9 passed=9 parity=9/9` |
| `📐️turn-trigonometry` | `executed=18 passed=18 parity=18/18` |
| `🎲️counter-randomness` | `executed=15 passed=15 parity=15/15` |
| `🦴️rig-solving` | `executed=18 passed=18 parity=18/18` |
| `👀️gaze-tracking` | `executed=9 passed=9 parity=9/9` |
| `🎞️animation-sampling` | `executed=18 passed=18 parity=18/18` |
| `🪀️spring-settling` | `executed=15 passed=15 parity=15/15` |
| `🏞️terrain-walking` | `executed=12 passed=12 parity=12/12` |
| `🦘️hop-ballistics` | `executed=21 passed=21 parity=21/21` |
| `🧠️behavior-choice` | `executed=27 passed=27 parity=27/27` |
| `🤝️bond-dynamics` | `executed=18 passed=18 parity=18/18` |
| `🎪️stage-trace` | `executed=6 passed=6 parity=3/3` (two subjects, three scenarios, compared pairwise; 13 scripts, 128 000 ticks) |

`bun TK/rehearse_stage_adapters.ts harness` (what the Rust subject projected in those runs against the TypeScript adapters, no tolerance): `harness: 3 cases, 18 scenarios, 4782 numbers, 0 differences`.

## 5. Bit-exactness on long runs

`bun TK/long_trace_digest.ts` composes and plays 48 sessions of 20 minutes (76 800 ticks each) through the TypeScript core: two menageries (the trace troupe, 5 species; the architecture menagerie, 20 species), four seeds (1, 7, 20261002, 4294967295), the three modes, quiet on and off. Every 2 to 20 seconds something happens — 18 883 events: 13 608 pointer moves in bursts that end at rest beside, on or behind an actor, 460 pointer exits, 810 pokes, 543 glance sets, 1 118 surveys (scrolled, a card moved, a surface gone, a shelf gained, laid out anew, unchanged; 248 of them bring new ground), 730 summons, 928 changes of mode, 686 of quiet. Per simulated second it records two FNV-1a digests over IEEE-754 bit patterns: the running digest of every frame so far (the digest of the stage-trace case, with the activity) and the digest of every number of the stage at that tick (needs, gaze velocities, `faced`, counters, rapports, goals).

| Run | Result |
|---|---|
| TypeScript | `48 sessions of 76800 ticks, 18883 events, 57600 checkpoints` |
| `bash TK/rust_scratch.sh wp-m run --release --offline --features sut --example long_trace_digest` | `rust against typescript: 48 sessions, 57600 checkpoints compared (two digests each), 0 mismatches; first mismatch: none` |

**57 600 checkpoints, 115 200 digests, 3 686 400 ticks: no mismatch.**

That the comparison can fail, and that the sessions reach what N and P added: `bun TK/stage_scratch.ts mutants` — `18 mutants, 3 survived (3 expected to), 0 surprises`.

| Mutant | Checkpoints that differ (of 57 600) |
|---|---|
| fused multiply-add in the goal of a walk | 13 438 |
| the same goal reassociated | 16 440 |
| the two words of a blink draw swapped | 53 838 |
| the eye height one unit in the last place off | 23 767 |
| reassociated quotient in the glide velocity of a floater | 13 |
| fused multiply-add in the eye height of the gaze | 3 191 |
| `TURN_REST` 57 instead of 56 (turning to the pointer) | 32 802 |
| `PERK_LINGER` 33 instead of 32 (greeting a resting pointer) | 15 066 |
| `PERK_COST` one unit in the last place off | 1 976 (stage digest only: curiosity) |
| fused multiply-add in the lean of the head | 30 000 (frame digest only) |
| the ground counted as a raised perch in `quarters` | 57 600 |
| every survey taken as new ground (`widened`) | 32 546 |
| an actor falls from vanished ground although new ground came | 12 161 |
| fused multiply-add in the energy of `needs_after` | 10 904 |
| reassociated quotient in `rapport_faded` | 6 140 |
| fused multiply-add in the mood ease | 0 — equivalent: `MOOD_EASE` is 2⁻⁵ |
| the sociability of a pairing candidate taken one tick later | 0 — blind spot, below |
| reassociated product in the fidget weight | 0 — blind spot of the long run, covered by the exact comparison of the weights |

**What the long run cannot see.** Quantities that only weigh a draw change a digest only when they flip a pick. For the behaviour functions that gap is closed by `rehearse_stage_adapters.ts` (every weight, share, need, drift, dwell and cast of the committed vectors is the TypeScript adapter's double). For the expressions inside `pair` the evidence is the reading of the two sources side by side. `cast_of` is not called by the stage; its evidence is the parity of `🧠️behavior-choice` (numpy) and the unit tests.

## 6. How the TypeScript was followed

The TypeScript changed under this package three times: the tuning pass (N), P's delta (turning round on `Actor.faced`, attending to the pointer, lean, pupil travel, spreading out, `castOf`, the consolidated randomness, `loop-seam`), and P's later steps (`GAZE_REACH` 32, `PERK_URGE`/`PERK_COST`, `uprooted`). Each step was ported from a `diff` of the last fully ported copy (`ported-from/`) against the repository file, with the copy replaced in the same step, and proven with the long run before the next.

One step in the tuning pass went wrong and is worth knowing: a snapshot was copied without diffing it again, and one line of `stride` (a hopping gait hindered in mid-hop finishes the hop on the spot) was not ported. Unit tests, all committed traces and the parity of the harness stayed green; the 20-minute run found it, and `--probe` named tick and member. It has a unit test since.

To re-sync after a further change: `diff TK/🗑️generated/wp-m/ported-from/<module>.ts P/…/🟦️.ts`, port, `bun TK/stage_scratch.ts prepare`, `bun TK/long_trace_digest.ts --minutes 5` and the Rust half (`0 mismatches` in about a minute), then the full run and §4.

## 7. Open

- **Vectors:** no committed script has a hopping gait hindered in mid-hop (§6); the long run covers it, the case does not.
- **`larger`/`smaller`** live in terrain (`pub(crate)`) and are used by the stage too; `📐️trigonometry` would be their home in both languages.
- **Blind spot of §5:** the transient quantities of `pair` have no bit-level evidence beyond the source reading; a projection of the pairing candidates in the stage-trace adapters of both languages would close it and needs an export the stage does not have.
- **`📓️report-wp-p.md` §2.1** says the Rust schema twin already carried `faced` and `activity`; it did not until this package added them.
- The Rust adapter of `🎪️stage-trace` replays every script once per scenario (no static cache is allowed); the exhaustive parity run of the case takes about two minutes.
- `TK/🗑️generated/wp-m/` holds the text results of the runs above (`parity/`, `pets-rs-*.txt`, `mutants.txt`, …), the recorded sessions and digests (5 MB), `ported-from/` and the scratch crate sources; its cargo directories were removed. `bun TK/stage_scratch.ts prepare` restores what a rerun needs.
