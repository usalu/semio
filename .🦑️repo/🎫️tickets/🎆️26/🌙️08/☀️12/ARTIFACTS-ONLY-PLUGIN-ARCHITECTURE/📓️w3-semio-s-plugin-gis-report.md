# W3 — `🌍️gis` plugin migration report

Plugin: `✏️s/🔌️plugins/🌍️gis/` (crate `semio-s-plugin-gis`).

## Clearance

Read SMO's live predicate (`SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md`, the sole
authority per this ticket's protocol) fresh at dispatch time. `🌍️gis` appears under **"RELEASED —
Wave C / late Wave M lanes complete"**: `🌍️gis | 🗺️gismap, 🏔️gisterrain | cargo test 171/0; 8×
emoji collision fixed; both config mutations semanticized; 42 TS mirrors`. Not HELD, not
NOT-SMO'S-TO-RELEASE. Proceeded.

## Starting state was already partially migrated — verified via git log/stat, not assumed

Before touching anything I found the plugin root `🦀️component.rs` and the artifact tree already
carrying APA-shaped edits from an earlier session/wave:

- `🦀️component.rs` (root) already contained `fn register_gis_exports()` — the exact 6-line fanout
  from `🔧️setup`, called via `.setup(register_gis_exports)` inside `pub fn plugin()`, with a
  docstring explicitly citing this ticket ("folded in from the dissolved `🔧️setup` facet per APA").
  `stat -f '%Sm'` → `Aug 12 16:23:08 2026`; `git log --oneline -3 -- …/🦀️component.rs` → single
  commit `945acb5d34`.
- `🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/terrain/🦀️component.rs` already
  existed (dated Aug 12 10:50, same batch as the rest of that standard's files) and was already
  mounted in `📦️glue.rs:45-46` (`pub mod terrain;`). The old `🔨️modules/🏔️terrain/` directory was
  already **empty** (`ls -la` → only `.`/`..`) — the file itself had been moved, only the empty
  shell directory remained.

This meant the "real code → move into artifact engine" and "🔨️modules → relocate" steps of the
brief were functionally done already; my work was to finish deleting the now-dead shells and verify
nothing was left dangling, not to re-author the move.

Folding the setup fanout into the plugin-root `🦀️component.rs` rather than a single artifact's
`⚙️engine` is consistent with `📓️w0-b-plugin-shape.md` §3's own conclusion: gis's setup fanout
spans **two** owned artifacts (`gismap`, `gisterrain`) plus **two** apps' config schemas, so no
single artifact engine is the natural sole owner — the census explicitly names "folding into a
`plugin()`/`register()` call inside the root `🦀️component.rs`" as the alternative to a standalone
facet directory. I left this as-is rather than re-splitting it across two artifact engines.

## What I did this wave

### Step 1 — deleted the dead facet directories

Verified each was doc-only and unmounted before deleting:
- `🛂️manifest/🦀️component.rs` — 1-line doc-only stub.
- `🎟️capabilities/🦀️component.rs` — 1-line doc-only stub.
- `🔧️setup/🦀️component.rs` — still held the **duplicate** copy of `register_gis_exports()` (10
  lines, 6 real) — a leftover twin of the already-relocated root-`🦀️component.rs` version, not a
  second, un-migrated call site. `grep -n "register_gis_exports"` repo-wide (excluding ticket
  scratch) showed exactly one production definition (root `🦀️component.rs:10`) and one caller
  (`.setup(register_gis_exports)` at root `🦀️component.rs:23`) after deletion — the setup-facet copy
  was dead.
- `grep -n "🛂️manifest\|🎟️capabilities\|🔧️setup\|🔨️modules" "📦️packages/🦀️rust/📦️glue.rs"` → zero
  matches before deletion, confirming none were `#[path]`-mounted.

Command: `rm -rf "🛂️manifest" "🎟️capabilities" "🔧️setup"`.

### Extra dir — `🔨️modules`

