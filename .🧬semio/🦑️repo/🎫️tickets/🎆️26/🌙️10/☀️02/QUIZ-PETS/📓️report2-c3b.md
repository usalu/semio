# 📓️ Work package C3b (second round): the pets' share of the quiz entry script

Ticket `2026/10/02/QUIZ-PETS`, second round, follow-up of C3 (`📓️report2-c3.md` §9, §11). `QR` = `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`, `M` = `QR/🔨️modules`, `S` = `🎓️teaching/🏛️architecture/❓️quiz`, `SP` = `S/📦️packages/🟦️typescript`, `TK` = this ticket folder, `G` = `TK/🗑️generated/c3b`. Work 2026-10-03 16:40–17:12 (Windows, bun 1.4.2, Vite 7.3.6 / Rollup 4.62.4) while B5, P1 and other tickets edited files beside me.

**State: done.** Everything of the pets glue that only runs once pets came now travels in a lazy chunk of its own: the "Play with the pets" group, the layer as the quiz sets it up (surfaces, keep-outs, controls, props, glances, tempo, error boundary), the scene and the names on stage. The entry script shrank by **932 B gzip** (2 591 B minified); the new lazy chunk is 3 198 B (1 500 B gzip). The quiz React suite (22 files, 831 tests), its typecheck, the site suite (5 files, 228 tests) and the site typecheck pass. The quiz taxonomy scope is clean. A dev browser drive and a run of the release build under its CSP both pass. **There is no overrun any more.** The site's own release build gives an entry of **253 371 B gzip, 6 629 B under the 260 000 B budget**. Most of the earlier overrun went away before my change, when the challenge-levels ticket moved run and results into lazy chunks (§6).

## 1. Measurement before the move (step 1)

Method: `TK/c3b_budget.ts` takes a snapshot of the preferences and i18n modules and writes variants of them to `G/variants.json`. `TK/c3b_budget.vite.ts` wraps the site's own Vite config (`S/🏗️builder/🌐️vite/🟦️.ts`), loads one variant's texts in place of the working tree's (Vite `load` hook, so nothing goes to the repository) and records each module's rendered length per chunk (`<outDir>.modules.json`). `TK/c3b_builds.sh` builds each variant into `G/<prefix>-<variant>`, then builds the first variant again, so any change in the tree between builds would show. Each build is weighed with `TK/site_bundle_weight.ts`, which applies `siteArtifactProblems`' rule: linked vs lazy, gzip. Every variant pins the same snapshot, so two builds differ only in what a variant removes.

| Build (16:49–16:50, tree with C3) | entry minified | entry gzip | Δ gzip |
|---|---|---|---|
| `asis` (snapshot as is) | 885 838 B | **254 306 B** | — |
| `nogroup`: no "Play with the pets" group (`PetsPlayground`, its row, `usePetPlay()`) | 884 330 B | 253 812 B | **−494 B** (−1 508 B min.) |
| `norows`: also no play/mischief checkboxes, no note | 883 509 B | 253 644 B | −168 B more (−821 B min.) |
| `nostrings`: also none of C3's 12 keys × 2 languages | 882 575 B | 253 332 B | −312 B more (−934 B min.) |
| `asis` again | 885 838 B | 254 306 B | identical: the tree did not move between builds |
| site's own command, `bun ./📜️script.ts build --outDir G/before-site --emptyOutDir` in `SP` | 885 838 B | 254 306 B | same file `🌐️-BoNQvX-E.js`: the probe config builds the same thing |

**Our share in the entry before the move: 974 B gzip in total.** That is 494 for the group, 168 for the rows and 312 for the strings. C3's 1 239 B also counted the topic marks (`petProp`, `PetTopic`, `data-pet-topic`) and the glue's selectors and play types.

The entry was already under budget at 254 306 B (C3 had measured 268 525 B at 13:47). The difference is the challenge-levels ticket's split, measured in §6.

## 2. What moved, and where

New module **`M/🐾️pets/🎪️stage/🟦️.tsx`**: the half of the glue that comes with the pets (registered module name, nested under `🐾️pets`; `verify taxonomy report --scope 🧰️framework/🛍️products/❓️quiz` → `clean=true errors=0 warnings=0`, 153 dirs / 199 files against 152 / 198 before). Moved into it:

