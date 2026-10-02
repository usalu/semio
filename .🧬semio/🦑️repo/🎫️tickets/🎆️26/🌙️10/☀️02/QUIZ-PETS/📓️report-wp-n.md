# 📓️ Report — work package N (tuning the pets in the real quiz layouts)

Ticket `2026/10/02/QUIZ-PETS`, written 2026-10-02. Paths: `P = 🧰️framework/🛍️products/🐾️pets`, `PR = P/🎯️targets/⚛️react`,
`Q = 🧰️framework/🛍️products/❓️quiz`, `QR = Q/🎯️targets/⚛️react`, `S = 🎓️teaching/🏛️architecture/❓️quiz`, `TK` = this ticket
folder, `EV = TK/🗑️generated/wp-n/final` (the evidence; 34 MB with `gallery/` and the logs beside it).

**State: the six defects are fixed in the TypeScript core, the React target and the quiz glue, judged by eye on a private
stack at 1440 × 900 and 375 × 812, light and dark, calm and lively. Every TypeScript suite is green. The Rust twins are
behind by everything in §3: parity and `cargo test` of the pets crate fail until they are re-synced (not run by me).**

## 1. The defects

### 1.1 Pets hover above the cards

- **Root cause.** The quiz named the whole `section[data-card]` as a surface. Its box starts with the cap row; only the
  title tab (72 × 26 px on "Heating") is drawn in that row, the body edge lies one tab height lower. A second cause sat
  behind it: the first text line of a body starts at the body's top, and its keep-out margin (4 px) reached over the edge.
- **Fix (domain-neutral, `PR/🔨️modules/📡️survey`).** A host names every part of a silhouette as a surface; the survey makes
  that work with three rules: (a) the visible box of every surface element is a keep-out of its own, grown 4 px sideways
  only — pets stand on things, never in front of them, so the body edge is free only beside the tab; (b) the margin of a
  keep-out never reaches over an edge the element lies under; (c) everything is cut down to what its clipping ancestors
  show before it counts. **Quiz (`QR/🔨️modules/🐾️pets`).** `QUIZ_PET_SURFACES` = the title tab
  (`[data-slot="window-chrome-chip-cap"]`), the body (`[data-slot="window-chrome-body-surface"]`) of every card in
  `#quiz-main`, and the footer line (`.quiz-app > footer`, which thereby is the floor: the stage floor behind it is cut
  away). `QUIZ_PET_KEEPOUTS` gained `.quiz-app > header`: its logo and title are neither controls nor text blocks, and a
  pet on the first card of a phone stood on the logo.
- **Evidence.** `EV/desktop-light-calm-01-home-30.png` (housy on the Leaderboard tab, waly on the Cooling tab, battery on
  the Settings tab, windowy on the body edge of Badges, cloudy hovering over "How it works"), `…-01-home-zoom.png`,
  `EV/desktop-dark-lively-01-home-*.png`, `EV/slow-hop-air-1-radiatory.png` (a hop from the Settings tab down onto the body
  edge, 16 frames 31 ms apart). Measured on every sample of every run: feet exactly on a surveyed edge (floaters their
  hover above it). The spec asserts it (§4).

### 1.2 Pile-up

- **Root cause.** Nothing in the stage kept actors apart except at arrival, and arrival fell back to "unspaced" when no
  spaced place existed; a perch that shrank clamped everybody into what was left; landings ignored who stood there.
- **Fix (`P/🔨️modules/🎪️stage`).** Arrival only on a place 8 px (bodies) from everybody, else the species waits off stage
  and is tried again every whole second; a newcomer also waits until whoever it replaces is gone (the stage never holds
  more actors than were summoned). After every ride a perch holds only as many as fit with 8 px per neighbour pair: the
  one its perch pulled farthest is crowded out — it fades where it stood — and the others are set apart with the least
  displacement. Landings come down beside whoever stands there, or are crowded out. Walk goals lie on the clear way
  between the neighbours; a walker stops 6 px before a neighbour; encounters only between neighbours with nobody between.
  A crowd also thins out by itself: the weight of a hop grows with the company on the surface, and where no hop is in
  reach the pet wanders — walks off, fades, arrives anew on a perch with room.
- **E's open points.** Hop gaits walk in whole hops and finish a hop on the spot when blocked
  (`EV/slow-hopgait-walk-1-windowy.png`). Turning is eased: eight ticks squeezed through a line before a walk, in flight
  for a hop, towards the partner in an encounter (`EV/slow-turn-2-battery.png`, 12 frames 16 ms apart).
