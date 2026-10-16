"""One-shot, assertion-guarded edit of the os-pack value tree (opd): removes the duplicated pack-record-schema items and wires the owner imports.
Re-reads every file at edit time, backs the original up once in the session scratchpad, writes atomically. Usage: dedupe.py [--write]"""
import os, shutil, sys
ROOT = "/Users/ueli/Documents/semio"
SP = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/opdpre/"
V = "🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/"
PK = "🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/"
write = "--write" in sys.argv


def backup(rel):
    b = SP + rel
    if not os.path.exists(b):
        os.makedirs(os.path.dirname(b), exist_ok=True)
        shutil.copyfile(os.path.join(ROOT, rel), b)


def edit(rel, pairs):
    p = os.path.join(ROOT, rel)
    t = open(p, encoding="utf8").read()
    for old, new in pairs:
        assert t.count(old) == 1, (rel, t.count(old), old[:80])
        t = t.replace(old, new, 1)
    print("edit  ", rel[-70:])
    if write:
        backup(rel)
        tmp = p + ".opd-tmp"
        open(tmp, "w", encoding="utf8").write(t)
        os.replace(tmp, p)


def delete(rel):
    p = os.path.join(ROOT, rel)
    assert os.path.isfile(p), rel
    print("delete", rel[-70:])
    if write:
        backup(rel)
        os.remove(p)
        d = os.path.dirname(p)
        while d != os.path.join(ROOT, PK.rstrip("/")) and not os.listdir(d):
            os.rmdir(d)
            d = os.path.dirname(d)


IMPORT = "use semio_framework_pack_record_schema::{controlled_schema,block_inner_shape,elem_shape_of,elem_tag_for_shape,map_inner_shape,schema_hash,schema_hash_controlled,statements_variants,table_spec_of,NumKind,ELEM_BOOL,ELEM_ENUM,ELEM_F64,ELEM_FALLBACK,ELEM_INT,ELEM_STR,ELEM_UINT};\n"
edit(V + "🦀️.rs", [
    ("pub use semio_framework_pack_record_token::*;\n", "pub use semio_framework_pack_record_token::*;\n" + IMPORT),
    ('#[path = "🔑️schema-hash/🦀️.rs"]\nmod schema_graph;\npub use schema_graph::*;\n\n', ""),
    ('#[path="🏭️schema/🦀️.rs"]\nmod controlled_schema;\n\n', ""),
])

CF = V + "🧱️canonical-forms/🦀️.rs"
cf = open(os.path.join(ROOT, CF), encoding="utf8").read()
b1 = cf[cf.index("/// ✂️ Extracts"):cf.index("/// 🧾️ Resolves")]
b2 = cf[cf.index("pub(super) fn block_inner_shape"):cf.index("/// 🚧️ Every shape")]
b3 = cf[cf.index("/// 🔢️ Which packed form"):cf.index("/// 🧮️ A sequence is packed-eligible")]
edit(CF, [(b1, ""), (b2, ""), (b3, "")])
TS = V + "🧮️table-soa/🦀️.rs"
ts = open(os.path.join(ROOT, TS), encoding="utf8").read()
edit(TS, [(ts[ts.index("/// 🏷️ `TableSoA` per-column"):ts.index("/// 📊️ Encodes `Shape::Table`")], "")])

for rel in [V + "🔑️schema-hash/🦀️.rs", V + "🏭️schema/🦀️.rs", V + "🧪️tests/🔬️schema-hash/🦀️.rs", V + "🧪️tests/🔬️schema-hash/💰️storage/🦀️.rs", V + "🧪️tests/🧫️fixtures/🔑️schema-hash/🔣️.json"]:
    delete(rel)

LIBROOT = PK + "🦀️.rs"
edit(LIBROOT, [("pub use pack::*;\n", "pub use pack::*;\n\n#[cfg(test)]\npub(crate) use semio_framework_trace_memory_testing as test_allocation;\n#[cfg(test)]\n#[global_allocator]\nstatic REQUESTED_ALLOCATOR: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;\n")])

CT = PK + "📦️packages/🦀️rust/Cargo.toml"
edit(CT, [
    ('semio-framework-pack-json = { path = "../../../../../../🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust" }\n', ""),
    ('semio-framework-pack-schema-index = { path = "../../../../../../🔨️modules/🎒️pack/🌱️value/🏭️schema/🗂️index/📦️packages/🦀️rust" }\n', ""),
])
print("(written)" if write else "(dry)")
