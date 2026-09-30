"""🏅️ W2-W-office: moves the xlsx strict/transitional conformance vocabularies onto the snapshot's authoritative XML parts.

Since the XLSX snapshot carries every XML part as one logical `XmlDocument` in `xml_parts` (`opc.parts` holds only the
opaque binary parts), every class edit that read or rewrote `opc.parts` XML bytes was a silent no-op. Each diff builder now
edits a copy of the snapshot — the logical XML parts, the content-type table, the opaque VML part — and diffs it against
the base with the base subset's own `diff_set_snapshot`; the hand-rolled sparse OPC diff helpers go away with it.

  python3 🧪️w2-w-office-xlsx-conformance-authority.py
"""
import pathlib

ROOT = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets")
ASYNC = "// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9\n"

HELPERS = '''//#region 🔖️Helpers
/// 🧭️ The main part's path, resolved through the root `officeDocument` relationship by type SUFFIX
/// so it resolves under either conformance class — matching by the transitional-shaped constant
/// verbatim would silently fail to find the main part of a genuinely Strict package.
{a}fn main_part_path(base: &XlsxSnapshot) -> Option<String> {{
    let relationship = base.opc.relationships_for("").iter().find(|relationship| relationship.rel_type.ends_with("/officeDocument"))?;
    Some(resolve_relationship_target("", &relationship.target))
}}
{part_text}
/// ✍️ Rewrites every attribute value equal to a member of `from` to `to`, through the whole
/// subtree — a namespace declaration is an ordinary attribute, which is why one walk covers
/// `xmlns`, `xmlns:r` and whatever prefixed alias a real package happens to use.
{a}fn retarget_namespace(node: &mut XmlNode, from: &[&str], to: &str) -> bool {{
    let XmlNode::Element {{ attrs, children, .. }} = node else {{ return false }};
    let mut changed = false;
    for attr in attrs.iter_mut() {{
        if from.contains(&attr.value.as_str()) && attr.value != to {{
            attr.value = to.to_string();
            changed = true;
        }}
    }}
    for child in children.iter_mut() {{
        changed |= retarget_namespace(child, from, to);
    }}
    changed
}}

{a}fn declares_namespace(node: &XmlNode, value: &str) -> bool {{
    let XmlNode::Element {{ attrs, children, .. }} = node else {{ return false }};
    attrs.iter().any(|attr| attr.value == value) || children.iter().any(|child| declares_namespace(child, value))
}}

/// 🔎️ Which member of a `[transitional, strict]` pair the package's logical XML parts actually declare.
{a}fn declared_pair_member(base: &XlsxSnapshot, pair: [&str; 2]) -> Option<String> {{
    pair.into_iter().find(|candidate| base.xml_parts.iter().any(|part| part.document.root.as_ref().is_some_and(|root| declares_namespace(root, candidate)))).map(str::to_string)
}}

{a}fn root_attribute(document: &XmlDocument, name: &str) -> Option<String> {{
    let XmlNode::Element {{ attrs, .. }} = document.root.as_ref()? else {{ return None }};
    attrs.iter().find(|attr| attr.name == name).map(|attr| attr.value.clone())
}}

/// 🔎️ The main part's root `conformance` attribute, if it declares one.
{a}pub fn conformance_attribute(base: &XlsxSnapshot) -> Option<String> {{
    root_attribute(&base.xml_part(&main_part_path(base)?)?.document, "conformance")
}}

/// ✍️ Sets — or, with `None`, removes — one attribute on the ROOT element only.
{a}fn set_root_attribute(document: &mut XmlDocument, name: &str, value: Option<&str>) -> bool {{
    let Some(XmlNode::Element {{ attrs, .. }}) = document.root.as_mut() else {{ return false }};
    match (attrs.iter().position(|attr| attr.name == name), value) {{
        (Some(index), Some(value)) => attrs[index].value = value.to_string(),
        (Some(index), None) => {{
            attrs.remove(index);
        }}
        (None, Some(value)) => attrs.push(XmlAttr {{ name: name.to_string(), value: value.to_string() }}),
        (None, None) => return false,
    }}
    true
}}

/// 🏅️ Stamps a whole snapshot into one conformance class: both namespace families across every
/// logical XML part, the `officeDocument` relationship base, and the main part's own `conformance`
/// attribute. Bijective by construction, so stamping back is an exact inverse — which is what makes
/// `SetSnapshot` invertible on this axis.
{a}pub fn stamp_conformance_class(mut snapshot: XlsxSnapshot, strict: bool) -> XlsxSnapshot {{
    let index = usize::from(strict);
    for part in snapshot.xml_parts.iter_mut() {{
        let Some(root) = part.document.root.as_mut() else {{ continue }};
        retarget_namespace(root, &MAIN_NAMESPACES, MAIN_NAMESPACES[index]);
        retarget_namespace(root, &RELATIONSHIP_NAMESPACES, RELATIONSHIP_NAMESPACES[index]);
    }}
    for relationships in snapshot.opc.relationships.values_mut() {{
        for relationship in relationships.iter_mut() {{
            let Some(prefix) = RELATIONSHIP_NAMESPACES.into_iter().find(|prefix| relationship.rel_type.starts_with(prefix)) else {{ continue }};
            relationship.rel_type = format!("{{}}{{}}", RELATIONSHIP_NAMESPACES[index], &relationship.rel_type[prefix.len()..]);
        }}
    }}
    if let Some(path) = main_part_path(&snapshot) {{
        if let Some(part) = snapshot.xml_part_mut(&path) {{
            set_root_attribute(&mut part.document, "conformance", if strict {{ Some("strict") }} else {{ None }});
        }}
    }}
    snapshot
}}

/// 🏅️ The one whole-package operation that moves `base` into (`strict`) or out of the strict conformance class: a
/// `set-snapshot` of [`stamp_conformance_class`]'s stamp — what a class conversion records as one edit.
pub fn stamp_conformance_class_mutation(base: &XlsxSnapshot, strict: bool) -> {agg} {{
    {agg}::SetSnapshot(set_snapshot::SetSnapshot {{ snapshot: stamp_conformance_class(base.clone(), strict) }})
}}
//#endregion 🔖️Helpers

//#region 🔖️DiffBuilders
/// 🔺️ The sparse diff carrying `base` to its edited copy `next`, through the base subset's own
/// `diff_set_snapshot` — none when the edit changed nothing.
{a}fn diff_to(base: &XlsxSnapshot, next: XlsxSnapshot) -> XlsxDiff {{
    if next == *base {{
        return XlsxDiff::default();
    }}
    diff_set_snapshot(base, &next)
}}

/// 🔺️ The diff of retargeting one namespace family across every logical XML part that declares it.
{a}fn diff_retarget_namespace(base: &XlsxSnapshot, from: [&str; 2], to: &str) -> XlsxDiff {{
    let mut next = base.clone();
    for part in next.xml_parts.iter_mut() {{
        if let Some(root) = part.document.root.as_mut() {{
            retarget_namespace(root, &from, to);
        }}
    }}
    diff_to(base, next)
}}

/// 🔺️ The diff of setting — or removing — the main part's root `conformance` attribute.
{a}fn diff_conformance_attribute(base: &XlsxSnapshot, value: Option<&str>) -> XlsxDiff {{
    let Some(path) = main_part_path(base) else {{ return XlsxDiff::default() }};
    let mut next = base.clone();
    let Some(part) = next.xml_part_mut(&path) else {{ return XlsxDiff::default() }};
    set_root_attribute(&mut part.document, "conformance", value);
    diff_to(base, next)
}}
{vml}
/// 🔺️ The diff of retyping one part: the logical XML part's own `content_type` and its
/// `[Content_Types].xml` override move together, because the two can never be allowed to drift apart.
{a}fn diff_set_content_type(base: &XlsxSnapshot, path: &str, content_type: &str) -> XlsxDiff {{
    let mut next = base.clone();
    let Some(part) = next.xml_part_mut(path) else {{ return XlsxDiff::default() }};
    part.content_type = content_type.to_string();
    next.opc.content_types.set_override(path, content_type);
    diff_to(base, next)
}}

{a}fn resolved_content_type(base: &XlsxSnapshot, path: &str) -> Option<String> {{
    base.opc.content_types.resolve(path).map(str::to_string)
}}
//#endregion 🔖️DiffBuilders'''

