# 📓️ Work package P1 (second round): the frame rate of the home overview while the pointer moves

Ticket `2026/10/02/QUIZ-PETS`, written 2026-10-03 (evening). `P` = `🧰️framework/🛍️products/🐾️pets`, `PR` =
`P/🎯️targets/⚛️react`, `S` = `🎓️teaching/🏛️architecture/❓️quiz`, `TK` = this ticket folder, `OUT` = `TK/🗑️generated/p1`.

**Status: done.** The 8–12 fps C4 saw is the panorama itself in Playwright's headless shell, which composites in
software: with the pets **off** the release build sweeps at 12.8 fps there, the GPU process's display compositor is busy
92–96 % of the sweep and the renderer's main thread only 23–41 %; pets change nothing measurable in that browser. On the
machine's graphics card (Chromium's new headless mode) the same overview holds the browser's frame clock (31.3 fps, which
an empty page also gets here) in every configuration, with no long task. The pets' own share is main-thread time per
frame: in the release build it was **+6.6 ms (calm) and +6.7 ms (lively) over pets off, now +3.2 and +2.8 ms**, and each
time the pointer entered or left a pet the whole page (≈ 2 950 elements) was restyled for 26–50 ms — **now never**.
Fixed in `📡️survey`, `🖌️depiction`, `🤏️grasp` and `🎨️.css`. Tests: React package 172/172, typecheck 0 errors, the site's
`pets` project 13 passed + 2 `fixme` (dev topology, private stack). No git command that modifies anything, no
`bun install`, no gate, no deploy check; only ports 6245 and 8945 were used and only the processes `p1_stack.sh` started
were stopped.

## 1. Method

**Stack** (`TK/p1_stack.sh`, a copy of C4's over `wp_j_private_stack.ts`): site 6245, proctor 8945, scratch
`PROCTOR_DATA` under `OUT/stack-<topology>`, `TEACHING_ARCHITECTURE_QUIZ_WATCH=off`, PIDs recorded and stopped by them.
`dev` = the site's dev server (`dev-site`) over the development proctor; `rehearsal` = the **release build**
(`PROCTOR_URL=http://127.0.0.1:8945 bun ./📜️script.ts build --outDir OUT/site-rehearsal --emptyOutDir`) served
statically with its real CSP over a production proctor that admits only 6245. Each build was made from the tree of its
moment (before: 15:44; after: 19:1x; final: 20:3x) — other tickets edited the quiz and the UI in between, so every
comparison below is **pets minus pets off of the same build**, never across builds alone.

**Browsers** (`TK/p1_gpu_probe.mjs`): `shell` = Playwright's `chromium-headless-shell` (ANGLE on SwiftShader — what the
site's Playwright projects and C4 used; an empty page runs at 60.3 fps); `gpu` = Chromium's new headless mode
(`channel: "chromium"`, ANGLE D3D11 on the NVIDIA RTX A2000; an empty page runs at **31.3 fps**, the headless frame clock
on this machine, so 32 fps is the ceiling there). 1440 × 900, DPR 1, `en-US`, an anonymous learner. Other agents' e2e
stacks (6061, 6161/6162, 8791, 8891/8892) ran on the machine the whole time: run-to-run spread is ±1.5 fps in the shell.

**Configurations:** the pets' choice written into the local store (`semio.quiz.<tenant>.preferences`:
`{ pets: off|still|calm|lively, petsChosen: true }`), reload, wait for the home strip and the first pet, 6 s to settle,
pointer parked at the start of the path.

**Sweep** (`TK/p1_measure.mjs`): 10 s of real input through CDP `Input.dispatchMouseEvent`, at most one move per 8 ms and
one per renderer acknowledgement, along a time-based Lissajous over the whole viewport
(x = 1440·(½ + 0.46 sin 2πt/3.7), y = 900·(½ + 0.4 sin(2πt/2.3 + 0.7))) — every run follows the same path whatever its
frame rate; it crosses gaps (pan) and cards (reveal/conceal), as a learner does.

**Instruments**, per configuration:
- *clean* runs (2 or 3, mode order reversed every other repetition): `requestAnimationFrame` counted in the page, frame
  gaps, `longtask` and `long-animation-frame` entries (scripts with their forced style and layout, by owner),
  `Performance.getMetrics` before/after → task, script, layout and style time per frame, layout and style counts;
