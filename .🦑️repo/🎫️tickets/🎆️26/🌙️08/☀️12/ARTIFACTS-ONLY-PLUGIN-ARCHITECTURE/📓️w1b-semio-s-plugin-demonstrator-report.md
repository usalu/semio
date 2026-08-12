# W1b — `🎪️demonstrator` (crate `semio-s-plugin-demonstrator`) — `.artifact()` declaration conversion

`apa-status: partial` — Steps 1/2 (the plugin's own artifact, `playground`) converted; static
verification (path mounts, `include!` resolution, `cargo metadata`) is clean; `cargo check
--all-targets` could not be driven to a green result in this session **through no fault found in this
plugin's own diff** — see "Step 6 — verification" for the full evidence trail (7 real attempts, 0 of
which ever implicated a `🎪️demonstrator` path; every failure traced to a *different* sibling plugin
mid-edit by a *different*, concurrently-running W1b session on the exact same shared `🎯️target`).
Steps (a)/(b) inventoried and confirmed already-correct/left-untouched respectively; Step (c) (the
`🎪️panes` non-taxonomy dir) assessed and deliberately NOT restructured this packet — see its own
section below for why and what the plan is.

**Mid-session concurrent edit, reconciled, not reverted**: partway through verification, an automated
pass (not this session) relocated `declaration()` from `⚙️engine/🦀️component.rs` to the artifact-root
file (`🗿️artifacts/🎪️playground/🦀️component.rs`), with a matching doc comment ("Lives at the artifact
root, not `⚙️engine` ... `declaration()` describes the artifact, it is not engine behaviour") and
follow-on edits to `pilot_languages()` (now `pub`, called cross-module), the plugin root's call site,
and two doc-comment cross-references. I verified this landed as a complete, self-consistent rename
(not a partial/broken edit — traced every one of the 4 touched call sites) and left it in place per
this repo's explicit instruction to treat such changes as intentional. **Everything below describes
the artifact's actual current on-disk shape**, i.e. this note, not my original engine-level
placement.

## Clearance

`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md`: `🎪️demonstrator`
is listed under "RELEASED — Wave C / late Wave M lanes complete" (its `🎪️playground` mutation facet,
1 mutation, SMO's own lane) and is **not** in the `HELD` list. Its own "Notes for consumers" section
explicitly assigns the four-artifact-kind IO census and the load-order bug to APA (this ticket), not
SMO, and states SMO "will neither fix nor disturb it." Clear to proceed.

## Changes — Step 1/2: `register()` → `declaration()`, `.artifact()` wired at the plugin root

### `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`

- `pub fn register()` (old :14-20, called `io_registry::register()` + 3 wrapper fns) → deleted. What
  it built now lives one taxonomy level up as `declaration()` (see next file) — moved there partway
  through this session by an automated pass, described in the note above; not reverted.
- `pub fn register_pilot_languages()` (old :23-92, five `dsl::register_language(dsl::LanguageSpec {...})` calls) → five fns
  (`playground_document_language`/`_op_language`/`_diff_language`/`_pack_language`/`_spr_language`,
  each returning `dsl::LanguageSpec` instead of registering it) + `pub fn pilot_languages()` /
  `build_pilot_languages()`, an `OnceLock`-backed `&'static [dsl::LanguageSpec]` — the exact pattern
  `🗒️note`'s own exemplar conversion established (same ticket, W1 report). `pilot_languages()` is
  `pub` (not private, as I originally wrote it) because `declaration()` now lives outside this module
  and calls it by its full path.
- `pub fn register_artifact_schema()` / `pub fn register_artifact_inferences()` (old :107-120,
  `SchemaRegistry` region) — **deleted outright**, not kept as dead wrappers: `declaration()` now
  calls `playground_artifact_schema_descriptor()` / `playground_artifact_inference_descriptor()`
  directly as builder arguments, and grep confirmed zero other call sites for either wrapper
  anywhere in the crate before deleting them.
- This file's own `pub mod io_registry { entries()/... }` (the v1-engine-level registry, returning
  `&'static [ComposerEntry]`) is unchanged — it is what `declaration()`'s `.composers(...)` now reads
  from its new home one level up.

### `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🦀️component.rs` (artifact root — where `declaration()` now actually lives)

New `//#region 🔖️Register` housing:
```rust
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.playground")
        .schema(crate::artifacts::playground::schema::playground_artifact_schema_descriptor())
        .inferences([crate::artifacts::playground::standards::v1::subsets::any::schema::inferences::playground_artifact_inference_descriptor()])
        .composers(crate::artifacts::playground::standards::v1::engine::io_registry::entries())
        .languages(crate::artifacts::playground::standards::v1::engine::pilot_languages())
        .build()
}
```
No `.document_codec()` call: playground owns no `ArtifactApp` — grepped every pane
(`grep -rn PLAYGROUND_DOCUMENT_SCHEMA`) and confirmed no app anywhere in the crate uses it as its
`DOCUMENT_SCHEMA`. No `.setup()` survives at the plugin root either, for the same reason: the one §6
function `ArtifactDeclaration` deliberately excludes is `register_app_schema_descriptor` (app-scope
config/presence schema), and playground has no app to own one.

This same file also had its old `pub mod io_registry { entries()/compose()/register() }` block (old
:31-55, a differently-typed `&'static [&'static ComposerEntry]` wrapper around the v1-engine registry)
**deleted outright** — not kept as a dead wrapper.
  **This is a deliberate deviation from `🗒️note`'s own precedent**, where the W1 report explicitly
  left an equivalent orphaned `io_registry` module in place ("flagged here for whoever next touches
  note"). I chose deletion instead because this module became orphaned **as a direct, sole
  consequence of this exact edit** (its only caller, `engine::register()`, is the very function I
  just deleted three paragraphs above — confirmed by `grep -rln "playground::io_registry"` returning
  zero hits anywhere in the repo once `register()` is gone), not because it was already dead before I
  arrived. CLAUDE.md's standing rule for this repo is unambiguous on exactly this situation ("no
  legacy code," "must not be pragmatic," "must not care about implementation effort," "many
  inconsistencies you must refactor") and it is a stronger, more general instruction than the
  packet's local precedent on one specific prior file. Verified no TypeScript twin exists at this
  taxonomy level (`ls` on the directory shows only `🦀️component.rs`), so nothing else needed updating
  to match.

### `✏️s/🔌️plugins/🎪️demonstrator/🦀️component.rs` (plugin root)

Before:
```rust
pub fn plugin() -> Plugin {
    crate::artifacts::playground::engine::register();
    crate::panes::bundle()
}
```
After (call site tracks `declaration()`'s final artifact-root location):
```rust
pub fn plugin() -> Plugin {
    let plugin = Plugin::builder(PLUGIN_ID)
        .label(PLUGIN_LABEL)
        .version(PLUGIN_VERSION)
        .artifact(crate::artifacts::playground::declaration())
        .build();
    crate::panes::bundle(plugin)
}
```
`PLUGIN_ID`/`PLUGIN_LABEL`/`PLUGIN_VERSION` moved here from `🎪️panes/🦀️component.rs` (where the old
`Plugin::new(...)` call lived) since plugin identity is now decided at the point `Plugin::builder(...)`
is first called, matching `🗒️note`'s own root-file shape.

### `✏️s/🔌️plugins/🎪️demonstrator/🎪️panes/🦀️component.rs`

`pub fn bundle() -> Plugin` (built its own `Plugin::new(...)`) → `pub fn bundle(bundle: Plugin) -> Plugin`
(takes the already-built `Plugin` the root handed it after registering the `playground` declaration,
then layers the six panes' `register_exports()`/`register_app()` calls onto it exactly as before —
order unchanged). This was the one structural change `.artifact()` required here that note's exemplar
never needed: `ArtifactDeclaration::register_all` is `pub(crate)`, reachable only from
`PluginBuilder::build()`, so the declaration MUST flow through `Plugin::builder(...)`, not the bare
`Plugin::new(...)` the pane bundle used before. Tests updated to call a local `test_bundle()` helper
(`bundle(Plugin::new(PLUGIN_ID, PLUGIN_LABEL, PLUGIN_VERSION))`, constants now declared inside
`#[cfg(test)] mod tests` since production identity now lives at the plugin root) — behaviorally
identical assertions, since `Plugin::builder(...).label(...).version(...).build()` and
`Plugin::new(...)` construct the same `plugin_id`/`label`/`version` fields.

### `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`

One-line doc-comment fix (was pointing at the deleted `register()`); current text (post-relocation, see
the note at the top of this report):
`"registration now flows through ArtifactDeclaration::register_all (.composers(engine::io_registry::entries()) on the artifact root's declaration())"`.

## `.setup()` status

**Never called at all, before or after this change.** Demonstrator's root `plugin()` never used
`.setup(...)` — confirmed by `grep -n '\.setup('` across the whole crate returning zero hits both
before and after this edit. There is no app-scope `register_app_schema_descriptor` escape hatch to
keep here either (playground owns no `ArtifactApp`), so unlike `🗒️note` this plugin needed no narrowed
`.setup()` call at all — the conversion is a clean `.artifact()`-only root.

## (a) `"3d.process"` / `"3d.procedural"` — already deleted, verified correct, not my edit

`git log`/`stat -f '%Sm'` on `🎪️panes/🌱️generator/🦀️component.rs` and `🎪️panes/🏭️bearbeiten/🦀️component.rs`
show both files already carry the deletion (their own doc comments cite this exact ticket + a
`📓️w3-semio-s-plugin-demonstrator-report.md` that does not exist in this ticket folder — evidence of
a prior, already-merged session's pass, not mine: `git status --porcelain` shows neither file
modified, i.e. already committed to HEAD). Both panes now register **only**
`register_document_codec_for_app::<A>` — no `register_mesh_*`/`register_dwg_*`/`register_solid_*`
calls remain for either kind. I verified the load-bearing claim the doc comments make (the owning
plugin really does self-register) rather than trusting the comment:

```
🌀️procedural/…/🧊️procedural3d/…/⚙️engine/🦀️component.rs:649:
  semio_framework_os::register_mesh_dwg_import_handler("3d.procedural", procedural3d_document_from_mesh);
🏭️process/…/🧊️process3d/…/⚙️engine/🦀️component.rs:44:
  semio_framework_os::register_mesh_dwg_import_handler("3d.process", process3d_document_from_mesh);
```

Both self-register from their own artifact `⚙️engine`, confirming the doc comments' claim and the
plugin-release-status ledger's note that this is APA's fix, already landed. Nothing further to do
here — no double-registration bug remains for either kind.

## (b) `"2d.map"` (🌍️gis) / `"3d.cad"` (📐️cad) — untouched, per explicit dispatch instruction

`🎪️panes/🗺️verfolgen/🦀️component.rs` (`register_2d_export_handlers`, `register_dwg_import_handler` for
`"2d.map"`) and `🎪️panes/📐️koordinator/🦀️component.rs` (six `register_solid_*`/`register_mesh_*` calls
for `"3d.cad"`) are **left exactly as they were** — confirmed by grep that demonstrator is still the
sole registrant for both kinds (`gis`'s own plugin dir and `cad`'s own plugin dir were not searched
for competing registrations, per the dispatch's explicit "sole registrant" premise, which the
UCAS composition ruling already settled: relocating this code into gis/cad would be deleted by them
shortly since they receive IO via composition in their own work, not bespoke per-plugin serializers).
**Do not delete or relocate these two panes' registrations** — recorded here per the dispatch's
instruction so the next agent does not "helpfully" remove them.

## (c) `🎪️panes/` — non-taxonomy plugin-root dir — assessed, NOT restructured this packet

**Plan, not execution.** Converting `🎪️panes/<name>/🦀️component.rs` (6 dirs) into a proper
`🎛️apps/🎪️demonstrator/<name>/🦀️component.rs` tree was assessed against the canonical app-taxonomy
shape (`🗒️note`'s own `🎛️apps/🗒️note/{🦀️component.rs,🎚️config,📌️panels,👥️presence,🎭️modes,🎮️commands,⚙️engine,🗣️terminology,📚️examples}`)
and against this crate's `📦️glue.rs`. Findings:

1. **The six panes do not fit the canonical app shape at all.** Every subdir a normal app owns
   (config/panels/presence/modes/commands/engine) already lives in the SIX OWNING plugins
   (`🌀️procedural`, `📐️cad`, `🧩️puzzle`, `🪵️sourcing`, `🏭️process`, `🌍️gis`) — each pane file here is a
   ~20-line wiring shim that imports `create_X_app()`/`XPlayApp` from a foreign crate and adds it to
   this bundle. Moving the shim into `🎛️apps/🎪️demonstrator/<name>/` would not gain any of the
   structure the taxonomy convention exists for; it is a pure rename with zero functional or
   organizational benefit for these six files specifically.
2. **Blast radius**: the crate's `📦️glue.rs` has 96 `#[path]` mounts total; the 7 panes-related ones
   (`🎪️panes/🦀️component.rs` + 6 pane files) would all need synchronized `#[path]` rewrites in the
   same shared, concurrently-edited file other live sessions also touch (per this packet's own hard
   rule: "STAY IN YOUR PLUGIN... anything else → sharedFileRequests, then STOP" — `glue.rs` IS inside
   my plugin, but a mid-move mistake there breaks the whole crate for every concurrent dev on this
   tree, and the packet's own words are explicit: "A half-converted app tree is worse than an
   untouched one").
3. **No correctness bug motivates the move** — unlike (a), this is pure taxonomy hygiene, not a fix
   for a live bug. Given (1) and (2), I judged this NOT cleanly-and-completely achievable at low risk
   within this packet's remaining scope and left `🎪️panes/` in place, exactly as the dispatch's own
   fallback instructs.

**Concrete plan for whoever picks this up:**
- Create `🎛️apps/🎪️demonstrator/{🌱️generator,📐️koordinator,🧩️aggregator,🗂️aussuchen,🏭️bearbeiten,🗺️verfolgen}/🦀️component.rs`,
  one per pane, moving each pane file's content verbatim (no logic change — this is a pure relocation).
- Replace the single `🎪️panes/🦀️component.rs` bundling file with `🎛️apps/🎪️demonstrator/🦀️component.rs`
  (currently an empty, unmounted stub — see below), keeping `pub fn bundle(bundle: Plugin) -> Plugin`
  verbatim.
- In `📦️glue.rs`: delete the `//#region 🎪️Panes` block (:287-303) and its 7 `#[path]` mounts; add an
  equivalent `pub mod apps { pub mod demonstrator { ... } }` block with 7 new `#[path]` mounts
  pointing at the new locations, in the same relative-path style every other mount in this file uses.
- Update `✏️s/🔌️plugins/🎪️demonstrator/🦀️component.rs`'s `crate::panes::bundle(plugin)` call to
  `crate::apps::demonstrator::bundle(plugin)`.
- Delete `🎪️panes/` once every file under it is confirmed moved (`find 🎪️panes -type f` empty).
- Verify with the same 4-step Step 6 checklist this report ran (path-mount count unchanged at 96,
  `include!` sweep, `cargo metadata`, `cargo check --all-targets`).
- **Note**: `🎛️apps/🦀️component.rs` at the plugin root already exists as a one-line doc-only stub
  (`"Apps facet for 🎪️demonstrator — document app factories registered via .register_document_app."`)
  and is **not currently mounted anywhere in `glue.rs`** (confirmed by grep) — it is dead, unreachable
  placeholder left by an earlier wave anticipating exactly this restructure. Left untouched (it is
  inside the allowed root-file set per Step 3's own list) but flagged so the next agent knows it is
  not live code and does not need special handling before mounting the real tree there.

## Step 3 — plugin root closure

Root already contains only the allowed set: `🦀️component.rs`, `🎛️apps` (empty/unmounted stub, see
above), `🗿️artifacts`, `📦️packages`, plus the one non-taxonomy holdover `🎪️panes` (assessed in (c)
above, left in place). No `AGENTS.md`/`README.md` exist and none were added, per the plugin-specific
note. No other doc-only facet dirs or stray root data files found.

## Step 4 — escape hatches and deps

`grep -rn "semio_framework_os::register_\|semio_framework::register_\|store::register_\|dsl::register_language(" ✏️s/🔌️plugins/🎪️demonstrator` (excluding the artifact `⚙️engine` file) returns **exactly** the 11
lines inside `🗺️verfolgen`/`📐️koordinator` — i.e. exactly the two kinds (b) names, nothing else. No
other escape-hatch registrar call exists anywhere outside an artifact engine. `semio-framework-os`
stays in `Cargo.toml` (it is not purge-eligible — `grep -rn "semio_framework_os::"` is non-empty,
required by koordinator/verfolgen, which this dispatch explicitly says not to touch).

## Step 5 — inventory (nothing found)

`grep -rn "thread_local!\|std::fs::\|std::env::\|std::process::\|Command::new(" ✏️s/🔌️plugins/🎪️demonstrator`
→ zero hits. Only `static` outside `#[cfg(test)]` is the pre-existing `OnceLock`-backed
`ENTRIES`/`LANGUAGES` caches (io_registry's composer table, `pilot_languages()`) — derived caches of
pure static data, not host/engine handles. Nothing to report.

## Step 6 — verification

1. **`#[path]` resolution**: 96 `#[path = "..."]` attributes in `📦️glue.rs`, resolved every one
   against the real filesystem relative to `📦️packages/🦀️rust` — **0 missing**.
2. **`include_str!`/`include_bytes!` resolution**: 36 macro invocations across the crate, resolved
   every one against its containing file's real directory (Python walk + `os.path.exists`, no
   pattern substitution) — **0 missing**.
3. **`cargo metadata --no-deps --format-version 1`**: `OK` (workspace resolves, output written to
   `scratch-w1b-demonstrator-metadata.txt` in this ticket folder).
4. **`cargo check -p semio-s-plugin-demonstrator --all-targets`**, `RUSTC_WRAPPER=""`,
   `CARGO_TARGET_DIR=".../🎯️target"` — **7 real attempts, full raw output pasted into
   `scratch-w1b-demonstrator-check-{1..7}.txt` in this ticket folder, none doctored.** Every single
   attempt failed before ever reaching `semio-s-plugin-demonstrator` — confirmed each time with
   `grep -c "🎪️demonstrator" <output>` → **0** in all 7 runs. What actually failed, per attempt (this
   ticket is running dozens of W1b conversions on other plugins in parallel right now, all sharing
   this one `CARGO_TARGET_DIR` per the dispatch's own hard rule, so this crate's dependency graph is
   red for reasons entirely outside this plugin):

   | # | what failed | plugin | relation to `🎪️demonstrator` |
   |---|---|---|---|
   | 1 | `couldn't read .../📄set-snapshot/↩️inverse/🦀️component.rs: No such file or directory` | `🗄️stdio` | transitive dep (note's own IO bridge machinery), mid-delete by another session |
   | 2 | `SemioDrawingMutation: OpText`/`OpBinary` not satisfied, `print_op`/`parse_op`/`encode_op`/`decode_op` missing | `🗄️stdio` (drawing subset) | same, different moment of the same in-flight rename |
   | 3 | `defined multiple times` (10 mutation names), unresolved `super::*` imports, `Procedural3dMutation::Generation`/`SetWidget` missing | `🪵️sourcing`, `🌍️gis`, `🏭️process`, `🌀️procedural` | **all four** of demonstrator's other pane dependencies, mid-edit simultaneously |
   | 4 | `MutationKind::SEMANTICS.kind must equal "group-nodes"` (kebab-form panics) | `🗄️stdio` (drawing subset) | same in-flight semantic-mutation ratchet, further along |
   | 5 | `no variant ... Group/Ungroup/Flatten/Unflatten` (20 errors) | `🗄️stdio` (drawing subset) | same ratchet, later still — vocabulary actively being renamed under us |
   | 6 | mutation-name collisions again + `cannot find function pilot_languages in module gisterrain::...engine` + `cannot find function declaration in module puzzle2d/puzzle3d::...engine` | `🌍️gis`, `🧩️puzzle`, `🏭️process`, `🌀️procedural` | **other sessions' own W1b `register()`→`declaration()` conversions, mid-flight, on the exact same mechanism this report used** |
   | 7 | mutation-name collisions again + `cannot find function declaration in module cad::engine` | `🌍️gis`, `📐️cad` | `📐️cad`'s own W1b conversion now also mid-flight (a plugin this report explicitly left untouched) |

   By attempt 6/7, the pattern is unambiguous: **every other plugin `🎪️demonstrator`'s panes depend on
   — `🌀️procedural`, `📐️cad`, `🧩️puzzle`, `🪵️sourcing`, `🏭️process`, `🌍️gis` — is being converted by its
   own concurrent W1b session at the same time as this one**, several mid-edit at the exact moment
   this crate's build reaches them. This is expected, sanctioned concurrent-churn behavior (see this
   session's own memory note on exactly this pattern), not a defect this report introduced: **0 of the
   49 distinct error lines across all 7 attempts names any path under `✏️s/🔌️plugins/🎪️demonstrator`.**
   I did not run an 8th attempt — retrying further only re-samples the same fleet-wide race, and the
   evidence bar (7 attempts, exhaustively grepped, every failure attributed to a *named*, *external*,
   *currently-in-progress* sibling conversion) already exceeds this same ticket's own established
   precedent (`🗒️note`'s W1 exemplar needed 6). **This crate's own `cargo check --all-targets` has not
   yet been observed to complete green in this session — it needs a re-run once the fleet's other
   plugins settle**, most directly `🌀️procedural`, `📐️cad`, `🧩️puzzle`, `🪵️sourcing`, `🏭️process`,
   `🌍️gis`, and `🗄️stdio`.

## sharedFileRequests

None. `📦️glue.rs` edits (the one docstring one-liner in the `🚪️io` component) stayed inside my plugin.
No files outside `✏️s/🔌️plugins/🎪️demonstrator` were touched.

## Files touched

(Final on-disk state, i.e. after the mid-session automated `declaration()` relocation described at the
top of this report — I authored the `register()`→`declaration()` conversion; the artifact-root
placement of `declaration()` itself landed via that separate pass, verified and kept.)

- `✏️s/🔌️plugins/🎪️demonstrator/🦀️component.rs` — `plugin()` rewritten to `Plugin::builder(...).artifact(crate::artifacts::playground::declaration()).build()`, plugin identity constants moved in from panes.
- `✏️s/🔌️plugins/🎪️demonstrator/🎪️panes/🦀️component.rs` — `bundle()` → `bundle(bundle: Plugin) -> Plugin`; tests updated with a local `test_bundle()` helper.
- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🦀️component.rs` — orphaned top-level `io_registry` module deleted; new `declaration()` (authored by the automated relocation pass, not this session, but verified correct here).
- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` — `register()`/`register_pilot_languages()`/`register_artifact_schema()`/`register_artifact_inferences()` → `pub fn pilot_languages()`/`build_pilot_languages()` + 5 named per-language builder fns (`declaration()` itself relocated out of this file, see above).
- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` — 1-line stale doc-comment fix (updated twice: once by me, once by the relocation pass to match the final path).

Nothing created, nothing deleted at the directory level (only the one orphaned `pub mod io_registry`
block deleted from inside an existing file).
