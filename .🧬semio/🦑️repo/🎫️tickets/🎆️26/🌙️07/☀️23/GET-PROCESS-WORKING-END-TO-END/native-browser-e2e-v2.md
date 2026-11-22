# Native Browser E2E v2 (rebuilt hub wasm, Firefox BiDi)

## Result: NOT green. Different failure than v1: the command-ingress trickle is gone, the guest now faults the instance.
Success criteria: Set Active Example answered = NO; canvas >= 1 = NO (0); no error logs = NO; screenshot exists = yes (`🗑️generated/e2e-process.png`, shows only a red line "original-turn.native-return-refused: semantic" on an empty dark canvas with the chrome bars).

## What was done
1. Hub wasm rebuilt natively (same wasm-prototype flags, same `native/target` + `native/build`, -j 4, no lock contention): exit 0, 400,059,993 B at 12:49 (log `🗑️generated/hub-v2.log`). Disk before 28 GB, after 14 GB (the build consumed ~14 GB of new intermediates in native/build; native is now 85 GB). A further rebuild would breach the 10 GB floor, so I stopped building (rule: stop and report).
2. Republished with `publish-modules.ts` (closure-hub.json, process only; extensions not rebuilt: not required for the stall, their wasm are from the old framework) and `activate-native.ts` (process-closure.json). Receipt updated (5 plugins).
3. Vite on 6022 (PID 25230) still running, serves 200.
4. `SETTLE_MS=150000 verify-process-firefox-bidi.ts`: stdout in `🗑️generated/e2e-stdout-v2.txt`.

## Timeline / evidence
- 10 s: banner "has not answered setContributions for 7 s". Console: `[os-shell] command stalled: process setContributions has held process#1's lane for 5000 ms`.
- Then (before 20 s, v1 waited 300 s): `setContributions command failed ... interactive-job.publication-stalled: typed operation 'setContributions' advanced 4096 publication units without moving its ladder` (retainedProgress all 0).
- Guest stderr: `[DEBUG] FEM cleanup semantic failure instance=1 stage=24 progress=Some(Progress(zero))` = the live maintenance pump (`⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns`) hit `RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT` on maintenance Stage(24) = the retained DOCUMENT ARCHIVE LOAD. That faults the instance ("semantic"), host sees `original-turn.native-return-refused: semantic`, `shell.documentTransfer.load-failed`, `setActiveExample refused: dispatch-failed`. 0 canvases. 20-150 s timeline: constant blank, no recovery.
- Interpretation: the old trickle (stale scrub shell / close ladder) is fixed by the framework fixes. The next blocker is the archive load (stage 24) starving: same as editor-agent open item 2 ("archive load stuck Running, 127 of 127 records, host lifetime fuel exhausted"), plus the publication stall of setContributions waiting on it (its publication ladder cannot advance while the archive load makes no progress).

## Source state note
The working tree's `🔁️cleanup-turns` and `🪜️maintenance-ladder` already contain newer edits than the wasm I built (the debug print gained an `archive-load: ...` diagnostic via `document_load_diagnostic`; the ladder gained `archive_pending` / `DOCUMENT_ARCHIVE_STEP_MINIMUM_ITEMS` / `document_archive_waits_for_host` gating around Stage(24)). Another agent is actively working this exact defect. Next rebuild will pick that up and print the archive diagnostic.

## Next steps (not done: disk floor)
- Free disk (native/build holds the editor agent's intermediates; owner must decide), rebuild hub, republish (`publish-modules.ts` closure-hub + `activate-native.ts` process-closure), rerun the check; the `[DEBUG] ... archive-load:` line should say why Stage(24) makes zero progress (copy-grant vs per-record demand, or waiting for host acknowledgement).
- Watch whether production grants (`INTERACTIVE_COMPONENT` 32 KiB copy) fund the archive step, same family as the history-replay finding in native-rust-editor-tests.md.

## Files changed
None in source by this run. Ticket: this file, `🗑️generated/hub-v2.log`, `hub-v2.out`, `e2e-stdout-v2.txt`, `e2e-process.png` (overwritten). Published (cache): `.cache/b/p/m/browser/prototype/modules/🏭️process`, activation receipt.
