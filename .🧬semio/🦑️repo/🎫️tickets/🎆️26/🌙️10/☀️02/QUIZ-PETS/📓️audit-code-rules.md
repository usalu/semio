# 📓️ Audit — code against the repo rules (QUIZ-PETS)

Read-only audit of ticket `2026/10/02/QUIZ-PETS`. Reads ran 2026-10-02 06:56 – 07:14 (local). Other agents were editing during the audit, so this is a snapshot:

| File | Last write seen | Remark |
|---|---|---|
| `P/🔨️modules/🎪️stage/🦀️.rs` | 07:02:32 | changed once **after** my read (06:58–07:00): the `act` match arm `Activity::Sleep if watched(..) =>` (now l. 825) replaced an inner `if`; same semantics. Line numbers below are of the 07:02 file. |
| `P/🧪️tests/🎪️stage-trace/🦀️.rs` | 07:01 | did not exist at 06:57; appeared while I audited. Not audited (tests). |
| `P/🔨️modules/🎪️stage/🟦️.ts` | 06:21 | read at 06:58 |
| `P/🔨️modules/🧠️behavior/🟦️.ts` / `🦀️.rs` | 06:06 / 06:44 | read at 06:57 / 06:59 |
| `PR/🔨️modules/🫧️layer`, `📡️survey` | 05:43 / 05:42 | read at 07:01 |
| `QR/🔨️modules/🐾️pets/🟦️.tsx` | 05:22 | read at 07:02 |

Abbreviations as in `📓️design.md`: `P` = `🧰️framework/🛍️products/🐾️pets`, `PR` = `P/🎯️targets/⚛️react`, `QR` = `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`, `AP` = `🎓️teaching/🏛️architecture/🐾️pets`, `S` = `🎓️teaching/🏛️architecture/❓️quiz`. Rust twins are `🦀️.rs` next to the `🟦️.ts`.

Method: whole-file reads of every core module (TS and Rust), schema (JSON, TS, Rust), validation, the React target, the quiz module and the site/AP glue; scripted greps (forbidden math, `Date`, console, CSP patterns, comments, name stems, test layout) and a script that parsed every first docstring line for emoji start and per-file uniqueness. Nothing was built, run or edited.

## Verdict

**0 blockers · 4 should-fix · 24 nits.** The determinism core is clean: no forbidden call in any `.ts` or `.rs` under `P/🔨️modules` and `P/🧬️schema`, and the twin pairs I compared line by line (trigonometry, randomness, rig, animation, terrain, behaviour, validation, stage) evaluate the same expressions in the same order. The React target is CSP-safe and tears everything down. What remains is mostly rule hygiene (docstring emoji uniqueness), duplicated helpers, a port collision, loosely typed Rust twins of closed schema values, and design text that lags the code.

## Findings

