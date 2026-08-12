# APA Wave 3 — `📐️cad` (crate `semio-s-plugin-cad`) migration report

## Correction to the prior entry in this file

A previous agent wrote "CLEARANCE REFUSED" here under the old, stricter Step-0 reading ("if not in
RELEASED, treat as HELD"). That reading was superseded — the corrected rule (📌️important.md §"How
to read that file — ABSENCE MEANS FREE, not held", and SMO's own predicate file, which now carries
an explicit banner: *"ABSENCE FROM THIS FILE MEANS FREE, NOT HELD... A plugin named in neither list
was never claimed by this ticket and needs no clearance from it — proceed"* and names `📐️cad`
explicitly as one of the five plugins wrongly skipped) makes `📐️cad` **FREE**. Re-checked at the
start of this session: `grep -n "cad" 📓️plugin-release-status.md` still returns no hits — absent
from RELEASED, absent from both HELD tables, absent from NOT SMO'S TO RELEASE. Proceeding per the
corrected rule. This report replaces the prior "blocked" entry; the previous agent's caution was
reasonable under the instruction as given, and the fault was the instruction, not the agent.

## What changed

### Step 1 — dead facet directories deleted
`🛂️manifest/🦀️component.rs`, `🎟️capabilities/🦀️component.rs`, `🔧️setup/🦀️component.rs` were all
single-line doc-only stubs. Confirmed unmounted: `grep -n "🛂️manifest\|🎟️capabilities\|🔧️setup"
📦️packages/🦀️rust/📦️glue.rs` → zero hits (no `#[path]` mount references either name anywhere in
glue). Deleted all three directories outright — no glue edit needed since nothing pointed at them.

Also deleted plugin-root `node_modules/` (vitest/vite build cache only — `.vite`, `.vite-temp`
subdirs, no source). No `.DS_Store` present at plugin root.

### Step 2 — plugin root closed; extra dirs relocated
Before: `AGENTS.md, README.md, node_modules, 🎛️apps, 🎟️capabilities, 📦️packages, 🔣️machine.json,
🔧️setup, 🔨️modules, 🖼️assets, 🗿️artifacts, 🛂️manifest, 🦀️component.rs, 🧩️extensions, 🧫️fixtures`.

After: `AGENTS.md, README.md, 🎛️apps, 📦️packages, 🗿️artifacts, 🦀️component.rs, 🧩️extensions`.

`🧩️extensions` is the one remaining entry beyond the canonical six — **sanctioned exception**, not
a residual violation. Verified all 4 subdirs (`🏛️aec-building-structure`, `🏢️aec-building`,
`📐️spatial-shape`, `🔥️aec-building-energy`) carry `[package.metadata.semio] role = "extension"`,
`extends = "cad"` in their own `Cargo.toml` — real Cargo workspace members, moving them changes
crate paths and workspace membership, out of this wave's scope per the extensions exception in the
dispatch packet and `📓️w0-b-plugin-shape.md` §6. Left untouched, inventoried only.

Relocations performed (`mv`, per-file grain preserved, no inlining):

1. **`🖼️assets/` (211 files) → `🗿️artifacts/📐️cad/📚️examples/🖼️assets/`** — moved as a unit
   exactly as instructed. File count verified identical before/after (211).
2. **`🧫️fixtures/` (1 file) → `🗿️artifacts/📐️cad/📚️examples/🧫️fixtures/`** — kept as its own
   sibling dir (not merged into `🖼️assets`) because a Cargo-metadata static route
   (`/cad-fixture`) points at it specifically; merging would have required splitting the route.
3. **`🔣️machine.json` (210KB, generated) → `🗿️artifacts/📐️cad/📚️examples/🔣️machine.json`** —
   read the file: `"kind": "spatial.stately-machine-view/v1"`, a generated Mermaid/Stately
   catalog view built by `bun ./📜️script.ts generate` (📦️packages/🟦️typescript/📜️script.ts
   `GenerateScript`) from the interaction-spec + stately modules. Purely a generate-time output,
   never read back by runtime code (`grep -rn "machine\.json"` outside the generator/doc-comment
   → zero other hits) — classification resolved (census had flagged it "CANNOT CLASSIFY"),
   relocated alongside the other example/reference artifacts and the generator's default `--out`
   updated to match (see below). Not regenerated — moved as-is, no upcast.
