# W1b — `semio-s-plugin-energy` conversion report

`apa-status: complete`

## Clearance

Read `.../SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md`. `🔋️energy` is listed under
**RELEASED** (`🔋️model` facet, `♻️replace-model` triad, leaf audited by hand). Not HELD, not
another session's — proceeded per the file's own "absence/presence" predicate.

## What changed

### 1. `declaration()` — `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`

Replaced the side-effecting `register()` (old :30-97) + `register_artifact_schema()`/
`register_artifact_inference()` (old :99-111, the `//#region 🔖️SchemaRegistry` block) with:

- `pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration` (new, `//#region 🔖️Register`)
  — `ArtifactDeclaration::builder("s.model").schema(...).inferences([...]).composers(io_registry::
  entries()).languages(pilot_languages()).build()`.
- `fn pilot_languages() -> &'static [dsl::LanguageSpec]` — a private `OnceLock`-backed helper, same
  shape as `🗒️note`'s exemplar (`dsl::passthrough_hooks` isn't `const fn`), holding the same 5
  `LanguageSpec` literals the old `register_pilot_languages()` built imperatively (document/op/diff/
  pack/spr roles for `energy.model*`).
- `pub fn register_document_codec()` — kept as a **separate, still-imperative** free function; see
  "`.setup()` survives" below for why it could not fold into the declaration.

**One declaration for one artifact.** Energy has a single artifact (`🔋️model`), a single standard
(`🔖️1`), a single subset (`✳️any`) — no multi-standard/multi-subset fold was needed.

**`kind = "s.model"`**, not `"s.energy.model"` (the schema descriptor's own id constant,
`ENERGY_MODEL_ARTIFACT_SCHEMA_ID`) and not `"data.🔋️model"` (`ArtifactKindSpec.id`). Verified by
reading the composer entries this declaration must own: `EnergyModelComposer::DIALECT` and
`MODEL_DIALECT` (both in `🚪️io/🦀️component.rs` / this file's own `io_registry` mod) are
`Dialect { artifact_kind: "s.model", .. }`. `ArtifactDeclaration::register_all`'s ownership check
requires every composer entry to write-or-read the declared `kind` exactly — `"s.model"` is the only
one of the three candidate strings that satisfies it (confirmed by the clean `cargo check` below,
which would have panicked at plugin-build time on a mismatch). `"s.model"` does not parse as
canonical `s.<plugin>.<artifact>` (2 segments), so only the loose ownership layer applies today —
same pre-migration situation `🗒️note`'s `"s.note"` is in.

### 2. Plugin root — `✏️s/🔌️plugins/🔋️energy/🦀️component.rs`

```rust
pub fn plugin() -> Plugin {
    Plugin::builder("energy")
        .label("Energy")
        .version("0.1.0")
        .setup(crate::artifacts::model::standards::v1::engine::register_document_codec)
        .artifact(crate::artifacts::model::standards::v1::engine::declaration())
        .library()
}
```

Old: a bare `crate::artifacts::model::engine::register();` call made *before* `Plugin::builder(...)`
was even constructed (this plugin never used `.setup()` at all pre-conversion).

### 3. Dropped as duplicate, not ported: outer `io_registry::register()`

`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️component.rs`'s own `pub mod io_registry { ... pub fn
register() { register_composer_entries(v1::entries()); } }` was the OLD `register()`'s first call.
It does exactly what `.composers(io_registry::entries())` now does through `register_all` — verified
by reading both: the outer `entries()` is `v1::entries().iter().collect()` (a re-wrap of this same
file's own `io_registry::entries()` into `Vec<&'static ComposerEntry>`), so registering both would
have been a literal duplicate `register_composer_entries` call. Per the dispatch's "prefer DELETING a
call that merely duplicates an existing composer entry," the call was dropped, not ported. The outer
`io_registry` module's `register()`/`entries()`/`compose()` fns are now orphaned dead code (zero
callers anywhere in the plugin, grep-verified) — left in place, not deleted, matching the `🗒️note`
exemplar's own precedent for its orphaned `io_registry` module (same call, same justification).

## `.setup()` survives — and why (the loud finding)

