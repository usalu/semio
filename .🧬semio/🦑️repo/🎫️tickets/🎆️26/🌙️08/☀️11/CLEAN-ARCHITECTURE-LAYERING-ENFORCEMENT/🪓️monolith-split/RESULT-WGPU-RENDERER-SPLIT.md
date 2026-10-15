# 🪓️ Result: wgpu Renderer Split (file A)

Executor: Sonnet 5.5. Seam map: `SEAM-MAP-WGPU-RENDERER-AND-SCENES.md` section A.
Paths are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/` (`E/`) unless absolute. `R/` = `E/🎯️targets/🧊️wgpu/🧊️renderer/`.

## 0. Status in one paragraph

The structural split is done and statically verified (lossless, syntax-valid, every path literal resolves). **It has NOT been compiled.** No `cargo check` could run for the whole session, for a reason outside my files: the compiler-ingress gate refuses every crate larger than 256 KiB / 32 files / 4000 active lines (`⚡️caching/🔣️policy.json` `storage.sourceBoundary`) before rustc starts, and the dependency `semio-framework-3d` (3.2 MB of Rust) is refused first, so even `-p semio-framework-os-renderer-wgpu` never reaches the compiler. No baseline exists either. Visibility widenings are static estimates, not compiler-required ones (section 5). The first real compile will most likely still report some privacy errors; section 9 gives the exact recipe and an auto-fixer for them.

## 1. Final line counts

| File | Lines |
|---|---|
| `R/🦀️.rs` (parent, was 19,807) | 1,006 |
| `R/🗂️asset-authority/🦀️.rs` (A1) | 755 |
| `R/🦴️glb-decode/🦀️.rs` (A2) | 2,416 |
| `R/🔁️asset-decode-jobs/🦀️.rs` (A3) | 975 |
| `R/📦️kernel-runtime/🦀️.rs` (K0, was inline `kernel_runtime`) | 81 |
| `R/📦️kernel-runtime/🧮️kernel-core/🦀️.rs` (K1) | 879 |
| `R/📦️kernel-runtime/📬️kernel-replay-client/🦀️.rs` (K2) | 1,388 |
| `R/📦️kernel-runtime/🗃️kernel-retained-state/🦀️.rs` (K3) | 938 |
| `R/📦️kernel-runtime/🏊️kernel-pool/🦀️.rs` (K4) | 2,253 |
| `R/📊️scale-bench/🦀️.rs` (S1, was inline `pub mod scale_bench`) | 893 |
| `R/🛎️frame-actions/🦀️.rs` (R1) | 1,106 |
| `R/🎨️presentation-raster/🦀️.rs` (R2) | 844 |
| `R/📮️mailbox-impl/🦀️.rs` (R3) | 1,605 |
| `R/🧾️frame-build/🦀️.rs` (R4) | 1,457 |
| `R/🖥️frame-presentation/🦀️.rs` (R5) | 1,424 |
| `R/🧠️app-runtime/🦀️.rs` (R6) | 1,462 |
| `R/🔬️native-socket-probe/🦀️.rs` (R7) | 435 |

16 new files, all between 81 and 2,416 lines; the parent is 1,006 lines (target was about 1,000). The deepest new path is 133 characters from the repo root (limit 256). Folder emojis were checked against `\p{RGI_Emoji}` with FE0F in bun: all 16 pass, all distinct, none collides with `📇️registry` / `🪶️sqlite`.

Parent contents: the original header and `mod` declarations, the 16 new `#[path]` declarations with `pub(crate) use <mod>::*;`, the crate-root `use` block, `ActorBudgetBridge`, task/log helpers, Host appearance/introduction/platform/locale, the five test-mod declarations, `BootDescriptor`, and the wasm export. Child boundaries differ from the seam map in three places, all to keep a leading attribute or doc comment with its item (see section 7).

## 2. Created and edited files

Created (all `R/…/🦀️.rs`): the 16 files in section 1.

Edited:
- `R/🦀️.rs` (the parent).
- 8 source-text test files (section 6): `E/🧪️tests/🧾️frame-action-ledger/🦀️.rs`, `…/🔬️wgpu-renderer-async-boundary/🦀️.rs`, `…/🧊️wgpu-renderer-standalone/🦀️.rs`, `…/🫀️settle-pump/🦀️.rs`, and in `E/🧱️elements/🐚️Shell/🧪️tests/`: `🎡️wgpu-wheel-and-escape-routing`, `🔄️sync-owner-lifetime`, `🔀️wgpu-document-relay`, `🌓️appearance-tour-and-footer-pills`. One line each.

