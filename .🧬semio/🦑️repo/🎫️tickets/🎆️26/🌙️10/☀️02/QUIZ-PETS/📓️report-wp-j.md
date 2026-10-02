# 📓️ Report — work package J (the architecture menagerie and the site)

Ticket `2026/10/02/QUIZ-PETS`, written 2026-10-02 07:00 (host time). Paths: `AP = 🎓️teaching/🏛️architecture/🐾️pets`,
`S = 🎓️teaching/🏛️architecture/❓️quiz`, `P = 🧰️framework/🛍️products/🐾️pets`, `PR = P/🎯️targets/⚛️react`,
`QR = 🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`, `TK` = this ticket folder. Tool output is in
`TK/🗑️generated/wp-j/` (logs, `walk/` and `stories/` with the screenshots).

**State: everything of the brief is done and the site is wired.** The architecture site ships the twenty pets; they load
lazily, pass every desktop and phone spec of the end-to-end gate on private stacks in both topologies, and the release
artifact holds its budget. Four things I saw by eye are not right yet and are not mine to fix (§6): pets hover 26 px above
the visible card edge, they pile up in a corner during a tall run, a phone shows no pets at home, and one spec of another
ticket (`👥️shared-presence`) fails for a reason that has nothing to do with pets.

## 1. What exists

| File | Content |
|---|---|
| `AP/🔣️.json` | the ensemble `semio.pets.ensemble/v1`, id `architecture`, title "Architecture and Technology Pets" / "Tierchen zu Architektur und Technologie"; twenty species paths in roster order (the owner's nine first); the 67 bonds of `📓️explore-topic-pets.md` §4.1, strongest first; casts `home` (core = the nine seeds in the owner's order, rotation = the other eleven) and `physics`, `heating`, `cooling`, `demand` (core 3 + rotation 4, §5.2) |
| `AP/🟦️.ts` | static imports of the ensemble and the twenty species; `export const ARCHITECTURE_MENAGERIE: Menagerie = assembleMenagerie(ensemble, [...])` and the same as default export |
| `AP/README.md` | roster table (directory, thing en/de, size and gait, grounds in words), casts, all 67 bonds with their reason, how to add a pet, how to look at them, the two suites |
| `S/🧪️tests/🐾️pet-cast/🟦️.ts` | vitest, node, 80 tests (§3) |
| `S/🧪️tests/🐕️pet-walk/🟦️.ts` | Playwright, 4 tests with the shared `device` fixture (§3) |
| `TK/wp_j_roster.ts` | prints the roster from disk and every ground that does not exist in the quiz files |
| `TK/wp_j_private_stack.ts` | a private stack without cargo: a copy of the already built `proctor.exe` over scratch data; `dev` = proctor 8921 + dev site 6191, `rehearsal` = production-mode proctor 8922 + the private release build served on 6192 |
| `TK/wp_j_playwright.config.ts` | the gate's window, limits and fixture for any spec folder (`WP_J_SPECS`) against a stack that is already up; projects `desktop` and `phone` |
| `TK/wp_j_walk_site.mjs` | walks the real site as a first-time learner (introduction, identity, home, reload, quiz page, run), screenshot and measurement of every pet per stop, optional `--watch` |
| `TK/wp_j_probe_perches.mjs` | runs the layer's own `survey` and the core's `perchesOf` inside the live page and prints surfaces, perches and what blocks the floor |
| `TK/wp_j_card_dom_probe.mjs`, `TK/wp_j_gallery_roster.mjs`, `TK/wp_j_typecheck.tsconfig.json` | card boxes; gallery roster and every scene in the sandbox; tsc over my three TypeScript files |

The five species of package H2 were on disk when I started; I wrote none of them.

## 2. Shared files I edited (small anchored `Edit`s, each file re-read right before)

