# 🧪 Round 2 regression report (cr2): round 3 run, compared with round 1

Runner: `🧰️cr/round.sh`. The coordinator stopped the first round-2 attempt at 16:21, and my own round-2 launch was killed too. Both were partial, so the full run was done as round 3 (16:40 to 18:04). The partial `round-2` folder is not used here.

Round 1 is partial too: it has no DONE marker and ran no e28–e32. Its status is shown only for comparison.

## Round 3 counts (copied from `round-3/counts.tsv`)

```
a06	6	0	9	OK	22s
a07	7	0	7	OK	6s
a08	8	0	8	OK	49s
a09	9	0	9	OK	319s
a10	10	0	11	OK	64s
a11	11	0	9	OK	71s
a12	12	0	9	OK	8s
a13	13	1	20	FAILED (failures=1)	251s
a14	14	0	2	OK	57s
a15	15	0	5	OK	23s
b05	5	0	36	OK	34s
e01	1	0	3	OK	8s
e02	2	0	4	OK	11s
e03	3	0	2	OK	17s
e04	4	1	5	FAILED (failures=1, errors=1)	23s
e16	16	1	7	FAILED (errors=1)	74s
e17	17	0	9	OK	21s
e18	18	0	31	OK (skipped=1)	63s
e19	19	0	9	OK	449s
e20	20	0	4	OK	4s
e21	21	0	1	OK	2s
e22	22	0	8	OK	21s
e23	23	0	21	OK	662s
e24	24	0	8	OK	8s
e25	25	0	12	OK	709s
e26	26	0	1	OK	29s
e28	28	1	15	FAILED (errors=1)	961s
e29	29	0	3	OK	142s
e30	30	0	13	OK	38s
e31	31	0	3	OK	31s
e32	32	0	2	OK	45s
DONE
```

Vitest and catalog status (`vitest-status.txt`): `lib rc=1`, `react rc=1`, `catalog rc=1`. All three are non-zero.

## Non-OK modules, round 3 (first error line)

| Module | Result | First error line |
|---|---|---|
| a13 (line 13, staffage_only) | 1 failure, 20 run | `FAIL: test_thermal_span_every_scene_draws_its_thermal_casts_on_one_span`; `AssertionError: 2 not less than or equal to 1` |
| e04 (line 4, ventilation architecture) | 1 failure, 1 error, 5 run | `ERROR: test_rooms_cut_windows_and_anchor_occupants`; `NameError: name '_occupant_height' is not defined`. Also `FAIL: test_closeup_radiator_scales_and_clears_floor`. |
| e16 (line 16, audience) | 1 error, 7 run | `ERROR: test_real_subprocess_renders_use_the_selected_audience`; `RuntimeError: Authored sources changed while rendering Fixture` |
| e28 (line 28, presentation) | 1 error, 15 run | `ERROR: test_encoded_chapter_preserves_body_camera_handoffs_audio_and_contextual_boundaries`; `subprocess.CalledProcessError` on `repo tools exec --light node ... render.mjs` |

## Failing vitest and catalog items, round 3

- **vitest lib** (`vitest-lib.log`):
  - `× sizes a front cast to the front and a side cast to the section`
  - `× equal the pinned digests of the reference export (measured values compared to 4 significant digits ...)`. First assertion: `AssertionError: radiator-transient/0000.svg: expected '907f8c2c...' to be 'b2d15aef...'`, in `🧪️tests/🚀️export-performance/🟦️.ts`.
  - The `Syntax Error: No display font for 'ArialUnicode'` line is stderr noise, not the failure.
- **vitest react** (`vitest-react.log`):
  - `× loads the authored archive as one ready row per manifest entry with an empty manifest check`. First assertion: `expected { ready: 146, invalid: 1 } to deeply equal { ready: 147 }`, in `🧪️tests/📚️catalog-loading/🟦️.tsx`.
  - The `HTMLCanvasElement.prototype.getContext not implemented` line is a jsdom warning, not the failure.
- **catalog** (`catalog.log`): `✗ manifest`, first mismatch `manifest-mismatch entries[111].heightM: Entry "panel-radiator" and its asset disagree on heightM` (also widthM for the same entry). The `[capacity] ... win32` lines are a tooling warning, not a catalog failure.

## Round 1 compared with round 3

| Module | Round 1 | Round 3 | Change |
|---|---|---|---|
| a06 | FAILED (setUpClass error) | OK | fixed |
| a08 | FAILED (7 errors) | OK | fixed |
| a09 | FAILED (4 errors) | OK | fixed |
| a11 | FAILED (1 error) | OK | fixed |
| a13 | OK | FAILED (1 failure) | new regression |
| b05 | FAILED (3 errors) | OK | fixed |
| e01 | FAILED (6 errors) | OK | fixed |
| e03 | no result | OK (2 tests) | fixed |
| e04 | OK | FAILED (1 failure, 1 error) | new regression |
| e16 | OK | FAILED (1 error) | new regression |
| e17 | FAILED (2 errors) | OK | fixed |
| e18 | FAILED (7 errors, 1 skipped) | OK (1 skipped) | fixed |
| e23 | FAILED (1 error) | OK | fixed |
| e28 | not run | FAILED (1 error) | not comparable |
| e29–e32 | not run | OK | not comparable |
| vitest lib / react / catalog | rc=1 each | rc=1 each | still failing |

Round 1 vitest and catalog failures had the same kinds of messages (display font, canvas, manifest mismatch) but different items. The catalog mismatch moved from `entries[122]` `mvhr-unit` clips to `entries[111]` `panel-radiator` heightM and widthM, and the lib export-digest failure is new in this run.

## Still open

- Regressions in round 3 that were green in round 1: a13 thermal span, e04 `_occupant_height` NameError, e16 authored-sources-changed guard.
- e28 presentation render subprocess fails.
- vitest lib (export digests and front/side cast sizing), vitest react (catalog archive ready count 146 vs 147), and the catalog manifest mismatch on `panel-radiator`.
- Not verified: e28–e32 were not run in round 1, so their comparison is incomplete. Round 1 was partial, so it is only a rough baseline.
