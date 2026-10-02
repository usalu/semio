# 📓️ Report — work package R (an explicit pets choice wins over the device's reduced-motion hint)

Ticket `2026/10/02/QUIZ-PETS`, written 2026-10-02 15:20 (host time, UTC+2). Paths: `P = 🧰️framework/🛍️products/🐾️pets`,
`PR = P/🎯️targets/⚛️react`, `Q = 🧰️framework/🛍️products/❓️quiz`, `QR = Q/🎯️targets/⚛️react`,
`S = 🎓️teaching/🏛️architecture/❓️quiz`, `AP = 🎓️teaching/🏛️architecture/🐾️pets`, `TK` = this ticket folder,
`EV = TK/🗑️generated/wp-r` (logs only, 4.7 MB; stacks, the private build and Playwright traces are deleted, except the
trace of the one crashed run in `EV/crash-13-42`).

**State: the rule is built, documented and proven. On the final code the quiz React suite (20 files, 640 tests), the
pets React suite (83), the site suite (133) and the three typechecks are green, and the whole `pets` Playwright project
passed three times in a row in both topologies (dev 14, 14, 14; rehearsal 14, 14, 14). One thing beyond the brief was
found and fixed, because the new rule is what makes it happen: under reduced motion the quiz gives every element a
transition of 0.01 ms, so pets that move there started hundreds of transitions a second (§3). One browser crash seen
before that fix is not proven to be caused by it (§6.1).**

## 1. The rule as built

