"""✂️ Splits the pack format module into the core, retained (admission cursors) and catalog (catalog cursors) crates (writes into the stage tree)."""
import re
from lib import *
from owners import *
from rules import Rules
from crates import *
import usetree, fmtsplit

FMT = P + "📐️format/"
CORE_PUBS = ("FOOTER_MAGIC", "FOOTER_SIZE", "FORMAT_VERSION_MAJOR", "Footer", "HEADER_SIZE", "Header", "MAGIC", "REQUIRED_KNOWN_MASK", "RawManifest", "Superblock")
ALLOC_LINE = ' assert_eq!(Some(check_allocation_layout::<u64>(usize::MAX,"same deliberately misleading canceled/work/allocation prose").err().unwrap().kind()),Some(ValueRefusalKind::OwnershipLimit));\n'

def run(R):
    base = read_base(FMT + "🦀️.rs")
    before, after, inner = fmtsplit.region(base)
    core_full = before + "\n" + after
    doc, uses, body = split_doc_uses_body(core_full)
    leaves = leaves_of_uses(uses)
    ret_items, cat_items = fmtsplit.partition(inner)

    # ---------------------------------------------------------------- core
    rest, removed = cut_items(body, {"retained_page_close_demands", "original_symbol_storage", "retained_inline_symbols", "producer_authority_tests", "source_producer_authority_tests"}, kinds=("fn", "mod"))
    page_demands = removed[[i for i, t in enumerate(removed) if "retained_page_close_demands" in t][0]]
    rest = rest.replace("const REQUIRED_KNOWN_MASK: u32", "pub const REQUIRED_KNOWN_MASK: u32")
    old_raw = "struct RawManifest {\n    schema_symref: u64,\n    schema_hash: [u8; 32],\n    doc_span: ByteRange,\n    doc_frame_count: u64,\n    symbols_span: ByteRange,\n    chunk_table_span: ByteRange,\n    field_index_span: ByteRange,\n    uncompressed_body_len: u64,\n    field_count: u64,\n    chunk_count: u64,\n    symbol_count: u64,\n}"
    assert rest.count(old_raw) == 1
    rest = rest.replace(old_raw, old_raw.replace("struct RawManifest", "pub struct RawManifest").replace("\n    ", "\n    pub "))
    rest = rest.replace("pub struct RawManifest {\n    pub pub", "pub struct RawManifest {\n    pub")
    rest = rest.replace("pub struct RawManifest {\n    schema_symref", "pub struct RawManifest {\n    pub schema_symref")
    assert "pub struct RawManifest" in rest and "pub const REQUIRED_KNOWN_MASK" in rest
    rest = R.rewrite(rest, "format")
    enc_new = R.rewrite(read_base(FMT + "🛫️encoding/🦀️.rs"), "format")
    enc_doc, enc_uses, enc_body = split_doc_uses_body(enc_new)
    core_tests = read_base(FMT + "🧪️tests/🔬️unit/🦀️.rs") + read_base(FMT + "🧪️tests/🔣️absolute-varint/🦀️.rs")
    core_uses, core_test_uses = build_uses_split(rest + enc_body, core_tests, map_leaves(leaves, R.rule_for("format")))
    write(FMT + "🦀️.rs", doc + "\n\n" + "\n".join(core_uses + core_test_uses) + "\n\n" + rest.lstrip("\n"))
    enc_leaves = map_leaves(leaves_of_uses([u for u in enc_uses if "crate::" in u["text"]]), R.rule_for("format"))
    enc_other = [u["text"] for u in enc_uses if "crate::" not in u["text"]]
    write(FMT + "🛫️encoding/🦀️.rs", enc_doc + "\n" + "\n".join(enc_other + usetree.emit(enc_leaves)) + "\n" + enc_body)

    # ----------------------------------------------- shared leaf candidates
    def cross(here, owner_keys):
        out = []
        for n, k in sorted(format_owners().items()):
            if k in owner_keys and k != here and n not in ("original_symbol_storage",):
                out.append(([ident(k), n], None, False))
        return out
    core_cands = [([ident("format"), n], None, False) for n in CORE_PUBS]

    # ---------------------------------------------------------- retained
    page_demands = R.rewrite(page_demands, "retained").replace("\nfn retained_page_close_demands", "\npub fn retained_page_close_demands")
    if page_demands.startswith("fn retained_page_close_demands"): page_demands = "pub " + page_demands
    assert "pub fn retained_page_close_demands" in page_demands
    ret_body = "\n\n".join(R.rewrite(t, "retained") for t in ret_items)
    for sig in ("    fn into_catalog_fault(self,", "    fn preview(self, byte: u8"):
        assert ret_body.count(sig) == 1, sig
        ret_body = ret_body.replace(sig, "    pub " + sig[4:])
    ret_region = "//#region 🔖️RetainedCanonicalSource\n" + ret_body + "\n//#endregion 🔖️RetainedCanonicalSource"
    ret_tests = ('#[cfg(test)]\n#[path = "../🧪️tests/🔬️retained-pack-source-laws/🦀️.rs"]\nmod retained_pack_source_laws;\n\n'
                 '#[cfg(test)]\n#[path = "../🧭️producer/📥️source/🧪️tests/🦀️.rs"]\nmod source_producer_authority_tests;\n')
    laws_src = read_base(FMT + "🧪️tests/🔬️retained-pack-source-laws/🦀️.rs")
    head, parts, _ = fmtsplit.split_laws(laws_src)
    def law_text(kind_items):
        return "\n\n".join(i["text"] for i in kind_items)
    ret_tests_scan = R.rewrite(read_base(FMT + "🧭️producer/📥️source/🧪️tests/🦀️.rs"), "retained") + R.rewrite(law_text(parts[0]), "retained")
    ret_leaves = map_leaves(leaves, R.rule_for("retained")) + core_cands
    ret_uses_l, ret_uses_t = build_uses_split(ret_region + page_demands, ret_tests_scan, ret_leaves)
    ret_uses = ret_uses_l + ret_uses_t
    ret_doc = ("//! 🪢️ Retained canonical Pack source admission: bounded, cancellable byte-at-a-time cursors for pages, segments, varints and the pack anchors.\n"
               "//! The framing, writer and reader live in `semio-framework-pack-format`; the catalog cursors live in `semio-framework-pack-format-catalog`.")
    ret_text = ret_doc + "\n\n" + "\n".join(ret_uses) + "\n\n" + page_demands.strip("\n") + "\n\n" + ret_region + "\n\n" + ret_tests
    write(FMT + "🎮️retained/🦀️.rs", ret_text)

    # ----------------------------------------------------------- catalog
    cat_body = "\n\n".join(R.rewrite(t, "catalog") for t in cat_items)
    cat_region = "//#region 🔖️RetainedCanonicalSource\n" + cat_body + "\n//#endregion 🔖️RetainedCanonicalSource"
    cat_mounts = ('#[path = "../🏷️symbols/🎟️storage/🦀️.rs"]\nmod original_symbol_storage;\n\n'
                  '#[path = "../🏷️symbols/🎮️decode/🦀️.rs"]\npub mod retained_inline_symbols;\n')
    cat_tests = ('#[cfg(test)]\n#[path = "../🧪️tests/🔬️retained-pack-catalog-laws/🦀️.rs"]\nmod retained_pack_catalog_laws;\n\n'
                 '#[cfg(test)]\n#[path = "../🧭️producer/🗂️catalog/🧪️tests/🦀️.rs"]\nmod producer_authority_tests;\n')
    cat_lib_scan = cat_region + R.rewrite(read_base(FMT + "🏷️symbols/🎟️storage/🦀️.rs"), "catalog") + R.rewrite(read_base(FMT + "🏷️symbols/🎮️decode/🦀️.rs"), "catalog")
    cat_tests_scan = R.rewrite(read_base(FMT + "🧭️producer/🗂️catalog/🧪️tests/🦀️.rs"), "catalog") + R.rewrite(law_text(parts[1]), "catalog")
    cat_leaves = map_leaves(leaves, R.rule_for("catalog")) + core_cands + cross("catalog", {"retained"})
    cat_uses_l, cat_uses_t = build_uses_split(cat_lib_scan, cat_tests_scan, cat_leaves)
    cat_uses = cat_uses_l + cat_uses_t
    cat_doc = ("//! 🗂️ Retained canonical Pack catalog: bounded, cancellable cursors for the manifest, the symbol table, the chunk table and the inline symbols.\n"
               "//! They consume the segment events admitted by `semio-framework-pack-format-retained`.")
    write(FMT + "🗂️catalog/🦀️.rs", cat_doc + "\n\n" + "\n".join(cat_uses) + "\n\n" + cat_mounts + "\n" + cat_region + "\n\n" + cat_tests)

    # symbols children
    st = R.rewrite(read_base(FMT + "🏷️symbols/🎟️storage/🦀️.rs"), "catalog")
    old = "RetainedPackSymbolScalars,RetainedPackSymbolSpan,RetainedPackCatalogFault"
    assert st.count(old) == 1
    write(FMT + "🏷️symbols/🎟️storage/🦀️.rs", st.replace(old, "RetainedPackSymbolScalars,RetainedPackCatalogFault"))
    write(FMT + "🏷️symbols/🎮️decode/🦀️.rs", R.rewrite(read_base(FMT + "🏷️symbols/🎮️decode/🦀️.rs"), "catalog"))

    # tests: names the old format root made visible to `use super::*` children
    all_use_items = [it for it in items(core_full) if it["kind"] == "use"]
    old_leaves = map_leaves(leaves_of_uses(all_use_items), R.rule_for("format"))
    avail_fc = {}
    for l in old_leaves:
        n = usetree.local_name(l)
        if n: avail_fc[n] = l
    fc_items = {it["name"]: it for it in items(core_full) if it["name"] and it["kind"] in ("fn", "struct", "enum", "const", "static", "type", "trait")}
    fc_provided = provided_names(read_stage_or_base(FMT + "🦀️.rs"))
    ret_provided = provided_names(read_stage_or_base(FMT + "🎮️retained/🦀️.rs"))
    cat_provided = provided_names(read_stage_or_base(FMT + "🗂️catalog/🦀️.rs"))
    avail_other = dict(avail_fc)
    for n, it in fc_items.items():
        avail_other.setdefault(n, [ident("format"), n])
    avail_cat = dict(avail_other)
    for n, k in sorted(format_owners().items()):
        if k == "retained": avail_cat.setdefault(n, [ident("retained"), n])
    for f in ("🧪️tests/🔬️unit/🦀️.rs", "🧪️tests/🔣️absolute-varint/🦀️.rs"):
        t0 = R.rewrite(read_base(FMT + f), "format")
        t0, need = augment_uses(t0, avail_fc, fc_provided)
        if f.endswith("🔬️unit/🦀️.rs"):
            t0 = t0.rstrip("\n") + "\n\n#[test]\nfn check_allocation_layout_refuses_counts_beyond_the_address_space(){\n    assert_eq!(Some(check_allocation_layout::<u64>(usize::MAX,\"same deliberately misleading canceled/work/allocation prose\").err().unwrap().kind()),Some(ValueRefusalKind::OwnershipLimit));\n}\n"
        write(FMT + f, t0)
    def prep(rel, here, avail, provided):
        t0 = R.rewrite(read_base(FMT + rel), here)
        if rel.startswith("🧭️producer/📥️source"):
            assert t0.count(ALLOC_LINE) == 1
            t0 = t0.replace(ALLOC_LINE, "")
        t0, need = augment_uses(t0, avail, provided)
        for n in need:
            if n in fc_items: assert fc_items[n]["vis"] == "pub", ("private core item needed", n)
        write(FMT + rel, t0)
        return need
    print("retained producer/source tests add", prep("🧭️producer/📥️source/🧪️tests/🦀️.rs", "retained", avail_other, ret_provided))
    print("catalog producer tests add", prep("🧭️producer/🗂️catalog/🧪️tests/🦀️.rs", "catalog", avail_cat, cat_provided))
    # laws files
    hdr = "\n".join(h["text"] for h in head)
    for tag, part, here, avail, provided, rel in (("retained", parts[0], "retained", avail_other, ret_provided, "🧪️tests/🔬️retained-pack-source-laws/🦀️.rs"), ("catalog", parts[1], "catalog", avail_cat, cat_provided, "🧪️tests/🔬️retained-pack-catalog-laws/🦀️.rs")):
        text = hdr + "\n\n" + "\n\n".join(i["text"] for i in part) + "\n"
        text = R.rewrite(text, here)
        text, need = augment_uses(text, avail, provided)
        print("laws", tag, "adds", need)
        write(FMT + rel, text)
    # symbols decode tests are rewritten by finalize

if __name__ == "__main__":
    import shutil
    shutil.rmtree(STAGE, ignore_errors=True)
    R = Rules({"format": format_owners(), "record": {}})
    run(R)
    print("ok")
