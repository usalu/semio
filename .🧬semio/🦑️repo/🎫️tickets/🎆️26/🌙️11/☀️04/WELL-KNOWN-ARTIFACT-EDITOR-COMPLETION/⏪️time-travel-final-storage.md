# Time Travel Final Storage Construction

TimeTravelLedger and TimeTravelStoreState expose initial_copy_bytes and initialize_empty_at for staged native app construction. Both write each field directly to its caller-funded final address. The ledger initializes its document owner recursively, retaining the schema-specific mutation-label function and the existing clock. The store initializes the eight snapshot-alias slots and two issuer-close slots individually. Dormant authoring, replay, mutation, and preview payloads remain inactive.

Copy demand sums typed Option discriminant bounds for absent slots, full conservative extents for initialized live headers, and the nested document initializer's quote. Capacity and release demand for the enclosing app storage are preserved by the constructor's admitting owner. These ports do not allocate, move a complete ledger, or change caller grants. They require exclusive aligned uninitialized target storage, documented in native safety docstrings.

Validation: nightly Rust parsed both files successfully. The independent structural/default oracle is being prepared separately. Actual native construction and execution remain pending the staged app route and compiler queue. No completed opening is claimed.
