# Result law4: production and structural fixes behind the still-red law suites

Executor Sonnet 5.5, foreground, no sub-agents, no cargo/rustc, no modifying git command. Runner: `law4-tools/one.py <suite> [-t name]` (law2 runner clone: emulated hoist, `SEMIO_SCRIPT_PROCESS_INVOCATION`, 120 s timeout). Rust syntax of every edited `.rs` was checked with `rustfmt --emit stdout --config skip_children=true` (parse only, exit 0). Nothing Rust was compiled.

## Outcome per item (before -> after)

| Item | Before | After | Status |
|---|---|---|---|
| `✏️s/🧪️tests/🗂️native-ownership` (energy epw workspace) | 3 pass / 1 fail | 4 / 0 | done |
| `✏️s/🔌️plugins/🧪️tests/📁fixture-placement` | 35 / 4 | 38 / 1 | 1 left: two blessed output dirs need a cargo run |
| `PL/🗄️stdio/…/📖️pdf/…/🪪️stream-roles` | 5 / 1 | 6 / 0 | done (Rust law test written, uncompiled) |
| `REPO/…/🔌️plugin/🧪️tests/📦️units` | 5 / 8 | 12 / 1 | 1 left: 171 stale consumer `Cargo.lock` files (cargo) |
| `OS/📡️spr/📜️history/🛬️decode/🧪️tests` | 0 / 1 | 0 / 1 | not done (see below) |
| semio `🫙️prefix` | 25 / 22 | 25 / 22 | not done (see below) |
| tree-sitter async-closure grammar blocker | blocked | blocked | no pinned fix available; recorded for deps |

## 1. Energy workspace membership (done)

Cause: `🔋️energy` listed the removable child `🧩️extensions/🌦️epw` as a member and as a provider path (violates the standing "extensions are removable child workspaces" law).

- `🔋️energy/Cargo.toml`: dropped the epw member, `semio-s-plugin-energy-epw` and `semio-s-artifact-stdio-epw` providers (the latter had no other user; checked with `git grep`).
- New `🔋️energy/🧩️extensions/🌦️epw/Cargo.toml`: own `[workspace]` (flow `🧮️math` pattern), its five workspace providers re-pathed relative to the epw dir, lints/package table copied; `📦️packages/🦀️rust/Cargo.toml` `workspace = "../.."`.
- Locks without cargo: `law4-tools/locksplit.py` + `energy-epw-split.py` parse `Cargo.lock`, compute reachability, and write (a) the epw lock as the reachable closure of the epw package (423 packages), (b) the energy lock pruned to what remains reachable from its other members (443 -> 424; the `stdio-epw*`/`stdio-contract*` family and the epw package dropped). Round trip of the parser/writer on the unmodified energy lock is byte identical. The model artifact keeps its path dependency on the epw crate (unchanged, still valid).
- Added `✏️s/🔌️plugins/🔋️energy/🧩️extensions/🌦️epw` to the `exclude` lists of `Cargo.toml` (root) and `✏️s/Cargo.toml`.
- Verify: native-ownership 4 pass / 0 fail. Not verified: `cargo metadata --locked` on the two locks (hand-derived; a cargo owner should run `cargo metadata --offline` once and expect no lock diff).

## 2. Fixture-placement migration (3 of 4 failures closed)

Rule used: ticket 26/05/30 FIXTURES-ARE-TESTING-EXAMPLES-ONLY canonical placement `D/🧪️tests/🧫️fixtures`, no `🧬️schema`/`🖼️assets` ancestor.

- rule 1 (13 fixtures outside tests): `📐️tessellate` fixture -> `📐️tessellate/🧪️tests/🧫️fixtures`; jack `rich-read` + `query` out of `🧬️schema/💡️inferences/🧪️tests` -> `✳️any/🧪️tests/🧫️fixtures/💡️inferences/*` (+ `.rs`, `.ts` readers and the `🧑‍💻dev/🔱️trinity/🪆️graph` Rust reader); gif 87a/89a diff fixtures -> subset `🧪️tests/🧫️fixtures/🔺️diff`; 7 TypeScript public fixture-entry modules (`🌀️procedural` sqlite, `🏗️fem` x3, `📏️layout`, `🖨️raster`, `🗄️stdio` registry part21, zip opc, jpg components) moved from `🧪️testing/🧫️fixtures` to `🧪️tests/🧫️fixtures` inside their package (relative depth unchanged, specifiers unchanged).
- rule 2 (include in non-test source): layout `intrinsic` test pointed at a fixture deleted by the 7880cc36 carve; restored `🧾️dictionary/🔣️.json` from `7880cc36^` to `✳️any/🧪️tests/🧫️fixtures/🧾️dictionary` and re-pointed the include. `generation3d` host: `generation3d_all_retained_mutation_fixtures_for_test(corpus: &str)` now takes the corpus, the `include_str!` lives in the `🧪️tests` caller. bim `element-solids`: the `#[cfg(any(test, feature = "testkit"))] pub mod fixtures` body moved to `🧪️tests/🧰️testkit/🦀️.rs` (mounted with `#[path]`). stdio registry `🧰️testing/🔀️routed-io`: removed the unused `SHIPPED_FLEET_SQLITE_OWNERS` const and its re-export.
- rule 4 (manifest readers): the law rows named stale aggregator manifests. `law4-tools/crateof.ts` resolves the compiling crate through `#[path]` mount chains and `Cargo.toml` lib/test paths; 19 rows re-pointed to the real crate manifest in `📁fixture-placement/🧫️fixtures/🔣️.json` (the Rust literals were already correct for that crate).
- Package edits: 7 `package.json` `exports` paths (only the path values; listed below). Kept per coordinator freeze message; no further manifest edits.
- Still red (1 test): rows for `🧊️gltf` (`🧪️tests/🧫️fixtures/🧊️gltf/🪑️components`) and `🏗️ifc/🛝️ramps` read directories that the Rust tests write under `BIM_BLESS=1` (glb/ifc bytes). Needs a cargo run of the two export tests with `BIM_BLESS=1`, then commit the blessed files.

