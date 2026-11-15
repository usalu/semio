# Memory Pressure Interpretation

The round monitor reads `kern.memorystatus_vm_pressure_level`. Apple’s sysctl handler converts the internal pressure level to dispatch notification flags before returning it, so a returned value of 1 means normal, while warning and critical are higher flags. The local SDK also defines the normal dispatch flag as `0x01`.

Primary implementation: [Apple XNU sysctl handler and conversion](https://github.com/apple-oss-distributions/xnu/blob/main/bsd/kern/kern_memorystatus_notify.c#L1754), [normal-level conversion](https://github.com/apple-oss-distributions/xnu/blob/main/bsd/kern/kern_memorystatus_notify.c#L1652).

The timestamped raw local cross-check, including `memory_pressure -Q` and SDK definitions, is in `round-1-memory-pressure-check.txt`. This check changed no system setting and simulated no memory pressure.
