# Fleet Cache Accounting Review

The recorded live fleet-state report charges 3580506112 bytes once to the native root and loose machine store. The separate charge probe excludes the 3400609792-byte obsolete undeclared alias and charges the current fleet-state measurement. These prove the measured case only.

Two accounting gaps remain:

1. With an existing fleet directory but no current fleet-state size record, quick status returns its usage as unknown. fleetStateBytesV1 converts null to zero; collection then subtracts zero from the loose quota and the host CAS size charge also receives zero. An old undeclared:native:f record is deliberately excluded. An initial, missing, or invalid size ledger can therefore make the protected state invisible to collection planning until a current measurement is recorded. Native root partial signals the missing measurement, but the machine plan treats it as zero. Require a current measurement or represent the unknown as a collection constraint, with tests for an empty ledger and an old-alias-only ledger.
2. Filtered status constructs the machine plan from the filtered report. The live fleet-state-only output reports all other stores and roots as zero and the machine as converged, despite not inspecting them. Calculate the machine plan from a full shallow report or label or omit it as incomplete for a filter.

The managed fleet control has no collect policy, and collection adds its recorded size as protected rather than a removal target. No live GC or source mutation was performed.
