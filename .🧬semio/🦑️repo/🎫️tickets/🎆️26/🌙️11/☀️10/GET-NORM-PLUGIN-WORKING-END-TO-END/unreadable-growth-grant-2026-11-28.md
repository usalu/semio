# Unreadable growth grant tjg63pHpDXhxolbJ5uoVWQ (2026-11-28)

## Cause

Grant `tjg63pHpDXhxolbJ5uoVWQ` (`cargo-growth`, label `unit-growth:IgISpGEq5Oo5rMhgYs_U9g`, pid 75202, the Energy component publication) is `~/Library/Caches/semio/host-capacity/grants/tjg63pHpDXhxolbJ5uoVWQ.json`. The row carries three keys that the schema `Grant` (closed, `additionalProperties:false`) does not have: `peakMegabytes`, `observations`, `deviationMegabytes`. Nothing else is wrong. The supplementary parent-grant reservation is not the cause: its fields (`operation`, `label`, `priority`, `reservedMegabytes`) are all valid.

The keys come from a transient version of the gate heartbeat. Pid 75202 started at 23:24 and runs the gate code as it was in the working tree then, where the observation statistics were stored inline in the grant row. At 23:36 the gate (`🎫️gate/🟦️.ts`, `statsPath`) and the reader (`📖️status/🟦️.ts`, `withGrantStatsV1`) and the schema (`GrantStats`, new definition) moved them into the sidecar `grants/<id>.stats`, so the row stays a pure `Grant` (the grant-format rule of the hold-and-wait report: no on-disk record format change). Readers started after 23:36 reject the inline row of the older, still running writer, and `readGrants` correctly refuses to guess because the grant is live and holds memory.

So this is version skew between a live writer and the current reader, not a defect of the current writer or schema. The schema is unchanged by this fix: the three keys are legitimately not part of the `Grant` contract.

## Fix

- Writer and schema already agree in the source (sidecar `.stats`, `GrantStats`). No compat shim in the reader was added (greenfield: no legacy support).
- New regression test `host-capacity/🧪️tests/🧾️grant-record/🟦️.ts` (4 cases): runs the real gate (`withHostCapacityV1`, 20 ms heartbeat) for a normal grant, a unit growth (`unit-growth:<parent>`), the supplementary parent-grant reservation (interactive `cargo-growth`, 759 MB) and a labelled grant; while the grant is held it reads the row from disk and asserts `isHostCapacityV1("Grant",row)`, no `peakMegabytes|observations|deviationMegabytes` key in the row, a valid `GrantStats` sidecar, and that `readHostCapacityStatusV1` lists the grant. This would have failed with the 23:24 writer.

## Live state

- Pid 75202 not killed. It rewrites the whole row every heartbeat (about 15 s, `observations` 138 -> 141 observed) from its in-memory state with the inline keys, so editing the record would be overwritten at once and would race its writer. Not rewritten.
- `repo capacity status` therefore still throws until 75202 ends (then the row is removed by its cleanup). Anything that must proceed before: only a restart of that publication under the current code writes a readable row. The only other option is a reader tolerance for these three keys, which I did not add (compat layer).

## Results

- New test: 4 pass.
- Host-capacity suites (91 test files in `host-capacity/🧪️tests` and the repo library's `host-capacity/🧪️tests`): 88 pass, 3 fail, none related:
  - `🌤️followup` g13 and g29 replays (2 tests): `fallbackCeilingMegabytes` 8192 vs 9126 reserved; estimates are being edited concurrently (`📐️estimates` is modified in the tree).
  - `🔁️restart/📜️child` and `🪶️lazy-graph/🧲️probe`: helper scripts that need an argument, not tests (picked up by my file glob).
