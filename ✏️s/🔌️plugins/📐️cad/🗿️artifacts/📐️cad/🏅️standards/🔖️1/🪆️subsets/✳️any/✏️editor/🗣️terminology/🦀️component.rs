//! 🗣️ CAD app — the single `app_labels!` block for the whole app plus the resolvers that pick a cell
//! from `CadConfig`'s locale/terminology pair. Every taxonomy node that renders text reads its
//! strings from here; there is deliberately no second label set anywhere in the plugin.

use crate::editor::cad::config::CadConfig;
use crate::editor::cad::TYPOLOGY_CATALOG;
use semio_framework_plugin::{AppLabels, Locale, Terminology};

//#region 🔖️Terminology
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the CAD app; one field per label makes every terminology×locale combination compile-checked.
    pub struct CadLabels {
        // entity nouns — remapped under the "reuse" terminology
        object: native_en "Object", native_de "Objekt", reuse_en "Building component", reuse_de "Baukomponente";
        objects: native_en "Objects", native_de "Objekte", reuse_en "Building components", reuse_de "Baukomponenten";
        primitive: native_en "Primitive", native_de "Grundkörper", reuse_en "Component part", reuse_de "Bauteil";
        // model-definition pane / document-tree section names
        pane_shape: native_en "Shape", native_de "Form", reuse_en "Shape", reuse_de "Form";
        pane_building: native_en "Building", native_de "Gebäude", reuse_en "Building", reuse_de "Gebäude";
        pane_energy: native_en "Energy", native_de "Energie", reuse_en "Energy", reuse_de "Energie";
        pane_structure_classic: native_en "Structure Classic", native_de "Tragwerk Klassisch", reuse_en "Structure Classic", reuse_de "Tragwerk Klassisch";
        references: native_en "References", native_de "Referenzen", reuse_en "References", reuse_de "Referenzen";
        nodes: native_en "Nodes", native_de "Knoten", reuse_en "Nodes", reuse_de "Knoten";
        // catalogue
        typologies: native_en "Typologies", native_de "Typologien", reuse_en "Typologies", reuse_de "Typologien";
        typology_box: native_en "Box", native_de "Quader", reuse_en "Box", reuse_de "Quader";
        typology_slab: native_en "Slab", native_de "Platte", reuse_en "Slab", reuse_de "Platte";
        typology_column: native_en "Column", native_de "Stütze", reuse_en "Column", reuse_de "Stütze";
        typology_beam: native_en "Beam", native_de "Träger", reuse_en "Beam", reuse_de "Träger";
        typology_wall: native_en "Wall", native_de "Wand", reuse_en "Wall", reuse_de "Wand";
        typology_external_wall: native_en "External Wall", native_de "Außenwand", reuse_en "External Wall", reuse_de "Außenwand";
        // inspector group titles
        reference: native_en "Reference", native_de "Referenz", reuse_en "Reference", reuse_de "Referenz";
        node: native_en "Node", native_de "Knoten", reuse_en "Node", reuse_de "Knoten";
        // tree item actions
        hide: native_en "Hide", native_de "Ausblenden", reuse_en "Hide", reuse_de "Ausblenden";
        show: native_en "Show", native_de "Anzeigen", reuse_en "Show", reuse_de "Anzeigen";
        lock: native_en "Lock", native_de "Sperren", reuse_en "Lock", reuse_de "Sperren";
        unlock: native_en "Unlock", native_de "Entsperren", reuse_en "Unlock", reuse_de "Entsperren";
        duplicate: native_en "Duplicate", native_de "Duplizieren", reuse_en "Duplicate", reuse_de "Duplizieren";
        delete: native_en "Delete", native_de "Löschen", reuse_en "Delete", reuse_de "Löschen";
        // inspector field chrome
        label: native_en "Label", native_de "Bezeichnung", reuse_en "Label", reuse_de "Bezeichnung";
        typology: native_en "Typology", native_de "Typologie", reuse_en "Typology", reuse_de "Typologie";
        hidden: native_en "Hidden", native_de "Ausgeblendet", reuse_en "Hidden", reuse_de "Ausgeblendet";
        locked: native_en "Locked", native_de "Gesperrt", reuse_en "Locked", reuse_de "Gesperrt";
        position: native_en "Position", native_de "Position", reuse_en "Position", reuse_de "Position";
        scale: native_en "Scale", native_de "Skalierung", reuse_en "Scale", reuse_de "Skalierung";
        rotation: native_en "Rotation", native_de "Drehung", reuse_en "Rotation", reuse_de "Drehung";
        slot: native_en "Slot", native_de "Platz", reuse_en "Slot", reuse_de "Platz";
        kind: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        id: native_en "Id", native_de "Id", reuse_en "Id", reuse_de "Id";
        source: native_en "Source", native_de "Quelle", reuse_en "Source", reuse_de "Quelle";
        width_world: native_en "Width (world)", native_de "Breite (Weltkoordinaten)", reuse_en "Width (world)", reuse_de "Breite (Weltkoordinaten)";
        // catalogue / tree chrome
        none_placeholder: native_en "(none)", native_de "(keine)", reuse_en "(none)", reuse_de "(keine)";
        // properties fallback + engagement chrome
        schema: native_en "Schema", native_de "Schema", reuse_en "Schema", reuse_de "Schema";
        utility: native_en "Utility", native_de "Werkzeug", reuse_en "Utility", reuse_de "Werkzeug";
        action_placeholder: native_en "Action", native_de "Aktion", reuse_en "Action", reuse_de "Aktion";
        ok: native_en "OK", native_de "OK", reuse_en "OK", reuse_de "OK";
        selected: native_en "selected", native_de "ausgewählt", reuse_en "selected", reuse_de "ausgewählt";
        step: native_en "Step", native_de "Schritt", reuse_en "Step", reuse_de "Schritt";
    }
}

