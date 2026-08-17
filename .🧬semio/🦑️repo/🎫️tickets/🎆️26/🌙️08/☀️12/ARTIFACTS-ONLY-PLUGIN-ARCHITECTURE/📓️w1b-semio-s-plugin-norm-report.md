# W1b — `📕️norm` (`semio-s-plugin-norm`) `.setup()` → `.artifact(declaration())` conversion

`apa-status: code-complete, cargo-check-inconclusive` — all 15 artifacts converted, plugin root
closed on the mechanism, glue.rs fan-out deleted, `cargo metadata`/`#[path]`/`include_str!`
verified. `cargo check -p semio-s-plugin-norm --all-targets` (`RUSTC_WRAPPER=""`) was attempted
three times; the shared ticket `CARGO_TARGET_DIR` was under extreme concurrent load the entire
time (up to 50 simultaneous `rustc`/`cargo check`/`cargo nextest` processes system-wide — dozens of
sibling W1b sessions sweeping other plugins at once). Attempts 1 and 2 both got far enough to
compile `semio-framework-plugin` (0 errors) and reach `semio-s-plugin-stdio` before failing —
**zero mentions of any `🔌️plugins/📕️norm` path in either failure** (grep-confirmed) — both failures
are inside stdio's own in-flight roster restructure (UCAS #2548, explicitly "not frozen" per
`📓️plugin-release-status.md`'s own "NOT SMO'S TO RELEASE" section). Attempt 3 was still blocked on
`Blocking waiting for file lock on build directory` after 6+ minutes with no output at all — never
reached rustc — when this report was finalized. See "Verification" for the real pasted output of
all three attempts.

## Clearance

`📓️plugin-release-status.md` (SMO ledger) lists `📕️norm` under **RELEASED — Wave C / late Wave M
lanes complete**: "all 15 — 392 triads — 5 facets migrated from scratch + 10 finished". That entry
is about SMO's mutation-triad migration, a different ticket than this one (APA). This wave's own
dispatch note says the same thing explicitly: "SMO reports norm fully migrated with 392 triads — do
not touch any of it." I did not touch any `🧬️mutations/**` content, any `SetSnapshot`/
`NoMutation`/`CollectionMutation` symbol, or anything under `✏️s/` outside `📕️norm`.

## What changed

### Per-artifact: `register()` → `declaration()` (all 15)

Every one of the 15 `⚙️engine/🦀️component.rs` files followed an identical shape (confirmed by
reading `📕️din4108` in full then diffing structure against all 14 siblings):

```
pub fn register_pilot_languages() { register_artifact_schema(); dsl::register_language(...) × 5; }
pub fn register() { register_pilot_languages(); register_artifact_inferences(); }
pub fn register_artifact_schema() { ::schema::register_artifact_schema_descriptor(...); }
pub fn register_artifact_inferences() { ::schema::register_artifact_inference_descriptor(...); }
pub fn register_io() { crate::artifacts::<x>::io_registry::register(); }  // never called — dead
pub mod io_registry { … entries() -> &'static [ComposerEntry] … }         // kept, now the .composers() source
```

All five became a single `pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration` plus
a private `pilot_languages()` data helper, exactly mirroring `🗒️note`'s exemplar (W1 report):

```rust
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.<variant>")
        .schema(crate::artifacts::<variant>::schema::<variant>_artifact_schema_descriptor())
        .inferences([crate::artifacts::<variant>::standards::v1::subsets::any::schema::inferences::<variant>_artifact_inference_descriptor()])
        .composers(crate::artifacts::<variant>::standards::v1::engine::io_registry::entries())
        .languages(pilot_languages())
        .build()
}

fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES.get_or_init(|| vec![ /* the same 5 LanguageSpec literals, register_language(...) stripped */ ]).as_slice()
}
```

**`kind` string choice** — used each artifact's own composer `Dialect.artifact_kind` (`"s.din4108"`,
`"s.en1990"`, … `"s.vdi3805"` — one segment per variant, confirmed by reading every one of the 15
`🚪️io/🦀️component.rs` `const DIALECT` definitions), **not** the schema descriptor's own `id` field
(`"s.norm.din4108"`, three segments) and **not** the media-IO `ArtifactKindSpec.id`
(`"computation.norm.din4108"`). This is the same choice note made (`"s.note"`, its `Dialect`'s
kind, not `"s.note.note"`, its schema descriptor id) — the declaration's ownership check walks
composer `writes`/`reads` against `kind`, so `kind` must equal the string the composer actually
uses. Verified per-family: every composer's `WRITES`/`reads()` is `DIALECT` alone (`s.<variant>`
only), so `writes_it == true` holds for all 15. None of the 15 has a `s.<variant>` that parses as
canonical `s.<plugin>.<artifact>` (single segment after `s.`), so the strict plugin-ownership layer
stays dormant for all 15 — same pre-migration state note documented for itself.

