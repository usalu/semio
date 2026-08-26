# H-SCHEMA-POLICY Report

## Current State

`🔣️taxonomy.json` is schema version 6 and still declares semantic filenames throughout: ecosystem/target `leafFilename` and `entryFilenames`, `semanticManifestFilename`, package allowlists/suffixes, taxonomy/example/test leaves, story leaves, text/binary specification filenames, surface schema leaves, test contributions and root data/document filenames. Direct production references across the root script, discovery module and repo tests include 70 `leafFilename`, 29 `taxonomyLeafFilenames`, 26 `entryFilenames`, 11 `exampleLeafFilenames`, 8 `exampleTestLeafFilenames`, 9 `semanticManifestFilename`, and 6/5 packaging name/suffix references.

The current area registry marks `✏️s/🔌️plugins` and the repo product clean, `✏️s/🔨️modules` and `🧰️framework` mixed, `🌎️hub` and `♻️mit-bestand` legacy, and `compose` exempt. `DISCOVERY_QUIET_AREAS` suppresses legacy, mixed, exempt and undeclared areas. `semanticActiveRoots` omits legacy/exempt. The emoji policy walks only clean roots.

## Fixed-Name Defect

`policyEmojiFixedFilenames` builds a blanket set from package/root/leaf/entry/example/test fields, hardcoded lock/tool names, and then recursively visits the entire taxonomy object, treating every string that looks like a filename as fixed. `policyEmojiEntryIsRenamable` additionally exempts every configured suffix. This is the broad preservation mechanism the target contract forbids.

Generated/framework-owned logic also excludes every dot name and paths containing broad directory names such as `app`, while sibling emoji uniqueness is bypassed for several taxonomy contexts. Those rules need exact path contracts and registry semantics, not ambient name/suffix exemptions.

## Incompatible Version 7 Cut

Add and validate `fileKinds`, `semanticDirectoryKinds`, `fixedFilenameContracts`, `configurableEntryContracts`, `packageBoundaryRules`, `packageGlueGrammar`, `pathExclusions`, `unicodeNormalization`, `variationSelectorPolicy`, `collisionPolicy`, and `areaEnforcement`. File kinds own extension chains and kind-only basenames. Semantic directories own registered emoji/slug identity. Fixed contracts contain exact pattern, authority, reason, configurability, scope, verification and expiry.

After consumers migrate, remove semantic filename fields and all aliases/fallbacks. Replace package scanners with file-kind IDs and exact entry contracts. Replace area states with repository-wide clean enforcement plus the opaque `compose/` exclusion; no quiet legacy/mixed modes remain.

## CLI and Tests

The root `CleanScript` currently preserves deletion-only bare clean and delegates only `test`/`coverage`. This is the correct safety boundary. Add explicit `clean taxonomy inventory|plan|apply|verify` dispatch before workspace cleaning. Root `VerifyScript` currently exposes `verify taxonomy report|enforce`; converge it on the same canonical resolver rather than keeping two policy engines.

Root `📋️project.json` already routes every target through `📜️script.ts`. Launch seed contains clean and old taxonomy report/enforce entries and must receive ordered inventory/plan/apply/verify entries, then regenerate `.vscode/launch.json` through the existing catalog mechanism.

Permanent tests belong in the repo-library test family. They must cover Unicode/VS16, compound extensions, fixed/configurable contracts, semantic extraction, collision/platform hazards, opaque symlinks, deterministic plan bytes, stale apply, rollback/cancellation at every phase, empty second plan and package purity. Test-only `picomatch`/`fast-glob` can cross-check glob and inventory behavior behind test code; runtime remains standard-library only.

## Writer Boundaries

Schema/type validation, normalization engine/transaction regions, root CLI/launch integration, tests/golden fixtures and shared manifests each require a sole writer. Root script, taxonomy JSON, discovery component and the monolithic repo test file are the principal overlap hazards; production writes should be serialized across these files even when other modules proceed in parallel.

## Acceptance Checks

Schema validation rejects every removed field; broad suffix and recursive taxonomy-filename exemptions disappear; every non-opaque area is enforced; bare clean remains deletion-only; all four explicit commands are registered through Bun/Nx/launch; policy and CLI share one canonical resolver; and the version-7 golden fixtures agree with independent third-party glob/inventory results.