/// 🗣️ B1: `cfg.locale`-driven counterpart of the deleted `ViewModel`-driven `is_de_locale`.
pub async fn cad_is_de_locale(cfg: &CadConfig) -> bool {
    cfg.locale.starts_with("de")
}
/// 🗣️ `CadConfig.locale` (a BCP-47 tag, was shell-provided `ViewModel.locale` pre-B1) mapped onto the
/// SDK's exhaustive `Locale` enum.
pub async fn cad_locale(cfg: &CadConfig) -> Locale {
    if cad_is_de_locale(cfg) {
        Locale::De
    } else {
        Locale::En
    }
}

/// 🗣️ `CadConfig.terminology` mapped onto the SDK's exhaustive `Terminology` enum; unknown/empty ids
/// fall back to `Native`.
pub async fn cad_terminology(cfg: &CadConfig) -> Terminology {
    if cfg.terminology == "reuse" {
        Terminology::Reuse
    } else {
        Terminology::Native
    }
}

/// 🗣️ Resolves the active `CadLabels` cell from the config-carried locale/terminology (was
/// shell-provided `ViewModel`, deleted by B1) via the SDK's two-axis `AppLabels::labels`.
pub async fn cad_labels(cfg: &CadConfig) -> &'static CadLabels {
    CadLabels::labels(cad_locale(cfg), cad_terminology(cfg))
}

/// 🗣️ Resolves a typology catalog entry's display label from its stable id; unknown ids fall back to the catalog's native English text or the raw id.
pub async fn typology_label<'a>(typology: &'a str, labels: &CadLabels) -> &'a str {
    match typology {
        "spatial.shape.primitive.box" => labels.typology_box.as_str(),
        "building.building.slab" | "structure.structure.onewayreinforcedconcreteslab" => labels.typology_slab.as_str(),
        "building.building.column" | "structure.structure.reinforcedconcretecolumn" => labels.typology_column.as_str(),
        "building.building.beam" => labels.typology_beam.as_str(),
        "building.building.wall" => labels.typology_wall.as_str(),
        "energy.energy.externalwall" => labels.typology_external_wall.as_str(),
        other => TYPOLOGY_CATALOG.iter().find(|entry| entry.typology == other).map_or(other, |entry| entry.label),
    }
}
//#endregion 🔖️Terminology