- **Evidence.** `EV/desktop-light-calm-06…08-run-*.png`, `…-09-results-physics-*.png`, `EV/desktop-dark-lively-03-run-cooling-0-*.png`:
  two pets left, two right of the task on the footer line, the others off stage. No sample of any run has two pets in
  each other (the only "overlaps" my measure ever flagged were encounter partners 2 px apart; the gap is 6 px now).

### 1.3 Phone has no pets

- **Root cause.** As J measured: keep-outs were not clipped by their scrolling ancestors. Two more: the stage worked in
  pixels while pets are drawn at 0.8, so headroom and widths were 25 % too large; and a pet on the only free card stood on
  the navigation bar (1.1).
- **Fix.** Survey rule (c); the layer (`PR/🔨️modules/🫧️layer`) hands the stage everything divided by the drawing size and
  multiplies the feet back (`staged`); a finger that is lifted sends `unpointed` (no hover on touch).
- **Evidence.** `EV/phone-light-calm-*`, `EV/phone-dark-lively-*`: identity 2 pets on the Settings card edge, home 2 (card
  edge, footer line), quiz page 2, board and settings 2 on the footer line, a run 0–2 on the task card's edge between the
  two footer tabs, never more than two, never on a control. No pets on the introduction, the badges page, a matching run
  and most results: the card fills the phone and nothing is left to stand on — the correct outcome.

### 1.4 Dense screens

Checked on desktop and phone: quiz page, run (sorting, classification, matching), results, leaderboard, badges, settings,
introduction page, learner page. 0 samples with a standing pet over text or a control, 0 stacked, nobody moves during a
run. Two more fixes came out of it: surfaces end 6 px inside the layer (`PET_EDGE_INSET`; a pet at the window edge was cut
off), and a hop is only offered when its arc touches no keep-out above both of its ends (a hop on a top-row card flew
through "All answers saved" in the navigation bar).

### 1.5 Pointer courtesy

`presenceOf` in the stage: while the pointer is inside the box of an actor (grown by 4 px) its opacity eases to 0.35
(1/32 per tick, 0.33 s) and back to 1 (1/16 per tick); on a still stage `frameOf` shows 0.35 at once, and the layer wakes
a still stage only for a pointer on or beside a pet. `EV/desktop-pointer-01-home-pointer.png`; asserted in the spec.

### 1.6 Stories port

`pets-stories` is **6069** (in no launch file before): `PR/📦️packages/🟦️typescript/📜️script.ts`,
`PR/🏗️builder/🌐️vite/🟦️.ts`, `.claude/launch.json`, `.vscode/🧩️launch.seed.jsonc`; `.vscode/launch.json` regenerated
(`plugin-registry:generate` exit 0, `check-generated` exit 0). The gallery answers 200 on 6069 without any environment.
`📓️design.md` §7/§11 still say 6071/6072.

## 2. Files

| File | Change |
|---|---|
| `P/🔨️modules/🎪️stage/🟦️.ts`, its unit suite | everything of 1.2, 1.4, 1.5; 20 new tests, each played on both companies (476 tests in the package, was 435) |
| `P/🔨️modules/🧠️behavior/🟦️.ts`, its unit suite | `Situation.crowd`; hop weight × (1 + crowd); calm hop 0.06 → 0.12; 1 new test |
| `PR/🔨️modules/📡️survey/🟦️.ts`, `PR/🔨️modules/🫧️layer/🟦️.tsx`, suites `📡️surface-survey`, `🫥️decorative-layer` | 1.1, 1.3, 1.4, 1.5 (77 tests, was 72) |
| `QR/🔨️modules/🐾️pets/🟦️.tsx`, `Q/🧪️tests/🐾️pet-companions/🟦️.tsx` | the two selectors; the suite checks them on the real card and the real client |
| `S/🧪️tests/🐕️pet-walk/🟦️.ts` | three new tests (edges, distance, beside a task; see-through; phone) |
| `P/🧪️tests/🧠️behavior-choice/{🐍️.py,🟦️.ts,🥒️.feature}`, `P/🧪️tests/🎪️stage-trace/{🐍️.py,🟦️.ts,🥒️.feature}` | the oracle restates the new weights; new law `apart`; three new scripts (`crowded-strip`, `tab-and-footer`, `pointer-rest`) |
| `P/🧫️fixtures/{🧠️behavior-choice,🎪️stage-trace,📡️surface-survey}/🔣️.json` | regenerated; a second run of each generator reports "unchanged" |
| `TK/generate_behavior_vectors.py`, `TK/generate_survey_vectors.py` | the new situations, scripts and survey rules (14 scenes) |
| outside my list, small anchored edits: `PR/🟦️.tsx` (export `PET_EDGE_INSET`), `PR/🏗️builder/🌐️vite/🟦️.ts` (port), `Q/README.md` (the stage sentence was false) | |
| new tools: `TK/wp_n_drive.mjs` (tour with series, bursts, probe), `TK/wp_n_slowmo.mjs` (page clock taken over, one tick per step), `TK/wp_n_gallery.mjs`, `TK/wp_n_stack.sh`; `TK/wp_j_private_stack.ts` takes `WP_STACK_SCRATCH` | |

