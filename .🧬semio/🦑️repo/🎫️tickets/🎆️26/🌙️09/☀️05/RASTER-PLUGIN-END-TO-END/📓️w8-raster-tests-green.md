# W8 — Raster `--lib --tests` Green

Scope: make `cargo check -p semio-s-plugin-raster --lib --tests` compile (it failed with **41 errors**, all
in `#[cfg(test)]` code) and then run `cargo test -p semio-s-plugin-raster --lib`.

Baseline log (pre-existing, from the shared warm `target/`):
`🗑️generated/check-shared-2.txt` — `error: could not compile 'semio-s-plugin-raster' (lib test) due to 41 previous errors; 11 warnings emitted`.

## Result

```
$ RUSTC_WRAPPER="" cargo check -p semio-s-plugin-raster --lib --tests --message-format short
exit=0
    Finished `dev` profile [unoptimized] target(s) in 16.39s
```

Zero errors. No new raster-owned warnings (the crate's 71 pre-existing warnings are untouched and belong to
the sibling `ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS` ticket).

## Error classes, framework change, and fix

All line numbers below are the POST-fix positions unless stated otherwise.

### (a) `no method named expect/dispatch/snapshot/remove found for opaque type impl Future<…>` — 25 errors

**Framework change.** The whole `PluginApp` surface went `async` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`):

| API | new signature site |
| --- | --- |
| `testkit::new_app::<A>()` | `:6810` `pub async fn new_app<A: ArtifactApp + Default>() -> VcsArtifactApp<A>` |
| `testkit::new_app_with_registry::<A>(manifest)` | `:6816` |
| `VcsArtifactApp::dispatch_typed` | `:23752` returns `impl Future<Output = Result<InvocationResult, Fault>>` |
| `PluginApp::render` | `:11995` `async fn render(&mut self, …)` |
| `PluginApp::window_measures` | `:11999` `async fn` |
| `PluginApp::load_document_pack` | `:11987` `async fn` |
| `PluginApp::handle_action` | `:11849` `async fn` |
| `PluginApp::ingest_operations` / `attach_backbone` | `:11964` / `:11988` `async fn` |
| `store::print_document_pack` | `🏪️store/🦀️.rs:11174` `pub async fn` |
| `store::ArtifactStore::new` / `::dispatch` | `🏪️store/🦀️.rs` — `dispatch` at `:15231` `pub async fn`; `new` now returns a future of `Result<…>` |
| `store::MemoryBackbone::pair` | `async` |
| `store::os_store::test_support::assert_document_text_round_trip` / `assert_document_pack_round_trip` / `assert_command_envelope_round_trip` / `assert_ingest_idempotent` | `🏪️store/🦀️.rs:19955 / :20023 / :20063`, `🔌️plugin/🦀️.rs:6977` — all `pub async fn` |
| raster's OWN `encode_op` / `decode_op` | `🧬️mutations/💾️binary/🦀️.rs:14` / `:19` — already `pub async fn`; the tests still called them synchronously |

The tests themselves were already `#[semio_framework_async_macros::async_test] async fn` (the repo convention,
same as `🧩️puzzle`/`🧱️block`/`🗒️note`); what was stale was the **`#[cfg(test)] pub(crate) mod testkit` helper
layer** in `✏️editor/🦀️.rs`, which was still plain `fn` and therefore never awaited anything.

**Fix.** Made raster's own test harness `async` and `.await`ed every framework call. This is the convention
`🌊️flow`'s editor testkit already follows (`✏️s/🔌️plugins/🌊️flow/…/✏️editor/🦀️.rs:2325-2331`); the older
`semio_framework::io::resolve_ready(…)` wrapper that `🧩️puzzle` still uses was deliberately NOT copied — the
callers are already `async fn`, so awaiting is both correct and free of `resolve_ready`'s
"future-was-not-ready-on-first-poll" panic.