| | |
|---|---|
| Default | `prefers-reduced-motion: reduce` decides the default only: a learner who has not chosen gets `still` there, `calm` on any other device, and the note under the row |
| Explicit choice | any of off / still / calm / lively in the preferences row, or the footer switch (on or off), is used as chosen on every device; forced colours still hide the layer (unchanged, the layer's own concern) |
| The fact "chosen" | `QuizPreferences.petsChosen: boolean`, next to `pets` and `petsLiveliness`. Only `withPets` sets it (`true`); the pets row and the footer switch are its only callers. `readPreferences` answers `true` only for a stored `true`, so preferences stored without the mark — or with `"true"`, `1`, `null` — count as not chosen. No other preference shape changed |
| Pure helper | `effectivePetMode(choice, chosen, reducedMotion)`: `choice === "off" \|\| chosen \|\| !reducedMotion ? choice : "still"`. Forced colours are not its concern (docstring says so; the provider test shows the mode is handed on unchanged under them) |
| Note | key kept: `quiz.preferences.petsReduced`. Not chosen: "Your device asks for less motion, so the pets stay still until you choose." / "Dein Gerät bittet um weniger Bewegung, deshalb bleiben die Tierchen reglos, bis du selbst wählst." Chosen: no note. No key added, none removed |
| Footer switch | on: the last liveliness, `calm` when there is none yet — also under reduced motion; off counts as choosing too (both go through `withPets`) |

### Decisions (taken without asking, as briefed)

1. **The row and `data-pets` carry the choice in effect, not the stored one.** A learner who has not chosen sees
   "Still" pressed on a device that asks for reduced motion, and `.quiz-app` says `data-pets="still"`. Otherwise a
   pressed "Calm" would stand over motionless pets beside a note that says the opposite, and choosing "Calm" would
   mean pressing a button that looks pressed. The stored `pets` stays `calm` until the learner chooses; when the device
   stops asking, the row and the pets follow at once (unit and browser test). This changes what P and I documented
   (`data-pets` = the stored choice); the quiz README and the design say so now.
2. **`effectivePetMode(choice, chosen, reducedMotion)`** — the learner's two facts first, then the device's. Every
   caller had to change anyway; an old two-argument call does not compile.
3. **`QuizPetsProvider` has a new required prop `chosen`**, beside `choice`. `QuizApp` is its only caller in the
   repository (and `TK/quiz_pets_preview`, updated).
4. **The key keeps its name.** `petsReduced` still means "the note under reduced motion"; only its text changed.
   German "bis du selbst wählst" instead of a literal "bis du wählst": it reads as German and stays informal.
5. **A site without pets** shows the stored choice in the row whatever the device asks for (`usePetsReduced` is false
   without a source): nothing shows there, so there is nothing to explain.
6. **The browser proof of "the pets move again" is a walk** — feet that travel at least 4 px while
   `data-pet-activity="walk"` — and it uses the documented tempo seam (`data-pets-tempo="8"`), as the brief allowed. A
   changed drawing alone would not tell a calm pet from one that is still fading in after a reload.

## 2. Files

### Shared quiz files (small anchored edits, each file re-read before each edit)

| File | What |
|---|---|
| `QR/🔨️modules/🐾️pets/🟦️.tsx` | header (the rule, link to the media query); `effectivePetMode` (new signature and body); `QuizPetsNotes` docstring; `QuizPetsProvider`: prop `chosen`, docstring, the one call of `effectivePetMode`; `usePetsReduced` docstring; `QUIZ_PETS_TEMPO` docstring |
| `QR/🔨️modules/🎛️preferences/🟦️.tsx` | header; import of `effectivePetMode`; `QuizPreferences.petsChosen`; `readPreferences` (one line, docstring); `withPets` (sets the mark, docstring); `PreferencesPanel` (docstring; `motionless`, `pets`, `forced`, `reduced`; the row's `value` and `onChange`) |
| `QR/🔨️modules/🌐️i18n/🟦️.ts` | two lines: `petsReduced` in both bundles |
| `QR/🟦️.tsx` | import line of the pets module (+ `effectivePetMode`, `usePetsReduced`); one line in `Client` (`const pets = …`); `data-pets={pets}`; `chosen={preferences.petsChosen}` on the provider |
| `Q/🧪️tests/🐾️pet-companions/🟦️.tsx` | §4 |
| `Q/🧪️tests/🏠️home-grid/🟦️.tsx` | `PREFERENCES` gains `petsChosen: false` (one line) |
| `Q/🧪️tests/📡️presence-client/🟦️.tsx` | the exact `onChange` object gains `petsChosen: false` (one line) |
| `Q/README.md` | `### Pets`: bullet **Choice** shortened, new bullet **Reduced motion**, **Switch** and **Tempo** adjusted |
| `S/🧪️tests/🐕️pet-walk/🟦️.ts` | §4 |
| `S/README.md` | the spec table row of `🐕️pet-walk`; the last sentence of `### Pets` |
| `AP/README.md` | the sentence on reduced motion in "Looking at them"; the spec row in "Tests" |

### Pets product (the finding of §3)

| File | What |
|---|---|
| `PR/🎨️.css` | `.pet-layer, .pet-layer * { transition-property: none !important; }` with its comment |
| `PR/🔨️modules/📡️survey/🟦️.ts` | `watchSurvey`: a transition that ends inside the ignored element no longer makes the survey stale (`settled`); two docstrings |
| `P/🧪️tests/🫥️decorative-layer/🟦️.tsx` | new test "never transitions: …" |
| `P/🧪️tests/🖌️pet-depiction/🟦️.tsx` | the assertion that the stylesheet names no transition now allows exactly the one declaration that forbids them |
| `P/🧪️tests/📡️surface-survey/🟦️.tsx` | the watch test: a `transitionend` inside the layer and on the layer is passed over, one on the body is not |
| `P/README.md` | "React target": frames show as written, nothing in the layer transitions. No sentence there stated the old rule |

### Ticket

`TK/📓️design.md` (§8.1 signature, mode and `data-pets`; §8.2 `petsChosen`, the row, the two-state note; §13.5 two rows;
§13.6 four rows and the tempo row), this report, `TK/quiz_pets_preview/main.tsx` (current preference shape, `chosen`),
and new tools: `wp_r_stack.sh`, `wp_r_gate.sh` (private stacks on 6197/8927 and 6198/8928, the spec runner),
`wp_r_hang.config.ts`, `wp_r_hang_probe.ts`, `wp_r_transitions_probe.ts`, `wp_r_steps.config.ts`, `wp_r_steps.mjs`.

Not touched: the pets cores (TypeScript and Rust), schemas, fixtures, vectors, `S/🎭️e2e/🎚️config`, launch files,
taxonomy, `QR/🎨️.css`. No command was added, so nothing was registered in `launch.json`.

## 3. Found on the way: moving pets under reduced motion started a transition with every frame

Before this work package pets never moved under reduced motion, so nobody could see it.

- **Mechanism.** `QR/🎨️.css` shortens every transition for a device that asks for reduced motion:
  `.quiz-app * { transition-duration: 0.01ms !important }`. The pet layer lies inside `.quiz-app`, and an element
  without a `transition-property` of its own transitions every property. Every write of a frame into a pet — the
  place and opacity of the `<svg>`, the `transform` of every bone, `cx`, `d` — therefore started a CSS transition.
  Each of them ended with a `transitionend`, and `watchSurvey` took every `transitionend` of the document for a reason
  to measure the page anew and to wake the pacer.
- **Measured** (`TK/wp_r_transitions_probe.ts`, calm pets, real time, two seconds, dev topology,
  `EV/transitions-before`): reduced motion **485** `transitionrun` events inside the layer (`transform` 429,
  `scrollbar-color` 41, `cx` 10, `opacity` 5) and 172 animations held at the end; without reduced motion **0**. The
  spec's own check, run against the unfixed stylesheet (`EV/gate-dev-reduced-before-fix`): 177 and 193 running
  transitions.
- **Fix.** The layer's stylesheet forbids transitions inside the layer (the header of that file always said "nothing
  here animates"; now it holds whatever the host says), and the survey watch passes over a transition that ends
  inside the layer, as it already did with mutations there — so it no longer depends on the stylesheet.
- **After** (`EV/transitions-final-dev`, `EV/transitions-final-rehearsal`): 0 events, 0 animations, computed
  `transition-property: none`, with and without reduced motion, in both topologies. The reduced-motion spec asserts it
  (`layerTransitions`) and its mean duration fell from 32.7 s (n = 24) to 28.3 s (n = 72).

## 4. Tests

### `Q/🧪️tests/🐾️pet-companions` (28 → 33 tests)

- `effectivePetMode`: all sixteen combinations of choice × chosen × reduced motion, written out as a table.
- The mark: default `false`; a write of every other preference leaves it `false` (exact object read back from a
  second tab); every choice through `withPets` sets it and it survives a later unrelated write; the switch both ways;
  six stored shapes without a valid mark read as not chosen; a stored `true` reads as chosen.
- The panel: every control outside the pets row (≥ 15 buttons and checkboxes) calls `onChange` with
  `petsChosen: false`; every button of the row with `true`.
- The provider: follows the device while nothing is chosen (still ↔ lively) and the learner's choice afterwards,
  whatever the device does.
- The note, both languages: not chosen → the note, "Still" pressed, mode `still`; chosen calm / lively / still → that
  mode, that button, no note, and back; `off` either way → no layer, no note; the device stops asking → no note; a
  site without pets → no note and the stored choice pressed.
- Forced colours: the forced note replaces the motion note for every chosen liveliness, and the mode handed to the
  layer is the chosen one.
- The client (real `QuizApp`, real layer): a fresh learner under reduced motion gets `data-pets="still"`, "Still"
  pressed and the note; theme and icon changes do not set the mark; choosing "Calm" gives `calm`, no note,
  `petsChosen: true` in storage, and a second client on the same storage starts calm without a note. The switch under
  reduced motion: off, then on → `calm`, chosen. Preferences stored as `{ pets: "lively", petsLiveliness: "lively" }`
  → still with the note under reduced motion, lively without it, lively for good once chosen.
- The double `motionPreference` now tells only the listeners of the motion query (`useMediaQuery` takes the event's
  `matches`; the old double would have switched forced colours and the phone layout on as well).

### `S/🧪️tests/🐕️pet-walk` (still 13 tests; the reduced-motion test is rewritten)

"a device that asks for reduced motion decides the default only: …": under `emulateMedia({ reducedMotion: "reduce" })`
a fresh learner has `data-pets="still"`, motionless pets (nothing is written into the layer for 90 frames, also while
the pointer crosses the window), the note and "Still" pressed; the device stops asking → no note, `calm`, the drawing
changes; it asks again → note, still, motionless; the learner chooses "Calm" → no note, a pet walks, no transition
runs inside the layer; after a reload `calm` is pressed, no note, a pet walks, and the device still asks for reduced
motion. Every wait is on a state (`data-pets`, `aria-pressed`, `data-pet-activity` with the feet's place, frames
without a write). New helpers `expectWalk` (the lively test uses it too) and `layerTransitions`.

## 5. Commands and real results

Unit suites and typechecks, each from the package folder named:

| Command | Result |
|---|---|
| 13:22 baseline, `bun ./📜️script.ts test` in `QR/📦️packages/🟦️typescript` | 19 files, 613 passed |
| 13:23 baseline, `bun ./📜️script.ts typecheck` there | exit 0, 0 errors |
| 15:15 final, `bun ./📜️script.ts test` there (`EV/unit-quiz-react-final.txt`) | **20 files, 640 passed, none failed** (another ticket added a file in between; no failure of anybody to name) |
| `bun ./📜️script.ts test "🐾️pet-companions" "🗣️translation-completeness" --reporter=verbose` there | **2 files, 45 passed** (33 + 12) |
| `bun ./📜️script.ts typecheck` there | **exit 0, 0 errors** |
| `bun ./📜️script.ts test` in `PR/📦️packages/🟦️typescript` | **4 files, 83 passed** (82 + 1) |
| `bun ./📜️script.ts typecheck` there | exit 0, 0 errors |
| `bun ./📜️script.ts test` in `S/📦️packages/🟦️typescript` | **5 files, 133 passed** |
| `bun ./📜️script.ts typecheck` there (covers the spec) | exit 0, 0 errors |
| `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/domain_docstring_emojis.py <ten files I touched>` (repository root) | no finding in what I wrote; five repeated emojis in `QR/🔨️modules/🎛️preferences` that were there before (§6.5) |

Browser, private stacks only (`bash TK/wp_r_stack.sh restart <dev\|rehearsal>`: dev = `dev-site` with
`TEACHING_ARCHITECTURE_QUIZ_WATCH=off` on 6197 with a development proctor on 8927; rehearsal = the release build of
the working tree, built with `NX_PLUGIN_NO_TIMEOUTS=true PROCTOR_URL=http://127.0.0.1:8928 bun ./📜️script.ts build
--outDir EV/site-rehearsal --emptyOutDir` in `S/📦️packages/🟦️typescript`, served on 6198 under its real policy with a
production-mode proctor on 8928). The spec runner is `bash TK/wp_r_gate.sh <topology> <label> <workers> <repeat>
[specs] [grep]`: `TK/wp_j_playwright.config.ts`, the pet spec in its desktop project and the phone spec beside it, as
P ran it.

| When | Command | Result |
|---|---|---|
| 13:37–13:55, before the fix of §3 | `wp_r_gate.sh dev … "🐕️pet-walk" "reduced motion"`, 39 runs in four batches (3, 6, 6, 24; the log of the first batch of 6 was overwritten by the second, which wrote to the same folder) | 38 passed, **1 "Target crashed"** (13:42, §6.1) |
| 14:23, unfixed stylesheet still served | the same, 2 runs | 2 failed on the new assertion (177 and 193 running transitions) — the check has teeth |
| 14:24–14:34, stylesheet fixed | `wp_r_gate.sh dev row-N 4 1`, N = 1…3, then `rehearsal` | dev 14, 14, 14 passed; rehearsal 14, 14, 14 passed |
| 14:44 | `wp_r_gate.sh dev soak-after-fix 4 72 "🐕️pet-walk" "reduced motion"` | 72 passed (9.0 min) |
| **15:02–15:12, final code** (stacks restarted, release rebuilt) | `wp_r_gate.sh dev final-N 4 1`, then `rehearsal` | **dev 14, 14, 14 passed; rehearsal 14, 14, 14 passed** (13 pet tests and the phone spec; logs `EV/gate-*-final-N/playwright.txt`) |
| 15:12 | `wp_r_gate.sh rehearsal soak-final 4 24 "🐕️pet-walk" "reduced motion"` | 24 passed |
| 15:14 | `WP_R_PROBE=wp_r_transitions_probe.ts … --config TK/wp_r_hang.config.ts`, both topologies | 0 transitions with and without reduced motion |
| 15:02 | `bun TK/site_bundle_weight.ts EV/site-rehearsal` | entry script 865 859 B, **gzip 250 475 B** (< 260 000); lazy pets stylesheet 963 B (was 904) — see §6.3 |

The reduced-motion test takes 19 to 47 s (median 28 s) on the dev stack with four workers; each of its two waits for
a walk has 120 s.

Not run, as told: cargo, the end-to-end gate, deploy-check, parity. No Rust twin is affected (no core changed).

## 6. Open

1. **One browser crash, cause not proven.** At 13:42:33 a run of the reduced-motion test ended with
   `page.evaluate: Target crashed` during its first wait for a walk, 2.7 s after the pets turned calm; the page had
   painted no frame for 4.7 s. Windows recorded it: `chrome-headless-shell.exe`, exception `0xc0000005` (access
   violation), fault offset `0xf6eec` — the only crash event of that browser in the last twelve hours, so nothing P's
   or other tickets' runs produced. A hunt for it (`TK/wp_r_hang_probe.ts`: learners who switch between still and calm
   at the test tempo under reduced motion) gave, with the transitions of §3 present: one more "Target crashed" in
   about 190 rounds (`EV/hang-2`), one page that stopped answering for more than twenty seconds in about 210 rounds
   (`EV/hang-4-transitions`; the three stacks the debugger got lie in React's event dispatch and in
   `watchSurvey → rest → pacer.show`), none in 96 and in about 130 rounds. After the fix: none in 240 rounds
   (`EV/hang-5-after-fix`), none in 96 soak runs of the test and none in the twelve runs of the whole project. That
   fits "the transition storm provoked it" but does not prove it: three events in roughly 670 exposures before, none
   in roughly 350 after. Windows logged no event for the second crash and the hang. If "Target crashed" shows up
   again in the `pets` project, `EV/crash-13-42/trace.zip` is the trace of the first one. (`EV/hang-3` is void: its
   watchdog took a slow reload for a hang; fixed in the tool afterwards.)
2. **The quiz's blanket rule stays as it is** (`.quiz-app * { transition-duration: 0.01ms !important }`, not mine to
   change, `QR/🎨️.css` is hot). Outside the pet layer it still turns every style write into a transition under reduced
   motion — the marks of other learners' cursors, for example, on every move of a peer. That is bounded by what moves
   there, but whoever owns that stylesheet may prefer `transition-property: none` to a duration nobody can see.
