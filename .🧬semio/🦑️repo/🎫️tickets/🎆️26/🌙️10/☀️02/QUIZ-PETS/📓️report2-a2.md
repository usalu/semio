# 📓️ Work package A2 (document contract, tables, registrations) — report

Ticket `2026/10/02/QUIZ-PETS`, second round, written 2026-10-02/03 (night). `P` = `🧰️framework/🛍️products/🐾️pets`, `AP` = `🎓️teaching/🏛️architecture/🐾️pets`, `Q` = `🧰️framework/🛍️products/❓️quiz`, `TK` = this ticket folder, `TEST` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`. Raw tool output: `TK/🗑️generated/a2/` (kept for the coordinator; it goes with the ticket's cleanup).

**Status: done.** Names registered, schema types exported (first version of this file at 00:55), activities extended, validators and documents switched to the required members, tables, vectors, README and verification finished. The repo MCP was unreachable (`CONNECTION_CLOSED`), so no ticket bookkeeping was done from this work package; nothing was committed.

## 1. What exists

### 1.1 Registrations

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, three anchored edits (never rewritten from parsed JSON):

- `members-of-modules` (after `🐾️pets`): `📝️draft 🚧️clearance 📏️spacing 🗓️schedule 👀️attention 🚶️locomotion 💞️sociability 🎯️choice 👥️population 🕰️clock 🎥️projection 🪢️swing 🧗️climbing 👆️gesture 💗️feeling ✨️effects 🪄️mischief 🤏️grasp 🪞️lifting 🧰️gear`
- `members-of-tests` (after `🎪️stage-trace`): `🪢️swing-dynamics 🪂️parachute-descent 🧗️wall-climbing 🪜️ladder-geometry 🎣️grapple-reach 🚧️clearance-proof 👆️gesture-recognition 💗️feeling-dynamics ⚗️chemistry-rules ✨️particle-motion 🪄️mischief-choice 🤏️pet-handling 🪞️fixture-lifting 🎆️effect-painting 🧰️gear-depiction 🎮️pet-play`
- `members-of-fixtures` (after `🎪️stage-trace`): the first eleven of those.

`📏️spacing` uses the same `📏` + U+FE0F as `📏️scoring` and resolves as `members-of-modules`.

### 1.2 Contract (design-v2 §21), in all three twins

| Name | TypeScript `P/🧬️schema/🟦️.ts` | JSON Schema `🔣️.json` `$defs` | Rust `🦀️.rs` |
|---|---|---|---|
| `ACTIVITIES` / `Activity` | 26 entries: the first eleven, then `hang tumble glide aim reel climb mantle slide carry trick purr dizzy shrug scoot push` | `Activity` enum; `Repertoire` has 26 optional lists | `Activity` (26 variants, `as_str`), `ACTIVITIES: [Activity; 26]`, `Repertoire` 26 fields + `clips(activity)` |
| `MOODS` / `Mood` | `content happy playful curious proud sleepy grumpy sad scared` | `Mood` | `Mood` (`Default` = `Content`), `MOODS` |
| `FOOTINGS` / `Footing` | `perch air chute hand rope ladder wall head` | `Footing` | `Footing`, `FOOTINGS` |
| `GEARS` / `Gear` | `climb ladder grapple parachute` | `Gear` | `Gear`, `GEARS` |
| `CUES` / `Cue` | `click circle countercircle stroke shake whim show` | `Cue` | `Cue`, `CUES` |
| `DRIFTS` / `Drift` | `fall rise burst orbit drift` | `Drift` | `Drift`, `DRIFTS` |
| `DEEDS` / `Deed` | `hello trick pet toss` | `Deed` | `Deed`, `DEEDS` |
| `Tint`, `Emitter`, `SpeciesState`, `Trick`, `Purr` | as §21 | objects, closed | structs (`Emitter.stroke_width` ↔ `strokeWidth`; `Trick.from` keeps its name) |
| `Trait`, `Effect`, `Reaction` | as §21 | objects, closed | structs; `Effect.on: Party` (`When`/`Near`, inline enum, `PARTIES`), `Effect.encounter: Option<Activity>` that only decodes `greet`/`cuddle`/`squabble`, `Reaction.place: Option<Placement>` (`where` on the wire; inline enum `Placement`, `PLACEMENTS`) |
| `Species` | `+ states, tricks, purr, emitters, gear, grip, reach, canopy?, mood` | required (all but `canopy`); `states` minItems 1, `gear` uniqueItems | required fields (no serde default), `canopy: Option<Shape>` |
| `Menagerie`, `Ensemble` | `+ chemistry: readonly Reaction[]` | required | `chemistry: Vec<Reaction>` |

`$defs` count 57 → 71 (pinned by the TypeScript bijection test and the Rust twin test). Units and ranges are stated in the docstrings and README: `Emitter.count` whole 1…32 alive at once, `life` seconds > 0, `speed` px/s ≥ 0, `spread` 0…1 of a full turn, `x`/`y` in the bone's coordinates; `SpeciesState.lasts` seconds > 0; `grip` > 0 and ≤ height; `reach` ≥ 0 and ≤ width; `Reaction.within` px > 0, `every` s > 0, `chance` 0…1; `Effect.amount` 0…1, `rapport` −1…1; `then` at least one effect; a trick may have no cues (asked for by chemistry only).

### 1.3 Validation (`P/🔨️modules/✅️validation/{🟦️.ts,🦀️.rs}`, Python reference in `P/🧪️tests/🧬️schema-conformance/🐍️.py`)

New members are checked structurally (closed objects, enums, types, whole `count`) and by rules. New rule codes:

| Code | Reported at | Rule |
|---|---|---|
| `duplicate-entry` | `/gear/i`, `/tricks/i/cues/j`, `/tricks/i/from/j` | gear, cues and `from` states are listed once (schema: `uniqueItems`) |
| `lasts-then` | `/states/i/then` | a state with `lasts` names `then` (one direction: `then` without `lasts` is accepted) |
| `missing-gear-clip` | `/gear/i` | `climb` → clips for `climb`, `mantle`, `slide`; `ladder` → `carry`, `climb`; `grapple` → `aim`, `reel`; `parachute` → `glide` |
| `missing-activity-clip` | `/repertoire/<activity>` | every species maps `hang`, `tumble`, `purr`, `dizzy`, `shrug`, `push` |
| `floater-gear` | `/gear/i` | a `float` species owns neither `climb` nor `ladder` nor `grapple` |

Existing codes cover the rest: `duplicate-id` (ids unique per list: states, tricks, emitters of a species; reaction ids of a menagerie or ensemble), `unknown-reference` (`state.clip`, `state.emitter`, `state.then`, `trick.clip`, `trick.emitter`, `trick.from[j]`, `trick.to`, `purr.clip`, `purr.emitter`, `emitter.bone`; trait `species` and `state`; effect `state` and `trick` against the species of the side `on` names — not judged when that species is unknown; with repeated species ids the first wins), `out-of-range` (all ranges above; `grip`/`reach` against the size only when that side is itself > 0; tint colours), `items-too-few` (`/states`, `/chemistry/i/then`), `slug-invalid`, `length-invalid`, `type-invalid` (also a fractional `count`), `value-invalid` (moods, gear, cues, drifts, effect side, encounter, placement, trait activity). An ensemble's reactions are judged without species (they resolve in the assembled menagerie). README table updated (`P/README.md` "Validation issues").

Vectors (`P/🧫️fixtures/🧬️schema-conformance/🔣️.json`, from `TK/generate_schema_vectors.py`): **32 accepted / 54 structural / 126 rules** (was 21/26/63), 25 codes. One existing vector was renamed in substance: `undeclared-property` now adds `nickname` (it used `mood`, which is a member now).

### 1.4 Documents

- **The sample menagerie** (generator, fixture keys `menagerie`, `ensemble`, `species`, `bases`) exercises everything: `blobby` maps all 26 activities (new clips `shimmer` and `dangle`), states `resting` → `glowing` (tint, emitter `sparkle`, 30 s → resting) → `radiant` (tint, overlay `shimmer`, `sparkle`, 12 s → glowing), six tricks (`cheer` click/whim + emitter, `brighten` and `blaze` circle up the ladder, `wane` and `fade` countercircle down, `shake-off` shake), emitters `sparkle` (burst, path star) and `hum` (rise, ellipse, strokeWidth 1), purr `nuzzle` + `hum`, all four gears, a path canopy, mood `content`; `hoppy`: states `resting`, `wound-up` (tint, 10 s), tricks `boing` (click, stroke) and `wind-up` (circle, shake; resting → wound-up), purr `snuggle`, gear grapple + parachute, mood `playful`; `floaty`: resting only, purr `glow`, parachute (glide → `drift`), mood `sleepy`. Three reactions (`radiance-cheers-the-spring`, `a-wound-up-spring-startles-the-balloon` below, `a-sleepy-balloon-calms-a-grumpy-blob` above) use every effect member. Bases (`mini`) carry the minimal members.
- **The twenty species** (`AP/<species>/🔣️.json`, one anchored edit each) and **the ensemble** `AP/🔣️.json` (`"chemistry": []`): see §3.
- **The trace troupe** (`TK/generate_behavior_vectors.py` → `🧫️fixtures/🎪️stage-trace` menagerie, `"chemistry": []`) and **the reference species** (`TK/reference-species.json`, also embedded by the animation and kinematics generators into `🎞️animation-sampling` and `🦴️rig-solving`; `TK/reference_menagerie.ts` gained `chemistry: []`).
- **Test literals** that are typed `Species`/`Menagerie` got the members so the React target and the quiz still typecheck: `P/🧪️tests/🖌️pet-depiction/🟦️.tsx` (SPECIMEN), `P/🧪️tests/🫥️decorative-layer/🟦️.tsx` (valid menagerie: owed activities mapped to its clips, `chemistry: []`), `Q/🧪️tests/🐾️pet-companions/🟦️.tsx` (doubles), the Rust adapters `🧠️behavior-choice/🦀️.rs` (inline species) and `🤝️bond-dynamics/🦀️.rs` (inline menagerie), the Rust behaviour unit suite (`kind()`, `bonded()`).

### 1.5 Behaviour tables (`P/🔨️modules/🧠️behavior/{🟦️.ts,🦀️.rs}`, `🧪️tests/🧠️behavior-choice/🐍️.py`, `🤝️bond-dynamics/🐍️.py`)

| Activity | dwell low…high (ticks) | spirit (`moodOf`) | energy /s | sociability /s | curiosity /s | rapport step | followers |
|---|---|---|---|---|---|---|---|
| hang | 1920…1920 | −0.3 | 0 | 0 | 0 | 0 | idle |
| tumble | 640…640 | −0.2 | 0 | 0 | 0 | 0 | idle |
| glide | 1280…1280 | 0.4 | 0 | 0.002 | −0.04 | 0 | idle |
| aim | 32…64 | 0.3 | −0.004 | 0.002 | −0.06 | 0 | idle |
| reel | 640…640 | 0.5 | −0.012 | 0.002 | −0.08 | 0 | idle |
| climb | 1920…1920 | 0.4 | −0.016 | 0.002 | −0.08 | 0 | idle |
| mantle | 32…32 | 0.4 | −0.016 | 0.002 | −0.08 | 0 | idle |
| slide | 640…640 | 0.3 | −0.002 | 0.002 | −0.04 | 0 | idle |
| carry | 1920…1920 | 0.3 | −0.014 | 0.002 | −0.06 | 0 | idle |
| trick | 128…128 | 0.8 | −0.01 | 0.002 | −0.08 | 0 | idle |
| purr | 192…384 | 1 | +0.004 | −0.06 | +0.01 | 0 | idle |
| dizzy | 96…160 | −0.3 | −0.002 | 0.002 | 0 | 0 | idle |
| shrug | 64…96 | −0.1 | −0.001 | 0.002 | +0.01 | 0 | idle |
| scoot | 320…320 | 0.2 | −0.012 | 0.002 | −0.02 | 0 | idle |
| push | 320…640 | 0.6 | −0.014 | 0.002 | −0.1 | 0 | idle |

`activityWeights` returns 26 numbers, 0 for every new activity; `idle` lists all fifteen as followers (graph still one strongly connected component). The existing eleven are untouched: the thirteen stage traces were re-recorded from the TypeScript subject and compared with the committed ones by a new guard in the generator (`traces as committed: 13 of 13 scripts`); behaviour of the existing eleven in the vectors is identical (only rows for the new activities were added). The React depiction (`ACTIVITIES.indexOf(frame.activity)` as a slot, `data-pet-activity` by name) and the stories gallery (iterates `ACTIVITIES`) need no change; their suites pass.

## 2. What is minimal (art agents replace these)

Every one of the twenty species got exactly: `states: [{ id: "resting", name: { en: "Resting", de: "In Ruhe" } }]`, `tricks: []`, `emitters: []`, `mood: "content"`, `purr.clip` = its cuddle clip, `grip` = 0.9 × height and `reach` = 0.25 × width (rounded to halves), `gear` by the refusals of CONTENT §5 (everything a species does not refuse; floaters nothing), and repertoire entries that only reuse existing clips: `hang` → idle loop, `tumble` → its fall clip (idle loop when it has none), `purr` → cuddle clip, `dizzy` and `shrug` → its second fidget, `push` → its first fidget, and for its gear `climb`/`mantle`/`carry` → its walk clip, `slide` → fall clip or idle loop, `glide` → idle loop, `aim` → first fidget, `reel` → second fidget. No new clips, states, tricks, emitters or canopies were drawn.

| Species | gear | purr | grip | reach | placeholder entries (activity → existing clip) |
|---|---|---|---|---|---|
| sunny (float) | — | bask | 46 | 11.5 | hang, tumble → bob; dizzy, shrug → ray-spin; push → ray-wave |
| cloudy (float) | — | snuggle | 38.5 | 14.5 | hang, tumble → drift; dizzy, shrug → billow; push → drip |
| housy | ladder, grapple | shelter | 47 | 12 | hang, tumble → breathe; aim, push → smoke-ring; reel, dizzy, shrug → shiver; climb, carry → waddle |
| solary | climb, ladder, parachute | snuggle | 47 | 11.5 | hang, tumble, glide, slide → bask; climb, mantle, carry → stroll; dizzy, shrug → gleam; push → sunbathe |
| radiatory | climb, ladder | hug | 49.5 | 13.5 | hang → simmer; tumble, slide → tumble; climb, mantle, carry → shuffle; dizzy, shrug → thermostat; push → ripple |
| pumpy | parachute | purr | 41.5 | 11.5 | hang, glide → hum; tumble → tumble; dizzy, shrug → exchange; push → rev |
| windowy (hop) | climb, parachute | open-up | 47 | 10 | hang, tumble, glide, slide → breathe; climb, mantle → hop; dizzy, shrug → gleam; push → air |
| waly | ladder | lean-on | 41.5 | 12 | hang, tumble → stand; climb, carry → march; dizzy, shrug → knock; push → loose-brick |
| battery | climb, ladder, grapple | snuggle | 47 | 10.5 | hang, tumble, slide → hum; climb, mantle, carry → bounce; aim, push → recharge; reel, dizzy, shrug → zap |
| windy | parachute | nuzzle | 50.5 | 12 | hang, glide → breeze; tumble → tumble; dizzy, shrug → lull; push → gust |
| boily | ladder, grapple | warm-up | 47.5 | 12 | hang → simmer; tumble → tumble; aim, push → whoomp; reel, dizzy, shrug → puff; climb, carry → plod |
| roofy | ladder, parachute | shelter | 38 | 14 | hang, tumble, glide → breathe; climb, carry → scamper; dizzy, shrug → ripple; push → loose-tile |
| insuly (hop) | climb, parachute | snuggle | 36 | 12 | hang, tumble, glide, slide → breathe; climb, mantle → hop; dizzy, shrug → tuck-in; push → fluff-up |
| shady | ladder, parachute | chill | 43 | 11 | hang, tumble, glide → lounge; climb, carry → shuffle; dizzy, shrug → roll; push → slat-wave |
| venty (float) | — | cosy | 37 | 11 | hang, tumble → hover; dizzy, shrug → exhale; push → spin-up |
| chilly | climb, parachute | thaw | 39.5 | 13 | hang, glide → chill; tumble, slide → tumble; climb, mantle → glide; dizzy, shrug → flurry; push → frost-over |
| kettly | grapple, parachute | cosy | 39.5 | 11.5 | hang, glide → simmer; tumble → tumble; aim, push → boil; reel, dizzy, shrug → sigh |
| flamy | — | warm | 41.5 | 7 | hang → glow; tumble → tumble; dizzy, shrug → ember; push → flicker |
| thermy | grapple, parachute | glow | 47 | 8 | hang, glide → steady; tumble → tumble; aim, push → reading; reel, dizzy, shrug → tap |
| servy | grapple | sync | 50.5 | 11.5 | hang, tumble → hum; aim, push → count; reel, dizzy, shrug → overheat |

Fixtures: the **trace troupe** — one resting state, no tricks or emitters, purr `nuzzle`, grip/reach 0.9 × height / 0.25 × width, gear mossy all four, sparky grapple + parachute, thorny climb, pebble none, misty parachute; moods happy, playful, grumpy, sleepy, content; owed activities on existing clips (hang → idle loop, tumble → `tumble`, purr → `nuzzle`, dizzy → `wiggle`, shrug → `stretch`, push → `bristle`, gear clips → `stroll`/`tumble`/`stretch`/`breathe`). The **reference species** (`TK/reference-species.json`) — resting state, purr/hang/tumble → `breathe`, dizzy/shrug/push → `wiggle`, no gear, grip 27, reach 10. The **bases** (`mini`) — the same pattern on `rest`/`step`. Only the sample menagerie carries real states, tricks, emitters, gear, canopy and chemistry.

## 3. Verification (real results)

From the repository root unless stated; level `fundamental` unless stated.

| # | Command | Result |
|---|---|---|
| 1 | `bun TK/a2_registration_check.ts`; `bun TK/taxonomy_name_probe_v2.ts` | `missing: 0 repeated: 0`, `validateTaxonomy problems: 0`; probe `0 fresh, 19 / 16 / 11 already present`, `baseline: 0 after: 0` |
| 2 | `cd P/📦️packages/🟦️typescript && bun ./📜️script.ts test` | exit 0, `Test Files 14 passed (14)`, `Tests 931 passed (931)` (siblings' new suites included) |
| 3 | same, `test quick` | exit 1: `3 failed / 928 passed` — all three in siblings' in-flight suites (`🚧️clearance`: "agrees with a walk along a 1/64 px lattice…" 19 > 20, "lets bodies overlap as soon as the rule 'eviction' is switched off" 0 > 0; `🧗️climbing`: "ladderFor keeps every law on generated stages" stage 120); none touches the contract |
| 4 | same, `typecheck` | exit 0 |
| 5 | `cd P/🎯️targets/⚛️react/📦️packages/🟦️typescript && bun ./📜️script.ts typecheck`; `… test` | exit 0; exit 0, `Test Files 6 passed (6)`, `Tests 105 passed (105)` |
| 6 | `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @semio-tech/pets-rs:test --skip-nx-cache` | exit 0, `192 passed; 0 failed` |
| 7 | `RUSTC_WRAPPER="" cargo clippy -p semio-framework-pets --all-targets --features sut --offline -- -D warnings` | exit 0 |
| 8 | `rustfmt --check --edition 2021 --config-path rustfmt.toml` on the eight Rust files touched | exit 0 (after two hand-applied wraps) |
| 9 | `cd Q/🎯️targets/⚛️react/📦️packages/🟦️typescript && bun ./📜️script.ts test pet-companions` | exit 0, `Tests 33 passed (33)`. The whole quiz suite: 39 failures in `translation-completeness`, `crowd-client`, `deputy-decisions`, `task-keyboard`, `learner-journey`, and `typecheck` 54 errors in quiz core/views (`challenge`, `Best`, `cards` …) — all from other tickets' in-flight quiz work; no pets file is named |
| 10 | `cd 🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript && bun ./📜️script.ts test` | exit 0, `Test Files 5 passed (5)`, `Tests 137 passed (137)` |
| 11 | `bun TK/validate_species.ts 🎓️teaching/🏛️architecture/🐾️pets/*/🔣️.json` | all twenty `ok` |
| 12 | `cd TEST && bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "🧬️schema-conformance"` | `executed=9 passed=9 failed=0 errored=0 parity=9/9` |
| 13 | same, `--case "🧠️behavior-choice"` | `executed=27 passed=27 failed=0 errored=0 parity=27/27` |
| 14 | same, `--case "🤝️bond-dynamics"` | `executed=18 passed=18 failed=0 errored=0 parity=18/18` |
| 15 | same, `--case "🎪️stage-trace"` | `executed=6 passed=6 failed=0 errored=0 parity=3/3` |
| 16 | `.venv/Scripts/python.exe -X utf8 TK/generate_schema_vectors.py` (twice) | `accepted=32 structural=54 rules=126 codes=…` (25 codes), second run `unchanged` |
| 17 | `.venv/Scripts/python.exe -X utf8 TK/generate_behavior_vectors.py` (twice) | `traces as committed: 13 of 13 scripts`; second run `unchanged` for behavior-choice, bond-dynamics, stage-trace |
| 18 | `TK/generate_animation_vectors.py`, `TK/generate_kinematics_vectors.py` (twice) | second runs: hashes of `🎞️animation-sampling`/`🪀️spring-settling` identical, kinematics `unchanged` for all four fixtures |
| 19 | `bun ./📜️script.ts schema check` before / after `schema generate` + `schema docs` | `findings=9325` with `schema-catalog-stale=1` → `findings=9324`, not stale; no finding names pets |
| 20 | `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/🐾️pets"` | `clean=true errors=0 warnings=0` (149 directories, 217 files with the siblings' new ones) |
| 21 | same, `--scope "🎓️teaching"` | `clean=false errors=18 warnings=0`: every error is in files of the `🎓️teaching` root that another ticket added (`package.json`, `Cargo.toml`, `Cargo.lock`, `bun.lock` — normalization moves and the `🔒️.lock` collision —, `.config/nextest.toml`) and one `generator-preview-invalid` of `.vscode/launch.json`; none names `🐾️pets` or a file of this work package |
| 22 | `node TK/a2_docstring_emojis.mjs` (21 touched files: TS, Rust, Python) | `problems 0` |

## 4. Deviations from design-v2 §21 and decisions

1. **Rust inline enums.** `Effect.on` is `Party { When, Near }` and `Reaction.where` is `place: Option<Placement>` (`where` is a Rust keyword; the wire name stays `where`); both are not `$defs` and are listed as inline in the twin test beside `Facing` and `Rate`. `Effect.encounter` is `Option<Activity>` with a deserializer that refuses anything but greet/cuddle/squabble, because the façade already exports the behaviour module's `Encounter = Activity` and `ENCOUNTERS`.
2. **Ranges** were not in §21 and were decided as in §1.2 (count whole 1…32 — the stage-wide cap of 160 stays the effects module's business; spread in turns).
3. **Ids are unique per list** (states, tricks, emitters separately), since effects name a state and a trick in separate members.
4. **`lasts-then` is one-directional** as briefed; `then` without `lasts` is accepted (vector `state-that-names-a-successor-and-stays`).
5. **`grip`/`reach` upper bounds** are checked only against a size side that is itself above 0, so a broken size is reported once.
6. **A trick may have no cue** (asked for only by chemistry); a reaction has at least one effect (`items-too-few`).
7. **Gear by nature**: from the refusals in CONTENT §5, not "all four unless unfit"; floaters (sunny, cloudy, venty) own nothing because CONTENT has them refuse the parachute too.
8. **The behaviour module's private `MOODS` table** (activity → the first round's scalar) keeps its name; the schema now exports `MOODS` (the nine moods). They do not collide (the table is module-private in both languages), but B2 may want to rename the table to `SPIRITS` when the scalar becomes "spirits" (§21 frame).
9. **Transient leniency.** For about an hour the new members were optional on the wire (JSON `required`, validators, serde defaults) so every save stayed green while the documents were updated; the final state has no trace of it.
10. **Generators:** `generate_behavior_vectors.py` gained `--tables` (behaviour and bond fixtures only) and a guard that refuses to write traces that differ from the committed ones unless `--rerecord` is passed; the troupe's `species()` takes gear and mood. `generate_schema_vectors.py`: new builders (`state`, `trick`, `emitter`, `reaction`, `hello`), the second-round sample and vectors.
11. **Fixture size:** `🧫️fixtures/🧬️schema-conformance/🔣️.json` grew from 530 kB to 1.17 MB (each inline vector embeds the larger base species).
12. **Schema catalog** regenerated (`🔣️schema-catalog.json`, `📓️schema-catalog.md`), which also refreshed entries of other tickets' schemas.

## 5. For the siblings

- A species literal in TypeScript, a JSON species in Rust tests and any test menagerie now need `states`, `tricks`, `purr`, `emitters`, `gear`, `grip`, `reach`, `mood` (and `chemistry` on menageries/ensembles); a menagerie shown by the layer must also have no findings, i.e. clips for `hang tumble purr dizzy shrug push` and for its gear's activities.
- New activities weigh 0 in `activityWeights`, are followed only by `idle`, and are reachable from `idle`; B-phase integrators change the tables, the Python oracles (`🧠️behavior-choice/🐍️.py`, `🤝️bond-dynamics/🐍️.py` — the lists `EPISODES`, `DWELL_*`, `MOODS`, `FOLLOWERS`, rates) and rerun `TK/generate_behavior_vectors.py` (with `--rerecord` when the traces move on purpose).
- `TK/check_motion_bits.rs` (a first-round proof tool, not in any gate) builds a species without the new members and would need them if it is ever run again.

## 6. Open

- `verify taxonomy report --scope "🎓️teaching"` is not clean because of the teaching root workspace files of another ticket (§3 row 21); the pets scope is clean, and every directory name of the second round resolves (§1.1) whether or not the siblings have created it yet.
- The quiz package's own failures (§3 row 9) and the three `quick`-level failures in `🚧️clearance` and `🧗️climbing` (§3 row 3) belong to other work; nothing in them names the contract.
- `TK/🗑️generated/a2/` holds the logs of this work package; it goes with the ticket's cleanup.
