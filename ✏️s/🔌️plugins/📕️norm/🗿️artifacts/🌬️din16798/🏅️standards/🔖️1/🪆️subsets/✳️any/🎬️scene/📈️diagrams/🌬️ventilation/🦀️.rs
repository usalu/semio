//! 🌬️ Ventilation diagrams: the outdoor-air rate of a room per category as persons and building components (EN 16798-1 Tables B.6 and B.7) against the supplied rate, and the capacity of the ventilation systems of the building (EN 16798-3).

use super::*;

/// 🧮️ The outdoor-air flow of a room in m³/h that the persons and that the building components require in `category` (method 1: `n · q_p` and `A · q_a`, litres per second times 3.6).
pub fn ventilation_components(zone: &ZoneDocument, category: ComfortCategory) -> (f64, f64) {
    let persons = f64::from(zone.occupants) * part_1::outdoor_air_per_person_l_s(category) * 3.6;
    let building = zone.floor_area_m2 * part_1::outdoor_air_per_area_l_s_m2(category, checks::parse_pollution(&zone.pollution_class)) * 3.6;
    (persons, building)
}

pub(super) fn rates(_document: &Din16798Snapshot, report: &CheckReport, zone: &ZoneDocument) -> Diagram2d {
    let check = format!("din16798-1.vent.{}", zone.id);
    let own = category_number(category_of(zone));
    let mut series: Vec<Series> = Vec::new();
    for category in CATEGORIES {
        let x = category_number(category);
        let (persons, building) = ventilation_components(zone, category);
        let name = category_name(category);
        series.push(Series::Polygon { id: format!("persons-{name}"), label: loc(&format!("Persons, category {name}: {persons:.0} m³/h"), &format!("Personen, Kategorie {name}: {persons:.0} m³/h")), ring: bar(x, 0.0, persons), fill: fill(Tone::Info, 0.6), hatch: None });
        series.push(Series::Polygon { id: format!("building-{name}"), label: loc(&format!("Building, category {name}: {building:.0} m³/h"), &format!("Gebäude, Kategorie {name}: {building:.0} m³/h")), ring: bar(x, persons, persons + building), fill: fill(Tone::Neutral, 0.6), hatch: None });
    }
    series.push(Series::Limit { id: "supplied".into(), label: loc("Supplied outdoor air", "Zugeführte Außenluft"), axis: AxisRef::Y, value: zone.outdoor_air_supplied_m3_h, style: dashed(check_tone(report, &check)) });
    let components: Vec<(f64, f64)> = CATEGORIES.iter().map(|c| ventilation_components(zone, *c)).collect();
    Diagram2d {
        id: format!("ventilation:{}", zone.id),
        title: loc(&format!("Ventilation rate of {}", zone.id), &format!("Lüftungsrate von {}", zone.id)),
        description: loc(
            "Required outdoor air per comfort category, stacked from the persons and the building components, against the outdoor air supplied to the room; dragging the marker changes the supplied outdoor air.",
            "Erforderliche Außenluft je Komfortkategorie, gestapelt aus dem Personen- und dem Gebäudeanteil, gegenüber der dem Raum zugeführten Außenluft; das Ziehen der Marke ändert die zugeführte Außenluft.",
        ),
        axes: vec![category_axis(), axis("Outdoor air flow", "Außenluftvolumenstrom", QuantityKind::VentilationRate, "m³/h", None)],
        series,
        markers: vec![marker("supplied", p(own, zone.outdoor_air_supplied_m3_h), MarkerShape::Diamond, report, &check, loc(&format!("Supplied outdoor air {:.0} m³/h of {}", zone.outdoor_air_supplied_m3_h, zone.id), &format!("Zugeführte Außenluft {:.0} m³/h von {}", zone.outdoor_air_supplied_m3_h, zone.id)), Some(bound(format!("zones[id={}].outdoorAirSuppliedM3H", zone.id), "m³/h", 1.0)))],
        annotations: Vec::new(),
        focus: Some(zone_focus(zone)),
        legend: true,
        table_twin: table(vec![
            col("Comfort category", "Komfortkategorie", "1", CATEGORIES.iter().map(|c| category_number(*c)).collect()),
            col("Persons component", "Personenanteil", "m³/h", components.iter().map(|c| c.0).collect()),
            col("Building component", "Gebäudeanteil", "m³/h", components.iter().map(|c| c.1).collect()),
            col("Total", "Summe", "m³/h", components.iter().map(|c| c.0 + c.1).collect()),
        ]),
    }
}

pub(super) fn capacity(document: &Din16798Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    if document.vent_systems.is_empty() {
        return None;
    }
    let served: Vec<f64> = document.vent_systems.iter().map(|v| document.zones.iter().filter(|z| z.vent_system_id == v.id).map(|z| z.outdoor_air_supplied_m3_h).sum()).collect();
    let mut series = Vec::new();
    let mut markers = Vec::new();
    for (index, vent) in document.vent_systems.iter().enumerate() {
        let check = format!("din16798-3.capacity.{}", vent.id);
        let x = (index + 1) as f64;
        series.push(Series::Polygon { id: format!("design-{}", vent.id), label: loc(&format!("Design capacity of {}: {:.0} m³/h", vent.id, vent.design_airflow_m3_h), &format!("Auslegungskapazität von {}: {:.0} m³/h", vent.id, vent.design_airflow_m3_h)), ring: bar_of(x - 0.2, 0.18, 0.0, vent.design_airflow_m3_h), fill: fill(check_tone(report, &check), 0.45), hatch: None });
        series.push(Series::Polygon { id: format!("served-{}", vent.id), label: loc(&format!("Served by {}: {:.0} m³/h", vent.id, served[index]), &format!("Versorgt von {}: {:.0} m³/h", vent.id, served[index])), ring: bar_of(x + 0.2, 0.18, 0.0, served[index]), fill: fill(Tone::Neutral, 0.6), hatch: None });
        markers.push(marker(&format!("design:{}", vent.id), p(x - 0.2, vent.design_airflow_m3_h), MarkerShape::Diamond, report, &check, loc(&format!("Design airflow {:.0} m³/h of {}", vent.design_airflow_m3_h, vent.id), &format!("Auslegungsvolumenstrom {:.0} m³/h von {}", vent.design_airflow_m3_h, vent.id)), Some(bound(format!("ventSystems[id={}].designAirflowM3H", vent.id), "m³/h", 10.0))));
    }
    Some(Diagram2d {
        id: "capacity:building".into(),
        title: loc("Capacity of the ventilation systems", "Kapazität der Lüftungsanlagen"),
        description: loc(
            "Design airflow of every ventilation system next to the outdoor air of the rooms it serves; dragging a marker changes the design airflow.",
            "Auslegungsvolumenstrom jeder Lüftungsanlage neben der Außenluft der von ihr versorgten Räume; das Ziehen einer Marke ändert den Auslegungsvolumenstrom.",
        ),
        axes: vec![axis("Ventilation system", "Lüftungsanlage", QuantityKind::Dimensionless, "1", None), axis("Outdoor air flow", "Außenluftvolumenstrom", QuantityKind::VentilationRate, "m³/h", None)],
        series,
        markers,
        annotations: Vec::new(),
        focus: None,
        legend: false,
        table_twin: table(vec![
            col("System index", "Anlagennummer", "1", (1..=document.vent_systems.len()).map(|i| i as f64).collect()),
            col("Design airflow", "Auslegungsvolumenstrom", "m³/h", document.vent_systems.iter().map(|v| v.design_airflow_m3_h).collect()),
            col("Served outdoor air", "Versorgte Außenluft", "m³/h", served),
        ]),
    })
}
