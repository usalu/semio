# Packet — Dissolve `vcs`'s `⚙️engine`

Target deleted: `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (183 lines).
The `⚙️engine/` directory no longer exists under `✳️any/`.

## Destination per region

| Region | Destination | Notes |
|---|---|---|
| `🔖️DocumentHelpers` (`empty_vcs_snapshot`) | `🧬️schema/🦀️component.rs`, new `//#region 🔖️DocumentHelpers` right after the header imports | Moved verbatim, just uses the file's existing `crate::artifacts::vcs::VcsSnapshot` fully-qualified style already present elsewhere in that file. |
| `🔖️Register` (`register`, `register_pilot_languages`) | `🎛️apps/🌿️vcs/🦀️component.rs`, new `//#region 🔌️Registration` right after `🔖️Constants` | Kept as real functions (not the newer `declaration()`-based pattern I found already live on `🧱️block/◻2d` — that pattern belongs to the separate, further-along ticket `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`; this packet's map only asked for the app-side function move, so I followed the map literally, matching that same ticket's *earlier* exemplar packet report `📓️packet-a2-block2d-report.md`, which used the identical "new `🔌️Registration` region in the app file" shape before it was later superseded). |
| `🔖️SchemaRegistry` (`register_artifact_schema`, `register_artifact_inference`) | `🎛️apps/🌿️vcs/🦀️component.rs`, new `//#region 🔖️SchemaRegistry` right after `🔌️Registration` | Verbatim, no logic changes. |
| `🔖️ArtifactEngine` (`VcsDemoEngine` struct + impl) | **deleted outright** | See "VcsDemoEngine evidence" below. |
| `🚪️DerivedIoRegistry` (`pub mod io_registry` w/ `ComposerEntry`s + export composer) | `🚪️io/🦀️component.rs`, new `//#region 🚪️DerivedIoRegistry` after `🎹️DerivedComposition` | Moved verbatim — this is the **low-level** `io_registry` (owns `entries() -> &'static [ComposerEntry]`), distinct from — and the callee of — the artifact-top-level `io_registry` wrapper (the shadowing defect, see below). |
| `🧪️Tests` (`empty_snapshot_matches_schema`) | Followed `empty_vcs_snapshot` into `🧬️schema/🦀️component.rs`, new `//#region 🧪️Tests` (the file had none before) | 1 assertion-bearing test in, 1 out — no drop. |

## The shadowing `declaration()`-equivalent repoint

This artifact has no literal `declaration()`/`.composers(...)` function (that pattern belongs to a further-migrated artifact shape, e.g. `📕️norm`). The load-bearing equivalent here is the artifact root's own pre-existing shadowing `io_registry` wrapper at `🗿️artifacts/🌿️vcs/🦀️component.rs` (lines 30-54, **not** one of the 6 mapped regions but a real call site):

```rust
pub mod io_registry {
    ...
    use crate::artifacts::vcs::standards::v1::engine::io_registry as v1;   // BEFORE
    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }
    pub fn register() { register_composer_entries(v1::entries()); }
}
```

