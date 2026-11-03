# R2 Cleanup Plan (agent ct, inventory only, nothing deleted)

Snapshot taken 2026-11-01 while about 20 agents were still working. Tool folders and files appeared and vanished during the walk (for example `🧰️xp` was gone at the final check, `🧰️rl` and `🧰️pp/temps-kettle.txt` appeared late). The coordinator must re-run the path check in this file before running `cleanup.sh`.

Run order at close: `bash cleanup.sh` (it does `cd /c/git/semio` and then `rm -rf` each quoted path). It is written but NOT run.

## 1. Delete (cleanup.sh)

Sizes are bytes unless noted.

### 1a. Whole generated folder
| Path | Size | Files |
|---|---|---|
| `🗑️generated/` | about 256 MB (du 261888 KB at last check) | about 14 800 (incl. about 165 PNG, 341 MP4, 14 150 SVG, all staffage cache and renders). Biggest children: `sm` 193 MB, `r2` 30 MB, `rs` 28 MB. Includes `🗑️generated/ct/` (my own inventory outputs and `status-all.txt`). |

### 1b. Loose outputs in ticket root
| Path | Size | Why |
|---|---|---|
| `r2-kp-dec-scene.png` | 544 488 | Render image, tool output (AGENTS: no images in ticket). See ambiguity A3. |
| `r2-kp-kettle-frames.png` | 173 219 | Render image, tool output. See ambiguity A3. |

### 1c. Tool folder outputs (scripts in the same folders are kept)
| Path | Size | Why |
|---|---|---|
| `🧰️au/run.log` | 1 889 | log |
| `🧰️au/scan.txt` | 60 233 | piped scan output, no reader |
| `🧰️cp/node_modules/` | about 1 KB (vitest cache, `.vite`, `.vite-temp`) | tool cache |
| `🧰️cp3/__pycache__/` | 15 547 | bytecode cache |
| `🧰️ds/dead1.tsv` | 16 537 | scan output |
| `🧰️ds/dups_diff.txt` | 1 912 | scan output |
| `🧰️ds/list.txt` | 8 699 | file list output, no reader |
| `🧰️ds/scan1.txt` | 30 040 | scan output |
| `🧰️ds/ts.txt` | 3 642 | file list output, no reader |
| `🧰️ds/ts_fn_dups.txt` | 5 017 | scan output |
| `🧰️ds/ts_scan.txt` | 6 026 | scan output |
| `🧰️ha3/🗑️generated_bak_city-car.json` | 132 814 | backup copy named after generated, AGENTS says delete (audit-paths-r2.md line 55 agrees) |
| `🧰️ha3/🗑️generated_bak_vintage-car-1929.json` | 173 579 | same as above |
| `🧰️hc/touched-py.txt` | 5 220 | piped list output |
| `🧰️pp/dump.txt` | 182 741 | dump output |
| `🧰️pp/dump2.txt` | 99 766 | dump output |
| `🧰️pp/temps-kettle.txt` | 4 200 | output, no reader |
| `🧰️qa/new.txt` | 1 351 | piped list output, no reader |
| `🧰️rl/` (whole folder: `cast-usage.txt` 0 B, `cast-usage.txt.ids` 2 786 B) | 2 786 | outputs only, no scripts in folder |
| `🧰️sm/files.txt` | 7 370 | piped list output, no reader |
| `🧰️tp/folders.json` | 169 | cache-path map written by `🧰️bj`/`🧰️bjv`/`🧰️bp` scripts, no reader in `🧰️tp` |
| `🧰️ed/` (empty directory) | 0 | empty leftover |

Total of 1b and 1c (excluding 1a): about 1.5 MB.

## 2. Keep (inputs, configs, reports, scripts)

