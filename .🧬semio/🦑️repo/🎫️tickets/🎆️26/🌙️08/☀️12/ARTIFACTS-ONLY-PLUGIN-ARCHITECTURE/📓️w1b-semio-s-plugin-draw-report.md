# W1b — `🖍️draw` (`semio-s-plugin-draw`) → `.artifact()` declaration

`apa-status: complete` for this plugin's W1b slice (declaration conversion + root close-out check).
`🖍️draw` was absent from `📓️plugin-release-status.md`'s HELD list → free per that ledger's own
"absence means free" rule. Step 0 clearance confirmed before starting.

## What changed

### 1. `🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`

Region `//#region 🔖️Register` (old :17-103) replaced, mirroring `🗒️note`'s exemplar exactly:

- **New** `pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration` (:17-27):
  ```rust
  semio_framework_plugin::ArtifactDeclaration::builder("s.draw")
      .schema(crate::artifacts::draw::schema::draw_artifact_schema_descriptor())
      .inferences([crate::artifacts::draw::schema::inferences::draw_artifact_inference_descriptor()])
      .composers(io_registry::entries())
      .languages(pilot_languages())
      .document_codec::<crate::apps::draw::DrawPlayApp>()
      .build()
  ```
  `"s.draw"` is the kind string, matching every `Dialect { artifact_kind: "s.draw", .. }` already used
  by the artifact's own composer table (`DRAW_DIALECT`) and its `derived_composition` module in
  `🚪️io/🦀️component.rs` — so `register_all`'s ownership check (writes/reads must touch `"s.draw"`)
  passes for every entry `io_registry::entries()` returns. `.composers(io_registry::entries())` is an
  unqualified reference to the *sibling* `pub mod io_registry` defined lower in this same file (the
  real, 7-entry typed table: `composer_entry_of::<DrawAnyComposer>()` + 6 hand-written export entries
  svg/pdf/png/json/dwg/dxf) — **not** the root `crate::artifacts::draw::io_registry` wrapper (see
  "orphaned module" below).
- **Deleted**: `register()` (the old `.setup()` target — called `io_registry::register()` +
  `register_pilot_languages()` + `register_artifact_schema()` + `register_artifact_inferences()` +
  `apps::draw::config::schema::register_app_schema()` + the document-codec registration, all as
  side effects), `register_artifact_schema()`, `register_artifact_inferences()`. All three confirmed
  zero call sites anywhere else in the repo before deleting (`grep -rn "artifacts::draw::engine::register"` /
  `register_artifact_schema\b"` / `register_artifact_inferences\b"` → only the deleted `register()`
  body itself).
- **Kept, unchanged**: `artifact_schema_registered()` (:33-35) — a query helper, not a registrar; no
  external call sites either, but it reads state rather than mutating a registry, so it's outside this
  wave's scope and left as-is.
- **Renamed + reshaped**: `register_pilot_languages()` (used to call `dsl::register_language(...)` five
  times as side effects) → private `fn pilot_languages() -> &'static [dsl::LanguageSpec]` (:37-92), an
  `OnceLock`-backed `Vec` leaked to `&'static` — identical mechanism to note's own `pilot_languages()`,
  needed because `dsl::passthrough_hooks` isn't `const fn` so the five `LanguageSpec`s can't be a bare
  `const` array. All five language ids (`draw.document`/`draw.op`/`draw.diff`/`draw.pack`/`draw.spr`)
  and their grammar/protocol constants are unchanged from the old `register_pilot_languages()` body —
  only the registration side effect was replaced with data construction.

### 2. `🖍️draw/🦀️component.rs` (plugin root)

