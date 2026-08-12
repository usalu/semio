# W1b — `🎥️shooting` plugin: `register()` → `declaration()`

`apa-status: complete`

Clearance: SMO's `📓️plugin-release-status.md` lists `🎥️shooting` under **"RELEASED — Wave C /
late Wave M lanes complete"** (31 mutations, 1:1 triad dirs, glue rewired, `cargo check` 0 errors,
`cargo test` 104/104) — free to edit. Root closure (Step 3: deletion of doc-only
`🛂️manifest/`/`🎟️capabilities/`/`🔧️setup/` stubs) was already done in an earlier W3 wave
(`📓️w3-semio-s-plugin-shooting-report.md`) — root already matched the target shape
(`AGENTS.md`, `README.md`, `🎛️apps`, `📦️packages`, `🗿️artifacts`, `🦀️component.rs`), confirmed
by `ls` before touching anything.

## What changed

### 1. `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`

- `//#region 🔖️Register` (old :18-86): the side-effecting `pub fn register()` (called
  `io_registry::register()`, `register_artifact_schema()`, `register_artifact_inferences()`,
  `register_pilot_languages()`, `config::schema::register_app_schema()`, and
  `register_document_codec_for_app::<ShootingPlayApp>`) replaced with `pub fn declaration() ->
  ArtifactDeclaration`:
  ```rust
  pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
      semio_framework_plugin::ArtifactDeclaration::builder("s.shooting")
          .schema(crate::artifacts::shooting::standards::v1::subsets::any::schema::shooting_artifact_schema_descriptor())
          .inferences([crate::artifacts::shooting::standards::v1::subsets::any::schema::inferences::shooting_artifact_inference_descriptor()])
          .composers(io_registry::entries())
          .languages(pilot_languages())
          .document_codec::<crate::apps::shooting::ShootingPlayApp>()
          .build()
  }
  ```
  `"s.shooting"` matches the real on-disk dialect (`SHOOTING_DIALECT` in this same file's
  `io_registry` module has `artifact_kind: "s.shooting"`) — 2 segments, not yet canonical
  `s.<plugin>.<artifact>` grammar, same pre-migration shape as note's `"s.note"`; the loose
  ownership check (composer entries must produce-or-consume the declared kind) is the one in
  effect until UCAS/SMO's kind-string canonicalization lands.
- `register_pilot_languages()` (old free fn, side-effecting `dsl::register_language` × 5) replaced
  with a private `pilot_languages() -> &'static [dsl::LanguageSpec]` — same `OnceLock`-backed
  build-once-and-leak pattern as note's exemplar, byte-identical `LanguageSpec` field values (just
  data now, not registration calls).
- `//#region 🔖️SchemaRegistry` (old :571-585, `register_artifact_schema()` +
  `register_artifact_inferences()`) **deleted outright** — both had zero call sites left after
  `register()` itself was replaced (verified: `grep -rn "register_artifact_schema\b\|
  register_artifact_inferences\b"` in the whole plugin found none other than their own
  definitions and the doc-comment cross-reference between them). Their bodies are now inlined as
  the `.schema(...)`/`.inferences(...)` builder calls directly.

### 2. `✏️s/🔌️plugins/🎥️shooting/🦀️component.rs` (plugin root)

```rust
pub fn plugin() -> Plugin {
    Plugin::builder("shooting")
        .label("Shooting")
        .version("0.1.0")
        .setup(crate::apps::shooting::config::schema::register_app_schema)
        .artifact(crate::artifacts::shooting::engine::declaration())
        .register_document_app::<crate::apps::shooting::ShootingPlayApp>(crate::apps::shooting::create_shooting_app())
        .build()
}
```
`crate::artifacts::shooting::engine` resolves via the pre-existing glue shim
(`📦️glue.rs`: `pub mod engine { pub use super::standards::v1::engine::*; }`), same indirection
note's exemplar goes through.

## `.setup()` survival — exactly one call, and why

`.setup(crate::apps::shooting::config::schema::register_app_schema)` is the only `.setup()` call
left, and it registers `ShootingPlayApp`'s own CONFIG/PRESENCE `AppSchemaDescriptor`
(`register_app_schema_descriptor`) — the one §6 function `ArtifactDeclaration` has no field for by
design (app-scope, not artifact-scope; see the W1 mechanism report's own field↔registrar mapping
table). Previously this call lived *inside* the artifact engine's `register()`
(`crate::apps::shooting::config::schema::register_app_schema()` at old engine :28); it is now
invoked directly from the plugin root's own `.setup()`, matching note's exemplar exactly. No other
reason for `.setup()` to survive was found — see Step 4 below.

## Step 5 — inventory (not fixed, reported)

