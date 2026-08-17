# Constitutional Crate Split — Recipe (validated on `note`)

## Target layout
For plugin `<p>` with app `<a>` (folder = app id minus plugin prefix and `-play` suffix; when the plugin has exactly one app matching the plugin name, folder = plugin name):

```
s/plugin/<p>/
  rs/lib.rs                     # bundle: semio_plugin! macro + wasm bridges + setup fn only (thin)
  app/<a>/
    rs/lib.rs                   # semio-s-app-<a>            general: document entities + derives
    engine/rs/lib.rs            # semio-s-app-<a>-engine     headless compute + example/initial builders
    dsl/rs/lib.rs                # semio-s-app-<a>-dsl        fixture + parse/print wrappers + dsl law tests
    op/rs/lib.rs                 # semio-s-app-<a>-op         operation enum + Operation/OperationDiff impls + op-text laws
    pack/rs/lib.rs                # semio-s-app-<a>-pack       encode/decode wrappers + pack law tests
    protocol/rs/lib.rs            # semio-s-app-<a>-protocol   encode_op/decode_op wrappers + protocol law tests
    ui/rs/lib.rs                  # semio-s-app-<a>-ui         DocumentApp impl, render/panels, manifest/create_X_app, all #[cfg(test)] app-behavior tests
```

For multi-app plugins (puzzle, fem, trinity, norm, s/space, reasoning-if-multi), repeat `app/<a>/` per app; shared code used by ≥2 apps of the plugin goes in an extra non-constitutional `s/plugin/<p>/shared/rs` crate (create only if actually needed).

## Content mapping (apply exactly, verbatim code — do not rewrite logic)

- **rs**: entity structs/enums with `#[derive(dsl::DslRecord/DslEnum/DslDocument, ...)]`, their `default_*()` helper fns, the `X_DOCUMENT_SCHEMA` constant. Nothing else. Deps: `dsl` (framework kernel facade) + `store` (kernel — **required**, the derive macros expand to `impl store::DocumentDsl`/`DocumentPack` directly) + serde/serde_json.
- **engine**: ALL pure compute over the document (tree helpers, id generation, media export/import compute, `empty_X_document()`, `semio_example_document()`/`_json()`). Deps: `rs`, the app's own `dsl` crate (for the example-fixture constant), `store`, `semio-framework-plugin` (if media import/export needs SDK types like `DwgDrawing`).
  - **CRITICAL — avoid circular deps**: if `apply_X_operation(projection, operation: &XOperation)` needs to match on the operation ENUM, it must live in `op`, NOT `engine` — `op` already depends on `engine`, so `engine` can never depend back on `op`. `NoteDiff::apply` calls it as a private fn inside `op`.
