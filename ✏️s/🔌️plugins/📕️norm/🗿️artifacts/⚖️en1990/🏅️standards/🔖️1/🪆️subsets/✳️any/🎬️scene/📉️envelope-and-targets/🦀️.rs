//! 📉️ Envelope and targets: the effect of every generated combination as a bar with the governing ultimate combination highlighted between the envelope limits, and the target reliability index β of the classes RC1 to RC3 over the reference period with the project in it. Both have a table twin. See [`super`].

use super::*;

const PERIODS: [f64; 7] = [1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0];

fn family_tone(family: combinations::Family) -> Tone {
    match family {
        combinations::Family::Str => Tone::Info,
        combinations::Family::Equ | combinations::Family::Acc | combinations::Family::Sei => Tone::Warning,
        combinations::Family::SlsChar | combinations::Family::SlsFreq | combinations::Family::SlsQp => Tone::Neutral,
    }
}

fn ultimate(family: combinations::Family) -> bool {
    matches!(family, combinations::Family::Str | combinations::Family::Equ | combinations::Family::Acc | combinations::Family::Sei)
}

/// 📉️ The structure-level design effect Σγψ·E of every combination as a bar; the ultimate combination with the largest effect is the governing one and drawn solid between the envelope limits (largest and smallest effect of all combinations), the active combination is framed and every bar carries a selecting marker bound to the worst check of its column.
pub(super) fn envelope(report: &CheckReport, columns: &[Column], active: Option<&Column>) -> Option<Diagram2d> {
    if columns.is_empty() {
        return None;
    }
    let totals: Vec<f64> = columns.iter().map(|c| c.combination.total()).collect();
    let (high, low) = (totals.iter().copied().fold(0.0, f64::max), totals.iter().copied().fold(0.0, f64::min));
    let span = (high - low).max(1.0);
    let governing = columns.iter().enumerate().filter(|(_, c)| ultimate(c.family)).max_by(|a, b| totals[a.0].total_cmp(&totals[b.0])).map(|(i, _)| i);
    let mut series = Vec::new();
    let mut markers = Vec::new();
    for (i, column) in columns.iter().enumerate() {
        let x = i as f64;
        let is_governing = governing == Some(i);
        series.push(Series::Polygon {
            id: format!("bar-{i}"),
            label: loc(&format!("{} · {} · lead {}: {:.1} kN", column.family.title().0, column.combination.equation.label(), column.leading(), totals[i] / 1e3), &format!("{} · {} · führend {}: {:.1} kN", column.family.title().1, column.combination.equation.label(), column.leading(), totals[i] / 1e3)),
            ring: rect(x + CELL_INSET, totals[i].min(0.0), x + 1.0 - CELL_INSET, totals[i].max(span * 1e-3)),
            fill: Fill { tone: if is_governing { Tone::Error } else { family_tone(column.family) }, opacity: if is_governing { 0.95 } else { 0.45 } },
            hatch: is_governing.then_some(Hatch::Diagonal),
        });
        let worst = worst_of_column(report, columns, column);
        markers.push(Marker {
            id: format!("envelope-{}", column.id),
            at: p(x + 0.5, totals[i]),
            shape: if is_governing { MarkerShape::Diamond } else { MarkerShape::Square },
            tone: if is_governing { Tone::Error } else { worst.map_or(Tone::Neutral, |w| utilisation_tone(w.utilization)) },
            label: loc(&format!("{} lead {}{}", column.combination.equation.label(), column.leading(), if is_governing { " (governing)" } else { "" }), &format!("{} führend {}{}", column.combination.equation.label(), column.leading(), if is_governing { " (maßgebend)" } else { "" })),
            utilisation: worst.map(|w| w.utilization.max(0.0)),
            bound_check: worst.map(|w| w.id.clone()),
            binding: None,
        });
    }
    series.push(Series::Limit { id: "envelope-max".into(), label: loc(&format!("Envelope maximum {:.1} kN", high / 1e3), &format!("Einhüllende Maximum {:.1} kN", high / 1e3)), axis: AxisRef::Y, value: high, style: stroke(Tone::Error, 1.5, Dash::Dashed) });
    series.push(Series::Limit { id: "envelope-min".into(), label: loc(&format!("Envelope minimum {:.1} kN", low / 1e3), &format!("Einhüllende Minimum {:.1} kN", low / 1e3)), axis: AxisRef::Y, value: low, style: stroke(Tone::Neutral, 1.5, Dash::Dashed) });
    if let Some(at) = active.and_then(|a| columns.iter().position(|c| c.id == a.id)) {
        let x = at as f64;
        series.push(Series::Polyline { id: "active-bar".into(), label: loc("Active combination", "Aktive Kombination"), points: vec![p(x, low), p(x + 1.0, low), p(x + 1.0, high), p(x, high), p(x, low)], style: stroke(Tone::Error, 3.0, Dash::Solid) });
    }
    Some(Diagram2d {
        id: "envelope:document".into(),
        title: loc("Envelope of all combinations", "Einhüllende aller Kombinationen"),
        description: loc(
            "The structure-level design effect Σγψ·E of every generated combination (unit influences); the governing ultimate combination is solid red, the dashed lines are the envelope limits and the framed bar is the active combination. Markers select a combination.",
            "Der Bemessungswert Σγψ·E der Gesamtstruktur jeder erzeugten Kombination (Einheitseinflüsse); die maßgebende Grenzzustandskombination ist durchgehend rot, die gestrichelten Linien sind die Einhüllenden und der umrahmte Balken ist die aktive Kombination. Marken wählen eine Kombination.",
        ),
        axes: vec![axis("Combination", "Kombination", QuantityKind::Dimensionless, "1", AxisScale::Linear, (0.0, columns.len() as f64)), axis("Design effect Σγψ·E", "Bemessungswert Σγψ·E", QuantityKind::Force, "N", AxisScale::Linear, (low - 0.1 * span, high + 0.1 * span))],
        series,
        markers,
        annotations: Vec::new(),
        focus: Some(selector_ref()),
        legend: true,
        table_twin: DataTable { columns: vec![column("Combination index", "Kombinationsindex", "1", (0..columns.len()).map(|i| i as f64).collect()), column("Design effect Σγψ·E", "Bemessungswert Σγψ·E", "N", totals), column("Governing", "Maßgebend", "1", (0..columns.len()).map(|i| f64::from(u8::from(governing == Some(i)))).collect())] },
    })
}

