# Result c6inf (packages P32, P33, P34, P35 of audit-monolith-census-6.md)

## Verdict

NOT STARTED on the split itself. The coordinator ended the slice (usage budget) before I cut any file. No repo source file was edited, so the tree is consistent. No cargo/bun/nx ran. No background command was started.

## What exists

- Tools workspace prepared: `C/c6inf-tools/`, a copy of `C/c6flow-tools/` with the `orig/` snapshots emptied. Its `modsplit.py` already emits explicit (non-glob) parent re-exports via `explicit_reexports`, so the "no glob re-exports" rule holds.
- Left to do before use: the copied scripts still point at `/Users/ueli/.cache/c6flow` (`emo.py`, `ambig.py`, `modsplit.py`, `frag.py`, `chk.py`, `vmod.py`, `oracle5.py`, `census-flow.ts`, `install.py`, `posrun.py`). Re-point them to `/Users/ueli/.cache/c6inf` (created, empty).
- Method (from reading `F/c5b-tools/modsplit.py` and `F/RESULT-CENSUS5-G.md`): record the original into `orig/<tag>.rs`, write `specs/<tag>.py` (children via `ch(emoji, kebab, mod, [(first_line, last_line)], doc)`, `wiring='block'`, `external=mounted(...)`), then `modsplit.py stage|lossless|paths|install`, then `vmod.py`, `defscan.py`, `ambig.py`, `modclash.py`, `fmt.py`, `readers.py`.

## Remaining list (all 13 files, none touched)

Prefix `MOD` = `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board`; `N` = `MOD/🔌️ports/➡️directed/➕️normal`; `D` = `MOD/🔌️ports/➡️directed/🕸️dag`.

| Pkg | Lines | File |
| --- | ---: | --- |
| P32 | 3,307 | `MOD/⚙️engine/🦀️.rs` |
| P32 | 2,796 | `N/🧭️host/🦀️.rs` |
| P32 | 3,144 | `N/🪣️fill/🦀️.rs` |
| P33 | 2,132 | `N/🎯️focus/🦀️.rs` |
| P33 | 2,406 | `N/🖼️scene/🦀️.rs` |
| P33 | 2,063 | `N/🗃️model/🦀️.rs` |
| P33 | 2,548 | `MOD/🧬️schema/🦀️.rs` |
| P34 | 1,599 | `N/🕹️gestures/🦀️.rs` |
| P34 | 1,644 | `N/🧭️host/🧪️tests/🔬️board-host/🦀️.rs` |
| P34 | 1,987 | `D/🕸️graph/🦀️.rs` |
| P34 | 1,943 | `D/🗃️model/🦀️.rs` |
| P34 | 1,765 | `D/🧭️host/🦀️.rs` |
| P35 | 1,462 | `D/🧭️host/🧪️tests/🔬️dag-unit/🦀️.rs` |

All were listed as PEER-held in the census; the coordinator owns them. Re-read before editing. Check the per-crate source gate (4,000 lines / 32 files / 256 KiB) via `census-own.ts` before any split; the board crates may need sibling crates.

## Crates to compile

None (no source changed).
