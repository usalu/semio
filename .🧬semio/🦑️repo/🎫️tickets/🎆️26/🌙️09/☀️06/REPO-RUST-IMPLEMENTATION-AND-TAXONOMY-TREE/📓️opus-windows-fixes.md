# 📓️ Opus `windows-fixes` — platform-correct file-mode authority and the frozen owner catalog

Scope: blockers 2 and 4 of `📓️opus-events.md` §5. Host: native Windows 11, Bun 1.4.2, `RUSTC_WRAPPER=""`.
Temporary output and probes: `🗑️generated/windows-fixes/`.

## 1. Blocker 2 — `Exact owner catalog mode drift` (fixed)

### Cause

`semanticOwnedInputFileSnapshot` reported `node.mode & 0o7777` straight from `fstat`. NTFS carries no POSIX
permission bits, so Node reports `0o666` for every regular file on Windows. `semanticExactOwnedFileCatalog`
requires `snapshot.mode === 0o644`, so the catalog load threw for every caller — including
`loadTaxonomy → getCargoWorkspaceIndex → resolveCargoPackageName → runCargoTestBudgeted`, i.e. every
`📦️packages/🦀️rust` nx `test` target.

Reproduction before the change (`🔨️modules/📡️events/📦️packages/🦀️rust`):

```
$ bun ./📜️script.ts test
error: Exact owner catalog mode drift: 🧰️framework/…/🧫️fixtures/⚖️readme-license-owner-authority/🔣️.json
      at semanticExactOwnedFileCatalog (…/📚️library/🔍️discovery/🟦️.ts:4867:42)
      at validateGeneratorContractsAgainstWorkspace (…:4947:19)
      at loadTaxonomy (…:1287:36)
      at getCargoWorkspaceIndex (…/📚️library/📦️packages/🟦️typescript/🟦️.ts:1378:20)
      at resolveCargoPackageName (…:1436:17)
      at runCargoTestBudgeted (…:1598:28)
```

### Change — `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`

One new mode authority plus its cache, and two call sites moved onto it. Nothing else was touched.

* `gitIndexFileModes(repoRoot)` — one `git ls-files --stage -z` per repo root, cached in an
  `ephemeralMap` exactly like the neighbouring `gitlinkBoundaryCache`; keeps stage-zero `100644`/`100755`
  blobs only, NFC-normalised. Outside a work tree it yields an empty map instead of throwing.
* `posixRegularFileMode(repoRoot, path, mode)` — on POSIX returns `mode & 0o7777` unchanged, so the strict
  check keeps its full force there. On `win32` it returns Git's index mode, falling back to `0o644` (what
  Git itself assumes for a new file on a `core.fileMode = false` filesystem) when the path is untracked.
* `semanticOwnedInputFileSnapshot` returns `posixRegularFileMode(repoRoot, path, node.mode)`. The raw
  `lstat`/`fstat` mode is still compared byte-for-byte across the open/read/ancestry checks, so the
  change-during-read detection is untouched — only the *reported* mode became platform-correct.
* `exactOwnerGeneratorPrestate` compares `posixRegularFileMode(...)` instead of `stat.mode & 0o7777`.

`snapshot.mode !== 0o644` in `semanticExactOwnedFileCatalog` was deliberately left strict: it is now a
meaningful assertion on both platforms rather than an unconditional Windows failure.

### Verification

`🗑️generated/windows-fixes/probe-mode.ts`, `probe-mode755.ts`:

```
[DEBUG] platform win32 snapshot.mode 644
[DEBUG] posixRegularFileMode(README.md) 644
[DEBUG] posixRegularFileMode(untracked-missing) 644
[DEBUG] catalog cases 40
[DEBUG] .devcontainer/compose-entrypoint.sh rawStat 666 authority 755
[DEBUG] README.md                          rawStat 666 authority 644
```

The `100755` case proves the authority is Git's index and not a hard-coded `0o644`.

The mode-drift error class is gone repo-wide: `📦️packages/🟦️typescript/🔬️index.test.ts` (582 tests,
239 s) and every case run below contain **zero** `mode drift` occurrences after the change.

Cargo itself is green for both subject crates:

```
$ RUSTC_WRAPPER="" cargo test -p semio-framework-repo-events -p semio-framework-repo-graphql
test result: ok. 5 passed; 0 failed …   (graphql)
test result: ok. … (events)
   Doc-tests semio_framework_repo_events   — ok. 0 passed
   Doc-tests semio_framework_repo_graphql  — ok. 0 passed
```

