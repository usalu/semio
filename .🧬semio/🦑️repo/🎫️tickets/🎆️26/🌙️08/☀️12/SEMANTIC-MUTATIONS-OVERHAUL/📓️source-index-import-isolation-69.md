# Source Index Import Isolation 69

## Terminal Context

The only real SourceIndex pair timed out before emitting its first child event, snapshot, marker, or progress record. The retained terminal evidence is [Source Index Pair — Sole Bounded Attempt 68](./source-index-pair-terminal-68.md). It does not identify import, source admission, source-file capture, classification, or any individual source as the cause.

## Proposed Import-only Boundary

[The new controller](../🧪️source-index-import-isolation-69/📜️script.ts) dynamically imports only the current root `📜️script.ts` in one owned child. It emits a marker immediately before import and another only after import resolves. It does not call `mutationTaxonomySourceIndex`, `inventoryTaxonomySources`, a classifier, or any inventory/CLI function.

The parent concurrently drains both child pipes and enforces one ten-second child lifetime. It clears the timer at normal terminal state; at expiry it signals only that child, with no grace extension or retry. It retains no-follow before/after captures of the controller and root entry, child stdout/stderr, and a terminal receipt in its exclusive run directory. Root review is required before `run`; no import test has started.

## Read-only Boundary Audit

Current root module evaluation statically imports the package TypeScript entry, discovery, and normalization modules. Thus this diagnostic can distinguish delayed root-module import/evaluation from the later exported-function execution boundary, but cannot assign a delay to a particular static dependency or top-level initializer.

`mutationTaxonomySourceIndex` is an exported synchronous function at root `📜️script.ts:20806`. On invocation it first calls private `mutationTaxonomySourceAdmission` (`:20751`), which calls `inventoryTaxonomySources` in normalization (`🟦️.ts:2910`). Only after completed admission does SourceIndex capture taxonomy/descriptor bytes, discover mutation roots, select source file facts, and iterate each selected file through `semanticOwnedInputFileSnapshot` (`:20807–20835`). The `mutationTaxonomySourceFileFacts` classification projection is at `:20779`; it is not reached by import alone.

Consequently, an import-only timeout is compatible with root/static-module evaluation delay. An import-only completion cannot establish that admission, Git collection, file observation, source snapshotting, root discovery, classification, or any semantic inventory succeeds. This controller creates no new roots, source authority, schema, or completion claim.

## Root Review And Superseding Preparation

Root fully read this controller before execution and found that workspaceRoot did not handle ENOENT while walking from the ticket. That would have stopped before child creation, not tested import. Root corrected only that lookup to continue on ENOENT/reject unsafe markers, added workspace-root and initial-leaf identity checks, recorded capture/terminal time and child PID, and renamed the receipt claim to sourceIndexDirectlyInvoked=false. Imports may evaluate static dependencies; the controller makes no assertion that an arbitrary imported initializer cannot do other work.

Current controller:7226bytes, SHAa8b0816ce9634d72346960683973eb3dcef012b7e40820c192fd582ba7304da5. The old0475… preparation hash remains historical. The corrected plan command exited0 (session28260) without importing root or creating a child. Actual run is still unexecuted at this release.

Exact launch proposal (pending canonical name/order/nofollow/pure-producer admission):

```json
{
  "name": "⚖️gate🧬️mutations🧾️import-isolation",
  "type": "node-terminal",
  "request": "launch",
  "command": "bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun \".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-index-import-isolation-69/📜️script.ts\" run",
  "cwd": "${workspaceFolder}",
  "presentation": {
    "group": "4_gate",
    "order": 410.528
  }
}
```

This is one ten-second import-only check, not a retry of the timed-out SourceIndex pair. No SourceIndex/inventory function is directly invoked and no source/API changes are included.

## Actual One-Run Import Result

The root executed the corrected `run` command exactly once (session21472), after the non-importing plan. The exclusive run is [run-sAYcAW](../🧪️source-index-import-isolation-69/🧫️run-sAYcAW/receipt.json). The owned child14566 exited0 without timeout; its two markers were 2026-08-28T03:19:47.475Z (before-import) and 03:19:47.610Z (after-import),135ms apart. Root independently checked that child PID absent after terminal state. stdout was181bytes and stderr0; no SourceIndex or inventory function was directly invoked.

The receipt reports exact before/after equality for the controller (a8b0816ce9634d72346960683973eb3dcef012b7e40820c192fd582ba7304da5,7226bytes) and root script (c539f565831cce420d5c755f2efe54362956d0f0d769a29ea3805a2e9f62d005,2832773bytes), including dev/inode/size/mtime. Only those two inputs were captured; static dependency inputs were not frozen or exhaustively captured. In particular, this new endpoint must not be equated with the earlier pair's six-input endpoint or used to assign the old timeout's cause.

Retained files, independently read after completion:

| File | Bytes | SHA256 |
| --- | ---: | --- |
| before.json | 766 | 478b559a713936d4329a01c4b2a75e1814e9d02b495af0183b80ec586c89d9ad |
| child.stdout.log | 181 | 7ea88521b8904f6fb1cfa743e804b8e939443a56ad3dae41e2eeeed12f4fdf15 |
| child.stderr.log | 0 | e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 |
| receipt.json | 1893 | 87889d7fb5228985c74aa82972566227dd5833e276881013d1caeaaf500f9d79 |

A first root read mistakenly requested `🔣️receipt.json`; the controller actually writes `receipt.json`. Exact directory listing and corrected full read found all four original files. That mistaken filename is not evidence of disappearance. No files were replaced or reconstructed.

Current root import completed. Admission, first SourceIndex completion, paired membership stability and semantic completeness remain unproved. The original timed-out pair is preserved unchanged; no pair retry or budget increase occurred. Registration410.528 is still a separate pending canonical join at this observation.

## Pair Instrumentation Limitation

Root's subsequent complete read of the pair controller confirms that its progress callback only accumulates counts in memory. The child's sole stdout marker is intentionally emitted after both SourceIndex calls, both snapshot serializations, and drift serialization. Thus zero stdout contains no evidence that invocation, import, admission, or source capture had not begun. Absence of before-manifest.json means the first snapshot plus that manifest write did not complete; it does not alone prove the first SourceIndex call failed to return. The next diagnostic must add phase observations around the actual unchanged API rather than infer a stage from this silent controller.
