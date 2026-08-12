# W1b — `semio-s-plugin-lowpoly` `.artifact()` conversion

`apa-status: done` — `.setup(engine::register)` replaced by `.artifact(engine::declaration())`; plugin root closes at exactly `🦀️component.rs` + `AGENTS.md` + `🎛️apps` + `🗿️artifacts` + `📦️packages`; `cargo check -p semio-s-plugin-lowpoly --all-targets` is 0 errors.

## Step 0 — clearance

Read `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md`. `💠️lowpoly` appears under **RELEASED — Wave C / late Wave M lanes complete** ("16 mutations, 1:1 triad dirs, glue rewired, `cargo check` 0 self-owned errors") and nowhere under **HELD**. Clear to proceed.

## What changed

### 1. `register()` → `declaration()` — `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`

Replaced the side-effecting `register()` (called `io_registry::register()`, `register_pilot_languages()`, `register_artifact_schema()`, `register_artifact_inferences()`, `register_app_schema()`, `register_document_codec_for_app`) with `pub fn declaration() -> ArtifactDeclaration`:

```rust
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.lowpoly")
        .schema(crate::artifacts::lowpoly::schema::lowpoly_artifact_schema_descriptor())
        .inferences([crate::artifacts::lowpoly::standards::v1::subsets::any::schema::inferences::lowpoly_artifact_inference_descriptor()])
        .composers(crate::artifacts::lowpoly::standards::v1::engine::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::lowpoly::LowpolyPlayApp>()
        .build()
}
```

- `register_artifact_schema()`, `register_artifact_inferences()`, `register_pilot_languages()` (the free-function side of the old `register()`) were deleted outright — measured zero call sites anywhere else in the repo first (`grep -rln <symbol> --include='*.rs' . | xargs grep -l lowpoly`, every hit was the lowpoly engine file itself; the framework/`gis`/`process3d` hits for the same generic names are those plugins' own independently-named copies, not callers of lowpoly's).
- `register_pilot_languages()`'s five `dsl::register_language(...)` calls became a private `pilot_languages() -> &'static [dsl::LanguageSpec]` (`OnceLock`-backed, leaked slice) — same pattern `🗒️note`'s exemplar uses, for the same reason (`dsl::passthrough_hooks` isn't `const fn`).
- `artifact_schema_registered()` kept as-is (unrelated query helper, doesn't call any register function, zero call sites but a legitimate public API surface — left untouched, same as note left its own unrelated helpers alone).
- Kind string: `"s.lowpoly"` — matches the existing `LOWPOLY_DIALECT`/`EXPORT_*_DIALECT` constants already used throughout `io_registry`, and mirrors note's own pre-migration `"s.note"` two-segment form (see W1 report's `kind: String` rationale).
- `composers`: pointed at the *engine's own* `io_registry::entries() -> &'static [ComposerEntry]` (9 entries: 1 native + 8 export directions to las/ply/png/json/dwg/stl/gltf/obj) — **not** the artifact-root `🗿️artifacts/💠️lowpoly/🦀️component.rs`'s `io_registry` wrapper, which returns `&'static [&'static ComposerEntry]` (a different type, a pre-existing double-indirection wrapper around the same table). That wrapper's `register()` fn is now orphaned (see below), matching note's own "now-orphaned `io_registry`" finding almost exactly.
- No `subset_validators`, `formats`, or `migrations` calls existed anywhere in lowpoly (`grep -rn 'register_subset_validator\|register_format_descriptors\|register_dialect_migration' ✏️s/🔌️plugins/💠️lowpoly/` → empty) — those declaration fields are simply left at their builder defaults (`&[]`/`Vec::new()`), correctly reflecting "this artifact has none," not an omission.
- `.composition::<Snapshot>()` not called — `LowpolySnapshot` has no children/links to declare (same as note).

### 2. Plugin root — `✏️s/🔌️plugins/💠️lowpoly/🦀️component.rs`

```rust
Plugin::builder("lowpoly")
    .label("Lowpoly")
    .version("0.1.0")
    .setup(crate::apps::lowpoly::config::schema::register_app_schema)
    .artifact(crate::artifacts::lowpoly::engine::declaration())
    .register_document_app::<crate::apps::lowpoly::LowpolyPlayApp>(crate::apps::lowpoly::create_lowpoly_app())
    .build()
```

