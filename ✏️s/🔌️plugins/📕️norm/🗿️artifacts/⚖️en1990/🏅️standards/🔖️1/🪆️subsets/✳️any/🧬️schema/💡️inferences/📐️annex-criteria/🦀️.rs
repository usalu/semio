//! 📐️ Annex criteria of the EN 1990 report: the declared A1.4 and A2.4 limits against the NDP recommendations, the Annex A2 traffic groups and execution situation, and the K_FI as applied by Annex B. See [`super`].

use super::*;

//#region 🔖️ComplianceReport
#[derive(Clone, Copy, PartialEq)]
enum Bound {
    AtLeast,
    AtMost,
}

/// 🧾️ One declared criterion that is compared with its NDP recommendation; a deviation is a warning with a remedy, never a silent pass.
struct Basis {
    id: String,
    clause: &'static str,
    subject: SubjectRef,
    title: (&'static str, &'static str),
    declared: f64,
    bound: f64,
    direction: Bound,
    kind: QuantityKind,
    unit: &'static str,
}

impl Basis {
    fn push(self, report: &mut CheckReport, document: &En1990Snapshot) {
        let quantity = |value: f64| Quantity::new(self.kind, value);
        let tolerance = 1e-9 * self.bound.abs().max(1.0);
        let holds = match self.direction {
            Bound::AtLeast => self.declared + tolerance >= self.bound,
            Bound::AtMost => self.declared - tolerance <= self.bound,
        };
        let (relation, verb_en, verb_de) = match self.direction {
            Bound::AtLeast => ("≥", "Raise", "Erhöhen"),
            Bound::AtMost => ("≤", "Reduce", "Verringern"),
        };
        let mut builder = CheckResult::assess(self.id, "EN 1990 Annex A criteria", ClauseId::new("EN 1990", self.clause, self.clause), self.subject.clone(), copy(self.title.0, self.title.1)).annex(document.annex);
        builder = match self.direction {
            Bound::AtLeast => builder.minimum(quantity(self.declared), quantity(self.bound)),
            Bound::AtMost => builder.utilization(quantity(self.declared), quantity(self.bound)),
        };
        builder = builder.status(if holds { CheckStatus::Pass } else { CheckStatus::Warning }).explanation(copy(
            format!("Declared {:.4}{unit} against the recommended {relation} {:.4}{unit}{}", self.declared, self.bound, if holds { "" } else { "; the declared criterion is more lenient than the recommendation" }, unit = self.unit),
            format!("Angesetzt {:.4}{unit} gegenüber empfohlen {relation} {:.4}{unit}{}", self.declared, self.bound, if holds { "" } else { "; das angesetzte Kriterium ist nachgiebiger als die Empfehlung" }, unit = self.unit),
        ));
        if !holds {
            let text = copy(format!("{verb_en} the criterion to {:.4}{}.", self.bound, self.unit), format!("{verb_de} des Kriteriums auf {:.4}{}.", self.bound, self.unit));
            builder = builder.remedy(match self.direction {
                Bound::AtLeast => Remedy::at_least(self.subject, quantity(self.declared), quantity(self.bound), text),
                Bound::AtMost => Remedy::at_most(self.subject, quantity(self.declared), quantity(self.bound), text),
            });
        }
        report.push(builder.ndp_reads().build());
    }
}

