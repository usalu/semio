# Restore Engine After Comment Stripping

## Cause

Repo-wide comment removal stripped `#` from lines that were **not** comments (e.g. dict literals, function calls), and deleted large spans of `compose/engine/main.py`.

## Fix (git show, no checkout)

- `compose/engine/main.py` ← `80bfd6772` (last good before commit `c522b2737` mass deletion)
- `compose/py/main.py` ← `41b248ef4` (commit `80bfd6772` left orphaned `[repo://...]` lines without `#`)
- `coda/engine/coda.py` ← `41b248ef4` (same class of corruption)

## Verification

- `python3 -m py_compile` on the three files
- `pytest compose/engine/engine.test.py` — 174 passed
