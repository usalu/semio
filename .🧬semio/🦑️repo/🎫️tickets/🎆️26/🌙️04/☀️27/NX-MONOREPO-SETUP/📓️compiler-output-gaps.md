# Compiler Outputs Missing From Cache Contracts

The refreshed native audit completed successfully in 22.0 seconds: 704 projects, 7,270 commands, 7,900 artifact/storage records and zero automated structural findings. A separate read of the native graph selected seven cached `build`, `bundle` or `wasm` targets with an explicit empty output list. Source inspection confirms that four of them perform real compiler or packaging work. The structural audit does not currently distinguish these from intentional completion targets.

| Target | Observed writer | Required follow-up |
| --- | --- | --- |
| `@semio-tech/framework-renderer-wgpu:wasm` | WGPU TypeScript `TrunkBuildScript` builds into the shared renderer store and copies stable JS/WASM names. | Exclusive finite compiler output, separate live module/asset routes, restore proof and complete compiler prerequisites. Trunk/Rust-target preparation is now explicit, but outputs remain empty. |
| `@semio-tech/framework-os-dev:build` | OS Dev TypeScript `BuildScript` calls plugin builds, engine builds and Vite, or invokes the WGPU script directly. | Replace this remaining opaque build path with the existing explicit production task graph. The package's public `build` script already selects `build-s-react-release`, but the generic Nx build target is still callable and selected by aggregate builds. Its direct WGPU branch also bypasses the new Trunk prerequisite. |
| `@semio-tech/ui-styling-py:build` | `🎨️styling/🏗️builder/🟦️.ts` deletes a shared Python build directory and invokes `uv build --wheel` there; the test later reads that wheel. | Publish the wheel into an exclusively owned deliverable directory, retain compiler/dependency state separately and verify restoration. |
| `@semio-tech/ui-styling-dotnet:build` | The same styling builder invokes `dotnet build` into the shared .NET artifacts store. | Publish the consumer's final files separately from native intermediates and verify restoration. |

The remaining three graph candidates are the print template completion target, report document completion target and workspace aggregate build. Their declared commands/dependencies describe completion or aggregation; they were not classified as compiler-output defects in this pass.

No real OS Dev, styling compiler or Trunk bundle was executed during this census, and no shared build directory was deleted. The four confirmed writer/configuration mismatches remain open. The native renderer `native-build` and `native-build-release` directory mismatches were fixed separately and passed production-publisher/Cargo/Nx restoration tests; see `📓️native-renderer-outputs.md`.
