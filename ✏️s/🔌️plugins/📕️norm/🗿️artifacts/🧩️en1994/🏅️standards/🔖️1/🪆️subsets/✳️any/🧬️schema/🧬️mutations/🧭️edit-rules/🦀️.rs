//! 🧭️ The `setField` / `insertItem` / `removeItem` / `applyRemedy` vocabulary of the En1994 editor: which value-tree path raises which concrete kind.

use semio_framework_value::{DslValue, ToValue};
use semio_s_plugin_norm_app_surface::{IdRemoval, InsertItemRule, NormEdit, NormEditRules, RemoveItemRule, SelectorKey, SetFieldRule, coerce_like, get_value_at_path};
use crate::En1994Mutation;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

/// 🪢 A row list whose rows are owned by the rows of another list through a key: removing the owner removes its rows in the same batch.
pub struct OwnedRows {
    pub owner: &'static str,
    pub rows: &'static str,
    pub key: &'static str,
}

/// 🪢 The stud rows belong to their beam (`beamId`), see `remove-beam`.
pub const OWNED_ROWS: &[OwnedRows] = &[OwnedRows { owner: "beams", rows: "studs", key: "beamId" }];

/// 📚 Every path a En1994 editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: NormEditRules = NormEditRules {
    move_item: &[],
    set_field: &[
        SetFieldRule { path: "beams[].transverseAsM2PerM", kind: "change-beam-transverse-as", selectors: &[SelectorKey::Id("id")], value: "newTransverseAsM2PerM" },
        SetFieldRule { path: "columns[].kind", kind: "change-column-kind", selectors: &[SelectorKey::Id("id")], value: "newKind" },
        SetFieldRule { path: "columns[].actions[].nKN", kind: "change-column-action-force-n", selectors: &[SelectorKey::Id("columnId"), SelectorKey::Id("id")], value: "newNKN" },
        SetFieldRule { path: "beams[].actions[].qAreaPa", kind: "change-beam-action-q-area-pa", selectors: &[SelectorKey::Id("beamId"), SelectorKey::Id("id")], value: "newQAreaPa" },
        SetFieldRule { path: "annex", kind: "change-annex", selectors: &[], value: "newAnnex" },
        SetFieldRule { path: "reliabilityClass", kind: "change-reliability-class", selectors: &[], value: "newReliabilityClass" },
        SetFieldRule { path: "steelFYPa", kind: "change-steel-fy-pa", selectors: &[], value: "newSteelFYPa" },
        SetFieldRule { path: "structureKind", kind: "change-structure-kind", selectors: &[], value: "newStructureKind" },
        SetFieldRule { path: "slabs[].concreteThicknessM", kind: "change-slab-thickness-m", selectors: &[SelectorKey::Id("id")], value: "newConcreteThicknessM" },
        SetFieldRule { path: "beams[].spanM", kind: "change-beam-span-m", selectors: &[SelectorKey::Id("id")], value: "newSpanM" },
        SetFieldRule { path: "slabs[].actions[].qAreaPa", kind: "change-slab-action-q-area-pa", selectors: &[SelectorKey::Id("slabId"), SelectorKey::Id("id")], value: "newQAreaPa" },
        SetFieldRule { path: "fatigueDetail", kind: "change-fatigue-detail", selectors: &[], value: "newFatigueDetail" },
        SetFieldRule { path: "fireRating", kind: "change-fire-rating", selectors: &[], value: "newFireRating" },
        SetFieldRule { path: "beams[].construction", kind: "change-beam-construction", selectors: &[SelectorKey::Id("id")], value: "newConstruction" },
        SetFieldRule { path: "insulationThicknessM", kind: "change-insulation-thickness-m", selectors: &[], value: "newInsulationThicknessM" },
        SetFieldRule { path: "beams[].slabThicknessM", kind: "change-beam-slab-thickness-m", selectors: &[SelectorKey::Id("id")], value: "newSlabThicknessM" },
        SetFieldRule { path: "beams[].spacingM", kind: "change-beam-spacing-m", selectors: &[SelectorKey::Id("id")], value: "newSpacingM" },
        SetFieldRule { path: "beams[].support", kind: "change-beam-support", selectors: &[SelectorKey::Id("id")], value: "newSupport" },
        SetFieldRule { path: "beams[].reinforcementProduct", kind: "change-beam-rebar-product", selectors: &[SelectorKey::Id("id")], value: "newReinforcementProduct" },
        SetFieldRule { path: "beams[].concreteFCkPa", kind: "change-beam-concrete-f-ck-pa", selectors: &[SelectorKey::Id("id")], value: "newConcreteFCkPa" },
        SetFieldRule { path: "beams[].concreteECmPa", kind: "change-beam-concrete-e-cm-pa", selectors: &[SelectorKey::Id("id")], value: "newConcreteECmPa" },
        SetFieldRule { path: "beams[].ltbLengthM", kind: "change-beam-ltb-length-m", selectors: &[SelectorKey::Id("id")], value: "newLtbLengthM" },
        SetFieldRule { path: "beams[].asHoggingM2PerM", kind: "change-beam-hogging-rebar-m2-per-m", selectors: &[SelectorKey::Id("id")], value: "newAsHoggingM2PerM" },
        SetFieldRule { path: "beams[].barSpacingM", kind: "change-beam-bar-spacing-m", selectors: &[SelectorKey::Id("id")], value: "newBarSpacingM" },
        SetFieldRule { path: "beams[].wkLimitM", kind: "change-beam-crack-width-limit-m", selectors: &[SelectorKey::Id("id")], value: "newWkLimitM" },
        SetFieldRule { path: "beams[].nCycles", kind: "change-beam-fatigue-cycles", selectors: &[SelectorKey::Id("id")], value: "newNCycles" },
        SetFieldRule { path: "beams[].steel.designation", kind: "change-beam-steel-designation", selectors: &[SelectorKey::Id("id")], value: "newDesignation" },
        SetFieldRule { path: "beams[].steel.heightM", kind: "change-beam-steel-height-m", selectors: &[SelectorKey::Id("id")], value: "newHeightM" },
        SetFieldRule { path: "beams[].steel.widthM", kind: "change-beam-steel-width-m", selectors: &[SelectorKey::Id("id")], value: "newWidthM" },
        SetFieldRule { path: "beams[].steel.twM", kind: "change-beam-steel-web-thickness-m", selectors: &[SelectorKey::Id("id")], value: "newTwM" },
        SetFieldRule { path: "beams[].steel.tfM", kind: "change-beam-steel-tf-m", selectors: &[SelectorKey::Id("id")], value: "newTfM" },
        SetFieldRule { path: "beams[].steel.aM2", kind: "change-beam-steel-area-m2", selectors: &[SelectorKey::Id("id")], value: "newAM2" },
        SetFieldRule { path: "beams[].steel.wPlYM3", kind: "change-beam-steel-w-pl-ym3", selectors: &[SelectorKey::Id("id")], value: "newWPlYM3" },
        SetFieldRule { path: "beams[].steel.iYM4", kind: "change-beam-steel-second-moment-m4", selectors: &[SelectorKey::Id("id")], value: "newIYM4" },
        SetFieldRule { path: "beams[].steel.aVM2", kind: "change-beam-steel-shear-area-m2", selectors: &[SelectorKey::Id("id")], value: "newAVM2" },
        SetFieldRule { path: "beams[].steel.rM", kind: "change-beam-steel-root-radius-m", selectors: &[SelectorKey::Id("id")], value: "newRM" },
        SetFieldRule { path: "beams[].sheeting.profile", kind: "change-beam-sheeting-profile", selectors: &[SelectorKey::Id("id")], value: "newProfile" },
        SetFieldRule { path: "beams[].sheeting.heightM", kind: "change-beam-sheeting-height-m", selectors: &[SelectorKey::Id("id")], value: "newHeightM" },
        SetFieldRule { path: "beams[].sheeting.ribWidthM", kind: "change-beam-sheeting-rib-width-m", selectors: &[SelectorKey::Id("id")], value: "newRibWidthM" },
        SetFieldRule { path: "beams[].sheeting.thicknessM", kind: "change-beam-sheeting-thickness-m", selectors: &[SelectorKey::Id("id")], value: "newThicknessM" },
        SetFieldRule { path: "beams[].sheeting.ribsParallelToBeam", kind: "change-beam-sheeting-ribs-parallel", selectors: &[SelectorKey::Id("id")], value: "newRibsParallelToBeam" },
        SetFieldRule { path: "beams[].actions[].kind", kind: "change-beam-action-kind", selectors: &[SelectorKey::Id("beamId"), SelectorKey::Id("id")], value: "newKind" },
        SetFieldRule { path: "beams[].actions[].category", kind: "change-beam-action-category", selectors: &[SelectorKey::Id("beamId"), SelectorKey::Id("id")], value: "newCategory" },
        SetFieldRule { path: "beams[].actions[].stage", kind: "change-beam-action-stage", selectors: &[SelectorKey::Id("beamId"), SelectorKey::Id("id")], value: "newStage" },
        SetFieldRule { path: "beams[].actions[].fKN", kind: "change-beam-action-point-force-n", selectors: &[SelectorKey::Id("beamId"), SelectorKey::Id("id")], value: "newFKN" },
        SetFieldRule { path: "beams[].actions[].deltaSigmaKPa", kind: "change-beam-action-delta-sigma-pa", selectors: &[SelectorKey::Id("beamId"), SelectorKey::Id("id")], value: "newDeltaSigmaKPa" },
        SetFieldRule { path: "beams[].actions[].deltaTauKPa", kind: "change-beam-action-delta-tau-pa", selectors: &[SelectorKey::Id("beamId"), SelectorKey::Id("id")], value: "newDeltaTauKPa" },
        SetFieldRule { path: "columns[].lengthM", kind: "change-column-length-m", selectors: &[SelectorKey::Id("id")], value: "newLengthM" },
        SetFieldRule { path: "columns[].outerSizeM", kind: "change-column-outer-size-m", selectors: &[SelectorKey::Id("id")], value: "newOuterSizeM" },
        SetFieldRule { path: "columns[].wallThicknessM", kind: "change-column-wall-thickness-m", selectors: &[SelectorKey::Id("id")], value: "newWallThicknessM" },
        SetFieldRule { path: "columns[].steelAM2", kind: "change-column-steel-area-m2", selectors: &[SelectorKey::Id("id")], value: "newSteelAM2" },
        SetFieldRule { path: "columns[].steelFYPa", kind: "change-column-steel-fy-pa", selectors: &[SelectorKey::Id("id")], value: "newSteelFYPa" },
        SetFieldRule { path: "columns[].concreteAM2", kind: "change-column-concrete-area-m2", selectors: &[SelectorKey::Id("id")], value: "newConcreteAM2" },
        SetFieldRule { path: "columns[].concreteFCkPa", kind: "change-column-concrete-f-ck-pa", selectors: &[SelectorKey::Id("id")], value: "newConcreteFCkPa" },
        SetFieldRule { path: "columns[].reinforcementAsM2", kind: "change-column-rebar-area-m2", selectors: &[SelectorKey::Id("id")], value: "newReinforcementAsM2" },
        SetFieldRule { path: "columns[].reinforcementFYkPa", kind: "change-column-rebar-f-yk-pa", selectors: &[SelectorKey::Id("id")], value: "newReinforcementFYkPa" },
        SetFieldRule { path: "columns[].iM4", kind: "change-column-second-moment-m4", selectors: &[SelectorKey::Id("id")], value: "newIM4" },
        SetFieldRule { path: "columns[].bucklingCurve", kind: "change-column-buckling-curve", selectors: &[SelectorKey::Id("id")], value: "newBucklingCurve" },
        SetFieldRule { path: "columns[].reinforcementProduct", kind: "change-column-rebar-product", selectors: &[SelectorKey::Id("id")], value: "newReinforcementProduct" },
        SetFieldRule { path: "columns[].actions[].kind", kind: "change-column-action-kind", selectors: &[SelectorKey::Id("columnId"), SelectorKey::Id("id")], value: "newKind" },
        SetFieldRule { path: "columns[].actions[].category", kind: "change-column-action-category", selectors: &[SelectorKey::Id("columnId"), SelectorKey::Id("id")], value: "newCategory" },
        SetFieldRule { path: "columns[].actions[].stage", kind: "change-column-action-stage", selectors: &[SelectorKey::Id("columnId"), SelectorKey::Id("id")], value: "newStage" },
        SetFieldRule { path: "columns[].actions[].mKNm", kind: "change-column-action-moment-nm", selectors: &[SelectorKey::Id("columnId"), SelectorKey::Id("id")], value: "newMKNm" },
        SetFieldRule { path: "slabs[].spanM", kind: "change-slab-span-m", selectors: &[SelectorKey::Id("id")], value: "newSpanM" },
        SetFieldRule { path: "slabs[].support", kind: "change-slab-support", selectors: &[SelectorKey::Id("id")], value: "newSupport" },
        SetFieldRule { path: "slabs[].fCkPa", kind: "change-slab-f-ck-pa", selectors: &[SelectorKey::Id("id")], value: "newFCkPa" },
        SetFieldRule { path: "slabs[].mFactor", kind: "change-slab-m-factor", selectors: &[SelectorKey::Id("id")], value: "newMFactor" },
        SetFieldRule { path: "slabs[].kFactor", kind: "change-slab-k-factor", selectors: &[SelectorKey::Id("id")], value: "newKFactor" },
        SetFieldRule { path: "slabs[].asM2PerM", kind: "change-slab-reinforcement-m2-per-m", selectors: &[SelectorKey::Id("id")], value: "newAsM2PerM" },
        SetFieldRule { path: "slabs[].sheeting.profile", kind: "change-slab-sheeting-profile", selectors: &[SelectorKey::Id("id")], value: "newProfile" },
        SetFieldRule { path: "slabs[].sheeting.heightM", kind: "change-slab-sheeting-height-m", selectors: &[SelectorKey::Id("id")], value: "newHeightM" },
        SetFieldRule { path: "slabs[].sheeting.ribWidthM", kind: "change-slab-sheeting-rib-width-m", selectors: &[SelectorKey::Id("id")], value: "newRibWidthM" },
        SetFieldRule { path: "slabs[].sheeting.thicknessM", kind: "change-slab-sheeting-thickness-m", selectors: &[SelectorKey::Id("id")], value: "newThicknessM" },
        SetFieldRule { path: "slabs[].sheeting.ribsParallelToBeam", kind: "change-slab-sheeting-ribs-parallel", selectors: &[SelectorKey::Id("id")], value: "newRibsParallelToBeam" },
        SetFieldRule { path: "slabs[].actions[].kind", kind: "change-slab-action-kind", selectors: &[SelectorKey::Id("slabId"), SelectorKey::Id("id")], value: "newKind" },
        SetFieldRule { path: "slabs[].actions[].category", kind: "change-slab-action-category", selectors: &[SelectorKey::Id("slabId"), SelectorKey::Id("id")], value: "newCategory" },
        SetFieldRule { path: "slabs[].actions[].stage", kind: "change-slab-action-stage", selectors: &[SelectorKey::Id("slabId"), SelectorKey::Id("id")], value: "newStage" },
        SetFieldRule { path: "slabs[].actions[].fKN", kind: "change-slab-action-point-force-n", selectors: &[SelectorKey::Id("slabId"), SelectorKey::Id("id")], value: "newFKN" },
        SetFieldRule { path: "slabs[].actions[].deltaSigmaKPa", kind: "change-slab-action-delta-sigma-pa", selectors: &[SelectorKey::Id("slabId"), SelectorKey::Id("id")], value: "newDeltaSigmaKPa" },
        SetFieldRule { path: "slabs[].actions[].deltaTauKPa", kind: "change-slab-action-delta-tau-pa", selectors: &[SelectorKey::Id("slabId"), SelectorKey::Id("id")], value: "newDeltaTauKPa" },
        SetFieldRule { path: "beams[].adjacentSpanM", kind: "change-beam-adjacent-span-m", selectors: &[SelectorKey::Id("id")], value: "newAdjacentSpanM" },
        SetFieldRule { path: "beams[].patchBearingLengthM", kind: "change-beam-patch-bearing-length-m", selectors: &[SelectorKey::Id("id")], value: "newPatchBearingLengthM" },
        SetFieldRule { path: "beams[].bridgeLanes", kind: "change-beam-bridge-lanes", selectors: &[SelectorKey::Id("id")], value: "newBridgeLanes" },
        SetFieldRule { path: "columns[].introducedForceN", kind: "change-column-introduced-force-n", selectors: &[SelectorKey::Id("id")], value: "newIntroducedForceN" },
        SetFieldRule { path: "beams[].crackedLengthFraction", kind: "change-beam-cracked-length-fraction", selectors: &[SelectorKey::Id("id")], value: "newCrackedLengthFraction" },
        SetFieldRule { path: "columns[].sectionHeightM", kind: "change-column-section-height-m", selectors: &[SelectorKey::Id("id")], value: "newSectionHeightM" },
        SetFieldRule { path: "columns[].sectionWidthM", kind: "change-column-section-width-m", selectors: &[SelectorKey::Id("id")], value: "newSectionWidthM" },
        SetFieldRule { path: "columns[].sectionTwM", kind: "change-column-section-tw-m", selectors: &[SelectorKey::Id("id")], value: "newSectionTwM" },
        SetFieldRule { path: "columns[].sectionTfM", kind: "change-column-section-tf-m", selectors: &[SelectorKey::Id("id")], value: "newSectionTfM" },
        SetFieldRule { path: "columns[].rebarCoverM", kind: "change-column-rebar-cover-m", selectors: &[SelectorKey::Id("id")], value: "newRebarCoverM" },
        SetFieldRule { path: "placements[].x", kind: "change-placement-x", selectors: &[SelectorKey::Id("id")], value: "newX" },
        SetFieldRule { path: "placements[].y", kind: "change-placement-y", selectors: &[SelectorKey::Id("id")], value: "newY" },
        SetFieldRule { path: "placements[].z", kind: "change-placement-z", selectors: &[SelectorKey::Id("id")], value: "newZ" },
        SetFieldRule { path: "placements[].yaw", kind: "change-placement-yaw", selectors: &[SelectorKey::Id("id")], value: "newYaw" },
        SetFieldRule { path: "beams[].studTransverseSpacingM", kind: "change-beam-stud-cross-spacing-m", selectors: &[SelectorKey::Id("id")], value: "newStudTransverseSpacingM" },
        SetFieldRule { path: "beams[].studEdgeDistanceM", kind: "change-beam-stud-edge-distance-m", selectors: &[SelectorKey::Id("id")], value: "newStudEdgeDistanceM" },
        SetFieldRule { path: "beams[].transverseAsTopM2PerM", kind: "change-beam-transverse-as-top", selectors: &[SelectorKey::Id("id")], value: "newTransverseAsTopM2PerM" },
        SetFieldRule { path: "beams[].endPlateWidthM", kind: "change-beam-end-plate-width-m", selectors: &[SelectorKey::Id("id")], value: "newEndPlateWidthM" },
        SetFieldRule { path: "beams[].endPlateHeightM", kind: "change-beam-end-plate-height-m", selectors: &[SelectorKey::Id("id")], value: "newEndPlateHeightM" },
        SetFieldRule { path: "beams[].endZoneLengthM", kind: "change-beam-end-zone-length-m", selectors: &[SelectorKey::Id("id")], value: "newEndZoneLengthM" },
        SetFieldRule { path: "studs[].fUPa", kind: "change-stud-fu-pa", selectors: &[SelectorKey::Id("id")], value: "newFUPa" },
        SetFieldRule { path: "studs[].heightM", kind: "change-stud-height-m", selectors: &[SelectorKey::Id("id")], value: "newHeightM" },
        SetFieldRule { path: "studs[].diameterM", kind: "change-stud-diameter-m", selectors: &[SelectorKey::Id("id")], value: "newDiameterM" },
        SetFieldRule { path: "studs[].xM", kind: "change-stud-xm", selectors: &[SelectorKey::Id("id")], value: "newXM" },
        SetFieldRule { path: "beams[].actions[].xM", kind: "change-beam-action-xm", selectors: &[SelectorKey::Id("beamId"), SelectorKey::Id("id")], value: "newXM" },
        SetFieldRule { path: "slabs[].actions[].xM", kind: "change-slab-action-xm", selectors: &[SelectorKey::Id("slabId"), SelectorKey::Id("id")], value: "newXM" },
        SetFieldRule { path: "slabs[].actions[].yM", kind: "change-slab-action-ym", selectors: &[SelectorKey::Id("slabId"), SelectorKey::Id("id")], value: "newYM" },
        SetFieldRule { path: "cases[].labelEn", kind: "change-case-label-en", selectors: &[SelectorKey::Id("caseId")], value: "newValue" },
        SetFieldRule { path: "cases[].labelDe", kind: "change-case-label-de", selectors: &[SelectorKey::Id("caseId")], value: "newValue" },
        SetFieldRule { path: "cases[].entryId", kind: "change-case-entry-id", selectors: &[SelectorKey::Id("caseId")], value: "newValue" },
        SetFieldRule { path: "cases[].numbers[].value", kind: "change-case-number", selectors: &[SelectorKey::Id("caseId"), SelectorKey::Id("key")], value: "value" },
        SetFieldRule { path: "cases[].texts[].value", kind: "change-case-text", selectors: &[SelectorKey::Id("caseId"), SelectorKey::Id("key")], value: "value" },
    ],
    insert_item: &[
        InsertItemRule { path: "slabs", kind: "insert-slab", selectors: &[], index: "index", item: "slab" },
        InsertItemRule { path: "beams", kind: "insert-beam", selectors: &[], index: "index", item: "beam" },
        InsertItemRule { path: "columns", kind: "insert-column", selectors: &[], index: "index", item: "column" },
        InsertItemRule { path: "placements", kind: "insert-placement", selectors: &[], index: "index", item: "item" },
        InsertItemRule { path: "studs", kind: "insert-stud", selectors: &[], index: "index", item: "item" },
        InsertItemRule { path: "beams[].actions", kind: "insert-beam-action", selectors: &[SelectorKey::Id("beamId")], index: "index", item: "action" },
        InsertItemRule { path: "columns[].actions", kind: "insert-column-action", selectors: &[SelectorKey::Id("columnId")], index: "index", item: "action" },
        InsertItemRule { path: "slabs[].actions", kind: "insert-slab-action", selectors: &[SelectorKey::Id("slabId")], index: "index", item: "action" },
        InsertItemRule { path: "cases", kind: "insert-case", selectors: &[], index: "index", item: "case" },
    ],
    remove_item: &[
    ],
};
/// 🆔️ Every collection of this family whose remove kind names the row by its id, with the parent selectors of the payload.
pub const ID_REMOVALS: &[IdRemoval] = &[
        IdRemoval { path: "beams[].actions", kind: "remove-beam-action", selectors: &[SelectorKey::Id("beamId")], id: "id" },
        IdRemoval { path: "columns[].actions", kind: "remove-column-action", selectors: &[SelectorKey::Id("columnId")], id: "id" },
        IdRemoval { path: "slabs[].actions", kind: "remove-slab-action", selectors: &[SelectorKey::Id("slabId")], id: "id" },
        IdRemoval { path: "columns", kind: "remove-column", selectors: &[], id: "id" },
        IdRemoval { path: "slabs", kind: "remove-slab", selectors: &[], id: "id" },
        IdRemoval { path: "beams", kind: "remove-beam", selectors: &[], id: "id" },
        IdRemoval { path: "placements", kind: "remove-placement", selectors: &[], id: "id" },
        IdRemoval { path: "studs", kind: "remove-stud", selectors: &[], id: "id" },
        IdRemoval { path: "cases", kind: "remove-case", selectors: &[], id: "id" },
];

