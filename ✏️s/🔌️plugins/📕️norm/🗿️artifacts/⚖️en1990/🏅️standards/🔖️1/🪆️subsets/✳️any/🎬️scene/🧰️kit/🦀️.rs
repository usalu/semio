//! 🧰️ Scene kit: vector helpers, the entity builder, utilisation bindings from the report, layers, legends and the entity-kind registry.

extern crate semio_s_artifact_norm_en1990_checks as checks;
extern crate semio_s_artifact_norm_en1990_inferences as inferences;
extern crate semio_s_artifact_norm_en1990_model as model;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

use checks::catalogue::En1990Cases;
use scene::*;
use scene::projection::{Fragment, Part};
use scene::projection_kit::design_case::forms;
use scene::projection_kit::{self, base_id, loc, unit, entity_ref, utilisation_words};
use std::collections::BTreeSet;

pub const LAYER_MEMBERS: &str = "members";
pub const LAYER_DECKS: &str = "decks";
pub const LAYER_LOADS: &str = "loads";
pub const LAYER_SUPPORTS: &str = "supports";
pub const LAYER_ANNOTATIONS: &str = "annotations";
pub const LAYER_ANALYSES: &str = "analyses";
pub const LAYER_CASES: &str = "cases";
pub const LAYER_GAMMA: &str = "gamma-bands";

const LAYERS: [(&str, &str, &str); 8] = [
    (LAYER_MEMBERS, "Members", "Bauteile"),
    (LAYER_DECKS, "Bridge decks", "Brückenfahrbahnen"),
    (LAYER_LOADS, "Actions", "Einwirkungen"),
    (LAYER_SUPPORTS, "Supports", "Lager"),
    (LAYER_ANNOTATIONS, "Annotations", "Beschriftungen"),
    (LAYER_ANALYSES, "Analyses", "Analysen"),
    (LAYER_CASES, "Design cases", "Bemessungsfälle"),
    (LAYER_GAMMA, "Permanent factor bands", "Bänder der ständigen Teilsicherheitsbeiwerte"),
];

/// 🏷️ One entity kind of the EN 1990 outliner, interaction domain and insert palette.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntityKindSpec {
    pub id: &'static str,
    pub collection: &'static str,
    pub label_en: &'static str,
    pub label_de: &'static str,
    pub icon: &'static str,
    pub scene_kind: EntityKind,
}

const fn kind(id: &'static str, collection: &'static str, label_en: &'static str, label_de: &'static str, icon: &'static str, scene_kind: EntityKind) -> EntityKindSpec {
    EntityKindSpec { id, collection, label_en, label_de, icon, scene_kind }
}

/// 🏷️ The single registry of EN 1990 entity kinds, in outliner order.
pub const ENTITY_KINDS: &[EntityKindSpec] = &[
    kind("member", "members", "Member", "Bauteil", "minus", EntityKind::Member),
    kind("permanent", "permanents", "Permanent action", "Ständige Einwirkung", "arrow-down", EntityKind::Load),
    kind("variable", "variables", "Variable action", "Veränderliche Einwirkung", "arrow-down-circle", EntityKind::Load),
    kind("accidental", "accidentals", "Accidental action", "Außergewöhnliche Einwirkung", "alert-triangle", EntityKind::Load),
    kind("seismic", "seismics", "Seismic action", "Erdbebeneinwirkung", "activity", EntityKind::Load),
    kind("bridgeSls", "bridgeSls", "Bridge SLS", "Brücken-GZG", "square", EntityKind::Plate),
    kind("fatigueSpectrum", "fatigueSpectra", "Fatigue spectrum bin", "Ermüdungskollektiv-Klasse", "bar-chart", EntityKind::Plate),
    kind("reliabilityAnalysis", "reliabilityAnalyses", "Reliability analysis", "Zuverlässigkeitsanalyse", "target", EntityKind::Plate),
    kind("testSeries", "testSeries", "Test series", "Versuchsreihe", "flask-conical", EntityKind::Plate),
    kind("factorOverride", "factorOverrides", "Factor override", "Faktorüberschreibung", "sliders-horizontal", EntityKind::Plate),
    kind("trafficGroup", "trafficGroups", "Traffic load group component", "Verkehrslastgruppen-Bestandteil", "route", EntityKind::Load),
    kind("combination", "combinations", "Combination selector", "Kombinationswahl", "list", EntityKind::Group),
    kind("case", "cases", "Design case", "Bemessungsfall", "clipboard-check", EntityKind::Solid),
    kind("effect", "effects", "Action effect", "Einwirkungsbeanspruchung", "link", EntityKind::Group),
    kind("placement", "placements", "Placement", "Platzierung", "map-pin", EntityKind::Group),
];

