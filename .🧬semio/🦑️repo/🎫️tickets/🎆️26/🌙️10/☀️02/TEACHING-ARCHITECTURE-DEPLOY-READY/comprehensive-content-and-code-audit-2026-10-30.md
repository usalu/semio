# Comprehensive Content and Code Audit

Date: 2026-10-30. Read-only. No `repo run`, Cargo, or Playwright.

Scope: `🎓️teaching/🏛️architecture` course quizzes and pets, `🎓️teaching/🏛️architecture/📚️course/🔣️.json`, `🧰️framework/🛍️products/📚️course` React modules and chrome bundles, `🧰️framework/🛍️products/🐾️pets`, `🎓️teaching/🛂️proctor`.

## Verdict

Learner-facing English and German strings are present, non-empty, and free of `TODO`, `FIXME`, and `TBD`. Every quiz id is in exactly one topic and has a `perfect-quiz` badge. Every species is on the home cast, and every topic, quiz, and question has a cast whose members are real species with a ground in that quiz or task.

Two content nits are worth fixing before treating the badge shelf as final copy: `demand-expert` and `excel-workbook-expert` share the emoji `📊`, and the five non-energy expert badges describe "this quiz" instead of naming it. Excel mockups also contain 493 blank cells; both languages are empty together, and the course schema allows that for `MockupText`.

The pet roster is 39 species, not 20. The energy twenty are exactly the `topic-energy` cast. Nineteen concept species cover Grasshopper, Excel, parametrics, BIM, and optimization.

## Quizzes

All ten files named by the course document resolve, and each `id` matches the expected slug.

| Quiz | Tasks | Kinds | Bilingual objects | Issues |
|---|---|---|---|---|
| physics | power-or-energy, powers, energies | classification, sorting, sorting | 504 | none |
| ventilation | ventilation-statements, air-change-rates, ventilation-profiles | classification, sorting, classification | 346 | none |
| heating | u-values, heating-load-and-demand | matching, matching | 310 | none |
| cooling | cooling-load-and-demand | matching | 118 | none |
| demand | standard-profiles, final-energy | classification, matching | 218 | none |
| parametric-design | parameter-domains, input-output-consistency, complexity-layers, design-tool-profiles, automation-levels | five classifications | 209 | none |
| grasshopper-data | canvas-terminology, type-casting, tree-operations, list-matching | four classifications | 1010 | one shared proper name |
| excel-workbook | workbook-terminology, spreadsheet-commands, copying-references | three classifications | 2926 | 493 blank mockup cells |
| bim-concepts | data-information, object-oriented-model, model-coordination | three classifications | 143 | none |
| optimization-decisions | optimization-objectives, pareto-dominance | two classifications | 90 | none |

Checked on every `{en, de}` object: both keys present, both strings, neither whitespace-only except the Excel blanks below, no `TODO` / `FIXME` / `TBD` / `WIP` / `lorem` / `unimplemented`.

Excel: 2433 populated pairs and 493 pairs where both `en` and `de` are `""`. Those sit on spreadsheet `rows` and `formula` fields, which the schema types as `MockupText` (no `minLength`). `Text` and `ShortText` still require `minLength: 1`. No cell is filled in only one language. Treat the blanks as empty spreadsheet cells, not missing translations.

Grasshopper: one node label is identical in both languages, `Structure Tools / Analyze`, on `canvas-terminology` item mockup. That reads as a product name, not an untranslated sentence.

## Course document

`🎓️teaching/🏛️architecture/📚️course/🔣️.json`

- Title, introduction (six paragraphs), six topics, short label `A&T` / `A&T`, and the parametrization assignment title and description are complete in both languages.
- Topic titles that stay the same in German are proper names: Grasshopper, Excel, Building Information Modelling.
- `unlocks`: `energy` is `0`; parametric, grasshopper, excel, bim, and optimization are `null` (not yet scheduled). Every topic key is present.
- Each of the ten quiz ids appears in exactly one topic. Topic quiz lists match the quiz file ids.
- The assignment `documents` object is bilingual by language key, but each side is a document record (namespace, id, producer, file), not a sentence. Both PDFs are named.

## Badges

Thirteen badges. Mapping is complete.

| Badge | Rule | Quiz or task kind |
|---|---|---|
| physics-expert | perfect-quiz, medium | physics |
| ventilation-expert | perfect-quiz, medium | ventilation |
| heating-expert | perfect-quiz, medium | heating |
| cooling-expert | perfect-quiz, medium | cooling |
| demand-expert | perfect-quiz, medium | demand |
| parametric-design-expert | perfect-quiz, medium | parametric-design |
| grasshopper-data-expert | perfect-quiz, medium | grasshopper-data |
| excel-workbook-expert | perfect-quiz, medium | excel-workbook |
| bim-concepts-expert | perfect-quiz, medium | bim-concepts |
| optimization-decisions-expert | perfect-quiz, medium | optimization-decisions |
| numerical-brain | perfect-tasks sorting, medium | course-wide; sorting exists on physics and ventilation |
| pattern-seer | perfect-tasks classification, medium | course-wide; classification exists on eight quizzes (not heating or cooling, which are matching only) |
| completionist | completed-quizzes | every quiz |

No badge points at an unknown quiz. No quiz lacks a perfect-quiz badge. Labels and descriptions are non-empty in both languages, with no placeholder tokens.

Wording gaps:

