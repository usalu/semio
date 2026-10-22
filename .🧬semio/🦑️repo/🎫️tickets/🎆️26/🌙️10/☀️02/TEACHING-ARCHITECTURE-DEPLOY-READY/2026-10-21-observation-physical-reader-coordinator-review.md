# Observation Physical Reader Coordinator Review

Prepared 2026-10-21T09:14:24.896Z. Static authored Source/schema review only; no actual Source/test/Native execution. Read the current full physical reader, header/block expansion and publisher functions plus the closed observation row schema. Actual facts kinds15 are represented by the closed row kind enum; schema preserves actual boolean/string/file-error/source-analysis fact families. No broad claim of all payload validity.

Concrete corrections sent to owner before final seal:

- Per-block16MiB packed/64MiB output,65536block roster and4Mi-row aggregate do not bound total allocation across many legal blocks. Add closed aggregate packed/output/row/node budget, debit before the next allocation/decompression, and make retained physical proof closures lean identity/ancestry data only. High accepted and is authoring neutral boundary cases before runtime update.
- Current synchronous open had O_RDONLY/O_NOFOLLOW without O_NONBLOCK; a regular-leaf-to-FIFO race could block before fstat and defeat cancellation. Use portable nonblocking open then actual descriptor validation.
- Current unconditional finally close can replace a prior physical/read/cancel failure. Retain the first actual primary with an explicit failed flag while closing.
- First block publication may create exact absent ob under actual writer, but must retain/recheck its admitted ancestor proof before creation/publication rather than treating recursive mkdir as authority. Owner must show actual lifetime proof or retain lean absence fence.

Typed missing referenced block remains strict partial-publication refusal. Known-incomplete group recovery is a separate real namespace invalidation followed by absent-header recompute; corruption/permissions/ancestry/cancel are not eligible absence. Final independent review and actual Native regression remain required.
