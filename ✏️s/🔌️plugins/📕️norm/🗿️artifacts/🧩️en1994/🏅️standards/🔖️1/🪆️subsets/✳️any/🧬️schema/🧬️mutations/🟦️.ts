/** 🧺️ `En1994Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireExternal, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ChangeBeamAdjacentSpanM, parseChangeBeamAdjacentSpanM } from "./↔️change-beam-adjacent-span-m/🧬️schema/🟦️.ts";
import { type ChangeBeamSpacingM, parseChangeBeamSpacingM } from "./↔️change-beam-spacing-m/🧬️schema/🟦️.ts";
import { type ChangeBeamSteelWidthM, parseChangeBeamSteelWidthM } from "./↔️change-beam-steel-width-m/🧬️schema/🟦️.ts";
import { type ChangeBeamStudCrossSpacingM, parseChangeBeamStudCrossSpacingM } from "./↔️change-beam-stud-cross-spacing-m/🧬️schema/🟦️.ts";
import { type ChangeBeamTransverseAs, parseChangeBeamTransverseAs } from "./↔️change-beam-transverse-as/🧬️schema/🟦️.ts";
import { type ChangeColumnSectionWidthM, parseChangeColumnSectionWidthM } from "./↔️change-column-section-width-m/🧬️schema/🟦️.ts";
import { type ChangeBeamEndPlateHeightM, parseChangeBeamEndPlateHeightM } from "./↕️change-beam-end-plate-height-m/🧬️schema/🟦️.ts";
import { type ChangeBeamSteelHeightM, parseChangeBeamSteelHeightM } from "./↕️change-beam-steel-height-m/🧬️schema/🟦️.ts";
import { type ChangeColumnSectionHeightM, parseChangeColumnSectionHeightM } from "./↕️change-column-section-height-m/🧬️schema/🟦️.ts";
import { type ChangeColumnKind, parseChangeColumnKind } from "./↪️change-column-kind/🧬️schema/🟦️.ts";
import { type ChangeColumnSectionTfM, parseChangeColumnSectionTfM } from "./▬change-column-section-tf-m/🧬️schema/🟦️.ts";
import { type ChangeColumnSectionTwM, parseChangeColumnSectionTwM } from "./▮change-column-section-tw-m/🧬️schema/🟦️.ts";
import { type RemoveColumn, parseRemoveColumn } from "./⛔️remove-column/🧬️schema/🟦️.ts";
import { type ChangeBeamSteelShearAreaM2, parseChangeBeamSteelShearAreaM2 } from "./✂️change-beam-steel-shear-area-m2/🧬️schema/🟦️.ts";
import { type InsertSlab, parseInsertSlab } from "./➕insert-slab/🧬️schema/🟦️.ts";
import { type InsertBeam, parseInsertBeam } from "./➕️insert-beam/🧬️schema/🟦️.ts";
import { type InsertCase, parseInsertCase } from "./➕️insert-case/🧬️schema/🟦️.ts";
import { type RemoveSlab, parseRemoveSlab } from "./➖remove-slab/🧬️schema/🟦️.ts";
import { type RemoveBeam, parseRemoveBeam } from "./➖️remove-beam/🧬️schema/🟦️.ts";
import { type RemoveCase, parseRemoveCase } from "./➖️remove-case/🧬️schema/🟦️.ts";
import { type InsertColumn, parseInsertColumn } from "./➗️insert-column/🧬️schema/🟦️.ts";
import { type ChangeBeamEndZoneLengthM, parseChangeBeamEndZoneLengthM } from "./➡️change-beam-end-zone-length-m/🧬️schema/🟦️.ts";
import { type ChangeStudDiameterM, parseChangeStudDiameterM } from "./➡️change-stud-diameter-m/🧬️schema/🟦️.ts";
import { type ChangeStudFUPa, parseChangeStudFUPa } from "./➡️change-stud-fu-pa/🧬️schema/🟦️.ts";
import { type ChangeStudHeightM, parseChangeStudHeightM } from "./➡️change-stud-height-m/🧬️schema/🟦️.ts";
import { type ChangeStudXM, parseChangeStudXM } from "./➡️change-stud-xm/🧬️schema/🟦️.ts";
import { type ChangeColumnActionForceN, parseChangeColumnActionForceN } from "./⬇️change-column-action-force-n/🧬️schema/🟦️.ts";
import { type ChangeColumnIntroducedForceN, parseChangeColumnIntroducedForceN } from "./⬇️change-column-introduced-force-n/🧬️schema/🟦️.ts";
import { type ChangeBeamSteelAreaM2, parseChangeBeamSteelAreaM2 } from "./⬛️change-beam-steel-area-m2/🧬️schema/🟦️.ts";
import { type ChangeColumnSteelAreaM2, parseChangeColumnSteelAreaM2 } from "./⬛️change-column-steel-area-m2/🧬️schema/🟦️.ts";
import { type ChangeBeamEndPlateWidthM, parseChangeBeamEndPlateWidthM } from "./⬜️change-beam-end-plate-width-m/🧬️schema/🟦️.ts";
import { type ChangeColumnOuterSizeM, parseChangeColumnOuterSizeM } from "./⬜️change-column-outer-size-m/🧬️schema/🟦️.ts";
import { type ChangeBeamSteelRootRadiusM, parseChangeBeamSteelRootRadiusM } from "./⭕️change-beam-steel-root-radius-m/🧬️schema/🟦️.ts";
import { type ChangeBeamCrackedLengthFraction, parseChangeBeamCrackedLengthFraction } from "./〰️change-beam-cracked-length-fraction/🧬️schema/🟦️.ts";
import { type ChangeBeamActionQAreaPa, parseChangeBeamActionQAreaPa } from "./🌀️change-beam-action-q-area-pa/🧬️schema/🟦️.ts";
import { type ChangeBeamLtbLengthM, parseChangeBeamLtbLengthM } from "./🌀️change-beam-ltb-length-m/🧬️schema/🟦️.ts";
import { type ChangeAnnex, parseChangeAnnex } from "./🌍️change-annex/🧬️schema/🟦️.ts";
import { type ChangeReliabilityClass, parseChangeReliabilityClass } from "./🎯change-reliability-class/🧬️schema/🟦️.ts";
import { type ChangeBeamActionKind, parseChangeBeamActionKind } from "./🏋️change-beam-action-kind/🧬️schema/🟦️.ts";
import { type ChangeBeamActionXM, parseChangeBeamActionXM } from "./🏋️change-beam-action-xm/🧬️schema/🟦️.ts";
import { type ChangeColumnActionKind, parseChangeColumnActionKind } from "./🏋️change-column-action-kind/🧬️schema/🟦️.ts";
import { type ChangeSlabActionKind, parseChangeSlabActionKind } from "./🏋️change-slab-action-kind/🧬️schema/🟦️.ts";
import { type ChangeSteelFYPa, parseChangeSteelFYPa } from "./🏋️change-steel-fy-pa/🧬️schema/🟦️.ts";
import { type InsertBeamAction, parseInsertBeamAction } from "./🏋️insert-beam-action/🧬️schema/🟦️.ts";
import { type RemoveBeamAction, parseRemoveBeamAction } from "./🏋️remove-beam-action/🧬️schema/🟦️.ts";
import { type ChangeBeamSheetingHeightM, parseChangeBeamSheetingHeightM } from "./🏗️change-beam-sheeting-height-m/🧬️schema/🟦️.ts";
import { type ChangeBeamSheetingProfile, parseChangeBeamSheetingProfile } from "./🏗️change-beam-sheeting-profile/🧬️schema/🟦️.ts";
import { type ChangeBeamSheetingRibWidthM, parseChangeBeamSheetingRibWidthM } from "./🏗️change-beam-sheeting-rib-width-m/🧬️schema/🟦️.ts";
import { type ChangeBeamSheetingThicknessM, parseChangeBeamSheetingThicknessM } from "./🏗️change-beam-sheeting-thickness-m/🧬️schema/🟦️.ts";
import { type ChangeSlabSheetingHeightM, parseChangeSlabSheetingHeightM } from "./🏗️change-slab-sheeting-height-m/🧬️schema/🟦️.ts";
import { type ChangeSlabSheetingProfile, parseChangeSlabSheetingProfile } from "./🏗️change-slab-sheeting-profile/🧬️schema/🟦️.ts";
import { type ChangeSlabSheetingRibWidthM, parseChangeSlabSheetingRibWidthM } from "./🏗️change-slab-sheeting-rib-width-m/🧬️schema/🟦️.ts";
import { type ChangeSlabSheetingThicknessM, parseChangeSlabSheetingThicknessM } from "./🏗️change-slab-sheeting-thickness-m/🧬️schema/🟦️.ts";
import { type ChangeStructureKind, parseChangeStructureKind } from "./🏗️change-structure-kind/🧬️schema/🟦️.ts";
import { type InsertColumnAction, parseInsertColumnAction } from "./🏛️insert-column-action/🧬️schema/🟦️.ts";
import { type RemoveColumnAction, parseRemoveColumnAction } from "./🏛️remove-column-action/🧬️schema/🟦️.ts";
import { type ChangeBeamActionCategory, parseChangeBeamActionCategory } from "./🏷️change-beam-action-category/🧬️schema/🟦️.ts";
import { type ChangeCaseLabelDe, parseChangeCaseLabelDe } from "./🏷️change-case-label-de/🧬️schema/🟦️.ts";
import { type ChangeCaseLabelEn, parseChangeCaseLabelEn } from "./🏷️change-case-label-en/🧬️schema/🟦️.ts";
import { type ChangeColumnActionCategory, parseChangeColumnActionCategory } from "./🏷️change-column-action-category/🧬️schema/🟦️.ts";
import { type ChangeSlabActionCategory, parseChangeSlabActionCategory } from "./🏷️change-slab-action-category/🧬️schema/🟦️.ts";
import { type ChangeCaseEntryId, parseChangeCaseEntryId } from "./📇️change-case-entry-id/🧬️schema/🟦️.ts";
import { type ChangeColumnBucklingCurve, parseChangeColumnBucklingCurve } from "./📉️change-column-buckling-curve/🧬️schema/🟦️.ts";
import { type ChangeBeamActionPointForceN, parseChangeBeamActionPointForceN } from "./📍️change-beam-action-point-force-n/🧬️schema/🟦️.ts";
import { type ChangeBeamPatchBearingLengthM, parseChangeBeamPatchBearingLengthM } from "./📍️change-beam-patch-bearing-length-m/🧬️schema/🟦️.ts";
import { type ChangePlacementX, parseChangePlacementX } from "./📍️change-placement-x/🧬️schema/🟦️.ts";
import { type ChangePlacementY, parseChangePlacementY } from "./📍️change-placement-y/🧬️schema/🟦️.ts";
import { type ChangePlacementYaw, parseChangePlacementYaw } from "./📍️change-placement-yaw/🧬️schema/🟦️.ts";
import { type ChangePlacementZ, parseChangePlacementZ } from "./📍️change-placement-z/🧬️schema/🟦️.ts";
import { type ChangeSlabActionPointForceN, parseChangeSlabActionPointForceN } from "./📍️change-slab-action-point-force-n/🧬️schema/🟦️.ts";
import { type InsertPlacement, parseInsertPlacement } from "./📍️insert-placement/🧬️schema/🟦️.ts";
import { type InsertStud, parseInsertStud } from "./📍️insert-stud/🧬️schema/🟦️.ts";
import { type RemovePlacement, parseRemovePlacement } from "./📍️remove-placement/🧬️schema/🟦️.ts";
import { type RemoveStud, parseRemoveStud } from "./📍️remove-stud/🧬️schema/🟦️.ts";
import { type ChangeSlabThicknessM, parseChangeSlabThicknessM } from "./📏change-slab-thickness-m/🧬️schema/🟦️.ts";
import { type ChangeBeamBarSpacingM, parseChangeBeamBarSpacingM } from "./📏️change-beam-bar-spacing-m/🧬️schema/🟦️.ts";
import { type ChangeBeamSpanM, parseChangeBeamSpanM } from "./📏️change-beam-span-m/🧬️schema/🟦️.ts";
import { type ChangeBeamStudEdgeDistanceM, parseChangeBeamStudEdgeDistanceM } from "./📏️change-beam-stud-edge-distance-m/🧬️schema/🟦️.ts";
import { type ChangeColumnLengthM, parseChangeColumnLengthM } from "./📏️change-column-length-m/🧬️schema/🟦️.ts";
import { type ChangeSlabSpanM, parseChangeSlabSpanM } from "./📏️change-slab-span-m/🧬️schema/🟦️.ts";
import { type ChangeBeamSteelSecondMomentM4, parseChangeBeamSteelSecondMomentM4 } from "./📐️change-beam-steel-second-moment-m4/🧬️schema/🟦️.ts";
import { type ChangeBeamSteelWPlYM3, parseChangeBeamSteelWPlYM3 } from "./📐️change-beam-steel-w-pl-ym3/🧬️schema/🟦️.ts";
import { type ChangeColumnSecondMomentM4, parseChangeColumnSecondMomentM4 } from "./📐️change-column-second-moment-m4/🧬️schema/🟦️.ts";
import { type ChangeSlabActionQAreaPa, parseChangeSlabActionQAreaPa } from "./📐️change-slab-action-q-area-pa/🧬️schema/🟦️.ts";
import { type ChangeBeamActionDeltaSigmaPa, parseChangeBeamActionDeltaSigmaPa } from "./🔁️change-beam-action-delta-sigma-pa/🧬️schema/🟦️.ts";
import { type ChangeBeamActionDeltaTauPa, parseChangeBeamActionDeltaTauPa } from "./🔁️change-beam-action-delta-tau-pa/🧬️schema/🟦️.ts";
import { type ChangeBeamFatigueCycles, parseChangeBeamFatigueCycles } from "./🔁️change-beam-fatigue-cycles/🧬️schema/🟦️.ts";
import { type ChangeFatigueDetail, parseChangeFatigueDetail } from "./🔁️change-fatigue-detail/🧬️schema/🟦️.ts";
import { type ChangeSlabActionDeltaSigmaPa, parseChangeSlabActionDeltaSigmaPa } from "./🔁️change-slab-action-delta-sigma-pa/🧬️schema/🟦️.ts";
import { type ChangeSlabActionDeltaTauPa, parseChangeSlabActionDeltaTauPa } from "./🔁️change-slab-action-delta-tau-pa/🧬️schema/🟦️.ts";
import { type ChangeColumnActionMomentNm, parseChangeColumnActionMomentNm } from "./🔄️change-column-action-moment-nm/🧬️schema/🟦️.ts";
import { type ChangeColumnRebarCoverM, parseChangeColumnRebarCoverM } from "./🔘change-column-rebar-cover-m/🧬️schema/🟦️.ts";
import { type ChangeCaseNumber, parseChangeCaseNumber } from "./🔢️change-case-number/🧬️schema/🟦️.ts";
import { type ChangeCaseText, parseChangeCaseText } from "./🔤️change-case-text/🧬️schema/🟦️.ts";
import { type ChangeFireRating, parseChangeFireRating } from "./🔥️change-fire-rating/🧬️schema/🟦️.ts";
import { type ChangeBeamSteelDesignation, parseChangeBeamSteelDesignation } from "./🔩️change-beam-steel-designation/🧬️schema/🟦️.ts";
import { type ChangeColumnSteelFYPa, parseChangeColumnSteelFYPa } from "./🔩️change-column-steel-fy-pa/🧬️schema/🟦️.ts";
import { type ChangeBeamHoggingRebarM2PerM, parseChangeBeamHoggingRebarM2PerM } from "./🕸️change-beam-hogging-rebar-m2-per-m/🧬️schema/🟦️.ts";
import { type ChangeBeamTransverseAsTop, parseChangeBeamTransverseAsTop } from "./🕸️change-beam-transverse-as-top/🧬️schema/🟦️.ts";
import { type ChangeColumnRebarAreaM2, parseChangeColumnRebarAreaM2 } from "./🕸️change-column-rebar-area-m2/🧬️schema/🟦️.ts";
import { type ChangeColumnRebarFYkPa, parseChangeColumnRebarFYkPa } from "./🕸️change-column-rebar-f-yk-pa/🧬️schema/🟦️.ts";
import { type ChangeSlabReinforcementM2PerM, parseChangeSlabReinforcementM2PerM } from "./🕸️change-slab-reinforcement-m2-per-m/🧬️schema/🟦️.ts";
import { type ChangeBeamConstruction, parseChangeBeamConstruction } from "./🛠️change-beam-construction/🧬️schema/🟦️.ts";
import { type ChangeBeamBridgeLanes, parseChangeBeamBridgeLanes } from "./🛣️change-beam-bridge-lanes/🧬️schema/🟦️.ts";
import { type ChangeBeamCrackWidthLimitM, parseChangeBeamCrackWidthLimitM } from "./🧬️change-beam-crack-width-limit-m/🧬️schema/🟦️.ts";
import { type ChangeBeamActionStage, parseChangeBeamActionStage } from "./🧭️change-beam-action-stage/🧬️schema/🟦️.ts";
import { type ChangeBeamSheetingRibsParallel, parseChangeBeamSheetingRibsParallel } from "./🧭️change-beam-sheeting-ribs-parallel/🧬️schema/🟦️.ts";
import { type ChangeColumnActionStage, parseChangeColumnActionStage } from "./🧭️change-column-action-stage/🧬️schema/🟦️.ts";
import { type ChangeSlabActionStage, parseChangeSlabActionStage } from "./🧭️change-slab-action-stage/🧬️schema/🟦️.ts";
import { type ChangeSlabSheetingRibsParallel, parseChangeSlabSheetingRibsParallel } from "./🧭️change-slab-sheeting-ribs-parallel/🧬️schema/🟦️.ts";
import { type ChangeSlabKFactor, parseChangeSlabKFactor } from "./🧮️change-slab-k-factor/🧬️schema/🟦️.ts";
import { type ChangeSlabMFactor, parseChangeSlabMFactor } from "./🧮️change-slab-m-factor/🧬️schema/🟦️.ts";
import { type ChangeInsulationThicknessM, parseChangeInsulationThicknessM } from "./🧯️change-insulation-thickness-m/🧬️schema/🟦️.ts";
import { type ChangeBeamSlabThicknessM, parseChangeBeamSlabThicknessM } from "./🧱change-beam-slab-thickness-m/🧬️schema/🟦️.ts";
import { type ChangeBeamConcreteECmPa, parseChangeBeamConcreteECmPa } from "./🧱️change-beam-concrete-e-cm-pa/🧬️schema/🟦️.ts";
import { type ChangeBeamSteelTfM, parseChangeBeamSteelTfM } from "./🧱️change-beam-steel-tf-m/🧬️schema/🟦️.ts";
import { type ChangeBeamSteelWebThicknessM, parseChangeBeamSteelWebThicknessM } from "./🧱️change-beam-steel-web-thickness-m/🧬️schema/🟦️.ts";
import { type ChangeColumnWallThicknessM, parseChangeColumnWallThicknessM } from "./🧱️change-column-wall-thickness-m/🧬️schema/🟦️.ts";
import { type ChangeSlabActionXM, parseChangeSlabActionXM } from "./🧱️change-slab-action-xm/🧬️schema/🟦️.ts";
import { type ChangeSlabActionYM, parseChangeSlabActionYM } from "./🧱️change-slab-action-ym/🧬️schema/🟦️.ts";
import { type InsertSlabAction, parseInsertSlabAction } from "./🧱️insert-slab-action/🧬️schema/🟦️.ts";
import { type RemoveSlabAction, parseRemoveSlabAction } from "./🧱️remove-slab-action/🧬️schema/🟦️.ts";
import { type ChangeBeamRebarProduct, parseChangeBeamRebarProduct } from "./🧵️change-beam-rebar-product/🧬️schema/🟦️.ts";
import { type ChangeColumnRebarProduct, parseChangeColumnRebarProduct } from "./🧵️change-column-rebar-product/🧬️schema/🟦️.ts";
import { type ChangeBeamSupport, parseChangeBeamSupport } from "./🪢️change-beam-support/🧬️schema/🟦️.ts";
import { type ChangeSlabSupport, parseChangeSlabSupport } from "./🪢️change-slab-support/🧬️schema/🟦️.ts";
import { type ChangeBeamConcreteFCkPa, parseChangeBeamConcreteFCkPa } from "./🪨️change-beam-concrete-f-ck-pa/🧬️schema/🟦️.ts";
import { type ChangeColumnConcreteAreaM2, parseChangeColumnConcreteAreaM2 } from "./🪨️change-column-concrete-area-m2/🧬️schema/🟦️.ts";
import { type ChangeColumnConcreteFCkPa, parseChangeColumnConcreteFCkPa } from "./🪨️change-column-concrete-f-ck-pa/🧬️schema/🟦️.ts";
import { type ChangeSlabFCkPa, parseChangeSlabFCkPa } from "./🪨️change-slab-f-ck-pa/🧬️schema/🟦️.ts";

export type En1994Mutation =
  | { ChangeAnnex: ChangeAnnex }
  | { ChangeReliabilityClass: ChangeReliabilityClass }
  | { ChangeStructureKind: ChangeStructureKind }
  | { ChangeSteelFYPa: ChangeSteelFYPa }
  | { ChangeFireRating: ChangeFireRating }
  | { ChangeInsulationThicknessM: ChangeInsulationThicknessM }
  | { ChangeFatigueDetail: ChangeFatigueDetail }
  | { InsertBeam: InsertBeam }
  | { RemoveBeam: RemoveBeam }
  | { ChangeBeamActionQAreaPa: ChangeBeamActionQAreaPa }
  | { ChangeBeamSpanM: ChangeBeamSpanM }
  | { ChangeBeamSlabThicknessM: ChangeBeamSlabThicknessM }
  | { ChangeBeamTransverseAs: ChangeBeamTransverseAs }
  | { ChangeBeamConstruction: ChangeBeamConstruction }
  | { InsertColumn: InsertColumn }
  | { RemoveColumn: RemoveColumn }
  | { ChangeColumnActionForceN: ChangeColumnActionForceN }
  | { ChangeColumnKind: ChangeColumnKind }
  | { InsertSlab: InsertSlab }
  | { RemoveSlab: RemoveSlab }
  | { ChangeSlabActionQAreaPa: ChangeSlabActionQAreaPa }
  | { ChangeSlabThicknessM: ChangeSlabThicknessM }
  | { ChangeBeamSpacingM: ChangeBeamSpacingM }
  | { ChangeBeamSupport: ChangeBeamSupport }
  | { ChangeBeamRebarProduct: ChangeBeamRebarProduct }
  | { ChangeBeamConcreteFCkPa: ChangeBeamConcreteFCkPa }
  | { ChangeBeamConcreteECmPa: ChangeBeamConcreteECmPa }
  | { ChangeBeamLtbLengthM: ChangeBeamLtbLengthM }
  | { ChangeBeamHoggingRebarM2PerM: ChangeBeamHoggingRebarM2PerM }
  | { ChangeBeamBarSpacingM: ChangeBeamBarSpacingM }
  | { ChangeBeamCrackWidthLimitM: ChangeBeamCrackWidthLimitM }
  | { ChangeBeamFatigueCycles: ChangeBeamFatigueCycles }
  | { ChangeBeamSteelDesignation: ChangeBeamSteelDesignation }
  | { ChangeBeamSteelHeightM: ChangeBeamSteelHeightM }
  | { ChangeBeamSteelWidthM: ChangeBeamSteelWidthM }
  | { ChangeBeamSteelWebThicknessM: ChangeBeamSteelWebThicknessM }
  | { ChangeBeamSteelTfM: ChangeBeamSteelTfM }
  | { ChangeBeamSteelAreaM2: ChangeBeamSteelAreaM2 }
  | { ChangeBeamSteelWPlYM3: ChangeBeamSteelWPlYM3 }
  | { ChangeBeamSteelSecondMomentM4: ChangeBeamSteelSecondMomentM4 }
  | { ChangeBeamSteelShearAreaM2: ChangeBeamSteelShearAreaM2 }
  | { ChangeBeamSteelRootRadiusM: ChangeBeamSteelRootRadiusM }
  | { ChangeBeamSheetingProfile: ChangeBeamSheetingProfile }
  | { ChangeBeamSheetingHeightM: ChangeBeamSheetingHeightM }
  | { ChangeBeamSheetingRibWidthM: ChangeBeamSheetingRibWidthM }
  | { ChangeBeamSheetingThicknessM: ChangeBeamSheetingThicknessM }
  | { ChangeBeamSheetingRibsParallel: ChangeBeamSheetingRibsParallel }
  | { ChangeBeamActionKind: ChangeBeamActionKind }
  | { ChangeBeamActionCategory: ChangeBeamActionCategory }
  | { ChangeBeamActionStage: ChangeBeamActionStage }
  | { ChangeBeamActionPointForceN: ChangeBeamActionPointForceN }
  | { ChangeBeamActionDeltaSigmaPa: ChangeBeamActionDeltaSigmaPa }
  | { ChangeBeamActionDeltaTauPa: ChangeBeamActionDeltaTauPa }
  | { ChangeColumnLengthM: ChangeColumnLengthM }
  | { ChangeColumnOuterSizeM: ChangeColumnOuterSizeM }
  | { ChangeColumnWallThicknessM: ChangeColumnWallThicknessM }
  | { ChangeColumnSteelAreaM2: ChangeColumnSteelAreaM2 }
  | { ChangeColumnSteelFYPa: ChangeColumnSteelFYPa }
  | { ChangeColumnConcreteAreaM2: ChangeColumnConcreteAreaM2 }
  | { ChangeColumnConcreteFCkPa: ChangeColumnConcreteFCkPa }
  | { ChangeColumnRebarAreaM2: ChangeColumnRebarAreaM2 }
  | { ChangeColumnRebarFYkPa: ChangeColumnRebarFYkPa }
  | { ChangeColumnSecondMomentM4: ChangeColumnSecondMomentM4 }
  | { ChangeColumnBucklingCurve: ChangeColumnBucklingCurve }
  | { ChangeColumnRebarProduct: ChangeColumnRebarProduct }
  | { ChangeColumnActionKind: ChangeColumnActionKind }
  | { ChangeColumnActionCategory: ChangeColumnActionCategory }
  | { ChangeColumnActionStage: ChangeColumnActionStage }
  | { ChangeColumnActionMomentNm: ChangeColumnActionMomentNm }
  | { ChangeSlabSpanM: ChangeSlabSpanM }
  | { ChangeSlabSupport: ChangeSlabSupport }
  | { ChangeSlabFCkPa: ChangeSlabFCkPa }
  | { ChangeSlabMFactor: ChangeSlabMFactor }
  | { ChangeSlabKFactor: ChangeSlabKFactor }
  | { ChangeSlabReinforcementM2PerM: ChangeSlabReinforcementM2PerM }
  | { ChangeSlabSheetingProfile: ChangeSlabSheetingProfile }
  | { ChangeSlabSheetingHeightM: ChangeSlabSheetingHeightM }
  | { ChangeSlabSheetingRibWidthM: ChangeSlabSheetingRibWidthM }
  | { ChangeSlabSheetingThicknessM: ChangeSlabSheetingThicknessM }
  | { ChangeSlabSheetingRibsParallel: ChangeSlabSheetingRibsParallel }
  | { ChangeSlabActionKind: ChangeSlabActionKind }
  | { ChangeSlabActionCategory: ChangeSlabActionCategory }
  | { ChangeSlabActionStage: ChangeSlabActionStage }
  | { ChangeSlabActionPointForceN: ChangeSlabActionPointForceN }
  | { ChangeSlabActionDeltaSigmaPa: ChangeSlabActionDeltaSigmaPa }
  | { ChangeSlabActionDeltaTauPa: ChangeSlabActionDeltaTauPa }
  | { ChangeBeamAdjacentSpanM: ChangeBeamAdjacentSpanM }
  | { ChangeBeamPatchBearingLengthM: ChangeBeamPatchBearingLengthM }
  | { ChangeBeamBridgeLanes: ChangeBeamBridgeLanes }
  | { ChangeColumnIntroducedForceN: ChangeColumnIntroducedForceN }
  | { ChangeBeamCrackedLengthFraction: ChangeBeamCrackedLengthFraction }
  | { ChangeColumnSectionHeightM: ChangeColumnSectionHeightM }
  | { ChangeColumnSectionWidthM: ChangeColumnSectionWidthM }
  | { ChangeColumnSectionTwM: ChangeColumnSectionTwM }
  | { ChangeColumnSectionTfM: ChangeColumnSectionTfM }
  | { ChangeColumnRebarCoverM: ChangeColumnRebarCoverM }
  | { ChangePlacementX: ChangePlacementX }
  | { ChangePlacementY: ChangePlacementY }
  | { ChangePlacementZ: ChangePlacementZ }
  | { ChangePlacementYaw: ChangePlacementYaw }
  | { InsertPlacement: InsertPlacement }
  | { RemovePlacement: RemovePlacement }
  | { ChangeBeamStudCrossSpacingM: ChangeBeamStudCrossSpacingM }
  | { ChangeBeamStudEdgeDistanceM: ChangeBeamStudEdgeDistanceM }
  | { ChangeBeamTransverseAsTop: ChangeBeamTransverseAsTop }
  | { ChangeBeamEndPlateWidthM: ChangeBeamEndPlateWidthM }
  | { ChangeBeamEndPlateHeightM: ChangeBeamEndPlateHeightM }
  | { ChangeBeamEndZoneLengthM: ChangeBeamEndZoneLengthM }
  | { InsertBeamAction: InsertBeamAction }
  | { RemoveBeamAction: RemoveBeamAction }
  | { InsertColumnAction: InsertColumnAction }
  | { RemoveColumnAction: RemoveColumnAction }
  | { InsertSlabAction: InsertSlabAction }
  | { RemoveSlabAction: RemoveSlabAction }
  | { ChangeBeamActionXM: ChangeBeamActionXM }
  | { ChangeSlabActionXM: ChangeSlabActionXM }
  | { ChangeSlabActionYM: ChangeSlabActionYM }
  | { InsertCase: InsertCase }
  | { RemoveCase: RemoveCase }
  | { ChangeCaseNumber: ChangeCaseNumber }
  | { ChangeCaseText: ChangeCaseText }
  | { ChangeCaseLabelEn: ChangeCaseLabelEn }
  | { ChangeCaseLabelDe: ChangeCaseLabelDe }
  | { ChangeCaseEntryId: ChangeCaseEntryId }
  | { ChangeStudDiameterM: ChangeStudDiameterM }
  | { ChangeStudFUPa: ChangeStudFUPa }
  | { ChangeStudHeightM: ChangeStudHeightM }
  | { ChangeStudXM: ChangeStudXM }
  | { InsertStud: InsertStud }
  | { RemoveStud: RemoveStud };

export const parseEn1994Mutation: NormWireReader<En1994Mutation> = normWireExternal<En1994Mutation>({
  ChangeAnnex: parseChangeAnnex,
  ChangeReliabilityClass: parseChangeReliabilityClass,
  ChangeStructureKind: parseChangeStructureKind,
  ChangeSteelFYPa: parseChangeSteelFYPa,
  ChangeFireRating: parseChangeFireRating,
  ChangeInsulationThicknessM: parseChangeInsulationThicknessM,
  ChangeFatigueDetail: parseChangeFatigueDetail,
  InsertBeam: parseInsertBeam,
  RemoveBeam: parseRemoveBeam,
  ChangeBeamActionQAreaPa: parseChangeBeamActionQAreaPa,
  ChangeBeamSpanM: parseChangeBeamSpanM,
  ChangeBeamSlabThicknessM: parseChangeBeamSlabThicknessM,
  ChangeBeamTransverseAs: parseChangeBeamTransverseAs,
  ChangeBeamConstruction: parseChangeBeamConstruction,
  InsertColumn: parseInsertColumn,
  RemoveColumn: parseRemoveColumn,
  ChangeColumnActionForceN: parseChangeColumnActionForceN,
  ChangeColumnKind: parseChangeColumnKind,
  InsertSlab: parseInsertSlab,
  RemoveSlab: parseRemoveSlab,
  ChangeSlabActionQAreaPa: parseChangeSlabActionQAreaPa,
  ChangeSlabThicknessM: parseChangeSlabThicknessM,
  ChangeBeamSpacingM: parseChangeBeamSpacingM,
  ChangeBeamSupport: parseChangeBeamSupport,
  ChangeBeamRebarProduct: parseChangeBeamRebarProduct,
  ChangeBeamConcreteFCkPa: parseChangeBeamConcreteFCkPa,
  ChangeBeamConcreteECmPa: parseChangeBeamConcreteECmPa,
  ChangeBeamLtbLengthM: parseChangeBeamLtbLengthM,
  ChangeBeamHoggingRebarM2PerM: parseChangeBeamHoggingRebarM2PerM,
  ChangeBeamBarSpacingM: parseChangeBeamBarSpacingM,
  ChangeBeamCrackWidthLimitM: parseChangeBeamCrackWidthLimitM,
  ChangeBeamFatigueCycles: parseChangeBeamFatigueCycles,
  ChangeBeamSteelDesignation: parseChangeBeamSteelDesignation,
  ChangeBeamSteelHeightM: parseChangeBeamSteelHeightM,
  ChangeBeamSteelWidthM: parseChangeBeamSteelWidthM,
  ChangeBeamSteelWebThicknessM: parseChangeBeamSteelWebThicknessM,
  ChangeBeamSteelTfM: parseChangeBeamSteelTfM,
  ChangeBeamSteelAreaM2: parseChangeBeamSteelAreaM2,
  ChangeBeamSteelWPlYM3: parseChangeBeamSteelWPlYM3,
  ChangeBeamSteelSecondMomentM4: parseChangeBeamSteelSecondMomentM4,
  ChangeBeamSteelShearAreaM2: parseChangeBeamSteelShearAreaM2,
  ChangeBeamSteelRootRadiusM: parseChangeBeamSteelRootRadiusM,
  ChangeBeamSheetingProfile: parseChangeBeamSheetingProfile,
  ChangeBeamSheetingHeightM: parseChangeBeamSheetingHeightM,
  ChangeBeamSheetingRibWidthM: parseChangeBeamSheetingRibWidthM,
  ChangeBeamSheetingThicknessM: parseChangeBeamSheetingThicknessM,
  ChangeBeamSheetingRibsParallel: parseChangeBeamSheetingRibsParallel,
  ChangeBeamActionKind: parseChangeBeamActionKind,
  ChangeBeamActionCategory: parseChangeBeamActionCategory,
  ChangeBeamActionStage: parseChangeBeamActionStage,
  ChangeBeamActionPointForceN: parseChangeBeamActionPointForceN,
  ChangeBeamActionDeltaSigmaPa: parseChangeBeamActionDeltaSigmaPa,
  ChangeBeamActionDeltaTauPa: parseChangeBeamActionDeltaTauPa,
  ChangeColumnLengthM: parseChangeColumnLengthM,
  ChangeColumnOuterSizeM: parseChangeColumnOuterSizeM,
  ChangeColumnWallThicknessM: parseChangeColumnWallThicknessM,
  ChangeColumnSteelAreaM2: parseChangeColumnSteelAreaM2,
  ChangeColumnSteelFYPa: parseChangeColumnSteelFYPa,
  ChangeColumnConcreteAreaM2: parseChangeColumnConcreteAreaM2,
  ChangeColumnConcreteFCkPa: parseChangeColumnConcreteFCkPa,
  ChangeColumnRebarAreaM2: parseChangeColumnRebarAreaM2,
  ChangeColumnRebarFYkPa: parseChangeColumnRebarFYkPa,
  ChangeColumnSecondMomentM4: parseChangeColumnSecondMomentM4,
  ChangeColumnBucklingCurve: parseChangeColumnBucklingCurve,
  ChangeColumnRebarProduct: parseChangeColumnRebarProduct,
  ChangeColumnActionKind: parseChangeColumnActionKind,
  ChangeColumnActionCategory: parseChangeColumnActionCategory,
  ChangeColumnActionStage: parseChangeColumnActionStage,
  ChangeColumnActionMomentNm: parseChangeColumnActionMomentNm,
  ChangeSlabSpanM: parseChangeSlabSpanM,
  ChangeSlabSupport: parseChangeSlabSupport,
  ChangeSlabFCkPa: parseChangeSlabFCkPa,
  ChangeSlabMFactor: parseChangeSlabMFactor,
  ChangeSlabKFactor: parseChangeSlabKFactor,
  ChangeSlabReinforcementM2PerM: parseChangeSlabReinforcementM2PerM,
  ChangeSlabSheetingProfile: parseChangeSlabSheetingProfile,
  ChangeSlabSheetingHeightM: parseChangeSlabSheetingHeightM,
  ChangeSlabSheetingRibWidthM: parseChangeSlabSheetingRibWidthM,
  ChangeSlabSheetingThicknessM: parseChangeSlabSheetingThicknessM,
  ChangeSlabSheetingRibsParallel: parseChangeSlabSheetingRibsParallel,
  ChangeSlabActionKind: parseChangeSlabActionKind,
  ChangeSlabActionCategory: parseChangeSlabActionCategory,
  ChangeSlabActionStage: parseChangeSlabActionStage,
  ChangeSlabActionPointForceN: parseChangeSlabActionPointForceN,
  ChangeSlabActionDeltaSigmaPa: parseChangeSlabActionDeltaSigmaPa,
  ChangeSlabActionDeltaTauPa: parseChangeSlabActionDeltaTauPa,
  ChangeBeamAdjacentSpanM: parseChangeBeamAdjacentSpanM,
  ChangeBeamPatchBearingLengthM: parseChangeBeamPatchBearingLengthM,
  ChangeBeamBridgeLanes: parseChangeBeamBridgeLanes,
  ChangeColumnIntroducedForceN: parseChangeColumnIntroducedForceN,
  ChangeBeamCrackedLengthFraction: parseChangeBeamCrackedLengthFraction,
  ChangeColumnSectionHeightM: parseChangeColumnSectionHeightM,
  ChangeColumnSectionWidthM: parseChangeColumnSectionWidthM,
  ChangeColumnSectionTwM: parseChangeColumnSectionTwM,
  ChangeColumnSectionTfM: parseChangeColumnSectionTfM,
  ChangeColumnRebarCoverM: parseChangeColumnRebarCoverM,
  ChangePlacementX: parseChangePlacementX,
  ChangePlacementY: parseChangePlacementY,
  ChangePlacementZ: parseChangePlacementZ,
  ChangePlacementYaw: parseChangePlacementYaw,
  InsertPlacement: parseInsertPlacement,
  RemovePlacement: parseRemovePlacement,
  ChangeBeamStudCrossSpacingM: parseChangeBeamStudCrossSpacingM,
  ChangeBeamStudEdgeDistanceM: parseChangeBeamStudEdgeDistanceM,
  ChangeBeamTransverseAsTop: parseChangeBeamTransverseAsTop,
  ChangeBeamEndPlateWidthM: parseChangeBeamEndPlateWidthM,
  ChangeBeamEndPlateHeightM: parseChangeBeamEndPlateHeightM,
  ChangeBeamEndZoneLengthM: parseChangeBeamEndZoneLengthM,
  InsertBeamAction: parseInsertBeamAction,
  RemoveBeamAction: parseRemoveBeamAction,
  InsertColumnAction: parseInsertColumnAction,
  RemoveColumnAction: parseRemoveColumnAction,
  InsertSlabAction: parseInsertSlabAction,
  RemoveSlabAction: parseRemoveSlabAction,
  ChangeBeamActionXM: parseChangeBeamActionXM,
  ChangeSlabActionXM: parseChangeSlabActionXM,
  ChangeSlabActionYM: parseChangeSlabActionYM,
  InsertCase: parseInsertCase,
  RemoveCase: parseRemoveCase,
  ChangeCaseNumber: parseChangeCaseNumber,
  ChangeCaseText: parseChangeCaseText,
  ChangeCaseLabelEn: parseChangeCaseLabelEn,
  ChangeCaseLabelDe: parseChangeCaseLabelDe,
  ChangeCaseEntryId: parseChangeCaseEntryId,
  ChangeStudDiameterM: parseChangeStudDiameterM,
  ChangeStudFUPa: parseChangeStudFUPa,
  ChangeStudHeightM: parseChangeStudHeightM,
  ChangeStudXM: parseChangeStudXM,
  InsertStud: parseInsertStud,
  RemoveStud: parseRemoveStud,
});
