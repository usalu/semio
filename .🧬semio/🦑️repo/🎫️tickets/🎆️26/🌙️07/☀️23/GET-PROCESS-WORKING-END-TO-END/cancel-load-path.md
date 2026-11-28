# Cancel-load path for Process 3D (hub v18 served at 6022)

## Verdict
The "Loading document" banner and its "Cancel replay" button never show for the Concrete Forest switch because the guest holds the entire load inside ONE poll turn. During it the shell receives no status. This is a product regression against v13, not expected behaviour and not a fast snapshot load. The shell publishes status correctly. No source fix was made: the cause is the guest turn scheduling and is owned by the hub work in progress.

## Path of an example switch (code evidence)
1. The combobox pick calls `dispatchActiveExample` (`🏛️ShellHost/🔎️command-bar/🔶️scope/🟦️.tsx:57-78`), which shows the transient "Opening example ..." notice (about 4 s) and dispatches `setActiveExample`.
2. The guest handler (`✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗿️artifact/🦀️.rs:44-71`) builds the snapshot and returns `Effect::LoadDocument{pack,spr}` via `reset_process3d_document_effect` (`✏️editor/🧰️kit/🦀️.rs:151-164`). The pack is tiny (2137 bytes for Demo, 2170 for Concrete Forest) and is expanded by the load itself.
3. The shell runs `applyLoadDocumentEffect` (`🏛️ShellHost/⚡️host-effects/📥️document-load-effects/🟦️.ts:26-70`) and then `loadDocumentPair`/`loadDocumentArchive` (`🔒️identity-document-ports/📎️hub-port-and-document-backbone/🟦️.ts:234-271`). Every polled status goes through `showLoad` and `programHistoryProjectionsWithLoadV1` (`🛠️ShellHelpers/🔩️plumbing/🟦️.ts:78-93`) into a `load` reprojection. `HistoryReprojectionStatus` (`🛠️ShellHelpers/⏪️time-travel/🟦️.tsx:495-521`, rendered at `🏛️ShellHost/🟦️.tsx:372`) shows banner and button via `historyReprojectionControlV1`.
4. The stepped driver is `ArchiveTransfer.drive` (`🧰️framework/🔨️modules/🧾️app-channel/🧳️archive-transfer/🟦️.ts`): admit, then poll, and it calls `progress(status)` only after each poll reply returns.
5. The guest side of a poll is the maintenance ladder (`🔌️plugin/🧵️instance/📥️staging/🪜️maintenance-ladder/🦀️.rs`). It reports `AwaitingInput "funded archive or replacement phase awaits its original host, worker or replacement owner"` and the `[DEBUG] archive/replacement wait` counter (line 21-25, 79) increments about 9000 times per second inside one turn. `should_yield` stays false: fuel is about 49.998M of 50M and `deadline_exceeded` never fires (`🔨️modules/🧵️job/⏱️context/🦀️.rs:556,629`). The status that would drive the banner exists (`reprojection_status`, `🧵️instance/🖼️render/📥️staging/📜️history-view/🦀️.rs:508-511`; `document_archive_status`, `🧵️instance/📄️docs/📥️staging/🧱️components/🦀️.rs:235`), but nothing can read it while the turn runs.

## Evidence (temporary `[DEBUG]` probes, removed afterwards; `git diff` on both touched TS files is empty)
Per-poll reply timings from `ArchiveTransfer.drive` (run `probe4`, `🗑️generated/probe4-live.txt`):

| load | admit reply (seq) | first poll reply | next | status seen by the shell |
|---|---|---|---|---|
| boot Demo | 35 ms (11) | 65825 ms (12) | 992 ms (13), 47 ms (14) | "running 62895/62886" at the end, then ready |
| Concrete Forest | 95 ms (49) | 33941 ms (50) | 4 ms (51) | "ready 392238/392224" only, no running status at all |

Cancel needs a reply round-trip (`requestCancel` is read at the top of the drive loop), so no cancel can reach the guest during those 34 to 66 s. Probe with a MutationObserver installed before page scripts (`probe-preload.js`, `process-cancel-probe.json`, run `probe2`): `[data-semio-history-reprojection]` appeared exactly once, for about 1 s at 74.6 s (boot Demo load, `kind=load`, text "62895 of 62886", button present), and never for the Concrete Forest pick (only the "Opening example Concrete Forest" notice 83.6 to 87.7 s). The main thread stayed responsive (BiDi evaluates answered every 5 s), so this is not rendering starvation. The v13 run (`browser-e2e-v13.md`, cancel at "87455 of 87461") shows polls yielded then; they no longer do.

## Reachable cancellable loads
- Example switch (Demo, No example, Concrete Forest): the only reachable whole-document load. Its banner works as designed, but the poll turn is unobservable on v18.
- Document file open (`applyRequestFileOpenEffect`, `...document-load-effects/🟦️.ts:73+`): needs the OS file picker, which is unreachable in headless Firefox over BiDi.
- `cancelTypedOperation` / "Cancel Operation" belongs to typed operations (raster, playbook procedural) and is not offered by process3d. It is not the same requirement and was not used.
- History replay is unrelated to a load and has no long operation here.
So no reachable path can show the banner on this hub. I did not substitute a weaker check.

## Scenario `process-cancel-load.json` (the product requirement, strict)
Steps: wait for the Demo document, record before-state (combobox, canvas count), arm a MutationObserver that clicks the `load` banner's "Cancel replay" as soon as it appears, pick Concrete Forest, require banner and click (240 s), require the load to stop with no banner, require the document unchanged (combobox still "Demo", canvas present; failure is reported as `[exception]`), wait 30 s and re-check, then Viewer, Editor, Actions, Increase, Undo and a final state. Console errors fail the run, `[DEBUG]` lines do not.

Run `cancelload1` (`LIGHT=1 TAG=cancelload1 SCENARIO=process-cancel-load.json ./run-forest-render.sh`, exit 1, expected on v18):
- banner shown and Cancel replay clicked: `[timeout]` (record `{"seen":null,"clicked":null,"banner":null}`)
- document unchanged after cancel: failed (Concrete Forest instead of Demo, because the load ran to completion uncancelled)
- Viewer, Editor, Actions, Increase, Undo: all clicked without console errors
It will pass only on a hub whose poll turns yield so that the banner becomes observable. Re-run it on the new hub. It supersedes `process-cancel.json` (same observer idea, but its checks cannot fail on a drifted document).

## Needed fix (hub owner)
Make the guest return from a poll turn while the archive/replacement phase only awaits (the `AwaitingInput` branch above): stop re-entering the ladder inside one turn so the host gets a status reply, and let `deadline_exceeded`/fuel bound the turn. A native law should assert that a load whose ladder reports only `AwaitingInput` ends the poll turn with a `running` status. I did not change Rust: that code is under concurrent edit (the `maintenance-ladder` file changed during this session) and the coordinator's rebuild is pending.

## Environment notes
- Headless Firefox was killed externally (`SIGKILL`/`SIGTERM`, `fin 2`) on 3 of 9 runs, leaving SIGSTOPped orphan `plugin-container` processes. I killed those orphans (ppid 1, state T). Reruns passed. Some other process or guard is killing Firefox.
- `run-forest-render.sh` gained an optional `PRELOAD_FILE` (script installed before page scripts); `probe-preload.js` uses it.

## Files
`cancel-load-path.md`, `process-cancel-load.json`, `process-cancel-probe.json`, `probe-preload.js`, `run-forest-render.sh` (PRELOAD_FILE). Run outputs: `🗑️generated/probe{1..4}-*`, `cancelload1-*`.
