# lrrebase: quiet16 `lastrun` fixture drift (10-27 21:06)

FOR coordinator: apply order `hold` then `lastrun` (lastrun is stacked on hold):
- `python3 -I C/quiet16-tools/staged/hold/stage.py apply` then `python3 -I C/quiet16-tools/staged/lastrun/stage.py apply`
- verify: `sh C/quiet16-tools/staged/hold/verify.sh <root>` after hold, `sh C/quiet16-tools/staged/lastrun/verify.sh <root>` after lastrun
- revert order: lastrun, then hold.
- Before hold is applied, `lastrun/stage.py check` reads 16 `base` + 1 `drifted` (the library quiet fixture does not exist on live; hold creates it). That is by design (stack): after hold all 17 read `base`.

## Cause
The "255 conflicts" were not conflicts: `git merge-file` exit 255 = error. The live file
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🤫️quiet/🧫️fixtures/🔣️.json`
does not exist on live (the whole library `🤫️quiet` test directory is created by `hold`). Nothing was regenerated live.

The real defect was in `hold`: its `files/23.json` (the fixture it creates) was byte-equal to lastrun's final fixture (sha 1aa32cb4...), i.e. `make.py hold` was run at 19:11 on a mirror that still had lastrun applied. Consequence: `hold` alone failed its own oracle (`FIXTURE DIFFERS FROM THE ORACLE`; policy/upgrade oracle of hold-only world do not carry lastrun's change), and lastrun's entry 16 (base 04fe33da..., the true hold-only fixture) read `drifted` after hold was applied.

## Fix (no hand merge)
- Applied hold to a mirror (`lr-reb`), regenerated the fixture with its own oracle (`🐍️oracle/🐍️.py`): result sha 04fe33da... == lastrun's recorded base, byte for byte.
- `hold/files/23.json` := that fixture; `hold/manifest.json` new := 04fe33da...; backups `hold/files/23.json.pre-lrrebase`, `hold/manifest.json.pre-lrrebase`.
- `lastrun` untouched: with lastrun applied on top, regenerating the oracle gives 1aa32cb4... == lastrun `files/16.json` (so base 04fe33da / new 1aa32cb4 were already correct).

## Verification (mirror lr-reb, sealed, guard clean; live unchanged; mirror dropped)
- hold alone (hold/verify.sh): oracles equal, `bun test` 100 pass / 1 skip / 0 fail (17 files), law simulation PASS, VERIFY OK.
- hold + lastrun (lastrun/verify.sh): 6 oracles + estimates oracle equal, stage check applied, `bun test` 106 pass / 0 fail (14 files), simulation LAW PASS, VERIFY OK.
- Revert chain on the mirror (lastrun then hold) returned all 24+17 entries to base; re-apply clean.
- Live now: hold 24/24 `base`; lastrun 16 `base` + the fixture `drifted` until hold lands.
Not run: framework root suite `🧪️tests/🟦️.ts` (SQLite oracle cases time out under load, see lastrun/verify.sh note; `C/quiet16-tools/suites.sh`).
Logs: `C/🗑️generated/lrrebase/[DEBUG] hold-verify.log`, `[DEBUG] lastrun-verify.log`.
