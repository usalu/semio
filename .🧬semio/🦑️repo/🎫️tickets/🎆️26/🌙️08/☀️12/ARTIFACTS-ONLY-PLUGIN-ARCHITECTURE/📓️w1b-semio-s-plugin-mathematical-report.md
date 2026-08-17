# W1b — `semio-s-plugin-mathematical` — `register()` → `declaration()` conversion

## Clearance (Step 0)

Read `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md`.
`➗️mathematical` appears in neither the RELEASED tables nor the HELD lists as belonging to *this*
wave's clearance question in a blocking way — it IS listed once, under "RELEASED — Wave C / late
Wave M lanes complete" (SMO's own mutation-facet migration, unrelated to this ticket's
`ArtifactDeclaration` work): `dsl_derive::Mutations → dsl::Mutations` bug fixed; 3 orphan triads
deleted; 6 funnel call sites. That confirms SMO is done with it and it is not another session's
held lane. Per the file's own stated default ("absence from this file means free, not held"),
proceeded.

## Step 1 — `register()` → `declaration()`

Single standard (`🔖️1`), single subset (`✳️any`) — one `register()` function, so one
`declaration()`, matching note's exemplar shape exactly (no multi-declaration fold needed).

File: `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`

- Deleted `pub fn register()` (old :19-27), `register_artifact_schema()` (old :30-32),
  `register_artifact_inference()` (old :37-39), `register_pilot_languages()` (old :42-92) — replaced
  by `pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration` (new :23-30) and a private
  `pilot_languages()` helper (new :33-92, same `OnceLock`-backed `&'static` slice pattern as note's,
  since `dsl::passthrough_hooks` isn't `const fn`).
- `declaration()` body:
  ```rust
  semio_framework_plugin::ArtifactDeclaration::builder("s.mathematical")
      .schema(crate::artifacts::mathematical::schema::mathematical_artifact_schema_descriptor())
      .inferences([crate::artifacts::mathematical::standards::v1::subsets::any::schema::inferences::mathematical_artifact_inference_descriptor()])
      .composers(crate::artifacts::mathematical::standards::v1::engine::io_registry::entries())
      .languages(pilot_languages())
      .document_codec::<crate::apps::mathematical::MathematicalPlayApp>()
      .build()
  ```
- `.composers(...)` points at `standards::v1::engine::io_registry::entries()` (the `pub mod
  io_registry` at the bottom of this same file, :328-395 pre-edit / unchanged), **not** the
  root-artifact-level double-wrapper (`🗿️artifacts/➗️mathematical/🦀️component.rs`'s own `pub mod
  io_registry { entries() -> &'static [&'static ComposerEntry] }`, which wraps the same slice as
  `&&ComposerEntry` for a different consumer). Verified this is the exact same bypass note's own
  declaration takes (`🗒️note` has an identical two-layer `io_registry` shape at its artifact root and
  its engine, and its `declaration()` also points straight at the engine-level `entries()`).
- `kind` string is `"s.mathematical"` — matches the `Dialect.artifact_kind` literal already used
  throughout this file's own `io_registry` module (`MATHEMATICAL_DIALECT`), and is the exact string
  `composer_entry_of::<MathematicalAnyComposer>()`'s `writes` resolves to, so the declaration's
  ownership check (composer must produce-or-consume the declared kind) passes. Non-canonical
  (2-segment, not `s.<plugin>.<artifact>`), same as note's `"s.note"` — only the loose ownership
  layer applies until the kind-string migration (UCAS/SMO territory) lands, exactly as documented on
  `ArtifactDeclaration::kind`'s own doc.
- `register_app_schema()` (app-scope config/presence, not artifact-scope) is untouched and still
  called only from the plugin root's `.setup()` — see Step 2.

Two doc-comment references to the deleted `engine::register` were updated to say `engine::declaration`:
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🦀️component.rs:9`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs:2`

Measured before deleting: `grep -rn "mathematical::engine::register\|register_artifact_schema\|register_artifact_inference\|register_pilot_languages"` across the plugin found exactly one real call site (the plugin root, Step 2) plus these two now-updated doc comments. No other caller existed.

## Step 2 — wire the plugin root

File: `✏️s/🔌️plugins/➗️mathematical/🦀️component.rs`

```rust
Plugin::builder("mathematical")
    .label("Mathematical")
    .version("0.1.0")
    .setup(crate::apps::mathematical::config::schema::register_app_schema)
    .artifact(crate::artifacts::mathematical::engine::declaration())
    .register_document_app::<crate::apps::mathematical::MathematicalPlayApp>(crate::apps::mathematical::create_mathematical_app())
    .build()
```

## `.setup()` status — survives, exactly one call, named

`.setup()` is **not** deleted. It now calls exactly
`crate::apps::mathematical::config::schema::register_app_schema` (defined at
`🎛️apps/➗️mathematical/🎚️config/🧬️schema/🦀️component.rs:22`) — `MathematicalPlayApp`'s own
CONFIG/PRESENCE schema. This is app-scope, not artifact-scope: `register_app_schema_descriptor` is
not in the §6 artifact-scoped registrar set the mechanism report enumerates, and
`ArtifactDeclaration` deliberately has no field for it (same escape hatch note's exemplar uses, same
justification). No other function is left behind in `.setup()` — the old `register()` body's other
four calls (schema, inference, io_registry/composers, pilot languages, document codec) all moved
into `declaration()`.

## Step 3 — plugin root closure

Already closed by an earlier W3 wave on this same ticket (see
`📓️w3-semio-s-plugin-mathematical-report.md` in this folder, already on disk): `🛂️manifest/`,
`🎟️capabilities/`, `🔧️setup/` were deleted there as doc-only unmounted stubs. Re-verified now:

```
$ ls -a "✏️s/🔌️plugins/➗️mathematical/"
AGENTS.md  README.md  🎛️apps  📦️packages  🗿️artifacts  🦀️component.rs
```
Exactly the target shape. No action needed this wave.

## Step 4 — escape hatches and deps

```
$ grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_app_io\|register_os_media_" "✏️s/🔌️plugins/➗️mathematical/"
(no output)
```
Zero matches — no violation of this class. `Cargo.toml`
(`📦️packages/🦀️rust/Cargo.toml`) has no `semio-framework-os` dependency at all (only the unrelated
`semio-framework-os-kernel`, aliased `dsl`/`store`/`protocol`), and
`grep -rn "semio_framework_os::"` in the crate is empty — nothing to purge, nothing was ever there.

## Step 5 — inventory (interior-mutable state, host handles, fs/env/process)

```
$ grep -rn "thread_local!" "✏️s/🔌️plugins/➗️mathematical/"                                → zero matches
$ grep -rn "OnceLock<" "✏️s/🔌️plugins/➗️mathematical/"
  🗿️artifacts/➗️mathematical/🦀️component.rs:170        static ENTRIES: OnceLock<Vec<&'static ComposerEntry>>
  .../⚙️engine/🦀️component.rs:335 (io_registry)         static ENTRIES: OnceLock<Vec<ComposerEntry>>
  .../⚙️engine/🦀️component.rs:35  (pilot_languages, new) static LANGUAGES: OnceLock<Vec<dsl::LanguageSpec>>
$ grep -rn "std::fs::\|std::env::\|std::process::\|Command::new" "✏️s/🔌️plugins/➗️mathematical/"  → zero matches
```
All three `OnceLock`s are memoized **derived data caches** (a `Vec<ComposerEntry>` / `Vec<LanguageSpec>`
built once and leaked to `&'static`), not host/engine handles — no `OnceLock<...Host>` /
`OnceLock<...Engine>` pattern exists in this plugin, so there is nothing in the distinct
host-handle violation class. No `thread_local!`, no filesystem/env/process/`Command::new` usage
anywhere in the plugin outside test code (there is none even inside tests).

## Step 6 — verification

**6.1 — every `#[path]` in `📦️glue.rs` resolves on disk** (script: parse every `#[path = "..."]`,
skip `"."` self-mounts, resolve the rest relative to `glue.rs`'s directory, check `-f`):
```
checked=84 missing=1
MISSING: ../../🎛️apps/➗️mathematical/🎮️commands/📄️document/🦀️component.rs
```
This one miss is **pre-existing, unrelated to this wave**, and independently corroborated three ways:
- It is the exact same miss already reported in this ticket folder's
  `📓️w3-semio-s-plugin-mathematical-report.md` (an earlier wave on this same plugin), which traced
  it to another session's in-flight `🎮️commands/📄️document/ → 📄️artifact/` rename that never
  repointed `glue.rs:416-417`.
- `stat -f '%Sm'` on the real directory: `🎮️commands/📄️artifact/🦀️component.rs` mtime `Aug 12
  15:31:13` — hours before this session started.
- `git log --oneline -3` on `📦️glue.rs` and on `🎮️commands/` show only unrelated auto-commits
  (`🚩️495/493/491` and `🚩️493/480/469`); nothing in this session's diff touches the `commands`
  module block in `glue.rs` (my edits were confined to two `🦀️component.rs` files: the engine's and
  the plugin root's — never `glue.rs`).

Per the plugin-specific dispatch note ("check whether it still dangles and report rather than
fixing another session's rename"), **left untouched**.

**6.2 — every `include_str!`/`include_bytes!` target in the plugin resolves** (script: for every
`.rs` file, resolve each literal path relative to that file's real directory, check `-f`):
```
checked=48 missing=0
```

**6.3 — `cargo metadata --no-deps --format-version 1 >/dev/null && echo OK`:**
```
OK
```

**6.4 — `cargo check -p semio-s-plugin-mathematical --all-targets`** (`RUSTC_WRAPPER=""`,
`CARGO_TARGET_DIR` under this ticket's `🎯️target`), real output, run once:
```
    Checking semio-s-plugin-mathematical v0.1.0 (/Users/ueli/Documents/semio/✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust)
error: couldn't read `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/./././../../🎛️apps/➗️mathematical/🎮️commands/📄️document/🦀️component.rs`: No such file or directory (os error 2)
   --> ✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📦️glue.rs:417:13
    |
417 |             pub mod document;
    |             ^^^^^^^^^^^^^^^^^

error: could not compile `semio-s-plugin-mathematical` (lib) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `semio-s-plugin-mathematical` (lib test) due to 1 previous error
```
Full log: `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/scratch-w1b-mathematical-check-1.txt`
(7587 lines; everything before line 7578 is upstream dependency compilation —
`semio-framework-schema`, `semio-framework-math`, `semio-framework-plugin` (the M1 mechanism itself,
0 errors), `semio-s-plugin-stdio` (693 warnings, 0 errors) — all clean).

**This is the same pre-existing `🎮️commands/📄️document → 📄️artifact` dangling mount as 6.1, not a
new error from this wave's edits.** The compiler never reaches any code this session touched — it
fails while expanding the `apps::mathematical::commands::document` module, which is wired from
`glue.rs:416-417` and was never edited by me (my two edits are in `⚙️engine/🦀️component.rs` and the
plugin-root `🦀️component.rs`, both `#[path]`-mounted correctly, both upstream of and unrelated to the
`commands` module tree). The `📓️w3` report already flagged this exact line as "will break `cargo
check -p semio-s-plugin-mathematical` until whoever owns that rename repoints it" — that prediction
is now the literal error observed. Per the dispatch note, not fixed.

**Consequence for verification confidence:** `declaration()`'s own code was checked for
call-target existence by direct grep/read against real function signatures (§6 mapping below), and
the `#[path]`/`include_str!` structural scripts both pass for everything except the one pre-existing
foreign miss — but the compiler itself never got to type-check `declaration()`, `pilot_languages()`,
or the plugin root's `.artifact(...)` call, because the crate fails to parse its module tree first.
This is a real, honest gap: the M1 mechanism was proven end-to-end on note; this plugin's own
conversion is structurally faithful to that proof but is **not compiler-verified** until the
`commands/document` mount is repointed by whoever owns that rename.

## Exhaustive call-target check (since the compiler couldn't do it)

Every symbol `declaration()` references, confirmed to exist by direct read, not grep-and-hope:
- `crate::artifacts::mathematical::schema::mathematical_artifact_schema_descriptor` — defined at
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs:74`, reachable via the `schema` shim at
  `📦️glue.rs:347-349`.
- `...::inferences::mathematical_artifact_inference_descriptor` — defined at
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs:61`.
- `...::standards::v1::engine::io_registry::entries` — defined in this same `⚙️engine/🦀️component.rs`
  (unchanged by this edit), mounted directly under `standards::v1::engine` at `glue.rs:36-37`.
- `crate::apps::mathematical::MathematicalPlayApp` — defined at
  `🎛️apps/➗️mathematical/🦀️component.rs:60`, `impl ArtifactApp for MathematicalPlayApp` at :62.
- `crate::apps::mathematical::config::schema::register_app_schema` — defined at
  `🎛️apps/➗️mathematical/🎚️config/🧬️schema/🦀️component.rs:22`.

## sharedFileRequests

**`✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📦️glue.rs:416-417`** — dangling `#[path]` mount
(`pub mod document;` pointing at a deleted `🎮️commands/📄️document/` directory; the real directory on
disk is `🎮️commands/📄️artifact/`). Pre-existing, confirmed by two independent sessions
(`📓️w3-semio-s-plugin-mathematical-report.md` and this report) and now confirmed by a real compiler
error, not just a script. Blocks `cargo check -p semio-s-plugin-mathematical` (both `lib` and `lib
test` targets) entirely — nothing past module-tree parsing runs. This is outside my boundary (a
`🎮️commands/**` app-command rename, not the artifact-declaration work this wave owns) and per the
plugin-specific dispatch note I did not fix it. Whoever owns the `document → artifact` rename needs
to change `glue.rs:416` from `#[path = "../../🎛️apps/➗️mathematical/🎮️commands/📄️document/🦀️component.rs"]`
to `.../🎮️commands/📄️artifact/🦀️component.rs` (and `pub mod document;` → `pub mod artifact;` plus any
call sites of `apps::mathematical::commands::document::*`, which I did not audit since I did not
enter that module tree) before this plugin — and this wave's `declaration()` conversion — can be
compiler-verified.

## apa-status: partial

The `register()` → `declaration()` conversion, plugin-root `.artifact()` wiring, and narrowed
`.setup()` are complete and structurally correct (every symbol they reference exists and is
reachable by direct read; both `#[path]` and `include_str!`/`include_bytes!` resolution scripts pass
except for one pre-existing, unrelated, already-reported foreign miss). What is **not** complete:
compiler verification, because the crate cannot build past module-tree parsing until the unrelated
`🎮️commands/📄️document → 📄️artifact` dangling mount (someone else's rename, flagged twice now in
this ticket folder) is repointed. This wave did not touch that file, per its own instruction not to
fix another session's rename.

---

## 10-line summary

Converted `➗️mathematical`'s single `register()` (standard 1, subset any) into
`declaration() -> ArtifactDeclaration` at `⚙️engine/🦀️component.rs` — schema, inferences, composers
(bypassing the artifact-root double-wrapper straight to `standards::v1::engine::io_registry::entries()`,
matching note's exact pattern), languages (new `pilot_languages()` `OnceLock` helper), document codec.
Plugin root now calls `.artifact(engine::declaration())`; `.setup()` survives for exactly
`register_app_schema` (app-scope, no `ArtifactDeclaration` field by design, same as note). Plugin root
was already closed by an earlier W3 wave. Zero escape-hatch calls, zero `semio-framework-os` dependency
— nothing to purge. Step 5 inventory: three `OnceLock`-backed derived-data caches (composer/language
lists), no host/engine handles, no `thread_local!`, no fs/env/process. `#[path]` script: 84 checked, 1
pre-existing missing (`🎮️commands/📄️document`, another session's unrepointed rename, already flagged in
this ticket's `📓️w3` report). `include_str!`/`include_bytes!` script: 48/48 resolve. `cargo metadata`:
OK. `cargo check -p semio-s-plugin-mathematical --all-targets`: **fails** on that same pre-existing
dangling mount before reaching any code this wave touched — real output pasted above, full log in
`scratch-w1b-mathematical-check-1.txt`. `apa-status: partial` — conversion done and hand-verified
symbol-by-symbol, compiler verification blocked by a foreign, already-reported bug, filed under
`## sharedFileRequests`.
