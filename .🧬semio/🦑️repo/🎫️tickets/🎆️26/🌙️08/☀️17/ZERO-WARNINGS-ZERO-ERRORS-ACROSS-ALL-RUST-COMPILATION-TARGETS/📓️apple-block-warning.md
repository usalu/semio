# Apple Blocks Future Incompatibility
The installed block 0.1.6 declares an external static using an uninhabited Class type. The upstream master source retains that declaration, so switching to its current branch alone would not fix it. [rust-block source](https://github.com/SSheldon/rust-block/blob/master/src/lib.rs)

The initial workspace used wgpu 27.0.1. Its resolved wgpu-hal 27.0.4 included block 0.1.6 and metal 0.32. The v28 backend also declares the same block and metal dependencies. [wgpu-hal 28 manifest](https://raw.githubusercontent.com/gfx-rs/wgpu/v28.0.0/wgpu-hal/Cargo.toml)

The v29 Metal backend switched to block2 and objc2-metal. Updating the workspace graphics stack to a released version with that backend is a candidate for removing this warning without suppressing it or patching Cargo's registry cache. This is an inference from the dependency manifests, not a verified workspace fix. [wgpu-hal 29 manifest](https://raw.githubusercontent.com/gfx-rs/wgpu/v29.0.0/wgpu-hal/Cargo.toml)

## Implemented Upgrade and Verification

Updated the existing graphics stack to wgpu 29.0.4, Vello 0.9, and vello_svg 0.10. The native UI, browser WebGPU backend, Raster, Infinite canvas, and renderer use the new API: owned instance descriptors with explicit display handles, current-surface outcomes, optional bind group and depth fields, and the revised pipeline descriptors. [wgpu 29.0.4 API changes](https://raw.githubusercontent.com/gfx-rs/wgpu/v29.0.4/CHANGELOG.md), [Vello 0.9 dependency manifest](https://raw.githubusercontent.com/linebender/vello/v0.9.0/Cargo.toml), [vello_svg 0.10 dependency manifest](https://raw.githubusercontent.com/linebender/vello_svg/v0.10.0/Cargo.toml)

Cargo.lock now resolves one wgpu version (29.0.4), and contains neither block nor metal. The native renderer's Cargo dependency tree confirms wgpu 29.0.4. No warning suppression or Cargo registry patch was used.

Native Raster passed both row-alignment and GPU scene readback tests with this dependency graph. The 32×32 scene's center alpha assertion ran successfully; no no-adapter skip message appeared in captured stderr. The full native compiler pass and browser checks are still in progress, so this is not yet a claim that every renderer target is clean.
