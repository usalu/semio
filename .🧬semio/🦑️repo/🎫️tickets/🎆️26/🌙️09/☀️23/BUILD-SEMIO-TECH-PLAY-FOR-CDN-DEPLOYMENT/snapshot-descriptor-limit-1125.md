# Snapshot Descriptor Limit 1125

Tree edited: `/Users/ueli/.sinc/play` (paths below are relative to it). Limit raised: 4 MiB (4194304) → 8 MiB (8388608).
Trigger: `semio-hub-stdio-pdf-component` descriptor JSON grew 3,656,980 → 4,686,455 B (roster correction, see `snapshot-pdf-cancellation-proof-1125.md`).

**Nothing was compiled, tested or run** (no cargo/rustc/bun). The running release build is the verifier.

## Places that bound the descriptor pair

P = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`

### Constants (raised)

| Path:line | Bounds | Change |
|---|---|---|
| `P/🖨️describe/🏗️component-build/🟦️.ts:16` `FRESH_DESCRIPTOR_MAX_BYTES` | fresh-build JSON + pack read (`🏭️fresh-component` 209-210) | 4 → 8 MiB |
| `P/📇️registry/✅️catalog-verification/🟦️.ts:12` `CATALOG_DESCRIPTOR_MAX_BYTES` | emission staging (`🛂️descriptor-emission` 112-113, the reported refusal), stage-cache readback (`🧩️component-stage` 116), descriptor-pair verify (`🧬️descriptor-pair` 57, `📁️filesystem` 18-19), fresh-build verifier (25, 90), model products (`📦️model` 70, `👓️reading` 29), browser-bundle `semantic.bin` (`🏗️materialization/🚀️commands` 254), native entrypoint (`📺️renderer/…/⌨️native-entrypoint/📦️modules` 28) | 4 → 8 MiB |
| `P/📇️registry/🧬️schema/🟦️.ts:6` `REGISTRY_DESCRIPTOR_MAX_BYTES` | `decodeRegistryDescriptorV1` (catalog generation / release verify via `renderCatalogFiles`) | 4 → 8 MiB |
| `🧰️framework/🔨️modules/📇️directory/🧬️schema/📂️document-open/🟦️.ts:117` `DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES` (TS) | browser runtime: `🏪️store/👷️worker/🪪️execution-lease` 262, `🔏️document-open` 203, `🧑‍💻dev/🔎️verification/🧾️publication` 75, `✏️s/🧑‍💻dev/🧩️catalog/🌐️browser-host/🏗️staging` 150/175/177, hub bootstrap selection 90/127, gis-browser-proof 33, bootstrap-claims 64 | 4 → 8 MiB |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🛰️execution-target-artifacts-and-stream/🦀️.rs:13` `DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES` (Rust) | native lease validation (98), directory client body (`📡️directory-client` 283), wgpu native plugin loading (301, 323), and the hub's `TRUSTED_DESCRIPTOR_MAX_BYTES` (`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/📜️contract/🦀️.rs:25`, an alias, so it follows), which bounds loader read (`🏗️loader` 164), bundle validation (177), pack decode `max_file_len`/`max_segment_len`, `open-targets` 126, and the publish command stdin (`📤️command` 34) | 4 → 8 MiB |

### Schema maxima (raised)

| Path:line | Bounds | Change |
|---|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json:3299` | `descriptor.byteLength` (execution-target lease) | 4194304 → 8388608 |
| same file `:5012` | `descriptor.byteLength` (browser document-open transport) | 4194304 → 8388608 |
| `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🔣️.json:263` | `TrustedBundleFileV1.byteLength`, referenced only by `package.descriptor` | 4194304 → 8388608 |

### Literal bound in production code (switched to the constant)

- `🌎️hub/🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🔄️rotation-and-current-pointer/🟦️.ts:53`: rotation `descriptor.semio` read was `4 * 1024 * 1024`. It now uses `DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES`, imported from `@semio-tech/framework/directory/schema` as the sibling bootstrap files do.

### Tests and fixtures (expected values that encode the limit)

- `🌎️hub/🧪️tests/🎁️trusted-stdio-gis-bundle-check/🟦️.ts:93`: the source-text check now expects `CATALOG_DESCRIPTOR_MAX_BYTES = 8 * 1024 * 1024`.
- `🌎️hub/🧪️tests/👤️browser-actor-checks/🟦️.ts:431`: GIS descriptor read, literal replaced by the constant (plus an import).
- `🌎️hub/🧪️tests/📄️browser-document-open/🟦️.ts:80`: installed descriptor bound, literal replaced by the constant (plus an import).
- `🌎️hub/🧪️tests/🏛️trusted-catalog-fixtures/🚀️stdio-gis-bootstrap-and-compiled-dependencies-fixtures/🟦️.ts:45`: `8_388_608` / `8_388_609`.
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🧬️stdio-gis-bootstrap/🔣️.json:142-143`: 8388608 / 8388609.
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/👥️two-package/🔣️.json:8-9`: 8388608 / 8388609. It is asserted equal to `TRUSTED_DESCRIPTOR_MAX_BYTES` and used by the integration test's max / max+1 vectors.
- `🌎️hub/🧪️tests/🧫️fixtures/🪪️execution-target-relay-v1/🔣️.json:17`: 8388608. `relay-checks` 284 requires it to equal the constant.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json:289`: `descriptorMaxBytes` 8388608. Hub `execution-target-lease-checks` 238 requires equality, and the os lease test's max+1 vector uses it.
- `P/📇️registry/🧪️tests/✅️catalog-publication/🟦️.ts:163,172`: the "oversized" vector padding and the oracle bound are both 8 MiB. Otherwise 4 MiB of padding would no longer be oversized.
- `P/🛂️describe/🧪️tests/🔬️example-assets/🦀️.rs`:
  - line 12: `DESCRIPTOR_MAX_BYTES` is now 8 MiB.
  - line 44: the source-text assertion now expects `8 * 1024 * 1024`.
  - line 60: the dominance threshold is now `* 3 / 8`, because the fixed 3,577,295 B body cannot exceed 3/4 of 8 MiB. 3/8 is 3,145,728 B, so the law still holds with margin.
  - The doc and message text was updated to match.
