//! 🌬️ Ventilation diagrams: the outdoor-air rate of a room per category by the selected design method against the supplied rate, and the capacity of the ventilation systems of the building (EN 16798-3).

use super::*;

/// 🧮️ The outdoor-air flow of a room in m³/h that the persons and that the building components require in `category` (method 1: `n · q_p` and `A · q_a`, litres per second times 3.6).
pub fn ventilation_components(zone: &ZoneDocument, category: ComfortCategory) -> (f64, f64) {
    let persons = f64::from(zone.occupants) * part_1::outdoor_air_per_person_l_s(category) * 3.6;
    let building = zone.floor_area_m2 * part_1::outdoor_air_per_area_l_s_m2(category, checks::parse_pollution(&zone.pollution_class)) * 3.6;
    (persons, building)
}

pub(super) fn rates(document: &Din16798Snapshot, report: &CheckReport, zone: &ZoneDocument) -> Diagram2d {
    let check = format!("din16798-1.vent.{}", zone.id);
    let own = category_number(category_of(zone));
    let usage = checks::parse_usage(&zone.usage_type);
    let pollution = checks::parse_pollution(&zone.pollution_class);
    let method = checks::VentMethod::parse(&zone.vent_method);
    let hygienic_minimum = part_1::hygienic_outdoor_air_minimum_m3_h(zone.occupants, usage);
    let mut series = Vec::new();
    let mut categories = Vec::new();
    let mut totals = Vec::new();
    let mut occupant_flows = Vec::new();
    let mut building_flows = Vec::new();
    let show_components = method == checks::VentMethod::Method1PerceivedAirQuality && usage != OccupancyType::Residential;
    for category in CATEGORIES {
        let delta = part_1::co2_above_outdoor_ppm(document.annex, category);
        let Ok(required) = part_1::required_outdoor_air_for_method(
            document.annex, method, zone.occupants, zone.floor_area_m2, category, usage, pollution, delta,
        ) else { continue };
        let x = category_number(category);
        let name = category_name(category);
        let total = required.max(hygienic_minimum);
        categories.push(x);
        totals.push(total);
        if show_components {
            let (persons, building) = ventilation_components(zone, category);
            occupant_flows.push(persons);
            building_flows.push(building);
            series.push(Series::Polygon { id: format!("persons-{name}"), label: loc(&format!("Persons, category {name}: {persons:.0} m³/h"), &format!("Personen, Kategorie {name}: {persons:.0} m³/h")), ring: bar(x, 0.0, persons), fill: fill(Tone::Info, 0.6), hatch: None });
            series.push(Series::Polygon { id: format!("building-{name}"), label: loc(&format!("Building, category {name}: {building:.0} m³/h"), &format!("Gebäude, Kategorie {name}: {building:.0} m³/h")), ring: bar(x, persons, persons + building), fill: fill(Tone::Neutral, 0.6), hatch: None });
        } else {
            series.push(Series::Polygon { id: format!("required-{name}"), label: loc(&format!("Required, category {name}: {total:.0} m³/h"), &format!("Erforderlich, Kategorie {name}: {total:.0} m³/h")), ring: bar(x, 0.0, total), fill: fill(Tone::Info, 0.6), hatch: None });
        }
    }
    if hygienic_minimum > 0.0 {
        series.push(Series::Limit {
            id: "hygienic-minimum".into(),
            label: loc("Hygienic minimum", "Hygienischer Mindestwert"),
            axis: AxisRef::Y,
            value: hygienic_minimum,
            style: solid(Tone::Info),
        });
    }
    series.push(Series::Limit { id: "supplied".into(), label: loc("Supplied outdoor air", "Zugeführte Außenluft"), axis: AxisRef::Y, value: zone.outdoor_air_supplied_m3_h, style: dashed(check_tone(report, &check)) });
    let description = if categories.is_empty() {
        loc("See the report for the design basis required for this zone.", "Die erforderliche Auslegungsgrundlage für diese Zone steht im Bericht.")
    } else {
        loc(
            "Compare the supplied outdoor airflow with the selected design method by comfort category. Drag the supplied-air marker to update the design.",
            "Vergleich des zugeführten Außenluftvolumenstroms mit dem gewählten Auslegungsverfahren nach Komfortkategorie. Durch Ziehen der Zuluftmarke lässt sich die Auslegung ändern.",
        )
    };
    let mut columns = if categories.is_empty() {
        vec![col("Supplied outdoor air", "Zugeführte Außenluft", "m³/h", vec![zone.outdoor_air_supplied_m3_h])]
    } else {
        vec![
            col("Comfort category", "Komfortkategorie", "1", categories),
            col("Required outdoor air", "Erforderliche Außenluft", "m³/h", totals),
        ]
    };
    if !occupant_flows.is_empty() {
        columns.push(col("Persons component", "Personenanteil", "m³/h", occupant_flows));
        columns.push(col("Building component", "Gebäudeanteil", "m³/h", building_flows));
    }
    Diagram2d {
        id: format!("ventilation:{}", zone.id),
        title: loc(&format!("Ventilation rate of {}", zone.id), &format!("Lüftungsrate von {}", zone.id)),
        description,
        axes: vec![category_axis(), axis("Outdoor air flow", "Außenluftvolumenstrom", QuantityKind::VentilationRate, "m³/h", None)],
        series,
        markers: vec![marker("supplied", p(own, zone.outdoor_air_supplied_m3_h), MarkerShape::Diamond, report, &check, loc(&format!("Supplied outdoor air {:.0} m³/h of {}", zone.outdoor_air_supplied_m3_h, zone.id), &format!("Zugeführte Außenluft {:.0} m³/h von {}", zone.outdoor_air_supplied_m3_h, zone.id)), Some(bound(format!("zones[id={}].outdoorAirSuppliedM3H", zone.id), "m³/h", 1.0)))],
        annotations: Vec::new(),
        focus: Some(zone_focus(zone)),
        legend: true,
        table_twin: table(columns),
    }
}

