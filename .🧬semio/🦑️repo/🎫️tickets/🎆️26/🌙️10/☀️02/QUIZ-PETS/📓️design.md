# 📓️ Quiz Pets — normative design

Ticket `2026/10/02/QUIZ-PETS`, goal `🎯runningframework🎯runningproducts`. This document is the contract every agent of the ticket works against. Evidence lives in the `📓️explore-*.md` and `📓️research-pet-animation.md` reports beside it; read the report named in your brief before touching the area it maps.

Path abbreviations:

| Short | Path |
|---|---|
| `P` | `🧰️framework/🛍️products/🐾️pets` (new, domain-neutral pets product) |
| `PR` | `P/🎯️targets/⚛️react` (render target `@semio-tech/pets-react`) |
| `Q` | `🧰️framework/🛍️products/❓️quiz` |
| `QR` | `Q/🎯️targets/⚛️react` (`@semio-tech/quiz-react`) |
| `S` | `🎓️teaching/🏛️architecture/❓️quiz` (the site) |
| `AP` | `🎓️teaching/🏛️architecture/🐾️pets` (new, the architecture menagerie) |
| `TK` | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS` (this ticket) |
| `TAX` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` |
| `TEST` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` (Protocol v2 harness) |

Every emoji in a path is the emoji followed by U+FE0F. Never retype one from memory: copy it from this document or an existing path.

---

## 1. Requirements and how the design answers them

| Owner's words | Answer |
|---|---|
| "Add pets to the ui" (quizzes) | A decorative, pointer-transparent pet layer mounted once in the quiz app beside the presence overlay (§8). |
| "Pets have a skeleton and are animated" | Every species is a rig: bones (parents first), parts rigidly attached to bones, a face, keyframed clips; forward kinematics yields a 2×3 matrix per bone per frame (§3, §4.3, §4.4). |
| "By default they are slightly active" | Preference `pets` defaults to `calm`: mostly idle with breathing, blinking, a fidget now and then, a short walk now and then (§5.6). |
| "When standing still they follow the cursor (with the eyes etc)" | Gaze layer: pupils spring towards the pointer, then a partner, then other people's cursors (§5.3). |
| "small motions like blinking" | Blink scheduler, idle clip, fidget clips (§5.3, §5.4). |
| "they walk on top of ui elements" | The shell surveys the top edges of cards and the floor into surfaces; the core cuts them into perches and walks, hops, floats and falls on them (§4.5, §6.3). |
| "they interact with each other (sometimes they like each other, sometimes they have small disputes)" | Bonds (authored affinity) + rapport (drift from shared history) drive encounters: greet, cuddle, squabble → sulk (§5.5). |
| "The pets are always fitting to the topics of the quizzes" | Casts per scene (`home` and one per quiz id); every species lists the quiz items that ground it and a site test fails when a cast pet has no ground in its quiz (§9). |
| sunny, cloudy, housy, solary, radiatory, pumpy, windowy, waly, battery, "etc" | The architecture menagerie ships these nine plus eleven derived from the quiz items: windy, boily, roofy, insuly, shady, venty, chilly, kettly, flamy, thermy, servy (§9, `📓️explore-topic-pets.md`). |

AGENTS.md rules that shape the design: domain-neutral framework + domain-specific extension (product `P`, menagerie `AP`); schema-first, TypeScript + Rust twins; one language-agnostic test per feature with a third-party oracle; event-driven (`advance(stage, events)`); state lanes (§2.3); accessible and customizable (§6.5, §8.2); both languages, no default; no runtime dependency; zero-touch scripts through `📜️script.ts`; launch entries.

## 2. Decisions

### 2.1 Placement

- **`P` is a new framework product** with a render-independent model (schema, TS core, Rust core) and a React target, exactly the shape of `Q`. Reason: the model is multi-implementation by rule, and no module outside products ships TS + Rust + a React target together. `P` knows nothing about quizzes.
- **`QR/🔨️modules/🐾️pets`** is the quiz's glue: preference, scene selection, lazy loading, mounting.
- **`AP`** is the domain extension: twenty species, their bonds and casts. The site passes it to `mountQuiz` like `logo` and `legal`.
- **The quiz schema, cores, fixtures, the server and the proctor are not touched.** Pets never travel over the wire and never enter a quiz document (closed schemas, revisions; see `📓️explore-quiz-site.md` §0).

### 2.2 Data flow

```
AP/<species>/🔣️.json ─┐
AP/🔣️.json (ensemble) ─┴─ AP/🟦️.ts → Menagerie ── S/🟦️.ts: mountQuiz(root, { …, pets: () => import("../🐾️pets/🟦️.ts") })
                                                        │ lazy chunk (menagerie + @semio-tech/pets-react + @semio-tech/pets)
QR/🔨️modules/🐾️pets: QuizPets(step, catalog, preferences) ── <PetLayer menagerie scene mode quiet …/>
PR shell: DOM survey, pointer, visibility, clock ──events──▶ P core: advance(menagerie, stage, events) ──▶ frameOf ──▶ PR depiction (inline SVG)
```

The release CSP forbids runtime `fetch` of site files; everything is statically imported and reaches the browser as a same-origin lazy chunk (dynamic `import()`), so the entry script budget (260 kB gzip) is not charged with pets and a learner with pets off never downloads them.

### 2.3 State lanes

| Lane | What |
|---|---|
| persisted local-only | the `pets` preference (`off`, `still`, `calm`, `lively`) in the quiz preferences slice |
| ephemeral local-only | the `Stage` (actors, perches, rapport); every device simulates its own pets |
| ephemeral shared (read only) | other learners' cursors, already on the client through presence, are fed to the stage as `glanced` points |
| persisted shared | none |

### 2.4 Determinism

The core is a pure fold: `advance(menagerie, stage, events) → stage`, `frameOf(menagerie, stage) → frame`. Same seed + same events = same frames, bit for bit, in TypeScript and Rust:

- Arithmetic is limited to `+ − × ÷`, `sqrt`, `abs`, `floor`, `min`, `max`, comparisons, and 32-bit integer operations (`Math.imul`, shifts, xor / `wrapping_mul`). **Forbidden in `P/🔨️modules` and `P/🧬️schema`:** `Math.sin`, `cos`, `tan`, `atan2`, `exp`, `pow`, `hypot`, `log`, `random`, `Date`, `performance`; Rust `f64::sin`, `cos`, `powf`, `exp`, `mul_add`, `hypot`. Sine and cosine come from `📐️trigonometry`.
- Twins evaluate the same expressions in the same order; no reassociation.
- Time is integer ticks, `TICKS_PER_SECOND = 64`. Seconds appear only in authored documents (`Clip.seconds`, `Locomotion.speed`); the core converts once per use with the same expression in both languages.
- Randomness is counter-based (`🎲️randomness`): a draw is a pure function of a key `[seed, stream, counter]`, so the order in which actors are processed never changes what they draw. Actor streams: `stream` = index of the species in `menagerie.species`, `counter` = `actor.draws` (incremented per decision). Reserved streams, named in `🎲️randomness`: `STAGE_STREAM = 0xffffffff` (`counter = stage.draws`), `CAST_STREAM = 0xfffffffe` (`castOf`), `ROTATION_STREAM = 0xfffffffd` (the layer's rotation spans).
- The stage and its frames never produce strings from numbers and never read the environment; render targets format numbers. (`✅️validation` builds JSON pointers from integer indices — deterministic, and outside the fold.)

## 3. Schema

`P/🧬️schema/🟦️.ts` is written and is the reference for field names; `P/🧬️schema/🔣️.json` (draft-07, `$id` `https://json.schemas.assets.semio-tech.com/framework/product/pets/schema.json`, one `$defs` entry per exported type with the same name, objects closed with `additionalProperties: false`, `"x-semio-formats": ["🔣️jsonschema","🦀️rust","🟦️typescript"]`) is normative once it exists, and `P/🧬️schema/🦀️.rs` restates it with serde (`deny_unknown_fields`, tagged enums on `kind`). Follow `Q/🧬️schema` in every convention (`📓️explore-quiz-core.md` §2).

Document rules beyond structure (checked by `✅️validation`, issue = `{ path, code }`, codes in kebab-case):

