# 🔍️ Final Requirements Audit — Quiz Product and Teaching Proctor

Read-only audit of the code and content as they are on 2026-10-01 (working tree, including the uncommitted
`🏠️home/🟦️.tsx` and `🧪️tests/🏠️home-grid/🟦️.tsx` coordinator fix). Nothing was built, run or modified; no test was
executed, so every statement is "found in the code", never "ran green". Paths are relative to `C:\git\semio`; `QZ` =
`🧰️framework/🛍️products/❓️quiz`, `RX` = `QZ/🎯️targets/⚛️react/🔨️modules`, `CAT` = `🎓️teaching/🏛️architecture`,
`EN` = `CAT/⚡️energy`.

Method: read the design (`📓️design.md`), the closing summary and the three earlier audits, then re-read the code and
every quiz file. Mechanical checks: en/de key sets of the client bundle diffed with awk (224 keys each, placeholders
included, diff empty); en/de pairs of every content file counted (cooling 57/57, demand 47/47, heating 65/65, physics
98/98, catalog 21/21; no empty text); grep for hard-coded JSX text, `aria-label`/`title` literals, formal German (`Sie`),
the old host `quizze.…` (all empty); every derived content value recomputed by hand.

## Status table

| # | Requirement | Status | Evidence |
|---|---|---|---|
| R1 | Declarative `quiz` product like `presentation`, not rendering | **Met** | `QZ/README.md:1-9` (model "knows nothing about the DOM or the network"); `QZ/📦️packages/🟦️typescript/package.json` has no `dependencies` (ajv/jstat/mathjs dev only); grep for `document.`/`window.`/`react`/`navigator.` in `QZ/🔨️modules`, `QZ/🧬️schema`, `QZ/📦️packages` finds only the words in `.rs` comments; contract `QZ/🧬️schema/🔣️.json` (draft-07, `Text` requires `en` and `de`, line 28-38); registered `🧰️framework/🛍️products/🔣️.json:24-27`; renderer is a separate target `QZ/🎯️targets/⚛️react`. |
| R2 | Proctor: small server, SQLite, users/scores/intermediate state | **Met** | `🎓️teaching/🛂️proctor/🔨️modules/🗄️storage/🦀️.rs:48-58` (event log, snapshots, projections, one SQLite file); `Cargo.toml` `rusqlite` bundled; roster + learner actors (`🔨️modules/🎭️actors`), `answer-recorded` events keep intermediate state (`QZ/🔨️modules/🧾️lifecycle/🟦️.ts:100-110`), leaderboard/learner/run projections (`🔭️projections/🦀️.rs:185-203`). |
| R3 | Topic tree `teaching/architecture/energy/{physics,heating,cooling}/{quiz,clip,…}` | **Partly met** | `quiz` leaves exist for physics, heating, cooling and an extra `demand` (`EN/*/❓️quiz/🔣️.json`). No `🎬️clip` directory anywhere under `🎓️teaching`; `🎓️teaching/README.md` ("Topic tree") says clips are "not built yet". See G1. |
| R4 | Intro on first visit; step 2 anonymous/pseudonym/name; new if unknown or anonymous, else load; no password | **Met** | `RX/🧭️session/🟦️.ts:91-94` (not introduced → introduction → identity → home), `:111`; form with three radio options + handle input and "no password" note `RX/🪪️identity/🟦️.tsx:30-34,88-132`; decider `QZ/🔨️modules/🧾️lifecycle/🟦️.ts:56-63` (anonymous → always `learner-registered`; unclaimed handle → registered; claimed → `learner-recalled`); client adopts the recalled id `RX/🧭️session/🟦️.ts:450-463`. |
| R5 | Badges Heating expert / Numerical Brain / Pattern seer + catalog's others; rules reachable | **Met** (reachable by analysis; no test on real content, see G4) | Catalog `CAT/❓️quiz/🔣️.json:37-87`: `heating-expert` = `perfect-quiz heating`, `numerical-brain` = `perfect-tasks sorting` (selects `powers`, `energies` in physics), `pattern-seer` = `perfect-tasks classification` (selects `power-or-energy`, `standard-profiles`), plus physics/cooling/demand experts and `completionist`. Evaluation `QZ/🔨️modules/🏅️badges/🟦️.ts:9-22` (score exactly 1; perfect-tasks over all runs). A perfect run is attainable: sorting perfect = no discordant pair; matching cards are unique per dimension, all values in the content are distinct; classification credit 1 per item; means of exact 1s are exactly 1. |
| R6 | Run finished as a whole; results only after submit | **Met** | Submit disabled until every task complete `RX/▶️run/🟦️.tsx:79-82,141`; `run-incomplete` rejection `QZ/🔨️modules/🧾️lifecycle/🟦️.ts:120-122`; sheet is solution-free (`QZ/🔨️modules/🃏️sheet/🟦️.ts:10-12`), run view carries `result` only when submitted (`QZ/🔨️modules/👁️views/🟦️.ts:71`), learner view `score` only when a result exists (`:56`); no per-task feedback in the task views; confirm dialog `RX/▶️run/🟦️.tsx:216-258`. See N1 (crowd hints during a run). |
| R7 | Partial credit; sophisticated metric punishing large misorders for numeric sortings | **Met** | `QZ/🔨️modules/📏️scoring/🟦️.ts:70-87` (pair concordance weighted by the distance of the true values, `log10` for logarithmic quantities), matching `:89-114`, classification by spider-profile distance `:126-154`; explained to the learner in the catalog intro (`CAT/❓️quiz/🔣️.json:17-20`); third-party oracle scipy `kendalltau`/`spearmanr` for the unit-weight and rank readings (`QZ/🧪️tests/📏️sorting-concordance/🐍️.py:97-120`), Rust twin has the same case. |
| R8 | Randomized presentation per run | **Met** | Seed = FNV-1a of the run id, MT19937, Fisher–Yates: task order, item order, `draw` subsets, category order, value-card order (`QZ/🔨️modules/🃏️sheet/🟦️.ts:26-53`); `draw` set on 7 of 9 tasks (all but `standard-profiles`, see N7). See N2 (client picks the seed). |
| R9 | Big leaderboard table | **Met** | Full sortable table (rank, learner, total, best per quiz, badges, runs, last activity; `aria-sort`, own row `aria-current`) `RX/🏆️leaderboard/🟦️.tsx:201-289`, wide page behind the centre card; centre card is an excerpt (top 5 + own row, `:22,125-197`). See D4 for scale. |
| R10 | Content of the four site quizzes | **Met** | Details in "R10 content check" below: every requested task exists with enough items, en/de complete, no physically wrong value found. Minor notes N8-N10. |
| R11 | UI like play/demonstrator: grid of quizzes, leaderboard central | **Met** | `RX/🏠️home/🟦️.tsx:36-42` (ring of nine, leaderboard at index 4 = centre of 3×3), `HOME_GRID_TRACKS` larger centre `:69`; `LayeredOverview` + `OverviewCard` shared with play/demonstrator (`:21`, `RX/🪟️chrome/🟦️.tsx:11,40`); grid CSS at 768/1024 px `RX/🎨️.css:49-87`. |
| R12 | Proctor zero-touch on Docker at `proctor.quizzes.…`; client on CDN at `quizzes.…` | **Partly met** | Artifacts exist and agree: hosts `🚀️deploy/🔣️.json`, API-only image with production env baked in (`🚀️deploy/Dockerfile:62-82`), compose = proctor + Caddy with automatic TLS, health-gated start, volumes (`compose.yaml:17-58`, `Caddyfile`), `publish` verb + CNAME/`404.html`/`_headers`, manual Pages + GHCR workflow `.github/workflows/architecture-quiz.yml`, backup/DNS guide `CAT/❓️quiz/README.md`; no leftover `quizze.…` host. No live evidence: GHCR push and Pages deploy were "not run" (closing summary). See G2. |
| R13 | Presence of every user shared, including cursors | **Partly met** (low) | Roster room + per-place room, cursors relative to anchors, keyboard focus, drags: `RX/👥️presence/🟦️.tsx:419-449,505-524,786-833,929-960`; server admission `🎓️teaching/🛂️proctor/🔨️modules/👥️presence/🦀️.rs:72-123`; text roster for AT (`PresenceList` `:990`). Exceptions: G3. |
| R14 | Multilayered home like play: glass over card content, hover shows page clear | **Met** | `RX/🏠️home/🟦️.tsx:205-223` (`LayeredOverview rest="grid"`, hash routing, keyboard focus reveals, Escape closes, reduced motion `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx:441-442,578,752`), pages are `inert`/`aria-hidden` backdrops (`:396-399`), showing a page never runs a command (`RX/📖️quiz-page/🟦️.tsx:1-8`). Phones get list mode without glass (same as play). |
| R15 | Presence/what others think inside quizzes; home shows real subpages as live grid | **Met** | Thinking room + crowd: `RX/👥️presence/🟦️.tsx:509,517,521`, `RX/🗳️crowd/🟦️.tsx:87-92`, shown in the run (`RX/▶️run/🟦️.tsx:76-77,210-213`), on the quiz page (`RX/📖️quiz-page/🟦️.tsx:181-182`) and results (`RX/🏁️results/🟦️.tsx:214`); all nine panes stay mounted (`RX/🏠️home/🟦️.tsx:216`), home watches the page and thinking rooms at 4 Hz (`RX/👥️presence/🟦️.tsx:469-475,520`), leaderboard polled every 10 s (`RX/🏆️leaderboard/🟦️.tsx:19`, `RX/🏠️home/🟦️.tsx:95`), crowds refreshed when the board counts a new submission (`:98-100`). See N4, N6. |