Files touched (15, one per family):
`🗿️artifacts/{📕️din4108,📗️din16798,📙️din18599,📘️en1990,📘️en1991,📘️en1992,📘️en1993,📘️en1994,📘️en1995,📘️en1996,📘️en1997,📘️en1998,📘️en1999,📓️iso16757,📔️vdi3805}/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`

`register_io()` was dropped entirely rather than ported: it had **zero call sites** repo-wide even
before this change (grep-confirmed on the pre-edit tree) — `register_norm_exports` never called it,
only `register()` (schema+inferences+languages) was ever reached. `.composers()` in `declaration()`
now sources composer entries directly from the same `io_registry::entries()` the dead `register_io`
pointed at, so nothing is lost — the dead indirection is what's gone.

**One pre-existing bug carried forward, not fixed**: `📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`'s
five `LanguageSpec`s (`grammar`/`grammar_path`/`protocol`/`protocol_path`) point at
`crate::artifacts::en1999::{dsl,op,diff,snapshot::pack,spr}::*`, not `crate::artifacts::iso16757::*`
— confirmed present verbatim in the pre-edit file (not introduced by this edit; `iso16757`'s own
`id`/`extension`/`hooks` strings are correct, only the grammar/protocol source module is wrong).
This compiles (both are sibling artifacts in the same crate) so it did not show up as an error in
any check to date; it is a real latent copy-paste bug in iso16757's DSL wiring, out of this wave's
scope to fix (register→declaration is a data-shape move, not a correctness pass over grammar
wiring). Flagged here for whoever next touches iso16757's DSL.

**Cosmetic note**: `iso16757` and `vdi3805` wrote their original `dsl::register_language(...)` calls
as `crate::dsl::register_language(crate::dsl::LanguageSpec { … })` (fully `crate::`-qualified)
where the other 13 used bare `dsl::…` — both resolve to the same external `dsl` crate (Rust 2018+
puts extern crates in the crate-root namespace, so `crate::dsl` and `dsl` are the same path here).
Preserved as-is in the converted `pilot_languages()` bodies; not a defect, not touched further.

### Plugin root — `✏️s/🔌️plugins/📕️norm/🦀️component.rs`

`.setup(crate::register_norm_exports)` → `.setup(crate::config::schema::register_app_schema)` +
15 × `.artifact(crate::artifacts::<variant>::engine::declaration())`. The `engine` path used is the
pre-existing "keep pre-migration module paths resolving for external callers" shim
(`pub mod engine { pub use super::standards::v1::engine::*; }`, present per-artifact in
`📦️glue.rs` already) — same shim note's own conversion called through.

