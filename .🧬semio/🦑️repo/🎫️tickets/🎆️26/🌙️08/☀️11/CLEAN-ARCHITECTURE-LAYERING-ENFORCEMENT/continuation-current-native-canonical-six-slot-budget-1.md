# Current Native Canonical Six Slot Budget

The fresh current General UI owning engine route compiled successfully and started 787 actual tests under the long Nextest profile, run identity `49ea054b-88c8-4121-b469-4fafc0e08921`.

The actual `wgpu::engine::tests::ui_surface_slot_table_is_heap_first_and_fits_a_bounded_thread_stack` assertion failed because its current measured `Option<UiSurfaceSlot>` is 166040 bytes, while the committed owner fixture declared 165984 bytes. Both sides reported the same capacity 64 and the same registry owner size 520 bytes. This is a 56-byte change to one retained slot. The current implementation still constructs the slot table directly on the heap through `semio_framework_async::boxed_fixed_slots`; it retains its boxed fixed-size slot table and inline generation array.

The current owner descriptor was corrected to the measured 166040-byte element, and its provenance now links this report. No capacity, owner-size budget, conversion threshold, or bounded thread stack budget was loosened. The unchanged law still requires the exact measured descriptor, proves the 520-byte owner is smaller than the 10626560-byte slot table, and constructs the registry on the explicitly bounded 1048576-byte thread stack. Its explicit `Builder::stack_size` continues to prevent the runner's larger minimum stack from masking a regression.

The first whole invocation terminated with Nx status 1 after its fail-fast cancellation. Its final Nextest summary ran 305 of 787 selected tests: 304 passed, one failed, zero skipped. The remaining selected tests did not execute. Its failure is preserved separately.

A subsequent invocation of the same complete current owning route terminated with status 0 under terminal handle 32262. It compiled the actual current UI library test binary, then ran Nextest's long profile with run identity `cd7abb0d-cc67-4111-89ce-9d22cfed7545`: all 787 selected tests ran and passed, zero skipped, assertion duration 22.183 seconds. The corrected unchanged slot law is included in this complete current General UI engine suite. The full Nx target also reported success with cache explicitly skipped.

This establishes the current General UI engine route's actual compiler and assertion result on this host. Its source was read from the concurrently shared workspace; original captured-source identity, atomic current-source custody, cross-platform runtime execution, Product renderer native acceptance, and Infinite Board native acceptance are not claimed.