| From | What |
|---|---|
| `M/🎛️preferences` | `PetsPlayground` (the whole "Play with the pets" row: name column, groups, buttons, status line — markup and hooks `data-pets-play`, `data-pets-player` unchanged), `PET_DEED_LABELS`, `PET_DEED_SAID` |
| `M/🐾️pets` (glue) | `PET_DEEDS`, `PetDeed`, `PET_HOME_SCENE`, `petScene`, `petPlayers`, `petNames`, `QUIZ_PET_SURFACES`, `QUIZ_PET_KEEPOUTS`, `QUIZ_PET_CONTROLS`, `QUIZ_PET_PROPS`, `QUIZ_PETS_TEMPO`, `petsTempo`, `peerGlances`, `PetBoundary`, and the `<Layer …/>` element itself (now `QuizPetLayer`) |

What stays in the entry, and why:

- **Glue `M/🐾️pets/🟦️.tsx`:** the choice (`PET_CHOICES`, `effectivePetMode`, `switchedPets`), because the preferences and the client render it before any pet arrives. The topic marks (`petProp`, `PetTopic`, `usePetTopic`), because the DOM must carry them whenever the layer surveys it. The provider (loading, media queries, context), the hooks the settings read, and the footer switch.
- **Two tiny stand-ins that render the loaded half:** `QuizPets` renders `<shown.half.QuizPetLayer pets={shown}/>`. `PetsPlay` replaces `usePetPlay` and renders `<shown.half.PetsPlayground …/>` (or nothing) with the settings' own `row`/`name` classes, so no class string is duplicated.
- **Loading:** the provider fetches `Promise.all([source(), stage(), quizHalf()])`. `quizHalf = () => import("./🎪️stage/🟦️.tsx")` loads in parallel with the render target and the menagerie, and only when pets are wanted. The `stage` test seam still replaces only the render target. Because the scene is computed through the loaded half (`loaded?.half.petScene(…)`) and is a string, the context's memo stays as stable as before.
- **Preferences:** the two checkboxes and their note stay. They must render (disabled, with the reason) while pets are off or still, which is before and without any chunk. Their remaining cost is 157 B gzip (§3).
- **Package barrel `QR/🟦️.tsx`:** re-exports the moved names from the new module (tests and API unchanged), plus `PetsPlay` and the types `ShownPets` and `PetDeed`. `usePetPlay` is gone: nothing used it once `PetsPlay` existed.

### Strings: they stay in the quiz bundles (decided by the test's rules)

`🗣️translation-completeness` checks every `"quiz.<group>.<key>"` literal in `QR/🟦️.tsx` and `M/**/🟦️.ts(x)` (the new module is inside that glob), lines 66–71. The literal must be a leaf of `QUIZ_BUNDLE_EN`, every leaf there must be used, and the German keys must equal the English ones (line 35). `QUIZ_BUNDLE_EN`/`DE` are what the i18n module registers in the entry, and `QuizLabelKey`/`quizText` are typed against them. That makes the brief's premise **true**. Moving the 9 keys only the group uses (about 200 of the 317 B gzip) into the lazy chunk would need one of two things:

1. A second bundle registered when the chunk arrives. That breaks "used keys are registered" unless the test is rewritten to merge two registries. It also splits the key type and `quizText`. This would bend the rule to the code.
2. Keys the regex cannot see. That removes them from the completeness, placeholder and informal-German checks altogether.

Both break the rules, so all strings stay. The play/mischief rows' 3 keys must render before the chunk anyway.

### One chunk or two

The quiz half could not join the render target's chunk without a cost:

- **One chunk** would need a single dynamically imported module that statically imports `@semio-tech/pets-react`. The package barrel could not re-export it: `pets-react` imports its stylesheet, and Rollup keeps the side effects of statically imported modules (`moduleSideEffects`), so the stylesheet and possibly code would land in the entry.
- **Two chunks (chosen):** the new module imports **no value of the pets product** and runs nothing when evaluated. The barrel can re-export it while Rollup leaves it out of the entry, as it does for the run and results modules.

