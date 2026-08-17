# W1b — `🌍️gis` (`semio-s-plugin-gis`) — `register()` → `declaration()` conversion

`apa-status: in-progress` — conversion complete and self-consistent (both `#[path]`-mount and
`include_str!`/`include_bytes!` resolution scripted-verified, `cargo metadata` clean); the one
`cargo check -p semio-s-plugin-gis --all-targets` run (§Verification below) never reached
`semio-s-plugin-gis` itself — it aborted earlier in the dependency graph on a `semio-s-plugin-stdio`
compile error unrelated to this change (see below). Per "Prefer ONE cargo run at the very end", not
re-run; do not treat the absence of a green gis-specific result as a passing OR failing verdict for
this conversion — it is simply not yet confirmed.

## Clearance

`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md` lists
`🌍️gis` under **"RELEASED — Wave C / late Wave M lanes complete"** (§ table row: *"`🗺️gismap`,
`🏔️gisterrain` — `cargo test` 171/0; 8× emoji collision fixed; both config mutations semanticized; 42
TS mirrors"*). Not present in either HELD section. Free to edit.

## Plugin-specific starting state

The plugin was already closed at 0 errors and its root `🦀️component.rs` had ALREADY been partially
folded per this ticket by a prior pass: a `register_gis_exports()` fn wrapped in `.setup()` that
called both artifacts' `register_pilot_languages()` plus both apps' `register_app_schema()` — real
fan-out code moved out of a (now-absent) `🔧️setup/` facet directory, per the plugin-specific
instruction. That prior fold was NOT yet the declaration mechanism (still imperative calls), so W1b's
job was converting that fan-out into two `.artifact(declaration())` calls plus narrowing `.setup()`
down to only the app-schema half. `🎛️apps`/`🗿️artifacts`/`📦️packages`/`AGENTS.md`/`README.md`/
`🦀️component.rs` was already the plugin root's complete file list — no `🛂️manifest/`/`🎟️capabilities/`/
`🔧️setup/` dirs existed to delete (confirmed by directory listing before editing).

`2d.map` (owned by `gismap`, per plugin-specific note): confirmed gis registers no IO for it — grepped
the whole plugin for any `register_*`/`.composers(...)` call naming `"2d.map"`: zero. `gismap`'s own
`declaration()` composer table only ever writes/reads `"s.gismap"` and the foreign stdio export
dialects (svg/pdf/png/json/dwg/dxf); `"2d.map"` appears only as `AppIo.artifact.id` /
`MediaPortSpec.kind_id` (interchange-kind port typing, app-level, outside `ArtifactDeclaration`'s
scope) — nothing to touch.

`3d.mesh` (declared by `🏔️gisterrain`'s `mesh_artifact_kind()`, per plugin-specific note): left alone,
untouched by this conversion — it is an `ArtifactKindSpec` registered via the app builder's own
`.artifact_kind(...)` call (`🎛️apps/🧊️3d/🦀️component.rs:181`), a different mechanism from
`ArtifactDeclaration` entirely, and outside this wave's scope regardless.

## What changed

### 1. `gismap`'s artifact engine — `register()` → `declaration()`

File: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`

- **:439-452 (old) → :439-515 (new, region renamed `🔖️Registration` → `🔖️Register`)** — the old
  side-effecting `pub fn register()` (outer `io_registry::register()` + `register_pilot_languages()` +
  `register_artifact_schema()` + `register_artifact_inferences()` + a direct
  `register_document_codec_for_app::<Gis2dPlayApp>` call) replaced by:
  ```rust
  pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
      semio_framework_plugin::ArtifactDeclaration::builder("s.gismap")
          .schema(crate::artifacts::gismap::schema::gismap_artifact_schema_descriptor())
          .inferences([crate::artifacts::gismap::standards::v1::subsets::any::schema::inferences::gismap_artifact_inference_descriptor()])
          .composers(crate::artifacts::gismap::standards::v1::engine::io_registry::entries())
          .languages(pilot_languages())
          .document_codec::<crate::apps::gis2d::Gis2dPlayApp>()
          .build()
  }
  ```
- **`kind: "s.gismap"`** — matches the artifact's own composer dialects
  (`GISMAP_DIALECT: Dialect { artifact_kind: "s.gismap", .. }`, same file's `io_registry` module),
  exactly as note's exemplar keys off `NOTE_DIALECT.artifact_kind == "s.note"`. **Not** `"2d.map"`
  (that's `ArtifactKindSpec.id`, a separate OS media-capability registration, untouched — see above)
  and **not** `"s.gis.gismap"` (that's the schema descriptor's own internal `id`, a different field
  entirely, unrelated to `register_all`'s ownership check).
- **`register_pilot_languages()` → private `pilot_languages() -> &'static [dsl::LanguageSpec]`** — same
  5 language specs (`gis.gismap`/`gis.gismap.op`/`gis.gismap.diff`/`gismap.pack`/`gismap.spr`),
  verbatim, now built once behind a `OnceLock<Vec<_>>` and leaked to `&'static` (mirrors note's helper
  exactly).
- **`register_artifact_schema()`/`register_artifact_inferences()` deleted** — each had exactly one call
  site (inside the old `register()`, confirmed by grep before deleting); their bodies are now inline
  `.schema(...)`/`.inferences([...])` builder arguments.
- `.composers(...)` points at this same file's own `io_registry::entries()` (unmoved, :684-... — the
  REAL 7-entry table: `composer_entry_of::<GisMapAnyComposer>()` plus 6 hand-written export entries
  SVG/PDF/PNG/JSON/DWG/DXF). This is the same table the artifact root's own thin `io_registry::register()`
  wrapper (`🗿️artifacts/🗺️gismap/🦀️component.rs:118-120`) already forwarded to via
  `register_composer_entries(v1::entries())` — pointed the declaration at the real source, not the
  wrapper (whose `entries()` returns `&'static [&'static ComposerEntry]`, an incompatible type for
  `.composers()`, which wants `&'static [ComposerEntry]`).

