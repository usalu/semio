#!/usr/bin/env python3
"""W3 mp4+avi facet-mirror + grammar-leaf generator (ticket-local, temporary — see CLAUDE.md
"temporary files ... inside the ticket folder"). Writes real, honest (non-fabricated) facet
mirrors (TS/GraphQL/JSON-Schema/proto) and grammar leaves (text: g4/ebnf/grammar.semio +
graphql/json/proto/ts; binary: ksy/spicy/abnf/protocol.semio + ts) for both new format artifacts'
snapshot/diff/mutations schema levels plus the artifact-level facet. Mirrors the Rust types
already hand-written in the sibling component.rs files field-for-field.
"""
import os

MP4_SCHEMA = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema"
AVI_SCHEMA = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema"

def write(path, content):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as fh:
        fh.write(content if content.endswith("\n") else content + "\n")

def ts_field(name, ty, optional=False):
    q = "?" if optional else ""
    return f"  /** @state persistent */ {name}{q}: {ty};"

def gql_scalar(rust_ty):
    return {"String": "String", "u8": "Int", "u16": "Int", "u32": "Int", "i32": "Int", "i64": "Int",
            "bool": "Boolean", "f32": "Float", "f64": "Float"}.get(rust_ty, rust_ty)

#region MP4 facets — snapshot
def gen_mp4_snapshot():
    base = f"{MP4_SCHEMA}/📸️snapshot"
    write(f"{base}/🟦️component.ts", """\
/** 🧬️ Mp4Snapshot — ISO-BMFF: ftyp typed, decoded per-track sample tables, everything else
 * typed-raw retained. Mirrors 🦀️component.rs field-for-field. */
export interface Mp4Ftyp {
  majorBrand: string;
  minorVersion: number;
  compatibleBrands: string[];
}
export type Mp4Codec =
  | { codec: "avc"; sps: number[][]; pps: number[][]; nalLengthSize: number }
  | { codec: "other"; fourcc: string; raw: number[] };
export interface Mp4Sample {
  data: number[];
  duration: number;
  ctsOffset: number;
  sync: boolean;
}
export interface Mp4Track {
  trackId: number;
  timescale: number;
  codec: Mp4Codec;
  width: number;
  height: number;
  samples: Mp4Sample[];
}
export interface Mp4Box {
  fourcc: string;
  data: number[];
}
export interface Mp4Snapshot {
""" + "\n".join([ts_field("schema", "string"), ts_field("ftyp", "Mp4Ftyp"), ts_field("tracks", "Mp4Track[]"), ts_field("unknownBoxes", "Mp4Box[]")]) + "\n}\n")

    write(f"{base}/🔗️component.graphql", """\
type Mp4Ftyp { majorBrand: String! minorVersion: Int! compatibleBrands: [String!]! }
union Mp4Codec = Mp4CodecAvc | Mp4CodecOther
type Mp4CodecAvc { sps: [[Int!]!]! pps: [[Int!]!]! nalLengthSize: Int! }
type Mp4CodecOther { fourcc: String! raw: [Int!]! }
type Mp4Sample { data: [Int!]! duration: Int! ctsOffset: Int! sync: Boolean! }
type Mp4Track { trackId: Int! timescale: Int! codec: Mp4Codec! width: Int! height: Int! samples: [Mp4Sample!]! }
type Mp4Box { fourcc: String! data: [Int!]! }
type Mp4Snapshot {
  schema: String! @state(class: PERSISTENT)
  ftyp: Mp4Ftyp! @state(class: PERSISTENT)
  tracks: [Mp4Track!]! @state(class: PERSISTENT)
  unknownBoxes: [Mp4Box!]! @state(class: PERSISTENT)
}
""")

    write(f"{base}/🔣️component.json", """\
{
  "$id": "https://semio.tech/schema/stdio.mp4.snapshot.json",
  "title": "Mp4Snapshot",
  "description": "ISO-BMFF: ftyp typed, decoded per-track sample tables, everything else typed-raw retained.",
  "type": "object",
  "required": ["schema", "ftyp", "tracks", "unknownBoxes"],
  "properties": {
    "schema": { "type": "string", "x-semio-state": "persistent" },
    "ftyp": {
      "type": "object", "x-semio-state": "persistent",
      "properties": { "majorBrand": { "type": "string" }, "minorVersion": { "type": "integer" }, "compatibleBrands": { "type": "array", "items": { "type": "string" } } }
    },
    "tracks": {
      "type": "array", "x-semio-state": "persistent",
      "items": {
        "type": "object",
        "properties": {
          "trackId": { "type": "integer" }, "timescale": { "type": "integer" },
          "codec": { "oneOf": [
            { "type": "object", "properties": { "codec": { "const": "avc" }, "sps": { "type": "array" }, "pps": { "type": "array" }, "nalLengthSize": { "type": "integer" } } },
            { "type": "object", "properties": { "codec": { "const": "other" }, "fourcc": { "type": "string" }, "raw": { "type": "array", "items": { "type": "integer" } } } }
          ] },
          "width": { "type": "integer" }, "height": { "type": "integer" },
          "samples": { "type": "array", "items": {
            "type": "object",
            "properties": { "data": { "type": "array", "items": { "type": "integer" } }, "duration": { "type": "integer" }, "ctsOffset": { "type": "integer" }, "sync": { "type": "boolean" } }
          } }
        }
      }
    },
    "unknownBoxes": { "type": "array", "x-semio-state": "persistent", "items": { "type": "object", "properties": { "fourcc": { "type": "string" }, "data": { "type": "array", "items": { "type": "integer" } } } } }
  }
}
""")

    write(f"{base}/🛰️component.proto", """\
syntax = "proto3";
package semio.stdio_mp4.snapshot;

message Mp4Ftyp {
  string major_brand = 1;
  uint32 minor_version = 2;
  repeated string compatible_brands = 3;
}
message Mp4CodecAvc { repeated bytes sps = 1; repeated bytes pps = 2; uint32 nal_length_size = 3; }
message Mp4CodecOther { string fourcc = 1; bytes raw = 2; }
message Mp4Codec { oneof codec { Mp4CodecAvc avc = 1; Mp4CodecOther other = 2; } }
message Mp4Sample { bytes data = 1; uint32 duration = 2; int32 cts_offset = 3; bool sync = 4; }
message Mp4Track {
  uint32 track_id = 1;
  uint32 timescale = 2;
  Mp4Codec codec = 3;
  uint32 width = 4;
  uint32 height = 5;
  repeated Mp4Sample samples = 6;
}
message Mp4Box { string fourcc = 1; bytes data = 2; }
message Mp4Snapshot {
  string schema = 1;
  Mp4Ftyp ftyp = 2;
  repeated Mp4Track tracks = 3;
  repeated Mp4Box unknown_boxes = 4;
}
""")

    # 📝️text — the DSL text form IS a hex dump of the real ISO-BMFF bytes engine::{decode_mp4,encode_mp4}
    # produce/consume (mirrors stdio.png's own honesty-boundary pattern exactly — see that
    # artifact's 📝️text/🦀️component.rs doc comment).
    tbase = f"{base}/📝️text"
    write(f"{tbase}/🦀️component.rs", """\
//! 📝️ Text representation codec surface for `stdio.mp4` (snapshot).

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
""")
    write(f"{tbase}/🅰️component.g4", """\
// 🅰️ ANTLR grammar for `stdio.mp4`'s DSL text representation (store::ArtifactDsl::parse_dsl /
// print_dsl). MP4/ISO-BMFF has no textual syntax of its own — the DSL text IS a whitespace-
// tolerant ASCII hex dump of the REAL binary ISO-BMFF bytes `⚙️engine::{decode_mp4,encode_mp4}`
// produce/consume (see ../💾️binary/🥋️component.ksy for that binary's own real box grammar).
grammar Stdio_mp4_snapshot;

document : hexByte (WS? hexByte)* EOF ;
hexByte  : HEXDIGIT HEXDIGIT ;

HEXDIGIT : [0-9a-fA-F] ;
WS       : [ \\t\\r\\n]+ ;
""")
    write(f"{tbase}/🔤️component.ebnf", """\
(* stdio.mp4 snapshot text form — whitespace-tolerant ASCII hex dump of the real ISO-BMFF bytes;
   MP4 has no textual syntax of its own (see ../💾️binary/🥋️component.ksy for the real grammar). *)
document = hex_byte , { ws , hex_byte } ;
hex_byte = hex_digit , hex_digit ;
hex_digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "a" | "b" | "c" | "d" | "e" | "f" | "A" | "B" | "C" | "D" | "E" | "F" ;
ws = { " " | "\\t" | "\\r" | "\\n" } ;
""")
    write(f"{tbase}/📖️component.grammar.semio", """\
dialect grammar stdio.mp4.snapshot
root = document
; preamble line + its trailing newline are stripped before this grammar runs
; (store::semio_format::split_text_preamble). MP4 has no textual syntax of its own — the DSL
; text is a whitespace-tolerant ASCII hex dump of the REAL binary ISO-BMFF bytes; see
; ../💾️binary/🥋️component.ksy for that binary's own honest box-tree grammar.
document = hex_byte (WS hex_byte)*
hex_byte = HEXDIG HEXDIG
HEXDIG = %x30-39 / %x41-46 / %x61-66
WS = *(%x20 / %x09 / %x0D / %x0A)
""")
    write(f"{tbase}/🔗️component.graphql", "# stdio.mp4 snapshot text facet — same shape as the schema-level Mp4Snapshot (../🔗️component.graphql); the DSL wire form is a hex dump, not a distinct text grammar.\n" + open(f"{base}/🔗️component.graphql", encoding="utf-8").read())
    write(f"{tbase}/🔣️component.json", open(f"{base}/🔣️component.json", encoding="utf-8").read())
    write(f"{tbase}/🛰️component.proto", open(f"{base}/🛰️component.proto", encoding="utf-8").read().replace("package semio.stdio_mp4.snapshot;", "package semio.stdio_mp4.snapshot.text;"))
    write(f"{tbase}/🟦️component.ts", "// stdio.mp4 snapshot text facet — same shape as ../🟦️component.ts; the DSL wire form is a hex dump of the real bytes, not a distinct text grammar.\n" + open(f"{base}/🟦️component.ts", encoding="utf-8").read())

    bbase = f"{base}/💾️binary"
    write(f"{bbase}/🦀️component.rs", """\
//! 💾️ Binary representation codec surface for `stdio.mp4` (snapshot).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
""")
    write(f"{bbase}/🥋️component.ksy", """\
meta:
  id: stdio_mp4_snapshot
  endian: be
doc: |
  Shared `.semio` binary envelope (store::semio_format::wrap_binary) wrapping a `stdio.mp4`
  payload: the REAL ISO-BMFF file bytes `crate::artifacts::mp4::standards::isobmff::engine::
  {decode_mp4,encode_mp4}` produce/consume — `ftyp` typed, `moov`/`trak`/`stbl` walked for real
  per-sample tables, `mdat` sample bytes copied verbatim, everything else typed-raw.
seq:
  - id: envelope_magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
    endian: le
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
    doc: "stdio.mp4.pack v1"
  - id: boxes
    type: iso_bmff_box
    repeat: eos
    doc: The real top-level ISO-BMFF box stream (ftyp, free/uuid/..., mdat, moov).
types:
  iso_bmff_box:
    doc: One ISO/IEC 14496-12 box — 32-bit size (or 64-bit largesize when size==1) + 4-byte type.
    seq:
      - id: size32
        type: u4
      - id: fourcc
        type: str
        size: 4
        encoding: ASCII
      - id: largesize
        type: u8
        if: size32 == 1
      - id: body
        size: (size32 == 1 ? largesize : (size32 == 0 ? -1 : size32)) - (size32 == 1 ? 16 : 8)
  ftyp_body:
    doc: "§4.3 FileTypeBox — major_brand + minor_version + compatible_brands*."
    seq:
      - id: major_brand
        type: str
        size: 4
      - id: minor_version
        type: u4
      - id: compatible_brands
        type: str
        size: 4
        repeat: eos
  moov_trak_tkhd:
    doc: "§8.3.2 TrackHeaderBox — this codec reads only track_id (see engine module doc comment
      for the documented normal-form scope: matrix/volume/timestamps are not retained)."
    seq:
      - id: version
        type: u1
      - id: flags
        size: 3
      - id: creation_and_modification_time
        size: "version == 1 ? 16 : 8"
      - id: track_id
        type: u4
  stbl_stsd_avc1_avcc:
    doc: "ISO/IEC 14496-15 AVCDecoderConfigurationRecord — configurationVersion, AVCProfileIndication,
      profile_compatibility, AVCLevelIndication, lengthSizeMinusOne, then SPS*/PPS* each u2-length-prefixed."
    seq:
      - id: configuration_version
        type: u1
      - id: avc_profile_indication
        type: u1
      - id: profile_compatibility
        type: u1
      - id: avc_level_indication
        type: u1
      - id: length_size_minus_one_reserved
        type: u1
      - id: num_sps_reserved
        type: u1
""")
    write(f"{bbase}/🌶️component.spicy", """\
module Stdio_mp4_snapshot;
# Real ISO-BMFF box-stream grammar (Zeek/Spicy) for the payload wrapped by the shared `.semio`
# binary envelope — see ../🥋️component.ksy for the fully typed field layout this mirrors.

public type EnvelopeHeader = unit {
    magic: bytes &size=8;
    token_len: uint32;
    token: bytes &size=self.token_len;
};

public type IsoBmffBox = unit {
    size32: uint32;
    fourcc: bytes &size=4;
    body: bytes &size=(self.size32 > 8 ? self.size32 - 8 : 0);
};

public type FtypBody = unit {
    major_brand: bytes &size=4;
    minor_version: uint32;
    compatible_brands: bytes &eod;
};
""")
    write(f"{bbase}/🔠️component.abnf", """\
; abnf stdio.mp4 snapshot (binary) — the shared `.semio` binary envelope wrapping the REAL
; ISO-BMFF file bytes (store::semio_format::wrap_binary; magic/token-len/token per that codec).
document      = envelope-magic token-len token iso-bmff-box-stream
envelope-magic = %x89 %x53 %x45 %x4D %x0D %x0A %x1A %x0A
token-len     = 4OCTET                                  ; u32, little-endian
token         = 18OCTET                                 ; UTF-8 "stdio.mp4.pack v1"

; the real ISO/IEC 14496-12 box stream.
iso-bmff-box-stream = *iso-bmff-box
iso-bmff-box  = box-size box-type box-body
box-size      = 4OCTET                                  ; u32 big-endian; 0 = "to EOF", 1 = 64-bit largesize follows
box-type      = 4ALPHA                                  ; e.g. "ftyp" "moov" "trak" "mdia" "minf" "stbl" "mdat" "free"
box-body      = *OCTET                                  ; exactly (box-size - 8) bytes (nested boxes for container types)

; §4.3 ftyp body.
ftyp-body     = major-brand minor-version *compatible-brand
major-brand   = 4ALPHA
minor-version = 4OCTET
compatible-brand = 4ALPHA

; §8.5.2 stsd first sample entry, avc1 case's avcC child (ISO/IEC 14496-15).
avcc-body     = config-version profile compat level length-size-byte num-sps *sps-entry num-pps *pps-entry
config-version = %x01
sps-entry     = 2OCTET *OCTET                           ; u16 length + NAL bytes
pps-entry     = 2OCTET *OCTET
""")
    write(f"{bbase}/📡️component.protocol.semio", """\
dialect protocol stdio.mp4.snapshot
; shared `.semio` binary envelope (store::semio_format::wrap_binary) wrapping the REAL
; ISO-BMFF file bytes this artifact's engine (⚙️engine::{decode_mp4,encode_mp4}) decodes/encodes.
envelope-magic = %x89.53.45.4D.0D.0A.1A.0A
token-len = U32LE
token = UTF8(18)                            ; "stdio.mp4.pack v1"
iso-bmff-box-stream = iso-bmff-box*
iso-bmff-box = size32:U32BE fourcc:ASCII(4) body:BYTES(size32 - 8)
; §4.3 ftyp — see ../🥋️component.ksy for the fully typed per-box bodies (tkhd/avcC/stbl tables).
ftyp-body = major-brand:ASCII(4) minor-version:U32BE compatible-brand:ASCII(4)*
""")
    write(f"{bbase}/🟦️component.ts", "// stdio.mp4 snapshot binary facet — same shape as ../🟦️component.ts.\n" + open(f"{base}/🟦️component.ts", encoding="utf-8").read())
