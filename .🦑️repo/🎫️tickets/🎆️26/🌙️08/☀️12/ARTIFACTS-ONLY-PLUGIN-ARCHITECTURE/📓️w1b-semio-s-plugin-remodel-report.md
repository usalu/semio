# W1b — `📸️remodel` (`semio-s-plugin-remodel`) — `register()` → `declaration()` conversion

`apa-status: complete`

## Clearance

`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md` lists
`📸️remodel` under **"RELEASED — Wave C / late Wave M lanes complete"** (§ table row: *"34 mutations
replace all 20 `Set*`; no whole-collection setter survives; `cargo check` 0 errors"*). Not present in
either HELD section. Free to edit.

## What changed

### 1. `📸️remodel`'s artifact engine — `register()` → `declaration()`

File: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`

- **:34-42 (old) → :36-44 (new)** — the old side-effecting `pub fn register()` (5 calls: root's
  `io_registry::register()`, `register_artifact_schema()`, `register_artifact_inference()`,
  `register_pilot_languages()`, `register_app_schema()`, plus a direct
  `register_document_codec_for_app::<RemodelPlayApp>` call) replaced by:
  ```rust
  pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
      semio_framework_plugin::ArtifactDeclaration::builder("s.remodel")
          .schema(crate::artifacts::remodel::schema::remodel_artifact_schema_descriptor())
          .inferences([crate::artifacts::remodel::standards::v1::subsets::any::schema::inferences::remodel_artifact_inference_descriptor()])
          .composers(crate::artifacts::remodel::standards::v1::engine::io_registry::entries())
          .languages(pilot_languages())
          .document_codec::<crate::apps::remodel::RemodelPlayApp>()
          .build()
  }
  ```
  Single artifact, single standard (`🔖️1`), single subset (`✳️any`) — this plugin has exactly one
  `register()` to convert, so one `declaration()` covers the whole artifact (no fold-multiple-in or
  repeated `.artifact()` calls needed).
- **`kind: "s.remodel"`** — chosen to match the artifact's own composer dialect
  (`REMODEL_DIALECT: Dialect { artifact_kind: "s.remodel", .. }`, defined in this same file's
  `io_registry` module, :595 pre-edit) exactly as note's exemplar keys off `NOTE_DIALECT.artifact_kind
  == "s.note"`. **Not** `"3d.remodel"` — that string is `ArtifactKindSpec.id` (a different, OS
  media-capability registration in the artifact's root `🦀️component.rs`, `artifact_kind()`, untouched
  by this ticket) and would fail `register_all`'s ownership check against every composer entry, all of
  which read/write `"s.remodel"`.
- **`register_pilot_languages()` → private `pilot_languages() -> &'static [dsl::LanguageSpec]`** — same
  5 language specs (`remodel.document`/`remodel.op`/`remodel.diff`/`remodel.pack`/`remodel.spr`),
  verbatim, now built once behind a `OnceLock<Vec<_>>` and leaked to `&'static` (mirrors note's own
  helper exactly — `dsl::passthrough_hooks` isn't `const fn`, so the array can't be a `const`).
- **`register_artifact_schema()`/`register_artifact_inference()` deleted** — both had exactly one call
  site each (inside the old `register()`, confirmed by grep before deleting); their bodies are now
  inline `.schema(...)`/`.inferences([...])` arguments instead of free functions.
- `.composers(...)` points at this same file's own `io_registry::entries()` (:581-682 pre-edit,
  unmoved) — the REAL 8-entry table (`composer_entry_of::<RemodelAnyComposer>()` plus 7 hand-written
  export entries: LAS/PLY/PNG/JSON/DWG/STL/GLTF/OBJ). This is the same table the artifact root file's
  own thin `io_registry::register()` wrapper (`🗿️artifacts/📸️remodel/🦀️component.rs:1132-1154`) already
  forwarded to via `register_composer_entries(v1::entries())` — I pointed the declaration at the real
  source, not that wrapper (its `entries()` returns `&'static [&'static ComposerEntry]`, an
  incompatible type for `.composers()`, which wants `&'static [ComposerEntry]`).

