"""✂️ Splits the pack record (value) module into schema, retained, encode, preflight and record crates."""
import re, os
from lib import *
from owners import *
from rules import Rules
from crates import *
import usetree

VAL = P + "🌱️value/"
TRACE_TEST = FW + "⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"

W_FNS = ["elem_shape_of", "table_spec_of", "block_inner_shape", "statements_variants", "map_inner_shape", "elem_tag_for_shape"]
ELEM = ["ELEM_FALLBACK", "ELEM_BOOL", "ELEM_INT", "ELEM_UINT", "ELEM_F64", "ELEM_STR", "ELEM_ENUM"]
DELEGATORS = ["encode_record_body_controlled", "encode_record_body_into", "encode_projected_record_body_into", "encode_value_record_body_controlled", "encode_value_record_body_into", "encode_document_controlled"]
E_EXPORTS = ["ProjectedSymbolScratch", "SourceTextLocator", "SourceTextKind", "BorrowedProjectedPackOperation", "BorrowedPackSource", "OwnedVariantPackSource", "BorrowedProjectedPackCursor", "BorrowedProjectedPackProgress", "BorrowedProjectedPackFailure", "OwnedProjectedPackWireCursor", "OwnedProjectedPackWireReady", "OwnedProjectedPackWireStep"]
RR_EXPORTS = ["RetainedRecordBodySourceDecode", "RetainedRecordBodyReadLimits", "RetainedRecordBodySourceStep", "RetainedRecordReceiving"]
MOUNT_MODS = ["original_record_source", "original_record_receiving", "original_storage", "original_storage_tests", "controlled_encoding", "operation_page_tests", "borrowed_preflight", "intrinsic_media_tests", "borrowed_preflight_tests", "intrinsic_boundary_laws", "schema_storage_tests"]

def pub_ize(text, names, kinds):
    """Makes the named top-level items `pub` (items in `text` that are private); returns new text."""
    for it in items(text):
        if it["name"] in names and it["kind"] in kinds and not it["vis"]:
            pass
    out = text
    for n in names:
        for kw in kinds:
            pat = re.compile(r"(?m)^(" + kw + r"\s+" + re.escape(n) + r"\b)")
            out = pat.sub(lambda m: "pub " + m.group(1), out)
    return out

class Pieces:
    pass

def split_pieces():
    X = Pieces()
    X.base = read_base(VAL + "🦀️.rs")
    X.doc, X.uses, X.body = split_doc_uses_body(X.base)
    X.hleaves_raw = leaves_of_uses(X.uses)
    b1, X.ret_mid, b2 = lines_between(X.body, "//#region 🔖️RetainedValue", "//#endregion 🔖️RetainedValue")
    body = b1 + "\n" + b2
    b1, X.tags_mid, b2 = lines_between(body, "//#region 🔖️Tags", "//#endregion 🔖️Tags")
    body = b1 + "\n" + b2
    b1, X.hash_mid, b2 = lines_between(body, "//#region 🔖️SchemaHash", "//#endregion 🔖️SchemaHash")
    body = b1 + "\n" + b2
    body, X.w_removed = cut_items(body, set(W_FNS) | set(ELEM) | {"NumKind"}, kinds=("fn", "const", "enum"))
    body, X.mounts_removed = cut_items(body, set(MOUNT_MODS), kinds=("mod",))
    body, X.pubuse_removed = cut_uses(body, ["original_record_source::", "original_record_receiving::", "controlled_encoding::", "borrowed_preflight::"])
    body, X.deleg_removed = cut_items(body, set(DELEGATORS), kinds=("fn",))
    X.rec_body = body
    X.hash_mid, X.hash_mount = cut_items(X.hash_mid, {"controlled_schema"}, kinds=("mod",))
    return X

