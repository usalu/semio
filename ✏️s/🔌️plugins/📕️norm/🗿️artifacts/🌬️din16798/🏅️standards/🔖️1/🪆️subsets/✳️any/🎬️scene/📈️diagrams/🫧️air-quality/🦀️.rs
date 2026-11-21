//! 🫧️ Air-quality diagrams: the CO₂ concentration limit per category (EN 16798-1 Tables B.9 and NA.9) and the relative-humidity design references (Tables B.16 and NA.10) against the room's design values.

use super::*;

/// 🧮️ The CO₂ concentration in ppm that `category` allows in a room: the outdoor concentration plus the permitted excess.
pub fn co2_limit_ppm(document: &Din16798Snapshot, category: ComfortCategory) -> f64 {
    document.outdoor_co2_ppm + part_1::co2_above_outdoor_ppm(document.annex, category)
}

pub(super) fn co2(document: &Din16798Snapshot, report: &CheckReport, zone: &ZoneDocument) -> Diagram2d {
    let check = format!("din16798-1.co2.{}", zone.id);
    let own = category_number(category_of(zone));
    let categories: &[ComfortCategory] = if document.annex == AnnexChoice::De
        && checks::parse_usage(&zone.usage_type) == OccupancyType::Residential
    {
        &[]
    } else {
        &CATEGORIES
    };
    let limits: Vec<f64> = categories.iter().map(|c| co2_limit_ppm(document, *c)).collect();
    let mut series: Vec<Series> = categories.iter().zip(&limits).map(|(c, limit)| {
        let name = category_name(*c);
        Series::Polygon { id: format!("limit-{name}"), label: loc(&format!("Limit of category {name}: {limit:.0} ppm"), &format!("Grenzwert der Kategorie {name}: {limit:.0} ppm")), ring: bar(category_number(*c), document.outdoor_co2_ppm, *limit), fill: fill(if category_number(*c) == own { check_tone(report, &check) } else { Tone::Neutral }, 0.35), hatch: None }
    }).collect();
    series.push(Series::Limit { id: "outdoor".into(), label: loc("Outdoor air", "Außenluft"), axis: AxisRef::Y, value: document.outdoor_co2_ppm, style: dashed(Tone::Info) });
    Diagram2d {
        id: format!("co2:{}", zone.id),
        title: loc(&format!("CO₂ concentration of {}", zone.id), &format!("CO₂-Konzentration von {}", zone.id)),
        description: if categories.is_empty() {
            loc("Consult the report for the dwelling design basis.", "Die Auslegungsgrundlage der Wohneinheit steht im Bericht.")
        } else {
            loc(
                "Compare the room's design CO₂ concentration with the published design values by category. Drag the marker to update the design.",
                "Vergleich der Auslegungs-CO₂-Konzentration mit den veröffentlichten Auslegungswerten je Kategorie. Durch Ziehen der Marke lässt sich die Auslegung ändern.",
            )
        },
        axes: vec![category_axis(), axis("CO₂ concentration", "CO₂-Konzentration", QuantityKind::Dimensionless, "ppm", None)],
        series,
        markers: vec![marker("co2", p(own, zone.co2_ppm), MarkerShape::Diamond, report, &check, loc(&format!("CO₂ concentration {:.0} ppm of {}", zone.co2_ppm, zone.id), &format!("CO₂-Konzentration {:.0} ppm von {}", zone.co2_ppm, zone.id)), Some(bound(format!("zones[id={}].co2Ppm", zone.id), "ppm", 10.0)))],
        annotations: Vec::new(),
        focus: Some(zone_focus(zone)),
        legend: true,
        table_twin: if categories.is_empty() {
            table(vec![col("Design CO₂", "Auslegungs-CO₂", "ppm", vec![zone.co2_ppm])])
        } else {
            table(vec![
                col("Comfort category", "Komfortkategorie", "1", categories.iter().map(|c| category_number(*c)).collect()),
                col("CO₂ limit", "CO₂-Grenzwert", "ppm", limits),
            ])
        },
    }
}

pub(super) fn humidity(document: &Din16798Snapshot, report: &CheckReport, zone: &ZoneDocument) -> Diagram2d {
    let check = format!("din16798-1.rh.{}", zone.id);
    let own = category_number(category_of(zone));
    let bands: Vec<_> = CATEGORIES.into_iter().filter_map(|c| {
        part_1::humidity_band(document.annex, c).map(|(low, high)| (category_number(c), low, high))
    }).collect();
    Diagram2d {
        id: format!("humidity:{}", zone.id),
        title: loc(&format!("Relative humidity of {}", zone.id), &format!("Relative Feuchte von {}", zone.id)),
        description: loc(
            "Compare the room design humidity with the reference setpoints for installed humidification and dehumidification.",
            "Vergleich der Auslegungsfeuchte mit den Richtwerten für installierte Be- und Entfeuchtung.",
        ),
        axes: vec![category_axis(), axis("Relative humidity", "Relative Feuchte", QuantityKind::Dimensionless, "%", Some((0.0, 100.0)))],
        series: vec![Series::Band { id: "humidity-band".into(), label: loc("Humidity control references", "Richtwerte der Feuchteregelung"), lower: bands.iter().map(|b| p(b.0, b.1)).collect(), upper: bands.iter().map(|b| p(b.0, b.2)).collect(), fill: fill(Tone::Info, 0.3) }],
        markers: vec![marker("humidity", p(own, zone.rh_percent), MarkerShape::Circle, report, &check, loc(&format!("Relative humidity {:.0} % of {}", zone.rh_percent, zone.id), &format!("Relative Feuchte {:.0} % von {}", zone.rh_percent, zone.id)), Some(bound(format!("zones[id={}].rhPercent", zone.id), "%", 1.0)))],
        annotations: Vec::new(),
        focus: Some(zone_focus(zone)),
        legend: true,
        table_twin: table(vec![
            col("Comfort category", "Komfortkategorie", "1", bands.iter().map(|b| b.0).collect()),
            col("Humidification", "Befeuchtung", "%", bands.iter().map(|b| b.1).collect()),
            col("Dehumidification", "Entfeuchtung", "%", bands.iter().map(|b| b.2).collect()),
        ]),
    }
}