#endregion

#region MP4 facets — diff
def gen_mp4_diff():
    base = f"{MP4_SCHEMA}/🔺️diff"
    write(f"{base}/🟦️component.ts", """\
/** 🔺️ Mp4Diff — sparse per-field diff. Mirrors 🦀️component.rs field-for-field. */
export interface IndexedModified<D> { index: number; diff: D; }
export interface IndexedAdded<T> { index: number; item: T; }
export interface IndexedDiff<T, D> { removed: number[]; modified: IndexedModified<D>[]; added: IndexedAdded<T>[]; }

export interface Mp4SampleDiff { data?: number[]; duration?: number; ctsOffset?: number; sync?: boolean; }
export interface Mp4TrackDiff {
  trackId?: number; timescale?: number; codec?: import("../📸️snapshot/🟦️component").Mp4Codec;
  width?: number; height?: number; samples?: IndexedDiff<import("../📸️snapshot/🟦️component").Mp4Sample, Mp4SampleDiff>;
}
export interface Mp4Diff {
  ftyp?: import("../📸️snapshot/🟦️component").Mp4Ftyp;
  tracks?: IndexedDiff<import("../📸️snapshot/🟦️component").Mp4Track, Mp4TrackDiff>;
  unknownBoxes?: IndexedDiff<import("../📸️snapshot/🟦️component").Mp4Box, import("../📸️snapshot/🟦️component").Mp4Box>;
}
""")
    write(f"{base}/🔗️component.graphql", """\
type Mp4SampleDiff { data: [Int!] duration: Int ctsOffset: Int sync: Boolean }
type Mp4TrackDiff { trackId: Int timescale: Int codec: Mp4Codec width: Int height: Int samplesRemoved: [Int!] }
type Mp4Diff {
  ftyp: Mp4Ftyp
  tracksRemoved: [Int!]
  unknownBoxesRemoved: [Int!]
}
""")
    write(f"{base}/🔣️component.json", """\
{
  "$id": "https://semio.tech/schema/stdio.mp4.diff.json",
  "title": "Mp4Diff",
  "description": "Sparse per-field diff: ftyp whole-replace, tracks/unknownBoxes index-keyed collection triples.",
  "type": "object",
  "properties": {
    "ftyp": { "type": "object" },
    "tracks": { "type": "object", "properties": { "removed": { "type": "array", "items": { "type": "integer" } }, "modified": { "type": "array" }, "added": { "type": "array" } } },
    "unknownBoxes": { "type": "object", "properties": { "removed": { "type": "array", "items": { "type": "integer" } }, "modified": { "type": "array" }, "added": { "type": "array" } } }
  }
}
""")
    write(f"{base}/🛰️component.proto", """\
syntax = "proto3";
package semio.stdio_mp4.diff;
import "snapshot.proto";

message Mp4SampleDiff { optional bytes data = 1; optional uint32 duration = 2; optional int32 cts_offset = 3; optional bool sync = 4; }
message IndexedModifiedSample { uint32 index = 1; Mp4SampleDiff diff = 2; }
message IndexedAddedSample { uint32 index = 1; semio.stdio_mp4.snapshot.Mp4Sample item = 2; }
message Mp4SamplesDiff { repeated uint32 removed = 1; repeated IndexedModifiedSample modified = 2; repeated IndexedAddedSample added = 3; }

message Mp4TrackDiff {
  optional uint32 track_id = 1; optional uint32 timescale = 2;
  optional semio.stdio_mp4.snapshot.Mp4Codec codec = 3;
  optional uint32 width = 4; optional uint32 height = 5;
  optional Mp4SamplesDiff samples = 6;
}
message IndexedModifiedTrack { uint32 index = 1; Mp4TrackDiff diff = 2; }
message IndexedAddedTrack { uint32 index = 1; semio.stdio_mp4.snapshot.Mp4Track item = 2; }
message Mp4TracksDiff { repeated uint32 removed = 1; repeated IndexedModifiedTrack modified = 2; repeated IndexedAddedTrack added = 3; }

message Mp4Diff {
  semio.stdio_mp4.snapshot.Mp4Ftyp ftyp = 1;
  Mp4TracksDiff tracks = 2;
}
""")
    for sub in ["📝️text", "💾️binary"]:
        sbase = f"{base}/{sub}"
        if sub == "📝️text":
            write(f"{sbase}/🦀️component.rs", """\
//! 📝️ Text representation codec surface for `stdio.mp4` (diff).

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
""")
            write(f"{sbase}/🅰️component.g4", """\
// 🅰️ ANTLR grammar for stdio.mp4's diff text form — mirrors ../../📸️snapshot/📝️text/🅰️component.g4:
// a whitespace-tolerant hex dump, this time of the JSON-serialized Mp4Diff (op codecs are the
// handcrafted `OpText`/`OpBinary` JSON round-trip in 🧬️mutations/🦀️component.rs, not a bespoke
// diff-text grammar — the mutation vocabulary IS the diff's textual protocol).
grammar Stdio_mp4_diff;
document : hexByte (WS? hexByte)* EOF ;
hexByte  : HEXDIGIT HEXDIGIT ;
HEXDIGIT : [0-9a-fA-F] ;
WS       : [ \\t\\r\\n]+ ;
""")
            write(f"{sbase}/🔤️component.ebnf", "(* stdio.mp4 diff text form — see ../🅰️component.g4 *)\ndocument = hex_byte , { ws , hex_byte } ;\nhex_byte = hex_digit , hex_digit ;\nhex_digit = \"0\" | \"1\" | \"2\" | \"3\" | \"4\" | \"5\" | \"6\" | \"7\" | \"8\" | \"9\" | \"a\" | \"b\" | \"c\" | \"d\" | \"e\" | \"f\" ;\nws = { \" \" } ;\n")
            write(f"{sbase}/📖️component.grammar.semio", "dialect grammar stdio.mp4.diff\nroot = document\n; op codecs (📄set-snapshot and every real mutation variant) are the handcrafted OpText/OpBinary\n; JSON round-trip in 🧬️mutations/🦀️component.rs; this facet documents the same hex-of-bytes shape.\ndocument = hex_byte (WS hex_byte)*\nhex_byte = HEXDIG HEXDIG\nHEXDIG = %x30-39 / %x41-46 / %x61-66\nWS = *(%x20 / %x09 / %x0D / %x0A)\n")
        else:
            write(f"{sbase}/🦀️component.rs", """\
//! 💾️ Binary representation codec surface for `stdio.mp4` (diff).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
""")
            write(f"{sbase}/🥋️component.ksy", "meta:\n  id: stdio_mp4_diff\n  endian: be\ndoc: |\n  Op-level binary framing for Mp4Mutation (protocol::OpBinary::encode_op/decode_op in\n  🧬️mutations/🦀️component.rs) — one JSON-serialized mutation per encoded op, length-independent\n  (the caller's op-log framing supplies length); this leaf documents the payload shape only.\nseq:\n  - id: json_utf8\n    type: str\n    size-eos: true\n    encoding: UTF-8\n    doc: One compact JSON-serialized Mp4Mutation (tagged enum, camelCase fields).\n")
            write(f"{sbase}/🌶️component.spicy", "module Stdio_mp4_diff;\n# Op-binary payload: one JSON-serialized Mp4Mutation per record (see 🧬️mutations/🦀️component.rs's\n# hand-rolled OpBinary impl — this is a documentation leaf, not a distinct wire codec).\npublic type Op = unit {\n    json_utf8: bytes &eod;\n};\n")
            write(f"{sbase}/🔠️component.abnf", "; abnf stdio.mp4 diff (binary) — one JSON-serialized Mp4Mutation op payload (see\n; 🧬️mutations/🦀️component.rs's OpBinary impl).\ndocument = *OCTET                                    ; UTF-8 JSON bytes\n")
            write(f"{sbase}/📡️component.protocol.semio", "dialect protocol stdio.mp4.diff\n; op-binary payload: one JSON-serialized Mp4Mutation per record.\ndocument = json:UTF8(*)\n")
        write(f"{sbase}/🔗️component.graphql", f"# stdio.mp4 diff {sub} facet — same shape as ../🔗️component.graphql.\n" + open(f"{base}/🔗️component.graphql", encoding="utf-8").read())
        write(f"{sbase}/🔣️component.json", open(f"{base}/🔣️component.json", encoding="utf-8").read())
        write(f"{sbase}/🛰️component.proto", open(f"{base}/🛰️component.proto", encoding="utf-8").read())
        write(f"{sbase}/🟦️component.ts", f"// stdio.mp4 diff {sub} facet — same shape as ../🟦️component.ts.\n" + open(f"{base}/🟦️component.ts", encoding="utf-8").read())
