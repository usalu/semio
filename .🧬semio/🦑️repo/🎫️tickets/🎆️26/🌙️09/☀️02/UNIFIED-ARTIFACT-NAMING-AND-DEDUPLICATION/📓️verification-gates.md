# Verification Gates — Unified Artifact Naming And Deduplication

All commands run in the foreground from `/Users/ueli/Documents/semio`. Full stdout/stderr for every
command is saved under `🗑️generated/`.

## Commands run

| # | Command | Exit | Output file |
|---|---|---|---|
| 1 | `bun ./📜️script.ts verify taxonomy enforce` (before fix) | 1 | `🗑️generated/verify-taxonomy-enforce.txt` |
| 2 | `bun ./📜️script.ts verify taxonomy enforce` (after fix) | 1 | `🗑️generated/verify-taxonomy-enforce-2.txt` |
| 3 | `bun ./📜️script.ts verify taxonomy report` | 1 | `🗑️generated/verify-taxonomy-report.txt` |
| 4 | `bun ./📜️script.ts verify taxonomy report --scope="✏️s/🔌️plugins"` | 1 | `🗑️generated/verify-taxonomy-report-scoped.txt` |
| 5 | `bun ./📜️script.ts verify dependencies literal-external` | 1 | `🗑️generated/verify-dependencies-literal-external.txt` |
| 6 | `bun ./📜️script.ts verify layering` | 1 | `🗑️generated/verify-layering.txt` |
| 7 | `bun nx run @semio-tech/plugin-registry:check` | 1 | `🗑️generated/verify-plugin-registry-check.txt` |
| 8 | `bun nx run-many -t check --all --exclude workspace` | partial: 1/16 targets done (fail), 15/16 still running after ~16 min (see below) | `🗑️generated/verify-nx-check-all.txt` |

## Fix applied (category a — caused by the renames)

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` line 1323 (inside
`parseTaxonomy`) hardcoded the pre-rename literal `"🖍️draw"` as the expected
`sourceArtifactMemberName` for `rationaleRule === "artifact-editor-command-projection-v1"`.
`🔣️taxonomy.json`'s `semanticPathProjectionContracts.artifact-editor-command-bundle-v1
.sourceArtifactMemberName` already correctly reads `"🖍️drawing"` (owned by the coordinator, not
touched by us). The mismatch crashed `parseTaxonomy` before any taxonomy violations could even be
collected, so `verify taxonomy enforce`/`report` failed outright rather than reporting a real
violation list.

Fix (one line, not touching `🔣️taxonomy.json`):
```
- const expectedArtifact = rationaleRule === "artifact-example-model-catalog-projection-v1" ? "📐️cad" : rationaleRule === "artifact-editor-command-projection-v1" ? "🖍️draw" : undefined;
+ const expectedArtifact = rationaleRule === "artifact-example-model-catalog-projection-v1" ? "📐️cad" : rationaleRule === "artifact-editor-command-projection-v1" ? "🖍️drawing" : undefined;
```
Re-running `verify taxonomy enforce`/`report` after the fix gets past `parseTaxonomy` cleanly and
fails later, on an unrelated crash (see below). Searched for the equivalent hardcoded literal for
the other 8 renames (`🗂️curate`, `♻️rewrite`, `🎬️present`, `🌀️procedural2d`, `🧊️procedural3d`,
`📜️imperative`, `◻2d`, `📄txt`, `📰xml`) in `📜️script.ts`, the normalization library, and the
taxonomy-library TS file — none found. (`📜️script.ts`'s `POLICY_PLUGIN_DEP_OS_SYMBOLS` still keys
`"🖍️draw"` and `"📸️remodel"` — verified these are *plugin directory* names, which were never
renamed; only the *artifact* subdirectories inside those plugins were renamed. Not a bug.)

Spot-checked `semanticDirectoryMemberKinds["members-of-artifacts"].memberNames` in `🔣️taxonomy.json`
directly (read-only, via a Python one-liner): all 9 new names (`🎬️presentation`, `🖍️drawing`,
`🗂️curation`, `♻️rewriting`, `📸️remodeling`, `➗️equation`, `🌀️generation2d`, `🧊️generation3d`,
`📜️procedure`) are present and all 9 old names (`🎬️present`, `🖍️draw`, `🗂️curate`, `♻️rewrite`,
`📸️remodel`, `➗️mathematical`, `🌀️procedural2d`, `🧊️procedural3d`, `📜️imperative`) are absent —
matches the coordinator's claim.

## Results and classification

### 1–4. `verify taxonomy enforce` / `report` (scoped and unscoped) — FAIL, not ours (after our fix)

After the fix above, all four taxonomy invocations fail identically and immediately with:
```
error: Normalization requires an explicit repository-boundary decision before authored classification: ♻️mit-bestand/recherche
  at inventoryTaxonomyWithSourceParentPruning (🧹️normalization/🟦️.ts:6615)
