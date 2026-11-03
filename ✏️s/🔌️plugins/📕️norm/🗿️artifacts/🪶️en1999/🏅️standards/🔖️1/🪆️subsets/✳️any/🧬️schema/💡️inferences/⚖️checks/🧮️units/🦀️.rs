//! 🧮️ EN 1999 check units in exactly the order of `evaluate_structure`: integrity, the action-field rows, then per collection its empty verdict and its rows; the heavy rows (fire, shells) are tier T1.

use super::{evaluate_scoped, Scope};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, CostTier, Rows, UnitSource};
use semio_s_plugin_norm_compliance::CheckResult;

/// 🧮️ The unit source of the EN 1999 aluminium subject: `SharedEvaluation<En1999Units>` is the one evaluation path of editor and viewer.
pub struct En1999Units;

type Rowing = Rows<En1999Snapshot>;

fn run(scope: Scope<'static>) -> impl Fn(&En1999Snapshot) -> Vec<CheckResult> + Send + Sync + 'static {
    move |d| evaluate_scoped(d, scope).checks
}

fn rows<T>(mut rows: Rowing, items: &[T], tag: &'static str, id: impl Fn(&T) -> &str, tier: CostTier, extra: &[&str]) -> Rowing {
    for (index, item) in items.iter().enumerate() {
        rows = rows.row(tag, collection_of(tag), index, id(item), tier, extra, move |d: &En1999Snapshot| evaluate_scoped(d, Scope::Row(tag, index)).checks);
    }
    rows
}

/// 📖️ What the integrity unit reads: the annex, the id column of every collection (duplicate ids), the collections the reference verdicts walk (sections, materials, placements, outline vertices) and the section and material references of the members; a dimension or load edit of a member leaves it clean.
const INTEGRITY_READS: [&str; 19] = [
    "annex",
    "materials",
    "sections",
    "placements",
    "outlineVertices",
    "members.*.id",
    "members.*.sectionId",
    "members.*.materialId",
    "connections.*.id",
    "fireScenarios.*.id",
    "fatigueDetails.*.id",
    "fatigueCategories.*.id",
    "patchLoads.*.id",
    "ringStiffeners.*.id",
    "sheetProfiles.*.id",
    "transverseStiffeners.*.id",
    "coldFormed.*.id",
    "shells.*.id",
    "weldRuns.*.id",
];

fn collection_of(tag: &str) -> &'static str {
    match tag {
        "memberActions" | "members" => "members",
        "connectionActions" | "connections" => "connections",
        "coldFormedActions" | "coldFormed" => "coldFormed",
        "shellActions" | "shells" => "shells",
        "fireScenarios" => "fireScenarios",
        "weldRuns" => "weldRuns",
        "fatigueCategories" => "fatigueCategories",
        "patchLoads" => "patchLoads",
        "ringStiffeners" => "ringStiffeners",
        "sheetProfiles" => "sheetProfiles",
        "fireInsulations" => "fireInsulations",
        "transverseStiffeners" => "transverseStiffeners",
        "sectionOutlines" => "sections",
        _ => "fatigueDetails",
    }
}

impl UnitSource for En1999Units {
    type Document = En1999Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = semio_s_plugin_norm_compliance::Owners::new(&["members", "connections", "coldFormed", "shells", "fireScenarios", "fatigueDetails", "weldRuns", "fatigueCategories", "patchLoads", "sheetProfiles", "fireInsulations", "ringStiffeners", "transverseStiffeners", "cases"], &["en1999.1-1.empty", "en1999.8.empty", "en1999.1-2.empty", "en1999.1-3.empty", "en1999.1-4.empty", "en1999.1-5.empty", "en1999.ref.", "en1999.6.1.outline.", "en1999.integrity."]);

    fn units(d: &En1999Snapshot) -> Vec<CheckUnit<En1999Snapshot>> {
        let part = |rows: Rowing, name: &'static str, collection: &str| rows.part_reading(name, &["annex", &format!("{collection}.*.id")], run(Scope::Part(name)));
        let mut r = Rows::new(["annex", "reliabilityClass"]).global_reading(&INTEGRITY_READS, run(Scope::Part("integrity")));
        r = rows(r, &d.members, "memberActions", |e| e.id.as_str(), CostTier::T0, &[]);
        r = rows(r, &d.connections, "connectionActions", |e| e.id.as_str(), CostTier::T0, &[]);
        r = rows(r, &d.cold_formed, "coldFormedActions", |e| e.id.as_str(), CostTier::T0, &[]);
        r = rows(r, &d.shells, "shellActions", |e| e.id.as_str(), CostTier::T0, &[]);
        r = part(r, "members-empty", "members");
        r = rows(r, &d.members, "members", |e| e.id.as_str(), CostTier::T1, &["sections", "materials", "weldRuns", "outlineVertices", "transverseStiffeners"]);
        r = part(r, "connections-empty", "connections");
        r = rows(r, &d.connections, "connections", |e| e.id.as_str(), CostTier::T1, &["materials", "members"]);
        r = part(r, "fire-empty", "fireScenarios");
        r = rows(r, &d.fire_scenarios, "fireScenarios", |e| e.id.as_str(), CostTier::T1, &["members", "sections", "materials", "fireInsulations"]);
        r = part(r, "fatigue-empty", "fatigueDetails");
        r = rows(r, &d.fatigue_details, "fatigueDetails", |e| e.id.as_str(), CostTier::T0, &["members", "fatigueDetails", "fatigueCategories"]);
        r = rows(r, &d.fatigue_categories, "fatigueCategories", |e| e.id.as_str(), CostTier::T0, &["members", "fatigueDetails"]);
        r = rows(r, &d.patch_loads, "patchLoads", |e| e.id.as_str(), CostTier::T0, &["members", "sections", "materials", "coldFormed", "transverseStiffeners"]);
        r = rows(r, &d.fire_insulations, "fireInsulations", |e| e.id.as_str(), CostTier::T0, &["fireScenarios", "fireInsulations"]);
        r = rows(r, &d.sheet_profiles, "sheetProfiles", |e| e.id.as_str(), CostTier::T0, &["coldFormed", "sheetProfiles"]);
        r = rows(r, &d.ring_stiffeners, "ringStiffeners", |e| e.id.as_str(), CostTier::T0, &["shells", "ringStiffeners"]);
        r = rows(r, &d.transverse_stiffeners, "transverseStiffeners", |e| e.id.as_str(), CostTier::T0, &["members", "sections", "materials", "transverseStiffeners"]);
        r = part(r, "coldFormed-empty", "coldFormed");
        r = rows(r, &d.cold_formed, "coldFormed", |e| e.id.as_str(), CostTier::T0, &["materials", "sheetProfiles"]);
        r = part(r, "shells-empty", "shells");
        r = rows(r, &d.shells, "shells", |e| e.id.as_str(), CostTier::T1, &["materials", "ringStiffeners"]);
        r = rows(r, &d.weld_runs, "weldRuns", |e| e.id.as_str(), CostTier::T0, &["members", "sections"]);
        r = rows(r, &d.sections, "sectionOutlines", |e| e.id.as_str(), CostTier::T0, &["outlineVertices"]);
        r.finish()
    }
}