/// 🆔️ The id of the combination selector group entity.
pub const COMBINATION_ENTITY: &str = "combination";

/// 🗂️ The ids of every collection of the document, in outliner order; the single enumeration behind [`entity_kind_of`] and [`entities_of`].
fn rows_of(d: &model::En1990Snapshot) -> [(&'static str, Vec<&str>); 13] {
    [
        ("members", d.members.iter().map(|e| e.id.as_str()).collect()),
        ("permanents", d.permanents.iter().map(|e| e.id.as_str()).collect()),
        ("variables", d.variables.iter().map(|e| e.id.as_str()).collect()),
        ("accidentals", d.accidentals.iter().map(|e| e.id.as_str()).collect()),
        ("seismics", d.seismics.iter().map(|e| e.id.as_str()).collect()),
        ("bridgeSls", d.bridge_sls.iter().map(|e| e.id.as_str()).collect()),
        ("fatigueSpectra", d.fatigue_spectra.iter().map(|e| e.id.as_str()).collect()),
        ("reliabilityAnalyses", d.reliability_analyses.iter().map(|e| e.id.as_str()).collect()),
        ("testSeries", d.test_series.iter().map(|e| e.id.as_str()).collect()),
        ("factorOverrides", d.factor_overrides.iter().map(|e| e.id.as_str()).collect()),
        ("trafficGroups", d.traffic_groups.iter().map(|e| e.id.as_str()).collect()),
        ("combinations", vec![COMBINATION_ENTITY]),
        ("cases", d.cases.iter().filter(|e| case_declared(e)).map(|e| e.id.as_str()).collect()),
    ]
}

/// 🧭️ The registry row owning an entity id, searching every collection of the document.
pub fn entity_kind_of(document: &model::En1990Snapshot, id: &str) -> Option<&'static EntityKindSpec> {
    let base = base_id(id);
    rows_of(document).iter().find(|(_, ids)| ids.contains(&base)).and_then(|(collection, _)| ENTITY_KINDS.iter().find(|k| k.collection == *collection))
}

/// 🆔️ Every document entity in outliner order: the canonical order of the scene.
pub fn entities_of(document: &model::En1990Snapshot) -> Vec<EntityRef> {
    rows_of(document).iter().flat_map(|(collection, ids)| ENTITY_KINDS.iter().find(|k| k.collection == *collection).into_iter().flat_map(move |k| ids.iter().map(move |id| entity_ref(k.scene_kind, id, collection)))).collect()
}