def record_owner_map(X):
    om = {}
    for n in all_defined(X.tags_mid) | set(W_FNS) | set(ELEM) | {"NumKind"} | all_defined(X.hash_mid) | {"controlled_schema"}:
        om[n] = "schema"
    for n in all_defined(X.ret_mid) | set(RR_EXPORTS):
        om[n] = "rretained"
    for n in set(DELEGATORS) | set(E_EXPORTS):
        om[n] = "encode"
    om["measure_document_borrowed"] = "preflight"
    for n in all_defined(X.rec_body) | pub_names(X.rec_body) | {"EncodeOptions", "DecodeOptions", "scalar_witness", "intrinsic"}:
        om.setdefault(n, "record")
    return om

def trace_mount(key):
    rel = os.path.relpath(TRACE_TEST, os.path.dirname(libroot(key)))
    return ('#[cfg(test)]\n#[path = "' + rel + '"]\npub(crate) mod test_allocation;\n'
            '#[cfg(test)]\n#[global_allocator]\nstatic REQUESTED_ALLOCATOR: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;\n')

ALIAS_LEAVES = [(["protocol", "codec", "PackEncodeOptions"], "EncodeOptions", False), (["protocol", "codec", "PackDecodeOptions"], "DecodeOptions", False)]

def candidates(here, X, om, R, alias=True):
    leaves = map_leaves(X.hleaves_raw, R.rule_for(here))
    if alias: leaves += ALIAS_LEAVES
    for n, k in sorted(om.items()):
        if k != here and n not in ("tests", "intrinsic", "scalar_witness", "EncodeOptions", "DecodeOptions"):
            leaves.append(([ident(k), n], None, False))
    return leaves

def filtered(cands, scan):
    mine = {ident(k) for k, *_ in CRATES}
    defined = all_defined(scan)
    return [l for l in cands if not (l[0][0] in mine and usetree.local_name(l) in defined)]

def scan_texts(rels, R, here):
    out = ""
    for rel in rels:
        out += R.rewrite(read_stage_or_base(rel), here)
    return out

def pubify_first(text, kw_re=r"(?m)^(fn|const|enum)\s"):
    return re.sub(kw_re, lambda m: "pub " + m.group(1) + " ", text, count=1)

def build_schema(X, om, R):
    here = "schema"
    tags = re.sub(r"(?m)^const (TAG_)", r"pub const \1", X.tags_mid)
    shapes = "//#region 🔖️Shapes\n" + "\n\n".join(pubify_first(re.sub(r"\A(\s*//#[^\n]*\n)+\s*", "", t)) for t in X.w_removed) + "\n//#endregion 🔖️Shapes"
    hash_ = R.rewrite(X.hash_mid, here)
    body = tags + "\n\n" + shapes + "\n\n" + hash_
    cs_rel = VAL + "🏭️schema/🦀️.rs"
    cs = read_base(cs_rel)
    for n in ("compare_text", "sort"):
        a = "pub(super) fn " + n
        assert cs.count(a) == 1
        cs = cs.replace(a, "pub fn " + n)
    write(cs_rel, cs)
    scan = body + cs + read_base(VAL + "🏭️schema/🗂️index/🦀️.rs")
    uses = build_uses(scan, filtered(candidates(here, X, om, R, alias=False), body))
    doc = "//! 🏷️ Record wire vocabulary: the field and element tags, shape navigation and the canonical schema graph hash every encoder, decoder and cursor of a Pack record shares."
    text = doc + "\n\n" + "\n".join(uses) + '\n\n#[path = "../🏭️schema/🦀️.rs"]\npub mod controlled_schema;\n\n' + body.strip("\n") + "\n"
    write(VAL + "🧬️schema/🦀️.rs", text)