`.setup()` retained for **exactly one** call: `crate::config::schema::register_app_schema`
(`🎚️config/🧬️schema/🦀️component.rs:14`), which calls
`::schema::register_app_schema_descriptor(...)` — the one §6 registrar `ArtifactDeclaration`
deliberately has no field for (app-scope config/presence schema, not an artifact concern; same
carve-out note's own plugin root uses). Confirmed by reading the function body: it registers ONE
`AppSchemaDescriptor` (`id: "s.norm.norm"`) whose `config`/`presence` facets are
`🎚️config/🧬️schema/🦀️component.rs` and `👥️presence/🧬️schema/🦀️component.rs` respectively — a single
descriptor shared by all 15 `PlayApp`s, not per-app data.

### `📦️packages/🦀️rust/📦️glue.rs`

Deleted `register_norm_exports()` (the 16-line `.setup()` fan-out: 1 app-schema call + 15
`engine::register()` calls) — its only call site was the plugin root `.setup(...)`, now replaced.
Nothing else referenced it (grep-confirmed before deleting). No `#[path]` mounts were touched; the
function body was pure Rust, not module wiring.

## `.setup()` status — why it survives, exhaustively

One `.setup()` call remains in the whole plugin: `crate::config::schema::register_app_schema` on
the plugin root. It registers `register_app_schema_descriptor` — outside `ArtifactDeclaration`'s
field set by design (see mapping table in the W1 report). No other `.setup()` call exists anywhere
in `📕️norm` (grep-confirmed: `register_norm_exports` was the only other one, and it is deleted).

## Root-dir question — filed, not guessed

Plugin-specific dispatch note for `📕️norm` names four root dirs (`🎚️config`, `👥️presence`,
`📄️artifact`, `🖥️app-surface`) and instructs: if genuinely shared across ≥2 apps, say so and LEAVE
them rather than guess a split. All four are genuinely shared across all 15 apps, not owned by any
one of them — confirmed by reading each:

- **`🎚️config`/`👥️presence`** — `register_app_schema()` registers ONE `AppSchemaDescriptor`
  (`"s.norm.norm"`) whose `config` facet is `🎚️config/🧬️schema/🦀️component.rs` and whose `presence`
  facet is `👥️presence/🧬️schema/🦀️component.rs` (`include_str!("../../👥️presence/🧬️schema/🦀️component.rs")`
  — a direct cross-directory reference, i.e. these two dirs are already one registration unit, not
  two independent per-app facets). `NormConfig`'s own doc comment states this explicitly: "ONE type
  reused by every app rather than fifteen byte-identical copies … lives in `🫀️core` … the shallowest
  taxonomy node common to every consumer."
- **`📄️artifact`** (`document.rs`: `Quantity`, `ClauseId`, `CheckResult`, `CheckReport`,
  `NormFamily`, `NormHost`, `SetArtifactMutation<D>`, the shared `OpText`/`OpBinary` impls, the
  `NormArtifactRecord`/`impl_norm_artifact_record!` codec consolidation) — used by all 15
  `⚙️engine` modules (every one imports from `crate::document::*`) and all 15 artifacts' snapshot
  codecs (`impl_norm_artifact_record!` is invoked once per family). Not owned by any single
  artifact.
- **`🖥️app-surface`** (`🫀️core.rs`: `render_report`/`render_document_json`/`norm_io`/
  `artifact_kind_spec`/`export_media`/`import_media`/`commit_snapshot_fields`/…) — its own header
  doc comment states the reasoning verbatim: *"the shallowest common ancestor of fifteen sibling
  apps is the plugin's own `🫀️core`"*. Used by all 15 apps' `🦀️component.rs` (`crate::app_surface::*`
  imports throughout).

Forcing any of these four into one artifact's or one app's engine would misattribute shared
infrastructure to a single owner — exactly the "wrong split across 15 apps is expensive to undo"
risk the dispatch note warns against. **Filed, left in place**, matching the note's own instruction.
This means `📕️norm`'s plugin root does not yet meet the literal "only 🦀️component.rs, AGENTS.md,
README.md, 🎛️apps, 🗿️artifacts, 📦️packages" closure bar from the general W3 Step 3 instructions —
a deliberate, documented exception for this one plugin, not an oversight.

## Escape hatches / deps (Step 4)

- `grep -rn "semio_framework_os::"` → **0 hits** anywhere in `📕️norm`. Nothing to remove.
- `grep -rn "register_composer_entries\|register_artifact_schema_descriptor\|register_artifact_inference_descriptor\|dsl::register_language\|store::register_dialect_migration\|register_document_codec"`
  outside `⚙️engine/🦀️component.rs` files → **0 hits** (all such calls now live only inside the 15
  `declaration()`/`ArtifactDeclaration::register_all` paths, none hand-called elsewhere).
- Cargo.toml: the crate depends on `semio-framework-os-kernel` (not `semio-framework-os` — different
  package), and `grep -rn "semio_framework_os_kernel::"` in `📕️norm` → **0 hits** too, so this
  dependency is unused. Left untouched — outside this task's stated purge condition (which names
  `semio_framework_os::` specifically) and a Cargo.toml edit on a shared, concurrently-checked crate
  is not something to do speculatively; flagged for whoever next audits norm's Cargo.toml.

## Inventory (Step 5, report-only)

