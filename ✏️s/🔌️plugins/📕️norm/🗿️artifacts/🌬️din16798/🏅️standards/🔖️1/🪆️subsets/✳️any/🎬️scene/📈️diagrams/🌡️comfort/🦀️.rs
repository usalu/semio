//! 🌡️ Comfort diagrams: adaptive comfort bands of Figure B.1 and seasonal design limits of Tables B.2 and NA.3.

use super::*;

const STEPS: [f64; 5] = [10.1, 15.0, 20.0, 25.0, 29.9];

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
    let mut series: Vec<Series> = [ComfortCategory::III, ComfortCategory::II, ComfortCategory::I]
        .into_iter()
        .map(|category| {
            let (tone, opacity) = band_tone(category);
            let name = category_name(category);
            Series::Band {
                id: format!("category-{name}"),
                label: loc(&format!("Category {name}"), &format!("Kategorie {name}")),
                lower: STEPS.iter().filter_map(|x| part_1::adaptive_operative_bounds(*x, category).map(|bounds| p(*x, bounds.0))).collect(),
                upper: STEPS.iter().filter_map(|x| part_1::adaptive_operative_bounds(*x, category).map(|bounds| p(*x, bounds.1))).collect(),
                fill: fill(tone, opacity),
            }
        })
        .collect();
    series.push(Series::Polyline { id: "comfort-temperature".into(), label: loc("Comfort temperature θ_c", "Komforttemperatur θ_c"), points: STEPS.iter().map(|x| p(*x, part_1::adaptive_comfort_temperature_c(*x))).collect(), style: solid(Tone::Neutral) });
    let centre = part_1::adaptive_comfort_temperature_c(theta);
    let markers = vec![
        marker("operative", p(theta, zone.t_op_summer_c), MarkerShape::Circle, report, &check, loc(&format!("Summer operative temperature {:.1} °C of {}", zone.t_op_summer_c, zone.id), &format!("Operative Sommertemperatur {:.1} °C von {}", zone.t_op_summer_c, zone.id)), None),
        marker("running-mean", p(theta, centre), MarkerShape::Diamond, report, &check, loc(&format!("Running-mean outdoor temperature {theta:.1} °C"), &format!("Gleitender Außenlufttemperatur-Mittelwert {theta:.1} °C")), Some(bound("thetaRmC".to_owned(), "°C", 0.1))),
    ];
    let mut columns = vec![
        col("Running-mean outdoor temperature", "Gleitender Außenlufttemperatur-Mittelwert", "°C", STEPS.to_vec()),
        col("Comfort temperature", "Komforttemperatur", "°C", STEPS.iter().map(|x| part_1::adaptive_comfort_temperature_c(*x)).collect()),
    ];
    if own != ComfortCategory::IV {
        let limits: Vec<_> = STEPS.iter().filter_map(|x| part_1::adaptive_operative_bounds(*x, own)).collect();
        columns.push(col("Lower limit of the category", "Untergrenze der Kategorie", "°C", limits.iter().map(|b| b.0).collect()));
        columns.push(col("Upper limit of the category", "Obergrenze der Kategorie", "°C", limits.iter().map(|b| b.1).collect()));
    }
    Diagram2d {
        id: format!("adaptive:{}", zone.id),
        title: loc(&format!("Adaptive comfort of {}", zone.id), &format!("Adaptiver Komfort von {}", zone.id)),
        description: loc(
            "EN Figure B.1 reference bands for categories I to III. They apply to free-running summer and shoulder-season conditions with operable windows and freely adjustable clothing. The German Annex uses hourly data.",
            "Referenzbereiche nach Bild B.1 für Kategorien I bis III. Sie gelten im Sommer und in der Übergangszeit für frei betriebene Räume mit öffenbaren Fenstern und frei anpassbarer Bekleidung. Der deutsche Anhang verwendet Stundenwerte.",
        ),
        axes: vec![axis("Running-mean outdoor temperature", "Gleitender Außenlufttemperatur-Mittelwert", QuantityKind::Temperature, "°C", Some(part_1::ADAPTIVE_RUNNING_MEAN_RANGE_C)), axis("Operative temperature", "Operative Temperatur", QuantityKind::Temperature, "°C", None)],
        series,
        markers,
        annotations: Vec::new(),
        focus: Some(zone_focus(zone)),
        legend: true,
        table_twin: table(columns),
    }
}

