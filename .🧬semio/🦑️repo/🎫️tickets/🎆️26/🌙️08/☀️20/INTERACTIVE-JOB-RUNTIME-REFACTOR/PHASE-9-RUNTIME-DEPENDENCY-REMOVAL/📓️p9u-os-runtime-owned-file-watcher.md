# P9u OS Runtime Owned File Watcher

## Outcome

The OS runtime has no direct `notify` dependency or source use. Store synchronization now consumes a repo-owned `OwnedFileChangeWatcher` from the existing native OS-services boundary. Filesystem probing is cancellable, bounded to eight directory entries per WorkerPool I/O-lane step, and handed back through a single-slot channel. The actor's UI/runtime turn only performs non-blocking `try_recv` polling and retains its existing 200 ms change debounce.

The dependency ratchet is green at 210 current third-party dependencies versus the 238 baseline; `rust:notify` is listed among the 28 removed dependencies.

## Census And Contract

The only runtime consumer was Store sync's non-WASM `ArtifactActor`: a `notify::RecommendedWatcher` watched the persisted artifact's containing directory non-recursively and sent only a generic wakeup. No event kind, path, cookie, or platform-specific metadata reached product behavior. The replacement therefore preserves the consumed contract:

- the first directory snapshot establishes a baseline without a false change;
- create, content/metadata modification, rename/replace, and delete alter the deterministic sorted snapshot;
- one probe is in flight and one completion slot exists at any time;
- dropped watchers set an atomic cancellation flag observed before every bounded step and completion;
- probe errors release the in-flight state and retry on a later poll;
- Store coalesces detected changes through the existing 200 ms deadline before re-reading persistence.

The implementation uses only `std::fs`, `std::path`, and `std::time`, so the same code applies to macOS, Linux, and Windows. It is exported only by the native OS-services crate. The OS kernel's services dependency remains under `cfg(not(target_arch = "wasm32"))`, and Store's WASM actor has no filesystem watcher, producing a clean unsupported/no-op boundary rather than browser emulation.

## Files

- Added `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️file_watcher.rs`: structured Snapshot, Probe, and Watcher regions plus golden behavior/cancellation test.
- Updated `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust/📦️glue.rs`: internal module wiring and re-export.
- Updated `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs`: injected WorkerPool ownership, non-blocking watcher polling, existing debounce integration, and removal of the `notify` adapter/channel.
- Updated `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml`: removed `notify`; assigned the already-existing optional OS-services edge to the native `sync` feature.
- Updated `Cargo.lock`: removed the now-unreachable `notify` package graph.

## Verification

All commands used the ticket-local `🧪️target-p9u-notify` target where applicable.

| Gate | Exit | Evidence |
| --- | ---: | --- |
| Focused watcher golden test | 0; 1 passed | `📝️p9u-notify-watcher-tests-2.txt` |
| Services Nx quick suite | 0; 34 passed | `📝️p9u-notify-services-nx-quick-1.txt` |
| OS kernel native debug, `--features sync` | 0 | `📝️p9u-notify-native-check-3.txt` |
| OS kernel native release, `--features sync` | 0 | `📝️p9u-notify-native-release-check-2.txt` |
| OS kernel WASM, `--features sync --target wasm32-unknown-unknown` | 0 | `📝️p9u-notify-wasm-check-2.txt` |
| Scoped rustfmt and zero direct `notify` census | 0 | `📝️p9u-notify-static-and-fmt-1.txt` |
| Bun dependency ratchet | 0; 210/238 | `📝️p9u-notify-dependency-ratchet-1.txt` |

The earlier attempt to compile Store's full `cfg(test)` surface stopped on 58 pre-existing Store async-trait/stale-await test diagnostics, recorded in `📝️p9u-notify-native-test-1.txt`. The watcher was moved to OS services specifically so its contract could be compiled and executed independently; both its focused test and the complete 34-test services quick suite are green. Native debug/release and WASM production builds are green. Remaining compiler output consists of existing warnings outside this packet; there are no errors or packet-local blockers.
