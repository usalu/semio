//! 🧰️ Report helpers of the EN 1990 compliance report: the copy, quantity and subject helpers, the resistance check builder and the action list per member. See [`super`].

use super::*;

//#region 🔖️ComplianceReport
pub(super) fn copy(en: impl Into<String>, de: impl Into<String>) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

pub(super) fn q_force(n: f64) -> Quantity {
    Quantity::new(QuantityKind::Force, n)
}

pub(super) fn q_len(m: f64) -> Quantity {
    Quantity::new(QuantityKind::Length, m)
}

pub(super) fn q_dim(v: f64) -> Quantity {
    Quantity::new(QuantityKind::Dimensionless, v)
}

pub(super) fn member_label(member: &Member) -> LocalizedCopy {
    copy(member.label_en.clone(), member.label_de.clone())
}

pub(super) fn project_subject(document: &En1990Snapshot) -> SubjectRef {
    SubjectRef::new(
        document.project_id.clone(),
        "projectId",
        copy(format!("Project {}", document.project_id), format!("Projekt {}", document.project_id)),
    )
}

pub(super) fn member_path(member_id: &str, field: &str) -> String {
    format!("members[id={member_id}].{field}")
}

pub(super) fn member_subject(member: &Member, field: &str) -> SubjectRef {
    SubjectRef::new(&member.id, member_path(&member.id, field), member_label(member))
}

fn influence_for(effects: &[MemberEffect], member_id: &str, action_id: &str) -> f64 {
    effects.iter().find(|e| e.member_id == member_id && e.action_id == action_id).map_or(0.0, |e| e.influence)
}

/// 🚦️ The exclusive group id of a variable action: its typed `trafficGroups` row (lower-case group id such as `gr1a`) wins over the free `group` text.
pub fn effective_group(document: &En1990Snapshot, variable: &VariableAction) -> String {
    document.traffic_groups.iter().find(|row| row.variable_id == variable.id).map_or_else(|| variable.group.clone(), |row| row.group.as_str().to_string())
}

/// 🆔️ The registry id of the coefficient of a component category in a traffic load group.
pub fn group_coefficient_id(group: LoadGroup, category: &str) -> String {
    format!("ndp.en1990.a2.group.{}.{category}", group.as_str())
}

/// 🔢️ The coefficient applied to the declared characteristic value of a traffic component: 1.0 without a table row, the registry value of the (group, category) pair with one, `None` when the registry holds no value for the pair.
pub fn group_coefficient(document: &En1990Snapshot, variable: &VariableAction) -> Option<f64> {
    document.traffic_groups.iter().find(|row| row.variable_id == variable.id).map_or(Some(1.0), |row| ndp_lookup(document.annex, &group_coefficient_id(row.group, &variable.category)))
}

/// ⚡️ The document's actions as generator input with `influence(action_id)` as effect factor: the sign of the effect alone decides γG,sup or γG,inf for permanents and whether a variable action is counted.
pub fn actions_with(document: &En1990Snapshot, influence: impl Fn(&str) -> f64) -> Vec<Action> {
    let mut actions = Vec::new();
    for permanent in &document.permanents {
        let class = match permanent.kind {
            semio_s_artifact_norm_en1990_model::PermanentKind::GSup | semio_s_artifact_norm_en1990_model::PermanentKind::GInf => ActionClass::Permanent,
            semio_s_artifact_norm_en1990_model::PermanentKind::Prestress => ActionClass::Prestress,
            _ => continue,
        };
        actions.push(Action::new(&permanent.id, class, influence(&permanent.id) * permanent.gk).overrides(factor_overrides::action_override(document, &permanent.id)));
    }
    for variable in &document.variables {
        let value = influence(&variable.id) * variable.qk * group_coefficient(document, variable).unwrap_or(1.0);
        let category = resolve_psi_category(document.annex, &variable.category, document.altitude_m);
        actions.push(Action::new(&variable.id, ActionClass::Variable, value).category(category).group(effective_group(document, variable)).overrides(factor_overrides::action_override(document, &variable.id)));
    }
    actions.extend(document.accidentals.iter().map(|a| Action::new(&a.id, ActionClass::Accidental, influence(&a.id) * a.ad)));
    actions.extend(document.seismics.iter().map(|s| Action::new(&s.id, ActionClass::Seismic, influence(&s.id) * s.a_ed())));
    actions
}

/// ⚡️ The member's actions as generator input: effect = influence × magnitude.
pub fn member_actions(document: &En1990Snapshot, member_id: &str) -> Vec<Action> {
    let mut actions = actions_with(document, |action_id| influence_for(&document.effects, member_id, action_id));
    apply_wind_cap(document, member_id, &mut actions);
    actions
}

