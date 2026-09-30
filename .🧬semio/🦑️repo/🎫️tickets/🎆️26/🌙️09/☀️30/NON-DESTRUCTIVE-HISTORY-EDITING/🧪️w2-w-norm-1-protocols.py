#!/usr/bin/env python3
"""🧪️ W2-W norm-1 follow-up: writes the EN 1991 / EN 1990 binary op protocol (`💾️binary/📡️.protocol.semio`, one record per
kind of the aggregate's `KINDS`, the codec's only source of tags) and the frame descriptions beside it (`🥋️.ksy`, `🌶️.spicy`,
`🔠️.abnf`) for the frame `semio_s_artifact_norm_contract::payload_op_binary` writes: `format u8`, `tag u8`, payload JSON.
EN 1990 keeps the tags its hand-rolled codec used; EN 1991 (whose op bytes carried no tag) numbers its kinds in order.

    python3 🧪️w2-w-norm-1-protocols.py
"""
import importlib.util
from pathlib import Path

TICKET = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("vectors", TICKET / "🧪️w2-w-norm-1-vectors.py")
vectors = importlib.util.module_from_spec(spec)
spec.loader.exec_module(vectors)

EN1990_TAGS = {
    "change-annex": 0, "change-project-id": 1, "change-consequence-class": 2, "change-reliability-class": 3,
    "change-design-working-life-category": 4, "change-design-working-life-years": 5, "change-reference-period-years": 6,
    "change-supervision-level": 7, "change-inspection-level": 8, "change-beta-computed": 9, "change-permanents": 10,
    "change-variables": 11, "change-accidentals": 12, "change-seismics": 13, "change-members": 14, "change-effects": 15,
    "insert-permanent": 16, "remove-permanent": 17, "insert-variable": 18, "remove-variable": 19, "insert-accidental": 20,
    "remove-accidental": 21, "insert-seismic": 22, "remove-seismic": 23, "insert-member": 24, "remove-member": 25,
    "insert-effect": 26, "remove-effect": 27, "change-altitude-m": 28, "change-bridge-sls": 29,
}


def protocol(standard, aggregate, tags):
    """📡️ The protocol document: header, frame and one record per kind."""
    records = "".join(f"record {kind} tag={tag}\nfield payload bytes\n" for kind, tag in tags)
    return (
        "dialect protocol\n"
        f"protocol {standard}.mutations\n"
        "version 1\n"
        f"schema norm.{standard}.mutations\n"
        "start record\n"
        "framing record\n"
        "\n"
        f"# `{aggregate}` op frame (`semio_s_artifact_norm_contract::payload_op_binary`): `format u8` (`OP_BINARY_FORMAT`), then\n"
        "# `tag u8`, then the leaf payload (`Mutation::payload_value`) as canonical UTF-8 JSON to the end of the op. Each record is\n"
        "# one mutation kind at its wire tag; this file is the only source of those tags, looked up by the kind's semantic kind.\n"
        "header fixed 2\n"
        "field format u8\n"
        "field tag u8\n"
        f"{records}"
    )


def facets(standard):
    """🧾️ The frame in Kaitai Struct, Spicy and ABNF."""
    name = f"norm_{standard}_mutations"
    return {
        "🥋️.ksy": f"meta:\n  id: {name}\n  endian: le\nseq:\n  - id: format\n    contents: [0x01]\n  - id: tag\n    type: u1\n  - id: payload\n    type: str\n    encoding: UTF-8\n    size-eos: true\n",
        "🌶️.spicy": f"module Norm_{standard}_mutations;\npublic type Op = unit {{\n    format: uint8 &requires=($$ == 1);\n    tag: uint8;\n    payload: bytes &eod;\n}};\n",
        "🔠️.abnf": f"; abnf norm.{standard} mutations — one op frame\nop = format tag payload\nformat = %x01\ntag = OCTET ; the kind's record tag in 📡️.protocol.semio\npayload = *OCTET ; the leaf payload as UTF-8 JSON\n",
    }


if __name__ == "__main__":
    for standard in ("en1991", "en1990"):
        kinds = vectors.rust_kinds(standard)
        tags = [(kind, EN1990_TAGS[kind] if standard == "en1990" else index) for index, kind in enumerate(kinds)]
        assert len({tag for _, tag in tags}) == len(tags) == len(kinds)
        binary = vectors.subset_root(standard) / "🧬️schema/🧬️mutations/💾️binary"
        (binary / "📡️.protocol.semio").write_text(protocol(standard, vectors.STANDARDS[standard]["aggregate"], tags), encoding="utf-8")
        for name, text in facets(standard).items():
            (binary / name).write_text(text, encoding="utf-8")
        print(binary)