This is exactly the ticket's warned-about defect shape: this wrapper's `entries()` returns `&'static [&'static ComposerEntry]` (a `.iter().collect()` view) while the engine's own `entries()` (now moved into `🚪️io`) returns `&'static [ComposerEntry]`. The `use ... as v1` import was repointed, fully qualified:

```rust
use crate::artifacts::vcs::standards::v1::subsets::any::io::io_registry as v1;   // AFTER
```

I did not touch this wrapper's own `entries()`/`register()` bodies — they already call `v1::entries()` through the alias, so repointing the one import line was sufficient and there was no bare/unqualified `io_registry::entries()` call anywhere in this plugin to fall into the shadow trap.

## The 9 `::engine::` references outside the engine dir — all 9 were CODE, 0 doc comments

(8 matched the literal grep `::engine::`; the 9th, `use crate::artifacts::vcs::engine;` in the mutations test file, has no trailing `::` after `engine` so the literal pattern misses it — counted here since it's the same reference class.)

| # | File:line (before) | What it was | Fix |
|---|---|---|---|
| 1 | `✏️s/🔌️plugins/🌿️vcs/🦀️component.rs:10` | `.setup(crate::artifacts::vcs::engine::register)` | → `.setup(crate::apps::vcs::register)` |
| 2 | `🎛️apps/🌿️vcs/🦀️component.rs:180` | `crate::artifacts::vcs::engine::empty_vcs_snapshot()` in `initial_snapshot()` | → `crate::artifacts::vcs::standards::v1::subsets::any::schema::empty_vcs_snapshot()` |
| 3 | `🗿️artifacts/🌿️vcs/🦀️component.rs:34` | `use crate::artifacts::vcs::standards::v1::engine::io_registry as v1;` (the shadow-defect wrapper import) | → `use crate::artifacts::vcs::standards::v1::subsets::any::io::io_registry as v1;` |
| 4 | `🧬️schema/📸️snapshot/📝️text/🦀️component.rs:34` | test, `assert_dsl_round_trip(&...engine::empty_vcs_snapshot())` | → `...schema::empty_vcs_snapshot()` |
| 5 | `🧬️schema/📸️snapshot/💾️binary/🦀️component.rs:33` | test, `let projection = ...engine::empty_vcs_snapshot();` | → `...schema::empty_vcs_snapshot()` |
| 6 | `🧬️schema/📸️snapshot/💾️binary/🦀️component.rs:50` | test, `create_document_envelope(..., ...engine::empty_vcs_snapshot(), None)` | → `...schema::empty_vcs_snapshot()` |
| 7 | `📦️packages/🦀️rust/📦️glue.rs:304` | legacy shim `pub mod engine { pub use super::standards::v1::engine::*; }` | **removed**, along with the `pub mod engine;` `#[path=...]` mount (lines 37-38) |
| 8 | `🧬️schema/🔺️diff/📝️text/🦀️component.rs:134` | test, `let base = ...engine::empty_vcs_snapshot();` | → `...schema::empty_vcs_snapshot()` |
| 9 | `🧬️schema/🧬️mutations/🦀️component.rs:37` | `use crate::artifacts::vcs::engine;` (test import), feeding 6 bare `engine::empty_vcs_snapshot()` call sites at lines 43/50/57/64/73/82 | import repointed to `use crate::artifacts::vcs::standards::v1::subsets::any::schema::empty_vcs_snapshot;`; all 6 call sites mechanically rewritten `engine::empty_vcs_snapshot()` → `empty_vcs_snapshot()` |

