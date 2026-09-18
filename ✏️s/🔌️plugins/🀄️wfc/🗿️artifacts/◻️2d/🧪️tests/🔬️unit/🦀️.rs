//! 🧪️ WFC 2D artifact root — identity, capability and declaration laws.

use crate::{artifact_kind, definition, WFC_2D_DIALECT, WFC_2D_DOCUMENT_SCHEMA};

/// 🪪️ The dialect, the document schema and the capability identity are ONE string, spelled once.
#[test]
fn dialect_matches_the_document_schema() {
    assert_eq!(WFC_2D_DOCUMENT_SCHEMA, "s.wfc.wfc2d");
    assert_eq!(WFC_2D_DIALECT.artifact_kind, WFC_2D_DOCUMENT_SCHEMA);
    assert_eq!(WFC_2D_DIALECT.standard.0, "1");
}

/// 🗿️ The OS kind id is the `<dimension>.<component_kind>` pair the plan fixed.
#[test]
fn artifact_kind_is_the_declared_os_kind() {
    let kind = artifact_kind();
    assert_eq!(kind.id, "2d.wfc2d");
    assert_eq!(kind.component_kind, "wfc2d");
    assert_eq!(kind.dimension, "2d");
    assert_eq!(kind.schema, WFC_2D_DOCUMENT_SCHEMA);
}

/// 🧾️ Every capability row parses — a malformed identity would otherwise only surface at boot.
#[test]
fn definition_builds() {
    definition().expect("every wfc2d capability row is well formed");
}