/// 📏️ The declared deflection, vibration and bridge SLS criteria of a member against the A1.4 and A2.4 recommendations of the NDP registry.
pub(super) fn push_limit_basis(report: &mut CheckReport, document: &En1990Snapshot, member: &Member) {
    let annex = document.annex;
    if !document.structure_kind.is_bridge() {
        for (key, field, declared, id, clause, title, unit) in [
            ("net-ratio", "deflectionLimitRatio", member.deflection_limit_ratio, "ndp.en1990.sls.w_net.ratio", "A1.4.3", ("Declared limit L/n of w_net,fin", "Angesetzte Grenze L/n von w_net,fin"), ""),
            ("w3-ratio", "deflectionLimitRatioW3", member.deflection_limit_ratio_w3, "ndp.en1990.sls.w3.ratio", "A1.4.3", ("Declared limit L/n of w3", "Angesetzte Grenze L/n von w3"), ""),
            ("vibration", "vibrationFrequencyMin", member.vibration_frequency_min, "ndp.en1990.sls.floor_frequency_hz", "A1.4.4", ("Declared minimum natural frequency", "Angesetzte Mindest-Eigenfrequenz"), " Hz"),
        ] {
            if declared > 0.0 {
                Basis { id: format!("en1990.a14.basis.{key}.{}", member.id), clause, subject: member_subject(member, field), title, declared, bound: ndp(annex, id), direction: Bound::AtLeast, kind: QuantityKind::Dimensionless, unit }.push(report, document);
            }
        }
        return;
    }
    let Some(sls) = document.bridge_sls.iter().find(|row| row.member_id == member.id) else { return };
    let subject = |field: &str| SubjectRef::new(&sls.id, format!("bridgeSls[id={}].{field}", sls.id), member_label(member));
    let rail = document.structure_kind == semio_s_artifact_norm_en1990_model::StructureKind::RailBridge;
    let mut rows = Vec::new();
    if document.structure_kind == semio_s_artifact_norm_en1990_model::StructureKind::Footbridge && sls.deck_acceleration_limit > 0.0 {
        rows.push(("acceleration", "2.4.3.2", ("Declared footbridge acceleration limit", "Angesetzte Fußgängerbrücken-Beschleunigungsgrenze"), sls.deck_acceleration_limit, "ndp.en1990.a2.footbridge.acc_v", QuantityKind::Acceleration, " m/s²", "deckAccelerationLimit"));
    }
    if rail {
        if sls.deck_acceleration_limit > 0.0 {
            rows.push(("acceleration", "2.4.4.2.1", ("Declared deck acceleration limit", "Angesetzte Deckbeschleunigungsgrenze"), sls.deck_acceleration_limit, "ndp.en1990.a2.rail.acc_slab", QuantityKind::Acceleration, " m/s²", "deckAccelerationLimit"));
        }
        if sls.deck_twist_limit > 0.0 {
            rows.push(("twist", "2.4.4.2.2", ("Declared deck twist limit", "Angesetzte Verwindungsgrenze"), sls.deck_twist_limit, "ndp.en1990.a2.rail.twist_120", QuantityKind::Dimensionless, " rad", "deckTwistLimit"));
        }
        if sls.bridge_deflection_limit > 0.0 && member.span > 0.0 {
            rows.push(("deflection", "2.4.4.3", ("Declared deflection limit", "Angesetzte Durchbiegungsgrenze"), sls.bridge_deflection_limit, "ndp.en1990.a2.rail.deflection.ratio", QuantityKind::Length, " m", "bridgeDeflectionLimit"));
        }
    }
    for (key, clause, title, declared, id, kind, unit, field) in rows {
        let bound = match key {
            "twist" => ndp(annex, id) / 3000.0,
            "deflection" => member.span / ndp(annex, id),
            _ => ndp(annex, id),
        };
        Basis { id: format!("en1990.a2.basis.{key}.{}.{}", member.id, sls.id), clause, subject: subject(field), title, declared, bound, direction: Bound::AtMost, kind, unit }.push(report, document);
    }
}

fn group_subject(variable: &semio_s_artifact_norm_en1990_model::VariableAction) -> SubjectRef {
    SubjectRef::new(&variable.id, format!("variables[id={}].group", variable.id), copy(format!("Traffic group of {}", variable.id), format!("Verkehrsgruppe von {}", variable.id)))
}

