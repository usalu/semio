# Incremental Unit History

Failed or killed native attempts now leave usable rows: each successful compiled unit is appended to `units.jsonl` the moment it completes.

## Design
- Same store file, same row schema (`ObservationV1`). A record is complete exactly when its line is newline-terminated; the unterminated tail is an interrupted append and is dropped by `readUnitHistoryV1` (every terminated line must still validate, so real corruption throws).
- `appendUnitHistoryV1` (admission module): under the store's existing exclusive resource lease it truncates an unterminated tail left by a killed writer, appends whole records with one write and `fsync`s. Crash loses at most the record in flight; concurrent writers (drivers, compaction) are serialized by the lease.
- Identity of an observation is (key, atMs). The row appended at completion has `remainingMs = wallMs` and `atMs = unit.endMs`; the end-of-run `recordUnitHistoryV1` writes the same identity with the critical-path `remainingMs` and replaces it (`compactUnitHistoryV1` is last-wins per identity). `unitObservationsV1` no longer takes `atMs` (uses `unit.endMs`); shared `unitObservationV1` builds one row.
- Bounded: the driver compacts the store once at start (window per key), appends during the run (at most one row per unit), final record compacts again. Failed attempts therefore grow the file by at most one run of rows between compactions.
- Broker: optional `onCompleted(unit)` option (errors logged, never fatal). Driver `component-pass` measures the unit's output bytes at completion, queues appends on a serial chain (append errors only logged) and awaits the chain before the final record. Offload gate and remote worker code untouched.
- Cached units and failed units are not recorded (as before). Units whose output directory is gone are skipped at completion and picked up by the end-of-run record if the run finishes.
- Schema: description of `ObservationV1` documents format and identity; no new policy constants.

## Tests (`🎟️admission/🧪️tests/🟦️.ts`)
- Neutral vectors `journal` in the fixtures: refinement/dedupe, cross-checked with d3-array `rollup`.
- Concurrent appends (`Promise.all`) all present, well-formed, equal to the compacted vector.
- Crash simulation: a record cut at every byte offset; read sees all complete rows, next append repairs the tail to exactly the expected bytes; a terminated invalid line throws.
- Result under `repo test-file`: 14 pass, 2 fail (the 2 are the known nested real-Cargo harness `Undeclared original Repo build selection`).

## Unverified
- `tsc` over the repo crashed (node abort) so no type check output; runtime via bun only.
- `component-pass` tests: 45 pass, 24 fail, the failures seen are the same `Undeclared original Repo build selection` harness class (not compared against a clean tree: no git stash allowed). No component-pass test exercises the new append path end to end.
- `♻️replay` suite: 3 pass, 1 fail (`physical charge freezes broker admission...`, "Compiler broker did not deliver its sampled state"); deterministic, not obviously related to the completion hook, not compared against a clean tree (the broker file also carries other agents' uncommitted changes).
- Real kill (SIGKILL of a live driver) not run; the byte-cut simulation covers every torn state of a single append.
