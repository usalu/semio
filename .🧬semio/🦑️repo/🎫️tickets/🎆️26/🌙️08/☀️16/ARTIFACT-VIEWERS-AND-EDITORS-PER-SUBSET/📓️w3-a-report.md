# W3-A (dissolution) — lane report

Ticket: `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET`. Contract: `📋️contract-freeze.md` §6.
Precondition verified before starting: 0 `🎛️apps` dirs, 143 `👁️viewer` + 143 `✏️editor` surfaces.

Full command output: `🧪️w3-a-gates.txt` (same folder).

## What landed

### 1. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`

- Deleted `appsDirName`, `appChildDirs`, `appComponentDirs`, `appSchemaSpecFilenames`.
- `pluginChildDirs`: `["🎛️apps","🎮️commands"]` → `["🎮️commands"]`.
- `semanticCollections` drops the `"🎛️apps": { kind: "app" }` entry.
- Every `surface*` key W0 added left untouched. `schemaVersion` stays `5` (already bumped in W0).

### 2. `📚️library/🔍️discovery/🟦️component.ts`

Removed the four deleted fields from the `Taxonomy` interface and from `validateTaxonomy` (including
the now-dead `appFacetPathIsDeclared`/`appFacetChildDirs` helper pair — their sole caller was the
`appSchemaSpecFilenames` validation block being deleted). Every check that used to assert against
`appChildDirs`/`appComponentDirs` was repointed at the surface twin already added in W0
(`surfaceChildDirs`/`surfaceRequiredChildDirs`), not dropped — the "📚️examples must be present",
"commands facet at every command-owning scope", and "banned bare 🧮️config/🕸️wasm" checks all still
run, now against surfaces instead of apps.

### 3. `📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`

Baseline before this ticket: **18 pre-existing failures** (`📓️w0-i-report.md`). After this lane:
**170 pass, 18 fail — the identical 18, zero new.**

Pins updated (none of these were among the 18 baseline failures — all were previously-passing
assertions I had to keep passing against the new shape):
- `appComponentDirs`/`appChildDirs`/`appSchemaSpecFilenames` literal assertions → `surfaceRequiredChildDirs`/`surfaceChildDirs`/`surfaceSchemaSpecFilenames`.
- `pluginChildDirs` literal: `["🎛️apps","🎮️commands"]` → `["🎮️commands"]`.
- The "commands facet at every command-owning scope" test's key list: `"appChildDirs"` → `"surfaceChildDirs"`.
- The two `policyWindowCompletenessBreaches`/`policyModeCompletenessBreaches` fixture tests: their
  on-disk fixture shape moved from `<owner>/🎛️apps/<app>/🎭️modes/…` to
  `<owner>/🗿️artifacts/<artifact>/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/…`, matching the
  real surface tree every other fixture in this file already uses for `🏅️standards`/`🪆️subsets`.

One transient false alarm, self-caused and resolved: I started the `nx repo-lib:test` verification run
in the background *before* finishing the fixture rewrites above, so that first run raced my own save
and showed 20 fail (2 spurious). Re-ran clean after the edit landed: 18/18, confirmed twice.

### 4. Root `📜️script.ts` — every walker that read `taxonomy.appsDirName`/`appChildDirs`/`appComponentDirs`/`appSchemaSpecFilenames`

New shared helper `policySurfaceRoots(repoRoot, ownerRoot, taxonomy)` — every viewer/editor surface
dir that actually exists under an owner's `🗿️artifacts/*/🏅️standards/🔖️*/🪆️subsets/*/` tree. Used by:

- **`policyWindowCompletenessBreaches`** and **`policyModeCompletenessBreaches`** — `walk(...)` now
  starts from each real surface root instead of `<owner>/🎛️apps`.