### 2. `gisterrain`'s artifact engine — `register()` → `declaration()`

File: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`

Identical shape:
```rust
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.gisterrain")
        .schema(crate::artifacts::gisterrain::schema::gisterrain_artifact_schema_descriptor())
        .inferences([crate::artifacts::gisterrain::standards::v1::subsets::any::schema::inferences::gisterrain_artifact_inference_descriptor()])
        .composers(crate::artifacts::gisterrain::standards::v1::engine::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::gis3d::Gis3dPlayApp>()
        .build()
}
```
`kind: "s.gisterrain"` (matches `GISTERRAIN_DIALECT.artifact_kind`, same rationale as gismap above; not
`"3d.mesh"` — that string is `mesh_artifact_kind()`'s `ArtifactKindSpec.id`, app-level, untouched).
`.composers(...)` points at the real 8-entry table (LAS/PLY/PNG/JSON/DWG/STL/GLTF/OBJ export +
`composer_entry_of::<GisTerrainAnyComposer>()`). `register_pilot_languages()`/`register_artifact_schema()`/
`register_artifact_inferences()` deleted the same way; `pilot_languages()` carries the same 5 specs
(`gis.gisterrain`/`gis.gisterrain.op`/`gis.gisterrain.diff`/`gisterrain.pack`/`gisterrain.spr`).

### 3. Both artifact roots — outer `io_registry` modules left orphaned, untouched

`🗿️artifacts/🗺️gismap/🦀️component.rs:98-122` and `🗿️artifacts/🏔️gisterrain/🦀️component.rs:63-87` each
carry a thin `io_registry` module (`entries()`/`compose()`/`register()` wrapper around the engine
file's real table). Their `register()` fns are now orphaned (zero call sites repo-wide after this
conversion — confirmed by grep) — left in place rather than deleted, exactly matching what the W1
exemplar did for note's own equivalent orphaned module: removing it is unrelated cleanup outside this
wave's scope.

### 4. Plugin root — fan-out `.setup()` narrowed, `.artifact()` added

File: `✏️s/🔌️plugins/🌍️gis/🦀️component.rs`

```rust
fn register_gis_exports() {
    crate::apps::gis2d::config::schema::register_app_schema();
    crate::apps::gis3d::config::schema::register_app_schema();
}

