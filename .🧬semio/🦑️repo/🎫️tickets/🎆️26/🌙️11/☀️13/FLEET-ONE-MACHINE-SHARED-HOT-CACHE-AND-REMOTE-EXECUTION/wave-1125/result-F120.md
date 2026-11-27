# F120: taxonomy reference edits publish the build owner registry through its writer

Lane F120, 2026-11-26. Foreground, no sub-agents, no graph/run-many/cargo. Lock `graph` taken only for the one writer swap (re-checked free, released at once). Library = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.

## 1. L1 confirmed on a private fixture
Dry-run plan (`inventoryTaxonomy` + `planTaxonomy`) on a git fixture in a temp dir (script kept: `T/🧰️scripts/F120/probe-dry-plan.ts.txt`; the live repo was never read for the plan). A registry named `📋️build-owners.json` with an owner rooted at `NoEmoji/`, and a file `NoEmoji/📋️project.json` the taxonomy moves to `🧪️NoEmoji/`: the plan carries two edits on the registry (`root` and `manifest` strings rewritten) next to the code edits. So the generic JSON engine did rewrite a registry without validation, without the registry lock and without the writer.

## 2. Fix
- Declared mapping (schema-first): `🧹️normalization/✍️own-file-writers/🔣️.json` lists `{id, fileName, authority}` rows (now: `build-owner-registry` for `📋️build-owners.json`), schema `🧬️schema/🔣️.json`; `🟦️.ts` reads and validates it (unknown writer id, a file name declared twice, wrong version are refused) and offers `ownFileWriterFor(path)` -> `{check, publish}`. No file name in the engine.
- `🧹️normalization/🚀️apply/🏁️transaction-run/🔧️transaction-steps/🟦️.ts` (`applyReferenceEditGroups`): for a path with a declared writer it first runs `check` on the computed new text (refuses before any backup is frozen, so the rollback state model stays valid), then passes `publish` to the engine.
- `🧹️normalization/🛟️recovery/🟦️.ts` `applyReferenceEditAtomically`: new optional last parameter `ownWriter`; after the engine's own preimage check it hands `(target, preimage, bytes)` to the writer and returns; the engine's staging/exchange is skipped for that path. The recovery module stays self-contained (the isolated source oracles concatenate it without imports).
- `registry-consistency/🔏️writer/🟦️.ts`: the options type gained `mode` (the published file keeps the preimage mode, so the engine's restore/ownership mode checks hold). `publish` goes through `updateBuildOwnerRegistryV1`: lock, preimage hash compared with the planned preimage (a changed registry refuses), validation with `readBuildOwnerRegistryV1`, compare-and-swap rename.

## 3. Tests (`repo test-file`)
`🧹️normalization/🧪️tests/✍️own-file-writers/🟦️.ts` (plain suite, like its siblings): 4 pass / 0 fail.
- declared mapping: registry has a writer, `📋️project.json` none; unknown id, duplicate file name, wrong version refused;
- a real `applyTaxonomyPlan` on a private fixture: the move renames an owner root, state `committed`, registry holds the new root and manifest, validates, keeps its mode 0640, no temp left;
- an edit that would produce a duplicate root (another owner already at the target root) is refused with state `rolled-back`; registry bytes and the whole workspace tree (modes and bytes) equal the state before, nothing moved;
- the writer refuses a registry that is no longer the preimage, invalid text, and publishes valid text with the requested mode.
Also green: registry-consistency 43/0, `🧪️tests/🛟️transaction-recovery-authority` 6/0. `🧪️tests/🔄️transaction-v2/🟦️.ts` cannot run through `repo test-file` (its fixture needs the suite runner env: "Invalid transaction fixture run identity", 62 fail before any code runs, not caused by this change). `repo typecheck` of all touched paths: passed (15 files). `dangling-imports.py` on all touched folders: exit 0.

## 4. Known limits
- A writer failure after the backup (a registry changed by a peer between `check` and `publish`) ends in the existing rollback drift class ("unowned restored edit"), the same class as any engine edit that fails after its backup. The common refusal (invalid or duplicate result) is caught by `check` before.
- L5 (rollback restore of the pre-image bytes) is unchanged: the pre-image is trusted and not re-validated.
- The registry lock is per machine (unchanged from F115).

## 5. T1 (`rename_groups_en1999.py`)
Ticket script `…/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/leaf-kinds/rename_groups_en1999.py`: every file write now goes through `write_atomic` (temp beside the file, mode kept, `os.replace`); for the root registry it first runs `validated_registry` (version 1, unique owner names, roots, native roots and package names, manifest exactly `<root>/📋️project.json` or `Cargo.toml`, mirroring `readBuildOwnerRegistryV1`), so a duplicate refuses before anything is replaced. The `TH_FORCE` guard is unchanged. The script was only compiled, never run.

## Kept scripts
`T/🧰️scripts/F120/`: `land.py`, `land2.py` (landing), `new-module.ts`, `new-policy.json`, `new-schema.json` (initial module text, superseded by the live file), `probe-dry-plan.ts.txt`.
