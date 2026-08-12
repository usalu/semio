# W1b — `semio-s-plugin-fem` → `.artifact()` declaration conversion

`apa-status: done` — conversion complete, self-consistent (path/include resolution + `cargo metadata`
verified), and compiler-verified: `cargo check -p semio-s-plugin-fem --all-targets` finishes with
**0 errors** (see Verification §4 for the real pasted output and the concurrent-churn story behind why
it took 5 attempts).

## Clearance

Read `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md` (SMO's
live predicate). `🏗️fem` appears in neither RELEASED nor HELD — per that file's own explicit wording
("ABSENCE FROM THIS FILE MEANS FREE, NOT HELD"), it was never claimed by SMO and needed no clearance.
Proceeded.

## What changed

Plugin root was already closed before this session (only `🦀️component.rs`, `AGENTS.md`, `README.md`,
`🎛️apps`, `🗿️artifacts`, `📦️packages` present) and its 8 compute dirs already relocated into the two
artifact engines by an earlier wave — this session was conversion-only, per the dispatch's own note.

1. **`✏️s/🔌️plugins/🏗️fem/🦀️component.rs`** (whole file, 8→17 lines) — `plugin()` now calls
   `.artifact(crate::artifacts::fem2d::engine::declaration())` and
   `.artifact(crate::artifacts::fem3d::engine::declaration())` in place of
   `.setup(crate::register_all_engines)`. `.setup()` is narrowed to
   `crate::register_app_schemas` — the one call fem still needs outside `ArtifactDeclaration` (see
   below).

2. **`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`**
   (`Register` region, :11-98 → :11-93) — `register()` / `register_artifact_schema()` /
   `register_artifact_inferences()` / side-effecting `register_pilot_languages()` replaced by
   `pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration` (:19-28) + a private
   `pilot_languages() -> &'static [dsl::LanguageSpec]` helper (:30-92, `OnceLock`-backed, same shape as
   `🗒️note`'s exemplar — `dsl::passthrough_hooks` isn't `const fn`). `declaration()`:
   ```rust
   ArtifactDeclaration::builder("s.fem2d")
       .schema(crate::artifacts::fem2d::schema::fem2d_artifact_schema_descriptor())
       .inferences([…fem2d_artifact_inference_descriptor()])
       .composers(io_registry::entries())        // this file's own sibling `pub mod io_registry`
       .languages(pilot_languages())
       .document_codec::<crate::apps::fem2d::Fem2dPlayApp>()
       .build()
   ```
   `kind` is `"s.fem2d"`, matching the composer table's own `FEM2D_DIALECT.artifact_kind` exactly
   (verified by reading the `io_registry` module in the same file before writing the call — same
   ownership-check discipline the W1 report demonstrated on note).