For the same reason `PET_DEEDS` and `PET_HOME_SCENE` stay written out rather than imported. Importing `DEEDS` from `@semio-tech/pets` would pull the schema module into a third shared chunk. The suite still holds both equal to the product's.

## 3. Measurement after the move (step 3)

Same probe, variants of the moved layout (`bun TK/c3b_budget.ts glue-moved`; `nogroup` here removes the `PetsPlay` stand-in):

| Build (17:00–17:01) | entry minified | entry gzip | lazy quiz half |
|---|---|---|---|
| `asis` | 883 247 B | **253 374 B** | `🟦️-Cv1WII43.js` 3 198 B, gzip 1 499 B |
| `nogroup` (no `PetsPlay`) | 883 077 B | 253 316 B (−58) | |
| `norows` | 882 256 B | 253 159 B (−157) | |
| `nostrings` | 881 322 B | 252 842 B (−317) | |
| `asis` again | 883 247 B | 253 374 B | identical |

**Before → after: entry 885 838 → 883 247 B minified (−2 591 B), gzip 254 306 → 253 374 B (−932 B).** Per-module rendered lengths (`G/before-asis.modules.json` vs `G/after-asis.modules.json`): only two entry modules changed, glue 6 454 → 4 503 and preferences 14 716 → 12 435. The barrel's re-exports render nothing, and the new module (4 791 rendered) sits alone in the lazy chunk, importing React, `jsx`, `BodyButton`, `useAnnouncement` and `localized` from the entry chunk. The delta is therefore C3b's alone. C3's remaining share in the entry is 532 B gzip: rows 157, strings 317, stand-in 58.

Contents check (`grep -F`): the entry no longer contains `data-pets-play`, `data-pets-player`, `quiz-drag-ghost`, `[data-presence-layer] .quiz-peer`, `data-pets-tempo`, `"quiz.preferences.petsHello"` or `quiz.preferences.petsPlayWith`. It does still contain `data-pets-allow` (the rows), `data-pet-topic` (the marks) and the key names in the two bundles. It has five dynamic imports: the menagerie, pets-react, run, results and the quiz half. No `pet-layer` code is in the entry. The CSS is unchanged at 40 764 B gzip.

Lazy chunks at 17:00: menagerie 969 122 B (111 830 gz), pets-react with the core 175 710 B (62 207 gz), shared schema/validation 14 818 B (5 012 gz), run 42 413 B (13 867 gz), results 10 522 B (3 217 gz), quiz half 3 198 B (1 499 gz).

**Final site build (17:09):** `bun ./📜️script.ts build --outDir G/after-site --emptyOutDir` in `SP` gives entry 883 247 B, **gzip 253 371 B**, and quiz half 3 198 B (1 500 gz). The pets-react chunk had meanwhile grown to 183 082 B (64 659 gz) from B5/P1's edits, which are lazy and not mine. The entry's 3-byte difference from 17:00 comes from a different chunk-hash string.

## 4. Commands and real results