- *traced* run (own context, 5 s): CDP tracing with `devtools.timeline`, `…timeline.stack`, `…timeline.frame`, main-thread
  self time by kind, forced `Layout`/`UpdateLayoutTree` inside scripts attributed to the owner of the top stack frame,
  GPU-process tasks, plus a sampled CPU profile (250 µs) → time inside any frame of the pets' modules;
- *counted* run (own context, 10 s): wrappers that count, by caller, `getBoundingClientRect`, `getComputedStyle`,
  `createTreeWalker`, `matches`, `requestAnimationFrame`, `setAttribute`, style writes, mutation/resize observer calls.
- Owners: in a development build every module under a `🐾️pets` folder (core, React target, the quiz glue, the
  menagerie); in a release build the chunk that holds `pet-layer`.
- Targeted probes: `p1_style_probe.mjs` (one kind of write per frame on still pets, traced: restyled elements, style,
  paint, pre-paint, layerize, GPU), `p1_write_probe.mjs` (what pets write per frame and how far each matrix moved),
  `p1_mutation_probe.mjs` (which page mutations reach a survey-like observer), `p1_hand_probe.mjs`, `p1_hand_check.mjs`,
  `p1_rules_probe.mjs` (the hand cursor), `p1_species_shape.ts` (groups and elements per species).

## 2. Before

Median fps of the clean runs and main-thread time per frame (task / script / layout / style, ms):

| build · browser | off | still | calm | lively |
|---|---|---|---|---|
| dev · shell | 10.6 · 43.2/35.4/0.7/3.0 | 10.4 · 44.2/36.1/0.7/2.9 | 10.6 · 52.0/40.0/0.8/5.9 | 11.0 · 43.9/32.7/0.7/5.3 |
| dev · gpu | 27.9 · 19.2/14.6/0.3/1.4 | 24.3 · 24.9/19.1/0.4/1.7 | 24.3 · 28.7/19.7/0.5/4.5 | 23.4 · 30.2/20.8/0.5/4.6 |
| release · shell | 12.8 · 13.9/7.8/0.5/1.9 | 11.4 · 17.4/9.7/0.5/2.1 | 11.9 · 20.6/11.0/0.6/4.4 | 12.3 · 18.9/9.9/0.6/4.0 |
| release · gpu | 31.9 · 9.1/4.4/0.3/1.2 | 32.0 · 10.1/5.0/0.3/1.3 | 31.8 · 15.7/6.7/0.5/4.2 | 31.8 · 15.8/6.6/0.4/4.6 |

Release · gpu: frame gaps p50 31.3 / p95 31.6 ms in every configuration, 0–1 long task per 10 s. Release · shell:
p50 75–92 ms, p95 117–142 ms.

**Where a shell frame goes** (dev traces, `TK/p1_trace_threads.mjs`, 5 s): GPU process `VizCompositorThread` busy
4 606 ms (pets off) and 4 821 ms (calm) of 5 015/5 040 ms; renderer main thread 2 616 / 3 251 ms; raster workers
< 0.3 s. The glass of the panorama — the `ui-veil` `backdrop-filter: blur() saturate()` over a strip of nine
viewport-sized live pages that moves with every frame of the pan — is composited in software there. Pets are not the
cause of 8–12 fps; that is the panorama in a browser without a graphics card.

**The pets' share** (release · gpu, traced and counted, per frame unless said):

| | still | calm | lively |
|---|---|---|---|
| main thread over pets off (clean runs) | +1.0 ms | +6.6 ms | +6.7 ms |
| style recalculation over off | +0.1 | +2.9 | +3.4 |
| script over off (clean) / inside the pets' modules (profile) | +0.6 / 0.60 | +2.2 / 1.17 | +2.2 / 1.33 |
| GPU-process tasks (off 1.1) | 1.1 | 3.3 | 3.3 |
| surveys per second · layout reads · computed styles · `matches` per second | 7 · 823 · 1 078 · 16 851 | 7 · 767 · 1 004 · 16 966 | 7 · 790 · 1 035 · 19 722 |
| `setAttribute` per second · pets' rAF per second | 0 · 7 | 4 032 · 30 | 4 003 · 31 |
| forced style/layout inside the pets' scripts (5 s) | 37, 10.3 ms | 36, 9.2 ms | 40, 10.5 ms |

