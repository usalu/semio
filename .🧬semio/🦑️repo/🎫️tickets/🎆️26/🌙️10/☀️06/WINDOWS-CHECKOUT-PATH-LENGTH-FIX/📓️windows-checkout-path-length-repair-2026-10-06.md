# Windows Checkout Path Length Repair

## Problem

`git pull` on Windows at `C:\git\semio` failed with `Filename too long` while materializing ticket fixture trees under `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/controlled-cargo-caller-inputs/📸️predecessors/🧰️framework/...`. Five tracked paths reached 261–286 UTF-16 units below `C:\git\semio`, above the legacy `MAX_PATH` budget of 260 even with `core.longpaths=true`.

## Findings

- Windows compares UTF-16 code units, not UTF-8 bytes.
- All offenders lived in one ticket input tree that mirrored full repo paths twice (`📸️predecessors` and `🧩️callers`) under an already long ticket slug and input folder name.
- The deepest production-relative segment was `📋️owner-command-policy` under native orchestration tests.
- Non-ticket repo paths already stayed below 230 UTF-16 units at `C:\git\semio`.

## Fix

Shortened the offending segments while preserving mirrored repo semantics:

| Before | After | Saved (UTF-16) |
| --- | --- | ---: |
| `controlled-cargo-caller-inputs` | `cargo-caller-inputs` | 11 |
| `📸️predecessors` | `📸️pred` | 9 |
| `🧩️callers` | `🧩️call` | 5 |
| `📋️owner-command-policy` | `📋️owner-cmd-policy` | 7 |

Updated references in:

- `cargo-caller-inputs/📜️script.ts`
- `interface-port-owner-inputs/📜️script.ts`
- `CURRENT-CONTROLLED-CARGO-PUBLIC-SOURCE-GATE.md`
- `🧰️framework/.../📚️library/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/.../📚️library/🔣️taxonomy.json`
- `🧰️framework/.../📋️native-orchestration/{🧪️tests,🧫️fixtures,🧬️schema}/📋️owner-cmd-policy`
- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`

Removed the empty leftover `controlled-cargo-caller-inputs/` directory after the rename.

## Verification gate

Extended `🧫️fixtures/🪟️windows-checkout-paths/🔣️.json` to version 2 with two checkout roots:

- `C:\Users\username\source\repos\semio` — ticket files only, max 259 units
- `C:\git\semio` — all tracked/untracked-on-disk repo files, max 259 units

`🧪️tests/🔬️workspace-contract/🟦️.ts` now enforces both roots.

## Result

Filesystem scan after repair: max path 267 units inside the shortened ticket fixture tree, 230 units repo-wide; **0 paths ≥ 260** at `C:\git\semio` across the working tree.

Windows developers should still run `setup git` (sets `core.longpaths=true`) and enable OS long-path support, but the repo no longer depends on those settings for the previously failing checkout paths.
