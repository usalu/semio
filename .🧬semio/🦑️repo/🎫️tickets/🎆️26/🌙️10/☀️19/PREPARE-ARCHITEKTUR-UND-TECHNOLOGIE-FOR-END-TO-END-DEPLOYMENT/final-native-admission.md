# Final Native Admission Verification

After the Rust companion and manifest public-export source repairs, the actual canonical command `bun repo preflight --scope @teaching/architecture-course` was run on the current Windows checkout. It exited with code **1**, preserved by the invoking PowerShell command. Bun reported version 1.4.2 (Windows x64).

The current source route was inspected before executing it: `runOwnedBuildInvocationV1` dispatches preflight and returns before opening repository cache partitions. `runBuildPreflightV1` reads the authored requirements and actual process/filesystem/tool facts, then refuses before allocation when requirements are unmet. This route has no provisioning branch.

The exact observed requirements were Linux x64, Debian 12, 77,309,411,328 available bytes (72 GiB), seven writable unused loop devices, writable loop-control, and `fallocate`, `mkfs.ext4`, `mount`, `umount`, `losetup`, `cc`, `pkg-config`, and `docker`.

The exact observed facts were `win32`, `x64`, empty distribution and distribution version, `capEff: "0"`, **71,246,671,872** available bytes, `loopControl: false`, `freeLoops: 0`, and only `docker` from the required executable set. The command refused for the OS/distribution, missing CAP_SYS_ADMIN, insufficient available bytes, unavailable loop-control/devices, and the seven missing executables. Architecture matched.

`SEMIO_TICKET_DIR`, `TEMP`, `TMP`, and `TMPDIR` were set to the existing ticket and its `🗑️generated/tmp` directory. The actual combined command output was written to `🗑️generated/current-preflight-final.log`. No environment credentials were printed, no host/provider policy was changed, no store was allocated, and no compiler build was attempted. The output log is temporary and belongs to final ticket cleanup; this report preserves the result.

This verifies the current canonical admission refusal after the source repairs. It does not establish a native build, image, Compose deployment, browser rehearsal or end-to-end release pass. The admitted Linux Debian 12 runner prerequisite remains unresolved.
