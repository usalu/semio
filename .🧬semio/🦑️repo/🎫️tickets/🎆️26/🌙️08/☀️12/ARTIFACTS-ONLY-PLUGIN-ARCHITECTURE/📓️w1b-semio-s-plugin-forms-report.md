# W1b — `semio-s-plugin-forms` (`📋️forms`) — `register()` → `declaration()` conversion

`apa-status: partial` — the conversion itself (Step 1–6 of the dispatch) is done, compiler-verified
correct, and introduces zero new errors. The crate does not reach a clean `cargo check`, but every
remaining error is pre-existing, unrelated to this ticket, and outside `📋️forms`' mutation-semantics
territory (SMO's, not APA's) — see "Pre-existing errors found (not fixed)" below.

## Step 0 — clearance

Read `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md`.
`📋️forms` appears in neither the RELEASED tables nor the HELD list — per that file's own explicit
"ABSENCE FROM THIS FILE MEANS FREE" rule, proceeded.

## Step 1 — `register()` → `declaration()`

File: `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`

- Replaced `pub fn register()` (old :34-42) + `pub fn register_pilot_languages()` (old :45-96) with
  `pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration` (new `🔖️Register` region) +
  a private `fn pilot_languages() -> &'static [dsl::LanguageSpec]` helper (`OnceLock`-backed, mirrors
  note's exemplar exactly — `dsl::passthrough_hooks` isn't `const fn`).
- One declaration for the one artifact/subset this plugin owns (`s.forms`, standard `1`, subset
  `any`) — forms has exactly one `register()` to fold in, no multi-standard branching needed.
