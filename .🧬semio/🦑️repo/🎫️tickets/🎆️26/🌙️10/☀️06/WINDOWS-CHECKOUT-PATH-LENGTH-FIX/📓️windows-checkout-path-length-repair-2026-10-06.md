# Windows Checkout Path Length Repair

## Problem

`git pull` on Windows at `C:\git\semio` failed with `Filename too long` while materializing ticket fixture trees under `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/cargo-caller-inputs/📸️pred/🧰️framework/...`. Five tracked paths reached 261–286 UTF-16 units below `C:\git\semio`, above the legacy `MAX_PATH` budget of 260 even with `core.longpaths=true`.

## Root cause

- Windows compares UTF-16 code units, not UTF-8 bytes.
- The offender mirrored full repo paths twice (`📸️pred` and `🧩️call`) under an already long ticket slug and input folder name.
- Renaming segments alone was not enough; the mirror trees had to go.

## Fix

### 1. Consolidate cargo-inputs mirrors into ledgers

Replaced `cargo-inputs/📸️pred/` and `cargo-inputs/🧩️call/` directory trees (~117 files) with compact ledger JSON:

- `cargo-inputs/📥️ledger/📸️pred.json`
- `cargo-inputs/📥️ledger/🧩️call.json`

Refactored `cargo-inputs/📜️script.ts` to read/write ledgers instead of mirrored paths.

### 2. Shorten remaining long segments

| Before | After |
| --- | --- |
| `controlled-cargo-caller-inputs` | `cargo-inputs` |
| `📸️predecessors` | `📸️pred` (ledger key only) |
| `🧩️callers` | `🧩️call` (ledger key only) |
| `📋️owner-command-policy` | `📋️owner-cmd-policy` |
| `production-provider-drafts` | `prod-drafts` |

### 3. Remove ticket garbage

- Deleted 18 oversized `CURRENT-*-AUDIT.md` files (5–64 MB each) from `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/`.
- Ran `bun ./📜️script.ts clean` to remove gitignored ticket junk and oversized artifacts.

## Verification gate

Extended `🧫️fixtures/🪟️windows-checkout-paths/🔣️.json` to version 2 with two checkout roots:

- `C:\Users\username\source\repos\semio` — ticket files only, max 260 units
- `C:\git\semio` — all tracked files, max 259 units

`🧪️tests/🔬️workspace-contract/🟦️.ts` enforces both roots.

## Result

After repair: **0 tracked paths ≥ 259 UTF-16 units** at `C:\git\semio`. Max tracked path ~244 units.

Windows developers should still run `setup git` (sets `core.longpaths=true`) and enable OS long-path support, but the repo no longer depends on those settings for the previously failing checkout paths.
