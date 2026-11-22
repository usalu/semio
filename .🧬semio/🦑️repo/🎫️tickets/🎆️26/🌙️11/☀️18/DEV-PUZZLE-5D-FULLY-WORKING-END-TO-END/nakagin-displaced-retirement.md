# Nakagin displaced snapshot retirement

The native cursor enters the Store's displaced queue after returned reads. The queue front is a 1,456-byte `ArtifactStoreDisplacedSnapshotRetirement<Puzzle5dPlaySnapshot>` with its original 2,400-byte `ArcRootRetirement<Puzzle5dPlaySnapshot>` admitted. Its selected child is `SharedControlledRetirement<Puzzle5dPlaySnapshot>`.

At the first displaced turns, the Arc root repeatedly quotes one unit of Work and changes physical demand from 56 copy bytes and 32 capacity bytes to eight copy bytes, then 24 copy bytes and 36 release bytes. At the 256th sampled visit the same held snapshot cursor changes from 32 copy bytes to 24 copy bytes and 32 release bytes, and then to one copy byte. Each advances a different rung of the same original snapshot. The v2 editor's fixture census independently recorded 32,768 active close turns and zero consecutive idle turns with nine displaced owners remaining. Production continued to make progress under its admitted grant.

The exact fixture traces are `nakagin-displaced-owner.gdb` and `nakagin-displaced-rung.gdb`. The queue and snapshot retirement production code did not require a change for this finding.

## Final retained history cache

The v6 native executable moved beyond Store Displaced into the app-level RetainedFields rung. At its original 131,072-turn fixture horizon it reported 131,072 active close turns and zero consecutive idle turns. The last quote was copy 32, capacity 16, release zero, and depth three, with the layout's unchanged one-item, 32 KiB copy, 16 MiB capacity, 1 MiB release and depth 4,096 close grant.

The exact v6 switch test passed in 86.45 seconds under `nakagin-retained-fields-v6.gdb`, which changes only `OriginalFixtureCaller::maximum_close_turns` from 131,072 to 524,288 in the diagnostic process. The first retained-field turn releases the history view. The second admits the 800-byte history projection-cache retirement. By the third turn an actual retained cache owner sits at the disassembled `CommandHistory` offset `0x458`. Sparse observations show funded progression through copy 32 and release 32 at retained-field turn 16,384, copy 56 and capacity 32 at turn 65,536, and copy 24 and release 12 at turn 262,144. The unchanged owner cleared before the diagnostic horizon and the test returned terminal empty with normal process exit. This is a finite cleanup cost for the history cache containing retained snapshot, config, and history roots.

The existing fixture-only `maximumCloseTurns` now uses the proven 524,288 horizon in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json`. Native mounted policy, original caller stock, per-turn Work, idle count and Drop ownership checks use the existing controls. The original production-mounted turn grants also governed the successful diagnostic run.
