import os, re, sys
sys.path.insert(0, "/Users/ueli/.cache/hdr4/tools")
from h3reg import *
from rsitems import *
write = "--write" in sys.argv
B = R + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base"
TD = B + "/🚪️io/📝️text/🔺️diff/🦀️.rs"; BD = B + "/🚪️io/💾️binary/🔺️diff/🦀️.rs"
TS = B + "/🚪️io/📝️text/📸️snapshot/🦀️.rs"; BS = B + "/🚪️io/💾️binary/📸️snapshot/🦀️.rs"
TC = B + "/🚪️io/📝️text/📸️snapshot/🧱️document-parts/🦀️.rs"; BC = B + "/🚪️io/💾️binary/📸️snapshot/🧱️document-parts/🦀️.rs"
M = "crate::standards::v1_1::subsets::base"
SETS = {
 "text": (TD, TS, TC, "text", ["hex_encode","hex_decode","enc_str","dec_str","split_top_level","strip_brackets","encode_option","decode_option","enc_prolog","dec_prolog","enc_declaration","enc_quote","dec_quote","dec_declaration","enc_doctype","dec_doctype","dec_svg_node","enc_svg_node"]),
 "binary": (BD, BS, BC, "binary", ["write_bytes_lp","read_bytes_lp","write_str_lp","read_str_lp","enc_prolog_bin","dec_prolog_bin","enc_doctype_bin","dec_doctype_bin","dec_attr_bin","enc_declaration_bin","dec_declaration_bin","dec_svg_node_bin","enc_svg_node_bin"]),
}
edits = {}
creates = {}
XMLNAMES = ["XmlDoctype", "XmlDtdDeclaration", "XmlExternalId", "XmlQuote", "XmlDeclaration"]
for kind, (diff, snap, core, kd, names) in SETS.items():
    dt = rd(diff); st = rd(snap)
    fs = top_fns(dt)
    for n in names: assert n in fs, (kind, n)
    cut = sorted(((fs[n][0], fs[n][1], n) for n in names))
    body = "".join(dt[s:e].rstrip("\n") + "\n\n" for s, e, n in cut)
    rest = dt
    for s, e, n in sorted(cut, reverse=True):
        rest = rest[:s] + rest[e:]
    body = body.replace("pub(crate) fn ", "pub fn ")
    # core imports
    imp = []
    xml = [x for x in XMLNAMES if re.search(r"\b%s\b" % x, body)]
    if xml: imp.append("use semio_s_artifact_stdio_xml::schema::snapshot::{%s};" % ", ".join(xml))
    sv = [x for x in ("SvgAttr", "SvgNode") if re.search(r"\b%s\b" % x, body)]
    if sv: imp.append("use crate::schema::snapshot::{%s};" % ", ".join(sv))
    # cross-core references (text core uses binary? no)
    header = "//! 🧱️ The %s codecs of the document parts (nodes, doctype, declaration, prolog) that the snapshot and the diff codecs share.\n\n" % kd
    cross = ""
    core_text = header + "\n".join(imp) + "\n\n" + body.rstrip("\n") + "\n"
    creates[core] = core_text
    # diff file: import extracted names it still uses
    used = [n for n in names if re.search(r"\b%s\b" % n, rest)]
    core_path = "%s::io::%s::snapshot::document_parts" % (M, kd)
    imp_line = "use %s::{%s};\n" % (core_path, ", ".join(used))
    # insert after each `use super::*;` inside the codec modules
    count = rest.count("use super::*;\n")
    rest = rest.replace("use super::*;\n", "use super::*;\n" + imp_line)
    # drop the cross-import of the other family's primitive from the diff (now from core)
    other = "binary" if kind == "text" else "text"
    edits[diff] = rest
    print(kind, "extracted", len(cut), "bytes", sum(e - s for s, e, n in cut), "diff after", len(rest), "imports into", count, "modules; used:", used)
    # snapshot file: replace the diff imports by core imports
    st2 = re.sub(r"use %s::io::%s::diff::(\{[^}]*\}|\w+);" % (re.escape(M), kd), lambda m: "use %s::%s;" % (core_path, m.group(1)), st)
    # mount the core module
    mount = '#[path = "🧱️document-parts/🦀️.rs"]\npub mod document_parts;\n'
    if kind == "text":
        assert '#[path="🧮️attributes/🦀️.rs"]\npub mod attributes;' in st2
        st2 = st2.replace('#[path="🧮️attributes/🦀️.rs"]\npub mod attributes;', '#[path="🧮️attributes/🦀️.rs"]\npub mod attributes;\n' + mount, 1)
    else:
        st2 = st2.rstrip("\n") + "\n\n" + mount
    edits[snap] = st2
    print(kind, "snapshot diff-imports left:", len(re.findall(r"io::%s::diff::" % kd, st2)))
# cross imports between the two diff files: text diff imports enc_svg_node_bin from binary diff; binary diff imports enc_svg_node from text diff
for diff, kd, ok in ((TD, "text", "binary"), (BD, "binary", "text")):
    t = edits[diff]
    t = re.sub(r"use %s::io::%s::diff::(\{[^}]*\}|\w+);" % (re.escape(M), ok), lambda m: "use %s::io::%s::snapshot::document_parts::%s;" % (M, ok, m.group(1)), t)
    t = t.replace("%s::io::%s::diff::enc_svg_node" % (M, ok), "%s::io::%s::snapshot::document_parts::enc_svg_node" % (M, ok))
    edits[diff] = t
    print(kd, "diff leftovers of other diff:", re.findall(r"io::%s::diff::\w+" % ok, t))
for p, t in edits.items(): print("edit", p[len(B) + 1:], len(t))
for p, t in creates.items(): print("create", p[len(B) + 1:], len(t))
if write:
    for p, t in edits.items(): wr(p, t)
    for p, t in creates.items():
        os.makedirs(os.path.dirname(p), exist_ok=True); open(p, "w", encoding="utf8").write(t)
    print("written")
