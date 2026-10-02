# 📓️ Work package I — quiz integration of the pets

Ticket `2026/10/02/QUIZ-PETS`, written 2026-10-02 05:40 (host time). Paths: `Q = 🧰️framework/🛍️products/❓️quiz`,
`QR = Q/🎯️targets/⚛️react`, `P = 🧰️framework/🛍️products/🐾️pets`, `PR = P/🎯️targets/⚛️react`,
`S = 🎓️teaching/🏛️architecture/❓️quiz`, `TK` = this ticket folder.

**State: everything of the brief is done and the wiring is applied.** The quiz client loads a site's menagerie and
`@semio-tech/pets-react` lazily and mounts the real `PetLayer`; the only thing missing for pets on the architecture
site is work package J passing `pets` in `S/🟦️.ts`.

## 1. What exists

### New files (mine)

| File | Content |
|---|---|
| `QR/🔨️modules/🐾️pets/🟦️.tsx` | `PET_CHOICES`, `PetChoice`, `effectivePetMode(choice, reducedMotion)`, `PET_HOME_SCENE`, `petScene(step, runs, catalog)`, `petCastNames(menagerie, scene, locale)`, `QUIZ_PET_SURFACES`, `QUIZ_PET_KEEPOUTS`, `peerGlances()`, types `QuizPetsSource`, `QuizPetsStage`, the components `QuizPetsProvider` (loads) and `QuizPets` (mounts the layer), the hook `usePetCast()` |
| `Q/🧪️tests/🐾️pet-companions/🟦️.tsx` | 24 jsdom tests (see §4) |
| `TK/site_bundle_weight.ts` | weighs a built site the way `siteArtifactProblems` does (linked vs lazy scripts and stylesheets, gzip) |
| `TK/alias_targets_check.ts` | proves every pets alias/path I added names an existing file |
| `TK/serve_site_build.ts` | static origin for a private release build (to look at it under its real CSP) |
| `TK/quiz_pets_preview/{vite.config.ts,index.html,main.tsx}` | browser preview of the quiz glue with real cards, the preferences panel and the real layer, no proctor needed |

### How it works

- `QuizOptions.pets?: QuizPetsSource` (`() => Promise<Menagerie>`, type-only import from `@semio-tech/pets`).
- `QuizApp` wraps `Client` in `<QuizPetsProvider source={options.pets} choice={preferences.pets} state={state} locale={locale}>`.
  The provider computes the effective mode (`useMediaQuery("(prefers-reduced-motion: reduce)")` from
  `@semio-tech/ui-react/chrome`, so it follows a change at runtime), and — only when a source exists and the mode is not
  `off` — fetches `source()` and `import("@semio-tech/pets-react")` together. A failed fetch is swallowed (no console
  output, no retry loop; choosing pets again retries), a fetch that outlives the provider or the wish is dropped.
- `<QuizPets />` sits right after `<PresenceOverlay />` inside `.quiz-app` and renders
  `<PetLayer menagerie scene mode quiet surfaces keepouts glances />` inside an error boundary (a throwing layer
  disappears, the quiz stays).
  - scene: `petScene` (quiz id for an opened quiz page, a run, results; else `home`), quiet: `step.screen === "run"`.
  - surfaces: `#quiz-main [data-card]`; keep-outs: the layer's own `PET_KEEPOUTS` (taken from the lazily loaded
    module) plus `[data-quiz-item], [data-quiz-drop]`; glances: tips of `[data-presence-layer] .quiz-peer:not([hidden])`.
- `data-pets={preferences.pets}` on `.quiz-app` (the choice, not the effective mode).
- Preference `QuizPreferences.pets: PetChoice`, default `calm`; a `Segments` row "Pets" after the text size row; under
  it `quiz.preferences.petsCast` with the names of the scene's cast (`usePetCast()`), only while pets show.

## 2. Shared files I edited (all with small anchored `Edit`s)

