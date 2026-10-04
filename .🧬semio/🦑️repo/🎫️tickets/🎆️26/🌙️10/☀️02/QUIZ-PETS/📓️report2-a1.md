# 📓️ Work package A1 (second round) — the stage split, behaviour-free

Ticket `2026/10/02/QUIZ-PETS`, second round, design-v2 §15.2 and §23. `P` = `🧰️framework/🛍️products/🐾️pets`, `M` = `P/🔨️modules`, `TK` = this ticket folder, `TEST` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`. Tool output: `TK/🗑️generated/a1/`.

**State: landed 2026-10-03 00:28 (WEST), all proofs run afterwards on the live tree (00:29–01:02).** `M/🎪️stage/🟦️.ts` (1 728 lines) and its Rust twin (2 215 lines) are now a façade plus ten modules in both languages. Not one definition changed: every function, type, struct and constant has the same text as in the committed stage, only its file and its sharing keyword (`export` / `pub(crate)`) are new. The thirteen committed traces, the 57 600 long-run checkpoints (TypeScript before vs after, Rust vs TypeScript) and Protocol v2 parity are identical.

## 1. Module map as landed

| Module (directory) | Holds | 🟦️.ts | 🦀️.rs | Rust unit tests |
|---|---|---|---|---|
| `🎪️stage` (façade) | `openStage`, `apply`, `advance`, re-export of `frameOf`; the header with NORMATIVE ORDER, DISTANCE, THE POINTER, DRAWS, LAZINESS (BITS in Rust) and a new PARTS paragraph | 104 | 138 | 6 (5 + `quick`), and the shared kit |
| `📝️draft` | types `Body` (TS), `Draft`, `Room`, `Launch`; `draftOf`, `sealed`; lookups `hoverOf`, `indexOf`, `surfaceOf`, `clipsOf`, `clipAt`, `clipOf`, `breathOf`, `facingTo`, `blinkAt`, `actorKey`, `stageKey`, `shoulders`, `beatOf`; `shift`, `settle`, `release`, `remove`; `carve`, `crowdOn`; constants `RATE` (Rust), `BLINK_LOW`, `BLINK_HIGH`, `ORIGIN`, `NO_CLIPS` (TS) | 241 | 250 | 3 |
| `📏️spacing` | `vacancy`, `soars`, `clearway`, `hindered`, `roomsFor`, `quarters`; `MEET_GAP`, `COMFORT_GAP`, `CONTACT` | 138 | 168 | — |
| `🗓️schedule` | `sociable`, `pairingTick`, `arrivalTick`; `WARMUP` | 47 | 62 | 1 |
| `👀️attention` | `watched`, `presenceOf`, `headingOf`, `turn`, `gazeGoal`, `gazeRests`, `look`, `wink`, `cheer`, `swivel`, `hail`, `perk`, `leant`, `squeezeOf`; 23 constants (`POINTER_TICKS`, gaze, blink, mood, wake, shy, turn, lean, perk) | 218 | 297 | 8 |
| `🚶️locomotion` | `vanish`, `crowdOut`, `drop`, `touch`, `paced`, `stroll`, `attend`, `hopsOf`, `stride`, `aim`, `plunge`, `fly`, `leave`; `PATIENCE`, `LEAVE_REACH`, `GLIDE_TICKS` | 274 | 329 | 1 |
| `💞️sociability` | `sulk`, `recall`, `bond`, `reconcile`, `pair`, `parted`, `reachable`, `approach`, `meet`, `poke`; `POKE_REACH`, `POKE_CHEER`, `ENCOUNTER_REACH`, `ENCOUNTER_RISE`, `PAIR_WEIGHT`, `NO_RAPPORTS` (TS) | 219 | 265 | 1 |
| `🎯️choice` | `decide`, `conclude`; `STROLL_LEAST` | 110 | 134 | — |
| `👥️population` | `measure`, `haul` (Rust), `carry`, `seat`, `widened`, `ride`, `arrive`, `spawn`, `spread`, `survey`, `summon`, `freeze`, `tune`; `CENTRED` (Rust) | 345 | 444 | 8 |
| `🕰️clock` | `act`, `lull`, `step`, `pass`; `FADE_STEP`, `SHY_STEP` | 116 | 164 | 2 |
| `🎥️projection` | `replaces`, `weightOf`, `layer`, `poseOf`, `paceOf`, `behind` (Rust), `frameOf`; `BLEND_TICKS`, `BREATH_STAGGER` | 144 | 223 | 1 |
| **total** | 143 TS / 144 Rust definitions | **1 956** | **2 474** | **31** (as before) |

The functions per module are exactly those of `📓️explore2-core-as-built.md` §5 (with the Rust-only helpers `haul` → population, `behind` → projection, `RATE` → draft, `CENTRED` → population). Inside every module the definitions keep their relative order from the old file; the regions are new (`🔖️Constants` first, then one region per theme).

Import graph as landed (`a → b` = a imports b; checked from the import lines of the landed files, the same as the proposal of CORE §5, acyclic):

```
stage → clock, population, sociability, draft, projection
clock → choice, attention, population, sociability, schedule, locomotion, draft
choice → attention, sociability, spacing, locomotion, draft
sociability → attention, spacing, schedule, locomotion, draft
population → spacing, locomotion, draft
projection → attention, schedule, draft
locomotion → spacing, draft
attention, spacing, schedule → draft
draft → schema, animation, randomness, behavior
```

Sharing. TypeScript exports what another part imports (70 names); Rust marks the same items `pub(crate)` (and the fields of `Draft`, `Room`, `Launch`). The glue mounts the ten parts as private modules (`mod draft;` …), the façade keeps `pub mod stage;`, so the crate exports exactly what it did. The barrel `P/📦️packages/🟦️typescript/🟦️.ts` is unchanged (it lists `🎪️stage` only); measured: of the 70 names the parts export only `frameOf` is reachable through `@semio-tech/pets` (the intended re-export), and the façade exports `advance, frameOf, openStage`.

## 2. Tests

- **TypeScript**: the stage suite stays the suite of the façade (it tests through `advance` and `frameOf`). Its arithmetic test now reads every part (`const PARTS = ["🎪️stage", "📝️draft", …, "🎥️projection"]`, one `it`, the failing part named in the message), so the test count is unchanged (510 tests in the eight original suites).
- **Rust**: the 31 tests moved to the module that owns what they test — draft 3, schedule 1, attention 8, locomotion 1, sociability 1, population 8, clock 2, projection 1; the façade keeps 5 + `quick::every_script_replays_into_its_committed_trace_however_time_is_cut`. Test bodies are unchanged; only their `use` lines are new. The façade's test module is `pub(crate) mod tests` (precedent: `crate::schema::tests`) and shares its kit: `troupe, meadow, ticked, surveyed, summoned, actor_of, run, rested, pointed, plain, FAR, LEVEL, TROUPE`. Its arithmetic test scans all eleven Rust sources (`SOURCES`, `FORBIDDEN`).
- Five constants are `pub(crate)` only because a test of another part reads them: `BLINK_LOW`, `BLINK_HIGH` (draft), `CENTRED` (population), `TURN_TICKS` (attention), `SHY_STEP` (clock).

## 3. How it was landed

1. Snapshot of the committed pets core (`HEAD` for the three schema files A2 was already editing) into `TK/🗑️generated/a1/before/` and `…/after/` at the depth it has in the repository.
2. `TK/split_stage.ts split` cut both stage files into their definitions (docstring, attributes, body verbatim), dealt them to the parts, derived each part's imports and sharing from its code and wrote the parts into `after/`; headers are generated from the table in the tool.
3. Verified in the scratch (§4.1), then every repository file was written with the Write tool and compared with `cmp` against the verified scratch file: 32 files, all identical.
4. The parts were written first (inert: nothing imported or mounted them), then in quick succession (00:27–00:28): the stage TS suite (three Edits), the Rust façade suite, the glue mounts, the Rust façade, the TypeScript façade.

## 4. Proofs (real output)

### 4.1 Before landing, in the scratch (committed core + split)

| Check | Result |
|---|---|
| `bun TK/split_stage.ts check --old before --new after` | `ts: 143 definitions in the old stage (1728 lines), 143 in 11 parts (1956 lines), 143 identical` · `rs: 144 definitions in the old stage (2215 lines), 144 in 11 parts (2474 lines), 144 identical` · `0 problem(s)` (also: no part imports a name it does not use or uses one it does not import) |
| `tsc --noEmit` (scratch tsconfig) | exit 0 |
| vitest fundamental / quick | `Tests 510 passed (510)` / `Tests 510 passed (510)` |
| `bun TK/a1_scratch.ts traces after` | `after: 13 of 13 scripts replay into their committed trace (128000 ticks)` |
| `bash TK/a1_cargo.sh clippy --offline --all-targets --features sut -- -D warnings` | exit 0, no finding |
| `bash TK/a1_cargo.sh test --offline --lib` | `test result: ok. 189 passed; 0 failed` |
| `rustfmt --check` (20 Rust files) | exit 0 |
| long run, TypeScript: the 48 sessions recorded on the live tree before the split, replayed through `before/` and `after/` | `48 and 48 sessions, 57600 checkpoints compared (two digests each), 0 mismatches` (both pairs) |
| long run, Rust (`after/`) against TypeScript (`after/`) | `rust against typescript: 48 sessions, 57600 checkpoints compared (two digests each), 0 mismatches; first mismatch: none` |

### 4.2 After landing, on the live tree

| Check | Result |
|---|---|
| `bun TK/split_stage.ts check --old <before> --new P` | `143 … 143 identical`, `144 … 144 identical`, `0 problem(s)` |
| `bun ./📜️script.ts typecheck` (`P/📦️packages/🟦️typescript`) | exit 0 |
| `bun ./📜️script.ts test` | `Test Files 12 passed (12)`, `Tests 837 passed (837)` (the siblings' new suites included) |
| `bun ./📜️script.ts test quick` | `Tests 3 failed / 834 passed (837)`: the three failures are in sibling modules still being written — `🧗️climbing` (`ladderFor > keeps every law on generated stages`) and `🚧️clearance` (two tests) —, none in the stage. The stage suite alone at quick: `Tests 185 passed (185)` |
| `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @semio-tech/pets-rs:test` | `test result: ok. 192 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out` · `Successfully ran target test` |
| `… @semio-tech/pets-rs:test-quick` | `test result: ok. 199 passed; 0 failed` (incl. `stage::tests::quick::every_script_replays_into_its_committed_trace_however_time_is_cut`) |
| `RUSTC_WRAPPER="" cargo clippy -p semio-framework-pets --all-targets --features sut --offline -- -D warnings` | exit 0 |
| the same with `--target wasm32-unknown-unknown` (lib) | exit 0 (`bun ./📜️script.ts verify rust-warnings --target native -p semio-framework-pets` stops before compiling with `native misses semio-hub-dag.`: the gate no longer accepts a single package, so clippy was run directly) |
| `rustfmt --check` (20 files + glue) | exit 0 |
| `.venv/Scripts/python.exe TK/generate_behavior_vectors.py` | `unchanged …🧠️behavior-choice…`, `unchanged …🤝️bond-dynamics…`, `traces as committed: 13 of 13 scripts`, `unchanged …🎪️stage-trace\🔣️.json` |
| in `TEST`: `bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "🎪️stage-trace"` | `[test] level=exhaustive cases=1 executed=6 passed=6 failed=0 errored=0 parity=3/3` |
| long run, TypeScript before (old stage, recorded 23:59–00:02) vs after (split, 00:4x) | `48 and 48 sessions, 57600 checkpoints compared (two digests each), 0 mismatches`; the two `typescript-digests.json` are byte-identical (18 883 events in both) |
| long run, Rust (`bash TK/rust_scratch.sh wp-m run --release --offline --features sut --example long_trace_digest`, wp-m crate mounting the live split twins) | `rust against typescript: 48 sessions, 57600 checkpoints compared (two digests each), 0 mismatches; first mismatch: none` |
| `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/🐾️pets"` | `clean=true errors=0 warnings=0` |
| `bun TK/a1_docstring_check.ts` | `0 problem(s) in 32 file(s)` (every docstring starts with an emoji, none repeated in a file, no console or `[DEBUG]`) |

