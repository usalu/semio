# W1-E Report — Surface Scaffolder + Four Viewer/Editor Policies

Lane 1-E. Scope: Deliverable A (`new surface` scaffolder) + Deliverable B (four policies), per
`📋️contract-freeze.md` §6.

## What landed

### Deliverable A — `new surface` scaffolder

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/📜️script.ts`,
new region `//#region 🔖️SurfaceScaffolder` (inserted after `//#endregion 🗿️TaxonomyValidator`,
before `validatePlaygroundSessions`).

- `resolveSubsetRel` — resolves bare CLI ids (`plugin kind standard subset`) to the real emoji
  directory names by stripping non-ASCII codepoints and matching (`surfaceStripEmoji`,
  `surfaceResolveChildDir`); throws a precise error naming the failing segment. `subsetArg === "*"`
  is accepted as an alias for `subsetAnyDirName` (`✳️any`).
- `discoverOwnedSubsetRels` — every subset dir across `taxonomy.pluginAreas` whose `🧬️schema` facet
  is present (the "owned" predicate contract §6 freezes — schema alone, independent of `🚪️io`; this
  is why 🌀️procedural's `🧩️assembly`, which has schema but no io, still counts and the `--all` total
  is exactly 143 subsets × 2 roles = 286).
- `scaffoldSurfaceTree` — creates the 19-file shape from the ticket brief, every path segment read
  from `🔣️taxonomy.json` (`surfaceDirNames`, `surfaceComponentLangs`, `surfaceRequiredChildDirs`,
  `modesDirName`, `modeRequiredChildDirs`, `windowsDirName`, `windowLeafLangs`,
  `windowRequiredChildDirs`, `taxonomyLeafFilenames`, `windowEmptyFacetFilename`) except the default
  mode/window ids (`SURFACE_DEFAULT_MODE_DIRNAME = {viewer: "👁️view", editor: "✏️edit"}`,
  `SURFACE_DEFAULT_WINDOW_DIRNAME = "🪟️main"`), which are not taxonomy vocabulary — documented inline
  as per-subset authoring content a W2 packet replaces. Never overwrites (`scaffoldWriteIfAbsent`
  skips and reports existing leaves).
- Every generated `🦀️component.rs`/`🟦️component.ts` carries a `SCAFFOLD` marker
  (`scaffoldRustLeaf`/`scaffoldTsLeaf`) that `policySubsetSurfaceCompletenessBreaches` (Deliverable B)
  flags as a distinct medium breach kind.
- `NewScript` (CLI class) + `runSurfaceScaffoldAll` (`--all`/`--dry-run`), registered in the
  `ScriptRouter` as `.register("new", NewScript)`.

Registered as a permanent nx target (not a raw CLI-only script):
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/📋️project.json`
gained a `"new"` target (`bun ./📜️script.ts new`, `forwardAllArgs: true`) and
`.../📇️registry/package.json` gained a `"new"` script (`bun nx run @semio-tech/plugin-registry:new`).

**Not done**: a `.vscode/🧩️launch.seed.jsonc` entry. My "exclusive lease" section names exactly two
paths (registry `📜️script.ts`, root `📜️script.ts` policy region); the seed file is not in it and is
plausibly under concurrent edit by other lanes, so I left it alone rather than risk a collision. The
nx target (`bun nx run @semio-tech/plugin-registry:new -- surface --all`) is the ready-made command a
launch entry would wrap — a follow-up lane/dev can add the seed row in one line.

### Deliverable B — four policies

Root `📜️script.ts`, new region `//#region 🔧️PolicyRuleArtifactViewersEditors`
(after `//#endregion 🔧️PolicyRuleSubsetConformance`, ~`:6929`, before
`//#region 🔧️PolicyRuleHandcraftedSpecP3`). All four registered in the top-level runner
(`export const policy = defineLint(...)`, ~`:13928`–`13932`).

