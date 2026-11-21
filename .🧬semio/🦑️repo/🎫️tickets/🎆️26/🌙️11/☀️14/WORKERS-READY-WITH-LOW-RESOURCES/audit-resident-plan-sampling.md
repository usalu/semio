# Resident Plan Memory Sampling

The live coordinator reported memory pressure `warn`, about 12,206 MiB of swap used and load 61.26 on ten cores at 02:44 UTC. A separate resident slice plan had a 2,048 MiB reservation and 8,434 MiB observed. The capacity collector measures the attached worker tree's physical footprint, independently of its RSS. The plan's booking is a scheduling estimate. The pinned implementation and original requested size of that long-running external invocation could not be established. [Graph plan pressure audit](audit-graph-plan-pressure-2026-11-21.md)

The current resident plan guard started another asynchronous process-tree check on every timer tick even while the previous check was pending. On Darwin each check reads the process table and invokes the physical footprint collector. Linux walks the process table and reads the members' memory accounting. A held-sampling fixture used the production plan guard and its normal pressure timer. On the fifth pressure tick SQLite found five concurrent sample calls. The process-tree sampling port now admits one sample at a time per slice. Pressure and cgroup checks still run at their existing cadence. When the sample settles, the next tick can take another. A sample completed after the worker's terminal result cannot amend the outcome.

The native sample fixture does not attribute the live host's pressure to this overlap. It proves the bounded per-slice behavior under a delayed sample. No live worker or lease was changed.

The combined resident plan and owner look suite passed all 85 tests with 732 assertions across 16 files. Scoped staged and unstaged whitespace checks passed.