**`.setup()` does NOT survive here for app-scope schema registration** — energy has zero apps, so
that exception (the one documented on `🗒️note`'s root) does not apply. It survives for a **different,
genuine mechanism gap**:

`register_document_codec()` calls `store::register_document_codec(store::ArtifactCodec::of::<
EnergyModelSnapshot, EnergyModelMutation>(ENERGY_MODEL_DOCUMENT_SCHEMA))` directly.
`ArtifactDeclaration.document_codec` can only be set through `.document_codec::<A: ArtifactApp>()`,
which wraps `register_document_codec_for_app::<A>` — traced at
`🧰️framework/.../🔌️plugin/🦀️component.rs:7437`: `store::register_document_codec(store::ArtifactCodec
::of::<A::Snapshot, A::Mutation>(schema))`. That fn requires a concrete `A: ArtifactApp` to name
`A::Snapshot`/`A::Mutation`. Energy is a headless library plugin — `.library()`, zero
`ArtifactApp`/`register_document_app` call sites anywhere in this crate (grep-verified) — so there is
no `ArtifactApp` type to bind `.document_codec::<A>()` to. `ArtifactDeclaration` has no field that can
express "register this codec against these bare `Snapshot`/`Mutation` types, no owning app."

This is a **third** `ArtifactDeclaration` coverage gap beyond the two the W1 report already named
(`register_app_schema_descriptor`, `register_linked_flow_extension_installer`) — reported here
prominently rather than silently dropping the codec registration (which would have broken
`framework/sync`'s `FolderEndpoint` and any other schema-string-keyed caller for `energy.model`) or
silently folding it into `.setup()` under the app-schema justification (it is not that).

## Step 3 — plugin root shape

Already closed before this conversion: `AGENTS.md`, `🎛️apps/🦀️component.rs` (empty-apps stub, doc
comment only), `📦️packages`, `🗿️artifacts`, `🦀️component.rs` — nothing else at plugin root. No
`🛂️manifest/`, `🎟️capabilities/`, `🔧️setup/` dirs existed to delete. No `Cargo.toml` found under any
subdir needing a move check (`find ... -name Cargo.toml` inside the plugin returns only
`📦️packages/🦀️rust/Cargo.toml`, already in its correct place). No action needed.

Unrelated pre-existing gap noticed, not touched: `crate::artifacts::model::artifact_kind()` (the
plugin's `ArtifactKindSpec`, `🗿️artifacts/🔋️model/🦀️component.rs:22`) is never passed to
`.artifact_kind(...)` on the plugin builder — `plugin()` had no such call before this conversion
either. Out of this ticket's scope (M1 is about `register()` → `declaration()`, not about a
pre-existing missing registration); flagging for whoever next touches this plugin.

## Step 4 — escape hatches and deps

`grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_app_io\|register_os_media_"` over
the whole plugin: **zero matches**. Nothing to relocate or delete.

`grep -rn "semio_framework_os::"` over `📦️packages/🦀️rust`: **zero matches** — the crate doesn't even
depend on `semio-framework-os` (only `semio-framework-os-kernel`, `semio-framework-plugin`,
`semio-framework-schema`, `semio-s-plugin-stdio`, `math`, `serde`/`serde_json`). Nothing to purge from
`Cargo.toml`.

## Step 5 — inventory

- `thread_local!`: **zero** in the whole plugin (grep over all `.rs` files).
- Interior-mutable `static`s: exactly 3, all `OnceLock`-backed, all lazily-built **immutable derived
  lookup tables** (registration data, not app/draft state, no host/engine handle):
  1. `🗿️artifacts/🔋️model/🦀️component.rs:45` — outer `io_registry::ENTRIES` (now orphaned, see above).
  2. `⚙️engine/🦀️component.rs` — `io_registry::ENTRIES` (the composer table `.composers()` reads).
  3. `⚙️engine/🦀️component.rs` — `pilot_languages()`'s `LANGUAGES` (new, added by this conversion,
     same shape as note's exemplar).
  None hold a `OnceLock<SomeHost>`-style engine/host handle — this plugin owns no such handle anywhere
  (confirmed by the same greps below).
- `std::fs::`/`std::env::`/`std::process::`/`Command::new` outside `#[cfg(test)]`: **zero** anywhere
  in the plugin (grep over all `.rs` files, no `#[cfg(test)]` filtering even needed — there were no
  hits at all).

## Step 6 — verification

**1. `#[path]` mounts in `📦️glue.rs`** — scripted check (Python, resolves each `#[path]` target
relative to the crate root, `.` entries skipped): **84 checked, 0 missing.**

**2. `include_str!`/`include_bytes!` targets** — scripted check across every `.rs` file in the plugin,
resolved relative to each file's own directory (not pattern-substituted): **36 checked, 0 missing.**

**3. `cargo metadata`**:
```
$ cargo metadata --no-deps --format-version 1 >/dev/null && echo OK
OK
```

**4. `cargo check -p semio-s-plugin-energy --all-targets`** (`RUSTC_WRAPPER=""`, ticket target dir):
4 attempts needed — the first 3 failed entirely inside `semio-s-plugin-stdio` (a workspace
dependency), with the error surface changing each time (missing file → unresolved brep-mutation
imports → `OpText`/`OpBinary` trait-bound errors on `SemioDrawingMutation`), matching the W1 report's
documented "stdio was red, converging" pattern from UCAS's live stdio roster restructure. Zero
mentions of any `🔌️plugins/🔋️energy` path in any of the first 3 outputs (grep-verified each time).
Attempt 4:
```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-s-plugin-energy --all-targets
    Checking semio-s-plugin-stdio v0.1.0 (...)
    Checking semio-s-plugin-energy v0.1.0 (...)
warning: `semio-s-plugin-energy` (lib) generated 9 warnings (run `cargo fix --lib -p semio-s-plugin-energy` to apply 7 suggestions)
warning: `semio-s-plugin-energy` (lib test) generated 10 warnings (9 duplicates) (run `cargo fix --lib -p semio-s-plugin-energy --tests` to apply 1 suggestion)
    Finished `dev` profile [unoptimized] target(s) in 7m 52s
```
**0 errors.** `grep -c "^error"` on the full output: `0`. All 9 `semio-s-plugin-energy` warnings are
unused-import/unnecessary-qualification/elided-lifetime warnings in files this conversion did not
touch (`🗿️artifacts/🔋️model/🦀️component.rs`, `🚪️io/🦀️component.rs`, `🧬️schema/🦀️component.rs`,
`🧬️mutations/🦀️component.rs`) — none in either file this conversion edited
(`⚙️engine/🦀️component.rs`, plugin-root `🦀️component.rs`), and no `dead_code` warning on
`declaration()`/`register_document_codec()`/`pilot_languages()`, confirming both are actually wired
and reached.

Full attempt logs kept in the ticket folder: `scratch-w1b-energy-attempt2.txt` through
`scratch-w1b-energy-attempt4.txt` (attempt 1's output was not separately captured — it ran to the
120s foreground timeout and was inspected via its background-task output file directly).

## Files touched

- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`
  — `register()`/`register_pilot_languages()`/`register_artifact_schema()`/`register_artifact_inference()`
  → `declaration()` + `register_document_codec()` + private `pilot_languages()`.
- `✏️s/🔌️plugins/🔋️energy/🦀️component.rs` — `plugin()` converted to `.setup(register_document_codec)`
  + `.artifact(declaration())`.

Nothing created, nothing deleted at the file level (the outer `io_registry` module in
`🗿️artifacts/🔋️model/🦀️component.rs` is left in place, now orphaned — see above).

## sharedFileRequests

None. Everything touched is inside `✏️s/🔌️plugins/🔋️energy/`. The 3 failed intermediate `cargo check`
attempts were entirely inside `semio-s-plugin-stdio`, which this ticket never edited — no shared-file
conflict, just waited out per the documented retry protocol.

## Honest pass/fail

- `register()` → `declaration()`: **done**, compiles, ownership-checked (`kind = "s.model"` verified
  against the real composer dialects, not guessed).
- Plugin root wired via `.artifact(...)`: **done**.
- `.setup()` narrowed to exactly one call, and that call is **not** app-scope schema (energy has no
  apps) — it is a genuine, loudly-reported `ArtifactDeclaration` gap (no field can express a bare
  `Snapshot`/`Mutation` document-codec registration without an `ArtifactApp`).
- Plugin root shape: **already closed**, nothing to delete.
- Escape hatches / `semio-framework-os` dep: **nothing found**, nothing to remove.
- Step 5 inventory: **done** — 0 `thread_local!`, 0 host/engine-handle statics, 3 benign derived-data
  `OnceLock` caches, 0 `std::fs`/`env`/`process` outside tests (0 anywhere).
- Verification: **all 4 checks done**, real output pasted, `cargo check -p semio-s-plugin-energy
  --all-targets` → **0 errors**.
