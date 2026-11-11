# Host sampling fails safe under load

## Finding
The machine-wide gate sampler (`host-capacity/📡️sampling`) reads `/proc` without any loan, so the telemetry loan skip (`native-build/📈️telemetry/⏱️sampling`, advisory only) never feeds admission. Zeros in `repo fleet status` meant the capacity reader failed (sample read threw) and the section was left out; `currentSample` had no fallback.

## Changes
- `🎫️gate/📖️status`: `currentSample` keeps the last good sample (same host declaration) when the live read fails and returns it with its own `atMs`; no earlier sample or an aborted wait still fails.
- `📏️decision` `agedHostSampleV1`: a sample older than `Protocol.staleSampleMilliseconds` (30 s) reads as raised pressure with partial counters, so the existing soft pause admits one run at a time (override path, minimum jobs) and grows nothing.
- `🚢️fleet-status`: host fact `ageSeconds`, report `memory.sampleAgeSeconds`, line "sample N old" (en/de), `memoryNone` line instead of zeros, finding `sample-stale` (policy `sampleStaleSeconds`).

## Tests (managed bun)
`host-capacity/🧪️tests/🕰️stale-sample/{🧫️fixtures,🥒️.feature,🟦️.ts}`; fleet corpus and Python oracle updated (new `sample-starved` case, regenerated seeds). Host-capacity main, sampling, gate (31) and fleet suites pass; `repo fleet status` shows real numbers with "sample 0 s old".