/// 🔗️ The entities a projection of `id` reads besides itself, so a change of any of them re-projects `id`.
pub fn entity_dependencies(document: &model::En1990Snapshot, id: &str) -> Vec<String> {
    let base = base_id(id);
    let d = document;
    let mut out: BTreeSet<String> = BTreeSet::new();
    if d.members.iter().any(|m| m.id == base) {
        out.extend(d.effects.iter().filter(|e| e.member_id == base).map(|e| e.action_id.clone()));
        out.extend(d.bridge_sls.iter().filter(|s| s.member_id == base).map(|s| s.id.clone()));
        out.extend(d.fatigue_spectra.iter().filter(|s| s.member_id == base).map(|s| s.id.clone()));
        out.extend(d.reliability_analyses.iter().filter(|s| s.member_id == base).map(|s| s.id.clone()));
    }
    out.extend(d.reliability_analyses.iter().filter(|e| e.id == base && !e.member_id.is_empty()).map(|e| e.member_id.clone()));
    out.extend(d.fatigue_spectra.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    out.extend(d.bridge_sls.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    let is_action = d.permanents.iter().any(|a| a.id == base) || d.variables.iter().any(|a| a.id == base) || d.accidentals.iter().any(|a| a.id == base) || d.seismics.iter().any(|a| a.id == base);
    if let Some(row) = d.factor_overrides.iter().find(|o| o.id == base) {
        out.extend(model_actions(d));
        out.extend(inferences::factor_overrides::targets(d, row));
    }
    if let Some(row) = d.traffic_groups.iter().find(|r| r.id == base) {
        out.insert(row.variable_id.clone());
        out.extend(d.effects.iter().filter(|e| e.action_id == row.variable_id).map(|e| e.member_id.clone()));
    }
    out.extend(d.traffic_groups.iter().filter(|r| r.variable_id == base).map(|r| r.id.clone()));
    if is_action {
        out.extend(d.factor_overrides.iter().map(|o| o.id.clone()));
        out.extend(d.effects.iter().filter(|e| e.action_id == base).map(|e| e.member_id.clone()));
        out.extend(d.permanents.iter().map(|a| a.id.clone()).chain(d.variables.iter().map(|a| a.id.clone())).chain(d.accidentals.iter().map(|a| a.id.clone())).chain(d.seismics.iter().map(|a| a.id.clone())));
    }
    if base == COMBINATION_ENTITY {
        out.extend(d.members.iter().map(|m| m.id.clone()));
    }
    out.remove(base);
    out.into_iter().collect()
}

fn model_actions(d: &model::En1990Snapshot) -> impl Iterator<Item = String> + '_ {
    d.permanents.iter().map(|a| a.id.clone()).chain(d.variables.iter().map(|a| a.id.clone())).chain(d.accidentals.iter().map(|a| a.id.clone())).chain(d.seismics.iter().map(|a| a.id.clone()))
}

/// 🔗️ The member id plus every bridge SLS id that verifies the same member: the subjects whose checks colour the member.
pub fn dependents<'a>(d: &'a model::En1990Snapshot, member: &'a str) -> Vec<&'a str> {
    std::iter::once(member).chain(d.bridge_sls.iter().filter(|e| e.member_id == member).map(|e| e.id.as_str())).chain(d.fatigue_spectra.iter().filter(|e| e.member_id == member).take(1).map(|e| e.id.as_str())).collect()
}

/// 🧭️ The collection whose entity draws each check family that sits on a document row, longer prefixes first; families that sit on the project itself are in [`DOCUMENT_CHECKS`].
pub const CHECK_OWNERS: &[(&str, &str)] = &[
    ("en1990.annex-c.partial-factors.", "reliabilityAnalyses"),
    ("en1990.a2.traffic-group-combination.", "members"),
    ("en1990.a2.traffic-group-governing.", "members"),
    ("en1990.a2.rail-resonance.", "bridgeSls"),
    ("en1990.a2.rail-dynamic.", "bridgeSls"),
    ("en1990.a2.traffic-group.", "trafficGroups"),
    ("en1990.annex-c.alpha.", "reliabilityAnalyses"),
    ("en1990.annex-c.form.", "reliabilityAnalyses"),
    ("en1990.a2.transient.", "members"),
    ("en1990.e12.analysis.", "members"),
    ("en1990.e11.geometry.", "members"),
    ("en1990.e10.material.", "members"),
    ("en1990.fat.damage.", "fatigueSpectra"),
    ("en1990.a2.bridge.", "members"),
    ("en1990.a2.basis.", "bridgeSls"),
    ("en1990.a2.rail-", "bridgeSls"),
    ("en1990.annex-d.", "testSeries"),
    ("en1990.override.", "factorOverrides"),
    ("en1990.actions.", "members"),
    ("en1990.equ.", "members"),
    ("en1990.a14.", "members"),
    ("en1990.str.", "members"),
    ("en1990.geo.", "members"),
    ("en1990.sls.", "members"),
    ("en1990.fat.", "members"),
    ("en1990.6.", "members"),
];

/// 🧭️ The collection that owns the check `check_id` (longest family prefix), `None` for a check outside the owner table.
pub fn check_owner(check_id: &str) -> Option<&'static str> {
    CHECK_OWNERS.iter().find(|(prefix, _)| check_id.starts_with(prefix)).map(|(_, collection)| *collection)
}

