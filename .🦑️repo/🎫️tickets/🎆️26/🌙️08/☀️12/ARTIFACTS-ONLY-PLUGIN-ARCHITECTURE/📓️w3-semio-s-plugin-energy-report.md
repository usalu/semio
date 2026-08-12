# W3 — `semio-s-plugin-energy` (🔋️energy) migration report

Clearance check performed before starting: `📓️w0-census.md` (§ "Cross-ticket coordination") flags
`🪐️space`/`🔋️energy` clearance as ambiguous vs SMO's self-contradicting status.md — but APA's own
`📓️status.md:57` resolves this explicitly: *"energy had no report but was explicitly released"*.
SMO's live `📓️plugin-release-status.md` also lists `🔋️energy` under **RELEASED** (`🔋️model` facet,
`♻️replace-model` triad, leaf audited by hand). Proceeded on that basis.

## What changed

### Step 1 — dead facet directories deleted
All three were doc-only markers (single-line `//!` docstring, zero real code) **and** mounted in
`📦️glue.rs` as `plugin_manifest` / `plugin_setup` / `plugin_capabilities` (confirmed unused anywhere
else in the crate or workspace via `grep -rn "plugin_manifest\|plugin_setup\|plugin_capabilities"`).
Deleted the directories and removed their three `#[path]` mounts:

