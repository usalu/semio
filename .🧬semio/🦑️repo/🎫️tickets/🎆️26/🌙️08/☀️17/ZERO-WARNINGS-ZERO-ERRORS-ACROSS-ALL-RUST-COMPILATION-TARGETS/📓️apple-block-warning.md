# Apple Blocks Future Incompatibility
The installed block 0.1.6 declares an external static using an uninhabited Class type. The upstream master source retains that declaration, so switching to its current branch alone would not fix it. [rust-block source](https://github.com/SSheldon/rust-block/blob/master/src/lib.rs)

The current workspace uses wgpu 27.0.1. Locally resolved wgpu-hal 27.0.4 includes block 0.1.6 and metal 0.32. The v28 backend also declares the same block and metal dependencies. [wgpu-hal 28 manifest](https://raw.githubusercontent.com/gfx-rs/wgpu/v28.0.0/wgpu-hal/Cargo.toml)

The v29 Metal backend switched to block2 and objc2-metal. Updating the workspace graphics stack to a released version with that backend is a candidate for removing this warning without suppressing it or patching Cargo's registry cache. This is an inference from the dependency manifests, not a verified workspace fix. [wgpu-hal 29 manifest](https://raw.githubusercontent.com/gfx-rs/wgpu/v29.0.0/wgpu-hal/Cargo.toml)

The upstream releases page currently lists v30.0.1 and v29.0.4. No dependency changes have been made yet. A complete inventory of direct and transitive wgpu users and a real native/browser renderer test are still required. [wgpu releases](https://github.com/gfx-rs/wgpu/releases)