- **`policyTaxonomyDirsBreaches`** — two fixes, per contract §6:
  1. Its `walkForWindows` sub-walk (recognized-window-child-dir check) retargeted the same way.
  2. The `NestedFacetWalk` region's top-level "is this a recognized artifact child dir" check now also
     accepts `newArtifactChildDirs` (`"🏅️standards"`, previously never recognized — every new-shape
     artifact's `🏅️standards` dir fell into the "not recognized" branch, and there its contents were
     never inspected because nothing matched). Added a new `//#region SubsetFacetWalk` that descends
     `🏅️standards/<s>/🪆️subsets/<sub>/`, validates each subset's children against `subsetChildDirs`,
     reuses the existing schema/io representation-dir validators (factored into two closures,
     `validateSchemaFacet`/`validateIoFacet`, shared between the artifact-level and subset-level
     call), and validates each `👁️viewer`/`✏️editor` surface's own children against `surfaceChildDirs`.
     **Measured effect: 22 genuinely new breaches surfaced** (20 `taxonomy-dirs-surface-*`, 1
     `taxonomy-dirs-standard-*`, 1 `taxonomy-dirs-subset-*`) — real W2 packets that added a disallowed
     extra top-level dir directly under a surface (stray `⚙️engine/`, custom `renderer/`, `session/`,
     `chrome/`, `catalog/`, `maphost/` dirs outside `surfaceChildDirs`). **Not mass-fixed here**, per
     the ticket's own instruction — full list in `🧪️w3-a-gates.txt` §7.
- **`ExamplesScript.collectExampleRoots`** — the `[artifactsDirName, appsDirName]` two-kind loop split:
  artifact-level unchanged, app-level replaced with a `policySurfaceRoots` walk over each surface's
  `📚️examples/`. Verified live via `bun ./📜️script.ts examples verify` — surface example roots
  (`…/✏️editor/📚️examples/…`) are now correctly discovered.
- **`policySemioArtifactExamplesBreaches`** — the app-examples block replaced with a surface-examples
  block. Two shape changes, not just a path swap: (a) the `⚙️engine`-under-app check was dropped
  entirely — surfaces have no `⚙️engine` facet (`surfaceChildDirs` never listed one; the ENGINELESS
  ticket moved engine ownership to the module level); (b) `📚️examples` is no longer *required* — per
  contract §7.5 it's in `surfaceChildDirs` but not `surfaceRequiredChildDirs`, so a surface with no
  examples is legal and produces no breach; only a surface that *does* carry `📚️examples/` gets its
  contents validated.
- **`policyComponentFileBreaches`** — the separate `walk(appsDirName)` call was simply deleted: its
  `walk` is already unconditionally recursive from `walk(artifactsDirName)`, which now reaches every
  nested surface facet on its own (no separate root needed).
- **`policyDiscoverAppSchemaOwners`/`policyAppSchemaFacetCompletenessBreaches`/`policyAppSchemaFacetRole`**
  (the "APP-SCHEMA-FACETS" wave A2 scanners) — retargeted from walking `<plugin>/🎛️apps/<app>/` to
  `policySurfaceRoots`, and `taxonomy.appSchemaSpecFilenames` → `taxonomy.surfaceSchemaSpecFilenames`.
  Live run: **498 real breaches** (surfaces whose `type Config = …` binding exists but the
  `🎚️config/🧬️schema`/`👥️presence/🧬️schema` facet dir itself is missing) — confirms the retarget is
  live, not silently vacuous.
- **`policyPluginClosedShapeBreaches`** — the fallback "relocate under X or Y" message text updated
  (no `🎛️apps` alternative left to suggest); the "a plugin is EXACTLY 🎛️apps + 🗿️artifacts + …" reason
  strings corrected to drop `🎛️apps`.
- **`policyEmojiSiblingIdentityIsStructural`** — dropped the now-dead `parent === "🎛️apps"` structural
  exemption (0 real dirs can ever match it again).