PART_TEXT = '''
/// 📖️ An opaque OPC part's text — the legacy VML drawing is a binary-authority part, never a logical XML part.
{a}fn part_text(base: &XlsxSnapshot, path: &str) -> Option<String> {{
    String::from_utf8(base.opc.part(path)?.bytes.clone()).ok()
}}
'''

VML = '''
/// 🔺️ The diff of adding a legacy VML drawing part — an opaque OPC part, since its content type is
/// not XML — together with its content-type override.
{a}fn diff_insert_vml_part(base: &XlsxSnapshot, path: &str, markup: &str) -> XlsxDiff {{
    if base.opc.part(path).is_some() {{
        return XlsxDiff::default();
    }}
    let mut next = base.clone();
    next.opc.set_part(path, VML_CONTENT_TYPE, markup.as_bytes().to_vec());
    diff_to(base, next)
}}

/// 🔺️ The diff of removing a legacy VML drawing part and its content-type override.
{a}fn diff_remove_vml_part(base: &XlsxSnapshot, path: &str) -> XlsxDiff {{
    let key = path.trim_start_matches('/');
    let mut next = base.clone();
    next.opc.parts.retain(|part| part.path != key);
    next.opc.content_types.overrides.retain(|(name, _)| name.trim_start_matches('/') != key);
    diff_to(base, next)
}}
'''

