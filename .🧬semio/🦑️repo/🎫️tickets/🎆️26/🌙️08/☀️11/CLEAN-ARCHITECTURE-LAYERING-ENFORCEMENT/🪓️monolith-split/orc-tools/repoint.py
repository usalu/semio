"""Re-points every consumer of the removed os-pack record codec copies to the framework owner path (orc slice).
Re-reads each file at edit time, asserts exact bespoke anchors, backs the original up once in the session scratchpad, writes atomically.
Usage: repoint.py [--write]"""
import os, re, shutil, sys
ROOT = "/Users/ueli/Documents/semio"
SP = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/orcpre/"
M = "🧰️framework/🛍️products/💻️os/🔨️modules/"
write = "--write" in sys.argv

HEAD = r"(?<![\w:])((?:[A-Za-z_][A-Za-z0-9_]*::)*(?:semio_framework_os_pack|os_pack))"
CODEC = "encode_document|decode_document|encode_record_body|decode_record_body|decode_record_body_exact|encode_document_controlled|decode_document_controlled|encode_record_body_controlled|decode_record_body_exact_controlled|EncodeOptions|DecodeOptions"
SCALAR = "ScalarRecordField|ScalarRecordView|ScalarRecordWireStep|ScalarRecordWireWitness"
RULES = [
    ("codec", re.compile(HEAD + r"::(" + CODEC + r")\b"), r"\1::record::\2"),
    ("scalar", re.compile(HEAD + r"::(" + SCALAR + r")\b"), r"\1::record::scalar_witness::\2"),
    ("value", re.compile(HEAD + r"::value::(encode_document|decode_document)\b"), r"\1::record::\2"),
    ("hash", re.compile(HEAD + r"::content_hash\b"), r"\1::stored_content_hash"),
]
ALIAS = re.compile(r"(?<![\w:])pack::(decode_document_controlled|encode_document_controlled)\b")