/// 🚦️ Annex A2 traffic load groups: the components of one group share a group id and every other group is mutually exclusive with it (Tables A2.1 to A2.3).
pub(super) fn push_traffic_groups(report: &mut CheckReport, document: &En1990Snapshot) {
    if !document.structure_kind.is_bridge() {
        return;
    }
    let traffic: Vec<_> = document.variables.iter().filter_map(|variable| traffic_group(&variable.category).map(|group| (variable, group))).collect();
    let clause = ClauseId::new("EN 1990", "A2.2.2", "Tables A2.1-A2.3");
    let title = copy("Mutually exclusive traffic load groups", "Sich gegenseitig ausschließende Verkehrslastgruppen");
    let distinct: std::collections::BTreeSet<&str> = traffic.iter().map(|(_, group)| *group).collect();
    if distinct.len() < 2 {
        return report.push(not_applicable("en1990.a2.traffic-groups".into(), "EN 1990 Annex A2", clause, project_subject(document), title, document.annex, copy("Fewer than two traffic load groups are declared.", "Weniger als zwei Verkehrslastgruppen angegeben.")));
    }
    let effective = |variable: &semio_s_artifact_norm_en1990_model::VariableAction| effective_group(document, variable);
    let violates = |(variable, group): &(&semio_s_artifact_norm_en1990_model::VariableAction, &str)| {
        effective(variable).is_empty()
            || traffic.iter().any(|(other, other_group)| other.id != variable.id && ((effective(other) == effective(variable) && other_group != group) || (other_group == group && effective(other) != effective(variable))))
    };
    let offender = traffic.iter().find(|entry| violates(entry));
    let utilisation = 1.0 + traffic.iter().filter(|entry| violates(entry)).count() as f64 / traffic.len() as f64;
    let mut builder = CheckResult::assess("en1990.a2.traffic-groups", "EN 1990 Annex A2", clause, offender.map_or_else(|| project_subject(document), |(variable, _)| group_subject(variable)), title).annex(document.annex)
        .utilization(Quantity::new(QuantityKind::Dimensionless, utilisation), Quantity::new(QuantityKind::Dimensionless, 1.0));
    builder = match offender {
        None => builder.status(CheckStatus::Pass).explanation(copy(
            format!("{} traffic load groups ({}) are modelled as mutually exclusive groups.", distinct.len(), distinct.iter().copied().collect::<Vec<_>>().join(", ")),
            format!("{} Verkehrslastgruppen ({}) sind als sich ausschließende Gruppen modelliert.", distinct.len(), distinct.iter().copied().collect::<Vec<_>>().join(", ")),
        )),
        Some((variable, group)) => builder.status(CheckStatus::Warning).explanation(copy(
            format!("Traffic action {} (group {group}) acts together with other load groups; groups gr1a to gr5, the footbridge and the rail load models are mutually exclusive and their components share one group id.", variable.id),
            format!("Verkehrslast {} (Gruppe {group}) wirkt mit anderen Lastgruppen zusammen; die Gruppen gr1a bis gr5, die Fußgängerbrücken- und die Eisenbahnlastmodelle schließen sich aus und ihre Bestandteile teilen eine Gruppen-Id.", variable.id),
        )).remedy(Remedy::one_of(group_subject(variable), vec![group.to_string()], copy(format!("Set the group of {} to {group}.", variable.id), format!("Die Gruppe von {} auf {group} setzen.", variable.id)))),
    };
    report.push(builder.build());
}

