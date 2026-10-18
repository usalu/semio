# 2D Compute Repoint

Slug: 2D-COMPUTE-REPOINT, prefix `2dc`. Executor: Sonnet. 2026-10-17 23:00 to 00:10. Logs: `🗑️generated/2dc-*`, tools: `2dc-tools/`.

## 0. Verdict

DONE (static and runtime evidence from the non-cargo gates; no cargo ran). The ownership contract names the real owner, every consumer imports `semio_framework_2d_compute` directly with a direct Cargo provider, and `lint rust-source-direction` fell from 486 census problems to 8, none of them a compute-family problem except 2 (a peer's invalid manifest, section 6). Compile check for the new dependency edges is queued for the compile window (section 7).

## 1. Findings

- The facade `semio-framework-2d` deliberately keeps `pub use semio_framework_2d_compute as compute;` (its lib doc "composes", the `📤️exports` fixture lists 8 modules incl. `compute`, and the export test pins it). The 425 old paths therefore still resolved; they were not compile errors. The defect was the contract: `🧮️compute/📏️ownership/🔣️.json` named the facade (`modulePath: ["compute"]`), but the facade no longer mounts the declaration source, the compute crate does (as its lib root). Result: `unproven-family-declaration` (2), `declaration-origin` (8), 433 `unproven-family-provider`, 53 `unproven-family-binding` (baseline run of the family inspector: 498 problems).
- Decision: keep the designed facade module (no change to its API), re-point the contract to the owner and make consumers import the owner directly. The facade `compute` module is no longer a family route for the gate.
- Items moved: none (the carve already moved the six `Engine*` types plus the `Engine` trait into `semio-framework-2d-compute`; this slice moved the consumers).

## 2. Contract and gate (library code)

- `🧮️compute/📏️ownership/🔣️.json`: owner = compute package/lib root, `modulePath: []`, `externName: semio_framework_2d_compute`, `packageName: semio-framework-2d-compute`.
- Gate (`🕸️dependencies/🧭️direction/🦀️source/📍️ownership/🟦️.ts`): a family declared at the crate root (empty `modulePath`) was not representable. Added `declared()` (root = crate root file, else `graph.targets`), removed the non-empty `modulePath` requirement in `readRustFamilyOwnershipContract`, activation also recognises `semio_framework_2d_compute` roots and the `semio-framework-2d-compute` package override. Schema `🧬️schema/📍️binding-origin/🔣️.json`: `minItems: 1` removed.
- Test added (`🧪️tests/🟦️.ts`): "a crate-root declaration with an empty module path admits only the direct root route and a direct provider" (5 expectations: direct import, qualified path, old facade route refused, missing provider refused, bare name refused). `bun test` of that file: 77 pass, 0 fail (includes the schema-vs-contract test against the new real contract). The hermetic fixture corpus (own `provider` crate named `semio_framework_2d`, `modulePath: ["compute"]`) stays as it is: it proves the nested-module mode.
- Text readers: `🧮️compute/🧪️testing/📍️consumer/🟦️.ts` and `🏃️host/🟦️.ts` (`semio_framework_2d_compute::<Name>`, dependency `semio-framework-2d-compute`); `🧮️compute/🧪️tests/📍️ownership/🟦️.ts` (own manifest, lib name, root declaration); plugin `fixture-channel-interfaces/🔣️.json` (`originalSource` import, `originalSha256` recomputed, `canonicalHandles`); `app-declarations-fixture` test: 3 pass. Three consumer descriptors (`🏛️ownership/🧮️compute/🧪️tests/🧫️fixtures`: puzzle 3d, hub space, plugin sdk) pointed at manifests/sources that no longer own the code; re-pointed to the owning manifest/source and the two pinning schemas (`🧬️schema/🔣️.json` consts) updated; puzzle 3d and hub witnesses 3 pass each. The plugin witness `🧪️tests/🟦️.ts` is an empty stub from a peer (0 tests).
- 2d facade export test (`◻️2d/🧪️tests/📤️exports/🦀️.rs`): `TypeId::of::<crate::compute::EngineHandles>()` replaced by `use crate::compute as _;` (module existence stays pinned, no family spelling through the facade).

## 3. Codemod (`2dc-tools/codemod.py`, journal `journal-rs.json`)

- 407 Rust files, 437 sites: `semio_framework_2d::compute::` to `semio_framework_2d_compute::` (use lines 346+, qualified types, impls, doc links). Ticket artifacts and the gate's hermetic fixtures are untouched on purpose. No generator emitted the path (none found in TS/JSON/py under the live roots).
- 26 child modules that relied on `use super::*` for `EngineHandles` got `use semio_framework_2d_compute::EngineHandles;` (`bare-imports.py`, `journal-bare.json`). 3 `include!` fragments (plugin-runtime builder contract tests) use `&semio_framework_2d_compute::EngineHandles` and their two includers dropped the now unused import.

## 4. Manifests, workspaces, locks

- 239 package manifests: 231 replaced `semio-framework-2d` by `semio-framework-2d-compute` (no remaining use of the facade), 2 added it (bim model, flow drawing still use the facade), 6 test-only crates (cad editor tests, 5 plugin companions `tests-*`) had the facade only under `[dev-dependencies]`; the gate requires a normal provider, so the compute dependency now sits in `[dependencies]` and the dev entry was dropped. `semio-framework-os-inference` documents `EngineCache` in doc comments only and needs no dependency. Cycle check: compute depends only on `semio-framework-hash`.
- 32 workspace roots got `[workspace.dependencies.semio-framework-2d-compute]` (workspace-inherited consumers), `puzzle/🧊️3d` with its indented table form fixed in a second pass.
- Locks (coordinator addition): `locksync.py` copied the carved 2d family blocks (facade deps, compute, engine, retire, path-flatten/stroke, booleans, trace, regions, text, fonts and their closure) verbatim from the current `✏️s/Cargo.lock` into the 26 locks that still carried the pre-carve facade block, kept version-sort order; `manifests.py` adds/removes the consumer dependency lines. 117 lock files carry compute lines (list: `2dc-tools/locks-touched.txt`; of these 26 are the repaired stale ones: `2dc-tools/locksync-report.json`). `validate-locks.py`: all 180 locks parse as blocks, byte/semver sorted, dependency lists sorted, no duplicates, and no dangling dependency touches the 2d family; remaining dangling edges (os-directory, plugin crates, test-fixtures in `🧰️framework/Cargo.lock`) are pre-existing and not mine. For `cargo update --workspace --offline` confirmation: the locks in `locks-touched.txt`, plus six consumer packages whose block is absent from their lock (new test crates): `semio-s-artifact-flow-flow-unit-tests-2`, `semio-s-artifact-puzzle-2d-v1-editor-tests`, `semio-s-artifact-puzzle-5d-v1-unit-tests` (their `Cargo.lock` next to the workspace), `semio-framework-plugin-tests-04-2/-08/-11-2` (root `Cargo.lock`). Cargo adds them on its next resolve.
- TOML: 313 changed `Cargo.toml` parse with `Bun.TOML` except one peer-owned manifest (section 6).

## 5. Gate before and after

| Check | Before | After |
|---|---|---|
| family inspector only (`2dc-tools/family.ts`, original contract via override) | 498 problems (declaration 2, provider 433, binding 53, context 2, origin 8) | 2 (`unproven-family-context`), run `2dc-after2` |
| `lint rust-source-direction` (rg7 invocation, `2dc-tools/gate.sh`) | 17 strict + 3 runtime + 486 census (RESULT-REPO-GATES-7, same invocation) | 17 strict + 3 runtime + 8 census (`🗑️generated/2dc-gate-after.log`; 2 context, 4 `missing-input`/`unsupported-expression`/generator/template rows from peer churn, 0 family rows besides the 2) |
| gate unit file `ownership/🧪️tests/🟦️.ts` | 76 pass | 77 pass |
| rustfmt stdin parse of all 437 touched `.rs` | | 0 parse errors; 15 `norm` editor files report the internal "left behind trailing whitespace" format error, identical at HEAD |
| scoped resolution oracle (POS, `2dc-tools/family-s1.json`: facade, compute, puzzle 3d editor, hub space, plugin decl-app, flow drawing, cad, plugin tests-17; lib and test, `--no-base`) | | no row names `semio_framework_2d_compute` or an `Engine*` family type in any consumer crate; the rows are the usual stub-layer noise (396 lib, 625 test), none new in these crates |

## 6. Open items

1. `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/📦️packages/🦀️rust/Cargo.toml` (peer, mtime 22:45) is invalid TOML: line 48 reads `...["testkit"] }[[test]]` and the key `semio-s-artifact-stdio-semio-value-model` is defined twice. Because of it the crate's editor and viewer roots have no module context (the 2 remaining family problems). That crate also never had a facade dependency although its sources import the compute family: once the peer fixes the manifest it needs `semio-framework-2d-compute` as a normal dependency.
2. The 17 `framework-no-products` and 3 runtime-path rows are unchanged (REPO-GATES-7 section 5, item 2).

## 7. Compile queue

When the compile window opens: `cd /Users/ueli/Documents/semio && bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-2d -p semio-framework-2d-compute -p semio-s-artifact-puzzle-3d-editor -p semio-hub-space-app -p semio-framework-plugin-decl-app -p semio-framework-os-flow-drawing -p semio-s-artifact-cad-cad -p semio-framework-plugin-tests-17 --tests --message-format=short`, then the same for one crate per plugin family, then `cargo update --workspace --offline` over `locks-touched.txt` and a diff against the hand patches.