#endregion

#region MP4 facets — mutations
def gen_mp4_mutations():
    base = f"{MP4_SCHEMA}/🧬️mutations"
    write(f"{base}/🟦️component.ts", """\
/** 🧬️ Mp4Mutation — named-variant vocabulary. Mirrors 🦀️component.rs field-for-field. */
export type Mp4Mutation =
  | { mutation: "noMutation" }
  | { mutation: "setSnapshot"; snapshot: import("../📸️snapshot/🟦️component").Mp4Snapshot }
  | { mutation: "setFtyp"; ftyp: import("../📸️snapshot/🟦️component").Mp4Ftyp }
  | { mutation: "insertTrack"; index: number; track: import("../📸️snapshot/🟦️component").Mp4Track }
  | { mutation: "removeTrack"; index: number }
  | { mutation: "setTrackDimensions"; trackIndex: number; width: number; height: number }
  | { mutation: "setTrackCodec"; trackIndex: number; codec: import("../📸️snapshot/🟦️component").Mp4Codec }
  | { mutation: "insertSample"; trackIndex: number; index: number; sample: import("../📸️snapshot/🟦️component").Mp4Sample }
  | { mutation: "removeSample"; trackIndex: number; index: number }
  | { mutation: "setSampleSync"; trackIndex: number; index: number; sync: boolean }
  | { mutation: "addUnknownBox"; index: number; item: import("../📸️snapshot/🟦️component").Mp4Box }
  | { mutation: "removeUnknownBox"; index: number };
""")
    write(f"{base}/🔗️component.graphql", """\
enum Mp4MutationKind { NO_MUTATION SET_SNAPSHOT SET_FTYP INSERT_TRACK REMOVE_TRACK SET_TRACK_DIMENSIONS SET_TRACK_CODEC INSERT_SAMPLE REMOVE_SAMPLE SET_SAMPLE_SYNC ADD_UNKNOWN_BOX REMOVE_UNKNOWN_BOX }
type Mp4Mutation {
  mutation: Mp4MutationKind!
  snapshot: Mp4Snapshot
  ftyp: Mp4Ftyp
  index: Int
  trackIndex: Int
  width: Int
  height: Int
  sync: Boolean
}
""")
    write(f"{base}/🔣️component.json", """\
{
  "$id": "https://semio.tech/schema/stdio.mp4.mutations.json",
  "title": "Mp4Mutation",
  "description": "Named-variant mutation vocabulary, discriminated by the `mutation` tag.",
  "type": "object",
  "required": ["mutation"],
  "properties": {
    "mutation": { "enum": ["noMutation", "setSnapshot", "setFtyp", "insertTrack", "removeTrack", "setTrackDimensions", "setTrackCodec", "insertSample", "removeSample", "setSampleSync", "addUnknownBox", "removeUnknownBox"] }
  }
}
""")
    write(f"{base}/🛰️component.proto", """\
syntax = "proto3";
package semio.stdio_mp4.mutations;
import "snapshot.proto";

message Mp4Mutation {
  oneof mutation {
    bool no_mutation = 1;
    semio.stdio_mp4.snapshot.Mp4Snapshot set_snapshot = 2;
    semio.stdio_mp4.snapshot.Mp4Ftyp set_ftyp = 3;
    InsertTrack insert_track = 4;
    uint32 remove_track = 5;
    SetTrackDimensions set_track_dimensions = 6;
    SetTrackCodec set_track_codec = 7;
    InsertSample insert_sample = 8;
    RemoveSample remove_sample = 9;
    SetSampleSync set_sample_sync = 10;
    AddUnknownBox add_unknown_box = 11;
    uint32 remove_unknown_box = 12;
  }
}
message InsertTrack { uint32 index = 1; semio.stdio_mp4.snapshot.Mp4Track track = 2; }
message SetTrackDimensions { uint32 track_index = 1; uint32 width = 2; uint32 height = 3; }
message SetTrackCodec { uint32 track_index = 1; semio.stdio_mp4.snapshot.Mp4Codec codec = 2; }
message InsertSample { uint32 track_index = 1; uint32 index = 2; semio.stdio_mp4.snapshot.Mp4Sample sample = 3; }
message RemoveSample { uint32 track_index = 1; uint32 index = 2; }
message SetSampleSync { uint32 track_index = 1; uint32 index = 2; bool sync = 3; }
message AddUnknownBox { uint32 index = 1; semio.stdio_mp4.snapshot.Mp4Box item = 2; }
""")
    for sub in ["📝️text", "💾️binary"]:
        sbase = f"{base}/{sub}"
        if sub == "📝️text":
            write(f"{sbase}/🦀️component.rs", "//! 📝️ Text representation codec surface for `stdio.mp4` (mutations) — the real op text\n//! codec is `protocol::OpText` in ../🦀️component.rs (`print_op`/`parse_op`, one compact JSON\n//! line per op); this leaf documents that shape via the grammar file below.\n\npub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!(\"📖️component.grammar.semio\");\npub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), \"::📖️component.grammar.semio\");\n")
            write(f"{sbase}/🅰️component.g4", "// 🅰️ ANTLR grammar for stdio.mp4's op text form (protocol::OpText::print_op/parse_op in\n// ../🦀️component.rs): one compact, single-line JSON object per op, tagged by \"mutation\".\ngrammar Stdio_mp4_mutations;\ndocument : jsonLine EOF ;\njsonLine : ~[\\r\\n]+ ;\n")
            write(f"{sbase}/🔤️component.ebnf", "(* stdio.mp4 op text form — one compact single-line JSON object per op *)\ndocument = json_line ;\njson_line = { any_char_except_newline } ;\n")
            write(f"{sbase}/📖️component.grammar.semio", "dialect grammar stdio.mp4.mutations\nroot = document\n; protocol::OpText::print_op/parse_op (../🦀️component.rs): one compact single-line JSON\n; object per op, tagged by \"mutation\" (camelCase field names, matching the Rust enum's\n; #[serde(tag=\"mutation\", rename_all=\"camelCase\")]).\ndocument = json_line\njson_line = *(%x20-10FFFF)  ; any non-newline UTF-8 text\n")
        else:
            write(f"{sbase}/🦀️component.rs", "//! 💾️ Binary representation codec surface for `stdio.mp4` (mutations) — the real op binary\n//! codec is `protocol::OpBinary` in ../🦀️component.rs (`encode_op`/`decode_op`, JSON bytes).\n\npub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!(\"📡️component.protocol.semio\");\npub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), \"::📡️component.protocol.semio\");\n")
            write(f"{sbase}/🥋️component.ksy", "meta:\n  id: stdio_mp4_mutations\n  endian: be\ndoc: |\n  protocol::OpBinary::encode_op/decode_op (../🦀️component.rs): one JSON-serialized Mp4Mutation\n  per record (length supplied by the caller's op-log framing).\nseq:\n  - id: json_utf8\n    type: str\n    size-eos: true\n    encoding: UTF-8\n")
            write(f"{sbase}/🌶️component.spicy", "module Stdio_mp4_mutations;\npublic type Op = unit {\n    json_utf8: bytes &eod;\n};\n")
            write(f"{sbase}/🔠️component.abnf", "; abnf stdio.mp4 mutations (binary) — one JSON-serialized Mp4Mutation op payload.\ndocument = *OCTET                                    ; UTF-8 JSON bytes\n")
            write(f"{sbase}/📡️component.protocol.semio", "dialect protocol stdio.mp4.mutations\ndocument = json:UTF8(*)\n")
        write(f"{sbase}/🔗️component.graphql", f"# stdio.mp4 mutations {sub} facet — same shape as ../🔗️component.graphql.\n" + open(f"{base}/🔗️component.graphql", encoding="utf-8").read())
        write(f"{sbase}/🔣️component.json", open(f"{base}/🔣️component.json", encoding="utf-8").read())
        write(f"{sbase}/🛰️component.proto", open(f"{base}/🛰️component.proto", encoding="utf-8").read())
        write(f"{sbase}/🟦️component.ts", f"// stdio.mp4 mutations {sub} facet — same shape as ../🟦️component.ts.\n" + open(f"{base}/🟦️component.ts", encoding="utf-8").read())
