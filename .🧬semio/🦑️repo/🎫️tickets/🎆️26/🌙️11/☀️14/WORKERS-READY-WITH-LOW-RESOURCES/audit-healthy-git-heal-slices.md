# Periodic Healthy Git Heal Slices

The serialized replica turn did a complete `manifest.ids()` snapshot, decoded every live path and awaited serial local filesystem probes inside `watchGit` before ordinary publication or follow. The sampled coordinator PID 31890 stayed alive with Git activity, a health record seventeen minutes stale, and native stacks including `lstat` and `openat`. The stripped runtime did not identify the source routine or prove the cause of that live stall.

`🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🟦️.ts` now caps healthy periodic heal work at 2,048 visited IDs or the existing 10 ms sweep quantum, measured between filesystem operations. It captures the existing book ID ceiling once and resumes in ascending order on the next ordinary turn, filtering queued and non-put entries before path decoding. There is no full ID snapshot for periodic healthy scans. The fast stage-index check keeps its earlier position. Each bounded healthy slice stays before normal publication and follow, so any discovered damage enters the same queue in its current turn. A completed healthy sweep reports the aggregate and requests a full refresh once. The full immediate path still handles Git corruption, a missing index and overdue stage objects.

The heal keeps only one local filesystem operation active at a time, awaits it, and rechecks cancellation after the probe and after any small-ref hash before changing replica state. The budget is cooperative. A single stalled kernel filesystem operation can exceed it. No state-mutating repair continues outside the issuing turn.

The actual private daemon regression delayed twelve shared Git leaf probes. The red run showed zero normal turn or publication events between the first and last probe. The corrected test verifies ordered first-pass probes with a normal journal turn between them and independently confirms the ordinary path's exact journal event. SQLite evaluates the trace. A separate deferred-lstat case verifies cancellation leaves both the applied file record and pending queue unchanged. The full current owner file also exercised urgent daemon recovery and physical pack ordering. Session 70565 recorded 7 passes and 38 assertions. Captures are `🗑️generated/healthy-git-heal-red.log` and `🗑️generated/healthy-git-heal-final.log`.

```sh
repo tools exec --light bun -- test './🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests/🩹️heal.ts'
```
