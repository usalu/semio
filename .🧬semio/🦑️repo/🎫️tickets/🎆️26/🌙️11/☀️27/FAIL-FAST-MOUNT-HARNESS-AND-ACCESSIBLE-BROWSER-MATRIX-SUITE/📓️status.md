# 🧪 Fail-Fast Mount Harness and Accessible Browser Matrix: Status (2026-11-27 14:40)

## Done and tested (graph-independent)

| Piece | Path | Proof |
|---|---|---|
| Fail-fast mount runner: per-phase deadline, cancellation, first failure skips the rest, reason in en and de (last refusal kept on a deadline), total budget capped at 60 s | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🏔️mount/` | 9/0; vectors `🧪️tests/🔣️.json` agree with the Python oracle `🐍️.py`; plan schema agrees with Ajv 2020 |
| Locating by role and accessible name (ARIA 1.2 implicit roles, accname 1.2 subset); one self-contained function runs in jsdom and is injected into pages | `…/🧪️testing/♿️accessible/` | 13/0; every query matches `@testing-library/dom` `queryAllByRole`, except one `inert` case (dom-accessibility-api 0.6 doesn't implement inert), marked `oracle:false` |
| Interaction matrix: the schema only admits role plus en/de name targets (no selector field exists), rows run fail-fast, every live locate is checked against Playwright `getByRole` | `…/🧪️testing/🧮️matrix/` | 12/0 with a fake page; Ajv and the in-repo validator refuse CSS/XPath/single-language/unknown steps |
| Fleet relay: a loopback service is reachable from the fleet network only during the run; peers outside `SEMIO_FLEET_NETWORK` are refused | `…/🧪️testing/🛰️fleet-service/` | 2/0; membership vectors agree with Python `ipaddress` |
| Worker requests forward `SEMIO_TEST_SERVICE_ORIGIN` (schema allowlist, http(s) origin only) | `…/🏗️build/🌐️remote-execution/🚀️offload/🔬️checkout-test/` (patch `🧰️scripts/apply-forward-environment.py`) | checkout-test 23/0, typecheck passed; landed at 13:5x and recorded in wave-1127 `landings.md` |
| Play extension: pane mount phases (deep link `/#<pane>`, shell beacon, quiet settle, tour by role and name), one bun test per language per row file | `🏢️semio-tech/🎡️play/🧪️tests/🧮️matrix/🟦️.ts`, `📖️texts/`, `⚖️laws/` | typecheck passed; law test not run yet (no matrix JSON yet) |
| Suite driver `TestMatrixScript` / `TestMatrixFormsScript`, router `test-matrix-forms` | `🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/` | typecheck passed |

Fleet probe (`🧰️scripts/worker-probe/`): two plain files ran in parallel on g1-m11 and g1-m12 in 5.9 s. Workers reach this Mac at 192.168.178.70; `dist` and `🗑️generated` are not replicated; g1-m12 has `chromium_headless_shell-1248`, g1-m11 has no Playwright browser.

## Seal quiet window (wave-1127 brief addendum 14:40)

- My `test-matrix-forms` target in `🏢️semio-tech/🎡️play/📋️project.json` was written during H84's proof read 10 (started 13:16). I removed it at 14:37; `project.json` is identical to HEAD again. The target is staged in `🧰️scripts/staged-test-matrix-forms-target.json` (insert after `test-e2e-forms` once the coordinator lifts the window).
- My forms serve (`🧰️scripts/serve-forms.sh`) sat in "Reading the project graph slice" for 26 minutes. I stopped it at 14:37 (my own pid) so it doesn't compete with the proof read.

## Open (needs the graph: forms activation + serve)

1. Serve forms (`🧰️scripts/serve-forms.sh`), then author `🏢️semio-tech/🎡️play/🧪️tests/🧮️matrix/📋️forms/🔣️.json` from the live accessible names (the en/de labels are in `🧪️tests/🎭️acceptance/🧫️fixtures/📋️forms/🔣️.json`), plus the mount file and one file per row.
2. Measure the mount files locally: target is under 60 s, fail-fast (a broken plan must stop at its phase).
3. Re-apply the staged target, then run `repo run @semio-tech/semio-tech-play:test-matrix-forms` with `SEMIO_TEST_FILE_OFFLOAD=always` (≥2 workers) and with `never`, and compare the rows.
4. Workers without a full chromium (g1-m11): `launchBrowserV1` uses `channel:"chromium"`; check whether `repo tools setup` installs it on workers, or whether the file must refuse with a precise reason.

## Freeze widened (wave-1127 brief addendum 16:50)

From 16:50 the freeze also covers `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`, `🧰️framework/🔨️modules/🏃️process/`, manifests, project.json and suite tables. My writes under `📚️library/` happened during proof read 10, before I knew about either addendum: the checkout-test `environment` field (5 files, 13:5x) and the new folders `🧪️testing/🏔️mount`, `♿️accessible`, `🧮️matrix`, `🛰️fleet-service` (14:00 to 14:30, imported only by Play test code). Reported to the coordinator at 16:5x; left in place until the coordinator decides, because removing them would be another write during the read. From now on: only stage and test in those paths. Play app/test code (`🏢️semio-tech/🎡️play/🧪️tests/🧮️matrix/`) remains allowed.

Coordinator decision (17:0x): keep the library writes, the read had already absorbed them. Stage and test only until the coordinator messages that the freeze has lifted.

## Landing 23:07 (repo land)

`test-matrix-forms` target landed in `🏢️semio-tech/🎡️play/📋️project.json` through `repo land --wave "Wave 1127"` in the 23:00 slot (suite-law passed), rollback point `stage-matrix-20261127T220735-a65e56`. Earlier checks failed on the overlay defect (20:17, ref land-check-2), which H89 fixed at 23:01.

Forms serve restarted 20:52 after the coordinator fixed a dangling shard import (`unit-cache/🚪️launch/🟦️.ts`, 20:47:55); at 23:04 its graph read was still scanning the file inventory after 7883 s.