#endregion

#region MP4 facets — artifact root
def gen_mp4_artifact():
    write(f"{MP4_SCHEMA}/🔗️component.graphql", "# Mp4Artifact — full artifact state, mirrors Mp4Snapshot field for field.\ntype Mp4Artifact {\n  schema: String! @state(class: PERSISTENT)\n  ftyp: Mp4Ftyp! @state(class: PERSISTENT)\n  tracks: [Mp4Track!]! @state(class: PERSISTENT)\n  unknownBoxes: [Mp4Box!]! @state(class: PERSISTENT)\n}\n")
    write(f"{MP4_SCHEMA}/🔣️component.json", '{\n  "$id": "https://semio.tech/schema/stdio.mp4.json",\n  "title": "Mp4Artifact",\n  "description": "Full artifact state, mirrors Mp4Snapshot field for field.",\n  "type": "object",\n  "properties": {\n    "schema": { "type": "string", "x-semio-state": "persistent" },\n    "ftyp": { "type": "object", "x-semio-state": "persistent" },\n    "tracks": { "type": "array", "x-semio-state": "persistent" },\n    "unknownBoxes": { "type": "array", "x-semio-state": "persistent" }\n  }\n}\n')
    write(f"{MP4_SCHEMA}/🛰️component.proto", 'syntax = "proto3";\npackage semio.stdio_mp4;\nimport "snapshot.proto";\n\nmessage Mp4Artifact {\n  string schema = 1;\n  semio.stdio_mp4.snapshot.Mp4Ftyp ftyp = 2;\n  repeated semio.stdio_mp4.snapshot.Mp4Track tracks = 3;\n  repeated semio.stdio_mp4.snapshot.Mp4Box unknown_boxes = 4;\n}\n')
    write(f"{MP4_SCHEMA}/🟦️component.ts", '/** 🧬️ Mp4Artifact — full artifact state, mirrors Mp4Snapshot field for field. */\nexport interface Mp4Artifact {\n  schema: string;\n  ftyp: import("./📸️snapshot/🟦️component").Mp4Ftyp;\n  tracks: import("./📸️snapshot/🟦️component").Mp4Track[];\n  unknownBoxes: import("./📸️snapshot/🟦️component").Mp4Box[];\n}\n')
#endregion

