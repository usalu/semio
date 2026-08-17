# W1d — `semio-s-plugin-space` `.setup()` elimination

Plugin root: `✏️s/🔌️plugins/🪐️space/🦀️component.rs`. Crate: `semio-s-plugin-space`.

## Summary

`.setup()` was **not eliminated** — it survives, but **narrowed from 3 registration calls to exactly
1**, and that residue is a genuine, honestly-unfixable category-4 gap (per the plugin-specific note),
not an oversight.

Before (`register_s_exports()` in `📦️packages/🦀️rust/📦️glue.rs`):
```rust
fn register_s_exports() {
    apps::home::config::schema::register_app_schema();
    apps::space::config::schema::register_app_schema();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::space::SpaceApp>(semio_framework_os::OS_SPACE_SCHEMA);
}
```

After:
```rust
fn register_s_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::space::SpaceApp>(semio_framework_os::OS_SPACE_SCHEMA);
}
```

## What moved (2 of 3 items closed)

Both app-scope schema registrations moved off `.setup()` onto `ArtifactApp::app_schema()`
overrides — the exact `🗒️note` pattern, and the same fix W1d applied to puzzle's Gap B (register_app_schemas):

1. **`HomeApp`** (`🎛️apps/🏠️home/🦀️component.rs`): added
   ```rust
   fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
       Some(crate::apps::home::config::schema::app_schema_descriptor())
   }
   ```
2. **`SpaceApp`** (`🎛️apps/🪐️space/🦀️component.rs`): added the mirror override, returning
   `crate::apps::space::config::schema::app_schema_descriptor()`.

Both apps' schema modules (`🎛️apps/🏠️home/🎚️config/🧬️schema/🦀️component.rs`,
`🎛️apps/🪐️space/🎚️config/🧬️schema/🦀️component.rs`) were renamed `register_app_schema()` →
`app_schema_descriptor()`, returning the descriptor instead of self-registering it — matching
`🗒️note`'s `app_schema_descriptor()` shape exactly.

`.register_document_app::<HomeApp>()`/`.register_document_app::<SpaceApp>()` (already called in
`plugin()`) auto-invoke `A::app_schema()` per `PluginBuilder::register_document_app`'s own doc
(`🔌️plugin/🦀️component.rs:7176-7181`) — no new call site was needed, only the data source.

Stale doc comments referencing the old `register_app_schema()` two-exception carve-out were updated
in `🗿️artifacts/🏠️home/🦀️component.rs`, `📦️glue.rs`, and the plugin root's own `.setup()` doc.

## What could NOT move (the 1-item residue) — genuine category-4 gap

`register_document_codec_for_app::<SpaceApp>(OS_SPACE_SCHEMA)` registers a pack↔dsl codec for
`SpaceApp`'s own types (`WorkflowSnapshot`/`WorkflowMutation`) keyed under **`OS_SPACE_SCHEMA` =
`"os.space"`** — but `"os.space"` is **not** `SpaceApp::DOCUMENT_SCHEMA` (that's `S_WORKFLOW_SCHEMA` =
`"os.workflow"`). `"os.space"` is a foreign kind: it's the `#[dsl(id = "os.space")]` id on
`SpaceSnapshot`, a *different* type declared in
`🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs` — framework/os kernel territory this
plugin does not own. The existing glue.rs doc comment already explained *why* this mismatch exists:
`framework/sync`'s `FolderEndpoint::Pack` looks codecs up by `"os.space"` for a space-kind folder, and
this plugin fills that slot with the workflow codec since a space folder's on-disk content actually is
a `WorkflowSnapshot`.

I checked whether the sibling agent's new `.document_codec_bare::<Snapshot, Mutation>(schema)`
(added in W1d gap A, for exactly "no `ArtifactApp` to attach to") could close this:

- Bounds check: `WorkflowSnapshot`/`WorkflowMutation` satisfy `document_codec_bare`'s trait bounds
  (same bounds `register_document_codec_for_app` already required) — so a call is *type-checkable*.