| File | Where | What |
|---|---|---|
| `QR/🟦️.tsx` | l. 9-10 (header), 48 (import), 91-93 (re-exports), 175-177 + 184 (`QuizOptions`), 403-404 (docstring), 459 (`data-pets`), 483 (`<QuizPets />`), 528-533 (provider around `Client`) | the wiring |
| `QR/🔨️modules/🎛️preferences/🟦️.tsx` | l. 7-8, 16, 43-44 (`PET_CHOICE_LABELS`), 54, 57-58, 69, 126-127, 130, 153-159 | field, read default, row, cast line |
| `QR/🔨️modules/🌐️i18n/🟦️.ts` | EN l. 104-109, DE l. 446-451 (six keys each, contiguous, end of `preferences`); `summary` l. 101 and 443 now also names the pets | keys |
| `QR/🔨️modules/🪟️chrome/🟦️.tsx` | l. 161-162 (docstring), 174 (`min-w-[1.5rem]` + `justify-center` on a segment) | see §5.6 |
| `QR/📦️packages/🟦️typescript/package.json` | l. 26-27 | `@semio-tech/pets`, `@semio-tech/pets-react` as `workspace:*` |
| `QR/📦️packages/🟦️typescript/tsconfig.json` | paths l. 10-11, include l. 43 | paths + suite |
| `QR/🧪️tests/🎚️config/🟦️.ts` | aliases l. 21-22, include l. 52 | aliases + suite |
| `S/🏗️builder/🌐️vite/🟦️.ts` | l. 135-136 | **the two site aliases** `@semio-tech/pets-react` and `@semio-tech/pets` — **J must not add them again** |
| `Q/🧪️tests/📡️presence-client/🟦️.tsx` | l. 839 | `pets: "calm"` in the exact `onChange` object |
| `Q/🧪️tests/🏠️home-grid/🟦️.tsx` | l. 61 | `pets: "calm"` in `PREFERENCES` |
| `Q/README.md` | new `### Pets` before `### State classes` (l. 312) | docs |
| `bun.lock` | 2 lines (the two dependencies in the quiz-react entry) | by `bun install` |

Not edited: `QR/🎨️.css` (no rule needed, see §5.7), `QR/🔨️modules/🏠️home`, the quiz schema, cores, fixtures, oracles,
`TAX`, launch files, `S/🟦️.ts`.

## 3. Commands and real results

All from `QR/📦️packages/🟦️typescript` unless stated.

| When | Command | Result |
|---|---|---|
| 04:40 baseline | `bun ./📜️script.ts typecheck` | exit 1, 133 `error TS`, all in `🧰️framework/🔨️modules/{🛂️manifest (83), 🖱️ui/🧬️contract/🧵️retained/** (46), 🎭️actor/🤖️generated (4)}`; 0 in the quiz — not mine, present before my first edit |
| 04:42 baseline | `bun ./📜️script.ts test` | 17 files, 423 tests passed |
| 04:52 | `bun install` (repo root) | exit 0, "no changes" to installs; `bun.lock` diff = the two dependency lines |
| 05:36 final | `bun ./📜️script.ts typecheck` | exit 1, the same 133 errors, 0 outside `🧰️framework/🔨️modules/**` |
| 05:36 final | `bun ./📜️script.ts test` | **18 files, 447 tests passed** (423 + 24 of `🐾️pet-companions`) |
| 05:37 | `bun ./📜️script.ts typecheck` in `PR/📦️packages/🟦️typescript` | exit 0 (at 05:22 it had 3 errors in `PR/📖️stories/🟦️.tsx`, F's in-flight edit; gone since) |
| 04:53 / 05:34 | `bun TK/alias_targets_check.ts` (repo root) | 6 × ok |
| 05:35 | `GET http://localhost:6061/@fs/…/QR/🔨️modules/🐾️pets/🟦️.tsx` (the running dev site of another session) | 200, the dynamic import resolves through the alias |

Site build, private out dir (nothing was written to `S/📦️packages/🟦️typescript/dist`; no e2e gate was running, only the
dev site on 6061): in `S/📦️packages/🟦️typescript`,
`NX_PLUGIN_NO_TIMEOUTS=true bun ./📜️script.ts build --outDir <TK/🗑️generated/wp-i/…> --emptyOutDir`, then
`bun TK/site_bundle_weight.ts <dir>`.

| | entry script | gzip | entry stylesheet gzip | lazy script | lazy stylesheet | files / total |
|---|---|---|---|---|---|---|
| 04:53, preference + glue in the tree, no dynamic import yet | 748 618 B | **213 957 B** | 37 972 B | – | – | 58 / 2 144 033 B |
| 05:34, wired | 751 800 B | **215 394 B** (+1 437) | 37 977 B | `assets/🟦️-BM9Qkhfu.js` 44 587 B, gzip 17 193 B | `assets/🟦️-BImWDBgO.css` 904 B, gzip 370 B | 60 / 2 192 742 B |

Budget 260 000 B gzip for the document's scripts: 44 606 B headroom. (No clean pre-ticket baseline exists in this
dirty tree; the last recorded value was 182.1 kB on 2026-09-29, before the hardening and icon work of other tickets.)
Proof that the pets are lazy: the document links only `assets/🌐️-*.js` and `assets/🌐️-*.css`; the entry contains
`import("./🟦️-BM9Qkhfu.js")` with Vite's dependency list `["assets/🟦️-BM9Qkhfu.js","assets/🟦️-BImWDBgO.css"]`;
`pet-layer`, `surveyed`, `summoned` occur only in the lazy script; no `[DEBUG]` anywhere. Both lazy names carry a
content hash that satisfies the `/assets/*` rule of `siteArtifactProblems`. The menagerie chunk comes on top once J
imports `AP/🟦️.ts` dynamically.

