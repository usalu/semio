# 🧭 R3 registry report (W19, id `reg`)

Date: 2026-11-03. Scope: registry consistency, OI-125, OI-102, OI-127. W08 to W11 assets were not touched.

## Phase 1: registry consistency

Index: `S/🗂️catalog/🔣️.json` (`semio.staffage/v2`, 9 categories, 5 collections, 149 entries). It is the one index the library browser and the catalog module use: `🔨️modules/🗂️catalog/📥️files/🟦️.ts` and `🎯️targets/⚛️react/🔨️modules/📚️catalog/🟦️.ts`. There is no second hardcoded asset list.

Checker: `T/🧰️r3-reg/check_registry.py` (read-only). Output: `T/🗑️generated/r3-reg/check.json`.

| Check | Result |
|---|---|
| Asset folders under `🗂️catalog/<category>/` | 149 |
| Asset folders with `🔣️.json` | 149 |
| Index entries | 149 |
| Folder without index entry / index entry without folder | 0 / 0 |
| Manifest `asset.id` equals folder name | 149 of 149 |
| Manifest category maps to the index category folder | 149 of 149 |
| Index title has non-empty `en` and `de` | 149 of 149 |
| Manifest `asset.title` has non-empty `en` and `de` | 149 of 149 |
| Every pose and clip title has `en` and `de` | all, 0 missing |
| Index `poses` and `clips` present in the manifest | all, 0 missing |
| Duplicate ids in the index | 0 |

Cast manifest: `V/🧍️staffage/🔣️.json` (`version` 1, 94 casts, 30 `shared-*`).

| Check | Result |
|---|---|
| Cast `asset` exists in the catalog | 94 of 94 |
| Cast `pose` exists in the asset `poses[].id` | all casts that have one |
| Cast `clip` exists in the asset `clips[].id` | all casts that have one |
| Cast `view` in the enum `dark-line, dark-solid, light-line, light-solid, thermal, forces, moments` | 94 of 94 |
| Raw duplicate keys in the cast JSON (checked with `object_pairs_hook`) | 0 |
| `shared-*` ids with identical content under different ids | 0 |
| Any cast with identical content under different ids | 0 |

Not checked in this pass: the cast `environment` ids against `🌤️environments/🔣️.json`. The task named asset, pose and view only.

Mechanical issues found: none. Phase 2 made no registry edits.

## Phase 2: small items

### OI-125 (oracle docstrings): already resolved, no edit
- Every oracle docstring under `S/🧪️tests` reads `uv run --project 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python --group test python -I 🐍️.py [--write]`. Checked with grep over `🎓️teaching/🏛️architecture` for `uv run` and `--with` (excluding `.venv` and `node_modules`).
- Non-mechanical note: `S/🧪️tests/🧰️support/🧰️.ts:59` runs `tool("uv", ["run", "--quiet", ...packages.flatMap((name) => ["--with", name]), "python", ...])`. This is runner code, not a docstring. I did not change it. It is a runtime dependency decision for Sonnet or Opus to review.
- A broad whole-repo grep for `uv run` timed out in the background and was stopped. It produced no output, and the targeted greps above were sufficient.

### OI-102 (cache key folders): listed, not deleted
The cache is live. Another render is writing lock files, so the count changed during the session: 423 entries at one point, 418 and 429 key folders on later scans. The list below is the 2026-11-03 snapshot of `T/🗑️generated/r3-reg/cache_scan.json` (scanner: `T/🧰️r3-reg/cache_scan.py`). Incomplete means the folder has no cast subfolder, an empty cast subfolder, or `.tmp` leftovers.

- 29 incomplete key folders, all with no cast subfolder (only `.lock-<cast>` files):
  `02aeafe5…`, `1939bfd6…`, `356d01af…`, `3646a46c…`, `3dc3ea70…`, `501ad23f…`, `57274c7c…`, `5ef22aa6…`, `67eee135…`, `6cc42766…`, `8656e6a3…`, `865db9ee…`, `882f9858…`, `8a278837…`, `8bbbfbe4…`, `8db7edd9…`, `8ee71105…`, `a7d2ddf5…`, `aad31b09…`, `ba2ef70c…`, `c1400be1…`, `c3962585…`, `c65d30e8…`, `c92bcde4…`, `cdd59a26…`, `e6c7d152…`, `f6cad216…`, `feb57c15…` (32-character keys, shown here as their first eight characters)
- Plus `.tmp-1dfdd3a50008`, a temporary folder that is empty.
- Earlier, `.tmp-becad1f9553c` held only an empty `out` folder. It was gone by the second scan.
- The OI-102 baseline said 3. The current number is higher because the render is still running.
- Nothing was deleted. The baseline says `ensure_casts` self-heals these folders.

### OI-127 (report drift): correction notes appended
- `T/r2-g5-report.md`: appended "Correction note (OI-127, r3-reg, 2026-11-03)". The report does not name the removed cast ids, so no id is corrected in that note.
- `T/r2-g6-report.md`: appended the same kind of note. Current counts in `V/🧪️tests/🔣️staffage_only.json`: `rule_b.allowed` = 8 (report says 12), `rule_a.allowed` = 18 (report says 15).
- Both reports keep their original text. Only the notes were appended, with Edit.

## Not done (owned by others)
- OI-91 and OI-135 belong to W18, per the coordinator's later message. I did not run them.

## Files changed
- `T/r2-g5-report.md` (appended note)
- `T/r2-g6-report.md` (appended note)
- `T/r3-reg-report.md` (new, this file)
- `T/🧰️r3-reg/check_registry.py`, `T/🧰️r3-reg/cache_scan.py` (new, read-only scripts)
- Output: `T/🗑️generated/r3-reg/check.json`, `T/🗑️generated/r3-reg/cache_scan.json` (delete when the ticket closes)

No asset, index, cast manifest, or cache folder was modified.

## Open for the coordinator
1. The OI-127 counts in the coordinator's open-items table (`15 vs 14`, `12 vs 8`) do not match the current guard JSON (`18` and `8`). Someone should confirm which numbers are right.
2. `S/🧪️tests/🧰️support/🧰️.ts:59` uses `uv run --with` at runtime. Sonnet or Opus to decide whether this is in scope for OI-125.
3. The OI-102 list changes while the render runs. Re-scan after the render finishes before acting on it.