- `demand-expert` and `excel-workbook-expert` both use `📊`. The Excel topic uses the same emoji. The demand badge is the collision to change if each badge needs its own mark.
- physics through demand name the quiz in the description. parametric-design, grasshopper-data, excel-workbook, bim-concepts, and optimization-decisions say "Scored 100% in this quiz on medium or harder." / "100 % in diesem Quiz auf Mittel oder schwerer erreicht." The rule still names the quiz id; the sentence does not.
- Spacing of `100 %` is inconsistent: several energy descriptions use a narrow no-break space, ventilation and the five later badges use a normal space.

## Pets

`🎓️teaching/🏛️architecture/🐾️pets/🔣️.json` declares 39 species files. All 39 directories exist, ids match the file stems, and every `name` and `thing` pair is a non-empty bilingual string with no placeholders. Required species fields (`id`, `name`, `thing`, `grounds`, `size`, `palette`, `bones`, `parts`, `face`, `clips`) are present. Bond endpoints are all declared species.

The energy twenty, in roster order: sunny, cloudy, housy, solary, radiatory, pumpy, windowy, waly, battery, windy, boily, roofy, insuly, shady, venty, chilly, kettly, flamy, thermy, servy.

The nineteen concept species: rhino, grasshopper, slider, colour-wheel, workbook, worksheet, cell, formula, double-curved-surface, continuous-parameter, discrete-parameter, execution-plan, model-3d, plan-2d, quantities, pareto, heatmap, minimized-surface, bar-chart.

Casts: 45 scenes, no unknown species, no duplicate home members, no orphan scenes.

| Scene group | Count | Coverage |
|---|---|---|
| home | 1 | all 39 species (9 core, 30 rotation) |
| topic-energy | 1 | the energy twenty (9 core, 11 rotation) |
| topic-parametric, topic-grasshopper, topic-excel, topic-bim, topic-optimization | 5 | exactly that topic's concept species |
| quiz ids | 10 | one cast per quiz |
| question-quiz-task | 28 | one cast per task in all ten quizzes |

Every species in a quiz cast has at least one ground in that quiz. Every species in a question cast has a ground in that quiz or on that task. Parametric quiz and topic casts have an empty rotation because that topic has exactly three species, all in the core. Ventilation's quiz rotation is empty by design (venty and windowy only).

## React modules and chrome bundles

`🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/` (62 TypeScript modules) and `🎓️teaching/🏛️architecture/📚️course/` source: no `TODO`, `FIXME`, `TBD`, or `unimplemented`.

`//` lines in these trees are module-level `//#region` / `//#endregion` markers, not comments inside function bodies. Exported React functions in the dirty course modules (`home`, `app`, `results`, `access`, `chrome`, `scrollport`) open with an emoji docstring.

Chrome bundles `🔠️english-bundle/🟦️.ts` and `🔡️german-bundle/🟦️.ts`:

- 436 `phrase(...)` calls on each side, in the same order.
- 462 object keys on each side; no key only on one side. German is annotated `typeof COURSE_BUNDLE_EN`.
- No empty phrases and no `TODO` / `FIXME` / `TBD`.
- Five hint sentences use the same `{{count}}`, `{{small}}`, `{{quantity}}`, `{{large}}`, `{{times}}`, and `{{difference}}` tokens. German word order moves `{{quantity}}` earlier. The token sets match.
- Eighteen identical phrase pairs are shared tokens or proper names (`Normal`, `Desktop`, `Tablet`, `English`, `Deutsch`, `Pseudonym`, `Quiz`, `Status`, `Minimum`, `Maximum`, `online`, and interpolation-only lines).

The word `placeholder` in `home` and `i18n` is a parameter or interpolation name, not unfinished copy.

## Proctor and pets product

`🎓️teaching/🛂️proctor`: no `TODO` / `FIXME` / `TBD` / `unimplemented` in TypeScript, Rust, or JSON. Operator messages in the proctor bootstrap messages file are complete in English and German. The brace tokens `{mode}`, `{path}`, `{command}`, `{reason}`, `{count}`, `{example}`, `{detail}`, `{actual}`, `{host}`, and `{expected}` match on both sides. Fixture courses under the proctor test fixtures are bilingual test data, not the live architecture course.

The pets product has no `TODO`, `FIXME`, or `TBD` in TypeScript, TSX, or JSON. Rust doc comments on types and methods start with an emoji. Region markers sit at module scope.

## Git status and house rules

`git status` for the course site, architecture course, architecture pets, pets product, and proctor lists 67 dirty paths (44 of them TypeScript, TSX, or Rust). The rest of the working tree is much larger and was not reviewed line by line.

Dirty React course modules:

- course React `home`
- course React `app`
- course React `results`
- course React `access`
- course React `chrome` and its `scrollport` module, plus the scrollport schema

None of those files contain `TODO`, `FIXME`, `TBD`, or `unimplemented`. Their exported functions open with an emoji docstring. `//` comments in the course React tree and in architecture course sources are module-level region markers, not comments inside function or method bodies.

No source path under architecture, the course product, the pets product, or proctor exceeds 256 characters once `dist`, `node_modules`, and generated output are excluded. Release artifacts under `📚️course/📦️packages/🟦️typescript/dist/` were not counted.

## What was not run

Schema validation, typecheck, unit tests, and the browser suite were not executed, because another agent is already running the end-to-end browser suite.
