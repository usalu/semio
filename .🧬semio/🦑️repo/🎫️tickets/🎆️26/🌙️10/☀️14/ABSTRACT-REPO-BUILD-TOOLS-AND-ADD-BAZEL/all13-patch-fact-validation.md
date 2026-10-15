# ALL13 Logged Patch Fact: Fresh Public Validation

Root's independent two-read current witness found that the logged stale patch fact exactly matches the current patch. This follow-up ran only the real public observation validator under a fresh finite Original. No production Source, stored receipt, policy, key, quota or process was changed; no FactStore, ledger or producer was opened.

## Actual Run

Ticket-only `qualify/📜️script.ts --all13-patch-fact` extracts the one exact patch file row from the final 1 MiB of the existing terminal ALL13 log. The required path is the real library `⚡️caching/🩹️patches/nx@23.2.0.patch`; it does not substitute the current fact for the logged input. A fresh `createScriptProcessEnvelope`/`withScriptProcessEnvelope` has owner `all13-patch-fact-read-only`, maximum elapsed **60,000 ms**, and workspace root equal to the actual repository. Original checks and awaited progress bracket the real calls; both use the original signal and new empty stable-directory/symlink sets.

The module's **`observationsHold([logged], limit, new Set(), new Set(), original.control.signal, onStale)`** returned **true** at both limits **1** and **64**. Both stale lists are empty. Terminal status is **0**, total diagnostic elapsed **39 ms**. Independent bounded physical reads before and after show identical current size, SHA, inode, mtime and ctime, with no differing logged field:

- Size: **121,224**.
- SHA-256: `b8ed82f74478d01ac1756a9d006e3ea7bb62d7303ab03a92b7681833674274e3`.
- Inode: **332424039**.
- mtime and ctime: **1792042481937.7683**.

Exact inputs/results: `🗑️generated/all13-patch-fact-validation.json`; actual console result: `🗑️generated/all13-patch-fact-validation.log`. The observation implementation read identity is `425fcec4e001631bf977ca6e30c992c2c5ee02441b645fb6282f7211e9db884a`. This validates the logged singleton at this fresh point in time, not ALL13's complete historical candidate. No current byte difference is established.

## What the Existing Validator Actually Does

`buildGraphSourceFactsHoldV1` calls public `observationsHold` directly. That public function does not consult `projectChecks`, `inferenceValidationScopes` or the same-receiver asynchronous delivery memo. It first checks semantic consistency of the entire input array, then physically checks rows through its bounded workers. Its normal file branch checks size and stamps, falling back to SHA for changed stamps. The file-only diagnostic needs no registered parser/provider, so it does not bootstrap an inference producer.

The separate private `projectObservationHoldsV1` memo is keyed by the exact validation-frame Map (or parent Map); within it the key is observation identity (kind/path, with provider only for analysis), and each cached entry also compares exact JSON of `row.fact`. New frames have separate WeakMap entries. This memo can preserve a previous per-frame result until final validation, but it is **not on the final public validation path**. A previous frame's memo cannot directly return false from this fresh or final public call. No evidence here identifies it as the actual ALL13 cause.

A different mechanism exists in the public consistency preflight: if two rows have the same kind/path but different file SHA meanings, it returns false and calls `onStale` with the **second** row. That second row can itself be physically current. `observationMeaningV1` compares file SHA meaning separately from stamp hints, so identical SHA with changed metadata is not a contradictory file meaning. The terminal ALL13 trace retained only the reported row, not the complete candidate array or the first contradictory row; therefore this is an explicit possible explanation, **not a reproduced or proven ALL13 contradiction**. No synthetic fact was created to claim otherwise.

The other unresolved class is a failed/transient physical probe at the historical validation instant: public file checks return false on stat/read failure. This run neither observed such an error nor proves one occurred. Current equality and fresh success cannot choose between these historical explanations. The absence of a historical complete final rowset or exact failing probe error is the remaining diagnostic boundary; a Source fix or fresh full retry is not justified as a proven validator repair by this singleton result alone.
