# W3 — `🎥️shooting` plugin closure report

Plugin: `✏️s/🔌️plugins/🎥️shooting/` (crate `semio-s-plugin-shooting`). Clearance: SMO's
`📓️plugin-release-status.md` lists `🎥️shooting` under **"RELEASED — Wave C / late Wave M lanes
complete"** (31 mutations, 1:1 triad dirs, glue rewired, `cargo check` 0 errors, `cargo test`
104/104) — free to edit, not HELD, not another session's.

## What changed

Deleted the three doc-only, unmounted facet stub directories at plugin root:

- `✏️s/🔌️plugins/🎥️shooting/🛂️manifest/🦀️component.rs` — 1-line doc stub (`//! 🛂️ Manifest facet for `🎥️shooting` — identity surfaces live on `Plugin::builder` in the parent.`)
- `✏️s/🔌️plugins/🎥️shooting/🎟️capabilities/🦀️component.rs` — 1-line doc stub
- `✏️s/🔌️plugins/🎥️shooting/🔧️setup/🦀️component.rs` — 1-line doc stub

Verified before deletion (`grep -n "🛂️manifest\|🎟️capabilities\|🔧️setup" "✏️s/🔌️plugins/🎥️shooting/📦️packages/🦀️rust/📦️glue.rs" "✏️s/🔌️plugins/🎥️shooting/🦀️component.rs"`) → **zero matches** — none of the three facets were `#[path]`-mounted anywhere, so this is doc-only+unmounted → straight deletion per the packet's decision table. No real code, no JSON fixture data (unlike trinity's `🛂️manifest`), nothing to relocate.

Root `🦀️component.rs`'s `.setup(crate::artifacts::shooting::engine::register)` call is unrelated —
it invokes the artifact engine's own registration fn via the `PluginBuilder::setup` hook, not the
now-deleted `🔧️setup/` facet directory.

No `.DS_Store` / `node_modules` found at plugin root (none to remove).

## Step 2 — plugin root closure

Root now contains exactly the target shape:
```
AGENTS.md  README.md  🎛️apps  📦️packages  🗿️artifacts  🦀️component.rs
```
No relocation needed — matches the packet's own note ("No extra root dirs per the census — largely
facet-dir deletion").

## Step 3 — escape-hatch call sites

`grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_2d_export_handlers\|register_app_io\|register_os_media_" "✏️s/🔌️plugins/🎥️shooting/"` → **one hit, and it is prose, not a call**:
`🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:397`, a doc-comment
sentence that *names* `register_dwg_import_handler`'s callback signature while explaining why the
DWG importer can't reach session-only camera state — no such function is called anywhere in the
crate. **Zero escape-hatch violations in `🎥️shooting`.** Nothing to relocate, nothing to delete.

## Step 4 — dependency purge

`semio-framework-os` stays in `📦️packages/🦀️rust/Cargo.toml:43`. `grep -rn "semio_framework_os::" `
found one real call site: `🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:136`
— `semio_framework_os::rasterize_svg_to_png_base64(...)`, a plain utility function, not a member of
the `register_*` escape-hatch family. Same precedent as gis's sanctioned type usage — dependency
correctly stays.

## Step 5 — inventory only (nothing touched)

- `thread_local!` — zero hits repo-wide inside the plugin (`grep -rn "thread_local!" --include="*.rs" .`).
- `std::fs` / `std::env` / `std::process` / `Command::new` — zero hits outside test code (none at all, in fact).
- `fn seed(` — zero hits.

`🎥️shooting` carries no interior-mutable app state, no draft-lane candidate fields, no fs/env/process
usage, no seed functions. Nothing to propose for the `Draft` shape.

## Step 6 — structural verification

1. `ls -a "✏️s/🔌️plugins/🎥️shooting/"`:
```
.  ..  AGENTS.md  README.md  🎛️apps  📦️packages  🗿️artifacts  🦀️component.rs
```
2. `#[path]` mount sweep (python script walking every `#[path = "..."]` in `📦️glue.rs` relative to
   the glue file's directory, skipping the grouping `"."` entries):
```
total #[path] attrs: 295
non-'.' path mounts checked: 168
missing: 0
```
   All 168 real file mounts resolve. Zero dangling.
3. `grep -rn "🔌️plugins/🎥️shooting/🛂️manifest\|🔌️plugins/🎥️shooting/🎟️capabilities\|🔌️plugins/🎥️shooting/🔧️setup" --include="*.rs" --include="*.ts" --include="*.json" .` (repo-wide, excluding ticket scratch) → **zero matches**. No dangling references anywhere to the three deleted paths.
4. `cargo metadata --no-deps --format-version 1` (the one sanctioned cargo command), scoped with
   `CARGO_TARGET_DIR` under this ticket's `🎯️target`:
```
$ cd /Users/ueli/Documents/semio && CARGO_TARGET_DIR=".../ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/🎯️target" cargo metadata --no-deps --format-version 1 >/dev/null 2>stderr.txt; echo EXIT:$?
EXIT:0
```
   stderr.txt was empty. Workspace graph loads cleanly — no workspace-member breakage introduced.
   (Note: an initial attempt piped through the macOS-absent `timeout` binary and printed "FAILED"
   from a shell "command not found", not from cargo itself; the real invocation above is the one
   that counts and it is clean.)

## Files touched

- Removed: `✏️s/🔌️plugins/🎥️shooting/🛂️manifest/🦀️component.rs` (and its now-empty parent dir)
- Removed: `✏️s/🔌️plugins/🎥️shooting/🎟️capabilities/🦀️component.rs` (and its now-empty parent dir)
- Removed: `✏️s/🔌️plugins/🎥️shooting/🔧️setup/🦀️component.rs` (and its now-empty parent dir)
- Created: this report

No files created/updated inside `🎛️apps`, `🗿️artifacts`, `📦️packages`, or `🦀️component.rs` — none
were needed; the plugin had zero escape-hatch call sites and zero extra root dirs beyond the three
deleted facets.

## sharedFileRequests

None. `pluginChildDirs`/policy-gate/registry-codegen/taxonomy-test updates (the repo-wide
`🛂️manifest`/`🎟️capabilities`/`🔧️setup` → `[apps, artifacts]` flip) are explicitly out of this
per-plugin packet's scope per `📓️important.md` — "the flip is the LAST thing APA does," landing
repo-wide, not per plugin.

## Concurrent-churn observations

`git log --oneline -3 -- "✏️s/🔌️plugins/🎥️shooting/"` immediately after the deletion showed a new
auto-commit (`fd01661f06`) on top of the pre-existing history (`11334431b9`, `1caac91709`) — the
repo's auto-commit picked up this wave's deletion promptly, consistent with expected behaviour, not
churn from another session. No other session's edits were observed landing in this plugin's tree
during the wave.

## apa-status: complete
