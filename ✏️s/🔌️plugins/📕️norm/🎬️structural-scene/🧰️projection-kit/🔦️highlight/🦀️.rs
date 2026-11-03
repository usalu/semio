//! 🔦️ Highlight: the owner of a check is its entity. Every fragment colours the entity of each owner by the worst check filed under it, whichever module drew the entity, and the checks filed under none (the documented document-level checks) become one global badge label. The audit behind the per-artifact ownership law lives here too.

use super::{base_id, binding_of_check, status as scene_status};
use crate::projection::{Fragment, Part, SceneBuilder, SITE_ENTITY};
use crate::*;
use norm::{CheckReport, CheckResult};
use semio_framework_value::ToValue;
use std::collections::{BTreeMap, BTreeSet};

/// 🏷️ The id of the global badge label that sums up the document-level checks.
pub const GLOBAL_BADGE: &str = "global-checks";

fn severity(status: CheckStatus) -> u8 {
    match status {
        CheckStatus::Fail => 3,
        CheckStatus::Warning => 2,
        CheckStatus::Pass => 1,
        CheckStatus::NotApplicable => 0,
    }
}

fn rank(check: &CheckResult) -> (u8, f64) {
    (severity(scene_status(check.status)), check.utilization)
}

fn governs(known: &CheckResult, check: &CheckResult) -> bool {
    rank(known).0.cmp(&rank(check).0).then(rank(known).1.total_cmp(&rank(check).1)).is_lt()
}

fn counts(check: &CheckResult) -> bool {
    !matches!(check.status, norm::CheckStatus::NotApplicable | norm::CheckStatus::Info) && check.utilization.is_finite()
}

/// 🌡️ The governing check filed under each owner in one pass: the most severe, then the highest utilisation; informative and not applicable rows never govern.
pub fn worst_by_owner(report: &CheckReport) -> BTreeMap<&str, &CheckResult> {
    let mut worst: BTreeMap<&str, &CheckResult> = BTreeMap::new();
    for check in report.checks.iter().filter(|c| !c.subject.entity_id.is_empty() && counts(c)) {
        if worst.get(check.subject.entity_id.as_str()).is_none_or(|known| governs(known, check)) {
            worst.insert(check.subject.entity_id.as_str(), check);
        }
    }
    worst
}

/// 🌡️ The governing check among the document-level checks, filed under no entity.
pub fn worst_global(report: &CheckReport) -> Option<&CheckResult> {
    report.checks.iter().filter(|c| c.subject.entity_id.is_empty() && counts(c)).fold(None, |worst, check| if worst.is_none_or(|known| governs(known, check)) { Some(check) } else { worst })
}

fn colour(entity: &mut SceneEntity, governing: &CheckResult) {
    let binding = binding_of_check(governing);
    if entity.utilisation.as_ref().is_none_or(|known| (severity(known.status), known.ratio) < (severity(binding.status), binding.ratio)) {
        entity.utilisation = Some(binding);
        entity.appearance.channel = ColourChannel::Utilisation;
        entity.content_hash = entity.compute_hash();
    }
}

/// 🏞️ The reference of the project site entity.
pub fn site_ref() -> EntityRef {
    EntityRef::new(EntityKind::Plate, SITE_ENTITY, "project")
}

/// 🏞️ `fragment` led by the project site: the domain-neutral ground plate of every artifact that stands for the project and owns the document-level checks, coloured by the worst of them.
pub fn with_site(fragment: Fragment, report: &CheckReport) -> Fragment {
    let half = 3.0;
    let outline = Polygon2::solid(vec![Vec2::new(-half, -half), Vec2::new(half, -half), Vec2::new(half, half), Vec2::new(-half, half)]);
    let mut builder = SceneBuilder::new();
    builder
        .plate(&site_ref(), PlateGeometry { frame: Frame::WORLD, outline, thickness: 0.05, offset: -0.05, stiffeners: Vec::new() })
        .named("Project site", "Projektstandort")
        .detail("Ground plate that stands for the project and carries its document-level checks", "Bodenplatte als Stellvertreter des Projekts mit seinen Dokument-Prüfungen")
        .labelled("Project", "Projekt")
        .layer("site", "Site", "Standort")
        .opacity(0.35)
        .add();
    builder.absorb(fragment);
    let mut led = builder.into_fragment();
    if let Some(governing) = worst_global(report) {
        colour(&mut led.entities[0], governing);
    }
    led
}