4. **`🔨️modules/` (14 files) split by content, per Step 2's own rule (compute → artifact engine;
   app-surface → app engine):**
   - 13 files (`🎬️actions`, `🎰️stately`, `🏃️runtime`, `📄️artifact`, `📐️brepjs` +AGENTS.md,
     `📐️geometry`, `📔️registry`, `🔍️query`, `🗺️spatial`, `🟦️index.ts`, `🧪️tests`, `🧬️typology`)
     → `🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/<name>/` — compute/query/state
     logic for the cad artifact, now siblings of the pre-existing Rust engine submodules
     (`📥️geometry-import`, `🔄️transformation`, `🔍️construct`, `🕹️interaction`) at the same
     `⚙️engine` level.
   - `📺️renderer/🟦️component.tsx` (R3F UI component, 7645 lines) → `🎛️apps/📐️cad/⚙️engine/📺️renderer/`
     — app-surface UI, not artifact compute; the app's `⚙️engine/` dir already existed (empty) and
     is exactly the taxonomy's own `appChildDirs` slot for this.
   - `🔨️modules/` itself is now empty; removed with `rmdir`.

### Step 3 — escape-hatch call sites
`grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_2d_export_handlers\|
register_app_io\|register_os_media_" ✏️s/🔌️plugins/📐️cad/ --include="*.rs"` → **zero hits**,
confirming `📓️w0-a-escape-hatch.md`'s finding that cad registers no IO handlers of its own
(`"3d.cad"` is currently served only by `🎪️demonstrator/🎪️panes/📐️koordinator`, a foreign-plugin
violation that belongs to demonstrator's own wave, not this one). **Per the dispatch note, did not
write new IO for `"3d.cad"`** — that would be inventing scope, not converting an existing call
site. No-op for this step.

### Step 4 — dependency purge
`grep -rn "semio_framework_os::" ✏️s/🔌️plugins/📐️cad/ --include="*.rs"` → zero hits (both before
and re-verified after all edits). Removed the unused `semio-framework-os = { ... }` line from
`📦️packages/🦀️rust/Cargo.toml` (was declared, zero uses — matches census's "UNVERIFIED why
declared" for cad/writer/trinity/draw; for cad the answer is simply dead weight). Kept
`semio-framework-os-kernel` (still used, different crate).

### Step 5 — inventory only, nothing authored
- **`thread_local!`**: exactly one site, `🎛️apps/📐️cad/🦀️component.rs:947-949`:
  ```rust
  thread_local! {
      static CAD_PREVIEW_SEQ: std::cell::RefCell<u64> = std::cell::RefCell::new(0);
  }
  ```
  A `u64` monotonic tick counter backing `CadPlayApp::gesture_preview` (`:955-967`), used only to
  stamp a "staleness sequence" on a live rubber-band preview payload built from
  `config.engagement_session_json` — the app's own doc-comment (`:940-945`) already states every
  other former TLS field was migrated to `CadConfig`/`CadConfigMutation` and this counter is "the
  sole surviving interior-mutable field... not app state." This matches `📓️draft-lane-spec.md`'s
  own inventory (cad flagged as "smallest; a good first exemplar"). **Not touched** — draft-lane
  authoring is explicitly out of bounds for this session (⛔ inventory only).
  - Proposed (not authored) Draft shape: a `preview_seq: u64` field is arguably not draft
    *content* at all — it never round-trips through a snapshot/diff, it is a monotonic staleness
    stamp derived from config state that already IS the draft-equivalent value
    (`engagement_session_json`). Likely candidate for an **inference** (derived, recomputed) rather
    than a `Draft` field, or folded away entirely once real draft state exists for the engagement
    session. Flagging for SMO/whoever authors cad's draft facet, not deciding here.
- **`Mutex`/`OnceLock`** (sanctioned-location check): one site,
  `🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:851-852`:
  ```rust
  fn last_cad_computer_contributions_json() -> &'static Mutex<String> {
      static SLOT: OnceLock<Mutex<String>> = OnceLock::new();
  ```
  Lives inside `🗿️artifacts` — sanctioned location per `📓️w0-census.md` Class C. A write-once
  registration-style cache (contributed extension catalog JSON), not draft/session state.
- **`std::fs`**: `🗿️artifacts/📐️cad/🎬️interaction-spec/🦀️component.rs:640` (`std::fs::read_dir`)
  and `:655` (`std::fs::read_to_string`) — both inside `#[cfg(test)]` (the enclosing `mod tests`
  starts well before line 589; every hit sits inside `every_interaction_asset_on_disk_parses_as_
  interaction_spec`, a regression test walking the example-asset tree). **Inventory only, per
  dispatch instruction** — did not refactor the pattern. Did, however, update the *path literals*
  these tests and ~15 `include_str!` sites in the same file pass to `std::fs`/`include_str!`,
  because the directory they point at (`🖼️assets`) moved in this same wave — leaving them pointing
  at a directory that no longer exists would be strictly worse than "inventory only," not more
  conservative. See the reference-repointing list below.
- **`std::env` / `std::process` / `Command::new`**: zero hits anywhere in the plugin (main crate
  and the 4 extension crates).
- **Network calls** (`reqwest`, `TcpStream`, `hyper::`): zero hits.
- **`fn seed(`**: zero hits.

### Step 6 — structural verification (no cargo, per standing instruction)

1. **Closed root shape**:
   ```
   $ ls -a "✏️s/🔌️plugins/📐️cad/"
   .  ..  AGENTS.md  README.md  🎛️apps  📦️packages  🗿️artifacts  🦀️component.rs  🧩️extensions
   ```
   Six canonical entries + the one sanctioned `🧩️extensions` exception (role="extension" crates,
   inventoried not moved — see Step 2).

2. **Every `#[path]` mount in `📦️glue.rs` resolves to a real file** — checked exhaustively with a
   script (not by eye), since the dispatch flagged this as the highest-risk failure mode:
   ```
   total #[path] mounts (non-'.'): 147
   ALL RESOLVE OK
   ```
   (Python: parse every `#[path = "..."]` literal out of glue.rs, `os.path.normpath(join(crate_dir,
   literal))`, assert `os.path.isfile`. Zero missing.)

3. **No dangling references to anything moved or removed**:
   - `grep -rln "🔨️modules"` across the whole plugin tree → all remaining hits are false
     positives from the *unrelated* framework path `🧰️framework/🔨️modules/...` (a different,
     framework-owned directory that happens to share the segment name) — verified line-by-line,
     none reference cad's own now-deleted `🔨️modules/`.
   - `grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_2d_export_handlers\|
     register_app_io\|register_os_media_"` → zero (re-verified post-edit, unchanged from Step 3).
   - `grep -rn "semio_framework_os::"` → zero (re-verified post-edit).
   - Spot-resolved 4 representative moved-asset paths from their new referencing locations
     (`os.path.isfile` after literal substitution) — all four resolved: a `🖼️assets` interaction
     JSON from `🎬️interaction-spec`'s new relative path, the renderer's new engine-index import,
     the renderer's new asset path, and the `🧪️tests` module's new asset path.

4. **Every moved file exists as its own file at its new path** — confirmed via `find` listing
   (15 subdirectories now under the artifact's `⚙️engine/` — the 4 pre-existing Rust submodules
   plus the 11 relocated TS module dirs; `🟦️index.ts` and `📐️brepjs/AGENTS.md` both present;
   211/211 asset files present at the new location; nothing pasted into `glue.rs` or any parent
   module).

## Reference repointing (full list, file → what changed)

Because the directory move touched a live, cross-referenced TS bundle plus Rust `include_str!`
sites, every referencer was found by grep and repointed with an exact relative-path computation
(`os.path.relpath`, not hand-counted, then spot-verified against disk):

| File | What changed |
|---|---|
| `📦️packages/🟦️typescript/tsconfig.json` | `include` array: 6 `../../🔨️modules/...` entries → new `⚙️engine`/app-engine paths |
| `📦️packages/🟦️typescript/🧪️vitest.config.ts` | `DOMAIN_FILES` (6 entries) + `environmentMatchGlobs` (renderer jsdom match) repointed |
| `📦️packages/🟦️typescript/📋️project.json` | `namedInputs.default` globs repointed to the two new locations (artifact `⚙️engine/**` + app `⚙️engine/📺️renderer/**`) |
| `📦️packages/🟦️typescript/📜️script.ts` | `dependencyBoundaryBreachesForBundleDir` target dir → new artifact-engine path (renderer now out of this lint's scope, noted in an updated doc-comment — see "Design note" below); 3 dynamic `import()` targets in `GenerateScript`; `machine.json` default `--out` |
| `📦️packages/🦀️rust/Cargo.toml` | `sourceRoots` (storybook) 3 entries repointed; `/cad-fixture` static-asset `root` repointed; `semio-framework-os` dependency line removed (Step 4) |
| `🗿️artifacts/📐️cad/🎬️interaction-spec/🦀️component.rs` | 12 `include_str!("../../../🖼️assets/...")` → `"../📚️examples/🖼️assets/..."`; 1 `CARGO_MANIFEST_DIR.join("../../🖼️assets/...")` → `"../../🗿️artifacts/📐️cad/📚️examples/🖼️assets/..."` |
| `🗿️artifacts/…/⚙️engine/🦀️component.rs` (root engine facade) | 1 `include_str!` prefix repointed |
| `🗿️artifacts/…/⚙️engine/📥️geometry-import/🦀️component.rs` | 5 `include_str!` prefixes repointed |
| `🗿️artifacts/…/⚙️engine/🕹️interaction/🦀️component.rs` | 49 `include_str!` prefixes repointed (one script, exact-count verified against the 49 pre-existing occurrences) |
| `🗿️artifacts/…/⚙️engine/🧪️tests/🟦️component.ts` | asset-path prefix repointed (1); **pre-existing dangling import fixed**: `"../📄️document/🟦️component.ts"` → `"../📄️artifact/🟦️component.ts"` (the folder was already named `📄️artifact`, not `📄️document`, before this session touched anything — same repo-wide `📄️document→📄️artifact` rename fallout `📓️w0-census.md` §6 flags for other plugins; fixed here since it's inside a file I was already relocating, one line, unambiguous, in-boundary) |
| `🗿️artifacts/…/⚙️engine/📐️brepjs/🟦️component.ts` | asset-path prefix repointed (1) |
| `🗿️artifacts/…/⚙️engine/🏃️runtime/🟦️component.ts` | asset-path prefix repointed (12 `import.meta.glob`/`join` call sites) |
| `🎛️apps/📐️cad/⚙️engine/📺️renderer/🟦️component.tsx` | barrel import (`../🟦️index.ts`) and brepjs import (`../📐️brepjs/...`) repointed across the new app↔artifact boundary; 1 asset-path prefix repointed |

All internal cross-imports *among* the 13 co-located engine-dir files (e.g. `🏃️runtime` importing
`../🟦️index.ts`, `📐️brepjs` importing `../🟦️index.ts`) needed **no changes** — they moved together
as a unit and stayed siblings, so their existing relative paths remained correct by construction.
Only the one file that crossed the app/artifact boundary (`📺️renderer`) and the asset-referencing
lines needed recomputed prefixes.

## Design note — one dependency-boundary lint now covers less

`📦️packages/🟦️typescript/📜️script.ts`'s `policy` lint (`dependencyBoundaryBreachesForBundleDir`)
previously scanned all 6 "domain files" (the 13-ish source files) as one bundle, including
`📺️renderer`. Since renderer moved out to app-surface (a different architectural layer with a
different appropriate dependency policy than a headless compute library), it no longer shares a
bundle boundary with the artifact-engine files. Updated the lint's target dir to the artifact
engine location and documented the renderer exclusion in the function's doc-comment rather than
silently narrowing its coverage. This is a judgment call, not a mechanical rename — flagging for
visibility. No new lint was written for renderer; app-surface files elsewhere in the repo are not
independently boundary-linted either, so this keeps cad consistent with the rest of the fleet.

## Files touched

**Removed:**
- `✏️s/🔌️plugins/📐️cad/🛂️manifest/` (dir + doc-stub file)
- `✏️s/🔌️plugins/📐️cad/🎟️capabilities/` (dir + doc-stub file)
- `✏️s/🔌️plugins/📐️cad/🔧️setup/` (dir + doc-stub file)
- `✏️s/🔌️plugins/📐️cad/node_modules/` (build cache)
- `✏️s/🔌️plugins/📐️cad/🔨️modules/` (now-empty dir, after its 14 children moved)

**Moved** (per-file grain preserved throughout):
- `🖼️assets/**` (211 files) → `🗿️artifacts/📐️cad/📚️examples/🖼️assets/**`
- `🧫️fixtures/🖼️concrete-forest-reference.png` → `🗿️artifacts/📐️cad/📚️examples/🧫️fixtures/…`
- `🔣️machine.json` → `🗿️artifacts/📐️cad/📚️examples/🔣️machine.json`
- `🔨️modules/{🎬️actions,🎰️stately,🏃️runtime,📄️artifact,📐️brepjs,📐️geometry,📔️registry,🔍️query,
  🗺️spatial,🧪️tests,🧬️typology}/` (+`🟦️index.ts`) →
  `🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/<same-name>/`
- `🔨️modules/📺️renderer/` → `🎛️apps/📐️cad/⚙️engine/📺️renderer/`

**Updated** (reference repointing + Step 4 purge — full list in the table above):
- `📦️packages/🟦️typescript/tsconfig.json`, `🧪️vitest.config.ts`, `📋️project.json`, `📜️script.ts`
- `📦️packages/🦀️rust/Cargo.toml`
- `🗿️artifacts/📐️cad/🎬️interaction-spec/🦀️component.rs`
- `🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`
- `🗿️artifacts/…/⚙️engine/📥️geometry-import/🦀️component.rs`
- `🗿️artifacts/…/⚙️engine/🕹️interaction/🦀️component.rs`
- `🗿️artifacts/…/⚙️engine/🧪️tests/🟦️component.ts`
- `🗿️artifacts/…/⚙️engine/📐️brepjs/🟦️component.ts`
- `🗿️artifacts/…/⚙️engine/🏃️runtime/🟦️component.ts`
- `🎛️apps/📐️cad/⚙️engine/📺️renderer/🟦️component.tsx`

**Left untouched (inventoried, not moved/edited):**
- `🧩️extensions/{🏛️aec-building-structure,🏢️aec-building,📐️spatial-shape,🔥️aec-building-energy}/`
  — role="extension" Cargo crates, out of scope per the extensions exception.
- `🎬️interaction-spec/🦀️component.rs:640,655` — `std::fs` pattern itself (only its path *literals*
  were updated, per above).
- `🎛️apps/📐️cad/🦀️component.rs:947-949` — `CAD_PREVIEW_SEQ` thread_local (draft-lane inventory
  only, not authored).
- `📦️packages/🟦️typescript/📦️index.ts` — pre-existing, unrelated stale flat paths
  (`../../🗿️artifacts/📐️cad/🧬️schema/…`, `…/🪓️decomposer/…`, `…/🚪️io/…`) that predate this
  session and don't match the current nested `🏅️standards/🔖️1/🪆️subsets/✳️any/…` tree at all —
  same phenomenon `📓️w3-semio-s-plugin-energy-report.md` filed as a sharedFileRequest (fixing
  requires knowing the intended WASM-facade shape post-restructure, a call for whoever owns the
  artifact schema layout, not a mechanical rename this wave should guess at). Filed below.

## Verification commands run, with real output

```
$ ls -a "✏️s/🔌️plugins/📐️cad/"
.  ..  AGENTS.md  README.md  🎛️apps  📦️packages  🗿️artifacts  🦀️component.rs  🧩️extensions

$ python3 <parse #[path]="..." out of glue.rs, assert os.path.isfile for each>
total #[path] mounts (non-'.'): 147
ALL RESOLVE OK

$ grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_2d_export_handlers\|register_app_io\|register_os_media_" ✏️s/🔌️plugins/📐️cad --include="*.rs"
<no output>

$ grep -rn "semio_framework_os::" ✏️s/🔌️plugins/📐️cad --include="*.rs"
<no output>

$ grep -rln "🔨️modules" ✏️s/🔌️plugins/📐️cad
<only 🧰️framework/🔨️modules/... hits — unrelated framework path, verified by hand>

$ find 🗿️artifacts/📐️cad/📚️examples/🖼️assets -type f | wc -l
211   # matches pre-move count exactly
```

**Cargo verification intentionally deferred** — per standing instruction, no `cargo` was run
(shared build lock + `semio-framework-plugin` currently red from a peer session's in-flight
`self.children`/E0499 work; a cargo run right now would tell this session nothing about its own
correctness and only burn the shared lock). All verification above is structural: path-mount
resolution, exhaustive grep, and directory/file-count reconciliation.

## Inventory summary (Step 5, restated compactly)

| Kind | Location | Notes |
|---|---|---|
| `thread_local!` | `🎛️apps/📐️cad/🦀️component.rs:947` | `RefCell<u64>` preview-tick counter; likely inference/derived, not draft content — see Step 5 above |
| `Mutex`/`OnceLock` | `🗿️artifacts/…/⚙️engine/🦀️component.rs:851-852` | sanctioned location, write-once contribution-catalog cache |
| `std::fs` | `🎬️interaction-spec/🦀️component.rs:640,655` | inside `#[cfg(test)]`, regression test only |
| `std::env`/`process`/`Command::new` | none | — |
| network | none | — |
| `fn seed(` | none | — |

## sharedFileRequests

1. **File**: `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📦️index.ts` (whole file, 13 `export *`
   lines). **Reason**: pre-existing stale flat paths (`../../🗿️artifacts/📐️cad/🧬️schema/…`, etc.)
   that don't match the actual nested `🏅️standards/🔖️1/🪆️subsets/✳️any/…` tree — predates this
   session (not touched by Step 2's relocation, since Step 2 only covers the extra plugin-root
   dirs, not the pre-existing WASM facade). Same class of issue energy's report filed. **Not
   fixed**: correcting it requires knowing the intended WASM-facade shape post the standards-
   versioning restructure that produced this drift — a call for whoever owns cad's artifact schema
   layout (SMO/UCAS), not a mechanical path substitution this wave should guess at. **Patch**: not
   written (13-line facade rewrite, needs an owner decision first, not a diff).
2. **Not a file request, an observation**: `🎪️demonstrator/🎪️panes/📐️koordinator/🦀️component.rs`
   (10 `register_*` calls registering `"3d.cad"`, foreign-plugin escape-hatch, per
   `📓️w0-a-escape-hatch.md` §2d) is the only IO handler `"3d.cad"` currently has anywhere in the
   repo. Deleting it (demonstrator's own wave) without first authoring real IO under cad's own
   `🚪️io` tree will leave `"3d.cad"` import/export completely unhandled — flagging for whoever
   picks up demonstrator's wave and/or a future cad IO-authoring wave, per the dispatch's explicit
   instruction not to write that IO myself this wave.

## Concurrent-churn observations

- Disk pressure observed mid-session (`/System/Volumes/Data` at 100% capacity, 119Mi free) caused
  one transient tool-output write failure (`ENOSPC`) on an unrelated `ls` call; retried
  successfully immediately after with no data loss — not caused by this session's own edits (moves
  are renames, not copies; net new bytes written were the small text-file diffs above). Consistent
  with `📌️important.md` rule 5's shared-build-lock congestion — many concurrent sessions' `target/`
  dirs likely account for it, not this wave.
- `git log --oneline -5 -- ✏️s/🔌️plugins/📐️cad` at session start showed the auto-committer several
  commits behind other plugins' activity (`16619a9699` etc., matching the repo-wide counter at the
  time) — no evidence of a peer session mid-edit inside `📐️cad` specifically; the plugin was
  genuinely idle before this session, consistent with SMO's "never claimed" finding.
- No other session's files were read for editing purposes beyond the ones listed above, all inside
  the `📐️cad` plugin boundary.

## apa-status: complete

Steps 1–6 all executed and structurally verified. Cargo-level (`cargo check -p
semio-s-plugin-cad`, `cargo test -p semio-s-plugin-cad --lib`, and the TS-side
`bun nx run @semio-tech/cad-js:test`) verification is the one remaining gap, deliberately deferred
per standing instruction (shared lock + red SDK) — flagging as the top risk for the consolidated
build to check first, specifically:
1. The 147 glue.rs `#[path]` mounts (Rust compile-time — resolved structurally, not compiled).
2. The 13-file TS engine bundle's internal imports (structurally verified via `os.path.relpath`,
   not run through `tsc`/`vitest`).
3. The `🎬️interaction-spec` test that asserts "≥40 interaction assets" under the new
   `🖼️assets` location — file count (211) reconciled, but the test itself was not executed.
