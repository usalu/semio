# Result: CRATE-CARVE-FB (prefix `fb`): `semio-framework-3d-brep-engine`

Executor: Sonnet 5.5, foreground, no sub-agents, no ticket tool, no modifying git command. Tools: `F/fb-tools/` (`carve.py`, `items.py`, `manifests.py`, `depscan.py`, `depcompare.py`, `apicmp.py`, `lossless.py`, `tomlcheck.ts`, `taxcheck.ts`, `fb-census.ts`, `fb-full-census.ts`, `pos/` = copy of the POS union oracle with sandboxes under `/Users/ueli/.cache/fb/`, `orig/engine/` = snapshot taken before the first edit). Logs: `F/🗑️generated/fb-*`.

## 0. Verdict: DONE (static evidence only)

The engine and the four new crates are inside the gate for lib and test. The engine no longer refuses any of its dependents. No real cargo ran (`📓️FLEET.md` has no `COMPILE ADMITTED` line), no test was executed. The exact real-check commands are in section 6.

| Crate (all under `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/`) | Before lib (files / bytes / lines) | After lib | Before test | After test |
| --- | --- | --- | --- | --- |
| `semio-framework-3d-brep-engine` (`📦️packages/🦀️rust`) | 8 / 272,018 / 3,934 OVER (bytes 1.038) | 6 / 188,297 / 2,798 (0.718) | 22 / 462,971 / 6,101 OVER (1.766) | 14 / 223,400 / 3,060 (0.852) |
| `semio-framework-3d-brep-kernel-contract` (`🧬️kernel-contract`) new | | 1 / 29,957 / 368 | | 2 / 30,397 / 381 |
| `semio-framework-3d-brep-mesh-io` (`📦️mesh-io`) new | | 1 / 38,485 / 537 | | 6 / 59,437 / 669 |
| `semio-framework-3d-brep-shape-value` (`🪅️shape-value`) new | | 2 / 17,685 / 269 | | 3 / 18,195 / 280 |
| `semio-framework-3d-brep-engine-tests` (`🧪️tests`) new | | 1 / 548 / 11 | | 11 / 157,752 / 1,920 |

All rows are inside the carving target (3,500 lines, 28 files, 230,000 bytes) except the engine test closure, which is 223,400 bytes (inside the target as well). Source: the canonical counter `readRustSourceClosureV1` through `fb-tools/fb-census.ts` (scoped, `🗑️generated/fb-census-after1.log`) and `fb-tools/fb-full-census.ts` (all 287 workspaces, 2,623 crates, `🗑️generated/fb7-crate-census.csv`). Before rows: `cgc7-crate-census.csv` and `fb-census-before.tsv`.

## 1. Design

The engine root owns `Brep` and every `impl Brep` block (registry, sync API, operation jobs, retention, retirement, analysis session). Inherent impls cannot leave the crate that defines the type, so the carve takes everything that does not need `Brep`'s private state:

| New crate | Content | Direction |
| --- | --- | --- |
| `kernel-contract` | `GeometryKind`, `GeometryHandle`, `BrepTopology`, `ClosestPoint`, `BrepError`, `SolidFaceLoops`, the `BrepKernel` trait, `map_err` (was a private engine fn, now `pub`, needed by the engine and by shape-value) | below everything; deps: representation, mesh-engine (the trait names `MeshExporter`/`MeshImporter`), value, value-derive |
| `mesh-io` | the former `📦️mesh-io` module (STL/OBJ/GLB, `MeshImportCursor`, `TriangleMesh`) plus `mesh_data_from_mesh_transfer` (moved from the engine root; it needs only `BrepError`, `MeshTransfer`, mesh-engine and the component-reference table) | above contract |
| `shape-value` | `ShapeValue`, `ShapeRoot`, `ShapeWire`, `ShapeComponent`, `ImportedShape`, `ShapeTessellationJob` and the `🎟️original` translation (the pure first 240 lines of the old `🪅️shape-value/🦀️.rs`) | above contract |
| engine (stays) | `Brep`, `BrepKernel` impl, codecs, operation jobs, retention, retirement, analysis session, new module `🚚️shape-exchange` (the `impl Brep { export_shape, import_shape, import_shape_mapped }` half of the old shape-value file, which needs the private registry) | above the three |
| `engine-tests` | black-box laws moved out of the engine test closure (sync API, operation jobs, shape value, session analysis) | dev-dependency on the engine only; lib is 11 lines |

The engine re-exports exactly the 35 root names it exported before (`apicmp.py`: old 35 = new 35, only-old and only-new both empty), as explicit `pub use` lists, so the 36 direct dependents (all in `✏️s`) and the tessellation dev-dependency need no change. Dependency direction is downward only; no new crate depends on the engine; the tests crate depends on the engine as a dev-dependency only.

Facade by design: the engine remains the public B-Rep API crate (36 direct and 160 transitive lib dependents), so re-pointing every consumer was not cleaner than an explicit re-export.

