# S-Generator-Unknown-Closure

## Outcome

Closed the three residual generator classifications without adding a broad exclusion or output root. The strict version-7 registry now contains 14 `owned`, 4 `external`, 0 `unknown`, and 0 `unsafe` contracts; the ownership type and validator admit only `owned` or `external`.

## Changes

- Removed `ownerless-ui-icons`; the stale ignored tree has no generator ownership contract.
- Removed false `root-layering-declarations` ownership.
- Set `layeringGeneratedContractIds` to `[]`.
- Kept `package.json`, `Cargo.toml`, and `go.work` as authored fixed contracts (`root-package`, `root-cargo`, `root-go-work`) and reject any generator output claiming those root manifests.
- Reclassified `setup-wizard-config` as `external` with seven exact tracked file outputs. It does not own `.ralph-tui`, `.ralph-tui/**`, or another subtree.
- Added exact Ralph filename contracts for configuration, lock, progress, sessions, and dynamic PRD `prd.json`/`prd.md` leaves.
- Added exact Ralph directory contracts for `.ralph-tui`, `.ralph-tui/prd`, and `.ralph-tui/prd/*`; no Ralph contract uses `**`.
- Extended strict validation for zero unresolved ownership, absent false contracts, exact Ralph outputs/contracts, no recursive Ralph wildcard, Ralph single ownership, fixed-only root manifests, empty layering-generated IDs, and existence of every tracked generator output.
- Added language-agnostic tests with `fast-glob` filesystem parity plus focused negative cases for unknown ownership, recursive Ralph scope, incomplete Ralph output coverage, and false root generation.

## Evidence

Commands ran from `/Users/ueli/Documents/semio` on 2026-08-26.

- Strict load: `loadTaxonomy()` and `validateTaxonomy()` returned `[]`.
- Focused Bun tests:
  `bun test …/🧪️index.test.ts --test-name-pattern 'generator ownership|unsettled, broad Ralph'`
  Result: 2 passed, 0 failed, 20 assertions.
- Git/schema parity: `git ls-files -- .ralph-tui` was compared byte-for-byte with `setup-wizard-config.outputRoots`.
  Result: `{"trackedRalph":7,"owned":14,"external":4,"layeringGenerated":0}`.
- Parent combined selector independently reported 40 passed, 0 failed.

## Canonical hashes

- `🔣️taxonomy.json`: `0ae9ba190562362a6b3abb107cd03dddb6cadf724b549f93f80006180e4c5d18`
- `🔍️discovery/🟦️component.ts`: `15c88ec0d4fa6a75af1ca5f4dea13f9946026aa02cab4985967f993e1deb647a`
- `🧪️index.test.ts`: `b2fd9ac7ee839ca4df3127ec817115a0d9407c34e49b120c03c6f44b53e71921`

## Boundaries

No normalization engine, catalog, Compose/temp-Compose, AGENTS, unrelated generator owner, or Git state was modified. The already-trashed stale UI icon tree was not touched.
