# L2 Rust 5D and Hub lane: blocked by full disk

Status: no pass/fail counts obtained. No source failure was observed; every attempt stopped at ENOSPC.

## Root blocker
- `/home` (nvme0n1p8, 465G) is 100% full: 2.3G free (608M before I deleted my own first attempt).
- Largest consumers are old ticket build dirs: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REMODEL-PLUGIN-END-TO-END/🗑️generated` 137G (modified within the last day, so left alone), `🎆️26/🌙️11/☀️18/DEV-PUZZLE-5D-FULLY-WORKING-END-TO-END/🗑️generated` 57G (`w/b` 27G wasm cargo build dir, `native-target/b` 27G), `🎆️26/🌙️11/☀️04/GET-PUZZLE-ARTIFACTS-WORKING-END-TO-END/🗑️generated` 28G, repo `.cache/b` 70G.
- My `rm -rf .../DEV-PUZZLE-5D.../🗑️generated/w/b` (stale wasm build intermediates) was denied by the permission classifier, and I did not work around it. Deleting it (or other finished tickets' generated dirs, per CLAUDE.md "delete generated when done") frees enough for the hub build.

## What I ran
Script `l2-native.sh` (this folder): private mount namespace, native cargo with the precedent toolchain/flags, `--jobs 3 --offline`, build dir on tmpfs (6g) with an overlayfs whose read-only lower is the prior ticket `native-target/b` cache (works in a user namespace with `userxattr`).
- Attempt 1: tmpfs-only build dir, `cargo test -p semio-hub-puzzle --lib` from `🌎️hub`: 6G tmpfs full during the ~1100-crate hub graph (wasm-bindgen-macro-support rlib: ENOSPC).
- Attempt 2: overlay on the cached lower: reused part of the graph (451 crates rebuilt in ~15 min because upstream sources changed), then ENOSPC again on tmpfs upper at 5d `mut-any-3`, `mut-any-part2d`, 2d `mut-any-4`. Log: `🗑️generated/l2-hub-test.log`. The only errors in it are ENOSPC; zero real compile errors seen up to that point.
- 5d `check`/`build`/`test`/`verify-*` targets, command crates (`🎮️commands/📤️jobs`, `🧵️retained`) and `🏭️bridge` were not reached.
- `repo run` was not attempted: prior tickets document the graph/resident service stuck queued, and memory (15G, ~0 free during builds, other lanes compiling) did not allow a parallel try.

## Source fixes
None.

## To finish (needs the coordinator or user)
1. Free >=15G on /home (for instance the finished DEV-PUZZLE-5D `🗑️generated/w/b`, 27G), or grant a bigger tmpfs.
2. Then run from this folder, in this order (one namespace so caches are shared), e.g.:
   `./l2-native.sh x 'cd 🌎️hub && cargo test -p semio-hub-puzzle --lib --jobs 3 --offline'` and
   `./l2-native.sh x 'cd "$P5" && cargo test --jobs 3 --offline --locked'` (the `$P5` variable is the 5d rust package dir).
   Set `L2_SIZE` and swap the overlay for a disk dir once space exists.
