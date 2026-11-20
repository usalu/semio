# Cargo Offload and Local Compiler Capacity

Shooting builds entered the host-capacity offload branch when the fleet endpoint was published and the host pressure probe said to wait. The branch rewrote the command with the fleet fanout ceiling as `--parallelism=10`. The Cargo launcher reads an explicit parallelism option as an exact local compiler request. With one local slot free, this rewrite made the build wait for ten slots before it could start.

The early offload branch now forwards the original arguments to the Cargo launcher. Omitting the option retains the compiler planner, and an explicit value remains the caller’s request. Cargo’s existing lease establishes the local jobserver width, while individual units retain their fleet routing. The diagnostic reports this separation in English and German.

Two regression cases exercise the real host-capacity branch with a busy-host probe and published fleet endpoint, then pass the resulting arguments through the actual Cargo launcher parser. Both reproduced the incorrect exact-ten request before the fix. After the fix, the four cargo-scaling tests passed, including the existing local-only cases. The targeted fleet readiness test also passed. These are local protocol tests and do not submit work to a live fleet.
