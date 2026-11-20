# Cargo Store Collection Progress

The recorded `repo cache gc --area=cargo-store` run exited 124 after reporting no progress for 13 minutes. Free space rose from about 3.1 GiB to 16 GiB during the collection. The command buffered collector messages until completion, and the delegated Cargo stages did not publish progress to the original invocation. Component run collection also sized every run recursively before checking whether it was eligible for removal.

The standard command now publishes throttled progress from actual scan batches, file measurements, and deletion batches to the original invocation. It uses the repository's cancellable tree remover while retaining the Cargo session locks and profile leases through deletion. Ordinary result lines stay in the final report, so JSON output keeps its original shape.

Component run collection now identifies superseded candidates from shallow recency and materialization metadata. It measures candidates oldest first. In apply mode with live volume feedback it stops when the target is reached, and it rechecks recency after measuring each candidate before removal. When the platform cannot measure free space, apply uses the bounded estimate from allocated blocks. Dry run uses the same allocated-block estimate and leaves its candidates in place. Incremental and unit scans still measure all bytes needed for their footprint budgets.

## Verification

- The new targeted regression failed against the original collector because it reported no measured-run progress. It now verifies the measured and removed candidate, exact dry-run bytes for the fixture, protection of held and newest runs, and rejection of a run changed during measurement. Ajv validates the portable fixture, and the existing Python planner provides an independent removal oracle.
- The repository-wrapped direct collector suites passed 24 tests. One separate explicit-reclaim integration test was excluded on rerun after the repository's test fence refused access to the live checkout lease store. The cache command and generic removal suite passed all 19 tests.
- No live cache collection was launched during verification.

## Files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/⌨️command/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/📏️tree/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🌱️incremental/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🧱️units/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🧩️runs/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🧩️runs/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🧩️runs/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🧩️runs/🧪️tests/🧫️fixtures/🔣️.json`