- `✏️s/🔌️plugins/🔋️energy/🛂️manifest/` — removed
- `✏️s/🔌️plugins/🔋️energy/🎟️capabilities/` — removed
- `✏️s/🔌️plugins/🔋️energy/🔧️setup/` — removed
- `📦️packages/🦀️rust/📦️glue.rs` — removed the `pub mod plugin_manifest;` / `pub mod plugin_setup;` /
  `pub mod plugin_capabilities;` lines (previously lines 455–460 in the `//#region 🔖️Plugin` block);
  `pub mod plugin_apps;` kept (real facet, zero-app crate still needs the mount for `.library()`
  parity with other plugins' shape).

No `.DS_Store` / `node_modules` junk found at plugin root.

### Step 2 — plugin-root `⚙️engine` (50 modules) relocated into the artifact
This was the bulk of the packet (census: "the single largest violation"). Moved all 50 domain
module directories, one file each (`🦀️component.rs`), preserving per-file grain — no merges, no
inlining:

`⚙️engine/{air_exchange,air_system,airflow_network,calendar,coils,comfort,controls,curves,daylight,
dispatch,economics,electrical,envelope,error,evaporative,fans,faults,fenestration,gains,geometry,
heat_recovery,humidity_eq,hvac_topo,iaq,ideal_hvac,kernel,material,meters,metrics,model,num,output,
plant,precompute,props,refrigeration,results,room_air,schedule,shw,sim,site,sizing,solar,
solar_thermal,terminal,units,water,zone_air,zone_hvac}/🦀️component.rs`

→ moved to
`🗿️artifacts/🔋️model/🏅️standards/🔖️1/⚙️engine/<module>/🦀️component.rs` for each — sitting alongside
the pre-existing `🗿️artifacts/🔋️model/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (the artifact's
registration/engine facade, untouched) and `🟦️component.ts` sibling. This is exactly the taxonomy's
own `standardChildDirs: ["⚙️engine","🪆️subsets"]` shape, one level down from where the 50 modules
sat before.

The plugin-root `⚙️engine/` directory is now empty and was removed (`rmdir`).

`📦️packages/🦀️rust/📦️glue.rs` — all 50 `#[path]` mounts in the `//#region ⚙️Engine` block
(lines 30–129) repointed from `../../⚙️engine/<module>/🦀️component.rs` to
`../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/⚙️engine/<module>/🦀️component.rs` (done with a scripted
prefix replace, `grep -c` confirmed exactly 50 occurrences replaced, `grep` afterward for the old
`"../../⚙️engine` prefix returns zero hits). The `//#region 🔖️FlatReExports` block (`pub use
air_exchange::*` etc., lines ~132–182) needed no change — those reference the `pub mod` names, not
paths. File header docstring (lines 1–19) updated to describe the new location instead of the
stale plugin-root one; no behavioral change.

None of the 50 moved files contain their own internal `#[path]` attributes (`grep` confirmed) — they
are leaf files, all wiring lives centrally in `glue.rs`, so no further repointing was needed inside
them.

### Step 2 — plugin root now closed
`ls -a "✏️s/🔌️plugins/🔋️energy/"` → `AGENTS.md`, `🎛️apps`, `📦️packages`, `🗿️artifacts`,
`🦀️component.rs`. (No `README.md` exists for this plugin — pre-existing, `AGENTS.md` fills that
role; left untouched per instructions.) All five entries are within the allowed six.

### Step 3 — escape-hatch call sites
None found. `grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_2d_export_handlers\|register_app_io\|register_os_media_"`
over the whole plugin directory returns zero hits. `📓️w0-a-escape-hatch.md` also lists no
energy call sites. No-op.

### Step 4 — dependency purge
Already clean. `📦️packages/🦀️rust/Cargo.toml` depends only on `semio-framework-os-kernel` (not the
forbidden `semio-framework-os` OS-host crate) — confirmed by reading the `[dependencies]` block and
by `grep -rn "semio_framework_os::"` (excluding `_os_kernel`) returning zero hits in the plugin. No
change needed.

### Step 5 — inventory (no edits made, as instructed)
- **`thread_local!` / interior-mutable app state**: none. `grep -rn "thread_local!"` → zero hits.
  This plugin has **zero apps** (`.library()` build, confirmed in `🦀️component.rs`), so there is no
  Draft-lane debt to inventory — no per-app `thread_local!` scratch exists to convert.
- **Interior mutability found (not app state)**: two `std::sync::OnceLock`-backed statics —
  `🗿️artifacts/🔋️model/🦀️component.rs:45` (`static ENTRIES: OnceLock<Vec<&'static ComposerEntry>>`)
  and `🗿️artifacts/🔋️model/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (`static ENTRIES: OnceLock<Vec<ComposerEntry>>`
  inside `pub mod io_registry`). Both are lazy-initialized, write-once registration-table caches
  (`ComposerEntry` rows for import/export dialect wiring), not mutable draft/scratch state — no
  Draft-lane action implied.
- **`std::fs` / `std::env` / `std::process` / `Command::new`**: none, anywhere, in or out of
  `#[cfg(test)]`.
- **Network calls** (`reqwest`, `TcpStream`, `hyper::`, `std::net::`): none.
- **`fn seed(`**: none.

### Plugin-specific note — zero apps
Per the packet's own callout: this plugin builds via `.library()`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs:138`, docstring "Finishes
a library-only plugin (no apps) — used by headless crates like energy"), which is a first-class,
already-supported framework shape — not a violation to invent an app for. The `🎛️apps/` directory
already exists at plugin root with only its doc-only marker file (`🦀️component.rs`, one line,
mounted as `plugin_apps` in glue) — this predates my session and I left it exactly as-is; I did not
invent an app. If a taxonomy/policy gate later complains that this plugin "declares no apps," that
is a gate design question for W2/W5 (report-mode taxonomy seal), not something fixable from inside
a zero-app library plugin — flagging per instructions rather than inventing one.

## Files touched

**Removed:**
- `✏️s/🔌️plugins/🔋️energy/🛂️manifest/🦀️component.rs` (+ dir)
- `✏️s/🔌️plugins/🔋️energy/🎟️capabilities/🦀️component.rs` (+ dir)
- `✏️s/🔌️plugins/🔋️energy/🔧️setup/🦀️component.rs` (+ dir)
- `✏️s/🔌️plugins/🔋️energy/⚙️engine/` (50 subdirs, now empty, `rmdir`'d)

**Moved** (50 files, git will show as delete+add or rename depending on detection):
- `✏️s/🔌️plugins/🔋️energy/⚙️engine/<module>/🦀️component.rs` →
  `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/⚙️engine/<module>/🦀️component.rs`
  for `<module>` ∈ the 50 listed in Step 2 above.

**Updated:**
- `✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📦️glue.rs` — removed 3 dead facet mounts, repointed 50
  `#[path]` attributes, refreshed the header docstring.

## Verification commands

1. Baseline (step 0), launched **before** any edit:
   `cd "/Users/ueli/Documents/semio" && CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-s-plugin-energy 2>&1 | tail -20`
   — did not get CPU time before I had to start editing (the shared build lock across ~30
   concurrent agent sessions never let it start compiling; `ps` showed `0:00.24` cumulative CPU
   minutes after it had been alive 10+ minutes). **No true pre-edit baseline was captured** — see
   honest caveat below.

2. Post-edit check — **[FILL IN ONCE THE BACKGROUND RUN COMPLETES — see live status below]**

3. `cargo test -p semio-s-plugin-energy --lib` — **[FILL IN]**

4. `bun nx run @semio-tech/energy-plugin:test-quick` — target exists in
   `📦️packages/🦀️rust/📋️project.json` (`test`, `test-quick`, `test-long`, `test-exhaustive`, all
   routed through `bun ./📜️script.ts test <arg>`). **[FILL IN result once run]**

5. `ls -a "✏️s/🔌️plugins/🔋️energy/"` →
   ```
   .
   ..
   AGENTS.md
   🎛️apps
   📦️packages
   🗿️artifacts
   🦀️component.rs
   ```
   Confirmed closed: only the allowed entries remain (no README.md exists for this plugin, which is
   fine — AGENTS.md is the doc file present).

## sharedFileRequests

1. **File**: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs:5`
   **Region**: top-of-file doc comment.
   **Reason**: references the pre-move path `✏️s/🔌️plugins/🔋️energy/⚙️engine/site/🦀️component.rs` in
   prose ("unlike energy's plugin-side `EpwWeather::parse`
   (`✏️s/🔌️plugins/🔋️energy/⚙️engine/site/🦀️component.rs`)"). Now stale — the real path is
   `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/⚙️engine/site/🦀️component.rs`. This
   file is inside `🗄️stdio`, which is UCAS's exclusive claim right now — out of my boundary, not
   touched. One-line doc-comment fix, no functional impact.
   **Patch**: not written (single-line prose fix, trivial for the owning session to apply directly).

2. **File**: `✏️s/🔌️plugins/🔋️energy/📦️packages/🟦️typescript/📦️index.ts`
   **Region**: whole file (12 `export * as … from "../../🗿️artifacts/🔋️model/…"` lines).
   **Reason**: pre-existing and unrelated to this wave's `⚙️engine` relocation — every path in this
   file (e.g. `../../🗿️artifacts/🔋️model/🧬️schema/🟦️component.ts`,
   `../../🗿️artifacts/🔋️model/🪓️decomposer/🟦️component.ts`) targets a flat
   `🗿️artifacts/🔋️model/<facet>/` shape that no longer exists on disk — the real tree is nested under
   `🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/…`. `stat` shows this file was last
   touched 2026-08-12 10:50, well before my session (~15:49) and before I touched anything — this
   predates APA W3 and looks like fallout from an earlier standards-versioning restructure (not this
   ticket's doing). Not touched: out of this packet's scope (Step 2 only covers `⚙️engine`), and
   fixing the TS facade correctly requires knowing the intended WASM-facade shape post-restructure,
   which is a call for whichever session owns the artifact schema layout (SMO/UCAS), not a
   mechanical path rename I should guess at under APA's narrow W3 scope.

## Concurrent-churn observations

- At the time of step 0/step 6, `ps aux | grep "cargo check -p"` showed **~30 concurrent cargo check
  processes** sharing the same `CARGO_TARGET_DIR` build lock (plugins observed: mathematical,
  puzzle, trinity, remodel, sequence, raster, note, energy, dag, space, sourcing, playbook, and
  others) — consistent with `📌️important.md` rule 5 ("the lock serializes concurrent agents...
  normal, wait it out"). My own baseline check for `semio-s-plugin-energy` (pid 93922, launched
  ~15:49) had accumulated essentially zero CPU time after 10+ minutes wall-clock, i.e. it never got
  past waiting for the lock in that window.
- No error observed originating from my own plugin's paths in any partial output — there simply was
  no output yet at report-drafting time.
- Repo auto-commit already picked up my moves/edits (`git log --oneline -5` on the plugin dir shows
  a fresh commit `1caac91709` on top of the pre-session HEAD `a46ac1f883`) — confirms rule 2, no
  manual git command was run by me.

## apa-status

**[FILL IN once verification completes: complete | partial]**