- `policySubsetSurfaceCompletenessBreaches` (medium) — walks `policyListTopLevelSubsetDirs` (existing
  helper at `:6693`, already the exact `<plugin>/🗿️artifacts/<kind>/🏅️standards/<std>/🪆️subsets/<sub>`
  walker), skips non-owned subsets (no `🧬️schema`), and for each of `taxonomy.surfaceRoles` checks:
  surface dir present → ≥1 mode with ≥1 window carrying both `windowLeafLangs` leaves → no `SCAFFOLD`
  residue in the surface's `🦀️component.rs`/`🟦️component.ts` leaves. Missing-surface and
  incomplete-window findings are `kind: "taxonomy/surface-completeness"`; scaffold residue is a
  separate `kind: "taxonomy/surface-scaffold-residue"`, exactly as contract §6 requires ("not a
  pass", not folded into completeness).
- `policyViewerPurityBreaches` (high) — for every subset's `👁️viewer` dir, walks every file
  (`policyWalkRelFiles`) and flags `.mutation(`, `Emit::mutations`, `artifact_mutations`, `::editor::`.
- `policyContributedSurfaceTargetBreaches` (high, with a medium "unknown owner" edge case) — builds an
  owner map (subset-suffix → owning plugin root, keyed by `🧬️schema` presence), then for every surface
  dir under a NON-owning subset path checks the contributing plugin's `🦀️component.rs` tree for
  `.depends_on("<owner>")`, mirroring `policyContributionTargetBreaches` (`:7628`) exactly. Dormant on
  the current tree (no contributed/mirrored surface exists yet — confirmed 0 breaches, see Verification
  below) but structurally exercised: it correctly classified all 143 subsets as self-owned in the run.
- `policyOsConfigShapeBreaches` (high) — locks the already-shipped C4 facet
  (`🧰️framework/🛍️products/💻️os/🎚️config`): all 5 `schemaFormats` leaves, the frozen schema id
  `os.config.opening` (content-matched in `🧬️schema/🦀️component.rs`, not folder-name-matched), and
  both `set-default-app`/`clear-default-app` mutation triads (content-matched via each mutation's
  `kind: "..."` `SemanticDescriptor` field, so a folder-slug rename can't silently break the lock).

## Verification (exact commands + output)

**1. Dry-run batch scaffold, real tree, before any scaffold existed:**

```
$ cd 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry
$ bun ./📜️script.ts new surface --all --dry-run
new surface --all: would scaffold 286/286 surface(s) across 143 owned subset(s) (5434 file(s) would be created, 0 already present).
```

286/286 = 143 owned subsets × 2 roles, exactly the contract §0 target. 5434 files = 19 files/surface ×
286, matching contract §7.8's "≈5400 files" estimate almost exactly.

**2. Smoke test — scaffolded `✏️s/🔌️plugins/🗒️note` end to end** (kind `🗒️note`, standard `1`, subset
`✳️any`, both roles):

```
$ bun ./📜️script.ts new surface note note 1 any viewer
✏️s/…/🪆️subsets/✳️any#viewer: created 19 file(s), 0 already present
$ bun ./📜️script.ts new surface note note 1 any editor
✏️s/…/🪆️subsets/✳️any#editor: created 19 file(s), 0 already present
```

Idempotency check — re-running `--all --dry-run` and the same single-surface command:

```
$ bun ./📜️script.ts new surface --all --dry-run
new surface --all: would scaffold 284/286 surface(s) … (5396 file(s) would be created, 38 already present).
$ bun ./📜️script.ts new surface note note 1 any viewer
✏️s/…/🪆️subsets/✳️any#viewer: created 0 file(s), 19 already present
```

286 → 284 (38 = 19×2 files now present), reruns skip every existing leaf. Confirms "never overwrite".

Root policy after the smoke test:

```
$ bun ./📜️script.ts policy   # (repo root)
```

Full breach cache (`.🦑️repo/⚡️cache/breaches/compose.json`) counted for the new kinds:

| kind | count |
|---|---:|
| `taxonomy/surface-completeness` | 284 (the 284 surfaces not yet scaffolded) |
| `taxonomy/surface-scaffold-residue` | 2 (exactly the 🗒️note/✳️any viewer + editor I scaffolded) |
| `taxonomy/viewer-purity` | 0 |
| `plugin-dependency/contributed-surface-target` | 0 |
| `taxonomy/os-config-shape` | 0 |

Scoped to `✏️s/🔌️plugins/🗒️note`: **zero** `taxonomy/surface-completeness` breaches (both surfaces are
structurally complete) and **exactly two** `taxonomy/surface-scaffold-residue` breaches — one per role,
each "still carries 5 SCAFFOLD-marker leaf(ves)" (2 surface leaves + 1 mode leaf + 2 window leaves = 5)
— i.e. exactly the "reports the scaffold-residue breach and no completeness breach" the ticket asks to
demonstrate.

The overall `bun ./📜️script.ts policy` run exits 1 (24 727 high-priority breaches, 34 rule kinds) —
this is **pre-existing repo state**, not caused by this lane: the dominant kind is
`handcrafted-grammar/spec-distinctness` (22 079 of them, unrelated to this ticket), and none of the
four new policies' kind strings appear in the high-priority tally (`policyViewerPurityBreaches`,
`policyContributedSurfaceTargetBreaches`, `policyOsConfigShapeBreaches` are all `"high"` priority and
all three returned 0 findings, confirmed above).

**3. `bun ./📜️script.ts check` (registry package):**

```
$ cd 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry
$ bun ./📜️script.ts check
… plugin registry catalog is fresh (59 plugin crates, 58 playgrounds, 23 framework packages); .vscode/launch.json is fresh.
$ echo $?
0
```

Exit 0. The run also emits ~208 warn-only `discoverPackageProblems`/taxonomy-tree diagnostics (all
pre-existing `manifest-without-marker`/`unknown-lang` findings from unrelated plugins, plus 6 new
`"🗒️note: …/🦀️component.rs is not declared by any #[path] in 📦️glue.rs"` warnings for the smoke-test
scaffold leaves — expected and harmless: `pluginTaxonomyStates["✏️s/🔌️plugins"] = "mixed"`, so these
stay warn-only exactly like every other not-yet-wired taxonomy leaf in the tree, per the taxonomy's own
`_surfaceComment` sequencing note).

**4. `bun nx run @semio-tech/repo-lib:test` (repo root):**

```
168 pass
18 fail
970 expect() calls
Ran 186 tests across 1 file. [9.38s]
```

The 18 failing test names match `📓️w0-i-report.md`'s enumerated baseline exactly (`loadTaxonomy > …`
×3, `discoverPackages > …` ×4, `computeWorkspaces > …`, `dependency-boundary > …`,
`micro-commit > …`, `package boundary guards > …`, `playground static sites > …` ×2,
`command budgets > …` ×2, `resolveCargoPackageName > …` ×2, `ui scrollbar styling > …`) — same set,
same count. **0 new failures.**

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/📜️script.ts` —
  `//#region 🔖️SurfaceScaffolder` + router registration.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/📋️project.json` —
  `"new"` nx target.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/package.json` —
  `"new"` npm script.
- `📜️script.ts` (repo root) — `//#region 🔧️PolicyRuleArtifactViewersEditors` (four policies) + four
  `breaches.push(...)` registrations in the top-level `policy` runner.
- Smoke-test output, left in place as real forward progress (2 of the 286 target surfaces are now
  genuinely scaffolded, flagged correctly as scaffold residue so W2 cannot mistake them for done):
  `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/**` and
  `.../✳️any/✏️editor/**` (19 files each, 38 total).

## Not done, and why

- `.vscode/🧩️launch.seed.jsonc` registration — out of my exclusive lease (see above); the nx target
  covers the same capability (`bun nx run @semio-tech/plugin-registry:new -- surface --all`).
- Did not run `new surface --all` for real (non-dry-run) across the whole repo — that is W2's job
  (286 surfaces × 19 files, one per plugin packet, per contract §7.8: "the scaffolder emits shape
  only… handcrafted per subset by the W2 packet that owns it"). This lane's job was the tool and its
  policies, verified end-to-end on one real subset.