| When | Command (cwd) | Result |
|---|---|---|
| 16:48 | `bun TK/c3b_budget.ts` (repo root) | 4 variants, 24 keys found |
| 16:49–16:50 | `bash TK/c3b_builds.sh before asis nogroup norows nostrings` | §1, all exit 0 (a first attempt failed: the barrel re-exports `PET_DEED_*`, so the probe now keeps them; Rollup tree-shakes them out unused) |
| 16:50 | `NX_PLUGIN_NO_TIMEOUTS=true bun ./📜️script.ts build --outDir G/before-site --emptyOutDir` (`SP`) | exit 0, entry gzip 254 306 B |
| 16:50 | `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/❓️quiz"` (root) | `clean=true errors=0 warnings=0`, 152 dirs |
| 16:59 | `bun ./📜️script.ts typecheck` (`QR/📦️packages/🟦️typescript`) | exit 0, 0 errors |
| 16:59 | `bun ./📜️script.ts test "🐾️pet-companions" "🗣️translation-completeness"` (same) | **2 files, 57 passed**, no console output (the suite needed no change; `settle()` covers the third `import()`) |
| 17:00–17:01 | `bun TK/c3b_budget.ts glue-moved`; `bash TK/c3b_builds.sh after asis nogroup norows nostrings` | §3, all exit 0 |
| 17:02 | `bun ./📜️script.ts test` (`QR/…`) | **22 files, 831 passed** |
| 17:02 | `bun ./📜️script.ts test` · `typecheck` (`SP`) | **5 files, 228 passed** · exit 0 |
| 17:03 | taxonomy report, as above | `clean=true errors=0 warnings=0`, 153 dirs / 199 files |
| 17:04 | `node TK/c3b_code_rules.mjs` | stage, glue, preferences and the three tools clean. The barrel lists `📶️`×4 and `❓️`×2: already in the index (`git show :<barrel>`), other tickets' docstrings, not mine |
| 17:05 | preview `bun node_modules/vite/bin/vite.js --config TK/quiz_pets_preview/vite.config.ts --port 6257 --strictPort`, then `node TK/c3_preview_check.mjs --base http://localhost:6257/ --out G/browser` | `failed: []`, `noise: []` (console empty). Group per pet (blobby, hoppy, floaty × Hello/Trick/Pet/Toss) through the lazy half. Deeds on blobby → `greet`; `trick`→`idle`; `purr`; `hang`→`tumble`→`glide`. Status lines and Enter on a focused Hello work. Play off → no group; still → no group, boxes disabled, note shown. German wording correct. Narrow: nobody on stage, no overflow. Screenshots `G/browser/*.png`; the row looks as before. Server stopped by PID 41908 |
| 17:06 | `PROCTOR_URL=http://127.0.0.1:6195 … build --outDir G/after-site-dead --emptyOutDir` (`SP`, dead proctor so nothing reaches production), `bun TK/serve_site_build.ts G/after-site-dead 6196`, `node TK/c3b_release_check.mjs` | exit 0, `failed: []`. The document links one script. Fetched at runtime: run, results, menagerie, shared, pets-react and the quiz half `🟦️-DQ4cldYz.js`. **No CSP violation**, no page error, empty console. Six pets on stage, six groups on the first visit's settings; Hello on Solary → status "Solary, the solar panel says hello.", `data-pet-activity="greet"`. Screenshot `G/release/release-first-visit.png`. Server stopped by PID 5312 |
| 17:08–17:09 final | quiz `typecheck` · `test`; site `test` · `typecheck` · `build --outDir G/after-site` | exit 0 (0 errors) · **22 files, 831 passed** · **5 files, 228 passed** · exit 0 (0 errors) · entry **253 371 B gzip** |

Not run, as instructed: e2e gate, deploy-check, `bun install`. No git command that modifies anything. Ports 6257 and 6196 were started and stopped by me and are free at the end.

## 5. Files

Product, by small anchored edits (each file re-read right before editing):

- `M/🐾️pets/🟦️.tsx`: header; imports; regions Scene/Play/Stage reduced to the player types and the topic marks; `QuizPetsHalf`, `quizHalf`, `ShownPets` (exported), `LoadedPets`; provider (three-way fetch, scene and players through the half); `PetsPlay` replaces `usePetPlay`; `QuizPets` as stand-in; `PetBoundary` gone.
- `M/🎛️preferences/🟦️.tsx`: imports; `PET_DEED_LABELS`, `PET_DEED_SAID` and `PetsPlayground` gone; `PreferencesPanel` docstring; `usePetPlay()` gone; the group row is now `<PetsPlay text={text} row={row} name={name} />`.
- `QR/🟦️.tsx`: the pets re-exports (lines 103–107).
- `🧰️framework/🛍️products/❓️quiz/README.md` `### Pets`: the two halves and why the strings stay; four lazy chunks; `PetsPlay`.
- `S/README.md`: pets as four lazy chunks; deploy-check step 4 weights (entry 253 374 B, quiz half 1 499 B).

New: `M/🐾️pets/🎪️stage/🟦️.tsx`.