/// 🧢️ F*_W of Annex A2.2.2: while a traffic load acts, a persistent wind action of a bridge member with a declared basic wind speed v_b contributes at most F_Wk·(v*/v_b)² (registry `a2.fw.speed_cap`); without a declared speed the wind action stays uncapped.
fn apply_wind_cap(document: &En1990Snapshot, member_id: &str, actions: &mut [Action]) {
    let Some(speed) = document.bridge_sls.iter().find(|row| row.member_id == member_id).map(|row| row.basic_wind_speed).filter(|speed| *speed > 0.0) else { return };
    let ratio = (ndp(document.annex, "ndp.en1990.a2.fw.speed_cap") / speed).powi(2);
    if !(ratio < 1.0) {
        return;
    }
    for action in actions.iter_mut().filter(|action| action.class == ActionClass::Variable && non_simultaneous_family(&action.category) == Some("wind") && !is_execution_category(&action.category)) {
        action.cap = Some(action.value.abs() * ratio);
    }
}

/// 🧭️ The generator context of one limit-state family of the document, `None` when structure kind or reliability class are invalid.
pub fn context_for(document: &En1990Snapshot, set: FactorSet) -> Option<Context> {
    let k_fi = factor_overrides::global_override(document, "k_fi");
    let mut ctx = Context::new(document.annex, document.structure_kind.domain(), set, k_fi.or_else(|| k_fi_for_rc(document.reliability_class.number()))?);
    if k_fi.is_some() {
        ctx = ctx.with_k_fi_override();
    }
    Some(match factor_overrides::global_override(document, "xi") {
        Some(xi) => ctx.with_xi(xi),
        None => ctx,
    })
}

pub(super) fn annex_tag(document: &En1990Snapshot, set: FactorSet) -> String {
    let domain = if document.structure_kind.is_bridge() { Domain::Bridge } else { Domain::Building };
    format!("EN 1990 {} {}", match set { FactorSet::EquA => "EQU", FactorSet::StrB => "STR", FactorSet::GeoC | FactorSet::Geo2 => "GEO" }, set.label(domain))
}

/// ✅️ One effect-versus-resistance check; a failing check carries the `at_least` remedy on the resistance field.
pub(super) struct ResistanceCheck<'a> {
    pub id: String,
    pub part: String,
    pub clause: ClauseId,
    pub subject: SubjectRef,
    pub title: LocalizedCopy,
    pub annex: AnnexChoice,
    pub effect: f64,
    pub resistance: f64,
    pub explanation: LocalizedCopy,
    pub remedy_en: &'a str,
    pub remedy_de: &'a str,
}

impl ResistanceCheck<'_> {
    pub(super) fn build(self) -> CheckResult {
        self.build_citing(&[])
    }

    pub(super) fn build_citing(self, ndps: &[&str]) -> CheckResult {
        let mut builder = CheckResult::assess(self.id, self.part, self.clause, self.subject.clone(), self.title).utilization(q_force(self.effect), q_force(self.resistance)).annex(self.annex).explanation(self.explanation);
        for id in ndps {
            builder = builder.ndp(id);
        }
        builder = builder.ndp_reads();
        if self.effect > self.resistance {
            builder = builder.remedy(Remedy::at_least(
                self.subject,
                q_force(self.resistance),
                q_force(self.effect),
                copy(format!("{} to at least {:.1} kN.", self.remedy_en, self.effect / 1000.0), format!("{} auf mindestens {:.1} kN.", self.remedy_de, self.effect / 1000.0)),
            ));
        }
        builder.build()
    }
}

pub(super) fn not_applicable(id: String, part: &str, clause: ClauseId, subject: SubjectRef, title: LocalizedCopy, annex: AnnexChoice, reason: LocalizedCopy) -> CheckResult {
    CheckResult::assess(id, part, clause, subject, title).annex(annex).not_applicable(reason).build()
}
//#endregion 🔖️ComplianceReport

/// 📜️ Registry ids (`ndp.en1990.*`) of the unverified values a variable action category relies on.
pub(super) fn psi_ndps(document: &En1990Snapshot) -> Vec<&'static str> {
    let mut ids = Vec::new();
    for variable in &document.variables {
        match variable.category.as_str() {
            "temperature_bridge" if document.annex == AnnexChoice::De => ids.push("ndp.en1990.e90-25.1"),
            "footbridge_crowd" | "footbridge_service" => ids.push("ndp.en1990.e90-25.2"),
            _ => {}
        }
    }
    ids
}

/// 📜️ Registry ids cited by the ULS combination checks of the document.
pub(super) fn uls_ndps(document: &En1990Snapshot, equ: bool) -> Vec<&'static str> {
    let mut ids = psi_ndps(document);
    if document.annex == AnnexChoice::De && !equ {
        ids.push("ndp.en1990.e90-16.1");
    }
    if equ && document.structure_kind.is_bridge() {
        ids.push("ndp.en1990.e90-26.1");
    }
    ids
}