pub fn plugin() -> Plugin {
    Plugin::builder("gis")
        .label("GIS")
        .version("0.1.0")
        .setup(register_gis_exports)
        .artifact(crate::artifacts::gismap::engine::declaration())
        .artifact(crate::artifacts::gisterrain::engine::declaration())
        .register_document_app::<crate::apps::gis2d::Gis2dPlayApp>(crate::apps::gis2d::create_gis2d_app())
        .register_document_app::<crate::apps::gis3d::Gis3dPlayApp>(crate::apps::gis3d::create_gis3d_app())
        .build()
}
```
The two `crate::artifacts::{gismap,gisterrain}::engine::register_pilot_languages()` calls that used to
sit in `register_gis_exports()` are gone — that responsibility now lives on each artifact's own
`declaration()`, wired via `.artifact(...)`.

## Does `.setup()` survive, and why

**Yes, narrowed to exactly one fn (`register_gis_exports`) carrying exactly two calls**:
`crate::apps::gis2d::config::schema::register_app_schema` and
`crate::apps::gis3d::config::schema::register_app_schema`. Both register their respective
`ArtifactApp`'s CONFIG/PRESENCE schema — an app-scope concern (`register_app_schema_descriptor`), one
of the two §6 functions `ArtifactDeclaration` deliberately has no field for (per the mechanism's own
doc at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:935-938`). Two apps means two
calls instead of note's one, hence the wrapper fn survives (a bare `fn()` pointer can't carry two
calls inline) — same shape as every other two-app-schema plugin in this wave. No other reason for
`.setup()` survives: every other registration the old fan-out performed (both artifacts' pilot
languages) now runs through `.artifact(declaration())`.

## Step 4 — escape hatches and deps

- `grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_app_io\|register_os_media_"` across
  the whole plugin: **zero actual calls** — one doc-comment mention of `register_dwg_import_handler`
  (`🗺️gismap/…/⚙️engine/🦀️component.rs:357`, describing a frozen upstream signature, not a call site).
  Nothing to relocate or delete.
- `grep -rn "semio_framework_os::"` in the crate: **4 hits**, all inside `#[cfg(test)]` test bodies in
  `🗺️gismap/…/⚙️engine/🦀️component.rs` (`semio_framework_os::DwgEntity`/`DwgColor` — plain data-type
  construction, not a registration call). `semio-framework-os` therefore stays in
  `📦️packages/🦀️rust/Cargo.toml` — the purge condition ("grep empty") does not hold.

## Step 5 — inventory

- `thread_local!`: **0** matches anywhere in the plugin.
- `static` declarations (7 total, all inventoried, none a host/engine handle):
  - `🗿️artifacts/🗺️gismap/🦀️component.rs:104` `static ENTRIES: OnceLock<Vec<&'static ComposerEntry>>` — the orphaned root-file `io_registry` wrapper's own cache (§3 above).
  - `⚙️engine/🦀️component.rs:457` (gismap, new) `static LANGUAGES: OnceLock<Vec<dsl::LanguageSpec>>` — this conversion's own `pilot_languages()` cache.
  - `⚙️engine/🦀️component.rs:566` (gismap) `static ONCE: std::sync::Once` — test-only guard (`ensure_stdio_semio_registered_for_tests`, inside `#[cfg(test)] mod tests`) so `io_dispatch`'s stdio bridge resolves in a bare `cargo test` process.
  - `⚙️engine/🦀️component.rs:684` (gismap) `static ENTRIES: OnceLock<Vec<ComposerEntry>>` — the real composer table cache (`io_registry::entries()`).
  - `🗿️artifacts/🏔️gisterrain/🦀️component.rs:69` `static ENTRIES: OnceLock<Vec<&'static ComposerEntry>>` — orphaned root-file wrapper's cache, same shape as gismap's.
  - `⚙️engine/🦀️component.rs:247` (gisterrain, new) `static LANGUAGES: OnceLock<Vec<dsl::LanguageSpec>>` — this conversion's `pilot_languages()` cache.
  - `⚙️engine/🦀️component.rs:395` (gisterrain) `static ENTRIES: OnceLock<Vec<ComposerEntry>>` — the real composer table cache.
  - None hold a host/engine handle (no `OnceLock<SomeEngineHost>` pattern anywhere) and none are
    interior-mutable app-gesture/draft state — all seven are either data caches or a test-setup guard.
- `std::fs`/`std::env`/`std::process`/`Command::new` outside `#[cfg(test)]`: **1 finding** —
  `🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🗺️map/🦀️component.rs:77` `std::env::var("SEMIO_ASSET_BASE_URL")`
  in `apply_gis_map_tile_base_url` (production `render()` path, rewrites tile URL templates to an
  absolute asset base the host publishes). Pre-existing, unrelated to this conversion — inventoried
  per Step 5, not touched.

## Step 3 — plugin root closure

`✏️s/🔌️plugins/🌍️gis/` already contained only `🦀️component.rs`, `AGENTS.md`, `README.md`, `🎛️apps/`,
`📦️packages/`, `🗿️artifacts/` before this session touched it — confirmed by a top-level directory
listing before editing. No `🛂️manifest/`/`🎟️capabilities/`/`🔧️setup/` dirs and no `#[path]` mounts to
them existed. Nothing to delete or relocate for this step.

## Verification

