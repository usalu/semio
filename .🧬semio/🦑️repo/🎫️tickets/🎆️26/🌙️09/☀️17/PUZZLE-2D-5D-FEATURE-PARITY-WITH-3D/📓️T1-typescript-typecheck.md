# T1 — TypeScript type-check + law-test audit of today's fleet edits

Read-only audit. Raw tool output saved under `🗑️generated/T1/`. All paths repo-relative.

## 1. Changed-file list (today, this effort)

Built from `git status --porcelain` (uncommitted) plus `git log --since="2026-09-17 00:00"`
cross-referenced against `📓️wave-*-report.md` (some files — e.g. 2A's law file — were already
committed today by an earlier auto-commit and no longer show in `git status`).

**Renderer / OS host (uncommitted):**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`
- `…/🧱️elements/🌐️World3dHost/🟦️.tsx`, `…/🏛️ShellHost/🟦️.tsx`, `…/🖥️Board2dHost/🟦️.tsx`,
  `…/🪪️WasmSessionLoader/🟦️.tsx`, `…/🛠️ShellHelpers/🟦️.tsx` (+ its two `🧪️tests`),
  `…/🗣️Interpreter/🟦️.tsx` (+ its `🧪️tests/🪟️tree-windows`), `…/🖋️InkCanvasHost/🟦️.tsx`
  (+ its `📖️stories`), `…/🖥️Board2dHost/🧪️tests/🧩️component/🟦️.ts` (new, untracked)

**`🖱️ui` framework module (mixed staged/committed):**
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` (the Action-line target — normalizer,
  Actions-panel gate wiring, `Search`/`applySearchSpaceAction`)
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts` (`Board2dScene.domainId`/`transformFlags` twins)
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx` + `🧪️tests/🧩️component/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` — **already
  committed** (0b460ed19f, 2026-09-17 13:49:55) — 2A's React law file for the Action line

