//! 🗣️ BIM editor terminology: the one `app_labels!` block for the whole app (never split per window or panel), so every locale cell is compile-checked
//! in one place. No language is the default: the locale always comes from the addressed view model. `{name}` and `{count}` are named placeholders.

macro_rules! bim_labels {
    ($( $field:ident: $en:literal, $de:literal; )+) => {
        semio_framework_ui_locale::app_labels! {
            pub struct BimLabels {
                $( $field: native_en $en, native_de $de, reuse_en $en, reuse_de $de );+
            }
        }
    };
}

//#region 🔖️Labels
// 🗣️ A plain `//` comment: rustdoc does not document macro invocations.
bim_labels! {
    window_plan: "Plan", "Grundriss";
    window_world: "3D", "3D";
    window_section: "Section", "Schnitt";
    window_schedule: "Schedule", "Mengen";
    panel_outliner: "Outliner", "Struktur";
    panel_properties: "Properties", "Eigenschaften";
    panel_library: "Library", "Bibliothek";
    mode_edit: "Edit", "Bearbeiten";

    kind_site: "Site", "Grundstück";
    kind_building: "Building", "Gebäude";
    kind_storey: "Storey", "Geschoss";
    kind_grid: "Grid line", "Rasterlinie";
    kind_wall: "Wall", "Wand";
    kind_curtain_wall: "Curtain wall", "Vorhangfassade";
    kind_column: "Column", "Stütze";
    kind_beam: "Beam", "Träger";
    kind_slab: "Slab", "Decke";
    kind_roof: "Roof", "Dach";
    kind_opening: "Opening", "Öffnung";
    kind_stair: "Stair", "Treppe";
    kind_railing: "Railing", "Geländer";
    kind_space: "Space", "Raum";
    kind_material: "Material", "Material";
    kind_wall_type: "Wall type", "Wandtyp";
    kind_slab_type: "Slab type", "Deckentyp";
    kind_roof_type: "Roof type", "Dachtyp";
    kind_column_type: "Column type", "Stützentyp";
    kind_beam_type: "Beam type", "Trägertyp";
    kind_window_type: "Window type", "Fenstertyp";
    kind_door_type: "Door type", "Türtyp";

    group_sites: "Sites", "Grundstücke";
    group_buildings: "Buildings", "Gebäude";
    group_storeys: "Storeys", "Geschosse";
    group_grids: "Grid lines", "Rasterlinien";
    group_walls: "Walls", "Wände";
    group_curtain_walls: "Curtain walls", "Vorhangfassaden";
    group_columns: "Columns", "Stützen";
    group_beams: "Beams", "Träger";
    group_slabs: "Slabs", "Decken";
    group_roofs: "Roofs", "Dächer";
    group_openings: "Openings", "Öffnungen";
    group_stairs: "Stairs", "Treppen";
    group_railings: "Railings", "Geländer";
    group_spaces: "Spaces", "Räume";
    group_materials: "Materials", "Materialien";
    group_wall_types: "Wall types", "Wandtypen";
    group_slab_types: "Slab types", "Deckentypen";
    group_roof_types: "Roof types", "Dachtypen";
    group_column_types: "Column types", "Stützentypen";
    group_beam_types: "Beam types", "Trägertypen";
    group_window_types: "Window types", "Fenstertypen";
    group_door_types: "Door types", "Türtypen";

    field_name: "Name", "Name";
    field_site: "Site", "Grundstück";
    field_building: "Building", "Gebäude";
    field_storey: "Storey", "Geschoss";
    field_host: "Host", "Träger-Bauteil";
    field_wall_type: "Wall type", "Wandtyp";
    field_slab_type: "Slab type", "Deckentyp";
    field_roof_type: "Roof type", "Dachtyp";
    field_column_type: "Column type", "Stützentyp";
    field_beam_type: "Beam type", "Trägertyp";
    field_material: "Material", "Material";
    field_category: "Category", "Kategorie";
    field_level: "Level", "Ebene";
    field_height: "Height", "Höhe";
    field_width: "Width", "Breite";
    field_depth: "Depth", "Tiefe";
    field_thickness: "Thickness", "Dicke";
    field_length: "Length", "Länge";
    field_offset: "Offset", "Versatz";
    field_sill: "Sill", "Brüstung";
    field_cut_height: "Plan cut height", "Schnitthöhe im Grundriss";
    field_stringer: "Stringer", "Wangen";
    field_nosing: "Nosing", "Trittüberstand";
    field_tread_thickness: "Tread thickness", "Trittstärke";
    field_riser: "Risers", "Setzstufen";
    field_landing_depth: "Landing depth", "Podesttiefe";
    field_rail_profile: "Top rail section", "Handlaufprofil";
    field_post_profile: "Post section", "Pfostenprofil";
    field_baluster: "Balusters", "Stäbe";
    field_infill: "Infill", "Füllung";
    field_elevation: "Elevation", "Höhenlage";
    field_top_elevation: "Top elevation", "Oberkante";
    field_base_offset: "Base offset", "Fußversatz";
    field_top: "Top", "Kopfbindung";
    field_location: "Location line", "Bezugslinie";
    field_phase: "Phase", "Phase";
    field_axis: "Axis", "Achse";
    field_start: "Start", "Anfang";
    field_end: "End", "Ende";
    field_position: "Position", "Position";
    field_origin: "Origin", "Ursprung";
    field_rotation: "Rotation", "Drehung";
    field_latitude: "Latitude", "Breitengrad";
    field_longitude: "Longitude", "Längengrad";
    field_true_north: "True north", "Nordrichtung";
    field_label: "Label", "Bezeichnung";
    field_number: "Number", "Nummer";
    field_usage: "Usage", "Nutzung";
    field_kind: "Kind", "Art";
    field_overhang: "Overhang", "Überstand";
    field_shape: "Shape", "Form";
    field_flight: "Flight", "Lauf";
    field_post_spacing: "Post spacing", "Pfostenabstand";
    field_density: "Density", "Dichte";
    field_layers: "Layers", "Schichten";
    field_panes: "Panes", "Scheiben";
    field_flip_hand: "Flip hand", "Anschlag spiegeln";
    field_flip_facing: "Flip facing", "Seite spiegeln";
    field_panel_material: "Panel material", "Füllungsmaterial";
    field_mullion_material: "Mullion material", "Pfostenmaterial";
    field_max_riser: "Max riser", "Max. Steigung";
    field_min_tread: "Min tread", "Min. Auftritt";
    field_conductivity: "Conductivity", "Leitfähigkeit";
    field_specific_heat: "Specific heat", "Wärmekapazität";
    field_frame_width: "Frame width", "Rahmenbreite";
    field_frame_depth: "Frame depth", "Rahmentiefe";
    field_leaves: "Leaves", "Flügel";
    field_swing: "Swing", "Aufschlag";
    field_area: "Area", "Fläche";
    field_volume: "Volume", "Volumen";
    field_mass: "Mass", "Masse";
    field_count: "Count", "Anzahl";
    field_type: "Type", "Typ";
    field_mixed: "Mixed", "Gemischt";
    section_authored: "Authored", "Eingegeben";
    section_inferred: "Inferred", "Abgeleitet";
    section_summary: "Summary", "Übersicht";

    action_add_named: "Add {name}", "{name} hinzufügen";
    action_delete: "Delete", "Löschen";
    action_delete_selection: "Delete selection", "Auswahl löschen";
    action_rename: "Rename", "Umbenennen";
    action_select_all: "Select all", "Alles auswählen";
    action_clear_selection: "Clear selection", "Auswahl aufheben";
    action_show_all_storeys: "All storeys", "Alle Geschosse";
    action_isolate_storey: "Isolate storey", "Geschoss isolieren";

    measure_storey: "Storey", "Geschoss";
    measure_cut_height: "Cut height", "Schnitthöhe";
    measure_projection: "Projection", "Projektion";
    measure_isolate: "Isolate storey", "Geschoss isolieren";
    measure_section_enabled: "Section plane", "Schnittebene";
    measure_section_axis: "Axis", "Achse";
    measure_section_offset: "Offset", "Versatz";
    measure_section_depth: "Depth", "Tiefe";
    measure_all_storeys: "All storeys", "Alle Geschosse";

    projection_three_point: "Perspective", "Perspektive";
    projection_orthographic: "Orthographic", "Orthografisch";
    projection_axonometric: "Axonometric", "Axonometrie";
    projection_one_point: "One point", "Ein Fluchtpunkt";
    projection_two_point: "Two point", "Zwei Fluchtpunkte";

    column_name: "Name", "Name";
    column_type: "Type", "Typ";
    column_storey: "Storey", "Geschoss";
    column_count: "Count", "Anzahl";
    column_length: "Length", "Länge";
    column_area: "Area", "Fläche";
    column_volume: "Volume", "Volumen";
    column_height: "Height", "Höhe";
    column_mass: "Mass", "Masse";
    row_total: "Total", "Summe";

    status_elements: "{count} elements", "{count} Elemente";
    status_selected: "{count} selected", "{count} ausgewählt";
    status_storey: "Storey {name}", "Geschoss {name}";
    status_all_storeys: "All storeys", "Alle Geschosse";
    status_no_storey: "No storey", "Kein Geschoss";

    empty_outliner: "The model is empty", "Das Modell ist leer";
    empty_properties: "Nothing selected", "Nichts ausgewählt";
    empty_library: "The library is empty", "Die Bibliothek ist leer";
    empty_schedule: "No quantities yet", "Noch keine Mengen";
    empty_plan: "No storey to show", "Kein Geschoss anzuzeigen";

    progress_inference: "Inferring the model", "Modell wird abgeleitet";
    cancel_inference: "Cancel", "Abbrechen";
    unknown_body: "Unknown body", "Unbekannter Bereich";
    section_line: "Section line", "Schnittlinie";
    summary_project: "Project", "Projekt";
    summary_schema: "Schema", "Schema";
}
//#endregion 🔖️Labels

/// 🗣️ Resolves the label set of the addressed view's locale and terminology.
pub fn bim_labels(view_state: &semio_framework_plugin::ViewModel) -> &'static BimLabels {
    semio_framework_plugin::resolve_labels::<BimLabels>(view_state)
}

impl BimLabels {
    /// 🧩️ Fills the `{name}` placeholder of a label.
    pub fn named(text: semio_framework_ui_locale::LabelText, name: &str) -> String {
        text.as_str().replace("{name}", name)
    }

    /// 🧮️ Fills the `{count}` placeholder of a label.
    pub fn counted(text: semio_framework_ui_locale::LabelText, count: usize) -> String {
        text.as_str().replace("{count}", &count.to_string())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