`🔨️modules/🏔️terrain/` was already empty (real file pre-relocated, see above). Deleted the empty
shell: `rm -rf "🔨️modules/🏔️terrain" "🔨️modules"`.

### Root junk

Deleted stray `.DS_Store` at plugin root (`node_modules`/`.DS_Store` are always junk per policy,
never legitimate content).

## Files removed

- `✏️s/🔌️plugins/🌍️gis/🛂️manifest/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🎟️capabilities/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🔧️setup/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🔨️modules/🏔️terrain/` (already-empty dir)
- `✏️s/🔌️plugins/🌍️gis/.DS_Store`

No files created; no files updated this wave (the artifact-engine move and the root-fanout fold-in
both predate this session, per git evidence above).

## Step 2 — plugin root closed

```
$ ls -a "✏️s/🔌️plugins/🌍️gis/"
.
..
AGENTS.md
README.md
🎛️apps
📦️packages
🗿️artifacts
🦀️component.rs
```

Exactly the target shape: `🎛️apps` + `🗿️artifacts` + root `🦀️component.rs`/`AGENTS.md`/`README.md` +
`📦️packages`.

## Step 3 — escape-hatch call sites

`rg` against the family (`register_mesh_*`, `register_solid_*`, `register_dwg_*`,
`register_2d_export_handlers`, `register_app_io`, `register_os_media_*`) inside `🌍️gis/`: **zero
matches**, confirmed both by the W0-A census and by my own re-check post-edit. Per this ticket's own
notes for this plugin: gis owns `"2d.map"` but has never self-registered IO for it —
`🎪️demonstrator` is the sole handler today. **Did not author IO for `"2d.map"`** — that gap belongs
to UCAS's composition work, per explicit instruction. Nothing to convert in this step.

## Step 4 — dependency purge

`grep -rn "semio_framework_os::" ✏️s/🔌️plugins/🌍️gis/ --include="*.rs"` → 4 hits, all inside
`🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:463-484` — plain type
usage (`semio_framework_os::DwgEntity { .. }`, `semio_framework_os::DwgColor::ByLayer`) inside a
sanctioned artifact-engine location, not a `register_*` escape-hatch call. This matches the census's
"2 symbols: `DwgColor`, `DwgEntity`" count for gis exactly. **Left `semio-framework-os` in
`Cargo.toml`** — real, compliant, in-engine usage, not a removal candidate; nothing to purge.

## Step 5 — inventory only, nothing changed

- **Env read in render path (inventory, per explicit instruction not to touch):**
  `std::env::var("SEMIO_ASSET_BASE_URL")` at
  `🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🗺️map/🦀️component.rs:77`, plus
  `unsafe { std::env::set_var(...) }` / `remove_var(...)` at lines 116/121 inside test setup/teardown.
  This is the only production env-var read in any plugin app tree repo-wide per the W0-A census.
