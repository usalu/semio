# Wave-5 Fixup: `os-host-full` Shared Surface

## Problem
`semio-s-plugin-space` was the only crate enabling `semio-framework-os` feature `os-host-full`. That feature had never been compiled in CI/dev gates; default-feature `cargo check -p semio-framework-os` was green while the space plugin gate failed with ~116 errors — all in framework host surfaces, not in the plugin tree.

## Root causes and fixes

### 1. Missing `zip` dependency
- **Cause:** `🔨️modules/🪐️space` uses `zip::ZipWriter` / `ZipArchive` under `os-host-full`; feature was empty (`os-host-full = []`).
- **Fix:** Optional `zip = "2.4"` on `semio-framework-os`; `os-host-full = ["dep:zip", "semio-framework-os-kernel/sync"]`.

### 2. `store_sync` never mounted
- **Cause:** Host imports `crate::store_sync::…` but glue did not expose kernel `store::sync`.
- **Fix:** `#[cfg(feature = "os-host-full")] pub use store::sync as store_sync;` in host glue.

### 3. Unqualified `space` / `workflow` + dual naming
- **Cause:** Nested host modules referred to `space::` / `workflow::` without imports; glue mounted kernel as `workflow_kernel` while call sites mixed `workflow::` and `crate::workflow_kernel::`.
- **Name chosen:** Public API spelling **`workflow`** (sibling of `space`). Path-mount stays private **`mod workflow_kernel`** so host `pub mod workflow` can re-export kernel vocabulary and layer media/registry helpers without a module-name collision. Crate-root `pub use` goes through `crate::workflow::…`.
- **Fix:** `use crate::space` / `use crate::workflow` in nested modules; consolidate re-exports.

### 4. P6 — `DslOps` no longer emits OpText/OpBinary
- **Cause:** `WorkflowMutationDsl` / `RunMutationDsl` missing handcrafted op codecs after derive change.
- **Fix:** Handcrafted `OpText` + `OpBinary` delegating to `dsl::variants_binary::{encode_op,decode_op}` (same shape as playbook / existing `WorkflowMutation` / `RunMutation` impls).

### 5. P6 — `CollectionSnapshot` (and space snapshots) missing `DocumentPack` / `DocumentDsl`
- **Cause:** Derive no longer emits pack/dsl; envelope id must be two dot-separated segments.
- **Fix:** Handcrafted `DocumentDsl` + `DocumentPack` for `SpaceSnapshot` / `CollectionSnapshot` with `#[dsl(id = "s.space"|"s.collection")]`; envelope round-trip tests.

### 6. `OsMediaFormat` private import
- **Cause:** Re-export path too narrow through kernel mount.
- **Fix:** Widen via `crate::workflow` / registry re-exports (`os_resource_media_capability`, etc.).

### 7. Follow-on (space plugin + host) found while greening tests
- **Projection→Snapshot** call-site updates in the space plugin (framework already renamed).
- **Draft id collision:** `create_draft` hashed only `"draft"` → identical ids across tests; now content-addresses kind/schema/name/now_ms/seq.
- **Catalog URI pointer reuse:** `list_os_space_catalog_entries` now skips unreadable URIs (`let Ok(payload) = port.read(…)`) so recycled `Arc` pointer keys cannot poison later tests.
- **Split media registries:** `media_export_raster` kept stub `register_os_media_*` Always re-exported at crate root, while `export_os_app_instance_media` lived in `workflow` — registrations never reached exporters. With `os-host-full`, stubs now `pub use crate::workflow::{register_*, OsMediaExportResult}`.

## Deliberately left for later rename wave
- `WorkflowDocument` (not renamed to `*Snapshot`).
- Local variables/comments that still say “projection” where they do not break the build.
- DB read-model `🛢️db/📽️projection` and 3D camera “projection” terms — untouched.

## Gates (verbatim tails)

### `cargo check -p semio-framework-os` (default)
```
  |
2 | #![feature(linkage)]
  |            ^^^^^^^
  |
  = note: `#[warn(unused_features)]` (part of `#[warn(unused)]`) on by default

warning: `semio-framework-os` (lib) generated 10 warnings (run `cargo fix --lib -p semio-framework-os` to apply 8 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.85s
```

### `cargo check -p semio-framework-os --features os-host-full`
```
  |
2 | #![feature(linkage)]
  |            ^^^^^^^
  |
  = note: `#[warn(unused_features)]` (part of `#[warn(unused)]`) on by default

warning: `semio-framework-os` (lib) generated 36 warnings (run `cargo fix --lib -p semio-framework-os` to apply 4 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 1.20s
```

### `cargo check -p semio-s-plugin-space`
```
    |
180 | fn resolve_backbone_bytes(app: &HomeApp, uri: &str) -> Option<Vec<u8>> {
    |                           ^^^ help: if this is intentional, prefix it with an underscore: `_app`

warning: `semio-s-plugin-space` (lib) generated 10 warnings (run `cargo fix --lib -p semio-s-plugin-space` to apply 10 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 3.04s
warning: the following packages contain code that will be rejected by a future version of Rust: block v0.1.6
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
```

### `cargo test -p semio-s-plugin-space --lib`
```
test apps::space::engine::tests::compiled_dag_wire_literal_mentions_app_instances ... ok
test apps::space::component::tests::space_workflow_context_menu_stays_within_budget_with_destructive_tail ... ok
test apps::space::component::tests::space_declares_expected_actions_and_examples ... ok
test apps::space::component::tests::space_manifest_uses_studio_app_id ... ok
test apps::space::component::tests::space_window_kind_actions_scope_editing_to_workflow ... ok

test result: ok. 87 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s

```

### `bun ./📜️script.ts policy 2>&1 | rg -i 'home|space'`
```
(empty)
```
(File size 0 bytes — no matches.)

### `cargo check --workspace 2>&1 | rg -c '^error'`
```
81
```
Breakdown: `semio-framework-os-kernel-db` (57) + `semio-compose-rs` (22). **Unrelated** to `os-host-full` / space plugin (unresolved `db_engine` / `db_ids` / compose). Target crates `semio-framework-os` and `semio-s-plugin-space` are green under the dedicated gates above.

## Could not validate
- Repo MCP (`ticket_open` / `ticket_reopen` / `ticket_close` / `repo://goals`) was unavailable in this session — ticket folder used as given; ticket not closed via MCP.
