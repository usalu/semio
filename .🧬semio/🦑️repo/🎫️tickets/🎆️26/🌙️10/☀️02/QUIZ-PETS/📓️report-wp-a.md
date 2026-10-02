# 📓️ Work package A (foundation) — report

`P` = `🧰️framework/🛍️products/🐾️pets`, `TK` = this ticket folder, `TAX` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, `TEST` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`. Tool output of every command below is in `TK/🗑️generated/wp-a/`.

## 1. What exists

### Registrations (design §11, all but Cargo and the launch files)

| Registry | Entry | How |
|---|---|---|
| `TAX` `members-of-products` | `🐾️pets` | anchored Edit after `❓️quiz` |
| `TAX` `members-of-modules` | `📐️trigonometry 🦴️rig 🎞️animation 🏞️terrain 🧠️behavior 🎪️stage 🖌️depiction 📡️survey ⏲️pacing 🫧️layer 🐾️pets` | anchored Edit after `🎲️randomness` (inside the quiz block, away from the list end other agents append to) |
| `TAX` `members-of-tests` | the eleven new Protocol v2 cases, `🖌️pet-depiction 📡️surface-survey ⏲️frame-pacing 🫥️decorative-layer`, `🐾️pet-companions 🐾️pet-cast 🐕️pet-walk` | anchored Edit after `🧬️schema-conformance` |
| `TAX` `members-of-fixtures` | the eleven case names plus `🖌️pet-depiction 📡️surface-survey 🐾️pet-companions` | anchored Edit after `🧬️schema-conformance` |
| `TAX` `semanticDirectoryKinds` | `teaching-pets` (`🐾️`, `^pets$`, parent `teaching-architecture`) | inserted before `teaching-quiz-stack` |
| `TAX` `semanticDirectoryMemberKinds` | `members-of-teaching-pets` (owner `teaching-pets`), the twenty species directories | inserted before `members-of-teaching-energy` |
| `🧰️framework/🛍️products/🔣️.json` | member `🐾️pets`, id `framework.product.pets` | Edit |
| root `package.json` `workspaces` | `P/🎯️targets/⚛️react/📦️packages/🟦️typescript`, `P/📦️packages/🟦️typescript` | hand-inserted at their code point position (see deviations) |
| root `package.json` `scripts` | `dev:pets:stories`, `typecheck:pets:react`, `test:pets`, `test:pets:react`, `test:pets:rs` | Edit after the quiz rows |
| `bun.lock`, `node_modules/@semio-tech/{pets,pets-react}` | workspace links | `bun install` (three times, each exit 0) |
| schema catalog | scope `framework.product.pets` | `bun ./📜️script.ts schema generate` (see §2) |

`🎚️config` and `🔬️unit` were already resolvable and were not added again.

### Files

| File | Content |
|---|---|
| `P/📦️packages/🟦️typescript/{package.json, 📋️project.json, 📜️script.ts, 🟦️.ts}` | `@semio-tech/pets`, mirror of the quiz core package. The glue re-exports the schema and all eight modules (75 runtime names, no collision of values or types). devDependencies: `ajv`, `d3-ease` (+ types), `gl-matrix`, `polygon-clipping` (the libraries the unit suites import), `typescript`, `vitest`. |
| `P/🟦️.ts` | façade over the package glue |
| `P/🧪️tests/🎚️config/🟦️.ts` | vitest config, `include: ["../../🔨️modules/*/🧪️tests/🔬️unit/🟦️.ts", "../../🧬️schema/🧪️tests/🔬️unit/🟦️.ts"]`. The glob works with emoji paths on Windows (it picks up six suites today). |
| `P/🧬️schema/🔣️.json` | draft-07, 57 `$defs` = the 57 exported types of the TypeScript twin (pinned by a test), objects closed, `x-semio-formats` on every object, optional string `$schema` on `Species`, `Menagerie`, `Ensemble` |
| `P/🔨️modules/✅️validation/🟦️.ts` | `Issue`, `menagerieIssues`, `speciesIssues`, `ensembleIssues`, `assembleMenagerie` |
| `P/🔨️modules/✅️validation/🧪️tests/🔬️unit/🟦️.ts` | 136 tests: every vector, ajv as cross-check, a sweep that breaks every property of the sample documents in turn, the amended loop seam, assembly, the `$defs` ↔ exported types bijection |
| `P/🔮️oracles/🔣️.json` | `pets-numpy`, `pets-scipy`, `pets-jsonschema`, `pets-ajv-structure` (JS, hosted in the core package), no-oracle decision `pets-stage-trace`, profile `pets-float-v1` (1e-9), `oracleHostPackages`; no `subjectFeatures`. Siblings have since added `pets-react-gl-matrix` and `pets-react-aria-query`. |
| `P/🧪️tests/🧬️schema-conformance/{🥒️.feature, 🐍️.py, 🟦️.ts}` | Protocol v2 case, three scenarios |
| `P/🧫️fixtures/🧬️schema-conformance/🔣️.json` | generated (520 kB): the sample menagerie and 107 vectors |
| `P/README.md` | layout, commands, domain model, sample menagerie, both code tables with the pointer every code is reported at |
| `TK/generate_schema_vectors.py` | the vector generator (composes the documents, takes every expectation from the oracle adapter, replays the three oracle handlers before writing) |
| `TK/taxonomy_registration_check.ts` | read-only check that all 71 names of §11 resolve against the taxonomy on disk |
| `TK/wp_a_typecheck.tsconfig.json` | tsc project over the foundation files |

### Capability ids (for the features of the siblings)

`pets-<case slug>`: `pets-numpy` declares `pets-turn-trigonometry, -counter-randomness, -rig-solving, -gaze-tracking, -spring-settling, -terrain-walking, -behavior-choice, -bond-dynamics`; `pets-scipy` declares `pets-animation-sampling, -hop-ballistics, -behavior-choice`; both allow `pets-float-v1` and `ordered-json-v1`. `pets-stage-trace` covers capability `pets-stage-trace`.

### The sample menagerie (fixture keys)

`menagerie` (id `sample`: `blobby` walker, `hoppy` hopper, `floaty` floater; three bonds 0.6 / −0.4 / 0.1; casts `home` and `meadow`), `ensemble` (with `$schema`), `species` (`[{ path, document }]` in ensemble order, each with `$schema`), `bases` (`species`, `menagerie`, `ensemble`: the smallest valid ones), then `accepted` (19), `structural` (26), `rules` (62). All four shape kinds, all three gaits and all eleven activities occur; `floaty.whirl` is a looping rotation 0 → 360, `floaty.face.above` is set. `assembleMenagerie(ensemble, species.map(d => d.document))` equals `menagerie` (tested).

## 2. Verification (real output)

| Command | Result |
|---|---|
| `bun TK/taxonomy_registration_check.ts` | `checked=71 mismatches=0 taxonomyProblems=0` |
| `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/🐾️pets"` | `clean=true errors=0 warnings=0` (run twice; 89 files, 71 directories at the first run, siblings' folders included) |
| `cd P/📦️packages/🟦️typescript && bun ./📜️script.ts test` | exit 0, `Test Files 6 passed (6)`, `Tests 261 passed (261)`, 10.4 s |
| `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @semio-tech/pets:test` | exit 0, same counts, `Successfully ran target test for project @semio-tech/pets` |
| `.venv/Scripts/python.exe TK/generate_schema_vectors.py` | `accepted=19 structural=26 rules=62`, twenty codes, second run `unchanged` |
| every vector against the TypeScript validator (ad-hoc `bun -e`) | `bad 0 of 107` |
| `cd TEST && bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/🐾️pets"` | exit 1 because of 5865 breaches elsewhere in the repo (`temp/`, other plugins); **no line names a pets path** (the first run named three of mine — description lines starting with `*` are Gherkin steps — fixed; a second run named sibling D's `🦘️hop-ballistics/🐍️.py`, gone in the last run) |
| `bun ./📜️script.ts oracle exhaustive --owner "…/🐾️pets" --case "🧬️schema-conformance"` | `cases=1 executed=3 passed=3 failed=0 errored=0` |
| `bun ./📜️script.ts parity exhaustive --owner "…/🐾️pets" --case "🧬️schema-conformance"` | `cases=1 executed=6 passed=6 failed=0 errored=0 parity=3/3` (TypeScript subject only) |
| `tsc --noEmit -p TK/wp_a_typecheck.tsconfig.json` | 0 errors in pets files (174 errors, all in other trees the harness import pulls in) |
| `bun ./📜️script.ts schema check` before / after `schema generate` | 0 findings for the pets and quiz schemas both times; `schema-catalog-stale=1` before, gone after (9345 → 9344 findings repo-wide) |
| `bun TK/validate_species.ts 🎓️teaching/🏛️architecture/🐾️pets/*/🔣️.json` | all twenty species `ok` |
| `bun ./📜️script.ts workspaces --check` (repo library package) | still stale by seven entries of other tickets; neither pets entry is listed as missing |

`--case` takes the directory name with its emoji (`"🧬️schema-conformance"`); the bare slug selects nothing (`cases=0`, exit 0).

Schema catalog: `schema generate` rewrote `🔣️schema-catalog.json` only (+295 −27): the new pets scope (244 lines), the six quiz exports that were missing (`Handle, HandleView, IdentityClaim, Limits, Motion, TaskIcon`) and refreshed hashes of about two dozen schema files other tickets changed. `📓️schema-catalog.md` did not change.

## 3. Deviations and decisions

1. **Workspaces by hand, not by `workspaces-write`.** The command would also have registered seven in-flight packages of other tickets (four Rust module packages, `📊️viz-kernel`, two `import-edges` fixtures). I inserted only the two pets entries.
2. **TypeScript contract touched (coordinator's file), minimally:** `readonly $schema?: string` on `Species`, `Menagerie`, `Ensemble` (amendment); the docstring emoji of `Palette` (🎨️ → 🌈️) and `Tuned` (🎚️ → 📻️), which repeated those of `Color` and `CHANNELS`; `Color`'s docstring now says lowercase. No field name changed.
3. **Colours are lowercase `#rrggbb`** (`^#[0-9a-f]{6}$`), one canonical form; the roster uses lowercase throughout and all twenty species pass.
4. **Positive means above zero** for sizes, radii, widths, seconds, speed and `hover`; only a rectangle's corner `radius` may be 0. `Ticks` is an integer without a minimum (so the stage may hold stamps of its choosing); `Ticked.ticks`, `Stage.tick`, `Frame.tick` are ≥ 0. `Needs`, `opacity`, `lid` are in [0, 1] and `mood` in [−1, 1] in the schema, as the contract's docstrings state — nothing validates a stage at run time, but a stage test that checks frames against the schema would catch a value outside.
5. **Structural codes** follow the quiz: `type-invalid`, `required`, `property-unknown`, `value-invalid`, `slug-invalid`, `length-invalid`, `items-too-few`, then the thirteen rule codes of §3. Where the schema and a rule overlap the rule's code is reported (a trait of 1.2 is `out-of-range`, a single key is `key-order`).
6. **Pointers of the rules** are in the README table; the ones that needed a decision: `bone-order` at `/bones/i` for a later bone without parent, at `/bones/i/parent` otherwise; `missing-gait-clip` at `/locomotion/gait`; `float-hover` at `/locomotion/hover` in both directions; `loop-seam` at the last key's `value`; `duplicate-*` at every later occurrence; an empty repertoire list counts as no clip.
7. **Loop seam (amendment):** equal values, or on `rotation` a difference that is a whole number of turns (`floor(d ÷ 360) = d ÷ 360`, allowed arithmetic only). Accepted vectors `full-turn-loop`, `opposite-angles-loop`, `two-turns-back-loop`; rejected `half-turn-loop`, `offset-of-a-full-turn` (only rotation wraps), `open-loop`.
8. **`assembleMenagerie` drops the `$schema` hints** of the species and of the ensemble and does not validate; `ensembleIssues` cannot resolve species ids (it has only paths), so bond and cast references are judged by `menagerieIssues` on the assembled menagerie. A repeated species path is `duplicate-id` at `/species/i`.
9. **The oracle of the case is two things, and only one is third-party.** python-jsonschema judges the structure of every vector and proves, for each rule document, that the schema either accepts it or names the committed keyword. The rule findings themselves are recomputed in `🐍️.py` by a second implementation written from the design text — there is no library for them; the registry and the feature say so. Structural rejections are projected as booleans only, because a Rust core that decodes into typed twins cannot name pointer and code; the TypeScript validator is held to pointer and code in its unit suite.
10. **`pets-stage-trace` rests on `specification-vectors` and `metamorphic-laws` only.** Claiming `independent-implementations` while the case has one subject adapter is a contract breach (`claimed-implementations-missing`); phase 2 adds it together with the Rust adapter. Until then the scenarios of that case must not be `@mode-differential`.
11. **`pets-float-v1`** has one `tolerance` (the harness compares absolute differences); the description says so.
12. A structurally invalid species inside a menagerie makes the references to it `unknown-reference` as well — by the rule that a value failing its structure is not judged further and therefore declares no id.

## 4. Open

- **Phase 2 (Rust):** `P/🧬️schema/🦀️.rs`, `✅️validation/🦀️.rs` and the `🦀️.rs` adapter of the case; `subjectFeatures` (`sut`) in the registry; root `Cargo.toml`; `independent-implementations` for `pets-stage-trace`. The README names the crate and `@semio-tech/pets-rs` already; `test:pets:rs` points at a project that does not exist yet.
- **Launch files** (seed rows, `.claude/launch.json`) are work package F's.
- **Registry entries for the JavaScript oracles of the core unit suites** (`gl-matrix`, `d3-ease`, `polygon-clipping` with `hostPath` = the core package) are not there; only `pets-ajv-structure` is. Their owners (B, C, D) add them; the libraries are declared in the package's devDependencies.
- **Fundamental budget:** the package's vitest run is killed after 15 s. It takes 6–10 s on this loaded host with six suites (my suite about 3 s; `strideTo` and `hopOf` of terrain 1 s each); one early run was killed at 15 s while other agents were building. Suites that grow should move slow cases to a higher level.
- `P/🧬️schema/🧪️tests/🔬️unit/🟦️.ts` does not exist; the glob tolerates that. The twin check lives in the validation suite.
- `🗑️generated/wp-a` holds the logs of this work package; it goes with the ticket's cleanup.
- Nothing was committed; no cargo, e2e or deploy command was run.