IMPORTS_OLD = [
    "use crate::standards::v_ecma_376::subsets::base::schema::diff::{NamedModified, NamedTripleDiff, XlsxDiff, XlsxOpcContentTypesDiff, XlsxOpcCtEntriesDiff, XlsxOpcDiff, XlsxOpcPartDiff, XlsxOpcPartsDiff, XlsxOpcRelationshipsDiff};\n",
    "use semio_s_artifact_stdio_xml::schema::snapshot::{xml_document_from_text, xml_document_to_text, XmlAttr, XmlDocument, XmlNode};\n",
    "use semio_s_artifact_stdio_zip::opc::{resolve_relationship_target, OpcPart};\n",
]
IMPORTS_NEW = [
    "use crate::standards::v_ecma_376::subsets::base::schema::diff::{diff_set_snapshot, XlsxDiff};\n",
    "use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlDocument, XmlNode};\n",
    "use semio_s_artifact_stdio_zip::opc::resolve_relationship_target;\n",
]

for subset, agg, vml in (("🔒️strict", "XlsxStrictMutation", True), ("🌉️transitional", "XlsxTransitionalMutation", False)):
    path = ROOT / subset / "🧬️schema/🧬️mutations/🦀️.rs"
    text = path.read_text()
    for old, new in zip(IMPORTS_OLD, IMPORTS_NEW):
        assert text.count(old) == 1, (subset, old)
        text = text.replace(old, new)
    start = text.index("//#region 🔖️Helpers")
    end = text.index("//#endregion 🔖️DiffBuilders") + len("//#endregion 🔖️DiffBuilders")
    body = HELPERS.format(a=ASYNC, agg=agg, part_text=PART_TEXT.format(a=ASYNC) if vml else "", vml=VML.format(a=ASYNC) if vml else "")
    path.write_text(text[:start] + body + text[end:])
    print("rewrote", subset)