const CATALOGUE: [&str; 11] = ["HEB300", "HEB320", "HEB340", "HEB360", "HEB400", "HEB450", "HEB500", "HEB550", "HEB600", "HEB650", "HEB700"];

fn catalogue_designation(height_m: f64) -> Option<&'static str> {
    CATALOGUE.iter().copied().filter_map(|designation| semio_s_artifact_norm_en1994_model::SteelSection::from_catalogue(designation).map(|section| (designation, section.height_m))).min_by(|a, b| (a.1 - height_m).abs().total_cmp(&(b.1 - height_m).abs())).map(|(designation, _)| designation)
}

/// 🔀️ Conforms a handle or panel write to the leaf it addresses: a number written to a flag becomes a flag, a count is rounded, and a number written to `steel.designation` is the catalogue pick by section height.
fn conform(document: &En1994Snapshot, path: &str, value: &DslValue) -> DslValue {
    let tree = document.to_value();
    match (get_value_at_path(&tree, path), value) {
        (Ok(DslValue::Bool(_)), DslValue::Number(number)) => DslValue::Bool(number.as_f64() >= 0.5),
        (Ok(DslValue::String(_)), DslValue::Number(number)) if path.ends_with(".steel.designation") => catalogue_designation(number.as_f64()).map_or_else(|| value.clone(), |designation| DslValue::String(designation.to_string())),
        (Ok(current), _) => coerce_like(current, value),
        _ => value.clone(),
    }
}

/// 🎯️ Resolves an editor edit into the concrete per-leaf kind mutations it denotes through [`EDIT_RULES`]; every editable leaf owns one kind and a nested action removal is raised by [`ID_REMOVALS`].
pub fn resolve_edit(document: &En1994Snapshot, edit: &NormEdit) -> Result<Vec<En1994Mutation>, String> {
    match edit {
        NormEdit::SetField { path, value } => EDIT_RULES.resolve(document, &NormEdit::SetField { path: path.clone(), value: conform(document, path, value) }),
        _ => EDIT_RULES.resolve_with_id_removals(ID_REMOVALS, document, edit),
    }
}
