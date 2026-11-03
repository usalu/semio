# 🧭 Path, Naming and Stray Audit (round 2, agent pl)

Scope: every new, modified or deleted path from `git status --porcelain -z --untracked-files=all` over `🎓️teaching/🏛️architecture` and this ticket folder. Read-only: nothing was moved or deleted.

Inventory: 1017 entries = 870 untracked, 135 modified, 12 deleted. 782 are ticket-folder files, 235 are source-side (88 untracked, 135 modified, 12 deleted).

## 1. Path length (AGENTS.md: no path over 256)

Method: `C:\git\semio\` (13 chars) plus the repo-relative path with `/` turned into `\`. Measured in UTF-16 code units (how Windows counts) and in code points. Emoji count as 2 UTF-16 units. Script: `🧰️pl/audit_paths.py`, run as a file with `core.quotepath=false` status input.

| Metric | Value |
|---|---|
| Longest path, all 1017 entries | 153 UTF-16 units (146 chars): `.🧬semio/.../🧰️ha3/🗑️generated_bak_vintage-car-1929.json` |
| Longest source-side path | 123 UTF-16 units (116 chars): deleted `…/🏃️activity/🧬️schema/🔣️.schema.json` |
| Longest new (untracked) source path | 114 UTF-16 units: `…/🗂️catalog/🔌️appliances/thermostatic-radiator-valve/🔣️.json` |
| Paths over 256 | **0** |
| Paths over 240 (at-risk) | **0** |

Result: no path is at risk. The deepest real margin is about 100 units. Note that `.🧬semio/…/🗑️generated` holds about 19.7k files from other agents; they are well under the limit but the folder must be cleaned at ticket close.

## 2. Naming and taxonomy of the new staffage assets, tests and modules

Sibling pattern checked against tracked files: emoji-prefixed category folders, `🔣️.json` for catalog data, `🟦️.ts` / `🟦️.tsx` for TS modules and tests, `🐍️.py` for Python, `🥒️.feature` for specs, `🧬️schema/` for schemas, and `🧪️tests/<emoji-name>/` for test cases. Tracked sibling test folders such as `⏱️clock-determinism` are emoji-prefixed too.

Conforming (no deviation):
- Catalog: 51 new `🔣️.json` assets under `🧍️staffage/🗂️catalog/<category>/<slug>/`. The leaf slugs are ASCII, matching the tracked catalog siblings (`🌳️vegetation/agave/🔣️.json`), so they are the sibling convention.
- Staffage tests: `🧍️staffage/🧪️tests/<emoji>-<slug>/` with `🟦️.ts`, `🥒️.feature`, `🐍️.py` (for example `🌡️thermal-span`, `🫕️phase-change`, `🚀️export-performance`). Consistent.
- Staffage fixtures `🧍️staffage/🧫️fixtures/<emoji>-<slug>/🔣️.json`. Consistent.
- Export module `🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts` and its `🧬️schema/🔣️.schema.json` and `🧬️schema/🟦️.ts`. Consistent.
- React target `🎯️targets/⚛️react/🔨️modules/🎛️viewer/📏️range/🟦️.tsx`, and tests `🧪️tests/📏️range-span/🟦️.tsx` and `🥒️.feature`. Consistent.

Deviations:

| # | Path (repo-relative, abbreviated) | Deviation | Suggested fix |
|---|---|---|---|
| D1 | `🎓️teaching/🏛️architecture/.🧬semio/x` (0 bytes) and `.🧬semio/🦑️repo/` beneath it | A dot-folder `.🧬semio` and a stray empty file `x` were created inside the architecture source tree. This looks like a misdirected write. It is not a valid taxonomy folder. | Remove both (owner decision; I did not delete). |
| D2 | `…/🎨️visuals/🧪️tests/🐍️icons.py`, `🐍️staffage_only.py`, `🔣️icons.json`, `🔣️staffage_only.json` | Stem is a bare word after the emoji (`🐍️icons.py`), not the bare `<emoji>.<ext>` form. The deleted sibling pair `🧪️tests/🐍️silhouettes.py` and `🔣️silhouettes.json` used this same shape, so it follows the old sibling convention rather than introducing a new one. | Optional: rename to `🐍️.py` / `🔣️.json` per test folder, or accept as the visuals convention. |
| D3 | `…/🏗️bim/🎬️clip/🧱️model/🧪️tests/🧭️cutter-winding.py` and `.json` | Same as D2, emoji plus slug stem, and the test lives directly in `🧪️tests/` rather than in a `🧪️tests/<emoji-name>/` folder. | Move into `🧪️tests/🧭️cutter-winding/🐍️.py` and `🔣️.json`, or accept. |
| D4 | `…/📊️excel/🎬️clip/🧪️cases/🧍️occupants/🔣️.json` | Uses `🧪️cases/` instead of `🧪️tests/`. The sibling `🧪️tests/🧍️occupants/🐍️.py` sits beside it. | Rename `🧪️cases` to `🧪️tests` or move the case data under `🧪️tests/🧍️occupants/`. |
| D5 | `…/🧍️staffage/🧪️tests/🫕️phase-change/🐍️.py` etc. | Conforming, listed for completeness. | None. |

No ASCII-only folder appears in any new staffage path other than the catalog and test slugs listed above, which match the sibling convention.

## 3. Stray tool-output files (txt, log, png, jsonl, out)

Source folders (tracked or untracked, under `🎓️teaching/🏛️architecture`):
- No stray tool output in the status list. The only tracked `.png` files (`🏙️typology/📝️assignment/*.png`) are legitimate assets.
- Ignored `.venv` and `…/🐍️python/.🧬semio/🎓️teaching/⚡️cache/` contain third-party `.txt` and `.png` (site-packages). They are environment content, not ticket output, and are outside git.
- The `x` file in D1 is a stray artifact.

Ticket folder, status-listed root and `🧰️` scratch (all untracked, all tool output; delete at ticket close):
- Root: `r2-kp-dec-scene.png`, `r2-kp-kettle-frames.png`.
- `🧰️pf/thermal_orig_block.txt`, `🧰️ds/ts_fn_dups.txt`, `🧰️ds/dups_diff.txt`, `🧰️ds/ts_scan.txt`, `🧰️ds/scan1.txt`, `🧰️ds/list.txt`, `🧰️ds/ts.txt`, `🧰️hc/touched-py.txt`, `🧰️ht/modules.txt`, `🧰️fr/casts.txt`, `🧰️hs/files.txt`, `🧰️sm/files.txt`, `🧰️au/scan.txt`.

Also in the ticket folder, not a stray extension but generated-looking: `🧰️ha3/🗑️generated_bak_vintage-car-1929.json`. It is a backup copy named after the generated folder and should be removed.

The `🗑️generated/` tree holds about 19.7k files, including `.txt` and `.png` renders such as `av/*.png` and `au/*.txt`. Per AGENTS.md these must be removed when the ticket closes. I did not delete them.

## Out of scope but noticed (source-side deletions in this ticket's diff)

- `…/🏗️authoring/🎨️visuals/🫟️silhouettes/README.md` and `…/🧪️tests/🐍️silhouettes.py`, `🔣️silhouettes.json`, `🫟️silhouettes/🔣️.json`, `🧬️schema/🔣️.schema.json` are deleted. Not a naming issue, but confirm the deletion is intended.
- `…/🏃️activity/` (internal gains) lost its `🖼️.svg` files, `🔣️.json`, `🧪️tests/` and `🧬️schema/`.

## Commands

- Status: `git -c core.quotepath=false status --porcelain -z --untracked-files=all -- "🎓️teaching/🏛️architecture" "<T>"`, written to `🗑️generated/pl/status.bin`.
- Length and naming audit: `🧰️pl/audit_paths.py`. Source listing and extension shapes: `🧰️pl/src_list.py`. Longest paths: `🧰️pl/longest.py`.
- Generated outputs in `🗑️generated/pl/` are removed at the end of this audit.
