# Kernel Test Target Attribution Correction

The actual R1 command targeted @semio-tech/framework-os-kernel:test. Its30 Store/SPR fixture compiler errors are genuine for that OS-kernel test binary and remain mutation-owned, but this command selected the wrong crate for the newly authored ui_turn_patch_owner_ laws. It did not establish that those laws were blocked by their dependencies.

Read-only source chain:

- framework/🔨️modules/🎠️kernel/🦀️component.rs contains both laws at the captured1559/1579 positions.
- framework/🔨️modules/🛂️manifest/🦀️component.rs4143/4144 includes that exact source as pub mod kernel.
- framework/📦️packages/🦀️rust/📦️glue.rs mounts/reexports manifest; its Cargo package is semio-framework.
- The separate OS-kernel glue does not mount this source. Compiling OS-kernel cfg(test) fixtures is not the same as compiling it as a library dependency.

The existing pure native router is @semio-tech/framework-rs:test-wire-retirement-native with --args='--lib ui_turn_patch_owner_ -- --nocapture'. It calls runCargoTestBudgeted for semio-framework with those explicit arguments. The generic framework-rs:test router additionally forwards the same arguments to Vitest, so it is unsuitable for this Rust-only selector without its own routing repair.

Root sent the exact target correction to the sole native compiler and Dag before another attempt; no parallel compiler or source revert was launched. Mutation owns the genuine OS30 fixture repairs independently. The corrected base-framework gate has not run at the time of this report; no two-test semantic RED or GREEN is claimed.