## 3. Causes

1. **Writes into the drawings.** Six calm pets wrote 86.5 bone-group transforms per frame with the pointer resting and
   109 + 23 pupil coordinates while it swept (`p1_write_probe.mjs`); **72 % (resting) and 38 % (sweeping) of those
   matrices had moved by less than 0.002 in every linear entry and less than 0.05 px** — breathing below anything one
   can see. Every written group costs a style recalculation of two elements at ≈ 9–11 µs each on this page (the host's
   stylesheet makes every element expensive), any write repaints the pet and costs one layerization of the page per frame
   (≈ 1 ms, independent of how many writes) and a GPU raster (`p1_style_probe.mjs`: all groups of six pets written →
   288 elements restyled, 2.5–3.4 ms style, 1.0–1.4 ms layerize, 1.8–2.3 ms GPU per frame). The same writes through the
   CSSOM `transform` or the SVG DOM transform list cost the same; `will-change` on or off the pets made no difference.
   Each part had its own group even where consecutive parts follow one bone: 377 groups for the twenty species.
2. **The survey walked the pets.** The layer itself is neither inert nor hidden, so the walk over the page visited the
   ≈ 390 SVG elements of six pets and matched each against four selectors: ≈ 2 400 `matches` per survey, two thirds of
   the walk.
3. **The survey runs 7–9 times a second while the pointer sweeps, even for still pets.** Every mutation batch of the
   sweep holds visible mutations (`p1_mutation_probe.mjs`, pets off, 10 s: 90 callbacks, 2 973 records, half of them
   behind inert panes, the rest the children of `span[data-icon]` in every card and in the header being replaced —
   522 in the badges card alone); with the pointer at rest there are none. Filtering inert subtrees would not save one
   survey, so the cadence stayed and each survey got cheaper (§4). The churn is the page's (§7).
4. **The hand restyled the whole page.** `[data-pet-cursor="grab"] *` (`🎨️.css`): a browser restyles every element a
   rule could reach whenever an attribute the rule names changes — whatever the value — so every time the pointer
   entered or left a pet the document (≈ 2 950 elements with the nine live pages) was restyled: 26–29 ms per toggle
   (`p1_style_probe.mjs`, `cursor`/`mark`); on the overview 20 passes onto a pet and off cost 20 full restyles, 1 050 ms
   of style, the longest 51 ms (`p1_hand_check.mjs` while the grabbing rule still named `data-pet-cursor`, which
   restyles exactly as the old rule did). The page also asks for a cursor of its own on every element (the design system's SVG cursors), so a cursor
   inherited from the root alone would not show (`p1_hand_probe.mjs`: the footer and the strip under the pets compute
   `url(…default…svg), default`).
5. **Forced style at the start of a survey** (`stageBox` reads the layer's box first): 36–43 per 5 s, ≈ 0.25 ms each
   (0.06 ms per frame). It is the panorama's own pending style (its rAF wrote the strip and the veil earlier in the
   same frame) computed early, not extra work; avoiding it would mean reading after the frame and a frame of lag behind
   scrolling. Left as is.

Checked and not causes: the pets' pointer handlers read no layout (only `closest`); inside a step every read comes before
every write; one animation frame request per frame (30–32/s at 32 fps); observer callbacks 8–9/s, cheap; no lifted copy
on the home overview (`copies: 0`); no pets script in any long animation frame (none ≥ 5 ms).

## 4. What I changed

| File | Change |
|---|---|
| `PR/🔨️modules/📡️survey/🟦️.ts` | `seen(root, selectors, apart)`: never looks into the stage's own element (`frame`, the pet layer) — the walker rejects it at its root, the fast path filters it out —, and asks each element it walks once whether it matches any selector (one union `matches`) before asking which; the fixture loop's own frame check went (it is the rule now); docs of `survey` and `SurveyOptions` |
| `PR/🔨️modules/🖌️depiction/🟦️.ts` | one group per **run** of consecutive pieces (parts, eyes, mouth) on the same bone (`Piece`, `Run`, `runs`; `depict` and `depictionMarkup` share it): 377 → 261 groups, 854 → 738 elements for the twenty species (`p1_species_shape.ts`), 7 → 3 groups for the test rig; `paint` rewrites a bone's matrix, a pupil, a lid and the mouth only once the drawing moved by `PAINT_SHIFT` (0.05 px) or `PAINT_LINEAR` (0.002 of a matrix entry, a lid's openness) since it was **written** (`visibly`), so the drawing never strays further than that from the frame; numbers are still written with three decimals; place, tilt, opacity, tint and names unchanged |
| `PR/🔨️modules/🤏️grasp/🟦️.ts` | `grab` is shown on the one element under the pointer (inline `cursor: grab !important`, its own inline cursor and a `style` attribute it did not have given back exactly when the pointer leaves it, the pet or the page); `data-pet-cursor` stays the root's account of the hand (for tests and hosts) but no rule names it; new `PET_HELD` = `data-pet-held` on the root only while a pet is held |
| `PR/🎨️.css` | the hand: only `[data-pet-held], [data-pet-held] * { cursor: grabbing !important }` (changes at pick-up and let-go only); the unused class variants `.pet-cursor-grab(bing)` removed |
| `PR/🟦️.tsx` | barrel: `PAINT_LINEAR`, `PAINT_SHIFT`, `PET_HELD` |
| `P/README.md` | the depiction's write economy, the survey's blind spot (the layer), the hand's marks |
| `P/🧪️tests/📡️surface-survey/🟦️.tsx` | new: "never looks into the stage's own element, and asks every element it walks once whether it is anything at all" (both query paths, `matches` counted per element) |
| `P/🧪️tests/🖌️pet-depiction/🟦️.tsx` | groups per run (shapes, matrices, eyes, mouth, followers `[1, 1, 1, 0]`, a body move writes 2 groups instead of 6); new: "rewrites a bone, a pupil, a lid or the mouth only once the drawing moved by a step one could see…" |
| `P/🧪️tests/🤏️pet-handling/🟦️.tsx` | the cursor test asserts the inline mark, `PET_HELD` and that nothing is left behind; new: "shows grab on the one element under the pointer and gives that element's own cursor back exactly…" (also: no rule names `data-pet-cursor`) |
| `P/🧪️tests/🧰️gear-depiction/🟦️.tsx` | the stylesheet pin: one cursor rule, `data-pet-held`, nothing names `data-pet-cursor` |