## R10 content check

| Quiz | Requested tasks | Found | Items (drawn) | Verdict |
|---|---|---|---|---|
| Physikalisches Verständnis `EN/🧲️physics` | classify power vs energy; sort powers tea light → nuclear plant; sort energies phone charge → world consumption | `power-or-energy` classification (16 items: 8 power, 8 energy; draw 12); `powers` sorting tea light 35 W … Sun (14; draw 10; spans ~25 decades); `energies` sorting phone 15 Wh … world 164,000 TWh (12; draw 9; ~16 decades) | ok | present, bilingual |
| Heizen `EN/🔥️heating` | component → U-value incl. historic/single glazing; building + standard → heating load and heating energy demand | `u-values` matching (17; draw 10; single glazing 5.8 … passive roof 0.10, box-type window, 1950s concrete roof, 1980s masonry); `heating-load-and-demand` matching, two dimensions (11; draw 8; passive house … 1960s house) | ok | present, bilingual |
| Kühlen `EN/❄️cooling` | building use → air change rate; building + standard → cooling load and demand | `air-change-rates` (15; draw 10; warehouse 0.13 … ISO-5 cleanroom 540 1/h); `cooling-load-and-demand`, two dimensions (9; draw 7; passive house … data centre) | ok | present, bilingual |
| Energiebedarf `EN/📊️demand` | classify energy standard → heating/cooling/ventilation/cost profiles as spider diagrams; building + standard → total energy demand | `standard-profiles` classification, 4 axes, 6 profile categories A–F, 6 standards (no `draw`); `final-energy` matching (9; draw 7; 14 … 409 kWh/(m²·a)) | ok | present, bilingual |

