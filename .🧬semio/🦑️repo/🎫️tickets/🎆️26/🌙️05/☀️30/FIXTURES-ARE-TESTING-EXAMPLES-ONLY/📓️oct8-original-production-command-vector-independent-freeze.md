# Original Production Command Vector Freeze

Read-only authored observation on 2026-10-08; no compiler/tool/test was run. Source hashes are observations, not an adoption of these files as this lane’s authored changes. Historical compiler receipts establish original argument vectors only; their source drift prevents current production credit.

## Current Source SHA256

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🟦️.ts`: `75f9fda6567bd46094495271c169596a319dc5b97fdc441e604423a0caa54b37`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🔣️.json`: `64ac2a9ed81204d3438fb4af86aa1c2f23fd0b5d8e5840762c2277a1f2083176`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`: `41e323567f8d22cc7b240cf820287426472f9783ada698234af0489827597cb7`
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`: `2b22368ffda273b7164e694974bd84558c437b33bda8b271fcd8b50c1e1dccfc`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🌐️wasm/📜️script.ts`: `8568e1966f732075a9b64102a708f4458102b24452702b2c980f12d4b8d65823`

## Four Guest Vectors

### 1

```json
["check", "--manifest-path", "Cargo.toml", "--lib", "--target", "wasm32-wasip2", "-p", "semio-framework", "-p", "semio-framework-replication", "-p", "semio-framework-os-kernel", "-p", "semio-framework-os", "-p", "semio-framework-plugin", "--features", "semio-framework-plugin/component-guest,semio-framework-os-kernel/deflate,semio-framework-replication/deflate", "--message-format=json"]
```

Exact ordered current vector equals retained actual receipt args: **true**. Receipt SHA256 `87b28995a52355968d1744900bfac3c50ce685f22919dbd24cbbc7b01841ec47`; status 0. Receipt path `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/fixture-boundary-production/build/semio-cargo-provenance/cargo-unit-provenance-cargo-relay-ed7184ca-7d92-4de2-b2c9-088565b26671.json`.
### 2

```json
["check", "--manifest-path", "Cargo.toml", "--lib", "--target", "wasm32-unknown-unknown", "-p", "semio-framework-os-kernel", "--features", "semio-framework-os-kernel/sync", "--message-format=json"]
```

Exact ordered current vector equals retained actual receipt args: **true**. Receipt SHA256 `c03d369af6fbe30f40645918d8576e38f16fc4b79ef533bf1c3f855b84697797`; status 0. Receipt path `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/fixture-boundary-production/build/semio-cargo-provenance/cargo-unit-provenance-cargo-relay-3cc07682-4d64-4a48-be23-54baff9d440a.json`.
### 3

```json
["check", "--manifest-path", "Cargo.toml", "--lib", "--target", "wasm32-unknown-unknown", "-p", "semio-framework-os-renderer-wgpu", "--message-format=json"]
```

Exact ordered current vector equals retained actual receipt args: **true**. Receipt SHA256 `bcbf2549d0f3b9f44e89b983708169711e50d1d54a719a5565e38dac404b9e29`; status 0. Receipt path `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/fixture-boundary-production/build/semio-cargo-provenance/cargo-unit-provenance-cargo-relay-17c5b035-24c7-4e5a-ae08-0d23b12e3765.json`.
### 4

```json
["check", "--manifest-path", "✏️s/Cargo.toml", "--lib", "--target", "wasm32-wasip2", "-p", "semio-s-plugin-stdio", "--message-format=json"]
```

Exact ordered current vector equals retained actual receipt args: **true**. Receipt SHA256 `64f2e50e6e6f638a50a7e9326233f96f116e27b85c1dd1f8e84da2c940111684`; status 0. Receipt path `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/fixture-boundary-production/build/semio-cargo-provenance/cargo-unit-provenance-cargo-relay-caef0682-dad1-4f5e-bae5-16f6958626f4.json`.

## Hub Six and WGPU

Hub protected selection remains `stdio,gis,note,writer,draw,puzzle`, mapping respectively to `semio-hub-stdio`, `semio-hub-gis`, `semio-hub-note`, `semio-hub-writer`, `semio-hub-draw`, `semio-hub-puzzle`. Each request uses `wasm-release`, `rootCdylib: true`, with output `<cargo_package_underscored>.wasm`. The fresh producer’s exact logical component vector is `["rustc","-p",cargoPackage,"--lib","--crate-type","cdylib","--target","wasm32-wasip2","--profile","wasm-release"]`; descriptor emitter is `["build","-p","semio-framework-plugin-describe"]`. Both run from repository cwd, with the producer’s JSON capture adding `--message-format=json`; no explicit feature/default-feature override appears in these vectors. Default ordinary bootstrap remains stdio,gis,note; the protected six selection is explicit and must remain so.

WGPU retained source query is `["build","--target=wasm32-unknown-unknown","--manifest-path",<renderer Cargo.toml>, ...release?["--release"]:[], "--offline","--frozen","--locked","--message-format=json"]`. Development omits release; release includes it. No explicit package or feature override. Metadata vector is `["metadata","--locked","--offline","--format-version=1","--manifest-path",<renderer Cargo.toml>]`; Trunk vector is `["build","--config",<private Trunk.toml>,"--skip-version-check","--offline","true","--color","never"]`. Config retains locked/frozen/offline, profile release boolean, worker/web bindgen and wasm-opt z. This source observation is not a completed actual development/release command receipt. The cancelled run did not reach those compiler queries.

The exact physical Cargo owner must come from parsed system authority. Native’s current correction identifies WGPU workspace_root as repository root (55 members), not Framework. Package-path ancestry and no-deps identity alone cannot substitute for actual resolve closure. Core repair must retain the vectors above and acquire their current selected dependency closure at each actual consuming route.
