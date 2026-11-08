//! 🌡️ Comfort diagrams: the adaptive comfort bands of EN 16798-1 Annex B.2 and the operative temperature bands per category of Table B.2.

use super::*;

/// 📏️ The validity range of the running-mean outdoor temperature of the adaptive model in °C (EN 16798-1 Annex B.2).
pub const RUNNING_MEAN_RANGE_C: (f64, f64) = (10.0, 30.0);

const STEPS: [f64; 5] = [10.0, 15.0, 20.0, 25.0, 30.0];

/// 🧮️ The comfort temperature `θ_c = 0.33 θ_rm + 18.8` and the lower and upper operative limit of `category` at the running-mean outdoor temperature `theta_rm_c`.
pub fn adaptive_limits(theta_rm_c: f64, category: ComfortCategory) -> (f64, f64, f64) {
    let centre = part_1::adaptive_comfort_temperature_c(theta_rm_c);
    let half = part_1::adaptive_band_k(category);
    (centre - half, centre, centre + half)
}

/// 🧮️ The lower and upper operative limit of `category` in the season, EN 16798-1 Table B.2.
pub fn operative_bands(category: ComfortCategory, summer: bool) -> (f64, f64) {
    part_1::operative_temp_band(category, if summer { "summer" } else { "winter" })
}

fn band_tone(category: ComfortCategory) -> (Tone, f64) {
    match category {
        ComfortCategory::I => (Tone::Info, 0.45),
        ComfortCategory::II => (Tone::Info, 0.28),
        ComfortCategory::III => (Tone::Warning, 0.2),
        ComfortCategory::IV => (Tone::Error, 0.12),
    }
}

pub(super) fn adaptive(document: &Din16798Snapshot, report: &CheckReport, zone: &ZoneDocument) -> Diagram2d {
    let check = format!("din16798-1.adaptive.{}", zone.id);
    let theta = document.theta_rm_c;
    let own = category_of(zone);
    let mut series: Vec<Series> = [ComfortCategory::IV, ComfortCategory::III, ComfortCategory::II, ComfortCategory::I]
        .into_iter()
        .map(|category| {
            let (tone, opacity) = band_tone(category);
            let name = category_name(category);
            Series::Band {
                id: format!("category-{name}"),
                label: loc(&format!("Category {name}"), &format!("Kategorie {name}")),
                lower: STEPS.iter().map(|x| p(*x, adaptive_limits(*x, category).0)).collect(),
                upper: STEPS.iter().map(|x| p(*x, adaptive_limits(*x, category).2)).collect(),
                fill: fill(tone, opacity),
            }
        })
        .collect();
    series.push(Series::Polyline { id: "comfort-temperature".into(), label: loc("Comfort temperature θ_c", "Komforttemperatur θ_c"), points: STEPS.iter().map(|x| p(*x, adaptive_limits(*x, own).1)).collect(), style: solid(Tone::Neutral) });
    let centre = adaptive_limits(theta, own).1;
    let markers = vec![
        marker("operative", p(theta, zone.t_op_summer_c), MarkerShape::Circle, report, &check, loc(&format!("Summer operative temperature {:.1} °C of {}", zone.t_op_summer_c, zone.id), &format!("Operative Sommertemperatur {:.1} °C von {}", zone.t_op_summer_c, zone.id)), None),
        marker("running-mean", p(theta, centre), MarkerShape::Diamond, report, &check, loc(&format!("Running-mean outdoor temperature {theta:.1} °C"), &format!("Gleitender Außenlufttemperatur-Mittelwert {theta:.1} °C")), Some(bound("thetaRmC".to_owned(), "°C", 0.1))),
    ];
    Diagram2d {
        id: format!("adaptive:{}", zone.id),
        title: loc(&format!("Adaptive comfort of {}", zone.id), &format!("Adaptiver Komfort von {}", zone.id)),
        description: loc(
            "Operative temperature against the running-mean outdoor temperature with the category bands I to IV around the comfort temperature; dragging the diamond changes the running-mean outdoor temperature.",
            "Operative Temperatur über dem gleitenden Außenlufttemperatur-Mittelwert mit den Kategorie-Bändern I bis IV um die Komforttemperatur; das Ziehen der Raute ändert den gleitenden Mittelwert der Außenlufttemperatur.",
        ),
        axes: vec![axis("Running-mean outdoor temperature", "Gleitender Außenlufttemperatur-Mittelwert", QuantityKind::Temperature, "°C", Some(RUNNING_MEAN_RANGE_C)), axis("Operative temperature", "Operative Temperatur", QuantityKind::Temperature, "°C", None)],
        series,
        markers,
        annotations: Vec::new(),
        focus: Some(zone_focus(zone)),
        legend: true,
        table_twin: table(vec![
            col("Running-mean outdoor temperature", "Gleitender Außenlufttemperatur-Mittelwert", "°C", STEPS.to_vec()),
            col("Comfort temperature", "Komforttemperatur", "°C", STEPS.iter().map(|x| adaptive_limits(*x, own).1).collect()),
            col("Lower limit of the category", "Untergrenze der Kategorie", "°C", STEPS.iter().map(|x| adaptive_limits(*x, own).0).collect()),
            col("Upper limit of the category", "Obergrenze der Kategorie", "°C", STEPS.iter().map(|x| adaptive_limits(*x, own).2).collect()),
        ]),
    }
}