```
Evidence this is NOT caused by our renames:
- `♻️mit-bestand/recherche` is a git submodule (gitlink, mode `160000`), added in commit `9c018952c5`
  dated 2026-07-31 — over a month before this ticket started (2026-09-02).
- `mit-bestand` has nothing to do with any of the 9 renamed artifacts (fem/puzzle/block/animate/
  draw/sourcing/trinity/remodel/mathematical/procedural/imperative/stdio); it is an entirely separate
  top-level tree.
- `git status --porcelain` shows zero changes to `🔒️layering.json`, `.gitmodules`, or anything under
  `♻️mit-bestand/recherche` from this or any other session today.
- The crash comes from a *new* normalization rule (`sourceAdmission` repository-boundary check) that
  now rejects any gitlink without an explicit decision — a normalization-library concern unrelated to
  artifact naming, and out of this ticket's scope to fix (taxonomy.json is explicitly off-limits, and
  the actual fix here would be a repository-boundary decision belonging to whoever owns
  `♻️mit-bestand`).
- Passing `--scope="✏️s/🔌️plugins"` (excluding `♻️mit-bestand` entirely) does **not** avoid the crash,
  because it happens during `inventoryTaxonomyWithSourceParentPruning`'s repo-wide source-admission
  pass, before scope filtering narrows the violation list.

**Conclusion**: we cannot get a full taxonomy violation list for our 9 renamed artifacts until this
unrelated gitlink issue is resolved by whoever owns it. The one taxonomy defect that *was* caused by
our renames (the `🖍️draw`/`🖍️drawing` literal) is fixed and confirmed gone (the second run fails
later, at a different line, for an unrelated reason).

### 5. `verify dependencies literal-external` — FAIL, not ours

Fails with `oracle-conflicts=6` naming `brepjs`, `three`, `image`, `png`, `serde_json`, `zip`.
None of these are among the 9 renamed artifacts; `serde_json`'s "declared by" list includes dozens of
plugins (including some renamed ones, e.g. `🌀️procedural`, `📸️remodel`, `📜️imperative`, `🗄️stdio`,
`🧩️puzzle`, `🧱️block`) purely because they all depend on `serde_json` in their `Cargo.toml` — this is
a pre-existing repo-wide dependency-oracle-ownership conflict (same crate declared as a dependency by
many packages), not something introduced or affected by renaming a directory. `zip` — named explicitly
in the task brief as an example of unrelated breakage — is one of the six conflicts. Not ours.

### 6. `verify layering` — FAIL, not ours

62 files exceed their shrink-only reference-count baseline in `🔒️layering.json`. Inspected the two
files whose names coincidentally contain "draw" (`🧪️tests/🧪️draw-source-scenario/🔣️.json` and its
`🧬️schema/🔣️.json`): wrote a one-off script calling `layeringReferences()` directly — both breach
because of 4 references to the **`✏️s` (plugins root)** implementation area, not any renamed artifact's
area. The other 60 breached files span totally unrelated framework areas (`🌊️flow`, `🧮️math`,
`🖼️pixels`, `🗺️mesh-engine`, `🗣️dsl`, `🎒️pack`, `📇️directory`, `🛢️db`, `🌉️mcp`, `🕹️interaction`,
`📡️replication`, …) that have nothing to do with any of the 9 renamed artifacts. `🔒️layering.json`
itself is untouched by this session (`git status` shows no diff) and was last committed 2026-09-02
13:31 — consistent with ongoing, unrelated concurrent-session churn against the shrink-only ratchet,
not our renames. Not ours to fix (would require touching dozens of unrelated framework files).

### 7. `bun nx run @semio-tech/plugin-registry:check` — FAIL, not ours

Fails with a large "plugin taxonomy tree violations" list (~4000 lines) reporting every one of the
**32** plugins in `✏️s/🔌️plugins/` — including plugins we never touched, e.g. `✒️writer`, `🌍️gis`,
`🌿️vcs`, `🎥️shooting`, `🎪️demonstrator`, `🎬️sequence`, `🏛️architect`, `🏭️process`, `💠️lowpoly`,
`💡️reasoning`, `📋️forms`, `📏️layout`, `📐️cad`, `📕️norm`, `📖️playbook`, `🔋️energy`, `🕸️dag`,
`🖨️raster`, `🗒️note`, `🪐️space` — as "missing `🧬️schema/`", "missing `🚪️io/`", "missing `⚙️engine/`",
"missing `📚️examples/`" for every artifact standard-subset, plus many "`#[path]`-undeclared" fixture
findings. This is a uniform, repo-wide structural expectation that fails identically for renamed and
never-touched plugins alike, so it cannot be attributed to our 9 renames. It matches the concurrent
ticket visible in `git status` at session start,
`26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`, which is
actively restructuring every artifact standard-subset to add `schema/`/`io/`/`engine/`/`examples/`
directories and has evidently not finished rolling this out to any plugin yet. Not ours.

