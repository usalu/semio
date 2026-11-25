# 🧽️ GR24: Graph Image Space (`.cache/b/g/m`, 2026-11-25 01:44 to 02:02)

Lane GR24. Lock `lock.graph-closure` taken 01:44 and released at the end. No sub-agents, one `repo` command at a time, load checked before every heavy step (paused once at 31). Nothing restarted. No images detached or resized.

## Verdict
- Before: the 18 GiB image had **77 to 83 MiB free** (floor 1 GiB, reclaim target 2 GiB). After: **8.7 GiB free** (52 % used).
- The space was not the GR22 layer data. It was **three dead build journals** of the generation store (`generations/j`, 8.58 GiB). The existing collector saw only two of them (6.41 GiB). The third was the newest group (2.3 GiB, 226 rows files, last written 00:37), and it had no header `<key>.json`. That is a build killed in its first wave, probably by the 00:59 panic. `readBuildJournalV1` resumes only from the header, so no code can read those rows, but the old rule always kept the newest group.
- Code fix (schema/format rule, not a list of paths): `staleBuildJournalGroupsV1` now treats the newest group as dead too once it has rested beyond the grace (1 h) without its header. The real collection then removed all 3 groups (8.58 GiB).

## 1. What occupies the image (du, depth ≤ 3)
| part | before | after | class |
|---|---|---|---|
| `generations/j` journals: e65c… (19:39 to 19:51, header), 6fdc… (23:15 to 23:18, header), 6747… (00:36 to 00:37, **no header**) | 8.6 GiB | 0 | dead staging of abandoned builds (no holder, `lsof +D` empty) |
| `generations/o` fragments, `members.jsonl`, `m`, `f`, `x` (current generation 260) | 0.94 GiB | 0.94 GiB | current, in use |
| `inference/selections/8d8f…/ce` closure store (e 474 M / 41,134 entries, l 2.8 G / 58,761 layers, x 946 M, travel 280 M, s 212 M, v 20 M) | 4.8 GiB | 4.8 GiB | live store. ~99 % of the entries were written before GR22 (only 158 entries and 3 layers are newer than 00:07) |
| other 117 selection scopes (inference 880 M, np 360 M, sa 312 M, oc 119 M, mr 50 M, rr 20 M) | 1.6 GiB | 1.6 GiB | optional scoped caches. The optional quota reports 491 MiB of 1 GiB used, within its low-water mark |
| `derived-nodes` | 817 MiB | 817 MiB | current (declared budget 512 MiB, left to the graph owner) |
| `part-ids` 274 M, `build-input-closures` 230 M, `selected` 154 M, `retired` 20 M, `build-lock-closures` 18 M, `snapshots` 13 M | 0.7 GiB | 0.7 GiB | current |

Dry run (old code, `GR24/🗑️generated/gc-dry-before.txt`, 13 min, mostly waiting in `source.served`): 2 dead journals 6.41 GiB; 13,127 unreferenced closure inventories 79 MB; 4,090 superseded observation blocks 18 MB; 17 superseded entries + 368 layers + 9 values 16 MB. `repo doctor --only=caches`: healthy, 1 note (undeclared `native:native-play-artifacts`). This scope does not report the graph floor; the earlier floor error came from the storage probe.

## 2. GR22's old-format data: measured, not deleted
- A sample of 514 of the 41,134 entries: 512 have the old sidecar `{facts, members, walks}` and 2 have the current `{facts}`. 61 of the 514 carry non-empty walks. 332 have no layers and empty walks.
- The current loader still reads these v3 records (`💾️entries/🟨️.mjs` `load` checks only the version, id and digest, and reads `sidecar.facts`). The old entry I checked has only ordinary `lstat/field/exists` rows, with no `walk:1` derived rows. So "no code reads them" cannot be shown from the format. I wrote no format rule for them: a wrong rule would re-derive ~41 k entries on a host that is already overloaded.
- Not needed for the target either: 8.7 GiB is free.

## 3. Changes
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🚰️reclaim/🟦️.ts` `staleBuildJournalGroupsV1` (lines ~34 to 40): the headerless newest group is dead after the grace (docstring links the journal format).
- `…/🧹️cache-gc/🧪️tests/🗞️journals/🟦️.ts`: a new law covers a headerless newest group, with the Python oracle updated to the same rule. A header-carrying single group stays kept.
- Hashes are in `T/landings.md`.

## 4. Tests and runs
- `repo test-file …/🧹️cache-gc/🧪️tests/🗞️journals/🟦️.ts`: **4 pass / 0 fail** (18 expects).
- `repo cache gc --area=graph` 02:00:21 to 02:00:33: "Removed 3 dead build journals (8.58 GiB); Reclaimed 8.59 GiB … 8.66 GiB free, target 2.00 GiB".

## 5. Open items
1. Closure store `ce`: it has no closed record schema, so a stale v3 entry can only be collected when it is superseded in its slot. If the GR22 identity really makes all old entries unreachable, the clean fix is a record version bump (v4) in `💾️entries` + `🧹️retention` `readRecord`. The existing "another format → drop after grace" rule would then free ~3 to 4 GiB of layers, index and stamps. This is for GR22's owner to decide: it costs one re-derivation of every entry.
2. `ce/s` stamps and `ce/travel` pointers are only count-capped, never reference-collected. `x/rows.dict` (154 MB) is append-only.
3. `derived-nodes` is 817 MiB against its 512 MiB budget.
4. The ledger has no area for `generations`, `part-ids`, `selected` or `snapshots` (only inference, closures, derived-nodes, stat-index, lock-closures are declared). Journals were collectable only through the reclaim port.