### Still red — a second, unrelated blocker unmasked behind the first

With the mode fix in place both `bun ./📜️script.ts test` runs get past the catalog and now die one step
later, identically, on a `💻️os` contract:

```
$ bun ./📜️script.ts test      # 📡️events/📦️packages/🦀️rust and 🔗️graphql/📦️packages/🦀️rust
error: Invalid taxonomy schema:
- generatorContracts["wgpu-frame-worker"] tracked output
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🤖️generated/🟨️.js" is missing.
      at loadTaxonomy (…/📚️library/🔍️discovery/🟦️.ts:1288:38)
```

This is **not** platform-specific and **not** caused by the mode change — it is a committed inconsistency
in the renderer tree (HEAD `5e03e56997`), diagnosed with `🗑️generated/windows-fixes/probe-wgpu.ts`:

* the wgpu package has already been projected — `…/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu` (the
  catalog's `sourceRoot`) no longer exists (`git ls-files` warns `could not open directory`), and all four
  projected destinations are occupied. `semanticPackageSourceOutputPhase` therefore correctly reports that
  the pre-state phase is over, so `nestedCargoGeneratedPrestate` cannot excuse the missing output.
* the tracked output itself was never committed at the new coordinate: `git ls-files` under
  `🎞️frame-worker/` lists only `🟦️.ts`.
* the registered generator cannot produce it either:

  ```
  $ bun x nx run @semio-tech/framework-os:generate-wgpu
  error: Current WGPU package catalog digest drift
        at parseCanonicalWgpuPackageCatalog (…/🔍️discovery/🟦️.ts:4198:78)
  ```

  `🧊️wgpu/🪪️package-catalog.json` hashes `fe9cc288e4f6411a41ec97e5d82d93866c35d74f266bb034aa57c13c8959bf27`
  while `generatorContracts["wgpu-frame-worker"].packageGeneration.catalogSha256` in `🔣️taxonomy.json` still
  declares `72823763f2bcf38a87af4902245813ff4b24d82f1fc164121e073fcea4794559`. Both were last written in
  commit `fe7c8a8f8b` (589).

Fixing it means editing `💻️os/📺️renderer` and the shared `🔣️taxonomy.json` digest — another fleet's module,
outside this agent's authorised surface, and being actively committed to right now. It belongs to the
renderer/`💻️os` owner or the wave-4 wiring agent. Until it lands, `runCargoTestBudgeted` cannot reach
cargo, even though cargo passes when invoked directly.

## 2. Blocker 4 — the "stale" `⚖️readme-license-owner-authority` fixture (no change; the premise does not hold)

`📓️opus-events.md` predicted that this fixture's rows for the deleted
`📚️library/📦️packages/🐹️go/{README.md,📋️project.json}` would fail "its statute case". That prediction was
never executed. It is wrong, and acting on it would be destructive.

### Evidence

`🗑️generated/windows-fixes/probe-cases.ts` walks all 40 catalog rows against the worktree:

```
[DEBUG] absent-both 18 🧰️framework/🔨️modules/🖱️ui/🖼️assets/README.md
[DEBUG] absent-both 24 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🔗️graphql/README.md
[DEBUG] absent-both 27 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🐹️go/README.md
[DEBUG] absent-both 33 🧰️framework/🔨️modules/🖱️ui/🖼️assets/LICENSE.md
[DEBUG] total 40 absentBoth 4
[DEBUG] contract expectedCounts {"fixed":4,"license":8,"projected":36,"readme":32,"referenceBindings":62,"total":40}
```

Rows 18 and 33 (`🖱️ui/🖼️assets`) have had neither source nor destination on disk since long before this
ticket, and nothing has ever failed because of them. The catalog is **frozen historical owner evidence**,
not a mirror of the current tree:

* `semanticExactOwnedFileCatalog` validates shape, classification, census and digest — it never stats a
  row's `sourcePath` or an `ownerEvidence.evidencePaths` entry.
* `semanticExactOwnedFileProjectionAuthority` is only consulted for paths that are already in the taxonomy
  inventory (`exactOwnedFileResolution` in `🧹️normalization/🟦️.ts:6401`), so a row whose source and
  destination are both absent is never reached.
* no test under `📚️library/🧪️tests` asserts that catalog rows exist (`grep` over `cases` +
  `exist|lstat|readFile|sourcePath` finds only shape assertions).

Empirically, after the mode fix, **no** run mentions `🐹️go`: `path-emoji-statutes` (38 tests),
`readme-move-source-authority` (17), `frozen-markdown-coordinates` (36), `testing-readme-coordinates`,
`readme-current-source-revision`, `readme-current-source-activation`, `readme-reviewed-fixture-inputs`,
and `🔬️index.test.ts` (582) — zero hits across all of them.

### Why it must not be rewritten

Any byte change to the file invalidates, all at once:

* `semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"].authorityCatalogSha256` in the
  shared `🔣️taxonomy.json`;
* `expectedCounts` (`total: 40`);
* every `currentSourceRevisions[*].catalogCaseIndex`, which addresses rows **by ordinal** — deleting rows 24
  and 27 renumbers everything after them;
* the ordinal references in `🧪️tests/🔖️readme-current-source-revision/🔣️.json` (`cases[31]`),
  `🗺️testing-readme-coordinates/🔣️.json` (`row: 31`), `👀️readme-reviewed-fixture-inputs` (`/cases/31/…`)
  and the `/cases/28/sourcePath` citation in the 26/08/17 ticket notes.

No generator exists for it (`grep` for the cohort id finds only readers). Rewriting a digest-pinned,
ordinal-addressed frozen artefact to remove two rows that provably nothing reads would be pure risk. **The
fixture was left byte-identical.** If the repo later decides frozen catalogs must track deletions, that is a
deliberate re-freeze of all 40 rows (including the two pre-existing `🖱️ui/🖼️assets` ones) plus the digest
and every ordinal reference — its own ticket, not a side effect of the Go move.

## 3. Separate native-Windows defects observed (not fixed, not in scope)

Recorded because the next agent on Windows will hit them.

1. `🧪️tests/🔏️path-emoji-statutes/🟦️.ts:175` — `renderCatalogReadme` in the root `📜️script.ts` builds its
   output path with `node:path.join`, so on Windows it yields
   `🧰️framework\🔨️modules\🖼️assets\README.md` where the contract declares
   `🧰️framework/🔨️modules/🖼️assets/README.md`. Needs `posix.join`.
2. `🧪️tests/🔏️path-emoji-statutes/🟦️.ts:65` — `mutationVectorRegistryBreaches` reports
   `mutation-vector-missing` for `🗿️sample/🏅️standards/🔖️1/🪆️subsets/{✳️camera,✳️any}/🔮️oracle/🔣️.json`.
3. Several `📚️library` test fixtures still point at pre-relocation coordinates and additionally look
   emoji/separator-mangled, so the cases abort with `ENOENT` before asserting anything:
   `🗺️testing-readme-coordinates/🔣️.json`, `🔖️readme-current-source-revision/🔣️.json`,
   `🟢️readme-current-source-activation/🔣️.json` and the three
   `👀️readme-reviewed-fixture-inputs/🧫️fixtures/**/🔣️.json` all name
   `…/🧫️fixtures/🔣️readme-license-owner-authority.json` (flat, pre-589) instead of
   `…/🧫️fixtures/⚖️readme-license-owner-authority/🔣️.json`, and
   `🗺️testing-readme-coordinates/🔣️.json` carries `…/🔨️modules/testREADME.md` and
   `…/🧪️tests/🧪️test/🟦️ing-readme-coordinates.ts` — both clearly corrupted rewrites of
   `🔨️modules/🧪️test/README.md` and `🧪️tests/🗺️testing-readme-coordinates/🟦️.ts`.
4. `test-exact-cargo-laws` fails 25/26 with `SEMIO_TEST_ARTIFACT_DIR is required` — the route does not set
   the variable its cases demand.
5. `🔬️index.test.ts`: 383 pass / 199 fail on this host, dominated by Windows long-path
   (`Filename too long`) and `\` vs `/` separator assumptions. Pre-existing; unchanged by this work.

## 4. Files changed

* `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts` — +30/−2: `gitIndexFileModeCache`,
  `gitIndexFileModes`, exported `posixRegularFileMode`; `semanticOwnedInputFileSnapshot` and
  `exactOwnerGeneratorPrestate` moved onto it.
* `.🧬semio/…/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE/📓️opus-windows-fixes.md` — this report.
* `.🧬semio/…/🗑️generated/windows-fixes/{probe-mode,probe-mode755,probe-cases,probe-wgpu}.ts` — the probes
  quoted above.

Nothing else was touched. No Git-modifying command was run.