### 2. Artifact root — `register_artifact_schema`/`register_artifact_inference` doc-neighbors untouched

`🗿️artifacts/📸️remodel/🦀️component.rs`'s own `io_registry` module (:1131-1155, thin
`entries()`/`compose()`/`register()` wrapper around the engine file's real table) is now orphaned
(zero call sites repo-wide after this conversion, confirmed by grep) — left in place rather than
deleted, exactly matching what the W1 report did for note's own equivalent orphaned module: removing
it is unrelated cleanup outside this wave's scope.

### 3. Plugin root — `.setup()` → `.artifact()`

File: `✏️s/🔌️plugins/📸️remodel/🦀️component.rs`

```rust
pub fn plugin() -> Plugin {
    Plugin::builder("remodel")
        .label("Remodel")
        .version("0.1.0")
        .setup(crate::apps::remodel::config::schema::register_app_schema)
        .artifact(crate::artifacts::remodel::engine::declaration())
        .register_document_app::<crate::apps::remodel::RemodelPlayApp>(crate::apps::remodel::create_remodel_app())
        .build()
}
```

## Does `.setup()` survive, and why

**Yes, narrowed to exactly one call**: `crate::apps::remodel::config::schema::register_app_schema`.
This registers `RemodelPlayApp`'s CONFIG/PRESENCE schema — an app-scope concern
(`register_app_schema_descriptor`), which is one of the two §6 functions `ArtifactDeclaration`
deliberately has no field for (per the mechanism's own doc at
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:935-938` and the W1 report's
field-mapping table). Identical shape to note's exemplar. No other reason for `.setup()` survives —
every other registration the old `register()` performed now runs through `.artifact(declaration())`.

## Step 4 — escape hatches and deps

- `grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_app_io\|register_os_media_"` across
  the whole plugin: **zero matches**. Nothing to relocate or delete.
- `grep -rn "semio_framework_os::"` in the crate: **2 hits**, both in
  `⚙️engine/🦀️component.rs` (`remodel_png_export`, using `semio_framework_os::OsMediaExportResult` — a
  plain data type, not a registration call). `semio-framework-os` therefore stays in
  `📦️packages/🦀️rust/Cargo.toml` — the purge condition ("grep empty") does not hold.

## Step 5 — inventory

- `thread_local!`: **0** matches anywhere in the plugin.
- `static` declarations (5 total, all inventoried, none a host/engine handle):
  - `⚙️engine/🦀️component.rs:118` `static COUNTER: AtomicU64` — monotonic id counter (`next_remodel_id`), plain uniqueness generator, not draft/gesture state.
  - `⚙️engine/🦀️component.rs:50` (new) `static LANGUAGES: OnceLock<Vec<dsl::LanguageSpec>>` — this conversion's own `pilot_languages()` cache.
  - `⚙️engine/🦀️component.rs:581` `static ENTRIES: OnceLock<Vec<ComposerEntry>>` — the real composer table cache (`io_registry::entries()`).
  - `🗿️artifacts/📸️remodel/🦀️component.rs:1137` `static ENTRIES: OnceLock<Vec<&'static ComposerEntry>>` — the orphaned root-file wrapper's own cache (see above).
  - `⚙️engine/🌟️feature/🦀️component.rs:250,815` two `static PATTERN: OnceLock<[...; 256]>` — lazily-built lookup tables for the ORB/AKAZE feature descriptor (pure derived constants from a fixed algorithm, recomputed once, never mutated after init).
  - None hold a host/engine handle (no `OnceLock<SomeEngineHost>` pattern anywhere in this plugin) and none are interior-mutable app-gesture/draft state — all five are either monotonic counters or lazily-built immutable derived caches.
- `std::fs`/`std::env`/`std::process`/`Command::new` outside `#[cfg(test)]`: **0** matches anywhere in the plugin.

## `3d.mesh` note (plugin-specific instruction)

Confirmed: `📸️remodel` tags engine output with `schema: "stdio.semio.mesh"` (`mesh_data_to_semio_mesh`,
`⚙️engine/🦀️component.rs`) when building export payloads, and declares a `mesh:out` **app IO port**
pinned to `kind_id: "3d.mesh"` in `remodel_mesh_out_port()` (`🗿️artifacts/📸️remodel/🦀️component.rs:151-163`,
app-level `AppIo`, untouched — outside `ArtifactDeclaration`'s scope, which only covers the artifact's
own composer/schema/inference/language/codec/migration registrations). Grepped the whole plugin for any
`register_*` call naming `"3d.mesh"` as an owned kind: **zero** — `📸️remodel` registers no IO, composer,
schema, or inference for `3d.mesh`; it only *references* that kind id as a port pin (consumer-side
typing, "this port carries meshes of this kind," not ownership). Nothing to leave alone beyond
confirming it: this plugin's `declaration()` builder key is `"s.remodel"` throughout, never `"3d.mesh"`,
so `register_all`'s ownership check has nothing to trip over here.

## Step 3 — plugin root closure

`✏️s/🔌️plugins/📸️remodel/` already contained only `🦀️component.rs`, `🎛️apps/`, `📦️packages/`,
`🗿️artifacts/` before this session touched it (no `AGENTS.md`/`README.md` present, no
`🛂️manifest/`/`🎟️capabilities/`/`🔧️setup/` dirs to delete). Nothing to do for this step.

## Verification

**1. `#[path]` mounts in `📦️glue.rs` resolve** — scripted (Python, normpath + `os.path.isfile` against
each literal `#[path = "..."]` string, resolved relative to `📦️glue.rs`'s own directory):
```
checked: 178
missing: 0
```

**2. `include_str!`/`include_bytes!` targets resolve** — scripted (walked every `.rs` file under the
plugin, resolved each literal target relative to its OWN containing file's directory, not
pattern-substituted):
```
checked: 48
missing: 0
```

**3. `cargo metadata --no-deps --format-version 1`**:
```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo metadata --no-deps --format-version 1 > /dev/null && echo OK
OK
```

**4. `cargo check -p semio-s-plugin-remodel --all-targets`** (`RUSTC_WRAPPER=""`), full transcript at
`scratch-w1b-remodel-cargo-check.txt` in this ticket folder (8083 lines — a from-scratch build against
this ticket's own isolated `🎯️target` dir, so it recompiled the whole dependency graph, not just this
crate):
```
warning: `semio-s-plugin-remodel` (lib) generated 16 warnings (run `cargo fix --lib -p semio-s-plugin-remodel` to apply 13 suggestions)
warning: `semio-s-plugin-remodel` (lib test) generated 17 warnings (14 duplicates) (run `cargo fix --lib -p semio-s-plugin-remodel --tests` to apply 2 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 11m 04s
```
**0 errors** (`grep -c "^error"` on the full transcript → `0`). All 16/17 warnings are pre-existing
(unused imports in `🔺️diff/📝️text`/`🚪️io`/`glue.rs`, one `dead_code` on `RemodelEngine.artifact`, one
elided-lifetime and one ambiguous-glob-import from `protocol::testkit` — a cross-crate glob collision
in framework/os's own glue.rs, unrelated to this plugin) — grepped the transcript for `declaration`/
`pilot_languages`: **zero warnings on either new symbol**.

## Files touched

- `✏️s/🔌️plugins/📸️remodel/🦀️component.rs` — `.setup(engine::register)` → `.setup(config::schema::register_app_schema)` + `.artifact(engine::declaration())`.
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` — `register()`/`register_pilot_languages()`/`register_artifact_schema()`/`register_artifact_inference()` → `declaration()` + private `pilot_languages()`.

Nothing else created, moved, or deleted.

## sharedFileRequests

None. This session touched only `📸️remodel`'s own two files; nothing outside `✏️s/🔌️plugins/📸️remodel/`
was written.
