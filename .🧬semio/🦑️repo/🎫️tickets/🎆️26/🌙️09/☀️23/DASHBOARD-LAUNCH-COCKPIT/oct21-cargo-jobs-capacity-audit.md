# Cargo Jobs and Host Memory Admission

Read-only source audit; queued71129/29672 were not changed or inspected through private state. No command, producer, cancellation or policy change occurred.

The outer owner is `repo/modules/library/build/host-capacity` (physical `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts`). `buildCapacityAskV1` reads Cargo `--parallelism` only before `--`; it does not read Cargo `--jobs`/`-j` or `CARGO_BUILD_JOBS`, and deliberately ignores `SEMIO_BUILD_PARALLEL` for Cargo. The existing neutral Cargo fixture asserts its own parallelism4 despite build-parallel9.

Actual `🏗️build/🔣️policy.json` declares Cargo tasks-shaped, build-kind, fixed1536MiB. Process `🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts:123` sizes minimumJobs1 and per-job2048MiB from build kind. Gate `🎫️gate/🟦️.ts:106` displays minimum demand fixed+minimumJobs×perJob, capped by the host budget. Thus3584MiB is the one-job minimum. It is not a fixed operation reservation at every concurrency level: higher admitted jobs reserve1536+2048×jobs. A future explicit `--parallelism=1` can cap preferred concurrency, but cannot reduce the3584 minimum or alter the independent physical memory floor.

Granted arguments rewrite only pre-separator `--parallelism`; Cargo tail is untouched. Repository Cargo launcher `📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts:152` requests exact compiler parallelism, creates that many jobserver tokens and sizes the rustc admission broker to the matching memory share. This lower compiler mechanism is distinct from the outer host reservation. No source evidence supports using Cargo jobs/environment flags to refine the outer request.

Conclusion: the current displayed minimum cannot be truthfully lowered through a supported jobs axis. This does not identify a holder, explain host pressure, or prove a liveness defect. Existing requests remain untouched.