#region AVI facets — snapshot
def gen_avi_snapshot():
    base = f"{AVI_SCHEMA}/📸️snapshot"
    write(f"{base}/🟦️component.ts", """\
/** 🧬️ AviSnapshot — RIFF/AVI 1.0. Mirrors 🦀️component.rs field-for-field. */
export interface AviMainHeader {
  microSecPerFrame: number; maxBytesPerSec: number; paddingGranularity: number; flags: number;
  totalFrames: number; initialFrames: number; streams: number; suggestedBufferSize: number;
  width: number; height: number; reserved: number[];
}
export interface AviStreamHeader {
  fccType: string; fccHandler: string; flags: number; priority: number; language: number;
  initialFrames: number; scale: number; rate: number; start: number; length: number;
  suggestedBufferSize: number; quality: number; sampleSize: number;
  rcFrameLeft: number; rcFrameTop: number; rcFrameRight: number; rcFrameBottom: number;
}
export type AviStreamFormat =
  | { format: "bitmapInfo"; size: number; width: number; height: number; planes: number; bitCount: number; compression: string; sizeImage: number; xPelsPerMeter: number; yPelsPerMeter: number; colorsUsed: number; colorsImportant: number }
  | { format: "waveFormat"; formatTag: number; channels: number; samplesPerSec: number; avgBytesPerSec: number; blockAlign: number; bitsPerSample: number; extra: number[] }
  | { format: "raw"; data: number[] };
export interface AviChunk { fourcc: string; data: number[]; keyframe: boolean; }
export interface AviStream { strh: AviStreamHeader; strf: AviStreamFormat; chunks: AviChunk[]; }
export interface RiffChunk { fourcc: string; data: number[]; }
export interface AviSnapshot {
""" + "\n".join([ts_field("schema", "string"), ts_field("mainHeader", "AviMainHeader"), ts_field("streams", "AviStream[]"), ts_field("idx1Present", "boolean"), ts_field("unknownChunks", "RiffChunk[]")]) + "\n}\n")

    write(f"{base}/🔗️component.graphql", """\
type AviMainHeader { microSecPerFrame: Int! maxBytesPerSec: Int! paddingGranularity: Int! flags: Int! totalFrames: Int! initialFrames: Int! streams: Int! suggestedBufferSize: Int! width: Int! height: Int! reserved: [Int!]! }
type AviStreamHeader { fccType: String! fccHandler: String! flags: Int! priority: Int! language: Int! initialFrames: Int! scale: Int! rate: Int! start: Int! length: Int! suggestedBufferSize: Int! quality: Int! sampleSize: Int! rcFrameLeft: Int! rcFrameTop: Int! rcFrameRight: Int! rcFrameBottom: Int! }
union AviStreamFormat = AviBitmapInfo | AviWaveFormat | AviRawFormat
type AviBitmapInfo { size: Int! width: Int! height: Int! planes: Int! bitCount: Int! compression: String! sizeImage: Int! xPelsPerMeter: Int! yPelsPerMeter: Int! colorsUsed: Int! colorsImportant: Int! }
type AviWaveFormat { formatTag: Int! channels: Int! samplesPerSec: Int! avgBytesPerSec: Int! blockAlign: Int! bitsPerSample: Int! extra: [Int!]! }
type AviRawFormat { data: [Int!]! }
type AviChunk { fourcc: String! data: [Int!]! keyframe: Boolean! }
type AviStream { strh: AviStreamHeader! strf: AviStreamFormat! chunks: [AviChunk!]! }
type RiffChunk { fourcc: String! data: [Int!]! }
type AviSnapshot {
  schema: String! @state(class: PERSISTENT)
  mainHeader: AviMainHeader! @state(class: PERSISTENT)
  streams: [AviStream!]! @state(class: PERSISTENT)
  idx1Present: Boolean! @state(class: PERSISTENT)
  unknownChunks: [RiffChunk!]! @state(class: PERSISTENT)
}
""")

    write(f"{base}/🔣️component.json", """\
{
  "$id": "https://semio.tech/schema/stdio.avi.snapshot.json",
  "title": "AviSnapshot",
  "description": "RIFF/AVI 1.0: avih typed, per-stream strh typed + strf discriminated by fccType, movi chunks assigned to their stream with idx1-derived keyframe flags, everything else typed-raw.",
  "type": "object",
  "required": ["schema", "mainHeader", "streams", "idx1Present", "unknownChunks"],
  "properties": {
    "schema": { "type": "string", "x-semio-state": "persistent" },
    "mainHeader": {
      "type": "object", "x-semio-state": "persistent",
      "properties": {
        "microSecPerFrame": { "type": "integer" }, "maxBytesPerSec": { "type": "integer" }, "paddingGranularity": { "type": "integer" }, "flags": { "type": "integer" },
        "totalFrames": { "type": "integer" }, "initialFrames": { "type": "integer" }, "streams": { "type": "integer" }, "suggestedBufferSize": { "type": "integer" },
        "width": { "type": "integer" }, "height": { "type": "integer" }, "reserved": { "type": "array", "items": { "type": "integer" } }
      }
    },
    "streams": {
      "type": "array", "x-semio-state": "persistent",
      "items": {
        "type": "object",
        "properties": {
          "strh": { "type": "object" },
          "strf": { "oneOf": [
            { "type": "object", "properties": { "format": { "const": "bitmapInfo" } } },
            { "type": "object", "properties": { "format": { "const": "waveFormat" } } },
            { "type": "object", "properties": { "format": { "const": "raw" } } }
          ] },
          "chunks": { "type": "array", "items": { "type": "object", "properties": { "fourcc": { "type": "string" }, "data": { "type": "array" }, "keyframe": { "type": "boolean" } } } }
        }
      }
    },
    "idx1Present": { "type": "boolean", "x-semio-state": "persistent" },
    "unknownChunks": { "type": "array", "x-semio-state": "persistent", "items": { "type": "object", "properties": { "fourcc": { "type": "string" }, "data": { "type": "array" } } } }
  }
}
""")

    write(f"{base}/🛰️component.proto", """\
syntax = "proto3";
package semio.stdio_avi.snapshot;

message AviMainHeader {
  uint32 micro_sec_per_frame = 1; uint32 max_bytes_per_sec = 2; uint32 padding_granularity = 3; uint32 flags = 4;
  uint32 total_frames = 5; uint32 initial_frames = 6; uint32 streams = 7; uint32 suggested_buffer_size = 8;
  uint32 width = 9; uint32 height = 10; repeated uint32 reserved = 11;
}
message AviStreamHeader {
  string fcc_type = 1; string fcc_handler = 2; uint32 flags = 3; uint32 priority = 4; uint32 language = 5;
  uint32 initial_frames = 6; uint32 scale = 7; uint32 rate = 8; uint32 start = 9; uint32 length = 10;
  uint32 suggested_buffer_size = 11; int32 quality = 12; uint32 sample_size = 13;
  int32 rc_frame_left = 14; int32 rc_frame_top = 15; int32 rc_frame_right = 16; int32 rc_frame_bottom = 17;
}
message AviBitmapInfo { uint32 size = 1; int32 width = 2; int32 height = 3; uint32 planes = 4; uint32 bit_count = 5; string compression = 6; uint32 size_image = 7; int32 x_pels_per_meter = 8; int32 y_pels_per_meter = 9; uint32 colors_used = 10; uint32 colors_important = 11; }
message AviWaveFormat { uint32 format_tag = 1; uint32 channels = 2; uint32 samples_per_sec = 3; uint32 avg_bytes_per_sec = 4; uint32 block_align = 5; uint32 bits_per_sample = 6; bytes extra = 7; }
message AviRawFormat { bytes data = 1; }
message AviStreamFormat { oneof format { AviBitmapInfo bitmap_info = 1; AviWaveFormat wave_format = 2; AviRawFormat raw = 3; } }
message AviChunk { string fourcc = 1; bytes data = 2; bool keyframe = 3; }
message AviStream { AviStreamHeader strh = 1; AviStreamFormat strf = 2; repeated AviChunk chunks = 3; }
message RiffChunk { string fourcc = 1; bytes data = 2; }
message AviSnapshot {
  string schema = 1;
  AviMainHeader main_header = 2;
  repeated AviStream streams = 3;
  bool idx1_present = 4;
  repeated RiffChunk unknown_chunks = 5;
}
""")

    tbase = f"{base}/📝️text"
    write(f"{tbase}/🦀️component.rs", "//! 📝️ Text representation codec surface for `stdio.avi` (snapshot).\n\npub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!(\"📖️component.grammar.semio\");\npub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), \"::📖️component.grammar.semio\");\n")
    write(f"{tbase}/🅰️component.g4", "// 🅰️ ANTLR grammar for `stdio.avi`'s DSL text representation. RIFF/AVI has no textual syntax\n// of its own — the DSL text IS a whitespace-tolerant ASCII hex dump of the REAL binary RIFF\n// bytes `⚙️engine::{decode_avi,encode_avi}` produce/consume (see ../💾️binary/🥋️component.ksy).\ngrammar Stdio_avi_snapshot;\ndocument : hexByte (WS? hexByte)* EOF ;\nhexByte  : HEXDIGIT HEXDIGIT ;\nHEXDIGIT : [0-9a-fA-F] ;\nWS       : [ \\t\\r\\n]+ ;\n")
    write(f"{tbase}/🔤️component.ebnf", "(* stdio.avi snapshot text form — whitespace-tolerant ASCII hex dump of the real RIFF bytes;\n   see ../💾️binary/🥋️component.ksy for the real grammar. *)\ndocument = hex_byte , { ws , hex_byte } ;\nhex_byte = hex_digit , hex_digit ;\nhex_digit = \"0\" | \"1\" | \"2\" | \"3\" | \"4\" | \"5\" | \"6\" | \"7\" | \"8\" | \"9\" | \"a\" | \"b\" | \"c\" | \"d\" | \"e\" | \"f\" ;\nws = { \" \" | \"\\t\" | \"\\r\" | \"\\n\" } ;\n")
    write(f"{tbase}/📖️component.grammar.semio", "dialect grammar stdio.avi.snapshot\nroot = document\n; preamble stripped before this runs (store::semio_format::split_text_preamble). RIFF/AVI has\n; no textual syntax of its own — the DSL text is a whitespace-tolerant ASCII hex dump of the\n; REAL binary RIFF bytes; see ../💾️binary/🥋️component.ksy for the real chunk-tree grammar.\ndocument = hex_byte (WS hex_byte)*\nhex_byte = HEXDIG HEXDIG\nHEXDIG = %x30-39 / %x41-46 / %x61-66\nWS = *(%x20 / %x09 / %x0D / %x0A)\n")
    write(f"{tbase}/🔗️component.graphql", "# stdio.avi snapshot text facet — same shape as ../🔗️component.graphql; the DSL wire form is a hex dump.\n" + open(f"{base}/🔗️component.graphql", encoding="utf-8").read())
    write(f"{tbase}/🔣️component.json", open(f"{base}/🔣️component.json", encoding="utf-8").read())
    write(f"{tbase}/🛰️component.proto", open(f"{base}/🛰️component.proto", encoding="utf-8").read().replace("package semio.stdio_avi.snapshot;", "package semio.stdio_avi.snapshot.text;"))
    write(f"{tbase}/🟦️component.ts", "// stdio.avi snapshot text facet — same shape as ../🟦️component.ts; hex-dump wire form.\n" + open(f"{base}/🟦️component.ts", encoding="utf-8").read())

    bbase = f"{base}/💾️binary"
    write(f"{bbase}/🦀️component.rs", "//! 💾️ Binary representation codec surface for `stdio.avi` (snapshot).\n\npub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!(\"📡️component.protocol.semio\");\npub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), \"::📡️component.protocol.semio\");\n")
    write(f"{bbase}/🥋️component.ksy", """\
meta:
  id: stdio_avi_snapshot
  endian: le
doc: |
  Shared `.semio` binary envelope (store::semio_format::wrap_binary, big-endian magic per that
  codec) wrapping a `stdio.avi` payload: the REAL little-endian RIFF/AVI 1.0 file bytes
  `crate::artifacts::avi::standards::v1_0::engine::{decode_avi,encode_avi}` produce/consume.
seq:
  - id: envelope_magic
    endian: be
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
    doc: "stdio.avi.pack v1"
  - id: riff
    type: riff_chunk
    doc: The real RIFF('AVI ' hdrl movi idx1) container.
types:
  riff_chunk:
    seq:
      - id: fourcc
        type: str
        size: 4
        encoding: ASCII
      - id: chunk_size
        type: u4
      - id: body
        size: chunk_size
      - id: pad
        size: chunk_size % 2
  main_avi_header:
    doc: "MainAVIHeader — 14 DWORDs, 56 bytes."
    seq:
      - id: micro_sec_per_frame
        type: u4
      - id: max_bytes_per_sec
        type: u4
      - id: padding_granularity
        type: u4
      - id: flags
        type: u4
      - id: total_frames
        type: u4
      - id: initial_frames
        type: u4
      - id: streams
        type: u4
      - id: suggested_buffer_size
        type: u4
      - id: width
        type: u4
      - id: height
        type: u4
      - id: reserved
        type: u4
        repeat: expr
        repeat-expr: 4
  avi_stream_header:
    doc: "AVIStreamHeader — 64 bytes; rcFrame is 4 LONGs (not SHORTs)."
    seq:
      - id: fcc_type
        type: str
        size: 4
        encoding: ASCII
      - id: fcc_handler
        type: str
        size: 4
        encoding: ASCII
      - id: flags
        type: u4
      - id: priority
        type: u2
      - id: language
        type: u2
      - id: initial_frames
        type: u4
      - id: scale
        type: u4
      - id: rate
        type: u4
      - id: start
        type: u4
      - id: length
        type: u4
      - id: suggested_buffer_size
        type: u4
      - id: quality
        type: s4
      - id: sample_size
        type: u4
      - id: rc_frame_left
        type: s4
      - id: rc_frame_top
        type: s4
      - id: rc_frame_right
        type: s4
      - id: rc_frame_bottom
        type: s4
  bitmap_info_header:
    doc: "BITMAPINFOHEADER — 40 bytes."
    seq:
      - id: size
        type: u4
      - id: width
        type: s4
      - id: height
        type: s4
      - id: planes
        type: u2
      - id: bit_count
        type: u2
      - id: compression
        type: str
        size: 4
        encoding: ASCII
      - id: size_image
        type: u4
      - id: x_pels_per_meter
        type: s4
      - id: y_pels_per_meter
        type: s4
      - id: colors_used
        type: u4
      - id: colors_important
        type: u4
""")
    write(f"{bbase}/🌶️component.spicy", """\
module Stdio_avi_snapshot;
# Real little-endian RIFF/AVI grammar for the payload wrapped by the shared `.semio` binary
# envelope — see ../🥋️component.ksy for the fully typed field layout this mirrors.

public type RiffChunkHeader = unit {
    fourcc: bytes &size=4;
    chunk_size: uint32 &byte-order=spicy::ByteOrder::Little;
};

public type MainAviHeader = unit {
    micro_sec_per_frame: uint32 &byte-order=spicy::ByteOrder::Little;
    max_bytes_per_sec: uint32 &byte-order=spicy::ByteOrder::Little;
    padding_granularity: uint32 &byte-order=spicy::ByteOrder::Little;
    flags: uint32 &byte-order=spicy::ByteOrder::Little;
    total_frames: uint32 &byte-order=spicy::ByteOrder::Little;
    initial_frames: uint32 &byte-order=spicy::ByteOrder::Little;
    streams: uint32 &byte-order=spicy::ByteOrder::Little;
    suggested_buffer_size: uint32 &byte-order=spicy::ByteOrder::Little;
    width: uint32 &byte-order=spicy::ByteOrder::Little;
    height: uint32 &byte-order=spicy::ByteOrder::Little;
};
""")
    write(f"{bbase}/🔠️component.abnf", """\
; abnf stdio.avi snapshot (binary) — shared `.semio` envelope wrapping the REAL little-endian
; RIFF/AVI 1.0 file bytes (store::semio_format::wrap_binary).
document      = envelope-magic token-len token riff-avi
envelope-magic = %x89 %x53 %x45 %x4D %x0D %x0A %x1A %x0A
token-len     = 4OCTET                                 ; u32 little-endian
token         = 18OCTET                                ; UTF-8 "stdio.avi.pack v1"

riff-avi      = "RIFF" riff-size "AVI " *riff-entry
riff-size     = 4OCTET                                 ; u32 little-endian
riff-entry    = riff-list / riff-chunk
riff-list     = "LIST" 4OCTET 4ALPHA *riff-entry        ; size, list-type, children
riff-chunk    = 4ALPHA 4OCTET *OCTET [pad-byte]         ; fourcc, size (LE), data, even-pad

; hdrl: avih (56 bytes, 14 DWORDs) + strl* (strh 64 bytes + strf, format-dependent).
avih-body     = 10(4OCTET) 4(4OCTET)                    ; 10 typed fields + dwReserved[4]
strh-body     = 4ALPHA 4ALPHA 4OCTET 2OCTET 2OCTET 4OCTET 4OCTET 4OCTET 4OCTET 4OCTET 4OCTET 4OCTET 4OCTET 4(4OCTET)
bitmapinfo-body = 4OCTET 4OCTET 4OCTET 2OCTET 2OCTET 4ALPHA 4OCTET 4OCTET 4OCTET 4OCTET 4OCTET

; idx1: AVIOLDINDEX, 16-byte entries.
idx1-entry    = 4ALPHA 4OCTET 4OCTET 4OCTET             ; ckid, flags, offset, size
pad-byte      = %x00
""")
    write(f"{bbase}/📡️component.protocol.semio", """\
dialect protocol stdio.avi.snapshot
; shared `.semio` binary envelope wrapping the REAL little-endian RIFF/AVI 1.0 file bytes this
; artifact's engine (⚙️engine::{decode_avi,encode_avi}) decodes/encodes.
envelope-magic = %x89.53.45.4D.0D.0A.1A.0A
token-len = U32LE
token = UTF8(18)                              ; "stdio.avi.pack v1"
riff-avi = magic:ASCII(4) size:U32LE form:ASCII(4) entries:riff-entry*
riff-entry = riff-list | riff-chunk
riff-list = tag:ASCII(4)="LIST" size:U32LE list-type:ASCII(4) children:riff-entry*
riff-chunk = fourcc:ASCII(4) size:U32LE data:BYTES(size) pad:BYTES(size % 2)
; avih/strh/strf/idx1 fully typed fields — see ../🥋️component.ksy for the exhaustive layout.
""")
    write(f"{bbase}/🟦️component.ts", "// stdio.avi snapshot binary facet — same shape as ../🟦️component.ts.\n" + open(f"{base}/🟦️component.ts", encoding="utf-8").read())
