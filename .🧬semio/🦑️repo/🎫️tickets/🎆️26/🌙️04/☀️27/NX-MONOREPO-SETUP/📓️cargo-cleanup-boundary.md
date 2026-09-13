# Cargo Cleanup Boundary

## 2026-09-13

The native Cargo router previously wrote a shared hourly throttle stamp and launched a detached cache-prune process after component builds and ordinary build/check/test commands. This was hidden orchestration outside Nx; the build could report completion while cleanup continued without its task owner.

That function, throttle stamp producer and its four invocation paths have been removed. Explicit repo:cache-prune remains uncached and editor-accessible. Native compilation, artifact capture and publication are unchanged.

The schema-first regression fixture executes the actual native Cargo script in a private workspace through Nx. Its cleanup entry is replaced only inside that fixture with a marker-writing probe, so the red test cannot prune any real cache. The red build passed native compilation and then failed after 8.1 seconds because it created the cleanup stamp. After removal, build/check/test all passed through native Nx in 13.8 seconds, retained the declared artifact receipt, matched native Cargo metadata and created neither a cleanup stamp nor a detached cleanup marker. Successful fixture output was removed.

The explicit pruner still does not hold active producer/reader leases for all Cargo, Vite and agent directories. Its exclusive prune lease and file ages are insufficient to establish safety against active builds. The misleading concurrency claim was corrected in its current orchestration module. No shared cache was pruned during this work. Full retention qualification remains required.

The pruning command module was concurrently moved from the top-level caching script into 🧹️pruning/📋️orchestration/🟦️.ts; that move was preserved. The new regression is registered in the complete cache-contract suite.