Not touched: Scenes (B), Shell `🦀️.rs`, the lib root `📦️packages/🦀️rust/📚️library/🦀️.rs`, `Cargo.toml`, any other crate.

## 3. How the `include!` / `#[path]` question resolved

A is `include!`d into the crate root. I did not need a compiler to settle where `#[path]` resolves inside it: A's existing header `#[path = "../../../🧱️elements/🛰️Dock/…"]` only makes sense relative to A's own directory (relative to the lib root it would point outside the package), and `include_str!` at the old line 9822 likewise. So child `#[path]` literals in A are relative to `R/` and are written as `#[path = "🗂️asset-authority/🦀️.rs"]`; nested ones in `K0` are relative to `R/📦️kernel-runtime/`. This is an inference from existing, compiling code, **not** a compile result.

Wiring chosen:
- Every A child: `#[path = "<folder>/🦀️.rs"] mod <snake>;` + `pub(crate) use <snake>::*;` in the parent. `🔬️native-socket-probe` uses `pub use` (it owns `run_native`, `run_smoke`, `run_socket_grant_probe`, `upload_icon_atlas`, so those public paths are unchanged). `📊️scale-bench` stays `pub mod scale_bench` (gated `cfg(not(wasm32))`) with its region markers. `kernel_runtime` stays `pub(crate) mod kernel_runtime` (gated `cfg(not(wasm32))`), no re-export at the root, exactly as before.
- Children start with `//! <folder emoji> …` then `use super::*;`. The two ex-inline modules (`kernel_runtime`, `scale_bench`) keep their own `use` block instead of `use super::*;` (they never saw root names unqualified), and their old `///` module docs became `//!` under a new emoji-first line.
- The `#[path = "."]` trick on the inline `kernel_runtime` is gone; the file-level `#[path]` replaces it. Inner bodies of `kernel_runtime` and `scale_bench` were dedented by four spaces (verified none of the dedented lines is inside a multi-line string or block comment).
- `K1`–`K4`: all pre-existing `pub(super)` became `pub(crate)`. This is semantics-preserving: in the inline module `pub(super)` meant "visible in the crate root", i.e. crate-wide, and one level deeper `pub(super)` would have shrunk to `kernel_runtime` only.

## 4. Path-literal rewrites (all verified to resolve to existing files)

Rule: one extra `../` per directory level moved down. I checked in the tool that the rewritten literal resolves to the same absolute path as the old one, and afterwards that all 90 `#[path]`/`include!`/`include_str!`/`include_bytes!` literals in the parent and the 16 children exist on disk (0 missing).

| Child | Literal (old → new, abbreviated) |
|---|---|
| A3 | `../../../🧪️tests/🔬️wgpu-renderer-renderer-io-retained/…` → `../../../../🧪️tests/…` |
| K2 | `../../../🧪️tests/🗞️typed-result-page/…` → `../../../../../🧪️tests/…` |
| K4 | `include_str!("../../../../../🔌️plugin/…/🔣️.json")` → seven `../` |
| K4 | `include!("../../../🧪️tests/🔬️wgpu-renderer-kernel-runtime-semantic-document/…")` → five `../` |
| S1 | `include_str!` (inside a function) `../../../../../🔌️plugin/…` → six `../` |
| R2 | `#[path = "../../../../../../../../🔨️modules/🖱️ui/…/📥️enqueue/…"]` → nine `../` |
| R6 | `include_bytes!("../../…8…/🔨️modules/🖼️assets/🔤️fonts/🚀️anta/…/🔤️outline.ttf")` → nine `../` |
| R7 | `#[path="🪶️sqlite/🧪️tests/🦀️.rs"]` and `#[path="🪶️sqlite/🦀️.rs"]` → `../🪶️sqlite/…` |

The parent's own literals (lines 1–217) are unchanged.

## 5. Visibility widenings (static, not compiler-driven)

Rules from the task say "only where the compiler requires". With no compiler available I widened by static analysis, so the set is a **superset of what is probably required and may still miss some**. Nothing was narrowed or made `pub` beyond `pub(crate)` (`pub(super)` in `kernel_runtime` children).

