# Immutable SELECT Parent Lookup

The source-correct cold native closure scanned all 436 declarations and 271 direct members for every emitted qualified name. The actual reference official alignment run observes 5,805 TYPEOF calls and 36,613 qualified result names. Applied to that same call surface, the cold scan alone would require at least 25,885,391 work ticks, exceeding its default ten-million-work budget before other evaluation. This is a source-derived estimate from actual TS calls, not native execution.

Add a schema-derived immutable sorted direct-parent table beside existing borrowed metadata, with bounded stack normalization and binary lookup through the existing metadata interface. Keep the result Vec as the live closure queue and all original context/tick/make/push_value semantics. No request cache, parser, heap index or allocator warmup is needed. The original retained machine remains unchanged pending a separate coordinated optimization.

A source-parity test precedes the new table. The independent IfcOpenShell/NumPy graph oracle must verify every direct edge and constructor expectation. Native lookup/population tests are authored; rustfmt is syntax only and authentic compiler ingress remains required.