## 2. Files moved, split and edited

Module files stayed in the taxonomy node; the moved text is lossless (section 4). Changes:

- New: `🧬️kernel-contract/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`; `📦️mesh-io/🧪️tests/🖼️preview/🦀️.rs`; `🪅️shape-value/🧪️tests/🔬️unit/🦀️.rs` (one new law, below); `🚚️shape-exchange/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`; `🧪️tests/{🦀️.rs,🔭️api,⏱️operation-jobs,🪅️shape-values,🔎️analysis}/🦀️.rs`; four `📦️packages/🦀️rust/{Cargo.toml,📋️project.json}`.
- Edited: `🦀️.rs` (engine root, 2,202 → 1,817 lines), `📦️mesh-io/🦀️.rs`, `🪅️shape-value/🦀️.rs`, engine `Cargo.toml`, white-box test files `🧪️tests/🔬️unit/🦀️.rs`, `⏱️operation-jobs/🧪️tests/🔬️unit/🦀️.rs`, `🔎️analysis-session/🧪️tests/🔬️unit/🦀️.rs`.
- Registrations: `🧰️framework/Cargo.toml` (4 byte-sorted members, 3 `[workspace.dependencies]` tables), `📋️build-owners.json` (4 rows `@semio-tech/3d-brep-{engine-tests,kernel-contract,mesh-io,shape-value}-rs`, 2,876 unique names, json parses), `.vscode/launch.json` (24 routes, orders `900.9800n`, `922.9800n`, `903.9800n`, inserted after the `3d-testing-support` block; parses). `.vscode/🧩️launch.seed.jsonc` is untouched: it holds none of the producer routes (0 hits for `server-throttle` and every `semio-framework-3d-*`). No corpus row (UI-only).

Visibility changes forced by the crate boundary (the only API additions): `MeshImportCursor::cleanup_complete` (`pub(super)` to `pub`), new `MeshImportCursor::is_active` / `set_active(bool)` replacing the engine's direct write of the former `pub(super) active` field; `map_err` (`pub` in contract). Test-only: the white-box tests keep reading private session state inside the engine.

Test partition (67 test fns before = 67 after, identical name multiset): white-box laws that read `kernel.body`/`kernel.live`/`Entity` or mutate the body stay in the engine (5 root laws, `extract_and_absorb…`, one shape import law, one session validate law, the retention law); `brep_error_contract…` went to contract; the four preview/component-range laws went to mesh-io (fixtures stay at `⚙️engine/🧪️tests/🧫️fixtures`, so the TS readers keep their paths); everything else to `engine-tests`. Two mechanical rewrites in the moved operation-jobs tests: `snapshot` now uses `representation()` and `live_handles()` (the private pending-mutation counter is zero throughout those laws; the private variant stays with the white-box law), and one `kernel.body` read became `representation()`. One added law in shape-value: a value whose root does not resolve in its own body is refused with `shape value is inconsistent: root` (reasoned from `ShapeValue::check`, not executed). Include paths of moved test files were updated; 35 `#[path]`/`include_*!` literals in the engine tree resolve (0 missing).

Manifest cleanup of the engine: removed `semio-framework-mesh-component-references` and `semio-framework-value-derive` (no longer named anywhere in its closure); added the three new crates through `.workspace = true`. Dev-dependencies are unchanged.

## 3. cfg(test) rule (coordinator message)

No produced crate references another crate's `cfg(test)` item. Checked: the only `cfg(test)` items in the engine are `BTreeMap`, `tessellate_solid`, the retention test mount, the unit-test mount and `pub(crate) use semio_framework_3d_testing_support as test_support`; all are used only by tests inside the engine. mesh-io's `TriangleMesh::triangle_count` is used only by mesh-io's own tests. `engine-tests` imports from the engine only names from the non-test public set (`apicmp.py` equality) and from real lower crates; it uses neither `test_support` nor `tessellate_solid` of the engine. No `cfg(any(test, …))` was added. The union oracle sets `cfg(test)` globally, so I also confirmed by grep that no moved test file names `test_support`.

## 4. Verification (real results)

