# Trace one-shot allocation refusal

RequestedAllocator now supports one explicitly scoped current-thread refusal of alloc, alloc_zeroed, or realloc. It preserves requested-byte accounting, leaves original backing intact after refused realloc, and resets the flag on unwind without installing a second global allocator.

Focused native run 70082 passed all three selected tests, including cross-thread isolation and unwind restoration. Independent SQLite fixture run 28404 passed one test with three assertions. These tests establish the allocator seam; they do not establish the JSON writer or native allocation migration.
