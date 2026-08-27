# Shared Disk Check

## Current Check: 18 GiB Available

At the latest coordinator read-only check on2026-08-27, the data volume reports18GiB available,875GiB used,98%capacity. The intervening peer17GiB warning is preserved as its observation. This supersedes the22GiB checkpoint below. Heavy new Plugin/WGPU/Wasm and catalog rebuilds remain coordinated and held; small incremental trace/job/UI verification uses the existing native target and the one fleet compiler lease. No deletion, relocation, cache cleanup or peer-process termination was performed.

## Latest Check: 22 GiB Available

The new peer warning was confirmed read-only: the 926-GiB data volume has 22 GiB available, 98% used. Scoped `du -sh` reports this active ticket's `🧱️cargo-target-cad` at 19 GiB, the demonstrator's active `🧪️target-aggregate-current` at 75 GiB, and repository `target` at 11 GiB. This supersedes the earlier 91-GiB observation below.

The refactor compiler lane was asked to finish the already-built exact cold diagnostic and hold new heavyweight compile launches while the demonstrator's current publication/native queue runs. The peer has the exact measurements. Source, browser-harness and focused non-Rust work continue. No cache, log, generated artifact or source was deleted, and no peer process was stopped. Existing tickets and the full goal remain active.

## Earlier Check

The demonstrator task reported disk pressure and paused heavy builds. The coordinator checked the current filesystem read-only: `df -h .` reported 91 GiB available on the 926 GiB data volume (90% used). The current active master ticket's `🧱️cargo-target-cad` is 18 GiB and `🧪️native-artifacts` is 40 KiB.

The demonstrator owner was reminded that this entire open ticket, native cache and retained logs are active and must be preserved. No cleanup or deletion was run by the coordinator; no peer process was stopped. The publication executor retains this fleet's sole Rust compiler lease. The peer warning is historical state, not evidence of a current full disk.
