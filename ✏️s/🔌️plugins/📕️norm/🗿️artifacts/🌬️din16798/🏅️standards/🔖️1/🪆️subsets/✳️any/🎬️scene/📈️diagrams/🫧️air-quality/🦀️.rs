//! 🫧️ Air-quality diagrams: the CO₂ concentration limit per category (EN 16798-1 Table B.5 with the national absolute limits) and the relative-humidity band per category (Table B.16) against the room's design values.

use super::*;

/// 🧮️ The CO₂ concentration in ppm that `category` allows in a room: the outdoor concentration plus the permitted excess.
pub fn co2_limit_ppm(document: &Din16798Snapshot, zone: &ZoneDocument, category: ComfortCategory) -> f64 {
    let annex = checks::annex_params::AnnexParams::for_choice(document.annex);
    document.outdoor_co2_ppm + part_1::co2_limit_above_outdoor_ppm(&annex, checks::parse_usage(&zone.usage_type), category, document.outdoor_co2_ppm)
}

pub(super) fn co2(document: &Din16798Snapshot, report: &CheckReport, zone: &ZoneDocument) -> Diagram2d {
    let check = format!("din16798-1.co2.{}", zone.id);
    let own = category_number(category_of(zone));
    let limits: Vec<f64> = CATEGORIES.iter().map(|c| co2_limit_ppm(document, zone, *c)).collect();
    let mut series: Vec<Series> = CATEGORIES.iter().zip(&limits).map(|(c, limit)| {
        let name = category_name(*c);
        Series::Polygon { id: format!("limit-{name}"), label: loc(&format!("Limit of category {name}: {limit:.0} ppm"), &format!("Grenzwert der Kategorie {name}: {limit:.0} ppm")), ring: bar(category_number(*c), document.outdoor_co2_ppm, *limit), fill: fill(if category_number(*c) == own { check_tone(report, &check) } else { Tone::Neutral }, 0.35), hatch: None }
    }).collect();
    series.push(Series::Limit { id: "outdoor".into(), label: loc("Outdoor air", "Außenluft"), axis: AxisRef::Y, value: document.outdoor_co2_ppm, style: dashed(Tone::Info) });
    Diagram2d {
        id: format!("co2:{}", zone.id),
        title: loc(&format!("CO₂ concentration of {}", zone.id), &format!("CO₂-Konzentration von {}", zone.id)),
        description: loc(
            "Permitted CO₂ concentration per comfort category above the outdoor concentration against the design concentration of the room; dragging the marker changes the design concentration.",
            "Zulässige CO₂-Konzentration je Komfortkategorie über der Außenluftkonzentration gegenüber der Auslegungskonzentration des Raums; das Ziehen der Marke ändert die Auslegungskonzentration.",
        ),
        axes: vec![category_axis(), axis("CO₂ concentration", "CO₂-Konzentration", QuantityKind::Dimensionless, "ppm", None)],
        series,
        markers: vec![marker("co2", p(own, zone.co2_ppm), MarkerShape::Diamond, report, &check, loc(&format!("CO₂ concentration {:.0} ppm of {}", zone.co2_ppm, zone.id), &format!("CO₂-Konzentration {:.0} ppm von {}", zone.co2_ppm, zone.id)), Some(bound(format!("zones[id={}].co2Ppm", zone.id), "ppm", 10.0)))],
        annotations: Vec::new(),
        focus: Some(zone_focus(zone)),
        legend: true,
        table_twin: table(vec![
            col("Comfort category", "Komfortkategorie", "1", CATEGORIES.iter().map(|c| category_number(*c)).collect()),
            col("CO₂ limit", "CO₂-Grenzwert", "ppm", limits),
        ]),
    }
}

pub(super) fn humidity(report: &CheckReport, zone: &ZoneDocument) -> Diagram2d {
    let check = format!("din16798-1.rh.{}", zone.id);
    let own = category_number(category_of(zone));
    let bands: Vec<(f64, f64)> = CATEGORIES.iter().map(|c| part_1::humidity_band(*c)).collect();
    Diagram2d {
        id: format!("humidity:{}", zone.id),
        title: loc(&format!("Relative humidity of {}", zone.id), &format!("Relative Feuchte von {}", zone.id)),
        description: loc(
            "Permitted relative humidity per comfort category against the design humidity of the room; dragging the marker changes the design humidity.",
            "Zulässige relative Feuchte je Komfortkategorie gegenüber der Auslegungsfeuchte des Raums; das Ziehen der Marke ändert die Auslegungsfeuchte.",
        ),
        axes: vec![category_axis(), axis("Relative humidity", "Relative Feuchte", QuantityKind::Dimensionless, "%", Some((0.0, 100.0)))],
        series: vec![Series::Band { id: "humidity-band".into(), label: loc("Permitted humidity", "Zulässige Feuchte"), lower: CATEGORIES.iter().zip(&bands).map(|(c, b)| p(category_number(*c), b.0)).collect(), upper: CATEGORIES.iter().zip(&bands).map(|(c, b)| p(category_number(*c), b.1)).collect(), fill: fill(Tone::Info, 0.3) }],
        markers: vec![marker("humidity", p(own, zone.rh_percent), MarkerShape::Circle, report, &check, loc(&format!("Relative humidity {:.0} % of {}", zone.rh_percent, zone.id), &format!("Relative Feuchte {:.0} % von {}", zone.rh_percent, zone.id)), Some(bound(format!("zones[id={}].rhPercent", zone.id), "%", 1.0)))],
        annotations: Vec::new(),
        focus: Some(zone_focus(zone)),
        legend: true,
        table_twin: table(vec![
            col("Comfort category", "Komfortkategorie", "1", CATEGORIES.iter().map(|c| category_number(*c)).collect()),
            col("Minimum humidity", "Mindestfeuchte", "%", bands.iter().map(|b| b.0).collect()),
            col("Maximum humidity", "Höchstfeuchte", "%", bands.iter().map(|b| b.1).collect()),
        ]),
    }
}
