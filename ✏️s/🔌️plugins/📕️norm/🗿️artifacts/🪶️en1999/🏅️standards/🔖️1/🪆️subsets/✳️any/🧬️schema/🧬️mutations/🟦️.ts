/** 🧺️ `En1999Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireTagged, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ChangeMaterialDesignation, parseChangeMaterialDesignation } from "./⚗️change-material-designation/🧬️schema/🟦️.ts";
import { type ChangeMaterialProductForm, parseChangeMaterialProductForm } from "./⚗️change-material-product-form/🧬️schema/🟦️.ts";
import { type ChangeSheetActionCategory, parseChangeSheetActionCategory } from "./❄️change-sheet-action-category/🧬️schema/🟦️.ts";
import { type ChangeSheetActionGKLine, parseChangeSheetActionGKLine } from "./❄️change-sheet-action-gk-line/🧬️schema/🟦️.ts";
import { type ChangeSheetActionKind, parseChangeSheetActionKind } from "./❄️change-sheet-action-kind/🧬️schema/🟦️.ts";
import { type ChangeSheetActionLocalLoadK, parseChangeSheetActionLocalLoadK } from "./❄️change-sheet-action-local-load-k/🧬️schema/🟦️.ts";
import { type ChangeSheetActionMYK, parseChangeSheetActionMYK } from "./❄️change-sheet-action-myk/🧬️schema/🟦️.ts";
import { type ChangeSheetActionMZK, parseChangeSheetActionMZK } from "./❄️change-sheet-action-mzk/🧬️schema/🟦️.ts";
import { type ChangeSheetActionNK, parseChangeSheetActionNK } from "./❄️change-sheet-action-nk/🧬️schema/🟦️.ts";
import { type ChangeSheetActionQKLine, parseChangeSheetActionQKLine } from "./❄️change-sheet-action-qk-line/🧬️schema/🟦️.ts";
import { type ChangeSheetActionSource, parseChangeSheetActionSource } from "./❄️change-sheet-action-source/🧬️schema/🟦️.ts";
import { type ChangeSheetActionTorsionK, parseChangeSheetActionTorsionK } from "./❄️change-sheet-action-torsion-k/🧬️schema/🟦️.ts";
import { type ChangeSheetActionVYK, parseChangeSheetActionVYK } from "./❄️change-sheet-action-vyk/🧬️schema/🟦️.ts";
import { type ChangeSheetActionVZK, parseChangeSheetActionVZK } from "./❄️change-sheet-action-vzk/🧬️schema/🟦️.ts";
import { type ChangeSheetMaterialId, parseChangeSheetMaterialId } from "./❄️change-sheet-material-id/🧬️schema/🟦️.ts";
import { type ChangeSheetSpan, parseChangeSheetSpan } from "./❄️change-sheet-span/🧬️schema/🟦️.ts";
import { type ChangeSheetThickness, parseChangeSheetThickness } from "./❄️change-sheet-thickness/🧬️schema/🟦️.ts";
import { type ChangeSheetWelded, parseChangeSheetWelded } from "./❄️change-sheet-welded/🧬️schema/🟦️.ts";
import { type ChangeSheetWidth, parseChangeSheetWidth } from "./❄️change-sheet-width/🧬️schema/🟦️.ts";
import { type InsertColdFormedSheet, parseInsertColdFormedSheet } from "./➕insert-cold-formed-sheet/🧬️schema/🟦️.ts";
import { type InsertConnectionAction, parseInsertConnectionAction } from "./➕insert-connection-action/🧬️schema/🟦️.ts";
import { type InsertConnection, parseInsertConnection } from "./➕insert-connection/🧬️schema/🟦️.ts";
import { type InsertFatigueCategory, parseInsertFatigueCategory } from "./➕insert-fatigue-category/🧬️schema/🟦️.ts";
import { type InsertFatigueDetail, parseInsertFatigueDetail } from "./➕insert-fatigue-detail/🧬️schema/🟦️.ts";
import { type InsertFireScenario, parseInsertFireScenario } from "./➕insert-fire-scenario/🧬️schema/🟦️.ts";
import { type InsertMaterial, parseInsertMaterial } from "./➕insert-material/🧬️schema/🟦️.ts";
import { type InsertMemberAction, parseInsertMemberAction } from "./➕insert-member-action/🧬️schema/🟦️.ts";
import { type InsertMember, parseInsertMember } from "./➕insert-member/🧬️schema/🟦️.ts";
import { type InsertOutlineVertex, parseInsertOutlineVertex } from "./➕insert-outline-vertex/🧬️schema/🟦️.ts";
import { type InsertPatchLoad, parseInsertPatchLoad } from "./➕insert-patch-load/🧬️schema/🟦️.ts";
import { type InsertPlacement, parseInsertPlacement } from "./➕insert-placement/🧬️schema/🟦️.ts";
import { type InsertRingStiffener, parseInsertRingStiffener } from "./➕insert-ring-stiffener/🧬️schema/🟦️.ts";
import { type InsertSectionElement, parseInsertSectionElement } from "./➕insert-section-element/🧬️schema/🟦️.ts";
import { type InsertSection, parseInsertSection } from "./➕insert-section/🧬️schema/🟦️.ts";
import { type InsertSheetAction, parseInsertSheetAction } from "./➕insert-sheet-action/🧬️schema/🟦️.ts";
import { type InsertShellAction, parseInsertShellAction } from "./➕insert-shell-action/🧬️schema/🟦️.ts";
import { type InsertShell, parseInsertShell } from "./➕insert-shell/🧬️schema/🟦️.ts";
import { type InsertTransverseStiffener, parseInsertTransverseStiffener } from "./➕insert-transverse-stiffener/🧬️schema/🟦️.ts";
import { type InsertWeldRun, parseInsertWeldRun } from "./➕insert-weld-run/🧬️schema/🟦️.ts";
import { type RemoveColdFormedSheet, parseRemoveColdFormedSheet } from "./➖remove-cold-formed-sheet/🧬️schema/🟦️.ts";
import { type RemoveConnectionAction, parseRemoveConnectionAction } from "./➖remove-connection-action/🧬️schema/🟦️.ts";
import { type RemoveConnection, parseRemoveConnection } from "./➖remove-connection/🧬️schema/🟦️.ts";
import { type RemoveFatigueCategory, parseRemoveFatigueCategory } from "./➖remove-fatigue-category/🧬️schema/🟦️.ts";
import { type RemoveFatigueDetail, parseRemoveFatigueDetail } from "./➖remove-fatigue-detail/🧬️schema/🟦️.ts";
import { type RemoveFireScenario, parseRemoveFireScenario } from "./➖remove-fire-scenario/🧬️schema/🟦️.ts";
import { type RemoveMaterial, parseRemoveMaterial } from "./➖remove-material/🧬️schema/🟦️.ts";
import { type RemoveMemberAction, parseRemoveMemberAction } from "./➖remove-member-action/🧬️schema/🟦️.ts";
import { type RemoveMember, parseRemoveMember } from "./➖remove-member/🧬️schema/🟦️.ts";
import { type RemoveOutlineVertex, parseRemoveOutlineVertex } from "./➖remove-outline-vertex/🧬️schema/🟦️.ts";
import { type RemovePatchLoad, parseRemovePatchLoad } from "./➖remove-patch-load/🧬️schema/🟦️.ts";
import { type RemovePlacement, parseRemovePlacement } from "./➖remove-placement/🧬️schema/🟦️.ts";
import { type RemoveRingStiffener, parseRemoveRingStiffener } from "./➖remove-ring-stiffener/🧬️schema/🟦️.ts";
import { type RemoveSectionElement, parseRemoveSectionElement } from "./➖remove-section-element/🧬️schema/🟦️.ts";
import { type RemoveSection, parseRemoveSection } from "./➖remove-section/🧬️schema/🟦️.ts";
import { type RemoveSheetAction, parseRemoveSheetAction } from "./➖remove-sheet-action/🧬️schema/🟦️.ts";
import { type RemoveShellAction, parseRemoveShellAction } from "./➖remove-shell-action/🧬️schema/🟦️.ts";
import { type RemoveShell, parseRemoveShell } from "./➖remove-shell/🧬️schema/🟦️.ts";
import { type RemoveTransverseStiffener, parseRemoveTransverseStiffener } from "./➖remove-transverse-stiffener/🧬️schema/🟦️.ts";
import { type RemoveWeldRun, parseRemoveWeldRun } from "./➖remove-weld-run/🧬️schema/🟦️.ts";
import { type ChangeMemberMYEd, parseChangeMemberMYEd } from "./⤴️change-member-my-ed/🧬️schema/🟦️.ts";
import { type ChangePatchBearingLength, parseChangePatchBearingLength } from "./⬇️change-patch-bearing-length/🧬️schema/🟦️.ts";
import { type ChangePatchCornerRadius, parseChangePatchCornerRadius } from "./⬇️change-patch-corner-radius/🧬️schema/🟦️.ts";
import { type ChangePatchLoad, parseChangePatchLoad } from "./⬇️change-patch-load/🧬️schema/🟦️.ts";
import { type ChangePatchPosition, parseChangePatchPosition } from "./⬇️change-patch-position/🧬️schema/🟦️.ts";
import { type ChangePatchSubjectId, parseChangePatchSubjectId } from "./⬇️change-patch-subject-id/🧬️schema/🟦️.ts";
import { type ChangePatchWebAngle, parseChangePatchWebAngle } from "./⬇️change-patch-web-angle/🧬️schema/🟦️.ts";
import { type ChangePatchWebHeight, parseChangePatchWebHeight } from "./⬇️change-patch-web-height/🧬️schema/🟦️.ts";
import { type ChangePatchX, parseChangePatchX } from "./⬇️change-patch-x/🧬️schema/🟦️.ts";
import { type ChangeRingDepth, parseChangeRingDepth } from "./⭕️change-ring-depth/🧬️schema/🟦️.ts";
import { type ChangeRingInternal, parseChangeRingInternal } from "./⭕️change-ring-internal/🧬️schema/🟦️.ts";
import { type ChangeRingShellId, parseChangeRingShellId } from "./⭕️change-ring-shell-id/🧬️schema/🟦️.ts";
import { type ChangeRingThickness, parseChangeRingThickness } from "./⭕️change-ring-thickness/🧬️schema/🟦️.ts";
import { type ChangeRingX, parseChangeRingX } from "./⭕️change-ring-x/🧬️schema/🟦️.ts";
import { type ChangeAnnex, parseChangeAnnex } from "./🌍️change-annex/🧬️schema/🟦️.ts";
import { type ChangeReliabilityClass, parseChangeReliabilityClass } from "./🎯change-reliability-class/🧬️schema/🟦️.ts";
import { type ChangeCategoryDetailCategory, parseChangeCategoryDetailCategory } from "./🎯️change-category-detail-category/🧬️schema/🟦️.ts";
import { type ChangeCategoryDetailType, parseChangeCategoryDetailType } from "./🎯️change-category-detail-type/🧬️schema/🟦️.ts";
import { type ChangeCategoryElementId, parseChangeCategoryElementId } from "./🎯️change-category-element-id/🧬️schema/🟦️.ts";
import { type ChangeCategoryLocationX, parseChangeCategoryLocationX } from "./🎯️change-category-location-x/🧬️schema/🟦️.ts";
import { type ChangeCategoryM1, parseChangeCategoryM1 } from "./🎯️change-category-m1/🧬️schema/🟦️.ts";
import { type ChangeCategoryM2, parseChangeCategoryM2 } from "./🎯️change-category-m2/🧬️schema/🟦️.ts";
import { type ChangeCategoryMemberId, parseChangeCategoryMemberId } from "./🎯️change-category-member-id/🧬️schema/🟦️.ts";
import { type ChangeActionCategory, parseChangeActionCategory } from "./🏋️change-action-category/🧬️schema/🟦️.ts";
import { type ChangeActionGKLine, parseChangeActionGKLine } from "./🏋️change-action-gk-line/🧬️schema/🟦️.ts";
import { type ChangeActionKind, parseChangeActionKind } from "./🏋️change-action-kind/🧬️schema/🟦️.ts";
import { type ChangeActionLocalLoadK, parseChangeActionLocalLoadK } from "./🏋️change-action-local-load-k/🧬️schema/🟦️.ts";
import { type ChangeActionMZK, parseChangeActionMZK } from "./🏋️change-action-mzk/🧬️schema/🟦️.ts";
import { type ChangeActionQKLine, parseChangeActionQKLine } from "./🏋️change-action-qk-line/🧬️schema/🟦️.ts";
import { type ChangeActionSource, parseChangeActionSource } from "./🏋️change-action-source/🧬️schema/🟦️.ts";
import { type ChangeActionTorsionK, parseChangeActionTorsionK } from "./🏋️change-action-torsion-k/🧬️schema/🟦️.ts";
import { type ChangeActionVYK, parseChangeActionVYK } from "./🏋️change-action-vyk/🧬️schema/🟦️.ts";
import { type ChangeActionVZK, parseChangeActionVZK } from "./🏋️change-action-vzk/🧬️schema/🟦️.ts";
import { type ChangeMemberNEd, parseChangeMemberNEd } from "./🏋️change-member-n-ed/🧬️schema/🟦️.ts";
import { type ChangeMemberBearingLength, parseChangeMemberBearingLength } from "./🏗️change-member-bearing-length/🧬️schema/🟦️.ts";
import { type ChangeMemberBucklingLengthT, parseChangeMemberBucklingLengthT } from "./🏗️change-member-buckling-length-t/🧬️schema/🟦️.ts";
import { type ChangeMemberBucklingLengthY, parseChangeMemberBucklingLengthY } from "./🏗️change-member-buckling-length-y/🧬️schema/🟦️.ts";
import { type ChangeMemberC1, parseChangeMemberC1 } from "./🏗️change-member-c1/🧬️schema/🟦️.ts";
import { type ChangeMemberLength, parseChangeMemberLength } from "./🏗️change-member-length/🧬️schema/🟦️.ts";
import { type ChangeMemberLtbLength, parseChangeMemberLtbLength } from "./🏗️change-member-ltb-length/🧬️schema/🟦️.ts";
import { type ChangeMemberMaterialId, parseChangeMemberMaterialId } from "./🏗️change-member-material-id/🧬️schema/🟦️.ts";
import { type ChangeMemberRestrainedLtb, parseChangeMemberRestrainedLtb } from "./🏗️change-member-restrained-ltb/🧬️schema/🟦️.ts";
import { type ChangeMemberSectionId, parseChangeMemberSectionId } from "./🏗️change-member-section-id/🧬️schema/🟦️.ts";
import { type ChangeMemberSupport, parseChangeMemberSupport } from "./🏗️change-member-support/🧬️schema/🟦️.ts";
import { type ChangeMemberTWeldOffset, parseChangeMemberTWeldOffset } from "./🏗️change-member-t-weld-offset/🧬️schema/🟦️.ts";
import { type ChangeMemberTransverseWeld, parseChangeMemberTransverseWeld } from "./🏗️change-member-transverse-weld/🧬️schema/🟦️.ts";
import { type ChangePlacementX, parseChangePlacementX } from "./📍️change-placement-x/🧬️schema/🟦️.ts";
import { type ChangePlacementY, parseChangePlacementY } from "./📍️change-placement-y/🧬️schema/🟦️.ts";
import { type ChangePlacementYaw, parseChangePlacementYaw } from "./📍️change-placement-yaw/🧬️schema/🟦️.ts";
import { type ChangePlacementZ, parseChangePlacementZ } from "./📍️change-placement-z/🧬️schema/🟦️.ts";
import { type ChangeMemberBucklingLength, parseChangeMemberBucklingLength } from "./📏️change-member-buckling-length/🧬️schema/🟦️.ts";
import { type ChangeSectionFlangeThickness, parseChangeSectionFlangeThickness } from "./📐️change-section-flange-thickness/🧬️schema/🟦️.ts";
import { type ChangeSectionHeight, parseChangeSectionHeight } from "./📐️change-section-height/🧬️schema/🟦️.ts";
import { type ChangeSectionKind, parseChangeSectionKind } from "./📐️change-section-kind/🧬️schema/🟦️.ts";
import { type ChangeSectionOuterDiameter, parseChangeSectionOuterDiameter } from "./📐️change-section-outer-diameter/🧬️schema/🟦️.ts";
import { type ChangeSectionWebThickness, parseChangeSectionWebThickness } from "./📐️change-section-web-thickness/🧬️schema/🟦️.ts";
import { type ChangeSectionWidth, parseChangeSectionWidth } from "./📐️change-section-width/🧬️schema/🟦️.ts";
import { type ChangeFatigueDeltaSigmaC, parseChangeFatigueDeltaSigmaC } from "./🔄️change-fatigue-delta-sigma-c/🧬️schema/🟦️.ts";
import { type ChangeFatigueDeltaSigmaEd, parseChangeFatigueDeltaSigmaEd } from "./🔄️change-fatigue-delta-sigma-ed/🧬️schema/🟦️.ts";
import { type ChangeFatigueDetailCategory, parseChangeFatigueDetailCategory } from "./🔄️change-fatigue-detail-category/🧬️schema/🟦️.ts";
import { type ChangeFatigueM1, parseChangeFatigueM1 } from "./🔄️change-fatigue-m1/🧬️schema/🟦️.ts";
import { type ChangeFatigueM2, parseChangeFatigueM2 } from "./🔄️change-fatigue-m2/🧬️schema/🟦️.ts";
import { type ChangeFatigueMemberId, parseChangeFatigueMemberId } from "./🔄️change-fatigue-member-id/🧬️schema/🟦️.ts";
import { type ChangeFatigueNCycles, parseChangeFatigueNCycles } from "./🔄️change-fatigue-n-cycles/🧬️schema/🟦️.ts";
import { type ChangeConnActionCategory, parseChangeConnActionCategory } from "./🔗️change-conn-action-category/🧬️schema/🟦️.ts";
import { type ChangeConnActionGKLine, parseChangeConnActionGKLine } from "./🔗️change-conn-action-gk-line/🧬️schema/🟦️.ts";
import { type ChangeConnActionKind, parseChangeConnActionKind } from "./🔗️change-conn-action-kind/🧬️schema/🟦️.ts";
import { type ChangeConnActionLocalLoadK, parseChangeConnActionLocalLoadK } from "./🔗️change-conn-action-local-load-k/🧬️schema/🟦️.ts";
import { type ChangeConnActionMYK, parseChangeConnActionMYK } from "./🔗️change-conn-action-myk/🧬️schema/🟦️.ts";
import { type ChangeConnActionMZK, parseChangeConnActionMZK } from "./🔗️change-conn-action-mzk/🧬️schema/🟦️.ts";
import { type ChangeConnActionNK, parseChangeConnActionNK } from "./🔗️change-conn-action-nk/🧬️schema/🟦️.ts";
import { type ChangeConnActionQKLine, parseChangeConnActionQKLine } from "./🔗️change-conn-action-qk-line/🧬️schema/🟦️.ts";
import { type ChangeConnActionSource, parseChangeConnActionSource } from "./🔗️change-conn-action-source/🧬️schema/🟦️.ts";
import { type ChangeConnActionTorsionK, parseChangeConnActionTorsionK } from "./🔗️change-conn-action-torsion-k/🧬️schema/🟦️.ts";
import { type ChangeConnActionVYK, parseChangeConnActionVYK } from "./🔗️change-conn-action-vyk/🧬️schema/🟦️.ts";
import { type ChangeConnActionVZK, parseChangeConnActionVZK } from "./🔗️change-conn-action-vzk/🧬️schema/🟦️.ts";
import { type ChangeConnBoltsBoltsPerRow, parseChangeConnBoltsBoltsPerRow } from "./🔗️change-conn-bolts-bolts-per-row/🧬️schema/🟦️.ts";
import { type ChangeConnBoltsDiameter, parseChangeConnBoltsDiameter } from "./🔗️change-conn-bolts-diameter/🧬️schema/🟦️.ts";
import { type ChangeConnBoltsEdgeDistance, parseChangeConnBoltsEdgeDistance } from "./🔗️change-conn-bolts-edge-distance/🧬️schema/🟦️.ts";
import { type ChangeConnBoltsGauge, parseChangeConnBoltsGauge } from "./🔗️change-conn-bolts-gauge/🧬️schema/🟦️.ts";
import { type ChangeConnBoltsMaterial, parseChangeConnBoltsMaterial } from "./🔗️change-conn-bolts-material/🧬️schema/🟦️.ts";
import { type ChangeConnBoltsPitch, parseChangeConnBoltsPitch } from "./🔗️change-conn-bolts-pitch/🧬️schema/🟦️.ts";
import { type ChangeConnBoltsPlateThickness, parseChangeConnBoltsPlateThickness } from "./🔗️change-conn-bolts-plate-thickness/🧬️schema/🟦️.ts";
import { type ChangeConnForceMethod, parseChangeConnForceMethod } from "./🔗️change-conn-force-method/🧬️schema/🟦️.ts";
import { type ChangeConnKind, parseChangeConnKind } from "./🔗️change-conn-kind/🧬️schema/🟦️.ts";
import { type ChangeConnMaterialId, parseChangeConnMaterialId } from "./🔗️change-conn-material-id/🧬️schema/🟦️.ts";
import { type ChangeConnMemberId, parseChangeConnMemberId } from "./🔗️change-conn-member-id/🧬️schema/🟦️.ts";
import { type ChangeConnWeldsBetaW, parseChangeConnWeldsBetaW } from "./🔗️change-conn-welds-beta-w/🧬️schema/🟦️.ts";
import { type ChangeConnWeldsFillerAlloy, parseChangeConnWeldsFillerAlloy } from "./🔗️change-conn-welds-filler-alloy/🧬️schema/🟦️.ts";
import { type ChangeConnWeldsHazExtent, parseChangeConnWeldsHazExtent } from "./🔗️change-conn-welds-haz-extent/🧬️schema/🟦️.ts";
import { type ChangeConnWeldsLength, parseChangeConnWeldsLength } from "./🔗️change-conn-welds-length/🧬️schema/🟦️.ts";
import { type ChangeFireDurationS, parseChangeFireDurationS } from "./🔥️change-fire-duration-s/🧬️schema/🟦️.ts";
import { type ChangeFireMemberId, parseChangeFireMemberId } from "./🔥️change-fire-member-id/🧬️schema/🟦️.ts";
import { type ChangeFireThetaA, parseChangeFireThetaA } from "./🔥️change-fire-theta-a/🧬️schema/🟦️.ts";
import { type ChangeWeldThroat, parseChangeWeldThroat } from "./🔥️change-weld-throat/🧬️schema/🟦️.ts";
import { type ChangeBoltCount, parseChangeBoltCount } from "./🔩change-bolt-count/🧬️schema/🟦️.ts";
import { type ChangeVertexElementId, parseChangeVertexElementId } from "./🔷️change-vertex-element-id/🧬️schema/🟦️.ts";
import { type ChangeVertexRing, parseChangeVertexRing } from "./🔷️change-vertex-ring/🧬️schema/🟦️.ts";
import { type ChangeVertexSectionId, parseChangeVertexSectionId } from "./🔷️change-vertex-section-id/🧬️schema/🟦️.ts";
import { type ChangeVertexY, parseChangeVertexY } from "./🔷️change-vertex-y/🧬️schema/🟦️.ts";
import { type ChangeVertexZ, parseChangeVertexZ } from "./🔷️change-vertex-z/🧬️schema/🟦️.ts";
import { type ChangeConnHoleStagger, parseChangeConnHoleStagger } from "./🦴️change-conn-hole-stagger/🧬️schema/🟦️.ts";
import { type ChangeConnLoadEccentricity, parseChangeConnLoadEccentricity } from "./🦴️change-conn-load-eccentricity/🧬️schema/🟦️.ts";
import { type ChangeTransverseGirderId, parseChangeTransverseGirderId } from "./🦴️change-transverse-girder-id/🧬️schema/🟦️.ts";
import { type ChangeTransversePosition, parseChangeTransversePosition } from "./🦴️change-transverse-position/🧬️schema/🟦️.ts";
import { type ChangeTransverseSides, parseChangeTransverseSides } from "./🦴️change-transverse-sides/🧬️schema/🟦️.ts";
import { type ChangeTransverseThickness, parseChangeTransverseThickness } from "./🦴️change-transverse-thickness/🧬️schema/🟦️.ts";
import { type ChangeTransverseWidth, parseChangeTransverseWidth } from "./🦴️change-transverse-width/🧬️schema/🟦️.ts";
import { type ChangePlateThickness, parseChangePlateThickness } from "./🧱change-plate-thickness/🧬️schema/🟦️.ts";
import { type ChangeElementOutstand, parseChangeElementOutstand } from "./🧱️change-element-outstand/🧬️schema/🟦️.ts";
import { type ChangeElementStiffenerDepth, parseChangeElementStiffenerDepth } from "./🧱️change-element-stiffener-depth/🧬️schema/🟦️.ts";
import { type ChangeElementStiffenerThk, parseChangeElementStiffenerThk } from "./🧱️change-element-stiffener-thk/🧬️schema/🟦️.ts";
import { type ChangeElementWeldDirection, parseChangeElementWeldDirection } from "./🧱️change-element-weld-direction/🧬️schema/🟦️.ts";
import { type ChangeElementWeldPosition, parseChangeElementWeldPosition } from "./🧱️change-element-weld-position/🧬️schema/🟦️.ts";
import { type ChangeElementWeldProcess, parseChangeElementWeldProcess } from "./🧱️change-element-weld-process/🧬️schema/🟦️.ts";
import { type ChangeElementWeldTJunction, parseChangeElementWeldTJunction } from "./🧱️change-element-weld-t-junction/🧬️schema/🟦️.ts";
import { type ChangeElementWelded, parseChangeElementWelded } from "./🧱️change-element-welded/🧬️schema/🟦️.ts";
import { type ChangeElementWidth, parseChangeElementWidth } from "./🧱️change-element-width/🧬️schema/🟦️.ts";
import { type ChangeWeldRunDirection, parseChangeWeldRunDirection } from "./🧵️change-weld-run-direction/🧬️schema/🟦️.ts";
import { type ChangeWeldRunElementId, parseChangeWeldRunElementId } from "./🧵️change-weld-run-element-id/🧬️schema/🟦️.ts";
import { type ChangeWeldRunHazExtent, parseChangeWeldRunHazExtent } from "./🧵️change-weld-run-haz-extent/🧬️schema/🟦️.ts";
import { type ChangeWeldRunLength, parseChangeWeldRunLength } from "./🧵️change-weld-run-length/🧬️schema/🟦️.ts";
import { type ChangeWeldRunMemberId, parseChangeWeldRunMemberId } from "./🧵️change-weld-run-member-id/🧬️schema/🟦️.ts";
import { type ChangeWeldRunOffset, parseChangeWeldRunOffset } from "./🧵️change-weld-run-offset/🧬️schema/🟦️.ts";
import { type ChangeWeldRunProcess, parseChangeWeldRunProcess } from "./🧵️change-weld-run-process/🧬️schema/🟦️.ts";
import { type ChangeWeldRunStart, parseChangeWeldRunStart } from "./🧵️change-weld-run-start/🧬️schema/🟦️.ts";
import { type ChangeWeldRunTJunction, parseChangeWeldRunTJunction } from "./🧵️change-weld-run-t-junction/🧬️schema/🟦️.ts";
import { type ChangeWeldRunThroat, parseChangeWeldRunThroat } from "./🧵️change-weld-run-throat/🧬️schema/🟦️.ts";
import { type ChangeShellActionCategory, parseChangeShellActionCategory } from "./🫙️change-shell-action-category/🧬️schema/🟦️.ts";
import { type ChangeShellActionGKLine, parseChangeShellActionGKLine } from "./🫙️change-shell-action-gk-line/🧬️schema/🟦️.ts";
import { type ChangeShellActionKind, parseChangeShellActionKind } from "./🫙️change-shell-action-kind/🧬️schema/🟦️.ts";
import { type ChangeShellActionLocalLoadK, parseChangeShellActionLocalLoadK } from "./🫙️change-shell-action-local-load-k/🧬️schema/🟦️.ts";
import { type ChangeShellActionMYK, parseChangeShellActionMYK } from "./🫙️change-shell-action-myk/🧬️schema/🟦️.ts";
import { type ChangeShellActionMZK, parseChangeShellActionMZK } from "./🫙️change-shell-action-mzk/🧬️schema/🟦️.ts";
import { type ChangeShellActionNK, parseChangeShellActionNK } from "./🫙️change-shell-action-nk/🧬️schema/🟦️.ts";
import { type ChangeShellActionQKLine, parseChangeShellActionQKLine } from "./🫙️change-shell-action-qk-line/🧬️schema/🟦️.ts";
import { type ChangeShellActionSource, parseChangeShellActionSource } from "./🫙️change-shell-action-source/🧬️schema/🟦️.ts";
import { type ChangeShellActionTorsionK, parseChangeShellActionTorsionK } from "./🫙️change-shell-action-torsion-k/🧬️schema/🟦️.ts";
import { type ChangeShellActionVYK, parseChangeShellActionVYK } from "./🫙️change-shell-action-vyk/🧬️schema/🟦️.ts";
import { type ChangeShellActionVZK, parseChangeShellActionVZK } from "./🫙️change-shell-action-vzk/🧬️schema/🟦️.ts";
import { type ChangeShellFabricationClass, parseChangeShellFabricationClass } from "./🫙️change-shell-fabrication-class/🧬️schema/🟦️.ts";
import { type ChangeShellLength, parseChangeShellLength } from "./🫙️change-shell-length/🧬️schema/🟦️.ts";
import { type ChangeShellMaterialId, parseChangeShellMaterialId } from "./🫙️change-shell-material-id/🧬️schema/🟦️.ts";
import { type ChangeShellRadius, parseChangeShellRadius } from "./🫙️change-shell-radius/🧬️schema/🟦️.ts";
import { type ChangeShellThickness, parseChangeShellThickness } from "./🫙️change-shell-thickness/🧬️schema/🟦️.ts";
import { type ChangeShellWelded, parseChangeShellWelded } from "./🫙️change-shell-welded/🧬️schema/🟦️.ts";

export type En1999Mutation =
  | ChangeAnnex
  | InsertMember
  | RemoveMember
  | ChangeMemberNEd
  | ChangeMemberMYEd
  | ChangeMemberBucklingLength
  | ChangeMaterialDesignation
  | ChangePlateThickness
  | ChangeWeldThroat
  | ChangeBoltCount
  | ChangeMaterialProductForm
  | ChangeSectionKind
  | ChangeSectionHeight
  | ChangeSectionWidth
  | ChangeSectionFlangeThickness
  | ChangeSectionWebThickness
  | ChangeSectionOuterDiameter
  | ChangeElementWidth
  | ChangeElementOutstand
  | ChangeElementWelded
  | ChangeElementWeldPosition
  | ChangeElementWeldProcess
  | ChangeElementWeldTJunction
  | ChangeElementWeldDirection
  | ChangeElementStiffenerDepth
  | ChangeElementStiffenerThk
  | ChangeMemberSectionId
  | ChangeMemberMaterialId
  | ChangeMemberLength
  | ChangeMemberSupport
  | ChangeMemberLtbLength
  | ChangeMemberC1
  | ChangeMemberRestrainedLtb
  | ChangeMemberBearingLength
  | ChangeMemberTransverseWeld
  | ChangeMemberTWeldOffset
  | ChangeActionKind
  | ChangeActionCategory
  | ChangeActionSource
  | ChangeActionGKLine
  | ChangeActionQKLine
  | ChangeActionVYK
  | ChangeActionVZK
  | ChangeActionMZK
  | ChangeActionTorsionK
  | ChangeActionLocalLoadK
  | ChangeConnMemberId
  | ChangeConnMaterialId
  | ChangeConnKind
  | ChangeConnForceMethod
  | ChangeConnBoltsMaterial
  | ChangeConnBoltsDiameter
  | ChangeConnBoltsEdgeDistance
  | ChangeConnBoltsPitch
  | ChangeConnBoltsGauge
  | ChangeConnBoltsPlateThickness
  | ChangeConnWeldsFillerAlloy
  | ChangeConnWeldsLength
  | ChangeConnWeldsBetaW
  | ChangeConnWeldsHazExtent
  | ChangeConnActionKind
  | ChangeConnActionCategory
  | ChangeConnActionSource
  | ChangeConnActionGKLine
  | ChangeConnActionQKLine
  | ChangeConnActionNK
  | ChangeConnActionVYK
  | ChangeConnActionVZK
  | ChangeConnActionMYK
  | ChangeConnActionMZK
  | ChangeConnActionTorsionK
  | ChangeConnActionLocalLoadK
  | ChangeFireMemberId
  | ChangeFireThetaA
  | ChangeFireDurationS
  | ChangeFatigueMemberId
  | ChangeFatigueDetailCategory
  | ChangeFatigueDeltaSigmaC
  | ChangeFatigueDeltaSigmaEd
  | ChangeFatigueNCycles
  | ChangeFatigueM1
  | ChangeFatigueM2
  | ChangeSheetMaterialId
  | ChangeSheetThickness
  | ChangeSheetWidth
  | ChangeSheetSpan
  | ChangeSheetWelded
  | ChangeSheetActionKind
  | ChangeSheetActionCategory
  | ChangeSheetActionSource
  | ChangeSheetActionGKLine
  | ChangeSheetActionQKLine
  | ChangeSheetActionNK
  | ChangeSheetActionVYK
  | ChangeSheetActionVZK
  | ChangeSheetActionMYK
  | ChangeSheetActionMZK
  | ChangeSheetActionTorsionK
  | ChangeSheetActionLocalLoadK
  | ChangeShellMaterialId
  | ChangeShellRadius
  | ChangeShellThickness
  | ChangeShellLength
  | ChangeShellWelded
  | ChangeShellFabricationClass
  | ChangeShellActionKind
  | ChangeShellActionCategory
  | ChangeShellActionSource
  | ChangeShellActionGKLine
  | ChangeShellActionQKLine
  | ChangeShellActionNK
  | ChangeShellActionVYK
  | ChangeShellActionVZK
  | ChangeShellActionMYK
  | ChangeShellActionMZK
  | ChangeShellActionTorsionK
  | ChangeShellActionLocalLoadK
  | ChangeWeldRunMemberId
  | ChangeWeldRunElementId
  | ChangeWeldRunStart
  | ChangeWeldRunLength
  | ChangeWeldRunOffset
  | ChangeWeldRunThroat
  | ChangeWeldRunProcess
  | ChangeWeldRunTJunction
  | ChangeWeldRunDirection
  | ChangeWeldRunHazExtent
  | ChangeVertexSectionId
  | ChangeVertexRing
  | ChangeVertexElementId
  | ChangeVertexY
  | ChangeVertexZ
  | ChangePlacementX
  | ChangePlacementY
  | ChangePlacementZ
  | ChangePlacementYaw
  | InsertWeldRun
  | RemoveWeldRun
  | InsertOutlineVertex
  | RemoveOutlineVertex
  | InsertPlacement
  | RemovePlacement
  | InsertMaterial
  | RemoveMaterial
  | InsertSection
  | RemoveSection
  | InsertConnection
  | RemoveConnection
  | InsertFireScenario
  | RemoveFireScenario
  | InsertFatigueDetail
  | RemoveFatigueDetail
  | InsertColdFormedSheet
  | RemoveColdFormedSheet
  | InsertShell
  | RemoveShell
  | ChangeMemberBucklingLengthY
  | ChangeMemberBucklingLengthT
  | ChangeConnBoltsBoltsPerRow
  | ChangeCategoryMemberId
  | ChangeCategoryElementId
  | ChangeCategoryDetailCategory
  | ChangeCategoryDetailType
  | ChangeCategoryLocationX
  | ChangeCategoryM1
  | ChangeCategoryM2
  | ChangePatchSubjectId
  | ChangePatchPosition
  | ChangePatchX
  | ChangePatchBearingLength
  | ChangePatchWebHeight
  | ChangePatchWebAngle
  | ChangePatchCornerRadius
  | ChangePatchLoad
  | ChangeRingShellId
  | ChangeRingX
  | ChangeRingDepth
  | ChangeRingThickness
  | ChangeRingInternal
  | InsertFatigueCategory
  | RemoveFatigueCategory
  | InsertPatchLoad
  | RemovePatchLoad
  | InsertRingStiffener
  | RemoveRingStiffener
  | InsertMemberAction
  | RemoveMemberAction
  | InsertConnectionAction
  | RemoveConnectionAction
  | InsertSectionElement
  | RemoveSectionElement
  | InsertSheetAction
  | RemoveSheetAction
  | InsertShellAction
  | RemoveShellAction
  | ChangeTransverseGirderId
  | ChangeTransversePosition
  | ChangeTransverseWidth
  | ChangeTransverseThickness
  | ChangeConnLoadEccentricity
  | ChangeConnHoleStagger
  | ChangeTransverseSides
  | InsertTransverseStiffener
  | RemoveTransverseStiffener
  | ChangeReliabilityClass;

export const parseEn1999Mutation: NormWireReader<En1999Mutation> = normWireTagged<En1999Mutation, "mutation">("mutation", {
  changeAnnex: parseChangeAnnex,
  insertMember: parseInsertMember,
  removeMember: parseRemoveMember,
  changeMemberNEd: parseChangeMemberNEd,
  changeMemberMYEd: parseChangeMemberMYEd,
  changeMemberBucklingLength: parseChangeMemberBucklingLength,
  changeMaterialDesignation: parseChangeMaterialDesignation,
  changePlateThickness: parseChangePlateThickness,
  changeWeldThroat: parseChangeWeldThroat,
  changeBoltCount: parseChangeBoltCount,
  changeMaterialProductForm: parseChangeMaterialProductForm,
  changeSectionKind: parseChangeSectionKind,
  changeSectionHeight: parseChangeSectionHeight,
  changeSectionWidth: parseChangeSectionWidth,
  changeSectionFlangeThickness: parseChangeSectionFlangeThickness,
  changeSectionWebThickness: parseChangeSectionWebThickness,
  changeSectionOuterDiameter: parseChangeSectionOuterDiameter,
  changeElementWidth: parseChangeElementWidth,
  changeElementOutstand: parseChangeElementOutstand,
  changeElementWelded: parseChangeElementWelded,
  changeElementWeldPosition: parseChangeElementWeldPosition,
  changeElementWeldProcess: parseChangeElementWeldProcess,
  changeElementWeldTJunction: parseChangeElementWeldTJunction,
  changeElementWeldDirection: parseChangeElementWeldDirection,
  changeElementStiffenerDepth: parseChangeElementStiffenerDepth,
  changeElementStiffenerThk: parseChangeElementStiffenerThk,
  changeMemberSectionId: parseChangeMemberSectionId,
  changeMemberMaterialId: parseChangeMemberMaterialId,
  changeMemberLength: parseChangeMemberLength,
  changeMemberSupport: parseChangeMemberSupport,
  changeMemberLtbLength: parseChangeMemberLtbLength,
  changeMemberC1: parseChangeMemberC1,
  changeMemberRestrainedLtb: parseChangeMemberRestrainedLtb,
  changeMemberBearingLength: parseChangeMemberBearingLength,
  changeMemberTransverseWeld: parseChangeMemberTransverseWeld,
  changeMemberTWeldOffset: parseChangeMemberTWeldOffset,
  changeActionKind: parseChangeActionKind,
  changeActionCategory: parseChangeActionCategory,
  changeActionSource: parseChangeActionSource,
  changeActionGKLine: parseChangeActionGKLine,
  changeActionQKLine: parseChangeActionQKLine,
  changeActionVYK: parseChangeActionVYK,
  changeActionVZK: parseChangeActionVZK,
  changeActionMZK: parseChangeActionMZK,
  changeActionTorsionK: parseChangeActionTorsionK,
  changeActionLocalLoadK: parseChangeActionLocalLoadK,
  changeConnMemberId: parseChangeConnMemberId,
  changeConnMaterialId: parseChangeConnMaterialId,
  changeConnKind: parseChangeConnKind,
  changeConnForceMethod: parseChangeConnForceMethod,
  changeConnBoltsMaterial: parseChangeConnBoltsMaterial,
  changeConnBoltsDiameter: parseChangeConnBoltsDiameter,
  changeConnBoltsEdgeDistance: parseChangeConnBoltsEdgeDistance,
  changeConnBoltsPitch: parseChangeConnBoltsPitch,
  changeConnBoltsGauge: parseChangeConnBoltsGauge,
  changeConnBoltsPlateThickness: parseChangeConnBoltsPlateThickness,
  changeConnWeldsFillerAlloy: parseChangeConnWeldsFillerAlloy,
  changeConnWeldsLength: parseChangeConnWeldsLength,
  changeConnWeldsBetaW: parseChangeConnWeldsBetaW,
  changeConnWeldsHazExtent: parseChangeConnWeldsHazExtent,
  changeConnActionKind: parseChangeConnActionKind,
  changeConnActionCategory: parseChangeConnActionCategory,
  changeConnActionSource: parseChangeConnActionSource,
  changeConnActionGKLine: parseChangeConnActionGKLine,
  changeConnActionQKLine: parseChangeConnActionQKLine,
  changeConnActionNK: parseChangeConnActionNK,
  changeConnActionVYK: parseChangeConnActionVYK,
  changeConnActionVZK: parseChangeConnActionVZK,
  changeConnActionMYK: parseChangeConnActionMYK,
  changeConnActionMZK: parseChangeConnActionMZK,
  changeConnActionTorsionK: parseChangeConnActionTorsionK,
  changeConnActionLocalLoadK: parseChangeConnActionLocalLoadK,
  changeFireMemberId: parseChangeFireMemberId,
  changeFireThetaA: parseChangeFireThetaA,
  changeFireDurationS: parseChangeFireDurationS,
  changeFatigueMemberId: parseChangeFatigueMemberId,
  changeFatigueDetailCategory: parseChangeFatigueDetailCategory,
  changeFatigueDeltaSigmaC: parseChangeFatigueDeltaSigmaC,
  changeFatigueDeltaSigmaEd: parseChangeFatigueDeltaSigmaEd,
  changeFatigueNCycles: parseChangeFatigueNCycles,
  changeFatigueM1: parseChangeFatigueM1,
  changeFatigueM2: parseChangeFatigueM2,
  changeSheetMaterialId: parseChangeSheetMaterialId,
  changeSheetThickness: parseChangeSheetThickness,
  changeSheetWidth: parseChangeSheetWidth,
  changeSheetSpan: parseChangeSheetSpan,
  changeSheetWelded: parseChangeSheetWelded,
  changeSheetActionKind: parseChangeSheetActionKind,
  changeSheetActionCategory: parseChangeSheetActionCategory,
  changeSheetActionSource: parseChangeSheetActionSource,
  changeSheetActionGKLine: parseChangeSheetActionGKLine,
  changeSheetActionQKLine: parseChangeSheetActionQKLine,
  changeSheetActionNK: parseChangeSheetActionNK,
  changeSheetActionVYK: parseChangeSheetActionVYK,
  changeSheetActionVZK: parseChangeSheetActionVZK,
  changeSheetActionMYK: parseChangeSheetActionMYK,
  changeSheetActionMZK: parseChangeSheetActionMZK,
  changeSheetActionTorsionK: parseChangeSheetActionTorsionK,
  changeSheetActionLocalLoadK: parseChangeSheetActionLocalLoadK,
  changeShellMaterialId: parseChangeShellMaterialId,
  changeShellRadius: parseChangeShellRadius,
  changeShellThickness: parseChangeShellThickness,
  changeShellLength: parseChangeShellLength,
  changeShellWelded: parseChangeShellWelded,
  changeShellFabricationClass: parseChangeShellFabricationClass,
  changeShellActionKind: parseChangeShellActionKind,
  changeShellActionCategory: parseChangeShellActionCategory,
  changeShellActionSource: parseChangeShellActionSource,
  changeShellActionGKLine: parseChangeShellActionGKLine,
  changeShellActionQKLine: parseChangeShellActionQKLine,
  changeShellActionNK: parseChangeShellActionNK,
  changeShellActionVYK: parseChangeShellActionVYK,
  changeShellActionVZK: parseChangeShellActionVZK,
  changeShellActionMYK: parseChangeShellActionMYK,
  changeShellActionMZK: parseChangeShellActionMZK,
  changeShellActionTorsionK: parseChangeShellActionTorsionK,
  changeShellActionLocalLoadK: parseChangeShellActionLocalLoadK,
  changeWeldRunMemberId: parseChangeWeldRunMemberId,
  changeWeldRunElementId: parseChangeWeldRunElementId,
  changeWeldRunStart: parseChangeWeldRunStart,
  changeWeldRunLength: parseChangeWeldRunLength,
  changeWeldRunOffset: parseChangeWeldRunOffset,
  changeWeldRunThroat: parseChangeWeldRunThroat,
  changeWeldRunProcess: parseChangeWeldRunProcess,
  changeWeldRunTJunction: parseChangeWeldRunTJunction,
  changeWeldRunDirection: parseChangeWeldRunDirection,
  changeWeldRunHazExtent: parseChangeWeldRunHazExtent,
  changeVertexSectionId: parseChangeVertexSectionId,
  changeVertexRing: parseChangeVertexRing,
  changeVertexElementId: parseChangeVertexElementId,
  changeVertexY: parseChangeVertexY,
  changeVertexZ: parseChangeVertexZ,
  changePlacementX: parseChangePlacementX,
  changePlacementY: parseChangePlacementY,
  changePlacementZ: parseChangePlacementZ,
  changePlacementYaw: parseChangePlacementYaw,
  insertWeldRun: parseInsertWeldRun,
  removeWeldRun: parseRemoveWeldRun,
  insertOutlineVertex: parseInsertOutlineVertex,
  removeOutlineVertex: parseRemoveOutlineVertex,
  insertPlacement: parseInsertPlacement,
  removePlacement: parseRemovePlacement,
  insertMaterial: parseInsertMaterial,
  removeMaterial: parseRemoveMaterial,
  insertSection: parseInsertSection,
  removeSection: parseRemoveSection,
  insertConnection: parseInsertConnection,
  removeConnection: parseRemoveConnection,
  insertFireScenario: parseInsertFireScenario,
  removeFireScenario: parseRemoveFireScenario,
  insertFatigueDetail: parseInsertFatigueDetail,
  removeFatigueDetail: parseRemoveFatigueDetail,
  insertColdFormedSheet: parseInsertColdFormedSheet,
  removeColdFormedSheet: parseRemoveColdFormedSheet,
  insertShell: parseInsertShell,
  removeShell: parseRemoveShell,
  changeMemberBucklingLengthY: parseChangeMemberBucklingLengthY,
  changeMemberBucklingLengthT: parseChangeMemberBucklingLengthT,
  changeConnBoltsBoltsPerRow: parseChangeConnBoltsBoltsPerRow,
  changeCategoryMemberId: parseChangeCategoryMemberId,
  changeCategoryElementId: parseChangeCategoryElementId,
  changeCategoryDetailCategory: parseChangeCategoryDetailCategory,
  changeCategoryDetailType: parseChangeCategoryDetailType,
  changeCategoryLocationX: parseChangeCategoryLocationX,
  changeCategoryM1: parseChangeCategoryM1,
  changeCategoryM2: parseChangeCategoryM2,
  changePatchSubjectId: parseChangePatchSubjectId,
  changePatchPosition: parseChangePatchPosition,
  changePatchX: parseChangePatchX,
  changePatchBearingLength: parseChangePatchBearingLength,
  changePatchWebHeight: parseChangePatchWebHeight,
  changePatchWebAngle: parseChangePatchWebAngle,
  changePatchCornerRadius: parseChangePatchCornerRadius,
  changePatchLoad: parseChangePatchLoad,
  changeRingShellId: parseChangeRingShellId,
  changeRingX: parseChangeRingX,
  changeRingDepth: parseChangeRingDepth,
  changeRingThickness: parseChangeRingThickness,
  changeRingInternal: parseChangeRingInternal,
  insertFatigueCategory: parseInsertFatigueCategory,
  removeFatigueCategory: parseRemoveFatigueCategory,
  insertPatchLoad: parseInsertPatchLoad,
  removePatchLoad: parseRemovePatchLoad,
  insertRingStiffener: parseInsertRingStiffener,
  removeRingStiffener: parseRemoveRingStiffener,
  insertMemberAction: parseInsertMemberAction,
  removeMemberAction: parseRemoveMemberAction,
  insertConnectionAction: parseInsertConnectionAction,
  removeConnectionAction: parseRemoveConnectionAction,
  insertSectionElement: parseInsertSectionElement,
  removeSectionElement: parseRemoveSectionElement,
  insertSheetAction: parseInsertSheetAction,
  removeSheetAction: parseRemoveSheetAction,
  insertShellAction: parseInsertShellAction,
  removeShellAction: parseRemoveShellAction,
  changeTransverseGirderId: parseChangeTransverseGirderId,
  changeTransversePosition: parseChangeTransversePosition,
  changeTransverseWidth: parseChangeTransverseWidth,
  changeTransverseThickness: parseChangeTransverseThickness,
  changeConnLoadEccentricity: parseChangeConnLoadEccentricity,
  changeConnHoleStagger: parseChangeConnHoleStagger,
  changeTransverseSides: parseChangeTransverseSides,
  insertTransverseStiffener: parseInsertTransverseStiffener,
  removeTransverseStiffener: parseRemoveTransverseStiffener,
  changeReliabilityClass: parseChangeReliabilityClass,
});
