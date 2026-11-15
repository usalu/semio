# Heavy Owners Read Local-First (Solo Wave)

## Root cause
`fleetReadV1` classifies owners with `classifyOwnersV1` against `ownerGrantV1(half the host)`. On a 16 GB Mac the grant is 1756 managed MB (8192 - max(2200, 3800) = 4392 MB footprint / 2 / 1.25). `@semio-tech/framework-os-kernel` has a recorded cost of about 3492 managed MB, so it is heavy. A heavy owner was "never inferred here": it was asked from the fleet and, when the fleet did not serve (wait ended, or fleet unreachable), the read threw `graph.wanted-expired` (unreachable: immediately). A machine alone could therefore never plan, which violates local-first. The grant is the share each of many PARALLEL owners gets; nothing compared a heavy owner with what one owner alone can be given.

## Design
Fleet-first stays. When the fleet does not serve, remaining light owners are inferred by the waves as before; remaining heavy owners are inferred locally, one at a time, alone, after the light waves, each with the solo grant.

Solo grant (`soloGrantV1`, `💰️cost`): footprint = min(host - `soloReserveMegabytes` (new, 2048) - `floorMegabytes` (3800), available - `availableFloorMegabytes` (2048)); managed = footprint / factor / (1 + margin). 16 GB Mac at rest: 10536 MB footprint, 4214 managed MB, enough for the kernel (3492); at 7599 MB available: 2220, so it is refused. `planSoloV1` splits heavy owners into `fit` and `over`.

Result of the read: `FleetReadV1.solo = {owners, grant}` (replaces `heavy`); `missing` keeps all projects left for this machine. Order of checks (unreachable and after the wait): more light owners than the waves can infer -> `graph.wanted-expired` as before; then heavy owners over the solo grant -> typed refusal `graph.owner-too-heavy` (`{named, limit}`, line in en/de naming owner and numbers); else the solo owners are announced (`solo` text, en/de). The caller `fleetReadStaleGenerationV1` passes `soloGrant` and returns the result.

Execution (`📦️project-nodes/🟨️.mjs`): `graphOwnerEnforcementV1` now also returns `heavy(names)` (recorded cost over the share, the same `classifyOwnersV1`) and `soloLimitBytes()` (soloGrant of the memory available at that moment, never below the share). The inference sorts heavy jobs last (stable), runs the light ones on the existing workers (`loop(lightEnd)`), then collects and runs the heavy ones sequentially (`loop(Infinity)`) with `limitBytes` per run. A light owner stopped at the share whose solo limit is larger is re-queued as solo instead of refused (first run without a table). Only a stop at the solo limit ends the read with `graph.owner-too-heavy` (limit now in physical MB, same unit as the named costs).

Governor (`🧠️memory/🟨️.mjs`): `run(task,{kind,key,limitBytes})`; `enforce` uses the owner's own limit, else `ownerLimitBytes`. Typings updated.

## Files
- `🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts`, `💰️cost/🧬️schema/🔣️.json` (policy `soloReserveMegabytes`; texts `solo`, `exceeds` en+de; unused `refused`, `among` removed): `grantOfFootprintV1`, `soloGrantV1`, `planSoloV1`
- `🪪️graph-state/🙋️wanted/📖️read/🟦️.ts`: solo logic, `SoloWaveV1`
- `🕸️graph/🤝️prefetch/🟦️.ts`: passes `soloGrant`
- `🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs`: enforcement + solo wave
- `🧠️memory/🟨️.mjs`, `🟨️.d.mts`: per-run limit
- `📖️texts/🔣️.json`, `🏡️resident/🧬️schema/🔣️.json`: `owner-too-heavy` texts reworded (no more "the fleet is asked first")
- Tests: `💰️cost/🧪️tests`, `🙋️wanted/🧪️tests/📖️read` (unreachable + fits -> solo; wait expired + fits -> solo; exceeds -> typed refusal en/de), `🧠️memory/🧪️tests/🛑️overrun`, `📦️project-nodes/🧪️tests/🛑️too-heavy`, `🕸️graph/🧪️tests/🙋️fleet-read` (+ `🌍️small-booking`)

## Test results
Run with `repo tools exec bun -- test ./<file>`: `too-heavy` 6 pass, `fleet-read` 5 pass. The host size is now injected as a port (`hostMegabytes` of `graphOwnerEnforcementV1` and of `fleetReadStaleGenerationV1`) instead of the declared `SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES`, which `hostProfileV1` caps at the real host memory. The `too-heavy` solo test also used a recorded cost at `atMs:1`, which decayed to zero; it now records `Date.now()`. Unrelated failures seen elsewhere: `memory/🧪️tests` `psProcessRowsV1` missing and reservation tests needing `SEMIO_TEST_ARTIFACT_DIR`.

## Not verified
No live `activate-prototype` run (resident service untouched). The solo wave in the real inference is covered by source-structure tests plus the enforcement and governor behaviour tests, not by an end-to-end owner run.