Real browser (Browser pane), release build with `PROCTOR_URL=http://127.0.0.1:6195` (dead on purpose, so nothing
talks to the production proctor), served by `bun TK/serve_site_build.ts <dir> 6196`: the document boots under its CSP,
`.quiz-app` has `data-pets="calm"`, the resource list holds only the two entry assets (a site without `pets` downloads
no pets chunk), and importing the lazy script and linking the lazy stylesheet by hand raised **no**
`securitypolicyviolation` (`.pet-layer` then computes `position: fixed; pointer-events: none`). The only console error
was the refused connection to the dead proctor.

Real browser, `TK/quiz_pets_preview` (dev, port 6193, sample menagerie of `P/🧫️fixtures/🧬️schema-conformance`):
lazy load through the quiz glue works; `.pet-layer` is the last child of `.quiz-app`, `aria-hidden="true"`, z-index 35,
`pointer-events: none`; `elementFromPoint` over a pet returns `MAIN`; three `svg.pet` (blobby, hoppy, floaty) were
painted; the names line reads "At home here: Blobby, the blob · Hoppy, the spring · Floaty, the balloon" and sits
under the row (row y 129, line y 158, full row width); German row "Tierchen: Aus | Reglos | Ruhig | Lebhaft"; clicking
"Aus" sets `data-pets="off"`. Segment widths after the fix of §5.6: Off 24, Still 26, Calm 32.3, Lively 36.7 px (Off was
22 px before). The pane is hidden in this session, so `visibilityState` is `hidden` and frames only run while a
screenshot is taken: I measured the DOM instead of judging by eye, and got one small screenshot of the pets on the floor.

Not run, by instruction: cargo, e2e, deploy-check, parity. So the Playwright specs (`🗣️both-languages`,
`🥞️layered-home`, the future `🐕️pet-walk`) are **unverified** against the new row and the layer.

## 4. The suite `Q/🧪️tests/🐾️pet-companions/🟦️.tsx`

Choice: default `calm`, persistence of all four, unknown stored value → `calm`; the four buttons in both languages,
pressed state, `onChange` object. Liveliness: `effectivePetMode` table; runtime change of the motion preference
(stubbed `matchMedia` with change events); nothing loaded when off under reduced motion. Scene: every step kind,
every home page, unknown page, missing catalog, unknown run. Layer (injected fake stage): loaded once, exact props
(`surfaces`, composed `keepouts`, `glances`), scene/quiet/mode follow the step; off = nothing loaded, nothing
rendered; no source = nothing loaded; failed source and throwing stage are silent (console spies) and retried when
pets are chosen anew; a fetch dropped by switching off or unmounting; a throwing layer leaves the children; the
selectors against the real markup (`QuizCard` in `#quiz-main`, a card outside `main`, `ClassificationTaskView` items
and drop zones, the real `PET_KEEPOUTS`); `peerGlances` against the real `PresenceOverlay`. Cast: names per scene and
language against `lodash/uniq`, fallback to home, and equality with the cast `petCast` of `@semio-tech/pets-react`
resolves; the line in the panel in both languages, gone when off or without provider. Client (`QuizApp`, real
`PetLayer`, sample menagerie): `data-pets`, lazy mount, layer is the last child and has no focusable node, names
line, Off removes layer and line and is stored, Still mounts again without a second fetch; stored `off` fetches
nothing; a site without pets shows none.

## 5. Decisions and deviations from the design / brief

