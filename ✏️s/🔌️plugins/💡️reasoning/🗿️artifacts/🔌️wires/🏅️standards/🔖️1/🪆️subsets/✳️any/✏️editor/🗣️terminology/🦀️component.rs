//! 🗣️ Wires play app — the single `app_labels!` block plus the locale resolvers every taxonomy node
//! reaches for. Deliberately ONE block for the whole app (never split per window/panel): the macro's
//! value is that every locale×terminology combination is compile-checked in one place.

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the mindmap wires app; one field per label makes every locale×terminology combination compile-checked. No distinct reuse-terminology concept for this app, so reuse repeats native.
    pub struct WiresLabels {
        identities: native_en "Identities", native_de "Identitäten", reuse_en "Identities", reuse_de "Identitäten";
        relationships: native_en "Relationships", native_de "Beziehungen", reuse_en "Relationships", reuse_de "Beziehungen";
        identity_kinds: native_en "Identity kinds", native_de "Identitätsarten", reuse_en "Identity kinds", reuse_de "Identitätsarten";
        relationship_kinds: native_en "Relationship kinds", native_de "Beziehungsarten", reuse_en "Relationship kinds", reuse_de "Beziehungsarten";
        relationship_kind_owns: native_en "Owns", native_de "Besitzt", reuse_en "Owns", reuse_de "Besitzt";
        relationship_kind_is: native_en "Is", native_de "Ist", reuse_en "Is", reuse_de "Ist";
        relationship_kind_references: native_en "References", native_de "Referenziert", reuse_en "References", reuse_de "Referenziert";
        relationship_kind_has: native_en "Has", native_de "Hat", reuse_en "Has", reuse_de "Hat";
        window_main: native_en "Canvas", native_de "Leinwand", reuse_en "Canvas", reuse_de "Leinwand";
        mode_edit: native_en "Edit", native_de "Bearbeiten", reuse_en "Edit", reuse_de "Bearbeiten";
    }
}
//#endregion 🔖️Labels

//#region 🔖️Resolvers
/// 🗣️ Resolves a relationship kind's display name — takes `&WiresLabels` (an app-only type), so it
/// stays app-level regardless of consumer count (artifacts must never depend on apps).
pub fn relationship_kind_display_name(kind: &str, labels: &WiresLabels) -> String {
    match kind {
        "owns" => labels.relationship_kind_owns.as_str().to_string(),
        "is" => labels.relationship_kind_is.as_str().to_string(),
        "references" => labels.relationship_kind_references.as_str().to_string(),
        "has" => labels.relationship_kind_has.as_str().to_string(),
        _ => kind.to_string(),
    }
}
//#endregion 🔖️Resolvers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{AppLabels, Locale, Terminology};

    #[test]
    fn relationship_kind_display_name_resolves_labels() {
        assert_eq!(relationship_kind_display_name("is", WiresLabels::labels(Locale::En, Terminology::Native)), "Is");
        assert_eq!(relationship_kind_display_name("unknown", WiresLabels::labels(Locale::En, Terminology::Native)), "unknown");
    }
}
//#endregion 🧪️Tests
