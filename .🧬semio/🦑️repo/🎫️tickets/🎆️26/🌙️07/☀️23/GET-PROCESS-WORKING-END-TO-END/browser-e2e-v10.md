# Browser E2E v10: freeze diagnosis, cancel design, blocked browser

## Status: NOT done. Items 1-3 are not fixed. The last hours of work were diagnosis; at the end Firefox could no longer be launched from this session, so nothing further can be verified here.

## Blocker (new)
Headless Firefox no longer starts from the tool sandbox: `sandbox_extension_issue_file_to_process failed for /Applications/Firefox.app/Contents/MacOS/plugin-container.app: 1 (Operation not permitted)`, then no BiDi endpoint. It worked in all earlier rounds. I did not disable the sandbox (that would weaken a security control). To continue: allow the sandbox to run Firefox, or run `./run-e2e.sh` yourself with `STEPS="$(cat process-cancel.json)"`.

## 1. Viewer then Editor freeze: findings
- It is not specific to Concrete Forest: the same freeze reproduces on the Demo document (steps: wait for the initial document, click Viewer, anything after it). Viewer then Editor passed on Demo in v6/v7 (older hub), so it is a regression from the newer hub or the TS changes landed since.
- Evidence of a main-thread stall: right after the Viewer click the console log stops dead at `Render ... Waiting for Paint` (last entry at 65 s in a 9 minute run) and every later `script.evaluate` never answers (90 s timeout each, six times); screenshots still work (compositor). No `error`/`warn` entries, no `[os-shell]` line, no reload (no second `[vite] connecting`). So it is a synchronous JS stall after a React commit in the Viewer switch, not a guest panic or an app-instance restart error (those would log).
- Not caused by this ticket's TS changes on that path (contributions-refresh and the example picker are not involved in a role switch); the guest status change only touches the document load status arithmetic.
- Next step needs a JS profile of the stalled content process (macOS `sample` of the busiest `plugin-container`, or the Firefox profiler through `MOZ_PROFILER_STARTUP`, which did not write a file on SIGTERM). Candidate area: `switchToSessionRole` and the spawned-session path in `ShellHost`.
- No law written (no root cause).

## 2. Load cancel: findings
- The banner already has a cancel for a document load: `HistoryReprojectionStatus` (`ShellHelpers/⏪️time-travel`) offers `cancelReplay` for `kind === "load"`, which dispatches `historyEditCancelReplay`, and the guest cancels a live document load (`cancel_reprojection_turns`: `request_terminal(Cancelled)`, previous document stays, zero trace). A separate control wired to `CancelDocumentArchiveLoad` would duplicate that path; the AbortSignal route (`DocumentArchiveLoadHost.requestCancel`, `task.signal` in `loadDocumentArchive`) is the other existing cancel.
- Defect observed in v9: pressing Cancel replay at `129 of 135` did not stop the Concrete Forest load: the banner vanished about 90 s later and the example applied. So the guest cancel does not take effect once the load has passed the fold (the replacement in `AwaitReplacement` completes and commits), or the example switch re-applies the document after the cancel. This is where the fix belongs: after `request_terminal(Cancelled)` the replacement drive must stop at its next turn and restore the previous document, and the example pick must not re-apply after a cancelled load.
- Not implemented; `process-cancel.json` (steps: initial document, trace, pick Concrete Forest, wait for the banner, Cancel replay, wait for the load to stop, state, Viewer, Editor) is ready and currently fails at "state after cancel" (`Concrete Forest` applied).

## 3. Long-phase progress: findings
- Stage counter observed stuck at 129 of 135 during the long phase (fold 127 + 1 + stage 1; no archive members, so the member counter cannot move). The stage is updated only while the maintenance ladder polls `AwaitingMembers`/`AwaitReplacement`; the long phase is inside the replacement job's own steps (initializer/candidate views and the record receivers). Items-done/total must come from those drivers (`drive_candidate_views` has `view_member_ordinal`/`children.len()`; no member exists here, so the signal must come from the decode job: bytes or records decoded of the envelope). Not implemented.

## Harness changes this round
- Console errors starting with `[vite]` no longer fail the run (dev-server HMR noise).
- `evaluate` timeout 90 s; every scenario starts with a step that waits for the initial document (canvas present, no `Loading document`, example select present); a pointer step on a missing element is a failed step instead of a crash.

## Scenario status
`process-interactions.json` does not exit 0 (freeze after Viewer). Nothing of this round is green.
