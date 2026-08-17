# W4a Report — Dependency-layer stdio artifacts

## Done (cargo check green)
- binary, txt, json (W2)
- xml, csv, md
- deflate (zlib inflate/deflate + Adler32 in engine)
- zip (CRC32 + deflate method; IO via binary+deflate)

All wired in glue.rs, plugin register + artifact_kind, TS barrels.