def build_rretained(X, om, R):
    here = "rretained"
    ret = R.rewrite(X.ret_mid, here)
    mounts = ('#[path = "📖️source/🦀️.rs"]\nmod original_record_source;\npub use original_record_source::{RetainedRecordBodySourceDecode,RetainedRecordBodyReadLimits,RetainedRecordBodySourceStep};\n\n'
              '#[path = "🫴️receiving/🦀️.rs"]\nmod original_record_receiving;\npub use original_record_receiving::RetainedRecordReceiving;\n\n'
              '#[path = "../🎟️storage/🦀️.rs"]\nmod original_storage;\n')
    tests = ('#[cfg(test)]\n#[path = "../🎟️storage/🧪️tests/🦀️.rs"]\nmod original_storage_tests;\n\n'
             '#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n')
    # unit tests split
    ut = read_base(VAL + "🧪️tests/🔬️unit/🦀️.rs")
    its = items(ut); names = [i["name"] for i in its]
    a = names.index("retained_value_cursor"); b = names.index("retained_record_body_fault_is_inline_sticky_and_closes_exactly")
    rr_tests = ut[its[a]["start"]:its[b]["end"]]
    rec_tests = ut[:its[a]["start"]] + ut[its[b]["end"]:]
    rec_tests = re.sub(r"\n{3,}", "\n\n", rec_tests)
    X.rec_unit_tests = rec_tests
    X.rr_unit_tests = rr_tests
    children = ["📖️source/🦀️.rs", "🫴️receiving/🦀️.rs"]
    scan = ret
    for c in children: scan += R.rewrite(read_base(VAL + "🛬️decode/" + c), here)
    scan += R.rewrite(read_base(VAL + "🎟️storage/🦀️.rs"), here)
    tscan = R.rewrite(read_base(VAL + "🎟️storage/🧪️tests/🦀️.rs"), here) + R.rewrite(rr_tests, here)
    ul, ut = build_uses_split(scan, tscan, filtered(candidates(here, X, om, R), ret))
    uses = ul + ut
    doc = "//! 🪝️ Retained record decoding: the value and record-body cursors that admit a wire body one granted step at a time, plus source decode, receiving and the stack storage they own."
    text = doc + "\n\n" + "\n".join(uses) + "\n\n" + mounts + "\n" + ret.strip("\n") + "\n\n" + tests
    write(VAL + "🛬️decode/🦀️.rs", text)
    for rel in ("📖️source/🦀️.rs", "🫴️receiving/🦀️.rs"):
        write(VAL + "🛬️decode/" + rel, R.rewrite(read_base(VAL + "🛬️decode/" + rel), here))
    write(VAL + "🎟️storage/🦀️.rs", R.rewrite(read_base(VAL + "🎟️storage/🦀️.rs"), here))
    # storage tests: `use super::*` children of the retained root
    st_rel = VAL + "🎟️storage/🧪️tests/🦀️.rs"
    st = R.rewrite(read_base(st_rel), here)
    avail = {}
    for l in candidates(here, X, om, R):
        n = usetree.local_name(l)
        if n: avail[n] = l
    st, need = augment_uses(st, avail, provided_names(text))
    print("storage tests add", need)
    write(st_rel, st)
    rt, need = augment_uses("use super::*;\n" + R.rewrite(rr_tests, here), avail, provided_names(text))
    print("rr unit tests add", need)
    write(VAL + "🛬️decode/🧪️tests/🔬️unit/🦀️.rs", "//! 🧷️ Retained value and record-body cursor laws that read cursor internals.\n" + rt)

def tree_files(root_rel):
    return crawl(root_rel)

def build_encode(X, om, R):
    here = "encode"
    rel = VAL + "🛫️encode/🦀️.rs"
    t = read_base(rel)
    doc, uses, body = split_doc_uses_body(t)
    deleg = "\n\n".join(X.deleg_removed)
    deleg = deleg.replace("controlled_encoding::", "")
    body = body.replace("pub(super) use borrowed::projected_record_body_into;", "use borrowed::projected_record_body_into;")
    body = body.replace("pub(super) fn intrinsic_document", "pub fn intrinsic_document")
    body = re.sub(r"pub\(super\) fn ", "fn ", body)
    assert "pub(super)" not in body
    body = R.rewrite(body, here)
    deleg = R.rewrite(deleg, here)
    files = [f for f in tree_files(rel)]
    scan = body + deleg; tscan = ""
    for f in files[1:]:
        if "🧪️tests" in f: tscan += R.rewrite(read_stage_or_base(f), here)
        else: scan += R.rewrite(read_stage_or_base(f), here)
    own = [l for l in leaves_of_uses(uses) if not (l[2] and l[0] == ["super"])]
    own = map_leaves(own, R.rule_for(here))
    cands = own + candidates(here, X, om, R)
    ul, ut = build_uses_split(scan, tscan, filtered(cands, body + deleg))
    lines = ul + ut
    text = doc + "\n" + "\n".join(lines) + "\n\n" + deleg.strip("\n") + "\n\n" + body.lstrip("\n")
    write(rel, text)

