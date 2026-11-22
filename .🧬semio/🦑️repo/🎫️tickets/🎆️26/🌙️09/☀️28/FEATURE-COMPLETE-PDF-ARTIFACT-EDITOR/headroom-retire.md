# Headroom retirement: `Retired graph member changed`

## Diagnosis
Not a race. Deterministic and stuck since a 14:29 retirement intent in `.cache/b/g/m/retired/dd60e1f7...json`.

- The intent retired group `inference/selections/6c39.../np` (7 planned members, `committed:false`).
- `np/owner-peaks.json` is a process-private spill of the graph writer. `spill()` skips it in the inventory while it sits at its original path, so the plan never lists it.
- The group was renamed to `retired/<id>.d`. The spill moved with it. At the new path `spill(rel)` no longer matches, so `directories()` inventories it as an ordinary row.
- `retirementCatalog` then sees 8 survivors against 7 planned members. The unlisted one throws `Retired graph member changed` on every capture, and the build service classifies that as deterministic.
- Every `repo run` that reaches storage-headroom cleanup fails, and the stuck intent also blocks all further retirements.

## Fix (`closures/🟦️.ts`)
- `retiredSpill(rel)` recognises a spill below `retired/<id>.d/` by re-running the existing `spill()` rules against every role prefix.
- `directories()` inventories such a file as a trusted `retired` row flagged `spill`. It is not content-read, and it is unlinked with the group by the existing committed-retirement path.
- `retirementCatalog` excludes spill rows from survivor and plan-member accounting.
- A genuinely foreign or changed member is still refused. The integrity law is unchanged.

## Test
`tests/♻️groups/🟦️.ts`, scenarios "travelled spill settles" and "travelled spill foreign-member". They write `np/owner-peaks.json`, invalidate the group, then retire it. The first settles with an empty catalog. The second, with a forged member added, is still refused.

## Verification
- `repo test-file` on `tests/♻️groups/🟦️.ts`: 13 pass, 0 fail, including the two new scenarios and the existing `member-changed` and `new-member` refusals.
- A real `repo run @semio-tech/framework-renderer-wgpu:native-build` did not reach the cleanup step. It queued about 25 minutes on memory capacity and graph-slice workers, and my 1700 s timeout cancelled it with SIGTERM. The run log is `/tmp/hr-run.log`.
- Not verified: the live stuck intent (`.cache/b/g/m/retired/dd60e1f7...`) is still unsettled. Rerun with a longer timeout once the machine is less loaded, then check that `.cache/b/g/m/retired` is empty.

## Not changed
A truly changed or forged member is still a hard refusal. That is the existing integrity law, with tests, and I did not relax it into skip or requeue. The concurrency-safe case here was the spill, which is now accounted for.