pub(super) fn operative(report: &CheckReport, zone: &ZoneDocument) -> Diagram2d {
    let own = category_number(category_of(zone));
    let steps: Vec<f64> = CATEGORIES.iter().map(|c| category_number(*c)).collect();
    let band = |id: &str, label: LocalizedCopy, summer: bool, tone: Tone| Series::Band {
        id: id.to_owned(),
        label,
        lower: CATEGORIES.iter().map(|c| p(category_number(*c), operative_bands(*c, summer).0)).collect(),
        upper: CATEGORIES.iter().map(|c| p(category_number(*c), operative_bands(*c, summer).1)).collect(),
        fill: fill(tone, 0.28),
    };
    let summer_check = format!("din16798-1.top.summer.{}", zone.id);
    let winter_check = format!("din16798-1.top.winter.{}", zone.id);
    Diagram2d {
        id: format!("operative:{}", zone.id),
        title: loc(&format!("Operative temperature bands of {}", zone.id), &format!("Operative Temperaturbänder von {}", zone.id)),
        description: loc(
            "Permitted operative temperature per comfort category in summer and winter (Table B.2) with the design temperatures of the room; dragging a marker changes the design temperature.",
            "Zulässige operative Temperatur je Komfortkategorie im Sommer und im Winter (Tabelle B.2) mit den Auslegungstemperaturen des Raums; das Ziehen einer Marke ändert die Auslegungstemperatur.",
        ),
        axes: vec![category_axis(), axis("Operative temperature", "Operative Temperatur", QuantityKind::Temperature, "°C", None)],
        series: vec![band("summer", loc("Summer band", "Sommerband"), true, Tone::Warning), band("winter", loc("Winter band", "Winterband"), false, Tone::Info)],
        markers: vec![
            marker("summer", p(own, zone.t_op_summer_c), MarkerShape::Circle, report, &summer_check, loc(&format!("Summer operative temperature {:.1} °C of {}", zone.t_op_summer_c, zone.id), &format!("Operative Sommertemperatur {:.1} °C von {}", zone.t_op_summer_c, zone.id)), Some(bound(format!("zones[id={}].tOpSummerC", zone.id), "°C", 0.1))),
            marker("winter", p(own, zone.t_op_winter_c), MarkerShape::Square, report, &winter_check, loc(&format!("Winter operative temperature {:.1} °C of {}", zone.t_op_winter_c, zone.id), &format!("Operative Wintertemperatur {:.1} °C von {}", zone.t_op_winter_c, zone.id)), Some(bound(format!("zones[id={}].tOpWinterC", zone.id), "°C", 0.1))),
        ],
        annotations: Vec::new(),
        focus: Some(zone_focus(zone)),
        legend: true,
        table_twin: table(vec![
            col("Comfort category", "Komfortkategorie", "1", steps),
            col("Summer minimum", "Sommerminimum", "°C", CATEGORIES.iter().map(|c| operative_bands(*c, true).0).collect()),
            col("Summer maximum", "Sommermaximum", "°C", CATEGORIES.iter().map(|c| operative_bands(*c, true).1).collect()),
            col("Winter minimum", "Winterminimum", "°C", CATEGORIES.iter().map(|c| operative_bands(*c, false).0).collect()),
            col("Winter maximum", "Wintermaximum", "°C", CATEGORIES.iter().map(|c| operative_bands(*c, false).1).collect()),
        ]),
    }
}