pub(super) fn operative(document: &Din16798Snapshot, report: &CheckReport, zone: &ZoneDocument) -> Diagram2d {
    let own = category_number(category_of(zone));
    let usage = checks::parse_usage(&zone.usage_type);
    let limits: Vec<_> = CATEGORIES.into_iter().filter_map(|category| {
        part_1::seasonal_design_temperatures(document.annex, usage, category)
            .map(|(heat, cool)| (category_number(category), heat, cool))
    }).collect();
    let adaptive = checks::ComfortModel::parse(&zone.comfort_model) == checks::ComfortModel::Adaptive;
    let mut series = Vec::new();
    let mut columns = vec![col("Comfort category", "Komfortkategorie", "1", limits.iter().map(|row| row.0).collect())];
    if !limits.is_empty() {
        let heating = loc("Winter heating minimum", "Heizminimum Winter");
        series.push(Series::Polyline { id: "winter".into(), label: heating, points: limits.iter().map(|row| p(row.0, row.1)).collect(), style: solid(Tone::Info) });
        columns.push(col("Winter heating minimum", "Heizminimum Winter", "°C", limits.iter().map(|row| row.1).collect()));
        if !adaptive {
            let cooling = loc("Summer cooling maximum", "Kühlmaximum Sommer");
            series.push(Series::Polyline { id: "summer".into(), label: cooling, points: limits.iter().map(|row| p(row.0, row.2)).collect(), style: solid(Tone::Warning) });
            columns.push(col("Summer cooling maximum", "Kühlmaximum Sommer", "°C", limits.iter().map(|row| row.2).collect()));
        }
    } else {
        columns = vec![
            col("Winter design temperature", "Auslegungstemperatur Winter", "°C", vec![zone.t_op_winter_c]),
            col("Summer design temperature", "Auslegungstemperatur Sommer", "°C", vec![zone.t_op_summer_c]),
        ];
    }
    let summer_check = format!("din16798-1.top.summer.{}", zone.id);
    let winter_check = format!("din16798-1.top.winter.{}", zone.id);
    Diagram2d {
        id: format!("operative:{}", zone.id),
        title: loc(&format!("Seasonal design temperatures of {}", zone.id), &format!("Saisonale Auslegungstemperaturen von {}", zone.id)),
        description: loc(
            "Compare the room design temperatures with the reference heating minimum and cooling maximum. Drag a marker to change the design temperature.",
            "Vergleich der Auslegungstemperaturen mit den Richtwerten für Heizminimum und Kühlmaximum. Durch Ziehen einer Marke lässt sich die Auslegungstemperatur ändern.",
        ),
        axes: vec![category_axis(), axis("Operative temperature", "Operative Temperatur", QuantityKind::Temperature, "°C", None)],
        series,
        markers: vec![
            marker("summer", p(own, zone.t_op_summer_c), MarkerShape::Circle, report, &summer_check, loc(&format!("Summer operative temperature {:.1} °C of {}", zone.t_op_summer_c, zone.id), &format!("Operative Sommertemperatur {:.1} °C von {}", zone.t_op_summer_c, zone.id)), Some(bound(format!("zones[id={}].tOpSummerC", zone.id), "°C", 0.1))),
            marker("winter", p(own, zone.t_op_winter_c), MarkerShape::Square, report, &winter_check, loc(&format!("Winter operative temperature {:.1} °C of {}", zone.t_op_winter_c, zone.id), &format!("Operative Wintertemperatur {:.1} °C von {}", zone.t_op_winter_c, zone.id)), Some(bound(format!("zones[id={}].tOpWinterC", zone.id), "°C", 0.1))),
        ],
        annotations: Vec::new(),
        focus: Some(zone_focus(zone)),
        legend: true,
        table_twin: table(columns),
    }
}