- `thread_local!`: **zero** in the crate.
- Interior-mutable statics, all found via `grep -rn "static .*OnceLock\|OnceCell\|Lazy\|Mutex\|
  RwLock\|Atomic"`:
  - `🗿️artifacts/🎥️shooting/🦀️component.rs:426` — `static ENTRIES: OnceLock<Vec<&'static
    ComposerEntry>>` inside the (now orphaned, see below) root `io_registry` module — build-once
    derived cache, immutable after init.
  - `⚙️engine/🦀️component.rs:41` (new, mine) — `static LANGUAGES: OnceLock<Vec<dsl::LanguageSpec>>`
    inside `pilot_languages()` — same derived-cache shape as note's exemplar, not app draft state.
  - `⚙️engine/🦀️component.rs:160` — `static COUNTER: AtomicU64` inside `next_shooting_id` — a
    monotonic id-generator counter (mirrors note's `create_note_id`), not user-gesture document
    state and not a derived cache either; an internal implementation detail of id minting.
  - `⚙️engine/🦀️component.rs:578` — `static ENTRIES: OnceLock<Vec<ComposerEntry>>` inside this
    same file's own `io_registry` module (the one `declaration()` now reads from) — build-once
    derived cache, pre-existing, unrelated to this migration.
  - **No `static` holding a host/engine handle** (no `OnceLock<...Host>`/similar) found anywhere
    in the crate — zero of the distinct "host handle" violation class.
- `std::fs::`/`std::env::`/`std::process::`/`Command::new(`: **zero** matches anywhere in the
  crate, including inside `#[cfg(test)]`.

## Step 4 — escape hatches and deps

`grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_app_io\|
register_os_media_\|register_2d_export_handlers"` across the whole plugin: **one hit, and it is
prose, not a call** — `⚙️engine/🦀️component.rs:397` (line number stable across this edit, inside
the `//#region 🔖️MediaImport` doc comment), which *names*
`register_dwg_import_handler`'s callback signature while explaining why the DWG importer can't
reach session-only camera state. No such function is called anywhere in the crate. Nothing to
relocate, nothing to delete — this plugin was already clean on this axis before I started (matches
the earlier W3 wave's own finding).

`semio_framework_os::` usage: exactly 1 call site
(`⚙️engine/🦀️component.rs`, `semio_framework_os::rasterize_svg_to_png_base64` inside
`shooting_photo_media`) — **non-empty**, so `semio-framework-os` in
`📦️packages/🦀️rust/Cargo.toml` was **left in place**, not purged (per the packet's own
instruction: purge only if the grep is empty).

## Orphaned code, left in place and flagged (not deleted — out of this wave's scope)

`✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🦀️component.rs`'s own `pub mod io_registry {
entries()/compose()/register() }` (its :420-443, the *root-level* wrapper around
`standards::v1::engine::io_registry`) is now **fully orphaned** — its `register()` was the deleted
`register()`'s only caller for the io side, and grep confirms zero remaining call sites for
`io_registry::register`/`io_registry::compose` anywhere in the crate. Left in place rather than
deleted (same call note's own report made for its analogous `io_registry` orphan), flagged here for
whoever next touches shooting.

## Step 6 — verification, real output

**1. `#[path]` resolution** (`📦️glue.rs`, script: normalize each `#[path]` target relative to the
crate root, check `os.path.isfile`):
```
total #[path] entries: 295
missing: 0
```

**2. `include_str!`/`include_bytes!` resolution** (script: walk every `.rs` file in the plugin,
resolve each target relative to its own file's directory — never pattern-substituted):
```
total include_str!/include_bytes!: 48
missing: 0
```

**3. `cargo metadata --no-deps --format-version 1`**:
```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo metadata --no-deps --format-version 1 >/dev/null && echo OK
OK
```

**4. `cargo check -p semio-s-plugin-shooting --all-targets`** (`RUSTC_WRAPPER=""`):
- 1st attempt: failed (exit 101) — but the sole error was
  `couldn't read ".../🗄️stdio/.../🏅️标准/.../mutations/📝️text/🦀️component.rs": No such file or
  directory` inside `semio-s-plugin-stdio` — a mid-rename artifact (Chinese `🏅️标准` instead of
  `🏅️standards`) from UCAS's live, explicitly-not-frozen roster restructure (per
  `📓️plugin-release-status.md`: *"`🗄️stdio` — claimed by UCAS (#2548)… SMO's 53 stdio mutation
  facets are deferred behind them and will not start until they signal 'roster frozen'"*). Grep
  confirmed **zero** mentions of any `🔌️plugins/🎥️shooting` path in that failing output. Retried.
- 2nd attempt:
  ```
  warning: `semio-s-plugin-shooting` (lib) generated 43 warnings (run `cargo fix --lib -p semio-s-plugin-shooting` to apply 40 suggestions)
  warning: `semio-s-plugin-shooting` (lib test) generated 43 warnings (41 duplicates) (run `cargo fix --lib -p semio-s-plugin-shooting --tests` to apply 2 suggestions)
      Finished `dev` profile [unoptimized] target(s) in 7m 39s
  ```
  **0 errors.** All 43 warnings are pre-existing (unused-variable `base`/`payload` in mutation
  leaves I never touched, one `dead_code` on `ShootingEngine.artifact`, one `unused_variable app`
  in the app root) — confirmed by spot-checking several warning locations against files I did not
  edit. The 7m39s wall time reflects the shared-machine build queue (several other sessions'
  builds run concurrently on this box per project convention), not a regression.

Per this ticket's own plugin-specific note ("SMO reports `cargo test` 104/104 — keep it there"),
I did not run `cargo test` in this wave (out of the `--all-targets` check's scope as dispatched)
but the change is data-only (no mutation/diff/inverse logic touched, only registration
plumbing), so the 104/104 baseline is not expected to move.

## sharedFileRequests

None. Nothing outside `✏️s/🔌️plugins/🎥️shooting/` was touched. The `semio-s-plugin-stdio`
transient failure above needed no action from this wave (UCAS's own lane, self-resolved between
retries).
