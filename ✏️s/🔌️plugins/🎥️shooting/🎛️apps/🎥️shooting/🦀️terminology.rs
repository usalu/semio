//! 🗣️ Shooting play app — the single `app_labels!` block plus the locale resolver every taxonomy node
//! reaches for. Deliberately ONE block for the whole app (never split per window/panel): the macro's
//! value is that every locale combination is compile-checked in one place.

use crate::apps::shooting::config::ShootingConfig;

//#region 🔖️Labels
/// 🗣️ Complete UI label set for the shooting app; one field per label makes every locale combination compile-checked.
semio_framework_plugin::app_labels! {
    pub struct ShootingLabels {
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
/// 🗣️ Resolves the active label set from `cfg.locale`; falls back to native English.
pub fn shooting_play_labels(cfg: &ShootingConfig) -> &'static ShootingLabels {
    semio_framework_plugin::resolve_labels_for_locale::<ShootingLabels>(&cfg.locale)
}
//#endregion 🔖️Resolvers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shooting_labels_resolve_native_english_by_default() {
        assert_eq!(shooting_play_labels(&ShootingConfig::default()).shots.as_str(), "Shots");
    }

    #[test]
    fn shooting_labels_resolve_german_from_the_config_locale() {
        assert_eq!(shooting_play_labels(&ShootingConfig { locale: "de-DE".into(), ..ShootingConfig::default() }).shots.as_str(), "Aufnahmen");
    }
}
//#endregion 🧪️Tests