- Field mapping (`.builder("s.forms")` — matches the `FORMS_DIALECT`/`NOTE_DIALECT`-style constant
  already used in this file's own `io_registry`, e.g. line ~401):
  - `.schema(...)` ← `crate::artifacts::forms::schema::forms_artifact_schema_descriptor()` (same call
    the deleted `register_artifact_schema()` made).
  - `.inferences([...])` ← `crate::artifacts::forms::standards::v1::subsets::any::schema::inferences::forms_artifact_inference_descriptor()`
    (same call the deleted `register_artifact_inference()` made).
  - `.composers(...)` ← `crate::artifacts::forms::standards::v1::engine::io_registry::entries()` — **note:**
    my first draft wrote `...::v1::subsets::any::engine::io_registry::entries()` (copying the schema
    path's shape) and the compiler caught it immediately (`E0433: cannot find engine in any`) — per
    `📦️glue.rs:37-38`, `engine` is mounted directly at `standards::v1::engine`, a sibling of
    `subsets`, not nested inside `subsets::any`, exactly mirroring note's own mount. Fixed before the
    final verification run (see "Verification" below — this was the one real bug in my draft, caught
    by the compiler and corrected, not left in).
  - `.languages(pilot_languages())` ← the 5 `dsl::register_language(...)` calls (`forms.forms`,
    `forms.forms.op`, `forms.forms.diff`, `forms.pack`, `forms.spr`), byte-identical `LanguageSpec`
    values, now returned as data instead of called imperatively.
  - `.document_codec::<crate::apps::forms::FormsPlayApp>()` ← the old
    `semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<FormsPlayApp>(FORMS_DOCUMENT_SCHEMA)`
    call.
- **Deleted** the now-orphaned `🔖️SchemaRegistry` region (`register_artifact_schema()` /
  `register_artifact_inference()`, old :379-391) — confirmed zero remaining call sites repo-wide
  before deleting (same treatment note's own report gave its equivalent free functions).
- `crate::artifacts::forms::io_registry::register()` (the OLD top-level derived registry's own
  `register()`, at `🗿️artifacts/📋️forms/🦀️component.rs:93-95`, which just called
  `register_composer_entries(v1::entries())`) is now **orphaned** (zero call sites) — left in place,
  not deleted, exactly matching note's own precedent for its equivalent orphaned `io_registry`
  module (w1 report: "left in place rather than deleted, since removing it is unrelated cleanup
  outside this wave's scope"). Flagged here for whoever next touches `📋️forms`' artifact root.

## Step 2 — plugin root wiring

File: `✏️s/🔌️plugins/📋️forms/🦀️component.rs`

```rust
Plugin::builder("forms")
    .label("Forms")
    .version("0.1.0")
    .setup(crate::apps::forms::config::schema::register_app_schema)
    .artifact(crate::artifacts::forms::engine::declaration())
    .register_document_app::<crate::apps::forms::FormsPlayApp>(crate::apps::forms::create_forms_app())
    .build()
```

`.setup(crate::artifacts::forms::engine::register)` replaced by `.artifact(crate::artifacts::forms::engine::declaration())`.

### `.setup()` survives — exactly one call, and why

`.setup(crate::apps::forms::config::schema::register_app_schema)` — registers `FormsPlayApp`'s own
CONFIG/PRESENCE schema. This is app-scope, not artifact-scope: `ArtifactDeclaration` deliberately has
no field for `register_app_schema_descriptor` (see `w1-mechanism-report.md` §"Exhaustive
declaration-field ↔ registration-function mapping" — it's one of the two functions the mechanism
names as loudly-excluded-by-design, not silently dropped). Identical treatment to note's own
exemplar. No other `.setup()` call exists or was needed.

## Step 3 — plugin root closure

Already closed before this ticket touched it: `✏️s/🔌️plugins/📋️forms/` contains only `🦀️component.rs`,
`🎛️apps`, `🗿️artifacts`, `📦️packages` — no `🛂️manifest/`, `🎟️capabilities/`, or `🔧️setup/` dirs, no
`AGENTS.md`/`README.md` present either. Nothing to relocate or delete.

## Step 4 — escape hatches and deps

- `grep -rln "register_mesh_\|register_solid_\|register_dwg_\|register_app_io\|register_os_media_"` and
  `grep -rln "semio_framework_os::"` over the whole plugin: **zero hits**. Nothing to relocate, no
  duplicate IO to delete.
- `📦️packages/🦀️rust/Cargo.toml` depends on `semio-framework-os-kernel` (as `dsl`/`protocol`/`store`)
  and `semio-framework-os-flow` (as `flow`), **not** `semio-framework-os` — nothing to purge.

## Step 5 — inventory (report only, not fixed)

- `thread_local!`: **zero** hits anywhere in the plugin.
- `std::fs::`/`std::env::`/`std::process::`/`Command::new` outside `#[cfg(test)]`: **zero** hits.
- Interior-mutable statics (`static … OnceLock/OnceCell/Mutex/RefCell/Atomic/Once`): 4 hits, all lazy
  memoized **derived data caches**, none a host/engine handle:
  - `🗿️artifacts/📋️forms/🦀️component.rs:79` — `OnceLock<Vec<&'static ComposerEntry>>`, the old
    top-level derived `io_registry::entries()` (now orphaned, see Step 1).
  - `.../⚙️engine/🦀️component.rs:52` (new) — `OnceLock<Vec<dsl::LanguageSpec>>` backing this
    ticket's own `pilot_languages()`.
  - `.../⚙️engine/🦀️component.rs:213` — `AtomicU64` id-generator counter (`create_form_id`), a
    process-unique-id sequence, not app/document state.
  - `.../⚙️engine/🦀️component.rs:386` — `OnceLock<Vec<ComposerEntry>>` backing the live
    `io_registry::entries()` this declaration's `.composers()` now points at.
  - None hold a host/engine handle (no `OnceLock<SomeEngineHost>` pattern) and none are a draft-lane
    mutable cache masquerading as app state — all four are pure, immutable-after-first-build derived
    data, same class as note's own `pilot_languages`/`ENTRIES` statics.

## Step 6 — verification

**1. `#[path]` resolution in `📦️glue.rs`** — scripted (Python, resolves each `.rs`-targeting `#[path]`
relative to `📦️glue.rs`'s own directory; the `#[path = "."]` grouping-only entries, documented at the
top of that file, are correctly excluded from file-resolution since they mark directories, not files):
```
Total #[path] entries: 144
Total .rs-file #[path] entries: 79
Missing: 0
```

**2. `include_str!`/`include_bytes!` resolution** — scripted, resolved against each call site's own
containing file's directory (never pattern-substituted):
```
Total include_str!/include_bytes! call sites: 49
Missing: 0
```

**3. `cargo metadata --no-deps --format-version 1`**: `OK` (empty stderr, exit 0) — workspace
manifests intact.

**4. `cargo check -p semio-s-plugin-forms --all-targets`** (`RUSTC_WRAPPER=""`,
`CARGO_TARGET_DIR=".../🎯️target"`) — 4 attempts, following the note exemplar's retry-and-wait
protocol for concurrent workspace churn:

- **Attempt 1**: caught my own bug — `error[E0433]: cannot find engine in any` at the `.composers(...)`
  call (wrong module path, see Step 1 above). Fixed immediately.
- **Attempt 2**: `error: couldn't read .../📄set-snapshot/↩️inverse/🦀️component.rs: No such file or
  directory` inside `semio-s-plugin-stdio`'s own `📦️glue.rs` — a dangling `mod` from another session's
  in-flight file deletion (SMO's `SetSnapshot` retirement, per this repo's own standing note never to
  reintroduce that name under `✏️s/`). Zero mentions of any `📋️forms` path. Not mine; retried.
- **Attempt 3**: different stdio-side failure — 6 errors, `SemioDrawingMutation: OpText`/`OpBinary` not
  satisfied, inside `semio-s-plugin-stdio`'s `drawing` subset. Again zero mentions of `📋️forms`. Not
  mine; retried.
- **Attempt 4**: `semio-s-plugin-stdio` compiled clean, exposing `semio-s-plugin-forms`' **own**
  state — **14 real errors, all pre-existing, none in code this ticket touched**:
  ```
  error: could not compile `semio-s-plugin-forms` (lib) due to 14 previous errors; 14 warnings emitted
  error: could not compile `semio-s-plugin-forms` (lib test) due to 14 previous errors; 14 warnings emitted
  ```

**0 errors attributable to this ticket's diff** (`git diff --stat` for this session: exactly 2 files —
`⚙️engine/🦀️component.rs` and the plugin root `🦀️component.rs`; neither appears in any of the 14
remaining errors' locations).

## Pre-existing errors found (not fixed) — SMO's mutation-facet territory

12 `E0599` "no variant named X found for enum FormMutation" errors, in 3 command files:

| file | line | missing variant |
|---|---|---|
| `🎛️apps/📋️forms/🎮️commands/📥️import/🦀️component.rs` | 20 | `RemoveStep` |
| `🎛️apps/📋️forms/🎮️commands/📥️import/🦀️component.rs` | 22 | `UpdatePlaybook` |
| `🎛️apps/📋️forms/🎮️commands/📥️import/🦀️component.rs` | 25 | `AddStep` |
| `🎛️apps/📋️forms/🎮️commands/❓️question/🦀️component.rs` | 203 | `AddBlock` |
| `🎛️apps/📋️forms/🎮️commands/❓️question/🦀️component.rs` | 226 | `RemoveBlock` |
| `🎛️apps/📋️forms/🎮️commands/❓️question/🦀️component.rs` | 283 | `MoveBlock` |
| `🎛️apps/📋️forms/🎮️commands/❓️question/🦀️component.rs` | 312 | `AddBlock` |
| `🎛️apps/📋️forms/🎮️commands/📃️step/🦀️component.rs` | 21 | `AddStep` |
| `🎛️apps/📋️forms/🎮️commands/📃️step/🦀️component.rs` | 48 | `UpdateStep` |
| `🎛️apps/📋️forms/🎮️commands/📃️step/🦀️component.rs` | 72 | `RemoveStep` |
| `🎛️apps/📋️forms/🎮️commands/📃️step/🦀️component.rs` | 92 | `MoveStep` |
| `🎛️apps/📋️forms/🎮️commands/📃️step/🦀️component.rs` | 108 | `UpdatePlaybook` |

The actual `FormMutation` enum (`🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:32-43`)
already carries the **new, triad-shaped** variant set — `CreateStep`, `DeleteStep`, `ReorderStep`,
`RenameStep`, `ChangeStepDescription`, `CreateBlock`, `DeleteBlock`, `MoveBlockToStep`, `ReplaceBlock`,
`ChangeFormTitle` — but these 3 command files still construct the **old** whole-doc-style variant
names. This is SMO's own semantic-mutations triad conversion, landed on the enum but not yet
propagated to its command call sites — squarely SMO's territory (per this repo's own standing
constraint never to touch `🧬️mutations`-adjacent semantics casually, and per the SMO ledger's own
definition of "RELEASED" requiring both the facet *and* its call sites to be migrated). I did not
guess at the mapping: the compiler's own "similar name" suggestions are demonstrably wrong in at
least one case (`RemoveStep` → suggested `RenameStep`, an unrelated operation) — a mechanical fix here
would risk landing incorrect behavior, not just a compile fix.

2 `E0308` "mismatched types" errors, both `JsonValue`/`serde_json::Value` mismatches:

| file | line | detail |
|---|---|---|
| `🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` | 9 | expected `JsonValue`, found `Value` |
| `🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` | 9 | expected `Value`, found `JsonValue` |

Both bridge forms' own `FormsSnapshot` JSON leaves against stdio's `JsonSnapshot.value` field, whose
type appears to have changed under a newtype (`JsonValue`) as part of the same UCAS/stdio roster
restructure the SMO ledger describes as "not frozen." Not touched — stdio's roster is not mine to
alter, and correcting the wrapper direction without knowing the intended `JsonValue` shape risks
guessing wrong.

All 14 are confirmed pre-existing (not introduced by this session): the erroring files' mtimes
(`10:50`/`11:08` local) are hours before this session's work, they are already committed
(`git log --oneline -3` shows them landed, not staged/in-flight), and none appear anywhere in this
session's own `git diff --stat` (exactly 2 files touched, listed above).

## Files touched

- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` —
  `register()`/`register_pilot_languages()` → `declaration()` + `pilot_languages()`; deleted the
  orphaned `register_artifact_schema()`/`register_artifact_inference()` pair.
- `✏️s/🔌️plugins/📋️forms/🦀️component.rs` — `.setup(engine::register)` → `.setup(register_app_schema)` +
  `.artifact(engine::declaration())`.

Nothing else created, deleted, or moved.

## sharedFileRequests

1. **SMO** — the 12 `FormMutation` variant-name mismatches above (3 files under
   `🎛️apps/📋️forms/🎮️commands/`) need their call sites rewired to the already-landed triad variant
   set. Blocks `semio-s-plugin-forms` from ever reaching a clean `cargo check` until fixed; not
   attempted here (wrong-guess risk, out of APA's scope).
2. **UCAS / stdio roster owner** — the 2 `JsonValue`/`Value` mismatches in forms' own json export/import
   serializer leaves, caused by a type change on stdio's `JsonSnapshot.value` field. Not attempted here
   (stdio's roster is explicitly "not frozen" per the SMO ledger; the correct fix direction depends on
   stdio's own intended shape, which is not APA's to decide).
3. `🗿️artifacts/📋️forms/🦀️component.rs:73-97` (`pub mod io_registry { entries()/compose()/register() }`)
   is now fully orphaned (zero call sites repo-wide) — left in place per note's own precedent; flagged
   for whoever next does general cleanup on `📋️forms`.

No other shared-file requests.
