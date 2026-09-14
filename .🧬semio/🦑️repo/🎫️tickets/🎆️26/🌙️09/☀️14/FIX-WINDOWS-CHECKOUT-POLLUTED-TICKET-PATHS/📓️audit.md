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

## Path length (MAX_PATH)

Windows hits **~260 characters** on the full path (`C:\…\semio\` + relative path).

### Draw-source test retention

Retained runs under `END-TO-END-TAXONOMY-NORMALIZATION/📓️draw-source-scenarios/🧪️runs/` nested a full fake ticket path (`.🧬semio/🦑️repo/🎫️tickets/…/DRAW-SOURCE-SCENARIOS/📓️submitted-plans/`) inside each `🧪️s-test-*` folder — often **400+** characters. That matches `Filename too long` during checkout or `git status`.

**Repo fixes:** gitignore `**/🧪️runs/`, `**/📓️progress/🧪️s-test-*/`, `**/semio-normalization/`; removed committed progress stubs; shortened fixture `transactionTicketSegments` to `["🎫️ticket"]`.

**Local cleanup before checkout** (PowerShell, from repo root):

```powershell
git config --global core.longpaths true
$base = "\\?\$((Get-Location).Path)"
$junk = Join-Path $base ".🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️08\☀️17\END-TO-END-TAXONOMY-NORMALIZATION\📓️draw-source-scenarios"
if (Test-Path -LiteralPath $junk) { Remove-Item -LiteralPath $junk -Recurse -Force }
```

Two din16798 mutation snapshot paths are still ~261 chars relative to repo root; long paths or a short clone root (e.g. `C:\s\semio`) avoids MAX_PATH on those alone.

## After merge

On Windows: `git config core.longpaths true`, delete any local `📓️draw-source-scenarios` junk as above, then `git pull` / `git checkout`.

## Prevention

- Never create ticket month folders by string-replacing emoji; use repo MCP `ticket_open` (`🌙️MM` segments).
- Do not commit paths containing `< > | : * ? "` or U+FFFD.
- Do not commit `🧪️runs` / `🧪️s-test-*` evidence under real ticket trees (use tmp + gitignore).
- Optional: CI scan `git ls-files` for Windows-forbidden characters and path length budgets.
