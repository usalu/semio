# F117: writers of `📋️build-owners.json` that bypass `updateBuildOwnerRegistryV1`

Read-only audit, 2026-11-26. No source edits, no git, no sub-agents. Scope: library `🏗️build` (all non-test `.ts`/`.mjs`), library `🧹️normalization` and `🏗️authoring`, `✏️s`, `🧰️framework` outside the library, root `📜️script.ts`, and the ticket `🧰️scripts` folders (5166 script files, grep by registry name and write primitive). The shared writer is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧾️registry-consistency/🔏️writer/🟦️.ts`. The registry root file is `📋️build-owners.json`.

## Summary

- The reconciler (`registry-consistency/🟦️.ts`) and the adoption module now delegate to the writer. No direct write to the registry remains in shipped library code outside the writer.
- One live path can rewrite the registry without the writer: the generic taxonomy reference-edit engine (`🧹️normalization/🛟️recovery/🟦️.ts:524`). It rewrites JSON string tokens in any `.json` file, and nothing in normalization excludes the registry. Whether the root file is inside its inventory was not verified against a live plan (see L1).
- One ticket script writes the root registry with a plain non-atomic `open(...,"w")` (T1). It is a one-off, but it is runnable today behind an env guard.

## Live hits (shipped library code)

| ID | File:line | Method | Atomic | Validates registry | Notes |
|---|---|---|---|---|---|
| L1 | `🧹️normalization/🛟️recovery/🟦️.ts:524` `applyReferenceEditAtomically` | staged write, `durableRename` into place, preimage compare | yes (rename) | no (only byte hash and mode) | Generic JSON token rewrite. `referenceAdapter` maps `.json` to `"json"` (`✂️reference-edits/🟦️.ts:170`), so a taxonomy module move rewrites path strings in `📋️build-owners.json` the same as any other JSON. No registry lock; the taxonomy transaction lease is the only guard. |
| L2 | `🧹️normalization/🚀️apply/🏁️transaction-run/🔧️transaction-steps/🟦️.ts:188-213` `applyReferenceEditGroups` | caller of L1 | via L1 | no | Drives every reference edit group of a taxonomy apply. |
| L3 | `🧹️normalization/🔍️inventory-planning/🟦️.ts:301` | `readFileSync` then `applyEditsToContent` | n/a (plans bytes) | no | Computes the rewritten bytes that L1 publishes. |
| L4 | `🧹️normalization/♻️projection-staleness/🟦️.ts:113,128`, `🧹️normalization/🗑️removal-references/🟦️.ts:38`, `🧹️normalization/🪞️artifact-reference-projection/🟦️.ts:627` | `applyEditsToContent` on file content | n/a | no | Same generic engine, other entry points. Included in the L1 fix. |
| L5 | `🧹️normalization/🛟️recovery/🟦️.ts:429` `copyFileSync` (rollback restore) and `:424` symlink, `:272` `linkSync` | restore of pre-images from backup | yes (link or copy then rename) | no | Rollback puts the pre-apply bytes back. Pre-image is trusted, so this only matters if the pre-image was already invalid. Low risk, but it bypasses the validator. |

Recommended fix for L1 to L5 (one change): exclude `📋️build-owners.json` from the generic reference-edit inventory and from `referenceAdapter`. Give it a dedicated registry edit kind. For that kind, the taxonomy apply computes the new rows and publishes through `updateBuildOwnerRegistryV1` with `compute` that re-derives rows from the post-move tree (the same path as `reconcileBuildOwnerRegistry`), so validation and the compare-and-swap apply. Add a registry guard in `applyReferenceEditAtomically` that refuses any edit to that path that is not of the registry kind. Rollback (L5) should run `readBuildOwnerRegistryV1` on the restored bytes and refuse an invalid one. Add a test that a taxonomy move of a module named in the registry leaves the registry validated and equal to a fresh reconcile.

## Live but not a direct write (checked and clean)

- `🕸️graph/🧾️registry-consistency/🟦️.ts`: no write primitives; delegates to the writer. Compliant.
- `🕸️graph/🧾️registry-consistency/🔏️writer/🟦️.ts:47,58,65`: the sanctioned writer. Compliant.
- `🧪️testing/🧫️private-repository/🧰️adoption/🟦️.ts:53`: now `createOwnerRegistryTextV1`. Compliant.
- Read-only sites (no write found): `⚡️caching/🕸️graph/✅️verification/🟦️.ts:21`, `🏗️build/👁️observation/🟨️.mjs:836` (statSync and readFileSync only), `🏗️build/🧭️invocation/🟦️.ts:122`, `🧭️discovery/🟦️.ts:64,110`, `🤝️prefetch/🟦️.ts:34`, `🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs:193,426`, `🧾️registry-reads/🟨️.mjs`, `🎛️commands/🛠️dev/🎮️playground/🟦️.ts:54`, `🎛️commands/✅️verify/🟦️.ts:52`, `⚖️laws/📋️build-owner-registry/🟦️.ts`.
- `🏗️authoring/📦️crate-tree/🟦️.ts:471-477`: calls the reconciler (compliant). It also writes backup files `…-owner-rows.json` (not the registry). The restore side was not traced, so treat its restore as unverified.
- `🏗️authoring/🧬️mutation-tree/🟦️.ts:396-415` and `🏗️authoring/📦️crate-tree/🧱️scaffold/🟦️.ts:67-88`: write workspace, package, router and Cargo.lock files, not the registry. Not hits.
- `✏️s/` and `🧰️framework` outside the library, and root `📜️script.ts`: no non-test source references the file name. Not hits.

## Generic writers (cannot be ruled out by name)

Many library `🏗️build` modules write to caller-supplied paths (`🧩️units/⌨️command`, `📈️counter-ledger`, `🗓️scheduler/💾️store`, `📥️inputs/🔭️read-oracle/🪞️mirror` copyFileSync, `🏗️authoring/🗂️collection-catalog:330`, `🏗️authoring/📦️crate-tree/🎮️command/🟦️.ts:98` writing `result.text`, `🧵️digests`, `🧹️normalization/🎮️command-contract`). None names the registry, but none is blocked from receiving its path. Only a source law (like the existing `registry-consistency` law test that no other library source publishes the registry) enforces the rule. Recommended: extend that law test to scan for `📋️build-owners.json` or `BUILD_OWNER_REGISTRY_FILE` in every write call site, not only publish calls.

## Ticket scripts

Scanned 5166 `🧰️scripts` files. 99 mention the registry or its readers. Real write sites:

| ID | File:line | Method | Atomic | Validates | Status |
|---|---|---|---|---|---|
| T1 | `🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/leaf-kinds/rename_groups_en1999.py:20` | `open(f,"w").write(u)`, `files=[OWNERS]` (`common.py:9` `OWNERS = SEMIO + "/📋️build-owners.json"`) | no | no | One-off, live path. Regex renames crate names across the root registry. Refuses unless env `TH_FORCE` is set. Whether it ran is not known from this audit. Fix: delete it, or if ever needed, re-run through `updateBuildOwnerRegistryV1`. Do not run it as is. |
| T2 | `🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R40/lab/scaf.sh:3` | `cp` of the root registry into an overlay dir `$ov` | no | no | Scratch copy only, not the real root. Low risk. Fix: create it with `createOwnerRegistryTextV1` if the overlay is kept. |
| T3 | `…FLEET…/🧰️scripts/F115/stage.py:33-49` | Python mirror of the writer's stage, rename and CAS publish | yes (rename or link) | yes (`readBuildOwnerRegistryV1` port) | Not called by library code. A second writer copy outside the shared module. Fix: delete it after landing, since the shared writer is now the only one. |
| T4 | `…FLEET…/🧰️scripts/F115/land-adoption.py` | edits the adoption module to call `createOwnerRegistryTextV1` | n/a | n/a | Landing script that already made the change. Not a runtime writer. Compliant. |
| T5 | `…FLEET…/🧰️scripts/F115/land-adoption-test.py:17-21` | `writeFileSync` into a checkout fixture | no | no | Test fixture writes. Location of `checkout` was not verified as temp. Confirm before any reuse. |
| T6 | `…WORKERS-READY…/🧰️scripts/r233-farm.py:115` | `writeFileSync(registry, …)` | no | no | Writes only into a `mkdtemp` root inside a test. Safe. |

Read-only or injected-reader ticket scripts (no write to the root file): `R156/new/*.ts`, `R209/discovery.base.ts`, `E62/land1.py`, `F31/scan-ownerless.py`, `R30/payload/*`, `R211/stage/*`, `R208/*`, `R167/patches.py`, `R151/*`, `R140/*`. Not hits.

## Recommended fixes, by priority

1. L1 to L5: exclude the registry from the generic reference-edit engine and route it through `updateBuildOwnerRegistryV1`. Add a guard in `applyReferenceEditAtomically`. Add the test above. This is the only live path that can change the registry without validation or the lock.
2. T1: delete `rename_groups_en1999.py` (or mark it do-not-run in the NORM ticket). It is the only ticket script that writes the root file non-atomically.
3. T3: delete the Python writer mirror `F115/stage.py` once landed, so the shared writer is the only implementation.
4. Extend the registry-consistency law test to fail on any write primitive that targets the registry path outside the writer module.
5. T2 and T5: low risk, tidy when the ticket closes.

## Limits

- L1 applies only if the root `📋️build-owners.json` is inside the taxonomy inventory. Normalization has no explicit exclusion for it, but I did not run a plan to confirm that. Verify with a dry-run plan before relying on it.
- Generic helpers were checked by name and call site, not traced to every caller.
- No file was modified. The only file written is this report.