/// 📄️ The check families that belong to the project as a whole (classification, working life, reliability class and target, reference integrity, input validity, design cases without geometry); they select the global badge.
pub const DOCUMENT_CHECKS: &[&str] = &[
    "en1990.members", "en1990.table-2.1.", "en1990.annex-b.", "en1990.annex-c.beta", "en1990.a2.working-life", "en1990.a2.traffic-groups", "en1990.input.", "en1990.integrity.", "en1990.e90.",
];

/// 🧭️ Where a check is drawn: on one document row, or on the global badge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckRoute {
    Entity(String),
    Document,
}

/// 🧭️ The document row that owns `check`: the row its subject names, else (for a subject that is the project or empty) the only row of the collection [`check_owner`] files the family under; `None` for a document-level check.
pub fn owner_entity(document: &model::En1990Snapshot, check: &norm::CheckResult) -> Option<String> {
    let subject = base_id(&check.subject.entity_id);
    if !subject.is_empty() && entity_kind_of(document, subject).is_some() {
        return Some(subject.to_owned());
    }
    if !subject.is_empty() && subject != document.project_id {
        return None;
    }
    let collection = check_owner(&check.id)?;
    let rows = rows_of(document).into_iter().find(|(name, _)| *name == collection)?.1;
    (rows.len() == 1).then(|| rows[0].to_owned())
}

/// 🧭️ The route of `check`: its owner row, else the document when it sits in a document-level family; `None` is a check no entity or badge draws, which the routing law forbids.
pub fn route_of(document: &model::En1990Snapshot, check: &norm::CheckResult) -> Option<CheckRoute> {
    owner_entity(document, check).map(CheckRoute::Entity).or_else(|| DOCUMENT_CHECKS.iter().any(|p| check.id.starts_with(p)).then_some(CheckRoute::Document))
}

/// 🎯️ The row to select and frame for the check `check_id` of `report`, resolved through [`owner_entity`].
pub fn focus_of_outcome(document: &model::En1990Snapshot, report: &norm::CheckReport, check_id: &str) -> Option<String> {
    projection_kit::check_with(report, check_id).and_then(|check| owner_entity(document, check))
}

/// 📶️ The governing binding among the checks owned by any of the rows `ids`.
pub fn binding(document: &model::En1990Snapshot, report: &norm::CheckReport, ids: &[&str]) -> Option<UtilisationBinding> {
    projection_kit::binding_where(report, |check| owner_entity(document, check).is_some_and(|owner| ids.contains(&owner.as_str())))
}

/// 🧱️ The section outline of a member: the depth follows the span (L/16, 0.2 to 1.6 m), the width is half of it.
pub fn member_depth(span: f64) -> f64 {
    (span / 16.0).clamp(0.2, 1.6)
}

pub fn section_shape(span: f64) -> SectionShape {
    let h = member_depth(span);
    SectionShape::parametric(ProfileKind::Rect, &[("h", h), ("b", h / 2.0)])
}

/// 🚦️ The tone of one check by id; neutral when the check is absent or not applicable.
pub fn check_tone(report: &norm::CheckReport, id: &str) -> Tone {
    report.checks.iter().find(|c| c.id == id).map_or(Tone::Neutral, |c| match c.status {
        norm::CheckStatus::Fail => Tone::Error,
        norm::CheckStatus::Warning | norm::CheckStatus::UnverifiedBasis => Tone::Warning,
        norm::CheckStatus::Pass => Tone::Info,
        norm::CheckStatus::NotApplicable => Tone::Neutral,
        norm::CheckStatus::Info => Tone::Info,
    })
}

/// 🧱️ Everything one scene entity needs before sealing; `ghost` draws an entity that does not take part in the active combination.
pub struct Spec {
    pub kind: EntityKind,
    pub id: String,
    pub collection: &'static str,
    pub geometry: Geometry,
    pub role: MaterialRole,
    pub binding: Option<UtilisationBinding>,
    pub name: LocalizedCopy,
    pub detail: LocalizedCopy,
    pub layer: &'static str,
    pub handles: Vec<String>,
    pub deformation: Option<DeformationOverlay>,
    pub ghost: bool,
}