```rust
pub fn plugin() -> Plugin {
    Plugin::builder("draw")
        .label("Draw")
        .version("0.1.0")
        .setup(crate::apps::draw::config::schema::register_app_schema)
        .artifact(crate::artifacts::draw::engine::declaration())
        .register_document_app::<crate::apps::draw::DrawPlayApp>(crate::apps::draw::create_draw_app())
        .build()
}
```
`.setup(crate::artifacts::draw::engine::register)` → `.artifact(crate::artifacts::draw::engine::declaration())`.
`.setup()` narrowed to exactly one call: `apps::draw::config::schema::register_app_schema` (registers
`DrawPlayApp`'s config+presence `AppSchemaDescriptor` — `register_app_schema_descriptor`, the one §6
function `ArtifactDeclaration` has no field for by design; see that struct's own doc). Note that
draw's `artifact_kind()` (the `ArtifactKindSpec` for `"2d.drawing"`) was **already** registered
through `apps::draw::create_draw_app()`'s own `.artifact_kind(crate::artifacts::draw::artifact_kind())`
manifest call (`🎛️apps/🖍️draw/🦀️component.rs:212`), not through the plugin root — untouched, out of
scope for this swap.

### 3. Root data file relocated (Step 3)

`🗿️artifacts/🖍️draw/🛂️manifest.json` (14 lines, `layerKinds` reference data — `shape`/`path`/`text`/
`image`/`group`/`boolean`/`trace`) had **zero** `.rs`/`.ts` references anywhere in the repo
(`grep -rln "🛂️manifest.json"` → empty) and sat directly at the artifact-kind root, where every other
converted artifact (`🗒️note`, `💠️lowpoly`) has only `🦀️component.rs`. Moved to
`🗿️artifacts/🖍️draw/📚️examples/🛂️manifest.json` — this exact filename-at-this-exact-relative-path is
already the established pattern (`📏️layout/🗿️artifacts/📏️layout/📚️examples/🛂️manifest.json` is
byte-identically named/located; `📐️cad/🗿️artifacts/📐️cad/📚️examples/🔣️machine.json` is the same
pattern for a different filename). No `.rs`/`.ts` edits needed since nothing referenced it.

## `.setup()` survives — why

Exactly one call, `apps::draw::config::schema::register_app_schema` — registers `s.draw.draw`'s
app-scope config/presence `AppSchemaDescriptor` (`register_app_schema_descriptor`). Confirmed via
`🎛️apps/🖍️draw/🎚️config/🧬️schema/🦀️component.rs:32-50`: it calls `::schema::register_app_schema_descriptor`
with `include_str!`'d config + presence facet leaves — the identical shape to `🗒️note`'s own surviving
`.setup()` call. `ArtifactDeclaration`'s own doc names `register_app_schema_descriptor` as one of
exactly two §6 functions with no field on purpose (app-scope, not artifact-scope). **No other reason**
for `.setup()` to survive was found — searched for `register_mesh_*`/`register_solid_*`/`register_dwg_*`/
`register_app_io`/`register_os_media_*`/`register_linked_flow_extension_installer` anywhere in the
plugin: zero hits.

## Step 4 — escape hatches / deps

- `register_mesh_*`/`register_solid_*`/`register_dwg_*`/`register_app_io`/`register_os_media_*`: **zero
  call sites** anywhere in `✏️s/🔌️plugins/🖍️draw/` (measured, not assumed).
- `semio_framework_os::` (the exact crate `semio-framework-os`, as opposed to the *different* package
  `semio-framework-os-kernel`, which the crate legitimately depends on and uses via `dsl`/`store`/
  `protocol` extern-crate aliases): **zero call sites**. Cargo.toml (`📦️packages/🦀️rust/Cargo.toml`)
  has **no** `semio-framework-os` dependency at all — only `semio-framework-os-kernel` (a distinct
  package) — so there was nothing to purge; the earlier grep hit that flagged this was a substring
  false-positive (`semio-framework-os` inside `semio-framework-os-kernel`), verified with an exact-name
  grep before concluding "nothing to purge."
- `🚪️io composer tree`: draw's own `derived_composition` module (`🚪️io/🦀️component.rs`) and the
  `io_registry::entries()` table this ticket's `declaration()` now feeds from `.composers()` are the
  same table — no duplicate IO registrations found to delete (unlike lowpoly's 7-of-15 finding).

## Step 5 — inventory only (not touched, reported)

- **`thread_local!` inside `handle()`** — `🎛️apps/🖍️draw/🦀️component.rs:164`:
  ```rust
  thread_local! {
      static DRAW_SESSION: std::cell::RefCell<DrawSession> = std::cell::RefCell::new(DrawSession::default());
  }
  ```
  Per-call user-gesture state (`DrawSession` holds gesture machinery for command dispatch), not a
  derived cache and not a host/engine handle — inventory only, per the dispatch's own classification.
- **`🔄️fsm` crate** — `✏️s/🔌️plugins/🖍️draw/🔄️fsm/` (own `Cargo.toml`, confirmed with
  `find 🔄️fsm -name Cargo.toml`) — inventory only, **not moved**, per the plugin-specific note (an
  earlier agent briefly broke cargo machine-wide attempting this).
- **`OnceLock` statics** (three: `LANGUAGES` in the new `pilot_languages()`, `ENTRIES` in the root
  `io_registry` wrapper, `ENTRIES` in the real `standards::v1::engine::io_registry`) — all hold
  `Vec<…>` data slices leaked to `&'static`, the exact same lazy-static-data pattern as `🗒️note`'s own
  `LANGUAGES`/`ENTRIES`. **Not** a host/engine handle (no `OnceLock<SomeEngineHost>` found anywhere in
  the plugin) — inventory only, no distinct violation class triggered.
- `std::fs`/`std::env`/`std::process`/`Command::new` outside `#[cfg(test)]`: **zero hits** anywhere in
  the plugin (measured).

## Step 3 — plugin root close-out

Plugin root (`✏️s/🔌️plugins/🖍️draw/`) now contains exactly: `🦀️component.rs`, `🎛️apps/`, `🗿️artifacts/`,
`📦️packages/`, and the inventory-only `🔄️fsm/` (a separate crate, never moved — see plugin-specific
note; this is the one entry outside the mechanism's "exactly `🎛️apps` + `🗿️artifacts` (+ root
`🦀️component.rs`, docs, `📦️packages`)" shape, and it is **structural, not fixable by this wave** —
moving a `Cargo.toml`-bearing dir is explicitly forbidden). No `🛂️manifest/`/`🎟️capabilities/`/
`🔧️setup/` doc-only dirs exist at the plugin root — nothing to delete there. No `AGENTS.md`/`README.md`
found at the plugin root either (not created — out of scope, not requested).

## Step 6 — verification, real output

**1. `#[path]` resolution** — every non-`.` `#[path = "…"]` in `📦️packages/🦀️rust/📦️glue.rs`, resolved
relative to that file's directory:
```
total path attrs (non-'.'): 90
missing: 0
```

**2. `include_str!`/`include_bytes!` resolution** — every target across all `.rs` files under
`✏️s/🔌️plugins/🖍️draw/`, resolved against the real file (not pattern-substituted):
```
total include_str!/include_bytes! calls: 48
missing: 0
```

**3. `cargo metadata`**:
```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo metadata --no-deps --format-version 1 >/dev/null && echo OK
OK
```

**4. `cargo check -p semio-s-plugin-draw --all-targets`** (`RUSTC_WRAPPER=""` disabled per the hard
rule) — 3 attempts before a clean pass, all documented, none touching draw's own paths:
- Attempt 1: `semio-s-plugin-stdio` failed to compile — a missing file under
  `🗄️stdio/…/📄set-snapshot/↩️inverse/🦀️component.rs` (banned `SetSnapshot` mutation mid-deletion by
  another session). Zero `🖍️draw` paths in the output.
- Attempt 2: `semio-s-plugin-stdio` failed differently — 9 `E0599` errors, all on
  `SemioDrawingMutation` variants (`MoveNode`/`DragNodes`/`Rotate`/… — a different in-flight stdio
  refactor). `grep -c "🖍️draw"` on the full output → 1, and that one hit is only the
  `Checking semio-s-plugin-draw-fsm v0.1.0` line (the inventory-only fsm crate building as a
  dependency), not an error.
- Attempt 3: stdio compiled clean; draw itself reached:
  ```
      Finished `dev` profile [unoptimized] target(s) in 6m 55s
  ```
  **0 errors.** One `dead_code` warning (pre-existing, `DrawEngine.artifact` field, unrelated to this
  change) and one `unused_qualifications` warning on my own new `.composers(crate::artifacts::draw::
  standards::v1::engine::io_registry::entries())` line — fixed immediately (simplified to the
  unqualified sibling-module reference `io_registry::entries()`, since `declaration()` and
  `pub mod io_registry` are siblings in the same file).
- Re-ran after that one-line simplification:
  ```
      Finished `dev` profile [unoptimized] target(s) in 5m 01s
  ```
  **0 errors**, warning count dropped 8→7 (the qualification warning gone; the remaining 7 are all
  pre-existing — unused imports/variables, one dead_code field — confirmed unrelated to this change by
  content, since none mention `declaration`, `pilot_languages`, or `ArtifactDeclaration`).

Full transcripts: `scratch-w1b-draw-cargo-check-1.txt` / `-2.txt` / `-3.txt` / `-final.txt` in this
ticket folder.

## Files touched

- `✏️s/🔌️plugins/🖍️draw/🦀️component.rs` — `.setup(engine::register)` → `.setup(apps::draw::config::
  schema::register_app_schema)` + `.artifact(engine::declaration())`.
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` —
  `register()`/`register_artifact_schema()`/`register_artifact_inferences()` deleted;
  `register_pilot_languages()` → `declaration()` + private `pilot_languages()`.
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🛂️manifest.json` → moved to
  `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/📚️examples/🛂️manifest.json` (new dir created, file relocated,
  no content changed).

Nothing else created or deleted.

## Orphaned module (flagged, not deleted — matches note's own precedent)

`✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🦀️component.rs`'s `pub mod io_registry { entries()/compose()/
register() }` (:397-421) is now unreachable from `declaration()` (which reads the real table directly
from `standards::v1::engine::io_registry::entries()`). `register()`'s only caller was the just-deleted
`engine::register()`; `entries()`/`compose()` have no callers either (`grep -rn
"artifacts::draw::io_registry"` → only this module's own definition). Left in place, not deleted — this
mirrors `🗒️note`'s own W1 report, which found and explicitly left an identically-shaped orphaned
`io_registry` wrapper for exactly the same reason (deleting it is unrelated cleanup outside this
wave's scope). Flagged here for whoever next touches draw.

## sharedFileRequests

None. Nothing outside `✏️s/🔌️plugins/🖍️draw/` was edited. The two stdio failures hit during
verification (banned-mutation deletion, `SemioDrawingMutation` variant rename) were transient
concurrent-session churn in `🗄️stdio` — not touched, not mine, resolved itself by the third attempt.

## apa-status

`complete` — `declaration()` built and wired, `.setup()` narrowed to its one justified call, plugin
root inventoried (only the un-movable `🔄️fsm` crate departs from the target shape, and that is
structural per the plugin-specific note, not a defect of this wave), one true data-relocation finding
fixed (`🛂️manifest.json`), `cargo metadata` OK, `#[path]`/`include_str!` exhaustively verified 0
missing, `cargo check -p semio-s-plugin-draw --all-targets` real output pasted at 0 errors.
