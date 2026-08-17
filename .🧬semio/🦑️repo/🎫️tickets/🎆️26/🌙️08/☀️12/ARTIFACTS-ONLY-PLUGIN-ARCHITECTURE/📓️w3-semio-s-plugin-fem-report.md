# APA Wave 3 — `🏗️fem` (crate `semio-s-plugin-fem`) migration report

Supersedes the prior content of this file (an earlier agent read "absent from RELEASED" as "held"
and stopped without editing — `important.md`'s own correction names this exact session as one of
the five that did this). Re-derived per the corrected rule below.

## Step 0 — clearance

Read SMO's live predicate:
`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md`.

`grep -n "fem" … -i` → zero hits. `🏗️fem` appears in **none** of the four sections (RELEASED ×3
tables, HELD — lane in flight, HELD — between waves, NOT SMO'S TO RELEASE). Per the file's own
explicit preamble — *"ABSENCE FROM THIS FILE MEANS FREE, NOT HELD… A plugin named in neither list
was never claimed by this ticket and needs no clearance from it — proceed"* — and per
`important.md`'s named correction (`🏗️fem` listed by name as one of five plugins wrongly stopped
on this exact misreading) — **`🏗️fem` is free. Proceeded.**

## What I found on arrival — a migration already mid-flight, left broken

Before touching anything I ran the standard census (`ls -a`, `git log --oneline -5 -- <path>`,
`stat -f '%Sm'`). The plugin root's eight compute dirs (`➗️formulation`, `🏗️model`,
`📏️elements2d`, `🔢️sparse`, `🕸️mesh`, `🖥️app-surface`, `🧊️elements3d`, `🧮️analyses`) were
already **empty** (`ls -a` → only `.`/`..`, dir mtimes `Aug 12 17:59`), and
`git log --oneline -3 -- "✏️s/🔌️plugins/🏗️fem"` showed the most recent commit,
`fd01661f06…495` (Aug 12 18:08:12), had already **renamed** all eight files' single
`🦀️component.rs` each into `🗿️artifacts/{◻2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/<name>/…`
(five into `◻2d`/fem2d: `formulation`, `model`, `elements2d`, `sparse`, `analyses`; two into
`🧊️3d`/fem3d: `mesh`, `elements3d`) and `🖥️app-surface` into
`🎛️apps/◻2d/⚙️engine/🖥️app-surface/🦀️component.rs` — exactly the split and the app-surface
exception this dispatch specifies, already correctly decided by whichever concurrent session
committed it (confirmed via `git show fd01661f06 --name-status -M100%`, all renames R100).

**But `📦️packages/🦀️rust/📦️glue.rs`'s `//#region 🏗️Kernel modules` (lines 29-45) still pointed at
the eight now-nonexistent plugin-root paths** (`#[path = "../../🏗️model/🦀️component.rs"]` etc.) —
the same commit that moved the files touched `glue.rs` too (a plain `M`, not part of the renames)
but only added the new `🗿️Artifacts`/`🎛️Apps` shim regions; it never repointed this one older
region. Confirmed exhaustively with a Python `#[path]` resolution sweep (script below) before any
edit: **258 non-self `#[path]` attributes total, 8 missing, all 8 in this one region.** This is a
dangling workspace-crate compile error waiting to happen (the target file must exist for `mod X;`
to compile, independent of whether `X` is referenced elsewhere) — a genuinely broken intermediate
state left by a concurrent APA/W3 session, not something I introduced.

## What I changed

**`✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📦️glue.rs`, `//#region 🏗️Kernel modules` (was lines
30-45)** — repointed all 8 `#[path]` targets to the locations the git rename had already
established, **keeping every crate-root module name unchanged** (`pub mod model;`,
`pub mod analyses;`, `pub mod elements2d;`, `pub mod elements3d;`, `pub mod formulation;`,
`pub mod mesh;`, `pub mod sparse;`, `pub mod app_surface;`) — trinity's technique, exactly as the
dispatch instructed:

