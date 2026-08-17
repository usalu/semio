# W3 — `🖍️draw` plugin closure report

(Supersedes this file's earlier content, written under the pre-correction clearance
reading. Clearance rule since corrected: SMO confirmed absence from all four sections of
`plugin-release-status.md` means FREE, not blocked. Re-verified below.)

Clearance: `🖍️draw` is absent from all four sections of SMO's
`📓️plugin-release-status.md` (RELEASED, HELD in-flight, HELD between-waves, NOT SMO'S TO
RELEASE) → FREE, confirmed before touching anything.

## What changed

### Step 1 — dead facet directories deleted
All three were the 1-line doc-only stub, confirmed unmounted first via
`grep -n "🛂️manifest\|🎟️capabilities\|🔧️setup" 📦️glue.rs` (zero hits):
- `✏️s/🔌️plugins/🖍️draw/🛂️manifest/🦀️component.rs` — deleted
- `✏️s/🔌️plugins/🖍️draw/🎟️capabilities/🦀️component.rs` — deleted
- `✏️s/🔌️plugins/🖍️draw/🔧️setup/🦀️component.rs` — deleted

Safe against the taxonomy gate: `🔣️taxonomy.json:386-388` already reads
`"pluginChildDirs": ["🎛️apps"]` — the relaxed shape the ticket notes promised, confirmed
read-only, not edited by me.

No `.DS_Store`/`node_modules` existed at plugin root — nothing to remove there.

### Root `🛂️manifest.json` (476B) — relocated, not deleted
Read it: `{"schema":"manifest","id":"draw-layers","name":"Draw Document Layers","layerKinds":[shape,path,text,image,group,boolean,trace]}`.
**This is not fixture data** — `🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/📜️script.ts:44-74`
(`findManifestFiles`) walks the whole repo tree matching the filename prefix
`🛂️manifest.json` and feeds every hit into a Rust+TS kind-catalog codegen pipeline
(`GraphManifestDocument`/`MANIFEST_IDS`). Its own comment (script.ts:59-66) states the
canonical taxonomy explicitly: *"the `🗿️artifacts/<component>/🛂️manifest.json` taxonomy...
sits directly under the component's own artifact folder with no 'manifest'-named parent
directory at all."* Verified against already-closed plugins on disk (`🧩️puzzle/🗿️artifacts/🧊️3d/🛂️manifest.jsondefault.manifest.json`,
`🔱️trinity/🗿️artifacts/🔌️jack/🛂️manifest.jsonnakagin.manifest.json`) — same convention, already live.
Moved: `✏️s/🔌️plugins/🖍️draw/🛂️manifest.json` → `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🛂️manifest.json`
(bare filename, no descriptor suffix — only one manifest source for this artifact). No
`include_str!`/`include_bytes!` referenced it from Rust; no repo file referenced the old
path (grep clean, see Step 6 evidence below).

### Step 2 — plugin root
Final root: `🦀️component.rs`, `🎛️apps`, `🗿️artifacts`, `📦️packages`, plus **`🔄️fsm`
(NOT relocated this wave — see Concurrent-churn observations)**. No `AGENTS.md`/`README.md`
existed before or after (matches the plugin-specific note: "No AGENTS.md — do not add one").

### Step 3 — escape-hatch call sites
`grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_2d_export_handlers\|register_app_io\|register_os_media_"` across the whole plugin →
**zero hits**. Draw's SVG/PDF/PNG/JSON/DWG/DXF import/export are already artifact-native,
mounted directly under `🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/{📥️import,📤️export}/…` in `📦️glue.rs` — nothing to relocate.

### Step 4 — dependency purge
`grep -rn "semio_framework_os::"` (bare, excluding `_kernel`) across the whole plugin →
**zero hits**. Removed the unused dependency from
`✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/Cargo.toml`:
```diff
-semio-framework-os = { path = "../../../../../🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust", package = "semio-framework-os" }
```
(was line 31, between `semio-framework` and `semio-framework-plugin`). `semio-framework-os-kernel`
(a distinct crate, aliased `dsl`/`store`/`protocol` in `📦️glue.rs`) is heavily used and was left untouched.

## Step 5 — inventory only (nothing changed)