/// 🏁️ The cell of a member and a combination: the utilisation of the worst check of that member evaluated on that column, the governing cell of every member (largest utilisation over all columns) hatched with a diamond marker bound to its check; the square markers above the columns select the column and its worst check, the frame is the active combination. Sweeping the column markers (or the combination track) walks the structure through every combination; the governing combination of each member is visible in one row. Table twin with one utilisation column per member.
pub(super) fn governing_per_member(document: &En1990Snapshot, report: &CheckReport, columns: &[Column], active: Option<&Column>) -> Option<Diagram2d> {
    let mut cells: std::collections::BTreeMap<(usize, usize), &super::norm::CheckResult> = std::collections::BTreeMap::new();
    for check in report.checks.iter().filter(|c| c.utilization.is_finite()) {
        let Some(row) = document.members.iter().position(|m| m.id == check.subject.entity_id) else { continue };
        let Some(found) = column_for_check(columns, check) else { continue };
        let Some(col) = columns.iter().position(|c| c.id == found.id) else { continue };
        cells.entry((row, col)).and_modify(|best| if check.utilization > best.utilization { *best = check }).or_insert(check);
    }
    let members: Vec<usize> = (0..document.members.len()).filter(|row| cells.keys().any(|(r, _)| r == row)).collect();
    if members.is_empty() {
        return None;
    }
    let (n, rows) = (columns.len(), members.len());
    let mut series = Vec::new();
    let mut notes = Vec::new();
    let mut markers = Vec::new();
    let mut table = vec![column("Combination index", "Kombinationsindex", "1", (0..n).map(|c| c as f64).collect())];
    for (line, &row) in members.iter().enumerate() {
        let member = &document.members[row];
        let y = (rows - 1 - line) as f64;
        let governing = (0..n).filter_map(|c| cells.get(&(row, c)).map(|check| (c, *check))).max_by(|a, b| a.1.utilization.total_cmp(&b.1.utilization));
        notes.push(DiagramNote { id: format!("row-{line}"), at: p(-ROW_LABEL_WIDTH + 0.1, y + 0.5), text: loc(&member.label_en, &member.label_de), tone: Tone::Neutral });
        for c in 0..n {
            let Some(check) = cells.get(&(row, c)) else { continue };
            let u = check.utilization.max(0.0);
            let is_governing = governing.is_some_and(|(gc, _)| gc == c);
            series.push(Series::Polygon {
                id: format!("cell-{line}-{c}"),
                label: loc(&format!("{} in {} lead {}: {:.0} %", member.label_en, columns[c].combination.equation.label(), columns[c].leading(), u * 100.0), &format!("{} in {} führend {}: {:.0} %", member.label_de, columns[c].combination.equation.label(), columns[c].leading(), u * 100.0)),
                ring: rect(c as f64 + CELL_INSET, y + CELL_INSET, c as f64 + 1.0 - CELL_INSET, y + 1.0 - CELL_INSET),
                fill: Fill { tone: utilisation_tone(u), opacity: 0.2 + 0.7 * (u / 1.2).min(1.0) },
                hatch: is_governing.then_some(Hatch::Diagonal),
            });
        }
        if let Some((c, check)) = governing {
            let u = check.utilization.max(0.0);
            let col = &columns[c];
            markers.push(Marker {
                id: format!("governing-{}", member.id),
                at: p(c as f64 + 0.5, y + 0.5),
                shape: MarkerShape::Diamond,
                tone: utilisation_tone(u),
                label: loc(&format!("Governing for {}: {} · {} · lead {}, {:.0} %", member.label_en, col.family.title().0, col.combination.equation.label(), col.leading(), u * 100.0), &format!("Maßgebend für {}: {} · {} · führend {}, {:.0} %", member.label_de, col.family.title().1, col.combination.equation.label(), col.leading(), u * 100.0)),
                utilisation: Some(u),
                bound_check: Some(check.id.clone()),
                binding: None,
            });
        }
        table.push(column(&format!("Utilisation of {}", member.label_en), &format!("Ausnutzung von {}", member.label_de), "1", (0..n).map(|c| cells.get(&(row, c)).map_or(0.0, |check| check.utilization.max(0.0))).collect()));
    }
    for (c, col) in columns.iter().enumerate() {
        let worst = worst_of_column(report, columns, col);
        markers.push(Marker {
            id: format!("sweep-{}", col.id),
            at: p(c as f64 + 0.5, rows as f64 + 0.5),
            shape: MarkerShape::Square,
            tone: if active.is_some_and(|a| a.id == col.id) { Tone::Error } else { worst.map_or(Tone::Neutral, |w| utilisation_tone(w.utilization)) },
            label: loc(&format!("{} · {} · lead {}", col.family.title().0, col.combination.equation.label(), col.leading()), &format!("{} · {} · führend {}", col.family.title().1, col.combination.equation.label(), col.leading())),
            utilisation: worst.map(|w| w.utilization.max(0.0)),
            bound_check: worst.map(|w| w.id.clone()),
            binding: None,
        });
    }
    if let Some(at) = active.and_then(|a| columns.iter().position(|c| c.id == a.id)) {
        let x = at as f64;
        series.push(Series::Polyline { id: "active-column".into(), label: loc("Active combination", "Aktive Kombination"), points: vec![p(x, 0.0), p(x + 1.0, 0.0), p(x + 1.0, rows as f64), p(x, rows as f64), p(x, 0.0)], style: stroke(Tone::Error, 3.0, Dash::Solid) });
    }
    Some(Diagram2d {
        id: "governing-combinations:document".into(),
        title: loc("Governing combination per member", "Maßgebende Kombination je Bauteil"),
        description: loc(
            "Utilisation of every member under every generated combination; the hatched cell with the diamond is the governing combination of the member. Selecting a diamond or a square marker selects the underlying check, shows its combination in the viewport and frames it here; Tab walks the markers in combination order.",
            "Ausnutzung jedes Bauteils unter jeder erzeugten Kombination; die schraffierte Zelle mit der Raute ist die maßgebende Kombination des Bauteils. Eine Raute oder ein Quadrat wählt den zugehörigen Nachweis, zeigt dessen Kombination im Viewport und rahmt sie hier ein; Tab geht die Marker in Kombinationsreihenfolge durch.",
        ),
        axes: vec![axis("Combination", "Kombination", QuantityKind::Dimensionless, "1", AxisScale::Linear, (-ROW_LABEL_WIDTH, n as f64)), axis("Member", "Bauteil", QuantityKind::Dimensionless, "1", AxisScale::Linear, (0.0, rows as f64 + 1.0))],
        series,
        markers,
        annotations: notes,
        focus: Some(selector_ref()),
        legend: false,
        table_twin: DataTable { columns: table },
    })
}

