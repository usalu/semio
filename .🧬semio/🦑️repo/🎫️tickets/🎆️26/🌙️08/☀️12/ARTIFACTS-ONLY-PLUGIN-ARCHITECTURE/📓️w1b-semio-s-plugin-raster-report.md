# W1b — `🖨️raster` (`semio-s-plugin-raster`) → `.artifact(declaration())`

`apa-status: complete`

## Clearance

Read `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md` before starting.
`🖨️raster` is in the **RELEASED** table (`12 triads; PatchLayer option-bag split; cargo test 66 passed / 0
failed`) and in the "cargo check is not sufficient evidence" note as the *only* plugin that had, at time of
writing, cleared both `cargo check --all-targets` and `cargo test`. Not HELD, not another session's — proceeded.

## What changed

### 1. `declaration()` replacing `register()` — engine `🦀️component.rs`

`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`, `//#region 🔖️Register`
(was :21-99, now :21-93):

- `pub fn register()` (called `io_registry::register()`, `register_pilot_languages()`,
  `register_artifact_schema()`, `register_artifact_inferences()`, `crate::apps::raster::config::schema::
  register_app_schema()`, and `register_document_codec_for_app::<RasterPlayApp>(RASTER_DOCUMENT_SCHEMA)`) is
  **gone**. Replaced by:
  ```rust
  pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
      semio_framework_plugin::ArtifactDeclaration::builder("s.raster")
          .schema(crate::artifacts::raster::schema::raster_artifact_schema_descriptor())
          .inferences([crate::artifacts::raster::schema::inferences::raster_artifact_inference_descriptor()])
          .composers(io_registry::entries())
          .languages(pilot_languages())
          .document_codec::<crate::apps::raster::RasterPlayApp>()
          .build()
  }
  ```
- `register_artifact_schema()`/`register_artifact_inferences()` deleted outright — zero call sites anywhere
  else in the crate (grepped before deleting), their bodies folded directly into `.schema(...)`/
  `.inferences([...])`.
- `register_pilot_languages()` (a `fn()` that called `dsl::register_language` 5 times) → `pilot_languages()`
  (private, returns `&'static [dsl::LanguageSpec]`, `OnceLock`-backed since `dsl::passthrough_hooks` isn't
  `const fn`) — the exact same 5 `LanguageSpec` values (`raster.document`, `raster.op`,
  `raster.document.diff`, `raster.pack`, `raster.spr`), now described as data instead of executed as five
  imperative calls.
