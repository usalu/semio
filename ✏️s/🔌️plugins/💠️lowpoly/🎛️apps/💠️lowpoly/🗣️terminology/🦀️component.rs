//! 🗣️ Lowpoly play app — the single `app_labels!` label set. Never split across taxonomy nodes: every
//! window/panel/option component imports `LowpolyLabels` from here.

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the lowpoly mesh editor; one field per label makes every locale combination compile-checked.
    pub struct LowpolyLabels {
        meshes: native_en "Meshes", native_de "Netze", reuse_en "Meshes", reuse_de "Netze";
        primitives: native_en "Primitives", native_de "Primitive", reuse_en "Primitives", reuse_de "Primitive";
        paint_layers: native_en "Paint Layers", native_de "Malebenen", reuse_en "Paint Layers", reuse_de "Malebenen";
        vertices: native_en "Vertices", native_de "Eckpunkte", reuse_en "Vertices", reuse_de "Eckpunkte";
        edges: native_en "Edges", native_de "Kanten", reuse_en "Edges", reuse_de "Kanten";
        faces: native_en "Faces", native_de "Flächen", reuse_en "Faces", reuse_de "Flächen";
        flip_normal: native_en "Flip normal", native_de "Normale umkehren", reuse_en "Flip normal", reuse_de "Normale umkehren";
        primitive_box: native_en "Cube", native_de "Würfel", reuse_en "Cube", reuse_de "Würfel";
        primitive_plane: native_en "Plane", native_de "Ebene", reuse_en "Plane", reuse_de "Ebene";
        primitive_cylinder: native_en "Cylinder", native_de "Zylinder", reuse_en "Cylinder", reuse_de "Zylinder";
        primitive_cone: native_en "Cone", native_de "Kegel", reuse_en "Cone", reuse_de "Kegel";
        primitive_ico_sphere: native_en "Ico Sphere", native_de "Ikokugel", reuse_en "Ico Sphere", reuse_de "Ikokugel";
        object: native_en "Object", native_de "Objekt", reuse_en "Building component", reuse_de "Baukomponente";
        transform: native_en "Transform", native_de "Transformation", reuse_en "Transform", reuse_de "Transformation";
        utility_params: native_en "Utility Params", native_de "Werkzeugparameter", reuse_en "Utility Params", reuse_de "Werkzeugparameter";
        window_main: native_en "Model", native_de "Modell", reuse_en "Model", reuse_de "Modell";
        window_uv: native_en "UV", native_de "UV", reuse_en "UV", reuse_de "UV";
        // inspector field labels
        name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        smooth_shading: native_en "Smooth Shading", native_de "Weiche Schattierung", reuse_en "Smooth Shading", reuse_de "Weiche Schattierung";
        selection: native_en "Selection", native_de "Auswahl", reuse_en "Selection", reuse_de "Auswahl";
        selection_mode: native_en "Selection Mode", native_de "Auswahlmodus", reuse_en "Selection Mode", reuse_de "Auswahlmodus";
        utility: native_en "Utility", native_de "Werkzeug", reuse_en "Utility", reuse_de "Werkzeug";
        selected: native_en "selected", native_de "ausgewählt", reuse_en "selected", reuse_de "ausgewählt";
        extrude: native_en "Extrude", native_de "Extrudieren", reuse_en "Extrude", reuse_de "Extrudieren";
        triangulate: native_en "Triangulate", native_de "Triangulieren", reuse_en "Triangulate", reuse_de "Triangulieren";
        extrude_distance: native_en "Extrude Distance", native_de "Extrusionsabstand", reuse_en "Extrude Distance", reuse_de "Extrusionsabstand";
        inset_amount: native_en "Inset Amount", native_de "Einzugsbetrag", reuse_en "Inset Amount", reuse_de "Einzugsbetrag";
        bevel_amount: native_en "Bevel Amount", native_de "Fasenbetrag", reuse_en "Bevel Amount", reuse_de "Fasenbetrag";
        bevel_segments: native_en "Bevel Segments", native_de "Fasensegmente", reuse_en "Bevel Segments", reuse_de "Fasensegmente";
        loop_cuts: native_en "Loop Cuts", native_de "Schleifenschnitte", reuse_en "Loop Cuts", reuse_de "Schleifenschnitte";
        decimate_ratio: native_en "Decimate Ratio", native_de "Dezimierungsverhältnis", reuse_en "Decimate Ratio", reuse_de "Dezimierungsverhältnis";
        snap_grid: native_en "Snap Grid", native_de "Rastergröße", reuse_en "Snap Grid", reuse_de "Rastergröße";
        mirror_axis: native_en "Mirror Axis", native_de "Spiegelachse", reuse_en "Mirror Axis", reuse_de "Spiegelachse";
        brush_size: native_en "Brush Size", native_de "Pinselgröße", reuse_en "Brush Size", reuse_de "Pinselgröße";
        brush_opacity: native_en "Brush Opacity", native_de "Pinseldeckkraft", reuse_en "Brush Opacity", reuse_de "Pinseldeckkraft";
        brush_hardness: native_en "Brush Hardness", native_de "Pinselhärte", reuse_en "Brush Hardness", reuse_de "Pinselhärte";
        // engagement + measures
        snap: native_en "Snap", native_de "Einrasten", reuse_en "Snap", reuse_de "Einrasten";
        smooth: native_en "Smooth", native_de "Glätten", reuse_en "Smooth", reuse_de "Glätten";
        show_edges: native_en "Show Edges", native_de "Kanten anzeigen", reuse_en "Show Edges", reuse_de "Kanten anzeigen";
        select: native_en "Select", native_de "Auswählen", reuse_en "Select", reuse_de "Auswählen";
        mesh: native_en "Mesh", native_de "Netz", reuse_en "Mesh", reuse_de "Netz";
        face: native_en "Face", native_de "Fläche", reuse_en "Face", reuse_de "Fläche";
        edge: native_en "Edge", native_de "Kante", reuse_en "Edge", reuse_de "Kante";
        vertex: native_en "Vertex", native_de "Eckpunkt", reuse_en "Vertex", reuse_de "Eckpunkt";
        rectangle: native_en "Rectangle", native_de "Rechteck", reuse_en "Rectangle", reuse_de "Rechteck";
        lasso: native_en "Lasso", native_de "Lasso", reuse_en "Lasso", reuse_de "Lasso";
        selective: native_en "Selective", native_de "Selektiv", reuse_en "Selective", reuse_de "Selektiv";
        additive: native_en "Additive", native_de "Additiv", reuse_en "Additive", reuse_de "Additiv";
        subtractive: native_en "Subtractive", native_de "Subtraktiv", reuse_en "Subtractive", reuse_de "Subtraktiv";
        invertive: native_en "Invertive", native_de "Invertierend", reuse_en "Invertive", reuse_de "Invertierend";
        brush_group: native_en "Brush", native_de "Pinsel", reuse_en "Brush", reuse_de "Pinsel";
    }
}

/// 🗣️ Resolves the labels for a config's locale — the single call site every render/measure/engagement
/// builder uses.
pub fn lowpoly_play_labels(config: &crate::apps::lowpoly::config::LowpolyConfig) -> &'static LowpolyLabels {
    semio_framework_plugin::resolve_labels_for_locale::<LowpolyLabels>(&config.locale)
}

/// 🗣️ Resolves a primitive catalogue entry's display label from its stable kind; unknown kinds fall
/// back to the catalog's native English text.
pub fn primitive_catalog_label(kind: &str, fallback_label: &'static str, labels: &LowpolyLabels) -> semio_framework_plugin::Label {
    match kind {
        "box" => labels.primitive_box.into(),
        "plane" => labels.primitive_plane.into(),
        "cylinder" => labels.primitive_cylinder.into(),
        "cone" => labels.primitive_cone.into(),
        "ico_sphere" => labels.primitive_ico_sphere.into(),
        _ => semio_framework_plugin::Label::data(fallback_label),
    }
}
//#endregion 🔖️Labels