/// 🎯️ The target reliability index β of RC1 to RC3 (Table B2) over the reference period T with the declared class emphasised; the vertical line is the project period and the markers are the target and the computed β of the project.
pub(super) fn beta_targets(document: &En1990Snapshot, report: &CheckReport) -> Diagram2d {
    let mut series = Vec::new();
    let mut table = vec![column("Reference period T", "Bezugszeitraum T", "a", PERIODS.to_vec())];
    for rc in 1..=3u8 {
        let values: Vec<f64> = PERIODS.iter().map(|years| target_reliability_index_for(rc, *years).unwrap_or(0.0)).collect();
        let own = rc == document.reliability_class.number();
        series.push(Series::Polyline {
            id: format!("rc{rc}"),
            label: loc(&format!("RC{rc} target β"), &format!("RC{rc} Zielwert β")),
            points: PERIODS.iter().zip(&values).map(|(years, beta)| p(*years, *beta)).collect(),
            style: stroke(if own { Tone::Warning } else { Tone::Neutral }, if own { 3.0 } else { 1.5 }, if own { Dash::Solid } else { Dash::Dashed }),
        });
        table.push(column(&format!("RC{rc} target β"), &format!("RC{rc} Zielwert β"), "1", values));
    }
    {
        let domain = document.structure_kind.domain();
        let (g_sup, _) = gamma_g(document.annex, domain, FactorSet::StrB);
        for rc in 1..=3u8 {
            let applied = k_fi_for_rc(rc).map_or(0.0, |k| k * g_sup);
            table.push(column(&format!("K_FI·γ_G,sup of RC{rc}"), &format!("K_FI·γ_G,sup von RC{rc}"), "1", vec![applied; PERIODS.len()]));
        }
    }
    let period = document.reference_period_years.clamp(PERIODS[0], PERIODS[PERIODS.len() - 1]);
    series.push(Series::Limit { id: "period".into(), label: loc(&format!("Reference period {:.0} a", document.reference_period_years), &format!("Bezugszeitraum {:.0} a", document.reference_period_years)), axis: AxisRef::X, value: period, style: stroke(Tone::Info, 1.5, Dash::Dotted) });
    let check = report.checks.iter().find(|c| c.id == "en1990.annex-c.beta");
    let mut markers = Vec::new();
    if let Some(target) = target_reliability_index_for(document.reliability_class.number(), document.reference_period_years) {
        markers.push(Marker {
            id: "reference:target".into(),
            at: p(period, target.clamp(0.0, BETA_MAX)),
            shape: MarkerShape::Square,
            tone: Tone::Warning,
            label: loc(&format!("Target β = {target:.2} (RC{})", document.reliability_class.number()), &format!("Zielwert β = {target:.2} (RC{})", document.reliability_class.number())),
            utilisation: None,
            bound_check: None,
            binding: None,
        });
    }
    markers.push(Marker {
        id: "computed".into(),
        at: p(period, document.beta_computed.clamp(0.0, BETA_MAX)),
        shape: MarkerShape::Circle,
        tone: check.map_or(Tone::Neutral, |c| utilisation_tone(c.utilization)),
        label: loc(&format!("Computed β = {:.2}", document.beta_computed), &format!("Berechnetes β = {:.2}", document.beta_computed)),
        utilisation: check.map(|c| c.utilization).filter(|u| u.is_finite() && *u >= 0.0),
        bound_check: check.map(|c| c.id.clone()),
        binding: None,
    });
    Diagram2d {
        id: "beta-targets:document".into(),
        title: loc("Target reliability index by reliability class", "Zielwert des Zuverlässigkeitsindex je Zuverlässigkeitsklasse"),
        description: loc(
            "The recommended minimum reliability index β of RC1 to RC3 over the reference period (EN 1990 Annex B Table B2: 1-year and 50-year anchors, other periods by the failure-probability conversion); the declared class is solid, the vertical line is the project's reference period and the circle is the computed β.",
            "Der empfohlene Mindestwert des Zuverlässigkeitsindex β von RC1 bis RC3 über den Bezugszeitraum (EN 1990 Anhang B Tabelle B2: Anker bei 1 und 50 Jahren, andere Zeiträume über die Umrechnung der Versagenswahrscheinlichkeit); die gewählte Klasse ist durchgezogen, die senkrechte Linie ist der Bezugszeitraum des Projekts und der Kreis das berechnete β.",
        ),
        axes: vec![axis("Reference period T", "Bezugszeitraum T", QuantityKind::Time, "a", AxisScale::Log10, (PERIODS[0], PERIODS[PERIODS.len() - 1])), axis("Reliability index β", "Zuverlässigkeitsindex β", QuantityKind::Dimensionless, "1", AxisScale::Linear, (0.0, BETA_MAX))],
        series,
        markers,
        annotations: Vec::new(),
        focus: None,
        legend: true,
        table_twin: DataTable { columns: table },
    }
}

