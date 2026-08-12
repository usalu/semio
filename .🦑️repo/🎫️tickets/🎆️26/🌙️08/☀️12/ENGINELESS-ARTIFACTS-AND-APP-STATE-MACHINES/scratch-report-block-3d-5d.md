# Engineless Artifacts — `🧱️block` `🧊️3d` + `🖐️5d` Engine Dissolution

Ticket: `26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES` (#2553)
Package verified: `semio-s-plugin-block`
Exemplar mirrored: the already-dissolved `◻2d` sibling.

## (a) Engine directories dissolved

| Engine dir (absolute) | LOC before | Destinations |
| --- | --- | --- |
| `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` | 238 | `🧬️schema/🦀️component.rs` (helpers), `🧬️schema/💡️inferences/🦀️component.rs` (derived compute), `🚪️io/🦀️component.rs` (io_registry), `🎛️apps/🧊️3d/🦀️component.rs` (AppIo); `Block3dEngine` deleted |
| `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` | 223 | `🧬️schema/🦀️component.rs` (helpers), `🧬️schema/💡️inferences/🦀️component.rs` (derived compute), `🚪️io/🦀️component.rs` (io_registry), `🎛️apps/🖐️5d/🦀️component.rs` (AppIo); `Block5dEngine` deleted |

Both directories held exactly one file and **no `Cargo.toml`** (`find <dir> -name Cargo.toml` → empty for both, checked before deletion as the ticket requires). Both are now `rm -rf`'d.

## (b) Region-by-region disposition

### `🧊️3d` engine (238 LOC)

| Source region | Item(s) | Destination | Rule |
| --- | --- | --- | --- |
| `🔖️DocumentHelpers` | `empty_block3d_snapshot()`, `next_id()` | `🗿️artifacts/🧊️3d/…/🧬️schema/🦀️component.rs` → new `//#region 🔖️DocumentHelpers` | 3 (pure document helpers) |
| `🔖️DocumentHelpers` | `resolve_active_mesh_url()` | `🗿️artifacts/🧊️3d/…/🧬️schema/💡️inferences/🦀️component.rs` → `//#region 🔖️PuzzleCatalogFragment` | 8 (reachable only from the catalog-fragment inference path, so it travels with its sole caller) |
| `🔖️PuzzleCatalogFragment` | `puzzle3d_catalog_fragment()` | `🗿️artifacts/🧊️3d/…/🧬️schema/💡️inferences/🦀️component.rs` → `//#region 🔖️PuzzleCatalogFragment` | 2 (Snapshot→Value projection) |
| `🔖️Io` | `block3d_io() -> AppIo` | `🎛️apps/🧊️3d/🦀️component.rs` → new `//#region 🔖️Io` | 4 (returns `AppIo`) |
| `🔖️ArtifactEngine` | `struct Block3dEngine` + `impl Block3dEngine::new` | **DELETED** | 1 (fossil — see below) |
| `🚪️DerivedIoRegistry` | `mod io_registry` (+ nested `🔖️ExportEntries`) | `🗿️artifacts/🧊️3d/…/🚪️io/🦀️component.rs` → `//#region 🚪️DerivedIoRegistry` (with inner `//#region 🔖️ExportEntries` preserved) | 5 |
| `🧪️Tests` | 4 tests | split across the three destinations — see (d) | 9 |

### `🖐️5d` engine (223 LOC)

| Source region | Item(s) | Destination | Rule |
| --- | --- | --- | --- |
| `🔖️DocumentHelpers` | `empty_block5d_snapshot()`, `next_id()` | `🗿️artifacts/🖐️5d/…/🧬️schema/🦀️component.rs` → new `//#region 🔖️DocumentHelpers` | 3 |
| `🔖️PuzzleCatalogFragment` | `puzzle5d_catalog_fragment()` | `🗿️artifacts/🖐️5d/…/🧬️schema/💡️inferences/🦀️component.rs` → `//#region 🔖️PuzzleCatalogFragment` | 2 |
| `🔖️Io` | `block5d_io() -> AppIo` | `🎛️apps/🖐️5d/🦀️component.rs` → new `//#region 🔖️Io` | 4 |
| `🔖️ArtifactEngine` | `struct Block5dEngine` + `impl Block5dEngine::new` | **DELETED** | 1 (fossil) |
| `🚪️DerivedIoRegistry` | `mod io_registry` (+ nested `🔖️ExportEntries`) | `🗿️artifacts/🖐️5d/…/🚪️io/🦀️component.rs` → `//#region 🚪️DerivedIoRegistry` | 5 |
| `🧪️Tests` | 3 tests | split across the three destinations — see (d) | 9 |

### `*Engine` struct verdict (classification rule 1)

Both structs are confirmed fossils and were deleted outright:

- `grep -rn "Block3dEngine\|Block5dEngine" ✏️s 🧰️framework` → the **only** hits were the definitions inside the two engine files themselves. Zero external constructions, zero external references.
- `grep -rn "trait ArtifactEngine\|impl.*ArtifactEngine for" ✏️s 🧰️framework` → zero trait definitions and zero impls in shipped source (the single textual hit is a prose comment in `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs:223` that itself documents the count as zero).
- Post-deletion re-grep for `Block3dEngine|Block5dEngine` across `✏️s/🔌️plugins/🧱️block` → empty.

No first-of-its-kind exception was found; this matches the `◻2d` exemplar and the ticket's project-wide expectation.

### Rule 6 (`register*()` wiring) — nothing to move

Neither engine file contained a `register*()` function. The only registration surface was `io_registry::register()`, which lives in the **artifact root's** thin wrapper module (`🗿️artifacts/🧊️3d/🦀️component.rs` / `🖐️5d/🦀️component.rs`), not in the engine, and was already superseded by each root's `declaration()`. Left in place, only its `use` target re-pointed (see (c)). No `//#region 🔌️Registration` was needed in either app — same outcome as the `◻2d` exemplar.

### Rule 7 (stateful behaviour) — nothing to park

No `&mut self` methods beyond snapshot construction, no `thread_local!`, no mutable caches. The `OnceLock<Vec<ComposerEntry>> ENTRIES` static travelled intact with `io_registry` into `🚪️io` (it is the exemplar's own convention there, not app state). The reserved app-side stub dirs `🎛️apps/🧊️3d/⚙️engine` and `🎛️apps/🖐️5d/⚙️engine` were left untouched, as instructed.

## (c) Unqualified / stale paths found and how they were qualified

The critical hazard the ticket flagged is real in this packet: each artifact root (`🗿️artifacts/🧊️3d/🦀️component.rs`, `🖐️5d/🦀️component.rs`) has its **own** local `io_registry` wrapper whose `entries()` returns `&'static [&'static ComposerEntry]` — a *different type* from the real `🚪️io` one returning `&'static [ComposerEntry]`. Every moved or re-pointed call was therefore written fully qualified.

| # | File | Was | Now |
| --- | --- | --- | --- |
| 1 | `🗿️artifacts/🧊️3d/🦀️component.rs:141` | `use crate::artifacts::block3d::standards::v1::engine::io_registry as v1;` | `use crate::artifacts::block3d::standards::v1::subsets::any::io::io_registry as v1;` |
| 2 | `🗿️artifacts/🧊️3d/🦀️component.rs:174` (`declaration()` `.composers(...)`) | `…::standards::v1::engine::io_registry::entries()` | `…::standards::v1::subsets::any::io::io_registry::entries()` |
| 3 | `🗿️artifacts/🖐️5d/🦀️component.rs:129` | `use crate::artifacts::block5d::standards::v1::engine::io_registry as v1;` | `use crate::artifacts::block5d::standards::v1::subsets::any::io::io_registry as v1;` |
| 4 | `🗿️artifacts/🖐️5d/🦀️component.rs:162` (`declaration()` `.composers(...)`) | `…::standards::v1::engine::io_registry::entries()` | `…::standards::v1::subsets::any::io::io_registry::entries()` |
| 5 | `🧬️schema/🧬️mutations/🦀️component.rs` (3d, test import) | `crate::artifacts::block3d::engine::empty_block3d_snapshot` | `crate::artifacts::block3d::schema::empty_block3d_snapshot` |
| 6 | `🧬️schema/🧬️mutations/🦀️component.rs` (5d, test import) | `crate::artifacts::block5d::engine::empty_block5d_snapshot` | `crate::artifacts::block5d::schema::empty_block5d_snapshot` |
| 7 | `🎛️apps/🧊️3d/🦀️component.rs` `initial_snapshot()` | `…block3d::engine::empty_block3d_snapshot()` | `…block3d::schema::empty_block3d_snapshot()` |
| 8 | `🎛️apps/🧊️3d/🦀️component.rs` `io()` + `.io(…)` in manifest | `…block3d::engine::block3d_io()` | bare local `block3d_io()` (now defined in this same file) |
| 9 | `🎛️apps/🧊️3d/🦀️component.rs` `export_media()` | `…block3d::engine::puzzle3d_catalog_fragment(…)` | `…block3d::schema::inferences::puzzle3d_catalog_fragment(…)` |
| 10 | `🎛️apps/🧊️3d/🎮️commands/🌀️vortex`, `🔘️vortex-kind`, `🧱️representation`, `🖌️brush` (4 sites) | `…block3d::engine::next_id(…)` | `…block3d::schema::next_id(…)` |
| 11 | `🎛️apps/🖐️5d/🦀️component.rs` `initial_snapshot()` | `…block5d::engine::empty_block5d_snapshot()` | `…block5d::schema::empty_block5d_snapshot()` |
| 12 | `🎛️apps/🖐️5d/🦀️component.rs` `io()` + `.io(…)` in manifest | `…block5d::engine::block5d_io()` | bare local `block5d_io()` (now defined in this same file) |
| 13 | `🎛️apps/🖐️5d/🦀️component.rs` `export_media()` | `…block5d::engine::puzzle5d_catalog_fragment(…)` | `…block5d::schema::inferences::puzzle5d_catalog_fragment(…)` |
| 14 | `🎛️apps/🖐️5d/🎮️commands/🔘️grip-kind`, `🌱️grip` (2 sites) | `…block5d::engine::next_id(…)` | `…block5d::schema::next_id(…)` |

Both spellings the ticket warned about were checked (`::engine::`, `standards::v1::engine`, `subsets::any::engine`) and the real internal module names were confirmed by reading `📦️glue.rs` — they are literally `block3d` / `block5d`, mounted at `crate::artifacts::block{3,5}d`.

Additionally, `📦️glue.rs` (the sole mounting mechanism) was updated: the `#[path = "…/⚙️engine/🦀️component.rs"] pub mod engine;` mounts and the `pub mod engine { pub use super::standards::v1::engine::*; }` shims were removed for **both** block3d and block5d. `grep -n "engine" 📦️glue.rs` → 0 hits.

Doc-comment references to the dead module were also corrected in `🎛️apps/🧊️3d/🦀️component.rs` (module header, `KIT_CATALOG_ARTIFACT_ID` doc, `export_media` doc, manifest comment), `🎛️apps/🖐️5d/🦀️component.rs` (same four), and `🎛️apps/🧊️3d/🌍️world/🦀️component.rs` (module header).

## (d) Assertion counts — before vs after

Counting method: `grep -c "assert" <file>`, cross-checked per-test by hand.

### block3d — 11 before, 11 after

| Test | Asserts | Before (engine file) | After (destination) |
| --- | --- | --- | --- |
| `empty_definition_matches_default` | 1 | `⚙️engine/🦀️component.rs` | `🧬️schema/🦀️component.rs` `//#region 🧪️Tests` |
| `resolve_active_mesh_url_prefers_matching_tags` | 2 | `⚙️engine/🦀️component.rs` | `🧬️schema/💡️inferences/🦀️component.rs` `//#region 🧪️PuzzleCatalogFragment` |
| `puzzle3d_catalog_fragment_maps_vortices` | 2 | `⚙️engine/🦀️component.rs` | `🧬️schema/💡️inferences/🦀️component.rs` `//#region 🧪️PuzzleCatalogFragment` |
| `block3d_io_declares_the_catalog_out_port` | 6 | `⚙️engine/🦀️component.rs` | `🎛️apps/🧊️3d/🦀️component.rs` `//#region 🔖️Manifest` |
| **total** | **11** | **11** | **1 + 2 + 2 + 6 = 11** ✅ |

### block5d — 6 before, 6 after

| Test | Asserts | Before (engine file) | After (destination) |
| --- | --- | --- | --- |
| `empty_definition_matches_default` | 1 | `⚙️engine/🦀️component.rs` | `🧬️schema/🦀️component.rs` `//#region 🧪️Tests` |
| `puzzle5d_catalog_fragment_maps_grips` | 2 | `⚙️engine/🦀️component.rs` | `🧬️schema/💡️inferences/🦀️component.rs` `//#region 🧪️PuzzleCatalogFragment` |
| `block5d_io_declares_the_catalog_out_port` | 3 | `⚙️engine/🦀️component.rs` | `🎛️apps/🖐️5d/🦀️component.rs` `//#region 🔖️Manifest` |
| **total** | **6** | **6** | **1 + 2 + 3 = 6** ✅ |

Every assertion survived; no test was dropped, merged, or weakened. No new test *files* were created — all tests landed in existing `#[cfg(test)] mod tests` blocks in the destination files.

Post-move whole-file `grep -c assert` for context (these include each file's pre-existing tests, so they are larger than the migrated counts above): 3d schema 1, 3d inferences 9 (6 pre-existing + 3 net new lines from the 2 migrated tests... see per-test table for the authoritative split), 3d app 39; 5d schema 1, 5d inferences 7, 5d app 33.

## (e) Compiler output and error attribution

Command (both mandated flags):

```
RUSTC_WRAPPER="" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES/🎯️target" cargo check -p semio-s-plugin-block --all-targets
```

Note on running it: this ticket's target dir is shared by ~12 concurrent sibling agents, so cargo serialized the runs and the check spent a long time `Blocking waiting for file lock on build directory`. An earlier attempt also aborted on a `semio-s-plugin-stdio` failure (`couldn't read …🧿️semio/…/📄set-snapshot/↩️inverse/🦀️component.rs`); a concurrent session has since fixed that (the working-tree glue.rs now has 0 references to that path where committed HEAD had 3), and **stdio compiled cleanly in the final run** (`grep -c "could not compile .semio-s-plugin-stdio"` → 0).

Verbatim final lines:

```
For more information about an error, try `rustc --explain E0080`.
warning: `semio-s-plugin-block` (lib) generated 138 warnings
error: could not compile `semio-s-plugin-block` (lib) due to 8 previous errors; 138 warnings emitted
warning: build failed, waiting for other jobs to finish...
warning: `semio-s-plugin-block` (lib test) generated 144 warnings (137 duplicates)
error: could not compile `semio-s-plugin-block` (lib test) due to 8 previous errors; 147 warnings emitted
```

The crate does **not** reach `Finished`. I am not claiming green. Error tally: `5 × error[E0080]`, `3 × error[E0308]`. Per-error attribution follows; **all 8 are pre-existing, in files this packet never edited.**

### Group 1 — 3 × `E0308` in JSON export serializer leaves (NOT mine)

```
error[E0308]: `?` operator has incompatible types
  --> …/🗿️artifacts/🧊️3d/…/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs:10:16
   |
10 |           value: serde_json::to_value(snapshot)
11 | |             .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?,
   | |____^ expected `JsonValue`, found `Value`
   = note: `?` operator cannot convert from `serde_json::Value` to `JsonValue`
```

Identical error at the `🖐️5d` and **`◻2d`** equivalents. Attribution:
- All three files are **unmodified** (`git status --short` → no output) and last committed `2564722008` (2026-08-10), well before this session.
- I edited `🚪️io/🦀️component.rs` only; these are `🚪️io/📤️export/🧵️serializers/…/🔣️json/…` leaves several levels deeper, untouched.
- **Decisive**: the third instance is in `◻2d` — the already-dissolved exemplar that is explicitly not in my assignment and that I never opened for writing. A cause that also fires in `◻2d` cannot be my packet.
- Real cause: a framework-side `store`/`dsl` type change (`JsonValue` vs `serde_json::Value`) landing under these leaves — matching the ticket's "framework `🏪️store`/`📡️spr`/`🗣️dsl`" known context.

### Group 2 — 5 × `E0080` on `#[derive(Mutations)]` for `Block5dMutation` (NOT mine)

```
error[E0080]: evaluation panicked: #[derive(Mutations)]: Block5dMutation::UpdatePart2d's
              MutationKind::SEMANTICS.kind must equal "update-part2d" (its own kebab form)
  --> …/🗿️artifacts/🖐️5d/…/🧬️schema/🧬️mutations/🦀️component.rs:30:73
   |
30 | #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
   |                                                           ^^^^^^^^^^^^^^
```

…and the same for `UpdatePart3d`, `MoveGrip2d`, `MoveGrip3d`, `ResizeGrip3d`.

Attribution:
- This is the one erroring file I did touch — but my **only** change to it is line 133, a test-module `use crate::artifacts::block5d::schema::empty_block5d_snapshot;`. The error is at **line 30**, the derive on the enum, and concerns enum variants at lines 40–62 that are pre-existing enum body.
- Root cause located in untouched, clean, already-committed slug leaves: they declare e.g. `kind: "move-grip-3d"` / `"update-part-3d"` / `"resize-grip-3d"` (hyphen before the digit) while the derive computes the kebab form of `MoveGrip3d` as `move-grip3d` (no hyphen before the digit). A slug-naming mismatch.
- Those leaves (`🖌️update-part-2d`, `🧊update-part-3d`, `📍move-grip-2d`, `🧭move-grip-3d`, `📏resize-grip-3d`) are all `git status` clean and last committed `11334431b9` (2026-08-12) — an ancestor of this session's start HEAD `20252aa16d`, i.e. pre-existing.

### Decisive negative signals that this packet introduced nothing

```
$ grep -cE "E0432|E0433|E0583|unresolved import|cannot find module|file not found for module" <log>
0

$ (errors mentioning engine|io_registry|block3d_io|block5d_io|puzzle*_catalog_fragment|
   empty_block*_snapshot|next_id)
0
```

This is the strongest evidence available: a botched module dissolution — a stale `#[path]` mount, a dropped `pub mod`, a mis-qualified `io_registry`, a moved symbol left dangling — fails as `E0432`/`E0433`/`E0583`/"file not found for module". There are **zero** such errors, and **zero** errors naming any symbol this packet moved or any `⚙️engine` path. The module graph resolves exactly as intended, and the `io_registry` type-confusion hazard did not materialise (an unqualified rebind to the root's `&'static [&'static ComposerEntry]` wrapper would have surfaced as an `E0308` at the `.composers(...)` call — it does not appear).

Error count re-derived rather than trusted: the ticket noted ~14 pre-existing errors traced to `🗄️stdio/📦️glue.rs` and framework modules; it is now **8**, and stdio itself compiles. The reduction is other sessions' repair work landing, not anything from this packet.

## (f) Deviations from plan

1. **`resolve_active_mesh_url` placed with the inference, not with the plain schema helpers.** It sat under the engine's `🔖️DocumentHelpers` region, but its only caller anywhere in the repo is `puzzle3d_catalog_fragment` (verified: `grep -rn "resolve_active_mesh_url" ✏️s` returns only the definition, its one internal call, and its own test). Under classification rule 8 it travels with the path that reaches it, so it went to `💡️inferences` alongside its caller rather than being split from it.
2. **`io_registry::register()`/`compose()` left at the artifact root.** The ticket's rule 6 concerns `register*()` inside the *engine*; these live in the roots' own pre-existing thin wrapper and were already superseded by `declaration()`. Only their `use` target was re-pointed. Nothing was moved into `📦️glue.rs` and no `//#region 🔌️Registration` was added — matching the `◻2d` exemplar, whose registration region is comment-only.
3. **`📦️glue.rs` was edited.** Not called out explicitly in the assignment, but unavoidable and correct: it is the sole module-mounting mechanism, so the `pub mod engine;` mounts and the `pub mod engine { … }` shims had to go or the deleted files would still be `#[path]`-mounted. This is inside `✏️s/🔌️plugins/🧱️block/**` and is not `📜️script.ts`/`🔣️taxonomy.json`/`AGENTS.md`/stdio, so it violates no hard rule.
4. **Doc comments were rewritten** in five files beyond the pure code moves, because they named `crate::artifacts::block{3,5}d::engine` — a module that no longer exists. Leaving them would have shipped stale references.

## (g) Files touched

### Removed (directories, each holding exactly one `🦀️component.rs`, no `Cargo.toml`)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/`

### Updated
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/🌍️world/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/🎮️commands/🌀️vortex/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/🎮️commands/🔘️vortex-kind/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/🎮️commands/🧱️representation/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/🎮️commands/🖌️brush/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🎛️apps/🖐️5d/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🎛️apps/🖐️5d/🎮️commands/🔘️grip-kind/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🎛️apps/🖐️5d/🎮️commands/🌱️grip/🦀️component.rs`

### Created
None (no new source files, no new test files, no new example files — every destination already existed).

Scratch logs written under the ticket folder (`.txt`, per the hard rule): `scratch-cargo-check-1.txt`, `scratch-cargo-check-2.txt`, `scratch-cargo-check-final.txt`, `scratch-cargo-check-block-final.txt`.

## Structural verification

```
$ grep -rn "::engine::\|standards::v1::engine\|subsets::any::engine" ✏️s/🔌️plugins/🧱️block
(no output)

$ find ✏️s/🔌️plugins/🧱️block -path "*🗿️artifacts*" -name "⚙️engine" -type d
(no output)

$ grep -n "engine" ✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs
(no output)

$ grep -rn "Block3dEngine\|Block5dEngine" ✏️s/🔌️plugins/🧱️block
(no output)
```

App-side reserved stub dirs correctly preserved:
```
$ find ✏️s/🔌️plugins/🧱️block/🎛️apps -name "⚙️engine" -type d
✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/⚙️engine
✏️s/🔌️plugins/🧱️block/🎛️apps/🖐️5d/⚙️engine
✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/⚙️engine
```

---

**VERDICT — Structural: PASS (all four greps empty, both `⚙️engine` dirs deleted, app stubs preserved, 11/11 and 6/6 assertions survived). Compile: PASS BY ATTRIBUTION, NOT GREEN — `semio-s-plugin-block` does not reach `Finished`, but all 8 errors (5×E0080, 3×E0308) are pre-existing in untouched, already-committed files (one of them in the out-of-scope `◻2d` exemplar), and there are zero unresolved-module/import errors and zero errors naming any symbol or path this packet moved.**