Post-fix, `grep -rn "::engine::" ✏️s/🔌️plugins/🌿️vcs --include="*.rs"` → 0. Remaining bare-word `engine` hits are all doc comments/unrelated identifiers (`EngineHandles`, `_engines: &EngineHandles`) — two of which I updated in passing since they were now-stale prose pointing at a directory that no longer exists (`🎛️apps/🌿️vcs/🦀️component.rs`'s file header, and the new `empty_vcs_snapshot` doc comment in `🧬️schema/🦀️component.rs` which deliberately keeps the `⚙️engine` name as a "was:" lineage note).

## `VcsDemoEngine` evidence

`grep -rn "VcsDemoEngine" ✏️s/🔌️plugins/🌿️vcs --include="*.rs"` (before deletion) → exactly 2 hits, both inside the engine file itself: the `pub struct VcsDemoEngine { ... }` definition and its own `impl VcsDemoEngine { pub fn new(...) -> Self { ... } }` block. No construction site (`VcsDemoEngine::new(...)` call) anywhere else in the plugin or repo (`grep -rln "VcsDemoEngine" ✏️s` → only that one file). Repo-wide: `grep -rln "trait ArtifactEngine" ✏️s` → 1 hit, but it is a doc comment in `🪐️space/🏠️home`'s schema file quoting the grep result itself ("a trait that has zero implementations... `grep -rn "trait ArtifactEngine"` → 0 repo-wide"), not a real trait definition; `grep -rln "impl.*ArtifactEngine for" ✏️s` → 0. Deleted outright per the map, as instructed.

## Assertion arithmetic

1 test in the engine file (`empty_snapshot_matches_schema`) → 1 test in `🧬️schema/🦀️component.rs`'s new `mod tests`. No other test bodies were touched; the 6 mechanical `engine::` → bare rewrites in the mutations test file did not add/remove/rename any `#[test]` fn.

## Compiler — ran, real compilation happened, never reached `semio-s-plugin-vcs`

```
RUSTC_WRAPPER="" cargo check -p semio-s-plugin-vcs --all-targets
```

Full output: `scratch-vcs-engine-cargo-check.txt` in this ticket folder (396KB). This was a genuine compile, not a silent sccache no-op: hundreds of KB of real warnings were emitted across `semio-framework-*` and `semio-s-plugin-stdio` crates, and `Checking semio-framework-plugin v0.1.0 (...)` / `Checking semio-s-plugin-stdio v0.1.0 (...)` lines are present. It failed here:

```
    Checking semio-s-plugin-stdio v0.1.0 (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust)
error[E0425]: cannot find type `SemioMeshSnapshot` in this scope
  --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/.../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🦀️component.rs:99:24
   |
99 | #[mutations(snapshot = SemioMeshSnapshot, diff = SemioMeshDiff, schema = "s.stdio.semio.mesh")]
   |                        ^^^^^^^^^^^^^^^^^ not found in this scope
...
error: could not compile `semio-s-plugin-stdio` (lib) due to 1 previous error; 602 warnings emitted
```

**Attribution: (c) upstream, not mine, not this ticket's fault.** `semio-s-plugin-vcs`'s `Cargo.toml` hard-depends on `semio-s-plugin-stdio` (`path = "../../../🗄️stdio/📦️packages/🦀️rust"`), and `grep -n "Checking\|Compiling" scratch-vcs-engine-cargo-check.txt` shows no `semio-s-plugin-vcs` line at all — the crate this packet touched was never reached. `git status --porcelain ✏️s/🔌️plugins/🗄️stdio` shows live modified/added files there (`🦀️component.rs`, `glue.rs`, a new `📦aabb` inference leaf) not made by me — this matches the ticket's own briefing that stdio is mid another session's live vocabulary migration. **I did not touch anything under `✏️s/🔌️plugins/🗄️stdio`.**

In lieu of a real compile, I manually verified: brace/paren balance on all 10 modified `.rs` files (all matched, e.g. `apps/🌿️vcs/🦀️component.rs` 162/162 braces, 480/480 parens), confirmed `dsl::`/`::schema::` bare-path usage (no local `use` needed) is the established pattern elsewhere in this same crate (both are `extern crate ... as {dsl,schema}` aliases at the crate root in `glue.rs`), and re-read every edited region end-to-end for syntax and reference correctness.

**Status: refactor complete, all three structural greps green (see below), but compiler-UNVERIFIED for `semio-s-plugin-vcs` itself due to upstream `semio-s-plugin-stdio` breakage.** Re-run `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-vcs --all-targets` once stdio is green again.

## Structural verification (all green)

```
find ✏️s/🔌️plugins/🌿️vcs -path "*🗿️artifacts*" -name "⚙️engine" -type d   → (empty, 0 results)
grep -rn "::engine::" ✏️s/🔌️plugins/🌿️vcs --include="*.rs"                  → (empty, 0 results)
grep -rn "VcsDemoEngine" ✏️s/🔌️plugins/🌿️vcs --include="*.rs"               → (empty, 0 results)
```

## Deviations from the literal instructions

- Kept `register()`/`register_pilot_languages()`/`register_artifact_schema()`/`register_artifact_inference()` as real functions in `🎛️apps/🌿️vcs/🦀️component.rs`, not the newer fully-declarative `declaration()` pattern I found already live on `🧱️block/◻2d` while researching the exemplar — that pattern is scoped to the separate, further-along ticket `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE` (confirmed by that file's own doc comment citing that ticket ID) and is out of scope here. The block2d exemplar's *original* packet report (`📓️packet-a2-block2d-report.md`, this same ticket) used the identical "new `🔌️Registration` region in the app file" shape I used — the later `declaration()` refactor happened afterward, in a different ticket.
- Fixed two now-stale doc-comment mentions of `⚙️engine` (in `🎛️apps/🌿️vcs/🦀️component.rs`'s file header and in the io/component.rs header) so they don't point at a directory that no longer exists — these weren't in the "9 references" list (not `::engine::` code paths) but were adjacent prose that would otherwise mislead the next reader.

## Noticed but deliberately NOT touched (belongs to the machine-state redesign)

`🎛️apps/🌿️vcs/🦀️component.rs`'s `seed_vcs_demo_history` dispatches `ArtifactCommand::CheckoutCheckpoint`/`SwitchAlternative`/`CreateAlternative`/`CommitCheckpoint` extensively (lines ~90-153), and `VcsPlayApp::initial_snapshot`/`seed` wire directly into `ArtifactStore<VcsSnapshot, VcsDemoMutation>` with `active_alternative_id` reads at lines 106/115. All of this is exactly the thesis called out in `📓️vcs-machine-state-design.md` — "what you are looking at" operations misfiled as document mutations. Out of scope per this packet's explicit instruction; left untouched.

## Files touched

- Removed: `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (and the now-empty `⚙️engine/` directory)
- Updated: `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- Updated: `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- Updated: `✏️s/🔌️plugins/🌿️vcs/🎛️apps/🌿️vcs/🦀️component.rs`
- Updated: `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🦀️component.rs`
- Updated: `✏️s/🔌️plugins/🌿️vcs/🦀️component.rs`
- Updated: `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📦️glue.rs`
- Updated: `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs`
- Updated: `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs`
- Updated: `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs`
- Updated: `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- Scratch: `scratch-vcs-engine-cargo-check.txt` in this ticket folder (full cargo check output, 396KB)
