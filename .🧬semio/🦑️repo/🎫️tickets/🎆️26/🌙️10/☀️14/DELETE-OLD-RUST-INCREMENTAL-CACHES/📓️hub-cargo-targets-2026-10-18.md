# Hub Cargo Targets On 18 October

No `rustc` or `cargo` process was running. The shared Cargo store under `⚡️cache/cargo` was already down to 0.26 GiB. The large Rust outputs were private Cargo target directories inside hub sandboxes, plus the preallocated Rust disk image.

## Removed

- `🌐hub/s14-c12-overlay-order/.c12-build` — 16.83 GiB
- `🌐hub/s14-c13-overlay/.c13-build` — 15.91 GiB
- `🌐hub/s15-ex1-overlay/.ex1-build` — 17.92 GiB
- `🌐hub/s15-rs1-overlay/.rs1-build` — 6.98 GiB
- 18 smaller Cargo targets under framework, tickets, and hub scratches — 4.18 GiB
- Unmounted and deleted `⚡️cache/rust/fixed/fixed.dmg` — 16 GiB image

## Volume

Available space on the data volume moved from 14 GiB to 53 GiB.

Sandbox checkouts around those targets were left in place. The mounted images under `.cache/b` hold Nx, Bazel, and tool caches rather than Cargo target directories.


## Nested artifacts

The overlay size scan showed each large sandbox is a checkout plus one Cargo target. After those targets were removed, 30 nested `cargo-artifacts-*` and `cargo-plugin-host` directories inside `s14-c12-overlay-order` and `s14-c13-overlay` were removed as well: 5.84 GiB. Available space on the data volume is 55 GiB.
