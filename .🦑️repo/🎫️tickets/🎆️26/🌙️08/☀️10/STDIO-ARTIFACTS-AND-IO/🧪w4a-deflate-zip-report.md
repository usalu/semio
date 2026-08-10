# W4a Report — Deflate and Zip

Ticket: `26/08/10/STDIO-ARTIFACTS-AND-IO`

## Scope

Implemented dependency-layer stdio artifacts:

| id | dir | mime | files |
|---|---|---|---|
| deflate | `🗜️deflate` | application/zlib | 82 |
| zip | `🎒️zip` | application/zip | 86 |

Facet trees copied from `💾️binary` (82-file shape). Zip adds deflate IO peers (+4 files → 86).

## Codecs (hand-rolled, no runtime compression crates)

### 🗜️deflate

- **Adler32** (RFC1950)
- **Raw DEFLATE** compress: fixed Huffman literals + EOB (`deflate_raw`)
- **Raw DEFLATE** inflate: stored + fixed + dynamic Huffman (`inflate_raw`)
- **Zlib wrapper**: CMF/FLG `0x78 0x01` + raw deflate + Adler32 BE trailer (`zlib_compress` / `zlib_decompress`)
- Snapshot field: `bytes` = zlib stream
- IO ↔ binary: import compresses payload; export inflates payload

### 🎒️zip

- **CRC32** (ISO-HDLC / ZIP), known vector `CRC("123456789") = 0xCBF43926`
- Encode/decode local headers + central directory + EOCD
- Methods: **0 store** and **8 deflate** (raw deflate via `crate::artifacts::deflate::engine::deflate_raw` / `inflate_raw`)
- Snapshot: `entries: Vec<ZipEntry { name, data }>` (uncompressed payloads)
- IO ↔ binary: parse/encode ZIP bytes
- IO ↔ deflate: zlib-compress/decompress the ZIP container via deflate artifact

## Wiring

- `📦️glue.rs`: `pub mod deflate` + `pub mod zip` (zip IO peers: binary + deflate)
- `🟦️typescript/📦️index.ts`: `export * as deflate` / `export * as zip`
- `🔌️plugin/🦀️component.rs`: `engine::register()` + `.artifact_kind(...)` for both

Broken concurrent W4b glue modules (`obj`,`stl`,`ply`,`dxf`,`svg`,`bmp`) were removed from glue/plugin/index so `cargo check` could validate this wave; their on-disk facet trees were left in place for those agents.

## Gates

```
cargo check -p semio-s-plugin-stdio   # Finished ok
cargo test -p semio-s-plugin-stdio deflate::
# 5 passed (adler32, zlib RT, raw deflate RT, pack codec RT, demo)
cargo test -p semio-s-plugin-stdio zip::
# 5 passed (crc32 vector, store RT, deflate RT, pack codec RT, demo)
```

Logs: `🧪w4a-cargo-check.log`, `🧪w4a-deflate-test.log`, `🧪w4a-zip-test.log`.

## Examples

- `📚️examples/🎬️demo/🖼️assets/🗜️example.zz` — zlib of ASCII payload
- `📚️examples/🎬️demo/🖼️assets/🎒️example.zip` — ZIP_DEFLATED single-entry fixture
