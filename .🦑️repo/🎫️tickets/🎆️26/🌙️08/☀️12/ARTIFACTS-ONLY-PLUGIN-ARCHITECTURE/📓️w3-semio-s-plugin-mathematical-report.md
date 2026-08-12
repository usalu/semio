# W3 — `➗️mathematical` (crate `semio-s-plugin-mathematical`) — plugin migration report

## Clearance (Step 0)

Read `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md`.
`➗️mathematical` appears under **"RELEASED — Wave C / late Wave M lanes complete"**:
> `➗️mathematical` | `➗️mathematical` | `dsl_derive::Mutations` → `dsl::Mutations` bug fixed; 3 orphan triads deleted; 6 funnel call sites

Not HELD, not another session's. Free to proceed.

## What changed

### Step 1 — deleted the three dead facet directories

All three were confirmed doc-only (single-line stub `//! …`), zero fixture data, zero mounting in
`📦️packages/🦀️rust/📦️glue.rs`:

- `grep -n "🛂️manifest\|🎟️capabilities\|🔧️setup" "✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📦️glue.rs"` → **no output, unmounted**.
- Plugin root `🦀️component.rs`'s `.setup(crate::artifacts::mathematical::engine::register)` call is the
  `Plugin::builder` setup **hook parameter**, not a reference to the `🔧️setup/` facet directory — confirmed
  by reading the file; it never imports from `crate::setup` or similar.
- No `Cargo.toml` in any of the three (`find ... -name Cargo.toml` → empty) — pure directory delete, no
  workspace-topology risk.
- Deleted: `✏️s/🔌️plugins/➗️mathematical/🛂️manifest/` (1 file), `.../🎟️capabilities/` (1 file), `.../🔧️setup/` (1 file).
- No plugin-root `.DS_Store` / `node_modules` present (per census, mathematical had none).

### Step 2 — close plugin root

Already closed after Step 1 — the census (`📓️w0-b-plugin-shape.md` §2) recorded `➗️mathematical` with
"extra dirs beyond {apps, artifacts, packages}: none", so there was nothing to relocate. Verified on disk
(see Step 6.1).

### Step 3 — escape-hatch call sites

`grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_2d_export_handlers\|register_app_io\|register_os_media_" "✏️s/🔌️plugins/➗️mathematical/"` → **zero matches**. No violation of this class exists in this plugin. No changes needed.

### Step 4 — dependency purge

`Cargo.toml` (`📦️packages/🦀️rust/Cargo.toml`) has **no** `semio-framework-os` dependency at all — only
`semio-framework-os-kernel` (aliased `dsl`/`store`/`protocol` in glue.rs), which is a different crate and is
used extensively by the plugin's own DSL/store/protocol code. `grep -rn "semio_framework_os::"` → zero
matches. Nothing to purge.

## Files touched

- Removed: `✏️s/🔌️plugins/➗️mathematical/🛂️manifest/🦀️component.rs` (dir removed)
- Removed: `✏️s/🔌️plugins/➗️mathematical/🎟️capabilities/🦀️component.rs` (dir removed)
- Removed: `✏️s/🔌️plugins/➗️mathematical/🔧️setup/🦀️component.rs` (dir removed)
- Created: this report file.

No other file was edited (no `#[path]` mounts existed for the three facets, so none needed repointing).

## Step 6 — structural verification (no cargo compile)

**6.1 — closed shape:**
```
$ ls -a "✏️s/🔌️plugins/➗️mathematical/"
.
..
AGENTS.md
README.md
🎛️apps
📦️packages
🗿️artifacts
🦀️component.rs
```
Exactly the target shape: `🦀️component.rs`, `AGENTS.md`, `README.md`, `🎛️apps`, `🗿️artifacts`, `📦️packages`.

**6.2 — every `#[path = "..."]` in `📦️glue.rs` resolves to a real file:**
Script: read every `#[path = "..."]` attribute, skip self-mounts (`"."`), resolve the rest relative to
`glue.rs`'s directory, check `-f`.
```
total #[path] attrs: 153 ; dot(self) mounts: 69 ; real-file-target mounts checked: 84
MISSING[74]: ../../🎛️apps/➗️mathematical/🎮️commands/📄️document/🦀️component.rs
checked: 84  missing: 1
```
**This one miss is pre-existing concurrent churn, not caused by this wave.** Evidence:
- `✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🎮️commands/` on disk today has `📄️artifact/`,
  `📐️geometry/`, `🕸️graph/`, `🗣️locale/` — **no `📄️document/`**. `stat` shows `📄️artifact/🦀️component.rs`
  mtime `Aug 12 15:31:13`, i.e. a `document → artifact` rename landed there ~2h before this wave started.
- `glue.rs` line 416-417 still reads `#[path = "../../🎛️apps/➗️mathematical/🎮️commands/📄️document/🦀️component.rs"] pub mod document;` — unrenamed. `glue.rs` mtime is `Aug 12 17:30:19` (touched *after* the rename, by something else — likely the auto-commit bot re-timestamping, not a content fix), and `git log --oneline -3` shows unrelated auto-commits (`🚩️493-495`) around that window, nothing that patched this line.
- I never touched `🎛️apps/` or this line — this is outside my Step 1-4 scope (dead-facet deletion) and reads
  as another session's in-flight app-command rename (SMO's `🎮️commands/**` rewrite lane, or UCAS's W4
  type-repoint) that hasn't yet repointed `glue.rs`. Per the hard rule "never fix another session's file",
  left as-is and flagged here for the consolidated build.

