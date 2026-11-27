# Official XLSX browser gates on a full S host (port 6071)

Host: `CLEAN=1 S_DATA_DIR=/Users/ueli/Documents/semio/.cache/pane-data/gates-6071 ./claude-fulls-serve.sh s 6071` (the published and activated components were reused; `claude-fulls-serve.sh` gained the `CLEAN=1` parameter that wipes the data folder first). The data folder cannot live under this ticket: the shell refuses a kept-document target whose path plus the 60-character storage suffix exceeds 256 characters (`natural-file.invalid-request`, `ShellHost/🗂️local-catalog/📥️artifact/🟦️.ts`). Server stopped at the end.

Evidence paths are short on purpose (the stdio gate refuses planned paths over 256 UTF-16 units): `🗑️generated/g/<tag>/` (`e`, `d`, `me`) in this ticket; logs and table copies in `🗑️generated/claude-gates/`.

## Verdicts

| Gate | en | de | Evidence |
| --- | --- | --- | --- |
| Stdio artifact-acceptance, natural-file, xlsx (9 of 9 checks) | pass, exit 0 | pass, exit 0 | `🗑️generated/g/e/{journey.json,table.md,exports/}`, `g/d/...`; logs `claude-gates/acc-en.log`, `acc-de.log` |
| Hub I/O matrix `stdio-office/xlsx` | fail, exit 1 | not run (en failed; each run takes about 15 min through `repo run`) | `🗑️generated/g/me/{io-matrix.json,table.md}`, `claude-gates/io-en.log` |

The I/O matrix row: format export (`xlsx`, 1732 B, fflate and DOMParser oracles) ok, picker import round trip identical, 0 faults, but the framework document round trip is red: `export wrote no file` (`shell.documentTransfer.export-failed`, console: `Error: app-channel.disposed` for `stdio-office#3`).

The known 1-in-5 reopen defect did not appear in either acceptance run (one run per locale after the fixes, plus an earlier en run; all passed the close-reopen and reload-reopen steps).

## Root causes and fixes

1. Wrong owner pinned in the matrix. Row `xlsx` pinned `s.stdio.xlsx@ecma-376/*#editor`, but a natural open of the Transitional sample correctly resolves to `.../transitional#editor`. Fixed in `🧫️fixtures/🔣️.json`; contract test added ("pins the dialect editor that the sample content selects").
2. Stale edit witness. The harness counted edits from `#s-checkin` and ledger ids `framework.history.entry.*`, which the History panel no longer uses (ids are panel-scoped, `panel:<instance>::framework.history.checkin`), and the panel was never raised in the gate's rendered edit path. `readShell` now reads both id forms (visible check-in row); the gate raises the History panel for the edit and closes it again afterwards (`raiseHistory` re-exported through `testing/program-matrix`). A raised panel persisted across the reload and covered the Settings tab.
3. Repeated identical notices were not recorded (the host reuses one notice element), so the fifth save looked like "no file-saved". The gate now dismisses visible notices before each expectation (`clearNotices`).
4. Malformed-file refusal: the shell now answers `shell.openFile.content-mismatch`; the gate accepted only `file-open-failed` and `format-unresolved`. Accepted now.
5. German reload: `seatLocale` found the Settings control only by English text. It now also matches `framework.settings` and `Einstellungen`, and uses `clickUncovered`.
6. Open I/O matrix defect, not fixed (same lifecycle area as the shell agent's reopen fix, files changing live): after the matrix imports a file (instance #3) and closes its windows, the spawned entry stays active while its channel is disposed, so Export Document fails. Reproduced standalone: open xlsx via picker, close the table window, press `exportArtifactDocument`. Reported to the shell agent (ad858c7b51de8ab5a). Suggested fix: retire the spawned entry or re-resolve the target to a live session when the channel is disposed (`resolveSyncTargetSession` in `ShellHost/🗄️shell-uri-documents/🔶️scope/🟦️.ts`, `exportDocumentArchive` in `📤️document-transfer-tabs/🔶️scope/🟦️.tsx`).


## Files changed

- `✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🧫️fixtures/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🧪️contract/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🏃️browser/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🧪️testing/🧮️program-matrix/🟦️.ts`
- Ticket: `claude-fulls-serve.sh` (CLEAN), probes `claude-gates-open-probe.ts`, `claude-gates-ids-probe.ts`, this file.

## Follow-up 1 (done): stale selectors in the tool-run matrix

`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/⏯️tool-run-matrix/🟦️.ts` now reads the check-in row from `#s-checkin` or the visible `panel:<instance>::framework.history.checkin` row and the ledger from `[id*="framework.history.entry."]`. No localized labels are involved: the count is parsed from the trailing `(N)`, which is language-neutral. No contract test added: the file is a browser-driven script with no offline test seam, and its selectors are exercised by the real browser matrices (the acceptance gate uses the same id forms and passed in en and de). Not run in a browser.

## Follow-up 2 (pending): rerun of `stdio-office/xlsx` matrix row

Waiting for the shell agent (ad858c7b51de8ab5a) to report the `app-channel.disposed` / stale-active-target fix. It was messaged with the repro; its last status (2026-11-26 evening) was still working on the reopen loop and had not touched the matrix repro. Rerun plan: fresh 6071 server (`CLEAN=1`), `repo run os-hub:io-matrix --exclude-dependencies -- --serve http://127.0.0.1:6071/ --locale en|de --only stdio-office/xlsx --tag me|md --out <ticket>/🗑️generated/g` under `claude-resource-guard.sh` with MIN_DISK_GIB=20.
