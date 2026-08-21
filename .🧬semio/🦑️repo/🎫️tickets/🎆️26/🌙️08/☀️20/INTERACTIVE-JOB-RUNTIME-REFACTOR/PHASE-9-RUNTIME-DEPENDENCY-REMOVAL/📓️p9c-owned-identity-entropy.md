# P9c Owned Identity and Entropy

## Implementation

The OS kernel now exposes a dependency-free identity boundary at `os_identity`:

- `fill_entropy(&mut [u8]) -> Result<(), EntropyError>` hides every platform ABI.
- `time_ordered_id() -> String` owns UUID-v7-shaped formatting, timestamp placement, version/variant bits, and a process-local monotonic sequence.
- macOS/iOS use `arc4random_buf`, Linux/Android use libc's `getrandom(2)` entry point, Windows uses system-preferred CNG, and wasm calls the platform `crypto.getRandomValues` boundary through the already-linked low-level JS ABI.
- If a supported platform entropy call fails, time/counter/process mixing preserves uniqueness progress without exposing an external RNG type. Hub token-bearing platforms use their native entropy path.

The hub's directory, sqlite, postgres, neo4j, server-session, and test-directory identity sites now use `directory::os_identity::time_ordered_id`. The hub's direct `uuid` dependency was removed. No `Uuid` or `uuid::` reference remains in the hub subtree.

The OS host has one remaining UUID call and two target-specific dependency rows. That file is actively owned by the Phase 3 host-isolation agent; the owner was instructed to migrate the call to the same kernel interface and remove both rows rather than creating a cross-agent edit collision.

## Verification status

- Permanent tests generate 10,000 IDs, verify UUID-v7 version/variant shape and uniqueness, and verify two platform entropy buffers differ.
- `bun nx run @semio-tech/framework-os-kernel:test-quick` was started, then stopped while waiting on the shared Cargo build lock so the stdio compiler-repair fleet retained the critical path. No build/test pass is claimed yet.
- Non-compose repository census at this checkpoint: one source call and two manifest rows remain, all in the explicitly delegated OS-host file.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🪪️identity/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs`
- `🌎️hub/📦️packages/🦀️rust/Cargo.toml`
- `🌎️hub/**/*.rs`