`.setup(crate::artifacts::lowpoly::engine::register)` → `.artifact(crate::artifacts::lowpoly::engine::declaration())`. `.setup()` re-pointed at exactly one call, `crate::apps::lowpoly::config::schema::register_app_schema` (confirmed `pub fn register_app_schema()` at `🎛️apps/💠️lowpoly/🎚️config/🧬️schema/🦀️component.rs:47`), pulled out of the old `register()` body — same treatment note's exemplar gave `NotePlayApp`'s config schema.

## `.setup()` — survives, and why

Kept for exactly one call: `register_app_schema()`. This registers `LowpolyPlayApp`'s CONFIG/PRESENCE schema — an app-scope concern, not one of the 9 artifact-scoped §6 registrars `ArtifactDeclaration` models (see the struct's own doc at `🧰️framework/…/🔌️plugin/🦀️component.rs:936`: `register_app_schema_descriptor` is one of exactly two functions deliberately excluded, called out loudly rather than silently dropped). This is the same, and only, reason `.setup()` survives on `🗒️note` — no other reason, no other call folded in.

## Step 3 — plugin root closure

Directory listing of `✏️s/🔌️plugins/💠️lowpoly/` after conversion: `AGENTS.md`, `🦀️component.rs`, `🎛️apps/`, `📦️packages/`, `🗿️artifacts/`. No `README.md`, no `🛂️manifest/`, `🎟️capabilities/`, or `🔧️setup/` directories exist (confirmed by `find … -iname '*manifest*' -o -iname '*capabilities*' -o -iname '*setup*'` → empty) and none are `#[path]`-mounted in `📦️glue.rs`. The root was already closed to spec before this session touched it — nothing to relocate or delete for Step 3.

## Step 4 — escape hatches and deps

`grep -rn 'register_mesh_\|register_solid_\|register_dwg_\|register_app_io\|register_os_media_' ✏️s/🔌️plugins/💠️lowpoly/` → **zero matches**. The dispatch's own plugin-specific note already says this facet was deleted before this session started; measured, not assumed. `register()`'s own doc comment (now moved onto `declaration()`) already documented that these calls were never carried over because they'd have duplicated the `io_registry`-driven composer registration. `Cargo.toml` depends on `semio-framework-os-kernel`, not `semio-framework-os` — nothing to purge (`grep -rn 'semio_framework_os::' ✏️s/🔌️plugins/💠️lowpoly/` also empty).

## Step 5 — inventory only (not touched)

- **`thread_local!`** — `🎛️apps/💠️lowpoly/🦀️component.rs:48`, `LOWPOLY_SCRATCH: RefCell<LowpolyScratch>`. Its own doc comment: "Mid-gesture scratch survives across `ArtifactApp::handle` calls." This is **user-gesture state**, not a derived cache — correctly out of scope for this wave.
- **`OnceLock`** (derived caches, not host handles) — two: `🗿️artifacts/💠️lowpoly/🦀️component.rs`'s artifact-root `io_registry::ENTRIES` (the now-orphaned wrapper, see below) and the engine's own `io_registry::ENTRIES` (backs `declaration().composers`). A third was added by this conversion: `pilot_languages()`'s `LANGUAGES` OnceLock, same pattern as note's. None hold a host/engine handle (no `OnceLock<...Host>` anywhere in this plugin).
- **`std::env::var`** — one hit, `⚙️engine/🧵️media/🦀️component.rs:123`, `EXPORT_LOWPOLY_FOREST_MESH`, inside `#[test] fn export_concrete_forest_left_lowpoly_mesh_json` under `#[cfg(test)]` — excluded by the dispatch's own carve-out.
- **`std::fs::`/`std::process::`/`Command::new`** — zero hits anywhere in the plugin.

## Newly-orphaned code (flagged, not deleted)