/// 🚧️ The transient execution situation (Annex A2): permanent actions with F*_W, Q_Sn,k and Q_c only, Eq. 6.10 with the Set B factors.
pub(super) fn push_execution_situation(report: &mut CheckReport, document: &En1990Snapshot, member: &Member, actions: &[Action]) {
    let Some(ctx) = context_for(document, FactorSet::StrB).map(Context::transient) else { return };
    if !actions.iter().any(|action| action.class == ActionClass::Variable && is_execution_category(&action.category) && action.value != 0.0) {
        return;
    }
    let Some(combination) = combinations(&ctx, actions, Equation::E610).ok().and_then(|all| governing(&all, false).cloned()) else { return };
    report.push(
        ResistanceCheck {
            id: format!("en1990.a2.transient.{}", member.id),
            part: annex_tag(document, FactorSet::StrB),
            clause: ClauseId::new("EN 1990", "A2.2.1", "execution"),
            subject: member_subject(member, "rdStr"),
            title: copy("Execution situation (transient) Eq. 6.10", "Bauzustand (vorübergehend) Gl. 6.10"),
            annex: document.annex,
            effect: combination.total(),
            resistance: member.rd_str,
            explanation: copy(format!("E_d = {:.1} N ({}) vs R_d,STR = {:.1} N", combination.total(), combination.describe(), member.rd_str), format!("E_d = {:.1} N ({}) gegenüber R_d,STR = {:.1} N", combination.total(), combination.describe(), member.rd_str)),
            remedy_en: &format!("Increase R_d,STR of {} for the execution situation", member.label_en),
            remedy_de: &format!("R_d,STR von {} für den Bauzustand erhöhen", member.label_de),
        }
        .build_citing(&[]),
    );
}

/// 🎚️ Annex B: the K_FI of the reliability class and the unfavourable partial factors it scales.
pub(super) fn push_k_fi_applied(report: &mut CheckReport, document: &En1990Snapshot) {
    let (Some(k_fi), Some(domain)) = (k_fi_for_rc(document.reliability_class.number()), Some(document.structure_kind.domain())) else { return };
    let (g_sup, g_inf) = gamma_g(document.annex, domain, FactorSet::StrB);
    let g_q = gamma_q(document.annex, domain, FactorSet::StrB, "other");
    let subject = SubjectRef::new(document.project_id.clone(), "reliabilityClass", copy("Applied K_FI", "Angewendetes K_FI"));
    report.push(
        CheckResult::assess("en1990.annex-b.kfi-applied", "EN 1990 Annex B", ClauseId::new("EN 1990", "Annex B", "B3.3 Table B3"), subject, copy("K_FI applied to the unfavourable partial factors", "K_FI auf die ungünstigen Teilsicherheitsbeiwerte angewendet"))
            .informative(q_dim(k_fi * g_sup))
            .annex(document.annex)
            .explanation(copy(
                format!("K_FI = {k_fi:.2} (RC{}) scales the unfavourable factors only: γ_G,sup {g_sup:.3} → {:.3}, γ_Q {g_q:.3} → {:.3}; γ_G,inf = {g_inf:.3}, γ_P and γ_M are unchanged.", document.reliability_class.number(), k_fi * g_sup, k_fi * g_q),
                format!("K_FI = {k_fi:.2} (RC{}) skaliert nur die ungünstigen Beiwerte: γ_G,sup {g_sup:.3} → {:.3}, γ_Q {g_q:.3} → {:.3}; γ_G,inf = {g_inf:.3}, γ_P und γ_M bleiben unverändert.", document.reliability_class.number(), k_fi * g_sup, k_fi * g_q),
            ))
            .ndp_reads()
            .build(),
    );
}

fn table_subject(row: &TrafficGroup, field: &str) -> SubjectRef {
    SubjectRef::new(&row.id, format!("trafficGroups[id={}].{field}", row.id), copy(format!("Traffic group row {}", row.id), format!("Verkehrsgruppenzeile {}", row.id)))
}