**Puzzle plugin (mixed, mostly already committed as part of `0b460ed19f`):**
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts`
- `◻️2d` and `🖐️5d` schema twins: `🧬️schema/🟦️.ts`, `🧬️schema/🔺️diff/🟦️.ts`,
  `🧬️schema/📸️snapshot/🟦️.ts`, `🧬️schema/🧬️mutations/🟦️.ts`,
  `✏️editor/🎚️config/🧬️schema/🟦️.ts`, `✏️editor/🪟️window/🧬️schema/🟦️.ts` (12 files)
- `◻️2d/…/🧪️tests/🌐️third-party-puzzle-2d-1/🟦️.ts`, `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️tool-job-puzzle-reserved-routes/🟦️.ts`

**Root:** `📜️script.ts` (repo-root task router — `toolJobPuzzleReservedRoutesExact` etc.)

Ticket-local probes (`🔍️browser-probe.ts`, `🔍️browser-probe-5d.ts`, `🟦️2F-publication-diff.ts`,
`🌐️I2-third-party-oracle-probe.ts`) exist but are test-harness scripts, not covered by any
project tsconfig; excluded from the type-check below (not app/library source).

## 2. How this repo type-checks TS, and what I ran

No root `nx typecheck`. Two real per-package `tsconfig.json` + `TypecheckScript` (`runBunx(["tsc",
"--noEmit","-p","tsconfig.json"])`) exist and both transitively pull in most of the changed files
via imports even though their own `include` globs don't cover every directory:

1. **`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json`**
   (`include: ../../**/*.ts(x)` = the `🧑‍🎨engine` root, its `🧱️elements`, `🧪️tests`). Covers
   Board2dHost, World3dHost, ShellHost, ShellHelpers, WasmSessionLoader, InkCanvasHost,
   Interpreter, engine-contract directly, and (via imports) the root `📜️script.ts` and the `🖱️ui`
   Action-line target/law file transitively.
   `cd` into it, `bunx tsc --noEmit -p tsconfig.json` → **50 s, exit 2, 858 `error TS`**
   → `🗑️generated/T1/tsc-renderer-react.txt`.
2. **`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json`**
   (`include` explicitly lists `🧱️elements/**`, `🔨️modules/**`, `🧪️tests/**` under `🖱️ui`).
   Covers the `🖱️ui` Action-line target, `🌳️Tree` + its test, and (via imports) the same shared
   framework files as #1.
   `bunx tsc --noEmit -p tsconfig.json` → **26 s, exit 2, 664 `error TS`**
   → `🗑️generated/T1/tsc-ui-react.txt`.

Both runs are fast (not the >15 min case), so no need for the ad-hoc fallback there. **They
overlap substantially** (both transitively include the shared framework/`📜️script.ts` files), so
their totals are not additive — don't sum 858+664.

3. **Puzzle plugin TS** (schema twins + `📦️packages/🟦️typescript/📜️script.ts`) has **no
   tsconfig at all** and no `typecheck` command in its own `📜️script.ts` (confirmed by reading
   it — only a `test` and `publication-authority-audit` command exist). Per the task's fallback
   rule, ran an ad-hoc invocation reusing the two tsconfigs' compiler options
   (`--noEmit --skipLibCheck --strict --target ESNext --module ESNext --moduleResolution bundler
   --esModuleInterop --resolveJsonModule --isolatedModules --allowImportingTsExtensions --lib
   DOM,ESNext`) directly over the 15 changed puzzle files:
   **12 s, exit 2, 485 `error TS`** → `🗑️generated/T1/tsc-puzzle-adhoc.txt`. Validated the ad-hoc
   setup against the real runs: the exact same "Cannot find name 'Bun'" / `ImportMeta.dir` noise
   for the root `📜️script.ts` appears in run #1 too, confirming this is repo-wide missing
   `bun-types` coverage, not an artifact of my invocation.

## 3. Diagnostics — grouped by file

### 3a. Diagnostics genuinely caused by today's edits (4)

**`🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx`** — 2A slice
(Search/engagement-line law rewrite, committed 0b460ed19f):

| line | message | fix |
|---|---|---|
| 6473 | `TS7006: Parameter 'draft' implicitly has an 'any' type.` | annotate the inline `Search` `onSubmit` callback: `onSubmit: (draft: string) => submitted.push(draft)` |
| 6860 | `TS7006: Parameter 'next' implicitly has an 'any' type.` | `onChange: (next: string) => changed.push(next)` in the new "Search onChange carries the typed line verbatim…" law |
| 6900 | `TS7006: Parameter 'next' implicitly has an 'any' type.` | same, `onSubmit: (next: string) => submitted.push(next)` |

Root cause: `Search`'s `input.onChange`/`onSubmit` prop type isn't inferred at these call sites
(likely a loosely-typed prop signature), so the rewritten inline arrows trip `noImplicitAny`.
Confirmed by diffing `git diff a4cda597ea 0b460ed19f -- <file>` — all three lines fall inside 2A's
own edited hunks (6460-6498, 6834-6910); every other diagnostic in this file (143 of 146 distinct
error lines) sits outside those hunks → pre-existing.

**`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts`** — 2F
slice (`target_regions` field, same change wave-2B's hand-off already flagged as breaking the
Rust DSL examples):

| line | message | fix |
|---|---|---|
| 244 | `TS2741: Property 'targetRegions' is missing in type '{ schema; camera; nodes; edges; meta }' but required in type 'Puzzle2dArtifact'.` | `parsePuzzle2dArtifact` was never updated after `targetRegions: Puzzle2dTargetRegion[]` was added to `Puzzle2dArtifact` (`git show 0b460ed19f` confirms the field was added but no matching parser wiring). **No `parsePuzzle2dTargetRegion` function exists anywhere in the file either** — `Puzzle2dTargetRegion` (the interface) has zero parser. Needs: a `parsePuzzle2dTargetRegion(value, at)` parser mirroring `parsePuzzle2dNode`'s shape, and a `targetRegions: …GuardArray(row["targetRegions"], …).map(…)` line in `parsePuzzle2dArtifact`, matching what 5d's `parsePuzzle5dArtifact` already does correctly for its analogous `targetVolumes` (`git diff` shows 5d wired `parsePuzzle5dTargetVolume` into `parsePuzzle5dArtifact` on the same day — 2d's slice missed the parser-wiring half). |

This is a real client/schema-twin gap left by 2F's `target_regions` addition, on the TS side of
the same defect wave-2B already reported on the Rust side (DSL table breaking `concrete-forest`).

### 3b. Diagnostics in changed files, confirmed pre-existing (counted only)

Checked by diffing each flagged file's touched line ranges (`git diff -U0` / `git diff --staged
-U0`, and `git show <last-touching-commit>` for already-committed files) against every error
line; none of the below fall inside an edited range:

| file | error count | representative | verdict |
|---|---|---|---|
| `…/🌐️World3dHost/🟦️.tsx` | 5 | TS2345/TS2322/TS2769 at 1605, 1691, 5326, 7223, 7228 | pre-existing, outside all of today's 13 hunks |
| `…/🏛️ShellHost/🟦️.tsx` | 15 | TS2353/TS2339/TS2304/TS2367 at 1163…7140 | pre-existing, outside today's ~10 hunks |
| `…/🛠️ShellHelpers/🟦️.tsx` | 1 | TS2322 `Uint8Array` → `BlobPart` at 656 | pre-existing |
| `…/🪪️WasmSessionLoader/🟦️.tsx` | 1 | TS2353 `bindings` on `FlowBrowserOptions` at 122 | pre-existing (2E/2B's additions are at 298-333, untouched by this) |
| `…/🖋️InkCanvasHost/🟦️.tsx` | 3 | TS2339 `documentJson` on `InkCanvasScene` at 971×2, 983 | pre-existing |
| `…/🗣️Interpreter/🟦️.tsx` | 2 | TS2345 at 308, TS2339 `ImportMeta.dir` at 2228 | pre-existing (today's hunks are 51-1898, both errors outside) |
| `…/🌳️Tree/🧪️tests/🧩️component/🟦️.tsx` | 4 | TS2322 `string`→`UiLabel` at 111,153,154,169 | pre-existing |
| `…/🌳️Tree/📖️stories/🧪️.story.tsx` | 2 | TS2769/TS2741 | not in today's changed list at all (story file untouched) |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` (root) | 4 | TS2339/TS2345 at 6208, 6240, 11450, 11465 | pre-existing — today's edit here is only 4 import/export-list additions (9566-9688) |
| `📜️script.ts` (repo root) | 98 | Bun/ImportMeta noise + assorted | pre-existing — today's edits are 5 small hunks (3668-3712, 6585, 6704, 6736, 25560), none overlap |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts` | 5 | `TS2867/TS2868: Cannot find name 'Bun'` ×4, `TS2339 ImportMeta.dir` ×1 | pre-existing repo-wide bun-types gap (same class appears for the root `📜️script.ts` in run #1) |
| `◻️2d/…/🧬️schema/🔺️diff/🟦️.ts` | 2 | `TS2552/TS2304: Cannot find name 'parsePuzzle2dNodePatch'/'parsePuzzle2dEdgePatch'` | pre-existing — `Puzzle2dNodePatch`/`EdgePatch` *types* exist but their parser functions never did; today's diff only added the unrelated `targetRegions` block (lines 12-14, 24-32, 187-199) |
| `🖐️5d/…/🧬️schema/🔺️diff/🟦️.ts` | 2 | same pattern for `parsePuzzle5dPartPatch`/`FastenerPatch` | pre-existing, same systemic gap, mirrored in 5d |
| `🖐️5d/…/🧬️schema/📸️snapshot/🟦️.ts` | 2 | TS2322 `ArtifactRef` at 319 (inside `parseArtifactChildHandle`, untouched today), TS2552 `parsePuzzle5dCatalogPartKindExtra` at 326 (sibling of Grip/Fastener/Rope parsers that also predate today) | pre-existing |
| `🖐️5d/…/🧬️schema/🟦️.ts` | 2 | TS2322 tuple mismatch in `parsePuzzle5dGripTemplate` at 373-374 | pre-existing — today's edit here (`targetVolumes`, correctly wired) sits at lines 19-267, well before this untouched function |
| all remaining files pulled in transitively (`🛢️db` test, `📇️inventory`, `backbone-envelope-io`, `space-artifact-creation-owner`, `normalization`, `chunkkey`, mailbox/turnscheduler tests, `document-contract` tests for 6 unrelated plugins, etc.) | ~700 combined across the 3 runs | — | **not in the changed-file list at all** — pulled in only because tsc typechecks the whole reachable import graph; pure pre-existing repo noise |

**Total pre-existing/unrelated diagnostics across the three runs: ~2003** (858 + 664 + 485 raw,
minus the 4 real ones above, minus double-counting of the shared transitively-included files
between run #1 and run #2 — the two runs are not disjoint programs). No attempt was made to
de-duplicate the exact overlap count between run #1 and run #2 line-by-line; both are reported
raw in `🗑️generated/T1/`.

## 4. `bun test` results for the named law files

Per the read-only/no-nx constraint, only `bun test <file>` was used (no vitest CLI, no nx). All
five target files import `describe`/`it` from `"vitest"`, not `bun:test`.

| slice | file | result |
|---|---|---|
| 2A React laws | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` | **0 pass / 0 fail** — not runnable this way. The file only *defines* `export async function registerTests1(vitest, dependencies, testSource)`; nothing calls it at module scope, so bun's runner finds no test to collect. Confirms wave-2A's own note: "the `🖱️ui` module has no standalone TS package, so its vitest suite only runs through the forbidden workspace `nx test` target." → `bun-test-2A.txt` |
| 2B's five host laws + 2E's host vitals law | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` | **FAIL — the file cannot even load.** `SyntaxError: Export named 'worldSceneContentBoundsKey' not found in module '…/⚛️react/📦️packages/🟦️typescript/🟦️.tsx'` aborts the whole run before any of today's tests execute (0 pass, 1 fail, 1 error). Traced: the renderer react target file imports `worldSceneContentBounds`/`worldSceneContentBoundsKey` from a world/r3f module at line 546-547 — an import block **not touched by any commit today** (today's only edits to that file are two lines at 1208/1240) — and no function of that name exists anywhere in the repo (`grep -rn "export function worldSceneContentBounds"` = 0 hits). This is a **pre-existing broken import**, unrelated to the puzzle-2d/5d ticket, but it is the reason 2B's and 2E's new engine-contract laws cannot be executed at all outside the full nx/vitest harness (matches both 2B's and 2E's own "written, not executed" admission — this is *why*). → `bun-test-2B2E-engine-contract.txt` |
| 2I's three new TS laws | `🛠️ShellHelpers/🧪️tests/🧩️component/🟦️.ts` (`🧰️ActionPaneGate`) | **PASS — 11 pass, 0 fail, 32 expect() calls.** → `bun-test-2I-shellhelpers.txt` |
| 2I's three new TS laws | `🖥️Board2dHost/🧪️tests/🧩️component/🟦️.ts` (handle-vitals + catalogue kind-hover) | **PASS — 2 pass, 0 fail, 9 expect() calls.** → `bun-test-2I-board2dhost.txt` |
| board2d scene lanes round-trip | `🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🚚️surface-scene-lanes/🟦️.tsx` | **0 pass / 0 fail** — not runnable this way, same factory-function shape as the 2A file (this test file itself is untouched today — last commit 2026-09-14 — only the `🎬️scene/🧫️fixtures/🚚️board2d-scene-lanes/🔣️.json` fixture it reads changed today, for `domain_id`/`transformFlags`). Cannot confirm the round-trip against today's fixture changes via `bun test` alone. → `bun-test-board2d-scene-lanes.txt` |

**Summary: 2 of 5 named law groups actually ran (both pass, 13/13); the other 3 are structurally
unrunnable outside the nx/vitest harness this audit is forbidden to invoke** — one of them (the
engine-contract file, covering 2B+2E) additionally has a pre-existing broken import blocking it
even under vitest until `worldSceneContentBounds`/`worldSceneContentBoundsKey` are re-exported or
the dead import is removed.

## 5. Bottom line

- Of ~2000 total TypeScript diagnostics surfaced across the three narrowest real type-checks, only
  **4 are attributable to today's fleet edits**: three `TS7006` implicit-any params in 2A's new
  `Search` laws (trivial, add parameter types), and one real product gap — 2F's `Puzzle2dArtifact.targetRegions`
  field has no parser, unlike 5d's correctly-wired `targetVolumes` sibling added the same day.
- Everything else found in a changed file is pre-existing and unrelated (confirmed line-by-line
  against `git diff`/`git show` hunks for each file).
- 2 of the 5 requested law-test runs pass cleanly (2I's ShellHelpers + Board2dHost laws, 13/13).
  2A's and the scene-lanes round-trip test cannot run via `bun test` at all (factory-function
  shape, need the nx-orchestrated harness). The shared engine-contract file (2B+2E) cannot run
  because of an unrelated pre-existing broken import.
