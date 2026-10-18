# Result glob: census-5 glob re-exports removed

Executor prefix `glob`. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/glob-tools/` (Python plus the closure counter), pre-edit backups of every edited Rust and TS file: `F/glob-tools/orig/` indexed by `F/glob-tools/manifest.json`. Logs: `F/🗑️generated/glob-oracle-{1,2,3,final}.log`, `glob-census-after.jsonl`. No cargo, nx, bun repo or modifying git command was run. No sub-agents.

## 0. Verdict

All 89 census-5 `use <child>::*;` globs (the 77 `pub use` of `audit-split-verification.md` D1 plus 12 private `use child::*` of the same splits in ip, ir, sc, ed) are gone from the 17 split roots, and the `export type { ProgramDiff };` of D2 is gone. Every replacement is an explicit name list. Static evidence only; the real compile is for the coordinator (section 5).

Three globs of the same files are not census-5 and stayed: `pub use lifted_app::*;` (document-media root, in HEAD), `pub use execution_target_module::*;` (directory-client root, in HEAD), `use semio_framework_ui_contract::*;` (typed-publication root, in HEAD, external crate).

## 1. Rule applied

Per root the tool lists every child item that is not private (`pub`, `pub(crate)`, `pub(super)`, with its own `#[cfg]` attributes) and matches identifiers over the whole module tree (all `#[path]` mounts and `include!` fragments, strings and comments stripped, `{ident}` format captures included).

| Root kind | `pub` child items | `pub(crate)`/`pub(super)`/private-glob items |
| --- | --- | --- |
| Crate root (lib path of the crate): dd, ir, ip, he, hg, hr, hs, ej, ex, ed, rc, ch, wc | explicit `pub use child::{A, B};` (all of them, API kept) | private `use child::{A, B};`, only names used outside the defining child; the cfg of the item, or the common mount cfg of all users (`#[cfg(test)]`, `any(test, feature = ..)`, wasm), is copied onto the import; non-pub traits are always imported (method scope) |
| Not a crate root: sc (`scene`), dc (`client`), dgd (`component`), pub (`typed_publication`) | no re-export; consumers re-pointed to the child module (child `mod` made `pub`/`pub(crate)`) | private `use` as above |

Names that the root already defines or imports explicitly are skipped (an explicit import would clash, the glob lost silently); duplicates across children: none.

## 2. Files changed

Roots (explicit lists): the 17 `🦀️.rs` of the table in section 4.

Re-pointed consumers (module is not a crate root):
- sc: 13 files, 51 lines. `semio_s_artifact_layout_layout_engine::scene::X` became `…::scene::<child>::X` (the two brace lists became nested use trees). Files: `⚙️engine/📤️export/` (`📕️pdf-png-and-package-encoding`, `🏗️job-admission-and-close`, test fragments `✒️pdf-drawing-laws`, `🚧️caps-json-checkpoint-pdf-style`), `🎮️commands/` (`↔️canvas-pointer-move`, `👇️canvas-pointer-down`, `📄️patch-document` tests), `🖼️canvas`, `🕹️layout-play-app`, editor root `🦀️.rs:248` and `🧪️tests/🔬️unit`, `🧬️mutations/🪜reorder-frame` and `🏷️set-drawing-text` tests. `scene` children with pub items are now `pub mod` (`display_list_and_hit_testing`, `text_fonts`, `layout_engine_and_views`, `scene_painting_and_queries`). No other user of the crate name exists (grep over all `.rs` and `Cargo.toml`, no renames).
- dc: the crate root `🧩️composition/🦀️.rs` was the real consumer (`pub use client::*;`). It now lists `pub use client::<child>::{..}` per child (cfg variants kept), `client::{ExecutionTargetModule..}` (through the untouched pre-existing glob), `agent_delegations`, `document_http`, the three canonical-checkpoint names, and cfg-gated `native` / `browser`. The six children are `pub(crate) mod`. `🪪️identity/🦀️.rs` (sibling of `client`) imports from the child modules. The companion `🧪️tests/🧶️companion/🦀️.rs` replaced `pub use semio_framework_os_directory_client::client::*;` by the explicit list of crate-root names (same set the glob produced, minus the locally declared `document_http`/`native`).
- dgd: `🧩️expand/🦀️.rs` re-exports the six `expand_*` from their child modules; its `testkit::component` hook (`pub use crate::component::*;`) lists the 35 pub names per child. The five children are `pub(crate) mod`.
- pub: nothing outside the tree uses the two `pub(crate)` fns; one private `use` remains.

TS: `✏️s/🔌️plugins/🏛️architect/…/🧬️schema/🔺️diff/🟦️.ts` drops `export type { ProgramDiff };` (the file keeps its `import type`); the importer `…/🚪️io/📝️text/🔺️diff/🔣️json/🟦️.ts` now imports `type ProgramDiff` from `🔺️diff/🧾️program-diff/🟦️.ts`. `bun build --no-bundle` exit 0 on both. No other importer of the type exists.

## 3. Verification (all executed)

