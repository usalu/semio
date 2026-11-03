//! 🧱️ The wall evaluation of the EN 1996 masonry checks: the clause-by-clause compliance assessment of one wall. See [`super`].

use super::*;

#[path = "📏️wall-geometry-and-material/🦀️.rs"]
mod wall_geometry_and_material;
use wall_geometry_and_material::{wall_paths_and_basics, WallBasics, check_slenderness, check_material_conformance, check_unit_and_opening_fit};

#[path = "🏋️load-case-checks/🦀️.rs"]
mod load_case_checks;
use load_case_checks::check_load_cases;

#[path = "🔥️fire-durability-and-reinforcement/🦀️.rs"]
mod fire_durability_and_reinforcement;
use fire_durability_and_reinforcement::{check_fire, check_durability, check_reinforced_masonry, check_simplified_and_basement};

//#region ⚖️Evaluate
pub(super) fn evaluate_wall(annex: AnnexChoice, masonry_class: MasonryClass, default_situation: DesignSituation, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, storeys: u32, wall: &MasonryWall) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let WallBasics { path_t, path_support, path_fb, path_mortar, path_mortar_f, path_unit_h, lambda, a, t } = wall_paths_and_basics(wall);
    check_slenderness(annex, wall, &mut out, &path_t, &path_support, lambda, t);
    check_material_conformance(annex, wall, &mut out, &path_fb, &path_mortar_f, &path_unit_h);
    check_unit_and_opening_fit(annex, wall, &mut out);
    if wall.load_cases.is_empty() {
        out.push(
            CheckResult::assess(
                format!("en1996.loads.empty.{}", wall.id),
                "EN 1996-1-1",
                clause("EN 1996-1-1", "§2", "2"),
                wall_subject(wall, &wall_path(wall, "loadCases")),
                loc("Load cases present", "Lastfälle vorhanden"),
            )
            .not_applicable(loc("No load cases on wall.", "Keine Lastfälle an der Wand."))
            .annex(annex)
            .build(),
        );
        return out;
    }
    check_load_cases(annex, masonry_class, default_situation, reliability_class, wall, &mut out);
    check_fire(annex, masonry_class, default_situation, reliability_class, wall, &mut out);
    check_durability(annex, wall, &mut out, &path_mortar);
    check_reinforced_masonry(annex, wall, &mut out, &wall_path(wall, "asHorizontalM2"), a);
    check_simplified_and_basement(annex, masonry_class, default_situation, reliability_class, storeys, wall, &mut out);
    out.extend(document_case_results(annex, masonry_class, default_situation, reliability_class, storeys, wall));
    out
}
//#endregion