Ticket tools (kept, none permanent, so nothing goes in `launch.json`): `TK/c3b_budget.ts`, `TK/c3b_budget.vite.ts`, `TK/c3b_builds.sh`, `TK/c3b_code_rules.mjs`, `TK/c3b_release_check.mjs`. Updated C3's `TK/quiz_pets_preview/main.tsx`: `petScene` now comes from the package. Generated: `G` (48 MB: 12 builds, logs, module maps, screenshots). Delete it with `🗑️generated` when the ticket closes; I keep it for the coordinator, as C3 did.

Untouched: the pets core and React shell (B5, P1), i18n, the suites, the site package code, launch files, the taxonomy file (`🎪️stage` was already a registered module name).

## 6. Remaining overrun and whose growth it is

**No overrun is left: the entry is 253 371 B gzip, 6 629 B of headroom.** The history, from the challenge-levels ticket's own measurements (`QUIZ-CHALLENGE-LEVELS/📓️report-site-final.md` §1), consistent with mine:

| Point | Entry gzip |
|---|---|
| `HEAD` (commit 2026-10-02 22:24) | 254 965 |
| C3 (13:47), C3 adding 1 239 | 268 525 |
| challenge-levels before its split | 269 098 |
| challenge-levels moves run and results into lazy chunks (`useRunScreens`), `TaskGlyph`/`useAnnouncement` into `🪟️chrome`, `/* @__PURE__ */` on module-level expressions | 254 301 |
| my "before" (16:49) | 254 306 |
| after C3b | 253 374 / 253 371 |

Growth from `HEAD` to the working tree before the split was +14 133 B gzip. Per module, unminified, as that ticket attributes it:

- `▶️run` +13.1 k (challenge levels, adaptive layout)
- `🧭️session` +11.1 k
- quiz `🌐️i18n` +9.0 k (challenges, clock, hints, pets)
- `🧩️task` +5.6 k
- `🎛️preferences` +4.8 k (pets round 2 and per-quiz challenge)
- `🏁️results` +4.8 k
- core `⛰️challenge` +4.3 k
- catalog +3.1 k
- `📖️quiz-page` +2.4 k
- `🃏️matching` +2.4 k
- React `⛰️challenge` +2.1 k
- pets glue +1.4 k

About two thirds of that is the challenge ticket's own code, and most of it is now lazy.

What is left of the pets in the entry:

- C3's rows, strings and stand-in: 532 B gzip.
- First-round strings and the choice, switch, provider and topic marks: the glue renders 4 503 B unminified, plus the first round's pets keys.

All of this is needed before any chunk arrives.

## 7. Decisions

1. **One module, `🐾️pets/🎪️stage`, for everything that runs only once pets came.** It is nested beside its glue so the repeated concepts stay together, and it uses an already registered name.
2. **Two lazy chunks, not one** (§2): the barrel may re-export only a side-effect-free module that imports no `pets-react` value.
3. **Strings stay in `QUIZ_BUNDLE_EN`/`DE`** (§2); this costs 317 B gzip.
4. **The play/mischief rows stay in the entry**: they must say "off, and why" before any chunk exists, and cost 157 B.
5. **The scene and names are computed through the loaded half**, but the provider still memoises on the scene string, so context consumers re-render no more often than before.
6. **`PetsPlay` replaces `usePetPlay`.** The settings no longer decide whether the group shows; the glue does, from its context. The settings hand in only their row and name classes (customisable, no duplicated class strings).
7. **`PET_DEEDS` and `PET_HOME_SCENE` stay written out** in the lazy half: no third shared chunk. The suite still compares them to the product's.
8. **No suite change was needed.** The exact props the fake layer receives (`glances: peerGlances`, `keepouts: "button, p, ${QUIZ_PET_KEEPOUTS}"`) are identical, and the third `import()` settles within the suite's `settle()`.

## 8. Open

- **README numbers:** `S/README.md` deploy-check step 4 names this entry weight. Whoever next changes the entry noticeably should update it (the site suite pins no number).
- **B5/P1:** the pets-react chunk grew from 175 710 B to 183 082 B (gzip 62 207 → 64 659) during my work. It is lazy and has no budget impact.
- **Coordinator:** delete `G` (48 MB) with `🗑️generated` when the ticket closes.