1. Top-level items used from another partition (word match on comment- and string-stripped text, across all children and the parent): 128 items in the A children, 49 in the K children, via a table from `🗑️generated/renderer-widen-plan.json`. Cross-checked afterwards: 0 private top-level items remain that are referenced from another file of the tree.
2. Top-level items referenced from outside the tree by `crate::name`, `super::name`, `use crate::{…}` or by name in the crate-root-mounted test files (`🧪️tests/…`): 31 items (for example `PngStructureCursor`, `RENDERER_IO_SLOTS`, `AppPresentPhase`, `GLB_OUTLINE_SCALE`, `boot_runtime`, `NativeSocketProbeSnapshot`).
3. Private fields and inherent-impl methods/associated constants: widened when the owner type's name and the member name (as `.name`, `::name`, `name:` or `{ name }`) both appear in a different file of the tree or in an external file of the crate. 720 members (715 fields/methods, 5 tuple fields such as `RuntimeMailbox(Arc<…>)`). Trait-impl methods were skipped. This is the least precise bucket (name-level plus type mention); expect some members widened that the compiler would not have needed and a few it would still require.
4. `thread_local!` static `ICON_ATLAS_RUNTIME` (R7, used by R6): `pub(crate) static` inside the macro.

Inspect or narrow with the compiler later; the three scripts that applied these are in section 8 and are idempotent.

## 6. Source-text law tests that read A

Re-pointed in this crate (they run in `cargo test -p semio-framework-os-renderer-wgpu`):

- New in the parent: `#[cfg(test)] pub(crate) const RENDERER_SOURCE: &str = concat!(include_str!(…) × 17)` which concatenates the parent and all 16 children. It relies on `concat!` eagerly expanding `include_str!` (stable behaviour, but **not compiled here**).
- The eight tests above now read `crate::RENDERER_SOURCE` instead of `include_str!`/`read_to_string` of the old single file. This covers the six listed Shell tests (`tour-overlay-and-chord-glyphs` and `wgpu-ui-prefs-themes-i18n` were left unchanged because every needle they search — `semioWgpuSetHostPlatform`, `semioWgpuSetHostLocale`, `Cell::new(cfg!(target_os = "macos"))` — still lives in the parent; `appearance-tour-and-footer-pills` was switched so its `match_media` absence check now covers the whole tree) plus the four renderer-level tests `frame-action-ledger`, `async-boundary`, `standalone`, `settle-pump`.
- `🧪️tests/🧭️boot-axis-parity` reads the parent for `WgpuBootDescriptor` / `WGPU_BOOT_FIELD_CAPACITY`; both stay in the parent, so it is unchanged. I confirmed each remaining needle with `grep -F`.
- Two caveats: tests that slice "from needle to end of text" (`split(...).nth(1)`) now see the rest of the concatenation rather than the rest of the old file; a forbidden-text assertion on such a remainder could in theory now see later files.

**Not done (found, outside my ownership, will fail at runtime until re-pointed).** These read the old file path and search text that moved to a child. Heuristic affected-literal counts (string literals of 10+ characters present in the old file and absent from the new parent; coincidental matches inflate small counts). These are TS (harness-run) or other-crate Rust tests, several in areas other fleet agents are editing, so I left them:

| Test | Reads | Heuristic count |
|---|---|---|
| `E/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` (lines 236, 1906; `include_str!` of A, uses `replace_region` on frame structs) | A | 47 |
| `🧰️framework/🔨️modules/🖱️ui/🧪️tests/♻️physical-job-close/🟦️.ts` | A | 18 |
| `E/🧪️tests/♻️frame-close/🟦️.ts` | A | 34 |
| `E/🧪️tests/🔬️interactivity-mounted-frame-transaction/🟦️.ts` | A | 26 |
| `E/🧪️tests/🛑️scene-pointer-cancellation/🟦️.ts` (`?raw` import) | A | 10 |
| `E/🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts` | A | 8 |
| `E/🧪️tests/🧾️frame-action-ledger/🟦️.ts` | A | 8 |
| `E/🧪️tests/🖼️wgpu-raster-residency/🟦️.ts` | A | 7 |
| `E/🧪️tests/📨️browser-frame-transport/🟦️.ts` (line 970) | A | 6 (probably coincidental) |
| `E/🧪️tests/🧲️scene-input-residency/🟦️.ts`, `🪟️window-lifecycle-template-drag`, `🫀️settle-pump` (TS), `⏱️frame-latency`, `🔬️targets-wgpu-mounted-layout-unit` (expects `frame_before_input_step`, now in `🧠️app-runtime`), `🔬️interactivity-mounted-layout-text` (mutation index 7 needs `process_worker_pool`, now in `🔁️asset-decode-jobs`) | A | 1–4 |