**6.3 — grep for references to the three removed module paths (confirm no dangle from my own deletion):**
```
$ grep -rn "➗️mathematical/🛂️manifest\|➗️mathematical/🎟️capabilities\|➗️mathematical/🔧️setup" "✏️s/🔌️plugins/➗️mathematical/"
(no output)
$ grep -rln "➗️mathematical/🛂️manifest\|➗️mathematical/🎟️capabilities\|➗️mathematical/🔧️setup" . --include="*.rs" --include="*.ts" --include="*.json"
.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/ROOT-SCRIPT-POLICY-REVIVAL-AND-TAXONOMY-LINT-PREP/canonicalize-check.ts
```
The one repo-wide hit is a scratch checklist file belonging to an **unrelated, older ticket** (`26/08/05`),
already referencing pre-migration paths (`🔨️modules/🔧️op`, `🛂️manifest/🗿️artifact`) that don't exist on disk
today either — a stale snapshot, not a live call site, and out of my boundary to touch.

**6.4 — workspace still loads (the one permitted cargo command):**
```
$ cd /Users/ueli/Documents/semio && cargo metadata --no-deps --format-version 1 >/dev/null 2>/tmp/cargo_metadata_err.txt; echo "exit=$?"
exit=0
$ cat /tmp/cargo_metadata_err.txt
(empty — no stderr)
```
Workspace manifest graph loads cleanly. (Not strictly required since no directory containing a `Cargo.toml`
was moved or removed — only three unmounted, crate-less stub files were deleted — but run anyway as
confirmation.)

## Step 5 — inventory only (interior-mutable state, fs/env/process/network, seed)

```
$ grep -rn "thread_local!" "✏️s/🔌️plugins/➗️mathematical/"                          → zero matches
$ grep -rn "std::fs\|std::env\|std::process\|Command::new" "✏️s/🔌️plugins/➗️mathematical/" → zero matches
$ grep -rn "fn seed(" "✏️s/🔌️plugins/➗️mathematical/"                                → zero matches
$ grep -rn "reqwest\|hyper::|TcpStream|UdpSocket|tokio::net" "✏️s/🔌️plugins/➗️mathematical/" → zero matches
```
**Nothing to inventory.** `➗️mathematical` has no `thread_local!` app state, no filesystem/env/process/network
side effects outside `#[cfg(test)]` (none found at all, in or out of test), and no `fn seed(`. No Draft-lane
fields or verb-slugs to propose — the app has no scratch state that needs a typed Draft facet.

## sharedFileRequests

None. No framework file, no `🔣️taxonomy.json`, no other plugin was touched.

## Concurrent-churn observations

- `✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🎮️commands/` was renamed `📄️document/ → 📄️artifact/`
  by another session (~15:30-15:31) without updating the corresponding `#[path]` mount in
  `📦️packages/🦀️rust/📦️glue.rs` (line 416-417, still says `📄️document`). This is a **real dangling mount**
  that will fail `cargo check -p semio-s-plugin-mathematical` once that plugin is next compiled — belongs to
  whichever session owns the `🎮️commands` rename (SMO's app-command rewrite, per `📌️important.md`'s
  cross-session protocol table), not to APA. Flagging for the consolidated build to route to the right owner
  before it's mistaken for something this wave broke.
- No other churn observed. `git log --oneline -3` at time of writing: `fd01661f06`, `11334431b9`, `a445617cae`
  (all unrelated auto-commits, none touching `➗️mathematical`).

## apa-status: complete

---

## 12-line summary

`➗️mathematical` cleared via SMO's live predicate (RELEASED, Wave C/late-M). Deleted the three dead
plugin-root facets `🛂️manifest/`, `🎟️capabilities/`, `🔧️setup/` — all doc-only one-line stubs, unmounted in
`glue.rs`, no fixtures, no `Cargo.toml`. Plugin root now closed to exactly `🦀️component.rs`, `AGENTS.md`,
`README.md`, `🎛️apps`, `🗿️artifacts`, `📦️packages` (census already showed zero extra dirs, so Step 2 needed
no relocation). Zero escape-hatch (`register_mesh_*`/`register_app_io`/etc.) call sites in this plugin —
Step 3 no-op. `Cargo.toml` never depended on `semio-framework-os` (only the unrelated
`semio-framework-os-kernel`) — Step 4 no-op. Step 5 inventory is empty: no `thread_local!`, no fs/env/process/
network, no `fn seed(`. Structural verification: `ls -a` confirms closed shape; exhaustively checked all 84
real-file-target `#[path]` mounts in `glue.rs` (153 total attrs, 69 are `"."` self-mounts) — **83 resolve, 1
missing**, but that miss (`🎮️commands/📄️document/🦀️component.rs`) is pre-existing churn from another
session's in-flight `document → artifact` command rename, unrelated to my deletion — flagged under
Concurrent-churn, not fixed. `grep` confirms zero dangling references to the three deleted facets anywhere
in the repo (one hit is a stale scratch file from an unrelated 08/05 ticket). `cargo metadata --no-deps`
exits 0 with empty stderr — workspace still loads. **Risk for the consolidated build: the pre-existing
`📄️document`→`📄️artifact` dangling `#[path]` mount in mathematical's `glue.rs:416-417` will break
`cargo check -p semio-s-plugin-mathematical` until whoever owns that rename repoints it** — not this wave's
fault, but worth fixing before the consolidated build runs.