3. **`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`**
   (`Register` region, :22-110 → :22-104) — identical treatment: `declaration()` built on
   `"s.fem3d"`, `.composers(io_registry::entries())` (verified against `FEM3D_DIALECT.artifact_kind ==
   "s.fem3d"`), `.document_codec::<crate::apps::fem3d::Fem3dPlayApp>()`, `pilot_languages()` helper
   carrying the 5 fem3d language specs verbatim (fem3d's document/op/diff roles have no protocol, unlike
   fem2d's — preserved exactly, not homogenized).

4. **`✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📦️glue.rs`** (:47-51) — `register_all_engines()` (which
   called each artifact's side-effecting `engine::register()`) renamed and narrowed to
   `register_app_schemas()`, now calling only `crate::apps::fem2d::config::schema::register_app_schema()`
   and `crate::apps::fem3d::config::schema::register_app_schema()` — the two app-scope config/presence
   registrations `ArtifactDeclaration` has no field for.

No files created or deleted.

## Does `.setup()` survive, and why

Yes — narrowed to exactly `crate::register_app_schemas`, which does exactly two calls:
`crate::apps::fem2d::config::schema::register_app_schema()` and
`crate::apps::fem3d::config::schema::register_app_schema()`. Both call
`::schema::register_app_schema_descriptor(...)` — `register_app_schema_descriptor` is the one §6
function the W1 mechanism report documents as deliberately absent from `ArtifactDeclaration` (app-scope
config/presence schema, not an artifact-scope concern). fem has **two** document-owning apps
(`Fem2dPlayApp`, `Fem3dPlayApp`), each with its own app-schema descriptor, so unlike note (one app, one
narrowed `.setup()` call directly to `register_app_schema`) fem needed one small free function
(`register_app_schemas`, in `📦️glue.rs` where `register_all_engines` already lived) fanning out to both
— `.setup(fn())` takes exactly one `fn()`, not a list. This is still "`.setup()` doing exactly one
category of thing," per the dispatch's rule, not a second reason kept alive.

No other reason for `.setup()` was found — grepped for `.setup(` across the whole plugin, one call site.

## Step 5 — inventory

- `thread_local!`: **0 hits** (`grep -rn "thread_local!" ✏️s/🔌️plugins/🏗️fem/`).
- `static … OnceLock<…>` holding a **host/engine handle**: **0 hits**. Six `OnceLock` statics exist,
  all derived-cache shape (`Vec<ComposerEntry>`/`Vec<&'static ComposerEntry>`/`Vec<dsl::LanguageSpec>`
  built once and leaked to `'static`, exactly mirroring note's own `pilot_languages()`/`io_registry`
  convention) — none hold an engine/host type like `OnceLock<BrepEngineHost>`.
- `std::fs::`/`std::env::`/`std::process::`/`Command::new` outside `#[cfg(test)]`: **0 hits**.
- `register_mesh_*`/`register_solid_*`/`register_dwg_*`/`register_app_io`/`register_os_media_*`:
  **0 hits** — nothing to remove (measured, not assumed).
- The two artifact-root `pub mod io_registry { … register() }` functions (`◻2d/🦀️component.rs:270-272`,
  `🧊️3d/🦀️component.rs:320-322`) are now **orphaned** — their only call site was each engine's own old
  `register()` (`crate::artifacts::fem2d::io_registry::register()` /
  `crate::artifacts::fem3d::io_registry::register()`), which no longer exists; `.composers()` in
  `declaration()` now does that registration via `ArtifactDeclaration::register_all`. Left in place
  (unrelated cleanup, out of this wave's scope), flagged here — same disposition note's own orphaned
  `io_registry` module got in the W1 report.
- `semio-framework-os-kernel` (the `dsl`/`store`/`pack`/`protocol`/`vcs` extern-crate alias) stays in
  `Cargo.toml` — `grep -rn "semio_framework_os::"` (the distinct escape-hatch namespace the dispatch asks
  about) is empty, so there is nothing to purge; `semio-framework-os-kernel` is a different, load-bearing
  dependency used throughout the crate, not the one in question.
- `.artifact_kind(crate::artifacts::fem{2,3}d::computation_artifact_kind())` calls
  (`🎛️apps/◻2d/🦀️component.rs:287`, `🎛️apps/🧊️3d/🦀️component.rs:271`) are `AppBuilder::artifact_kind`
  calls on each app's own manifest (registering the `computation.fem{2,3}d` OS-catalog kind for
  `results:out`), unrelated to `ArtifactDeclaration`/`PluginBuilder::artifact_kind` — out of scope, not
  a finding.

## Verification

**1. `#[path]` resolution in `📦️glue.rs`** — scripted (resolve every `#[path = "…"]` file target against
the real filesystem, relative to `📦️glue.rs`'s own directory): **258 file-mount entries, 0 missing.**

**2. `include_str!`/`include_bytes!` resolution** — scripted across every `.rs` file in the plugin
(re-resolved against the real file per the dispatch's instruction, not pattern-substituted): **100
calls, 0 missing.**

**3. `cargo metadata --no-deps --format-version 1`**:
```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo metadata --no-deps --format-version 1 >/dev/null && echo OK
OK
```

**4. `cargo check -p semio-s-plugin-fem --all-targets`** (`RUSTC_WRAPPER=""`, ticket-scoped
`CARGO_TARGET_DIR`) — took 5 attempts, none caused by this diff, each measured before retrying rather
than assumed:
- **Attempt 1**: `Blocking waiting for file lock on build directory` (another session held the shared
  ticket `🎯️target` lock) — left running rather than killed, per this ticket's hard rule.
- **Attempt 2** (after the lock cleared): failed inside `semio-s-plugin-stdio` — `couldn't read
  ✏️s/🔌️plugins/🗄️stdio/…/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs:
  No such file or directory`. Verified concurrent, not mine: `git log`/`stat` on `🗄️stdio`'s
  `📦️glue.rs` showed a commit at 21:24, four minutes after my own fem edits (21:10-21:11) and mid-run;
  `ls` on that mutations dir showed no `set-snapshot`/`snapshot` entry at all — a live in-flight rename
  away from `SetSnapshot` (consistent with this ticket's own hard rule banning that token under `✏️s/`).
  `fem`'s `Cargo.toml` legitimately depends on `semio-s-plugin-stdio` (used throughout its io leaves),
  so this is a real transitive block, not something to route around.
- **Attempt 3**: lock contention again (`ps aux` showed 6+ concurrent `cargo check`/`cargo test`
  processes from other sessions racing the same shared `🎯️target` at the time) — this attempt was
  killed by my own `Monitor` wrapper's 15-minute cap mid-block, an artifact of my tooling choice, not
  a build failure; relaunched directly via `nohup`/`disown` afterward instead of a auto-killing wrapper.
- **Attempt 4**: stdio's missing-file error was gone (the rename landed), but 4 new `E0080` const-eval
  panics appeared, all inside stdio's own
  `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🦀️component.rs:42`
  (`#[derive(Mutations)]: SemioDrawingMutation::{GroupNodes,UngroupNode,FlattenNode,UnflattenNode}'s
  MutationKind::SEMANTICS.kind must equal … (its own kebab form)`) — the repo-wide
  `SemanticMutation`/`MutationKind.semantic_kind` ratchet the W1 mechanism report's own "Notes for
  consumers" section warned would land later and "can force mechanical follow-ups." `stat` on that file
  showed a 21:49:23 mtime, 13 seconds before I checked it — actively being edited live. Zero mentions of
  any `🔌️plugins/🏗️fem` path in this output.
- **Attempt 5** (after a short wait): clean.
```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-s-plugin-fem --all-targets
warning: `semio-s-plugin-fem` (lib) generated 33 warnings (run `cargo fix --lib -p semio-s-plugin-fem` to apply 25 suggestions)
warning: `semio-s-plugin-fem` (lib test) generated 50 warnings (31 duplicates) (run `cargo fix --lib -p semio-s-plugin-fem --tests` to apply 5 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 3m 52s
warning: the following packages contain code that will be rejected by a future version of Rust: semio-s-plugin-fem v0.1.0 (...)
FIFTH-RUN-EXIT: 0
```
`grep -c "^error"` on the full log: **0**. All 33+50 warnings are pre-existing style lints (unused
imports/functions, dead-code-shaped `Fem3dEngine` fields, an `index_of` never called) — none are new
`ArtifactDeclaration`-related warnings beyond the one shared framework-level `child_slots`/`link_slots`
`dead_code` warning already documented in the W1 mechanism report (unexercised until some plugin calls
`.composition()`; fem has no children/links to declare, matching note's own case).

**`cargo test -p semio-s-plugin-fem --lib`** was not run — out of this dispatch's Step 6 scope (which
asks for `cargo check --all-targets` only); flagging so it isn't mistaken for having been verified.

## sharedFileRequests

None. Nothing outside `✏️s/🔌️plugins/🏗️fem/` was touched (glue.rs's `register_all_engines`→
`register_app_schemas` rename is entirely inside the plugin's own package). The stdio churn encountered
during verification (attempts 2 and 4 above) was observed only, never touched — both are SMO/UCAS's
live, in-flight work on `🗄️stdio`, which resolved on its own by attempt 5.

## apa-status

`done` — declaration wiring converted on both artifacts (`fem2d`, `fem3d`), plugin root unchanged in
shape (already closed by an earlier wave), `.setup()` narrowed to exactly one app-scope call
(`register_app_schemas`, justified above), step-5 inventory clean (no `thread_local!`, no host/engine
`OnceLock` handles, no `std::fs`/`env`/`process`/`Command::new` outside tests, no
`register_mesh_*`/`register_solid_*`/`register_dwg_*`/`register_app_io`/`register_os_media_*` calls),
path/include resolution scripted clean (258/258, 100/100), `cargo metadata` clean, and
`cargo check -p semio-s-plugin-fem --all-targets` real, pasted, **0 errors**.