- **op**: the operation enum (`#[derive(dsl::DslOps)]`) + diff struct + `impl protocol::Operation`/`OperationDiff` + the private `apply_X_operation` fn (see above). Deps: `rs`, `dsl` (framework kernel, for the derive), `protocol` (framework kernel, for the `Operation`/`OperationDiff` traits), `store`. Dev-dep: `engine` (only if tests need `empty_X_document()`).
- **dsl**: `pub const X_EXAMPLE_TEXT: &str = include_str!(...)` (recompute the relative path — the fixture lives at `s/plugin/<p>/example/*` or `app/<a>/example/*`, check where it actually is), `parse_dsl`/`print_dsl` thin wrappers over `store::DocumentDsl`, `assert_dsl_round_trip` tests (one for the fixture, one for a hand-built "representative document" exercising every variant — copy verbatim from the original file's DSL/OpText test region if present). Deps: `rs`, `store`.
- **pack**: `encode`/`decode` thin wrappers over `store::DocumentPack` (error type is `store::PackError`), `assert_dsl_pack_equivalence` tests. Deps: `rs`, `pack` (kernel, may end up unused — fine), `store`. Dev-dep: the app's `dsl` crate (to parse the fixture for a pack test).
- **protocol**: `encode_op`/`decode_op` thin wrappers over `protocol::OpBinary` (error type is `protocol::ProtocolError`, from the framework kernel protocol facade's re-export of `protocol_core::ProtocolError`), `assert_op_text_binary_equivalence` test, and — if the original file had a whole-store round-trip test (`assert_document_text_round_trip`/`assert_document_pack_round_trip` via a real `DocumentStore`) — put it here too (needs no extra deps: `NoteDocument` already satisfies `DocumentDsl+DocumentPack` via derive in `rs`, no need to depend on the `dsl`/`pack` surface crates for this). Deps: `rs`, `op`, `protocol` (kernel), `store`. Dev-dep: `engine` (for `empty_X_document()`).
- **ui**: everything else — canvas/wire-shape event types, action-dispatch helpers, terminology (`app_labels!`), command labels, panels, render/scenes/shell, the `XPlayApp` struct + `impl DocumentApp`, `create_X_app()` manifest builder, and **every `#[cfg(test)]` test that exercises the live app** (`new_app::<XPlayApp>()`, `handle_action`, `render`). Deps: `rs`, `engine`, `op`, `semio-framework-plugin`. Dev-dep: the app's `dsl` crate if a test needs the raw fixture text directly.
  - **Test-module import gotcha**: add `use semio_framework_plugin::PluginApp;` — `VcsDocumentApp<A>::render`/`handle_action` are `PluginApp` trait methods, not inherent, and testkit's `new_app`/`new_app_with_registry` return `VcsDocumentApp<A>`.
- **plugin bundle** (`s/plugin/<p>/rs/lib.rs`, already exists — REWRITE it, do not create new): keep only the `register_X_exports()` fn (host/media-codec registration, calling into `engine` + `ui`) and the `semio_framework_plugin::semio_plugin! { id, label, version, setup, apps: [ ui::create_X_app => ui::XPlayApp ] }` macro block. Update its `Cargo.toml` to depend on just `semio-framework-os`, `semio-framework-plugin`, the app's `rs`, `engine`, `ui` crates (drop the old direct `vcs`/`store`/`protocol`/`dsl` deps — no longer needed at this layer).

## Mechanical steps

1. **Read the ENTIRE original `lib.rs` file first**, including deep into any `//#region 🧪️Tests`/nested sub-regions — test regions are often much larger than the initial `#region` marker suggests (in `note` a whole extra `🔖️DslAndOpText` sub-region with ~10 more tests sat past where a naive line-count estimate would stop). Missing this means silently losing test coverage.
2. `mkdir -p s/plugin/<p>/app/<a>/{rs,engine/rs,dsl/rs,op/rs,pack/rs,protocol/rs,ui/rs}` (mkdir -p handles the nesting fine).
3. **Compute every relative path with Python's `os.path.relpath`, never by hand** — off-by-one `../` arithmetic errors are the single biggest time sink. One-liner: `python3 -c "import os; print(os.path.relpath('TARGET_DIR', 'FROM_DIR'))"`. Do this for every single path in every Cargo.toml before writing it.
4. Write each `Cargo.toml` (package name `semio-s-app-<a>[-<slot>]`, `[lints] workspace = true`, `[lib] path = "lib.rs"`, deps per the mapping above using `package = "..."` where the dep-key differs from the actual package name).
5. Write each `lib.rs`, moving code **verbatim** (preserve doc comments, region markers, exact formatting) — this is a redistribution, not a rewrite.
6. Rewrite the plugin bundle's `lib.rs` + `Cargo.toml`.
7. **Register all 7 new crate paths as explicit root `Cargo.toml` workspace members** — do NOT rely on implicit inclusion via the dependency graph; `pack` and `protocol` in particular are usually only reachable via `[dev-dependencies]` of sibling crates and are invisible to Cargo's implicit member detection otherwise. Report the exact 7 paths back rather than editing root `Cargo.toml` yourself if working alongside other concurrent edits to that shared file — **or** edit it directly if you're the only one touching it in this pass (check for `<<<<<<<` conflict markers / recent mtime first).
8. Run `cargo metadata --no-deps --format-version 1` (path/member sanity), then `cargo check -p <all 8 new package names> --all-targets`, fixing errors as they surface (mostly: missing `store` dep on `rs`, missing `PluginApp` import in `ui` tests, wrong relative path arithmetic).
9. Run `cargo test -p <all 8 crates minus the bundle>` and confirm pass. Pre-existing unrelated failures (e.g. an icon-catalog gap unconnected to your change) are fine to leave — verify by confirming the failing assertion doesn't reference anything you touched.
10. Delete nothing extra; don't leave the old monolithic content behind — the plugin bundle file is fully replaced.

## Verified working example
See the already-completed `note` app: `s/plugin/note/rs` (bundle) + `s/plugin/note/app/note/{rs,engine,dsl,op,pack,protocol,ui}/rs`. Read these files directly as a concrete template — they compile and 24/29 tests pass (5 pre-existing icon-catalog failures unrelated to structure).