**1. `#[path]` mounts in `📦️glue.rs` resolve** — scripted (Python, normpath + `os.path.isfile` against
each literal `#[path = "..."]` string, resolved relative to `📦️glue.rs`'s own directory):
```
total #[path] entries: 326
non-dot entries checked: 154
missing: 0
```

**2. `include_str!`/`include_bytes!` targets resolve** — scripted (walked every `.rs` file under the
plugin, resolved each literal target relative to its OWN containing file's directory, not
pattern-substituted):
```
checked: 100
missing: 0
```

**3. `cargo metadata --no-deps --format-version 1`**:
```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo metadata --no-deps --format-version 1 > /dev/null && echo OK
OK
```

**4. `cargo check -p semio-s-plugin-gis --all-targets`** (`RUSTC_WRAPPER=""`) — ran once, real output,
1414 lines total (full log at
`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/scratch-w1b-gis-cargo-check.txt`).
Started blocked on the shared `🎯️target` dir's file lock (dozens of concurrent sessions' `cargo
check` processes queued on the same ticket's shared `CARGO_TARGET_DIR` at the time — confirmed by
`ps aux`, ~40-70 live `cargo` processes throughout the wait), then proceeded once the lock cleared:

```
    Blocking waiting for file lock on build directory
[... framework crates compile, 40+ pre-existing warnings unrelated to this change ...]
    Checking semio-s-plugin-stdio v0.1.0 (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust)
error: couldn't read `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/./././././././././../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs`: No such file or directory (os error 2)
    --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs:5572:37
     |
5572 | ...                   pub mod inverse;
     |                       ^^^^^^^^^^^^^^^^

error: could not compile `semio-s-plugin-stdio` (lib) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
[... unrelated warnings from semio-framework-os-infinite continue, log ends there — semio-s-plugin-gis
     itself is never reached; cargo aborts the dependency graph on stdio's failure first]
```

**This is not gis's bug.** `semio-s-plugin-gis` depends on `semio-s-plugin-stdio`, and cargo's build
graph aborts on the first hard dependency failure before compiling dependents. The missing file is a
`📄set-snapshot` mutation facet's `↩️inverse` leaf under `🗿️artifacts/🧿️semio/...` — `🗄️stdio` is
explicitly listed in SMO's `📓️plugin-release-status.md` under **"NOT SMO'S TO RELEASE"** (*"claimed by
UCAS (#2548) for the `🧿️semio` subset roster restructure"*), i.e. another session's in-progress
refactor deleting exactly the kind of `SetSnapshot`-named mutation directory this repo's own CLAUDE.md
bans (`✏️s/**` MUST NOT write `SetSnapshot`/`NoMutation`/`CollectionMutation` anywhere) — a live rename
caught mid-flight by cargo's file read. Confirmed unrelated to this session's edits three ways:
- `grep -n "🌍️gis\|plugin_gis\|plugin-gis"` over the full 1414-line log: **zero matches**.
- `grep -c "^error"` over the full log: **2**, both on the two lines quoted above (both about stdio's
  missing file, none about gis).
- The failing path (`🗄️stdio/.../🗿️artifacts/🧿️semio/.../📄set-snapshot/↩️inverse/...`) touches no file
  this session read, edited, or that `semio-s-plugin-gis`'s own `#[path]`/`include_str!` trees
  reference (verified in items 1–2 above, both scripted against `🌍️gis`'s own tree only).

Per "Prefer ONE cargo run at the very end. Several sessions share this machine... never kill a build
for being slow" and the project's own "Concurrent Cargo Workspace Churn" pattern (repo-wide failures
from another session's in-flight refactor, 30-90+ min, poll rather than chase), this run is not being
retried inside this session. A follow-up `cargo check -p semio-s-plugin-gis --all-targets` once UCAS's
`🗄️stdio` roster restructure lands is the remaining action to flip this report's status to `complete`.

## Files touched

- `✏️s/🔌️plugins/🌍️gis/🦀️component.rs` — `register_gis_exports()` narrowed to the two app-schema
  calls; `.setup(register_gis_exports)` kept; added `.artifact(crate::artifacts::gismap::engine::declaration())`
  and `.artifact(crate::artifacts::gisterrain::engine::declaration())`.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` —
  `register()`/`register_pilot_languages()`/`register_artifact_schema()`/`register_artifact_inferences()`
  → `declaration()` + private `pilot_languages()`.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` —
  same conversion.

Nothing else created, moved, or deleted.

## sharedFileRequests

None. This session touched only `🌍️gis`'s own three files; nothing outside
`✏️s/🔌️plugins/🌍️gis/` was written.