`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

* `:1076` `pub async fn app()` → `new_app::<EditorApp<RasterPlayApp>>().await`
* `:1080` `pub async fn app_with_registry()` → `.await`
* `:1084` `pub async fn dispatch()` → `dispatch_typed(…).await.expect("dispatch")`
* `:1088` `pub async fn render()` (see class (h) — also re-serialized)
* `:1092` `pub async fn main_window_measures()` → `window_measures().await.remove(…)`
* `:1096` `pub async fn semio_app()` → `new_app(…).await`, `print_document_pack(…).await`, `load_document_pack(…).await`

and every call site inside `mod tests` grew `.await` (16 × `app()`, 3 × `semio_app()`, 1 × `app_with_registry()`,
16 × `render()`, 1 × `main_window_measures()`, 13 × `dispatch()`, 4 × `handle_action()`).

`🧬️schema/🧬️mutations/🦀️.rs:202-203` — `RasterStore::new(…).await.expect("valid artifact store fixture")`,
`store.dispatch(…).await.expect("apply")`.

`🧬️schema/📸️snapshot/💾️binary/🦀️.rs:116-140` — `ArtifactStore::new(…).await`, `.dispatch(…).await`,
`assert_command_envelope_round_trip(…).await`.

`🧬️schema/🧬️mutations/💾️binary/🦀️.rs:4075-4106, 5068-5092` — `encode_op(…).await` / `decode_op(…).await`,
`ArtifactStore::new(…).await`, `.dispatch(…).await`, `assert_document_text_round_trip(…).await`,
`assert_document_pack_round_trip(…).await`, `assert_command_envelope_round_trip(…).await`.

### (b) `expected VcsArtifactApp<EditorApp<RasterPlayApp>>, found future` (old `:1077`, `:1081`, `:1102`)

Same root cause as (a) — the `testkit::app()` / `app_with_registry()` / `semio_app()` bodies returned the
future instead of the app. Fixed by the `async fn` + `.await` conversion above; no separate change.

### (c) `RasterPlayApp: ArtifactApp is not satisfied` (old `:1484`)

**Framework change.** The trait split is now explicit: `ArtifactEditor` (`🔌️plugin/🦀️.rs:27345 ff.`, the
hand-authored, mostly SYNC trait an app implements) versus `ArtifactApp` (`:11492 ff.`, the ASYNC trait the
`EditorApp<E>` wrapper implements on the editor's behalf). `RasterPlayApp` is the `ArtifactEditor`;
`EditorApp<RasterPlayApp>` is the `ArtifactApp`. `testkit::assert_ingest_idempotent<A>` is bounded
`A: ArtifactApp + Default`.

**Fix.** `✏️editor/🦀️.rs:1483` —
`testkit::assert_ingest_idempotent::<RasterPlayApp, usize>(…)` → `::<EditorApp<RasterPlayApp>, usize>(…).await`.
(The crate's own `pub type RasterApp = VcsArtifactApp<EditorApp<RasterPlayApp>>` already spelled the wrapper
correctly, so only this one turbofish was stale.)

### (d) `dsl::Fault doesn't implement Display` (old `:1171`, `:1214`)

**Framework change.** `Fault` (`🧰️framework/🔨️modules/⚠️diagnostic/🦀️.rs:357`) is `#[derive(Clone, Debug,
PartialEq)]` only — no `Display` impl any more. Its fields are `origin / code: FaultCode(pub String) /
severity / message / scope / span / causes / retryable`.

**Fix.** `✏️editor/🦀️.rs:1171` and `:1214` — the two
`admit_artifact_envelope_ingress_page(…).unwrap_or_else(|(fault, _page)| panic!("…: {fault}"))` closures now
format the fault's fields, the same shape the host itself uses in
`🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs:392` (`format!("{}: {}", fault.code.0, fault.message)`) and
the same fix W4 applied:

```rust
panic!("Raster live envelope page admission failed: {}: {}", fault.code.0, fault.message)
panic!("cancelled Raster page admission failed: {}: {}", fault.code.0, fault.message)
```

### (e) `cannot find module or crate vcs` (`🧬️schema/🧬️mutations/🦀️.rs`, old `:100`, `:103`)

**Framework change (naming, not API).** `vcs::apply_mutation` lives at
`🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:1192` and is mounted at the root of the
`semio_framework_os_kernel` crate. Every consumer reaches it through an `extern crate
semio_framework_os_kernel as vcs;` role alias declared in the consumer's own lib root — see
`🧱️block/📦️packages/🦀️rust/🦀️.rs:16` and `🧩️puzzle/📦️packages/🦀️rust/🦀️.rs:20`. Raster's lib root
(`🖨️raster/📦️packages/🦀️rust/🦀️.rs:13-15`) declares `dsl`, `protocol` and `store` but never declared `vcs`.

**Fix.** `🧬️schema/🧬️mutations/🦀️.rs:79` — `use semio_framework_os_kernel as vcs;` inside the `#[cfg(test)] mod
tests` that needs it. Deliberately NOT a fourth root-level `extern crate … as vcs;`: raster's only `vcs::`
consumer is this test module, so a root alias compiles the lib target with
`warning: unused extern crate` (confirmed by trying it first) — which the sibling zero-warnings ticket would
immediately have to undo. Puzzle/block can afford the root alias because their `vcs::` call sites are
production code.