| Check | Command / tool | Result |
| --- | --- | --- |
| Lossless | `fb-tools/lossless.py` (non-blank line multiset of the snapshot vs the new tree) and a test-name count | 23 lines lost, all deliberate (moved import/mount lines, 4 changed include literals, `active`/`cleanup_complete` edits, the shape import list); 67 = 67 test fns with identical names |
| Union differential, brep stack + engine + 4 new crates (22 crates) vs HEAD `b84bf526c7b` (18 crates), `fb-tools/pos` (`pos.py run families/fb.json`, log `fb-pos-r4.*`) | cfgs lib, test, wasm32-unknown-unknown, wasm32-wasip2, wasip2 `--test` | NEW 0, GONE 0 in all five (1,320 / 2,971 errors on both sides, all stub-layer noise); typeck canary reached on both sides; privacy, trait-scope, resolve, type, other: 0 non-stub in each |
| Sensitivity | `--mutate` x4 (`fb-pos-m1.*`): private `ImportedShape.handle`, private `map_err`, private `is_active`, dropped `BrepKernel` import in the tests | caught: E0616, E0603 x2, E0624 x3, 31 trait-scope E0599 + 1 type |
| Findings of the oracle that were fixed | first runs (`r1`, `r2`) | missing `BrepKernel` imports in the moved tests; `MeshTransfer`/`SurfaceKind`/`TessellationStep` imports; kernel-contract lacked `semio-framework-mesh-engine` (the trait names its codecs); all now resolved |
| Dependencies vs sources | `depcompare.py` | named-but-undeclared crates: none in all 5 manifests; declared-but-unused: none (engine dev-deps `testing-support`, `trace`, `serde` are used only through macros or aliases, left as before) |
| Public API | `apicmp.py` | 35 = 35 names |
| Manifests | `tomlcheck.ts` | 5 manifests parse (Bun TOML), `workspace` resolves to `🧰️framework`, `[lib] path` and dependency paths exist, longest manifest path 128 bytes; the new members are byte-sorted (the member list has 9 pre-existing disorder pairs of peers elsewhere) |
| Taxonomy | `taxcheck.ts` (matchDirectoryKind) on 22 new directories + 16 earlier candidates | `checked=22 unresolved=0` (`crate-module-children`, `members-of-tests`, `packages`, `rust-language`, `tests`) |
| rustfmt parse | `rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true` on 18 files | 0 parse errors |
| Regions | script | 23 `.rs` files, 0 unbalanced `#region` pairs |
| Census | scoped and full (`fb7-*`) | section 0; full census 2,623 crates: the engine family has `libOver false`, `libCompilable true`, `testCompilable true` |
| Dependents | `fb7-crate-census.csv` vs `cgc7-crate-census.csv` | the engine had 160 transitive lib dependents (`blocksLib` 160, `blocksAny` 177 including test reach); 95 are now `libCompilable`, 65 remain refused only through other oversized crates (`libOver` of the engine family: all false; 0 regressions true to false). Totals: libCompilable 1,868 → 2,194 and testCompilable 1,811 → 2,115 (these totals include peer slices) |
| Dependency ratchet | `bun ./📜️script.ts verify dependencies` (`fb-verify-deps.log`) | exit 0, `309` current = `309` baseline, "clean — no new third-party dependencies" (no `write-baseline`; `🔒️dependencies.json` untouched, its `users` lists name the engine manifest for `parry3d`; the moved dev-dependencies add users `mesh-io` and `engine-tests` on the next baseline write) |

## 5. Not done / limits

- No cargo ran: `COMPILE ADMITTED` is absent. Tests (including the new shape-value law) are type-checked by the oracle only, never executed.
- `Cargo.lock` refresh is pending: `🧰️framework/Cargo.lock` plus 22 lock files that contain `semio-framework-3d-brep-engine` (`✏️s/Cargo.lock`, 18 under `✏️s/🔌️plugins`/`✏️s/🧑‍💻dev`, `🌎️hub/Cargo.lock`) need the new packages; run `update --workspace --offline` per workspace in the compile window. Not hand-edited.
- The six launch routes use `--lib`, so the white-box and black-box test routes run the lib tests of each crate (the tests crate keeps its laws in its lib closure, so `--lib` covers them). The engine itself still has no routes (like its sibling brep crates).
- The union oracle stubs everything outside the 22 crates; field and method privacy beyond the canary-reached typeck is covered by the oracle's privacy class (0) and by the sensitivity mutations, not by rustc against real dependencies.
- `carve.py` must not be re-run (the state file is stale after the manual shape-value edit); the live tree is the source of truth.

## 6. Commands for the compile queue

```
cd /Users/ueli/Documents/semio
bun repo cargo --store=shared --configuration=prototype -- update --workspace --offline --manifest-path 🧰️framework/Cargo.toml
bun repo cargo --store=shared --configuration=prototype -- check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-3d-brep-kernel-contract -p semio-framework-3d-brep-mesh-io -p semio-framework-3d-brep-shape-value -p semio-framework-3d-brep-engine -p semio-framework-3d-brep-engine-tests --tests --keep-going --message-format=short
bun repo cargo --store=shared --configuration=prototype -- test --manifest-path 🧰️framework/Cargo.toml -p semio-framework-3d-brep-kernel-contract -p semio-framework-3d-brep-mesh-io -p semio-framework-3d-brep-shape-value -p semio-framework-3d-brep-engine -p semio-framework-3d-brep-engine-tests --lib
bun repo cargo --store=shared --configuration=prototype -- check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-3d-brep-engine --target wasm32-wasip2
```
Then refresh the lock files of the dependent workspaces and re-run `verify dependencies write-baseline` only if the coordinator wants the new `users` rows recorded.
