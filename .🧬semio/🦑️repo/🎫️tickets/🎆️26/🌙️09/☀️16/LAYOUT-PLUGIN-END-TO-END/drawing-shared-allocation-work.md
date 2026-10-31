# Fallible Shared Allocation Work

The pinned nightly-2026-07-20 standard library alloc/src/sync.rs exposes Arc::try_new_uninit behind allocator_api. The implementation requests the real Arc allocation without consuming the input. Successful initialization uses the unique MaybeUninit owner and assume_init; a real allocator refusal returns the exact input. No custom reference-count implementation or private Arc layout is introduced.

The schema and three genuine owning laws were authored before the implementation: each missing grant axis has zero work/allocation; a real null attempt costs one and preserves the original String pointer; retry uses the same finite two-unit stock; successful aligned/ZST allocation matches actual heap observations. Serde JSON supplies the value oracle. Tests are not yet run.

The preissued Source constructor remains a separately scoped fixed ownership-birth phase. This helper does not prove whole Source-constructor allocation failure handling.

## Actual Owning Gate

Canonical unfiltered allocation-return --lib completed with outer exit 0: 6 passed, 0 failed, 0 ignored, 0 filtered. The 10 observed backend owner inputs match pre/post hashes. The real allocator null test, original input pointer retention, exact successful allocation extents for aligned/ZST inputs, finite retry stock, and all three existing parent allocation-return laws passed. Broader Sealed/Source/Native consumers remain unverified.