### (f) `Result<Media, MediaError> is not a future` + `expected Media, found &Media` (old `:1521-1534`)

**Framework change.** Two separate moves:

1. `ArtifactEditor::export_media` (`🔌️plugin/🦀️.rs:27395`) is **sync** — only the `ArtifactApp` mirror
   (`:11588`) is `async`. `semio_framework_plugin::resolve_ready(RasterPlayApp::export_media(…))` therefore
   tried to poll a plain `Result`.
2. `PluginApp::import_media` (`:12041` / `:25969`) now takes `media: Media` **by value**, not `&Media`, and is
   `async`.

**Fix.** `✏️editor/🦀️.rs:1519-1524` — call `RasterPlayApp::export_media("image:out"|"document:out"|"unknown:out", &doc)`
directly (no `resolve_ready`, no `.await`). `:1533` —
`app.import_media("image:in", media, &testkit::meta("local")).await.expect("import image:in")`.
Also dropped the now-dead `let app = RasterPlayApp;` binding at the top of
`raster_io_declares_image_in_out_and_export_media_covers_all_ports` (unused-variable warning).

### (g) `PluginCloseStep::AwaitingInput { .. } not covered` (`🧬️mutations/💾️binary/🦀️.rs`, old `:4132`)

**Framework change.** `PluginCloseStep` (`🔌️plugin/🦀️.rs:11762`) gained a fourth variant:
`AwaitingInput { reason: &'static str }`, produced when a close awaits a worker handback (`:16456`, `:16473`).

**Fix.** `🧬️schema/🧬️mutations/💾️binary/🦀️.rs:4133` — `close_raster_candidate`'s match got the missing arm.
A fresh, unshared candidate can never await a handback, so it panics exactly like the framework's own fixture
closer does (`🔌️plugin/🦀️.rs:6839`):

```rust
semio_framework_plugin::PluginCloseStep::AwaitingInput { reason } => panic!("fresh Raster candidate close unexpectedly awaited input: {reason}"),
```

### (h) `ComponentTree: ToValue is not satisfied` (surfaced only after (a) was fixed)

**Framework change.** `ComponentTree`
(`🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🎭️present.rs:85`) no longer implements the kernel's
`ToValue`, so `dsl::os_pack::json::to_json_string(&tree)` no longer type-checks. The framework's replacement is
`semio_framework_plugin::testkit::project_and_retire_fixture_tree(tree) -> Result<String, &'static str>`
(`🔌️plugin/🦀️.rs:6762`), which projects the tree to JSON **and** drives its bounded
`BuiltTreeRetirement` to terminal — i.e. it also satisfies the owned-surface retirement discipline the old
`to_json_string` call silently skipped.

**Fix.** `✏️editor/🦀️.rs:1088-1090` — raster's `testkit::render` now mirrors `🌊️flow`'s
(`🌊️flow/…/✏️editor/🦀️.rs:2328-2331`):

```rust
pub async fn render(app: &mut RasterApp, body_key: &str) -> String {
    let tree = app.render(body_key, None, &ViewModel::default()).await.expect("render");
    framework_testkit::project_and_retire_fixture_tree(tree).expect("rendered fixture observation and retirement")
}
```

`serde_json`-based projections (`🧩️puzzle`, `🧱️block`, `📕️norm`) were NOT copied — raster carries no `serde`
dependency and must not gain one.

## `Cargo.toml` dev-dependency (reported wasm-build failure)

Reported: ``dependency.semio-framework-async-macros` was not found in `workspace.dependencies``.
Checked, no change needed:

* `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/Cargo.toml` `[dev-dependencies]` already reads
  `semio-framework-async-macros = { path = "../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust" }`
  — the same `path = …` form `🧩️puzzle` and `🔱️trinity` use.
* Root `Cargo.toml:161` also still carries the `[workspace.dependencies]` entry.

So the `workspace = true` form the wasm lane tripped on is already gone; nothing was edited here (W7 owns
plugin `Cargo.toml` metadata).

## Files touched

1. `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
2. `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
3. `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs`
4. `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs`

`git diff --stat`: `4 files changed, 98 insertions(+), 93 deletions(-)`.
Nothing under `🧰️framework/`, no plugin `Cargo.toml`, no plugin registry, no root `📜️script.ts`, no
`🗑️generated` folder was touched.

## `cargo test -p semio-s-plugin-raster --lib`

_(filled in below — see "Test run".)_
