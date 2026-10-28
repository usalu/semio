"""🧩️ Move canonical native concept/body DTOs without duplicating endpoint ownership."""
from pathlib import Path

concepts = Path("C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts")
tess = concepts / "🔺️tessellation"
body = concepts / "🧱️body"
path = tess / "🦀️.rs"
source = path.read_text(encoding="utf-8")

def declaration(name):
    """📜️ Retain exact derives, wire attributes and fields of one current native DTO."""
    declaration_start = source.index("pub struct " + name + " {")
    start = source.rfind("#[derive(", 0, declaration_start)
    end = source.index("\n}", declaration_start) + 2
    return source[start:end]

shared_names = ["IfcConceptTarget", "IfcConceptSource", "IfcConceptLabel", "IfcConceptDiagnostic", "IfcConceptTargetReport"]
shared = "//! 🧩️ Shared native concept provenance, selected targets and diagnostic wire fields.\n\n" + "\n\n".join(declaration(name) for name in shared_names) + "\n"
request = declaration("IfcBodyTessellationRequest").replace("IfcBodyTessellationRequest", "IfcBodyConceptRequest")
report = declaration("IfcBodyTessellationReport").replace("IfcBodyTessellationReport", "IfcBodyConceptReport")
domain = """//! 🧾️ Canonical native selected Body inputs and outputs share exact representation wire fields.
use super::super::concept_types::*;
use super::super::source_index::IfcSourceIds;
use super::super::preparation::IfcMvdRequest;
use super::super::native::positive_id;
use semio_framework_value::ValueError;

""" + request + "\n\n" + report + """
impl IfcSourceIds for IfcBodyConceptRequest {
    fn source_len(&self) -> usize { self.targets.len() }
    fn source_id(&self, row: usize) -> Result<u64, ValueError> { positive_id(&self.targets[row].representation) }
}
impl IfcMvdRequest for IfcBodyConceptRequest {
    fn valid_row(&self, row: usize) -> bool { self.targets.get(row).is_some() }
}
"""
start = source.index(declaration("IfcConceptTarget"))
last = declaration("IfcBodyTessellationReport")
end = source.index(last) + len(last)
source = source[:start] + source[end:]
before = """#[path = "../../../../../../../../📥️source/🗂️index/🦀️.rs"]
mod source_index;
use source_index::{IdMatch, IfcSourceIds, IfcSourceIndex};"""
assert source.count(before) == 1
source = source.replace(before, "use super::source_index::{IdMatch, IfcSourceIndex};\nuse super::concept_types::*;\nuse super::body::{IfcBodyConceptRequest, IfcBodyConceptReport};")
source = source.replace("IfcBodyTessellationRequest", "IfcBodyConceptRequest").replace("IfcBodyTessellationReport", "IfcBodyConceptReport")
(concepts / "types.rs").write_text(shared, encoding="utf-8")
(body / "domain.rs").write_text(domain, encoding="utf-8")
path.write_text(source, encoding="utf-8")
for child in ["helpers.rs", "🧪️.rs"]:
    child_path = tess / child
    current = child_path.read_text(encoding="utf-8")
    child_path.write_text(current.replace("IfcBodyTessellationRequest", "IfcBodyConceptRequest").replace("IfcBodyTessellationReport", "IfcBodyConceptReport"), encoding="utf-8")
print("[DEBUG] Canonical native concept/body data extracted")
