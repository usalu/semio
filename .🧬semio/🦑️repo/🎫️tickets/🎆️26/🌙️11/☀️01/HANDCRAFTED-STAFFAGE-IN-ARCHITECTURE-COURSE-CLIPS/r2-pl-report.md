# 📏️ Round 2 Path Length Audit (pl)

Read-only audit. Nothing renamed. Script: `🧰️pl/measure_paths.py` (run with `uv run --project 🎓️teaching python`).

## Method
- Full path = `C:\git\semio\` (13 units) + repo-relative path.
- Length = UTF-16 code units (`len(s.encode("utf-16-le")) // 2`), so each emoji counts 2 and each variation selector (U+FE0F) counts 1.
- Limit: 256 is a violation (AGENTS.md, Windows MAX_PATH). 240 is the warning threshold.
- Scopes: every path listed in `r2-changed-files.md`, every file under the ticket folder T (including `🗑️generated`, `__pycache__`, `node_modules` caches), every file under S (`🧍️staffage`) and V (`🎨️visuals`).

## Counts
| Scope | Files | > 240 | > 256 (violations) | Max units |
|---|---|---|---|---|
| Listed in `r2-changed-files.md` | 295 (12 deleted, not on disk) | 0 | 0 | 123 |
| Ticket T (all files) | 15679 | 0 | 0 | 226 |
| Staffage S | 517 | 0 | 0 | 177 |
| Visuals V | 33 | 0 | 0 | 129 |
| Unique total | 16310 | 0 | 0 | 226 |

Result: no path exceeds 240 or 256 units. No violations.

## Worst 15 (all scopes)
All 15 are under T `🗑️generated/r2/media/...` (render output of the r2 render agent, 480p15 partial movie files and uncached chunks). They are generated and will be removed with that folder, so they are not a violation.

| Units | Path (relative to repo root) |
|---|---|
| 226 | T/🗑️generated/r2/media/videos/🐍️/480p15/partial_movie_files/Beat6_GrenzenDerFreienLueftung/partial_movie_file_list.txt |
| 218 | T/🗑️generated/r2/media/videos/🐍️/480p15/partial_movie_files/Beat3_Anlagenvergleich/partial_movie_file_list.txt |
| 217 | T/🗑️generated/r2/media/videos/🐍️/480p15/partial_movie_files/Beat2_Luftwechselrate/partial_movie_file_list.txt |
| 217 | T/🗑️generated/r2/media/videos/🐍️/480p15/partial_movie_files/Beat6_GrenzenDerFreienLueftung/uncached_00000.mp4 |
| 217 | T/🗑️generated/r2/media/videos/🐍️/480p15/partial_movie_files/Beat6_GrenzenDerFreienLueftung/uncached_00001.mp4 |
| 217 | T/🗑️generated/r2/media/videos/🐍️/480p15/partial_movie_files/Beat6_GrenzenDerFreienLueftung/uncached_00002.mp4 |
| 217 | T/🗑️generated/r2/media/videos/🐍️/480p15/partial_movie_files/Beat6_GrenzenDerFreienLueftung/uncached_00003.mp4 |
| 217 | T/🗑️generated/r2/media/videos/🐍️/480p15/partial_movie_files/Beat6_GrenzenDerFreienLueftung/uncached_00004.mp4 |
| 217 | T/🗑️generated/r2/media/videos/🐍️/480p15/partial_movie_files/Beat6_GrenzenDerFreienLueftung/uncached_00005.mp4 |
| 217 | T/🗑️generated/r2/media/videos/🐍️/480p15/partial_movie_files/Beat6_GrenzenDerFreienLueftung/uncached_00006.mp4 |
| 217 | T/🗑️generated/r2/media/videos/🐍️/480p15/partial_movie_files/Beat6_GrenzenDerFreienLueftung/uncached_00007.mp4 |
| 217 | T/🗑️generated/r2/media/videos/🐍️/480p15/partial_movie_files/Beat6_GrenzenDerFreienLueftung/uncached_00008.mp4 |
| 217 | T/🗑️generated/r2/media/videos/🐍️/480p15/partial_movie_files/Beat6_GrenzenDerFreienLueftung/uncached_00009.mp4 |
| 217 | T/🗑️generated/r2/media/videos/🐍️/480p15/partial_movie_files/Beat6_GrenzenDerFreienLueftung/uncached_00010.mp4 |
| 217 | T/🗑️generated/r2/media/videos/🐍️/480p15/partial_movie_files/Beat6_GrenzenDerFreienLueftung/uncached_00011.mp4 |

Worst 15 outside `generated` and `node_modules` (source-like files, tracked or authored):
| Units | Path |
|---|---|
| 149 | T/🧰️cp3/__pycache__/driver.cpython-312.pyc |
| 140 | T/🧰️nn/audit_narration_numbers.py |
| 139 | T/q2-vitest-react-alias.config.ts |
| 139 | T/🧰️scripts/ha2/check-touched.sh |
| 138 | T/🧰️cp2/edit_pot_1l_geometry.py |
| 138 | T/🧰️pa/oi33b_pumpjack_height.py |
| 138 | T/audit-narration-numbers-r2.md |
| 137 | T/🧰️cp2/edit_pot_1l_thermal.py |
| 136 | T/🧰️pf/thermal_orig_block.txt |
| 135 | T/a2/build-person-cooking.mjs |
| 135 | T/a2/build-standing-adult.mjs |
| 135 | T/🧰️a11/schema_followtip.cjs |
| 135 | T/🧰️pf/same_modulo_digits.py |
| 134 | T/🧰️cp/edit_pot_geometry.py |
| 133 | T/audit-authoring-digest.md |

Note: the 340 paths at or above 200 units are all under `generated` (T) or `node_modules` (S/T caches). Staffage (S) worst is 177 units, in `node_modules/.vite/vitest`. None of the tracked staffage or visuals paths exceeds 140 units.

## Open
- The 12 deleted paths from `r2-changed-files.md` are not on disk, so their length is measured from the list only.
- The render output under `T/🗑️generated/r2` will push the ticket folder close to 226 units. Clearing that folder removes the worst entries.
