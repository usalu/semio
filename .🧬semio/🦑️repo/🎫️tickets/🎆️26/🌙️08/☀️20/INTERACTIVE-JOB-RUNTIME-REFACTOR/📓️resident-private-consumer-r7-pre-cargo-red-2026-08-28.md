# Resident Private Consumer R7 Pre-Cargo RED

Canonical no-argument native target exited1 in taxonomy discovery before Cargo was invoked. Seventeen test functions were enumerated from actual Rust source, but zero compiled or executed. Five wgpu-frame-worker tracked output paths failed existence validation. This is infrastructure failure, not native17 behavior. No Wasm command followed. Resident source hold and sole native slot were released immediately.

The exact router accepts no arguments and passes only --lib to the shared runner. It does not explicitly select --no-fail-fast; no hidden override was inserted. This distinction did not affect this pre-Cargo failure.

Actual pre/post selected-input differences: []. Shared TS live-shell changes remain separate from compiled native authority. No Plugin/Opening/live funding or WGPU success is inferred.

## Full Actual Output

```text
> nx run @semio-tech/value-resident-rs:test

> bun ./📜️script.ts test

1116 | export function loadTaxonomy(): Taxonomy {
1117 |   if (cachedTaxonomy.current) return cachedTaxonomy.current;
1118 |   const taxonomy = loadCatalogTaxonomy();
1119 |   const workspaceRoot = taxonomyWorkspaceRoot();
1120 |   const problems = workspaceRoot ? validateGeneratorContractsAgainstWorkspace(workspaceRoot, taxonomy) : ["generatorContracts workspace root could not be resolved."];
1121 |   if (problems.length > 0) throw new Error(`Invalid taxonomy schema:\n${problems.map((problem) => `- ${problem}`).join("\n")}`);
                                            ^
error: Invalid taxonomy schema:
- generatorContracts["wgpu-frame-worker"] tracked output "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🏗️builder/🦀️.rs" is missing.
- generatorContracts["wgpu-frame-worker"] tracked output "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/💾️binary/🦀️.rs" is missing.
- generatorContracts["wgpu-frame-worker"] tracked output "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/📚️library/🟦️.ts" is missing.
- generatorContracts["wgpu-frame-worker"] tracked output "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/🧊️renderer/📇️registry/🦀️.rs" is missing.
- generatorContracts["wgpu-frame-worker"] tracked output "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/🧵️frame-worker/🤖️generated/🟨️.js" is missing.
      at loadTaxonomy (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts:1121:38)
      at getCargoWorkspaceIndex (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:1398:20)
      at resolveCargoPackageName (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:1456:17)
      at map (1:11)
      at runCargoTestBudgeted (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:1618:28)
      at run (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/📜️script.ts:8:11)
      at run (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:1048:71)
      at runBundleScriptMain (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:1078:16)

Bun v1.3.14 (macOS arm64)
Warning: command "bun ./📜️script.ts test" exited with non-zero status code


 NX   Running target test for project @semio-tech/value-resident-rs failed

Failed tasks:

- @semio-tech/value-resident-rs:test

Hint: run the command with --verbose for more details.
```

## Actual Post-Run Hashes

```text
11544b761834a296b649015287a5a5f8ccda56ca36bd3fe96fb24ca29624c2da  .config/nextest.toml
1a14cea5e9cc0e10f6fca0cfd7c1fceb7dc433f0e3788a626ecbee11fcc4f6fb  Cargo.lock
d11caa23a68359d9d77453fef63a85eb1d80f1ca990a1f20abf88d47edd951f2  Cargo.toml
a6fdd56e53ca147f37ba5f503b4dcff02810dd76e7bb91d0b116ba1c39377bb8  rust-toolchain.toml
c6e193d70e668a86a475cb00bdb8a59eec6ac6fb481e66b1c70f85b62877042d  🧰️framework/🔨️modules/🌱️value/💾️resident/📋️project.json
50793dbcbf2d873e8391faebfe436322470840a2db5d4e584b95032838f89ab3  🧰️framework/🔨️modules/🌱️value/💾️resident/📜️script.ts
c73b7b90a2efe859270f797c4ecfebd3457472e06462e991bdadb11fb0d750de  🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/Cargo.toml
9669b870a0f0e95a8466955cc76f1fec629bacf98928fa8430831cd5137ddbd8  🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/📋️project.json
4b977ef3f6fbe7d04fbdf90bcd186346b79f81788b0d529ea5501fc06b95bbb9  🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/📜️script.ts
8df81492f42dfa1232a718e917149b209d7151a72d5bea397f354091290f55ad  🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️fixture.json
3d9b729ec2fef59a179ce4425a7d1c0554c5937d19512065f3bf760568640b6a  🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️schema.json
c4655f43d54524f15015a753e2e9441c04d63b738601f6ebd3a63eec27a74238  🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧬️contract.json
42a213e71a8be05b8b9e9784f53525ba319a2256c1d9b21318ec9e300a1dab37  🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧬️schema.json
72b0e0ba9bab57f7b95c988c40171e7237b0e5ca00e84b063df03e2c3edd6530  🧰️framework/🔨️modules/🌱️value/💾️resident/🟦️component.ts
508b78726ae6747f476fdb7d60938b3d2349ea300ef8fc55d555502a3500c49f  🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs
fd5e4114b67f00a22db17f6b5203f3e78ab4b3c72ae1365223040eaf88f89428  🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️fixture.json
4394f95634fe7c1edfe6a06e4ab985cd83020332bd25c83473ac99566170408b  🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️schema.json
ebde45c9d5ff7f5276e7a33f464601c23b6018d3e412c67616beaeea488f297e  🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs
836fd1402a40e96cb3c1557a51f13151d04b28f25d64e4ac9ab208419ccf3a28  🧰️framework/🔨️modules/🌱️value/💾️resident/🧬️contract.json
6a684a67751efb699db63d374dcc9375fc6f895785802d5c14949e8a57e617a0  🧰️framework/🔨️modules/🌱️value/💾️resident/🧬️schema.json
```