**`thread_local!` at `🎛️apps/🖍️draw/🦀️component.rs:164`**, inside `DrawPlayApp::handle()`:
```rust
thread_local! {
    static DRAW_SESSION: std::cell::RefCell<DrawSession> = std::cell::RefCell::new(DrawSession::default());
}
```
Despite being lexically declared inside a function body, this is a genuine `thread_local!`
static — its storage is allocated once per OS thread on first access and **persists across
every subsequent call to `handle()` on that thread**, indistinguishable in lifetime from an
app-scoped singleton. The function-local declaration only limits where the identifier
`DRAW_SESSION` is nameable in source, not its actual reset behaviour — it does **not**
reset per-call. This is exactly the "scope is a single function rather than the app" nuance
the ticket flagged as interesting for the draft-lane design.

`DrawSession` (defined `🎛️apps/🖍️draw/🎮️commands/🖱️canvas/🦀️component.rs:559-564`) holds:
- `gesture: draw_gesture::Snapshot` — the live `fsm` statechart snapshot driving pointer
  gestures (draft-path point accumulation, marquee drag, shape drag, trace click). **Genuine
  user-gesture state**, not derived from the document — it is assembled purely from raw
  pointer events and only exists between pointer-down and commit/Escape. Proposed `Draft`
  field: `Draft.gesture: GestureSnapshot`. Proposed verb-slugs from the approved table:
  `drag` for marquee-select and shape-drag (continuous pointer movement accumulating a
  start/end rect), `insert` for draft-path point accumulation (`CommitDraft`), `create` for
  the single-click trace commit (`CommitTrace` — no intermediate draft state). The point-pick
  select path (`PickPoint`) mutates only `config.selected_ids`, never the document — it is an
  ephemeral config change, not an artifact draft, so it likely needs no verb-slug at all.