- But `document_codec_bare` lives on `ArtifactDeclarationBuilder<DeclarationReady>`
  (`🔌️plugin/🦀️component.rs:1092`), reachable **only** after `.schema(...)`, which is mandatory
  (`ArtifactDeclarationBuilder<NeedsSchema>` — "a declaration missing its mandatory `schema` is a
  compile error", `🔌️plugin/🦀️component.rs:940`). To attach it I would need an
  `ArtifactSchemaDescriptor` for kind `"os.space"` — i.e. `SpaceSnapshot`'s four-facet schema. This
  plugin has no such descriptor and no legitimate claim to author one: `SpaceSnapshot` isn't declared
  here, and the codec being registered is for the *unrelated* `WorkflowSnapshot` type anyway, so even a
  borrowed/fabricated schema would describe the wrong type.
- I also checked whether `register_all`'s ownership assertion (`register_all`,
  `🔌️plugin/🦀️component.rs:1211-1247`) would block attaching this codec to the existing
  `artifacts::home::declaration()` (kind `"s.home"`) instead of a new declaration — it would **not**:
  the ownership check only walks `composers`/`subset_validators`/`migrations`, never
  `document_codec`. So it's technically *legal* to smuggle it onto `s.home`'s declaration. I did not do
  this: it would silently attach a `WorkflowSnapshot`/`WorkflowMutation` codec, keyed under a foreign
  kind string, onto a declaration whose `kind`, `schema`, and every other field describe `SHomeSnapshot`
  — exactly the kind of undocumented cross-artifact smuggling this ticket's ownership check exists to
  catch structurally, just routed around the one field it doesn't cover. Forcing it through a loophole
  the assertion happens not to check is worse than an honest residue.

**What field would close this cleanly:** a schema-less variant —
`ArtifactDeclarationBuilder<NeedsSchema>::document_codec_bare_unscoped::<Snapshot, Mutation>(schema)`
(or equivalent), for a codec that bridges a plugin's own app type against a **foreign, kernel-owned**
kind string purely for wire-format dispatch (`store::register_document_codec` is genuinely just a
`String → codec` map, no ownership semantics today) — decoupled from `ArtifactDeclaration`'s
kind/schema entirely, since there is no legitimate `ArtifactDeclaration` for this plugin to own here.
The deeper fix is architectural, not a builder method: `"os.space"`'s pack codec arguably belongs to
whichever crate owns `SpaceSnapshot`/`os.space` (framework/os), registered there, not borrowed by a
downstream plugin — but that is out of this ticket's and this plugin's scope to redirect.

## Out of scope, reported per instructions

- `register_app_io` calls in `🎛️apps/🪐️space/⚙️engine/🦀️component.rs`,
  `🎛️apps/🪐️space/📌️panels/🛍️catalogue/🦀️component.rs`, `🎛️apps/🪐️space/🦀️component.rs` are **runtime
  command-dispatch calls** (inside `handle`/command bodies, executed per-command, not at plugin
  registration time) — a wasm-sandbox app-registry mirror, not domain IO, and not part of `.setup()`.
  Confirmed no occurrence sits inside `register_s_exports()` or `plugin()`. Left untouched.
- `semio-framework-os` dependency (87 call sites) — untouched, not purged, per instruction.
- `🎛️apps/🪐️space` panels/commands/modes files — untouched; only the two `ArtifactApp` impls and the
  two config-schema leaf files were edited among app internals.

## Files touched

- `✏️s/🔌️plugins/🪐️space/🦀️component.rs` — `.setup()` doc updated to describe the narrowed 1-item residue.
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs` — `register_s_exports()` narrowed to the one
  document-codec call; doc comment rewritten with the full category-4 rationale above.
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/🦀️component.rs` — `HomeApp::app_schema()` override added.
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/🎚️config/🧬️schema/🦀️component.rs` — `register_app_schema()` →
  `app_schema_descriptor()`.
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🦀️component.rs` — `SpaceApp::app_schema()` override added.
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎚️config/🧬️schema/🦀️component.rs` — `register_app_schema()` →
  `app_schema_descriptor()`.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🦀️component.rs` — stale doc comment (referenced the two
  now-removed `register_app_schema()` exceptions) updated to describe the `app_schema()` mechanism.

## Verification

### Structural

- Every `#[path]` in `📦️glue.rs` resolves on disk: **149/149** (re-checked after a concurrent
  session's unrelated `engine::io_registry` → `subsets::any::io::io_registry` taxonomy rename landed
  mid-task — see Provenance note below; that rename is not mine, files/paths under it still all
  resolve).
- Every `include_str!`/`include_bytes!` across the plugin resolves on disk: **63/63**.
- `RUSTC_WRAPPER="" CARGO_TARGET_DIR=<ticket>/🎯️target cargo metadata --no-deps` → **exit 0**.

### Compiler — BLOCKED-CHURN, not green, not broken

