# 🧹 W8 — `resolve_ready` de-async sweep across the fem plugin

> **NOTHING WAS COMPILED.** No `cargo`, `bun`, `nx`, `rustc` or any build/test command was run (host swap
> exhausted, per the coordinator's instruction). Every verdict below is static: the wrapped callee was grepped
> to its own `fn` / `async fn` declaration in the framework source and cited by `file:line`. Verification is
> `rustfmt --edition 2024 --emit stdout` (a formatter, not a compiler — it only proves the file still parses)
> plus a re-grep. The coordinator must compile.

## 1. The defect class

`semio_framework_plugin::resolve_ready` is
`🧰️framework/🔨️modules/🚪️io/🦀️.rs:891`

```rust
pub fn resolve_ready<F: std::future::Future>(fut: F) -> F::Output
```

It is bounded by `Future`, so wrapping a **sync** call in it is `E0277` at compile time. After the framework's
de-async pass, a large part of the fem plugin still wrapped calls whose callees are now plain `fn` — most
notably `vcs::apply_mutation`, `VcsArtifactApp::snapshot`, `ArtifactStore::snapshot`, `HistoryView::empty`,
`ArtifactView::new` and the whole sync `ArtifactEditor` / `ArtifactViewer` static surface.

W2 swept the fem2d **editor subtree** (`📓️w2-fem2d-dispatch.md` §4, 75 wrappers). This report covers
**everything else under `✏️s/🔌️plugins/🏗️fem/`**: all of fem3d, the fem2d mutation/wasm leaves, and the two
`🚪️io/` export leaves per artifact (inspected, not touched — W4's files).

## 2. Scope actually grepped

| Path | resolve_ready sites found |
|---|---|
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/**` | 55 (13 files) |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/**` | 16 (6 files) |
| `✏️s/🔌️plugins/🏗️fem/🦀️.rs` (plugin root) | **0** |
| `✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/🦀️.rs` (W7) | **0** |
| `✏️s/🔌️plugins/🏗️fem/⚙️engine/**`, `🎮️commands/**`, `🧪️oracle/**` | **0** |
| **total** | **71** |

Result: **53 wrappers removed, 18 justified survivors** (4 of them in W4's `🚪️io/` dirs, all correctly
keeping the wrapper). Re-grep after the sweep returns exactly those 18.

## 3. Callee reference table (each definition read directly)

`🔌️plugin` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`,
`🏪️store` = `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`,
`🌿️vcs` = `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs`.

| Callee | Definition | Signature | Verdict |
|---|---|---|---|
| `vcs::apply_mutation` | `🌿️vcs/🦀️.rs:1192` | `pub fn` | **sync → unwrap** |
| `VcsArtifactApp::snapshot` | `🔌️plugin/🦀️.rs:20559` (impl `:19219`) | `pub fn` | **sync → unwrap** |
| `ArtifactStore::snapshot` | `🏪️store/🦀️.rs:14342` (impl `:13899`) | `pub fn` | **sync → unwrap** |
| `HistoryView::empty` | `🔌️plugin/🦀️.rs:10128` (impl `:10126`) | `pub fn` | **sync → unwrap** |
| `ArtifactView::new` | `🔌️plugin/🦀️.rs:8205` (impl `:8201`) | `pub fn` | **sync → unwrap** |
| `ArtifactEditor::io` | `🔌️plugin/🦀️.rs:26921` (trait `:26626`) | `fn` | **sync → unwrap** |
| `ArtifactEditor::export_media` | `🔌️plugin/🦀️.rs:26931` | `fn` | **sync → unwrap** |
| `ArtifactEditor::import_media` | `🔌️plugin/🦀️.rs:26946` | `fn` | **sync → unwrap** |
| `ArtifactViewer::initial_snapshot` | `🔌️plugin/🦀️.rs:27079` (trait `:26975`) | `fn` | **sync → unwrap** |
| `world3d_meshes_json_from_kinds` | `🔌️plugin/🦀️.rs:38263` | `pub fn` | **sync → unwrap** |
| `ArtifactSerializer::serialize` | `🔌️plugin/🦀️.rs:762` (trait `:751`) | `async fn` | async → keep |
| `ArtifactStore::new` | `🏪️store/🦀️.rs:13959` | `pub async fn` | async → keep |
| `store::print_document_spr` | `🏪️store/🦀️.rs:10971` | `pub async fn` | async → keep |
| `testkit::new_app` | `🔌️plugin/🦀️.rs:6791` | `pub async fn` | async → keep |
| `testkit::new_app_with_registry` | `🔌️plugin/🦀️.rs:6797` | `pub async fn` | async → keep |
| `testkit::paired_apps` | `🔌️plugin/🦀️.rs:6827` | `pub async fn` | async → keep |
| `PluginApp::render` | `🔌️plugin/🦀️.rs:11960`, impl for `VcsArtifactApp` `:25314` | `async fn` | async → keep |
| `PluginApp::handle_action` | `🔌️plugin/🦀️.rs:11814`, impl `:24898` | `async fn` | async → keep |

**The two-trait trap.** `initial_snapshot` / `config_spec` / `io` / `export_media` / `import_media` each exist
**twice** in `🔌️plugin/🦀️.rs`: as `async fn` on `ArtifactApp` (`:11351`, `:11420`, `:11540`, `:11561`, `:11586`)
and as plain `fn` on `ArtifactEditor` (`:26808`…`:26946`) / `ArtifactViewer` (`:27079`…`:27146`). fem's call
sites are all *statically qualified on the editor/viewer type* (`Fem3dPlayApp::io()`,
`<Fem3dViewer as ArtifactViewer>::initial_snapshot()`), so they bind to the **sync** pair. The async pair is
reached only through the `impl<E: ArtifactEditor> ArtifactApp for EditorApp<E>` bridge (`:27230`) and the
`ViewerApp<V>` bridge (`:27498`), which fem never calls directly. Likewise `snapshot()` resolves to two
different sync methods (`VcsArtifactApp` for `app.snapshot()`, `ArtifactStore` for `store.snapshot()`) — both
sync, so the unwrap is safe either way.

## 4. Every site

| # | Site (`🗿️artifacts/…`) | Line | Callee | Callee definition | Kind | Verdict | Action |
|---|---|---|---|---|---|---|---|
| 1 | `◻️2d/…/✏️editor/🌉️wasm/🦀️.rs` | 14 | `Fem2dStore::new(..)` | `🏪️store/🦀️.rs:13959` | `pub async fn` (ArtifactStore) | async | wrapper **kept** |
| 2 | `◻️2d/…/✏️editor/🌉️wasm/🦀️.rs` | 16 | `store.snapshot()` | `🏪️store/🦀️.rs:14342` | `pub fn` (ArtifactStore) | sync | wrapper **removed** |
| 3 | `◻️2d/…/✏️editor/🦀️.rs` | 943 | `store::print_document_spr(..)` | `🏪️store/🦀️.rs:10971` | `pub async fn` | async | wrapper **kept** |
| 4 | `◻️2d/…/✏️editor/🦀️.rs` | 1129 | `testkit::new_app::<..>()` | `🔌️plugin/🦀️.rs:6791` | `pub async fn` | async | wrapper **kept** |
| 5 | `◻️2d/…/✏️editor/🦀️.rs` | 1141 | `testkit::new_app_with_registry::<..>(..)` | `🔌️plugin/🦀️.rs:6797` | `pub async fn` | async | wrapper **kept** |
| 6 | `◻️2d/…/✏️editor/🦀️.rs` | 1156 | `app.render(..)` | `🔌️plugin/🦀️.rs:11960` / impl `:25314` | `async fn` (PluginApp) | async | wrapper **kept** |
| 7 | `◻️2d/…/✏️editor/🦀️.rs` | 1450 | `testkit::paired_apps::<..>(..)` | `🔌️plugin/🦀️.rs:6827` | `pub async fn` | async | wrapper **kept** |
| 8 | `◻️2d/…/✏️editor/🦀️.rs` | 1456 | `instance_a.handle_action(..)` | `🔌️plugin/🦀️.rs:11814` / impl `:24898` | `async fn` (PluginApp) | async | wrapper **kept** |
| 9 | `◻️2d/…/✏️editor/🦀️.rs` | 1457 | `instance_b.handle_action(..)` | `🔌️plugin/🦀️.rs:11814` / impl `:24898` | `async fn` (PluginApp) | async | wrapper **kept** |
| 10 | `◻️2d/…/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs` | 28 | `SemioMeshToStl::serialize(..)` | `🔌️plugin/🦀️.rs:762` | `async fn` (ArtifactSerializer) | async | **left for W4** (io dir, untouched — wrapper is justified) |
| 11 | `◻️2d/…/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs` | 28 | `SemioMeshToObj::serialize(..)` | `🔌️plugin/🦀️.rs:762` | `async fn` (ArtifactSerializer) | async | **left for W4** (io dir, untouched — wrapper is justified) |
| 12 | `◻️2d/…/🧬️schema/🧬️mutations/💾️binary/🦀️.rs` | 65 | `Fem2dStore::new(..)` | `🏪️store/🦀️.rs:13959` | `pub async fn` (ArtifactStore) | async | wrapper **kept** |
| 13 | `◻️2d/…/🧬️schema/🧬️mutations/💾️binary/🦀️.rs` | 77 | `store.snapshot()` | `🏪️store/🦀️.rs:14342` | `pub fn` (ArtifactStore) | sync | wrapper **removed** |
| 14 | `◻️2d/…/🧬️schema/🧬️mutations/🦀️.rs` | 98 | `vcs::apply_mutation(..)` | `🌿️vcs/🦀️.rs:1192` | `pub fn` | sync | wrapper **removed** |
| 15 | `◻️2d/…/🧬️schema/🧬️mutations/🦀️.rs` | 149 | `vcs::apply_mutation(..)` | `🌿️vcs/🦀️.rs:1192` | `pub fn` | sync | wrapper **removed** |
| 16 | `◻️2d/…/🧬️schema/🧬️mutations/🦀️.rs` | 152 | `vcs::apply_mutation(..)` | `🌿️vcs/🦀️.rs:1192` | `pub fn` | sync | wrapper **removed** |
| 17 | `🧊️3d/…/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs` | 229 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 18 | `🧊️3d/…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs` | 59 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 19 | `🧊️3d/…/✏️editor/🎮️commands/⚪️add-node/🦀️.rs` | 34 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 20 | `🧊️3d/…/✏️editor/🎮️commands/⚪️add-node/🦀️.rs` | 43 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 21 | `🧊️3d/…/✏️editor/🎮️commands/⚪️add-node/🦀️.rs` | 51 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 22 | `🧊️3d/…/✏️editor/🎮️commands/⚪️add-node/🦀️.rs` | 62 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 23 | `🧊️3d/…/✏️editor/🎮️commands/⚪️add-node/🦀️.rs` | 66 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 24 | `🧊️3d/…/✏️editor/🎮️commands/⚪️add-node/🦀️.rs` | 77 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 25 | `🧊️3d/…/✏️editor/🎮️commands/⚪️add-node/🦀️.rs` | 79 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 26 | `🧊️3d/…/✏️editor/🎮️commands/📍️add-nodal-load/🦀️.rs` | 96 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 27 | `🧊️3d/…/✏️editor/🎮️commands/📍️add-nodal-load/🦀️.rs` | 98 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 28 | `🧊️3d/…/✏️editor/🎮️commands/📍️add-nodal-load/🦀️.rs` | 108 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 29 | `🧊️3d/…/✏️editor/🎮️commands/📍️add-nodal-load/🦀️.rs` | 117 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 30 | `🧊️3d/…/✏️editor/🎮️commands/📍️add-nodal-load/🦀️.rs` | 119 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 31 | `🧊️3d/…/✏️editor/🎮️commands/📍️add-nodal-load/🦀️.rs` | 127 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 32 | `🧊️3d/…/✏️editor/🎮️commands/📍️add-nodal-load/🦀️.rs` | 129 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 33 | `🧊️3d/…/✏️editor/🎮️commands/📍️add-nodal-load/🦀️.rs` | 136 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 34 | `🧊️3d/…/✏️editor/🎮️commands/📍️add-nodal-load/🦀️.rs` | 143 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 35 | `🧊️3d/…/✏️editor/🎮️commands/📍️add-nodal-load/🦀️.rs` | 152 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 36 | `🧊️3d/…/✏️editor/🎮️commands/📚️set-active-example/🦀️.rs` | 47 | `HistoryView::empty()` | `🔌️plugin/🦀️.rs:10128` | `pub fn` | sync | wrapper **removed** |
| 37 | `🧊️3d/…/✏️editor/🎮️commands/📚️set-active-example/🦀️.rs` | 59 | `ArtifactView::new(..)` | `🔌️plugin/🦀️.rs:8205` | `pub fn` | sync | wrapper **removed** |
| 38 | `🧊️3d/…/✏️editor/🎮️commands/📚️set-active-example/🦀️.rs` | 90 | `ArtifactView::new(..)` | `🔌️plugin/🦀️.rs:8205` | `pub fn` | sync | wrapper **removed** |
| 39 | `🧊️3d/…/✏️editor/🎮️commands/📚️set-active-example/🦀️.rs` | 106 | `ArtifactView::new(..)` | `🔌️plugin/🦀️.rs:8205` | `pub fn` | sync | wrapper **removed** |
| 40 | `🧊️3d/…/✏️editor/🎮️commands/🗂️remove-selection/🦀️.rs` | 58 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 41 | `🧊️3d/…/✏️editor/🎮️commands/🗂️remove-selection/🦀️.rs` | 60 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 42 | `🧊️3d/…/✏️editor/🎮️commands/🗂️remove-selection/🦀️.rs` | 62 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 43 | `🧊️3d/…/✏️editor/🎮️commands/🗂️remove-selection/🦀️.rs` | 69 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 44 | `🧊️3d/…/✏️editor/🎮️commands/🧮️set-analysis-settings/🦀️.rs` | 41 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 45 | `🧊️3d/…/✏️editor/🦀️.rs` | 822 | `world3d_meshes_json_from_kinds(..)` | `🔌️plugin/🦀️.rs:38263` | `pub fn` | sync | wrapper **removed** |
| 46 | `🧊️3d/…/✏️editor/🦀️.rs` | 1110 | `store::print_document_spr(..)` | `🏪️store/🦀️.rs:10971` | `pub async fn` | async | wrapper **kept** |
| 47 | `🧊️3d/…/✏️editor/🦀️.rs` | 1247 | `testkit::new_app::<..>()` | `🔌️plugin/🦀️.rs:6791` | `pub async fn` | async | wrapper **kept** |
| 48 | `🧊️3d/…/✏️editor/🦀️.rs` | 1260 | `testkit::new_app_with_registry::<..>(..)` | `🔌️plugin/🦀️.rs:6797` | `pub async fn` | async | wrapper **kept** |
| 49 | `🧊️3d/…/✏️editor/🦀️.rs` | 1275 | `app.render(..)` | `🔌️plugin/🦀️.rs:11960` / impl `:25314` | `async fn` (PluginApp) | async | wrapper **kept** |
| 50 | `🧊️3d/…/✏️editor/🦀️.rs` | 1585 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 51 | `🧊️3d/…/✏️editor/🦀️.rs` | 1586 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 52 | `🧊️3d/…/✏️editor/🦀️.rs` | 1605 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 53 | `🧊️3d/…/✏️editor/🦀️.rs` | 1606 | `HistoryView::empty()` | `🔌️plugin/🦀️.rs:10128` | `pub fn` | sync | wrapper **removed** |
| 54 | `🧊️3d/…/✏️editor/🦀️.rs` | 1607 | `ArtifactView::new(..)` | `🔌️plugin/🦀️.rs:8205` | `pub fn` | sync | wrapper **removed** |
| 55 | `🧊️3d/…/✏️editor/🦀️.rs` | 1608 | `Fem3dPlayApp::export_media(..)` | `🔌️plugin/🦀️.rs:26931` | `fn` (ArtifactEditor) | sync | wrapper **removed** |
| 56 | `🧊️3d/…/✏️editor/🦀️.rs` | 1623 | `HistoryView::empty()` | `🔌️plugin/🦀️.rs:10128` | `pub fn` | sync | wrapper **removed** |
| 57 | `🧊️3d/…/✏️editor/🦀️.rs` | 1624 | `ArtifactView::new(..)` | `🔌️plugin/🦀️.rs:8205` | `pub fn` | sync | wrapper **removed** |
| 58 | `🧊️3d/…/✏️editor/🦀️.rs` | 1625 | `Fem3dPlayApp::export_media(..)` | `🔌️plugin/🦀️.rs:26931` | `fn` (ArtifactEditor) | sync | wrapper **removed** |
| 59 | `🧊️3d/…/✏️editor/🦀️.rs` | 1634 | `app.snapshot()` | `🔌️plugin/🦀️.rs:20559` | `pub fn` (VcsArtifactApp) | sync | wrapper **removed** |
| 60 | `🧊️3d/…/✏️editor/🦀️.rs` | 1635 | `HistoryView::empty()` | `🔌️plugin/🦀️.rs:10128` | `pub fn` | sync | wrapper **removed** |
| 61 | `🧊️3d/…/✏️editor/🦀️.rs` | 1636 | `ArtifactView::new(..)` | `🔌️plugin/🦀️.rs:8205` | `pub fn` | sync | wrapper **removed** |
| 62 | `🧊️3d/…/✏️editor/🦀️.rs` | 1646 | `Fem3dPlayApp::import_media(..)` | `🔌️plugin/🦀️.rs:26946` | `fn` (ArtifactEditor) | sync | wrapper **removed** |
| 63 | `🧊️3d/…/✏️editor/🦀️.rs` | 1662 | `Fem3dPlayApp::io()` | `🔌️plugin/🦀️.rs:26921` | `fn` (ArtifactEditor) | sync | wrapper **removed** |
| 64 | `🧊️3d/…/👁️viewer/🦀️.rs` | 134 | `<Fem3dViewer as ArtifactViewer>::initial_snapshot()` | `🔌️plugin/🦀️.rs:27079` | `fn` (ArtifactViewer) | sync | wrapper **removed** |
| 65 | `🧊️3d/…/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs` | 28 | `SemioMeshToStl::serialize(..)` | `🔌️plugin/🦀️.rs:762` | `async fn` (ArtifactSerializer) | async | **left for W4** (io dir, untouched — wrapper is justified) |
| 66 | `🧊️3d/…/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs` | 28 | `SemioMeshToObj::serialize(..)` | `🔌️plugin/🦀️.rs:762` | `async fn` (ArtifactSerializer) | async | **left for W4** (io dir, untouched — wrapper is justified) |
| 67 | `🧊️3d/…/🧬️schema/🧬️mutations/💾️binary/🦀️.rs` | 63 | `Fem3dStore::new(..)` | `🏪️store/🦀️.rs:13959` | `pub async fn` (ArtifactStore) | async | wrapper **kept** |
| 68 | `🧊️3d/…/🧬️schema/🧬️mutations/💾️binary/🦀️.rs` | 74 | `store.snapshot()` | `🏪️store/🦀️.rs:14342` | `pub fn` (ArtifactStore) | sync | wrapper **removed** |
| 69 | `🧊️3d/…/🧬️schema/🧬️mutations/🦀️.rs` | 99 | `vcs::apply_mutation(..)` | `🌿️vcs/🦀️.rs:1192` | `pub fn` | sync | wrapper **removed** |
| 70 | `🧊️3d/…/🧬️schema/🧬️mutations/🦀️.rs` | 168 | `vcs::apply_mutation(..)` | `🌿️vcs/🦀️.rs:1192` | `pub fn` | sync | wrapper **removed** |
| 71 | `🧊️3d/…/🧬️schema/🧬️mutations/🦀️.rs` | 171 | `vcs::apply_mutation(..)` | `🌿️vcs/🦀️.rs:1192` | `pub fn` | sync | wrapper **removed** |

## 5. Files edited (14)

Line numbers are unchanged by the sweep — every removal is confined to a single line (the wrapper prefix plus
its matching `)`), no line is added or deleted.

| File (under `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/…/🏅️standards/🔖️1/🪆️subsets/🌐️any/`) | Wrappers removed |
|---|---|
| `🧊️3d/✏️editor/🦀️.rs` | 15 |
| `🧊️3d/✏️editor/🎮️commands/📍️add-nodal-load/🦀️.rs` | 10 |
| `🧊️3d/✏️editor/🎮️commands/⚪️add-node/🦀️.rs` | 7 |
| `🧊️3d/✏️editor/🎮️commands/🗂️remove-selection/🦀️.rs` | 4 |
| `🧊️3d/✏️editor/🎮️commands/📚️set-active-example/🦀️.rs` | 4 |
| `🧊️3d/✏️editor/🎮️commands/🧮️set-analysis-settings/🦀️.rs` | 1 |
| `🧊️3d/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs` | 1 |
| `🧊️3d/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs` | 1 |
| `🧊️3d/👁️viewer/🦀️.rs` | 1 |
| `🧊️3d/🧬️schema/🧬️mutations/🦀️.rs` | 3 |
| `🧊️3d/🧬️schema/🧬️mutations/💾️binary/🦀️.rs` | 1 |
| `◻️2d/🧬️schema/🧬️mutations/🦀️.rs` | 3 |
| `◻️2d/🧬️schema/🧬️mutations/💾️binary/🦀️.rs` | 1 |
| `◻️2d/✏️editor/🌉️wasm/🦀️.rs` | 1 |
| **total** | **53** |

`◻️2d/✏️editor/🦀️.rs` was inspected (7 sites) and **not** modified — W2 already swept it and all 7 survivors
are genuine async callees.

### The two non-test blockers

Almost all 53 are `#[cfg(test)]` code, but **two are production `pub fn`s** and therefore fail `cargo check`,
not just `cargo test`:

- `🧊️3d/🧬️schema/🧬️mutations/🦀️.rs:99` — `pub fn apply_fem3d_mutation`
- `◻️2d/🧬️schema/🧬️mutations/🦀️.rs:98` — `pub fn apply_fem2d_mutation`

Both were `let (next, _) = resolve_ready(vcs::apply_mutation(snapshot, mutation))?;` → now
`let (next, _) = vcs::apply_mutation(snapshot, mutation)?;`. The `?` and the destructuring shape are unchanged
(`apply_mutation` returns `Result<(P, Vec<MutationMessage>), MutationApplyError>`), so no local fallout.
These are the two the coordinator will see first in a `cargo check`. W2's §4 note (*"All are `#[cfg(test)]`
code, so this does not affect `cargo check`"*) was true for the fem2d **editor** subtree but is **not** true
for the fem2d/fem3d `🧬️mutations` leaves.

### Result / `.await` shape fallout — none needed

Every unwrap preserved the surrounding expression exactly:

- `resolve_ready(app.snapshot()).expect("snapshot")` → `app.snapshot().expect("snapshot")` — `VcsArtifactApp::snapshot` already returns `Result<A::Snapshot, Fault>`.
- `resolve_ready(store.snapshot()).expect("snapshot")` → `store.snapshot().expect(..)` — `Result<P, VcsError>`.
- `resolve_ready(Fem3dPlayApp::io()).expect(..)` → `Fem3dPlayApp::io().expect(..)` — returns `Option<AppIo>`, `.expect` valid on `Option`.
- `resolve_ready(Fem3dPlayApp::export_media(..)).expect(..)` / `.expect_err(..)` → `Result<Media, MediaError>`, unchanged.
- `dsl::json::parse(&resolve_ready(world3d_meshes_json_from_kinds(..)))` → `dsl::json::parse(&world3d_meshes_json_from_kinds(..))` — returns `String`, `&String` deref-coerces to `&str`.
- `HistoryView::empty()` / `ArtifactView::new(..)` / `ArtifactViewer::initial_snapshot()` return plain values; the wrapper was pure noise.

No `.await` was added or removed anywhere.

## 6. Sites left for W4 (`🚪️io/` directories — NOT touched)

| Site | Line | Callee | Callee definition | Verdict |
|---|---|---|---|---|
| `🧊️3d/…/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs` | 28 | `SemioMeshToStl::serialize(&mesh)` | `🔌️plugin/🦀️.rs:762` (trait `ArtifactSerializer` `:751`) | **async — wrapper is correct, keep** |
| `🧊️3d/…/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs` | 28 | `SemioMeshToObj::serialize(&mesh)` | same | **async — keep** |
| `◻️2d/…/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs` | 28 | `SemioMeshToStl::serialize(&mesh)` | same | **async — keep** |
| `◻️2d/…/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs` | 28 | `SemioMeshToObj::serialize(&mesh)` | same | **async — keep** |

**W4 has nothing to remove.** All four `🚪️io/` sites wrap `ArtifactSerializer::serialize`, which the trait
still declares `async fn` (`🔌️plugin/🦀️.rs:762`, with an explicit doc note that the future always resolves on
first poll and that `resolve_ready` is the intended sync bridge). Leave them exactly as they are.

## 7. Other de-async fallout searched for in the same files

| Class | Finding |
|---|---|
| `block_on(` | **0 hits** anywhere under `✏️s/🔌️plugins/🏗️fem/`. |
| `.await` on a now-sync callee | **0**. Every awaited callee in the fem tree was grepped to an `async fn` / `impl Future` definition — see below. |
| `async fn` in a now-sync trait impl | **0**. Every item in fem's `impl ArtifactEditor for Fem{2,3}dPlayApp` and `impl ArtifactViewer for Fem{2,3}dViewer` is already a plain `fn`, matching the sync traits. |
| `async fn` bodies with no `.await` | 123 in the fem tree — **left alone, by instruction**. They split into `#[semio_framework_async_macros::async_test]` tests and `🚪️io/` `ArtifactSerializer`/`ArtifactDeserializer` impls (`serialize`/`sniff`/`deserialize`), which *must* stay `async fn` to satisfy their async traits. An `async fn` that never awaits is at worst a lint, never an error. |

Every distinct awaited callee, checked to its definition:

| Awaited callee | Definition | Kind |
|---|---|---|
| `store.dispatch(..)` | `🏪️store/🦀️.rs:15231` | `pub async fn` |
| `app.dispatch_typed(..)` | `🔌️plugin/🦀️.rs:23326` | `pub fn … -> impl Future` |
| `app.load_document_pack(..)` | `🔌️plugin/🦀️.rs:11952` / impl `:25289` | `async fn` |
| `testkit::assert_undo_redo_round_trip(..)` | `🔌️plugin/🦀️.rs:6855` | `pub async fn` |
| `testkit::assert_viewer_never_mutates::<..>()` | `🔌️plugin/🦀️.rs:7081` | `pub async fn` |
| `testkit::assert_editor_and_viewer_share_dialect::<..>()` | `🔌️plugin/🦀️.rs:7100` | `pub async fn` |
| `os_spr::testkit::assert_mutation_diff_absorb_law(..)` | `📡️spr/🧪️testkit/🦀️.rs:533` | `pub async fn` |
| `os_spr::testkit::assert_mutation_inverse_law(..)` | `📡️spr/🧪️testkit/🦀️.rs:550` | `pub async fn` |
| `os_spr::testkit::assert_missing_target_is_error(..)` | `📡️spr/🧪️testkit/🦀️.rs:696` | `pub async fn` |
| `os_spr::testkit::assert_fatal_never_applies(..)` | `📡️spr/🧪️testkit/🦀️.rs:709` | `pub async fn` |
| `os_store::test_support::assert_document_text_round_trip(..)` | `🏪️store/🦀️.rs:19955` | `pub async fn` |
| `os_store::test_support::assert_document_pack_round_trip(..)` | `🏪️store/🦀️.rs:20023` | `pub async fn` |
| fem-local helpers `dispatch`, `fem3d_empty_app`, `app_with_load_case`, `with_dead_case`, `load_default_example`, `app_with_example` | fem tree, all declared `async fn` | `async fn` |

## 8. Verification performed (no compiler)

1. **Classification is machine-checked, not eyeballed.** `🔨️w8-resolve-ready-sweep.py` (in this ticket folder)
   parses each wrapper with a string-aware paren matcher, classifies the inner expression against an explicit
   SYNC / ASYNC prefix table, and **aborts on any unclassified site**. Dry run: `total=71 sync=53 async=18
   unknown=0 io=4`. Nothing was swept by pattern-guess.
2. **`🚪️io/` is hard-excluded in the script itself** (any path component starting with `🚪️io`), so W4's files
   are byte-identical — they only appear in the report.
3. **rustfmt parse-check**, all 14 edited files, `rustfmt --edition 2024 --emit stdout --quiet < <file>`:
   every one produced empty stderr, i.e. the file still parses as Rust 2024. (rustfmt is a formatter; this
   proves syntax only, not types.)
4. **Re-grep**: `grep -rn resolve_ready ✏️s/🔌️plugins/🏗️fem/` now returns **18** lines, each individually
   listed in §4 with an `async fn` callee.
5. **No import fallout**: zero `use …resolve_ready` in the fem tree — every call site was fully qualified
   `semio_framework_plugin::resolve_ready`, so no import is left dangling.
6. Spot-read of the seven most structurally awkward rewrites (nested-in-`&`, inside a closure argument,
   with `?`, with `.expect_err`, `assert_eq!` argument position) — all shown correct in §5.

## 9. Open risks for the coordinator

1. **Nothing compiled.** The two production sites in §5 are the highest-value check
   (`apply_fem2d_mutation` / `apply_fem3d_mutation`).
2. **Only the fem plugin was swept.** The same class exists elsewhere: `✏️s/🔌️plugins/📐️cad/…/🚪️io/🦀️.rs:450`
   and `✏️s/🔌️plugins/📸️remodel/…/🔺️stl/🔖️ascii/✳️any/🦀️.rs:23` both wrap
   `SemioMeshToStl::serialize` (async — those are fine), but no one has audited the *sync-callee* class in
   `📐️cad`, `📸️remodel`, `🧱️block`, `🧩️puzzle` or `🔱️trinity`. Recommend a repo-wide run of the same script
   with the plugin root widened.
3. **W5/W6 are adding new test dirs under `📈️analysis/🧪️tests/`** while this sweep ran. If their generated
   tests copy an old fem template they may reintroduce `resolve_ready(app.snapshot())`. Re-grep after they
   land: the expected steady-state count under `🏗️fem/` is **18**.
4. `📓️w2-fem2d-dispatch.md` §7.4 predicted *"fem3d was not swept … and almost certainly carries it"* — 
   confirmed, 48 of the 53 removals are fem3d.
