import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rg-rest-tools")
from lib import *
DM = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx"
B = DM + "/🏅️standards/🔖️ecma-376/🪆️subsets"
BASE = B + "/🧱️base"
M = BASE + "/🧬️schema/🧬️mutations"
MPKG = M + "/📦️packages/🦀️rust"
OLD = "semio-s-artifact-stdio-docx-base-mutations"
agg = rd(M + "/🦀️.rs")
# ---- xml-address (down)
XA = M + "/🧭️xml-address/🦀️.rs"
xa = rd(XA)
assert "use super::*;\n" in xa
fn_start = xa.index("/// 🌱️ Prepares a first run inside an empty paragraph")
fn_end = xa.index("/// 🧭️ Resolves a projected block/run selection to its canonical XML address")
fn_text = xa[fn_start:fn_end]
new_fn = '''/// 🌱️ The insertion index and first run of an empty paragraph that receives its first text without replacing its existing markup.
pub fn empty_paragraph_run_insertion(snapshot: &DocxSnapshot, address: &DocxXmlAddress, text: &str) -> Result<Option<(usize, XmlNode)>, ValueError> {
    let resolved = resolve_docx_xml_address(snapshot, address)?;
    if !is_word_name(&resolved.node, "p", &resolved.bindings) || contains_word_run(&resolved.node, &resolved.parent_bindings) {
        return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DOCX text target must be an existing run or an empty WordprocessingML paragraph"));
    }
    if text.is_empty() {
        return Ok(None);
    }
    let XmlNode::Element { children, .. } = &resolved.node else { unreachable!() };
    let mut attrs = Vec::new();
    set_xml_space(&mut attrs, needs_preserved_space(text));
    let node = XmlNode::Element { name: word_name_like(&resolved.node, "r")?, attrs: Vec::new(), children: vec![XmlNode::Element { name: word_name_like(&resolved.node, "t")?, attrs, children: vec![XmlNode::Text { text: text.into() }] }] };
    Ok(Some((children.len(), node)))
}

'''
xa2 = xa[:fn_start] + new_fn + xa[fn_end:]
xa2 = xa2.replace("use super::*;\n", '''extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_stdio_docx_base_diff::standards::v_ecma_376::subsets::base::schema::diff::DocxBlockPath;
use semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::DocxSnapshot;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};
''', 1)
xa2 = xa2.replace("pub(super)", "pub")
wr(XA, xa2)
NEWXA = "semio-s-artifact-stdio-docx-base-xml-address"
register_crate(DM + "/Cargo.toml", OLD, MPKG, NEWXA, M + "/🧭️xml-address", "Stdio docx base canonical xml addresses", "../../🦀️.rs", script=DM + "/📦️packages/🦀️rust/📜️script.ts")
# ---- edit-rules (up)
ER = M + "/🧭️edit-rules/🦀️.rs"
er = rd(ER)
assert "use super::*;\n" in er
imports = '''extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::{DocxSnapshot, DocxXmlPart};
use semio_s_artifact_stdio_docx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::{docx_xml_address, insert_xml_node, opc_layer, remove_content_type, remove_part, remove_relationship, remove_xml_node, replace_xml_node, set_content_type, set_part, set_relationship, DocxMutation};
use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
use semio_framework_value::ValueError;
'''
er = er.replace("use super::*;\n", imports, 1)
wr(ER, er)
NEWER = "semio-s-artifact-stdio-docx-base-edit-rules"
npkg_er = register_crate(DM + "/Cargo.toml", OLD, MPKG, NEWER, M + "/🧭️edit-rules", "Stdio docx base details-pane edit rules", "../../🦀️.rs", script=DM + "/📦️packages/🦀️rust/📜️script.ts")
# mutations-side deps for edit rules
add_dep_line(npkg_er + "/Cargo.toml", "dependencies", OLD, '%s = { workspace = true }' % OLD)
# ---- aggregator
a = agg
m1 = '#[path = "🧭️edit-rules/🦀️.rs"]\npub mod edit_rules;\n'
assert m1 in a
a = a.replace(m1, "", 1)
m2 = '#[path = "🧭️xml-address/🦀️.rs"]\npub mod xml_address;\n'
assert m2 in a
a = a.replace(m2, "pub use semio_s_artifact_stdio_docx_base_xml_address as xml_address;\n", 1)
m3 = "#[path = \"🔗opc-layer/🦀️.rs\"]\npub(crate) mod opc_layer;\n"
assert m3 in a
a = a.replace(m3, "#[path = \"🔗opc-layer/🦀️.rs\"]\npub mod opc_layer;\n", 1)
anchor = "/// 🧩️ Validates one addressed edit and prepares its compact target-part diff and exact inverse.\n"
assert anchor in a
a = a.replace(anchor, '''/// 🌱️ Prepares a first run inside an empty paragraph without replacing its existing markup.
pub fn empty_paragraph_text_mutation(snapshot: &DocxSnapshot, address: &DocxXmlAddress, text: &str) -> Result<Option<DocxMutation>, ValueError> {
    Ok(xml_address::empty_paragraph_run_insertion(snapshot, address, text)?.map(|(index, node)| DocxMutation::InsertXmlNode(insert_xml_node::InsertXmlNode { parent: address.clone(), index, node })))
}

''' + anchor, 1)
wr(M + "/🦀️.rs", a)
op = M + "/🔗opc-layer/🦀️.rs"
wr(op, rd(op).replace("pub(crate) fn", "pub fn"))
# ---- callers
prep = BASE + "/✏️editor/📬️preparation/🦀️.rs"
p = rd(prep)
old_call = "schema::mutations::xml_address::empty_paragraph_text_mutation("
assert old_call in p
wr(prep, p.replace(old_call, "schema::mutations::empty_paragraph_text_mutation(", 1))
ut = M + "/🧪️tests/🔬️unit/🦀️.rs"
u = rd(ut)
assert "let prepare = xml_address::empty_paragraph_text_mutation;" in u
wr(ut, u.replace("let prepare = xml_address::empty_paragraph_text_mutation;", "let prepare = empty_paragraph_text_mutation;", 1))
# edit-rules importers
for ed in (B + "/📏️strict/✏️editor/🦀️.rs", B + "/🔄️transitional/✏️editor/🦀️.rs", BASE + "/✏️editor/🦀️.rs"):
    t = rd(ed)
    line = [l for l in t.split("\n") if l.startswith("use semio_s_artifact_stdio_docx_base_mutations::") and l.endswith("::mutations::edit_rules;")]
    assert len(line) == 1, ed
    t = t.replace(line[0], "use semio_s_artifact_stdio_docx_base_edit_rules as edit_rules;", 1)
    wr(ed, t)
    mf = os.path.dirname(ed) + "/📦️packages/🦀️rust/Cargo.toml"
    add_dep_like(mf, OLD, NEWER, npkg_er)
et = M + "/🧭️edit-rules/🧪️tests/🦀️.rs"
e = rd(et)
old_use = "use semio_s_artifact_stdio_docx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::edit_rules::*;\n"
assert old_use in e
wr(et, e.replace(old_use, "use semio_s_artifact_stdio_docx_base_edit_rules::*;\n", 1))
# mutations manifest: dep on xml-address
add_dep_line(MPKG + "/Cargo.toml", "dependencies", NEWXA, '%s = { workspace = true }' % NEWXA)
print(run_owners(DM, True))