The stage-trace fixture: A2 regenerated it at 00:17 (before this split landed) because the troupe gained the new species members; its scripts and expected traces are untouched. Compared with the committed one (`HEAD`), all 13 scripts carry the same steps and the same expected trace (digests `calm-home 1758177737`, `lively-card 3055031182`, `moving-card 3448150708`, `quiet-run 811069282`, `still-and-back 2032106604`, `scene-change 1529883008`, `pokes-and-glances 245003349`, `narrow-stage 1073395578`, `crowded-strip 2275531786`, `tab-and-footer 1444943760`, `pointer-rest 3777510391`, `pointer-attention 3326711053`, `new-ground 3213029212`); only the menagerie differs. Because A2 was regenerating the same fixtures, the generator was first run through `TK/a1_generator_check.py` (the generator unchanged, its fixture writes replaced by a byte comparison): all three `unchanged`; only then the real generator, which therefore wrote nothing.

### 4.3 The comparison can still fail

`bun TK/stage_scratch.ts mutants` (M's 18 deliberately broken twins, now pointed at the part each expression lives in, against the live sessions): `18 mutants, 3 survived (3 expected to), 0 surprises`. Every count is the one of `📓️report-wp-m.md` §5 — fused goal 13 438, reassociated goal 16 440, swapped blink words 53 838, eye height one ulp off 23 767, reassociated glide 13, fused eye 3 191, `TURN_REST` 57 32 802, `PERK_LINGER` 33 15 066, `PERK_COST` one ulp off 1 976, fused lean 30 000, ground as good as a shelf 57 600, every survey new ground 32 546, falls from vanished ground 12 161, fused energy 10 904, reassociated fade 6 140; the fused mood, the eager-one-tick and the reassociated fidget survive as before —, so the split twin is not only equal on the sessions but breaks in exactly the same places.

## 5. Decisions and deviations

1. **`📏️spacing`, not `🚧️clearance`** (brief over design-v2 §15.2): the first round's grounded spacing lives in `📏️spacing`; `🚧️clearance` is A5's new pure module.
2. **`carve` and `crowdOn` stay in `📝️draft`** as CORE §5 lists them (spacing and population both use `crowdOn`).
3. **Header emojis**: a file's header must not repeat a docstring emoji of the same file and the per-function docstrings are kept verbatim, so four parts open with another emoji than their directory: `📝️draft` → ✏️ (`draftOf` is 📝️), `👀️attention` → 👁️ (`gazeGoal` is 👀️), `🚶️locomotion` → 🏃️ (`stroll` is 🚶️), `🎥️projection` → 📽️ (`frameOf` is 🎥️). Every header ends with `@see` to the façade and to its twin.
4. **Private Rust mounts** (`mod x;` in the glue): the crate's public surface is unchanged, and nothing new appears under `pets::`.
5. **Test-only sharing** of five constants (§2).
6. **Formatting**: prettier is not a gate for these files and the committed stage was not prettier-clean (42 lines differ under `prettier`, the suite 112, the README 130). Because bodies are byte-identical, `🎯️choice`, `👥️population`, `🕰️clock` and the stage suite keep those lines; the other parts and both façades are prettier-clean. Rust is `rustfmt`-clean.
7. **README** (`P/README.md`, layout table): the façade row now says what it is, and one row per part was added (anchored Edit; the file is also being edited by siblings).
8. **Ticket tools**: `long_trace_digest.ts` read `default` of `🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts`, which exports `ARCHITECTURE_MENAGERIE` today — it now takes that export (the record failed without it). `stage_scratch.ts` mounts the ten parts privately (`PARTS`), its mutants name the part their expression lives in (`MUTABLE`), and the test mount is only stripped where a part has one.
9. **Ticket bookkeeping**: `🎫️ticket.json` was not touched (the coordinator keeps it).

## 6. What later packages must know

- **Where things go.** A new footing or way to travel: its start, its tick of motion and its end in `🚶️locomotion`; its branch in `act` (`🕰️clock`); the "moving" tests in `lull` (`🕰️clock`), `paceOf` (`🎥️projection`, rate 64) and `gazeGoal` (`👀️attention`, `moving`); if it counts as a mover, `decide` (`🎯️choice`), `pair` (`💞️sociability`), `tune` (`👥️population`). A new time horizon: a function in `🗓️schedule` when it concerns the whole stage, and a line in all three of `lull` (`🕰️clock`), `paceOf` and the `wake` of `frameOf` (`🎥️projection`) — CORE X4 is now spread over two files, the clock header says so. A new activity an actor chooses by itself: `decide`/`conclude` (`🎯️choice`). What the pointer or the hand does to one pet: `👀️attention`; between pets and pokes: `💞️sociability`; surveys, walls, fixtures, arrivals: `👥️population`; frame fields: `🎥️projection`; a helper several parts need: `📝️draft` (keep the import graph of §1 acyclic: nothing below may import something above it).
- **A new event** is a branch in `apply` of the façade (TypeScript: the if-chain ends with `else poke(…)`, so a new kind must be added before it; Rust: `match` is exhaustive) and a handler in the part that owns it.
- **New runtime state**: TypeScript `Draft` is a mapped type over `Stage` and `Body` over `Actor`, so new fields arrive by themselves; `draftOf`/`sealed` copy fields by name and must list a new `Stage` field (both languages: Rust `Draft { stage, kinds, streams }` holds the `Stage` itself and needs nothing).
- **Sharing rules**: TypeScript `export` only what another part imports; Rust `pub(crate)`; a new part is mounted `mod <name>;` in `P/📦️packages/🦀️rust/🦀️.rs`, gets a directory registered in `members-of-modules`, and must be added to `PARTS` in the stage TS suite and to `SOURCES` in the Rust stage suite (the arithmetic checks) and, for the long-run tools, to `PARTS` in `TK/stage_scratch.ts`.
- **Tests**: Rust unit tests belong to `<part>/🧪️tests/🔬️unit/🦀️.rs` and may borrow `crate::stage::tests::{…}`; the TypeScript stage suite remains the place for behaviour seen through `advance`/`frameOf`.
- **Proof tools for the next refactor**: `TK/split_stage.ts check` compares the parts with the committed (pre-split) stage, so it is only meaningful until behaviour changes land; its `--old` tree was the snapshot of commit `83025db9517` (removed from the output; `git show 83025db9517:<path>` recreates the two stage files). `TK/a1_scratch.ts` and `TK/a1_cargo.sh` expect such snapshots below `🗑️generated/a1/{before,after}`. The long run: `bun TK/long_trace_digest.ts`, `bun TK/stage_scratch.ts prepare`, `bash TK/rust_scratch.sh wp-m run --release --offline --features sut --example long_trace_digest`; `bun TK/a1_scratch.ts compare <a> <b>` compares two TypeScript recordings checkpoint by checkpoint.

## 7. Files

Created (repository): `M/{📝️draft,📏️spacing,🗓️schedule,👀️attention,🚶️locomotion,💞️sociability,🎯️choice,👥️population,🕰️clock,🎥️projection}/{🟦️.ts,🦀️.rs}`; `M/{📝️draft,🗓️schedule,👀️attention,🚶️locomotion,💞️sociability,👥️population,🕰️clock,🎥️projection}/🧪️tests/🔬️unit/🦀️.rs`.
Updated (repository): `M/🎪️stage/🟦️.ts`, `M/🎪️stage/🦀️.rs`, `M/🎪️stage/🧪️tests/🔬️unit/🟦️.ts`, `M/🎪️stage/🧪️tests/🔬️unit/🦀️.rs`, `P/📦️packages/🦀️rust/🦀️.rs`, `P/README.md`.
Ticket tools: created `TK/split_stage.ts`, `TK/a1_scratch.ts`, `TK/a1_cargo.sh`, `TK/a1_generator_check.py`, `TK/a1_docstring_check.ts`; updated `TK/long_trace_digest.ts`, `TK/stage_scratch.ts`.
Output: `TK/🗑️generated/a1/` (logs of every run above, the recorded digests before and after; the scratch trees and cargo directories were removed). `TK/🗑️generated/wp-m/` keeps the live long-run sessions and digests (`long-trace/`), the regenerated scratch crate and `mutants.txt`; its cargo directories were removed (`bun TK/stage_scratch.ts prepare` and the runner rebuild them).