1. **Provider + mount instead of one component with props.** The cast names must reach `PreferencesPanel`, which is
   rendered deep below `Screen` and `HomeScreen`. Passing an optional prop would have meant editing `Screen` (4 places)
   and the hot `🏠️home` module; instead `QuizPetsProvider` (around `Client`, one edit) owns loading and a context, as
   `PresenceProvider`/`PresenceOverlay` do. `PreferencesPanel` reads `usePetCast()`; every existing call site compiles
   unchanged and shows no line without a provider. The injectable loader is the provider's `stage` prop.
2. **`petScene(step, runs, catalog)`** (three arguments): a home page is a quiz exactly when the catalog lists it —
   the same precedence `🏠️home` uses — instead of guessing from `HOME_PAGES`.
3. **`effectivePetMode` returns a `PetChoice`** (`off` stays `off`), so one function answers "load at all?" and "which mode?".
4. **Surfaces are `#quiz-main [data-card]` only.** `[data-layered-card]` (design §8.1) is `className="contents"`
   (`LayeredOverview` l. 438): it has no box and can never be a surface; the cards inside it are `[data-card]` sections
   in `#quiz-main` and are matched. Dialog cards are portalled outside `main` and are not matched.
5. **Words.** German `still` is "Reglos", not "Still": the e2e spec `S/🧪️tests/🗣️both-languages` fails on any button
   worded the same in both languages, and "Still"/"Ruhig" are near-synonyms in German. The cast line is "At home here:
   {{names}}" / "Hier zu Hause: {{names}}" because it lists the whole cast of the scene (core, then rotation), not who
   is on stage this minute (the layer does not tell); names are joined with " · " (names may contain commas).
   `quiz.preferences.summary` now also names the pets in both languages (no test pins it).
6. **Segments get a minimum width** (`QR/🔨️modules/🪟️chrome/🟦️.tsx` l. 174, not in my brief): "Off" was 22 px wide,
   and the 24 px circle of WCAG 2.5.8 around it cut into its neighbour by 1 px. `min-w-[1.5rem] justify-center`
   changes no existing segment (all were wider).
7. **No rule in `QR/🎨️.css`.** `PR/🟦️.tsx` imports its own `🎨️.css`, so the stylesheet travels with the lazy chunk
   and Vite links it when the chunk loads (allowed by `style-src 'self'`, verified above). The `animation-name` count
   of `🖼️task-icons` is untouched.
8. **Error boundary** around the layer (not in §8, demanded by `📓️explore-prior-design-constraints.md` M16). React
   itself reports a caught render error on the console; a failed *load* is completely silent.
9. **README**: `### Pets` stands before `### State classes` (after "What the others think"), not inside "Task kinds"
   next to the icon paragraph, because pets are not part of the quiz documents.
10. **No Protocol v2 case / fixture**: the glue is React-only; the suite is the design's §10 jsdom suite with `lodash`
    and the real render target as references. The name `🐾️pet-companions` registered under `members-of-fixtures` stays unused.
11. `PET_HOME_SCENE` and the home fallback exist in both `PR/🔨️modules/🫧️layer` and the quiz module (the quiz may not
    import the render target into its entry chunk); the suite holds them to each other.

## 6. Open

- **J**: pass `pets` in `S/🟦️.ts`; the site Vite aliases exist already (§2); the site vitest config and
  `S/📦️packages/🟦️typescript/📋️project.json` inputs are untouched. `QR/📦️packages/🟦️typescript/📋️project.json`
  inputs do not list `P/**` either.
- **e2e / CSP in the rehearsal topology** with a real menagerie: not run by me.
- **Dialogs (for G/E)**: `Dialog` sets `inert` on every child of `.quiz-app`, so while one is open every card is inside
  `[inert]`, the survey finds only the floor and the pets fall. The layer itself is inert then; pausing the show while
  `host.closest("[inert]")` holds would avoid it.
- **Perches (for E/G/J to judge by eye)**: in the preview all three pets stood on the floor — card tops close to the
  top of the viewport (y 6 and 88) are below the clearance or have text above them. Whether the home grid offers
  perches needs a look in a visible browser.
- The cast line of the architecture `home` scene will list twenty names; showing only the core would be the short variant.
- A failed load is retried only when the learner chooses pets again (or the client remounts).
- The suite reads `vectors.menagerie` of `P/🧫️fixtures/🧬️schema-conformance/🔣️.json` (blobby, hoppy, floaty; scene
  `home`); regenerating that fixture with other names breaks two assertions.
- The repo MCP server was down in this session; I opened/closed no ticket (work package of the open ticket).
- `TK/🗑️generated/wp-i` is deleted; the four ticket tools of §1 stay.