#endregion

#region AVI facets — diff
def gen_avi_diff():
    base = f"{AVI_SCHEMA}/🔺️diff"
    write(f"{base}/🟦️component.ts", """\
/** 🔺️ AviDiff — sparse per-field diff. Mirrors 🦀️component.rs field-for-field. */
export interface IndexedModified<D> { index: number; diff: D; }
export interface IndexedAdded<T> { index: number; item: T; }
export interface IndexedDiff<T, D> { removed: number[]; modified: IndexedModified<D>[]; added: IndexedAdded<T>[]; }

export interface AviChunkDiff { data?: number[]; keyframe?: boolean; }
export interface AviStreamDiff {
  strh?: import("../📸️snapshot/🟦️component").AviStreamHeader;
  strf?: import("../📸️snapshot/🟦️component").AviStreamFormat;
  chunks?: IndexedDiff<import("../📸️snapshot/🟦️component").AviChunk, AviChunkDiff>;
}
export interface AviDiff {
  mainHeader?: import("../📸️snapshot/🟦️component").AviMainHeader;
  streams?: IndexedDiff<import("../📸️snapshot/🟦️component").AviStream, AviStreamDiff>;
  idx1Present?: boolean;
  unknownChunks?: IndexedDiff<import("../📸️snapshot/🟦️component").RiffChunk, import("../📸️snapshot/🟦️component").RiffChunk>;
}
""")
    write(f"{base}/🔗️component.graphql", """\
type AviChunkDiff { data: [Int!] keyframe: Boolean }
type AviStreamDiff { strh: AviStreamHeader strf: AviStreamFormat chunksRemoved: [Int!] }
type AviDiff {
  mainHeader: AviMainHeader
  streamsRemoved: [Int!]
  idx1Present: Boolean
  unknownChunksRemoved: [Int!]
}
""")
    write(f"{base}/🔣️component.json", """\
{
  "$id": "https://semio.tech/schema/stdio.avi.diff.json",
  "title": "AviDiff",
  "description": "Sparse per-field diff: mainHeader whole-replace, streams/unknownChunks index-keyed collection triples.",
  "type": "object",
  "properties": {
    "mainHeader": { "type": "object" },
    "streams": { "type": "object", "properties": { "removed": { "type": "array" }, "modified": { "type": "array" }, "added": { "type": "array" } } },
    "idx1Present": { "type": "boolean" },
    "unknownChunks": { "type": "object", "properties": { "removed": { "type": "array" }, "modified": { "type": "array" }, "added": { "type": "array" } } }
  }
}
""")
    write(f"{base}/🛰️component.proto", """\
syntax = "proto3";
package semio.stdio_avi.diff;
import "snapshot.proto";

message AviChunkDiff { optional bytes data = 1; optional bool keyframe = 2; }
message IndexedModifiedChunk { uint32 index = 1; AviChunkDiff diff = 2; }
message IndexedAddedChunk { uint32 index = 1; semio.stdio_avi.snapshot.AviChunk item = 2; }
message AviChunksDiff { repeated uint32 removed = 1; repeated IndexedModifiedChunk modified = 2; repeated IndexedAddedChunk added = 3; }

message AviStreamDiff {
  optional semio.stdio_avi.snapshot.AviStreamHeader strh = 1;
  optional semio.stdio_avi.snapshot.AviStreamFormat strf = 2;
  optional AviChunksDiff chunks = 3;
}
message IndexedModifiedStream { uint32 index = 1; AviStreamDiff diff = 2; }
message IndexedAddedStream { uint32 index = 1; semio.stdio_avi.snapshot.AviStream item = 2; }
message AviStreamsDiff { repeated uint32 removed = 1; repeated IndexedModifiedStream modified = 2; repeated IndexedAddedStream added = 3; }

message AviDiff {
  semio.stdio_avi.snapshot.AviMainHeader main_header = 1;
  AviStreamsDiff streams = 2;
  optional bool idx1_present = 3;
}
""")
    for sub in ["📝️text", "💾️binary"]:
        sbase = f"{base}/{sub}"
        if sub == "📝️text":
            write(f"{sbase}/🦀️component.rs", "//! 📝️ Text representation codec surface for `stdio.avi` (diff).\n\npub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!(\"📖️component.grammar.semio\");\npub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), \"::📖️component.grammar.semio\");\n")
            write(f"{sbase}/🅰️component.g4", "// 🅰️ ANTLR grammar for stdio.avi's diff text form — op codecs are the handcrafted\n// OpText/OpBinary JSON round-trip in 🧬️mutations/🦀️component.rs.\ngrammar Stdio_avi_diff;\ndocument : hexByte (WS? hexByte)* EOF ;\nhexByte  : HEXDIGIT HEXDIGIT ;\nHEXDIGIT : [0-9a-fA-F] ;\nWS       : [ \\t\\r\\n]+ ;\n")
            write(f"{sbase}/🔤️component.ebnf", "(* stdio.avi diff text form — see ../🅰️component.g4 *)\ndocument = hex_byte , { ws , hex_byte } ;\nhex_byte = hex_digit , hex_digit ;\nhex_digit = \"0\" | \"1\" | \"2\" | \"3\" | \"4\" | \"5\" | \"6\" | \"7\" | \"8\" | \"9\" | \"a\" | \"b\" | \"c\" | \"d\" | \"e\" | \"f\" ;\nws = { \" \" } ;\n")
            write(f"{sbase}/📖️component.grammar.semio", "dialect grammar stdio.avi.diff\nroot = document\n; op codecs are the handcrafted OpText/OpBinary JSON round-trip in 🧬️mutations/🦀️component.rs.\ndocument = hex_byte (WS hex_byte)*\nhex_byte = HEXDIG HEXDIG\nHEXDIG = %x30-39 / %x41-46 / %x61-66\nWS = *(%x20 / %x09 / %x0D / %x0A)\n")
        else:
            write(f"{sbase}/🦀️component.rs", "//! 💾️ Binary representation codec surface for `stdio.avi` (diff).\n\npub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!(\"📡️component.protocol.semio\");\npub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), \"::📡️component.protocol.semio\");\n")
            write(f"{sbase}/🥋️component.ksy", "meta:\n  id: stdio_avi_diff\n  endian: be\ndoc: |\n  Op-level binary framing for AviMutation (protocol::OpBinary in 🧬️mutations/🦀️component.rs) —\n  one JSON-serialized mutation per encoded op.\nseq:\n  - id: json_utf8\n    type: str\n    size-eos: true\n    encoding: UTF-8\n")
            write(f"{sbase}/🌶️component.spicy", "module Stdio_avi_diff;\npublic type Op = unit {\n    json_utf8: bytes &eod;\n};\n")
            write(f"{sbase}/🔠️component.abnf", "; abnf stdio.avi diff (binary) — one JSON-serialized AviMutation op payload.\ndocument = *OCTET\n")
            write(f"{sbase}/📡️component.protocol.semio", "dialect protocol stdio.avi.diff\ndocument = json:UTF8(*)\n")
        write(f"{sbase}/🔗️component.graphql", f"# stdio.avi diff {sub} facet — same shape as ../🔗️component.graphql.\n" + open(f"{base}/🔗️component.graphql", encoding="utf-8").read())
        write(f"{sbase}/🔣️component.json", open(f"{base}/🔣️component.json", encoding="utf-8").read())
        write(f"{sbase}/🛰️component.proto", open(f"{base}/🛰️component.proto", encoding="utf-8").read())
        write(f"{sbase}/🟦️component.ts", f"// stdio.avi diff {sub} facet — same shape as ../🟦️component.ts.\n" + open(f"{base}/🟦️component.ts", encoding="utf-8").read())
