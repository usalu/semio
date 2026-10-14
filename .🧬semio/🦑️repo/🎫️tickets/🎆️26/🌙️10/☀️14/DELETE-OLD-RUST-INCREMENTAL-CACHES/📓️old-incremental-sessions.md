# Old Rust Incremental Sessions

Shared Cargo stores under `.🧬semio/🦑️repo/⚡️cache/cargo` (`build`, `target`, `canonical-architecture`). No `rustc` or `cargo` process was running. Selection matches `compactCargoIncrementalSessionsV1`: for each crate directory, every finalized `s-*` session except the newest is old. Crates that still contain a `-working` session were left untouched.

## Removed

- 262 superseded session directories
- 262 sibling `s-<timestamp>-<id>.lock` files
- Session contents measured 17.36 GiB before deletion

Volume available space moved from 14.46 GiB to 28.09 GiB (13.63 GiB returned). The gap versus 17.36 GiB is space that was still shared with retained files.

## Kept

- Newest finalized session of every other crate (1038 crate directories)
- 40 crate directories that still have an in-progress `-working` session, including their older sessions

After deletion a rescan reported zero superseded sessions left in those three stores.

## Elsewhere

`temp`, `.cache`, `extension-host-build-7a5OFB`, `~/.cargo`, `~/.cache`, and `~/Library/Caches` contain no `incremental` directories. Compiled artifacts beside these caches (`…/build`, `…/deps`, uplifted `target` files) were not removed.
