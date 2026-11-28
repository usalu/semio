# H81: unit-cache misses `differs in env:CARGO_ENCODED_RUSTFLAGS`

## Cause (measured in this Mac's lookups.jsonl, 4021 rows)
- Not H6, H49 or H69: their flags are appended after the key (`-Z threads=N`, offload thread rewrite, worker rust-src remap).
- (1) `CARGO_ENCODED_RUSTFLAGS` was a key input of every compile unit, but Cargo only gives it to build scripts; it sits in a rustc's environment only when the launcher (`repo cargo` ingress view) set it. Present in 64 compile rows, absent in 3901: present vs absent is a miss. Its value also varied: `-Z threads=0` doubled (124), empty (6), wasm link args (legit).
- (2) `-Z threads=0` (twice) in the key arguments: absent for 73 of 642 crates (e.g. `semio_framework_value_native_*`), depending on the launch path; threads never changes the output (shared units compile with one thread).

## Fix (landed 13:00)
- Compile units: `CARGO_ENCODED_RUSTFLAGS` removed from `key.environmentNames` (their flags are their arguments). Build script runs keep it, with machine paths neutralised and `-Z threads` removed.
- `-Z threads=N` (both spellings, any count) removed from the normalized key arguments.
- Law `🧪️tests/🚩️flags`: two simulated hosts (flags in env + calm vs none + memory pressure, other home/checkout/sysroot incl. rust-src remap) give one key; thread count never splits; real flags (`-C target-cpu`, link args) still do.

## Cost / unverified
- Keys of units that carried the threads flag change once (most of the fleet's compile keys): one miss wave, then identical on all machines. Not measured on workers yet; check `miss ... differs in` lines on m03 after workers pick up the replica (~30 min).
- Pre-existing failures untouched: main unit-cache test CPU-load flake, fleet count/counters, retention-io, admission typecheck error.
