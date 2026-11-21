# Pack Rescue Overhead Before Stage Publication

The serialized replica turn awaits `watchGit` before publishing paths and the Git stage. A due rescue pass at `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🟦️.ts:2381` was reading the object-name tables of every complete known or authorized pack even when there was no local pack old enough to rescue. The table reader allocates twenty bytes for each SHA-1 name. A 2,281,125-object index, the size previously observed on the coordinator, can therefore account for 45,622,500 bytes in a single name-table read. The earlier sample provides scale and does not identify the packs classified during a specific stalled turn.

`🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧷️packgate/🟦️.ts` now classifies complete packs and handles expired local fragments before loading object-name tables. An empty local work list returns without reading a known or authorized pack index or resolving Git. If local packs qualify, the same names still protect already-held objects. Invalid local indexes remain kept, and an already-cancelled operation performs neither file removal nor index reads. Known and authorized pack fragments remain protected.

The neutral fixture names known, authorized, fresh local, old invalid local and stale orphan packs. Ajv validates its schema and an independent SQLite query predicts which indexes are read and fragments retired. Before the patch the actual callsite opened both shared indexes in the no-rescue case. After the patch the focused case passed within the full owner file, including genuine private Git repack, aggressive gc and consolidation. Session 31070 exited 0 with 4 tests and 58 assertions in 9.79 seconds. Terminal capture is in `🗑️generated/pack-rescue-stage-gate.log`. Scoped diff check passed. No running replica was changed or restarted during verification.

```sh
repo tools exec --light bun -- test './🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests/🧷️packgate.ts'
```
