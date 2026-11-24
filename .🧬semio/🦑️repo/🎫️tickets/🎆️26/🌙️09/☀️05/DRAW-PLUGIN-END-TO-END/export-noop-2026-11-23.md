# Export Drawing Does Nothing (2026-11-23)

## Symptom
Engagement panel, `action.exportDocument`, format PDF, `...action.exportDocument.execute`: no download, no `URL.createObjectURL`, no anchor click, no console error.

## Chain trace (probe `probe-export-noop.mjs`, logs in `🗑️generated/probe-*`)
| link | result |
|---|---|
| click on execute | `onExecute` fires, the command page (463 B, action `exportDocument`) reaches the guest as a `turn` with `commandPage` |
| guest command | admitted, `commandIngress: command-pending` for ~690 turns, then the "started" `Invocation` frame and `Done` frame (the command promise resolves with NO requested effects: `performInvocation leftover exportDocument []`) |
| guest job and export bytes | the retained export operation runs to the end in the same actor: 3447 B PDF (`["semio.pdf","application/pdf","base64",4596]` on the typed-operation download page, lane 9) |
| host page decode | `consumeTypedOperationEffects` converts lane 9 into the effect `download-media-export` (operation id as `data`, marker `semio-segmented-handle-v1:base64`) |
| host delivery | the effect lands in `pendingCompletionEffects` (drain path, `drainTypedOperations`), settle status `idle`, frames `[]` |
| host download plumbing | never reached: `applyDownloadMediaExportEffect` is not called |

## Root cause
`pendingCompletionEffects` is only published by `subscribeOperationCompletions`, which fires on an `AppFrame::OperationCompleted`. The guest creates that frame from `typed_completion_outbox` witnesses, and the witness is pushed only when a `Terminal` result page is planned (`typed-operation-unit` publication). A download operation ends on the `Download` lane (its ACK moves the operation to `Retiring`, exactly like `Terminal`) but never pushed a completion witness. Consequently no `OperationCompleted` frame was ever produced for an export, the command's own reply had already resolved without effects, and the `download-media-export` effect stayed parked forever. Nothing faulted, so nothing was logged.

Verified by a temporary host hack (published a synthetic completion when the drain parked a `download-media-export`): `applyDownloadMediaExportEffect` ran, `createObjectURL(3447 B)` was called and Playwright received `DOWNLOAD semio.pdf`. The hack and all `[DEBUG]` logs are removed again; the host download plumbing (`SegmentedDownload`, `ShellHelpers` downloads) needed no change.

## Fix (guest Rust, component rebuild required)
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🪢️members/📥️staging/🧾️typed-operation-results/🦀️.rs` (`acknowledge_typed_operation_result`): when the download page is acknowledged and the retained download moves into `segmented_downloads`, the operation's completion witness is pushed (after checking the outbox has room before touching any state, like the existing `can_insert` checks). The host has by then consumed the page and parked the effect, and the `OperationCompleted` frame follows in a later turn, so the completion delivers `requestedEffects: [downloadMediaExport]` to the shell, which applies it through the existing `useOperationCompletionSubscriptions` pass.

## Memory (the `322699264 B` warning)
Measured the guest linear memory (hooked `WebAssembly.Instance`) around the export on a bare boot:
- 288,096,256 B before the click, 290,979,840 B after the export finished: the export costs about 3 MB and needs no 300 MB.
- The baseline is the boot-time pool (`OriginalReturnEscrowAmounts` shows `capacity_bytes` of about 267 MB in every `scope body` debug line). The 60 percent ceiling (322.7 MB) is crossed in journeys that attach a folder (sync/canonical-store sizes) before the export, so the warning in r13 coincides with the export step but is not caused by it.
- The export job progress: about 690 steps of ~10 work units each (the retained command spins `more-work` until the encoder finishes), roughly 3 s wall for a 1-layer document in swiftshader headless; no stall.

## Side observations
- The filename is `semio.pdf` (document id stem), not a drawing title; unrelated to this bug.
- The `[DEBUG] scope body / canonical source zero progress / sealer blocked` lines are other agents' temporary logs.