| Code | Rule |
|---|---|
| `duplicate-id` | ids are unique within species, bones, parts, eyes, clips of one species |
| `unknown-reference` | `Bone.parent`, `Part.bone`, `Eye.bone`, `Mouth.bone`, `Face.above`, `Track.bone`, repertoire clip ids, bond and cast species ids all resolve |
| `bone-order` | the first bone has no parent, every other bone has one, and it is listed earlier |
| `key-order` | a track has ≥ 2 keys, `at` strictly ascending, first `0`, last `1` |
| `loop-seam` | in a looping clip the first and last key of a track carry the same value; on the `rotation` channel the same angle modulo 360° (so `0 → 360` is a seamless full spin for fans, rotors and rays) |
| `ease-range` | `Ease` x-coordinates lie in [0, 1] |
| `out-of-range` | temperament, affinity (−1…1), sizes, speeds, seconds (> 0), radii (`pupil < radius`), colours (`#rrggbb`) |
| `self-bond`, `duplicate-bond` | a bond joins two different species, each unordered pair at most once |
| `missing-gait-clip` | a `walk` gait has a `walk` clip, a `hop` gait a `hop` clip |
| `float-hover` | `hover` is present exactly when the gait is `float` |
| `empty-cast`, `duplicate-scene` | a cast has at least one core species; scenes are unique |

Authoring conventions (normative for `AP`, checked where a code exists):

- Units are CSS pixels at scale 1. The feet stand on the origin; the body rises towards negative y; `size` is the rest bounding box (`width` centred on x = 0). Species face **right** at rest; the stage mirrors them.
- Track values: `x`, `y` are offsets in pixels, `rotation` an offset in degrees, `scaleX`/`scaleY` factors (rest = 1). Scaling a bone scales its children.
- Clip ids are free; the repertoire maps activities to clips.

## 4. Core modules (`P/🔨️modules`), TypeScript names (Rust: the same in snake_case)

Each module is `🟦️.ts` + `🦀️.rs` side by side, Rust unit tests in `🧪️tests/🔬️unit/🦀️.rs` (attached with `#[cfg(test)] #[path]`), TypeScript unit suites in `🧪️tests/🔬️unit/🟦️.ts` (picked up by a glob in the vitest config). Cross-module names below are fixed; everything else is the module owner's choice. Constants are starting values to be tuned by eye in the stories gallery; when tuned, the fixture generators are rerun.

### 4.1 `📐️trigonometry`

`sinTurns(turns)`, `cosTurns(turns)`: reduce to the first octant with exact operations (`t − floor(t)`, reflections), evaluate fixed odd/even polynomials in `r·2π` (Horner, coefficients as literals); absolute error ≤ 1e-12. `clamp(value, low, high)`, `lerp(from, to, amount)`, `smoothstep(amount)`.

### 4.2 `🎲️randomness`

`randomWords(key: readonly number[], count): number[]` returns the unsigned 32-bit words `numpy.random.SeedSequence(key).generate_state(count)` returns for the same key of unsigned 32-bit integers (pool size 4, the published hash-mix; 32-bit operations only). `unitOf(word)` = word ÷ 2³² (the one place a word becomes a unit), `randomUnit(key)` = `unitOf` of the first word, `randomBetween(key, low, high)`, `weightedIndex(weights, unit)` = index by cumulative weights (−1 when no weight is positive; the one weighted pick), `randomPick(key, weights)` = `weightedIndex(weights, randomUnit(key))`. The reserved streams `STAGE_STREAM`, `CAST_STREAM`, `ROTATION_STREAM` (§2.4) are exported here.

### 4.3 `🦴️rig`

- `type Affine = readonly [a, b, c, d, e, f]` in the SVG `matrix(a b c d e f)` convention; `IDENTITY`, `compose(parent, local)` (= parent × local), `invert(matrix)`, `transform(matrix, x, y): Point`.
- `type BonePose = { x, y, rotation: Degrees, scaleX, scaleY }`, `type Pose = readonly BonePose[]` (rig order), `restPose(species)` (all zero offsets, unit scale).
- `solveRig(species, pose): number[]`: six numbers per bone in rig order, each bone's world matrix relative to the feet origin; local = translate(bone.x + pose.x, bone.y + pose.y) × rotate((bone.rotation + pose.rotation) ÷ 360 turns) × scale(pose.scaleX, pose.scaleY).
- `lookOffset(eye: Point, target: Point, reach: number): Point`: direction from eye to target scaled to length `d ÷ (d + reach)`, so the result lies inside the unit disc; `(0, 0)` when the points coincide.
- `pupilReach(eye): number`: how far a pupil travels from the centre of its white, `radius − pupil − 0.25`, never below 0 (the one rule `frameOf`, the depiction and `TK/render_species_preview.mjs` share).

### 4.4 `🎞️animation`

- `easeBezier(ease, amount)`: CSS cubic-bezier, x solved by 48 bisection steps.
- `sampleTrack(track, phase)`, `clipTicks(clip)` = `floor(clip.seconds × 64 + 0.5)` (at least 1), `sampleClip(species, clip, ticks): Pose` (ticks since the clip began; looping clips wrap, others hold their last key), `blendPose(from, to, amount)`.
- `springStep(position, velocity, target, stiffness, damping): { position, velocity }`: one tick of semi-implicit Euler with `dt = 1/64`.
- `BLINK_TICKS = 12`, `lidAt(ticks)`: closure of a blink that began `ticks` ago, 0 → 1 → 0, 0 outside.

### 4.5 `🏞️terrain`

- `perchesOf(surfaces, keepouts, width, height, clearance, minimum): Perch[]`: for each surface with `clearance ≤ y ≤ height`, clip `[x0, x1]` to `[0, width]`, subtract the x-extent of every keep-out that intersects the band `[y − clearance, y)` over that span, keep stretches at least `minimum` wide; order: by surface order, then x.
- `perchAt(perches, surface, x): Perch | null`, `nearestPerch(perches, x, y): Perch | null`.
- `GRAVITY`, `fallStep(y, vy): { y, vy }`, `landingOf(perches, x, fromY, toY): Perch | null` (highest perch crossed between the two heights at `x`).
- `hopOf(from: Point, to: Point): { vx, vy, ticks } | null`: ballistic arc with an apex above both ends, `null` when out of reach (too high, too far, too long, or landing faster than `FALL_SPEED`). `hopStep(x, y, vx, vy, to, ticks)` advances a flight by one tick (the last tick lands exactly on `to`), `hopLanding(perches, from, to, hop)` is the perch a flight really ends on.
- `strideTo(x, goal, speed): number`: the next x one tick later at `speed` px/s without overshooting.

### 4.6 `🧠️behavior`

- `MODE_LIMITS: Record<PetMode, Limits>` (how many actors may move at once, dwell ranges, whether and how often encounters happen).
- `affinityOf(menagerie, rapports, a, b): number` (authored bond + drift, clamped to [−0.6, 1] — `AFFINITY_FLOOR`, §5.5; 0 when unlisted).
- `castOf(cast, capacity, epoch, seed): Slug[]`: the core first, then its visitors. Whenever a rotation exists and `capacity ≥ 2`, one seat belongs to a visitor from the rotation; the core gets the others — all of it while it fits, otherwise it takes turns; seats the core leaves free go to further visitors. Whoever takes turns is read off a ring from a start the seed picks (`randomWords([seed, CAST_STREAM, 0], 2)`: word 0 the core, word 1 the rotation), moved on by one per `epoch`.
- `activityWeights(actor, species, situation): number[]` in `ACTIVITIES` order (0 = not eligible) and `dwellOf(activity, mode, unit): Ticks`.
- `encounterOf(affinity, unit): "greet" | "cuddle" | "squabble"`.
- `needsAfter(needs, activity, ticks, temperament): Needs`, `rapportAfter(drift, encounter): number`.

### 4.7 `🎪️stage`