def build_preflight(X, om, R):
    here = "preflight"
    rel = VAL + "🫳️preflight/🦀️.rs"
    t = read_base(rel)
    doc, uses, body = split_doc_uses_body(t)
    body = body.replace("pub(super) fn borrowed_schema_hash", "pub fn borrowed_schema_hash")
    assert "pub(super)" not in body
    body = R.rewrite(body, here)
    scan = body; tscan = ""
    for f in tree_files(rel)[1:]:
        if "🧪️tests" in f: tscan += R.rewrite(read_stage_or_base(f), here)
        else: scan += R.rewrite(read_stage_or_base(f), here)
    own = [l for l in leaves_of_uses(uses) if not (l[2] and l[0] == ["super"])]
    own = map_leaves(own, R.rule_for(here))
    ul, ut = build_uses_split(scan, tscan, filtered(own + candidates(here, X, om, R), body))
    lines = ul + ut
    write(rel, doc + "\n" + "\n".join(lines) + "\n" + body)
    sc = VAL + "🫳️preflight/🏭️schema/🦀️.rs"
    s = read_base(sc)
    s = s.replace("pub(super) fn hash", "fn hash") if False else s
    write(sc, R.rewrite(s, here))

def build_record(X, om, R):
    here = "record"
    body = R.rewrite(X.rec_body, here)
    body = body.replace("//#region 🧪️Tests", "//#region 🧪️Tests")
    files = ["🔎️scalar-witness/🦀️.rs", "🌳️intrinsic/🦀️.rs"]
    intr = R.rewrite(read_base(VAL + "🌳️intrinsic/🦀️.rs"), here)
    intr = intr.replace("super::encode_value_record_body_controlled", ident("encode") + "::encode_value_record_body_controlled")
    intr = intr.replace("super::controlled_encoding::intrinsic_document", ident("encode") + "::intrinsic_document")
    assert "controlled_encoding" not in intr
    write(VAL + "🌳️intrinsic/🦀️.rs", intr)
    scan = body + intr + R.rewrite(read_base(VAL + "🔎️scalar-witness/🦀️.rs"), here)
    ul, ut = build_uses_split(scan, R.rewrite(X.rec_unit_tests, here), filtered(candidates(here, X, om, R, alias=False), body))
    lines = ul + ut
    doc = X.doc
    text = doc + "\n\n" + "\n".join(lines) + "\n\n" + body.lstrip("\n")
    write(VAL + "🦀️.rs", text)
    # unit tests (codec part)
    rt = R.rewrite(X.rec_unit_tests, here)
    a = '"/../../🌱️value/🧪️tests/🧫️fixtures/🔃️ordering/🔣️.json"'
    assert rt.count(a) == 1
    rt = rt.replace(a, '"/../../🧪️tests/🧫️fixtures/🔃️ordering/🔣️.json"')
    avail = {}
    for l in candidates(here, X, om, R, alias=False):
        n = usetree.local_name(l)
        if n: avail[n] = l
    rt, need = augment_uses(rt, avail, provided_names(text))
    print("record unit tests add", need)
    write(VAL + "🧪️tests/🔬️unit/🦀️.rs", rt)
    sw = VAL + "🔎️scalar-witness/🦀️.rs"
    write(sw, R.rewrite(read_base(sw), here))