Decisions: thresholds of 0.05 px / 0.002 (a twentieth of a pixel at a 25 px bone; three decimals were kept as the
precision of what is written); the survey's cadence kept (it is pinned by `🤏️pet-handling` "measures the page every eight
ticks while pets move", and the mutations that drive it are visible ones); no post-frame reading (§3.5); the hand writes
one CSSOM property on a page element as the lifting already does with originals (allowed under the CSP: no `style`
attribute is ever set), and keeps `data-pet-cursor` for C4's spec and hosts.

## 5. After

Release build, same method, pets minus pets off **of the same build** (main thread per frame, clean runs; style and GPU
from the same tables; the off baseline moved between builds with other tickets' changes):

| release · gpu | still | calm | lively |
|---|---|---|---|
| before (15:44 build): main thread over off | +1.0 ms | **+6.6 ms** | **+6.7 ms** |
| after survey + depiction (19:1x build) | +0.8 | +3.6 | +4.5 |
| final, with the hand (20:3x build) | +1.1 | **+3.2** | **+2.8** |
| style over off: before → final | +0.1 → +0.1 | +2.9 → +1.2 | +3.4 → +1.0 |
| inside the pets' modules (profile): before → final | 0.60 → 0.50 | 1.17 → 0.91 | 1.33 → 0.95 |
| `matches` per second: before → final | 16 851 → 5 049 | 16 966 → 5 342 | 19 722 → 5 353 |
| `setAttribute` per second: before → final | 0 → 0 | 4 032 → 2 158 | 4 003 → 2 099 |

Final release · gpu, absolute: off 32.0 fps · 7.4/3.7/0.2/1.0 ms; still 32.0 · 8.5/4.2/0.2/1.1; calm 31.9 ·
10.6/4.9/0.3/2.2; lively 31.9 · 10.2/4.8/0.3/2.0 (task/script/layout/style); frame gaps p50 31.3 / p95 31.6 ms, no long
task. GPU-process tasks stay ≈ 3.2–3.5 ms per frame with calm or lively pets (1.0 off): every frame still repaints some
pet.

Release · shell, final: off 12.3, still 11.9, calm 11.6, lively 11.8 fps (before 12.8 / 11.4 / 11.9 / 12.3; after the
first fixes 12.8 / 13.5 / 12.6 / 13.4) — the software compositor's ceiling, within the spread of the runs.

Development build (only the survey and depiction fixes; the hand came later), gpu: off 29.1, still 28.4, calm 27.2,
lively 26.6 fps (before 27.9 / 24.3 / 24.3 / 23.4); main thread over off: still +5.7 → +0.5, calm +9.5 → +5.7, lively
+11.0 → +5.1 ms per frame. Shell: 13.3 / 12.9 / 13.4 / 12.2 fps (before 10.6 / 10.4 / 10.6 / 11.0; the off baseline itself
moved with the tree).

**Writes per frame** (`p1_write_probe.mjs`, release, 6 pets, gpu): calm resting 86.5 → 39.0 transforms; calm sweeping
109.0 + 23.3 pupil → 52.4 + 12.3; lively sweeping 98.3 + 20.5 → 52.9 + 12.6.

**The hand** (`p1_hand_check.mjs`, dev, gpu, 20 passes onto a perched pet over the strip and off to the navbar): root
says `grab` on 10/10 passes, the element under the pointer (`div.grid`) computes `grab`, nothing is left on 10/10 passes
off; style recalculations: **calm 189, 222 ms in all, none of more than 1 000 elements (largest 351), longest 3.6 ms**;
the control with pets off at the same places: 146, 62 ms, largest 351, longest 2.9 ms. Before: 20 recalculations of
≈ 2 990 elements, 1 050 ms, longest 51 ms. `p1_rules_probe.mjs`: no loaded rule names `data-pet-cursor`.

**The site's `pets` project** (dev topology, final code): 13 passed, 2 skipped (`fixme` of B4 and B5), 84 s; the lively
minute sampled **777 frames** (C4's dev runs: 551–625) with 0 overlaps.

## 6. Commands and results

From the repository root unless a directory is named.

| Command | Result |
|---|---|
| `bash TK/p1_stack.sh restart dev` / `build` / `restart rehearsal` / `stop` | stack up on 6245/8945 each time; builds `✓ built in 14.08s`, `11.03s`, `6.27s`, exit 0; stopped by the recorded PIDs, both ports free afterwards |
| `node TK/p1_measure.mjs --url http://127.0.0.1:6245/ --label <before\|after\|final>-<dev\|release>-<shell\|gpu> --browser <shell\|gpu> --repeat <2\|3> --seconds 10 --probe [--dist OUT/site-rehearsal]` (10 runs) | the tables of §2 and §5 (`node TK/p1_table.mjs OUT/measure-*.jsonl` → `OUT/table-release.md`, `OUT/table-dev.md`) |
| `node TK/p1_gpu_probe.mjs` | shell: SwiftShader, empty page 60.3 fps; new headless: NVIDIA RTX A2000 D3D11, 31.3 fps |
| `node TK/p1_trace_threads.mjs <trace>` (dev, 5 s) | Viz compositor 4 606 / 4 821 ms busy (off / calm), main 2 616 / 3 251 ms |
| `node TK/p1_style_probe.mjs --browser gpu …` | §3.1 and §3.4 (restyled elements and costs per kind of write; `cursor` 2 957 elements, 25.9 ms; `mark` with the old rule 2 915, 29.0 ms; root-only rule 22, 0.6 ms) |
| `node TK/p1_write_probe.mjs --mode calm [--sweep]`, `--mode lively --sweep` | §3.1, §5 |
| `node TK/p1_mutation_probe.mjs --mode off\|still [--rest]` | §3.3; at rest 0 callbacks |
| `node TK/p1_hand_probe.mjs --rounds 6`, `node TK/p1_hand_check.mjs`, `node TK/p1_rules_probe.mjs` | §3.4, §5 |
| `bun TK/p1_species_shape.ts` | groups 377 → 261, elements 854 → 738 |
| `PR/📦️packages/🟦️typescript`: `bun ./📜️script.ts test` | exit 0, `Test Files 8 passed (8)`, `Tests 171 passed (171)` after my last change; again at the very end, with another package's concurrent edits of the layer and `🤏️pet-handling` in the tree: `Tests 172 passed (172)` |
| same: `bun ./📜️script.ts typecheck` | exit 0, 0 `error TS` (both times) |
| `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/📦️packages/🟦️typescript`: `bun ./📜️script.ts test pet-companions` | exit 0, `Tests 45 passed (45)` (after the survey and depiction changes) |
| `bash TK/p1_gate.sh dev final-1 4` (= `node node_modules/playwright/cli.js test --config S/🎭️e2e/🎚️config/🟦️.ts --project pets --no-deps --workers 4` against 6245/8945) | `13 passed (1.4m)`, `2 skipped`, exit 0, 84 s |
| `node TK/p1_code_rules.mjs` | every touched product file, suite, the stylesheet and the 13 tools clean (unique docstring emojis, no line comment, console or `[DEBUG]`, no forbidden write, no rule naming `data-pet-cursor`) |

## 7. What remains, and whose

**The panorama (UI `🥞️LayeredOverview`, the glass utilities, the quiz's home) — slow without pets:**
- In a browser that composites in software the glass is the frame rate: 12–13 fps with pets off (release), the display
  compositor 92–96 % busy. **The owner's Remote Desktop session** (the machine lists a "Microsoft Remote Display
  Adapter") may well render Chrome without its GPU — then this is what the owner sees, pets or not. Not verified in the
  owner's browser (`chrome://gpu` there would tell). Options are the UI's: a cheaper veil while the strip moves (no
  blur during the pan, or a blurred poster), a smaller blur radius, or no backdrop filter when compositing is in software.
- Forced synchronous layout by the page while the pointer moves (release, 5 s, pets off: 29 layouts and 45 style
  recalculations, ≈ 145 ms; dev names): the Navbar's `measure` (`UI/🧱️elements/🔝️Navbar/🟦️.tsx:139`, run from a
  `MutationObserver` on the navbar row, reads `getBoundingClientRect` at once) ≈ 66 + 47 ms per 5 s in dev; the quiz's
  presence `cursorAt` (`QR/🔨️modules/👥️presence:783`) ≈ 11 ms; the overview's own `onMove` (root box per
  `pointermove`) ≈ 8 ms.
- **Icon churn**: while the pointer sweeps, React re-renders the home cards and the header and every `span[data-icon]`
  replaces its children (≈ 150 child-list records and 9 mutation batches a second; none at rest). Every observer on the
  page pays: the UI's WindowChrome observer (≈ 1 600 calls per 10 s), the Navbar's `measure` above, and the pets' survey
  (7–9 a second).
- The development build's React costs about twice the release's main thread (dev numbers are for orientation only).

**The pets, not changed here:**
- Idle loops at rate 32 still rewrite ≈ 40–60 groups per frame and repaint the pets every frame: ≈ +1.2 ms style and
  ≈ +2.2 ms GPU work per frame for calm pets. Fewer frames would be the core's call (rates, B packages).
- The survey's cadence follows the page's mutations (§3.3), ≈ 0.5 ms per frame now; it gets cheaper when the page stops
  churning.
- Picking a pet up and letting it go still restyle the whole page once each (`data-pet-held` with `*`): two per drag,
  where the learner acts on purpose.

## 8. Files

Product: the ten files of §4. Ticket tools (kept): `TK/p1_stack.sh`, `TK/p1_gate.sh`, `TK/p1_measure.mjs`,
`TK/p1_table.mjs`, `TK/p1_trace_threads.mjs`, `TK/p1_trace_paint.mjs`, `TK/p1_style_probe.mjs`, `TK/p1_write_probe.mjs`,
`TK/p1_mutation_probe.mjs`, `TK/p1_gpu_probe.mjs`, `TK/p1_hand_probe.mjs`, `TK/p1_hand_check.mjs`,
`TK/p1_rules_probe.mjs`, `TK/p1_species_shape.ts`, `TK/p1_code_rules.mjs`.

## 9. Clean-up

Stopped: the stack, by the PIDs `p1_stack.sh` recorded (6245 and 8945 free). Deleted from `OUT`: both release builds,
the stack folders (proctor copies, scratch data, Vite cache), traces, Playwright results and every log; kept the two
tables `OUT/table-release.md` and `OUT/table-dev.md` (all runs, every column) and nothing else. The repo MCP was
unreachable in this session; no ticket bookkeeping from this package.