Values recomputed (all reproduce): Wh/kcal/PJ/EJ conversions, 592 EJ → 18.7 TW / 1.644·10¹⁷ Wh, 527 TWh/8,784 h → 60 GW,
1,361 W/m² × π(6,371 km)² → 174 PW, Sun share < 10⁻⁹, 4.19 × 80 K → 93 Wh, heating oil 10.0 kWh/l, petrol 8.8 kWh/l,
10 kWp → 9,500 kWh, 230 V × 16 A = 3.7 kW, 3 × 230 × 16 = 11 kW, 100 kW = 136 PS, Isar 2 1,410 MW net, 8 MW / 5 MW = 1.6;
heating loads (H_T + 0.34·n·2.5) × 32 K for EFH_E 160, EFH_I 73, EFH_J 50 (n = 0.5 as the item text and
`CAT/README.md` conventions state; the earlier audit's 52.8 used n = 0.6), GEG reference U-values (0.28/0.35/0.20/1.3/1.8
match Annex 1), passive-house criteria (10 W/m², 15 kWh/(m²·a), 0.8 / 0.15 / 0.10), insulation thickness of roof (17 cm),
passive wall (24 cm) and roof (35 cm); air change rates 0.13 (0.35 l/s·m² over 10 m), sports hall 1,800/2,227 m³, office
55/36, car park 6/2.5 = 2.4, cinema 27.7/8, restaurant 6.4, laboratory 25/3 = 8.3, operating room 60/3 = 20, ISO-5
0.45 m/s × 3,600 / 3 m = 540, ISO-6/ISO-7 ratio 2.5; cooling demand = load × full-load hours for all nine buildings;
demand quiz final energy (14, 23, 30, 48, 80, 122, 182, 274, 409) and costs (409 kWh × 12 ct ≈ 49 €). The two defects the
earlier content audit found (GEG-2024 label/value, `sfh-2000s` load) are fixed in the files. **No value that looks
physically wrong was found.** Remaining remarks: N8-N10.

