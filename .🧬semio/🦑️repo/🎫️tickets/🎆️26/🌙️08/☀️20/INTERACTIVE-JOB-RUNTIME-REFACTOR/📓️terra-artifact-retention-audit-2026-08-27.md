# Artifact Retention Audit

## Scope

Read-only investigation of disappearing verification logs and the CAD ticket-local Cargo target. No builds, clean commands, process termination, file moves, or deletions were performed.

## Current Evidence

- All five named evidence files were absent at audit time, including `🧪️flow-core-detailed-check-recovered-r1-2026-08-27.txt` and `🧪️member-latestwins-registered-r10-native-2026-08-27.txt`.
- `🧱️cargo-target-cad` still exists, but is only 44 KiB and contains `CACHEDIR.TAG`, `.rustc_info.json`, and `.future-incompat-report.json`; `debug/` and `.fingerprint/` are absent. The sibling `cargo-target-cad` is 4 KiB with only `CACHEDIR.TAG`.
- No relevant open descriptor or deleted-but-open descriptor was found under the master ticket. The current process census showed no process using either CAD target. Active Cargo compilation observed during the census writes to a different ticket (`26/08/17/FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG`), not this target.
- The named paths are not ignored: `git check-ignore -v --no-index` produced no match for each named `.txt` file, the recovered-log path, or either CAD target directory. The disappearance cannot be attributed to an ignore match.

## Confirmed Destructive Code Path, Not a Proven Invocation

The root workspace cleaner contains a code path that recursively deletes a ticket-local Cargo target while a ticket is active:

1. [`📜️script.ts`](/Users/ueli/Documents/semio/📜️script.ts:19343) sets a 10 MiB maximum for every directory below an individual ticket folder and does not exempt active tickets or `CARGO_TARGET_DIR`.
2. [`cleanTicketSizeRemovals`](/Users/ueli/Documents/semio/📜️script.ts:20883) recursively records every child directory above that threshold.
3. [`runWorkspaceClean`](/Users/ueli/Documents/semio/📜️script.ts:20957) applies those records for every discovered canonical ticket root.
4. [`cleanRemovePath`](/Users/ueli/Documents/semio/📜️script.ts:20748) performs `rmSync(abs, { recursive: true, force: true })`.

`🧱️cargo-target-cad` is inside the master ticket, so an invocation of the ordinary root `clean` command while the target exceeded 10 MiB would delete it wholesale. Cargo can subsequently recreate only top-level bookkeeping files before a failed compilation reaches `debug/.fingerprint`, which matches the current failure shape. The code path is proven; no current or historical process evidence proves that `clean` was actually invoked for this loss.

The destructive root-clean command is registered in [`.vscode/launch.json`](/Users/ueli/Documents/semio/.vscode/launch.json:2906). No automatic invocation was found in the inspected root/package/Nx configuration; the registration is a manually launchable command. This audit cannot establish whether it ran or which actor invoked it historically.

## Eliminated Candidates

- [`runCargoTestBudgeted`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:1593) preserves the caller supplied Cargo environment. Its only relevant `rmSync` removes an internally-created nextest metadata directory when `SEMIO_TEST_ARTIFACT_DIR` is absent; it cannot remove `CARGO_TARGET_DIR`.
- The registry `generate` runner deletes only names not in its own `🤖️generated` directory ([`📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:1853)). It cannot delete ticket logs or Cargo targets.

## Exact Text-Log Cleanup Rules

Coordinator evidence supplement: the recovered Flow diagnostics file was directly statted at 20,491 bytes before its later disappearance. This is below the cleaner's 5 MiB file threshold, so that individual size rule cannot explain its removal. An ancestor removal or another mechanism would need independent evidence; the source audit must not be treated as proof that the cleaner caused every missing log.

The ordinary root cleaner treats every canonical ticket tree as eligible. For every ticket folder it first collects Git-ignored files/directories (`cleanGitignoredUnder`, [`📜️script.ts`](/Users/ueli/Documents/semio/📜️script.ts:20864)), then independently applies size deletion rules:

- every regular file greater than 5 MiB is a `ticket-file` deletion candidate ([`📜️script.ts`](/Users/ueli/Documents/semio/📜️script.ts:20909));
- every child directory greater than 10 MiB is a `ticket-dir` deletion candidate ([`📜️script.ts`](/Users/ueli/Documents/semio/📜️script.ts:20913));
- all selected candidates are recursively removed by `rmSync` ([`📜️script.ts`](/Users/ueli/Documents/semio/📜️script.ts:20748)).

The five named paths are not Git-ignored, eliminating the first rule for them. Exact pre-deletion sizes and an opener/deleter process were unavailable because all five files were already absent and no deleted-but-open descriptor remained. The size rules are therefore a viable but unproven cause for the `.txt` loss.

## Active-Ticket Protection Gap And Minimal Repair

The master manifest declares `"status": "open"` in [`🎫️ticket.json`](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🎫️ticket.json:4). The repository's ticket vocabulary accepts precisely `open` and `closed`, but the cleaner never reads `🎫️ticket.json`; its protected prefixes are only map, hub, space, and cleaner-cache roots ([`📜️script.ts`](/Users/ueli/Documents/semio/📜️script.ts:20720)). Thus an active marker already exists but is ignored by cleanup.

Minimal safe repair: before `cleanGitignoredUnder` or `cleanTicketSizeRemovals` is called, parse the ticket folder's `🎫️ticket.json` and skip the whole folder unless its status is explicitly `closed`. The root cleaner should fail safe (skip and report) for a missing, unreadable, or invalid manifest. This preserves every active ticket's logs and caller-owned warm targets without inventing another marker or weakening cleanup of closed-ticket disposable artifacts.

## Safe Verification Strategy

- Before starting an expensive command, create its log under the active ticket and retain a small completion manifest beside it containing absolute path, inode, byte count, and SHA-256.
- While the command is running, record the same facts at start and completion. If the file disappears, immediately record `lsof +L1` for the exact path and the focused process census before rebuilding.
- Do not invoke the root `clean` command while a ticket retains active evidence or a warm target. Implement the manifest-status guard above in a separate implementation task before using workspace cleanup during active work.

## Conclusion

The root cleaner is the confirmed in-repository destructive mechanism that can erase the warm Cargo target, and it permits the observed missing `debug/.fingerprint` state after Cargo begins recreating the target. Its invocation is not proven. No inspected runner or registry generator can cause that target loss. The known text logs are absent and unignored, but the available read-only evidence does not prove whether they crossed the cleaner's 5 MiB threshold, were contained by a qualifying directory, or were deleted by another actor.
