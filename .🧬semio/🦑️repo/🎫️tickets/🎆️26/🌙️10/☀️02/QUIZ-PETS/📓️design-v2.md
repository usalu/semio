# 📓️ Quiz Pets — second round (normative)

Ticket `2026/10/02/QUIZ-PETS`, reopened 2026-10-02 (night). This document is the contract of the second round. It builds on `📓️design.md` (first round, with §13 "As built") and supersedes it wherever the two differ. Evidence: `📓️explore2-core-as-built.md` (CORE), `📓️explore2-react-as-built.md` (REACT), `📓️explore2-quiz-dom.md` (DOM), `📓️explore2-species-content.md` (CONTENT), `📓️research2-mechanics.md` (MECH), `📓️explore2-concurrent-work.md` (WORK). Read the report named in your brief before touching the area it maps.

Paths as in the first design: `P = 🧰️framework/🛍️products/🐾️pets`, `PR = P/🎯️targets/⚛️react`, `Q = 🧰️framework/🛍️products/❓️quiz`, `QR = Q/🎯️targets/⚛️react`, `S = 🎓️teaching/🏛️architecture/❓️quiz`, `AP = 🎓️teaching/🏛️architecture/🐾️pets`, `TK` = this ticket folder, `TEST = 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`. Every emoji in a path is the emoji followed by U+FE0F; copy names from `bun TK/taxonomy_name_probe_v2.ts` (run from the repository root), never retype them.

---

## 14. The owner's second request and the answers

| Owner's words | Answer |
|---|---|
| "The pet actions must go way further. They should also start to climb up ui elements, use grappling gun, use parachutes, use a ladder, etc" | New footings beside the perch (§16): `wall` (climbing the side edges of cards), `ladder` (a ladder a pet carries, raises and climbs; others may use it while it stands), `rope` (a grappling gun: aim, shoot, reel in or swing), `chute` (a parachute opens on every fall that would land hard), `head` (landing on another pet and sliding off). Species choose their gear by their nature. |
| "draggable in the free space (then they hang and fall afterwards - when too high then they use the parachute to land soft)" | Press and move picks a pet up by its scruff; it dangles under the pointer as a pendulum, is thrown with the pointer's velocity on release, and opens its parachute when the landing would be hard (§17). |
| "Make sure that they dont collide with each other." | A hard invariant at the end of every tick: the bodies of any two visible pets are disjoint. Planned paths, guarded motion, head landings and, as the last resort, a "poof" (the pet vanishes in a puff and arrives anew) — never an overlap (§18). |
| "Pets can do different tricks e.g. when circling around the sun something happens. The tricks are based on what the pet is" | Species tricks as data (clip, particles, resulting state), cued by gestures (circling clockwise or counter-clockwise, stroking, shaking a held pet), by clicks, by whim, or to show off to another pet (§19). |
| "Pets can have different states (e.g. strong shining, etc)" | Species states as data (tint, overlay clip, particles), changed by tricks, time and the neighbours (§19). |
| "Pets can have different moods." | Nine shared moods with intensity on one face (§19). |
| "The interaction of the pets with each other is dependant on the mood and state." | Chemistry: authored reactions `when A (state, mood, activity) is near B → effects` plus mood contagion; both shape encounters (§19). |
| "The user can interact with the pets with left clicking … first time hello, then … tricks or purr" | Click escalation by a leaky "heat": hello → a trick per click → purr → "enough" (a cooldown without punishment) (§17). |
| "Pets can start to alter ui elements to a certain degree … never change the document. As soon as the user interacts with the ui elements again then the pets are thrown out and the ui element returns" | Mischief on lifted copies (§20): the element stays where it is, untouched and usable, only transparent; the pet layer shows a copy that the pet pushes out of its stack. Any touch of the original restores it at once and throws the pet off. Only elements that fit the pet's topic (`grounds`). |

Decisions taken by the coordinator (owner-level, recorded here):