## Repo rules on the quiz/teaching code

| Rule | Status | Evidence |
|---|---|---|
| Two UI languages, en first, de second, German informal "du", no default | **Partly met** | Bundles `RX/🌐️i18n/🟦️.ts:50-319` (en) and `:322-591` (de) have 224 identical keys with identical placeholders; type-enforced `typeof QUIZ_BUNDLE_EN` (`:322`); no hard-coded JSX text or attribute literals; no "Sie/Ihr"; `QUIZ_LOCALES = LANGUAGES` en first, switch order en, de (`RX/🎛️preferences/🟦️.tsx:75`); the 13 identical en/de values are cognates or autonyms (`Deutsch`, `Name`, `Status`, `Quiz`, `Online: {{count}}` …). Baked defaults: D2. |
| Accessibility | **Met**, with D3, D5 | Labelled radio fieldset and handle input with `aria-invalid`/`aria-describedby`, `role=alert`/`status` (`RX/🪪️identity/🟦️.tsx:73-145`); skip link, `lang`, h1 focus after step change (`RX/🟦️.tsx:387-396,347-354`); sorting move up/down buttons, classification and matching `<select>`s, drag grips `aria-hidden`, polite live region per task, focus follows the moved item (`RX/↕️sorting`, `🗂️classification`, `🃏️matching`, `🧩️task/🟦️.tsx:30-56`); confirm dialog with focus trap, Escape, focus return (`RX/▶️run/🟦️.tsx:64-70,108-126`); spider diagram `role=img` + value table (`RX/🕸️radar/🟦️.tsx:430-517`); verdicts as symbol + word (`RX/🏁️results/🟦️.tsx:21-24`); own leaderboard row bar + bold + "(you)" (`RX/🎨️.css:90-93`); peer cursors decorative with a text roster; `prefers-reduced-motion` handled in CSS (`RX/🎨️.css:272-278`) and in `LayeredOverview` (`reducedMotion: "auto"`); 24 px minimum targets (`RX/🎨️.css:40-42`). |
| Progress and cancellation | **Met** | Identify (`RX/🪪️identity/🟦️.tsx:139-144`), start/resume run (`RX/🏠️home/🟦️.tsx:106-115,197-203`) and submit with saving/submitting/results phases (`RX/▶️run/🟦️.tsx:92-107,216-258`) all show a status and a Cancel through `AbortController`; a cancelled submission is reconciled (`RX/🧭️session/🟦️.ts:507-514`); `proctor rebuild` has progress and Ctrl+C (earlier audit, `🎓️teaching/🛂️proctor/🔨️modules/⌨️cli/🦀️.rs`). |
| Customizability | **Met** | Language, theme (system/light/dark), four text sizes (root font size scales every rem token), others' cursors, "show what others think" (`RX/🎛️preferences/🟦️.tsx:23-60,79-113`); persisted local-only, storage failures degrade (`RX/💾️persistence/🟦️.ts:22-61`). N8: no own reduced-motion/contrast switch. |
| Desktop → mobile → tablet | **Met** (by reading) | Desktop 3×3 with larger centre, tablet 2 columns with the leaderboard on its own row, phone list mode (`RX/🏠️home/🟦️.tsx:44-65,104`, `RX/🎨️.css:49-87`); matching/leaderboard/results tables scroll in their cards. Not looked at in a browser. |
| `launch.json` registration | **Met** | Rows for dev, test, typecheck, build, publish, docker-image-build/publish/check, docker-stack-check, site check, proctor check/rebuild/build (`.vscode/launch.json:1968-2068,4488-4576`); the earlier gap (check/rebuild/typecheck) is closed. |