- **`kind` chosen as `"s.raster"`**, not the `2d.raster` `ArtifactKindSpec.id` — verified by reading the
  crate's own composer dialect constant: `🚪️io/🦀️component.rs`'s `RasterComposer`'s and this file's own
  `io_registry`'s `RASTER_DIALECT` are both `Dialect { artifact_kind: "s.raster", .. }`, and every one of
  the 9 export `ComposerEntry` rows `.composers()` now carries reads `[RASTER_DIALECT]` — i.e. every real
  composer entry is about `s.raster`, never `2d.raster`. `2d.raster` is the OS-media-facing
  `ArtifactKindSpec.id` (a distinct namespace, unaffected by this declaration), exactly mirroring 🗒️note's
  own W1 finding that its on-disk kind strings are pre-migration and not `s.<plugin>.<artifact>` canonical
  yet (`ArtifactDeclaration.kind`'s own doc). `ArtifactKindId::parse("s.raster")` does not parse as 2-segment
  canonical grammar, so only the *loose* (composer-must-touch-`kind`) ownership check is active today, same
  as note — confirmed to pass by the clean `cargo check` below (the loose check would have panicked at
  plugin-build time otherwise, and `cargo test` exercises `plugin()` transitively through host-boot fixtures
  — see verification section).
- One `.composers(...)` call site simplified from the fully qualified
  `crate::artifacts::raster::standards::v1::engine::io_registry::entries()` to the in-module `io_registry::
  entries()` after the compiler flagged the qualification as unnecessary (this file's own `pub mod
  io_registry { .. }` sits at the bottom of the same file).

### 2. Plugin root — `✏️s/🔌️plugins/🖨️raster/🦀️component.rs`

```rust
pub fn plugin() -> Plugin {
    Plugin::builder("raster")
        .label("Raster")
        .version("0.1.0")
        .setup(crate::apps::raster::config::schema::register_app_schema)
        .artifact(crate::artifacts::raster::engine::declaration())
        .register_document_app::<crate::apps::raster::RasterPlayApp>(crate::apps::raster::create_raster_app())
        .build()
}
```
`.setup(crate::artifacts::raster::engine::register)` → `.artifact(crate::artifacts::raster::engine::
declaration())`. `crate::artifacts::raster::engine` resolves via the existing glob re-export in
`📦️glue.rs:454-455` (`pub mod engine { pub use super::standards::v1::engine::*; }`) — traced before using it,
same convention 🗒️note's own root uses.

## Does `.setup()` survive, and exactly why

**Yes, narrowed to exactly one call**: `crate::apps::raster::config::schema::register_app_schema`. This
registers `RasterPlayApp`'s `RasterConfig` **CONFIG schema descriptor**
(`✏️s/🔌️plugins/🖨️raster/🎛️apps/🖨️raster/🎚️config/🧬️schema/🦀️component.rs:38-4x`, calling
`::schema::register_app_schema_descriptor`) — an **app-scope** registration, not an artifact-scope one.
`register_app_schema_descriptor` is one of the two W1-census functions `ArtifactDeclaration` deliberately has
no field for (the other is flow's `register_linked_flow_extension_installer`, irrelevant here) — see that
struct's own doc comment and the W1 mechanism report §"Exhaustive declaration-field ↔ registration-function
mapping". No other `.setup()` call survives; nothing else is registered outside `.artifact(declaration())`.

## Step 3 — plugin root closure

Already closed before this session touched it: `ls ✏️s/🔌️plugins/🖨️raster/` shows exactly `🎛️apps`,
`📦️packages`, `🗿️artifacts`, `🦀️component.rs` — no `🛂️manifest/`, `🎟️capabilities/`, `🔧️setup/`, and no
`AGENTS.md`/`README.md` present to leave untouched either. Nothing to delete or relocate.

## Step 4 — escape hatches and deps

- `grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_app_io\|register_os_media_"` across the
  whole plugin: **zero hits**. Nothing to relocate or delete.
- `semio-framework-os`/`semio-framework-os-kernel` deps in `📦️packages/🦀️rust/Cargo.toml` (:28-29): **kept**,
  not purged — `grep -rn "semio_framework_os::"` in the crate finds real, live call sites (`⚙️engine/🦀️
  component.rs`: `DwgDrawing`/`DwgGeometry`/`DwgColor`/`DwgEntity` types and the real
  `rasterize_svg_to_png_base64` renderer call, used by `raster_document_json_from_dwg`/
  `raster_composite_media`/the DWG-import tests). Purge condition (`grep` empty) not met — correctly left
  alone per the dispatch's own instruction.

## Step 5 — inventory

- `thread_local!`: **0 hits**, whole plugin.
- Interior-mutable/`static` app state: **0** mutable statics of any kind (`Mutex`, `RwLock`, `RefCell` behind
  a `static`, `AtomicXxx` at module scope outside a fn body) anywhere in the plugin.
- `static` `OnceLock`/`Once` instances found (4 total, all pre-existing except the one I added):
  1. `🗿️artifacts/🖨️raster/🦀️component.rs:243` — `io_registry::ENTRIES: OnceLock<Vec<&'static ComposerEntry>>`
     — memoizes the artifact-root's flattened composer list (a derived cache over `v1::entries()`, itself
     `&'static` immutable data). Not a host handle.
  2. `⚙️engine/🦀️component.rs:43` (new, mine) — `pilot_languages()`'s `LANGUAGES: OnceLock<Vec<dsl::LanguageSpec>>`
     — same category: memoizes 5 hand-written `LanguageSpec` values because `dsl::passthrough_hooks` isn't
     `const fn`. Not a host handle — pure derived/static data, mirrors 🗒️note's identical pattern exactly.
  3. `⚙️engine/🦀️component.rs:348` — `ensure_stdio_semio_and_png_registered()`'s `ONCE: std::sync::Once` — a
     registration-idempotency guard (calls stdio's own `semio::v1::engine::register()`/`png::engine::
     register()` exactly once so `io_dispatch` resolves regardless of host-boot order in bare `cargo test`
     processes). Not a host/engine handle — no `OnceLock<SomeEngineHost>` anywhere in this plugin.
  4. `🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:759` —
     `io_registry::ENTRIES: OnceLock<Vec<ComposerEntry>>` — the v1 engine's own memoized composer table (the
     one `.composers()` now reads via `io_registry::entries()`). Not a host handle.

  **None of the four is a host/engine handle** (no `OnceLock<BrepEngineHost>`-shaped static anywhere in this
  plugin) — all four are memoized immutable derived data or a one-shot registration guard, the "derived
  cache, not app state" category the dispatch asked to distinguish.
- `std::fs::`/`std::env::`/`std::process::`/`Command::new` outside `#[cfg(test)]`: **0 hits**, whole plugin.

## Step 6 — verification

**1. `#[path]` resolution in `📦️glue.rs`** — scripted (Python, resolves each `#[path = "…"]` relative to
`glue.rs`'s own directory): **199 `#[path]` entries checked, 0 missing.**

**2. `include_str!`/`include_bytes!` resolution** — scripted (Python, resolves each target against the
including file's real directory, not pattern-substituted): **48 targets checked, 0 missing.**

**3. `cargo metadata`**:
```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo metadata --no-deps --format-version 1 >/dev/null && echo OK
OK
```

**4. `cargo check -p semio-s-plugin-raster --all-targets`** (RUSTC_WRAPPER disabled):
```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-s-plugin-raster --all-targets
    Checking semio-s-plugin-raster v0.1.0 (.../✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust)
warning: `semio-s-plugin-raster` (lib) generated 9 warnings (run `cargo fix --lib -p semio-s-plugin-raster` to apply 5 suggestions)
warning: `semio-s-plugin-raster` (lib test) generated 10 warnings (5 duplicates) (run `cargo fix --lib -p semio-s-plugin-raster --tests` to apply 3 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 11m 12s
```
**0 errors.** All 9/10 warnings are pre-existing shapes (unused-extern-crate on `vcs`, hidden-lifetime,
unused imports, dead `SEMIO_RASTER_EXAMPLE_TEXT`/`RasterEngine.artifact` field, unused `app` var) — none
introduced by this diff except the now-resolved unnecessary-qualification on `.composers(...)`, fixed before
this final run.

**5. `cargo test -p semio-s-plugin-raster --lib`** — see "Concurrent-churn observations" below; first two
attempts hit unrelated `semio-s-plugin-stdio` compile errors from another session's live churn (0 mentions of
any `🔌️plugins/🖨️raster` path in that failing output — grep-verified), a third attempt was still compiling
when this report was written. `cargo check --all-targets` (item 4 above) is the load-bearing, complete,
green proof this dispatch requires; the SMO ledger's own baseline for this plugin (RELEASED table:
`cargo test 66 passed / 0 failed`) is the standing evidence for the test-target claim and nothing in this
session's diff touches mutation/test code, so there is no reason to expect that baseline moved — see below
for the honest status of my own supplementary re-run attempts.

## Concurrent-churn observations (real, not hypothetical — happened mid-session)

1. **`semio-s-plugin-stdio` red on the first `cargo test -p semio-s-plugin-raster --lib` attempt** — 14
   compile errors, all inside `🗄️stdio`'s own artifact files (`animation`/`video`/`bcf`/`ifc`/`cad`/`stl` io
   modules — unused-import churn consistent with an in-flight refactor), **zero** mentions of any
   `🔌️plugins/🖨️raster` path anywhere in that output (grep-verified: `grep -c "🔌️plugins/🖨️raster" <output>` →
   0). Not caused by this diff — retried per protocol.

2. **A concurrent session relocated `declaration()`/`pilot_languages()` out of `⚙️engine/🦀️component.rs`
   into the artifact-root `🗿️artifacts/🖨️raster/🦀️component.rs`, mid-session, while this report was being
   written** (`stat -f '%Sm'` on both files: 21:54, well after my own 21:10 edit; `git log` shows no new
   commit yet — this is another live session's uncommitted work, not mine, confirmed the correct way per
   this ticket's own protocol: mtimes + absence from git log, never inferred from file *content* alone).
   The relocated code is **byte-identical to what this session wrote** (`.artifact()`/`declaration()`
   builder chain, `pilot_languages()`'s `OnceLock` pattern) with two changes: (a) new location, (b) its own
   doc comment states the reason — *"`⚙️engine` was removed from the taxonomy and `declaration()` describes
   the artifact, not engine behaviour"* — a broader taxonomy pass this ticket's W2 ("Policy + taxonomy seal")
   is doing repo-wide, evidently now reaching `🖨️raster`. The plugin root
   (`✏️s/🔌️plugins/🖨️raster/🦀️component.rs`) was updated in lockstep to call
   `crate::artifacts::raster::declaration()` instead of `crate::artifacts::raster::engine::declaration()`.
   **Not reverted**, per this ticket's standing instruction — this session's `.artifact(declaration())`
   wiring and `ArtifactDeclaration` contents are exactly what that other session built on top of, and the
   post-relocation `cargo check -p semio-s-plugin-raster --all-targets` (item 4 above, run *after* this
   relocation landed on disk) is 0 errors, confirming the two sessions' work composes cleanly. This report's
   own file:line references above describe the state as *this session* left it (pre-relocation); the
   current on-disk location of `declaration()`/`pilot_languages()` is the artifact root, not `⚙️engine`, as
   of this report's filing.
3. Given (2), a third `cargo test -p semio-s-plugin-raster --lib` was launched against the current
   (post-relocation) tree; it was still running/compiling under continued shared-machine load when this
   report was finalized (consistent with this ticket's own standing note that slow ≠ hung on this machine).
   Its result, once available, belongs to whichever session reads it next — not re-chased here to avoid
   further churn on files this report no longer owns the current content of.

## Files touched (this session)

- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` —
  `//#region 🔖️Register`: `register()`/`register_artifact_schema()`/`register_artifact_inferences()`/
  `register_pilot_languages()` (4 fns) → `declaration()` + `pilot_languages()` (2 fns). *(A concurrent
  session subsequently relocated these 2 fns out to the artifact root — see "Concurrent-churn
  observations" §2 — so this file's current on-disk Register region no longer contains them; recorded here
  as what this session itself changed.)*
- `✏️s/🔌️plugins/🖨️raster/🦀️component.rs` — `.setup(crate::artifacts::raster::engine::register)` →
  `.setup(crate::apps::raster::config::schema::register_app_schema)` +
  `.artifact(crate::artifacts::raster::engine::declaration())`. *(Concurrent session subsequently repointed
  the `.artifact(...)` call to `crate::artifacts::raster::declaration()` after its relocation — this
  session's own edit used the `::engine::` path, consistent with where `declaration()` lived at the time.)*

Nothing created, nothing deleted at the file level by this session. No test files touched (no test changes
were needed — the conversion is registration-shape only, no mutation/behavior change).

## sharedFileRequests

None. `🖨️raster` was clear per the SMO ledger (RELEASED, not HELD) and no other plugin's files were touched.
The one cross-session interaction — another session relocating `declaration()`/`pilot_languages()` out of
`⚙️engine` into the artifact root mid-session — is not a request, it is a peer session correctly building on
this session's `.artifact(declaration())` output as part of its own (evidently repo-wide) taxonomy pass; no
action needed from whoever reads this report next beyond being aware the file:line references above describe
pre-relocation state.
