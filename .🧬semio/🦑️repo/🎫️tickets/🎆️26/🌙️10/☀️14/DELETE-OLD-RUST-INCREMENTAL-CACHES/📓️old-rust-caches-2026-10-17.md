# Old Rust Caches On 17 October

The shared Cargo store had not produced a compilation unit for 70 hours. Live `rustc` processes did not have those cache files open. Every idle incremental crate and every intermediate unit directory was removed. Provenance logs and the mounted disk images that were being written were left in place.

## Removed

- 68 incremental crate directories, 5.58 GiB, all older than 2 hours
- 22 Cargo intermediate `build` directories, 26.78 GiB, holding the last unit of each package

## Volume

Available space on the data volume moved from 23 GiB to 54 GiB.

The fixed Rust image at `⚡️cache/rust/fixed` and the mounted images under `.cache/b` stayed. Those images were mounted and their files were being updated during this pass.