3. **The release build's entry script grew by 22.5 kB gzip between 14:00 and 14:24** (227 880 → 250 418 B; 250 475 B at
   15:02) through other tickets' work in the tree — my quiz code was already in the 14:00 build, and what I changed in
   between lies in the lazy pets chunk. 9.5 kB are left under the 260 000 B budget of `siteArtifactProblems`.
4. **The end-to-end gate itself was not run** (forbidden). The spec ran in its gate-like form on private stacks; the
   project `pets` of `S/🎭️e2e/🎚️config/🟦️.ts` is unchanged.
5. **Five docstring emojis repeat in `QR/🔨️modules/🎛️preferences/🟦️.tsx`** (🎛️ ×5, 🗣️ ×4, 🔠️ ×3, 🌓️ ×2, 💾️ ×2), all from
   before this work package; I added no docstring there and left them for the owner of that file.
6. **Not built, on purpose:** a browser test of the footer switch under reduced motion (the unit suite drives the
   real client through it) and a row for a site without pets that follows the device (decision 5).

## 7. Housekeeping

Started and stopped by me only, by the process ids `wp_j_private_stack.ts` wrote down: the private stacks on 6197/8927
and 6198/8928 (all four ports answer nothing now). Ports 6061, 6063, 6069, 6074, 6161, 6162, 8791, 8793, 8891, 8892
were never used; the proctor that still runs (`proctor-57816-…`, started 10:52) is another session's. No git command
that modifies anything (somebody else stages the tree: `git diff` shows my quiz edits as already in the index), no
cargo, no formatter; repository files were changed with Write and Edit only. The temporary `[DEBUG]` lines exist only
in the two probe tools of the ticket, which print their findings. The repo MCP server was down all session: no ticket
was opened, reopened or closed. `EV` keeps the logs this report cites; delete it with `🗑️generated` when the ticket
closes.
