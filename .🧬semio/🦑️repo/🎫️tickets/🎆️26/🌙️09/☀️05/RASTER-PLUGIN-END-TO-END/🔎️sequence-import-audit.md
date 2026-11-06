# Sequence Original Import Audit

The phase 1 parser can finish and place its output in self.value, then fail admission of the separate phase-2 transition. On retry phase 1 calls parser.step again instead of first checking the already-owned output. Add the same retained-output transition guard used in phases 3, 5 and 6, with a denial-after-parser-completion law.

Phase 7 synchronously projects child content, materializes a working scene, scans all steps and edges, clones both vectors, builds the edit and child preparation, and takes owned params before charging one fuel unit. These are input-sized operations and retained allocations without caller grant. A complete repair needs staged original-owned assembly with honest CPU, allocation and retirement frontiers; a fixed header demand cannot quote arbitrary scene size.

Child preparation calls prepare_child_one, which discards prepare_child_original work, then charges one fuel on Pending and none on Ready, refusal or error. The original API records actual delegate work even on failure. Consume that work for every result and preserve failure retained effects, checking whether Fault transports them before designing the failure path.

max_serial_in_snapshot returns u64, but phase 7 adds one unchecked after max(100). A valid step or edge ID with suffix 18446744073709551615 triggers checked-build overflow or unchecked wrap. Use checked addition and typed refusal with a boundary fixture. Native compilation and runtime tests remain unverified.