## Findings

### GAP (requirement not or only partly met)

- **G1 · R3 · no `🎬️clip` leaves.** The owner's tree is `{quiz, clip, …}`; only `quiz` exists. Declared out of scope in the
  closing summary and "not built yet" in `🎓️teaching/README.md`. Severity: low (requirement names the tree, content of
  clips was never specified).
- **G2 · R12 · nothing is deployed or published.** Image, Pages artifact, DNS and workflow are present and were proven
  locally (closing summary: build, `docker-image-check`, `docker-stack-check`), but the GHCR push and the Pages workflow were
  not run. `compose.yaml:17-19` defaults to `ghcr.io/usalu/architecture-quiz-proctor:latest` with `pull_policy: missing`
  and a `build:` block; on a fresh host with the package unpublished or private the first `docker compose up -d` depends on
  how Compose behaves when the pull fails (not verified here), and a build is a multi-minute Rust compile
  (`CAT/❓️quiz/README.md`: "pull … when the host does not have it"; "`--build` compiles it from the clone"). Zero-touch
  therefore holds only after the owner's one-time steps (publish image, set package public, Pages source + custom domain, DNS).
  Severity: medium for "finished".
- **G3 · R13 · presence is not shared for everyone, everywhere.** (a) A visitor on the first-visit introduction or on the
  identity step is in no room: rooms need an identified learner (`RX/👥️presence/🟦️.tsx:507-509`; design decision, §15).
  (b) The learner profile and preferences pages have no cursor room (`:435-438`). (c) A pointer is shared only while over an
  element with `data-presence-anchor` (`:774-776,790-792`); elsewhere no cursor. Severity: low; all three are documented
  decisions, but the owner's sentence is unconditional.
- **G4 · R5 · no test proves the badges are reachable on the real content.** `QZ/🧪️tests/🏠️home-grid/🟦️.tsx:105` uses a
  synthetic badge list; `CAT/❓️quiz/🧪️tests/🧪️catalog/🟦️.ts` only validates the files; the three named badge ids appear
  in no other test. A perfect run per quiz and a perfect-sorting/-classification run through `earnedBadges` over the real
  catalog would pin the owner's three badges. Reachability above is from reading, not from a test. Severity: low.

### DEFECT (works against a rule or is wrong)

Count: **5**.

