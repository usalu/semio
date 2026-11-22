# G2 result: nothing in the graph computed twice (wave-1121)

## Findings (census corrections, verified in code and by test)
1. Per-owner sharing already exists. `createProjectFleetPortV1` (`graph-state/project-records/🟦️.ts`) does local-first lookup (observation `🟨️.mjs:1179-1180`), fleet lookup, fleet claim (`tryGraphEntryV1`/`decideGraphEntryV1`, lease 30 s + heartbeat, dead claimant expires), bounded wait (min of 900 s, 60 s lease take-over), local inference, publish. Workers call `port.gate`, the owner pool calls `port.claim`: same state digest, same claim key, so the pool claim is already the shared fleet claim. The census claim "no worker-side claim" is wrong. No production change was needed.
2. Whole gate: `fetchable` is tested before `held > 0`, so a machine holding owners fetches a complete ahead head; `skipped` only appears when no complete head exists. New test pins it.
3. The default selected slice read (`SEMIO_BUILD_GRAPH_SLICE_READ` unset) infers the selected owners through the per-owner tier (`load/🟦️.ts:311-315`: restore-from-generation is the staged `generation` mode). Changing that default is a rollout decision and was not made; per-owner take restores owners from the fleet.
4. Implementation digest: rows are relative path + content sha256 or stamp text, sorted; stat facts and roots never enter (new test). No receipt in `/Users/Shared/semio-store/v1/origins/*` records an implementation digest, so a cross-machine comparison from receipts is impossible; local manifest digest 57b78959b8da... Derived-inferrer stamps are digests of corpus answers (platform neutral) but fall back to content facts when unavailable on one machine.
5. Record `json`: owner descriptors are repo files; no `/Users`, `/home`, `/private`, drive paths in owner project.json (only 3 ticket files). Fleet transport neutralises observation paths (`neutralRecordV1`); test asserts B's imported record has B's paths and none of A's.

## Tests (new, in-process fleet cache + claim table on a movable clock, two temp roots)
`graph-state/project-records/🧪️tests/🏭️fleet-machines`: B waits for A and never infers (computed 0); pool-style claim honoured by worker gate; dead claimant replaced after lease lapse, then C takes B's upload; machine holding 1 of 5 owners takes 4 from the fleet, inferring none; stalled and refusing fleet end in local inference. Plus whole-gate held>0 fetch and implementation digest neutrality.

## Measured defect to hand over (not changed)
A stalled or refusing fleet costs the first gate of a read about 30-34 s (two retries each waiting the 15 s breaker), after that owners skip instantly while the breaker is open. Bounded but heavy; cutting `gate` retries from 2 to 1 would halve it (affects the retry law in `📮️retry`).

## Real read
`repo show projects --json`: 127 s, refused by policy ("no sealed generation of the whole workspace"; local pointer is quarantined closure-missing). Not a sharing failure.