A reusable fix: one helper that returns the concatenation of the parent and every module reachable through its `#[path]` lines (Rust: `crate::RENDERER_SOURCE` already does this for in-crate tests; TS needs its own). I could not run the TS tests: `bun test` does not match `🟦️.ts` file names and the repo harness needs the cargo/ts toolchain.

Comments that cite line numbers of the old file (`🧊️renderer/🦀️.rs:12233`, `:15530`, `:14178`, …) in Shell, EngineCanvas, ui and other places are now stale. Documentation only; not refreshed.

## 7. Deviations from the seam map

- Boundaries moved up to include the attribute/doc that belongs to the first item: A2/A3 (`#[derive(Clone, Copy)]` before `RendererAssetProbePhase`), K0/K1 (`#[derive(Default)]` before `RendererSequenceAuthority`), R3/R4 (the `/// 🪪️ Submits owned work…` doc before `AppHandle`), R4/R5 (`#[derive(…)]` before `FramePreparationStep`), R5/R6 (the six-line `///` doc and `#[cfg]` before the `Send` assertion). The seam map's ranges would have orphaned them.
- `scale_bench` and `kernel_runtime` are inline modules, so their children contain the module body, not a nested `mod` (the seam map's table suggests the same but lists the wrapper lines inside the ranges).
- Region markers that already crossed the cut points (`//#region 🔖️KernelPoolState` in K3 closed in K4, `//#region 📡️RendererAssetAuthority` opened in A1 and closed in A3, plus the pre-existing mismatches) were moved verbatim, not rebalanced.
- Added a `#[cfg(test)] RENDERER_SOURCE` const (section 6); no other new items.

## 8. Verification actually performed

| Check | Command / method | Result |
|---|---|---|
| Compile, native lib | `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-renderer-wgpu --message-format=short` (retry loop, 22+ attempts, 01:44 to 02:36, plus a final `--message-format=json` loop) | **Not run.** First failure: stale `Cargo.lock` (`cargo fetch --locked`, peer added a workspace member at 01:40, lock refreshed by a peer at 02:03), then every attempt: `error: Rust crate Source refused before compiler: semio-framework-3d (Rust crate Source boundary exceeded before compiler byte allocation)`. Policy: `maxSourceBytes 262144`, `maxSourceFiles 32`, `maxActiveLines 4000`; 3d is 3.2 MB in 157 files, the renderer crate is far larger. The same failure hit `semio-framework-job` for another agent. Unsatisfiable until the policy or the crate sizes change. |
| Compile, `--tests`, wasm32 | not run | Same reason. `wasm32-unknown-unknown` is installed in the toolchains; the crate does compile for the browser (A is only excluded on `wasi`), so the wasm32 check must be run too once the gate admits the crate. |
| Baseline before edits | not obtainable | Same reason; every error you see later in my files is mine unless it appears in the untouched files. |
| Lossless | `renderer_lossless.py`: multiset of normalised non-blank lines (dedent, `pub(crate)`/`pub(super)` and path prefixes stripped, `//!`→`///`) of the original 19,807-line file vs parent + 16 children | 0 lines lost except the 5 removed wrapper lines (`#[path = "."]`, `mod kernel_runtime {`, `pub mod scale_bench {`, two closing braces); extras are exactly the new `mod`/`use`/doc/header/`RENDERER_SOURCE` lines. |
| Syntax | `rustfmt --edition 2021 --emit stdout < file` on the parent and all 16 children | 17/17 parse, exit 0, no errors on stderr. |
| Paths | script resolving all 90 literals | 0 missing. |
| Visibility closure | script: any private top-level item referenced from another tree file or from crate-root tests | 0. |
| Duplicate top-level names across children | script | only the pre-existing `cfg` pairs (`spawn_app_task`, `log_debug`, `app_now_ms`, `resolve_environment_boot_descriptor`, `renderer_worker_pool`, `RuntimeHostWaker`), all inside one file. |
| Emoji admission | bun `\p{RGI_Emoji}` check | 16/16. |
| `git diff --stat` for the parent | after all cuts | `371 insertions(+), 19172 deletions(-)` for `R/🦀️.rs`, in line with the 18.8k lines moved out. |
| The 8 edited tests | not run | cannot compile. |

## 9. How to finish once a compiler is available

All tooling is under `🗑️generated/` (do not delete before the follow-up): `renderer_split_lib.py`, `renderer_split.py` (sentinel cutter, already applied), `renderer-widen-plan.json`, `renderer_widen_final.py`, `renderer_widen_members.py`, `renderer_widen_static.py` (analysis), `renderer_lossless.py`, `renderer_diag.py`, `renderer_fix.py`, `renderer-check.sh`, and `renderer-A-original.rs.txt` (the untouched 19,807-line original, sha256 `4f6486d9…7a4`).

1. `🗑️generated/renderer-check.sh <log-name> json` runs the retry loop and writes cargo JSON to `🗑️generated/<log-name>.log`; add `--tests` as a third argument for the test targets; for wasm32 add `--target wasm32-unknown-unknown`.
2. `python3 renderer_diag.py <log> errors` summarises errors by code.
3. For E0603/E0624/E0616/E0451 run `python3 renderer_fix.py apply <log>`; it inserts `pub(crate)` (or `pub(super)` under `kernel_runtime`) at the definition site named by the diagnostic. Repeat until clean (privacy errors from type checking only appear after resolution errors are gone, so expect two or three rounds).
4. Glob ambiguity (E0659), if any: add an explicit `use super::name;` in the affected child, per the rule in the task.
5. Then run the in-crate tests, notably `frame_action_ledger`, the async-boundary and settle-pump laws, `wgpu-wheel-and-escape-routing`, `sync-owner-lifetime`, `wgpu-document-relay`, and confirm `RENDERER_SOURCE`'s `concat!` of `include_str!` compiles.
6. Optional narrowing: drop widenings from section 5 bucket 3 that no error requires.

## 10. Risks I could not eliminate without a compiler

- Unqualified names in a child that rely on something other than the parent's `use` block or root items (none found by inspection; `use super::*` forwards the root's private imports to descendants, which is the usual `mod tests { use super::*; }` behaviour).
- Remaining privacy errors on members used from another file where the owner type name is not mentioned in that file.
- `unused_imports` / `dead_code` warnings from the parent's `use` block and the `pub(crate) use child::*` re-exports; workspace lints are `warn` only.
- Whether the kernel semantic-document test (`include!` now inside `kernel_pool`) still resolves every name it used; it sees the K4 module scope, which has the same glob chain, but it used to see the whole `kernel_runtime` scope directly.
- `🪶️sqlite` / `📇️registry` siblings were not touched; `native_socket_sqlite_tests` now sits in `native_socket_probe` and refers to `super::{NativeSocketProbeSnapshot, store}` (both reachable through the glob chain).