| Check | Result |
| --- | --- |
| `rustfmt --edition 2024 --check` parse of all 33 edited `.rs` | 0 parse errors |
| Scoped differential `F/glob-tools/gorc.py` (descendant of `oracle5.py`: live tree against live tree with the pre-edit backups restored; `rustc +nightly-2026-07-20 --emit=metadata`, errors and warnings with `-W unused-imports` as multisets; configs lib, test, `testkit`, `testkit` plus test, `artifact-app-testing`, native features for dc) | `ORACLE CLEAN` for dd, ir, ip, he, hg, hr, hs, ex, ej, ed, rc, ch, wc, sc, pub, dc, dd-derive and the derive-expand crate root `dgdx` in the final state (`glob-oracle-final.log`) |
| Oracle sensitivity | removing `ModuleParser` from one list gave `+1 error E0433` as expected; the oracle also exposed and I fixed: 3 test-only/lib-only imports (`frame_id` false positive, `OwnerKey` needs `any(test, feature="testkit")`, `ArtifactRetainedCommandPhase` needs the oracle-mount cfg), the `{TEXT_FONT_ASSET_ROOT}` format capture, the sibling `identity` consumer of `client::` |
| `ambig.py live`, `defscan.py` on all 17 | `AMBIG OK`, 0 cross-file leftovers everywhere |
| Canonical closure counter (`readRustSourceClosureV1`, `glob-tools/census-own.ts`, no closure-reader error, so the quote!/glob limitation of RESULT-CENSUS5-C9 does not bite) | all 17 crates, lib and test: ratio <= 0.95 except `ex` test 0.985 (3,938 of 4,000 active lines, +11 lines from this change) |
| Re-search for external mounts of the 17 trees | only the expected crate-root mounts |
| Gaps in the check | see section 4 |

## 4. Open risks

1. Not checked statically: the `wasm32` arms (dc `browser`, `wall_now_ms`/`emit_*` cfg pairs, `DocumentSocketAdmissionFuture`, the wasm `Directory*Platform` traits) and typeck-level trait scope. Non-pub child traits are imported unconditionally and pub traits are `pub use`d; the three dc pub traits not imported (`InstalledServiceDriverV1`, `DirectoryTransportPlatform`, `DirectoryConnectionPlatform`) are never named outside their child (markers or `dyn` use).
2. Over-approximated imports can surface as `unused_imports` warnings in configs the oracle did not run (other features, other targets). No `deny(warnings)` exists in the workspace lints.
3. The dcu companion list is derived from the crate-root list, and the oracle cannot judge it (the baseline neutralizes the external glob, so the differential is meaningless there); compile it.
4. `ex` test closure is 62 active lines under the gate.
5. Pre-existing and unrelated: `directory/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:310,315` uses `crate::client::SocketGrantReceiptV1` in a crate that has no `client` module (companion-1 of os-directory); not touched, probably already broken before this change.
6. Per-crate overview (approximate count of names now imported from split children in the root, including explicit imports that existed before): dd 22, ir 51, ip 97, he 39, hg 30, hr 33, hs 62, ej 17, ex 69, ed 24, rc 19, ch 23, wc 32 (crate roots); sc 60, pub 1, dgd 18, dc 49 private imports.

## 5. Crates to compile (`cargo check -p <crate> --lib --tests`, through `bun repo cargo --store=shared -- …`)

Roots:
`semio-framework-plugin-document-media`, `semio-framework-plugin-interpreter-runtime`, `semio-framework-plugin-interpreter`, `semio-framework-plugin-host-effects` (+ `--features testkit`), `semio-framework-plugin-host-guest-runtime` (+ `testkit`), `semio-framework-plugin-host-routers`, `semio-framework-plugin-host-shard` (+ `testkit`), `semio-framework-plugin-retained-command` (+ `artifact-app-testing`), `semio-framework-plugin-child` (+ `artifact-app-testing`), `semio-framework-plugin-window-config`, `semio-framework-plugin-instance-typed-ops`, `semio-framework-os-kernel-dsl-derive-expand` (+ `testkit`), `semio-framework-os-directory-client` (+ `ureq,sync,testkit`; also `--target wasm32-unknown-unknown` for the browser arm).

Layout engine and consumers of `scene`: `semio-s-artifact-layout-layout-engine`, `-layout-engine-export`, `-layout-engine-json`, `semio-s-artifact-layout-layout-editor`, `semio-s-artifact-layout-layout-editor-commands`, `semio-s-artifact-layout-layout-editor-commands-tests`, `semio-s-artifact-layout-layout-mutations-frames`, `semio-s-artifact-layout-layout-mutations-frames-2`.

Companions that mount or use the changed trees: `semio-framework-os-directory-client-tests` (features `ureq,sync,testkit`), `semio-framework-os-kernel-dsl-derive-tests` (hook), `semio-framework-os-kernel-dsl-derive` (proc-macro consumer of the expand crate; unchanged mounts).

TS: `bun test` for the architect program `🔺️diff` tests and the importer `🚪️io/📝️text/🔺️diff/🔣️json` once `node_modules` is installed.