- Doc text only, same line count: `P/🛂️describe/🦀️.rs:20`, `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️app-label/🦀️.rs:331`, `🧰️framework/🔨️modules/🛂️manifest/🗣️contributions-and-examples/🟦️.ts:132`.

### Left unchanged

| Path:line | Bounds | Reason |
|---|---|---|
| `P/🖨️describe/🧬️schema/🔣️.json:141` | `FreshSourceEpochV1.files[].byteLength` | Bounds source-epoch input files (cargo dep-info compiler inputs). These never include the descriptor pair: the pdf component crate is a 3-line `plugin_exports!`, and no component `include_*!` reads a descriptor. |
| `P/🖨️describe/🧾️source-epoch/🟦️.ts:67` `FRESH_SOURCE_EPOCH_LIMITS.fileBytes` and its fixture `🧫️fixtures/🧾️fresh-source-epoch/🔣️.json:3` | source-epoch file bytes | Same reason. |
| `P/🖨️describe/🛂️descriptor-emission/🦀️.rs:288` `DESCRIBE_ARTIFACT_MAX_BYTES` | emitter output (emitter crate) | 1 GiB, not binding. The emitter already wrote the 4.69 MB file. No emitter-crate constant needed to change. |
| `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/📜️contract/🦀️.rs:21` `TRUSTED_BUNDLE_MAX_BYTES` and the bootstrap-bundle / rotation / gis-proof `4 * 1024 * 1024` reads of `trusted-catalog.json` | the bundle manifest | Not the descriptor. |
| `P/📇️registry/🌎️hub-source/🟦️.ts:46` `HUB_PLUGIN_MODULE_INDEX_MAX_BYTES` | plugin-module index | Not the descriptor. Module files (which include `🔣️.json`/`🛂️.descriptor.semio`) use `TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES` (component max, 1 GiB). |
| `🌎️hub/🏗️bootstrap/🧾️provenance/🧭️bootstrap-selection/🟦️.ts:268` | codec rows | Unrelated. |
| Job/retained grants, shard `memoryBytes`, genesis message, guest diagnostics, `LOAD_PROGRESS_UNIT_BYTES`, renderer `MAX_DESCRIPTOR` = 4096 (media export) | other 4 MiB / "descriptor" uses | Unrelated. |
| `…/🎞️frame-worker/🤖️generated/🟨️.js:23969` | bundled copy of the TS constant | Gitignored generated output. It is regenerated from the edited source. |

## Derived budgets checked

- `🏭️fresh-component` `admission = { remaining: 2 * FRESH_DESCRIPTOR_MAX_BYTES }` scales automatically to 16 MiB.
- `🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts:44` `DESCRIPTOR_PROBE_MAX_OUTPUT_BYTES` = 12 MiB is the base64 probe answer. Base64 of an 8 MiB pack is 11,184,812 B, under 12,582,912. Unchanged.
- Trusted-catalog pack decode (`🔍️descriptor-validation` 76-82):
  - `max_items` 262,144, `max_symbols` 131,072, `max_total_alloc` `TRUSTED_DESCRIPTOR_MAX_MATERIALIZATION` = 32 MiB.
  - The committed pdf JSON has 80,226 nodes and 978 distinct strings. That is about 103K items at 4.69 MB and about 184K at an 8 MiB JSON, both under 262,144.
  - The pdf pack is 757 KB, so the 32 MiB materialization is far off. These are structural caps, not proportional to bytes. Unchanged.
- `🌉️mcp/🏠️workspace/🔗️remote/📏️binding-limits/🦀️.rs:12` `HUB_VERIFIED_CATALOG_MAX_DESCRIPTOR_BYTES` = 32 MiB is the sum of the verified packs one binding uses. All 75 committed packs total 13,908,906 B (largest 861,064 B). Unchanged.
- Stage cache (`🏃️process/📦️artifacts/🏗️native-build/🗃️stage-cache`) has no per-file bound.
- Artifact publication and inventory (`readArtifactBytesV1`, play pages) cap the total at `maximumOutputBytes` = 8 GiB. `maximumManifestBytes` = 2 MiB applies only to page and cohort manifests, not descriptors. Unchanged.
- `readStableBuildFile` and `readCatalogFile` only take the caller's bound (plus the 1 GiB artifact ceiling).

## Runtime-side uncertainty

- Not confirmed whether `semio-plugin-describe` links code that uses the Rust `DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES`. Its sources likely depend on the directory schema crate through `semio_os_kernel`, so cargo will recompile the emitter after this edit.
  - The stage cache keys on the emitter binary's sha256 (`🛠️emitter` 109). If the constant is unused there, the binary and cache keys stay identical. If it is used, the describe stage cache re-keys once.
  - The `🛂️describe/🦀️.rs` edit is doc-only with the same line count, so it does not change codegen.
- The pdf pack after regeneration is unknown; it is estimated at about 1 MB from the 28 % JSON growth. Item counts above are estimated from the committed 3.66 MB JSON, not the new 4.69 MB one.
- No other runtime descriptor bounds were found in the browser host (TS) or the native host (Rust) beyond the constants above. The hub trusted catalog follows the Rust constant through its alias.
