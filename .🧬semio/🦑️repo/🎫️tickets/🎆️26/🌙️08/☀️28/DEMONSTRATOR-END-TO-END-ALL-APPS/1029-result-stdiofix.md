STATUS: IN PROGRESS. stdio gltf, dwg, pdf GREEN on wasm32-wasip2 (own artifact workspaces); fem 2d wasm workspace check shows NO stdio error any more (remaining errors are fem-owned: 2d-viewer, 2d-editor-panels). docx base GREEN; docx strict/transitional edits applied, verification queued. fem 3d check queued (stdiofix-11).

Checks: `C/🗑️generated/1029/stdiofix-N.{txt,rc}`; scripts `C/1029-tools/stdiofix/{chk,wait,err}.sh`.

## Fixed
- dwg editor ac1018 (`semio-s-artifact-stdio-dwg-editor-ac1018`): `value_derive` derive paths unresolved -> added `semio-framework-value-derive` dep + `extern crate semio_framework_value_derive as value_derive;` (same as the ac1024 editor).
- gltf / pdf: nothing to change; they compile standalone (gltf-model-snapshot `store`/`snapshot_root` and dwg-snapshot-objects/pdf-testkit errors in the old fem logs were fallout of concurrent fixes / "No space left on device" on the compiler image).
- docx base editor `📬️preparation`: `PagedOneItemEdit` now has `post_mode` + `PagedOneItemPost/Read`; added `post_mode() = Mutable`, `advance_demands(.., PagedOneItemPostRead)`, `advance(.., PagedOneItemPost)` + `post.mutable()?` (zip pattern).
- docx base package: `semio-framework-pack-json` added as dependency (needed by `impl_serde_op_codec!`).
- docx strict/transitional editors: base `canonical_docx_set_page_work!` was `pub(crate) use` + `crate::` paths, so it could never expand in another crate -> `#[macro_export]`, `$crate::` paths, `DOCX_TEXT_WORK_PAGE_BYTES`/`docx_set_page_demand`/`docx_address_demand`/`preparation::route` made `pub`; strict/transitional call `semio_s_artifact_stdio_docx_base::canonical_docx_set_page_work!`.
- docx strict: `impl OpText/OpBinary for DocxStrictMutation` lived in the strict crate (orphan rule E0117) -> leaf codec files now mounted in the strict-mutations crate (`io::{text,binary}::mutations`, same shape as base) and removed from strict `io/🦀️.rs`.
