# W1b — `semio-s-plugin-block` (`🧱️block`) — `.artifact()` conversion report

`apa-status: partial` — mechanism applied to 2 of 3 artifacts (block3d, block5d); block2d deliberately
deferred (see "block2d — deferred" below, real evidence, not a guess); crate does **not** compile
clean, but the 14 failures are pre-existing, unrelated, and (for 5 of them) in territory this
ticket's own HARD RULES forbid touching (`🧬️mutations/**`).

## Step 0 — clearance

Read `📓️plugin-release-status.md` (SMO, 26/08/12/SEMANTIC-MUTATIONS-OVERHAUL). `🧱️block` is listed
under **RELEASED — lane finished, compiles in the workspace check** (26/28/35→ wait, actually
26/37/41 mutations across 2d/3d/5d, "triad-dir↔variant counts verified 1:1 on disk"). Not in any
HELD section. Proceeded.

## Step 1/2 — `register()` → `declaration()`, root wired

Three artifacts, three `⚙️engine` modules — but only **two** had one on disk (see block2d below).

### block3d — `🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`
- Replaced `pub fn register()` + `pub fn register_pilot_languages()` (old :12-78) with
  `pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration` (kind `"s.block3d"`,
  `.schema(...)`, `.inferences([...])`, `.composers(...io_registry::entries())`,
  `.languages(pilot_languages())`, `.document_codec::<crate::apps::block3d::Block3dPlayApp>()`) + a
  private `pilot_languages()` `OnceLock`-backed helper — same shape as note's exemplar, same 5
  language specs, byte-identical grammar/protocol wiring.
- Deleted the now-orphaned free functions `register_artifact_schema()`/`register_artifact_inference()`
  (old `//#region 🔖️ArtifactSchemaRegistry`/`🔖️ArtifactInferenceRegistry`, ~14 lines) — `declaration()`
  calls the same two descriptor functions directly; the wrapper fns had zero other call sites
  (grep-verified) once `register()` was gone.

### block5d — `🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`
- Identical treatment: `declaration()` (kind `"s.block5d"`) + `pilot_languages()` helper, same two
  orphaned wrapper fns deleted.

### Root — `🧱️block/🦀️component.rs`
```rust
pub fn plugin() -> Plugin {
    Plugin::builder("block")
        .label("Block")
        .version("0.1.0")
        .setup(crate::register_block_exports)
        .artifact(crate::artifacts::block3d::engine::declaration())
        .artifact(crate::artifacts::block5d::engine::declaration())
        .register_document_app::<crate::apps::block2d::Block2dPlayApp>(crate::apps::block2d::create_block2d_app())
        .register_document_app::<crate::apps::block3d::Block3dPlayApp>(crate::apps::block3d::create_block3d_app())
        .register_document_app::<crate::apps::block5d::Block5dPlayApp>(crate::apps::block5d::create_block5d_app())
        .build()
}
```
`crate::artifacts::block3d::engine`/`block5d::engine` are the pre-existing shim modules
(`pub mod engine { pub use super::standards::v1::engine::*; }` in `📦️glue.rs`) — same indirection
note's exemplar uses, `declaration` reaches through the glob re-export.

### `📦️glue.rs` — `register_block_exports()` narrowed
```rust
pub fn register_block_exports() {
    crate::apps::block2d::register();

    crate::apps::block2d::config::schema::register_app_schema();
    crate::apps::block3d::config::schema::register_app_schema();
    crate::apps::block5d::config::schema::register_app_schema();
}
```
Removed the `block3d::engine::register()`/`block5d::engine::register()` calls (function renamed away
under them); kept `block2d::register()` (see below) and all three `register_app_schema()` calls —
`.setup()` survives narrowed to exactly the app-scope schema concern `ArtifactDeclaration` has no
field for, matching note's own justification.

## `.setup()` survives — why

