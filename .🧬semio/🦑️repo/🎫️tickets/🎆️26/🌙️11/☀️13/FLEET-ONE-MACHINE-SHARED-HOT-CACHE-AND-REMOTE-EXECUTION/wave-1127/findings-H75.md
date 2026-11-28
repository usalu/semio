# Findings H75
1. Client-side latency budgets only see the door. Resident-served commands run their phases in the service worker; the forwarded result already carries `serviceTiming` (`🏡️resident/🚪️client/🟨️.mjs` `forwardCommandV1`). Fold it into `latencyObserveV1` (new group source) or have the service print its profile document to the client; until then `uncovered` is the service time.
2. `cacheLookup` round trip has no counter (sealed `🧩️units`): add `profileRoundTripV1("cache-lookup")` in the units store read and set `Trips.cacheLookup.measured` true after the window lifts.
3. Graph writer stage timing (H43) as `diagnoseV1("timing", ...)` per stage belongs in graph code; sealed. The generic `SEMIO_DIAGNOSTICS=timing` stream already covers every `profilePhaseV1` phase.
4. Landing gate: H74's `repo land` should run the diagnostics law (`🩺️diagnostics/🧪️tests`) so a temporary debug line never lands.