/// 🔦️ `fragment` with the main entity of every owner coloured by its worst check (the entity whose id is the owner, else the first entity of that owner), never softening a stronger binding a module already set; the whole-run and global fragments also carry the project site and the global badge of the checks filed under no entity.
pub fn highlight_owners(mut fragment: Fragment, report: &CheckReport, part: Part<'_>) -> Fragment {
    let owners: BTreeSet<String> = fragment.entities.iter().map(|e| base_id(&e.entity.id).to_owned()).collect();
    let worst = worst_by_owner(report);
    for owner in &owners {
        let Some(governing) = worst.get(owner.as_str()) else { continue };
        let Some(index) = fragment.entities.iter().position(|e| e.entity.id == *owner).or_else(|| fragment.entities.iter().position(|e| base_id(&e.entity.id) == owner)) else { continue };
        colour(&mut fragment.entities[index], governing);
    }
    if matches!(part, Part::All | Part::Global) {
        fragment.annotations.extend(global_badge(report));
        if report.checks.iter().any(|c| c.subject.entity_id.is_empty() && c.status != norm::CheckStatus::NotApplicable) {
            fragment = with_site(fragment, report);
        }
    }
    fragment
}

/// 🌐️ The label that sums up the checks filed under no entity: their number, the worst utilisation and the tone of the most severe status; `None` when there is none.
pub fn global_badge(report: &CheckReport) -> Option<Annotation> {
    let rows: Vec<&CheckResult> = report.checks.iter().filter(|c| c.subject.entity_id.is_empty() && c.status != norm::CheckStatus::NotApplicable).collect();
    if rows.is_empty() {
        return None;
    }
    let worst = rows.iter().filter(|c| c.status != norm::CheckStatus::Info && c.utilization.is_finite()).map(|c| if c.status == norm::CheckStatus::Fail { c.utilization.max(1.0) } else { c.utilization }).fold(0.0, f64::max);
    let tone = match rows.iter().map(|c| severity(scene_status(c.status))).max() {
        Some(3) => Tone::Error,
        Some(2) => Tone::Warning,
        _ => Tone::Info,
    };
    let count = rows.len();
    Some(Annotation::Label {
        id: GLOBAL_BADGE.into(),
        anchor: Vec3::ZERO,
        text: LocalizedCopy::new(format!("{count} document-level checks, worst {worst:.2}"), format!("{count} Dokument-Prüfungen, größte Ausnutzung {worst:.2}")),
        tone,
    })
}

/// 🧪️ The outcome of [`ownership_audit`]: the checks without a valid owner, the failing checks whose entity is not coloured, the documented document-level checks and whether their badge and their owner, the project site, stand in the scene.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OwnershipAudit {
    pub orphans: Vec<String>,
    pub unlit: Vec<String>,
    pub global: Vec<String>,
    pub badge: bool,
    pub site: bool,
}

impl OwnershipAudit {
    /// ✅️ No orphan, no failing check without colour, and the badge stands whenever there is a document-level check.
    pub fn is_clean(&self) -> bool {
        self.orphans.is_empty() && self.unlit.is_empty() && (self.global.is_empty() || (self.badge && self.site))
    }
}

/// 🧪️ The valid owners of a check in `document`: the base ids of the scene `entities` and the ids of its design cases.
pub fn audit_owners<S: ToValue>(document: &S, entities: &[EntityRef]) -> BTreeSet<String> {
    let cases = norm::Owners::new(&["cases"], &[]).owner_ids(&document.to_value());
    entities.iter().map(|e| base_id(&e.id).to_owned()).chain(cases).collect()
}

/// 🧪️ Audits `report` (already filed under its owners) against the scene drawn from it: every check names an owner in `owners` (the scene entities and the design cases) or is a documented document-level check (`document_level`); a failing check of a scene entity colours that entity with a failing binding; the document-level checks show in the global badge and are owned by the project site, which a failing one colours.
pub fn ownership_audit(report: &CheckReport, scene: &StructuralScene, owners: &BTreeSet<String>, document_level: impl Fn(&str) -> bool) -> OwnershipAudit {
    let drawn: BTreeSet<&str> = scene.entities.iter().map(|e| base_id(&e.entity.id)).collect();
    let lit = |owner: &str| scene.entities.iter().any(|e| base_id(&e.entity.id) == owner && e.utilisation.as_ref().is_some_and(|u| u.status == CheckStatus::Fail));
    let mut audit = OwnershipAudit { badge: scene.annotations.iter().any(|a| a.id() == GLOBAL_BADGE), site: drawn.contains(SITE_ENTITY), ..OwnershipAudit::default() };
    for check in &report.checks {
        let owner = check.subject.entity_id.as_str();
        if owner.is_empty() && document_level(&check.id) {
            audit.global.push(check.id.clone());
            if check.status == norm::CheckStatus::Fail && audit.site && !lit(SITE_ENTITY) {
                audit.unlit.push(check.id.clone());
            }
        } else if !owners.contains(owner) {
            audit.orphans.push(check.id.clone());
        } else if check.status == norm::CheckStatus::Fail && drawn.contains(owner) && !lit(owner) {
            audit.unlit.push(check.id.clone());
        }
    }
    audit
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