## 3. pdf stream-roles (done, uncompiled)

`PdfDiff::between` was deliberately deleted in da607013 (diffs are mutation-built now), so the law that demanded it was stale (decision c). Added `pdf_stream_roles_multi_object_reorder_delete_and_path_refusal` to `🪪️stream-roles/🧪️tests/🦀️.rs`: two owners from the replay fixture, role reorder via `PdfDiff { admitted_stream_roles: Some(..) }` + inverse, object removal through `special_edit` refused without roles then accepted with `removed` role indices, retained owner 501/8, physical byte edit refused, non-empty path refused. TS law now requires `PdfDiff {` and `protocol::apply_diff` instead of `PdfDiff::between`. Needs `cargo test` by a cargo owner (types such as `RemoveObject`, `PdfIndexedDiff { removed: Vec<usize> }` were read from source, not compiled).

## 4. `📦️units` (12 of 13 tests green)

Cause: the framework crates moved into the root workspace (`package.workspace` = repo root) while the law still read `🧰️framework/Cargo.toml`/`🧰️framework/Cargo.lock`, and the `🧪️testing` facade was carved into `🧪️artifact-app-laws/*`.

- Re-pointed workspace and lock paths to the root; the facade assertions now follow the `#[path]` closure (`🧪️testing/🦀️.rs` + `🧪️artifact-app-laws/**`, within 32 files/256 KiB/4000 lines); `#[path="."]` mounts skipped in the surface-fixture walker.
- Re-blessed data that is a pure snapshot of text that legitimately changed: child-group remaining function hashes (3 functions: include depth after the carve, `contract: Some(..)` added), `feature-owner.remainingFunctions` (147 names unchanged, +9 new, sha refreshed), `feature-consumers.dependencies` (2d -> 2d-compute, +document-media from the fresh root lock), ownership dependency list (+job-worker, +job-worker-authority, +pack, taken from the crate manifest).
- Retired one-shot whole-file hashes (`afterSha256`, `thirdpartyPinsSha256`, `sha256`) in the lock tests; pins now compare with the standing `feature-locks.json` entry; the "unresolved Source frontier" tests became "resolved" (`missing*` lists empty, workspace lock now listed in `locks`). Schemas updated accordingly.
- Open (1 test, `actual feature verdict owner …`): 171 of 174 consumer locks (`law4-tools` run, list = every lock in `📋️locks.json` except root `Cargo.lock` plus two) still carry the old `semio-framework-plugin-ownership` dependency list. The ownership manifest adds three path deps; the stale locks also miss unrelated registry crates (e.g. tsv misses `serde`), so a faithful fix is a cargo lock refresh, not hand editing. With `locks` restricted to the fresh root lock the whole test passes (checked with a temporary variant, removed).

## 5. spr decoder (not done)

`RetainedHistoryDecode` still decodes whole frames (`decode_frame`, `maximum_records`); the law wants the granted, borrowed-range receiving shape built on `RetainedWireFieldDecode`, `RetainedSourceDictionary`, `OriginalWireSourceIdentity` (pieces exist in `protocol::codec`). The design table in `NON-DESTRUCTIVE-HISTORY-EDITING/📓️2026-10-10-core-history-frontiers.md` marks ten of thirteen frontiers "Pending" (Edit, Transition, Document, Whole HistoryLog, Exact validation). That is a multi-record Rust redesign (~13 KB decoder plus children) that cannot be written or checked without cargo; attempting it blind would risk the os history crate. Next owner: implement per that table, then the TS law (`decoder.includes(...)` proofs) goes green unchanged.

## 6. semio `🫙️prefix` (not done)

