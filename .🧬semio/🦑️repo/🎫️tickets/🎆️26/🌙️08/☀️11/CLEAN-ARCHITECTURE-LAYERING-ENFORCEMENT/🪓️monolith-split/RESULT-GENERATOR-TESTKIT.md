# Result: GENERATOR-TESTKIT (prefix `gtk`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/gtk-tools/`. Logs and reports: `F/🗑️generated/gtk-*`. Sandboxes: `/Users/ueli/.cache/gtk/`. Started 2026-10-17 21:08, resumed 22:10 after the usage-limit reset. Library root `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, generator `L/🏗️authoring/📦️crate-tree/`.

## 0. Verdict (kept current, 22:35)

IN PROGRESS. Generator change and unit tests are in (114 pass, 11 new); replay of the new analysis on the live crates is running (section 4). Coordination with MIRROR-UNWIND (mwu) is written into `RESULT-MIRROR-UNWIND.md` (notice section at its end): mwu edits `🧫️sandbox/🟦️.ts` and the new `🪞️unmirror/`, gtk edits `🟦️.ts`, `📝️render/🟦️.ts`, `🧰️testkit/🟦️.ts` and the tests.

## 1. Plan

1. Inventory: scan the live generated manifests of every carved artifact (`gtk-tools/gtk-inventory.ts`) and classify the hand edits (section 2).
2. Generator: new module `L/🏗️authoring/📦️crate-tree/🧰️testkit/🟦️.ts` derives the test-support wiring from the sources; `carveWrites` emits it; owner rows and lock entries drop stale rows (section 3).
3. Idempotence: (A) replay the analysis on the live carved crates and compare with the live manifests, for every carved artifact (`gtk-tools/gtk-replay.ts`); (B) full regeneration from a sandbox with the carve reverted from the slice backup (`gtk-tools/gtk-regen.ts`) for at least six packages (section 4).

## 2. Hand-edit inventory (live tree, 21:20, `gtk-tools/gtk-inventory.ts`, report `F/🗑️generated/gtk-inventory.json`)

64 artifacts have at least two generated crates. The hand edits to generated manifests fall into these classes (counts are live crates):

| class | what the hand edit is | where (examples) | generator before |
| --- | --- | --- | --- |
| T1 owner feature | `[features] testkit = []` on the crate that owns a `#[cfg(any(test, feature = "testkit"))]` item, plus `features = ["testkit"]` in the `[dev-dependencies]` row of each crate that names the item | forms, remodel, animate, zip, gif, svg, step, xml, deflate, fem-3d, flow, semio (38 owners), puzzle 2d/5d | none |
| T2 external forwards | `testkit = ["semio-framework-plugin/artifact-app-testing", "semio-framework-os-kernel/protocol-laws", "semio-framework-os-kernel/mutation-testing", "semio-s-plugin-puzzle-jobs/testing"]`: the dev-dependency features of the external crates the gated harness uses | flow unit-tests, puzzle 5d hub, puzzle 2d v1 (hand: `wire5d.py`, `wire2d.py`) | none |
| T3 optional dependency | `testkit = ["dep:semio-framework-trace-memory-testing"]` with the dev-only crate copied into `[dependencies]` as `optional = true` | dxf snapshot (the `#[global_allocator]` observer) | none |
| T4 forwarded declared features | `testing`/`testkit` copied to every crate with `crate/feature` forwards and a dev-dependency on every edge | sequence, trinity jack/rewriting, pdf (50 crates, 412 dev rows), gltf (`testing` chain) | already generated (`forwardFeatures`, only when the pre-carve package manifest declares the feature) |
| T5 required-features | none found: 0 `required-features` in any generated manifest | - | none |
| owner rows | `owners_sync.py`: rename a row registered under an older crate name, drop rows of crates that no longer exist | puzzle 5d/2d | `insertBuildOwnerRows` only added rows |
| lock entries | stale `[[package]]` blocks of crates an earlier apply produced; hand-synced "exactly the on-disk crates" | puzzle 5d/2d | `patchCargoLock` only added the carved crates |
| renamed crate names | not a manifest hand edit: plan naming (`mut-any-N`) changes with the plan; handled by the owner-row rename above | puzzle | - |

Also visible: dev-dependency rows are not always on generated crates (flow `unit-tests-2` has a dev-only edge to `unit-tests`; remodel and some stdio crates are hand-made with `{ workspace = true }` siblings); the replay tool compares generated crates only (description starts with "Component crate of").

## 3. Generator changes (all in `L/🏗️authoring/📦️crate-tree/`, tests in `L/🧪️tests/📦️crate-tree/🟦️.ts`)

New module `🧰️testkit/🟦️.ts`; `carveWrites` in `🟦️.ts` now computes the mirrors and file texts before the manifests, runs `planCarveWiring`, and emits:

- owner features: a crate declares a gate feature when an exported item other than a re-export sits behind `cfg(any(test, feature = "x"))` (or `cfg(feature = "testkit")`) or when another crate enables it (a re-exporter whose gate nobody enables declares nothing, as in the live tree);
- T2: the dev-dependency features of every external crate the gated code names, when the gated code names something one of those features adds (the reader follows the crate's own feature implications and the forwarded dependency features through the dependency sources: `loadExternalGates`); a crate without such a name gets `testkit = []` (animate, gif, svg, xml, step, ... as live);
- T3: a dev-only external crate the gated code names becomes an `optional = true` row in `[dependencies]` and a `dep:` entry;
- sibling forwards: `testkit = ["other/testkit"]` for crates whose gated code names a gated item of a crate it depends on;
- consumer dev-dependencies: `features = ["testkit"]` on the `[dev-dependencies]` row of a dependency whose exported gated item (a gated method counts when the code names the method and its type) the crate names, or whose crate its own gated code uses; names inside a `use` count only when the statement is rooted at that dependency's crate; a gated file mounted by several crates belongs to the crates that do not depend on another one of them;
- facade: gate features the facade declares itself, dev-dependency rows of consumed crates, `[[test]]` `required-features` (`withRequiredFeatures`, `requiredTestFeatures`) for integration targets that name a gated item of the facade library;
- features that the pre-carve package manifest declares keep the existing blunt forwarding path and are left out of the analysis;
- `syncBuildOwnerRows` (render): rename a row registered at the same root under another name, drop the rows of this artifact whose manifest is gone, add new rows; `registerBuildOwnerRows` uses it and lists renamed/removed rows beside the backup (`<backup>-owner-rows-changed.json`);
- `patchCargoLock(..., memberNames)`: drops lock blocks `<artifact>-*` that no workspace member and no carved crate has any more, and their entries in other dependency lists.

Tests (`describe("test-support wiring")` plus two in the carve describe, 11 new, suite 114 pass): gate attribute grammar; gate scan (exported names, re-export leaves, gated methods with their type, macros); owner with T2/T3 and consumer-only dev-dependency enabling; plain-API use of an external crate does not forward its features; forwards, declare-on-demand for re-exporters and skip of declared features; gated method confirmed by type name; shared gated file ownership; optional-row/feature-row/required-features rendering; an end-to-end carve of a fixture (gated helper using a dev-only crate: owner `testkit = ["dep:..."]` + optional row, consumer dev-dependency row, facade untouched); owner-row sync; lock pruning.

## 4. Idempotence

(A) and (B) results are filled in below as they finish.
