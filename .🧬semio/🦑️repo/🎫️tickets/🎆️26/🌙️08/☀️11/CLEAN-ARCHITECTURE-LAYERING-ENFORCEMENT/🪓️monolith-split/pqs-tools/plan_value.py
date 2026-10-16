"""value: lift the Canonical, Encode, Decode, Table and Document regions of 🎒️pack/🌱️value/🦀️.rs into children."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from splitter import *

C = lambda **k: k

SPEC = dict(
    target="value",
    imports=(13, 20),
    root_drop=["ByteReader", "ChunkId", "CodecId", "PackLimits", "Number", "ValueError", "ValueRefusalKind", "FieldSpec", "RecordLayout", "RecordSpecProducer", "HashMap", "HashSet", "ELEM_BOOL", "ELEM_ENUM", "ELEM_F64", "ELEM_INT", "ELEM_STR", "ELEM_UINT", "NumKind", "TAG_ABSENT", "TAG_BLOCK", "TAG_BYTES", "TAG_BYTES_CHUNKED", "TAG_ENUM", "TAG_EXPR", "TAG_F64", "TAG_FALSE", "TAG_INT", "TAG_LIST", "TAG_MAP", "TAG_NULL", "TAG_PACKED_F64", "TAG_PACKED_VARINT", "TAG_RECORD", "TAG_STATEMENTS", "TAG_STR", "TAG_STR_INLINE", "TAG_TABLE_SOA", "TAG_TRUE", "TAG_TUPLE", "TAG_UINT", "TAG_VALUE", "TAG_WIRE", "write_varint_i64", "write_varint_u64", "block_inner_shape", "elem_shape_of", "elem_tag_for_shape", "map_inner_shape", "schema_hash_controlled", "statements_variants", "table_spec_of"],
    skip_import=[],
    root_cfg={n: ["#[cfg(test)]"] for n in ["DslValue", "FieldValue", "RecordSpec", "RecordValue", "Shape", "WireEdgeLabel", "WireNode", "WireValue", "schema_hash"]},
    children=[
        C(kind="cluster", mod="canonical_forms", folder="🧾️canonical-forms", ranges=[(22, 254)],
          doc=["//! 🧾️ Canonical-mode helpers: record specs, depth limits, packed-numeric eligibility, the encode context and the deterministic string-interning pre-pass."]),
        C(kind="cluster", mod="record_encode", folder="📝️record-encode", ranges=[(256, 556)],
          doc=["//! 📝️ Wire encoding of `FieldValue`, `DslValue` and `Wire` literals: tag-prefixed, self-describing and canonical."]),
        C(kind="cluster", mod="record_decode", folder="📖️record-decode", ranges=[(558, 1018)],
          doc=["//! 📖️ Wire decoding: the symref and chunk source, the materialization admission owners and the tag-driven value decoders."]),
        C(kind="cluster", mod="table_soa", folder="🧮️table-soa", ranges=[(1020, 1252)],
          doc=["//! 🧮️ `TableSoA` columnar encode and decode for `Shape::Table`."]),
        C(kind="cluster", mod="document_codec", folder="🚪️document-codec", ranges=[(1254, 1254), (1257, 1491)], at=1254,
          doc=["//! 🚪️ Top-level document and record-body entry points, their controlled twins and the decode report."]),
    ],
)


def main():
    plan = Plan(SPEC).build()
    stage(plan)
    gone, added = multiset_report(plan)
    print("files:", {k.split("/")[-2] if k.endswith("🦀️.rs") and "/" in k else k: len(lines_of(v)) for k, v in plan.out.items()})
    print("REMOVED:", sum(gone.values()), json.dumps(dict(gone), ensure_ascii=False)[:3000])
    print("ADDED:", sum(added.values()), json.dumps(dict(added), ensure_ascii=False)[:6000])
    for m, r in plan.report["children"].items():
        print(m, "widened", len(r["widened"]), r["widened"][:120])
        print("   imports", r["imports"])
        print("   super", r["super_imports"])
        print("   pub", r["reexport_pub"])
        print("   priv", r["reexport_priv"])


if __name__ == "__main__":
    main()