#endregion

#region AVI facets — mutations
def gen_avi_mutations():
    base = f"{AVI_SCHEMA}/🧬️mutations"
    write(f"{base}/🟦️component.ts", """\
/** 🧬️ AviMutation — named-variant vocabulary. Mirrors 🦀️component.rs field-for-field. */
export type AviMutation =
  | { mutation: "noMutation" }
  | { mutation: "setSnapshot"; snapshot: import("../📸️snapshot/🟦️component").AviSnapshot }
  | { mutation: "setMainHeader"; mainHeader: import("../📸️snapshot/🟦️component").AviMainHeader }
  | { mutation: "setIdx1Present"; idx1Present: boolean }
  | { mutation: "insertStream"; index: number; stream: import("../📸️snapshot/🟦️component").AviStream }
  | { mutation: "removeStream"; index: number }
  | { mutation: "setStreamHeader"; streamIndex: number; strh: import("../📸️snapshot/🟦️component").AviStreamHeader }
  | { mutation: "setStreamFormat"; streamIndex: number; strf: import("../📸️snapshot/🟦️component").AviStreamFormat }
  | { mutation: "insertChunk"; streamIndex: number; index: number; chunk: import("../📸️snapshot/🟦️component").AviChunk }
  | { mutation: "removeChunk"; streamIndex: number; index: number }
  | { mutation: "setChunkKeyframe"; streamIndex: number; index: number; keyframe: boolean }
  | { mutation: "addUnknownChunk"; index: number; item: import("../📸️snapshot/🟦️component").RiffChunk }
  | { mutation: "removeUnknownChunk"; index: number };
""")
    write(f"{base}/🔗️component.graphql", """\
enum AviMutationKind { NO_MUTATION SET_SNAPSHOT SET_MAIN_HEADER SET_IDX1_PRESENT INSERT_STREAM REMOVE_STREAM SET_STREAM_HEADER SET_STREAM_FORMAT INSERT_CHUNK REMOVE_CHUNK SET_CHUNK_KEYFRAME ADD_UNKNOWN_CHUNK REMOVE_UNKNOWN_CHUNK }
type AviMutation {
  mutation: AviMutationKind!
  snapshot: AviSnapshot
  mainHeader: AviMainHeader
  index: Int
  streamIndex: Int
  keyframe: Boolean
  idx1Present: Boolean
}
""")
    write(f"{base}/🔣️component.json", """\
{
  "$id": "https://semio.tech/schema/stdio.avi.mutations.json",
  "title": "AviMutation",
  "description": "Named-variant mutation vocabulary, discriminated by the `mutation` tag.",
  "type": "object",
  "required": ["mutation"],
  "properties": {
    "mutation": { "enum": ["noMutation", "setSnapshot", "setMainHeader", "setIdx1Present", "insertStream", "removeStream", "setStreamHeader", "setStreamFormat", "insertChunk", "removeChunk", "setChunkKeyframe", "addUnknownChunk", "removeUnknownChunk"] }
  }
}
""")
    write(f"{base}/🛰️component.proto", """\
syntax = "proto3";
package semio.stdio_avi.mutations;
import "snapshot.proto";

message AviMutation {
  oneof mutation {
    bool no_mutation = 1;
    semio.stdio_avi.snapshot.AviSnapshot set_snapshot = 2;
    semio.stdio_avi.snapshot.AviMainHeader set_main_header = 3;
    bool set_idx1_present = 4;
    InsertStream insert_stream = 5;
    uint32 remove_stream = 6;
    SetStreamHeader set_stream_header = 7;
    SetStreamFormat set_stream_format = 8;
    InsertChunk insert_chunk = 9;
    RemoveChunk remove_chunk = 10;
    SetChunkKeyframe set_chunk_keyframe = 11;
    AddUnknownChunk add_unknown_chunk = 12;
    uint32 remove_unknown_chunk = 13;
  }
}
message InsertStream { uint32 index = 1; semio.stdio_avi.snapshot.AviStream stream = 2; }
message SetStreamHeader { uint32 stream_index = 1; semio.stdio_avi.snapshot.AviStreamHeader strh = 2; }
message SetStreamFormat { uint32 stream_index = 1; semio.stdio_avi.snapshot.AviStreamFormat strf = 2; }
message InsertChunk { uint32 stream_index = 1; uint32 index = 2; semio.stdio_avi.snapshot.AviChunk chunk = 3; }
message RemoveChunk { uint32 stream_index = 1; uint32 index = 2; }
message SetChunkKeyframe { uint32 stream_index = 1; uint32 index = 2; bool keyframe = 3; }
message AddUnknownChunk { uint32 index = 1; semio.stdio_avi.snapshot.RiffChunk item = 2; }
""")
    for sub in ["📝️text", "💾️binary"]:
        sbase = f"{base}/{sub}"
        if sub == "📝️text":
            write(f"{sbase}/🦀️component.rs", "//! 📝️ Text representation codec surface for `stdio.avi` (mutations) — the real op text\n//! codec is `protocol::OpText` in ../🦀️component.rs.\n\npub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!(\"📖️component.grammar.semio\");\npub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), \"::📖️component.grammar.semio\");\n")
            write(f"{sbase}/🅰️component.g4", "// 🅰️ ANTLR grammar for stdio.avi's op text form (protocol::OpText in ../🦀️component.rs):\n// one compact single-line JSON object per op, tagged by \"mutation\".\ngrammar Stdio_avi_mutations;\ndocument : jsonLine EOF ;\njsonLine : ~[\\r\\n]+ ;\n")
            write(f"{sbase}/🔤️component.ebnf", "(* stdio.avi op text form — one compact single-line JSON object per op *)\ndocument = json_line ;\njson_line = { any_char_except_newline } ;\n")
            write(f"{sbase}/📖️component.grammar.semio", "dialect grammar stdio.avi.mutations\nroot = document\n; protocol::OpText (../🦀️component.rs): one compact single-line JSON object per op, tagged\n; by \"mutation\" (camelCase field names).\ndocument = json_line\njson_line = *(%x20-10FFFF)\n")
        else:
            write(f"{sbase}/🦀️component.rs", "//! 💾️ Binary representation codec surface for `stdio.avi` (mutations) — the real op binary\n//! codec is `protocol::OpBinary` in ../🦀️component.rs.\n\npub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!(\"📡️component.protocol.semio\");\npub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), \"::📡️component.protocol.semio\");\n")
            write(f"{sbase}/🥋️component.ksy", "meta:\n  id: stdio_avi_mutations\n  endian: be\ndoc: |\n  protocol::OpBinary (../🦀️component.rs): one JSON-serialized AviMutation per record.\nseq:\n  - id: json_utf8\n    type: str\n    size-eos: true\n    encoding: UTF-8\n")
            write(f"{sbase}/🌶️component.spicy", "module Stdio_avi_mutations;\npublic type Op = unit {\n    json_utf8: bytes &eod;\n};\n")
            write(f"{sbase}/🔠️component.abnf", "; abnf stdio.avi mutations (binary) — one JSON-serialized AviMutation op payload.\ndocument = *OCTET\n")
            write(f"{sbase}/📡️component.protocol.semio", "dialect protocol stdio.avi.mutations\ndocument = json:UTF8(*)\n")
        write(f"{sbase}/🔗️component.graphql", f"# stdio.avi mutations {sub} facet — same shape as ../🔗️component.graphql.\n" + open(f"{base}/🔗️component.graphql", encoding="utf-8").read())
        write(f"{sbase}/🔣️component.json", open(f"{base}/🔣️component.json", encoding="utf-8").read())
        write(f"{sbase}/🛰️component.proto", open(f"{base}/🛰️component.proto", encoding="utf-8").read())
        write(f"{sbase}/🟦️component.ts", f"// stdio.avi mutations {sub} facet — same shape as ../🟦️component.ts.\n" + open(f"{base}/🟦️component.ts", encoding="utf-8").read())