- `thread_local!` — **0** in `📕️norm`.
- `std::fs::`/`std::env::`/`std::process::`/`Command::new` — **0** in `📕️norm`.
- Interior-mutable `static`s — all are `OnceLock<Vec<...>>` memoizing pure derived data (`ENTRIES`
  for composer tables, `LANGUAGES` for grammar/protocol specs), never a host/engine handle and never
  user-gesture state. Two per family (one in each root `🦀️component.rs`'s `io_registry` shim, one
  now in each `⚙️engine`'s `pilot_languages()`) plus a matching `ENTRIES` inside each `⚙️engine`'s own
  `io_registry` submodule — 45 total, all the same shape as note's own `io_registry::ENTRIES`.

## Orphaned code left in place (not this wave's scope)

Each artifact's root `🦀️component.rs` still has its own `pub mod io_registry { entries() / compose()
/ register() }`, wrapping the `⚙️engine`'s `io_registry::entries()` a second time. Its `register()`
(→ `register_composer_entries(v1::entries())`) was the ONLY caller of the now-deleted `register_io()`
— with `register_io()` gone, this root-level `io_registry::register()` has **zero call sites**
repo-wide (grep-confirmed), matching exactly the orphaned-`io_registry` pattern the W1 report
flagged for note itself ("left in place rather than deleted, since removing it is unrelated cleanup
outside this wave's scope"). Same treatment here, for the same reason, across all 15 artifacts.

## Verification

1. **`#[path]` resolution** — 2321 `#[path = "…"]` attributes in `📦️glue.rs`; 1639 non-`"."` entries
   resolved against the real filesystem (Python, no pattern substitution): **0 missing**.
2. **`include_str!`/`include_bytes!` resolution** — scanned every `.rs` file under `📕️norm`: 580
   calls, each resolved relative to its own file's directory against the real filesystem: **0
   missing**.
3. **`cargo metadata --no-deps --format-version 1`**:
   ```
   $ RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo metadata --no-deps --format-version 1 >/dev/null && echo OK
   OK
   ```
4. **`cargo check -p semio-s-plugin-norm --all-targets`** (`RUSTC_WRAPPER=""`, three attempts, real
   pasted output — never worked around the shared lock, never killed a build):

   **Attempt 1** — reached `semio-s-plugin-stdio` after `semio-framework-plugin` compiled clean (0
   errors, only pre-existing warnings, including the expected `dead_code` on `ArtifactDeclaration`'s
   unused `child_slots`/`link_slots` — same warning note's own W1 report documented):
   ```
   error: couldn't read `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/./…/../../🗿️artifacts/🧿️semio/
   🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs`:
   No such file or directory (os error 2)
     --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs:5572:37
   error: could not compile `semio-s-plugin-stdio` (lib) due to 1 previous error
   ```
   **Attempt 2** — different stdio failure (mid-rename `SemioDrawingMutation` missing variants —
   matches the "D2" `SemioMutation`/`SemioSubsetSnapshot` mid-rename defect note's own W1 report
   independently observed):
   ```
   error[E0599]: no variant, associated function, or constant named `MoveNode` found for enum
   `drawing::schema::mutations::component::SemioDrawingMutation` in the current scope
   error[E0599]: … `DragNodes` … `Rotate` … `Scale` … `ReorderNodes` … `DeleteLayer` … `CreateLayer`
   … `DeleteNode` … `CreateNode` … (9 errors total, all the same enum, all inside stdio)
   error: could not compile `semio-s-plugin-stdio` (lib) due to 9 previous errors; 606 warnings emitted
   ```
   `grep -c "🔌️plugins/📕️norm"` on both attempts' full output → **0** in each — confirmed neither
   failure touches this plugin's own files.

   **Attempt 3** — never got past acquiring the target-dir lock: `Blocking waiting for file lock on
   build directory`, no other output, still running after 6+ minutes (system-wide `ps aux` showed up
   to 50 concurrent `rustc`/`cargo check`/`cargo nextest` processes at points during this session —
   this is the "Concurrent Cargo Workspace Churn" precedent at a larger scale than previously
   documented). Left running in the background rather than killed; this report does not claim a
   passing result it did not observe.

   **Net assessment**: every attempt that reached the compiler compiled `semio-s-plugin-norm`'s own
   two direct upstream deps (`semio-framework-plugin`, `semio-framework-os-kernel`) cleanly and
   never surfaced a single error inside `📕️norm`'s own tree; the only failures seen are unrelated,
   independently-documented concurrent churn in `semio-s-plugin-stdio` (not this plugin's
   dependency graph reaching a real conclusion, but not this plugin's fault either). **This is not
   a substitute for a green `cargo check -p semio-s-plugin-norm --all-targets` — that result was not
   obtained** despite three real attempts; whoever next has a quiet window on this ticket's target
   dir should re-run it once (`RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p
   semio-s-plugin-norm --all-targets`) and confirm.

## sharedFileRequests

None. Every file touched is inside `📕️norm`'s own directory tree; the plugin's `Plugin::builder`
call and its 15 artifact engines are the only files this wave edits.
