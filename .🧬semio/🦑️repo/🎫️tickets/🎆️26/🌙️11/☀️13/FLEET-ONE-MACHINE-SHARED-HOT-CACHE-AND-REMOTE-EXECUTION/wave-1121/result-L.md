# Package L: lease ordering and registry hygiene

## Violations
1. `repo:compiler-writer` (image, 7) then `cargo:configuration-publication` (2): already fixed in source (`withRepositoryCargoConfigurationPublicationV1` takes no lease; content-addressed atomic rename). Records were from 2026-11-12. Pinned by a new policy test (still refused by rank, so a regression would show).
2. `cargo:registry:` (2) then `cargo:configuration-view:N` (1): already fixed in source (`withRepositoryCargoConfigurationViewsV1` opens views before the registry reader; multi-step consumers reuse the held view). Pinned by the same test.
3. `native-fixed-image:<cache>/b/t` (7) then `nx-tooling-provision` / `nx-sdk-snapshot:` (2): still produced by the tools store scope (`withRepositoryToolingStoreV1` opens the image before `withManagedToolingProvisionV1`, `planNxToolingStorageV1`, derivative readers). Moving those leases above the image breaks the daemon start order (snapshot before graph, rank 4), so the ranks stay and the nesting is declared: edges `image -> tool-singleton` and `image -> bazel-picker` with en+de reasons. The cycle detector stays enforced.

Order enforcement left `observe`: edge 3 is declared but not proven by a live run, many order and startup test files fail here for environment or source drift reasons independent of this change, and one unobserved site flipped to `enforce` would throw in production builds.

## Registry hygiene
Owner: the registry itself (`🗂️registry/🟦️.ts`). First publication of a process schedules one unref'd timer (`registry.prune.delayMilliseconds`); it runs `pruneLeaseRegistryIfDueV1`: stamp `prune.json` claimed first (interval 10 min, 20 s while a backlog remains), at most 256 folders and 20 ms per pass with a cursor, only folders with no live row whose directory and parent are gone (or gone and idle 24 h), plus dead rows. `SEMIO_LEASE_REGISTRY_PRUNE=off` disables. Doctor repair untouched.

Tests: `🧭️order/🧪️tests/🧹️prune/🟦️.ts` (5, Python oracle for survivors) and the new policy test in `🧭️order/🧪️tests/🟦️.ts`.
