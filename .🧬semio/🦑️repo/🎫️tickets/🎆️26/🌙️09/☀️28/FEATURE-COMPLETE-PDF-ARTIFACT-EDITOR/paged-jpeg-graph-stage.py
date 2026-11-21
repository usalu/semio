"""🪪️ Stages PDF-native graph reloan and revision-checked placement assembly."""
import runpy
from pathlib import Path

TICKET = Path(__file__).resolve().parent
writer_stage = runpy.run_path(str(TICKET / "paged-jpeg-writer-stage.py"))
writer_stage["stage"]()
BASE = writer_stage["BASE"]
OUT = TICKET / "🗑️generated" / "paged-jpeg-graph"
WRITER = writer_stage["OUT"]
one = writer_stage["one"]
span = writer_stage["span"]


def synthesize():
    foreign = (WRITER / "foreign-artifacts.rs").read_text()
    foreign = one(foreign, "pub mod dct_view;", """pub mod dct_view;

#[path = "🔎️dct-lookup.rs"]
pub mod dct_lookup;""")
    foreign = one(foreign, "    /// 📍️ Gives the number of original bytes already hashed.", """    /// 🔑️ Reads the exact authored DCT identity without allocating reference strings.
    pub fn parse_reference(reference: &ArtifactRef) -> Option<[u8; 32]> {
        if reference.dialect.artifact_kind != RAW_KIND || reference.dialect.standard != RAW_STANDARD
            || reference.dialect.subset != RAW_SUBSET
        {
            return None;
        }
        let hex = reference.artifact_id.strip_prefix("pdf:s.stdio.jpeg:sha256:")?.as_bytes();
        let hex: &[u8; 64] = hex.try_into().ok()?;
        let nibble = |byte| match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            _ => None,
        };
        let mut digest = [0; 32];
        for (value, pair) in digest.iter_mut().zip(hex.chunks_exact(2)) {
            *value = nibble(pair[0])? << 4 | nibble(pair[1])?;
        }
        Some(digest)
    }

    /// 📍️ Gives the number of original bytes already hashed.""")
    controlled = (BASE / "🔨️modules/🖼️images/🎞️controlled/🦀️.rs").read_text()
    qualifier = span(controlled, "fn qualified_dictionary", "//#region 🔖️Image resources")
    controlled = one(controlled, qualifier, "")
    controlled = one(controlled, "foreign_artifacts::PdfDctResourceIdentity",
                     "foreign_artifacts::{qualified_image_dictionary, PdfDctResourceIdentity}")
    controlled = one(controlled, "qualified_dictionary(dict)", "qualified_image_dictionary(dict)")
    qualifier = qualifier.replace("fn qualified_dictionary", """pub(crate) fn qualified_image_dictionary""").replace("i64::from(DEPTH)", "8")
    qualifier = one(qualifier, "    dict.len()", """    use crate::standards::v1_7::subsets::base::modules::lexer::{dict_get, dict_name};
    dict.len()""")
    foreign = one(foreign, "/// 🛂️ Native admission and resolution", qualifier + "/// 🛂️ Native admission and resolution")

    authoring = (writer_stage["JPEG_STAGE"] / "pdf-authoring-entry.rs").read_text()
    authoring = one(authoring, "pub mod baseline_resource;", """pub mod baseline_resource;

#[path = "📌️dct-placement.rs"]
pub mod dct_placement;""")
    names = span(authoring, "    let taken: std::collections::HashSet", '    let id = names.next()')
    authoring = one(authoring, names, "    let mut names = image_names(snapshot);\n")
    authoring = one(authoring, """    let paint = PdfOp::PaintXObject { name: id };
    match placement {
        ImagePlacement::Insert { matrix, .. } => changes.push(PdfMutation::AppendPageContent(AppendPageContent {
            index: page,
            content: vec![PdfOp::Save, PdfOp::Transform { matrix }, paint, PdfOp::Restore],
        })),
        ImagePlacement::Replace { operator, .. } => changes.push(PdfMutation::ReplaceContent(ReplaceContent { index: page, at: operator, op: paint })),
    }
    Ok(changes)
}""", """    changes.push(paint_image(page, placement, id));
    Ok(changes)
}

fn image_names(snapshot: &PdfSnapshot) -> impl Iterator<Item = String> + '_ {
""" + names.replace("    let mut names = ", "    ").replace("filter(|name|", "filter(move |name|").rstrip().removesuffix(";") + """
}

fn paint_image(page: usize, placement: ImagePlacement, id: String) -> PdfMutation {
    let paint = PdfOp::PaintXObject { name: id };
    match placement {
        ImagePlacement::Insert { matrix, .. } => PdfMutation::AppendPageContent(AppendPageContent {
            index: page,
            content: vec![PdfOp::Save, PdfOp::Transform { matrix }, paint, PdfOp::Restore],
        }),
        ImagePlacement::Replace { operator, .. } => PdfMutation::ReplaceContent(ReplaceContent { index: page, at: operator, op: paint }),
    }
}""")

    OUT.mkdir(parents=True, exist_ok=True)
    for path in WRITER.iterdir():
        if path.suffix == ".rs":
            (OUT / path.name).write_text(path.read_text())
    files = {
        "foreign-artifacts.rs": foreign,
        "image-controlled.rs": controlled,
        "authoring.rs": authoring,
        "🔎️dct-lookup.rs": (TICKET / "paged-pdf-dct-lookup.rs").read_text(),
        "📌️dct-placement.rs": (TICKET / "paged-pdf-dct-placement.rs").read_text(),
        "identity.rs": span(foreign, "const DCT_KIND:", "/// 🛂️ Native admission"),
        "graph-probe.rs": (TICKET / "paged-jpeg-graph-probe.rs").read_text(),
    }
    for name, source in files.items():
        (OUT / name).write_text(source)
    core = span(authoring, "/// 📍", '#[path = "🎛️replace-button.rs"]')
    (OUT / "authoring-core.rs").write_text('use super::*;\n#[path = "📌️dct-placement.rs"]\npub mod dct_placement;\n' + core)
    model = (BASE / "🧬️schema/📸️snapshot/📐️model/🔗️object-model/🦀️.rs").read_text()
    lexer = (BASE / "🔨️modules/🔤️lexer/🦀️.rs").read_text()
    helpers = span(lexer, "pub fn dict_get", "pub fn dict_i64") + span(lexer, "pub fn dict_name", "//#endregion 🔖️DictHelpers")
    (OUT / "dictionary-helpers.rs").write_text(helpers)
    object_helpers = span(model, "    pub fn as_dict", "    pub fn as_ref") + span(model, "    pub fn as_bool", "    pub fn as_f64")
    harness = (OUT / "probe.rs").read_text()
    harness = one(harness, "fn main() -> io::Result<()> {", "fn previous_writer_suite() -> io::Result<()> {")
    harness = one(harness, "    pub fn name(name: impl Into<String>)", object_helpers + "    pub fn name(name: impl Into<String>)")
    harness = harness.replace("#[derive(Debug, PartialEq, Eq)]", "#[derive(Clone, Debug, PartialEq, Eq)]")
    harness = one(harness, "pub mod schema {\n                    pub mod snapshot", "pub mod schema {\n                    pub use crate::stream_roles;\n                    pub mod snapshot")
    harness = one(harness, "    include!(\"identity.rs\");", "    use super::{PdfObject, PdfDictEntry};\n    include!(\"identity.rs\");")
    harness = one(harness, "    include!(\"lexer.rs\");", "    include!(\"lexer.rs\");\n    include!(\"dictionary-helpers.rs\");")
    (OUT / "graph-probe.rs").write_text(harness + (TICKET / "paged-jpeg-graph-probe.rs").read_text())


if __name__ == "__main__":
    synthesize()