- `openStage(seed): Stage`: empty, mode `calm`, not quiet.
- `advance(menagerie, stage, events): Stage`: folds the events in order (§5).
- `frameOf(menagerie, stage): Frame`: actors sorted by `y` then species id; per actor its `activity`, the pose (the idle loop runs underneath on the stage clock and the activity's clip fades in and out over 8 ticks; outside `still` and quiet the bone of the first eye leans after the gaze), `solveRig` (squeezed through a line while the actor turns round, up to `Actor.faced`), eyes (pupil offset = gaze × `pupilReach(eye)`, lid from the blink), mood.

### 4.8 `✅️validation`

`menagerieIssues(document: unknown): Issue[]`, `speciesIssues(document: unknown): Issue[]`, `ensembleIssues(document: unknown): Issue[]`, `assembleMenagerie(ensemble, species: readonly Species[]): Menagerie` (species in ensemble order). Structure is checked by owned code (no schema library at runtime), then the rules of §3. (The Rust twins judge documents already decoded into their typed twins; decoding refuses what the structure forbids, so `required` and `property-unknown` never come out of the Rust validator.)

## 5. Simulation rules (`🎪️stage` with `🧠️behavior`, `🏞️terrain`, `🎞️animation`, `🦴️rig`)

### 5.1 Events

| Event | Effect |
|---|---|
| `surveyed` | store size, surfaces, keep-outs; recompute perches (`clearance` = tallest species on stage or wanted + its hover, `minimum` = the widest). Every grounded actor rides its surface (keeps its offset from the surface's `x0`, held inside a perch of that surface, then seated 8 px from its neighbours; whoever does not fit is crowded out and arrives anew) or, when its surface is gone, starts to `fall` — unless the same survey brought new ground: then it is gone with its surface and arrives anew, and whoever shares a perch moves to one that holds nobody (§13.4). |
| `summoned` | store `wanted`. Actors not wanted start `leaving` (walk to the nearest perch end when it is within 3 body widths and the way is clear, else fade where they stand; removed at opacity 0). Wanted species not on stage appear (opacity 0 → 1) on the least crowded perch — the ground last —, at a place chosen by a stage draw 8 px from the others; when there is no such place they wait and are tried again every whole second. |
| `tuned` | store the mode; `still` freezes every actor in `idle` with rest pose, open eyes, centred gaze. |
| `hushed` | store `quiet`; quiet actors finish what they do, then only `idle` and `sleep`; their pupils still follow the pointer, but they neither turn to it, lean after it nor greet it. |
| `pointed` / `unpointed` | store the pointer and the tick. |
| `glanced` | store the points. |
| `poked` | the nearest actor within 1.2 × its height, if grounded, greets (facing the poke) and gains mood; ignored while `still` or quiet. |
| `ticked` | advance tick by tick (§5.2…§5.6). When no actor is on stage or the mode is `still`, jump (`tick += ticks`) without per-tick work. |

### 5.2 Per tick, per actor (in `menagerie.species` order)

1. **Motion by activity:** `walk` → `strideTo` towards `goal` (float gaits glide the same way at hover height; hop gaits advance in clip-length hops); `hop` (between perches) → ballistic step, landing by `landingOf`; `fall` → `fallStep` + `landingOf`, landing on the floor perch at the latest, then `land`; an actor that leaves the stage box is put back on the nearest perch (fade).
2. **Gaze:** target = pointer when it moved within the last 4 s and the actor is not walking or asleep; else the partner; else the nearest glance point; else straight ahead. `lookOffset(…, 32)` → spring towards it (`springStep` per axis). **The pointer beyond the eyes** (§13.4): an actor that stands idle by itself turns round to a pointer clearly on its other side, leans after its gaze, and greets a pointer that came to rest beside it when it is curious enough. The normative order of a tick is the header of `🎪️stage/🟦️.ts`.
3. **Blink:** when `tick ≥ actor.blink` a blink runs for `BLINK_TICKS`, then the next one is drawn 2…6 s ahead (one draw; one in six is a double blink 0.3 s later). Asleep: lids shut.
4. **Mood** eases towards the activity's mood (cuddle 1, greet 0.7, idle 0.3, squabble −0.8, sulk −0.6, sleep 0.1).
5. **Needs** drift (`needsAfter`).
6. **Decision** when `tick ≥ until`: draw the next activity from `activityWeights` (one `randomPick`), its dwell (`dwellOf`), its clip (uniform among the repertoire's clips for it), and for `walk` a goal on its perch or, for a hop, a reachable neighbouring perch (`hopOf`).

### 5.3 Continuous layers

Gaze, blink and the idle clip run under every grounded activity, so a pet that "stands still" is never frozen (except in `still` mode).

### 5.4 Activities

| Activity | Ends | Notes |
|---|---|---|
| `idle` | after its dwell | idle clip loops |
| `fidget` | after its clip | a signature clip of the species |
| `walk` | at its goal | walk/hop/float clip loops; faces its goal |
| `hop` | on landing | to a neighbouring perch |
| `fall` | on landing | when its perch vanished |
| `land` | after its clip or 0.3 s | squash |
| `sleep` | after its dwell, on a poke, or when the pointer comes within 1.5 × its height | lids shut |
| `greet`, `cuddle`, `squabble` | after 2…5 s | both partners, facing each other |
| `sulk` | after 3…6 s | after a squabble, facing away; ends with the rapport mending by a little |

### 5.5 Encounters

At most one encounter on stage at a time, and none while quiet or `still`. Every second the stage (one stage draw) may pair two grounded, idle actors on the same perch or on perches less than 3 widths apart; the chance is the mode's rate × the mean sociability of the two. `encounterOf(affinity, unit)`: affinity ≥ 0.4 mostly `cuddle`, ≤ −0.3 mostly `squabble`, otherwise `greet`. Both walk towards each other until a body width apart, then act for the same span; a squabble ends in `sulk` for both. Rapport: cuddle +0.1, greet +0.05, squabble −0.15, the end of a sulk +0.1; it drifts back to 0 over about ten minutes; the effective affinity never falls below −0.6, so disputes stay small.

### 5.6 Modes (starting values)

| | `still` | `calm` (default) | `lively` |
|---|---|---|---|
| actors moving at once | 0 | 1 | 2 |
| idle dwell | – | 6…20 s | 3…10 s |
| walk weight | 0 | low | high |
| fidget weight | 0 | medium | high |
| encounters | none | ≥ 90 s apart | ≥ 30 s apart |
| blink, gaze, idle clip | off | on | on |

Quiet (a run is in progress): no walks, hops, fidgets or encounters in any mode; actors idle, and sleep when their energy is low.

### 5.7 Frame rate and waking

`frameOf` reports `rate`: 64 while any actor walks, hops, falls, lands, turns round, appears, leaves, turns see-through or plays a non-looping clip; 32 while only gaze springs, blinks or looping clips move; 16 while every actor only sleeps; 0 when nothing moves (`still` mode, an empty stage, or resting actors without an idle loop), with `wake` = the tick of the next scheduled change (`null` when none). The shell sleeps accordingly (§6.4).

## 6. React target (`PR`, package `@semio-tech/pets-react`)

Layout: `PR/🟦️.tsx` (barrel), `PR/🎨️.css`, `PR/🔨️modules/{🖌️depiction, 📡️survey, ⏲️pacing, 🫧️layer}/🟦️.ts(x)`, `PR/📖️stories/…` (§7), `PR/🏗️builder/🌐️vite/🟦️.ts`, `PR/🧪️tests/🎚️config/🟦️.ts`, `PR/📦️packages/🟦️typescript/{package.json, tsconfig.json, 📋️project.json, 📜️script.ts, 🟦️.tsx}` (glue only). React suites live in `P/🧪️tests/<case>/🟦️.tsx` (glob in the vitest config). No dependency on `@semio-tech/ui-react`; React and `@semio-tech/pets` only.

### 6.1 Public API

```tsx
export interface PetLayerProps {
  readonly menagerie: Menagerie;
  readonly scene: string;                       // the cast on stage; an unknown scene falls back to "home", then to nothing
  readonly mode: PetMode;                       // "off" is expressed by not rendering the layer
  readonly quiet?: boolean;                     // a time of concentration
  readonly capacity?: number;                   // most actors at once; default by viewport width: < 768 → 2, < 1024 → 4, else 6
  readonly surfaces?: string;                   // selector of elements whose top edge carries pets; default "[data-pet-surface]"
  readonly keepouts?: string;                   // selector of what pets must not cover; default: interactive controls, text blocks, "[data-pet-keepout]"
  readonly glances?: () => readonly Point[];    // sampled with every survey
  readonly scale?: number;                      // default 1; 0.8 below 768 px
  readonly seed?: number;                       // default: a random seed per mount
  readonly zIndex?: number;                     // default 35
  readonly onCast?: (species: readonly Slug[]) => void; // who is on stage, in menagerie order, whenever that changes
  readonly tempo?: number;                      // default 1; how fast the pets' time passes, held between 1/8 and 8
}
export function PetLayer(props: PetLayerProps): ReactElement;
```

Also exported: `depict(species)` / `paint(depiction, actorFrame)` (the SVG renderer), `survey(root, options)`, `createPacer(…)`, and the core re-exports the stories need. Reduced motion is the host's decision (the quiz passes `still`); the layer itself additionally shows nobody under `forced-colors: active` and for a menagerie with validation issues.

### 6.2 Depiction (the drawing rules; `TK/render_species_preview.mjs` already follows them)

- One `<svg class="pet" data-pet="<species id>" data-pet-activity="<activity>" focusable="false" overflow="visible">` per actor, absolutely positioned at the layer's origin and moved by `style.transform = translate(x px, y px) scale(±scale, scale)` (CSSOM property writes only); `style.opacity` likewise; palette through `style.setProperty("--pet-body" | "--pet-accent" | "--pet-detail", colour)`.
- One `<g>` per part with `transform="matrix(…)"` (the bone's matrix, numbers rounded to 3 decimals), containing the shape element with classes `pet-fill-<paint>` and `pet-stroke-<paint>`; `stroke-width` attribute (default 2, none when the stroke is `none`); `stroke-linejoin` and `stroke-linecap` round (CSS).
- Eye: `<g transform=matrix(bone)><g transform="translate(x y) scale(1 k)">` with `k = 1 − 0.9 × lid`, a white `<circle r=radius>` (`pet-fill-paper pet-stroke-ink`, width 1.5) and a pupil `<circle r=pupil cx cy>` (`pet-pupil`).
- Mouth: `<path d="M x−w/2 y Q x y+w·0.5·mood x+w/2 y">` (`pet-stroke-ink`, width 1.5, no fill).
- The face is inserted after the part named by `face.above`, else last.
- Only attribute and CSSOM writes per frame, and only when the rounded value changed. No `innerHTML`, no `<style>`, no `setAttribute("style")`, no ids.
- `PR/🎨️.css` (plain CSS, no Tailwind utilities): `.pet-layer { position: fixed; inset: 0; overflow: hidden; pointer-events: none; contain: strict }`, `.pet { position: absolute; left: 0; top: 0; will-change: transform }`, paint classes (`fill: var(--pet-body)` …; ink = `var(--foreground, #001117)`, paper = `#f7f3e3`, pupil = `#001117`), `@media (forced-colors: active) { .pet-layer { display: none } }`, `@media print { .pet-layer { display: none } }`. No `animation-name:` declarations (a quiz test counts them in `QR/🎨️.css`; this file is separate, but keep the rule anyway).

### 6.3 Survey (DOM → `surveyed`)

- Surfaces: every element matching `surfaces` that is not inside `[inert]` or `[hidden]`, whose box intersects the stage, is not clipped away by a scrolling ancestor and is at least 48 px wide → `{ id, x0: left, x1: right, y: top }` with a stable id per element (a `WeakMap` counter). Plus the floor: `{ id: "floor", x0: 0, x1: width, y: height }`.
- Keep-outs, each cut down to what its clipping ancestors show: boxes of elements matching `keepouts` (not `[inert]`; never a surface or an ancestor of one), inflated by 4 px, where the margin never reaches over an edge the element lies under; every surface's own box, grown sideways only (pets stand on a thing, never in front of another one); plus the focused element inflated by 8 px (WCAG 2.4.11). The default selector is `PET_CONTROLS`, `PET_TEXTS` (text blocks, `code`, `output`, `progress`, `meter`, `[role="alert"]`, `[role="status"]`) and `[data-pet-keepout]`.
- Dirty on: `ResizeObserver` (root), `MutationObserver` (subtree: childList, attributes `class`, `hidden`, `inert`, `open`), capture-phase passive `scroll`, `resize`, `focusin`, `transitionend`; measured in one batch per animation frame, reads only; while actors move, re-measure at most every 8 ticks, otherwise only when dirty.
- Pointer: passive `pointermove`/`pointerdown`/`pointerleave` on `window` → `pointed`/`unpointed`; `pointerdown` whose target has no interactive ancestor and lies within an actor's box → `poked`. Never `preventDefault`, never `stopPropagation`.

### 6.4 Pacing

A fixed-step accumulator turns `requestAnimationFrame` timestamps into `ticked` events (at most 8 ticks per frame, the rest is dropped). The pacer follows `frame.rate`: 64 → every frame, 32 → every other frame, 16 → every fourth frame, 0 → no animation frame at all, one timer for `frame.wake` (every tick slept is then delivered, which the stage jumps over), and it wakes on any event. Hidden document (`visibilitychange`, `pagehide`): everything is cancelled; on return the clock restarts without catching up. Test seams: `now`, `requestFrame`, `cancelFrame`, `setTimer`, `clearTimer`.

### 6.5 Accessibility

The layer is `aria-hidden="true"`, has no focusable or interactive node, never takes pointer events, never changes layout or scroll, makes no sound and no network request, writes nothing to the console. Pause, stop, hide (WCAG 2.2.2): the host's preference; `still` shows the pets motionless, `off` removes them.

## 7. Stories gallery (`PR/📖️stories`)

A dev page for judging rigs and behaviour by eye: every species large (rest, each clip looping, moods, blink), and a sandbox with mock cards as surfaces where a chosen cast lives, with mode, quiet, capacity, scale and theme switches. `bun nx run @semio-tech/pets-react:dev` (port `PETS_STORIES_PORT`, default 6069) serves it for the menagerie module named by `PETS_MENAGERIE` (a path relative to the repository root; default: the sample menagerie of `P/🧫️fixtures`). `.claude/launch.json`: `pets-stories` (6069, sample) and `architecture-pets-stories` (6074, `AP/🟦️.ts`); the seed gets matching rows. (6071 and 6072 of the first draft were taken by other launch rows.) Stories are never part of a release build.

## 8. Quiz integration (`QR`)

### 8.1 Seam

`QuizOptions.pets?: QuizPetsSource` with `type QuizPetsSource = () => Promise<Menagerie>` (type-only import from `@semio-tech/pets`). `QR/🔨️modules/🐾️pets/🟦️.tsx` exports `QuizPetsProvider` (loading and context), `QuizPets` (the mount), `PetsSwitch` (the footer switch, §13.5) and pure helpers (`petScene(step, runs, catalog)`, `effectivePetMode(choice, chosen, reducedMotion)`, `switchedPets`, `petNames`): the provider loads the menagerie and `@semio-tech/pets-react` lazily when the effective mode is not `off`, ignores a failed load silently (pets are decoration; a failed load writes nothing to the console, a layer that throws is taken away by an error boundary and React reports what it caught), and `QuizPets` renders `<PetLayer>` next to `<PresenceOverlay>` inside `.quiz-app`.

- Scene: home without an opened quiz page → `home`; an opened quiz page, a run or the results of a run → that quiz's id; introduction and identity → `home`.
- Quiet: `step.screen === "run"`.
- Mode (`effectivePetMode`, §13.6): preference `off` → nothing; a choice the learner made (`QuizPreferences.petsChosen`) → that choice, whatever the device asks for; no choice yet and `prefers-reduced-motion: reduce` → `still`; otherwise the preference. The device's hint decides the default only. Forced colours are the layer's concern: it shows nobody under them, whatever its mode.
- Surfaces (`QUIZ_PET_SURFACES`): the title tab (`[data-slot="window-chrome-chip-cap"]`) and the body (`[data-slot="window-chrome-body-surface"]`) of every `#quiz-main [data-card]`, and the footer line `.quiz-app > footer` (`[data-layered-card]` has no box); keep-outs (`QUIZ_PET_KEEPOUTS`): the layer's defaults plus `[data-quiz-item], [data-quiz-drop], .quiz-app > header`.
- Glances: the boxes of `[data-presence-layer] .quiz-peer` marks (other learners' cursors) when cursors are shown.
- `data-pets="<choice>"` on `.quiz-app`: the choice in effect (the mode above), which the preferences press too.

### 8.2 Preference

`QuizPreferences.pets: PetChoice` with `PET_CHOICES = ["off", "still", "calm", "lively"]`, default `calm`, `petsLiveliness` (the last choice other than `off`, which the footer switch restores; `calm` while there is none) and `petsChosen` (whether the learner chose: a fact of its own, because the preferences are written as a whole on every change; only `withPets` — the pets' row and the footer switch, on or off — sets it, and stored preferences without it count as not chosen); a `Segments` row in `PreferencesPanel` that presses the choice in effect (a learner who has not chosen sees `still` pressed on a device that asks for reduced motion), and under it the names of the pets that are on stage right now (`Species.name` in the learner's language; the layer reports them through `onCast`) and, where it applies, why they do not show (forced colours) or why they stay still until the learner chooses (reduced motion and no choice yet: "Your device asks for less motion, so the pets stay still until you choose." / "Dein Gerät bittet um weniger Bewegung, deshalb bleiben die Tierchen reglos, bis du selbst wählst."; no note once the learner chose). i18n keys (three segments, both bundles): `quiz.preferences.pets`, `petsOff`, `petsStill`, `petsCalm`, `petsLively`, `petsCast`, `petsReduced`, `petsForced`, `petsShown`. Existing tests that pin the preferences object are updated (`📓️explore-quiz-react.md` §0.5).

### 8.3 Site

`S/🟦️.ts` passes `pets: () => import("../🐾️pets/🟦️.ts").then((module) => module.ARCHITECTURE_MENAGERIE)`. Aliases `@semio-tech/pets` and `@semio-tech/pets-react` are added where `@semio-tech/quiz-react` is aliased (site Vite config, site vitest config, `QR` vitest config and tsconfig). `S/📦️packages/🟦️typescript/📋️project.json` `namedInputs` gain `AP/**` and `P/**`.

## 9. The architecture menagerie (`AP`)

- `AP/🔣️.json`: the ensemble (`semio.pets.ensemble/v1`): id `architecture`, title, species paths, bonds, casts.
- `AP/<emoji><id>/🔣️.json`: one `Species` document each (`$schema` → `P/🧬️schema/🔣️.json#/$defs/Species`).
- `AP/🟦️.ts`: static imports of all of the above → `export const ARCHITECTURE_MENAGERIE: Menagerie = assembleMenagerie(ensemble, [...])`.
- `AP/README.md`: the roster, the casts, how to add a pet.

Roster, German nouns, grounds, character sheets, colours, bonds and casts: `📓️explore-topic-pets.md` §2–§6 are the brief. Directory names:

`☀️sunny ☁️cloudy 🏠️housy 🔆️solary ♨️radiatory 🌀️pumpy 🪟️windowy 🧱️waly 🔋️battery 💨️windy 🔥️boily 🛖️roofy 🧶️insuly 😎️shady 🌬️venty ❄️chilly 🫖️kettly 🕯️flamy 🌡️thermy 🖥️servy`

Casts: scene `home` → core = the nine seeds (in the owner's order), rotation = the other eleven; scenes `physics`, `heating`, `cooling`, `demand` → `📓️explore-topic-pets.md` §5.2 (core 3, rotation 4).

`Species.grounds` entries are `<quiz id>/<task id>/<item id>` for items, `<quiz id>/<task id>` for a whole task, `<quiz id>` for the topic as such. The site test `S/🧪️tests/🐾️pet-cast` fails when a ground does not exist in the quiz files or when a species in the cast of a quiz scene has no ground in that quiz (cloudy and thermy carry topic-level grounds).

Art direction (all species):

- 40…56 px tall at scale 1, a 2 px `ink` outline on the body, palette colours from §6.3 of the roster report, round friendly shapes, two eyes (radius 3…4.5, pupil ≈ 45 % of it) and a mouth on the body bone, stubby limbs as round-capped `line` parts where the thing has none.
- Bones: `root` at the feet, `body` above it, limbs and signature parts (rays, fan, sash, charge bars, flame …) as children, so fidgets move the thing-specific parts.
- Clips every species has: an idle loop (2.5…4 s, breathing), its gait clip (`walk`: 0.5…0.7 s loop with leg swing and bob; `hop`: one hop; `float`: a bob loop used for both idle and walking), two signature fidgets, `greet`, `cuddle`, `squabble`, `sulk`, `sleep` (loop), `land`.
- Judge every rig with `node TK/render_species_preview.mjs --out TK/🗑️generated/preview/<id>.png AP/<dir>/🔣️.json` and look at the PNG; `TK/reference-species.json` is a worked example.

## 10. Tests and oracles

Protocol v2 cases live in `P/🧪️tests/<case>/{🥒️.feature, 🐍️.py, 🟦️.ts, 🦀️.rs}` with vectors in `P/🧫️fixtures/<case>/🔣️.json`, generated by a ticket script (`TK/generate_<area>_vectors.py`, named in the feature header) from the oracle code, never hand-edited. Python oracles may use numpy, scipy and jsonschema only (the harness environment has them). Comparison profile `pets-float-v1` (absolute and relative tolerance 1e-9) for floats against oracles, `ordered-json-v1` for integers and structure.

| Case | Module | Oracle (id in `P/🔮️oracles/🔣️.json`) |
|---|---|---|
| `🧬️schema-conformance` | schema, validation | `pets-jsonschema` (python-jsonschema): every valid document passes, every invalid one fails with the committed code |
| `📐️turn-trigonometry` | trigonometry | `pets-numpy` (`numpy.sin`, `numpy.cos` of `2π·t`) |
| `🎲️counter-randomness` | randomness | `pets-numpy` (`numpy.random.SeedSequence`), exact |
| `🦴️rig-solving` | rig | `pets-numpy` (3×3 homogeneous products, `numpy.linalg.inv`) |
| `👀️gaze-tracking` | rig `lookOffset`, animation `springStep` | `pets-numpy` (`numpy.linalg.norm`; `numpy.linalg.matrix_power` of the step matrix) |
| `🎞️animation-sampling` | animation | `pets-scipy` (`scipy.optimize.brentq` on the Bézier, `numpy.interp`) |
| `🪀️spring-settling` | animation | `pets-numpy` (`matrix_power`), plus settling properties |
| `🏞️terrain-walking` | terrain | `pets-numpy` (membership on a sampled grid for perches; closed forms for strides) |
| `🦘️hop-ballistics` | terrain | `pets-scipy` (`scipy.integrate.solve_ivp` / roots of the arc) |
| `🧠️behavior-choice` | behavior | `pets-numpy` (`cumsum` + `searchsorted`), `pets-scipy` (`scipy.sparse.csgraph` reachability of every activity) |
| `🤝️bond-dynamics` | behavior | `pets-numpy` |
| `🎪️stage-trace` | stage | no-oracle decision `pets-stage-trace` (no third-party library simulates this model): scripted event logs, the projection is a digest of every frame's numbers plus invariants (actors stand on perches, never inside keep-outs, at most the mode's movers, one encounter); TypeScript and Rust must agree exactly |

TypeScript unit suites (`P/🔨️modules/<m>/🧪️tests/🔬️unit/🟦️.ts`) may add JavaScript oracles already installed (gl-matrix `mat2d`, d3-ease, pure-rand is not needed), registered with `hostPath`. React suites (`P/🧪️tests/<case>/🟦️.tsx`, jsdom): `🖌️pet-depiction` (markup rules, gl-matrix for the matrix strings), `📡️surface-survey`, `⏲️frame-pacing` (fake clock: no frame requested at rate 0 or when hidden), `🫥️decorative-layer` (aria-query: no accessible node; pointer-events; no console output). Quiz suite `Q/🧪️tests/🐾️pet-companions/🟦️.tsx` (preference, scene mapping, reduced motion, lazy mount, cast names). Site: `S/🧪️tests/🐾️pet-cast/🟦️.ts` (vitest: ajv against the schema, owned validator, grounds, casts) and `S/🧪️tests/🐕️pet-walk/🟦️.ts` (Playwright: pets appear on home, never intercept clicks, clean console and CSP in the rehearsal topology, `still` under reduced motion, `off` removes the layer).

## 11. Registrations

| Registry | Entry |
|---|---|
| `TAX` `members-of-products` | `🐾️pets` |
| `TAX` `members-of-modules` | `📐️trigonometry`, `🦴️rig`, `🎞️animation`, `🏞️terrain`, `🧠️behavior`, `🎪️stage`, `🖌️depiction`, `📡️survey`, `⏲️pacing`, `🫧️layer`, `🐾️pets` (`🎲️randomness`, `✅️validation` exist) |
| `TAX` `members-of-tests` | every case of §10 not yet listed |
| `TAX` `members-of-fixtures` | every case with vectors |
| `TAX` `semanticDirectoryKinds` | `teaching-pets` (`🐾️`, `^pets$`, parent `teaching-architecture`) |
| `TAX` `semanticDirectoryMemberKinds` | `members-of-teaching-pets` (owner `teaching-pets`): the twenty species directories |
| `🧰️framework/🛍️products/🔣️.json` | member `🐾️pets`, id `framework.product.pets` |
| root `package.json` | workspaces (`bun nx run @semio-tech/repo-lib:workspaces-write`), scripts `test:pets`, `test:pets:react`, `test:pets:rs`, `typecheck:pets:react`, `dev:pets:stories` |
| root `Cargo.toml` | member + `[workspace.dependencies] semio-framework-pets` (phase 2) |
| `.vscode/🧩️launch.seed.jsonc` → `bun nx run @semio-tech/plugin-registry:generate` | `🧪️test🐾️pets🟦️` 213.69, `🧪️test🐾️pets⚛️react` 213.7, `🧪️test🐾️pets🦀️` 213.71, `🛠️dev🐾️pets⚛️react🪁️typecheck` 213.72, `🛠️dev🐾️pets📖️stories` 213.73, `🛠️dev🎓️teaching🏛️architecture🐾️pets📖️stories` 213.74 |
| `.claude/launch.json` | `pets-stories`, `architecture-pets-stories` |
| schema catalog | `bun ./📜️script.ts schema generate` |

`TK/taxonomy_name_probe.ts` proves that every name above resolves and that the taxonomy validates with them (0 problems).

## 12. Work packages

Phase 1 (TypeScript, parallel), phase 2 (Rust twins and integration), phase 3 (audits and fixes). One owner per file; shared files are edited by their owner only, or with small anchored edits where stated.

| WP | Owner of | Notes |
|---|---|---|
| A foundation | `P/README.md`, `P/🟦️.ts`, `P/🧬️schema/🔣️.json`, `P/🔨️modules/✅️validation`, `P/📦️packages/🟦️typescript/*`, `P/🧪️tests/🎚️config`, `P/🔮️oracles/🔣️.json`, `P/🧪️tests/🧬️schema-conformance`, `P/🧫️fixtures/🧬️schema-conformance`, all registrations of §11 except Cargo and launch | registers every name of §11 in one batch first |
| B kinematics | `P/🔨️modules/{📐️trigonometry, 🎲️randomness, 🦴️rig}`, cases `📐️`, `🎲️`, `🦴️`, `👀️` | trigonometry and randomness land first; others depend on them |
| C animation | `P/🔨️modules/🎞️animation`, cases `🎞️`, `🪀️` | |
| D terrain | `P/🔨️modules/🏞️terrain`, cases `🏞️`, `🦘️` | |
| E behaviour and stage | `P/🔨️modules/{🧠️behavior, 🎪️stage}`, cases `🧠️`, `🤝️`, `🎪️` | integrates B, C, D by the names of §4 |
| F depiction and stories | `PR/📦️packages`, `PR/🟦️.tsx`, `PR/🎨️.css`, `PR/🔨️modules/🖌️depiction`, `PR/📖️stories`, `PR/🏗️builder`, `PR/🧪️tests/🎚️config`, suite `🖌️pet-depiction`, launch entries | |
| G layer | `PR/🔨️modules/{📡️survey, ⏲️pacing, 🫧️layer}`, suites `📡️`, `⏲️`, `🫥️` | adds its exports to `PR/🟦️.tsx` with anchored edits |
| H1…H4 art | five species each under `AP/<species>/🔣️.json` | H1: sunny, cloudy, windy, flamy, kettly · H2: housy, waly, windowy, roofy, insuly · H3: radiatory, pumpy, boily, thermy, chilly · H4: solary, battery, shady, venty, servy |
| I quiz | `QR/🔨️modules/🐾️pets`, suite `Q/🧪️tests/🐾️pet-companions`, anchored edits in `QR/🟦️.tsx`, `QR/🎨️.css`, i18n, preferences, test configs, the two tests that pin preferences | wiring edit last, in one step |
| J site | `AP/🔣️.json`, `AP/🟦️.ts`, `AP/README.md`, `S/🟦️.ts` edit, aliases, site tests, READMEs | after H and I |
| K…M Rust | schema + validation + kinematics · animation + terrain · behaviour + stage, `P/📦️packages/🦀️rust`, `🦀️.rs` adapters, Cargo registration | phase 2 |

Rules for everyone (other agents edit this tree at the same time; see `📓️explore-concurrent-work.md` §7):

1. Re-read a shared file immediately before each edit; small anchored `Edit`s; never `Write` over an existing file you do not own; no formatters, no scripted rewrites of repo files, no git command that modifies anything.
2. Keep the tree compiling at every save: new files first, wiring last.
3. Temporary logs carry the `[DEBUG] ` prefix and are removed before you finish; nothing in pets code writes to the console.
4. Docstrings start with an emoji (never `@emoji`); no comments inside definitions; no banned name stems (`core`, `common`, `util`, `utils`, `helper`, `helpers`, `misc`, `shared`, `base`, `lib`, `impl`).
5. Tool output goes to `TK/🗑️generated/<your work package>/`; scripts and Markdown notes stay in `TK`. Write a short `TK/📓️report-<work package>.md` (what exists, how it was verified with the exact commands and their real results, what is open) and answer in chat with a few lines pointing to it.
6. Do not claim a test passes unless you ran it and saw it pass.

## 13. As built

Where the code differs from §1–§12, with the reason in a line and the work package that decided it (details in its `📓️report-wp-<package>.md`). §4–§8 above were corrected in place where a statement had become wrong; everything else of §1–§12 stands as written, and this section is the list of what does not.

### 13.1 Schema and validation

| Design | As built | Why |
|---|---|---|
| §3 types as first written | `Species`, `Menagerie`, `Ensemble` carry an optional `$schema`; `assembleMenagerie` drops the hints and does not validate | authored documents point at the contract; a menagerie is data (A) |
| §3 `Actor`, `ActorFrame` | `Actor.faced: Ticks` (the tick at which the drawing has finished turning) and `ActorFrame.activity: Activity` | turning that can be reversed midway; a host or a test can see what a pet does (P) |
| §3 `Turns` unused | `sinTurns(turns: Turns)`, `cosTurns(turns: Turns)` | the type is used instead of deleted (P) |
| §3 colours `#rrggbb` | lowercase only (`^#[0-9a-f]{6}$`) | one canonical form (A) |
| §3 "> 0" | sizes, radii, widths, seconds, speed, hover above zero; only a rectangle's corner radius may be 0; `Needs`, `opacity`, `lid` in [0, 1], `mood` in [−1, 1] | the docstrings of the contract, made checkable (A) |
| §3 codes | seven structural codes like the quiz (`type-invalid`, `required`, `property-unknown`, `value-invalid`, `slug-invalid`, `length-invalid`, `items-too-few`) before the thirteen rule codes; where both apply the rule's code wins | precedent of `Q` (A) |
| §3 `loop-seam` | on `rotation` a difference of a whole number of turns, judged with a slack of 1e-9 turns | 0 → 360 is a seamless spin; authored decimals (17.3 → 377.3) miss a whole number by one unit in the last place — 2 112 of 36 005 one-decimal pairs were falsely reported (A, P) |
| §4.8 validators take `unknown` | the Rust twins take typed documents; `required` and `property-unknown` never come out of them | serde refuses what the structure forbids; quiz precedent (K) |
| §3 Rust number types | `Facing` and `Rate` are enums on their wire numbers, `Ticked.ticks` is `u64`, the tick of a stage and of a frame refuses a negative number; other tick stamps stay `i64` | closed sets are types; the contract gives stamps no lower bound (K, M) |

### 13.2 Kinematics, animation, terrain

| Design | As built | Why |
|---|---|---|
| §4.1 reduce `t − floor(t)` | the magnitude is reduced and the sign restored; "first octant" is the residue from the nearest quarter turn | the subtraction rounds for tiny negative angles; sine is exactly odd, cosine exactly even, so a mirrored pet gets mirrored matrices (B) |
| §4.2 | `unitOf`, `weightedIndex` and the three reserved streams live in `🎲️randomness`; behaviour and stage import them | one weighted pick, one conversion, one place for the streams (audit F3; P) |
| §4.3 | `pupilReach(eye)`; `compose`, `invert`, `IDENTITY` are API for tools and tests, the solver multiplies in place | one pupil rule for stage, depiction and preview tool (P); no allocation per bone (B) |
| §4.3–§4.5 edge cases | crossed `clamp` bounds, a singular `invert`, a reach ≤ 0, short poses, unlisted parents: every function is total and finite | each is a committed vector (B) |
| §4.4 | the wrap of a looping clip uses the integer remainder; an easing whose abscissa stands still (`cubic-bezier(1, 0, 0, 1)` at one half) is solved to 1e-5 only | exact on whole ticks; ill-conditioned for every solver (C) |
| §4.5 `perchesOf` minimum 1.5 × widest | minimum = the widest | narrow edges — a title tab, the free edge of a phone card — carried nobody (N) |
| §4.5 `hopOf` | a fourth refusal (landing faster than `FALL_SPEED`); `HOP_STEEPNESS` instead of a horizontal speed limit; `hopStep` takes the target and the ticks left; `hopLanding` added | a flat dart is not a hop; the last tick lands exactly (D) |
| §2.4 extremes | Rust uses the JavaScript semantics of `min`/`max` (NaN propagates) | bit parity (L) |

### 13.3 Behaviour and stage

| Design | As built | Why |
|---|---|---|
| §4.6 `affinityOf` in [−1, 1] | in [−0.6, 1] (`AFFINITY_FLOOR`) | §5.5 wants disputes small; the floor lives where everybody reads it (E) |
| §4.6 `castOf` "core first, then the rotation as fits" | one seat for a rotation visitor whenever a rotation exists and capacity ≥ 2; a core that does not fit takes turns; two rings from seeded starts | at home (9 core, 11 rotation, capacity 6) the eleven additions never appeared and the last seeds neither (E, P) |
| §5.2 step 5 needs per tick | needs are settled when an activity ends, rapports when they are touched (`stage.met`) | the result does not depend on how time is cut into `ticked` events; lulls can be jumped (E) |
| §5.5 first encounter after the gap | a warm-up of 20 s before the first encounter | a learner who stays a minute can see one (E) |
| §5.5 who meets | neighbours with nobody between, within 12 mean widths; weighted towards bonded pairs; in `calm` only one of the two walks; partners on different perches act across the gap | "one mover at once"; no walking through a third pet (E, N) |
| §4.7 pose "blended from the rest pose" | the idle loop runs underneath every activity; activity clips are added on top and fade in and out | nothing pops, and a pet that stands still is never frozen (E) |
| §5.7 rates 64/32/0 | plus 16 while every actor only sleeps; 0 with a wake tick for species without an idle loop | a sleeper needs a quarter of the frames (E) |
| §5.2 hop gaits | walk in whole hops and finish a hop on the spot when blocked | a hop cannot stop in the air (E, N) |
| §5.1 leaving "walk to the nearest perch end" | only within 3 body widths and with a clear way, else a fade where it stands | a scene change must not be a long exodus (E, N) |
| §5.2 actor counters from 0 | from the tick of arrival | a species that returns does not replay its first life (E) |
| §5.1 arrival "spaced, else wait" and nothing else | bodies keep 8 px; a walker stops 6 px before a neighbour; a perch holds only as many as fit; who does not fit is crowded out and arrives anew; landings come down beside whoever stands there | six actors piled up on one strip in a tall run (N) |
| — | a hop is offered only when its arc touches no keep-out; its weight grows with the company on the surface (`Situation.crowd`), calm hop weight 0.12 | hops flew through the navigation bar; crowds thin out by themselves (N) |
| — | `presenceOf`: an actor turns see-through (0.35) while the pointer rests on it | whoever points wants to see what lies beneath (N) |
| §5.1 `surveyed` "falls when its surface is gone" | when the same survey brought new ground it is gone with its surface and arrives anew instead | a fall would pass in front of what is there now (P) |
| §5.6 quiet | pupils still follow the pointer; no turn, lean or greeting; a poke is ignored | a run is a time of concentration: nothing may ask for a click (audit N7; P) |

### 13.4 The pointer, turning and where pets stand (work package P)

| Design | As built | Why |
|---|---|---|
| §4.7 pupil travel `radius − pupil − 0.75` | `radius − pupil − 0.25` (`pupilReach`); the gaze reaches half of its way at 32 px (`GAZE_REACH`, was 96) | on screen the pupils moved 0.9–1.55 px and one could not see them follow; now 1.3–1.9 px measured (audit S2) |
| §5.2 "faces its goal" (N: facing flips 8 ticks after `since`) | `Actor.facing` is the new way at once; the drawing sweeps round over the 8 ticks before `Actor.faced`; a turn back takes as long as the turn has run | a turn can be reversed midway without a jump, and standing actors can turn too |
| §1 "with the eyes etc": eyes only | an idle actor that stands by itself turns round to a pointer more than 12 px beyond its body (`TURN_CLEAR`), at most once per 56 ticks after a turn (`TURN_REST`) | the owner's "etc"; the dead zone and the rest keep a pointer that crosses over and back from flipping the pet |
| — | the bone that carries the first eye leans after the gaze: 5° and 1.5 px per unit across, 1 px per unit down (`leant`) | the head follows the eyes; the gaze is a spring, so the lean eases with it |
| — | a pointer that rested for half a second within 1.5 heights beside an idle actor makes it greet when its curiosity is at least 0.5; the greeting costs 0.6 of it | curious species perk up; the need is the cooldown, so `temperament.curiosity` scales how often |
| §5.1 arrival "a free perch chosen by a stage draw" | newcomers take the perches that hold the fewest, the ground (the lowest perches of the stage) only when no higher one is as empty | a first-time learner saw all six parked on the footer line for about 25 s (audit S10) |
| — | over new ground the company spreads: every perch that holds more than one gives up all but its first to perches that hold nobody | the company that gathered on the only edge of the introduction moves onto the cards when the overview appears |
| §6.2 attributes of a pet | `data-pet-activity` carries `ActorFrame.activity`, written only when it changes (a sixth painted slot per actor) | whoever watches the pets from outside — a test, a host's stylesheet — sees what a pet does; no per-frame attribute write |

Desktop pages other than the overview still show pets on the footer line only (quiz page, leaderboard, badges, settings, introduction, learner). Measured on the real DOM: the first card's tab and edge lie 35 and 57 px under a 29 px navigation bar, less than a pet is tall (40–56 px plus hover), and every later card begins 30 px below the card above. That is a real lack of room, not a defect of a rule; the layout would have to give the cards more air to change it.

### 13.5 React target

| Design | As built | Why |
|---|---|---|
| §6.1 props | plus `onCast` and `tempo` | the preferences name who is on stage; an end-to-end proof of an encounter needs minutes of stage time in seconds (P) |
| §6.1 | a menagerie with validation issues is never shown; a running show sets `pointer-events: none` on the layer itself | nothing the schema forbids (a `url(…)` as a colour) reaches the page; the layer must not take clicks when its stylesheet is missing (audit F17, F16; P) |
| §6.2 `.pet` CSS | plus `width: 1px; height: 1px; overflow: visible; transform-origin: 0 0`; ink, paper and pupil are overridable custom properties | a mirrored `<svg>` flips around its 300 × 150 default box otherwise (F) |
| §6.2 "nothing here animates" | `.pet-layer, .pet-layer * { transition-property: none !important }` | the quiz gives every element a transition of 0.01 ms under reduced motion (`.quiz-app * { transition-duration: 0.01ms !important }`); since pets may move there (13.6), every write of a frame started a transition — 485 `transitionrun` events in two seconds of calm pets, 170 to 190 animations held at once, none without reduced motion — and one of 39 runs of the reduced-motion spec ended in an access violation of the browser's renderer; 0 transitions since (R) |
| §6.2 rounding 3 decimals | the place of an actor to 2 decimals; `depictionMarkup` carries no place, opacity or palette | those live in the CSSOM only (F) |
| §6.3 surfaces and keep-outs | a surface and its ancestors are never keep-outs; every surface is solid sideways; margins never reach over an edge the element lies under; everything is clipped by its scrolling ancestors; `PET_TEXTS` also names `code`, `output`, `progress`, `meter`, `[role="alert"]`, `[role="status"]`; bare text is marked `data-pet-keepout` | pets hovered above cards, stood in front of them and were blocked by text scrolled out of sight (G, N); read-outs were not text (audit N5; P) |
| §6.3 one staleness | two urgencies (scroll and resize with the next frame; mutation, focus, transition end within 8 ticks); `focusout` watched; a survey equal to the last is not dispatched | pets ride a scrolling card frame by frame without surveying on every mutation (G) |
| §6.3 "transition end" makes a survey stale | not the end of a transition inside the layer (`watchSurvey` passes over what ends inside `ignore`, as it does with mutations there) | with the 0.01 ms transitions of the §6.2 row above ("nothing here animates") every frame of a pet ended transitions, each of which made the survey stale and woke the pacer: at the test tempo the page was measured anew with every frame, and one probe session stopped answering for more than twenty seconds; the stylesheet now prevents those transitions, the watch no longer depends on it (R) |
| §6.3 pointer on `window` | `pointerleave` on the root element; a lifted finger sends `unpointed`; a still stage hears of the pointer only on or beside a pet | `pointerleave` does not bubble; touch has no hover (G, N) |
| §6.4 | the stage lives in pixels ÷ the drawing size; surfaces end 6 px inside the layer (`PET_EDGE_INSET`); the show rests while the layer is inert or hidden; the rotation never advances while still or quiet | pets are drawn at 0.8 on a phone; a pet at the window edge was cut off; pets wait behind a dialog (G, N) |
| §6.5 focus margin | stays 8 px (the prior-constraints list said 24) | the quiz's focus ring is at most 4 px wide and every control is a keep-out already; 24 px would clear the perches beside every focused control (audit N4; P) |
| §6.1 unknown scene → `home` | kept | `petScene` only names `home` or a quiz of the catalog, and the site suite fails when a quiz has no cast (audit N1; P) |
| §7 ports 6071 / 6072 | 6069 / 6074 | the first two were taken by other launch rows (J, N) |
| §7 | the gallery takes its menagerie from a virtual module of the builder (`pets-stories:menagerie`) and finds it among all exports | no ambient declaration; named exports work (F) |
| — | `react-dom` is a devDependency of `@semio-tech/pets-react` | only the gallery and the suites use it (audit F24; P) |

### 13.6 Quiz and site

| Design | As built | Why |
|---|---|---|
| §8.1 one component | `QuizPetsProvider` + `QuizPets`, an error boundary around the layer | the cast names must reach the preferences deep in the tree; a layer that throws must not take the quiz down (I) |
| §8.1 `petScene(step, runs)` | `petScene(step, runs, catalog)` | a home page is a quiz exactly when the catalog lists it (I) |
| §8.1 surfaces `[data-layered-card], #quiz-main [data-card]` | the title tab and the body of every card, and the footer line; keep-outs plus the navigation bar | `[data-layered-card]` has no box; the card's box starts with an almost empty tab row, so pets hovered (I, N) |
| §8.2 cast line names the scene's cast | it names who is on stage right now ("Here right now" / "Gerade hier") | at home it listed eleven pets that never showed (audit S6; P) |
| §8.2 | a "Show pets" / "Tierchen anzeigen" checkbox on the footer of every screen; `petsLiveliness` remembers the last choice other than `off` | WCAG 2.2.2: the control was one card away from the motion (audit S1; P) |
| §8.2 | under the choice: why pets stay still (reduced motion) or do not show (forced colours) | a learner on "Lively" saw still pets, or none, without a reason (audit S8, N6; P) |
| §8.1 mode: `prefers-reduced-motion: reduce` → `still`, whatever the learner chose (I, P) | the device's hint decides the default only: `still` for a learner who has not chosen, and a choice the learner made — any of the four in the preferences, or the footer switch on or off — holds as chosen on every device; `effectivePetMode(choice, chosen, reducedMotion)`; the note reads "… stay still until you choose." and is gone once the learner chose | the owner's workstation is a Remote Desktop session, where every browser reports reduced motion, so the owner never saw the pets move; and a learner who explicitly asks for moving pets should get them (owner; R) |
| §8.2 `pets`, `petsLiveliness` | plus `petsChosen: boolean`, set by `withPets` only and read as `true` only when stored as `true` | the preferences are written as a whole on every change, so the presence of `pets` in storage proves nothing; preferences stored without the mark count as not chosen (R) |
| §8.1 `data-pets` carries the stored choice; the row presses it | both carry the choice in effect (`still` for a learner who has not chosen on a device that asks for reduced motion) | a pressed "Calm" over motionless pets would say the opposite of the note beside it, and choosing "Calm" would mean pressing a button that looks pressed (R) |
| — | the footer switch brings back `calm` when there was no liveliness yet, also on a device that asks for reduced motion | turning the pets on is asking for them; the row offers `still` to whoever wants them motionless (owner; R) |
| §8.1 quiet = a run | results are not quiet | a learner who reads a score is done concentrating, and the switch is on that screen (audit S9; P) |
| §8.2 German "Still" | "Reglos" | the both-languages spec fails on a button worded the same in both languages (I) |
| — | `data-pets-tempo` on the document root (`QUIZ_PETS_TEMPO`) | the test seam of the encounter proof and of the proof that calm pets walk under reduced motion once chosen; nothing in the client sets it (P, R) |
| — | segments get a minimum width of 1.5 rem | "Off" was 22 px wide (WCAG 2.5.8) (I) |
| §8.2 group label "Pets" | kept | the sibling groups are nouns too ("Language", "Theme", "Text size"); the options say what is set (audit N11; P) |
| §9 `AP/🟦️.ts` | one named export, no default | the gallery finds named exports; the default was unused (audit F21; P) |
| §10 `🐕️pet-walk` in project `desktop` | its own Playwright project `pets`, after `shortage`, nothing depends on it | a failing pets spec must not skip the presence and shortage projects (coordinator) |

### 13.7 Art

| Design | As built | Why |
|---|---|---|
| §9 one-shot gestures | `greet`, `cuddle`, `squabble`, `sulk` are loops in most species | the activities last 2…6 s; a one-shot would hold its last key (H1, H3) |
| §9 limbs as children of `body` | in H3's five species the legs are children of `root` | body bob, lean and squash never lift or bury the feet (H3) |
| §9 hop gait | the repertoire maps `walk` and `hop` to the one hop clip | the stage reads `walk` for a hopping species on the move (H2) |
| §9 `size` | the true rest box, stroke and idle loop included | pets overlapped the card edge they stood beside (O1) |
| §9 walk clips 0.5…0.7 s | solary 0.4375 s, servy 0.9375 s; every gait re-keyed so the stride per cycle is speed × clip seconds | feet slid 2.4–6.9 px per stance (O2) |
| §9 "appearing" props | pumpy's air streaks, boily's smoke ring and chilly's breath cloud rest behind the body and slide out | a part cannot be hidden by opacity (H3) |

Not built from the roster brief: the season flip of `windowy – sunny` and signature scenes. "One visitor at home" is built (13.3).

### 13.8 Tests, oracles, registrations

| Design | As built | Why |
|---|---|---|
| §10 oracles project library numbers | terrain oracles answer the binary64 value of the stated tick and refuse unless the library agrees within 1e-9; three cases carry `bit-patterns` scenarios as a declared supplement | the harness rounds onto a 1e-9 grid on which ten-bit fractions are ties; 1e-9 cannot show bit parity (B, D) |
| §10 `🧬️schema-conformance` | python-jsonschema judges the structure; the rule findings are recomputed by a second implementation written from the design text | no library knows the rules (A) |
| §10 `🎪️stage-trace` | no-oracle decision resting on specification vectors, laws (`apart` among them) and, since the Rust adapter, independent implementations; the traces are recorded from the TypeScript subject | no third-party library simulates this model; the residual risk is a shared misreading of the rules (A, E, N) |
| §10 scripts | thirteen: among them `crowded-strip`, `tab-and-footer`, `pointer-rest` (N), `pointer-attention`, `new-ground` (P) | every new rule has a trace |
| §10 `--case <slug>` | the directory name with its emoji | the bare slug selects nothing (C) |
| §10 JavaScript oracles | `pets-gl-matrix`, `pets-d3-ease`, `pets-polygon-clipping`, `pets-ajv-structure`, `pets-react-gl-matrix`, `pets-react-aria-query` | registered with `hostPath` (A, J) |
| §11 fixtures "every case with vectors" | `🖌️pet-depiction` and `🐾️pet-companions` are tests only | they build their inputs inline; the stale names were removed from `members-of-fixtures` (P) |
| §11 workspaces by `workspaces-write` | by hand | the command would have registered seven in-flight packages of other tickets (A) |
| §11 | `@semio-tech/pets:typecheck` (core, unit suites, adapters), launch row `🛠️dev🐾️pets🟦️🪁️typecheck` 213.715, script `typecheck:pets` | the core was typechecked only through the React package (audit F25; P) |
| §11 | `@semio-tech/quiz-react` nx inputs name the pets sources it bundles | a change of the pets did not invalidate the quiz's cached targets (P) |
| §11 dependency baseline | not rewritten | `write-baseline` would approve every unclassified entry of the repository, not only the pets' (P) |

### 13.9 Rust twins

| Design | As built | Why |
|---|---|---|
| §4 "the same in snake_case" | `advance` takes the stage by value; `cast_of(cast, usize, i64, u32)`; `random_pick` returns `Option<usize>` (`None` = −1); `Clip.loop` is the field `looping` | ownership, types that say what the twin floors, a keyword (K, M) |
| §2.4 | a host that decodes pets documents with serde_json must enable `float_roundtrip` | without it long decimals are read one unit in the last place off (K, L) |
| §2.4 | where the twins cannot agree — two species of one id, NaN coordinates, a fractional tick count, clip seconds beyond 1.4e17 — lies outside every valid document | recorded, not reconciled (L, M) |
