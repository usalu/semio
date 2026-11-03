# R2 continuous regression (agent cr)

## Round 1, 2026-11-01 (started 14:51, finished about 16:20)

Runner: `🧰️cr/round.sh 1`. Raw logs and counts: `🗑️generated/cr/round-1/` (`counts.tsv`, `vitest-status.txt`, `logs/`, `catalog.log`). Scope: (a) visuals tests list lines 6-15, (b) staffage package tests line 5, (c) vitest library and React, (d) full catalog check, (e) remaining energy modules, lines 1-4, 16-26, 28-32. Modules 27, 33, 34, 35 skipped (no tests).

### Counts

Visuals (a), list lines 6 to 15 (rc, ran, result):
- 6 `🎨️semio-style`: rc 5, 0 ran, setUpClass error (see item 1).
- 7 visuals base: rc 0, 7 OK.
- 8 `icons`: rc 1, 8 ran, 7 errors.
- 9 `integration`: rc 1, 9 ran, 4 errors.
- 10 `scenes`: rc 0, 11 OK.
- 11 `section_consumers`: rc 1, 9 ran, 1 error.
- 12 `sections`: rc 0, 9 OK.
- 13 `staffage_only` guard: rc 0, 20 OK.
- 14 `transforms`: rc 0, 2 OK.
- 15 `readout`: rc 0, 5 OK.
Total (a): 10 modules, 5 green, 5 red.

(b) staffage package `🧍️staffage/🧪️tests` (line 5): rc 1, 36 ran, 3 errors.

(c) vitest, ticket-local alias configs:
- Library: 2 of 53 files fail (both at load, item 11), 51 pass; 1001 tests pass, 10 skipped, 1011 total.
- React: 1 of 17 files fails (item 12), 16 pass; 106 pass, 1 skipped, 108 total.

(d) full catalog check: rc 1, `✗ manifest`, 147 assets all `✗` (item 13).

(e) remaining energy modules, 20 modules (lines 1-4, 16-26, 28-32): 14 green, 6 red (lines 1, 3, 17, 18, 23, 28). Line 3 (`🌬️ventilation/📦️volume`) exits rc 1 with no `Ran` line, import-time failure, not yet traced.

### Failures with traceback head and owner

1. Visuals `a06` `🎨️semio-style` (setUpClass): `SyntaxError: unterminated string literal (detected at line 312)` in `🏗️authoring/🔤️fonts/🐍️.py:312`, imported from `🎨️semio-style/🐍️.py:37`. Owner: fonts module (`🔤️fonts`), a syntax break in the shared fonts code. Blocks the 9 style tests and also `a08`.
2. Visuals `a08` `🧪️tests/🐍️icons`: the same `fonts/🐍️.py:312` SyntaxError (via `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:22`), plus `watt_anchor` in `🎨️visuals/🐍️.py:276` for `kind='laptop'`. Owner: fonts (syntax), then visuals `watt_anchor`.
3. Visuals `a09` `integration`: `KeyError: "Unknown cast 'demand-certificate-radiator'"` from `🧍️staffage/🐍️.py:93` (`_entry`) via `ensure_casts:266`. The cast id is not in `🧍️staffage/🔣️.json`. Owner: staffage manifest and the `📊️demand` `🏷️certificate` chapter (cast was not added to the manifest).
4. Visuals `a11` `section_consumers`: `NameError: name '_Path' is not defined` in `📊️demand/🎬️clip/🧪️practice/🐍️.py:84`. Owner: demand practice clip (`🧪️practice`, missing import).
5. Staffage package `b05` (`🧍️staffage/🧪️tests`): `RuntimeError: Staffage export failed` from `ensure_casts:275` / `_run:260` in the Cancellation tests (test lines 600 and 545): the failing progress callback and the interrupt tests. Owner: staffage cast export (`🧍️staffage/🐍️.py`) and the cast CLI; the cast export itself failed, not only the cancel path, so check the export log.
6. Interior gains `e01` (`❄️cooling/.../👥️internal-gains`): `KeyError: 'cooling-internal-gains-desk'` (and `-lamp`, activity `sleep`) at test line 36. Owner: internal-gains clip, its casts are not registered under the expected ids.
7. Output `e17` (`🏗️authoring/🎬️clip/📁️output`): `NameError: name '_Path' is not defined` in `📊️demand/🎬️clip/🔁️recap/🔊️speech/🐍️.py:25` and `🔁️recap/🐍️.py:74`. Owner: demand recap clip (same missing `_Path` import as item 4).
8. Video contract `e18` (`🏗️authoring/🎬️clip/🧪️tests`): `NameError: _Path` in `🔁️recap/🐍️.py:74` for every Beat scene. Same owner and fix as items 4 and 7.
9. Interaction `e23` (`🪟️presentation/🖱️interaction`): one error in `test_four_fps_transitions_capture_current_native_clicks_at_every_required_level`, from `🥞️hierarchy/🖌️render/🐍️.py:224`, `🌐️browser/🐍️.py:168`, and `⚙️runtime/🐍️.py:201` (browser render command failed). Owner: presentation browser render / clip runtime (`⚙️runtime`).
10. Presentation `e28` (`🪟️presentation/🧪️tests`): `test_encoded_chapter_preserves_body_camera_handoffs...` fails in `🪟️presentation/🐍️.py:128` `_transition` through the browser render batch, same path as item 9. Owner: presentation browser render.

Vitest:
11. Library `🧪️tests/🌡️thermal-span/🟦️.ts` and `🧪️tests/🎭️cast-export/🟦️.ts`: both files fail at load with `strict mode: required property "transport" is not defined at "semio.staffage.cast/v1#/allOf/2/then"` (ajv strictRequired). Owner: the cast schema `semio.staffage.cast/v1` (`allOf/2/then` requires `transport` without declaring it in `properties`). Schema-first, so fix the schema.
12. React `🧪️tests/📚️catalog-loading/🟦️.tsx`: `expected { ready: 146, invalid: 1 } to deeply equal { ready: 147 }`. Probably the same `plate-heat-exchanger` manifest mismatch as in the ht report. Owner: the staffage catalog entry.

Catalog check:
13. `suitability-error ... width is not defined` on all 147 assets at 1:50, 1:100, 1:200, 1:500. The error is raised in the suitability measurement path (`🖨️representation`, called from `🩺️check/🟦️.ts`), not in the assets. Owner: staffage representation/suitability module. Not grepped further, so the exact file line is open.

### Still open
- Line 3 (`🌬️ventilation/🎬️clip/📦️volume`) import failure: check `🗑️generated/cr/round-1/logs/e03.log`.
- Round 1 is red. Round 2 started at 16:21 with the same scope.
