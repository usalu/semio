# conc-ts tools (wave 1020e)

- `staged/🧩️units/` : the module to create under `L/🏗️build/🟦️typescript/🧩️units/` (engine, rules document, schema, tests, python oracle, fixture).
- `apply.py` : dry run by default; `--apply` creates the 6 files (create-only, rollback on error); `--undo`.
- `gen-fixture.py` : regenerates `staged/🧩️units/🧪️tests/🔣️.json` from the rules document + hand-written cases.
- `prep-engine.py` : copies the staged engine to `/private/tmp/conc-ts/engine` (validator import made absolute) for scratch runs.
- `mkoverlay.py <listed.txt> <family> <dir>` : frozen overlay of a program (files from `tsc --listFilesOnly`, node_modules rebuilt as symlink dirs, workspace packages -> overlay copies).
- `units-cli.ts plan|check <workspace> <family> --out <dir> [--session s] [--concurrency n] [--only unit,unit]` : scratch driver (bun).
- `mono-run.sh <tsconfig> <log> [cwd]` : one monolithic tsc, 3 GB heap, `/usr/bin/time -l`.
- `analyze_units.py`, `refine_experiment.py` : regex-based graph experiments on `git ls-files` (partition policy numbers).
Scratch: `/private/tmp/conc-ts/` only (overlays, stores, logs); nothing generated inside the ticket folder.

## Wave 1020e second round (groups 2 and 3)
- `staged2/fixes.py` + `staged2/fixes.json` (authored by `gen-fixes.py` from the live tree): group 2, anchored source fixes that make exported declarations nameable. `--root DIR` runs the same operations on an overlay; dry run default; idempotent; `--undo` restores from `C/🗑️generated/conc-ts/fixes-backup`.
- `staged3/apply.py` + `staged3/new/**` + `staged3/edits.json` (authored by `gen-group3.py`): group 3, the `typecheck units` wiring (needs group 1). `--root DIR` supported.
- `setup-overlay.sh <name> <family>`: frozen overlay `/private/tmp/conc-ts/ov-<name>` + `mono.json` + engine copy. Never `cp -r` an overlay (node_modules symlinks are followed: 21 GB).
