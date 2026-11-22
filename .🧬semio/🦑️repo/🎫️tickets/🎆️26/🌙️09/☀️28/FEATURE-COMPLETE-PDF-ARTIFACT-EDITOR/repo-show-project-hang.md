# repo show project hang under graph policy current

## Symptom
`repo show project @semio-tech/stdio-pdf --json --graph-policy=current` stalled 28 min ("last phase started: source.served"), then refused with the "every project" text.

## Root causes
1. Stall: `show` is a capacity operation, so `withBuildCapacityV1` queued it in the machine-wide gate for 3072 MB behind other agents' jobs (`repo capacity status` showed it `waiting`; 135-180 s on this machine, 28 min on a saturated one). `source.served` is only the last profile mark before the gate; `door.capacity` wraps the whole command. A sealed read under explicit `current` only reads a store and builds nothing, so it needs no grant. The parsed invocation also drops `--graph-policy`, so the check must read the raw args.
2. Wrong message: `loadGenerationGraphV1` with `closure` + `sealedOnly` (single project) fell into the `wholeCurrent||sealedOnly` branch and threw `neverSealed` ("a question about every project ...").

## Fix
- `🌡️host-capacity/🟦️.ts`: new `sealedCurrentReadV1(args, environment)` (show + explicit policy current, args before `--`); `withBuildCapacityV1` runs the body without a grant for it.
- `🕸️graph/🕰️generations/🔌️load/🟨️.mjs`, `.d.mts`, `🧬️schema/🔣️.json`: new bilingual message `projectNeverSealed` naming the project; `sealedOnly` refusals use it (code stays `GENERATION_NONE_SEALED`).

## Tests
- `🕸️graph/🕰️generations/🔌️load/🧪️tests/🔒️current-never-builds/🟦️.ts`: new single-project refusal test (en, de): names the project, no "every project", builds nothing. 8 pass.
- `🌡️host-capacity/🧪️tests/🔓️sealed-read/🟦️.ts`: new; Python oracle agrees on argument vectors, withBuildCapacityV1 skips the gate. 1 pass; `🧮️arguments` suite still 6 pass.

## Verification
`repo show project @semio-tech/stdio-pdf --json --graph-policy=current`: 6 s (was 135-180 s while the gate was busy), accurate refusal. `-rs`: 4 s.

## Remaining
This machine has no sealed whole-workspace generation, so the answer is still a refusal; it needs `repo graph --graph-policy fresh` once (or a fleet restore). Resolving one project locally without a sealed generation (slice read) was not changed.