- **Interior-mutable app state:** exactly one hit —
  `🎛️apps/◻2d/🌉️wasm/🦀️component.rs:18`, `store: RefCell<GisMapStore>` — the standard wasm-bindgen
  bridge field pattern shared by 17 apps repo-wide (census Class C, flagged there as
  UNVERIFIED-framework-mandated-vs-per-app; not genuinely user-gesture Draft state, not a derived
  cache either — it is the wasm host-bridge's own store handle). No `thread_local!`, `Mutex<`,
  `Cell<`, or `Atomic*` anywhere else in the plugin.
- No proposed `Draft` fields or verb-slugs — the one hit above is bridge plumbing, not session
  gesture state, so nothing maps to the verb table.
- `std::fs::`/`std::process::`/`Command::new`: zero hits.
- `fn seed(`: zero hits.
- No duplicate-kind cleanup touched (`🏔️gisterrain`'s `"3d.mesh"` declaration untouched, per
  instruction — UCAS's).

## Step 6 — structural verification (no cargo, as instructed)

**1. Closed shape:**
```
$ ls -a "✏️s/🔌️plugins/🌍️gis/"
. .. AGENTS.md README.md 🎛️apps 📦️packages 🗿️artifacts 🦀️component.rs
```

**2. Every `#[path = "..."]` mount in `📦️glue.rs` resolves to a real file.** Programmatic check
(154 non-`.` path mounts, `.` grouping mounts excluded since they carry no filename):
```
total non-'.' path mounts: 154
missing: 0
```
Verified with a small script resolving each `#[path="…"]` string relative to
`📦️packages/🦀️rust/` (the file `📦️glue.rs` lives in) and checking `os.path.isfile`. Zero dangling
mounts.

**3. No dangling references to anything moved/removed.**
```
$ grep -rln "🌍️gis/🛂️manifest\|🌍️gis/🎟️capabilities\|🌍️gis/🔧️setup\|🌍️gis/🔨️modules" \
    /Users/ueli/Documents/semio 2>/dev/null | grep -v "🎫️tickets" | grep -v "\.nx/" | grep -v "⚡️cache"
📜️script.ts
.cursor/plans/demonstrator_2x3_grid_b1bf0f7d.plan.md
```
- `📜️script.ts:4863` is the APA report-mode census table entry
  (`POLICY_PLUGIN_CLOSED_SHAPE_DESTINATIONS`) documenting the now-completed `🔨️modules` move —
  `priority: "medium"`, report-mode only per that file's own docstring (never gates
  `VerifyScript.runGate` until Wave 5 flips it), and repo-root `📜️script.ts` is explicitly outside
  my boundary — left untouched, will read as a stale-but-harmless census row until Wave 5's sweep.
  Filed nothing further since it's non-blocking and not mine to edit.
- `.cursor/plans/demonstrator_2x3_grid_b1bf0f7d.plan.md:38` references a pre-restructure path
  (`🛂️manifest/🗿️artifact/⚡️implementations/…`) that doesn't match current taxonomy shape even before
  my edit — a stale planning doc, not source, not touched.
- No hits at all in any `.rs`/`.ts` source file repo-wide.
- `grep -rn "mod setup\|mod manifest\|mod capabilities" ✏️s/🔌️plugins/🌍️gis/` → zero.

**4. Moved/relocated files exist at their own path, not pasted into a parent.** The one relocation
in this plugin's history (`🏔️terrain` → `gisterrain`'s engine) predates this session; confirmed its
target is still its own file, `#[path]`-mounted, not inlined:
```
$ ls "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/terrain/"
🦀️component.rs
```
mounted at `📦️glue.rs:45-46` (`#[path = "…/⚙️engine/terrain/🦀️component.rs"] mod terrain; pub use
component::*;` shape via `pub mod terrain;`).

Cargo verification intentionally **not run** — deferred per the coordinator's instruction (shared
build lock, `semio-framework-plugin` currently red from a peer session's in-flight `self.children`
work). All evidence above is structural (file existence, mount resolution, grep) per the coordinator's
explicit instruction.

## `## sharedFileRequests`

None. No framework file, no other plugin, no repo-root `📜️script.ts`, no `🔣️taxonomy.json` edit was
needed for this plugin's closure.

## `## Concurrent-churn observations`

- The plugin was already mid-migrated by an unidentified earlier pass (root `🦀️component.rs` at
  commit `945acb5d34`, `🏔️terrain`'s new home dated Aug 12 10:50 — same batch timestamp as most of
  this plugin's other files, consistent with a repo-wide batch touch rather than a targeted edit).
  I did not attribute this to a specific session; I verified it structurally (mount resolution +
  reference grep) rather than trusting the file's presence alone, per this ticket's own
  "live-predicate, not derived-artifact" rule.
- `.🦑️repo/⚡️cache/breaches/compose.json`, `.nx/workspace-data*/file-map.json` also matched the
  facet-path grep — these are generated caches, expected to still mention deleted paths until their
  next regeneration; not source, not edited.

## apa-status: complete
