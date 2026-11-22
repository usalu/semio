# Native Rust Build Report

Native dir `🗑️generated/native` is 58 GB at the end (cap 60 GB); disk free 45 GB. Tests needing bun ran with the bun runtime dir on PATH.

Env for every command (via `🗑️generated/ncargo.sh`): `CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=T/target CARGO_BUILD_BUILD_DIR=T/build CARGO_PROFILE_{DEV,TEST}_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off`, `-j 4`, T = `🗑️generated/native`.

## 1. Hub composition (DONE, passes)
Command (cwd `🌎️hub/🧩️compositions/🏭️process/🧩️component/📦️packages/🦀️rust`):
`cargo build --target wasm32-wasip2 --profile wasm-prototype --config profile.wasm-prototype.inherits="dev" --config profile.wasm-prototype.opt-level="s" --config profile.wasm-prototype.codegen-units=16 --config profile.wasm-prototype.lto="thin" --config profile.wasm-prototype.incremental=false`
Result: exit 0. Artifact (400 MB):
`🗑️generated/native/target/wasm32-wasip2/wasm-prototype/semio_hub_process_component.wasm`
The plain dev-profile build compiles all crates but fails at link: `wasm-component-ld ... functions count exceeds limit of 1000000`. The repo's `prototype-component` profile (opt-level s, cgu 16, thin LTO, no incremental) is required, as the schema in `🦑️repo/.../🦀️profile/🧬️schema/🔣️.json` states. `cargo test` (native, same dir): exit 0, 1 passed, 0 failed.

Root causes fixed to get there:
- `🧬️schema/🤖️generated/🏷️entity-kinds` missing: ran the generator (`readEntityCatalog` + `generatedTargets` from `🧬️schema/🏷️entity-kinds/📋️plan`) directly with bun (`🗑️generated/gen-entity-kinds.ts`).
- Stale styling tokens (`chrome_metrics::TRANSITION_*`, `DIALOG_ENTER_*`): re-ran `gen-styling.ts`; `🔣️.json` was newer than the generated `🔤️tokens/🦀️.rs`.
- Duplicate `use semio_plugin::WindowLayoutNode;` lines (3x) in 60+ `👁️viewer/🎭️modes/👁️view/🦀️.rs` and `✏️editor/🎭️modes/✏️edit/🦀️.rs` files across `✏️s/🔌️plugins/**` (E0252). Looks like a runaway codemod by another agent; deduplicated all repo-wide (`perl` on consecutive duplicate `use` lines). It may recur.

## 2. process3d (`cargo test --workspace --no-fail-fast`) - PARTIAL
Green: `-p ...-mutations` (113 unit tests + `process3d_resize_stock` 8 tests incl. the no-op arm), `-p ...-model` (all targets incl. `process3d_io_sqlite_snapshot`, 11 tests), `-p ...-host` 4 of 6 tests (235 pass / 2 fail over these three packages).
Still failing (framework runtime, not process3d logic):
- host `retained_laws::every_store_replacement_phase_unit_fits_the_interactive_step_budget` and `actual_atomic_publication_is_fail_closed_and_retires_stale_candidate`: `ArtifactStoreInitializationAuthority::terminal_is_empty` is false after `take_candidate` (`retained-laws/🦀️.rs:239`).
- editor package (`semio-s-artifact-process-process3d-editor`, lib + 7 integration targets, ~89 tests failing, ~95 passing): the framework test harness `settle_registered_typed_operation` stops after 4125 turns ("fuel 2000000 ... close owner Begin", WorkLimit; 48x), "original acceptance pump failed" (16x), "UI arena has no accepting lifetime registration" (16x, `🖱️ui/🧬️contract/🌱️values/.../🏟️collection-arena`), plus a few archive-door faults. The store close never leaves its Begin phase under the harness grant (max_copy 32768); owners elsewhere (Jack) pad copy bytes to 65536 for self-funded turns. Not fixed: needs a framework decision.

Root causes fixed and files changed (all under `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d`):
- `🔨️modules/🏠️host/🧰️owned/🧪️tests/🔬️retained-laws/🦀️.rs` and `.../🧫️fixtures/🔣️.json`: ported to `AuthoringWallet` effects (`cold`, `drive`, `body_grant`); `maximumAdvertisedTurns` 8 -> 16 (measured 9 turns).
- `🔨️modules/🏠️host/🧰️owned/🦀️.rs`: `process3d_demand_grant` pads `maximum_copy_bytes` to 65536 (root admission needs it).
- `🏅️standards/.../✏️editor/🌉️wasm/🧪️tests/🔬️unit/🦀️.rs`: `close_step_cold`.
- `🏅️standards/.../🧬️schema/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️retained-mounted-laws/🦀️.rs`: effect-taking `reserve_retained_allocation` / `close_step`, `crate::snapshot` paths.
- `🏅️standards/.../✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`, `🧮️census/🦀️.rs`, `🚦️public/🦀️.rs`: new output-slot API for `decode/encode_sqlite_snapshot_native` and `export/import` (helpers `decoded`, `encoded`, `stage_kind`), and a stale `../../🏅️standards/...` path to the bun support script fixed to `../../../🚪️io/...`.
- `🧪️tests/🔬️unit/🦀️.rs` + `🧫️fixtures/🔣️.json`: `insert_step_emit` returns `Result`; the binary mutation is now one canonical Pack document, so the LEB128 oracle test became `delete_step_binary_is_one_canonical_pack_document` (fixture `hex` replaced by `packMagicHex`).
- `🏅️standards/.../🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `every_mutation()` lacked the new `ResizeStock` variant (kinds() 16 vs 15).
- Repo-wide: deduplicated `use semio_plugin::WindowLayoutNode;` (see below).

## 3. Extensions (`cargo test --workspace`) - PASS
metal 5 passed, wood 6 passed, robotic 5 passed, concrete 5 passed (0 failed each). No source changes.

## 4. Bridge `cargo build` - PASS (exit 0).

## Source of the tripled `use semio_plugin::WindowLayoutNode;`
No checked-in generator or script contains that line; none found in any ticket folder. The 60+ mode files (`👁️viewer/🎭️modes/👁️view`, `✏️editor/🎭️modes/✏️edit`) were rewritten at ~04:43-04:47 by an ad-hoc, non-idempotent import-adding codemod run three times, almost certainly the window-layout migration in ticket `26/09/17/WGPU-RENDERER-REACT-PARITY` (notes `📓️opus55-window-layout-*.md`, 04:43). Not recurred after the 04:53 dedup (no file has >1 such line now).
