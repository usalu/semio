# W3 — `💠️lowpoly` plugin migration report

Ticket: `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE` (APA), #2549. Plugin: `💠️lowpoly` (crate
`semio-s-plugin-lowpoly`), directory `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/`.

## Clearance

SMO's live predicate (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md`)
lists `💠️lowpoly` under **"RELEASED — Wave C / late Wave M lanes complete"**: *"16 mutations, 1:1
triad dirs, glue rewired, `cargo check` 0 self-owned errors"*. Not HELD anywhere. Proceeded.

## What changed

### 1. `🔧️setup` facet fan-out folded into the artifact `⚙️engine`, `"3d.mesh"` deleted

`✏️s/🔌️plugins/💠️lowpoly/🔧️setup/🦀️component.rs` (25 lines, `pub fn register_lowpoly_exports()`) had
15 `semio_framework_os::register_mesh_*` calls: 7 for `"3d.lowpoly"` (lowpoly's own kind) and 8 for
`"3d.mesh"` (an ownerless kind co-declared by lowpoly, per dispatch: delete, never relocate, never
create a mesh artifact).

**Checked `🚪️io` first, per instruction.** `🗿️artifacts/💠️lowpoly/🦀️component.rs:348-369`
(`pub mod io_registry`) already calls `register_composer_entries(v1::entries())`, and
`🚪️io/🦀️component.rs`'s `LowpolyComposerComposition` (`impl ArtifactComposition`) already declares
`WRITES: Dialect { artifact_kind: "s.lowpoly", .. }` reading/writing every one of the same
DWG/glTF/JSON/LAS/OBJ/PLY/PNG/STL/TXT formats the 7 `"3d.lowpoly"` `register_mesh_*` calls existed
for. **Finding: the 7 `"3d.lowpoly"` calls were a pure duplicate of an existing composer registration
— deleted, not relocated.** The 8 `"3d.mesh"` calls were deleted outright per explicit ticket
instruction (UCAS ruled the kind disappears; stdio's `mesh` subset supersedes it).

The two calls in `register_lowpoly_exports()` that were **not** duplicates —
`crate::apps::lowpoly::config::schema::register_app_schema()` and
`register_document_codec_for_app::<LowpolyPlayApp>(...)` — were folded into
`🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`'s existing
`pub fn register()` (was `:80`, now `:84`, region `🔖️Register`), which already called
`io_registry::register()` + the three `register_pilot_languages/artifact_schema/artifact_inferences`
fns. Verified by grep that `register_app_schema()`/`register_document_codec_for_app::<LowpolyPlayApp>`
and `crate::artifacts::lowpoly::engine::register()` itself each had exactly one prior caller
(the deleted setup facet) — no other call site needed updating.

Plugin root `✏️s/🔌️plugins/💠️lowpoly/🦀️component.rs:10` changed
`.setup(crate::register_lowpoly_exports)` → `.setup(crate::artifacts::lowpoly::engine::register)`,
pointing the (still-imperative, `.setup()`-shaped — M1's declarative `ArtifactDeclaration` replacement
is W1 framework work, not in scope here) registration hook directly at the artifact's own `⚙️engine`
registration fn instead of a plugin-root facet.

### 2. Dead facet directories deleted

- `🔧️setup/` — real code (15-call fan-out), relocated per above, then deleted.
- `🛂️manifest/` — 1-line doc-only stub (`//! 🛂️ Manifest facet ...`), **unmounted** (no `#[path]`
  reference anywhere in `📦️glue.rs` — confirmed by grep before deleting). Deleted outright.
- `🎟️capabilities/` — 1-line doc-only stub, **unmounted**. Deleted outright.

No plugin-root `.DS_Store`/`node_modules` were present (checked, none to remove).

### 3. `📦️glue.rs` mount removed

`✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/📦️glue.rs` — removed the 3-line mount block
(previously :652-654):
```rust
#[path = "../../🔧️setup/🦀️component.rs"]
mod setup;
pub use setup::register_lowpoly_exports;
```
`🛂️manifest`/`🎟️capabilities` had no mount to remove (confirmed absent by grep pre-edit).

### 4. `Cargo.toml` dependency purge

`✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/Cargo.toml` — removed the `semio-framework-os` dependency
line. Grepped `semio_framework_os::` across the whole crate post-edit: **zero hits** — the 15 deleted
calls in `🔧️setup` were the only usage in the crate (matches W0 census's "4 symbols, all
`register_mesh_*`" finding for this plugin).

## Files touched

- **Updated**: `✏️s/🔌️plugins/💠️lowpoly/🦀️component.rs` (`.setup(...)` target)
- **Updated**: `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`
  (`register()` absorbs app-schema + document-codec registration, updated docstring)
- **Updated**: `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/📦️glue.rs` (setup mount removed)
- **Updated**: `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/Cargo.toml` (`semio-framework-os` dep removed)
- **Removed**: `✏️s/🔌️plugins/💠️lowpoly/🔧️setup/` (dir + `🦀️component.rs`)
- **Removed**: `✏️s/🔌️plugins/💠️lowpoly/🛂️manifest/` (dir + `🦀️component.rs`)
- **Removed**: `✏️s/🔌️plugins/💠️lowpoly/🎟️capabilities/` (dir + `🦀️component.rs`)

Nothing else in the plugin was touched. `🎛️apps/💠️lowpoly/**`, `🗿️artifacts/💠️lowpoly/**` (other than
the one `register()` edit above), and `AGENTS.md` are unchanged.

## Step 6 — structural verification (no cargo, per standing order)

**1. Plugin root shape:**
```
$ ls -a "✏️s/🔌️plugins/💠️lowpoly/"
.
..
AGENTS.md
🎛️apps
📦️packages
🗿️artifacts
🦀️component.rs
```
Exactly `🦀️component.rs`, `AGENTS.md`, `🎛️apps`, `🗿️artifacts`, `📦️packages` — the closed APA shape.
No README.md existed before this wave either (not something this wave removed).

**2. Every `#[path = "..."]` in `📦️glue.rs` resolves on disk** — checked exhaustively, not sampled:
```
$ GLUE="✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/📦️glue.rs"; DIR=$(dirname "$GLUE")
$ grep -n '#\[path = "' "$GLUE" | sed -E 's/^[0-9]+:.*#\[path = "([^"]+)"\].*/\1/' \
    | while read -r p; do [ "$p" = "." ] && continue; [ -f "$DIR/$p" ] || echo "MISSING: $p"; done
done checking, no MISSING lines above means all resolve
```
Zero `MISSING` lines printed — every mount target exists.

**3. Dangling-reference grep for everything moved/removed:**
```
$ grep -rn "register_lowpoly_exports\|🔧️setup" "✏️s/🔌️plugins/💠️lowpoly/" --include="*.rs"
✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/.../⚙️engine/🦀️component.rs:80:/// the deleted `🔧️setup` facet. ...
```
Only hit is the new docstring's own prose reference to the facet it replaced — not a code reference.
Confirmed zero occurrences of `register_lowpoly_exports` anywhere else repo-wide:
```
$ grep -rn "register_lowpoly_exports" . --include="*.rs" | grep -v "/🎯️target/"
(no output before this edit wave except the definition itself, which is now deleted)
```
```
$ ls "✏️s/🔌️plugins/💠️lowpoly/🔧️setup" "✏️s/🔌️plugins/💠️lowpoly/🛂️manifest" "✏️s/🔌️plugins/💠️lowpoly/🎟️capabilities"
ls: ...🔧️setup: No such file or directory
ls: ...🛂️manifest: No such file or directory
ls: ...🎟️capabilities: No such file or directory
```
```
$ grep -rn "semio_framework_os\b" "✏️s/🔌️plugins/💠️lowpoly/" --include="*.rs" --include="*.toml"
(zero hits)
```

**4. No files were "moved" this wave in the file-relocation sense** (nothing was folded into a parent
module) — the two lines that changed home (`register_app_schema`, `register_document_codec_for_app`
calls) were inlined as new statements into an *existing* function in an existing file
(`⚙️engine/🦀️component.rs`), not pasted as a whole file into another. No new `#[path]` mounts were
created or needed.

**5. `pluginChildDirs` re-checked** (deletion of 3 facets could only be safe if the taxonomy gate no
longer requires them):
```
$ grep -n -A2 '"pluginChildDirs"' "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
  "pluginChildDirs": [
    "🎛️apps"
  ],
```
Already relaxed to `["🎛️apps"]` by an earlier wave (per this dispatch's Step 1 note) — not this
plugin's gate to flip, and not touched here.

Cargo verification (`cargo check`/`cargo test`) was **intentionally deferred** per the standing order
("DO NOT RUN CARGO — the SDK is red"). All evidence above is structural (grep + `ls` + a `#[path]`
resolution sweep), not compile-verified.

## Step 5 — inventory only, nothing changed

### `thread_local!` — `🎛️apps/💠️lowpoly/🦀️component.rs:48-52`
```rust
thread_local! {
    static LOWPOLY_SCRATCH: std::cell::RefCell<crate::apps::lowpoly::session::LowpolyScratch> = ...;
}
```
Backing type `LowpolyScratch` (`🎛️apps/💠️lowpoly/🖌️session/🦀️component.rs:215-224`):
```rust
pub struct LowpolyScratch {
    stroke: Option<PaintStrokeSession>,
    stroke_drag_active: bool,
    stroke_dirty: u64,
    transform: Option<TransformSession>,
    transform_drag_active: bool,
    texture_cache: PaintTextureLut,
    preview_seq: u64,
}
```
Split by concern, per dispatch instruction (do not lift the struct wholesale):
- **Genuine user-gesture draft state** (belongs in the future typed `Draft`, per
  `📓️draft-lane-spec.md`'s shape): `stroke` (`PaintStrokeSession` — mid-gesture paint-stroke session),
  `stroke_drag_active`, `transform` (`TransformSession` — gumball/transform session), `transform_drag_active`.
  Proposed verb-slugs from the closed table: the pre-blessed domain verb `paint-stroke` is available
  for lowpoly specifically per the spec (*"available for lowpoly only if a stroke is genuinely
  indivisible; default to the core decomposition"* — this plugin's own stroke session already looks
  decomposable into `create-stroke`/`insert-stroke-point{index}`, not proposing the domain verb without
  a closer read of `PaintStrokeSession`'s own field shape, out of scope for this inventory-only pass);
  `bind-transform`/`unbind-transform` for the gumball session; `move`/`drag`/`rotate`/`scale` for the
  transform itself once bound — never `update`.
- **Derived cache, NOT draft state** (belongs in an inference, per dispatch instruction): `texture_cache`
  (`PaintTextureLut`) — populated by `refresh_texture_cache(projection)`, a pure function of the
  document snapshot (`projection: &LowpolySnapshot`), not of any user gesture. `preview_seq` (`u64`,
  monotonic per-key counter for `gesture_preview`) is closer to `stroke_dirty` (`u64`) — both look like
  low-severity bookkeeping counters, not user-authored state; neither obviously needs a `Draft` field,
  flagging both as UNVERIFIED-whether-draft-worthy rather than proposing verbs for them.

### `render()` mutation — `🎛️apps/💠️lowpoly/🦀️component.rs:342-348`
```rust
fn render(body_key: &str, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>) -> UiNode {
    let projection = doc.snapshot;
    ...
    let (scratch_projection, texture_cache) = LOWPOLY_SCRATCH.with(|scratch| {
        let mut scratch = scratch.borrow_mut();
        if matches!(body_key, LOWPOLY_PLAY_BODY_MAIN | LOWPOLY_PLAY_BODY_UV) {
            scratch.refresh_texture_cache(projection);
        }
        (scratch.transform_projection(), scratch.textures().clone())
    });
    ...
}
```
The one render-mutation violation in the census (`&ArtifactView` is immutable at the trait level, yet
`render()` mutates thread-local scratch mid-call). Confirms the dispatch note: `texture_cache` is a
**derived value** (a pure function of `projection`, recomputed lazily on render) — it belongs in an
inference, not a `Draft` field, and is architecturally the wrong thing to be sitting in the same
`RefCell` as the genuinely gestural `stroke`/`transform` fields. Not fixed this wave (inventory only).

### `mesh_artifact_kind()` — `🗿️artifacts/💠️lowpoly/🦀️component.rs:263-278` — **unchanged, per instruction**
```rust
pub fn mesh_artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec { id: "3d.mesh".into(), name: "3D Mesh".into(), ... }
}
```
Second `ArtifactKindSpec` in this file declaring `"3d.mesh"`, sitting alongside the real
`artifact_kind()` (`:243-258`, declares `"3d.lowpoly"`). **Not deleted** — the dispatch is explicit that
removing this declaration is UCAS's call, not APA's. Flagging prominently per instruction: `🌍️gis`'s
`🏔️gisterrain` artifact (`🗿️artifacts/🏔️gisterrain/🦀️component.rs:218` per W0-A census) declares the
identical `"3d.mesh"` shape independently — two plugins co-declaring the same ownerless kind. This
plugin's `register()` no longer *registers* IO for `"3d.mesh"` (deleted per §1 above); the
`ArtifactKindSpec` value itself, unused by any registration call now, is left in place exactly as
instructed.

### `std::fs`/`std::env`/`std::process`/`Command::new`/network, `fn seed(`
```
$ grep -rn "std::fs::\|std::env::\|std::process::\|Command::new(" "✏️s/🔌️plugins/💠️lowpoly/" --include="*.rs"
✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/.../⚙️engine/🧵️media/🦀️component.rs:123:
    if std::env::var("EXPORT_LOWPOLY_FOREST_MESH").ok().as_deref() != Some("1") {
```
Single hit, inside `#[test] fn export_concrete_forest_left_lowpoly_mesh_json()` (a debug-export test
gated by an env var, opt-in only) — test-only, matches the census's exclusion criteria, not a
production violation. `fn seed(` — zero hits, matches census (the one repo-wide `seed()` impl is
`🌿️vcs`, not lowpoly). No network calls found.

## `## sharedFileRequests`

None. Everything touched this wave was inside `✏️s/🔌️plugins/💠️lowpoly/`.

## `## Concurrent-churn observations`

`git log --oneline -3 -- "✏️s/🔌️plugins/💠️lowpoly"` showed the plugin's most recent commits
(`...🚩️494`, `...🚩️493`, `...🚩️491`) predate this wave's edits; `stat -f '%Sm'` on
`🦀️component.rs` before editing showed no suspiciously-fresh timestamp. No sign another session was
mid-edit in this plugin's directory during this wave.

## Honest pass/fail

`apa-status: complete`

Everything in Step 0–4 for this plugin is done: setup/manifest/capabilities facets deleted, the real
registration code folded into the artifact's own `⚙️engine`, `"3d.mesh"` registration deleted (not
relocated, no mesh artifact created), the plugin-root `.setup()` hook repointed, the `📦️glue.rs` mount
removed, and `semio-framework-os` purged from `Cargo.toml` with a verified-zero-remaining-usage grep.
Step 5 (thread_local/render-mutation/`mesh_artifact_kind`/fs-env inventory) is complete and
change-free as instructed. Step 6 structural evidence is pasted above in full.

**What the consolidated build should check first for this plugin**: (1) that
`crate::apps::lowpoly::config::schema::register_app_schema` and
`crate::apps::lowpoly::LowpolyPlayApp` really are reachable from
`🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` via the `crate::apps::`
path the way this report assumes (verified only by grep-precedent — `crate::artifacts::` files in this
crate had never referenced `crate::apps::` before this edit, so this is the first cross-reference of
its kind in the plugin, though the same shim-module structure `📦️glue.rs` already provides for
`crate::artifacts::lowpoly::engine` makes `crate::apps::lowpoly::*` resolve by the identical mechanism);
(2) that `LowpolyComposerComposition`'s `"s.lowpoly"` composer dialect and `artifact_kind()`'s
`"3d.lowpoly"` `ArtifactKindSpec` are in fact two names for the same registration surface at the OS
layer (assumed, not proven, when treating the 7 `"3d.lowpoly"` `register_mesh_*` calls as pure
duplicates rather than a distinct code path — this was the basis for deleting rather than relocating
them, and a compile+integration-level export/import round-trip test for `"3d.lowpoly"` OBJ/GLB/STL
would be the strongest confirmation).
