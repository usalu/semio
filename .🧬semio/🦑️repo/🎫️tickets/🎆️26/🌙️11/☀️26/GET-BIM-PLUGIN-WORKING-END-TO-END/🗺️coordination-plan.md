# 🗺 Coordination Plan

Host: macOS, 10 cores, 16 GB RAM, 74 GB free disk; at most two concurrent cargo builds at `-j 4`.

## Done
- 05:53 Ticket opened under goal `runningplugins` (no dedicated bim goal exists).
- 05:55 Agent rules written (`📓️agent-rules.md`).

- 06:00 ✅ typescript (`typescript.md`): typecheck 62 files 0 errors; sqlite snapshot 13, schema parsers 68, render world 6 pass. Python oracles not run (agent missed the locked env).
- 06:00 ✅ audit (`audit.md`): catalog regenerated 11-25 19:38 after descriptor 12:03; bim not in launch.json; feature gaps costs/modify/gestures.
- 06:01 `repo dependencies ensure` OK; locked python env with ifcopenshell/shapely at `.cache/b/d/m/python/environments/workspace-3b97251b6599d245`.
- 06:02 Wave 2: oracles agent (`oracles.md`), twins agent (`twins.md`).
- 06:03 Added `bim-react` to `.claude/launch.json` (port 6302 from the generated playground catalog); dev server started.

- 06:10 ✅ oracles (`oracles.md`): 32 python entry points agree; regenerated stale `export-bim-1-ifc-energy` measure tables. three.js solids oracles wait for the Rust subject.
- 06:20 ✅ pyroute (`pyroute.md`): `repo test-file` now runs `🐍️.py` oracles via the locked python; 36/36 bim pass. Stale nested-launches expectation → Haiku fix.
- 06:20 Dev server: descriptor emitter stale → rebuilding (`describe` crate) before the component build.

## Wave 1 (running)
| Agent | Model | Scope | Report |
|---|---|---|---|
| model | Sonnet | `🗿️artifacts/🏢️model` Rust workspace: check + test | `model-rust.md` |
| bridge-hub | Sonnet | `🏭️bridge`, hub compositions `🏙️bim` + stdio `🏠️bim` | `bridge-hub-rust.md` |
| typescript | Sonnet | bim TS, dev variant, hub guest tests, python oracles | `typescript.md` |
| audit | Haiku | s catalog/descriptor/launch state, prior bim tickets, test-twin gaps | `audit.md` |

## Queued
- Descriptor regeneration + s catalog regeneration once Rust is green.
- Browser end to end of the bim dev variant (load model, edit, view, export).
- Fill test-twin gaps found by the audit.