/// 🚦️ Annex A2 traffic load groups as a typed table: one row per traffic action (x: component, y: load group of its category), declared group ids that break the mutual exclusion are warned, every marker selects the traffic-group check whose utilisation is 1 + the share of offending components.
pub(super) fn traffic_groups(document: &En1990Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let check = report.checks.iter().find(|c| c.id == "en1990.a2.traffic-groups" && c.utilization.is_finite())?;
    let traffic: Vec<_> = document.variables.iter().filter_map(|variable| traffic_group(&variable.category).map(|group| (variable, group))).collect();
    let mut order: Vec<&str> = Vec::new();
    for (_, group) in &traffic {
        if !order.contains(group) {
            order.push(group);
        }
    }
    let effective = |variable: &super::model::VariableAction| super::inferences::effective_group(document, variable);
    let consistent = |variable: &super::model::VariableAction, group: &str| !effective(variable).is_empty() && traffic.iter().all(|(other, other_group)| other.id == variable.id || ((effective(other) == effective(variable)) == (*other_group == group)));
    let markers: Vec<Marker> = traffic
        .iter()
        .enumerate()
        .map(|(i, (variable, group))| {
            let row = order.iter().position(|g| g == group).unwrap_or(0);
            let ok = consistent(variable, group);
            Marker {
                id: format!("traffic-{}", variable.id),
                at: p(i as f64 + 0.5, row as f64 + 0.5),
                shape: MarkerShape::Square,
                tone: if ok { Tone::Info } else { Tone::Warning },
                label: loc(&format!("{} · {group} · declared group \"{}\"", variable.id, variable.group), &format!("{} · {group} · angegebene Gruppe \"{}\"", variable.id, variable.group)),
                utilisation: Some(check.utilization.max(0.0)),
                bound_check: Some(check.id.clone()),
                binding: None,
            }
        })
        .collect();
    let table = vec![
        column("Component index", "Bestandteil-Index", "1", (0..traffic.len()).map(|i| i as f64).collect()),
        column("Load group index", "Lastgruppen-Index", "1", traffic.iter().map(|(_, g)| order.iter().position(|o| o == g).unwrap_or(0) as f64).collect()),
        column("Characteristic value", "Charakteristischer Wert", "1", traffic.iter().map(|(v, _)| v.qk).collect()),
        column("Consistent group id (1 = yes)", "Konsistente Gruppen-Id (1 = ja)", "1", traffic.iter().map(|(v, g)| f64::from(u8::from(consistent(v, g)))).collect()),
    ];
    Some(Diagram2d {
        id: "traffic-groups:document".into(),
        title: loc("Traffic load groups (Annex A2)", "Verkehrslastgruppen (Anhang A2)"),
        description: loc(
            "Every traffic action against the mutually exclusive load group of its category (EN 1990 Tables A2.1 to A2.3); components of one group share a group id, other groups never act together. Warning markers break that rule; each marker selects the traffic-group check.",
            "Jede Verkehrslast gegenüber der sich ausschließenden Lastgruppe ihrer Kategorie (EN 1990 Tabellen A2.1 bis A2.3); Bestandteile einer Gruppe teilen eine Gruppen-Id, andere Gruppen wirken nie zusammen. Warnmarken verletzen diese Regel; jede Marke wählt die Verkehrsgruppen-Prüfung.",
        ),
        axes: vec![
            axis("Traffic component", "Verkehrslast", QuantityKind::Dimensionless, "1", AxisScale::Linear, (0.0, traffic.len().max(1) as f64)),
            axis("Load group", "Lastgruppe", QuantityKind::Dimensionless, "1", AxisScale::Linear, (0.0, order.len().max(1) as f64)),
        ],
        series: Vec::new(),
        markers,
        annotations: order.iter().enumerate().map(|(row, group)| DiagramNote { id: format!("group-{group}"), at: p(0.05, row as f64 + 0.8), text: loc(group, group), tone: Tone::Neutral }).collect(),
        focus: None,
        legend: false,
        table_twin: DataTable { columns: table },
    })
}