| File | What |
|---|---|
| `S/🟦️.ts` | `pets: () => import("../🐾️pets/🟦️.ts").then((module) => module.ARCHITECTURE_MENAGERIE)` in the `mountQuiz` call; one sentence and one `@see` in the docstring |
| `S/🧪️tests/🎚️config/🟦️.ts` | alias `@semio-tech/pets`; `🧪️tests/🐾️pet-cast/🟦️.ts` in `include` |
| `S/🎭️e2e/🎚️config/🟦️.ts` | `"🐕️pet-walk/🟦️.ts"` at the end of the `desktop` project's `testMatch` |
| `S/📦️packages/🟦️typescript/📋️project.json` | `namedInputs.default`: `AP/**/*` and `P/**/*` |
| `S/📦️packages/🟦️typescript/package.json` | dependency `@semio-tech/pets` (`workspace:*`); then `bun install` (exit 0, "no changes" to installs, lockfile saved) |
| `S/README.md` | row `🧪️tests/🐾️pet-cast` in the path table, row `🐕️pet-walk` in the spec table, section `### Pets` before `## Deploy` |
| `🎓️teaching/🏛️architecture/README.md` | row "Pets" in the parts table |
| `P/🔮️oracles/🔣️.json` | three entries after `pets-ajv-structure`: `pets-gl-matrix` (capabilities `pets-rig-solving`, `pets-turn-trigonometry`), `pets-d3-ease` (`pets-animation-sampling`), `pets-polygon-clipping` (`pets-terrain-walking`), each with `hostPath` = the core package and a rationale that states what the unit suite really compares (checked against the suites) |
| `.claude/launch.json`, `.vscode/🧩️launch.seed.jsonc` | `architecture-pets-stories` / `🛠️dev🎓️teaching🏛️architecture🐾️pets📖️stories`: port 6072 → **6074** (port, env, ready pattern) |
| `.vscode/launch.json` | regenerated, never hand-edited |
| `bun.lock` | by `bun install` |

