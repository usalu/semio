# Capsule Dream publication stall

The native Capsule Dream law finished its worker and received `interactive-job.publication-stalled` at fixture turn 42,607, after 48,543 Store publication units. This is the build with per-turn fixture renewal and the witness for accepted physical Store receipts. It already acknowledges physical progress even when the visible checkpoint remains unchanged.

`capsule-publication-stall.gdb` probes the original native executable recorded in `live-archive-turn-2-build.log`. GDB captured 7,088 document Store calls. The last one returned in Preparing with checkpoint (28, 152), a physical item receipt, and two Work units. The publication fault followed exactly 4,096 outer publication polls later with no Store invocation in between.

A separate native run with runtime diagnostics revealed that the Artifact lane's returned document-read count was continually two at the pauses, above the threshold of one. The old branch returned immediately and depended on a separate maintenance rotation to reclaim those reads. Retiring one previously staged Capsule Dream root consumed thousands of paid maintenance turns while the publication saw only an unmoving Store checkpoint.

The typed publication now drives one exact document snapshot-read return when this gate binds. It reuses the same disposer as maintenance, settles its physical receipt and Work against the originating context, and records progress after a nonempty receipt. A stalled disposer still accumulates the ordinary publication fault streak. The production changes are in `plugin/🧵️instance/📄️docs/📥️staging/🚪️ingress/🦀️.rs` and `plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs`.

The existing native Capsule Dream law exercises the publication route. Both Bun publication-stall tests passed, including the independent SQLite oracle for fixed frontiers with periodic physical receipts, and the source law for taking the retirement receipt before marking progress.
