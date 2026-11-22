# XLSX Pack schema authority

The freshly transpiled Office guest reached the general ECMA-376 app for both `stdio.xlsx` and `s.stdio.xlsx`. The Chromium diagnostic refused both `codec.pack-schema-hash` calls with `artifact codec schema has no structural record specification`. The same guest minted a 1,448-byte Pack and 247-byte SPR for each identity, then printed the `stdio.xlsx` document mirror from each pair. That isolates the missing capability to snapshot schema metadata. The diagnostic used component SHA-256 `29c9a312482dd4cb90d2ad42aa3bdfeff02bbfd5a2bc5e739ffc1294d87458ee` and the native-derived production browser bridge.

The generic plugin table takes its hash from `ArtifactPackMetadata::native_pack_schema_hash`. The former implementation could only hash a structural `RecordSpec`. The XLSX snapshot has a handwritten Pack encoder and decoder and publishes their binary protocol in the Base `📡️.protocol.semio`. The authored native factory `stdio.native.xlsx.v1` previously hashed this same file directly. The Base, Strict and Transitional XLSX editors share the Base `XlsxSnapshot` type, including its Pack implementation.

The Pack snapshot metadata adapter retains the structural hash calculation for record-backed snapshots. A handwritten `ArtifactPack` implementation can provide the existing binary protocol bytes for SHA-256 identification. The XLSX implementation supplies `COMPONENT_PROTOCOL_SEMIO.as_bytes()`. Both the bare Store factory and the guest artifact table now use this metadata adapter, alongside the existing native-only factory path. The XLSX root factory uses the resulting hash and the snapshot's existing `xlsx` extension. The artifact definition still pins `b81858b598a8d0bdcbcc36b48291e9d3a3f638adf4cac479e5ebce714c9a1a17` and the executable binding checks this value against the actual factory.

## Verification

- The schema-owned artifact definition gives the language-neutral hash expectation. Python `hashlib.sha256` over the checked-in protocol bytes independently reproduced the exact digest for `stdio.native.xlsx.v1`.
- The XLSX `model_laws` regression `xlsx_guest_pack_metadata_matches_the_authored_native_factory` requires the schema-authorized executable binding to validate, then checks that the same metadata called by the guest equals the native factory hash. It covers the previously missing `Some(hash)` from the shared snapshot type.
- Pinned Rustfmt parsed the five edited Rust files. The managed rebuild and matching Chromium replay belong to the combined verification.
