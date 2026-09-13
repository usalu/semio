# Native Incrementality Across Private Publication

## 2026-09-13

A native Cargo probe ran through an uncached private Nx target, using the repository's actual .cargo/config.toml and pinned rust-toolchain.toml. It compiled a local two-crate dependency graph into a fresh shared compiler directory, removed each invocation's private CARGO_TARGET_DIR, and selected a different private output root for the next invocation.

Cargo's JSON compiler-artifact records reported:

| Cycle | Dependency Fresh | Leaf Fresh |
|---|---|---|
| Cold | false | false |
| Identical repeat after private-output deletion | true | true |
| Leaf source edit after private-output deletion | true | false |

The probe passed in 3.1 seconds of Nx task-run time. Native Cargo reported 0.78 seconds cold, 0.07 seconds repeated and 0.47 seconds after the leaf edit. This confirms the current Cargo build-directory configuration retains compiler state independently of the private publication outputs. The successful fixture was removed.

This is a controlled native-library probe on macOS arm64. It does not qualify arbitrary Cargo build scripts, WGPU release optimization or all platforms. It also does not explain every repository rebuild observed during concurrent source development. Those must be interpreted against their actual changed inputs.

Input probe: 🔬️native-inventory/🦀️incremental-publication/📜️script.ts. Transient native evidence is retained only under the ticket's generated directory.
