# 📓️ Quiz Pets — closing summary

Ticket `2026/10/02/QUIZ-PETS`, goal `🎯runningframework🎯runningproducts`, closed 2026-10-02 (bookkeeping on disk; the repo MCP was down all day). Normative text: `📓️design.md` (its §13 "As built" lists every deviation). Per work package: `📓️report-wp-*.md`. Audits: `📓️audit-*.md`.

## What exists

| Where | What |
|---|---|
| `🧰️framework/🛍️products/🐾️pets` | A new, domain-neutral product: schema (`🧬️schema`, JSON Schema + TypeScript + Rust twins), eight twin modules (`📐️trigonometry`, `🎲️randomness`, `🦴️rig`, `🎞️animation`, `🏞️terrain`, `🧠️behavior`, `🎪️stage`, `✅️validation`), packages `@semio-tech/pets` and `semio-framework-pets` (`@semio-tech/pets-rs`), twelve language-agnostic cases with oracles, and the React target `@semio-tech/pets-react` (SVG depiction, DOM survey, pacing, `PetLayer`, a stories gallery). |
| `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🐾️pets` | The quiz's glue: `QuizOptions.pets`, lazy loading, scene and mode selection, the preference (off, still, calm, lively), the footer switch, the notes for reduced motion and forced colours. |
| `🎓️teaching/🏛️architecture/🐾️pets` | The menagerie of the site: twenty hand-crafted, rigged species (the owner's nine — sunny, cloudy, housy, solary, radiatory, pumpy, windowy, waly, battery — and windy, boily, roofy, insuly, shady, venty, chilly, kettly, flamy, thermy, servy), 67 bonds, casts for `home` and each of the four quizzes; every species names the quiz items that ground it. |

## The request, sentence by sentence

| Owner's words | State |
|---|---|
| Pets in the quiz UI | One decorative layer beside the presence overlay on every screen; pets load as a lazy chunk only when they are on. |
| A skeleton, animated | Bones (parents first), parts on bones, a face, keyframed clips; one 2×3 matrix per bone per frame. |
| Slightly active by default | Default `calm`: mostly idle with breathing and blinking, now and then a fidget or a short walk, an encounter every couple of minutes. |
| Standing still they follow the cursor (with the eyes etc) | Pupils follow the pointer (1.5–2 px of travel), the head leans after the gaze, a pet turns to face a pointer on its other side, and a curious one greets a pointer that lingers beside it. |
| Small motions like blinking | Blink scheduler, idle loop, two signature fidgets per species. |
| They walk on top of UI elements | On card tabs, card body edges and the footer line; they hop between levels, ride moving cards, fall when a perch vanishes, and keep off text, controls and the focused element. On desktop pages other than the overview only the footer line has headroom (see "Known limits"). |
| They interact (liking, small disputes) | Greet, cuddle and squabble → sulk → mending, driven by authored bonds plus a rapport that drifts with shared history. |
| Always fitting to the topics | Casts per scene (`home`, `physics`, `heating`, `cooling`, `demand`); a site test fails when a cast pet has no ground in its quiz or a ground names something that does not exist. |

## Verification on the final tree (run by the coordinator unless stated)

| Check | Result |
|---|---|
| `@semio-tech/pets` unit suites (`bun ./📜️script.ts test`) | 8 files, 510 tests passed; 1.2–5 s at the default level (budget 15 s) |
| `@semio-tech/pets-react` suites / `typecheck` | 4 files, 83 tests passed / exit 0 |
| `@semio-tech/quiz-react` suites | 20 files, 640 tests passed |
| `@teaching/architecture-quiz` suites | 5 files, 133 tests passed |
| `@semio-tech/pets-rs:test` | 184 passed, 0 failed |
| Protocol v2 `parity exhaustive --owner 🧰️framework/🛍️products/🐾️pets` | 12 cases, 186 executed, 186 passed, parity 183/183 (oracle, TypeScript, Rust); the TypeScript core is unchanged since that run |
| Taxonomy report, scopes pets and teaching | clean, 0 errors, 0 warnings |
| Gallery, lively, about one minute (Browser pane) | every pet on stage walked, two fidgeted, one changed cards twice; pets turn to the pointer; the layer is `aria-hidden`, pointer-transparent, nothing focusable |
| By agents, not repeated by the coordinator | clippy `-D warnings` on three targets (M); long-run bit-exactness TypeScript vs Rust: 48 sessions of 20 minutes, 57 600 checkpoints, 0 mismatches (M); Playwright project `pets`: 14 tests, three times in a row on private dev and rehearsal stacks (R); release entry script within the 260 kB gzip budget with pets in a lazy chunk (I, J, P, R) |

Not run by this ticket: the full `test-e2e` gate and `deploy-check` (they need the shared ports and cargo builds of other tickets; another session's rehearsal gate at 11:00 passed 31 tests including the pet specs, before the last two pet changes, which were verified on private stacks).

## Decisions worth knowing

- Pets never enter a quiz document or the wire: the site passes `pets: () => import("../🐾️pets/🟦️.ts")` to `mountQuiz`.
- The core is a pure fold (`advance`, `frameOf`) at 64 ticks per second, with exact IEEE operations only, own sine and cosine, and counter-based randomness equal to `numpy.random.SeedSequence`; that is what makes the Rust twin bit-exact.
- A run is a time of concentration: pets rest, do not react to answers, and ignore pokes.
- Reduced motion decides the default only (`still`); an explicit choice in the settings or with the footer switch holds on every device. On the owner's workstation (a Remote Desktop session reports reduced motion) the pets move after choosing "Calm" or "Lively" once.
- The Playwright spec `🐕️pet-walk` is its own project `pets`, last in the gate; nothing depends on it.

## Known limits and open points

1. Desktop pages other than the overview: card tabs lie closer to the navigation bar and to each other than a pet is tall, so pets stand only on the footer line there.
2. Art: encounter poses reach a few pixels into the partner; rotors, fans and the snowflake restart with a short kick after a sulk, cuddle or sleep; some fidget peaks leave the species box; battery's charge window is weak on the dark theme.
3. `🔒️dependencies.json` was not refreshed: the baseline command would approve 75 new entries of all tickets, five of them used by pets test code (`aria-query`, `d3-ease`, `polygon-clipping` and two type packages).
4. A starved host can still exceed the 15 s unit-test budget (1 kill in 24 loaded runs); the remaining time is start-up and transform. Enabling vitest's transform cache is a repo-wide decision.
5. One renderer crash was seen once in a Playwright run before a fix that stopped pets from starting CSS transitions under reduced motion; not reproduced in about 350 exposures afterwards, cause unproven.
6. The unbundled dev server fetches one JSON module per species per page load (21 requests); release builds bundle them.
7. No committed stage trace covers a hop that is hindered in mid-air; the code path is unit-tested in both languages.
8. The whole pets tree is new; it was staged in the git index by someone else during the day, nothing was committed by this ticket.

## How to look at it

- Launch entry `architecture-pets-stories` (port 6074): every species with every clip, and a sandbox where a cast lives on mock cards (scene, mode, capacity, theme, poke).
- The quiz: home shows five of the owner's nine in turn plus one visitor; open a quiz page to see that quiz's cast; Settings → Pets, or the "Show pets" switch on the footer.