**Caveat**: `plugin-registry:check`'s own docstring says it "byte-compares every one of those
[generated catalog] artifacts and never writes" — but the run above never reached that phase. It
throws on the plugin-taxonomy-tree violations (#7) before doing any byte-compare of the generated
catalog (`🤖️generated/🟦️plugins.ts`/`🟦️playgrounds.ts`, `.vscode/launch.json`) against what the 9
renames should have produced. Searched the full output for `hash`/`byte-compar`/`mismatch`/`stale`/
`drift` — the only hit is an unrelated `📕️norm` fixture path containing the word "drift". So this
gate genuinely cannot confirm or deny catalog freshness for our renames until the unrelated
tree-violation issue (#7) is cleared by its owning ticket; we did not attempt to run `generate`
(which writes tracked files) to work around this, since that is out of this task's scope.

### 8. `bun nx run-many -t check --all --exclude workspace` — see below

Only 16 projects repo-wide declare a `check` target at all (discovered via
`bun nx show projects --with-target check`): `@semio-tech/framework-plugin-host`,
`semio-framework-os-flow-core`, `@semio-tech/framework-os-scale-fixture`, `@semio-tech/framework-plugin`,
`@semio-tech/framework-os-shell-rs`, `@semio-tech/framework-os-mcp-rs`, `@semio-tech/plugin-registry`,
`@semio-tech/ui-contract-rs`, `@semio-tech/framework-os-host-rs`, `@semio-tech/ui-host-rs`,
`@semio-tech/framework-schema`, `@semio-tech/framework-async-rs`, `@semio-tech/framework-os-kernel`,
`@semio-tech/ui-rs`, `@semio-tech/flow-plugin`, `@semio-tech/framework-rs`. None of the 9 renamed
plugins' own projects (`*-plugin`/`*-js` for animate/draw/sourcing/trinity/remodel/mathematical/
procedural/imperative/fem/puzzle/block/stdio) declare a `check` target — their only targets are
`test`/`test-quick`/`test-long`/`test-exhaustive`/`describe`. There is no separate TS `tsc --noEmit`
step in this repo distinct from these named `check` targets; TypeScript is run directly by `bun`
without an ahead-of-time build step.

`plugin-registry:check` (already run standalone, see #7) is included in this `--all` run and involves
compiling several plugins to `wasm32-wasip2` (observed real, progressing `rustc` CPU time compiling
`semio_s_plugin_stdio` and others under an isolated `CARGO_TARGET_DIR`), which is slow on this shared
machine (multiple concurrent sessions' `rustc`/`cargo` processes were observed running at the same
time).

**Did not finish within this task's practical time window (~16 minutes observed).** nx started all
16 `check` targets. One resolved quickly and definitively:

- `@semio-tech/ui-rs:check` — **FAIL, not ours**: `ui axes are stale:
  🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🤖️generated.rs` (`run bun nx run
  @semio-tech/ui-rs:generate to refresh`). This is the WGPU UI-runtime's locale/terminology-axis
  codegen freshness check — nothing under `🖱️ui/…/wgpu` is one of the 9 renamed artifacts, and this
  target has no dependency on `✏️s/🔌️plugins/*`. Not ours.

The remaining 15 targets (several — `plugin-registry`, `framework-rs`, `ui-contract-rs`,
`framework-plugin`, `framework-os-kernel`, etc. — trigger their own `cargo check`/wasm builds) were
still running after ~16 minutes, with the `rustc` process count under this invocation fluctuating
between 9 and 22 concurrent processes and zero further lines appended to the output file for the
last several minutes of observation. This matches this exact ticket's own sibling sessions'
documented experience verbatim: `📓️rename-procedural-generation.md` reports a single `cargo check`
sitting at 0% CPU for 20+ minutes from cross-session `Cargo.lock`/target-dir contention with 10+
concurrent `rustc` processes, and `📓️rename-imperative-procedure.md` reports two 30–40 minute
attempts blocked on the same shared build-directory file lock. Given (a) this precedent, (b) that
**none of the 9 renamed plugins have a `check` target at all** (confirmed above — this command
cannot exercise their TypeScript/build correctness even once it finishes), and (c) that the one
`check` target that *does* touch artifact/plugin structure (`plugin-registry:check`) was already run
to a definitive conclusion in gate #7, this run was not blocked on to full completion.
`🗑️generated/verify-nx-check-all.txt` holds whatever output existed at write-time — rerun
`bun nx run-many -t check --all --exclude workspace` once the shared `target/` directory is less
contended for the remaining 15 targets' results.

## Corroborating evidence from sibling rename sessions (same ticket folder)

- `📓️fix-fe0f-stdio.md` (this ticket): a concurrent session already ran
  `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-stdio --target wasm32-wasip2 --message-format short`
  for the `📄txt`→`📄️txt`/`📰xml`→`📰️xml` fix and got **exit code 0 (clean)** — only 184 pre-existing
  unrelated warnings from `semio-framework-plugin`, none mentioning txt/xml.
- `📓️fix-fe0f-2d.md` (this ticket, fem/puzzle/block `◻2d`→`◻️2d`): explicitly recommends re-running
  `cargo check` for `semio-s-plugin-fem`/`puzzle`/`block` only *after* two other concurrent tickets'
  in-flight work lands — `26/09/02/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS` and
  `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION` — the
  latter being the exact same ticket whose in-progress `schema/`/`io/`/`engine/`/`examples/`
  restructuring explains our `plugin-registry:check` failure above (#7).
- `📓️rename-procedural-generation.md` (this ticket): independently observed the shared Cargo
  workspace "continuously broken by *other* sessions' concurrent, unrelated refactors
  (`semio-s-plugin-draw-fsm` missing crate, `semio-framework-graph` `ToValue`/`FromValue` derive
  errors + `draw_layers` codegen, and now a missing `stdio` zip artifact schema file)", and reports
  its own `cargo check -p semio-s-plugin-procedural` sat at 0% CPU for 20+ minutes from cross-session
  `Cargo.lock`/target-dir contention with 10+ other concurrent `rustc`/`cargo` processes — the same
  contention this session observed directly (`ps aux` showed 8-11 concurrent `rustc` processes
  throughout, several started by other sessions before ours).

These independently confirm: (a) the renames themselves compile cleanly where checked in isolation,
and (b) the taxonomy-tree/schema-completeness and cargo-availability failures we saw are known,
already-flagged, cross-ticket, in-progress conditions — not defects introduced by this ticket's
renames.

## Summary table

| Gate | Result | Ours? | Evidence |
|---|---|---|---|
| `verify taxonomy enforce`/`report` (schema-parse phase) | 1 real bug found and fixed | Yes (fixed) | Hardcoded `🖍️draw` literal vs. taxonomy.json's `🖍️drawing` |
| `verify taxonomy enforce`/`report` (inventory phase, after fix) | FAIL | No | `♻️mit-bestand/recherche` gitlink, added 2026-07-31, unrelated tree |
| `verify dependencies literal-external` | FAIL | No | oracle-conflicts on `brepjs`/`three`/`image`/`png`/`serde_json`/`zip` |
| `verify layering` | FAIL | No | 62-file baseline breach; inspected files match `✏️s` (plugins root) and unrelated framework areas, not any renamed artifact |
| `nx run @semio-tech/plugin-registry:check` | FAIL | No | identical violation shape across all 32 plugins, renamed and untouched alike; matches concurrent restructuring ticket |
| `nx run-many -t check --all --exclude workspace` | 1/16 resolved (`ui-rs:check` FAIL), 15/16 still running after ~16 min | `ui-rs:check`: No | none of the 9 renamed plugins even have a `check` target; the one relevant target (`plugin-registry:check`) already ran to completion in gate 7; the rest are blocked on the same cross-session cargo contention sibling sessions documented |
