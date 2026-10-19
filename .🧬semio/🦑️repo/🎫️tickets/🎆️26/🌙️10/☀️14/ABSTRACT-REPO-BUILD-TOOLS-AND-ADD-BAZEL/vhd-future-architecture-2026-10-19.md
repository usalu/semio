# Future Storage Architecture — 2026-10-19

Recommendation: remove mandatory disk images and use ordinary directories as the single repository storage implementation. Do not retain VHD/DMG/loop images as an alternate backend. This is the clean long-term choice for zero-touch development on native Windows, macOS, Linux and devcontainers. The present mounted-volume design buys strict physical capacity at the cost of privileged host requirements and a large image ownership/publication lifecycle. Its guarantees should be replaced with explicit directory accounting semantics, not preserved by naming conventions.

## Ownership and State

Separate durable local coordination/state from reconstructible cache in the schema and taxonomy. Durable state owns lease/queue databases, journals, allocation recovery records and any non-reconstructible operational facts. Cache owns downloaded and verified tools/dependencies, graph snapshots, task results and compiler intermediate products, with explicit reconstruction recipes and retention rules. Compiler working directories and in-progress downloads/publications are temporary owned workspaces, not freely evictable entries. Published outputs that other commands still consume require pins or references; their location alone does not authorize deletion.

Keep state outside cache eviction roots. An invocation acquires ownership through a repository-controlled directory owner, receives a bounded lifetime and cancellation/progress control, and closes only after all accepted work settles. Maintenance requires exclusive custody; it must not infer quiescence from absent processes or age alone. Crash recovery reconciles journals and incomplete publications before admitting new conflicting work.

## One Tool-Neutral Directory Contract

Author the schema first around root identity, entry identity/digests, reader pins, writer ownership, reservations, atomic publication, retention, recovery, progress and cancellation. Expose a repository-defined directory owner interface with operations that issue these capabilities. Nx, Bazel, Cargo, dependency bootstrap and tools consume this interface. Avoid image fields, mounted-device proofs and platform backend types in public contracts. Platform filesystem primitives remain internal implementation details where needed for correct locking, syncing and identity checks.

Reservations and inventory enforce cooperative repository policy. Use aggregate and per-domain logical budgets, bounded capture/temporary output, known subprocess destinations, preflight free-space checks, periodic accounting and cancellation when repository-observable limits are exceeded. Distinguish logical payload bytes from observed allocated bytes and host free space. State plainly that these controls do not impose an OS hard filesystem quota or prevent an uncontrolled child from consuming additional host disk between observations. A directory is not a sandbox. Keep existing useful kernel coordination and containment checks, but do not claim that filesystem-device equality separates sibling directories.

Publication should write to an owned staging directory, verify contents and metadata, sync through supported filesystem primitives, then rename into an immutable generation namespace under an exclusive writer. Readers pin a published generation. Eviction removes only unpinned, classified reconstructible entries; state and uncertain ownership are retained for recovery. Define and test actual cross-platform crash guarantees rather than transplanting volume-specific flush receipts.

## Remove the Image Architecture Completely

Replace the authored schemas, fixtures, store constructors and consumers together. Remove fixed-image creation/mounting, host-token volume privilege gates, OS image observers, image-specific publication/retirement, nested-image funding, backing receipts and cleanup exemptions. Replace tests of image mechanics with language-agnostic directory ownership, recovery, reservation, integrity, eviction and cancellation cases plus independent library oracles where appropriate. Update executable launch entries and project script routes with the implementation. No legacy backend, migration script or compatibility fallback is needed in this greenfield repository.

Keeping disk images as an alternate backend would retain almost all of the privileged setup, platform maintenance, proof composition, nested ownership and test matrix while making capacity behavior vary across machines. There is no demonstrated requirement in the inspected source that justifies this continuing burden once strict per-partition OS capacity is intentionally dropped. Keep the new interface narrow enough to avoid exposing the storage mechanism, but do not build a speculative pluggable backend system.

## Enforced Workers Are a Separate Future Requirement

Do not add an optional VM/container worker as part of this replacement. If a measured workload later needs genuinely enforced CPU, memory, disk or adversarial-process isolation, define that worker requirement separately and choose an environment whose actual host support satisfies it. Such a worker is an execution boundary with validated guarantees, lifecycle and cancellation, not an alternate cache format. It must not silently become a prerequisite for ordinary local development or be advertised as universally zero-touch before proving that on every supported host.

## Evidence and Limits

This recommendation builds on the source findings recorded in `vhd-removal-audit-2026-10-19.md`: repository partition budget/opening contracts, fixed native storage contract, Windows privilege gate, compiler reservation recovery, digest-addressed graph publication and Bazel store integration. It is a design recommendation only. No source implementation, runtime verification, tests or storage changes were performed.