## 3. What the Rust twins must mirror

`🧠️behavior/🦀️.rs`: `Situation` gains `crowd`; `hop = limits.hop * energy * urge * (1 + crowd)`; `MODE_LIMITS.calm.hop = 0.12`.
Adapters: `🧪️tests/🧠️behavior-choice/🦀️.rs` must pass `crowd`; `🧪️tests/🎪️stage-trace/🦀️.rs` must compute the law `apart`
(feet of two actors with the same non-null `perch` at least half of both widths − 1/128 apart).

`🎪️stage/🦀️.rs` (the twin already has some of it; check each):

| Function | Change |
|---|---|
| constants | `MEET_GAP = 6` (was 2), `ARRIVAL_GAP` → `COMFORT_GAP = 8`, new `SHY_OPACITY 0.35`, `SHY_STEP 0.03125`, `SHY_REACH 4`, `TURN_TICKS 8`, `CONTACT 0.0078125` |
| `presenceOf` (new) | 0.35 when `|pointer.x − x| ≤ width/2 + 4` and `y − height − 4 ≤ pointer.y ≤ y + 4`, else 1 |
| `headingOf`, `turning`, `swivel`, `squeezeOf` (new) | heading: walk/hop → sign of `goal − x`; idle/greet/cuddle/squabble with a partner → towards it (`other.x >= x`), sulk → away; the facing follows 8 ticks after `since`; a walker does not stride while it turns and restarts `since` at the flip; frame: bones entries 0, 2, 4 of every matrix × `1 − 2·smoothstep((tick − since)/8)`, pupils mirrored by the drawn side, the walk clip weighs 0 while turning |
| `stroll`, `attend`, `sulk`, `meet`, `decide` (hop) | no longer set `facing` |
| `act` | fade: leaving as before; else towards `presenceOf` (+1/16 up, −1/32 down); then `swivel`; `stride` only when not turning |
| `shoulders`, `beatOf`, `paced` (new) | half of both widths; ticks of a hop clip for a hopping gait, else 1; walk goals of a hop gait snapped down to whole hops (`speed × beat ÷ 64`) |
| `hindered` (new), `stride` | on every beat: at the goal when `|goal − x| ≤ speed/64` (snap), else end when a neighbour ahead would be closer than shoulders + 6 − 1/128 after the beat; a hop gait hindered in mid-hop sets `goal = x` |
| `clearway` (new), `decide` | goal on the stretch between the neighbours (8 px; a walker counts up to its goal); `roam` = room on one side; `crowd` counted; `elsewhere` (another perch with room) makes `hops` true; a hop without a launch calls `leave` (wander) |
| `soars` (new), `hopsOf` | a launch only when the body touches no keep-out and stays below the stage top while the feet are above both ends; landing place 8 px from others |
| `vacancy`, `crowdOut`, `vanish` (new), `touch` | landing beside whoever stands there, crowded out when farther than its own width or full; `ground` and `rehome` are gone (falling out of the stage and still-mode orphans vanish and arrive anew) |
| `carry`, `ride`, `seat` (new) | `carry` answers the strain (−1 when gone); `seat` evicts by strain while the perch is over its comfortable capacity, restores place and opacity of the evicted, then two passes with `clamp(place difference, shoulders, shoulders + 8)`, written as differences so an unchanged survey changes no bit |
| `roomsFor`, `arrive`, `spawn` | always spaced, no fallback; `spawn` stops while `actors.len() >= wanted.len()` |
| `arrivalTick` (new), `lull`, `step`, `pass`, `frameOf` | next whole second while a wanted species of the menagerie is missing and perches exist (not still); a lull ends there, `step` spawns there, an empty stage steps there, `wake` includes it; `lull` and `paceOf` use `opacity != presence` and `turning` |
| `measure` | minimum perch = the widest (was 1.5 ×) |
| `parted` (new), `pair`, `reachable`, `approach` | no pair with somebody between; the approach is clamped to the clear way and paced |
| `leave`, `summon`, `freeze` | walk-off only when nobody stands in the way, paced; a leaver without a perch is not kept; `freeze` removes whoever is in the air |
| `frameOf` | on a still stage `opacity = min(actor.opacity, presence)` |

