# Packet report — forms / reasoning / lowpoly / demonstrator / shooting

Assigned targets (per dispatch): `📋️forms`, `💡️reasoning`, `💠️lowpoly`, `🎪️demonstrator`, `🎥️shooting`.

## ⚠️ Deviation: `💡️reasoning` SKIPPED — HELD by SMO (#2545)

Before touching anything I read `../SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md` (the live
predicate this ticket's own `📌️important.md` names as authoritative: *"Absence from this file means
FREE, not held ... Only an explicit entry under HELD blocks you"*). `💡️reasoning` is explicitly listed
under `## HELD — lane in flight` (updated 2026-08-12, same day):

> `🏛️architect` (restructure landed, report written, awaiting confirmation), `🎞️animate`,
> `🏭️process`, `💡️reasoning`.

This matches `📓️packet-manifest.md`'s own dependency table: *"4 plugins are HELD by #2545 ... Do not
touch; coordinate."* `💡️reasoning`'s engine dir was left untouched. Its find-hit remains:
`✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine`. This is a
genuine cross-session hold, not an oversight — re-dispatch this one plugin once SMO releases it.

Work proceeded on the other 4 targets: `📋️forms`, `🎪️demonstrator` done directly; `🎥️shooting` and
`💠️lowpoly` dispatched as parallel sub-agents (both plugins had no `Cargo.toml` inside their engine
dirs — confirmed safe to delete before any relocation began, per the ticket's hard rule).

## Per-plugin table

| plugin | engine dir gone | `::engine::` refs | compile |
|---|---|---|---|
| `📋️forms` | ✅ | 0 | blocked-churn (stdio red) |
| `🎪️demonstrator` | ✅ | 0 (+2 out-of-scope other-plugin hits) | blocked-churn (stdio red) |
| `🎥️shooting` | ✅ | 0 (+1 false positive: stdio's own `engine::geometry`) | blocked-churn (stdio red) |
| `💠️lowpoly` | pending sub-agent | pending | pending |
| `💡️reasoning` | **SKIPPED — HELD** | n/a | n/a |

(This table is updated as the two dispatched sub-agents report back; see their sections below.)

## `📋️forms`

### What changed

Engine file: `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`
(356 lines) — DELETED (directory removed).

Destinations:
- `🔖️PlaybookVocabulary` (playbook re-exports, `initial_try_values`), `🔖️DocumentHelpers`
  (`empty_forms_snapshot`, `building_component_spec`, `default_example_spec`/`_json`,
  `onboarding_example_spec`/`_json`, `flatten_questions`), `🔖️QuestionLocation` (`QuestionLocation`,
  `locate_question`, `update_block_operation`), `🔖️Ids` (`create_form_id`, `forms_play_step_tree_id`),
  `🔖️Values` (`value_to_dsl`, `dsl_to_value`, `dsl_string_value`, `dsl_f64_value`, `json_string_value`,
  `json_f64_value`) → all moved into
  `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
  (pure document helpers, no `&mut self`, no app type).
- `forms_io()` (returns `AppIo`) → inlined into a new `🔖️Io` region in
  `✏️s/🔌️plugins/📋️forms/🎛️apps/📋️forms/🦀️component.rs` (the app's own file — it was the sole caller).
- `FormsEngine` struct (`new`/`into_snapshot`, fields `artifact`/`snapshot`) → **DELETED OUTRIGHT**.
  Verified zero external references before deletion (`grep -rn "FormsEngine" ✏️s/🔌️plugins/📋️forms`
  showed only its own definition). No `trait ArtifactEngine` exists repo-wide, confirmed independently.
- `io_registry` module (composer entries, export-only JSON bridge) → moved verbatim into
  `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
  as a new `🚪️DerivedIoRegistry` region.
- Tests: all 10 `#[test]` fns split — 9 to schema's new `🧪️Tests` region, 1
  (`forms_io_declares_dictionary_out_port`, renamed to keep it distinct from the pre-existing
  `forms_io_exposes_dictionary_out_port`) to the app file's existing `🔖️MediaPorts` test region.

### Shadow trap — handled

`✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️component.rs` has its own wrapper `pub mod io_registry`
(returns `&[&ComposerEntry]`, a `.iter().collect()` view) that shadows the relocated one. Its internal
`use crate::artifacts::forms::standards::v1::engine::io_registry as v1;` was repointed to
`crate::artifacts::forms::standards::v1::subsets::any::io::io_registry as v1;`, and `declaration()`'s
`.composers(...)` call was repointed the same way. Both fully qualified from crate root — no bare
`io_registry::entries()` anywhere in a moved body.

### Every unqualified path qualified

All ~40 call sites across `🎛️apps/📋️forms/**` that referenced `crate::artifacts::forms::engine::<X>`
were rewritten to `crate::artifacts::forms::schema::<X>` (mechanical, verified: every occurrence read
before substitution, no shadow exists at the `schema` shim — it re-exports the family-root file's own
items, distinct from the extern `semio_framework_schema as schema` alias because these call sites are
always fully qualified from `crate::`, never bare `schema::`). Files touched:
`🎭️modes/📝️blueprint/🪟️windows/▶️try/🦀️component.rs`, `📌️panels/🔍️inspection/🦀️component.rs`,
`🎮️commands/🧪️try/🦀️component.rs`, `🎮️commands/📐️vector/🦀️component.rs`,
`📌️panels/📄️artifact/🦀️component.rs`, `🎮️commands/🔘️option/🦀️component.rs`,
`🎮️commands/📥️import/🦀️component.rs`, `🎮️commands/❓️question/🦀️component.rs`,
`🎮️commands/📃️step/🦀️component.rs`, and the app root `🦀️component.rs` itself (also updated 2 stale
doc-comment mentions of `⚙️engine` that would otherwise mislead future readers, though they didn't
match the `::engine::` structural grep).

`📦️glue.rs`: removed `#[path=".../⚙️engine/🦀️component.rs"] pub mod engine;` (mounted at
`standards::v1::engine`, NOT nested under `subsets::any` — confirmed by reading the mount before
editing) and the shim `pub mod engine { pub use super::standards::v1::engine::*; }`.

### Assertion arithmetic (exact, not eyeballed)

Original engine file (`git show HEAD:<path>`): **10** `#[test]` fns, **22** `assert!`/`assert_eq!`/
`assert_ne!` calls. After: schema file gained **9** tests / **16** asserts; the app file's relocated
`forms_io` test carries the remaining **6** asserts (verified with `awk` isolating that one test body).
9 + 1 = 10 tests ✅. 16 + 6 = 22 asserts ✅. Exact parity, no assertion lost.

### Verification

```
grep -rn "::engine::\|standards::v1::engine\|subsets::any::engine" ✏️s/🔌️plugins/📋️forms   → 0 hits
find ✏️s/🔌️plugins/📋️forms -path "*🗿️artifacts*" -name "⚙️engine" -type d                    → 0 hits
```

Compiler: `RUSTC_WRAPPER="" CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-forms --all-targets`
— compilation never reaches `semio-s-plugin-forms`: it stops upstream at `semio-s-plugin-stdio`:

```
error: couldn't read `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/.../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs`: No such file or directory (os error 2)
    --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs:7024:37
error: could not compile `semio-s-plugin-stdio` (lib) due to 1 previous error
```

**(c) upstream, not (a) mine.** `semio-s-plugin-stdio` fails identically for the unrelated `📕️norm`
family's mesh subset — a dangling `#[path]` mount from another session's concurrent mutation-vocabulary
rename, explicitly documented in this ticket's own `📌️important.md` (*"stdio REGRESSED"* section) and
matching that exact error text verbatim. `✏️s/🔌️plugins/🗄️stdio/**` is off-limits per the hard rules —
not touched. Structural verification (the two greps above) is the only thing provable right now;
compile status is **complete but UNVERIFIED**, to be re-taken once stdio goes green.

## `🎪️demonstrator`

### What changed

Engine files: `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`
(142 lines) + a stray empty `🦀️component.ts` (2 lines) — DELETED (directory removed).

Destinations:
- `empty_playground_snapshot()` → new `🔖️DocumentHelpers` region in
  `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`.
- `PlaygroundArtifactEngine` struct → **DELETED OUTRIGHT**. Zero external references (only its own
  file). Its one test, `engine_owns_real_artifact`, called `engine.artifact()`/`.snapshot()` after
  `use protocol::ArtifactEngine;` — **this trait does not exist anywhere in the repo**
  (`grep -rn "trait ArtifactEngine" . ` excluding `target/` → 0 hits, confirmed independently, not
  assumed from the ticket's own claim). This test was **already non-compiling dead code before this
  packet touched it** (an unresolved-import + no-such-method double break); it is deleted along with
  the struct it tests, not "a passing assertion lost." Flagged loudly per the ticket's own instruction
  to report — not silently — any case that doesn't match the default fossil pattern; here it *does*
  match (zero construction sites), the only anomaly is the pre-broken test.
- `io_registry` module → new `🚪️DerivedIoRegistry` region in
  `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`.

No shadow at this artifact root (confirmed against the ticket's own shadow census: `🎪️demonstrator` is
the one plugin listed "no" shadow present) — `declaration()`'s `.composers(...)` call was repointed
directly, no wrapper alias to fix.

### Every unqualified path qualified

3 external call sites rewritten from `crate::artifacts::playground::engine::empty_playground_snapshot()`
to `crate::artifacts::playground::schema::empty_playground_snapshot()`:
`🧬️schema/📸️snapshot/📝️text/🦀️component.rs:31`, `🧬️schema/🔺️diff/📝️text/🦀️component.rs:72`,
`🧬️schema/🧬️mutations/🦀️component.rs` (its `use ... engine;` import repointed to `use ... schema;`,
call site updated to `schema::empty_playground_snapshot()`). `📦️glue.rs`: removed the
`#[path] pub mod engine;` mount (at `standards::v1`) and the `pub mod engine { pub use super::standards::v1::engine::*; }`
shim; also reworded its own crate-doc line that listed "...schema/snapshot/diff/dsl/op/spr/engine)"
(the literal substring wasn't `::engine::` so it wasn't a structural-grep hazard, but was stale and
worth fixing) and the plugin-root `🦀️component.rs`'s docstring that named the old
`crate::artifacts::playground::engine::register()` (same treatment).

### Assertion arithmetic

Original engine file: **2** tests / **3** asserts (`empty_snapshot_matches_schema`: 1 assert;
`engine_owns_real_artifact`: 2 asserts, on the already-non-compiling struct/trait pair above). After:
schema file gained **1** test / **1** assert (`empty_snapshot_matches_schema`, verbatim). Net: the one
live, compiling test/assertion survives exactly; the one dead, non-compiling test (2 asserts against a
nonexistent trait) is removed along with the struct it exercised.

### Verification

```
grep -rn "::engine::\|standards::v1::engine\|subsets::any::engine" ✏️s/🔌️plugins/🎪️demonstrator   → 0 hits
  (2 unrelated hits remain and are OUT OF SCOPE, both other plugins' engines referenced from
   demonstrator's panes: 🎪️panes/🗺️verfolgen/🦀️component.rs uses `gis::artifacts::gismap::engine::...`,
   🎪️panes/📐️koordinator/🦀️component.rs uses `cad::artifacts::cad::engine::...` — neither is
   demonstrator's own `🎪️playground` artifact, neither is in this packet's 5 target plugins)
find ✏️s/🔌️plugins/🎪️demonstrator -path "*🗿️artifacts*" -name "⚙️engine" -type d                → 0 hits
```

Compiler: `RUSTC_WRAPPER="" CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-demonstrator --all-targets`
— **identical upstream blocker**, compilation stops at `semio-s-plugin-stdio` with the same
`📄set-snapshot/↩️inverse` dangling-mount error quoted above. **(c) upstream, not mine.** Structural
verification only; compile status **complete but UNVERIFIED**, pending stdio.

## `🎥️shooting`

Dispatched to a sub-agent with the full region map, the shadow-trap location (artifact root's own
`io_registry` wrapper + its `use ... engine::io_registry as v1;`), and the exact glue.rs mount shape,
all pre-derived by me before dispatch. Independently spot-checked its diff after completion (declaration
call site, glue.rs mount/shim removal, `ShootingEngine` deletion) rather than trusting the summary at
face value.

### What changed

Engine file: `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`
(604 lines) — DELETED (directory removed).

Destinations:

| Symbol | Destination |
|---|---|
| `ShootingEngine` struct + impl | deleted outright — verified zero external references |
| `shooting_io`, `shooting_photos_out_port`, `shooting_photo_media` | `🎛️apps/🎥️shooting/🦀️component.rs` (new `🔖️Io` region) |
| `next_shooting_id`, `default_snapshot`, `default_snapshot_json`, `active_shot`, `active_asset`, `is_transparent_shooting_background` | schema `🦀️component.rs`, `🔖️DocumentHelpers` |
| `shooting_scene_to_semio_drawing`, `shooting_shape_path_segments`, `shooting_hex_color_to_rgba`, `shooting_base64_decode`, `shooting_ensure_semio_drawing_bridge_registered`, `shooting_drawing_to_svg_text`, `shooting_scene_svg`, `shooting_document_json_to_svg`, `shooting_icon_render_request_json` | schema `🦀️component.rs`, `🔖️MediaExport` |
| `shooting_document_json_from_dwg` | schema `🦀️component.rs`, `🔖️MediaImport` |
| `io_registry` module (9 export composer entries + `entries()`) | `🚪️io/🦀️component.rs`, new `🚪️DerivedIoRegistry` region |

### Shadow trap — handled (independently verified, not just trusted)

I re-read the artifact root myself after the sub-agent finished:
```
115: /// argument is qualified to `standards::v1::subsets::any::io::io_registry::entries()` (the `⚙️engine`
121: pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
125:     .composers(crate::artifacts::shooting::standards::v1::subsets::any::io::io_registry::entries())
513: pub mod io_registry {
516:     use crate::artifacts::shooting::standards::v1::subsets::any::io::io_registry as v1;
```
Both the `declaration()` call and the wrapper's internal alias correctly repoint at the new location,
fully qualified. `grep -n "engine" 📦️glue.rs` → 0 hits (mount and shim both removed, confirmed).

### Every unqualified path qualified

24 files under `🎛️apps/🎥️shooting/**` rewritten from `artifacts::shooting::engine::X` to
`artifacts::shooting::schema::X` (or, for `shooting_io`/`shooting_photos_out_port`/
`shooting_photo_media`, to the now-local unqualified names inside the app's own file).

### Assertion arithmetic

Original engine Tests region: **10** `#[test]` / **34** asserts (16 `assert!` + 18 `assert_eq!`). After:
schema **7** tests / **17** asserts, app **3** tests / **17** asserts. 7+3=10 ✅, 17+17=34 ✅.

### Verification

```
grep -rn "::engine::\|standards::v1::engine\|subsets::any::engine" ✏️s/🔌️plugins/🎥️shooting
  → 1 hit, false positive (a DIFFERENT plugin's real engine, not this ticket's target):
    🧬️schema/🦀️component.rs:7: use semio_s_plugin_stdio::artifacts::semio::standards::v1::engine::geometry::{...}
find ✏️s/🔌️plugins/🎥️shooting -path "*🗿️artifacts*" -name "⚙️engine" -type d   → 0 hits
```
Compiler: same upstream blocker as forms/demonstrator — `could not compile semio-s-plugin-stdio`,
identical `📄set-snapshot/↩️inverse` dangling-mount error. **(c) upstream, not mine.** As a fallback the
sub-agent ran `rustfmt --edition 2021 --check` on all 24 touched files — all parsed cleanly (only
pre-existing formatting diffs, no syntax errors), giving partial confidence beyond the structural greps
while `cargo check` itself stays blocked. Compile status: **complete but UNVERIFIED**, pending stdio.

## `💠️lowpoly`

Dispatched to a sub-agent (section appended below on completion).