#endregion

#region AVI facets — artifact root
def gen_avi_artifact():
    write(f"{AVI_SCHEMA}/🔗️component.graphql", "# AviArtifact — full artifact state, mirrors AviSnapshot field for field.\ntype AviArtifact {\n  schema: String! @state(class: PERSISTENT)\n  mainHeader: AviMainHeader! @state(class: PERSISTENT)\n  streams: [AviStream!]! @state(class: PERSISTENT)\n  idx1Present: Boolean! @state(class: PERSISTENT)\n  unknownChunks: [RiffChunk!]! @state(class: PERSISTENT)\n}\n")
    write(f"{AVI_SCHEMA}/🔣️component.json", '{\n  "$id": "https://semio.tech/schema/stdio.avi.json",\n  "title": "AviArtifact",\n  "description": "Full artifact state, mirrors AviSnapshot field for field.",\n  "type": "object",\n  "properties": {\n    "schema": { "type": "string", "x-semio-state": "persistent" },\n    "mainHeader": { "type": "object", "x-semio-state": "persistent" },\n    "streams": { "type": "array", "x-semio-state": "persistent" },\n    "idx1Present": { "type": "boolean", "x-semio-state": "persistent" },\n    "unknownChunks": { "type": "array", "x-semio-state": "persistent" }\n  }\n}\n')
    write(f"{AVI_SCHEMA}/🛰️component.proto", 'syntax = "proto3";\npackage semio.stdio_avi;\nimport "snapshot.proto";\n\nmessage AviArtifact {\n  string schema = 1;\n  semio.stdio_avi.snapshot.AviMainHeader main_header = 2;\n  repeated semio.stdio_avi.snapshot.AviStream streams = 3;\n  bool idx1_present = 4;\n  repeated semio.stdio_avi.snapshot.RiffChunk unknown_chunks = 5;\n}\n')
    write(f"{AVI_SCHEMA}/🟦️component.ts", '/** 🧬️ AviArtifact — full artifact state, mirrors AviSnapshot field for field. */\nexport interface AviArtifact {\n  schema: string;\n  mainHeader: import("./📸️snapshot/🟦️component").AviMainHeader;\n  streams: import("./📸️snapshot/🟦️component").AviStream[];\n  idx1Present: boolean;\n  unknownChunks: import("./📸️snapshot/🟦️component").RiffChunk[];\n}\n')
#endregion

if __name__ == "__main__":
    gen_mp4_snapshot()
    gen_mp4_diff()
    gen_mp4_mutations()
    gen_mp4_artifact()
    gen_avi_snapshot()
    gen_avi_diff()
    gen_avi_mutations()
    gen_avi_artifact()
    print("mp4 + avi facets written")
