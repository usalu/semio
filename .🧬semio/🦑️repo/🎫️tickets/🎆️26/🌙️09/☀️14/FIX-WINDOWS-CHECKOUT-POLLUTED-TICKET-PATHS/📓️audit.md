# Windows Checkout — Polluted Ticket Paths

## Symptom

`git checkout` on Windows fails with `error: invalid path` for entries under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/…` whose month segment contains literal `<`, `>`, `|` (for example `<|control37|>09` instead of `🌙️09`).

## Root cause

Accidental ticket folder names and nested paths from path-repair / Windows-reserved-name experiments:

- Literal `<|control37|>` in directory names (invalid on Windows).
- U+FFFD-prefixed month folders (`09`).
- Absolute-path segments pasted into the tree (`…/Users/ueli/Documents/semio/…`).
- Doubled ticket path segments under `PUZZLE-3D-END-TO-END`.

## Remediation (2026-09-14)

Removed **11** tracked paths via `git rm` (7 with `<>|` — the Windows blockers). Deleted matching on-disk junk under `🎆️26/`.

Repaired ticket generator scripts that still contained corrupted `📤️export` literals:

- `…/STDIO-ARTIFACTS-AND-IO/generators/w4a_finish_zip.py`
- `…/STDIO-ARTIFACTS-AND-IO/generators/w4a_force_deflate_zip.py`
- `…/ARTIFACT-SCHEMA-FACETS/🧪wave5-trinity-write-jack-leaves.py`

Duplicate content (e.g. `📓️2026-09-10-wave-AA-undo-history.md`) already exists under the canonical `🌙️09/☀️02/PUZZLE-3D-END-TO-END/` tree.

## After merge

On Windows: `git pull` then `git checkout` should succeed once this commit is on the branch.

## Prevention

- Never create ticket month folders by string-replacing emoji; use repo MCP `ticket_open` (`🌙️MM` segments).
- Do not commit paths containing `< > | : * ? "` or U+FFFD.
- Optional: CI scan `git ls-files` for Windows-forbidden characters in paths.