Narrowed to: (a) `crate::apps::block2d::register()` — block2d's whole registration surface, deferred
(below), and (b) three `register_app_schema()` calls — `register_app_schema_descriptor` is not in §6's
artifact-scoped registrar set (app config/presence schema, `ArtifactDeclaration` deliberately has no
field for it — see that struct's own doc at `🔌️plugin/🦀️component.rs:930-943`). Both reasons are the
same two exceptions note's own exemplar documents; no other reason survives.

## block2d — deferred, with evidence (not a guess)

`◻2d` never had the block3d/block5d `⚙️engine`-as-registration-hub shape to begin with: on disk it
only ever had `🚪️io/🦀️component.rs` (no `⚙️engine` dir at all — confirmed with `find`). Worse: **a
different, concurrently-running session is mid-flight restructuring block2d's registration surface
right now**, moving it from `crate::artifacts::block2d::engine` (a mount `git diff` shows the last
*committed* state, `fd01661f06`, actually had) into `crate::apps::block2d::register()` — an
uncommitted, in-progress change I found live on disk:

```
$ git status --porcelain -- ✏️s/🔌️plugins/🧱️block/
 D ✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs
 M ✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs
 M ✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/🦀️component.rs   (+135 lines: block2d_io(), register(), register_pilot_languages(), ...)
 M ✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs
 ...
```
`git diff` on `📦️glue.rs` shows the OLD committed line was
`crate::artifacts::block2d::engine::register();` and the live uncommitted state already reads
`crate::apps::block2d::register();` — a full ⚙️engine→app-layer move, reasoned (in that session's own
new doc comment) as "an artifact must never depend on an app, so both [I/O surface + registration]
live [in the app] rather than under `🗿️artifacts`." That is a different, larger architectural call
than this ticket's mandate, made by someone else, in progress. Per the "Concurrent Cargo Workspace
Churn" / "Live Predicate, Not Currency" protocol: this is **not mine to finish or collide with**.
Files were still (mtime, `stat -f '%Sm'`) unchanged for 7+ minutes at last check — paused, not
actively typing, but genuinely someone else's uncommitted work, confirmed via `git diff`, not
inferred from file contents alone.

**Consequence**: block2d's `plugin()` wiring stays exactly as it was
(`.setup(crate::register_block_exports)` covers it via `crate::apps::block2d::register()`, whatever
that resolves to once the other session lands) — I did not touch `🎛️apps/◻2d/🦀️component.rs`,
`🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`, or the deleted `⚙️engine` file.
block2d needs its own W1b pass once that session's restructure lands and commits.

## `kit.catalog` — confirmed, and where

Block does **not** declare `kit.catalog` as an `ArtifactDeclaration.kind` (each artifact's own kind is
`"s.block2d"`/`"s.block3d"`/`"s.block5d"`). `kit.catalog` appears only as a `MediaPortSpec.kind_id` on
each app's `"catalog:out"` output port (`block2d_io()`/`block3d_io()`/`block5d_io()` in each
`⚙️engine`, and the live-churning `🎛️apps/◻2d/🦀️component.rs`'s new `block2d_io()`) — a declared
OUTPUT MEDIA KIND for the puzzle-catalog-fragment seam (`puzzle3d_catalog_fragment`/
`puzzle5d_catalog_fragment`), not an artifact ownership claim. It is a real duplicate: puzzle's own
apps declare the identical `"catalog:out"`/`kit.catalog` port shape for the same seam (confirmed by
grep — this file's own `KIT_CATALOG_ARTIFACT_ID` constant and doc comments reference "the
`s/plugin/puzzle` catalog artifact kind"). UCAS's map should record `kit.catalog` as a shared *media
port kind id*, produced by block2d/block3d/block5d **and** puzzle2d/puzzle3d/puzzle5d, not owned by
either plugin's `ArtifactDeclaration`.

## Step 3 — plugin root closed

Already clean before I started: `find 🧱️block -maxdepth 1` → only `🦀️component.rs`, `AGENTS.md`,
`🎛️apps`, `🗿️artifacts`, `📦️packages`. No `🛂️manifest/`/`🎟️capabilities/`/`🔧️setup/` dirs, no stray
`#[path]` mounts at the root. No `README.md` (only `AGENTS.md` — the spec's "AGENTS.md/README.md"
allows either).

## Step 4 — escape hatches / deps

- `register_mesh_*`/`register_solid_*`/`register_dwg_*`/`register_app_io`/`register_os_media_*`: **0
  hits**, whole plugin, grepped.
- `semio_framework_os::` (the escape-hatch crate, distinct from `semio-framework-os-kernel` which the
  plugin legitimately depends on via `dsl`/`store`/`protocol`/`pack`/`vcs` extern-crate aliases): **0
  hits**. Cargo.toml has no `semio-framework-os` (non-kernel) dependency to purge.

## Step 5 — inventory only

- `thread_local!` / `OnceLock<...Host>` (host/engine-handle statics): **0 hits**, whole plugin.
- `std::fs::`/`std::env::`/`std::process::`/`Command::new(` outside `#[cfg(test)]`: **0 hits**.
- Orphaned (post-conversion, zero call sites, left in place per note's own precedent rather than
  scope-creeping a delete): each artifact ROOT's own thin `pub mod io_registry { entries()/compose()/
  register() }` (`🗿️artifacts/🧊️3d/🦀️component.rs:137-161`, `🗿️artifacts/🖐️5d/🦀️component.rs:126-149`)
  — these wrapped `standards::v1::engine::io_registry::entries()` and were called only from the old
  `engine::register()` I just deleted; nothing else calls them (grepped repo-wide). Flagged for
  whoever next touches block3d/block5d, not deleted.

## Step 6 — verify

**1. `#[path]` resolution** (scripted, all 469 non-`.` mounts in `📦️glue.rs`):
```
path mounts: 765 total, 469 non-dot, 0 missing
```

**2. `include_str!`/`include_bytes!` resolution** (scripted, re-resolved against real files, not
pattern-substituted):
```
checked: 159 missing: 0
```

**3. `cargo metadata --no-deps --format-version 1`**: `OK`.

**4. `cargo check -p semio-s-plugin-block --all-targets`** (`RUSTC_WRAPPER=""`, this ticket's
`🎯️target`), real output, machine under heavy concurrent load from ~10 other W1b/W3 sessions sharing
the same target dir (`load average 27-45`, ~7 min wall time):
```
warning: `semio-s-plugin-block` (lib) generated 131 warnings
error: could not compile `semio-s-plugin-block` (lib) due to 14 previous errors; 131 warnings emitted
warning: build failed, waiting for other jobs to finish...
warning: `semio-s-plugin-block` (lib test) generated 137 warnings (130 duplicates)
error: could not compile `semio-s-plugin-block` (lib test) due to 14 previous errors; 140 warnings emitted
```
**Not 0 errors.** All 14, verified one by one, are pre-existing and untouched by this diff:

- **8× `E0308`, "expected `JsonValue`, found `Value`" / reverse**, in each artifact's
  `🚪️io/📤️export/🧵️serializers/…/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` and the matching
  `📥️import/🧩️deserializers/…` leaf (2d/3d/5d × export+import = 6, plus 2 duplicate 3d entries in the
  log) — a `serde_json::Value` vs. some `JsonValue` alias mismatch, unrelated to registration.
  `git log --oneline -1` on all 5 distinct files → **same single commit `2564722008`**, `git status
  --porcelain` → clean (no uncommitted changes, not touched by me, not touched by any concurrent
  session today). Pre-existing, repo-committed, orthogonal to `.artifact()`.
- **5× `E0080`, "`#[derive(Mutations)]`: `Block5dMutation::<X>`'s `MutationKind::SEMANTICS.kind` must
  equal `"<x>"` (its own kebab form)"** (`UpdatePart2d`/`UpdatePart3d`/`MoveGrip2d`/`MoveGrip3d`/
  `ResizeGrip3d`), all at `🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/
  🦀️component.rs:30`. `git log --oneline -1` → commit `11334431b9`, mtime `19:33:43` — SMO's own
  semantic-mutations wave, hours before this session. **This file is under `🧬️mutations/**`, which
  this ticket's own HARD RULES forbid touching** — reported, not fixed, correctly.

