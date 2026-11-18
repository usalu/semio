# Cargo cache collector crash

- Cause: `profileOf` in `unit-cache/gc/units/🟦️.ts` matched a unit path by `startsWith(profileDirectory + "/")`; on Windows `join` yields backslashes, so no profile matched and `.profileDirectory` of undefined crashed (dry-run totals and real grouping).
- Fix: profile derived from the unit path (`dirname` x3 = profile directory, exact map lookup, unknown profile directories described by their own path); totals keys are forward-slash relative paths. Test added; profile discovery test made separator-neutral. `bun test` in the units folder: 10 pass.
- After fix: rust 72.8 GiB vs quota 85.7 GiB, no excess: quota = demand (used / (1 - headroom)), pressure of 1.6 GiB fell on dependencies (2.9 GiB to evict, outside the requested areas).
- Units: 3072 of 3073 are held, because on Windows the compiler lock probe is unsupported and units used in the last 3 h (busyMinimumAgeMilliseconds) count as held; many agents compile now.
- Open: plain `repo cache gc` waits 15 s on `native-fixed-image:...\b\d` while holding `bazel-session-collector` exclusively, then refuses.
