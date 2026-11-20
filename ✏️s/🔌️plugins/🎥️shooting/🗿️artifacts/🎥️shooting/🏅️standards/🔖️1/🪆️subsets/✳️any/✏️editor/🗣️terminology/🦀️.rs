//! 🗣️ Shooting play app — the single `app_labels!` block plus the locale resolver every taxonomy node
//! reaches for. Deliberately ONE block for the whole app (never split per window/panel): the macro's
//! value is that every locale combination is compile-checked in one place.

//#region 🔖️Labels
// 🗣️ Complete UI label set for the shooting app; one field per label makes every locale combination
// compile-checked. (A plain `//` comment, not `///`: rustdoc does not generate documentation for macro
// invocations, so a doc comment here is dead and clippy/rustc flag it as unused.)
semio_ui_locale::app_labels! {
    pub struct ShootingLabels {
        shot_framing: native_en "Framing", native_de "Bildausschnitt", reuse_en "Framing", reuse_de "Bildausschnitt";
        shot_framing_scene: native_en "Follow scene", native_de "Szeneneinstellung", reuse_en "Follow scene", reuse_de "Szeneneinstellung";
        shot_framing_fit: native_en "Fit model", native_de "Modell einpassen", reuse_en "Fit model", reuse_de "Modell einpassen";
        shot_framing_manual: native_en "Manual", native_de "Manuell", reuse_en "Manual", reuse_de "Manuell";
        drawing_projection: native_en "Projection", native_de "Projektion", reuse_en "Projection", reuse_de "Projektion";
        drawing_perspective: native_en "Perspective", native_de "Perspektivisch", reuse_en "Perspective", reuse_de "Perspektivisch";
        drawing_orthographic: native_en "Orthographic", native_de "Orthografisch", reuse_en "Orthographic", reuse_de "Orthografisch";
        eye_x: native_en "Camera position X", native_de "Kameraposition X", reuse_en "Camera position X", reuse_de "Kameraposition X";
        eye_y: native_en "Camera position Y", native_de "Kameraposition Y", reuse_en "Camera position Y", reuse_de "Kameraposition Y";
        eye_z: native_en "Camera position Z", native_de "Kameraposition Z", reuse_en "Camera position Z", reuse_de "Kameraposition Z";
        target_x: native_en "Camera target X", native_de "Kameraziel X", reuse_en "Camera target X", reuse_de "Kameraziel X";
        target_y: native_en "Camera target Y", native_de "Kameraziel Y", reuse_en "Camera target Y", reuse_de "Kameraziel Y";
        target_z: native_en "Camera target Z", native_de "Kameraziel Z", reuse_en "Camera target Z", reuse_de "Kameraziel Z";
        up_x: native_en "Camera up X", native_de "Kamera-Hochachse X", reuse_en "Camera up X", reuse_de "Kamera-Hochachse X";
        up_y: native_en "Camera up Y", native_de "Kamera-Hochachse Y", reuse_en "Camera up Y", reuse_de "Kamera-Hochachse Y";
        up_z: native_en "Camera up Z", native_de "Kamera-Hochachse Z", reuse_en "Camera up Z", reuse_de "Kamera-Hochachse Z";
        drawing_zoom: native_en "Zoom", native_de "Zoom", reuse_en "Zoom", reuse_de "Zoom";
        drawing_fov: native_en "Perspective field of view (°)", native_de "Perspektivischer Bildwinkel (°)", reuse_en "Perspective field of view (°)", reuse_de "Perspektivischer Bildwinkel (°)";
        drawing_section: native_en "SVG drawing", native_de "SVG-Zeichnung", reuse_en "SVG drawing", reuse_de "SVG-Zeichnung";
        drawing_fill_color: native_en "Fill color (empty for material)", native_de "Füllfarbe (leer für Material)", reuse_en "Fill color (empty for material)", reuse_de "Füllfarbe (leer für Material)";
        drawing_fill_opacity: native_en "Fill opacity", native_de "Deckkraft der Füllung", reuse_en "Fill opacity", reuse_de "Deckkraft der Füllung";
        drawing_live: native_en "Live camera", native_de "Live-Kamera", reuse_en "Live camera", reuse_de "Live-Kamera";
        drawing_saved: native_en "Shot camera", native_de "Aufnahmekamera", reuse_en "Shot camera", reuse_de "Aufnahmekamera";
        drawing_background: native_en "Background layer", native_de "Hintergrundebene", reuse_en "Background layer", reuse_de "Hintergrundebene";
        drawing_fill: native_en "Face fills", native_de "Flächenfüllungen", reuse_en "Face fills", reuse_de "Flächenfüllungen";
        drawing_visible_lines: native_en "Visible lines", native_de "Sichtbare Linien", reuse_en "Visible lines", reuse_de "Sichtbare Linien";
        drawing_hidden_lines: native_en "Hidden lines", native_de "Verdeckte Linien", reuse_en "Hidden lines", reuse_de "Verdeckte Linien";
        drawing_visible_color: native_en "Visible line color", native_de "Farbe sichtbarer Linien", reuse_en "Visible line color", reuse_de "Farbe sichtbarer Linien";
        drawing_hidden_color: native_en "Hidden line color", native_de "Farbe verdeckter Linien", reuse_en "Hidden line color", reuse_de "Farbe verdeckter Linien";
        drawing_visible_width: native_en "Visible line width (px)", native_de "Sichtbare Linienbreite (px)", reuse_en "Visible line width (px)", reuse_de "Sichtbare Linienbreite (px)";
        drawing_hidden_width: native_en "Hidden line width (px)", native_de "Verdeckte Linienbreite (px)", reuse_en "Hidden line width (px)", reuse_de "Verdeckte Linienbreite (px)";
        drawing_line_easing: native_en "Corner radius (px)", native_de "Eckenradius (px)", reuse_en "Corner radius (px)", reuse_de "Eckenradius (px)";
        drawing_merge_lines: native_en "Merge connected lines", native_de "Verbundene Linien zusammenführen", reuse_en "Merge connected lines", reuse_de "Verbundene Linien zusammenführen";
        drawing_enabled: native_en "On", native_de "Ein", reuse_en "On", reuse_de "Ein";
        drawing_disabled: native_en "Off", native_de "Aus", reuse_en "Off", reuse_de "Aus";
        inspection_title: native_en "Inspection", native_de "Inspektion", reuse_en "Inspection", reuse_de "Inspektion";
        shots: native_en "Shots", native_de "Aufnahmen", reuse_en "Shots", reuse_de "Aufnahmen";
        assets: native_en "Assets", native_de "Objekte", reuse_en "Assets", reuse_de "Objekte";
        add_shot: native_en "Add Shot", native_de "Aufnahme hinzufügen", reuse_en "Add Shot", reuse_de "Aufnahme hinzufügen";
        add_asset: native_en "Add Asset", native_de "Objekt hinzufügen", reuse_en "Add Asset", reuse_de "Objekt hinzufügen";
        svg_rectangle: native_en "SVG Rectangle", native_de "SVG Rechteck", reuse_en "SVG Rectangle", reuse_de "SVG Rechteck";
        png_rectangle: native_en "PNG Rectangle", native_de "PNG Rechteck", reuse_en "PNG Rectangle", reuse_de "PNG Rechteck";
        svg_ellipse: native_en "SVG Ellipse", native_de "SVG Ellipse", reuse_en "SVG Ellipse", reuse_de "SVG Ellipse";
        png_ellipse: native_en "PNG Ellipse", native_de "PNG Ellipse", reuse_en "PNG Ellipse", reuse_de "PNG Ellipse";
        glb_asset: native_en "GLB Asset", native_de "GLB-Objekt", reuse_en "GLB Asset", reuse_de "GLB-Objekt";
        shot: native_en "Shot", native_de "Aufnahme", reuse_en "Shot", reuse_de "Aufnahme";
        asset: native_en "Asset", native_de "Objekt", reuse_en "Asset", reuse_de "Objekt";
        camera_label_placeholder: native_en "Camera label", native_de "Kamera-Bezeichnung", reuse_en "Camera label", reuse_de "Kamera-Bezeichnung";
        load_camera: native_en "Load camera", native_de "Kamera laden", reuse_en "Load camera", reuse_de "Kamera laden";
        shot_label_placeholder: native_en "Shot label", native_de "Aufnahme-Bezeichnung", reuse_en "Shot label", reuse_de "Aufnahme-Bezeichnung";
        no_shot: native_en "No shot", native_de "Keine Aufnahme", reuse_en "No shot", reuse_de "Keine Aufnahme";
        format_select_label: native_en "Format", native_de "Format", reuse_en "Format", reuse_de "Format";
        shape_select_label: native_en "Shape", native_de "Form", reuse_en "Shape", reuse_de "Form";
        format_svg: native_en "SVG", native_de "SVG", reuse_en "SVG", reuse_de "SVG";
        format_png: native_en "PNG", native_de "PNG", reuse_en "PNG", reuse_de "PNG";
        shape_rectangle: native_en "Rectangle", native_de "Rechteck", reuse_en "Rectangle", reuse_de "Rechteck";
        shape_ellipse: native_en "Ellipse", native_de "Ellipse", reuse_en "Ellipse", reuse_de "Ellipse";
        window_scene: native_en "Scene", native_de "Szene", reuse_en "Scene", reuse_de "Szene";
        window_icon: native_en "Icon", native_de "Symbol", reuse_en "Icon", reuse_de "Symbol";
        measure_center_model: native_en "Center Model", native_de "Modell zentrieren", reuse_en "Center Model", reuse_de "Modell zentrieren";
        measure_sun: native_en "Sun", native_de "Sonne", reuse_en "Sun", reuse_de "Sonne";
        measure_sun_azimuth: native_en "Sun Azimuth", native_de "Sonnenazimut", reuse_en "Sun Azimuth", reuse_de "Sonnenazimut";
        measure_sun_elevation: native_en "Sun Elevation", native_de "Sonnenhöhe", reuse_en "Sun Elevation", reuse_de "Sonnenhöhe";
        measure_sun_intensity: native_en "Sun Intensity", native_de "Sonnenintensität", reuse_en "Sun Intensity", reuse_de "Sonnenintensität";
        measure_ambient: native_en "Ambient", native_de "Umgebungslicht", reuse_en "Ambient", reuse_de "Umgebungslicht";
        measure_shadow: native_en "Shadow", native_de "Schatten", reuse_en "Shadow", reuse_de "Schatten";
        measure_roughness: native_en "Roughness", native_de "Rauheit", reuse_en "Roughness", reuse_de "Rauheit";
        field_label: native_en "Label", native_de "Bezeichnung", reuse_en "Label", reuse_de "Bezeichnung";
        field_format: native_en "Format", native_de "Format", reuse_en "Format", reuse_de "Format";
        field_shape: native_en "Shape", native_de "Form", reuse_en "Shape", reuse_de "Form";
        field_width: native_en "Width", native_de "Breite", reuse_en "Width", reuse_de "Breite";
        field_height: native_en "Height", native_de "Höhe", reuse_en "Height", reuse_de "Höhe";
        field_name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        field_url: native_en "URL", native_de "URL", reuse_en "URL", reuse_de "URL";
    }
}
//#endregion 🔖️Labels

//#region 🔖️Resolvers
/// 🗣️ Resolves the active label set from the shared OS-owned view context.
pub fn shooting_play_labels(view_state: &semio_plugin::ViewModel) -> &'static ShootingLabels {
    semio_plugin::resolve_labels::<ShootingLabels>(view_state)
}
//#endregion 🔖️Resolvers

//#region 🧪️Tests

//#endregion 🧪️Tests