Temporary files: the logs `renderer-split-check-lib.log` and the widening dry-run outputs in `🗑️generated/` are tool output and may be deleted at ticket close; nothing was deleted by me.

## Hardening (full-resolution differential)

Full report: `RESULT-HARDEN-WGPU.md` (tools and logs in `🗑️generated/harden-wgpu/`). The differential ran `rustc +nightly-2026-07-20 --emit=metadata -o /dev/null` from the real lib root `📦️packages/🦀️rust/📚️library/🦀️.rs` against a sandbox copy of the crate source with the original 19,807-line renderer (and the other four monoliths) restored, for native lib, native `--cfg test` (native `--test` is blind, see the report), `wasm32-unknown-unknown` and `--test` on wasm32; sandbox patches (`extern crate` to `mod`, external globs neutralised) were applied to both trees.

- Before: one E0449 in the native lib (`🔬️native-socket-probe/🦀️.rs:50`, `pub(crate)` on a trait-impl method `native_snapshot_registration`; qualifier removed) and 5 new errors in `--cfg test` (the kernel semantic-document law names private `MountedProductReplayRecoveryOwner`, `KERNEL_CLOSE_QUEUED`, ... of sibling partitions).
- Definition scan (fields and methods used outside their child, method privacy through `impl` blocks in children): 170 widenings to `pub(super)`, all in `📦️kernel-runtime/` (K1 47, K2 77, K3 44, K4 1) plus `RuntimeMailboxInner.native_asset_http` in `🎨️presentation-raster/🦀️.rs:632`. Seeded check passes (three reverted widenings are re-found).
- After: native 450/450, `--cfg test` 511/511, wasm32 590/590, `--test` wasm32 654/654 distinct non-external errors (baseline/current), identical multisets; one benign pair (`read_os_clipboard`, cfg-gated, label text only).
- Residual: no typeck, so E0616/E0624 outside the scan's reach remain possible; run the real `cargo check` first.