- **D1 · "Switch identity" abandons an anonymous learner without any confirmation.** `RX/📇️profile/🟦️.tsx:62,113` call
  `session.forgetLearner()` (`RX/🧭️session/🟦️.ts:443`) which drops the learner id from the device at once. An anonymous
  learner cannot be recalled (`QZ/🔨️modules/🧾️lifecycle/🟦️.ts:57`: every anonymous identify registers a new learner), so
  runs, answers, badges and the leaderboard row are lost to that person; the identity hint even promises "Your progress
  stays tied to this device" (`quiz.identity.anonymousHint`). One mis-click on a card button destroys it. Severity: medium.
- **D2 · baked default language and a German-only document title.** `CAT/❓️quiz/🌐️.html:2` `<html lang="en">`,
  `:6` and `CAT/❓️quiz/🏗️builder/🌐️vite/🟦️.ts:33,35` title/loading title "Quizze · Architektur und Technologie" (German);
  the client never sets `document.title` (no match in `RX`), so an English reader sees a German tab title, and the static
  document says English while its title is German. `preferredLocale` falls back to `"en"` (`RX/🌐️i18n/🟦️.ts:32`) and the
  shared i18n port has `fallbackLng: "en"` (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🌐️i18n/🟦️.ts:2154,2230`). The repo
  rule is "no default language"; the design documents "English on ties" (§10), so the fallback is a conscious choice, the
  title and `lang` are not. Severity: low-medium.
- **D3 · empty `<td>` as a header cell in the runs table.** `RX/📇️profile/🟦️.tsx:141` `<td className={head} />` sits in
  `<thead>`; the action column (View result / Resume) has no accessible column name and the header row is not all `th`.
  Severity: low.
- **D4 · leaderboard cost grows with every learner and every answer.** The projector rebuilds the whole board from all
  learner states on every event batch that touches a learner, including each `answer-recorded`
  (`🎓️teaching/🛂️proctor/🔨️modules/🔭️projections/🦀️.rs:198-201`), keeps every learner state with all runs and results in
  memory (`:217-223`), the board has no paging or conditional fetch (`🔨️modules/❓️queries/🦀️.rs:69-71`), and every client
  re-downloads all rows every 10 s while home is visible (`RX/🏆️leaderboard/🟦️.tsx:19`, `RX/🏠️home/🟦️.tsx:95`) into a table that
  is always mounted (panes never suspend, `RX/🏠️home/🟦️.tsx:216`). Fine for a class; against the repo's "maximum performance" rule
  for a public leaderboard. Severity: low-medium.
- **D5 · earned vs not-earned badge is told apart visually only by opacity on the overview card.**
  `RX/🏅️badges/🟦️.tsx:56` (`text-muted-foreground opacity-50` vs `text-foreground`), the words exist only in `sr-only`
  (`:59-61`); the docstring claims "by weight, not by colour alone". The badges page does print "Not yet earned" (`:87`), the
  card does not. Rule: colour/appearance must not be the only carrier. Severity: low.

### NOTE

- **N1 · what others think is on by default in a run.** `showAnswers` defaults to true (`RX/🎛️preferences/🟦️.tsx:58`); in a
  run the live drafts of others or, when nobody thinks along, the counts of everyone's submitted answers are shown per item
  (`RX/▶️run/🟦️.tsx:76-77`). That is what R15 asks for, but a majority hint softens "results only after submit"; the opt-out
  exists. No solution is shown before submit.
- **N2 · the learner picks the seed.** The run id is client-generated and the seed is its FNV-1a hash
  (`QZ/🔨️modules/🎲️randomness`, design §3), so a modified client can grind run ids for an easy draw. Same trust model as the
  password-less identity; low for a fun leaderboard.
- **N3 · "Numerical Brain" covers the sorting kind only.** The rule is `perfect-tasks { taskKind: sorting }`, i.e. the two
  physics sortings. The U-value, load, demand and air-change matchings are numeric too and are not included; matches the
  owner's wording "numeric sortings", flagged because the interpretation is a content decision.
- **N4 · "really updating leaderboard" is polling.** 10 s interval, paused while the tab is hidden
  (`RX/🏆️leaderboard/🟦️.tsx:71-87`); no push although the presence socket exists. The crowds follow the polled run count.
- **N5 · an open run cannot be abandoned.** `start-run` is rejected with `run-open` and the client resumes the open run
  (`QZ/🔨️modules/🧾️lifecycle/🟦️.ts:93-94`, `RX/🧭️session/🟦️.ts:471-476`); a new random draw needs a submission first, or a quiz
  revision. By design (R6), but there is no "give up this run".
- **N6 · watch limit.** One socket watches at most 16 rooms (`RX/👥️presence/🟦️.tsx:471-475`): 3 pages + a room and a
  thinking room per quiz = 11 for four quizzes; from seven quizzes on, the tail of the list is silently cut. All nine panes
  are always mounted live (`lifecycle … Infinity`); no performance measurement found.
- **N7 · `standard-profiles` has no `draw`.** The authoring rule (`🎓️teaching/README.md`, "Adding a quiz" step 3) says set
  `draw` below the item count; all six standards appear in every run, only their order changes. Six items onto six profiles
  leaves no room, so it is a justified exception.
- **N8 · no own reduced-motion or contrast preference.** Both follow the operating system (`RX/🎨️.css:272`,
  `LayeredOverview reducedMotion: "auto"`); drag has no edge auto-scroll (`RX/🤏️drag/🟦️.ts`), the buttons/selects are the
  alternative.
- **N9 · sourcing.** Heating, air-change and physics items name a standard or dataset; the seven non-passive-house cooling
  load/demand items rest on "VDI 2078 practice" plus full-load-hour arithmetic, not on a table value
  (`EN/❄️cooling/❓️quiz/🔣️.json` `/tasks/1/items` 1, 2 and 4-8). Not verifiable offline: attribution of single-glazing Ug 5.8 to a
  window table of BAnz AT 04.12.2020 B1 (a Uw table), the DIN 1946-6 apartment formula, the DIN 1946-4 operating-room
  values. Numbers are plausible.
- **N10 · small content slips, none changes a ranking.** Heating `wall-geg`: 12 cm at λ 0.035 alone already gives
  0.28 W/(m²·K); with the masonry's own resistance about 9-10 cm do (`EN/🔥️heating/❓️quiz/🔣️.json` `/tasks/0/items/13`).
  Cooling `classroom`: 28 × 25 + 60 × 2.5 = 850 m³/h gives 4.7 1/h, text says ≈ 860 and 4.8. The earlier content audit's
  statement that every matching task except `u-values` keeps neighbours ≥ 1.25 apart is wrong for the heating task:
  heating demand 122 vs 135 (1.11) and 197 vs 224 (1.14), heating load 50 vs 60 (1.20), 95 vs 115 (1.21) fall below the
  README guideline; they are real IWU values and, since load and demand are rank-identical for all eleven buildings, a
  learner who orders one dimension orders the other, so the second dimension adds no independent challenge (the cooling
  task is not co-monotone: hospital 70 W/m² but 90 vs office tower 100 W/m² but 60).
- **N11 · first visit needs the proctor.** The introduction text lives in the catalog served by the proctor
  (`RX/🟦️.tsx:252`), so a first visit while the proctor is unreachable shows only the waiting card; returning visitors use the
  cached catalog.
- **N12 · not verified by this audit.** No test, browser or Docker run; tablet/phone layout, 150 % text size inside the compact
  grid cards and the live grid performance were judged from CSS and code only; the working tree contains the
  coordinator's uncommitted change in `RX/🏠️home/🟦️.tsx` and `QZ/🧪️tests/🏠️home-grid/🟦️.tsx`.
