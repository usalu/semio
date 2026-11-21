# Sparse Stage Register Visits

The stage host used to enumerate the entire manifest, applied-file table and pending queue to obtain only the virtual stage records. The manifest and file visits materialized full ID snapshots and decoded every live path. The queue used a snapshot of queued IDs, then decoded each queued path before testing the virtual realm.

The path book now records stage IDs on first intern and reconstructs the derived list in ID order while reopening existing checkpoints. The list is memory-only. The checkpoint format and file, queue and journal inputs retain their previous form. Stage projections use the existing manifest, applied and queue flags to skip absent and parked rows before decoding. The manifest-entry projection excludes the base. The applied and pending projections still include it.

The fixture exercises ordinary and stage paths, near-matching prefixes, deletes, absent rows, parked queue entries, duplicate interning and checkpoint reconstruction. Ajv validates the fixture and SQLite independently produces the ordered register projections. The red run returned the same 7 projected rows after 1,552 path decodes. The corrected run returned those rows after exactly 7 decodes. It passed both for ordinary intern and checkpoint reopening. Four additional selected private Git integration cases passed, covering stage change and unstage, restart and index recovery, waiting for missing objects, and stage settlement ahead of unrelated payload. Combined run session 18852 completed with 5 tests and 87 assertions in 18.35 seconds. Capture is in `🗑️generated/sparse-stage-registers.log`.

At a projected 350,000 live paths and 1,800 stage IDs, one whole-table projection can visit roughly 194 times as many path IDs as the new upper bound. Pending work now checks at most the recorded stage ID count, irrespective of its queue length. These are structural operation counts, and fleet latency still depends on the remaining Git and storage work.

```sh
repo tools exec --light bun -- test './🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/📇️book/🧪️tests/🟦️.ts' './🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests/🌿️git.ts' --test-name-pattern 'stage register projections|a stage of the archivist replicates|a lost local record does not matter|an entry whose object has not arrived|ready stage registers settle before unrelated worktree payload'
```