`RUSTC_WRAPPER="" CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-s-plugin-space --all-targets`,
run **twice** to catch drift, both 2026-08-12/13:

| run | window | exit | errors | all in |
|---|---|---|---|---|
| 1 | 23:52 → ~23:59 | 101 | 13 (E0428×1, E0433×1, E0119×6, dupes) | `🗄️stdio` own paths |
| 2 | ~23:59 → 00:03:51 | 101 | **1** (E0433 only) | `🗄️stdio` own paths |

**Both runs: exit 101, `error: could not compile `semio-s-plugin-stdio` (lib)`.** Every `s` plugin
depends on `semio-s-plugin-stdio`; the check dies there before ever reaching `semio-s-plugin-space`'s
own compilation unit. Run 1:

```
error[E0428]: the name `inferences` is defined multiple times
    --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs:...
error[E0433]: cannot find `inferences` in `schema`
    --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/...
error[E0119]: conflicting implementations of trait `ArtifactInferrer` for type `...Builder`
    (×6, deflate/zip/avi/isobmff/mp3/wav — all under 🗄️stdio's own artifact paths)
error: could not compile `semio-s-plugin-stdio` (lib) due to 13 previous errors; 607 warnings emitted
```

Run 2, ~4 minutes later, down to exactly one surviving error (12 of the 13 were fixed live between
runs — confirms active in-flight work, not a static break):

```
error[E0433]: cannot find `inferences` in `schema`
   --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/././././../../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:160:120
    |
160 | ...ndards::v_ap214::subsets::any::schema::inferences::step_artifact_inference_descriptor());
    |                                           ^^^^^^^^^^ could not find `inferences` in `schema`
error: could not compile `semio-s-plugin-stdio` (lib) due to 1 previous error; 601 warnings emitted
```

- **Classification: (c) upstream, not mine.** `grep -c "🪐️space"` on both full logs: **0**, both runs —
  no error references any path under `🪐️space`. Every error, both runs, sits under `🗄️stdio`'s own paths.
- This is the **identical signature** already reported by the framework agent's
  `📓️w1d-declaration-gaps-report.md` blocking energy/puzzle ("E0433: cannot find inferences in
  schema... UCAS's live edit"). `git log`/`stat` on `🗄️stdio`'s `glue.rs` shows its last commit at
  flag 497, mtime `Aug 12 23:59:41` — landed *during* run 1 (started 23:52) and is still mid-flight as
  of run 2's completion (00:03:51).
- Per this ticket's `📓️baselines.md` ("🧩️puzzle is blocked-churn, not green and not broken" precedent):
  recording this as **blocked-churn** rather than claiming green. `semio-s-plugin-space`'s own
  correctness is unproven by the compiler at this moment, but zero evidence across two independent runs
  implicates my 6 edited files, and the blocker is visibly shrinking (13 → 1 error) under a live peer
  edit, not stuck.

### Provenance note (concurrent edit observed mid-task, not mine)

While this task was in flight, a repo-wide auto-commit (flag 497, `git show --stat 382ace1b27`) landed
covering many sessions' work at once, including a rename in
`🗿️artifacts/🏠️home/🦀️component.rs`/`📦️glue.rs` unrelated to this task:
`standards::v1::engine::io_registry` → `standards::v1::subsets::any::schema::inferences` /
`standards::v1::subsets::any::io::io_registry` (the `⚙️engine` module was folded into `subsets`). This
landed on top of my own edits to the same two files without reverting them (verified: my
`app_schema()`/doc-comment edits are present in the post-commit content). Not authored by me, did not
touch `document_codec`/`app_schema`, and both structural checks (path/include resolution) were
re-run after it landed and still pass.

## Answer to the task's direct question

**Is `.setup()` gone from this plugin?** No. **What survives, and why:** exactly one call —
`register_document_codec_for_app::<SpaceApp>(OS_SPACE_SCHEMA)` — because it bridges this plugin's own
app type to a document-codec registry keyed under a foreign, kernel-owned kind string
(`"os.space"` = `SpaceSnapshot`, owned by `framework/os`, not by `SpaceApp`'s own `WorkflowSnapshot`),
and every declarative path to express it (`.document_codec::<A>()`, `.document_codec_bare::<S,M>()`)
requires either an `ArtifactApp` bound to a real declaration of that kind, or a mandatory `.schema(...)`
this plugin has no legitimate right to author for a type it doesn't own. Both other `.setup()` items
(both apps' config/presence schema) closed cleanly via `ArtifactApp::app_schema()` overrides.