| Module | Old (dangling) `#[path]` | New `#[path]` |
|---|---|---|
| `model` | `../../🏗️model/🦀️component.rs` | `../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🏗️model/🦀️component.rs` |
| `analyses` | `../../🧮️analyses/🦀️component.rs` | `../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🧮️analyses/🦀️component.rs` |
| `elements2d` | `../../📏️elements2d/🦀️component.rs` | `../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/📏️elements2d/🦀️component.rs` |
| `elements3d` | `../../🧊️elements3d/🦀️component.rs` | `../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🧊️elements3d/🦀️component.rs` |
| `formulation` | `../../➗️formulation/🦀️component.rs` | `../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/➗️formulation/🦀️component.rs` |
| `mesh` | `../../🕸️mesh/🦀️component.rs` | `../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🕸️mesh/🦀️component.rs` |
| `sparse` | `../../🔢️sparse/🦀️component.rs` | `../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🔢️sparse/🦀️component.rs` |
| `app_surface` | `../../🖥️app-surface/🦀️component.rs` | `../../🎛️apps/◻2d/⚙️engine/🖥️app-surface/🦀️component.rs` |

Justification for the fem2d/fem3d split (already decided by the prior commit, verified not
second-guessed): `formulation`/`model`/`elements2d`/`sparse`/`analyses` physically sit under
`◻2d` (fem2d), `mesh`/`elements3d` under `🧊️3d` (fem3d) — but because crate-root module names
stayed stable, **physical location does not restrict logical consumer**: I grepped
`app_surface::` and found it used from **both** `🎛️apps/◻2d/**` and `🎛️apps/🧊️3d/**` files
(`🎛️apps/🧊️3d/🎚️config/🦀️component.rs`, `🎛️apps/🧊️3d/🎮️commands/{🏋️loads,🧱️model}/🦀️component.rs`,
`🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`, plus the `◻2d`-side
equivalents) — confirming the "shared cross-app compute lives in one physical home, consumed via
`crate::`-absolute paths from both" pattern trinity's report proved for jack/rewrite is correctly
in effect here too, unaffected by which literal directory backs `app_surface`.

**Removed** the now-empty plugin-root dirs after their one file each had already moved out (my
`rmdir`, not a rename — the files were gone before I arrived): `➗️formulation/`, `🏗️model/`,
`📏️elements2d/`, `🔢️sparse/`, `🕸️mesh/`, `🖥️app-surface/`, `🧊️elements3d/`, `🧮️analyses/`.

### Step 1 — dead facet directories

All three unmounted (`grep -n "🛂️manifest\|🎟️capabilities\|🔧️setup" 📦️glue.rs` → zero hits,
verified before deleting) and doc-only (1-line stubs, no real code, no JSON fixture data present —
`find` on each dir returned exactly one file, the stub `🦀️component.rs`):
- `🛂️manifest/🦀️component.rs` — `//! 🛂️ Manifest facet for 🏗️fem — identity surfaces live on
  Plugin::builder in the parent.` — deleted, dir removed.
- `🎟️capabilities/🦀️component.rs` — `//! 🎟️ Capabilities facet for 🏗️fem — declare rights via
  PluginBuilder::capability / .local_backbone_storage().` — deleted, dir removed.
- `🔧️setup/🦀️component.rs` — `//! 🔧️ Setup facet for 🏗️fem — codec/language/importer
  registration hooked via .setup(...).` — deleted, dir removed.

No plugin-root `.DS_Store`/`node_modules` present (checked, none to remove).

### Step 2 — plugin root now closed

```
$ ls -a "✏️s/🔌️plugins/🏗️fem/"
. .. AGENTS.md README.md 🎛️apps 📦️packages 🗿️artifacts 🦀️component.rs
```
Exactly the target shape.

### Step 3 — escape-hatch call sites

```
$ grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_2d_export_handlers\|register_app_io\|register_os_media_" "✏️s/🔌️plugins/🏗️fem" --include="*.rs"
(zero hits)
```
No violation existed in this plugin (matches `📓️w0-a-escape-hatch.md`'s full census, which lists
no fem entries at all). No-op.

### Step 4 — dependency purge

```
$ grep -rn "semio_framework_os::" "✏️s/🔌️plugins/🏗️fem" --include="*.rs"
(zero hits)
$ grep -n "semio-framework-os\b" "✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/Cargo.toml"
(zero hits — only `semio-framework-os-kernel` is present, a different crate, still genuinely used
throughout via the `dsl`/`store`/`pack`/`protocol`/`vcs` extern-crate aliases in 📦️glue.rs:15-20)
```
Nothing to purge. `semio-framework-os-kernel` correctly left untouched.

### Step 5 — inventory only, nothing changed