impl Spec {
    pub fn build(self) -> SceneEntity {
        let (more_en, more_de) = utilisation_words(&self.binding);
        let failing = self.binding.as_ref().is_some_and(|b| b.status == scene::CheckStatus::Fail);
        SceneEntity {
            entity: entity_ref(self.kind, &self.id, self.collection),
            geometry: self.geometry,
            appearance: Appearance {
                role: self.role,
                channel: if self.binding.is_some() { ColourChannel::Utilisation } else { ColourChannel::Role },
                style: if self.ghost { StyleState::Ghost } else if failing { StyleState::Error } else { StyleState::Normal },
                opacity: if self.ghost { 0.35 } else { 1.0 },
                hatch: failing.then_some(Hatch::Diagonal),
                deformation: self.deformation,
            },
            utilisation: self.binding,
            label: Some(self.name.clone()),
            accessible_name: self.name,
            accessible_detail: LocalizedCopy::new(format!("{}{more_en}", self.detail.en), format!("{}{more_de}", self.detail.de)),
            layer: self.layer.to_string(),
            pick: PickPolicy::Whole,
            handles: self.handles,
            content_hash: String::new(),
        }
        .sealed()
    }
}

/// 📥️ Append-only scene accumulator of one projection part: entities in document order, owned annotations and tracks, and the points that bound the part.
#[derive(Default)]
pub struct Out {
    pub entities: Vec<SceneEntity>,
    pub annotations: Vec<Annotation>,
    pub tracks: Vec<ParameterTrack>,
    owners: Vec<Ownership>,
    points: Vec<Vec3>,
    scope: Scope,
    owner: Option<EntityRef>,
}

#[derive(Clone, Default)]
enum Scope {
    #[default]
    All,
    Entity(String),
    Global,
}

impl Out {
    /// 🎯️ An accumulator that keeps only what belongs to `part`: the live scene projects one document entity, or the global rest, at a time.
    pub fn scoped(part: Part<'_>) -> Self {
        let scope = match part {
            Part::All => Scope::All,
            Part::Entity(id) => Scope::Entity(id.to_owned()),
            Part::Global => Scope::Global,
        };
        Self { scope, ..Self::default() }
    }

    fn part(&self) -> Part<'_> {
        match &self.scope {
            Scope::All => Part::All,
            Scope::Entity(id) => Part::Entity(id),
            Scope::Global => Part::Global,
        }
    }

    /// 🎯️ Whether the entity `id` is part of this projection; modules test it before building an entity's geometry.
    pub fn wants(&self, id: &str) -> bool {
        self.part().wants(id)
    }

    /// 🔗️ From now on annotations and tracks belong to no entity.
    pub fn global(&mut self) {
        self.owner = None;
    }

    /// 📥️ Pushes an entity built outside the spec builder (the case forms of the projection kit); `collection` names the document collection that owns it.
    pub fn push_entity(&mut self, entity: SceneEntity, collection: &'static str, points: &[Vec3]) {
        self.owner = Some(entity_ref(EntityKind::Solid, base_id(&entity.entity.id), collection));
        if !self.wants(&entity.entity.id) {
            return;
        }
        self.points.extend_from_slice(points);
        self.entities.push(entity);
    }

    pub fn push(&mut self, spec: Spec, points: &[Vec3]) {
        self.owner = Some(entity_ref(spec.kind, base_id(&spec.id), spec.collection));
        if !self.wants(&spec.id) {
            return;
        }
        self.points.extend_from_slice(points);
        self.entities.push(spec.build());
    }

    /// 🌈️ Pushes a spec whose colour is the scalar field `active` of `fields` (one field per combination column), so the combination track recolours it without re-projecting.
    pub fn push_fields(&mut self, spec: Spec, points: &[Vec3], active: String, fields: Vec<ScalarField>) {
        self.owner = Some(entity_ref(spec.kind, base_id(&spec.id), spec.collection));
        if !self.wants(&spec.id) {
            return;
        }
        self.points.extend_from_slice(points);
        let mut entity = spec.build();
        entity.appearance.channel = ColourChannel::Fields { active, fields };
        self.entities.push(entity.sealed());
    }

    fn link(&mut self, kind: OwnedKind, id: &str) -> bool {
        if !self.part().keeps(self.owner.as_ref().map(|o| o.id.as_str())) {
            return false;
        }
        if let Some(owner) = &self.owner {
            self.owners.push(Ownership { kind, id: id.to_owned(), owner: owner.clone() });
        }
        true
    }

    pub fn annotate(&mut self, annotation: Annotation, anchor: &Vec3) {
        if self.link(OwnedKind::Annotation, annotation.id()) {
            self.points.push(anchor.clone());
            self.annotations.push(annotation);
        }
    }

    pub fn track(&mut self, track: ParameterTrack) {
        if self.link(OwnedKind::Track, &track.id) {
            self.tracks.push(track);
        }
    }

    pub fn into_fragment(self) -> Fragment {
        let bounds = Aabb::of_points(&self.points);
        Fragment { entities: self.entities, annotations: self.annotations, legends: Vec::new(), layers: Vec::new(), tracks: self.tracks, owners: self.owners, bounds }
    }
}

