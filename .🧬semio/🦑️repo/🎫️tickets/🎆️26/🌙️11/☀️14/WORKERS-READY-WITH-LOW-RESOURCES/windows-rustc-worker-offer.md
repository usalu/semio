# Windows Rust Worker Offer

## Observations

- The authorized scheduler observation at approximately 15:17 UTC showed a fresh load report from g1-m05 with its worker absent under `no-toolchain`. Ready had adopted the workspace. The report identifies the sidecar gate. It does not include the child's probe output or the Windows sysroot.
- The sidecar probes the checkout before assigning `no-toolchain`. The detector previously accepted the compiler identity and sysroot, then looked for `bin/rustc` on every host. A private Windows-shaped MSVC and GNU sysroot each contained a real file named `bin/rustc.exe`. A Python compiler shim at that path successfully returned Windows CRLF identity text and the private sysroot in real child processes. The old lookup still returned no toolchain for both cases. It also offered the wrong bare name in the Windows case where only `bin/rustc` existed.

## Change and verification

- The worker uses the compiler-reported host triple to choose the executable name from the closed policy. Windows uses `rustc.exe` and POSIX uses `rustc`, including when a POSIX sysroot carries Windows target libraries. The normal sidecar still uses its checkout and existing compiler resolution. Probe timeouts, tool identity, fleet target properties and PATH handling stay the same.
- Four private fixture cases passed after the change. Both Windows toolchains offer the executable path, target properties and the exact identity digest checked independently with Python. POSIX offers the bare name and an incomplete Windows sysroot offers nothing. The closed policy also passed both existing schema validators.

## Limits

The private fixture runs on macOS and proves the filename failure independently of the scheduler observation. Its Windows-named compiler is an interpreted test shim. Live worker registration and native Windows execution were not exercised. The offload action still encodes `/bin/sh` as the command executable, so offering a compiler on Windows by itself supplies no evidence of a completed native Windows offload.
