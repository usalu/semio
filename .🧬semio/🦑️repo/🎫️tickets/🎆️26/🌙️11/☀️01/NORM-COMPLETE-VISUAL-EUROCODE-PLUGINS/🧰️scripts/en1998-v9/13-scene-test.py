"""en1998 v9 step 13: scene test for the foundation-layer precedence of the spectrum. python -I"""
import sys; sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib import *
T = ANY / "🎬️scene/🧪️tests/🔬️unit/🦀️.rs"
sub(T, '''    let (declared_b, tone) = label_of(&document);
    assert!(declared_b.en.contains("ground type C") && declared_b.en.contains("spectra use: B"), "{}", declared_b.en);
    assert!(matches!(tone, Tone::Warning));
    document.site.v_s30_m_s = 250.0;
    let (derived_c, tone) = label_of(&document);
    assert!(derived_c.en.contains("ground type C") && !derived_c.en.contains("spectra use"), "{}", derived_c.en);
    assert!(matches!(tone, Tone::Info));
    assert!(matches!(super::response::annex_params(&document), super::schema::AnnexParams::En { ground: 'C', .. }));
''', '''    let (from_layers, tone) = label_of(&document);
    assert!(from_layers.en.contains("ground type C") && from_layers.en.contains("spectra follow these layers") && !from_layers.en.contains("spectra use"), "{}", from_layers.en);
    assert!(from_layers.de.contains("Spektren folgen diesen Schichten"), "{}", from_layers.de);
    assert!(matches!(tone, Tone::Info));
    document.site.v_s30_m_s = 150.0;
    let (layers_win, tone) = label_of(&document);
    assert!(layers_win.en.contains("ground type C") && layers_win.en.contains("spectra follow these layers"), "{}", layers_win.en);
    assert!(matches!(tone, Tone::Info));
    assert!(matches!(super::response::annex_params(&document), super::schema::AnnexParams::En { ground: 'C', .. }), "the foundation layers take precedence over the site v_s,30");
    document.soil_layers.clear();
    assert!(matches!(super::response::annex_params(&document), super::schema::AnnexParams::En { ground: 'D', .. }), "without layers the site v_s,30 applies");
    document.soil_layers = vec![row("sl-1", 30.0, 250.0)];
''')
CT = S / "🧪️tests/⚖️compliance/🦀️.rs"
sub(CT, '''#[semio_framework_async_macros::async_test]
async fn simple_masonry_measures_the_wall_rows_per_direction_and_reports_the_unverified_limit() {''', '''#[semio_framework_async_macros::async_test]
async fn the_spectrum_ground_follows_the_foundation_layers_before_the_site_input() {
    let mut snap = En1998Snapshot::compliant_de_office();
    snap.annex = "en".into();
    snap.site.en_ground_type = "B".into();
    snap.site.v_s30_m_s = 500.0;
    snap.soil_layers.clear();
    assert_eq!(part_1::effective_v_s30(&snap), (500.0, part_1::GroundSource::Site));
    assert_eq!(part_1::spectrum_ground_letter_of(&snap), 'B');
    let foundation = snap.foundations[0].id.clone();
    let layer = |id: &str, thickness_m: f64, v_s: f64| semio_s_artifact_norm_en1998_model::En1998SoilLayer { id: id.into(), foundation_id: foundation.clone(), thickness_m, unit_weight_n_m3: 18000.0, shear_wave_velocity_m_s: v_s, n1_60: 0.0, fines_percent: 0.0 };
    snap.soil_layers = vec![layer("sl-1", 30.0, 150.0)];
    let (v, source) = part_1::effective_v_s30(&snap);
    assert!((v - 150.0).abs() < 1e-9 && source == part_1::GroundSource::FoundationLayers { foundation_id: foundation.clone() });
    assert_eq!(part_1::spectrum_ground_letter_of(&snap), 'D', "the layers replace the site value");
    snap.site.liquefiable_layer = true;
    assert_eq!(part_1::spectrum_ground_letter_of(&snap), 'B', "S2 keeps the special study and the declared letter");
}

#[semio_framework_async_macros::async_test]
async fn simple_masonry_measures_the_wall_rows_per_direction_and_reports_the_unverified_limit() {''')
flush()