/// 🚦️ One check per `trafficGroups` row (EN 1990 A2.2.2 with EN 1991-2 Table 4.4a): the component exists, belongs to a traffic category of the row's group and the registry holds its coefficient; every registry coefficient is unverified, so a consistent row reports an unverified basis, never a compliant verdict.
pub(super) fn push_traffic_group_rows(report: &mut CheckReport, document: &En1990Snapshot) {
    if !document.structure_kind.is_bridge() {
        return;
    }
    for row in &document.traffic_groups {
        let clause = ClauseId::new("EN 1990", "A2.2.2", "Table A2.4, EN 1991-2 Table 4.4a");
        let title = copy("Traffic load group component", "Bestandteil der Verkehrslastgruppe");
        let builder = CheckResult::assess(format!("en1990.a2.traffic-group.{}", row.id), "EN 1990 Annex A2", clause, table_subject(row, "variableId"), title).annex(document.annex);
        let Some(variable) = document.variables.iter().find(|variable| variable.id == row.variable_id) else {
            let ids: Vec<String> = document.variables.iter().map(|variable| variable.id.clone()).collect();
            report.push(builder.status(CheckStatus::Fail).explanation(copy(format!("Component {:?} of {} is not a variable action.", row.variable_id, row.group.as_str()), format!("Bestandteil {:?} von {} ist keine veränderliche Einwirkung.", row.variable_id, row.group.as_str())))
                .remedy(Remedy::one_of(table_subject(row, "variableId"), ids, copy("Name an existing variable action.", "Eine vorhandene veränderliche Einwirkung benennen."))).build());
            continue;
        };
        let natural = traffic_group(&variable.category);
        let id = group_coefficient_id(row.group, &variable.category);
        let coefficient = ndp_lookup(document.annex, &id);
        let awaiting = ndp_awaits_value(document.annex, &id);
        let (en, de, status, utilisation) = match (natural, coefficient) {
            (_, None) if awaiting => (
                format!("Awaiting value: the registry cell {id} (EN 1991-2 Table 4.4a, {} accompanying {} in {}) is not transcribed yet; the combinations use the declared value unchanged, so this row is not a verified basis.", variable.id, variable.category, row.group.as_str()),
                format!("Wert ausstehend: die Registerzelle {id} (EN 1991-2 Tabelle 4.4a, {} als begleitende {} in {}) ist noch nicht übertragen; die Kombinationen verwenden den angegebenen Wert unverändert, diese Zeile ist daher keine geprüfte Grundlage.", variable.id, variable.category, row.group.as_str()),
                CheckStatus::UnverifiedBasis,
                None,
            ),
            (None, _) => (format!("{} (category {}) is not a road traffic component.", variable.id, variable.category), format!("{} (Kategorie {}) ist kein Straßenverkehrsbestandteil.", variable.id, variable.category), CheckStatus::Warning, None),
            (Some(group), _) if group != row.group.as_str() => (format!("{} belongs to {group} by its category, not to {}.", variable.id, row.group.as_str()), format!("{} gehört nach seiner Kategorie zu {group}, nicht zu {}.", variable.id, row.group.as_str()), CheckStatus::Warning, None),
            (Some(_), None) => (format!("The registry holds no coefficient for {} in {}; the declared value is used unchanged.", variable.category, row.group.as_str()), format!("Das Register enthält keinen Beiwert für {} in {}; der angegebene Wert wird unverändert verwendet.", variable.category, row.group.as_str()), CheckStatus::Warning, None),
            (Some(_), Some(c)) => (format!("{} acts in {} with coefficient {c:.2} on its characteristic value.", variable.id, row.group.as_str()), format!("{} wirkt in {} mit dem Beiwert {c:.2} auf seinen charakteristischen Wert.", variable.id, row.group.as_str()), if c <= 1.0 { CheckStatus::Pass } else { CheckStatus::Warning }, Some(c)),
        };
        let mut builder = builder.status(status).explanation(copy(en, de));
        if let Some(c) = utilisation {
            builder = builder.utilization(q_dim(c), q_dim(1.0));
        }
        if !matches!(status, CheckStatus::Pass | CheckStatus::UnverifiedBasis) {
            let target = natural.and_then(LoadGroup::from_id).map_or_else(|| vec![variable.id.clone()], |group| vec![format!("{group:?}")]);
            builder = builder.remedy(Remedy::one_of(table_subject(row, if natural.is_some() { "group" } else { "variableId" }), target, copy("Assign the component to its group.", "Den Bestandteil seiner Gruppe zuordnen.")));
        }
        if coefficient.is_some() || awaiting {
            builder = builder.ndp(&id);
        }
        report.push(builder.build());
    }
}
//#endregion 🔖️ComplianceReport