1. **The layer stays pointer-transparent.** Presses on pets are recognised by window-level listeners and geometry (REACT §0: real hit targets would break the quiz's drop zones and presence anchors). A control under the pointer always wins; a pet takes a press only where nothing interactive lies beneath it.
2. **Mischief never moves a real element.** Lifted copies only (DOM §4: gaps of 3 px, clip paths, measured boxes, lost clicks).
3. **No translucent overlap.** Where MECH proposes a "ghost", the pet poofs instead.
4. **A run is still a time of concentration** (`quiet`): no encounters, no whims, no walking over the task. Pets answer the learner's own clicks and can be picked up; mischief happens in a run only after the learner has been idle for a long while, and only if the learner allows it.
5. **Keyboard and single-pointer equivalents** (WCAG 2.1.1, 2.5.1, 2.5.7): a "Play with the pets" group in the settings offers hello, trick, pet and toss per pet on stage; every gesture trick is also reachable by clicks.
6. **Customisable:** preferences `petsPlay` (pets react to clicks and can be picked up; default on) and `petsMischief` (pets may play with the page; default on) beside the existing liveliness; `still` and reduced-motion-by-default switch all of this off.
7. **Push choice never reveals an answer:** which fixture is pushed, when and how far depends only on the topic match and the stage's own randomness, never on an item's value, correctness or the learner's answers.

## 15. Architecture of the second round

### 15.1 Layering (MECH §0.2)

Physics decides where a body can be (`footing`), activity decides what it looks like, mood decides what it wants.

- **Footing** (exclusive): `perch`, `air` (hop, fall, thrown), `chute`, `hand` (held by the learner), `rope`, `ladder`, `wall`, `head`. It replaces the overloaded `perch === null` of the first round (CORE §0).
- **Activity** (exclusive; the existing eleven, then appended in this order so recorded indices stay): `hang`, `tumble`, `glide`, `aim`, `reel`, `climb`, `mantle`, `slide`, `carry`, `trick`, `purr`, `dizzy`, `shrug`, `scoot`, `push`.
- **Mood** (enum + intensity), **state** (species data), needs and rapport (existing).

### 15.2 Modules of the core (`P/🔨️modules`)

The 1 728-line stage is split first, without changing behaviour (CORE §5 lists the functions per module and proves the graph acyclic): `📝️draft`, `🚧️clearance` (the first round's spacing functions, then §18), `🗓️schedule`, `👀️attention`, `🚶️locomotion`, `💞️sociability`, `🎯️choice`, `👥️population`, `🕰️clock`, `🎥️projection`, and the façade `🎪️stage` (`openStage`, `advance`, `frameOf`). TypeScript and Rust are split together; proof: the thirteen committed traces and the long-run digests are identical before and after.

New pure modules (each `🟦️.ts` + `🦀️.rs` + unit suites + a Protocol v2 case):

| Module | Content | Case | Oracle |
|---|---|---|---|
| `📐️trigonometry` (extended) | `atanTurns(y, x)`, `fastNegExp(x)` (MECH §0.1) | `📐️turn-trigonometry` | numpy |
| `🏞️terrain` (extended) | `wallsOf` (free stretches of walls), `segmentClear(p, q, rects)` (Liang–Barsky) | `🧗️wall-climbing` | numpy grid, scipy |
| `🪢️swing` | `swingStep` (SHAKE rod/rope step), the follow spring, release velocity from seven samples, chute descent and trigger (MECH §1–§3) | `🪢️swing-dynamics`, `🪂️parachute-descent` | scipy `solve_ivp`, numpy `polyfit` |
| `🧗️climbing` | walls (grab, climb, slide, mantle path), ladder geometry and placement rules, grapple reach and line of sight, zip and swing reel (MECH §3–§5) | `🧗️wall-climbing`, `🪜️ladder-geometry`, `🎣️grapple-reach` | numpy, scipy |
| `🚧️clearance` | bodies (`bodyOf`), `overlaps`, free-spot search, corridor claims, isotonic re-seating (`seatOf`, PAVA), head platforms (MECH §6) | `🚧️clearance-proof` | scipy `optimize` / numpy for the seating, an independent overlap test |
| `👆️gesture` | the press state machine (click, hold, pick-up), circle / stroke / shake detectors, the heat bucket (MECH §7, §8.4) | `👆️gesture-recognition` | numpy winding (`unwrap(arctan2)`), `scipy.signal.find_peaks` |
| `💗️feeling` | moods (impulse, priority, decay, contagion), species states, trick choice, chemistry evaluation (MECH §8) | `💗️feeling-dynamics`, `⚗️chemistry-rules` | numpy |
| `✨️effects` | particles as pure functions of emitter, index and tick; the `mix` hash (MECH §9) | `✨️particle-motion` | numpy `uint32` and vectorised formulas |
| `🪄️mischief` | which actor may lift which fixture (`grounds` ↔ fixture key), lift kinematics (push out, wobble, put back), reclaim (throw-off velocities) | `🪄️mischief-choice` | numpy |

Determinism rules of `📓️design.md` §2.4 stay binding (exact IEEE operations only; `atanTurns` and `fastNegExp` are polynomial/rational). New random streams get named constants in `🎲️randomness`; existing paths must not gain draws.

### 15.3 What the stage owns

The integration lives in the split stage modules: footings and travel in `🚶️locomotion` (with `🪢️swing`, `🧗️climbing`, `🚧️clearance`), the learner's hand in `👀️attention` (with `👆️gesture`), moods, states, tricks and chemistry in `💞️sociability` and `🎯️choice` (with `💗️feeling`), surveys with walls and fixtures in `👥️population`, the order of a tick in `🕰️clock`, and the frame in `🎥️projection` (with `✨️effects`, `🪄️mischief`). Runtime types (`Actor`, `Stage`) gain what these need; their owner is the integrator who needs the field, in all three schema twins at once. Every time-driven change is added to the three horizons (`lull`, `paceOf`, the frame's `wake`; CORE X4).

## 16. Getting around: wall, ladder, rope, parachute

Survey additions (§21): `walls` (the left and right edges of surface elements: `{ id, surface, side, x, y0, y1 }`, top to bottom) and their free stretches by `wallsOf` (keep-outs in the band beside the wall removed, like perches).

- **Gear** is species data: `gear ⊆ { climb, ladder, grapple, parachute }`. Floaters (gait `float`) have no gear except `parachute` when the artist gives them one: they change lanes instead.
- **Routes.** When an actor wants to reach a perch it cannot walk or hop to, it plans a route with its gear, preferring in this order what it owns: a standing ladder that leads there, the wall beside it, its own ladder, the grapple; a plan is only taken when every leg is clear (§18). No route: it stays.
- **Wall** (MECH §5): grab at the foot or from the top corner, climb at about 30 px/s with the `climb` clip (phase by distance), a grip budget, `mantle` over the corner onto the perch, `slide` down; it slips into a fall when the wall moves or vanishes. A pet may also rest on a wall for a while (this is how pets stay on pages whose card tops have no headroom).
- **Ladder** (MECH §4): `carry` to the spot, raise it against a higher edge (lean ratio 0.14…0.40), climb; the ladder is a stage object with an owner and a lifetime, one climber at a time, taken along or left standing for a while; it topples when its upper edge leaves.
- **Rope** (MECH §3): `aim`, the hook flies in a straight clear line to a perch edge above, then `reel`: a steep line zips straight, a slanted one swings (`swingStep` with a shortening rope) and mantles; a miss retracts and the pet shrugs.
- **Parachute** (MECH §2): opens when the predicted impact speed exceeds the hard-landing threshold (from rest about two body heights) and the species has one; descent approaches a terminal speed, sways, steers to a free spot, lands without the squash; a species without a parachute lands hard (`land` clip) and is `dizzy` for a moment after a long fall.
- **Head**: another pet's top is a one-way platform; whoever lands there slides off to a free side.
- Mode limits: `calm` uses gear rarely (a route when a perch above is the only way to its friend or its fixture), `lively` often; `still` and `quiet` never start a route.

## 17. The learner's hand: click, hold, drag, gestures

Events (§21): `pressed`, `dragged`, `released`, `cancelled`, `pointed` (with what lies under the pointer), `stirred`, `played`, `permitted`.

- **Press state machine** in `👆️gesture` (MECH §1.2, §7.1): a press only arms; release without movement = click; movement beyond the slop = pick-up; a long hold = purr while pressed. Escape, pointer cancel, blur, hidden, pause abort and undo.
- **Click escalation** (MECH §8.4): each click adds heat, heat leaks; hello (`greet`), then the species' click tricks in their order, then `purr`, then "enough" (`shrug`, turned away, clicks answered only with a glance for a few seconds). No counters on screen, no punishment.
- **Held** (footing `hand`, activity `hang`): gripped at the scruff (`Species.grip`), the anchor follows the pointer by a critically damped spring, the body swings under it by `swingStep`; the frame carries `tilt` and `pivot`. A held pet is solid: it is pushed out of other bodies, never through them (§18). Shaking it makes it `dizzy`.
- **Release**: velocity from the last seven samples (capped), then `tumble` through the air with air control towards a free spot; parachute rule of §16; landing mood by how it went (soft: proud or happy; hard: grumpy; the scared ones scared while held).
- **Gestures** (MECH §7): circling the pointer round a pet (quadrant winding, clockwise and counter-clockwise are different cues), stroking (back and forth over the body) → purr, shaking while held. Ordinary pointer travel must not trigger anything (zero false triggers on the committed corpus).
- **Courtesy changes**: the first round's see-through under the pointer goes; instead a pet under a resting pointer looks up and perks. A pet becomes see-through only while it is airborne over a control or text.
- **Touch**: taps work everywhere (click escalation). Dragging needs hit pads (`pointer-events: auto`, `touch-action: none`) on grounded pets, shown only for coarse pointers while play is permitted (MECH §11.2).
- `still` and reduced-motion-by-default: no press handling at all. `quiet`: clicks and pick-up work, tricks stay short, nothing is started by the pets themselves.

## 18. Never colliding

- **Body** (`bodyOf`): the species size box (plus hover), widened by held gear (canopy, lean), with a margin of 4 px (so resting neighbours keep the first round's comfort gap of 8 px).
- **Invariant**, checked after every `advance` in the suites and the traces: for any two visible actors the bodies are disjoint; the order of grounded actors on one perch changes only by a hop over.
- **Means** (MECH §6.3–§6.5): sequential update with check-before-commit; everything that flies, climbs or reels is planned first and claims its swept corridor (others respect claims); landing spots are reserved; falls steer; heads are platforms; a held pet is projected out of bodies; surveys re-seat a perch by isotonic regression executed as a guarded scoot; `scoot` lets a parked pet yield; waits time out into new goals.
- **Last resort — poof**: when no legal continuation exists, the pet vanishes at once (opacity 0, a dust burst in its place), is not a body, and arrives anew at a free spot. Poofs are counted and must stay rare.
- **Encounters** keep partners a body apart: the gap is the sum of both species' `reach` (how far their encounter clips lean out), so poses never touch.
- **Proof**: the executable invariant over the committed traces and over fuzzed traces (surveys that shrink, move and vanish; random drags over other pets and throws into them; mode changes), in TypeScript and Rust with equal digests; metrics published per run: overlaps (must be 0), near misses, poofs, mean waiting.

## 19. Moods, states, tricks, chemistry

- **Moods** (shared, CONTENT §A): `content`, `happy`, `playful`, `curious`, `proud`, `sleepy`, `grumpy`, `sad`, `scared`, each with an intensity in [0, 1]; impulses from events with priorities (`scared` > `grumpy` > `sad` > `proud` = `happy` = `playful` > `curious` > `content`; `sleepy` follows the energy need), a minimum hold, linear decay back to the species' resting mood (MECH §8.2). The face shows it (mouth bend, lid height, lid slant, posture offset: CONTENT §A); the first round's scalar becomes the mood's valence times intensity.
- **Moods steer behaviour**: weight multipliers per mood in `activityWeights`, which tricks are on offer, how an encounter turns out (`encounterOf` takes both moods), gaze manners.
- **States** (species data): `states: [{ id, name, tint?, clip?, emitter?, lasts?, then? }]`, the first is the resting state. A state can tint the palette, loop an overlay clip and run an emitter; `lasts` seconds later it falls back to `then`.
- **Tricks** (species data): `tricks: [{ id, name, clip, cues, emitter?, from?, to?, mood? }]`; cues: `click`, `circle`, `countercircle`, `stroke`, `shake`, `whim`, `show`. Circling clockwise climbs the species' state ladder, counter-clockwise descends it (CONTENT §8). `purr: { clip, emitter? }` is every species' contentment.
- **Emitters** (species data): `emitters: [{ id, bone, x, y, shape, fill, stroke, strokeWidth?, motion, count, life, speed, spread }]` with motions `fall`, `rise`, `burst`, `orbit`, `drift`; particles are pure functions of time (MECH §9), capped at 160 on stage.
- **Chemistry** (menagerie data): `chemistry: [{ id, when, near, within, where?, every, chance?, then }]` with traits `{ species, state?, mood?, activity? }` and effects `{ on, state?, mood?, amount?, rapport?, encounter?, trick? }` (CONTENT §7 has the sixty-four physics rules and six mood rules of the architecture menagerie; MECH §8.3 the evaluation and contagion). Evaluated in a fixed pair order on a slow beat with stage draws.

## 20. Mischief on lifted copies

- The host marks elements a pet may play with: `data-pet-prop="<key>"`, where the key is a ground in the menagerie's vocabulary (`<quiz>`, `<quiz>/<task>`, `<quiz>/<task>/<item>`). The survey reports them as fixtures `{ id, key, x, y, width, height }` when they are visible, not inert, not focused, small enough to copy (at most 80 nodes), and beside a free wall stretch or perch where a pet can work.
- An actor whose species has a ground equal to the key (or a prefix of it) may `push` the fixture: it goes there with its gear (typically up the card's wall in the page gutter), braces, and shoves. The core owns the lift's motion: `{ fixture, dx, dy, tilt, opacity }` — out of the stack by up to a body width, a wobble, later back in.
- The shell realises a lift without touching the document's content: the original keeps its place, size, focusability and listeners and only gets `opacity: 0` through the CSSOM; a deep copy without ids, names, `data-*` hooks and inline styles, `inert` and `aria-hidden`, with the computed look copied by CSSOM writes, is shown in the pet layer at the original's box plus the lift.
- **Reclaim**: pointer over the original, or pointer down, focus, key, input, change, drag start on it or inside it (and, in a run, any learner input at all) → the shell restores the original in the same task (copy removed, opacity restored) and sends `reclaimed`; the pet is thrown off (`tumble`, parachute rule, a startled then sheepish mood). Also on hidden, pause, unmount, `still`, scene change, and when the fixture's box changes.
- Limits: one lifted fixture at a time; only when `petsMischief` is on, the pointer is fine, the viewport is at least 1024 px wide, and the learner has not stirred for 12 s (in a run: 30 s); a lift lasts at most 20 s before the pet puts it back; `calm` at most every few minutes, `lively` more often. Rule 7 of §14 binds the choice.

## 21. Contract changes (documents, events, frame)

Documents (`P/🧬️schema`, three twins, `✅️validation` with new rule codes and vectors):

```ts
export const ACTIVITIES = [/* the first eleven */ "hang", "tumble", "glide", "aim", "reel", "climb", "mantle", "slide", "carry", "trick", "purr", "dizzy", "shrug", "scoot", "push"] as const;
export const MOODS = ["content", "happy", "playful", "curious", "proud", "sleepy", "grumpy", "sad", "scared"] as const;
export const FOOTINGS = ["perch", "air", "chute", "hand", "rope", "ladder", "wall", "head"] as const;
export const GEARS = ["climb", "ladder", "grapple", "parachute"] as const;
export const CUES = ["click", "circle", "countercircle", "stroke", "shake", "whim", "show"] as const;
export const DRIFTS = ["fall", "rise", "burst", "orbit", "drift"] as const;
export const DEEDS = ["hello", "trick", "pet", "toss"] as const;

export type Tint = { readonly body?: Color; readonly accent?: Color; readonly detail?: Color };
export type Emitter = { readonly id: Slug; readonly bone: Slug; readonly x: number; readonly y: number; readonly shape: Shape; readonly fill: Paint; readonly stroke: Paint; readonly strokeWidth?: number; readonly motion: Drift; readonly count: number; readonly life: number; readonly speed: number; readonly spread: number };
export type SpeciesState = { readonly id: Slug; readonly name: Text; readonly tint?: Tint; readonly clip?: Slug; readonly emitter?: Slug; readonly lasts?: number; readonly then?: Slug };
export type Trick = { readonly id: Slug; readonly name: Text; readonly clip: Slug; readonly cues: readonly Cue[]; readonly emitter?: Slug; readonly from?: readonly Slug[]; readonly to?: Slug; readonly mood?: Mood };
export type Purr = { readonly clip: Slug; readonly emitter?: Slug };
// Species gains: states (≥ 1), tricks, purr, emitters, gear, grip (scruff height above the feet), reach (how far encounter poses lean out), canopy? (a Shape drawn as its parachute), mood (its resting mood)
export type Trait = { readonly species: Slug; readonly state?: Slug; readonly mood?: Mood; readonly activity?: Activity };
export type Effect = { readonly on: "when" | "near"; readonly state?: Slug; readonly mood?: Mood; readonly amount?: number; readonly rapport?: number; readonly encounter?: "greet" | "cuddle" | "squabble"; readonly trick?: Slug };
export type Reaction = { readonly id: Slug; readonly when: Trait; readonly near: Trait; readonly within: number; readonly where?: "above" | "below" | "beside" | "any"; readonly every: number; readonly chance?: number; readonly then: readonly Effect[] };
// Menagerie and Ensemble gain: chemistry: readonly Reaction[]
```

Events (shell → core; `poked` is removed):

```ts
export type Wall = { readonly id: string; readonly surface: string; readonly side: 1 | -1; readonly x: number; readonly y0: number; readonly y1: number };
export type Fixture = { readonly id: string; readonly key: string; readonly x: number; readonly y: number; readonly width: number; readonly height: number };
// Surveyed gains: walls: readonly Wall[]; fixtures: readonly Fixture[]
// Pointed gains: over: "free" | "control"
export type Pressed = { readonly kind: "pressed"; readonly x: number; readonly y: number; readonly pointer: "mouse" | "pen" | "touch" };
export type Dragged = { readonly kind: "dragged"; readonly x: number; readonly y: number };
export type Released = { readonly kind: "released"; readonly x: number; readonly y: number };
export type Cancelled = { readonly kind: "cancelled" };
export type Reclaimed = { readonly kind: "reclaimed"; readonly fixture: string };
export type Stirred = { readonly kind: "stirred" };
export type Played = { readonly kind: "played"; readonly species: Slug; readonly deed: Deed };
export type Permitted = { readonly kind: "permitted"; readonly play: boolean; readonly mischief: boolean };
```

Frame (core → targets; numbers and enums only):

```ts
export type ToolFrame = { readonly kind: "chute"; readonly open: number; readonly sway: Turns } | { readonly kind: "rope"; readonly x: number; readonly y: number; readonly slack: number } | { readonly kind: "hook"; readonly x: number; readonly y: number } | { readonly kind: "gun"; readonly aim: Turns } | { readonly kind: "ladder"; readonly lean: Turns; readonly length: number };
// ActorFrame gains: footing, state, mood (the enum), spirits (the first round's number), tilt: Turns, pivot: Point (feet coordinates), tools: readonly ToolFrame[], body: Rect (its solid box, for hit tests)
export type LadderFrame = { readonly x0: number; readonly y0: number; readonly x1: number; readonly y1: number; readonly rungs: number; readonly opacity: number };
export type ParticleFrame = { readonly species: Slug; readonly emitter: Slug; readonly x: number; readonly y: number; readonly scale: number; readonly rotation: Turns; readonly opacity: number };
export type LiftFrame = { readonly fixture: string; readonly dx: number; readonly dy: number; readonly tilt: Turns; readonly opacity: number };
// Frame gains: ladders, particles, lifts, held: Slug | null (the shell shows the grabbing cursor and captures the pointer)
```

## 22. React target, quiz and site

- **Depiction** (`PR/🔨️modules/🖌️depiction`, new `🧰️gear`, `✨️effects`): whole-drawing tilt about the pivot; state tint by CSSOM custom properties; tools drawn domain-neutrally (canopy with cords — the species' `canopy` shape or the default —, rope and hook, gun, ladder with rungs) in ink, paper and the species' accent; particles from a pool of elements created once per emitter (no `id`, no `<defs>`/`<use>`: REACT §3 pins that); nothing written when nothing changed.
- **Grasp** (`PR/🔨️modules/🤏️grasp`): window-level capture listeners for press, move, release, cancel, Escape; a press counts for the pets only when the event target has no interactive ancestor (the host's `controls` selector) and the point lies in a visible pet's body of the last frame; then the shell swallows that press (so text selection and the element beneath do not also act), captures the pointer on the document element, shows the grab cursor, and sends `pressed`/`dragged`/`released`. Touch hit pads as in §17. `stirred` on any learner input.
- **Lifting** (`PR/🔨️modules/🪞️lifting`): the copies of §20 and the reclaim listeners.
- **Survey**: walls, fixtures, `over` for the pointer; `transitionend`/mutation watchers ignore what the layer itself causes.
- **Layer**: props `play`, `mischief`, `controls`, `props` selector; an imperative handle or callback for `played` (the settings group); `onCast` unchanged; survey cadence stays tied to the rate, but a held pet does not force surveys.
- **Stories gallery**: controls to force a footing, an activity, a state, a trick, a mood; a "hand" to drag; fixtures to push; a readout of poofs and the invariant.
- **Quiz** (`QR/🔨️modules/🐾️pets`, preferences, i18n): `data-pet-prop` on quiz-page task rows, run items (never table rows), results rows, home quiz cards (key = quiz id; cards are never lifted, only matched for topic); `petsPlay`, `petsMischief`; the "Play with the pets" group (names of the pets on stage, four deeds each, a polite status line for explicit deeds only); the run stays quiet as in §14.4. Everything in English and German (du).
- **Site** (`AP`): every species gains states, tricks, purr, emitters, gear, grip, reach, canopy and the new clips (`hang`, `tumble`, `glide`, `aim`, `reel`, `climb`, `mantle`, `slide`, `carry`, `purr`, `dizzy`, `shrug`, `scoot`, `push`, tricks; floaters skip the climbing set); the ensemble gains `chemistry`. CONTENT is the brief. `S/🧪️tests/🐾️pet-cast` checks the new data (every trick's clip and emitter exist, every state ladder is reachable, every chemistry trait names existing species, states and tricks, every species with `climb` has the climbing clips …). `S/🧪️tests/🐕️pet-walk` proves in a browser: click hello → trick → purr; drag, hang, drop, parachute; no two pets ever overlap during a lively minute with drags; a pet climbs a card wall; a ladder and a rope are used; circling changes a state; a lifted copy appears beside its transparent original and is restored at once on hover and on focus; a control under a pet still receives its click.

## 23. Work packages (second round)

Phase A (parallel): **A1** split of the stage (TypeScript + Rust, behaviour-free) · **A2** contract: schema twins, validation, sample menagerie, behaviour tables for the new activities, registrations · **A3** `🪢️swing` + trigonometry additions · **A4** `🧗️climbing` + terrain additions · **A5** `🚧️clearance` (pure part) · **A6** `👆️gesture` · **A7** `💗️feeling` · **A8** `✨️effects` + `🪄️mischief` (pure parts) · **A9** React depiction of gear, tilt, tint, particles + preview tool.
Phase B (after A1, A2 and the pure modules): **B1** body: footings, clearance in the stage, hand (held, thrown, chute, head) · **B2** mind: moods, states, tricks, clicks, gestures, chemistry in the stage · **B3** projection: frame, effects, tools · then **B4** gear: wall, ladder, rope routes · **B5** mischief in the stage.
Phase C (as their inputs land): **C1** grasp, lifting, survey and layer · **C2** stories gallery · **C3** quiz · **C4** site, chemistry data, tests, e2e · **H1…H4** art for twenty species.
Phase D: Rust twins for everything, parity, long-run and fuzz proofs.
Phase E: audits (Sonnet), tuning in the real app, fixes, the coordinator's own verification.

Rules for everyone: those of `📓️design.md` §12, plus (WORK §6): the owner may commit the whole tree at any moment, so it must compile and its suites must pass at every save; never run `bun install`; `NX_PLUGIN_NO_TIMEOUTS=true` for `bun nx`; private stacks only on the ports named in your brief with `TEACHING_ARCHITECTURE_QUIZ_WATCH=off`; another ticket (`QUIZ-ADAPTIVE-LAYOUT`) is rewriting quiz views and CSS — anchor on `data-*` hooks, keep quiz edits minimal; report in `TK/📓️report2-<package>.md`; tool output in `TK/🗑️generated/<package>/`.

## 24. As built

Where the code differs from §14–§23, with the reason in a line and the work package that decided it (details in its `📓️report2-<package>.md`).

### 24.1 Projection (B3)

| Design | As built | Why |
|---|---|---|
| §19 a state "can loop an overlay clip" | a state's clip overrides the idle loop **per (bone, channel)** it keys (sampled on the stage clock like the idle loop); every other channel keeps breathing; activity and trick clips layer on top as before; replacing clips (gaits, flights, landing, sleep) lay only the idle loop to rest, never the look | a look that stops a rotor must hold it, a look that only adds a prop must keep breathing (coordinator, H1–H4) |
| — | a new state's look blends in from the former one over `STATE_BLEND` = 16 ticks, channel by channel; the frame's `state` (the tint) switches in the middle of the blend; `Actor.former` (all three twins, set by `enter`) names the state blended from | state changes did not pop only when they came from `lasts` (coordinator, H3) |
| — | a landing begins with the idle loop at rest (where the flight left it) and hands it back only as it ends | the body jumped between a flight and its landing (coordinator, H3) |
| §19 face: bend, lid, slant, drop | bend = `spirits`; the mood's lid height is combined with the blink in `EyeFrame.lid` (`lid + (1 − lid) × blink`, 1 asleep); the drop sinks the bone of the first eye in the pose; the slant is no frame number — a target computes it as `faceOf({mood, intensity}).slant` from the frame's `mood` and `intensity` | the core owns poses and lids; one face table, no second copy in the frame |
| §21 `ActorFrame.x/y` = feet | `x`, `y` = where the rig's origin is placed before the drawing turns by `tilt` about `pivot`; the turn carries the rig's feet onto the actor's feet (`x, y` are the feet while `tilt` is 0) | A9's painter turns the placed rig about the pivot; the stage tilts bodies about the scruff (B1) |
| §21 `pivot` | the scruff `{0, −grip}`; the hands `{0.3 w, −0.65 h}` on a rope | the rope leaves the pivot (A9); the tilt is B1's `Actor.tilt` about the scruff |
| §16 wall | a climber leans `WALL_LEAN` = 0.04 turns towards its wall in the drawing only (given up over the mantle) | the clearance body stays upright; B4 may move it into `Actor.tilt` |
| §21 `body` | the clearance body of `📏️spacing.extentOf` (size box, hover, tilt, canopy, margin 4) | the same box the stage keeps apart (B1) |
| §21 frame | plus `Frame.puffs: PuffFrame[]` `{x, y, width, height, phase}` — the dust of a poof (§18) | B1 keeps `Stage.puffs`; a target needs it to draw the dust |
| §19 particles "capped at 160" | all plumes of all actors in drawing order, the youngest 160 (`capped`); opacity × the actor's opacity; no `depth`: every particle is drawn in front of the actors | A9's painter has one particle root, the layer's last child |
| §22 still | a still stage shows the look of each state at rest, the resting face, open eyes, no tilt, tools, particles, lifts, dust or held pet, rate 0, no wake | frames of a still stage must not change with time |

### 24.2 Dust, state attributes and the gallery (C2)

| Design | As built | Why |
|---|---|---|
| §18 "a dust burst in its place" | `✨️effects.paintPuffs` draws `Frame.puffs` in the effects root, behind every particle: per puff a pooled cloud (at most `PUFF_CLOUDS` = 8 at once) of a middle blob and `PUFF_RING` = 7 blobs on the ellipse of the body's half-size, spreading to the outline, shrinking to half, turning ¹⁄₂₀ turn, rising `PUFF_RISE` = ¼ height, opacity `1 − phase²`; ink outline under paper fill (`puffShape`); the layer's `Scenery.stage` takes `puffs` | a puff belongs to no species, so it is domain-neutral ink and paper like the tools (§22) |
| §22 depiction | `<svg class="pet">` carries `data-pet-footing`, `data-pet-state` (absent for a state the species does not have) and `data-pet-mood` beside `data-pet-activity`, each written only on change; `depictionMarkup` states them | tests and tools (C4's `🐕️pet-walk`, the gallery's check) read what a pet is doing without a frame |
| §22 stories "controls to force a footing, an activity, a state, a trick, a mood" | state, trick, mood and standing activities are folded into the stage by the stage's own draft functions (`enter`, `perform`, feeling, `shift`/`hail`/`purr`/`shrug`/`settle`) through a director that only the gallery's dev server puts between the layer and the core (`pets-stories:director`); footings are reached the learner's way — the gallery presses, carries and lets go of a pet with a pointer of its own (hand, air, chute, head) or asks for `toss`; wall, ladder and rope come only with the pets' own routes (B4) | no backdoor in the product: the layer runs unchanged code, the seam exists only on the dev server |

### 24.3 Mischief in the stage (B5)

| Design | As built | Why |
|---|---|---|
| §20 "the learner has not stirred for 12 s (in a run: 30 s)" | the learner's last stir is the latest of `stirred` (press, key, wheel, input), the last move of the pointer (`pointed`, `dragged`, `released`) and the last scroll | the shell sends `stirred` for inputs only; a learner who moves the pointer over the page is not idle |
| §20 "the pointer is fine" | the layer permits mischief to the stage only while `(pointer: fine)` matches (`PET_FINE_POINTER`; it tells the stage again when that changes and gives every lift back when it stops); the stage's gate of the pointer stands open | no event carries the device's pointer, and a finger cannot take a copy back by pointing at it |
| §20 "calm at most every few minutes, lively more often" | A8's cooldowns (calm 3 min, lively 45 s) count from the end of the last prank — and from the opening of the stage (`rested` 0), so the first prank of a session waits a cooldown too; a try that finds nobody able to go waits a cooldown as well | pets settle in before they play; a failed try does not repeat every tick |
| §20 "it goes there with its gear (typically up the card's wall in the page gutter)" | the post is the station (`stationFor`) with the most room for the copy (at most the pet's width), then the nearest: any perch beside the element, or — for a climber — the wall stretch beside it, its feet at the element's lower edge held between the rim and the foot of the stretch. The pet walks there along its perch — only when that walk is clear: it never passes another on its own perch — or climbs there (a B4 trip: over the rim, or up from a perch beside the wall), needing the grip for the lift only — afterwards it moves on from its wall, slides down and lets go where the wall ends (parachute rule). Where no such way is clear it slips over in a puff: dust where it stood (not a counted poof), see-through at its post at once, fading in while the copy appears | on the real quiz page (1440 × 900) the cards are packed (no card top carries a perch for the cast's tallest), every pet lives on the footer some 450 px below the task rows, and neither a ladder (≤ 3.6 heights) nor a rope (≤ 3.2) reaches; without the slip mischief would never be seen there |
| §20 "one lifted fixture at a time … Rule 7" | the pick: among the prospects (idle, whole pets on a perch whose grounds cover a key, each with its post; none while the mode's movers are out) word 0 of `[seed, MISCHIEF_STREAM = 0xfffffffa, tick, 0]` points at one (`chosenFixture`), the next are tried round and round when it cannot go; word 1 is the copy's travel unit; `[…, tick, 1]` the throw | one stream of its own (no other draw moves); only a fixture's key and box are read (a Proxy test shows it) |
| §20 lift | `Stage.lift` (`Prank`) holds the prank from the pick to its end: `since` ahead while the pet is on its way; the pusher `push`es from `since` until the tick after `liftEnds` (the stage ends the push itself at `liftEnds`, then the pet is playful, `tricked`) | — |
| §20 reclaim → thrown off, "a startled then sheepish mood" | the copy is gone at once (`lift.since` moved past the copy's end), the pusher is thrown (`thrownOff`, `tumble`, B1's parachute rule) and frightened (`evicted`, scared 0.4); once it is down and its fright has calmed it feels an impulse of sad 0.3 (`SHEEPISH`), and only then is the prank over | the nine shared moods have no sheepishness; a lower mood cannot replace a fright while it holds |
| §20 "Also on hidden, pause, unmount, still, scene change, and when the fixture's box changes" | the stage ends a prank quietly (the pusher stops; not thrown) when a survey no longer has its fixture or has it more than 0.5 px elsewhere, when mischief is no longer permitted, and when the stage turns still; a pusher that stops while the copy is out (picked up, its wall gone, summoned away) leaves the copy to vanish at once on its way out and to slide home by itself once it rests. The layer folds the give-backs since its last step into the stage right after the ticks, before the survey of that step: a survey has no fixture under the pointer or the focus, so a survey told first would end every reclaim by hover or focus quietly (seen in the browser: the pusher climbed on instead of tumbling). A give-back of a fixture whose box a survey found moved comes after that survey and ends quietly; since the shell gives back before it tells the stage why (hidden, scene, mischief withdrawn, still), those end as reclaims | the stage cannot tell a give-back's reason; the shell's order says what happened first |
| §15.1 activity graph | `climb → push`; `push →` idle, walk, fall, hang, tumble, climb, mantle, slide, scoot | a pusher on a wall comes from its resting climb and leaves by its wall; thrown, it tumbles |
| §16 wall pose | a pusher on a wall shows its push clip on the idle loop, leaning `WALL_LEAN` towards its wall (B3's tilt), not the paused climb | its push clip is what it does |

### 24.4 Gear on the real pages, attention and courtesy (F1)

| Design | As built | Why |
|---|---|---|
| §16 wall lean (24.1: drawing only) | the lean is part of the body: `📏️spacing.wallLeanOf` (full `WALL_LEAN` at the cling, none half a width past it, so it is given up over the mantle) turns the size box in `extentOf`, in the claims of trips (`bodyAt`) and in the projection's `tiltOf` | the drawing of a climber poked out of its upright body; the panning fuzz found drawn overlaps (`TK/stage_fuzz.ts --pan`) |
| §16 ladder "against a higher edge" | also `ladderTo`: a ladder against the lower end of a wall, from whose exit the climber takes hold of the wall (a leg ladder → wall, planned and claimed whole); `LADDER_TALL` 8 heights (a telescopic ladder); a ladder leans on its wall, not on what lies on top (it stands on when the perch on the rim goes); keep-outs that begin within `WALL_LIP` on the element's side of the wall are that element's body and not in a ladder's way | on the quiz pages at 1440 × 900 the lowest card wall ends some 400 px above the footer, the only perch; the survey grows card bodies past their sides |
| §16 rope | the hook bites the corner of an edge (`HOOK_INSET` inside its ends), `ROPE_LONG` 8 heights; a slanted rope whose swing would carry the pet into a keep-out under the hook is hauled in straight; a shooter whose line the element under the edge blocks steps back a width at a time (`aims`) | on the home overview every perch on a card has the card under it: ropes from the footer were blocked, or swung into the card and below the footer |
| §16 routes | two legs planned whole when one does not reach (`outingsTo`: to a perch on the way, then on); the trip's footholds name their perch and their facing | a pusher on the footer reaches a task row's station only by a ladder and then the wall |
| §16 "lively often" | an explorer: in a lively stage a walk becomes a trip to a wall rest or to a higher perch its gear reaches `CLIMB_SHARE` of the time | gear was seen in the browser only by chance |
| §17 see-through | no see-through under a resting pointer (the pet perks); a pet in the air, on a wall, a ladder or a rope whose box (inset by `WALL_LIP`) lies over a keep-out is shown at 0.35 (`presenceOf`) | climbers and rope-haulers pass in front of cards like fallers |
| §17 circling | while the pointer circles a pet (its last quarter turn no older than `CIRCLE_SLOW`) it starts no walk, hop, gear trip or encounter of its own | a circle holds its attention |
| §18 poofs "counted" | every puff counts (`dusted`): the last-resort poof, a fall through the bottom edge, a mischief slip-over — which is now the last resort after every prospect was tried by its gear | the audit found two of the three uncounted |
| §16 bonk | a hard bump against an edge of the stage leaves the pet dizzy once it lands | B4's open item |
