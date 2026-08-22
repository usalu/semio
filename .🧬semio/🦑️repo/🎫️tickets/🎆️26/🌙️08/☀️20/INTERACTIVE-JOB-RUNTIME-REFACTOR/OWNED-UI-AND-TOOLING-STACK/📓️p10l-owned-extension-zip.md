# P10L Owned Extension ZIP

//#region 🎯️Scope

This packet removes the OS plugin extension store's direct `fflate` implementation and manifest
row. The public synchronous `packExtensionPackage` / `unpackExtensionPackage` API, `.sxt` envelope,
manifest and asset layout, and SHA-256 package-content hash contract remain unchanged. No Cargo,
compose, allowlist, suppression, or new third-party dependency was used.

//#endregion 🎯️Scope

//#region 📦️OwnedCodec

`🟦️zip.ts` is the store-owned ZIP contract. Its exported surface contains only repository-owned
`ReadonlyMap<string, Uint8Array>` and `Map<string, Uint8Array>` types. The implementation uses the
platform-provided synchronous raw-DEFLATE primitive behind that boundary and owns the ZIP records,
CRC-32, UTF-8 file-name handling, and validation.

Encoding is deterministic: insertion order is explicit, the store sorts assets, timestamps are
fixed to the ZIP epoch, UTF-8 is marked explicitly, and compression level is fixed at 6. Decoding
supports the exact existing `.sxt` payload methods (stored and DEFLATE) and enforces these bounds:

- encoded ZIP: 256 MiB;
- entries: 4,096;
- UTF-8 name: 4,096 bytes;
- decoded entry: 256 MiB;
- decoded aggregate: 512 MiB.

The decoder also rejects invalid ranges, central/local name or method mismatches, directory
overlap, malformed UTF-8, unsafe or duplicate names, encryption, multi-disk records, ZIP64,
unsupported methods, size mismatches, and CRC mismatches. Inflation is given a hard
`maxOutputLength` derived from the declared and owned bounds.

//#endregion 📦️OwnedCodec

//#region 🧪️DifferentialFixture

Before deleting the store's `fflate` import and dependency row, a differential fixture exercised
the exact extension layout: `🛂️manifest.semio`, `component.wasm`, and the UTF-8 asset
`assets/icons/🧩️.txt` containing `Grüezi 🌍️`.

- current `fflate` encoder -> owned decoder: all three byte payloads matched;
- owned encoder -> current `fflate` decoder: all three byte payloads matched;
- pinned `fflate` ZIP: 541 bytes, SHA-256
  `43675c79f03ba52f45cc57eecabee2a9334e93957128e7975a2979528d14efa9`;
- owned deterministic ZIP: 542 bytes, SHA-256
  `d94d53695b8e28795dcba29bc051d72b08c3b9b9c58f3ad470d03d3a653eaf94`.

The legacy bytes are pinned in the permanent store test, so compatibility no longer depends on
`fflate` being installed. The tests additionally prove deterministic repeated encoding, owned
round-trip behavior, UTF-8 asset preservation, unchanged package-hash derivation, and bounded
rejection of a forged oversized entry.

//#endregion 🧪️DifferentialFixture

//#region 📊️Census

Packet start:

- dependency identities: **179** = Rust **63** + JavaScript **116**;
- JavaScript parity: manifests **83**, external rows **302**, evidenced **145**, unowned **157**,
  undeclared imports **0**;
- OS store `fflate` direct rows/imports: **1 / 1**.

Packet end:

- dependency identities: **178** = Rust **63** + JavaScript **115**;
- JavaScript parity: manifests **83**, external rows **301**, evidenced **144**, unowned **157**,
  undeclared imports **0**;
- OS store `fflate` direct rows/imports: **0 / 0**.

The frozen in-scope `js:fflate` identity is deleted. The explicitly out-of-scope `compose/` tree
still contains its own `fflate` source and manifest row and was not edited; dependency parity and
the freeze intentionally exclude that legacy tree.

//#endregion 📊️Census

//#region ✅️Validation

- `bun install`: **PASS**, lockfile saved, 2,039 installs checked across 2,081 packages.
- `bun nx run @semio-tech/plugin-extension-store:test-quick --skip-nx-cache`: **PASS**, 1 file and
  3 tests.
- `bun nx run @semio-tech/plugin-extension-store:test --skip-nx-cache`: **PASS**, 1 file and 3
  tests.
- `bun ./📜️script.ts verify dependencies parity js`: **PASS**, clean with zero undeclared imports;
  exact census 83 / 301 / 144 / 157 / 0.
- `bun ./📜️script.ts verify dependencies`: **PASS**, baseline 238, current 178, removed 60,
  additions 0; `js:fflate` is listed among removed identities.
- `bun ./📜️script.ts verify dependencies list rust | jq 'length'`: **63**.
- `bun ./📜️script.ts verify dependencies list js | jq 'length'`: **115**.
- Prettier check/write on every changed store source, manifest, config, and task file: **PASS**.

No Cargo command was run.

//#endregion ✅️Validation
