# Dependency Reader Current Source Audit

Source-only review; no runtime, physical build, network, or git mutation was performed. Production and test sources were not edited.

## Findings

1. **Shared consumer still invokes a Source writer.** `withJavascriptDependencyReadStorageV1` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts:71` calls `withPublishedBunWorkspaceOwnershipV1` after taking only the shared binding lease. Its actual CJS implementation calls `publishCurrentOwnership`, which writes an authored scope manifest when current ownership differs (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs:258`). A newly added/removed member or stale projection thus causes a compiler read to rewrite package.json before it discovers the expected generation has no ready receipt. Multiple shared consumers can enter that publication path concurrently and overwrite the same file. The inspector itself is read-only, but the complete reader is not. A nonpublishing current-ownership custody entry point should validate the already-published projection and refuse stale membership; publication belongs under exclusive installer custody.

2. **The actual installer reuse receipt bypasses the new finite descriptor reader.** `prepareJavascriptDependencies` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🟦️.ts:102` tests only `.isFile()` then uses `readFileSync(receipt,"utf8")` and `JSON.parse`. It does not enforce the 4 MiB receipt policy, one-link rule, retained descriptor identity, or bounded reads used by the new consumer. An oversized/tampered real generation receipt can allocate up to the partition bound, and a path replacement can change the object read between lstat and read. The previous-binding receipt in `bindJavascriptDependencyDirectoriesV1` checks size/link count but also reads by path after lstat, so growth/replacement can bypass its finite precheck. Reuse and predecessor validation should share the finite physical receipt primitive (and exact schema validation where applicable).

## Verified By Inspection

The reader uses the true store callback control, shared binding key matching the exclusive installer, an issued nested consumer phase that drains before final checks, exact expected-generation binding validation, retained finite receipt reads, source rechecks, physical link/root snapshots, final membership/store checks. Source input reads bound per-file and aggregate allocation, entries and package count. No additional deadlock defect was established in this bounded review.

Existing reader tests exercise neutral receipt DATA and source-shape assertions; they do not execute shared-reader overlap or the stale-membership writer path. No tests were run by this audit.
