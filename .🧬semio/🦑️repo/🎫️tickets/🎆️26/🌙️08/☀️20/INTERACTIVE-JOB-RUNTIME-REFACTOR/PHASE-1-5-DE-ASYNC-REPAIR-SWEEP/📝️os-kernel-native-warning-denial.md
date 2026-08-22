# OS Kernel Native Warning-Denial Sweep

## Scope

`semio-framework-os-kernel` and its direct leaf modules reached through the native plugin gate. Puzzle, FEM, renderer, Animate, framework-2d, and Phase 8 factory sources were not edited.

## Census

Initial kernel compile exposed 225 denied diagnostics, concentrated in direct OS leaves: public async contracts, duplicated `must_use`, large error transports, type-complexity, and mechanical modern-form lints. The compiler-proven fix pass applied 148 modernizations across the OS kernel closure.

`[DEBUG] os-kernel warning census: 0`

## Verification

* `cargo fmt -p semio-framework-os-kernel && cargo fmt --check -p semio-framework-os-kernel` — passed.
* `cargo clippy -p semio-framework-os-kernel --all-targets -- -D warnings` — passed.
* `cargo clippy -p semio-framework-plugin --all-targets -- -D warnings` now passes OS-kernel and stops in distinct unowned crates.

## Remaining Plugin-Gate Blockers

The exact plugin gate now fails after OS-kernel at:

* `semio-framework-schema`: public async trait contract lints plus local registry/value-form lints.
* `semio-framework-ui`: WGPU UI source and generated icon naming lints.

These sources are outside this OS-kernel ownership slice.
