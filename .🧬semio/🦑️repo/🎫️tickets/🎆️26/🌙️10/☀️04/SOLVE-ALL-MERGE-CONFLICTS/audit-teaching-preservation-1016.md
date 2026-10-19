# Independent Teaching Preservation Audit — 2026-10-16

Read-only finite verification of `preserve-teaching-stash-1016.md` against base `f856dff081373e131ebf72618120cb73e303a912`, HEAD `1bae6115bc56bf5e1b01212774dd76877a5f64fe`, latest stash `ec7b3361858df86599da79e5b6dde70ccbbc3625` and current files. No builds, tests, compilation, runtime or Git mutations. Only this report was written.

## Verified Claims

Parsed the exact 633 listed modified-path records: all 380 created/updated files exist; 376 equal stash bytes exactly; exactly four differ as documented. All 253 listed removed paths are absent. All 116 listed created/updated JSON files parse.

Independent base→HEAD path comparison confirms only four competing productive owners: teaching Cargo.toml, old teaching quiz build script, proctor Rust script, old framework quiz Rust script. Working-versus-stash diff of the four merged files confirms only: retained HEAD Cargo profile removal; added compiler-admission import and authority argument on the matching Vite entry; retained captured Cargo artifact verification imports and producer/invocation/consume arguments for proctor and framework course. The network and compiler control arguments remain. No unexplained competing HEAD delta found.

Independent HEAD old-owner tree census confirms 42 teaching quiz files and 208 framework quiz files. Stash new owners contain 69 and 233 files respectively. Every framework old suffix maps to new course suffix, allowing fixture move from `🧪️tests/🧫️fixtures` to `🧫️fixtures`. Teaching mapping additionally renames `📚️catalog/🟦️.ts` → `📚️course/🟦️.ts` and `🧪️tests/🧪️catalog/🟦️.ts` → `🧪️tests/🧪️course/🟦️.ts`; those new authored files exist and match stash. No path-level old owner omission found after these explicit mappings.

Workflow `.github/workflows/architecture-course.yml` is stash-exact and old architecture-quiz workflow is absent. New course deployment source/schema/image/Docker/Compose assets and proctor source/tests are included in the 376 exact comparisons. Therefore no accidental missing authored workflow/deploy/proctor source was found in this list. Root Cargo/launch integration remains parent-owned and is excluded from this result. Source preservation does not establish runtime or compile correctness.

## Remaining Outside-Lane Clusters

A bounded stash AM-path grouping outside teaching/course, repo build/dashboard, shared framework modules and ✏️ owners identifies OS product (1090 paths), pets (39), server (8), print (6), and hub compositions (8) as follow-up clusters. Counts describe stash changed paths, not failures. OS has many absent old locators and needs explicit HEAD relocation classification before claiming loss; no giant content census was attempted.

For four smaller clusters, direct current-versus-base/stash byte checks find:

| Cluster | Base-identical | Stash-exact | Different/merged | Missing original path |
| --- | ---: | ---: | ---: | ---: |
| pets | 37 | 0 | 1 | 1 |
| server | 7 | 0 | 0 | 1 |
| print | 5 | 0 | 1 | 0 |
| hub compositions | 5 | 0 | 3 | 0 |

These base-identical files prove their specific stash changes are absent at the same current source owner, rather than merely missing renamed paths. They do not prove that an equivalent implementation is nowhere else. Productive examples requiring preservation review:

- `🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/📡️survey/🟦️.ts`: current equals base; stash adds `[data-concealed]` exclusion to survey selectors/tree walking and observed attributes.
- `🧰️framework/🛍️products/📓️print/🧬️schema/🔀️diff/🦀️.rs`: current equals base; stash adds `ColdMutationDiff`/`ColdDiffAlgebra` and prepared mutable application. Must merge with the protocol owner rather than independently guess architecture.
- `🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️.rs` and its unit owner: current equals base; stash deltas remain unapplied at those locators. Server main TS delta is deletion of inline Vitest registration, so base identity there alone is not evidence of lost productive runtime content.
- `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🖊️drawing-reader/🟦️.ts` and `🏷️type/🟦️.ts`: current equals base, authored stash source-contract deltas absent locally.

Missing pets fixture `🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🎬️context-casts/🔣️.json` and server fixture `🧰️framework/🛍️products/🖥️server/🧫️fixtures/📥️body/🔣️.json` are locator candidates only until relocation/equivalent-content inspection. Shared module executor and parent root owner may cover corresponding integration; no edits were attempted here.