```
$ grep -rn "thread_local!" "✏️s/🔌️plugins/🏗️fem" --include="*.rs"                        → zero
$ grep -rn "std::fs::\|std::env::\|std::process::\|Command::new(" "…fem" --include="*.rs"  → zero
$ grep -rn "reqwest\|TcpStream\|hyper::\|std::net::" "…fem" --include="*.rs"                → zero
$ grep -rn "fn seed(" "…fem" --include="*.rs"                                               → zero
```
No interior-mutable app state, no filesystem/env/process/network side effects, no `seed` fn
anywhere in the plugin. **Nothing to inventory for the Draft lane** — fem's apps carry no
`thread_local!` scratch of any kind, so there is no genuine-gesture-vs-derived-cache split to
propose and no verb-slugs to suggest. (Consistent with fem being a headless
compute/FEM-analysis library — its own root doc-comment calls itself exactly that.)

Also confirmed: no `🧬️mutations/**` file was touched by this wave (out of scope, another ticket's
— the extensive triad structure already on disk under both artifacts' `🧬️schema/🧬️mutations/`
predates my edits, part of the same concurrent commit that did the file relocation, not authored
by me) and no banned identifier (`SetSnapshot`/`NoMutation`/`CollectionMutation`) was written
anywhere by this wave.

## Step 6 — structural verification (no cargo except the one sanctioned `cargo metadata`)

**1. Plugin root shape** — pasted above under Step 2. Matches target exactly.

