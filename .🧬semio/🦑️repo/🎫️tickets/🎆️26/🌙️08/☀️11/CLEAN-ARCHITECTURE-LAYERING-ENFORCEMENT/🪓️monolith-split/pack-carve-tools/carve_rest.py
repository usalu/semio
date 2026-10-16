"""✂️ Remaining module crates, the facade glue and the facade-hosted record laws."""
import re, os
from lib import *
from owners import *
from rules import Rules
from crates import *
import usetree, carve_record as CR
from carve_glue import GLUE, fmt_use, format_module

def uselist(key, names, indent="    "):
    return indent + "pub use " + ident(key) + "::{" + ", ".join(sorted(names)) + "};"

def module_block(name, key, names, cfg=None):
    out = ((cfg + "\n") if cfg else "") + "pub mod " + name + " {\n" + uselist(key, names) + "\n}\n"
    return out

def record_module(X, om):
    pub = pub_names(X.base)
    by = {}
    for n in pub:
        k = om[n]
        by.setdefault(k, []).append(n)
    out = ["pub mod record {"]
    for k in ("record", "schema", "rretained", "encode", "preflight"):
        if k in by: out.append(uselist(k, by[k]))
    out.append("}")
    return "\n".join(out) + "\n", by

def glue(X, om):
    g = read_base(GLUE)
    def rep(old, new):
        nonlocal g
        assert g.count(old) == 1, old
        g = g.replace(old, new)
    rep('#[path = "../../📐️format/🦀️.rs"]\npub mod format;\n', format_module())
    async_pub = pub_names(read_base(P + "⏳️async/🦀️.rs"))
    http_pub = pub_names(read_base(P + "🌐️http/🦀️.rs"))
    rep('#[path = "../../⏳️async/🦀️.rs"]\npub mod async_;\n', module_block("async_", "async", async_pub))
    rep('#[path = "../../🌐️http/🦀️.rs"]\npub mod http;\n', module_block("http", "http", http_pub))
    rep('#[cfg(not(target_arch = "wasm32"))]\n#[path = "../../🔌️io/🦀️.rs"]\npub mod io;\n', module_block("io", "io", {"recover_file", "write_atomic", "FilePackSink", "FilePackSource", "StreamingPackWriter"}, cfg='#[cfg(not(target_arch = "wasm32"))]'))
    rm, by = record_module(X, om)
    rep('#[path = "../../🌱️value/🦀️.rs"]\npub mod record;\n', rm)
    intr_pub = pub_names(read_base(P + "🌱️intrinsic/🦀️.rs"))
    rep('#[path = "../../🌱️intrinsic/🦀️.rs"]\npub mod intrinsic;\n', module_block("intrinsic", "intrinsic", intr_pub))
    write(GLUE, g)
    return by

LAW_TESTS = [("📦️operation-pages", "operation_page_tests", "🌱️value/🧪️tests/📦️operation-pages/🦀️.rs"),
             ("🎞️intrinsic-media", "intrinsic_media_tests", "🌱️value/🧪️tests/🎞️intrinsic-media/🦀️.rs"),
             ("🫳️preflight", "borrowed_preflight_tests", "🌱️value/🧪️tests/🫳️preflight/🦀️.rs"),
             ("intrinsic-boundary", "intrinsic_boundary_laws", "🌱️value/🌳️intrinsic/🧪️tests/🦀️.rs")]
SCHEMA_LAWS = [("schema-storage", "schema_storage_tests", "🌱️value/🧪️tests/🔬️schema-hash/💰️storage/🦀️.rs")]
WRAPPERS = [("🔬️record-laws", LAW_TESTS, "Record laws that drive the codec stack through the public `pack::record` paths: operation pages, intrinsic media, borrowed preflight and intrinsic boundaries."),
            ("🧮️record-schema-laws", SCHEMA_LAWS, "Record schema laws that drive the canonical schema hash and its scratch accounting through the public `pack::record` paths.")]

def fix_laws():
    rel = P + "🌱️value/🧪️tests/🫳️preflight/🦀️.rs"
    t = read_base(rel)
    a = "super::borrowed_preflight::borrowed_schema_hash"
    assert t.count(a) == 2
    write(rel, t.replace(a, ident("preflight") + "::borrowed_schema_hash"))

def record_laws(X, om, R):
    here = "facade"
    fix_laws()
    for folder, tests, doc in WRAPPERS:
        glob = "use pack::record::*;\n" if folder == "🔬️record-laws" else ""
        cands = map_leaves(X.hleaves_raw, R.rule_for(here))
        scan = "".join(read_base(P + rel) for _, _, rel in tests)
        I = ids(scan)
        leaves = [l for l in cands if usetree.local_name(l) in I]
        lines = usetree.emit(leaves)
        wrapper = P + "🧪️tests/" + folder + "/🦀️.rs"
        mounts = ""
        for _, mod, rel in tests:
            mounts += '#[path = "../../' + rel + '"]\nmod ' + mod + ';\n\n'
        rel_trace = os.path.relpath(CR.TRACE_TEST, os.path.dirname(wrapper))
        trace = CR.trace_mount("record").replace("#[cfg(test)]\n", "").replace(os.path.relpath(CR.TRACE_TEST, os.path.dirname(libroot("record"))), rel_trace)
        text = ("//! 🔬️ " + doc + "\n"
                "use pack::record;\n" + glob + "\n".join(lines) + "\n\n" + trace + "\n" + mounts.rstrip("\n") + "\n")
        write(wrapper, text)

R7 = ("// 🔓️ R7: `async fn` in a public trait is our deliberate crate-wide shape (R3 answers the lint's underlying `Send` concern structurally).\n"
      "// Never take rustc's suggested `-> impl Future<...> + Send` fix; it re-imposes `Send` R3 forbids.\n"
      "#![allow(async_fn_in_trait)]\n")

def allow_async_traits():
    for key in ("async", "http"):
        root = libroot(key)
        text = read_stage_or_base(root)
        if "#![allow(async_fn_in_trait)]" in text: continue
        L = text.split("\n")
        i = 0
        while i < len(L) and L[i].startswith("//!"): i += 1
        text = "\n".join(L[:i]) + "\n\n" + R7 + "\n".join(L[i:]).lstrip("\n").join(["\n", ""]) if False else "\n".join(L[:i]) + "\n\n" + R7 + "\n" + "\n".join(L[i:]).lstrip("\n")
        write(root, text)

def ensure_test_allocation():
    for key, *_ in CRATES:
        root = libroot(key)
        if not exists_stage_or_base(root): continue
        files = crawl(root)
        if not any("test_allocation" in read_stage_or_base(f) for f in files[1:]) : continue
        text = read_stage_or_base(root)
        if "mod test_allocation" in text: continue
        doc, uses, body = split_doc_uses_body(text)
        text = doc + "\n\n" + "\n".join(u["text"] for u in uses) + "\n\n" + CR.trace_mount(key) + "\n" + body
        write(root, text)
        print("test_allocation mounted in", key)

def run(R, X, om):
    for key in ("intrinsic", "async", "http", "io"):
        done = finalize_crate(libroot(key), lambda t, k=key: R.rewrite(t, k))
        print("finalize", key, [d.split("/")[-3:] for d in done])
    allow_async_traits()
    ensure_test_allocation()
    by = glue(X, om)
    record_laws(X, om, R)
    return by
