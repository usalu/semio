use std::fs;
use std::path::PathBuf;

use semio_framework_os_kernel::{
    parse_grammar, parse_protocol, print_protocol, verify_protocol_bytes, walk_protocol, Block, Framing, SemioDialect,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).ancestors().nth(7).expect("repo root").to_path_buf()
}

fn main() {
    let source = r#"dialect protocol
protocol flow.pack
version 1
schema flow
start frame
framing magic 0x8953504B0D0A1A0A
header fixed 32
field format_major u16
field format_minor u16
field flags u32
field header_crc32 u32
segment kind u8
segment flags u8
segment payload varint bytes
record field id u16 type tag
field tag varint
field body bytes
footer fixed 84
"#;
    let parsed = parse_protocol(source).expect("parse_protocol");
    assert_eq!(parsed.id, "flow.pack");
    assert!(matches!(parsed.framing, Framing::Magic(_)));
    assert!(parsed.blocks.iter().any(|b| matches!(b, Block::Header(_))));
    assert!(parsed.blocks.iter().any(|b| matches!(b, Block::Segment { .. })));
    assert!(parsed.blocks.iter().any(|b| matches!(b, Block::Footer(84))));
    let printed = print_protocol(&parsed);
    assert_eq!(parse_protocol(&printed).expect("reparse"), parsed);
    println!("ok round-trip");

    let walk_src = r#"dialect protocol
protocol demo.pack
version 1
schema demo
start frame
framing magic 0x8953504B0D0A1A0A
header fixed 12
field format_major u16
field format_minor u16
field flags u32
field header_crc32 u32
segment kind u8
segment flags u8
segment payload varint bytes
footer fixed 84
"#;
    let spec = parse_protocol(walk_src).expect("parse walk");
    let mut bytes = vec![0x89, b'S', b'P', b'K', 0x0D, 0x0A, 0x1A, 0x0A];
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&0u16.to_le_bytes());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.push(1);
    bytes.push(0);
    bytes.push(0);
    bytes.extend(std::iter::repeat(0u8).take(84));
    let trace = walk_protocol(&spec, &bytes).expect("walk Shape A");
    assert_eq!(trace.consumed, bytes.len());
    verify_protocol_bytes(&spec, &bytes).expect("verify");
    println!("ok walk Shape A {}", trace.consumed);

    let spr = r#"dialect protocol
protocol demo.spr
version 1
schema demo.operation
start record
framing record
field format u8
field ordinal varint
field body bytes
"#;
    let spr_spec = parse_protocol(spr).expect("spr");
    let spr_bytes = vec![1u8, 0x00, 0xAA, 0xBB];
    let spr_trace = walk_protocol(&spr_spec, &spr_bytes).expect("walk OpBinary");
    assert_eq!(spr_trace.consumed, 4);
    println!("ok walk OpBinary");

    let meta_path = repo_root().join("🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/📖️protocol.grammar.semio");
    let meta = fs::read_to_string(&meta_path).unwrap_or_else(|e| panic!("read {}: {e}", meta_path.display()));
    let g = parse_grammar(&meta).expect("meta grammar");
    assert_eq!(g.dialect, SemioDialect::Grammar);
    assert_eq!(g.id, "protocol");
    println!("ok self-host protocol.grammar.semio");

    let projected = parse_grammar(source).expect("project");
    assert_eq!(projected.dialect, SemioDialect::Protocol);
    assert_eq!(projected.id, "flow.pack");
    println!("ok parse_grammar projection");

    println!("ALL PROTOCOL M1/M2 PROBES PASSED");
}