- Every `*.md` in the ticket root (about 160 reports and audits, including `fleet-brief-r2.md`, `contract.md`, `summary.md`, `audit-*`, `r2-*-report.md`, `r2-changed-files.md`, `r2-open-items.md`, `r2-coordinator-rulings.md`) and every `*.md` in `📌️important/`.
- Every script: `*.py`, `*.sh`, `*.mjs`, `*.ts`, `*.tsx`, `*.cjs`, `*.ps1`, `*.feature` in the root and in every `🧰️*` folder, including my own `🧰️ct/inventory.sh` and the `🧰️ct/🔎️*.py` scripts.
- Cast, manifest and config JSON used as render or test input: `cast-smoke.json`, `bridge-py-manifest.json`, `f2_hrv_manifest.json`, `f2_shower_manifest.json`, `f2_split_manifest.json`, `🧰️a5/casts.json`, `🧰️a5/casts-humans.json`, `🧰️a5/probe.json`, `🧰️a10/clips.json`, `🧰️a11/casts-ac.json`, `🧰️a11/casts-base.json`, `🧰️a11/casts-front.json`, `🧰️a11/casts-nat.json`, `🧰️a11/casts-win.json`, `🧰️sc/casts.json`, `🧰️g5/b.json`, `🧰️ty/tsconfig.lib.json`, `🧰️ty/tsconfig.react.json`.
- `🎫️ticket.json` (ticket metadata).
- `🧰️ub/index.html` is the Vite entry for `🧰️ub/entry.ts` and `vite.config.ts`, so it is an input, not an output.
- `🧰️bc/map.tsv` is read by `🧰️bc/apply_map.py`, `perfile.py` and `where.py`, so it is an input.
- `📌️important/📝️.md` is a 0-byte file (keep, see ambiguity A5).

## 3. Ambiguous (NOT in cleanup.sh, the coordinator decides)

- A1. `🧰️ex/results.json` (16 223 B): a results file, but read by `🧰️ex/summary.py`. Delete if the summary is not needed.
- A2. `🧰️fr/casts.txt` (666 B): read by `🧰️fr/sum.py` as a WATCH list. Looks like input.
- A3. `r2-kp-report.md` lines 10 and 16 link to `T/r2-kp-kettle-frames.png` and `T/r2-kp-dec-scene.png`. Deleting the PNGs leaves two dead links in that report. Per AGENTS the PNGs go, so the report link should be fixed or the report should say the frames were removed. `audit-paths-r2.md` line 52 already lists them for removal.
- A4. `🧰️ht/modules.txt` (4 357 B, read by `🧰️ht/run_all.sh`), `🧰️pf/thermal_orig_block.txt` (1 757 B, read by `🧰️pf/make_orig.py`), `🧰️qa/ids.txt` (652 B, read by `🧰️qa/sheets.py`), `🧰️hs/files.txt` (5 527 B, read by `🧰️hs/audit_py.py`). Each is a data file that a script reads. Regenerable, but I left them out. Delete them if the scripts will not be rerun.
- A5. `📌️important/📝️.md` (0 bytes) and the empty `🧰️cp/node_modules/.vite-temp` (inside the node_modules delete, so no action).
- A6. `🧰️tp/folders.json` is in the delete list, but it has no reader in `🧰️tp`. If someone expects a `tp` rerun to read it, drop it from the list.
- A7. `🧰️rl/` was new in the last snapshot and has no scripts. Its owner may still be writing to it, so re-check before running.
- A8. `🎓️teaching/🏛️architecture/…/__pycache__/*.pyc` (about 15 dirs, gitignored, plus `.venv` under `⚡️energy/🏗️authoring/📦️packages/🐍️python/.venv`): Python bytecode and a virtualenv outside the ticket. Not ticket output, not deleted. Do not touch `.venv`.

## 4. Stray outputs outside the ticket (read-only check)

- `git status --porcelain -uall` (with `core.quotepath=false`) lists 1 099 untracked entries under `🎓️teaching/🏛️architecture/`. Excluding the ticket, 96 are source work: catalog `🔣️.json`, schemas, staffage fixtures and tests, `.tsx` and `.feature` files. These are inputs. Keep.
- No untracked `*.log`, `*.png`, `*.mp4`, `*.jsonl`, `*.webm`, `*.gif`, `*.svg` (outside T) and no `x` or `.🧬semio` folder under `🎓️teaching/🏛️architecture/`. No `🗑️generated` folder outside the ticket.
- Untracked-but-not-ignored files repo-wide are all source (see above). Gitignored caches at the repo root (`.cache`, `.nx`, `.repo`, `.venv`) contain `*.log` and `*.jsonl`. They are not this ticket's outputs, so out of scope. Not exhaustively checked.
- Tracked modifications under `🎓️teaching/` (the `M` and `D` entries in the session git status) were not checked against outputs because they are tracked source changes.

## 5. Verification

- `bash -n cleanup.sh` passes.
- Every `rm -rf` path exists at snapshot time (checked read-only). `🧰️xp/py.log` and `🧰️xp/ts.log` were removed from the list because `🧰️xp` no longer existed.
- Inventory script: `T/🧰️ct/inventory.sh`. Its outputs went to `T/🗑️generated/ct/`, which is deleted by cleanup step 1a.