## 4. Commands and real results (final code)

| Command | Result |
|---|---|
| `bun ./📜️script.ts test` in `P/📦️packages/🟦️typescript` (also `quick`, `long`, `exhaustive`) | each: 8 files, **476 passed** |
| `bun ./📜️script.ts test` / `typecheck` in `PR/📦️packages/🟦️typescript` | 4 files, **77 passed** / exit 0 |
| `bun ./📜️script.ts test` / `typecheck` in `QR/📦️packages/🟦️typescript` | 18 files, **492 passed** / exit 0, 0 errors |
| `.venv/Scripts/python.exe TK/generate_survey_vectors.py` | `{"scenes": 14, "surfaces": 40, "keepouts": 68}` |
| `.venv/Scripts/python.exe TK/generate_behavior_vectors.py` (twice) | 11 scripts recorded, laws kept; second run: all three fixtures "unchanged" |
| `oracle exhaustive --owner … --case "🧠️behavior-choice"` / `subject … --implementation typescript` | `executed=9 passed=9 failed=0` / `executed=9 passed=9 failed=0` |
| the same for `"🤝️bond-dynamics"` | `executed=6 passed=6` / `executed=6 passed=6` |
| the same for `"🎪️stage-trace"` | oracle: not exercised (no-oracle decision) / subject: `executed=3 passed=3 failed=0` |
| `🐕️pet-walk` on the private dev stack (`TK/wp_j_playwright.config.ts`, `--repeat-each 3`) | **21 passed** (2.3 min); `📱️phone` spec: 1 passed |
| `plugin-registry:generate`, `:check-generated` | exit 0, exit 0 |

Not run: cargo, parity (it fails for `🧠️behavior-choice` and `🎪️stage-trace` until §3 is ported; terrain is untouched),
the e2e gate, the rehearsal topology, deploy-check. `bun ./📜️script.ts typecheck` of the site package failed once in
between with three errors in another ticket's files (`🧭️session`, `🪪️identity`), none in pets files; the quiz package's
own typecheck was clean in my last two runs.

## 5. What still looks off

1. **Desktop pages have pets only on the footer line** (quiz page, leaderboard, badges, settings, introduction, learner):
   the first card is too close to the navigation bar and every other card lies within one pet height of the card above.
   That is the rule working, but it is less life than on the home screen.
2. **A first-time learner** sees all six on the footer line for about 25 s in calm; they then wander up one at a time
   (4 of 6 on cards after 90 s, `EV/desktop-first-visit-03-home-fresh-30.png`). Wandering is a fade out and a fade in a
   second later, not a journey.
3. **Drawings reach beyond the box of their species** (kettly's handle, roofy's eaves, sunny's rays, flamy's base):
   beside a card or a tab they touch its border by a few pixels; encounter partners touch arms. A matter for the art.
4. On a phone run two pets may stand between the "Overview" and "Submit quiz" tabs: calm, on nothing, but close to them.
5. A pet whose card vanishes or scrolls away falls straight down in front of whatever is there for up to half a second.
6. Turning shows the pet as a line for one frame; hop arcs on top-row cards are refused for tall pets, so they hop less there.
7. The evidence series in `EV` were taken before the last rule (hop arcs); it only removes hops. `📓️design.md` is not
   updated for any of this (§5.1, §5.2, §6.1, §6.3, §7, §8.1).

## 6. Housekeeping

Started and stopped by me: the private stack (8921/6191, restarted after every change of the code), the gallery on 6074 (its launcher was killed by the
task time limit; I stopped the Vite process it left) and on 6069. When stopping the latter I ended every process whose
command line named `pets-react:dev`; only my two ports were listening when I checked, but I did not check other ports
first. No git command that modifies anything, no cargo, no formatter. The repo MCP server was down; no ticket was opened
or closed.
