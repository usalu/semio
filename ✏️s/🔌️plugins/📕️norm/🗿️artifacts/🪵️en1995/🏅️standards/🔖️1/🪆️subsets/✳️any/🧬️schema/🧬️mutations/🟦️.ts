/** 🧺️ `En1995Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireExternal, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ChangeConnectionDiameter, parseChangeConnectionDiameter } from "./↔️change-connection-diameter/🧬️schema/🟦️.ts";
import { type ChangeConnectionEdgeDistance, parseChangeConnectionEdgeDistance } from "./↔️change-connection-edge-distance/🧬️schema/🟦️.ts";
import { type ChangeConnectionEndDistance, parseChangeConnectionEndDistance } from "./↔️change-connection-end-distance/🧬️schema/🟦️.ts";
import { type ChangeConnectionPlateThickness, parseChangeConnectionPlateThickness } from "./↔️change-connection-plate-thickness/🧬️schema/🟦️.ts";
import { type ChangeConnectionSpacing, parseChangeConnectionSpacing } from "./↔️change-connection-spacing/🧬️schema/🟦️.ts";
import { type ChangeConnectionT1, parseChangeConnectionT1 } from "./↔️change-connection-t1/🧬️schema/🟦️.ts";
import { type ChangeConnectionT2, parseChangeConnectionT2 } from "./↔️change-connection-t2/🧬️schema/🟦️.ts";
import { type ChangeMemberB, parseChangeMemberB } from "./↔️change-member-b/🧬️schema/🟦️.ts";
import { type ChangeMemberBearingLength, parseChangeMemberBearingLength } from "./↔️change-member-bearing-length/🧬️schema/🟦️.ts";
import { type ChangeMemberBucklingLengthY, parseChangeMemberBucklingLengthY } from "./↔️change-member-buckling-length-y/🧬️schema/🟦️.ts";
import { type ChangeMemberBucklingLengthZ, parseChangeMemberBucklingLengthZ } from "./↔️change-member-buckling-length-z/🧬️schema/🟦️.ts";
import { type ChangeMemberNotchDepth, parseChangeMemberNotchDepth } from "./↔️change-member-notch-depth/🧬️schema/🟦️.ts";
import { type ChangeMemberNotchDistance, parseChangeMemberNotchDistance } from "./↔️change-member-notch-distance/🧬️schema/🟦️.ts";
import { type ChangeMemberRestraintSpacing, parseChangeMemberRestraintSpacing } from "./↔️change-member-restraint-spacing/🧬️schema/🟦️.ts";
import { type ChangeMemberSpan, parseChangeMemberSpan } from "./↔️change-member-span/🧬️schema/🟦️.ts";
import { type ChangeMemberSupportLength, parseChangeMemberSupportLength } from "./↔️change-member-support-length/🧬️schema/🟦️.ts";
import { type ChangeMemberActionVK, parseChangeMemberActionVK } from "./↕️change-member-action-vk/🧬️schema/🟦️.ts";
import { type ChangeMemberH, parseChangeMemberH } from "./↕️change-member-h/🧬️schema/🟦️.ts";
import { type ChangeConnectionLoadDuration, parseChangeConnectionLoadDuration } from "./⏳️change-connection-load-duration/🧬️schema/🟦️.ts";
import { type ChangeMemberLoadDuration, parseChangeMemberLoadDuration } from "./⏳️change-member-load-duration/🧬️schema/🟦️.ts";
import { type ChangeConnectionActionKind, parseChangeConnectionActionKind } from "./⚖️change-connection-action-kind/🧬️schema/🟦️.ts";
import { type ChangeMemberActionKind, parseChangeMemberActionKind } from "./⚖️change-member-action-kind/🧬️schema/🟦️.ts";
import { type ChangeMemberMassKgPerM, parseChangeMemberMassKgPerM } from "./⚖️change-member-mass-kg-per-m/🧬️schema/🟦️.ts";
import { type ChangeMemberMassKgPerM2, parseChangeMemberMassKgPerM2 } from "./⚖️change-member-mass-kg-per-m2/🧬️schema/🟦️.ts";
import { type ChangeBracingBraceDepth, parseChangeBracingBraceDepth } from "./✖️change-bracing-brace-depth/🧬️schema/🟦️.ts";
import { type ChangeBracingBraceWidth, parseChangeBracingBraceWidth } from "./✖️change-bracing-brace-width/🧬️schema/🟦️.ts";
import { type ChangeBracingCompression, parseChangeBracingCompression } from "./✖️change-bracing-compression/🧬️schema/🟦️.ts";
import { type ChangeBracingKind, parseChangeBracingKind } from "./✖️change-bracing-kind/🧬️schema/🟦️.ts";
import { type ChangeBracingLength, parseChangeBracingLength } from "./✖️change-bracing-length/🧬️schema/🟦️.ts";
import { type ChangeBracingMember, parseChangeBracingMember } from "./✖️change-bracing-member/🧬️schema/🟦️.ts";
import { type ChangeBracingResistance, parseChangeBracingResistance } from "./✖️change-bracing-resistance/🧬️schema/🟦️.ts";
import { type ChangeBracingSpacing, parseChangeBracingSpacing } from "./✖️change-bracing-spacing/🧬️schema/🟦️.ts";
import { type ChangeBracingStabilisedCount, parseChangeBracingStabilisedCount } from "./✖️change-bracing-stabilised-count/🧬️schema/🟦️.ts";
import { type ChangeBracingStiffness, parseChangeBracingStiffness } from "./✖️change-bracing-stiffness/🧬️schema/🟦️.ts";
import { type InsertBracing, parseInsertBracing } from "./➕️insert-bracing/🧬️schema/🟦️.ts";
import { type InsertCase, parseInsertCase } from "./➕️insert-case/🧬️schema/🟦️.ts";
import { type InsertConnectionAction, parseInsertConnectionAction } from "./➕️insert-connection-action/🧬️schema/🟦️.ts";
import { type InsertConnection, parseInsertConnection } from "./➕️insert-connection/🧬️schema/🟦️.ts";
import { type InsertCutout, parseInsertCutout } from "./➕️insert-cutout/🧬️schema/🟦️.ts";
import { type InsertDeck, parseInsertDeck } from "./➕️insert-deck/🧬️schema/🟦️.ts";
import { type InsertDiaphragm, parseInsertDiaphragm } from "./➕️insert-diaphragm/🧬️schema/🟦️.ts";
import { type InsertFastenerPosition, parseInsertFastenerPosition } from "./➕️insert-fastener-position/🧬️schema/🟦️.ts";
import { type InsertFastener, parseInsertFastener } from "./➕️insert-fastener/🧬️schema/🟦️.ts";
import { type InsertFloor, parseInsertFloor } from "./➕️insert-floor/🧬️schema/🟦️.ts";
import { type InsertMemberAction, parseInsertMemberAction } from "./➕️insert-member-action/🧬️schema/🟦️.ts";
import { type InsertMember, parseInsertMember } from "./➕️insert-member/🧬️schema/🟦️.ts";
import { type InsertPanelLayer, parseInsertPanelLayer } from "./➕️insert-panel-layer/🧬️schema/🟦️.ts";
import { type InsertPanel, parseInsertPanel } from "./➕️insert-panel/🧬️schema/🟦️.ts";
import { type InsertPlacement, parseInsertPlacement } from "./➕️insert-placement/🧬️schema/🟦️.ts";
import { type InsertTaper, parseInsertTaper } from "./➕️insert-taper/🧬️schema/🟦️.ts";
import { type InsertTruss, parseInsertTruss } from "./➕️insert-truss/🧬️schema/🟦️.ts";
import { type RemoveBracing, parseRemoveBracing } from "./➖️remove-bracing/🧬️schema/🟦️.ts";
import { type RemoveCase, parseRemoveCase } from "./➖️remove-case/🧬️schema/🟦️.ts";
import { type RemoveConnectionAction, parseRemoveConnectionAction } from "./➖️remove-connection-action/🧬️schema/🟦️.ts";
import { type RemoveConnection, parseRemoveConnection } from "./➖️remove-connection/🧬️schema/🟦️.ts";
import { type RemoveCutout, parseRemoveCutout } from "./➖️remove-cutout/🧬️schema/🟦️.ts";
import { type RemoveDeck, parseRemoveDeck } from "./➖️remove-deck/🧬️schema/🟦️.ts";
import { type RemoveDiaphragm, parseRemoveDiaphragm } from "./➖️remove-diaphragm/🧬️schema/🟦️.ts";
import { type RemoveFastenerPosition, parseRemoveFastenerPosition } from "./➖️remove-fastener-position/🧬️schema/🟦️.ts";
import { type RemoveFastener, parseRemoveFastener } from "./➖️remove-fastener/🧬️schema/🟦️.ts";
import { type RemoveFloor, parseRemoveFloor } from "./➖️remove-floor/🧬️schema/🟦️.ts";
import { type RemoveMemberAction, parseRemoveMemberAction } from "./➖️remove-member-action/🧬️schema/🟦️.ts";
import { type RemoveMember, parseRemoveMember } from "./➖️remove-member/🧬️schema/🟦️.ts";
import { type RemovePanelLayer, parseRemovePanelLayer } from "./➖️remove-panel-layer/🧬️schema/🟦️.ts";
import { type RemovePanel, parseRemovePanel } from "./➖️remove-panel/🧬️schema/🟦️.ts";
import { type RemovePlacement, parseRemovePlacement } from "./➖️remove-placement/🧬️schema/🟦️.ts";
import { type RemoveTaper, parseRemoveTaper } from "./➖️remove-taper/🧬️schema/🟦️.ts";
import { type RemoveTruss, parseRemoveTruss } from "./➖️remove-truss/🧬️schema/🟦️.ts";
import { type ChangeMemberActionMK, parseChangeMemberActionMK } from "./⤴️change-member-action-mk/🧬️schema/🟦️.ts";
import { type ChangeMemberActionFPoint, parseChangeMemberActionFPoint } from "./⬇️change-member-action-f-point/🧬️schema/🟦️.ts";
import { type ChangeMemberActionQLine, parseChangeMemberActionQLine } from "./⬇️change-member-action-q-line/🧬️schema/🟦️.ts";
import { type ChangeDeckBarDiameter, parseChangeDeckBarDiameter } from "./🌉️change-deck-bar-diameter/🧬️schema/🟦️.ts";
import { type ChangeDeckBarProduct, parseChangeDeckBarProduct } from "./🌉️change-deck-bar-product/🧬️schema/🟦️.ts";
import { type ChangeDeckBarResistance, parseChangeDeckBarResistance } from "./🌉️change-deck-bar-resistance/🧬️schema/🟦️.ts";
import { type ChangeDeckBarSpacing, parseChangeDeckBarSpacing } from "./🌉️change-deck-bar-spacing/🧬️schema/🟦️.ts";
import { type ChangeDeckCompressionC90, parseChangeDeckCompressionC90 } from "./🌉️change-deck-compression-c90/🧬️schema/🟦️.ts";
import { type ChangeDeckDeckDepth, parseChangeDeckDeckDepth } from "./🌉️change-deck-deck-depth/🧬️schema/🟦️.ts";
import { type ChangeDeckDeckWidth, parseChangeDeckDeckWidth } from "./🌉️change-deck-deck-width/🧬️schema/🟦️.ts";
import { type ChangeDeckInitialPrestress, parseChangeDeckInitialPrestress } from "./🌉️change-deck-initial-prestress/🧬️schema/🟦️.ts";
import { type ChangeDeckLaminationProduct, parseChangeDeckLaminationProduct } from "./🌉️change-deck-lamination-product/🧬️schema/🟦️.ts";
import { type ChangeDeckLaminationWidth, parseChangeDeckLaminationWidth } from "./🌉️change-deck-lamination-width/🧬️schema/🟦️.ts";
import { type ChangeDeckLossCreep, parseChangeDeckLossCreep } from "./🌉️change-deck-loss-creep/🧬️schema/🟦️.ts";
import { type ChangeDeckLossMoisture, parseChangeDeckLossMoisture } from "./🌉️change-deck-loss-moisture/🧬️schema/🟦️.ts";
import { type ChangeDeckLossRelaxation, parseChangeDeckLossRelaxation } from "./🌉️change-deck-loss-relaxation/🧬️schema/🟦️.ts";
import { type ChangeDeckPlateSide, parseChangeDeckPlateSide } from "./🌉️change-deck-plate-side/🧬️schema/🟦️.ts";
import { type ChangeDeckShearStress, parseChangeDeckShearStress } from "./🌉️change-deck-shear-stress/🧬️schema/🟦️.ts";
import { type ChangeDeckSpan, parseChangeDeckSpan } from "./🌉️change-deck-span/🧬️schema/🟦️.ts";
import { type ChangeDeckWheelContact, parseChangeDeckWheelContact } from "./🌉️change-deck-wheel-contact/🧬️schema/🟦️.ts";
import { type ChangeDeckWheelLoad, parseChangeDeckWheelLoad } from "./🌉️change-deck-wheel-load/🧬️schema/🟦️.ts";
import { type ChangeMemberBridgeA, parseChangeMemberBridgeA } from "./🌉️change-member-bridge-a/🧬️schema/🟦️.ts";
import { type ChangeMemberBridgeB, parseChangeMemberBridgeB } from "./🌉️change-member-bridge-b/🧬️schema/🟦️.ts";
import { type ChangeMemberBridgeBeta, parseChangeMemberBridgeBeta } from "./🌉️change-member-bridge-beta/🧬️schema/🟦️.ts";
import { type ChangeMemberBridgeNObs, parseChangeMemberBridgeNObs } from "./🌉️change-member-bridge-n-obs/🧬️schema/🟦️.ts";
import { type ChangeMemberBridgeTLYears, parseChangeMemberBridgeTLYears } from "./🌉️change-member-bridge-tl-years/🧬️schema/🟦️.ts";
import { type ChangeMemberDampingXi, parseChangeMemberDampingXi } from "./🌊️change-member-damping-xi/🧬️schema/🟦️.ts";
import { type ChangeAnnex, parseChangeAnnex } from "./🌍️change-annex/🧬️schema/🟦️.ts";
import { type ChangeConnectionServiceClass, parseChangeConnectionServiceClass } from "./🌧️change-connection-service-class/🧬️schema/🟦️.ts";
import { type ChangeMemberServiceClass, parseChangeMemberServiceClass } from "./🌧️change-member-service-class/🧬️schema/🟦️.ts";
import { type ChangeReliabilityClass, parseChangeReliabilityClass } from "./🎯change-reliability-class/🧬️schema/🟦️.ts";
import { type ChangeMemberRole, parseChangeMemberRole } from "./🎯️change-member-role/🧬️schema/🟦️.ts";
import { type ChangeMemberActionFC90K, parseChangeMemberActionFC90K } from "./🏋️change-member-action-fc90-k/🧬️schema/🟦️.ts";
import { type ChangeMemberActionNK, parseChangeMemberActionNK } from "./🏋️change-member-action-nk/🧬️schema/🟦️.ts";
import { type ChangeMemberActionNTK, parseChangeMemberActionNTK } from "./🏋️change-member-action-ntk/🧬️schema/🟦️.ts";
import { type ChangeFloorDamping, parseChangeFloorDamping } from "./🏠️change-floor-damping/🧬️schema/🟦️.ts";
import { type ChangeFloorEiB, parseChangeFloorEiB } from "./🏠️change-floor-ei-b/🧬️schema/🟦️.ts";
import { type ChangeFloorEiL, parseChangeFloorEiL } from "./🏠️change-floor-ei-l/🧬️schema/🟦️.ts";
import { type ChangeFloorImposedLoad, parseChangeFloorImposedLoad } from "./🏠️change-floor-imposed-load/🧬️schema/🟦️.ts";
import { type ChangeFloorJoistSpacing, parseChangeFloorJoistSpacing } from "./🏠️change-floor-joist-spacing/🧬️schema/🟦️.ts";
import { type ChangeFloorMass, parseChangeFloorMass } from "./🏠️change-floor-mass/🧬️schema/🟦️.ts";
import { type ChangeFloorMember, parseChangeFloorMember } from "./🏠️change-floor-member/🧬️schema/🟦️.ts";
import { type ChangeFloorPermanentLoad, parseChangeFloorPermanentLoad } from "./🏠️change-floor-permanent-load/🧬️schema/🟦️.ts";
import { type ChangeFloorPsi2, parseChangeFloorPsi2 } from "./🏠️change-floor-psi2/🧬️schema/🟦️.ts";
import { type ChangeFloorSpan, parseChangeFloorSpan } from "./🏠️change-floor-span/🧬️schema/🟦️.ts";
import { type ChangeFloorWidth, parseChangeFloorWidth } from "./🏠️change-floor-width/🧬️schema/🟦️.ts";
import { type ChangeMemberActionCategory, parseChangeMemberActionCategory } from "./🏢️change-member-action-category/🧬️schema/🟦️.ts";
import { type ChangeCaseLabelDe, parseChangeCaseLabelDe } from "./🏷️change-case-label-de/🧬️schema/🟦️.ts";
import { type ChangeCaseLabelEn, parseChangeCaseLabelEn } from "./🏷️change-case-label-en/🧬️schema/🟦️.ts";
import { type ChangeConnectionLabelDe, parseChangeConnectionLabelDe } from "./🏷️change-connection-label-de/🧬️schema/🟦️.ts";
import { type ChangeConnectionLabelEn, parseChangeConnectionLabelEn } from "./🏷️change-connection-label-en/🧬️schema/🟦️.ts";
import { type ChangeMemberLabelDe, parseChangeMemberLabelDe } from "./🏷️change-member-label-de/🧬️schema/🟦️.ts";
import { type ChangeMemberLabelEn, parseChangeMemberLabelEn } from "./🏷️change-member-label-en/🧬️schema/🟦️.ts";
import { type ChangeCaseEntryId, parseChangeCaseEntryId } from "./📇️change-case-entry-id/🧬️schema/🟦️.ts";
import { type ChangeFastenerPositionX, parseChangeFastenerPositionX } from "./📍️change-fastener-position-x/🧬️schema/🟦️.ts";
import { type ChangeFastenerPositionY, parseChangeFastenerPositionY } from "./📍️change-fastener-position-y/🧬️schema/🟦️.ts";
import { type ChangeMemberSupport, parseChangeMemberSupport } from "./📍️change-member-support/🧬️schema/🟦️.ts";
import { type ChangePlacementX, parseChangePlacementX } from "./📍️change-placement-x/🧬️schema/🟦️.ts";
import { type ChangePlacementY, parseChangePlacementY } from "./📍️change-placement-y/🧬️schema/🟦️.ts";
import { type ChangePlacementYaw, parseChangePlacementYaw } from "./📍️change-placement-yaw/🧬️schema/🟦️.ts";
import { type ChangePlacementZ, parseChangePlacementZ } from "./📍️change-placement-z/🧬️schema/🟦️.ts";
import { type ChangeTaperApexAngle, parseChangeTaperApexAngle } from "./📐️change-taper-apex-angle/🧬️schema/🟦️.ts";
import { type ChangeTaperApexDepth, parseChangeTaperApexDepth } from "./📐️change-taper-apex-depth/🧬️schema/🟦️.ts";
import { type ChangeTaperInnerRadius, parseChangeTaperInnerRadius } from "./📐️change-taper-inner-radius/🧬️schema/🟦️.ts";
import { type ChangeTaperKind, parseChangeTaperKind } from "./📐️change-taper-kind/🧬️schema/🟦️.ts";
import { type ChangeTaperLamella, parseChangeTaperLamella } from "./📐️change-taper-lamella/🧬️schema/🟦️.ts";
import { type ChangeTaperMember, parseChangeTaperMember } from "./📐️change-taper-member/🧬️schema/🟦️.ts";
import { type ChangeTaperRodResistance, parseChangeTaperRodResistance } from "./📐️change-taper-rod-resistance/🧬️schema/🟦️.ts";
import { type ChangeTaperRods, parseChangeTaperRods } from "./📐️change-taper-rods/🧬️schema/🟦️.ts";
import { type ChangeTaperSide, parseChangeTaperSide } from "./📐️change-taper-side/🧬️schema/🟦️.ts";
import { type ChangeTaperTaperAngle, parseChangeTaperTaperAngle } from "./📐️change-taper-taper-angle/🧬️schema/🟦️.ts";
import { type ChangeCaseNumber, parseChangeCaseNumber } from "./🔢️change-case-number/🧬️schema/🟦️.ts";
import { type ChangeConnectionNumber, parseChangeConnectionNumber } from "./🔢️change-connection-number/🧬️schema/🟦️.ts";
import { type ChangeConnectionRows, parseChangeConnectionRows } from "./🔢️change-connection-rows/🧬️schema/🟦️.ts";
import { type ChangeConnectionShearPlanes, parseChangeConnectionShearPlanes } from "./🔢️change-connection-shear-planes/🧬️schema/🟦️.ts";
import { type ChangeCaseText, parseChangeCaseText } from "./🔤️change-case-text/🧬️schema/🟦️.ts";
import { type ChangeMemberFireDuration, parseChangeMemberFireDuration } from "./🔥️change-member-fire-duration/🧬️schema/🟦️.ts";
import { type ChangeConnectionActionFK, parseChangeConnectionActionFK } from "./🔩️change-connection-action-fk/🧬️schema/🟦️.ts";
import { type ChangeConnectionFastenerType, parseChangeConnectionFastenerType } from "./🔩️change-connection-fastener-type/🧬️schema/🟦️.ts";
import { type ChangeConnectionSteelPlate, parseChangeConnectionSteelPlate } from "./🔩️change-connection-steel-plate/🧬️schema/🟦️.ts";
import { type ChangeFastenerColumns, parseChangeFastenerColumns } from "./🔩️change-fastener-columns/🧬️schema/🟦️.ts";
import { type ChangeFastenerConnection, parseChangeFastenerConnection } from "./🔩️change-fastener-connection/🧬️schema/🟦️.ts";
import { type ChangeFastenerEdgeDistance, parseChangeFastenerEdgeDistance } from "./🔩️change-fastener-edge-distance/🧬️schema/🟦️.ts";
import { type ChangeFastenerEndDistance, parseChangeFastenerEndDistance } from "./🔩️change-fastener-end-distance/🧬️schema/🟦️.ts";
import { type ChangeFastenerLines, parseChangeFastenerLines } from "./🔩️change-fastener-lines/🧬️schema/🟦️.ts";
import { type ChangeFastenerLoadAngle, parseChangeFastenerLoadAngle } from "./🔩️change-fastener-load-angle/🧬️schema/🟦️.ts";
import { type ChangeFastenerMember, parseChangeFastenerMember } from "./🔩️change-fastener-member/🧬️schema/🟦️.ts";
import { type ChangeFastenerOffsetX, parseChangeFastenerOffsetX } from "./🔩️change-fastener-offset-x/🧬️schema/🟦️.ts";
import { type ChangeFastenerOffsetY, parseChangeFastenerOffsetY } from "./🔩️change-fastener-offset-y/🧬️schema/🟦️.ts";
import { type ChangeFastenerSpacingPar, parseChangeFastenerSpacingPar } from "./🔩️change-fastener-spacing-par/🧬️schema/🟦️.ts";
import { type ChangeFastenerSpacingPerp, parseChangeFastenerSpacingPerp } from "./🔩️change-fastener-spacing-perp/🧬️schema/🟦️.ts";
import { type ChangeTrussBays, parseChangeTrussBays } from "./🔺️change-truss-bays/🧬️schema/🟦️.ts";
import { type ChangeTrussBracingCapacity, parseChangeTrussBracingCapacity } from "./🔺️change-truss-bracing-capacity/🧬️schema/🟦️.ts";
import { type ChangeTrussChordDepth, parseChangeTrussChordDepth } from "./🔺️change-truss-chord-depth/🧬️schema/🟦️.ts";
import { type ChangeTrussChordForce, parseChangeTrussChordForce } from "./🔺️change-truss-chord-force/🧬️schema/🟦️.ts";
import { type ChangeTrussChordWidth, parseChangeTrussChordWidth } from "./🔺️change-truss-chord-width/🧬️schema/🟦️.ts";
import { type ChangeTrussHeight, parseChangeTrussHeight } from "./🔺️change-truss-height/🧬️schema/🟦️.ts";
import { type ChangeTrussKind, parseChangeTrussKind } from "./🔺️change-truss-kind/🧬️schema/🟦️.ts";
import { type ChangeTrussProduct, parseChangeTrussProduct } from "./🔺️change-truss-product/🧬️schema/🟦️.ts";
import { type ChangeTrussRestraintLength, parseChangeTrussRestraintLength } from "./🔺️change-truss-restraint-length/🧬️schema/🟦️.ts";
import { type ChangeTrussSpan, parseChangeTrussSpan } from "./🔺️change-truss-span/🧬️schema/🟦️.ts";
import { type ChangeTrussWebDepth, parseChangeTrussWebDepth } from "./🔺️change-truss-web-depth/🧬️schema/🟦️.ts";
import { type ChangeTrussWebWidth, parseChangeTrussWebWidth } from "./🔺️change-truss-web-width/🧬️schema/🟦️.ts";
import { type ChangeCutoutCentreHeight, parseChangeCutoutCentreHeight } from "./🕳️change-cutout-centre-height/🧬️schema/🟦️.ts";
import { type ChangeCutoutDepth, parseChangeCutoutDepth } from "./🕳️change-cutout-depth/🧬️schema/🟦️.ts";
import { type ChangeCutoutDistance, parseChangeCutoutDistance } from "./🕳️change-cutout-distance/🧬️schema/🟦️.ts";
import { type ChangeCutoutFace, parseChangeCutoutFace } from "./🕳️change-cutout-face/🧬️schema/🟦️.ts";
import { type ChangeCutoutKind, parseChangeCutoutKind } from "./🕳️change-cutout-kind/🧬️schema/🟦️.ts";
import { type ChangeCutoutLength, parseChangeCutoutLength } from "./🕳️change-cutout-length/🧬️schema/🟦️.ts";
import { type ChangeCutoutMember, parseChangeCutoutMember } from "./🕳️change-cutout-member/🧬️schema/🟦️.ts";
import { type ChangeCutoutRodResistance, parseChangeCutoutRodResistance } from "./🕳️change-cutout-rod-resistance/🧬️schema/🟦️.ts";
import { type ChangeCutoutRods, parseChangeCutoutRods } from "./🕳️change-cutout-rods/🧬️schema/🟦️.ts";
import { type ChangeCutoutSlope, parseChangeCutoutSlope } from "./🕳️change-cutout-slope/🧬️schema/🟦️.ts";
import { type ChangeMemberBridgeCrowd, parseChangeMemberBridgeCrowd } from "./🚶️change-member-bridge-crowd/🧬️schema/🟦️.ts";
import { type ChangeConnectionFUK, parseChangeConnectionFUK } from "./🛡️change-connection-fuk/🧬️schema/🟦️.ts";
import { type ChangeConnectionStrengthClass, parseChangeConnectionStrengthClass } from "./🛡️change-connection-strength-class/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmFastenerCapacity, parseChangeDiaphragmFastenerCapacity } from "./🛡️change-diaphragm-fastener-capacity/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmFastenerSpacing, parseChangeDiaphragmFastenerSpacing } from "./🛡️change-diaphragm-fastener-spacing/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmForce, parseChangeDiaphragmForce } from "./🛡️change-diaphragm-force/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmHoldDown, parseChangeDiaphragmHoldDown } from "./🛡️change-diaphragm-hold-down/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmKind, parseChangeDiaphragmKind } from "./🛡️change-diaphragm-kind/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmLength, parseChangeDiaphragmLength } from "./🛡️change-diaphragm-length/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmMethod, parseChangeDiaphragmMethod } from "./🛡️change-diaphragm-method/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmPanelWidth, parseChangeDiaphragmPanelWidth } from "./🛡️change-diaphragm-panel-width/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmPanels, parseChangeDiaphragmPanels } from "./🛡️change-diaphragm-panels/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmShearEd, parseChangeDiaphragmShearEd } from "./🛡️change-diaphragm-shear-ed/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmShearResistance, parseChangeDiaphragmShearResistance } from "./🛡️change-diaphragm-shear-resistance/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmWallHeight, parseChangeDiaphragmWallHeight } from "./🛡️change-diaphragm-wall-height/🧬️schema/🟦️.ts";
import { type ChangeMemberStrengthClass, parseChangeMemberStrengthClass } from "./🛡️change-member-strength-class/🧬️schema/🟦️.ts";
import { type ChangeUlsEquation, parseChangeUlsEquation } from "./🧮️change-uls-equation/🧬️schema/🟦️.ts";
import { type ChangePanelFireDuration, parseChangePanelFireDuration } from "./🧱️change-panel-fire-duration/🧬️schema/🟦️.ts";
import { type ChangePanelLayerGrade, parseChangePanelLayerGrade } from "./🧱️change-panel-layer-grade/🧬️schema/🟦️.ts";
import { type ChangePanelLayerOrientation, parseChangePanelLayerOrientation } from "./🧱️change-panel-layer-orientation/🧬️schema/🟦️.ts";
import { type ChangePanelLayerThickness, parseChangePanelLayerThickness } from "./🧱️change-panel-layer-thickness/🧬️schema/🟦️.ts";
import { type ChangePanelLoad, parseChangePanelLoad } from "./🧱️change-panel-load/🧬️schema/🟦️.ts";
import { type ChangePanelSpan, parseChangePanelSpan } from "./🧱️change-panel-span/🧬️schema/🟦️.ts";
import { type ChangePanelStrengthClass, parseChangePanelStrengthClass } from "./🧱️change-panel-strength-class/🧬️schema/🟦️.ts";
import { type ChangePanelWidth, parseChangePanelWidth } from "./🧱️change-panel-width/🧬️schema/🟦️.ts";

export type En1995Mutation =
  | { ChangeAnnex: ChangeAnnex }
  | { ChangeReliabilityClass: ChangeReliabilityClass }
  | { InsertMember: InsertMember }
  | { RemoveMember: RemoveMember }
  | { ChangeMemberLabelEn: ChangeMemberLabelEn }
  | { ChangeMemberLabelDe: ChangeMemberLabelDe }
  | { ChangeMemberRole: ChangeMemberRole }
  | { ChangeMemberStrengthClass: ChangeMemberStrengthClass }
  | { ChangeMemberServiceClass: ChangeMemberServiceClass }
  | { ChangeMemberSupport: ChangeMemberSupport }
  | { ChangeMemberB: ChangeMemberB }
  | { ChangeMemberH: ChangeMemberH }
  | { ChangeMemberSpan: ChangeMemberSpan }
  | { ChangeMemberSupportLength: ChangeMemberSupportLength }
  | { ChangeMemberBearingLength: ChangeMemberBearingLength }
  | { ChangeMemberBucklingLengthY: ChangeMemberBucklingLengthY }
  | { ChangeMemberBucklingLengthZ: ChangeMemberBucklingLengthZ }
  | { ChangeMemberRestraintSpacing: ChangeMemberRestraintSpacing }
  | { ChangeMemberNotchDepth: ChangeMemberNotchDepth }
  | { ChangeMemberNotchDistance: ChangeMemberNotchDistance }
  | { ChangeMemberMassKgPerM: ChangeMemberMassKgPerM }
  | { ChangeMemberMassKgPerM2: ChangeMemberMassKgPerM2 }
  | { ChangeMemberDampingXi: ChangeMemberDampingXi }
  | { ChangeMemberFireDuration: ChangeMemberFireDuration }
  | { ChangeMemberBridgeNObs: ChangeMemberBridgeNObs }
  | { ChangeMemberBridgeTLYears: ChangeMemberBridgeTLYears }
  | { ChangeMemberBridgeBeta: ChangeMemberBridgeBeta }
  | { ChangeMemberBridgeA: ChangeMemberBridgeA }
  | { ChangeMemberBridgeB: ChangeMemberBridgeB }
  | { ChangeMemberBridgeCrowd: ChangeMemberBridgeCrowd }
  | { InsertMemberAction: InsertMemberAction }
  | { RemoveMemberAction: RemoveMemberAction }
  | { ChangeMemberActionKind: ChangeMemberActionKind }
  | { ChangeMemberActionCategory: ChangeMemberActionCategory }
  | { ChangeMemberLoadDuration: ChangeMemberLoadDuration }
  | { ChangeMemberActionQLine: ChangeMemberActionQLine }
  | { ChangeMemberActionFPoint: ChangeMemberActionFPoint }
  | { ChangeMemberActionMK: ChangeMemberActionMK }
  | { ChangeMemberActionVK: ChangeMemberActionVK }
  | { ChangeMemberActionNK: ChangeMemberActionNK }
  | { ChangeMemberActionNTK: ChangeMemberActionNTK }
  | { ChangeMemberActionFC90K: ChangeMemberActionFC90K }
  | { InsertConnection: InsertConnection }
  | { RemoveConnection: RemoveConnection }
  | { ChangeConnectionLabelEn: ChangeConnectionLabelEn }
  | { ChangeConnectionLabelDe: ChangeConnectionLabelDe }
  | { ChangeConnectionFastenerType: ChangeConnectionFastenerType }
  | { ChangeConnectionStrengthClass: ChangeConnectionStrengthClass }
  | { ChangeConnectionServiceClass: ChangeConnectionServiceClass }
  | { ChangeConnectionDiameter: ChangeConnectionDiameter }
  | { ChangeConnectionNumber: ChangeConnectionNumber }
  | { ChangeConnectionRows: ChangeConnectionRows }
  | { ChangeConnectionSpacing: ChangeConnectionSpacing }
  | { ChangeConnectionEdgeDistance: ChangeConnectionEdgeDistance }
  | { ChangeConnectionEndDistance: ChangeConnectionEndDistance }
  | { ChangeConnectionT1: ChangeConnectionT1 }
  | { ChangeConnectionT2: ChangeConnectionT2 }
  | { ChangeConnectionSteelPlate: ChangeConnectionSteelPlate }
  | { ChangeConnectionPlateThickness: ChangeConnectionPlateThickness }
  | { ChangeConnectionShearPlanes: ChangeConnectionShearPlanes }
  | { ChangeConnectionFUK: ChangeConnectionFUK }
  | { InsertConnectionAction: InsertConnectionAction }
  | { RemoveConnectionAction: RemoveConnectionAction }
  | { ChangeConnectionActionKind: ChangeConnectionActionKind }
  | { ChangeConnectionLoadDuration: ChangeConnectionLoadDuration }
  | { ChangeConnectionActionFK: ChangeConnectionActionFK }
  | { ChangeUlsEquation: ChangeUlsEquation }
  | { InsertCase: InsertCase }
  | { RemoveCase: RemoveCase }
  | { ChangeCaseLabelEn: ChangeCaseLabelEn }
  | { ChangeCaseLabelDe: ChangeCaseLabelDe }
  | { ChangeCaseEntryId: ChangeCaseEntryId }
  | { ChangeCaseNumber: ChangeCaseNumber }
  | { ChangeCaseText: ChangeCaseText }
  | { ChangePlacementX: ChangePlacementX }
  | { ChangePlacementY: ChangePlacementY }
  | { ChangePlacementZ: ChangePlacementZ }
  | { ChangePlacementYaw: ChangePlacementYaw }
  | { InsertPlacement: InsertPlacement }
  | { RemovePlacement: RemovePlacement }
  | { InsertTaper: InsertTaper }
  | { RemoveTaper: RemoveTaper }
  | { ChangeTaperMember: ChangeTaperMember }
  | { ChangeTaperKind: ChangeTaperKind }
  | { ChangeTaperTaperAngle: ChangeTaperTaperAngle }
  | { ChangeTaperApexDepth: ChangeTaperApexDepth }
  | { ChangeTaperInnerRadius: ChangeTaperInnerRadius }
  | { ChangeTaperLamella: ChangeTaperLamella }
  | { ChangeTaperApexAngle: ChangeTaperApexAngle }
  | { ChangeTaperSide: ChangeTaperSide }
  | { ChangeTaperRods: ChangeTaperRods }
  | { ChangeTaperRodResistance: ChangeTaperRodResistance }
  | { InsertCutout: InsertCutout }
  | { RemoveCutout: RemoveCutout }
  | { ChangeCutoutMember: ChangeCutoutMember }
  | { ChangeCutoutKind: ChangeCutoutKind }
  | { ChangeCutoutFace: ChangeCutoutFace }
  | { ChangeCutoutDistance: ChangeCutoutDistance }
  | { ChangeCutoutLength: ChangeCutoutLength }
  | { ChangeCutoutDepth: ChangeCutoutDepth }
  | { ChangeCutoutCentreHeight: ChangeCutoutCentreHeight }
  | { ChangeCutoutSlope: ChangeCutoutSlope }
  | { ChangeCutoutRods: ChangeCutoutRods }
  | { ChangeCutoutRodResistance: ChangeCutoutRodResistance }
  | { InsertFastener: InsertFastener }
  | { RemoveFastener: RemoveFastener }
  | { ChangeFastenerConnection: ChangeFastenerConnection }
  | { ChangeFastenerMember: ChangeFastenerMember }
  | { ChangeFastenerColumns: ChangeFastenerColumns }
  | { ChangeFastenerLines: ChangeFastenerLines }
  | { ChangeFastenerSpacingPar: ChangeFastenerSpacingPar }
  | { ChangeFastenerSpacingPerp: ChangeFastenerSpacingPerp }
  | { ChangeFastenerEndDistance: ChangeFastenerEndDistance }
  | { ChangeFastenerEdgeDistance: ChangeFastenerEdgeDistance }
  | { ChangeFastenerOffsetX: ChangeFastenerOffsetX }
  | { ChangeFastenerOffsetY: ChangeFastenerOffsetY }
  | { ChangeFastenerLoadAngle: ChangeFastenerLoadAngle }
  | { InsertPanel: InsertPanel }
  | { RemovePanel: RemovePanel }
  | { ChangePanelStrengthClass: ChangePanelStrengthClass }
  | { ChangePanelSpan: ChangePanelSpan }
  | { ChangePanelWidth: ChangePanelWidth }
  | { ChangePanelLoad: ChangePanelLoad }
  | { ChangePanelFireDuration: ChangePanelFireDuration }
  | { InsertFloor: InsertFloor }
  | { RemoveFloor: RemoveFloor }
  | { ChangeFloorMember: ChangeFloorMember }
  | { ChangeFloorSpan: ChangeFloorSpan }
  | { ChangeFloorWidth: ChangeFloorWidth }
  | { ChangeFloorJoistSpacing: ChangeFloorJoistSpacing }
  | { ChangeFloorEiL: ChangeFloorEiL }
  | { ChangeFloorEiB: ChangeFloorEiB }
  | { ChangeFloorMass: ChangeFloorMass }
  | { ChangeFloorDamping: ChangeFloorDamping }
  | { ChangeFloorPermanentLoad: ChangeFloorPermanentLoad }
  | { ChangeFloorImposedLoad: ChangeFloorImposedLoad }
  | { ChangeFloorPsi2: ChangeFloorPsi2 }
  | { InsertPanelLayer: InsertPanelLayer }
  | { RemovePanelLayer: RemovePanelLayer }
  | { ChangePanelLayerThickness: ChangePanelLayerThickness }
  | { ChangePanelLayerOrientation: ChangePanelLayerOrientation }
  | { ChangePanelLayerGrade: ChangePanelLayerGrade }
  | { InsertFastenerPosition: InsertFastenerPosition }
  | { RemoveFastenerPosition: RemoveFastenerPosition }
  | { ChangeFastenerPositionX: ChangeFastenerPositionX }
  | { ChangeFastenerPositionY: ChangeFastenerPositionY }
  | { InsertDiaphragm: InsertDiaphragm }
  | { RemoveDiaphragm: RemoveDiaphragm }
  | { ChangeDiaphragmKind: ChangeDiaphragmKind }
  | { ChangeDiaphragmMethod: ChangeDiaphragmMethod }
  | { ChangeDiaphragmPanelWidth: ChangeDiaphragmPanelWidth }
  | { ChangeDiaphragmWallHeight: ChangeDiaphragmWallHeight }
  | { ChangeDiaphragmFastenerSpacing: ChangeDiaphragmFastenerSpacing }
  | { ChangeDiaphragmPanels: ChangeDiaphragmPanels }
  | { ChangeDiaphragmFastenerCapacity: ChangeDiaphragmFastenerCapacity }
  | { ChangeDiaphragmShearEd: ChangeDiaphragmShearEd }
  | { ChangeDiaphragmHoldDown: ChangeDiaphragmHoldDown }
  | { ChangeDiaphragmForce: ChangeDiaphragmForce }
  | { ChangeDiaphragmLength: ChangeDiaphragmLength }
  | { ChangeDiaphragmShearResistance: ChangeDiaphragmShearResistance }
  | { InsertTruss: InsertTruss }
  | { RemoveTruss: RemoveTruss }
  | { ChangeTrussKind: ChangeTrussKind }
  | { ChangeTrussProduct: ChangeTrussProduct }
  | { ChangeTrussSpan: ChangeTrussSpan }
  | { ChangeTrussHeight: ChangeTrussHeight }
  | { ChangeTrussBays: ChangeTrussBays }
  | { ChangeTrussChordWidth: ChangeTrussChordWidth }
  | { ChangeTrussChordDepth: ChangeTrussChordDepth }
  | { ChangeTrussWebWidth: ChangeTrussWebWidth }
  | { ChangeTrussWebDepth: ChangeTrussWebDepth }
  | { ChangeTrussChordForce: ChangeTrussChordForce }
  | { ChangeTrussRestraintLength: ChangeTrussRestraintLength }
  | { ChangeTrussBracingCapacity: ChangeTrussBracingCapacity }
  | { InsertBracing: InsertBracing }
  | { RemoveBracing: RemoveBracing }
  | { ChangeBracingKind: ChangeBracingKind }
  | { ChangeBracingMember: ChangeBracingMember }
  | { ChangeBracingStabilisedCount: ChangeBracingStabilisedCount }
  | { ChangeBracingSpacing: ChangeBracingSpacing }
  | { ChangeBracingLength: ChangeBracingLength }
  | { ChangeBracingBraceWidth: ChangeBracingBraceWidth }
  | { ChangeBracingBraceDepth: ChangeBracingBraceDepth }
  | { ChangeBracingCompression: ChangeBracingCompression }
  | { ChangeBracingResistance: ChangeBracingResistance }
  | { ChangeBracingStiffness: ChangeBracingStiffness }
  | { InsertDeck: InsertDeck }
  | { RemoveDeck: RemoveDeck }
  | { ChangeDeckLaminationProduct: ChangeDeckLaminationProduct }
  | { ChangeDeckBarProduct: ChangeDeckBarProduct }
  | { ChangeDeckSpan: ChangeDeckSpan }
  | { ChangeDeckDeckWidth: ChangeDeckDeckWidth }
  | { ChangeDeckDeckDepth: ChangeDeckDeckDepth }
  | { ChangeDeckLaminationWidth: ChangeDeckLaminationWidth }
  | { ChangeDeckBarSpacing: ChangeDeckBarSpacing }
  | { ChangeDeckBarDiameter: ChangeDeckBarDiameter }
  | { ChangeDeckPlateSide: ChangeDeckPlateSide }
  | { ChangeDeckInitialPrestress: ChangeDeckInitialPrestress }
  | { ChangeDeckLossRelaxation: ChangeDeckLossRelaxation }
  | { ChangeDeckLossCreep: ChangeDeckLossCreep }
  | { ChangeDeckLossMoisture: ChangeDeckLossMoisture }
  | { ChangeDeckBarResistance: ChangeDeckBarResistance }
  | { ChangeDeckShearStress: ChangeDeckShearStress }
  | { ChangeDeckCompressionC90: ChangeDeckCompressionC90 }
  | { ChangeDeckWheelLoad: ChangeDeckWheelLoad }
  | { ChangeDeckWheelContact: ChangeDeckWheelContact };

export const parseEn1995Mutation: NormWireReader<En1995Mutation> = normWireExternal<En1995Mutation>({
  ChangeAnnex: parseChangeAnnex,
  ChangeReliabilityClass: parseChangeReliabilityClass,
  InsertMember: parseInsertMember,
  RemoveMember: parseRemoveMember,
  ChangeMemberLabelEn: parseChangeMemberLabelEn,
  ChangeMemberLabelDe: parseChangeMemberLabelDe,
  ChangeMemberRole: parseChangeMemberRole,
  ChangeMemberStrengthClass: parseChangeMemberStrengthClass,
  ChangeMemberServiceClass: parseChangeMemberServiceClass,
  ChangeMemberSupport: parseChangeMemberSupport,
  ChangeMemberB: parseChangeMemberB,
  ChangeMemberH: parseChangeMemberH,
  ChangeMemberSpan: parseChangeMemberSpan,
  ChangeMemberSupportLength: parseChangeMemberSupportLength,
  ChangeMemberBearingLength: parseChangeMemberBearingLength,
  ChangeMemberBucklingLengthY: parseChangeMemberBucklingLengthY,
  ChangeMemberBucklingLengthZ: parseChangeMemberBucklingLengthZ,
  ChangeMemberRestraintSpacing: parseChangeMemberRestraintSpacing,
  ChangeMemberNotchDepth: parseChangeMemberNotchDepth,
  ChangeMemberNotchDistance: parseChangeMemberNotchDistance,
  ChangeMemberMassKgPerM: parseChangeMemberMassKgPerM,
  ChangeMemberMassKgPerM2: parseChangeMemberMassKgPerM2,
  ChangeMemberDampingXi: parseChangeMemberDampingXi,
  ChangeMemberFireDuration: parseChangeMemberFireDuration,
  ChangeMemberBridgeNObs: parseChangeMemberBridgeNObs,
  ChangeMemberBridgeTLYears: parseChangeMemberBridgeTLYears,
  ChangeMemberBridgeBeta: parseChangeMemberBridgeBeta,
  ChangeMemberBridgeA: parseChangeMemberBridgeA,
  ChangeMemberBridgeB: parseChangeMemberBridgeB,
  ChangeMemberBridgeCrowd: parseChangeMemberBridgeCrowd,
  InsertMemberAction: parseInsertMemberAction,
  RemoveMemberAction: parseRemoveMemberAction,
  ChangeMemberActionKind: parseChangeMemberActionKind,
  ChangeMemberActionCategory: parseChangeMemberActionCategory,
  ChangeMemberLoadDuration: parseChangeMemberLoadDuration,
  ChangeMemberActionQLine: parseChangeMemberActionQLine,
  ChangeMemberActionFPoint: parseChangeMemberActionFPoint,
  ChangeMemberActionMK: parseChangeMemberActionMK,
  ChangeMemberActionVK: parseChangeMemberActionVK,
  ChangeMemberActionNK: parseChangeMemberActionNK,
  ChangeMemberActionNTK: parseChangeMemberActionNTK,
  ChangeMemberActionFC90K: parseChangeMemberActionFC90K,
  InsertConnection: parseInsertConnection,
  RemoveConnection: parseRemoveConnection,
  ChangeConnectionLabelEn: parseChangeConnectionLabelEn,
  ChangeConnectionLabelDe: parseChangeConnectionLabelDe,
  ChangeConnectionFastenerType: parseChangeConnectionFastenerType,
  ChangeConnectionStrengthClass: parseChangeConnectionStrengthClass,
  ChangeConnectionServiceClass: parseChangeConnectionServiceClass,
  ChangeConnectionDiameter: parseChangeConnectionDiameter,
  ChangeConnectionNumber: parseChangeConnectionNumber,
  ChangeConnectionRows: parseChangeConnectionRows,
  ChangeConnectionSpacing: parseChangeConnectionSpacing,
  ChangeConnectionEdgeDistance: parseChangeConnectionEdgeDistance,
  ChangeConnectionEndDistance: parseChangeConnectionEndDistance,
  ChangeConnectionT1: parseChangeConnectionT1,
  ChangeConnectionT2: parseChangeConnectionT2,
  ChangeConnectionSteelPlate: parseChangeConnectionSteelPlate,
  ChangeConnectionPlateThickness: parseChangeConnectionPlateThickness,
  ChangeConnectionShearPlanes: parseChangeConnectionShearPlanes,
  ChangeConnectionFUK: parseChangeConnectionFUK,
  InsertConnectionAction: parseInsertConnectionAction,
  RemoveConnectionAction: parseRemoveConnectionAction,
  ChangeConnectionActionKind: parseChangeConnectionActionKind,
  ChangeConnectionLoadDuration: parseChangeConnectionLoadDuration,
  ChangeConnectionActionFK: parseChangeConnectionActionFK,
  ChangeUlsEquation: parseChangeUlsEquation,
  InsertCase: parseInsertCase,
  RemoveCase: parseRemoveCase,
  ChangeCaseLabelEn: parseChangeCaseLabelEn,
  ChangeCaseLabelDe: parseChangeCaseLabelDe,
  ChangeCaseEntryId: parseChangeCaseEntryId,
  ChangeCaseNumber: parseChangeCaseNumber,
  ChangeCaseText: parseChangeCaseText,
  ChangePlacementX: parseChangePlacementX,
  ChangePlacementY: parseChangePlacementY,
  ChangePlacementZ: parseChangePlacementZ,
  ChangePlacementYaw: parseChangePlacementYaw,
  InsertPlacement: parseInsertPlacement,
  RemovePlacement: parseRemovePlacement,
  InsertTaper: parseInsertTaper,
  RemoveTaper: parseRemoveTaper,
  ChangeTaperMember: parseChangeTaperMember,
  ChangeTaperKind: parseChangeTaperKind,
  ChangeTaperTaperAngle: parseChangeTaperTaperAngle,
  ChangeTaperApexDepth: parseChangeTaperApexDepth,
  ChangeTaperInnerRadius: parseChangeTaperInnerRadius,
  ChangeTaperLamella: parseChangeTaperLamella,
  ChangeTaperApexAngle: parseChangeTaperApexAngle,
  ChangeTaperSide: parseChangeTaperSide,
  ChangeTaperRods: parseChangeTaperRods,
  ChangeTaperRodResistance: parseChangeTaperRodResistance,
  InsertCutout: parseInsertCutout,
  RemoveCutout: parseRemoveCutout,
  ChangeCutoutMember: parseChangeCutoutMember,
  ChangeCutoutKind: parseChangeCutoutKind,
  ChangeCutoutFace: parseChangeCutoutFace,
  ChangeCutoutDistance: parseChangeCutoutDistance,
  ChangeCutoutLength: parseChangeCutoutLength,
  ChangeCutoutDepth: parseChangeCutoutDepth,
  ChangeCutoutCentreHeight: parseChangeCutoutCentreHeight,
  ChangeCutoutSlope: parseChangeCutoutSlope,
  ChangeCutoutRods: parseChangeCutoutRods,
  ChangeCutoutRodResistance: parseChangeCutoutRodResistance,
  InsertFastener: parseInsertFastener,
  RemoveFastener: parseRemoveFastener,
  ChangeFastenerConnection: parseChangeFastenerConnection,
  ChangeFastenerMember: parseChangeFastenerMember,
  ChangeFastenerColumns: parseChangeFastenerColumns,
  ChangeFastenerLines: parseChangeFastenerLines,
  ChangeFastenerSpacingPar: parseChangeFastenerSpacingPar,
  ChangeFastenerSpacingPerp: parseChangeFastenerSpacingPerp,
  ChangeFastenerEndDistance: parseChangeFastenerEndDistance,
  ChangeFastenerEdgeDistance: parseChangeFastenerEdgeDistance,
  ChangeFastenerOffsetX: parseChangeFastenerOffsetX,
  ChangeFastenerOffsetY: parseChangeFastenerOffsetY,
  ChangeFastenerLoadAngle: parseChangeFastenerLoadAngle,
  InsertPanel: parseInsertPanel,
  RemovePanel: parseRemovePanel,
  ChangePanelStrengthClass: parseChangePanelStrengthClass,
  ChangePanelSpan: parseChangePanelSpan,
  ChangePanelWidth: parseChangePanelWidth,
  ChangePanelLoad: parseChangePanelLoad,
  ChangePanelFireDuration: parseChangePanelFireDuration,
  InsertFloor: parseInsertFloor,
  RemoveFloor: parseRemoveFloor,
  ChangeFloorMember: parseChangeFloorMember,
  ChangeFloorSpan: parseChangeFloorSpan,
  ChangeFloorWidth: parseChangeFloorWidth,
  ChangeFloorJoistSpacing: parseChangeFloorJoistSpacing,
  ChangeFloorEiL: parseChangeFloorEiL,
  ChangeFloorEiB: parseChangeFloorEiB,
  ChangeFloorMass: parseChangeFloorMass,
  ChangeFloorDamping: parseChangeFloorDamping,
  ChangeFloorPermanentLoad: parseChangeFloorPermanentLoad,
  ChangeFloorImposedLoad: parseChangeFloorImposedLoad,
  ChangeFloorPsi2: parseChangeFloorPsi2,
  InsertPanelLayer: parseInsertPanelLayer,
  RemovePanelLayer: parseRemovePanelLayer,
  ChangePanelLayerThickness: parseChangePanelLayerThickness,
  ChangePanelLayerOrientation: parseChangePanelLayerOrientation,
  ChangePanelLayerGrade: parseChangePanelLayerGrade,
  InsertFastenerPosition: parseInsertFastenerPosition,
  RemoveFastenerPosition: parseRemoveFastenerPosition,
  ChangeFastenerPositionX: parseChangeFastenerPositionX,
  ChangeFastenerPositionY: parseChangeFastenerPositionY,
  InsertDiaphragm: parseInsertDiaphragm,
  RemoveDiaphragm: parseRemoveDiaphragm,
  ChangeDiaphragmKind: parseChangeDiaphragmKind,
  ChangeDiaphragmMethod: parseChangeDiaphragmMethod,
  ChangeDiaphragmPanelWidth: parseChangeDiaphragmPanelWidth,
  ChangeDiaphragmWallHeight: parseChangeDiaphragmWallHeight,
  ChangeDiaphragmFastenerSpacing: parseChangeDiaphragmFastenerSpacing,
  ChangeDiaphragmPanels: parseChangeDiaphragmPanels,
  ChangeDiaphragmFastenerCapacity: parseChangeDiaphragmFastenerCapacity,
  ChangeDiaphragmShearEd: parseChangeDiaphragmShearEd,
  ChangeDiaphragmHoldDown: parseChangeDiaphragmHoldDown,
  ChangeDiaphragmForce: parseChangeDiaphragmForce,
  ChangeDiaphragmLength: parseChangeDiaphragmLength,
  ChangeDiaphragmShearResistance: parseChangeDiaphragmShearResistance,
  InsertTruss: parseInsertTruss,
  RemoveTruss: parseRemoveTruss,
  ChangeTrussKind: parseChangeTrussKind,
  ChangeTrussProduct: parseChangeTrussProduct,
  ChangeTrussSpan: parseChangeTrussSpan,
  ChangeTrussHeight: parseChangeTrussHeight,
  ChangeTrussBays: parseChangeTrussBays,
  ChangeTrussChordWidth: parseChangeTrussChordWidth,
  ChangeTrussChordDepth: parseChangeTrussChordDepth,
  ChangeTrussWebWidth: parseChangeTrussWebWidth,
  ChangeTrussWebDepth: parseChangeTrussWebDepth,
  ChangeTrussChordForce: parseChangeTrussChordForce,
  ChangeTrussRestraintLength: parseChangeTrussRestraintLength,
  ChangeTrussBracingCapacity: parseChangeTrussBracingCapacity,
  InsertBracing: parseInsertBracing,
  RemoveBracing: parseRemoveBracing,
  ChangeBracingKind: parseChangeBracingKind,
  ChangeBracingMember: parseChangeBracingMember,
  ChangeBracingStabilisedCount: parseChangeBracingStabilisedCount,
  ChangeBracingSpacing: parseChangeBracingSpacing,
  ChangeBracingLength: parseChangeBracingLength,
  ChangeBracingBraceWidth: parseChangeBracingBraceWidth,
  ChangeBracingBraceDepth: parseChangeBracingBraceDepth,
  ChangeBracingCompression: parseChangeBracingCompression,
  ChangeBracingResistance: parseChangeBracingResistance,
  ChangeBracingStiffness: parseChangeBracingStiffness,
  InsertDeck: parseInsertDeck,
  RemoveDeck: parseRemoveDeck,
  ChangeDeckLaminationProduct: parseChangeDeckLaminationProduct,
  ChangeDeckBarProduct: parseChangeDeckBarProduct,
  ChangeDeckSpan: parseChangeDeckSpan,
  ChangeDeckDeckWidth: parseChangeDeckDeckWidth,
  ChangeDeckDeckDepth: parseChangeDeckDeckDepth,
  ChangeDeckLaminationWidth: parseChangeDeckLaminationWidth,
  ChangeDeckBarSpacing: parseChangeDeckBarSpacing,
  ChangeDeckBarDiameter: parseChangeDeckBarDiameter,
  ChangeDeckPlateSide: parseChangeDeckPlateSide,
  ChangeDeckInitialPrestress: parseChangeDeckInitialPrestress,
  ChangeDeckLossRelaxation: parseChangeDeckLossRelaxation,
  ChangeDeckLossCreep: parseChangeDeckLossCreep,
  ChangeDeckLossMoisture: parseChangeDeckLossMoisture,
  ChangeDeckBarResistance: parseChangeDeckBarResistance,
  ChangeDeckShearStress: parseChangeDeckShearStress,
  ChangeDeckCompressionC90: parseChangeDeckCompressionC90,
  ChangeDeckWheelLoad: parseChangeDeckWheelLoad,
  ChangeDeckWheelContact: parseChangeDeckWheelContact,
});