- Stale/inaccurate doc comments fixed in passing where directly adjacent to logic I was touching
  (`policyPluginRootShapeBreaches`'s "manifest, capabilities, setup, and apps" reason string,
  `POLICY_PLUGIN_CLOSED_SHAPE_LEGACY_FACETS`'s stale `pluginChildDirs` value claim).
- **Deliberately left untouched**: `POLICY_SEMANTIC_VOCABULARY_ALLOWLIST`'s ~150 seeded literal
  `🎛️apps/…` paths (a different, unrelated ticket's — SEMANTIC-MUTATIONS-OVERHAUL — historical
  allowlist; stale entries there are inert (an allowlist entry that never matches a real file is a
  no-op, not a breach), and two `policyAppIdFromCrateDir`/`policyNormalizeRelPath` legacy-path helpers
  that hardcode `"🎛️apps"` for an even older, already-dissolved `⚡️implementations` crate-layout —
  confirmed dead by the same logic (0 real paths can ever match), out of this lane's `appsDirName`/
  `appChildDirs` scope (they don't read the taxonomy field at all), not touched.

### 5. `🔌️plugin/📦️packages/🟦️typescript/📇️registry/📜️script.ts`

- Removed `APPS_DIRNAME`/`TAXONOMY.appsDirName`.
- New `surfaceDirsForPlugin(pluginRoot)` helper (mirrors root script's `policySurfaceRoots`, scoped to
  one plugin, paired with a `"<subset>/<roleDirName>"` label for findings).
- `validateTaxonomyTree`'s `//#region AppFacetWalk` → `//#region SurfaceFacetWalk`: the
  engine-requirement + examples block, the modes/windows block, and the legacy-config/wasm +
  config/presence-schema-owner block all retargeted the same way as the root script (engine
  requirement dropped, examples optional, same reasoning as §4).
- `discoverExamplesForPlayground` — the flat `<techRoot>/🎛️apps/<app>/📚️examples` scan replaced with a
  `surfaceDirsForPlugin` walk; the variant-suffix single-app fallback now matches against a surface's
  subset name (ASCII-stripped) instead of a literal app dir.
- Verified via `bun …/📇️registry/📜️script.ts check`: exit 0, "plugin registry catalog is fresh (59
  plugin crates, 58 playgrounds, 22 framework packages); .vscode/launch.json is fresh." — plus real new
  `surface "<subset>/<role>"` findings in the non-blocking taxonomy-tree output, confirming the retarget
  runs against real disk, not silently vacuous.

### 6. Rust taxonomy gate — `🔌️plugin/🦀️component.rs`

**Deleted, not retargeted** — `testkit::assert_constitutional_crates` and its private helper
`assert_taxonomy_components` (plus the four small JSON-parsing helpers exclusively used by it:
`load_taxonomy_json`, `string_array`, `object_string_values`, `schema_format_leaf_filenames`, and two
local closures). Reasoning, verified before deleting:

1. Repo-wide grep confirms **zero callers** of `assert_constitutional_crates` anywhere (only its own
   definition). `Plugin::builder` does not call it either, despite the function's own docstring
   claiming "Invoked automatically by `Plugin::builder`" — that claim is stale from an architecture two
   generations back (ticket `26/07/29/MOVE-APPS-INTO-S-PRODUCT-TREE-WITH-CONSTITUTIONAL-CRATES`,
   confirmed via old `.cursor/plans/*.md` scratch docs referencing it).
2. Its one reachable code path (for any manifest dir under `✏️s/🔌️plugins/`) tries four candidate
   `🎛️apps` locations and **unconditionally `panic!`s** if none is a directory. Since 0 `🎛️apps` dirs
   can ever exist again, this branch is now a permanent landmine, not a real gate — exactly the "hard
   runtime `assert!` that panics the build" class this ticket's ordering-lesson warns about, except
   confirmed dead rather than live.
3. The information it enforced (artifact-facet completeness, `pluginChildDirs` presence, app/surface
   `⚙️engine`+examples+config/presence shape) is fully covered live by the two TS-side gates already
   retargeted in items 4–5 above — `policyTaxonomyDirsBreaches` + `validateTaxonomyTree`, both now
   surface-aware.
4. CLAUDE.md: greenfield repo, no legacy support, refactor inconsistencies rather than patch around
   them.

The `plugin_child_dirs` assert itself (the one the brief named directly) needed **no code change** —
it already read `pluginChildDirs` dynamically from the live `🔣️taxonomy.json` (`string_array(&taxonomy,
"pluginChildDirs")`), so the taxonomy.json edit alone retargets it; it's simply gone now along with the
rest of the dead function.

Verified: `RUSTC_WRAPPER="" cargo check -p semio-framework-plugin --all-targets --keep-going` —
**clean, exit 0**, zero errors. No Rust-side `pluginChildDirs` reader remains anywhere in the repo
(confirmed via grep) — enforcement is now TS-only (root + registry `📜️script.ts`), matching my comment
left in place of the deleted region.

### 7. W1 soft gate → W3 hard gate — `owned_surface_gaps`

- `🔌️plugin/🖥️host/🦀️component.rs`: `AppRouter::owned_surface_gaps`'s own docstring updated (it was
  already pure/total/never-panics by design — "the caller decides whether to log or hard-fail" — no
  code change needed there, only the doc's W1→W3 framing).
- `🏃️run/🦀️component.rs`, `load_runtime_recursive`: the `eprintln!`-and-continue diagnostic replaced
  with `return Err(RunError::Host(...))` carrying every gap's fault message, joined. A missing owner
  surface for either role now fails the plugin load with a real `surface.missing-owner-surface`-derived
  error instead of merely being logged, per contract §3.

**Not independently verified running** — `semio-framework-plugin-host` and (transitively)
`semio-framework-os-run` currently fail to compile for reasons **confirmed unrelated to this change**:
an `AppFrame::Error` pattern-match at `🖥️host/🦀️component.rs:2709` missing a `report` field added by a
peer's channel-protocol commit (`5a1367d`, 2026-08-16 14:18:35, well before and unrelated to my edit to
a different region of the same file), plus 164 pre-existing `semio-s-plugin-stdio` errors from the live
FULL-STDIO peer ticket (confirmed via `git status --porcelain` showing dozens of in-flight `M`/`D`
entries under `✏️s/🔌️plugins/🗄️stdio/` right now). Full evidence in `🧪️w3-a-gates.txt` §8. My own change
compiles logically (mirrors the existing `.map_err(...).?"` pattern every other router registration in
the same function already uses) but the crate itself cannot currently reach `cargo check` for reasons
that predate and are outside this lane.

## Verification — see `🧪️w3-a-gates.txt` for full output

| check | result |
|---|---|
| `find … -name '🎛️apps'` | 0 (precondition held throughout) |
| `tsc --noEmit` | 19 pre-existing errors, unchanged, none in this lane's files |
| `cargo check -p semio-framework-plugin` | clean, exit 0 |
| registry `📇️registry/📜️script.ts check` | exit 0, catalog + launch.json fresh |
| `bun ./📜️script.ts verify gate` | dies at step 1 (dependency-cruiser, 827 pre-existing violations, then OOM-class exit 138) — unrelated to this lane, none of the violations touch files this lane changed |
| `bun nx run @semio-tech/repo-lib:test` | 170 pass, 18 fail — identical to the W0-I baseline, zero new |
| live `policy` lint run | `taxonomy/dirs` 217 (22 newly surfaced by the standards/subsets descent); `taxonomy/window-completeness` 35; `taxonomy/mode-completeness` 0 (verified correct, not vacuous); `taxonomy/semio-examples` 93; `app-schema/facet-completeness` 498 |

## What is NOT done, and why

1. **The 22 newly-surfaced `taxonomy/dirs` breaches (surface/standard/subset scoped) are not
   mass-fixed** — explicit ticket instruction ("report the count and do NOT mass-fix them here").
2. **`semio-framework-plugin-host`/`semio-framework-os-run` never reached a green `cargo check`** in
   this session — pre-existing, unrelated peer breakage (§8 of the gates file), not this ticket's
   required gate (only `semio-framework-plugin` was required, and is clean).
3. **`bun ./📜️script.ts verify gate` never completed** — dies at its first step on a massive
   pre-existing dependency-cruiser violation set (827) plus an apparent OOM on this ~10k-module
   workspace scan, before reaching any policy this lane touches. Compensated with a direct live run of
   every policy this lane changed (table above), which completed cleanly and produced sensible,
   surface-aware findings.
4. **`POLICY_SEMANTIC_VOCABULARY_ALLOWLIST`'s ~150 stale `🎛️apps/…` seeded entries** (a different,
   unrelated SEMANTIC-MUTATIONS-OVERHAUL ticket's allowlist) and two hardcoded-`"🎛️apps"` legacy-path
   helpers (`policyAppIdFromCrateDir`, inside `policyNormalizeRelPath`) for an even older, already-fully-
   dissolved `⚡️implementations` crate layout were left alone — confirmed dead/inert by the same
   "0 real paths can ever match" logic as the deleted Rust gate, but outside this lane's literal
   `appsDirName`/`appChildDirs` scope and not blocking anything.

## Files touched

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`
- `📜️script.ts` (root)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs`

Scratch/logs in this ticket folder: `🧪️w3-a-gates.txt`.