BESPOKE = {
    M + "🏪️store/📬️command/🦀️.rs": [("use semio_framework_os_pack::{DecodeOptions as PackDecodeOptions, EncodeOptions as PackEncodeOptions, PackError, PackRefusal, pack_rt};", "use semio_framework_os_pack::{PackDecodeOptions, PackEncodeOptions, PackError, PackRefusal, pack_rt};")],
    M + "🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs": [("use semio_framework_os_pack::{pack_rt, DecodeOptions as PackDecodeOptions, EncodeOptions as PackEncodeOptions, PackError};", "use semio_framework_os_pack::{pack_rt, PackDecodeOptions, PackEncodeOptions, PackError};")],
    M + "🏪️store/🚪️io/🦀️.rs": [("use semio_framework_os_pack::{DecodeOptions as PackDecodeOptions, EncodeOptions as PackEncodeOptions};", "use semio_framework_os_pack::{PackDecodeOptions, PackEncodeOptions};")],
    M + "🏪️store/🦀️.rs": [
        ("pub use semio_framework_os_pack::{DecodeOptions as PackDecodeOptions, EncodeOptions as PackEncodeOptions, PackError, PackRefusal, VerificationLevel as PackVerificationLevel};", "pub use semio_framework_os_pack::{PackDecodeOptions, PackEncodeOptions, PackError, PackRefusal, VerificationLevel as PackVerificationLevel};"),
        ("    pub use semio_framework_os_pack::{PackLimits, RetainedRecordBodyCursor, RetainedRecordBodyToken, RetainedValueContainer, RetainedValueCursor, RetainedValueRole, RetainedValueToken};\n", "    pub use semio_framework_os_pack::PackLimits;\n    pub use pack::record::{RetainedRecordBodyCursor, RetainedRecordBodyToken, RetainedValueContainer, RetainedValueCursor, RetainedValueRole, RetainedValueToken};\n"),
    ],
    M + "🏪️store/🧪️testing/🧭️prelude/🦀️.rs": [("pub use semio_framework_os_pack::{DecodeOptions as PackDecodeOptions, EncodeOptions as PackEncodeOptions, pack_rt, PackError, PackRefusal, VerificationLevel as PackVerificationLevel};", "pub use semio_framework_os_pack::{PackDecodeOptions, PackEncodeOptions, pack_rt, PackError, PackRefusal, VerificationLevel as PackVerificationLevel};")],
    M + "🏪️store/🧬️schema/📥️lifted/💾️binary-snapshot/🦀️.rs": [("use semio_framework_os_pack::{DecodeOptions as PackDecodeOptions, EncodeOptions as PackEncodeOptions, PackError, pack_rt};", "use semio_framework_os_pack::{PackDecodeOptions, PackEncodeOptions, PackError, pack_rt};")],
    M + "🏪️store/🧾️document/📥️mounted-pack/🦀️.rs": [("    pub use semio_framework_os_pack::{PackLimits, RetainedRecordBodyCursor, RetainedRecordBodyToken, RetainedValueContainer, RetainedValueCursor, RetainedValueRole, RetainedValueToken};\n", "    pub use semio_framework_os_pack::PackLimits;\n    pub use pack::record::{RetainedRecordBodyCursor, RetainedRecordBodyToken, RetainedValueContainer, RetainedValueCursor, RetainedValueRole, RetainedValueToken};\n")],
    M + "🗣️dsl/🔢️ieee754/🧪️tests/🦀️.rs": [("use semio_framework_os_kernel::os_pack::{self,EncodeOptions,DecodeOptions};", "use semio_framework_os_kernel::os_pack::{self,record::{DecodeOptions,EncodeOptions}};")],
    M + "🗣️dsl/🦀️.rs": [("    use semio_framework_os_pack::{decode_record_body_exact, encode_record_body, write_varint_u64, ByteReader, DecodeOptions, EncodeOptions};\n", "    use semio_framework_os_pack::record::{decode_record_body_exact, encode_record_body, DecodeOptions, EncodeOptions};\n    use semio_framework_os_pack::{write_varint_u64, ByteReader};\n")],
    "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs": [("    use store::os_pack::{ScalarRecordField as Field, ScalarRecordView};\n", "    use store::os_pack::record::scalar_witness::{ScalarRecordField as Field, ScalarRecordView};\n")],
    "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🔎️wire/🧪️tests/🔎️wire/🦀️.rs": [("use store::os_pack::{ScalarRecordWireStep, ScalarRecordWireWitness};\n", "use store::os_pack::record::scalar_witness::{ScalarRecordWireStep, ScalarRecordWireWitness};\n")],
}
ALIAS_FILES = [
    M + "🔌️plugin/🖥️host/🧪️testing/🧩️component/🗿️artifacts/🚫️snapshot-refusal/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs",
    M + "🔌️plugin/🖥️host/🧪️testing/🧩️component/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs",
]
TSV = [l.rstrip("\n").split("\t")[0].rsplit(":", 1)[0] for l in open(os.path.dirname(os.path.abspath(__file__)) + "/../🗑️generated/orc-usescan-removed.tsv", encoding="utf8")]
FILES = sorted(set(TSV) | set(BESPOKE) | set(ALIAS_FILES) | {M + "🎒️pack/⌨️cli/🦀️.rs"})

total = 0
for rel in FILES:
    p = os.path.join(ROOT, rel)
    t = open(p, encoding="utf8").read()
    orig = t
    counts = {}
    for old, new in BESPOKE.get(rel, []):
        assert t.count(old) == 1, (rel, t.count(old), old[:90])
        t = t.replace(old, new, 1)
        counts["bespoke"] = counts.get("bespoke", 0) + 1
    for name, rx, rep in RULES:
        t, k = rx.subn(rep, t)
        if k: counts[name] = k
    if rel in ALIAS_FILES:
        t, k = ALIAS.subn(r"pack::record::\1", t)
        assert k == 2, (rel, k)
        counts["alias"] = k
    if t == orig:
        print("UNCHANGED", rel[-90:]); continue
    total += sum(counts.values())
    print(counts, rel[-100:])
    if write:
        b = SP + rel
        if not os.path.exists(b):
            os.makedirs(os.path.dirname(b), exist_ok=True)
            shutil.copyfile(p, b)
        tmp = p + ".orc-tmp"
        open(tmp, "w", encoding="utf8").write(t)
        os.replace(tmp, p)
print("total edits", total, "files", len(FILES), "(written)" if write else "(dry)")