Not edited: `S/🏗️builder/🌐️vite/🟦️.ts` (sibling I's two aliases are there, verified, not duplicated), `P/📦️packages/🟦️typescript/package.json`
(gl-matrix 3.4.3, d3-ease 3.0.1, `@types/d3-ease` 3.0.2 and polygon-clipping 0.15.7 were already devDependencies in exactly the
installed versions), anything under `QR` or `PR`, the taxonomy (all names were registered by A).

## 3. The two suites

**`🐾️pet-cast` (vitest).** ajv (third party) compiles `P/🧬️schema/🔣️.json` and accepts the ensemble, each of the twenty
species and the assembled menagerie; the product's `ensembleIssues`, `speciesIssues` and `menagerieIssues` report nothing;
a species with its bones removed is rejected by both (so neither is vacuous). The menagerie assembled from the documents on
disk equals `ARCHITECTURE_MENAGERIE` and the default export (the module lists what the ensemble lists, in its order). The
ensemble names exactly the species directories beside it, each called after its species; twenty ids, the owner's nine
first. Every ground exists in the quiz files (`<quiz>`, `<quiz>/<task>`, `<quiz>/<task>/<item>`) and every species has
one. Casts: `home` plus exactly one per quiz of the catalog; `home.core` is the nine seeds and `home` knows all twenty;
every member of a quiz's cast is grounded in that quiz; no species twice in a scene; the core of every quiz holds a bonded
pair. Bonds join existing, different species, each pair once, and nobody is without a bond.

**`🐕️pet-walk` (Playwright).**
1. *Home*: a pet within 10 s of the overview; `data-pets="calm"`; one `.pet-layer`, `aria-hidden="true"`, computed
   `pointer-events: none`, no focusable node, role, id, `style` or `script` inside; every pet belongs to the core of `home`;
   the drawing changes (they live); the element under the middle of every card, control and pet is never part of the
   layer; a real click on the leaderboard card opens its page and the overview button closes it; no horizontal overflow.
2. *Cast*: for every quiz of the catalog, opening its page leaves nobody of another cast on screen and brings its whole
   core; Escape brings the home cast back. In a run of the first quiz only that quiz's species remain and nobody changes
   place for three seconds.
3. *Preference* (the "Pets" / "Tierchen" group of the preferences page): `still` keeps the pets and three drawings a
   second apart are identical; `off` removes the layer and every pet, survives a reload; `lively` brings them back, moving.
4. *Reduced motion*: with `emulateMedia({ reducedMotion: "reduce" })` pets appear and are motionless (the preference
   still says `calm`), also while the pointer moves; without it they move again.
The `device` fixture fails each test on a console error, page error, failed request, answer ≥ 400 or policy violation.

## 4. Verification (commands and their real results)

| Command | Result |
|---|---|
| `bun TK/validate_species.ts AP/*/🔣️.json` (start and end of my work) | 20 × `ok`, exit 0 |
| `bun TK/wp_j_roster.ts` | twenty directories with emoji + `fe0f`, `missing=[]` for every species |
| `bun -e` import of `AP/🟦️.ts` | 20 species in roster order, 67 bonds, casts `home:9+11 physics:3+4 heating:3+4 cooling:3+4 demand:3+4`, `menagerieIssues` `[]`, `ensembleIssues` `[]` |
| `bun ./📜️script.ts test` in `S/📦️packages/🟦️typescript` | `Test Files 5 passed (5)`, `Tests 128 passed (128)` (120 at my first run; other tickets added tests meanwhile); `bun ./📜️script.ts test pet-cast`: `80 passed (80)` |
| `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @teaching/architecture-quiz:test` | exit 0, `5 passed`, `129 passed`, "Successfully ran target test" |
| `bun ./📜️script.ts typecheck` in the site package (a target another agent added while I worked; it includes my specs and, through the entry, `AP/🟦️.ts`) | exit 0, 0 errors; `tsc -p TK/wp_j_typecheck.tsconfig.json`: 0 errors |
| `bun ./📜️script.ts test` in `P/📦️packages/🟦️typescript` | `8 passed`, `435 passed` (the suites that import the three registered libraries) |
| ajv of `P/🔮️oracles/🔣️.json` against the harness schema | valid, nine unique oracle ids |
| `bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/🐾️pets"` (test harness) | exit 1 from 5 874 lines of breaches elsewhere; no line names a pets path or a `pets-` id |
| `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @semio-tech/plugin-registry:generate`, then `…:check-generated` | exit 0 / exit 0; the generated file carries 6074 for the pets row, 6072 only for the `space` playground |
| `bun ./📜️script.ts verify taxonomy report --scope "🎓️teaching"` | `clean=true errors=0 warnings=0` |
| `… --scope "🧰️framework/🛍️products/🐾️pets"` | `clean=true errors=0 warnings=0` |

### Release build

`NX_PLUGIN_NO_TIMEOUTS=true bun nx run @teaching/architecture-quiz:build` (ports 6161/6162 were free): exit 0, built in
14.6 s. Measured with `bun TK/site_bundle_weight.ts <dist>` and checked with the site's own `siteArtifactProblems`:

| File | Raw | gzip | Budget |
|---|---:|---:|---|
| entry script `assets/🌐️-AKyPfJIm.js` (linked by the document) | 753 176 B | **215 896 B** | 260 000 B, 44 104 B headroom |
| entry stylesheet `assets/🌐️-DMQ324mT.css` (linked) | 253 845 B | 38 083 B | 60 000 B |
| lazy: the menagerie `assets/🟦️-CPGQW6UM.js` | 292 258 B | 35 401 B | – |
| lazy: render target with the model `assets/🟦️-BMmNZBKa.js` | 45 187 B | 17 399 B | – |
| lazy: constants both share `assets/🟦️-Cse00HC5.js` | 194 B | 167 B | – |
| lazy: the pets stylesheet `assets/🟦️-BImWDBgO.css` | 904 B | 370 B | – |
| whole artifact (62 files) | **2 487 543 B** | | 4 000 000 B |

- `siteArtifactProblems(dist, "https://proctor.quizzes.architektur-und-technologie.de")` → `[]` (also `[]` for the private
  rehearsal builds). The `🧪️deploy` vitest spec passes.
- The menagerie is **not** in the entry: "Heizkörper" and `between:[` occur only in the lazy chunks; the document links
  only the two `🌐️-*` files; the entry holds `import("./🟦️-….js")` twice with Vite's dependency lists.
- The pets rules are in the built CSS: `.pet-layer{position:fixed;inset:0;overflow:hidden;pointer-events:none;contain:strict}`,
  every paint class and the forced-colours and print rules, in the lazy stylesheet that Vite links with the render target
  (`style-src 'self'`). No `[DEBUG]` in any built file.

### The real app (private stacks; the gate itself was not run)

The gate builds the proctor with cargo, which I must not run. I ran the gate's specs with the gate's fixture through
`TK/wp_j_playwright.config.ts` against `bun TK/wp_j_private_stack.ts dev` (`PLAYWRIGHT_BASE_URL=http://127.0.0.1:6191`,
topology `dev`) and `… rehearsal` (release build with `PROCTOR_URL=http://127.0.0.1:8922`, static origin 6192,
production-mode proctor that admits exactly that origin, the document's real Content-Security-Policy), 2 workers.

| Run | Result |
|---|---|
| `🐕️pet-walk`, dev | 4 passed (37.6 s); again with `--repeat-each 3`: 12 passed (1.8 min) |
| `🐕️pet-walk`, rehearsal | 4 passed (26.6 s); `--repeat-each 3`: 12 passed (1.3 min) |
| all desktop specs (`🪪️first-visit`, `🥞️layered-home`, `🎯️quiz-runs`, `🏆️live-leaderboard`, `🗣️both-languages`, `🐕️pet-walk`), dev | 18 passed (5.7 min) |
| the same, rehearsal | 18 passed (2.8 min) |
| `🚀️site-boot` | dev 1 passed, rehearsal 1 passed |
| `📱️phone` (375 × 812, touch) | dev 1 passed, rehearsal 1 passed |
| `👥️shared-presence` | **fails** on both stacks, also on a fresh proctor, at its line 75 (`[data-crowd-item]` expected 0, received 1) — see §6.5; not caused by pets |
| `🔌️connection-shortage` | not run (it needs the gate's control endpoint) |

Walks with `node TK/wp_j_walk_site.mjs` (1440 × 900 light and dark, 375 × 812 light and dark, the rehearsal build): no page
error, no failed request, no policy violation in any of them; in the rehearsal build no console message at all (the dev
walks log two console *warnings* "WebSocket is closed before the connection is established" at my reload, which the gate's
fixture tolerates). At every stop: 0 controls whose middle is covered by the layer, 0 overlaps of a pet's drawing with a
visible text block or control, no horizontal overflow. A 90 s watch of the home screen (calm, 180 samples): 0 samples with
a pet over text or a control; three pets walked.

## 5. What I saw

**Stories gallery** (`PETS_STORIES_PORT=6074 PETS_MENAGERIE=AP/🟦️.ts bun nx run @semio-tech/pets-react:dev`, probed with
F's `TK/stories_probe.mjs` and `TK/wp_j_gallery_roster.mjs`; 52 responses, none failed, no console error; the server and
only its process tree were stopped afterwards). The page is titled "Pet stories · Architecture and Technology Pets" (German:
"Tiergeschichten"), twenty stories in roster order, 468 `svg.pet`. `stories/roster-light.png`: all twenty render, each
recognisable at a glance — sun with rays, cloud, house with red roof and chimney, blue panel, red radiator with heat
shimmer, teal heat pump with fan, window, brick wall, green battery with charge bars, slim wind turbine, boiler with fire
window, grey roof, pink insulation, violet blind with sunglasses, lime ventilation unit, blue chiller with frost,
kettle, tea light, thermometer, server rack. `stories/architecture-species.png`: Sunny large at rest and with its ten clips.
Sandbox (`stories/sandbox-<scene>.png`, capacity 6, nine seconds per scene): `home` pumpy and radiatory side by side on
Card 1, waly on Card 4, sunny and windowy on the shelf, battery on the floor; `physics` solary, kettly, sunny, battery,
flamy, windy; `heating` waly, radiatory, windowy, housy, insuly, roofy; `cooling` sunny on Card 2 beside the note, shady on
Card 3, chilly on Card 1 beside the button, venty and servy on the shelf cards, cloudy on the floor; `demand` boily and
solary together on Card 1, housy, pumpy, venty, insuly. On the mock cards the pets stand exactly on the top edge.

**Site, desktop 1440 × 900** (`walk/desktop-{light,dark,watch}-*.png`, `walk/rehearsal-light-*.png`).
- *Introduction / identity*: first pet 0.5 to 1.9 s after the navigation starts (five walks). Six of the home core stand on the footer line
  (cloudy and sunny hover), well apart. The settings card beside the introduction shows "Pets: Off · Still · Calm · Lively"
  and under it "At home here:" with all twenty names. Nothing is covered.
- *Home, first visit*: the same six stay on the floor (they were placed there on the introduction, where the only perch
  is the floor); they breathe, blink and one walks a few steps.
- *Home, returning learner* (reload): the six appear on card edges and the floor — e.g. cloudy and pumpy above
  "Physical Understanding", housy above "Heating", radiatory above "Energy Demand", solary above "Badges", sunny above
  "Settings". They do not cover any text or control, **but they do not touch the cards** (§6.1).
- *Quiz page*: within about six seconds the home pets fade and the quiz's troupe stands on the floor under the cards
  (heating: housy, insuly, radiatory, thermy/roofy, waly, windowy; cooling: sunny, venty, chilly, servy, shady, thermy;
  physics: cloudy, sunny, battery, flamy, kettly, windy; demand: boily, housy, insuly, pumpy, radiatory, solary).
- *Run*: calm — in eight seconds nobody changes place; breathing and blinking go on. With a task that leaves the floor
  free (`walk/rehearsal-light-5-run.png`) six pets line the floor under the card. With a task that reaches the bottom of the
  window they **pile up in the bottom-left corner** (`walk/desktop-dark-5-run.png`, §6.2).
- *Dark theme*: outlines switch to cream, every pet stays legible.

**Site, phone 375 × 812** (`walk/phone-{light,dark}-*.png`): **no pet** on the introduction, the identity step, the home
list or in a run; two pets at scale 0.8 on the footer line of a quiz page (battery and kettly for physics, boily and pumpy
for demand). Nothing is covered and nothing overflows — but there is almost nothing to see (§6.3).

## 6. Open — findings that belong to other packages

1. **Pets hover above the cards (I and G).** `QUIZ_PET_SURFACES` is `#quiz-main [data-card]`. That section starts with the
   26 px cap row that holds only the title chip (`TK/wp_j_card_dom_probe.mjs`: section `6,390 399x122`, title chip
   `6,390 72x26`, visible body `[data-slot="window-chrome-body-surface"]` `6,416 399x73`). The perch is cut at the top of the
   section beside the chip (`TK/wp_j_probe_perches.mjs`: `quiz:heating … y 390 … perches=[79..405]`), so a pet stands 26 px
   above the edge one sees. Pointing the selector at the body surface is not enough: the first text line of a card starts at
   the body's top and its keep-out reaches 4 px above it (`KEEPOUT_MARGIN`), which blocks the whole edge. It needs either a
   surface whose keep-outs below the edge do not count, or the title chip as part of the silhouette.
2. **Pile-up during a run (E, with G/I).** When a task card reaches the bottom of the window the floor's only perch is the
   strip left of the card (`floor … perches=[0..146]` at 1440 px); all six actors are clamped into it and stand in each other
   (x = 42…124). Nothing in the stage spaces actors when a perch shrinks, and nobody leaves for lack of room.
3. **No pets on a phone (G).** On the home list the floor is blocked by three keep-outs that a scrolling ancestor clips away
   completely — heading, link and paragraph of the next list section, below the fold — plus the "What is stored" link
   (`perches-phone.json`: `floor blockers: 5 of which fully clipped away: 3`). The survey clips surfaces by their scrolling
   ancestors but not keep-outs. Without the three, the floor would offer `0..286`. In a run the task fills the phone, so there
   is honestly no room.
4. **A first-time learner sees the pets only on the floor** until a cast changes or the page reloads: actors placed on the
   introduction stay on the floor and the home cards are out of their reach (E's hop limits). Whether the home screen should
   re-seat them is E's/I's call.
5. **`👥️shared-presence` fails, independent of pets.** Line 75 expects no `[data-crowd-item]` under the thinker's own item;
   `QR/🔨️modules/🗳️crowd/🟦️.tsx` (written by another ticket at 06:22 today) now renders an empty
   `<p data-crowd-item data-crowd-empty>` for every item nobody answered. Same failure on a fresh proctor, in dev and
   rehearsal (`presence_isolation.txt`). The pet layer renders no such node. I did not touch spec or module.
6. **Ports.** `pets-stories` keeps 6071 (design §7), which the seed also uses for the os hub document sweep (`--serve
   http://127.0.0.1:6071/`) and the root README for the `s` wgpu frontend; I only moved the architecture gallery, as
   briefed. `📓️design.md` §7/§11 and F's report still say 6072. 6074 is used by no launch file (seed, generated, Claude),
   no `Cargo.toml` port table and not by the devcontainer.
7. **Not run**: the gate itself (`test-e2e`, cargo), `🔌️connection-shortage`, deploy-check, docker gates, parity runs.

## 7. Decisions and deviations

1. German title of the ensemble uses "Tierchen", the word sibling I chose for the preference.
2. The site depends on `@semio-tech/pets` only; `@semio-tech/pets-react` is the quiz renderer's dependency (sibling I
   added it there) and the site imports nothing from it. The site vitest config got the one alias its test needs.
3. `🐕️pet-walk` finds the preference by its accessible name in the device's language (a constant in the spec:
   "Pets" / "Tierchen"), because the control carries no hook, and the options by their place in `off, still, calm, lively`;
   every press is checked against `data-pets` on `.quiz-app`, so a reordering fails loudly.
4. At home the spec demands "only species of the home core", not "the first six": E's `castOf` lets a core that does not
   fit take turns. In a run it demands "nobody of another cast" and "nobody changes place", not the whole core — a run may
   leave no free perch.
5. `🐾️pet-cast` checks more than briefed: directories against the ensemble, the module against the documents, a bond for
   every species, a bonded pair in every quiz core, both validators against a broken species.
6. The oracle ids are `pets-gl-matrix`, `pets-d3-ease`, `pets-polygon-clipping` (C and D proposed the latter two; B asked for
   the first with `pets-rig-solving` and `pets-turn-trigonometry`). Profile `pets-float-v1`.
7. The rehearsal stack uses 8922/6192 beside the briefed 8921/6191. I started and stopped: the gallery on 6074 (one process
   tree), the private stacks (dev three times, rehearsal twice), and by mistake one idle `python` process, which I ended.
   Ports 6061, 6161, 6162, 8791, 8891, 8892 were never touched; a dev site of another session was serving the working tree
   on 6061 when I wired the entry.
8. Heavy scratch (private builds, Vite caches, proctor copies and data, Playwright traces) is deleted; the logs, the two
   perch reports and the screenshots (12 MB) stay in `🗑️generated/wp-j` for the coordinator.
9. The repo MCP server was down in this session; I opened and closed no ticket. No git command that modifies anything, no
   cargo, no formatter.