- `preview_seq: u64` — a monotone counter bumped every gesture step, used only to tag
  preview payloads (`gesture_preview()`, itself `#[allow(dead_code)]` — "no caller exists
  inside this crate today; exercised by tests only until [a sync] bridge lands") with a
  freshness sequence number. **This is not a derived-from-document cache (so it doesn't
  belong in an inference either) and it is not draft geometry** — it's ephemeral wire-protocol
  bookkeeping for a not-yet-wired preview-sync path. Best fits the "ephemeral local-only"
  bucket in CLAUDE.md's four-way state split, not `Draft`.

`std::fs`/`std::env`/`std::process`/`Command::new`/network calls outside `#[cfg(test)]`,
and any `fn seed(`: **zero hits** anywhere in the plugin (including `🔄️fsm`).

## Step 6 — structural verification (cargo intentionally NOT run)

1. `ls -a "✏️s/🔌️plugins/🖍️draw/"` →
   ```
   . .. 🎛️apps 📦️packages 🔄️fsm 🗿️artifacts 🦀️component.rs
   ```
2. Every `#[path = "..."]` in `📦️packages/🦀️rust/📦️glue.rs` resolved against disk
   programmatically (90 non-`"."` path attributes checked): **all 90 resolve, zero
   dangling mounts.**
3. `grep -rln "🛂️manifest\b\|🎟️capabilities\b\|🔧️setup\b" ✏️s/🔌️plugins/🖍️draw` → no hits
   (no dangling references to the deleted facet dirs).
   `grep -rln "🔌️plugins/🖍️draw/🛂️manifest\.json"` repo-wide (excluding the new path) → no
   hits (no dangling references to the old manifest.json location).
   `grep -rn "semio_framework_os::"` (bare) → no hits (dependency removal is safe).
4. `🛂️manifest.json` exists as its own standalone file at the new path (not merged into
   any other file); confirmed via direct read after the move, in a separate tool call, to
   rule out the transient-move issue described below.

## `## sharedFileRequests`
None required for the work actually completed this wave. **Deferred**, for whoever attempts
the `🔄️fsm` relocation next: two lines in the repo-root `Cargo.toml` workspace `members`
array (lines 66-67) would need updating from
`"✏️s/🔌️plugins/🖍️draw/🔄️fsm/📦️packages/🦀️rust"` /
`"✏️s/🔌️plugins/🖍️draw/🔄️fsm/✨️macros/📦️packages/🦀️rust"` to their new
`🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🔄️fsm/…` equivalents, plus the
plugin's own `📦️packages/🦀️rust/Cargo.toml:27` `fsm` dependency path. I deliberately did not
touch root `Cargo.toml` myself (out of my plugin-directory boundary, and it is a
heavily-shared file across the five live sessions).

## `## Concurrent-churn observations`
**`🔄️fsm` relocation attempted and reverted — did not complete this wave.** Sequence,
each step verified in a separate tool call after the first:
1. Moved the whole `🔄️fsm` directory tree (a 2-crate boundary: the fsm kernel crate plus
   its `✨️macros` proc-macro sibling — matches `📓️w0-b-plugin-shape.md` §5's own proposed
   destination, `🗿️artifacts/draw/🏅️standards/🔖️1/⚙️engine/fsm/`, which that census already
   flagged "medium confidence — crate-boundary nuance"). Immediately after the `mv`, in the
   *same* shell invocation, `find` on the new path showed the full 10-file tree present.
2. Edited the 2 `📋️project.json` (`$schema` relative depth) and 2 `📜️script.ts` (framework
   library import relative depth) to the new depth (13/14 `../` respectively, verified by
   `os.path.normpath` resolution against real files before trusting the count).
3. A **later, separate** verification call found `🔄️fsm` back at the **original** plugin-root
   location — confirmed exhaustively with `os.walk` (not shell/encoding-dependent) that no
   directory named (or Unicode-normalizing to) `fsm` existed anywhere under the new `⚙️engine/`
   path; only the original path had it. `git status --porcelain` on those 4 files flipped
   from unstaged-modified (` M`) to staged-modified (`M `) to staged+working-modified (`MM`)
   across three consecutive checks seconds apart — direct evidence of another live process
   (the repo's auto-commit daemon, or a concurrent session) touching this exact directory
   while I worked it, consistent with the ticket's own "five live agent sessions" / "repo
   auto-commits" warnings.
4. The directory move-back carried my in-flight *content* edits along with it (the reverted
   files at the old path still had the new, now-wrong-for-that-location `../` depth) — this
   would have been a real regression (broken nx `$schema`/import paths) if uncaught. **I
   reverted all 4 files' content back to the depth correct for their current (original)
   location**, verified in a following call, and left the physical `🔄️fsm` directory alone.
   I did not touch the plugin's own `Cargo.toml` `fsm` dependency line (still points at the
   original path, correctly, since the directory is still there) or root `Cargo.toml`.
5. **Net effect: `🔄️fsm` is untouched from where it started** (all 10 files, byte-identical
   content to before my session, at the original path) — inventoried but not relocated. Retry
   once whatever concurrent process was touching this path has quieted; treat the
   `📓️w0-b-plugin-shape.md` §5 destination proposal as still the right target when it's safe.

Also observed (pre-existing, not caused by me): `📦️glue.rs` and the artifact's
`⚙️engine`/`📚️examples` paths were already renamed into the
`🏅️standards/🔖️1/🪆️subsets/✳️any/…` shape by earlier work (visible as staged renames in
`git status` before I made any edit) — consistent with the plugin-shape closure already
being mostly done by a prior wave; I built on top of that, did not redo it.

## Files created / updated / removed
- **Removed:** `✏️s/🔌️plugins/🖍️draw/🛂️manifest/🦀️component.rs`,
  `✏️s/🔌️plugins/🖍️draw/🎟️capabilities/🦀️component.rs`,
  `✏️s/🔌️plugins/🖍️draw/🔧️setup/🦀️component.rs` (dirs deleted with them)
- **Moved:** `✏️s/🔌️plugins/🖍️draw/🛂️manifest.json` → `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🛂️manifest.json`
- **Updated:** `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/Cargo.toml` (removed unused
  `semio-framework-os` dependency)
- **Unchanged (attempted then reverted, net no-op):** the 4 files under `🔄️fsm/` — see
  Concurrent-churn observations above; content verified byte-identical to session start.
- **Report:** this file.

`apa-status: partial` — everything except the `🔄️fsm` relocation is done and structurally
verified; `🔄️fsm` remains inventoried with a concrete destination and concrete blocker
(active concurrent churn on that exact path) rather than attempted again blind.