/// 🏁️ The global parts of the merged scene: used layers, the utilisation and role legends and the description.
pub fn finish(scene: &mut StructuralScene, description: LocalizedCopy) {
    let used = projection_kit::legends::used_layers(&scene.entities, &scene.annotations, LAYER_ANNOTATIONS);
    let roles = role_legend(&scene.entities);
    scene.layers = projection_kit::legends::layers(&LAYERS, &used, &[]);
    scene.legends = vec![
        projection_kit::legends::utilisation_legend(loc("Utilisation", "Ausnutzung")),
        Legend { id: "roles".into(), title: loc("Actions and members", "Einwirkungen und Bauteile"), scale: LegendScale::Roles { entries: roles } },
    ];
    scene.legends.extend(gamma_legend(&scene.entities));
    scene.description = description;
}

fn gamma_legend(entities: &[SceneEntity]) -> Option<Legend> {
    let fields = entities.iter().filter(|e| e.layer == LAYER_GAMMA).filter_map(|e| match &e.appearance.channel {
        ColourChannel::Fields { active, fields } => Some((active, fields)),
        _ => None,
    });
    let (mut lo, mut hi, mut first) = (f64::INFINITY, f64::NEG_INFINITY, None);
    for (active, fields) in fields {
        first.get_or_insert_with(|| active.clone());
        for field in fields {
            if let FieldSamples::Vertex { values } = &field.samples {
                values.iter().for_each(|v| (lo, hi) = (lo.min(*v), hi.max(*v)));
            }
        }
    }
    let field_id = first?;
    let (min, max) = if hi > lo { (lo, hi) } else { (lo.min(hi) - 0.5, lo.max(hi) + 0.5) };
    Some(Legend { id: "gamma".into(), title: loc("Permanent partial factor γ_G of the active combination", "Teilsicherheitsbeiwert γ_G der aktiven Kombination"), scale: LegendScale::Scalar { field_id, map: ColourMapRef { id: "viridis".into(), min, max }, unit: QuantityKind::Dimensionless } })
}

fn role_legend(entities: &[SceneEntity]) -> Vec<LegendRole> {
    projection_kit::legends::role_legend(entities, |role| {
        let label = match role {
            MaterialRole::Neutral => ("Members", "Bauteile"),
            MaterialRole::Concrete => ("Bridge decks", "Brückenfahrbahnen"),
            MaterialRole::Support => ("Supports", "Lager"),
            MaterialRole::Steel => ("Analyses", "Analysen"),
            MaterialRole::Load { kind } => match kind {
                LoadKind::Permanent => ("Permanent actions", "Ständige Einwirkungen"),
                LoadKind::Variable => ("Variable actions", "Veränderliche Einwirkungen"),
                LoadKind::Wind => ("Wind", "Wind"),
                LoadKind::Snow => ("Snow", "Schnee"),
                LoadKind::Thermal => ("Thermal actions", "Temperatureinwirkungen"),
                LoadKind::Prestress => ("Prestress", "Vorspannung"),
                LoadKind::Accidental => ("Accidental actions", "Außergewöhnliche Einwirkungen"),
                LoadKind::Seismic => ("Seismic actions", "Erdbebeneinwirkungen"),
                LoadKind::Other => ("Other actions", "Sonstige Einwirkungen"),
                },
            _ => return None,
        };
        let key = role.clone();
        Some((label, key))
    })
}

/// 🧊️ Whether the entry of `row` declares geometry, so the case stands in the scene.
pub fn case_declared(row: &model::CaseRow) -> bool {
    forms::declared::<En1990Cases>(row).is_some()
}