`🗿️artifacts/💠️lowpoly/🦀️component.rs:347-371`'s `pub mod io_registry { entries()/compose()/register() }` (the `Vec<&'static ComposerEntry>`-wrapping duplicate of the engine's own table) has **zero remaining call sites** now that the plugin root's `.setup()` no longer calls the old `register()`, which was its only caller (`crate::artifacts::lowpoly::io_registry::register()` at old `⚙️engine/🦀️component.rs:86`, now deleted). Left in place, exactly as note's W1 report left its own analogous orphan — deleting it is unrelated cleanup outside this wave's scope. Flagged here for whoever next touches lowpoly's IO surface.

## Verification

**1. `#[path]` mount resolution** (scripted, walked every non-`"."` `#[path]` in `📦️glue.rs`, `os.path.isfile` against the real filesystem): **126 mounts, 0 missing.**

**2. `include_str!`/`include_bytes!` resolution** (scripted, every `.rs` file under the plugin, targets re-resolved relative to their own file's directory, never pattern-substituted): **50 targets, 0 missing.**

**3. `cargo metadata`:**
```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo metadata --no-deps --format-version 1 >/dev/null && echo OK
OK
```

**4. `cargo check -p semio-s-plugin-lowpoly --all-targets`** (`RUSTC_WRAPPER=""`): 4 attempts. Attempts 1-3 were red, exclusively inside `semio-s-plugin-stdio` — `E0432`/`E0599`/`E0277` on `SemioDrawingMutation` variants (`DeleteLayer`/`CreateLayer`/`DeleteNode`/`CreateNode`/`MoveNode`/`DragNodes`) and `OpBinary`, converging 7 → 3 → 0 stdio-side errors across retries (SMO's live semantic-mutations rewrite of stdio's drawing subset — confirmed foreign: `git log -3` on that mutation dir shows its last commit predates this session, and the directory that attempt-1's error named, `…mutations/📄set-snapshot/`, no longer exists on disk at all — an in-flight `SetSnapshot` deletion, exactly SMO's banned-token sweep). Zero mentions of any `💠️lowpoly` path in any of the 3 red outputs (grep-verified each time). Attempt 4:
```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-s-plugin-lowpoly --all-targets
warning: `semio-s-plugin-lowpoly` (lib test) generated 22 warnings (18 duplicates) (run `cargo fix --lib -p semio-s-plugin-lowpoly --tests` to apply 4 suggestions)
warning: `semio-s-plugin-lowpoly` (lib) generated 19 warnings (run `cargo fix --lib -p semio-s-plugin-lowpoly` to apply 15 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 7m 45s
```
**0 errors.** All 19+22 lowpoly-side warnings checked individually — every one traced to a file this conversion never touched (`🔺️diff/🦀️component.rs`, `🔺️diff/📝️text/🦀️component.rs`, `🚪️io/🦀️component.rs`, `📦️glue.rs:506`, plus the pre-existing `LowpolyEngine.artifact`/`sync_snapshot_from_artifact` dead-code pair) — pre-existing, none introduced by this diff. Raw outputs saved: `scratch-w1b-lowpoly-cargo-check-{1,2,3,4}.txt` in this ticket folder.

## Files touched

- `✏️s/🔌️plugins/💠️lowpoly/🦀️component.rs` — `.setup(engine::register)` → `.setup(apps::lowpoly::config::schema::register_app_schema)` + `.artifact(engine::declaration())`.
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` — `register()`/`register_artifact_schema()`/`register_artifact_inferences()`/`register_pilot_languages()` → `declaration()` + private `pilot_languages()` helper; `artifact_schema_registered()` untouched.

Nothing created, nothing deleted at the file level.

## sharedFileRequests

None. This wave touched only files inside `✏️s/🔌️plugins/💠️lowpoly/`; the framework `ArtifactDeclaration` mechanism (W1) and stdio (SMO's lane) were both read-only for this session.

## apa-status: done

`.artifact()` conversion complete and compiler-verified at 0 errors on `--all-targets`. Plugin root already met the Step-3 closure bar before this session (no manifest/capabilities/setup dirs to remove). Step-5 inventory recorded above for the census, nothing remediated (out of scope by design). One newly-orphaned dead code path flagged (artifact-root `io_registry` wrapper), not deleted (unrelated cleanup).