Neither error set mentions `ArtifactDeclaration`, `.artifact(`, or `declaration()` anywhere in the
full log (grepped); none are in the 5 files I edited
(`🧱️block/🦀️component.rs`, `📦️glue.rs`, the two `⚙️engine/🦀️component.rs` files). The
`.artifact()`/`declaration()` mechanism itself is not implicated by any of the 14 failures — the crate
simply cannot reach a clean build today for reasons two other tickets own.

## Files touched

- `✏️s/🔌️plugins/🧱️block/🦀️component.rs` — `plugin()`: `.artifact()` ×2 added, doc comment.
- `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs` — `register_block_exports()` narrowed (2 lines
  removed, doc comment added); block2d's own line inside it left as the concurrent session wrote it.
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` —
  `register()`+`register_pilot_languages()`+`register_artifact_schema()`+`register_artifact_inference()`
  → `declaration()` + `pilot_languages()`.
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` —
  identical treatment.

Nothing created, nothing deleted at the file level. block2d untouched.

## sharedFileRequests

**None outside my plugin.** Within my plugin, flagging for whoever owns the live block2d
restructure (unidentified session, uncommitted): once `crate::apps::block2d::register()` lands and
commits, block2d needs its own `declaration()` conversion — the schema/inference/composer/language
calls that function now makes (`crate::artifacts::block2d::io_registry::register()`,
`register_pilot_languages()`, `register_artifact_schema()`, `register_artifact_inference()`,
`register_document_codec_for_app::<Block2dPlayApp>()`) map onto `ArtifactDeclaration` exactly like
block3d/block5d did, but I did not perform that conversion myself to avoid colliding with in-progress,
uncommitted work in a file I do not own the current diff of.

**Also for the record, not mine to fix**: the 8 `E0308` `JsonValue`/`Value` mismatches (repo-committed,
commit `2564722008`) and the 5 `E0080` block5d kebab-mismatch panics (SMO's `🧬️mutations/**`, commit
`11334431b9`) — both block a clean `cargo check` for this crate independent of anything in this
ticket's scope.

## apa-status: partial

- M1 `ArtifactDeclaration` applied to block3d + block5d: **built, wired, compiles clean itself** (no
  error in any file I touched).
- block2d: **deferred**, real uncommitted concurrent-session evidence pasted above, not a guess.
- `.setup()`: narrowed to exactly 2 justified exceptions (block2d's whole surface pending the above,
  and 3× app-schema registration), matching note's own precedent.
- Root plugin closure, escape-hatch census, inventory: **all clean, 0 findings**.
- `cargo check -p semio-s-plugin-block --all-targets`: **14 pre-existing errors, all independently
  verified (git log + git status + commit ids) to predate and be unrelated to this diff** — 5 of them
  in territory (`🧬️mutations/**`) this ticket is explicitly forbidden from touching.
