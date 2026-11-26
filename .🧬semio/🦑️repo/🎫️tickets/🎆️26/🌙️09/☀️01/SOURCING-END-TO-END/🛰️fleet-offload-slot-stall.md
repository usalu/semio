# 🛰️ Fleet Offload Held the Only Local Compile Slot for 3.5 Hours (2026-11-26)

## Observation
- `.cache/b/r/m/u/s/0.lock` (the single local compile slot of this 16 GB worker) was held by unit-cache pid 86667 for 3 h 31 min.
- That process was offloading `semio_s_artifact_stdio_semio` (from `cargo test -p semio-s-artifact-sourcing-curation --test curation_unit`) to fleet host `semio-g1-m08:6094` over an ESTABLISHED TCP connection that never answered.
- Every other local unit waited the full 10-minute slot timeout, so the sourcing hub rebuild advanced about 28 units in 1.5 h.

## Defects in the offload path (fleet mechanism, developed in parallel by peers)
1. A unit that is offloaded keeps the local compile slot for the whole remote round trip.
2. The remote round trip has no deadline: a silent fleet worker stalls the requester indefinitely.

## Action taken here
- Stopped the stuck run (its test target is re-run later); the slot moved on at once.
- Fallback to native tools as directed: `native-cargo.sh` now passes `repo cargo --offload=never`, so every sourcing build of this ticket compiles locally.