pub(super) fn capacity(document: &Din16798Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    if document.vent_systems.is_empty() {
        return None;
    }
    let required: Vec<f64> = document.vent_systems.iter().map(|v| {
        find(report, &format!("din16798-3.capacity.{}", v.id)).map_or(0.0, |check| check.computed.value)
    }).collect();
    let mut series = Vec::new();
    let mut markers = Vec::new();
    for (index, vent) in document.vent_systems.iter().enumerate() {
        let check = format!("din16798-3.capacity.{}", vent.id);
        let x = (index + 1) as f64;
        series.push(Series::Polygon { id: format!("design-{}", vent.id), label: loc(&format!("Design capacity of {}: {:.0} m³/h", vent.id, vent.design_airflow_m3_h), &format!("Auslegungskapazität von {}: {:.0} m³/h", vent.id, vent.design_airflow_m3_h)), ring: bar_of(x - 0.2, 0.18, 0.0, vent.design_airflow_m3_h), fill: fill(check_tone(report, &check), 0.45), hatch: None });
        series.push(Series::Polygon { id: format!("served-{}", vent.id), label: loc(&format!("Capacity needed at {}: {:.0} m³/h", vent.id, required[index]), &format!("Erforderliche Kapazität bei {}: {:.0} m³/h", vent.id, required[index])), ring: bar_of(x + 0.2, 0.18, 0.0, required[index]), fill: fill(Tone::Neutral, 0.6), hatch: None });
        markers.push(marker(&format!("design:{}", vent.id), p(x - 0.2, vent.design_airflow_m3_h), MarkerShape::Diamond, report, &check, loc(&format!("Design airflow {:.0} m³/h of {}", vent.design_airflow_m3_h, vent.id), &format!("Auslegungsvolumenstrom {:.0} m³/h von {}", vent.design_airflow_m3_h, vent.id)), Some(bound(format!("ventSystems[id={}].designAirflowM3H", vent.id), "m³/h", 10.0))));
    }
    Some(Diagram2d {
        id: "capacity:building".into(),
        title: loc("Capacity of the ventilation systems", "Kapazität der Lüftungsanlagen"),
        description: loc(
            "Compare each system's design airflow and required capacity, which accounts for the hygienic minimum in each served room. Drag a marker to change the design airflow.",
            "Vergleich von Auslegungsvolumenstrom und erforderlicher Kapazität je Anlage unter Berücksichtigung des hygienischen Mindestwerts jedes versorgten Raums. Durch Ziehen einer Marke lässt sich der Auslegungsvolumenstrom ändern.",
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
            col("Required capacity", "Erforderliche Kapazität", "m³/h", required),
        ]),
    })
}
