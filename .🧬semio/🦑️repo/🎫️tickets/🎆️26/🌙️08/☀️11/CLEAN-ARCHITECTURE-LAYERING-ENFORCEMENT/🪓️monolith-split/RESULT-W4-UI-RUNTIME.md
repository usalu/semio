# Result (wave 4): `semio-framework-ui-runtime` carved into five producer crates plus a thin facade

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git. Brief: `📓️WAVE4-BRIEF.md`, including its later rule "god objects". Tools, scripts and drafts are in `uirt-tools/`; logs are `🗑️generated/uirt-*.log`. `RT` = `🧰️framework/🔨️modules/🖱️ui/🧠️runtime`.

## 0. Status in one paragraph

The split is done and statically verified; **nothing was compiled and no test was run by cargo**. `bun repo cargo ...` fails with `Finite original maintenance authority required` and `bun repo run @semio-tech/ui-runtime-exchange-rs:check ...` with `Active graph readers retain required Nx pool headroom` (baseline and after, logs `uirt-cargo-check-*.log`, `uirt-nx-check-*.log`). Evidence is the full-resolution differential on a union sandbox (native, testkit, `--test`, `--test`+testkit, wasm32-wasip2, wasm32-unknown-unknown: only the baseline's external-crate noise remains, no new in-crate resolution error), the definition scan, the canonical census, and the repo's UI crate-boundary gate (`bun test`, 59 pass, 0 fail). Type, borrow and field-privacy errors are only covered by the scans, so the first real compile may still find some (section 7).

## 1. How the god-object rule was applied

`SurfaceReconciler` was the god object: one struct that the retained cursor reads, the handback registry drops into (`Drop` builds `SurfaceReconcileRetained`), and the job completes. A crate split made a cycle (reconciler -> retained -> cursor -> reconciler). I did not use an extension trait or widen internals. The struct was decomposed:

- `SurfaceShadow` (cursor crate) owns every retained owner (`surface`, `revision`, `allocator`, `document`, `native_resources`, `native_resource_loans`, `assembly`, `ordinals`, `key_index`, `root`, seal and retire scalars) and its whole impl: `read_record`, `capture_document`, `seal_step`, `retire_one`, `take_all`, the native-resource methods and the testkit oracle API.
- `SurfaceReconciler` (authority crate) is the thin owner `{ shadow, handback, retirement_armed }`: one-line delegations plus the registry `Drop`. Consumers keep `SurfaceReconciler::new(SurfaceId)` and every public method.
- Cycles are broken with narrow parameters, not `&mut WholeState`: the cursor reads `&SurfaceShadow` and completes with `Complete { shadow, patch }`; `SurfaceShadow::retire_one<H>(&mut self, handback: &mut Option<H>, release: impl FnOnce(H))` releases the owner's reservation at the same step number as before without knowing the registry; `seal_step` takes the owner's `size_of` as `owner_bytes`, so credit pricing is unchanged.
- The big white-box suites were split along the same lines (section 4): the tests live with the component whose privates they read, so no private field of an exported type was widened. Where the authority tests read the cursor or shadow they use narrow testkit-gated accessors (`usage`, `assembly_generation`, `pending_root`, `held_node`, `document`, `retire_scalar`).

## 2. Census (canonical `readRustSourceClosureV1`, host cfg; `uirt-tools/census.ts`, `🗑️generated/uirt-census-after.log`)

Before: lib 16 files / 366,728 bytes / 6,304 active lines (over the gate by 1.58x); test selection 44 files / 618,074 bytes / 11,256 lines.

| Crate | lib: files / bytes / lines | test selection: files / bytes / lines |
| --- | --- | --- |
| `semio-framework-ui-runtime` (facade) | 1 / 3,272 / 35 | 4 / 63,655 / 1,309 |
| `-exchange` | 4 / 23,198 / 451 | 7 / 31,907 / 684 |
| `-presentation` | 10 / 108,982 / 1,649 | 14 / 121,968 / 2,035 |
| `-reconcile-primitives` | 6 / 61,637 / 1,269 | 12 / 74,817 / 1,492 |
| `-reconcile-cursor` | 1 / 96,156 / 1,294 | 7 / 166,192 / 2,798 |
| `-reconcile` (authority) | 2 / 90,141 / 1,820 | 19 / 193,789 / 3,423 |

The wasm32-unknown and wasm32-wasip2 lib closures are identical (the crates have no target cfg). Every row is inside 3500 lines / 28 files / 230,000 bytes; the largest are the authority test selection (3,423 lines) and the cursor test selection (166,192 bytes).

## 3. Crate graph (downward only; the DAG is also checked by the repo's cycle oracles inside the gate test)

```
exchange <- presentation <- primitives <- cursor <- reconcile(authority) <- facade
```

| Crate (lib root, package dir `<node>/📦️packages/🦀️rust`) | Owns | First-party deps (+ dev) |
| --- | --- | --- |
| `semio-framework-ui-runtime-exchange` (`RT/📡️exchange/🦀️.rs`) | gateway, inbox, presence (module files stay in place, mounted with `#[path]`) | ui-contract |
| `-presentation` (`RT/🖼️presentation/🦀️.rs`) | context, dispatch, entity, present (+ native resources), tracking; owns `[[test]] native_resource_custody` | value, ui-scene, job, ui-contract, exchange (dev: job testkit, trace, serde_json) |
| `-reconcile-primitives` (`RT/🧱️primitives/🦀️.rs`) | new modules `🗃️containers`, `🚧️limits`, `📊️census`, `🧩️records`, `🧰️testkit` plus the existing `♻️retirement/🌲️tree` | job, ui-contract, presentation (dev: serde_json); feature `testkit` |
| `-reconcile-cursor` (`RT/⏭️cursor/🦀️.rs`) | `SurfaceShadow`, `SurfaceReconcileCursor`, stage/step, the test oracle, `🧰️testkit` fixtures | job, ui-contract, presentation, primitives (dev: job testkit, serde_json, primitives testkit); feature `testkit` |
| `-reconcile` (`RT/♻️reconcile/🦀️.rs`) | thin `SurfaceReconciler`, credit reservation, handback registry, retained state, job, patches, terminal, test guard, `📤️output` pool | job, ui-contract, presentation, primitives, cursor (dev: same with testkit, serde_json); feature `testkit` |
| `semio-framework-ui-runtime` (facade, `RT/📦️packages/🦀️rust/🦀️.rs`) | explicit `pub use` of the 37 items consumers use, the cfg(test) transaction oracle | the four above (dev: ui-contract, job testkit, authority testkit, trace, serde, serde_json) |

Why this cut: primitives are the pure bounded building blocks (no registry, no job). The shadow and cursor share private state tightly, so they stay in one module file of one crate; the registry/job/patch cluster is intrinsically coupled to the thin owner's `Drop`. Facade surface: `PresenceHub`; `COMPONENT_TREE_PRODUCER_DEPTH`, `ComponentTree`, `ComponentTreeProducer(Step)`, `DEFAULT_REVISION_TOLERANCE`, `TreeNode`, `UI_NATIVE_MESH_MESSAGE_BYTES`, `UiNativeMesh*`, `UiNativeResource*`, `is_stale_intent`; `SURFACE_RECONCILE_{ADMISSION_SLOTS,AGGREGATE_BYTES}`, `SurfaceReconcile{Job,JobStep,OutputReservation,OutputTransfer,Outputs,PublishedAck,PublishedPatch,ReadyPatch,Rejected,Reservation,Terminal}`, `SurfaceReconciler`, `close_surface_reconcile_handback_one`, `surface_reconcile_registry_{census,test_guard}`; `SURFACE_RECONCILE_{FLOOR_BYTES,FLOOR_ITEMS,MAX_ITEMS,PAGE_BYTES}`, `SurfaceReconcileLimits`, `surface_subtree_items`. The list is the union of every `semio_framework_ui_runtime::` path in the six dependents (`uirt-tools/consumer_uses.py`); no consumer needed an edit.

## 4. Behaviour moves forced by the boundaries (no logic changed)

1. `SurfaceReconciler` fields and the `Drop`-time retirement moved as in section 1; the retire ladder is the same (scalar steps 0-4, then the handback release, then done).
2. The resident-credit helpers the cursor prices with moved to `primitives::limits`: `SURFACE_RECONCILE_FLOOR_{BYTES,ITEMS}` (+ their asserts), `surface_reconcile_limits_admissible`, `reserve_surface_reconcile_floor`, `release_surface_reconcile(_one)`. The authority's `reserve_surface_reconcile` is the same three steps (admissibility, static-backing registration, floor permit) composed from them.
3. `SurfaceReconciler::new(impl AsRef<str>)` under `cfg(test)` is replaced by a separate `from_text` (also on `SurfaceShadow`) under `cfg(any(test, feature = "testkit"))`, so a unified `testkit` build can no longer change the `new(SurfaceId)` signature the plugin crates call. 59 test call sites were renamed.
4. Every `#[cfg(test)]` test API of the split code became `#[cfg(any(test, feature = "testkit"))]` (oracle, `reconcile`, `snapshot`, `mark_rejected`, `transaction_reader`, `fail_*` fields, `admit_vec_backing`, ...). The facade and the authority enable them through dev-dependencies.
5. `EntityStore` gained `pending_notifications` / `pending_emit_sources`; the test-only transaction oracle used `pub(crate)` `store.effects` and now reads these.
6. `dispatch` imports `Command` from the exchange crate; the present-unit suite keeps its place but takes a local ledger guard (the reconcile registry guard is above it).
7. The shared test fixtures and the ledger guard became testkit modules (`primitives::testkit::registry_guard`, `cursor::testkit::*`, re-exported by the cursor).

Test suites (files stay where they were except the split halves):

| Suite | Before | After |
| --- | --- | --- |
| `🧪️tests/🔬️reconcile-unit` (1,127 lines) | one file | deleted; `🔬️cursor-unit` (shadow/cursor/oracle laws), `🔬️authority-unit` (job, ledger, handback, patch, tree-window laws), `🔬️primitives-unit` (census map page, `admit_vec_backing`) |
| `📏️ownership` tests (616) | one file | cursor part in place, 3 ledger/registry laws in `🔒️authority/` |
| `📃️document` tests (269) | one file | 3 cursor laws in place, 2 job laws in `🔒️authority/` |
| `♻️retirement/🌲️tree` tests (144) | one file | first law and helpers in place (primitives), 2 registry laws in `🔒️authority/` |
| `📤️output`, `🩹️patch`, `🚪️handback` | in place | in place (authority); only `SurfaceReconciler::new` -> `from_text` and one accessor |

## 5. Wiring (recipe steps)

- Manifests: five new `Cargo.toml` (role `framework`, ids `ui-runtime-<x>`, exactly the three `rust-source-boundary` keys at the ceiling, `[lints] workspace`, `[lib] path = "../../🦀️.rs"`, `workspace = "../../../../../.."`), the facade manifest rewritten (dependencies are now the four crates; ui-contract/job/value/ui-scene moved out or to dev).
- `📋️project.json` per new crate (`@semio-tech/ui-runtime-<x>-rs`, tags `scope:framework`, `language:rust`).
- `🧰️framework/Cargo.toml` members: five rows, byte-sorted (the six runtime rows are in byte order); the root `Cargo.toml` is untouched.
- `📋️build-owners.json`: five owner rows around `@semio-tech/ui-runtime-rs` (names unique).
- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: 30 routes each, `🦀️ semio-framework-ui-runtime-<x> 🏗️build|🔎️check|🧪️test 🛠️prototype|🚀️release`, orders `900.02301-10` (4_build), `922.05001-10` (4_gate), `903.59001-10` (4_test), checked unique within group in both files.
- UI crate-boundary corpus: five rows added to `🖱️ui/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json`.
- Facade `📜️script.ts`: `test` runs the six packages bottom up; `tree-retirement-check` runs the first law against primitives and the other two against the authority; `test-native-resource-custody` targets the presentation package.
- Source-text readers re-pointed: `📜️script.ts` interactivity audit and `interactivity-live-reconcile` test read the live reconcile as the authority + cursor + primitives roots (new `policyReadUiReconcileSource`; the three roots are registered in `POLICY_SPLIT_SOURCE_ROOTS`); `media-slots/🎟️collect` test reads the cursor root for the native-resource needles; `runtime-tree-retirement` TS oracle reads the primitives mount, the cursor `retire_one` and the authority inventory test. The mounted-frame-transaction reader keeps reading the facade root (still no production `mod transaction`).
- Taxonomy: no registry edit; the new folders add only the `directory-kind-unresolved` class that the runtime already reports.

## 6. Verification (every command run, with its actual result)

1. Baseline before the first edit: census row above; pristine copy of the tree from `git archive HEAD` in the scratchpad; baseline oracle rows `native 456 errors`, `test 1052`.
2. **Union differential** (`uirt-tools/union.py`, `rustc +nightly-2026-07-20 --emit=metadata`, analysis only, extern crates removed and external globs neutralised identically; each crate mounted as a module, `crate::` rewritten per owning crate, `pub(crate)` -> `pub(in crate::<crate>)`, so cross-crate item privacy is enforced):

   | cfg | errors (split) | residual non-external errors | baseline residual |
   | --- | --- | --- | --- |
   | native | 470 | 21 (`E0531` noise) | 21 |
   | `testkit` | 575 | 21 | n/a |
   | `--test` | 1,135 | 23 (21 + 2 `E0220` in the transaction oracle) | 23 |
   | `--test` + testkit | 1,135 | 23 | n/a |
   | wasm32-wasip2 | 470 | 21 | 21 |
   | wasm32-unknown-unknown | 470 | 21 | 21 |

   The first runs found real defects, all fixed: missing `HashSet` and `TreeNode` imports, dropped `const`/`struct` helpers in the split suites. Sensitivity (`🗑️generated/uirt-union-seeded.log`): removing `pub` from `SurfaceLinearMap` in the sandbox adds `E0603 struct SurfaceLinearMap is private`. Limit: removing `pub` from a method (`seal_step`) is not reported, because rustc stops after resolution when external crates are missing; member privacy is the definition scan's job.
3. **Definition scan** (`uirt-tools/defscan2.py`): non-`pub` fields/methods of lower crates named in downstream files, minus names the downstream crate defines or std provides. It found one real defect (the handback test read `current.retire_scalar`, now `shadow.retire_scalar()` through a testkit accessor); the 13 remaining hits are same-name fields/methods of other types (`entries`, `complete`, `nodes`, `closing`, `rows`, `cursor`, `field`). Seeded check: adding `shadow.retire_scalar` to an authority file makes the hit count 13 -> 14.
4. Dependency completeness (`uirt-tools/extdeps.py`): each crate's closure references only declared first-party/third-party crates (primitives lacked a `serde_json` dev-dependency; added).
5. `uirt-tools/verify.ts` (Bun TOML): all manifests parse, names/ids/boundary keys/lib paths/dependency paths/`[[test]]` path/feature edges resolve, workspace membership byte-sorted, owner rows, 30 launch commands per file present exactly once with unique orders (both files), corpus rows, every `#[path]` and `include_*!` target exists (49 `.rs` files, 42 mounts), **each mounted file has exactly one mount**, longest path 131 bytes. Result: `ALL STATIC CHECKS PASSED`.
6. `bun test ./🟦️.ts` in `🖱️ui/🧪️tests/📦️crate-boundaries`: 59 pass, 0 fail, 2,482 expects (TOML agreement with `@iarna/toml`, launch commands, dependency lists, 4000/262144/32 budgets for all five new rows, forbidden tokens, graphlib cycle oracle).
7. rustfmt parse (`rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true,unstable_features=true --emit stdout`) of all 29 changed or new `.rs` files: exit 0.
8. Losslessness (`uirt-tools/lossless.py`): the line multiset differs only by the mechanical edits listed in section 4 (renames, widened `pub`, split helpers). Region markers: 62 names before, 62 after, none missing, every file balanced.
9. TS: `testRuntimeTreeRetirement()` runs OK against the new paths; the media-slots test: 5 pass, 1 fail in a `ui-contract` needle (`pub struct UiTextOriginalCopy`) that this change does not touch; root `📜️script.ts` and the other edited TS files transpile.
10. A synthetic shape check (`uirt-tools/shape_check.rs`, no repo crate) compiled with `rustc`: the generic `retire_one<H>` with disjoint-field borrows, `take_all`, the owner `Drop`, `retire_one_detached` (`drop` as the release) and `map_or(.., Cursor::usage)`.
11. `bun ./📜️script.ts verify taxonomy report --scope RT`: 22 errors before, 37 after; the 15 additions are all `directory-kind-unresolved` for the new folders, one removed (`🔬️reconcile-unit`).
12. `bun ./📜️script.ts verify dependencies`: **not completed**. It was killed by my 590 s alarm (exit 142) while still running, so there is no verdict (`🗑️generated/uirt-deps-gate.log`). Run it once with a longer budget.

## 7. Residual risks (first real compile)

1. **Type-level and member-privacy errors are unproven.** The oracle cannot type-check with missing external crates. Highest-risk spots: `SurfaceShadow::retire_one<H>` call sites, the `Complete { shadow, patch }` consumers in the job, the authority tests' use of `cursor.usage()`/`.shadow.document()`, and glob-import ambiguity in `🔬️authority-unit` (`use super::*` + two crate globs; same-item globs are fine, but a real name clash would show on use).
2. `size_of::<SurfaceReconciler>()` can differ by padding (nested shadow). It feeds `seal_step`'s `fixed` bytes and `required_ready_transfer_bytes()`. Laws that pin exact arithmetic are expressed with `size_of` symbolically, but the exact-cargo laws should be run once.
3. `Cargo.lock` (root and framework) was not refreshed (launcher refuses). Expect five new packages; the facade's dependency list shrinks (ui-contract, job, value, ui-scene become dev or move down). A `--locked` resolve fails until then.
4. Test-only API now needs the `testkit` feature: the facade and authority dev-dependencies enable it. A workspace-wide `cargo test` unifies features across the workspace; that only adds hooks (`fail_*` fields, oracle) and no longer changes any production signature.
5. Pre-existing stale needles in the interactivity audit (`interactivityLiveReconcileFailures` still looks for strings such as `new_with_limits(tree, &current, limits)` and `SURFACE_RECONCILE_SURFACE_BYTES: usize = 8 * 1_024 * 1_024`) are unchanged: they already did not match HEAD. I only re-pointed the reader so it sees the same one text as before.
6. `🔒️dependencies.json` was not edited (generated baseline, written by the coordinating session); the new crates add `serde_json` users in dev-dependencies.
7. The authority test selection is the largest (3,423 lines); a further split would move `🔒️authority` registry laws into a registry crate, which needs the registry to leave the thin owner first.

## 8. Files

Created: `RT/📡️exchange/{🦀️.rs,📦️packages/🦀️rust/{Cargo.toml,📋️project.json}}`, `RT/🖼️presentation/{...same}`, `RT/🧱️primitives/{🦀️.rs,🗃️containers,🚧️limits,📊️census,🧩️records,🧰️testkit}/🦀️.rs` and package files, `RT/⏭️cursor/{🦀️.rs,🧰️testkit/🦀️.rs}` and package files, `RT/♻️reconcile/📦️packages/🦀️rust/{Cargo.toml,📋️project.json}`, `RT/🧪️tests/{🔬️cursor-unit,🔬️authority-unit,🔬️primitives-unit}/🦀️.rs`, `RT/{📏️ownership,📃️document,♻️retirement/🌲️tree}/🧪️tests/🔒️authority/🦀️.rs`.

Edited: `RT/♻️reconcile/🦀️.rs` (now the authority root), `RT/♻️retirement/🌲️tree/{🦀️.rs,🧪️tests/🌲️tree/🦀️.rs,🧪️tests/🔬️runtime-tree-retirement/🟦️.ts}`, `RT/🎯️dispatch/🦀️.rs`, `RT/🪪️entity/🦀️.rs`, `RT/🎭️present/🦀️.rs` (suite mount re-added), `RT/🔄️transaction/🦀️.rs`, `RT/📃️document` and `RT/📏️ownership` and `RT/📤️output` and `RT/🚪️handback` test files, `RT/🧪️tests/{🔬️present-unit,🔬️transaction-unit}/🦀️.rs`, `RT/📦️packages/🦀️rust/{Cargo.toml,📜️script.ts,🦀️.rs}`, `🧰️framework/Cargo.toml`, `📋️build-owners.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, `🖱️ui/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json`, `📜️script.ts`, `📖️source-access/🟦️.ts`, `interactivity-live-reconcile/🟦️.ts`, `media-slots/🎟️collect/🧪️tests/🟦️.ts`.

Deleted: `RT/🧪️tests/🔬️reconcile-unit/🦀️.rs` (split into the three new suites).

Consumers re-pointed: none needed (the facade keeps every name the six dependents use, `SurfaceReconciler::new(SurfaceId)` unchanged); the source-text readers listed in section 5.