**2. Every `#[path = "..."]` in `📦️glue.rs` resolves to a real file — exhaustive, not sampled.**
Before my edit:
```
total (excluding self-mounts): 258
missing: 8
30 ../../🏗️model/🦀️component.rs
32 ../../🧮️analyses/🦀️component.rs
34 ../../📏️elements2d/🦀️component.rs
36 ../../🧊️elements3d/🦀️component.rs
38 ../../➗️formulation/🦀️component.rs
40 ../../🕸️mesh/🦀️component.rs
42 ../../🔢️sparse/🦀️component.rs
44 ../../🖥️app-surface/🦀️component.rs
```
After my edit, re-run of the identical script:
```
total (excluding self-mounts): 258
missing: 0
```
(Python, regex-anchored to `^\s*#\[path\s*=\s*"..."\]\s*$` so doc-comment prose is never
miscounted, `.` self-mounts excluded by design — same method trinity's report used.)

**3. Dangling-reference sweep, repo-wide, for every old path I removed:**
```
$ grep -rln "🏗️fem/➗️formulation\|🏗️fem/🏗️model\|🏗️fem/📏️elements2d\|🏗️fem/🔢️sparse\|🏗️fem/🕸️mesh\|🏗️fem/🖥️app-surface\|🏗️fem/🧊️elements3d\|🏗️fem/🧮️analyses\|🏗️fem/🛂️manifest\|🏗️fem/🎟️capabilities\|🏗️fem/🔧️setup" . | grep -v "🎯️target\|node_modules\|\.git/"
```
Hits, all inspected:
- `📜️script.ts` (repo root), lines 4865-4872 — the **report-mode-only** policy advisory table
  (same class trinity's report flagged for its own plugin). Its 7 entries for fem's compute dirs
  are now stale keys pointing at paths that no longer exist — harmless, never gates a build, will
  simply never match again. Repo-root file, out of my single-plugin boundary — not touched, filed
  under `sharedFileRequests`.
- A dozen+ hits in unrelated, already-closed historical ticket folders (`PLURALIZE-KIND…`,
  `REGISTRY-SCRIPT-REFACTOR…`, `DISSOLVE-CORE-FOLDERS…`, `EMOJI-PREFIX…`,
  `FINISH-FE0F-EMOJI-STYLE…`, plus this ticket's own `📓️w0-b-plugin-shape.md`) — scratch/census
  prose from other tickets or this ticket's own read-only recon doc, not live code, not touched.
- No hits in any `.rs`, `.ts`, `Cargo.toml`, `project.json`, or `package.json` outside the one
  file above.

**4. `cargo metadata` — the one sanctioned cargo command:**
```
$ CARGO_TARGET_DIR=".../ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/🎯️target" cargo metadata --no-deps --format-version 1 >/dev/null 2>stderr.txt && echo OK
OK
```
stderr empty. The workspace graph loads cleanly with fem's crate in its post-edit state — this is
the single strongest evidence available this wave (structural, not a full compile, but it does
parse every workspace member's `Cargo.toml` and would fail loudly on a broken member).

## Files touched

- **Updated**: `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📦️glue.rs` (8 `#[path]` targets repointed in
  `//#region 🏗️Kernel modules`, module names unchanged)
- **Removed**: `✏️s/🔌️plugins/🏗️fem/{➗️formulation,🏗️model,📏️elements2d,🔢️sparse,🕸️mesh,🖥️app-surface,🧊️elements3d,🧮️analyses}/`
  (8 now-empty dirs — their single files had already been moved out by a concurrent commit before
  I arrived; I only removed the leftover empty dirs)
- **Removed**: `✏️s/🔌️plugins/🏗️fem/🛂️manifest/`, `🎟️capabilities/`, `🔧️setup/` (dirs + doc-only
  `🦀️component.rs` each, all confirmed unmounted before deletion)

Not touched by me (pre-existing, part of the same concurrent commit `fd01661f06` that predates
this session): the `🗿️artifacts/{◻2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/✳️any/…` tree contents
themselves (engine files, schema, io, mutations, examples), `🎛️apps/{◻2d,🧊️3d}/**`, root
`🦀️component.rs`, `AGENTS.md`, `README.md`.

## sharedFileRequests

1. **File**: repo-root `📜️script.ts`, lines 4865-4872 (report-mode-only policy advisory table).
   **Reason**: 7 stale keys now name paths that no longer exist (`✏️s/🔌️plugins/🏗️fem/➗️formulation`
   etc.) — harmless (report-mode only, never gates a build), cosmetic cleanup for whoever does the
   W5 pass over that file. Same class of finding trinity's report filed for its own plugin.
   **Patch**: not written (seven one-line key deletions, trivial for the owning session; out of my
   single-plugin boundary).

## Concurrent-churn observations

- `git log --oneline -5 -- "✏️s/🔌️plugins/🏗️fem"` at session start showed the plugin's most recent
  commit (`fd01661f06…495`, Aug 12 18:08:12) had **already** performed the eight-file relocation
  and the fem2d/fem3d artifact-tree restructuring (flat `⚙️engine/` → nested
  `🪆️subsets/✳️any/⚙️engine/`) before I ever opened the directory — i.e. another concurrent
  session did the bulk of this wave's Step 2 work already, correctly, except for leaving
  `📦️glue.rs`'s older `🏗️Kernel modules` region unrepointed. This is exactly the scenario
  `important.md` §2 warns about (`git status` clean ≠ nothing happened) — caught here by `git log`
  + a directory listing showing empty dirs where the census expected files, not by trusting either
  signal alone.
- The commit's own message (`🐙️ueli🎆️26🌙️06☀️04🚩️495`, "Define subset conformance roundtrips
  architecture and parallel migration plan") names an unrelated ticket
  (`.cursor/plans/subset_conformance_roundtrips_c57a3e1a.plan.md`, per repo-root git status) — the
  repo's auto-commit sweeps up whatever is in the working tree at commit time regardless of which
  session or ticket authored it, so the commit message is not reliable evidence of *which* ticket
  did this fem work. Noting for whoever reconciles wave attribution later; did not attempt to
  identify the authoring session.
- Did not observe any further mid-session edit to this plugin's files while I worked (no fresher
  timestamps appeared between my `ls`/`grep` calls).
- Did not touch `🧬️mutations/**` anywhere in the tree (out of scope, confirmed — see Step 5).
- Did not author or touch any `thread_local!`/draft-lane facet (none exist in this plugin).
- Did not rename or re-declare any artifact kind id (`fem2d`/`fem3d`/`◻2d`/`🧊️3d` untouched, no
  `ArtifactKindSpec`/`id:` string added or changed by me).

## apa-status: complete

Step 0 (clearance) re-derived correctly this time. Steps 1-4 either already done by a concurrent
session (verified, not re-done) or genuine no-ops for this plugin (escape-hatch census, dependency
purge). Step 2's one broken piece — 8 dangling `#[path]` mounts left by the concurrent session —
is fixed, verified exhaustively (258/258 resolve), and the 8 emptied plugin-root dirs are removed.
Step 5 inventory is complete and clean (nothing found to report). Step 6 structural evidence is
pasted in full above, including the one sanctioned `cargo metadata` run, which succeeded.

**What the consolidated build should check first for this plugin**: that the 8 repointed
`#[path]` mounts in `📦️glue.rs`'s `🏗️Kernel modules` region actually **compile** (structurally
verified to resolve to real files and to parse via `cargo metadata`, not yet compiler-verified —
cargo check/build/test remain banned this wave per standing order). Given the files themselves
were moved byte-for-byte (git rename, not edited) and no internal `#[path]` or relative-path
assumptions exist inside them (unlikely given trinity's precedent that the four analogous files it
moved had none either), risk here should be low, but this is the one thing this wave could not
compiler-verify.
