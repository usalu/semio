# Presence and Tutorial Integration Review

## Reader Return Ordering and Board Loader

Both coordinator and executor inspected a prerequisite reader-return race. SnapshotRead return/Drop and ErasedSnapshotRead Drop currently announce a returned lease before releasing their own Arc. Concurrent maintenance can then take and release the registry owner first, leaving an ordinary reader Drop as the unbounded final destructor. Return must release the reader Arc while the unreturned registry still owns the root, then publish the returned flag. The private erased-to-typed transfer has the same early-return hazard: it must atomically take the exact registry slot before announcing return, preserving the untouched capability on lock contention. Cross-worker and contended-transfer laws are assigned before Presence adopts atomic final-owner retirement.

The coordinator also confirmed the renderer's Board loader resolves BoardSession from the shared framework surface despite its only real definition living in the Puzzle2d editor Wasm module. That module explicitly owns puzzle-domain seeds and types. The chosen repair is a puzzle-owned registered factory and real package/build route at product composition, with a domain-neutral renderer interface. No fake shared export, generic constructor cast, or permanently unavailable Board fallback is authorized. Its synchronous whole-vector-scene rendering remains a separate bounded-rendering obligation.

## Current Registered-Dispatch Checkpoint

The executor's r8/r9 native attempts advanced past the real pending-key ownership failure but did not pass. The coordinator read r9's actual runtime output: publication reaches AwaitingAck with `ValidationFailed("one-item publication requires preinstalled fixed applied and revision capacity")`; the document remains 13 instead of the expected 97. Strict Store Drop during assertion unwind aborts the process. The executor is repairing capacity authority across the intervening cold edit/rebase, keeping the production validation guard. Exact log: `🧪️member-latestwins-registered-r9-native-2026-08-27.txt`.

Source review additionally requested exact UTF-8 byte accounting in pending-key retirement and a proof that lost slot reservations cannot leave a closed pending entry permanently reinserted. The raw-wire owner independently requires an entire 4,096-byte page grant and omits retained vector capacity from terminal emptiness; a dedicated schema-first repair now covers zero/1/64/4,096-byte grants. These are active defects, not completed laws.

Presence local captures are moving to registered reader leases so operation cancellation can return a read owner while the Store remains live. Peer roster release needs exact final-owner participation across overlapping retired roots; neither an ordinary final Arc drop nor waiting on another retirement owner is acceptable. The executable mounted overlap/cancellation laws remain pending.

## Real Presence Cleanup

The coordinator read the new Store detach cursor and concrete CAD presence retirement. The domain cursor accounts for all three variable string fields and waits for captured readers; its terminal value must explicitly empty those fields because CadPresence::default contains nonempty utility/engagement strings. The Store detach makes local/peer roots unavailable for further mutation and routes late peer commits back as retained owners instead of dropping their payloads.

Two independently retired roster roots can share unchanged peer entries. The existing per-entry try_unwrap retry then creates a permanent wait: each retirement owns a reference the other requires to disappear. The executor is replacing this with an exact shared-entry/final-owner lifecycle and routing displaced/cancelled roster roots through it. An ordinary final Arc drop is not an acceptable shortcut. Tests must include overlapping roots, held readers, and cross-worker release.

The same concern applies to captured local presence roots. A mounted operation may cancel and retire its captured local root while the app's PresenceStore intentionally remains alive. Requiring the live store to release that root first can prevent the operation from closing, and closing operations before the app store then deadlocks shutdown. The executor has an explicit requirement to test operation cancellation while the app stays open, in addition to final app shutdown. No CAD shared-close pass is claimed.

The new registered-dispatch native test also uncovered an actual pending-admission lifecycle failure after the fixture manifest and process-pool invariants were repaired. Its r7 log reports the strict PendingLatestWinsCommand owner guard, followed by Store Drop panics during unwind. The executor owns the compiler lease and is repairing the exact transfer/retirement, not removing the guard. Log: `🧪️member-latestwins-registered-r7-native-2026-08-27.txt`.

## Tutorial Local Selection

The coordinator independently confirmed the renderer executor's reported contract gap. Tutorial snapshots require full per-domain local selection, including anchors and non-broadcast domains. ActiveSession.viewState has no such field. AppFrame::Ephemeral contains only the declared-broadcast projection; the current renderer handle leaves ephemeralSnapshot undefined. Remote Presence ingress is not authority to restore local selection.

The inherent VcsArtifactApp::interaction_state method currently clones the complete local interaction snapshot plus hover; it is not a bounded channel query. Existing interactionSelect applies active-mode/topology normalization and cannot guarantee exact snapshot restoration. Native Shell tutorial capture currently supplies an empty selection map and its restore path ignores selection, so native parity also needs implementation.

The Flow/interaction executor owns a schema-first bounded local read and typed atomic restore packet after the current wire/check boundary. Reads need captured instance/revision/generation authority and retained pagination. Restore belongs to the local interaction lane, must clear absent domains, preserve IDs containing commas plus granularity/anchors, validate declarations/topology, and return explicit rejection. Non-broadcast local selection must remain private from peers. The renderer executor owns its consumers and current typed UI repair; no removed selectionJson field or no-op fallback is authorized.

## Verification State

Native compound-envelope reads pass 2/2 and neural regression passes 43/43. The latest complete lifecycle cohort was 9 passed/1 failed; the later registered-path attempts aborted and are not passes. Presence overlap, local operation cancellation, tutorial local read/restore, fresh Wasm, all-app workflows, and eight-millisecond gates remain open.