| ID | Sev | Area | Where | Finding | Suggested fix |
|---|---|---|---|---|---|
| F1 | should-fix | launch / ports | `.claude/launch.json:590-592`, `.vscode/🧩️launch.seed.jsonc:1346,1354`, `PR/🏗️builder/🌐️vite/🟦️.ts:49`, `PR/📦️packages/🟦️typescript/📜️script.ts:24` | `pets-stories` takes port **6071**, which the seed already uses for `⚖️gate🗂️hub-document-sweep⚛️react` (`--serve http://127.0.0.1:6071/`, seed l. 4599 and 4610). Design said 6072 for the architecture gallery; the code uses 6074. | Move `pets-stories` to a free port (6072 and 6073 are unused in `.claude/launch.json`), change both defaults (`"6071"` in builder l. 49 and script l. 24 and the docstring l. 21), regenerate `.vscode/launch.json`, and update the design table. |
| F2 | should-fix | AGENTS: unique docstring emoji per file | `QR/🔨️modules/🐾️pets/🟦️.tsx:26,144,164`; `PR/🔨️modules/⏲️pacing/🟦️.ts:50`; `PR/🔨️modules/🖌️depiction/🟦️.ts:260`; `PR/🔨️modules/🫧️layer/🟦️.tsx:313`; `PR/📖️stories/🟦️.tsx:272,354,471`; `P/🧪️tests/📡️surface-survey/🟦️.tsx:118` | Ten docstrings repeat an emoji already used in the same file (`🎚️` l. 23/26, `📛️` l. 49/144, `🐾️` l. 1/164 in the quiz module; `⏲️` l. 1/50; `🏷️` l. 30/260; `🫧️` l. 1/313; stories `🎚️` 150/272, `🖼️` 221/354, `📖️` 1/471; test `🪟️` 59/118). All core `.ts`/`.rs` files, the schema, validation, AP and site files pass. (`QR/🎛️preferences/🟦️.tsx` and `QR/🟦️.tsx` also repeat, but those repeats pre-date the ticket.) | Pick a fresh emoji for each repeat. |
| F3 | should-fix | duplicated helpers (TS and Rust) | `P/🔨️modules/🧠️behavior/🟦️.ts:91-109` vs `🎲️randomness/🟦️.ts:90-108`; Rust `🧠️behavior/🦀️.rs:121-145` vs `🎲️randomness/🦀️.rs:93-117`; `TWO_POW_32` in `🎲️randomness/🟦️.ts:21`, `🎪️stage/🟦️.ts:45`, `🎲️randomness/🦀️.rs:21`, `🎪️stage/🦀️.rs:49`; stream ids `0xfffffffd` (`PR/🫧️layer/🟦️.tsx:44`), `0xfffffffe` (`🧠️behavior/🟦️.ts:278`), `0xffffffff` (`🎪️stage/🟦️.ts:44`) | `weightedIndex` is a line-for-line copy of the body of `randomPick` (four copies over both languages). The unit division is written twice (`randomUnit` vs stage `unitOf`) and Rust `random_unit` re-implements the first output word (`stir(pool_of(key)[0], INIT_B, INIT_B * MULT_B)`) instead of calling `random_words`. The three reserved randomness streams are scattered over three modules and the shell. | In `🎲️randomness` export `pickIndex(unit, weights)` and `unitOfWord(word)`; make `randomPick` = `pickIndex(randomUnit(key), weights)`, import both in behaviour and stage, and add one `STREAMS` constant block (stage, cast, rotation). Mirror in Rust. |
| F4 | should-fix | schema-first: closedness | JSON `🧬️schema/🔣️.json:583,658` (`facing` enum `[1,-1]`), `:674` (`rate` enum `[0,16,32,64]`), `:436` (`Ticked.ticks` integer ≥ 0) vs Rust `🧬️schema/🦀️.rs:603,671` (`facing: i8`), `:684` (`rate: u8`), `:496` (`ticks: Ticks = i64`) | TS twin is closed (`1 \| -1`, `0 \| 16 \| 32 \| 64`), the Rust twin accepts any `i8`/`u8`/negative `i64`. A Stage/Frame JSON with `facing: 5` or `rate: 7` decodes in Rust and is rejected by the schema. No validator in either language covers Stage or Frame documents, so nothing catches it. | Rust: `Facing` and `Rate` enums with explicit serde numbers (or newtype + `TryFrom`), `Ticks` ≥ 0 for events; or add `stageIssues`/`frameIssues` to the validator. |
| F5 | nit | design lags code | `📓️design.md` §5.7, §8.1, §11 vs code | (a) `Frame.rate` has a fourth value 16 (sleepers) in schema/TS/Rust/pacer/README but design §5.7 says 64/32/0. (b) `petScene(step, runs, catalog)` (`QR/🐾️pets:43`) has a third parameter the design lacks. (c) `QUIZ_PET_SURFACES = "#quiz-main [data-card]"` (l. 64) drops `[data-layered-card]` from design §8.1 (that marker only exists in ui-react `LayeredOverview`, which the quiz does not use — fine, but undocumented). (d) architecture gallery on 6074, not 6072. | Update the design (or README) so the contract matches. |
| F6 | nit | validation twin | `P/🔨️modules/✅️validation/🦀️.rs:28-49,514,521,533` | Rust judges typed documents (`&Species`, `&Menagerie`, `&Ensemble`), design §4.8 says `unknown`; the structural codes `required` and `property-unknown` can never be produced (`IssueCode::Required`, `PropertyUnknown` are dead variants; grep: no `report(..)` with them). Documented in the file header and covered by the conformance case (structural vectors project "rejected"), so this is a known, accepted asymmetry. | Drop the two dead variants (or say in the enum docstring they exist for wire parity only). |
| F7 | nit | JSON comments | `P/🔮️oracles/🔣️.json:3` (`_comment`), `P/🧫️fixtures/📡️surface-survey/🔣️.json:2` (`_comment`) | AGENTS: no comments inside definitions. `❓️quiz/🔮️oracles/🔣️.json` has the same key, so there is precedent, but the surface-survey fixture is the only fixture with one. | Move the text into the feature header or README; drop the keys. |
| F8 | nit | TS/Rust twin shape | `P/🔨️modules/🎪️stage/🟦️.ts:692,1019,1141` vs Rust `🎪️stage/🦀️.rs:232` | Rust factors `blink_at`, `facing_to`, `breath_of`, `behind`; the TS stage repeats the same expressions inline (blink formula 3×, `x >= y ? 1 : -1` ≥ 8×, breath-clip lookup in `poseOf` and `paceOf`). Twins drift when one is edited. | Add `blinkAt`, `facingTo`, `breathOf` to the TS stage. |
| F9 | nit | dead / test-only code | `P/🔨️modules/🦴️rig/🟦️.ts:24-33,111` and Rust `🦴️rig/🦀️.rs:25,110` | `compose` is never called by `solveRig` (it repeats the six expressions inline); `invert`/`IDENTITY` are used only by tests. `followersOf` (`🧠️behavior`) and `weightedIndex` serve tests and one internal call only. | Make `solveRig` call `compose` (or delete `compose` and say so), or keep as documented API and say "API" in the docstring. |
| F10 | nit | dead schema type | `P/🧬️schema/🟦️.ts:25`, `🦀️.rs:39`, `🔣️.json:30` | `Turns` is defined in all three and used nowhere. | Delete from the three twins (and any test that lists `$defs`), or use it (`sinTurns(turns: Turns)`). |
| F11 | nit | region labels | `🏞️terrain/🟦️.ts:22-24`, `🧠️behavior/🟦️.ts:18-22`, `🎪️stage/🟦️.ts:33-41` | `//#region 🔖️Adapters` wraps imports of **own** modules (not adapters); `🦴️rig`, `🎞️animation`, `✅️validation` have no region at all; React files use `🔌️Adapters`. | One convention: no region around plain imports. |
| F12 | nit | randomness doc/shape | `P/🔨️modules/🏞️terrain/🟦️.ts:155` vs Rust `🏞️terrain/🦀️.rs:224` | `hopOf` docstring says limits are "tested as *not within*", Rust docstring says "tested as *within*"; code is equivalent (NaN rejects in both). | Same sentence in both. |
| F13 | nit | validation float equality | `✅️validation/🟦️.ts:268-269`, Rust `🦀️.rs:371-372` | `loop-seam` tests `Math.floor(revolutions) !== revolutions` with `revolutions = (to - from) / 360`. Authored values with decimals (e.g. 17.3 → 377.3) can miss an integer by one ulp and be reported as a seam. Not reproduced; both twins would agree. | Compare with a tolerance (`abs(revolutions - round(revolutions)) < 1e-9`) in both. |
| F14 | nit | string from numbers in core | `✅️validation/🟦️.ts:39`, Rust `🦀️.rs:91` | Design §2.4 says the core never produces strings from numbers; validation builds JSON pointers from integer indices. Deterministic, but the sentence is not literally true. | Scope the sentence to stage/frame. |
| F15 | nit | `rest(leaving)` | `PR/🫧️layer/🟦️.tsx:245-248` | `rest` is registered as `watchVisibility(view, rest)`, so the parameter named `leaving` actually receives `hidden`. | Rename to `hidden`. |
| F16 | nit | stylesheet reliance | `PR/🎨️.css:5-11`, `PR/🫧️layer/🟦️.tsx:284` | `pointer-events: none` and `position: fixed` come only from `🎨️.css`; if the stylesheet fails to load, the unstyled layer holds in-flow 300×150 SVGs that can take clicks. | In `startShow` also set `host.style.pointerEvents = "none"` (CSSOM, CSP-safe). |
| F17 | nit | trusted palette | `PR/🖌️depiction/🟦️.ts:155-157` | `style.setProperty("--pet-body", species.palette.body)` trusts the menagerie; a non-validated `url(...)` value would trigger a request. The colour regexp lives in `✅️validation`, which nothing in the target calls. | Call `speciesIssues` at the quiz seam, or guard with the `#rrggbb` pattern in `depict`. |
| F18 | nit | quiz provider docs | `QR/🔨️modules/🐾️pets/🟦️.tsx:81,123-137` | `QuizPetsSource` is documented "called at most once per attempt"; React StrictMode runs the effect twice in development, so it is called twice (first result dropped). | "at most once per mounted attempt (twice under StrictMode)". |
| F19 | nit | console on failure | `QR/🔨️modules/🐾️pets/🟦️.tsx:152-162` | `PetBoundary` keeps the quiz alive but React 19 reports caught errors with `console.error`, so the "nothing in the console" claim is not true on that path. Layer faults inside `startShow` are swallowed silently (good). | Reword docs; optionally `onCaughtError` is the host's business. |
| F20 | nit | Rust serde dep | `P/📦️packages/🦀️rust/Cargo.toml:23-27` | `serde` is a non-optional runtime dependency of the crate (same as `quiz`; AGENTS says no external runtime libraries unless behind an interface). TOML comment l. 24-26 sits inside `[dependencies]`. | Keep, but state the exemption in the crate description or gate derives behind a `serde` feature; move the comment to the README. |
| F21 | nit | AP glue | `AP/🟦️.ts:36-37,40` | Double casts `as unknown as Ensemble` / `as unknown as readonly Species[]` hide JSON shape errors from the compiler; the default export is unused (the site imports the named export). | Keep the casts only if `S/🧪️tests/🐾️pet-cast` is the guard (it is), drop `export default`. |
| F22 | nit | content | `AP/🖥️servy/🔣️.json:4-5`; `AP/🌬️venty/🔣️.json` | Servy's `en` is "server rack" and `de` is "Rechenzentrum" (data centre): the two languages name different things. Venty is 36 px tall (design art direction: 40…56). | "Serverschrank" or change `en`; adjust venty `size.height` to ≥ 40. |
| F23 | nit | German word | `PR/📖️stories/🟦️.tsx:68,98,104`, `PR/📖️stories/🌐️.html:6` | Dev gallery says "Tiergeschichten", "Ein Tier", "Tiere"; quiz, AP and site say "Tierchen" consistently. | Use "Tierchen" in the gallery. |
| F24 | nit | deps | `PR/📦️packages/🟦️typescript/package.json:26` | `react-dom` is a runtime dependency but only the stories (`createRoot`) and the vite `dedupe` list use it; the layer imports `react` only. | Move to `devDependencies`. |
| F25 | nit | typecheck | `P/📦️packages/🟦️typescript/📋️project.json` | The core TS package has no `typecheck` target; core is typechecked only transitively through `pets-react`'s tsconfig, and the unit suites and Protocol v2 adapters (`🧪️tests/*/🟦️.ts`) not at all. | Add `tsconfig.json` + `typecheck` target like `pets-react`. |
| F26 | nit | stage sort comparator | `P/🔨️modules/🎪️stage/🟦️.ts:1290` | `(a, b) => y(a) - y(b) \|\| (species(a) < species(b) ? -1 : 1)` never returns 0, so it is not a valid comparator for equal elements. Species are unique per stage, so the result is total and matches Rust `behind`; the shape is still fragile. | `\|\| compareCodeUnits(a, b)` returning −1/0/1. |
| F27 | nit | oracle self-reference | `P/🧪️tests/🎪️stage-trace/🥒️.feature:2,10-11,34` | The committed stage traces are recorded from the TS subject (`record_stage_trace.ts`), so TS vs trace is self-referential; only TS vs Rust is independent. This is the recorded `pets-stage-trace` no-oracle decision and every building block has its own third-party oracle. | Accept; say so in `📓️report` as residual risk. |
| F28 | nit | doc drift | `PR/⏲️pacing/🟦️.ts:3-8,40-47` | Header talks about rate 64/32/0; rate 16 (every fourth frame) is handled generically but not mentioned. | One clause. |

