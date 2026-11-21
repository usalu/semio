//! 📚️ Typed input vocabulary of the catalogue: unit, bilingual label and choices of every case input key; generated from `./🔣️.json` by `🧰️scripts/catalogue-bridge-gen-inputs.py`. See [`super`].

use semio_s_plugin_norm_compliance::design_case::{InputChoice, InputSpec};
use semio_s_plugin_norm_compliance::UnitId;

/// 📚️ Every input key the evaluators read; rows with a `scope` apply to the listed catalogue ids only.
pub static VOCABULARY: &[InputSpec] = &[
    InputSpec::choice("test_protocol", "Measurement conditions", "Messbedingungen", &[InputChoice::new("confirmed", "Pre-completion test to DIN EN ISO 9972 Annex NA confirmed", "Prüfung vor Fertigstellung nach DIN EN ISO 9972 Anhang NA bestätigt"), InputChoice::new("unconfirmed", "Test conditions unconfirmed", "Prüfbedingungen unbestätigt")]).example_text("confirmed"),
    InputSpec::choice("test_ventilation", "Ventilation of the tested subject", "Lüftung im geprüften Bereich", &[InputChoice::new("mechanical", "With mechanical ventilation", "Mit raumlufttechnischer Anlage"), InputChoice::new("natural", "Without mechanical ventilation", "Ohne raumlufttechnische Anlage")]).example_text("mechanical"),
    InputSpec::number("test_volume", "Heated or cooled air volume", "Beheiztes oder gekühltes Luftvolumen", UnitId::CubicMetre).range(0.0, 10000000.0).example(450.0).scope("G26-01,G26-03,G26-04"),
    InputSpec::number("test_volume", "Heated or cooled air volume", "Beheiztes oder gekühltes Luftvolumen", UnitId::CubicMetre).range(0.0, 10000000.0).example(2000.0).scope("G26-02"),
    InputSpec::number("test_pressurisation", "Measured airflow at +50 Pa", "Gemessener Volumenstrom bei +50 Pa", UnitId::CubicMetrePerHour).range(0.0, 100000000.0).example(650.0),
    InputSpec::number("test_depressurisation", "Measured airflow at −50 Pa", "Gemessener Volumenstrom bei −50 Pa", UnitId::CubicMetrePerHour).range(0.0, 100000000.0).example(675.0),
    InputSpec::number("test_area", "Envelope area of the tested subject", "Hüllfläche des geprüften Bereichs", UnitId::SquareMetre).range(0.0, 10000000.0).example(800.0).scope("G26-02,G26-03"),
    InputSpec::choice("test_basis", "Normalisation method", "Bezugsgröße", &[InputChoice::new("volume", "Normalise by conditioned volume", "Bezug auf das konditionierte Luftvolumen"), InputChoice::new("area", "Normalise by envelope area when volume exceeds 1,500 m³", "Bezug auf die Hüllfläche bei Luftvolumen über 1.500 m³")]).example_text("volume").scope("G26-03"),
    InputSpec::choice("test_zone_basis", "Basis for separate zone verification", "Grundlage des getrennten Zonennachweises", &[InputChoice::new("selected", "Non-residential building with selected tested zones", "Nichtwohngebäude mit Prüfung ausgewählter Zonen"), InputChoice::new("different", "Non-residential zones with different requirements", "Zonen eines Nichtwohngebäudes mit unterschiedlichen Anforderungen"), InputChoice::new("unconfirmed", "Zone eligibility unconfirmed", "Zulässigkeit der Zonenprüfung unbestätigt")]).example_text("selected").scope("G26-03"),
    InputSpec::choice("test_sample_basis", "Basis for sampling units", "Grundlage der Stichprobe von Nutzeinheiten", &[InputChoice::new("confirmed", "Identical units, exterior-only access and Annex NB sampling confirmed", "Gleichartige, nur von außen erschlossene Einheiten und Stichprobe nach Anhang NB bestätigt"), InputChoice::new("unconfirmed", "Unit sampling eligibility unconfirmed", "Zulässigkeit der Stichprobe unbestätigt")]).example_text("confirmed").scope("G26-04"),
];