Needs `fn binary_into`, `fn document_into`, `workspace::decode(` in each of the 18 `🛬️native/🦀️.rs` and no `Owned::new(`; graph/table/value never mount `🫙️projection`; base encoder lacks `EncodingPrefix`. flow and the 18 `💰️reconstruction` modules were finished by c44e9645. This is a codec rewrite across 18 domains over the native decode control; not safe without a compiler. The law fixture (`🧫️fixtures/🫙️prefix`) already pins the grants/stops.

## 7. tree-sitter grammar (no pinned fix)

- Store/caches hold only `tree-sitter-wasms@0.1.13` (`web-tree-sitter` 0.20.8 and 0.27.0); other copies under `~/.cache/*/nm` and `.🧬semio/**` are the same 0.1.13. VS Code's bundled `copilot/dist/tree-sitter-rust.wasm` was probed: it also reports `hasError` on `async |a,b| a+b` (and it would be an external, unpinned dependency anyway).
- The repo's own Rust syntax module (`@semio-tech/compiler-syntax-rust-ts`) is a tokenizer, not a parser; the affected laws walk `impl_item`/`function_item` nodes, so it cannot replace tree-sitter. `rustfmt --emit stdout` parses async closures (nightly 2026-07-19, repo precedent in several laws) but only answers "parses", not the AST queries.
- For deps owner: need a `tree-sitter-rust` wasm supporting async closures (grammar >= the release that added `async` closure expressions) pinned via `tree-sitter-wasms` or a vendored wasm; affects `📑️tsv/…/📸️snapshot` (1 test) and `🏪️store/📜️space-history/…/📸️snapshot` (3 tests), plus any unit law with `hasError()` over `0c3e7894` sources. No `package.json` edit made for this.

## Files changed

Energy: `✏️s/🔌️plugins/🔋️energy/Cargo.toml`, `…/Cargo.lock`, `…/🧩️extensions/🌦️epw/Cargo.toml` (new), `…/🌦️epw/Cargo.lock` (new), `…/🌦️epw/📦️packages/🦀️rust/Cargo.toml`, `Cargo.toml`, `✏️s/Cargo.toml`.

Placement: `🌊️flow/🧩️extensions/📐️brep/📐️tessellate/{🧪️tests/🟦️.ts,🧪️tests/🧫️fixtures/🔣️.json}`; jack `…/✳️any/{🧪️tests/🧫️fixtures/💡️inferences/**, 🧬️schema/💡️inferences/🧪️tests/🪆️query/{🦀️.rs,🟦️.ts}}`; `✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/🧪️tests/🦀️.rs`; gif `9️⃣89a/…/🧱️base` and `7️⃣87a/…/✳️any` (`🧪️tests/🧫️fixtures/🔺️diff/🔣️.json`, `🧬️schema/🔺️diff/🧪️tests/🔬️unit/🟦️.ts`); fixture entry modules moved plus `package.json` of `🌀️procedural/📦️packages/🟦️typescript`, `🏗️fem/…`, `📏️layout/…`, `🖨️raster/…`, `🗄️stdio/📇️registry/…`, `🗄️stdio/🗿️artifacts/🎒️zip/…`, `🗄️stdio/🗿️artifacts/📸️jpg/…` (the exports batch the coordinator wanted recorded); layout `…/✳️any/🧪️tests/🧫️fixtures/🧾️dictionary/🔣️.json` (restored) and `✏️editor/🧪️tests/🔬️unit/🎞️intrinsic/🦀️.rs`; `🌀️procedural/…/🧊️generation3d/🔨️modules/🏠️host/{🦀️.rs,🧪️tests/🔬️retained-authority-laws/🦀️.rs}`; bim `…/🧬️schema/💡️inferences/🧊️element-solids/{🦀️.rs,🧪️tests/🧰️testkit/🦀️.rs}`; stdio registry `🧬️contract/🧰️testing/{🦀️.rs,🔀️routed-io/🦀️.rs}`; `📁fixture-placement/🧫️fixtures/🔣️.json`.

pdf: `…/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🪪️stream-roles/🧪️tests/{🦀️.rs,🟦️.ts}`.

units: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/{🟦️.ts, 📋️child-group-lock, 📋️child-group-owner, 📋️feature-consumers, 📋️feature-owner, 📋️feature-workspace-lock, 📋️surface-carrier-lock, 📋️surface-fixture-lock, 🧬️child-group-lock, 🧬️feature-workspace-lock, 🧬️surface-carrier-lock, 🧬️surface-fixture-lock}.json`.

Tools (inputs, kept): `law4-tools/` (`one.py`, `loc.sh`, `locksplit.py`, `energy-epw-split.py`, `placement.ts`, `rule2.ts`, `rule4*.ts`, `crateof.ts`, `owner.ts`, `units*.ts`). Generated output folder removed.

## Open for others

- cargo owner: refresh the 171 stale consumer locks (`📦️units`), `BIM_BLESS=1` the gltf components and ifc ramps fixtures, `cargo check`/`test` the pdf law test, `cargo metadata` the energy/epw locks, spr decoder and semio prefix implementations.
- deps owner: tree-sitter-rust wasm with async closures.