## Details

### 1. Determinism of the core

**Greps (TS):** over `P/🔨️modules/*/🟦️.ts`, `P/🧬️schema/🟦️.ts`, `P/🟦️.ts` the only `Math.*` calls are `abs`, `floor`, `min`, `max`, `sqrt`, `imul`. No `sin/cos/tan/atan2/exp/pow/hypot/log/random/cbrt/round/trunc/sign`, no `**`, no `Date`, `performance`, `process`, `import.meta`, `globalThis`, `toLocale*`, `localeCompare`, no `for…in`, no `Object.values`. `Map`/`Set` appear only in validation (membership and a `Map` that is **sorted by a total comparator before it is returned**) and in the React shell. `Object.keys(SHAPES)` and `Object.entries(member)` iterate in insertion order of literals/parsed JSON, deterministic. The only sorts are `validation` (total comparator on path then code, code-point order = UTF-8 order = Rust's byte `cmp`) and `frameOf` (see F26). No number is turned into a string in the stage/frame core (F14 for pointers).

**Greps (Rust):** only `.abs()`, `.floor()`, `.sqrt()`, `.rem_euclid()` (integers), `Ticks::min` (integers). No `sin/cos/tan/atan2/exp/ln/powf/powi/mul_add/hypot`, no `f64::max/min`, no `HashMap/HashSet`, no `f32`, no `SystemTime`/`Instant`/`std::env`. JS `Math.max/min` semantics (NaN, ±0) are reproduced by `larger`/`smaller` (`🏞️terrain/🦀️.rs:85-104`) and used wherever the TS uses `Math.max/min` on floats: terrain, stage (`measure`, `hops_of`, `ground`, `arrive`, `poke`, `weight_of`, `frame_of`). The Rust unit suites additionally grep their own source for forbidden calls (`🧪️tests/🔬️unit/🦀️.rs` in animation, terrain, behaviour, stage).

**Twin comparison (line by line):**

| Pair | Result |
|---|---|
| 📐️ trigonometry | Identical constants (fdlibm literals), kernels, reduction and sign handling (`0 - x`, never `-x`). `clamp/lerp/smoothstep` identical. |
| 🎲️ randomness | Identical hash-mix (`stir`, `blend`, pool loops, key words beyond the pool). Constants match NumPy's `SeedSequence`. Only difference: `random_unit` re-implements word 0 (F3); `random_pick` returns `Option` vs `-1` (documented). |
| 🦴️ rig | Identical `compose/invert/transform/solveRig/lookOffset`, same operation order, `0 - sine` form. `compose` is not called (F9). |
| 🎞️ animation | Identical `easeBezier` (48 bisections, same polynomial order), `sampleTrack`, `clipTicks`, `sampleClip` (integer remainder/min), `blendPose`, `springStep`, `lidAt`. Rust `sample_clip` starts from `rest_pose` and matches the TS channel assignment per track order. |
| 🏞️ terrain | Identical, including `nearestPerch` (`Math.max(a, 0, b)` ≙ `larger(larger(a, 0), b)`), `hopOf` (same `2 * RATE * RATE` grouping, same early outs), `hopStep/hopLanding`. Docstring wording differs (F12). |
| 🧠️ behavior | Identical tables, weights, dwell, shares, rapport, needs. `castOf`: TS floors a float `capacity`/`epoch`, Rust takes `usize`/`i64` and uses `rem_euclid`; same result for integers. `MODE_LIMITS` is a record in TS and a struct with `Index<PetMode>` in Rust. TS looks activities up with `ACTIVITIES.indexOf` (O(11) per call in `needsAfter`, `moodOf`, `rapportAfter`, `dwellOf`); Rust casts the enum — a small avoidable cost on the hot path, nit. |
| 🎪️ stage | Same order of operations and draws in `act`, `decide`, `hops_of`, `approach`, `meet`, `pair`, `lull`, `pass`, `carry`, `arrive`, `tune`, `poke`, `pose_of`, `pace_of`, `frame_of`; `Math.min/max` ↔ `smaller/larger`; counters wrap via `>>> 0` ↔ `wrapping_add`; `now >>> 0` ↔ `now as u32`. `advance` takes `Stage` by value in Rust (documented), by reference in TS. No semantic divergence found. |
| ✅️ validation | All 13 design §3 rule codes are emitted at the same pointers by both. TS also emits `type-invalid`, `required`, `property-unknown`, `value-invalid`, `slug-invalid`, `length-invalid`, `items-too-few`; Rust cannot emit `required`/`property-unknown` (F6). `key-order` is also used for "fewer than two keys" as the design says. |

### 2. AGENTS.md style rules

- **Docstring emoji:** every first docstring line in `P` starts with an emoji and never `@emoji`; uniqueness fails only in the files of F2. No docstring is nested inside a function body outside test `describe` blocks.
- **Comments inside definitions:** none in any `.ts`, `.tsx`, `.rs`, `.py`. CSS has three comments between rules in `PR/🎨️.css` and two in `PR/📖️stories/🎨️.css`, none inside a rule. JSON/TOML: F7, F20.
- **Name stems** (`core|common|util|utils|helper|helpers|misc|shared|base|lib|impl`): no file or directory under `P`, `AP` or `QR/🔨️modules/🐾️pets` matches.
- **`[DEBUG]`, TODO, FIXME, `console.*`, `println!`, `dbg!`, `debugger`:** none in product code (the only hits are string lists in the Rust unit tests that police forbidden calls).
- **No `any`, `@ts-ignore`, `unwrap()`, `expect()`, `panic!` in product code.**
- **Dependencies:** `@semio-tech/pets` has devDependencies only (ajv, d3-ease, gl-matrix, polygon-clipping, typescript, vitest — each imported by a test). `@semio-tech/pets-react` depends on `@semio-tech/pets`, `react`, `react-dom` (F24). Crate: `serde` (F20), `serde_json` optional (`sut`) and dev. `pub use serde_json` is feature-gated and explicit. Public TS API exposes only own types plus React's `ReactElement` and DOM lib types; public Rust API exposes only own types.
- **Links in docstrings:** present and meaningful throughout (`@see` to netlib, numpy, W3C, MDN, WCAG, twins).

### 3. Layout rules

- `P/🟦️.ts`, `P/🦀️.rs`, `P/📦️packages/🟦️typescript/🟦️.ts`, `P/📦️packages/🦀️rust/🦀️.rs`, `PR/📦️packages/🟦️typescript/🟦️.tsx` are glue only; `PR/🟦️.tsx` is a barrel plus the stylesheet import.
- Rust `#[test]`: none outside `🧪️tests/🔬️unit/🦀️.rs`; modules attach them with `#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"]`. (`🧬️schema/🦀️.rs:692` uses `pub(crate) mod tests` — nit, `mod tests` suffices unless other tests import it.)
- TS `describe/it/test/vi.*`: only in `🧪️tests` files.
- Exactly three `📜️script.ts` (pets TS, pets Rust, pets-react), each a `ScriptRouter` over the shared library; each `📋️project.json` target is `bun ./📜️script.ts <command>`; root `package.json:228-232` calls `nx run`. No `.sh/.mjs/.js/.ps1/.bat` in `P`, `AP` or the quiz module. 12 `🐍️.py` oracle adapters are Protocol v2 case files (repo convention).
- Registrations present: root workspaces (`package.json:115-116`), `Cargo.toml:156,368`, launch entries (see F1), scripts `test:pets`, `test:pets:react`, `test:pets:rs`, `typecheck:pets:react`, `dev:pets:stories`.
- Test coverage of the §10 table: `🧪️tests/` holds every case; at 06:57 `🎪️stage-trace` had no Rust adapter, at 07:01 `🦀️.rs` appeared.

### 4. Schema-first consistency (JSON ↔ TS ↔ Rust)

57 `$defs`; 44 carry `x-semio-formats`; the 13 without (`Slug, Degrees, Turns, Ticks, Color, Paint, Channel, Gait, Activity, PetMode, Ease, Shape, StageEvent`) are scalars, enums, tuples and unions — the same set the quiz schema leaves without it.

Field-by-field comparison of all object defs (Bone … Frame): names, required vs optional, `additionalProperties: false` ↔ `deny_unknown_fields`, enum members and order, tuple lengths (`Ease` `[f64; 4]`, `Bond.between` `[Slug; 2]`), nullable-but-required members (`Actor.perch/partner/clip`, `Stage.pointer`, `Frame.wake` via `nullable`), tagged unions (`Shape`, `StageEvent` on `kind`), `loop` ↔ `looping`, `$schema` ↔ `json_schema` all agree. Differences:

1. **Looser Rust types** than closed JSON/TS values: `facing`, `rate`, `Ticked.ticks` (F4).
2. `seed`/`draws`: JSON integer (seed ≤ 4294967295, draws no maximum), TS number, Rust `u32`; a draw counter beyond 2³² wraps in both cores by design (`>>> 0` / `wrapping_add`) but the schema does not say so.
3. `Menagerie.schema`/`Ensemble.schema`: JSON `const`, TS literal type, Rust `String` (checked by validation `value-invalid`).
4. `Text` `minLength: 1`, `Slug` pattern, `Color` pattern, numeric ranges: only in JSON and the validators; neither TS nor Rust types carry them.
5. `Bond.between` has `uniqueItems`, `Rapport.between` does not (fine: rapports are never authored).
6. `Turns` unused (F10).
7. Rust-only: `Activity::as_str`, `Repertoire::clips`, `PAINTS/CHANNELS/GAITS/ACTIVITIES/PET_MODES` arrays (TS `as const` arrays), the TS-only runtime arrays are used by validation.

Validator codes vs design §3: all 13 present in both (see table in §1). Pointers match the README table by construction (shared vectors).

### 5. React target safety

- **CSP:** no `innerHTML`, `outerHTML`, `insertAdjacentHTML`, `<style>`, `setAttribute("style")`, `cssText`, `eval`, `new Function`, blob/data URLs or `fetch` in `PR/🔨️modules/**`, `PR/🟦️.tsx`, `PR/🎨️.css`. Elements are created with `createElementNS`; position/opacity via `style.transform`/`style.opacity`, palette via `style.setProperty` — all CSSOM, which `style-src` without `unsafe-inline` allows. The stories page has an inline module script and inline `style=` props; it is dev-only and never part of a release.
- **Teardown:** `startShow.stop()` stops the pacer (frame + timer), ends `watchSurvey` (disconnects `ResizeObserver` and `MutationObserver`, removes five capture listeners), `watchPointer` (three listeners) and `watchVisibility` (three listeners), removes every SVG and the inline `z-index`. `watchForcedColors` removes its `change` listener. All `removeEventListener` calls match the add-time capture flag.
- **React state:** the layer renders one static `<div aria-hidden="true" class="pet-layer">`; frames are painted outside React; props become events through `direct()`. No per-frame state update. `useSyncExternalStore` for forced colours.
- **StrictMode:** effect 1 (start show) and its cleanup (stop show, `show.current = null`) are symmetrical; a second start picks a fresh random seed, harmless. Effect 2 (`direct`) is idempotent.
- **A11y/pointer:** `aria-hidden`, no focusable node (`focusable="false"` on the SVGs), `pointer-events: none` inherited by the pets, listeners passive, no `preventDefault`/`stopPropagation`, `contain: strict`. `forced-colors: active` hides via CSS **and** the layer renders nothing and runs nothing; `print` hides. `prefers-reduced-motion` is the host's decision as designed (the quiz maps it to `still`). See F16 for the stylesheet dependency.
- **Pause/hide:** hidden document, `[inert]`/`[hidden]` ancestor and `pagehide` cancel every frame and timer; return restarts the clock without catch-up (design §6.4).
- **Fault policy:** a throw inside `step` calls `stop()` and returns a quiet pace (silent, as designed).

### 6. Quiz module

- **Lazy:** the only runtime reference to `@semio-tech/pets-react` is `import("@semio-tech/pets-react")` (`QR/🐾️pets/🟦️.tsx:88`); `@semio-tech/pets` and `PetLayerProps` are `import type`. `QR/🟦️.tsx:48` and `🎛️preferences/🟦️.tsx:16` import the glue module (types, constants, `usePetCast`) only. The site loads the menagerie through `import("../🐾️pets/🟦️.ts")` (`S/🟦️.ts:29`). Entry graph carries no pets runtime. Both packages are declared as workspace dependencies of `quiz-react`, which is fine.
- **Silent failure / cancellation:** `Promise.resolve().then(() => Promise.all([source(), stage()])).then(ok, () => undefined)`; the `dropped` flag set by the effect cleanup discards a result that outlives the provider or a switched-off choice; a failed load is forgotten and retried when the learner toggles. A dynamic import cannot be aborted, the flag is the right tool.
- **i18n:** `quiz.preferences.pets`, `petsOff`, `petsStill`, `petsCalm`, `petsLively`, `petsCast` exist in the English and German bundles (`QR/🌐️i18n/🟦️.ts:104-109` and `446-451`); the German summary line (l. 443) was updated; German is informal ("Hier zu Hause: {{names}}") and uses **"Tierchen"** in both places, matching AP (`AP/🔣️.json:5`) and the site test. The gallery and one species disagree (F22, F23).
- **Wiring:** `data-pets` on `.quiz-app` (`QR/🟦️.tsx:459`), `<QuizPets />` after `<PresenceOverlay>` (l. 483), provider wraps `Client` only once a locale is known (l. 530). Quiet is `step.screen === "run"`. Peer glances use the stable function `peerGlances`; surfaces/keep-outs are module constants and the combined keep-out string is memoised in `loaded`, so `direct()` does not churn.

### 7. Other

- F8, F9, F10, F11 (duplication, dead code, naming between twins), F3 (shared constants).
- `Math.random()` appears only in the React shell (`PR/🫧️layer/🟦️.tsx:357`, default seed) — allowed.
- Rust uses direct indexing in a few places (`launches[…]`, `clips[…]`, `ACTIVITIES[pick]`); each is guarded by the weights/eligibility logic that makes the collection non-empty (hop weight is 0 unless `launches` is non-empty). A debug assertion would document it.
- `ModeLimits` (Rust) has no TS name; design §4.6 says `Record<PetMode, Limits>`. Fine, but name it in the design.
- No TODO/FIXME/`[DEBUG]`/stray scripts/generated output found inside `P`, `AP`, or the quiz module (the Vite dev caches under `node_modules/.vite/stories-6071|6072|6074` are untracked tool output; `stories-6072` is stale since the port moved).

## What was checked and is fine

- No forbidden math, time, environment, hash-order or locale dependence in any core file; Rust never uses `f64::max/min`.
- Twin pairs structurally aligned; constants byte-identical.
- No comments inside definitions in code; no banned stems; no `[DEBUG]`; nothing in product code writes to the console.
- No external runtime dependency beyond React for the React target and serde for the Rust crate.
- Rust tests live only in `🧪️tests/🔬️unit/🦀️.rs`; TS registrars only in test files; script chain intact; launch entries and root scripts present.
- React target: CSP-safe, torn down, StrictMode-safe, decorative, pointer-transparent.
- Quiz module: lazy, silent, cancellable, bilingual, one German word.
