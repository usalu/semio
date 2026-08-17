# Protocol dialect contract v2

## Model
ProtocolFile { id, version, schema, start, uses, framing, blocks }
Framing = Magic([u8;8]) | Record | Chunked
Block = Header | Segment | Record | Struct | Enum | Footer | Chain
Prim = U8|U16|U32|U64|I32|I64|F32|F64|Varint|Zigzag|Bytes|Utf8|Fixed(n)|Array|Ref

## Laws
1. parse_protocol retains every directive (no skip_line).
2. print_protocol(parse_protocol(x)) round-trips body; canonicalize is idempotent.
3. walk_protocol(spec, bytes) consumes exactly bytes.len() or returns ProtocolMismatch with offset.
4. use loads shared struct/enum fragments; local shadows fragment.
5. Specs are normative-and-verified, not codegen. Encoders stay handcrafted Rust.
