# Current Border Change Audit

- Absolute Windows UTF-16 path lengths in the new border-presentation directory peak at 93 code units; ticket inputs and reports peak at 114, excluding generated output. Both are below 256. This scoped check does not measure every existing modified path.
- No tracked Cargo.toml, package.json or AGENTS.md change was found under the UI module. The React project test target adds dependencies on existing styling-token and asset generation targets; it does not add a runtime library. Ticket restoration inputs restore-assets.ts and restore-styling.ts remain present.
- PendingBorder allocates an uninitialized usize slice sized to the current contributor count. Each pending step writes one distinct slot; the sweep does not mutate the contributor set while pending. Finish asserts the initialized count equals the allocation length before assume_init. Cancellation drops MaybeUninit backing without reading it. On this source review the initialization and ownership invariant holds. Compiler and tests remain required.

This is a scoped static audit, not a platform execution test.
