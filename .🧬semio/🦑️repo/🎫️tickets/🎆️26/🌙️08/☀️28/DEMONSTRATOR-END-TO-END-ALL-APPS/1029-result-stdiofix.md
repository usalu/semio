STATUS: GREEN for stdio gltf, dwg, pdf, docx on wasm32-wasip2 AND native (own artifact workspaces: stdiofix-{1,4,9,15} wasm, {24,31,32,33} native, all rc=0). fem 2d wasm workspace (stdiofix-41) and fem 3d (stdiofix-11/42) show NO stdio error; remaining fem errors are fem-owned (2d-engine, 3d-editor-session/commands, 2d-editor-panels/viewer). Final fem re-checks (stdiofix-41 2d, stdiofix-42 3d): only fem-2d-engine, fem-3d-editor-interaction, fem-3d-editor-visual-job fail (fem-owned, no stdio crate).

Checks: `C/🗑️generated/1029/stdiofix-N.{txt,rc}`; scripts `C/1029-tools/stdiofix/{chk,wait,err}.sh`.

## Fixed
- dwg editor ac1018 (`semio-s-artifact-stdio-dwg-editor-ac1018`): `value_derive` derive paths unresolved -> added `semio-framework-value-derive` dep + `extern crate semio_framework_value_derive as value_derive;` (same as the ac1024 editor).
- gltf / pdf: nothing to change; they compile standalone (gltf-model-snapshot `store`/`snapshot_root` and dwg-snapshot-objects/pdf-testkit errors in the old fem logs were fallout of concurrent fixes / "No space left on device" on the compiler image).
- docx base editor `📬️preparation`: `PagedOneItemEdit` now has `post_mode` + `PagedOneItemPost/Read`; added `post_mode() = Mutable`, `advance_demands(.., PagedOneItemPostRead)`, `advance(.., PagedOneItemPost)` + `post.mutable()?` (zip pattern).
- docx base package: `semio-framework-pack-json` added as dependency (needed by `impl_serde_op_codec!`).
- docx strict/transitional editors: base `canonical_docx_set_page_work!` was `pub(crate) use` + `crate::` paths, so it could never expand in another crate -> `#[macro_export]`, `$crate::` paths, `DOCX_TEXT_WORK_PAGE_BYTES`/`docx_set_page_demand`/`docx_address_demand`/`preparation::route` made `pub`; strict/transitional call `semio_s_artifact_stdio_docx_base::canonical_docx_set_page_work!`.
- docx strict: `impl OpText/OpBinary for DocxStrictMutation` lived in the strict crate (orphan rule E0117) -> leaf codec files now mounted in the strict-mutations crate (`io::{text,binary}::mutations`, same shape as base) and removed from strict `io/🦀️.rs`.

## Verification matrix
- wasm32-wasip2 `--workspace --keep-going`: gltf rc0 (stdiofix-1), dwg rc0 (stdiofix-4, after fix), pdf rc0 (stdiofix-9), docx rc0 (stdiofix-15, includes strict + transitional).
- native: gltf rc0 (31), dwg rc0 (32), pdf rc0 (33), docx rc0 (24).
- fem workspaces wasm: 2d (stdiofix-10, -41) and 3d (stdiofix-11) list zero stdio crates among `could not compile`; failures are only fem crates.
- Infra notes: several runs failed with "No space left on device" / vanished build dir (shared compiler image 48 GiB filled by peers) and with peer in-flight framework breaks (plugin-ownership genesis, instance-jobs history_row_position, decl-app tool_typing_edit, pack-record-retained duplicate import, tool-typing-edit); all disappeared on retry without my intervention.
- Cargo.lock of `docx` artifact workspace gained entries (the repo tool regenerated it for the new pack-json/job deps).

## Files changed
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/{Cargo.toml,🦀️.rs}
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/Cargo.lock
- docx base: 🧱️base/✏️editor/📬️preparation/🦀️.rs, 🧱️base/✏️editor/🦀️.rs (macro_export, $crate, pub items), 🧱️base/📦️packages/🦀️rust/Cargo.toml
- docx strict: 📏️strict/✏️editor/🦀️.rs, 📏️strict/🚪️io/🦀️.rs, 📏️strict/🚪️io/{📝️text,💾️binary}/🧬️mutations/🦀️.rs, 📏️strict/🧬️schema/🧬️mutations/📦️packages/🦀️rust/🦀️.rs, 📏️strict/📦️packages/🦀️rust/Cargo.toml
- docx transitional: 🔄️transitional/✏️editor/🦀️.rs, 🔄️transitional/📦️packages/🦀️rust/Cargo.toml
(all under `…/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/`)

SHARED CHANGE: none outside the stdio dwg/docx artifacts.
