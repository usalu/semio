# Browser Publication Read-Only Audit — 2026-10-08

The original warm browser serve cannot pass current owner publication discovery while the committed Stdio publication digests disagree with their protocol source bytes. This audit performed file reads and SHA-256 comparison only; no build, serve, registration, catalog mutation, or digest bypass was performed.

## Exact Authority Chain

- Cargo owner: `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/Cargo.toml:20–21` declares `[package.metadata.semio.publications]` with manifest `../../📇️publication/🔣️.json`.
- Manifest: `🌎️hub/🧩️compositions/🗄️stdio/📇️publication/🔣️.json`, publication `stdio-native-codecs`, output `trusted-stdio-catalog.json`, payload `📜️native-catalog.json`.
- Payload schema: `🧬️schema/🧬️trusted-stdio-catalog/🔣️.json`, definition `/$defs/TrustedStdioCatalogV1` relative to publication directory.
- `/nativeCodecs/9/protocolSourceSha256` identifies `s.stdio.jpg`, schema `stdio.jpg`, factory `stdio.native.jpg.v1`, extension `jpg`.
- Renderer `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📇️catalog/📣️publication/🟦️.ts:84` validates manifest and payload, admits no-follow regular files, computes SHA-256 from current bound source bytes, and rejects any mismatch before projecting output.

## Current Mismatch Snapshot

- `/nativeCodecs/9/protocolSourceSha256`
  - Published: `0d32030c224fda4733dc23733130d50931413bba28c83168817f294c8d1943c8`
  - Current source: `65fe81f12ad7c4f2d39e9d3f43cb69f147ef60960beb1e5976c065b587ed5c55`
  - Source: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio`
- `/nativeCodecs/17/protocolSourceSha256`
  - Published: `b92db22ccdb887e23b1e41c86071a9d236ed8415fede659200494c6b45472355`
  - Current source: `9c3f260b792c2d71461d6f46b493c17da850403061808e00ab7eedabaef9e7b8`
  - Source: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio`

JPEG factory receipt at audit time:

```json
{
  "artifact": "jpg",
  "factory_id": "stdio.native.jpg.v1",
  "artifact_kind": "s.stdio.jpg",
  "artifact_schema": "stdio.jpg",
  "extension": "jpg",
  "pack_schema_hash": "bacf1eb3e87697fa0d2dd1659effaf97aaba8f9496877c9b072a0e96151c683c",
  "runtime_capability_id": "s.stdio.jpg.standard.jfif-1-01.codec.codec-stdio-jpg-extension-jpg.v1",
  "descriptor_codec_id": "s.stdio.jpg.standard.jfif-1-01.codec.native-document.v1",
  "protocol_source_sha256": "0d32030c224fda4733dc23733130d50931413bba28c83168817f294c8d1943c8",
  "protocol_path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio",
  "definition_path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/📜️artifact-definition.json"
}
```

## Existing Canonical Workflow and Readiness

The registered command is `bun nx run @semio-tech/stdio-plugin:native-codec-projection --excludeTaskDependencies --skip-nx-cache -- refresh jpg`. The target is present in `.vscode/launch.json:20452–20455`; an existing artifact-scoped refresh invocation for PNG is present at line 44819. The router registers `native-codec-projection` at `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:1428`; its implementation at 1267–1310 supports `refresh <artifact>`.

The refresh path first runs the artifact-owned registered native test:

```text
bun nx run @semio-tech/stdio-jpg-rs:test --excludeTaskDependencies --skip-nx-cache -- long owned_fixture_publication_reports_canonical_logical_carriers -- --nocapture
```

It requires exactly one successful `[DEBUG] native-codec-publication=` receipt, checks the authored factory identity, checks protocol SHA against the compiled receipt, and updates only the factory catalog, artifact definition, and native catalog through `projectNativeCodecReceiptPublicationV1`. It checks protocol and all three original documents for concurrent changes before writing. The update helper rejects stale source and mismatched receipt identities (`📇️publication/♻️native-receipt/🟦️.ts:22–49`). It provides cancellation via SIGINT/SIGTERM. The no-argument path instead runs a full native live pack-hash projection; this is broader than the artifact-scoped repair.

Composition generation can regenerate publication payload/bindings from contribution receipts (`🧩️composition/🟦️.ts:161–177`), but it does not independently prove current compiled receipts and is not a safe substitute for publication validation. Existing trusted publication validation is implemented in `📇️publication/✅️trusted-stdio-catalog/🟦️.ts`; source-digest checks also live in the renderer itself.

The refresh workflow exists; successful current JPEG native receipt and post-publication renderer admission were not executed in this read-only audit. Current browser readiness remains blocked at publication provenance. Source owners may change concurrently, so all hashes above are a bounded snapshot and must be rechecked immediately before a subsequent owner-authorized refresh or serve. The original staged procedural plugin receipt does not establish current Stdio source provenance.

## Reproduction

From `/Users/ueli/Documents/semio`, read manifest and payload with JSON parsing; resolve each `integrity.path` relative to `🌎️hub/🧩️compositions/🗄️stdio/📇️publication`; compute `hashlib.sha256(path.read_bytes()).hexdigest()` and compare with its exact JSON pointer. This was executed for every declared binding, with the mismatches listed above. No passing native test or working browser runtime is claimed.
