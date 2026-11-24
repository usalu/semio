# O27 patch set for the play clone: warn pressure no longer halves the fleet share

Clone: `/Users/ueli/.sinc/play` (O26-patched is applied there; pre-images are the clone's files of 15:05). `./apply.sh /Users/ueli/.sinc/play` verifies every pre-image, refuses on mismatch, is idempotent. `./build.sh <clone> <scratch>` regenerates everything from the clone's current files.

## Needs a chain restart
- `job-slots.ts` + job-slots schema: DRIVER code, loaded once, effective only after the play chain (native-build driver) restarts.
- `units.ts` + units schema: read per `repo cargo` launcher / build start (the compile-unit -j count is chosen once at build start), so the NEXT build process sees them; the running build keeps 14 remote jobs.
- test/oracle files: not runtime.

## One-line interim alternative (restart without the patch set)
Set the divisor to 1 (no effect on the fleet share) in the clone:
- component jobs: `/Users/ueli/.sinc/play/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧵️job-slots/🧬️schema/🔣️.json`, key `definitions.Policy.const.pressureWarnDivisor`: ALREADY `1` in the clone now (check with grep; do not set it back to 2).
- compile units: `/Users/ueli/.sinc/play/🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧮️units/🧬️schema/🔣️.json`, key `definitions.Policy.const.pressureWarnDivisor`: currently `2`. CAUTION: in the unpatched code this key also halves the LOCAL compile units under warn (`pressure-warn`), so `1` lifts both. Accept that or use the patch set.
