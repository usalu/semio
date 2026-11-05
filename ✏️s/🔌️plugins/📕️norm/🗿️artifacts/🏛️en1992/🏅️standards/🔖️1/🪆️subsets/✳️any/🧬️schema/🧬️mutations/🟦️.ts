/** 🧺️ `En1992Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireExternal, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ChangeBarLayerCount, parseChangeBarLayerCount } from "./#️⃣change-bar-layer-count/🧬️schema/🟦️.ts";
import { type ChangeMemberWidth, parseChangeMemberWidth } from "./↔️change-member-width/🧬️schema/🟦️.ts";
import { type ChangeMemberHeight, parseChangeMemberHeight } from "./↕️change-member-height/🧬️schema/🟦️.ts";
import { type ChangeActionVk, parseChangeActionVk } from "./↘️change-action-vk/🧬️schema/🟦️.ts";
import { type ChangeAnchorC1, parseChangeAnchorC1 } from "./⚓️change-anchor-c1/🧬️schema/🟦️.ts";
import { type ChangeAnchorCracked, parseChangeAnchorCracked } from "./⚓️change-anchor-cracked/🧬️schema/🟦️.ts";
import { type ChangeAnchorD, parseChangeAnchorD } from "./⚓️change-anchor-d/🧬️schema/🟦️.ts";
import { type ChangeAnchorFCk, parseChangeAnchorFCk } from "./⚓️change-anchor-f-ck/🧬️schema/🟦️.ts";
import { type ChangeAnchorFUk, parseChangeAnchorFUk } from "./⚓️change-anchor-f-uk/🧬️schema/🟦️.ts";
import { type ChangeAnchorFYk, parseChangeAnchorFYk } from "./⚓️change-anchor-f-yk/🧬️schema/🟦️.ts";
import { type ChangeAnchorHeadDiameter, parseChangeAnchorHeadDiameter } from "./⚓️change-anchor-head-diameter/🧬️schema/🟦️.ts";
import { type InsertAnchor, parseInsertAnchor } from "./⚓️insert-anchor/🧬️schema/🟦️.ts";
import { type InsertCase, parseInsertCase } from "./➕️insert-case/🧬️schema/🟦️.ts";
import { type InsertMember, parseInsertMember } from "./➕️insert-member/🧬️schema/🟦️.ts";
import { type RemoveCase, parseRemoveCase } from "./➖️remove-case/🧬️schema/🟦️.ts";
import { type RemoveMember, parseRemoveMember } from "./➖️remove-member/🧬️schema/🟦️.ts";
import { type ChangeActionMk, parseChangeActionMk } from "./⤴️change-action-mk/🧬️schema/🟦️.ts";
import { type ChangeBarLayerDiameter, parseChangeBarLayerDiameter } from "./⭕change-bar-layer-diameter/🧬️schema/🟦️.ts";
import { type ChangeBarLayerAggregateSize, parseChangeBarLayerAggregateSize } from "./⭕️change-bar-layer-aggregate-size/🧬️schema/🟦️.ts";
import { type ChangeBarLayerAnchorageLength, parseChangeBarLayerAnchorageLength } from "./⭕️change-bar-layer-anchorage-length/🧬️schema/🟦️.ts";
import { type ChangeBarLayerBondCondition, parseChangeBarLayerBondCondition } from "./⭕️change-bar-layer-bond-condition/🧬️schema/🟦️.ts";
import { type ChangeBarLayerLapLength, parseChangeBarLayerLapLength } from "./⭕️change-bar-layer-lap-length/🧬️schema/🟦️.ts";
import { type ChangeBarLayerLapShare, parseChangeBarLayerLapShare } from "./⭕️change-bar-layer-lap-share/🧬️schema/🟦️.ts";
import { type ChangeBarLayerPosition, parseChangeBarLayerPosition } from "./⭕️change-bar-layer-position/🧬️schema/🟦️.ts";
import { type InsertBarLayer, parseInsertBarLayer } from "./⭕️insert-bar-layer/🧬️schema/🟦️.ts";
import { type RemoveBarLayer, parseRemoveBarLayer } from "./⭕️remove-bar-layer/🧬️schema/🟦️.ts";
import { type ChangeMemberBridgeSigmaC, parseChangeMemberBridgeSigmaC } from "./🌁change-member-bridge-sigma-c/🧬️schema/🟦️.ts";
import { type ChangeMemberSpan, parseChangeMemberSpan } from "./🌉️change-member-span/🧬️schema/🟦️.ts";
import { type ChangeMemberLiquidSRMax, parseChangeMemberLiquidSRMax } from "./🌊change-member-liquid-sr-max/🧬️schema/🟦️.ts";
import { type ChangeAnnex, parseChangeAnnex } from "./🌍️change-annex/🧬️schema/🟦️.ts";
import { type ChangeMemberExposure, parseChangeMemberExposure } from "./🌦️change-member-exposure/🧬️schema/🟦️.ts";
import { type ChangeMemberBridgeDeltaSigmaS, parseChangeMemberBridgeDeltaSigmaS } from "./🎢change-member-bridge-delta-sigma-s/🧬️schema/🟦️.ts";
import { type ChangeReliabilityClass, parseChangeReliabilityClass } from "./🎯change-reliability-class/🧬️schema/🟦️.ts";
import { type ChangeActionCategory, parseChangeActionCategory } from "./🏋️change-action-category/🧬️schema/🟦️.ts";
import { type ChangeActionGKLine, parseChangeActionGKLine } from "./🏋️change-action-gk-line/🧬️schema/🟦️.ts";
import { type ChangeActionKind, parseChangeActionKind } from "./🏋️change-action-kind/🧬️schema/🟦️.ts";
import { type ChangeActionNk, parseChangeActionNk } from "./🏋️change-action-nk/🧬️schema/🟦️.ts";
import { type ChangeActionPointForce, parseChangeActionPointForce } from "./🏋️change-action-point-force/🧬️schema/🟦️.ts";
import { type ChangeActionQKLine, parseChangeActionQKLine } from "./🏋️change-action-qk-line/🧬️schema/🟦️.ts";
import { type ChangeActionSource, parseChangeActionSource } from "./🏋️change-action-source/🧬️schema/🟦️.ts";
import { type ChangeActionTK, parseChangeActionTK } from "./🏋️change-action-tk/🧬️schema/🟦️.ts";
import { type ChangeActionVKPunch, parseChangeActionVKPunch } from "./🏋️change-action-vk-punch/🧬️schema/🟦️.ts";
import { type ChangeAnchorActionCategory, parseChangeAnchorActionCategory } from "./🏋️change-anchor-action-category/🧬️schema/🟦️.ts";
import { type ChangeAnchorActionGKLine, parseChangeAnchorActionGKLine } from "./🏋️change-anchor-action-gk-line/🧬️schema/🟦️.ts";
import { type ChangeAnchorActionKind, parseChangeAnchorActionKind } from "./🏋️change-anchor-action-kind/🧬️schema/🟦️.ts";
import { type ChangeAnchorActionMK, parseChangeAnchorActionMK } from "./🏋️change-anchor-action-mk/🧬️schema/🟦️.ts";
import { type ChangeAnchorActionNK, parseChangeAnchorActionNK } from "./🏋️change-anchor-action-nk/🧬️schema/🟦️.ts";
import { type ChangeAnchorActionPointForce, parseChangeAnchorActionPointForce } from "./🏋️change-anchor-action-point-force/🧬️schema/🟦️.ts";
import { type ChangeAnchorActionQKLine, parseChangeAnchorActionQKLine } from "./🏋️change-anchor-action-qk-line/🧬️schema/🟦️.ts";
import { type ChangeAnchorActionSource, parseChangeAnchorActionSource } from "./🏋️change-anchor-action-source/🧬️schema/🟦️.ts";
import { type ChangeAnchorActionTK, parseChangeAnchorActionTK } from "./🏋️change-anchor-action-tk/🧬️schema/🟦️.ts";
import { type ChangeAnchorActionVKPunch, parseChangeAnchorActionVKPunch } from "./🏋️change-anchor-action-vk-punch/🧬️schema/🟦️.ts";
import { type ChangeAnchorActionVK, parseChangeAnchorActionVK } from "./🏋️change-anchor-action-vk/🧬️schema/🟦️.ts";
import { type InsertAction, parseInsertAction } from "./🏋️insert-action/🧬️schema/🟦️.ts";
import { type InsertAnchorAction, parseInsertAnchorAction } from "./🏋️insert-anchor-action/🧬️schema/🟦️.ts";
import { type RemoveAction, parseRemoveAction } from "./🏋️remove-action/🧬️schema/🟦️.ts";
import { type RemoveAnchorAction, parseRemoveAnchorAction } from "./🏋️remove-anchor-action/🧬️schema/🟦️.ts";
import { type ChangeMemberConcreteGradeId, parseChangeMemberConcreteGradeId } from "./🏛️change-member-concrete-grade-id/🧬️schema/🟦️.ts";
import { type ChangeMemberDeflectionSensitive, parseChangeMemberDeflectionSensitive } from "./🏛️change-member-deflection-sensitive/🧬️schema/🟦️.ts";
import { type ChangeMemberFireSlabSystem, parseChangeMemberFireSlabSystem } from "./🏛️change-member-fire-slab-system/🧬️schema/🟦️.ts";
import { type ChangeMemberKind, parseChangeMemberKind } from "./🏛️change-member-kind/🧬️schema/🟦️.ts";
import { type ChangeMemberLabelDe, parseChangeMemberLabelDe } from "./🏛️change-member-label-de/🧬️schema/🟦️.ts";
import { type ChangeMemberLabelEn, parseChangeMemberLabelEn } from "./🏛️change-member-label-en/🧬️schema/🟦️.ts";
import { type ChangeMemberPrestressArea, parseChangeMemberPrestressArea } from "./🏛️change-member-prestress-area/🧬️schema/🟦️.ts";
import { type ChangeMemberPrestressEcc, parseChangeMemberPrestressEcc } from "./🏛️change-member-prestress-ecc/🧬️schema/🟦️.ts";
import { type ChangeMemberPrestressForce, parseChangeMemberPrestressForce } from "./🏛️change-member-prestress-force/🧬️schema/🟦️.ts";
import { type ChangeMemberPrestressLossRatio, parseChangeMemberPrestressLossRatio } from "./🏛️change-member-prestress-loss-ratio/🧬️schema/🟦️.ts";
import { type ChangeMemberPrestressSteelId, parseChangeMemberPrestressSteelId } from "./🏛️change-member-prestress-steel-id/🧬️schema/🟦️.ts";
import { type ChangeMemberPunchColPosition, parseChangeMemberPunchColPosition } from "./🏛️change-member-punch-col-position/🧬️schema/🟦️.ts";
import { type ChangeMemberPunchColumnDepth, parseChangeMemberPunchColumnDepth } from "./🏛️change-member-punch-column-depth/🧬️schema/🟦️.ts";
import { type ChangeMemberPunchColumnWidth, parseChangeMemberPunchColumnWidth } from "./🏛️change-member-punch-column-width/🧬️schema/🟦️.ts";
import { type ChangeMemberPunchingAsw, parseChangeMemberPunchingAsw } from "./🏛️change-member-punching-asw/🧬️schema/🟦️.ts";
import { type ChangeMemberPunchingBetaMethod, parseChangeMemberPunchingBetaMethod } from "./🏛️change-member-punching-beta-method/🧬️schema/🟦️.ts";
import { type ChangeMemberPunchingMomentC1, parseChangeMemberPunchingMomentC1 } from "./🏛️change-member-punching-moment-c1/🧬️schema/🟦️.ts";
import { type ChangeMemberPunchingMomentC2, parseChangeMemberPunchingMomentC2 } from "./🏛️change-member-punching-moment-c2/🧬️schema/🟦️.ts";
import { type ChangeMemberRebarGradeId, parseChangeMemberRebarGradeId } from "./🏛️change-member-rebar-grade-id/🧬️schema/🟦️.ts";
import { type ChangeMemberStirrupsDiameter, parseChangeMemberStirrupsDiameter } from "./🏛️change-member-stirrups-diameter/🧬️schema/🟦️.ts";
import { type ChangeMemberStirrupsLegs, parseChangeMemberStirrupsLegs } from "./🏛️change-member-stirrups-legs/🧬️schema/🟦️.ts";
import { type ChangeMemberSupport, parseChangeMemberSupport } from "./🏛️change-member-support/🧬️schema/🟦️.ts";
import { type ChangeMemberUseFem, parseChangeMemberUseFem } from "./🏛️change-member-use-fem/🧬️schema/🟦️.ts";
import { type ChangeCaseLabelDe, parseChangeCaseLabelDe } from "./🏷️change-case-label-de/🧬️schema/🟦️.ts";
import { type ChangeCaseLabelEn, parseChangeCaseLabelEn } from "./🏷️change-case-label-en/🧬️schema/🟦️.ts";
import { type ChangeTitle, parseChangeTitle } from "./🏷️change-title/🧬️schema/🟦️.ts";
import { type ChangeMemberLiquidSigmaS, parseChangeMemberLiquidSigmaS } from "./💦change-member-liquid-sigma-s/🧬️schema/🟦️.ts";
import { type ChangeMemberBearingWidth, parseChangeMemberBearingWidth } from "./💧change-member-bearing-width/🧬️schema/🟦️.ts";
import { type ChangeMemberRelativeHumidity, parseChangeMemberRelativeHumidity } from "./💧change-member-relative-humidity/🧬️schema/🟦️.ts";
import { type ChangeDesignWorkingLife, parseChangeDesignWorkingLife } from "./📅️change-design-working-life/🧬️schema/🟦️.ts";
import { type ChangeCaseEntryId, parseChangeCaseEntryId } from "./📇️change-case-entry-id/🧬️schema/🟦️.ts";
import { type ChangeAnchorHEf, parseChangeAnchorHEf } from "./📍change-anchor-h-ef/🧬️schema/🟦️.ts";
import { type ChangeBarPointBend, parseChangeBarPointBend } from "./📍️change-bar-point-bend/🧬️schema/🟦️.ts";
import { type ChangeBarPointDiameter, parseChangeBarPointDiameter } from "./📍️change-bar-point-diameter/🧬️schema/🟦️.ts";
import { type ChangeBarPointLayerId, parseChangeBarPointLayerId } from "./📍️change-bar-point-layer-id/🧬️schema/🟦️.ts";
import { type ChangeBarPointLength, parseChangeBarPointLength } from "./📍️change-bar-point-length/🧬️schema/🟦️.ts";
import { type ChangeBarPointY, parseChangeBarPointY } from "./📍️change-bar-point-y/🧬️schema/🟦️.ts";
import { type ChangeBarPointZ, parseChangeBarPointZ } from "./📍️change-bar-point-z/🧬️schema/🟦️.ts";
import { type ChangePlacementX, parseChangePlacementX } from "./📍️change-placement-x/🧬️schema/🟦️.ts";
import { type ChangePlacementY, parseChangePlacementY } from "./📍️change-placement-y/🧬️schema/🟦️.ts";
import { type ChangePlacementYaw, parseChangePlacementYaw } from "./📍️change-placement-yaw/🧬️schema/🟦️.ts";
import { type ChangePlacementZ, parseChangePlacementZ } from "./📍️change-placement-z/🧬️schema/🟦️.ts";
import { type InsertBarPoint, parseInsertBarPoint } from "./📍️insert-bar-point/🧬️schema/🟦️.ts";
import { type InsertPlacement, parseInsertPlacement } from "./📍️insert-placement/🧬️schema/🟦️.ts";
import { type RemoveBarPoint, parseRemoveBarPoint } from "./📍️remove-bar-point/🧬️schema/🟦️.ts";
import { type RemovePlacement, parseRemovePlacement } from "./📍️remove-placement/🧬️schema/🟦️.ts";
import { type ChangeDeltaCDev, parseChangeDeltaCDev } from "./📏️change-delta-c-dev/🧬️schema/🟦️.ts";
import { type ChangeMemberEffectiveDepth, parseChangeMemberEffectiveDepth } from "./📐️change-member-effective-depth/🧬️schema/🟦️.ts";
import { type ChangeSectionPointY, parseChangeSectionPointY } from "./📐️change-section-point-y/🧬️schema/🟦️.ts";
import { type ChangeSectionPointZ, parseChangeSectionPointZ } from "./📐️change-section-point-z/🧬️schema/🟦️.ts";
import { type InsertSectionPoint, parseInsertSectionPoint } from "./📐️insert-section-point/🧬️schema/🟦️.ts";
import { type RemoveSectionPoint, parseRemoveSectionPoint } from "./📐️remove-section-point/🧬️schema/🟦️.ts";
import { type ChangeMemberOrder, parseChangeMemberOrder } from "./🔀️change-member-order/🧬️schema/🟦️.ts";
import { type ChangeCaseNumber, parseChangeCaseNumber } from "./🔢️change-case-number/🧬️schema/🟦️.ts";
import { type ChangeCaseText, parseChangeCaseText } from "./🔤️change-case-text/🧬️schema/🟦️.ts";
import { type ChangeMemberAxisDistance, parseChangeMemberAxisDistance } from "./🔥change-member-axis-distance/🧬️schema/🟦️.ts";
import { type ChangeMemberFireRating, parseChangeMemberFireRating } from "./🔥️change-member-fire-rating/🧬️schema/🟦️.ts";
import { type ChangeReinforcementFYk, parseChangeReinforcementFYk } from "./🔩change-reinforcement-f-yk/🧬️schema/🟦️.ts";
import { type ChangeReinforcementDuctility, parseChangeReinforcementDuctility } from "./🔩️change-reinforcement-ductility/🧬️schema/🟦️.ts";
import { type ChangeReinforcementEpsUk, parseChangeReinforcementEpsUk } from "./🔩️change-reinforcement-eps-uk/🧬️schema/🟦️.ts";
import { type ChangeReinforcementES, parseChangeReinforcementES } from "./🔩️change-reinforcement-es/🧬️schema/🟦️.ts";
import { type ChangeReinforcementK, parseChangeReinforcementK } from "./🔩️change-reinforcement-k/🧬️schema/🟦️.ts";
import { type ChangeReinforcementName, parseChangeReinforcementName } from "./🔩️change-reinforcement-name/🧬️schema/🟦️.ts";
import { type ChangeStudRowAngle, parseChangeStudRowAngle } from "./🔩️change-stud-row-angle/🧬️schema/🟦️.ts";
import { type ChangeStudRowCount, parseChangeStudRowCount } from "./🔩️change-stud-row-count/🧬️schema/🟦️.ts";
import { type ChangeStudRowDiameter, parseChangeStudRowDiameter } from "./🔩️change-stud-row-diameter/🧬️schema/🟦️.ts";
import { type ChangeStudRowSpacingFirst, parseChangeStudRowSpacingFirst } from "./🔩️change-stud-row-spacing-first/🧬️schema/🟦️.ts";
import { type ChangeStudRowSpacingRadial, parseChangeStudRowSpacingRadial } from "./🔩️change-stud-row-spacing-radial/🧬️schema/🟦️.ts";
import { type InsertReinforcementGrade, parseInsertReinforcementGrade } from "./🔩️insert-reinforcement-grade/🧬️schema/🟦️.ts";
import { type InsertStudRow, parseInsertStudRow } from "./🔩️insert-stud-row/🧬️schema/🟦️.ts";
import { type RemoveReinforcementGrade, parseRemoveReinforcementGrade } from "./🔩️remove-reinforcement-grade/🧬️schema/🟦️.ts";
import { type RemoveStudRow, parseRemoveStudRow } from "./🔩️remove-stud-row/🧬️schema/🟦️.ts";
import { type ChangeOpeningDepth, parseChangeOpeningDepth } from "./🕳️change-opening-depth/🧬️schema/🟦️.ts";
import { type ChangeOpeningWidth, parseChangeOpeningWidth } from "./🕳️change-opening-width/🧬️schema/🟦️.ts";
import { type ChangeOpeningX, parseChangeOpeningX } from "./🕳️change-opening-x/🧬️schema/🟦️.ts";
import { type ChangeOpeningY, parseChangeOpeningY } from "./🕳️change-opening-y/🧬️schema/🟦️.ts";
import { type InsertOpening, parseInsertOpening } from "./🕳️insert-opening/🧬️schema/🟦️.ts";
import { type RemoveOpening, parseRemoveOpening } from "./🕳️remove-opening/🧬️schema/🟦️.ts";
import { type RemoveAnchor, parseRemoveAnchor } from "./🗑️remove-anchor/🧬️schema/🟦️.ts";
import { type ChangeMemberHdOverH, parseChangeMemberHdOverH } from "./🚰change-member-hd-over-h/🧬️schema/🟦️.ts";
import { type ChangeMemberTightness, parseChangeMemberTightness } from "./🚿change-member-tightness/🧬️schema/🟦️.ts";
import { type ChangeMemberCover, parseChangeMemberCover } from "./🛡️change-member-cover/🧬️schema/🟦️.ts";
import { type ChangeMemberLiquidFCtEff, parseChangeMemberLiquidFCtEff } from "./🧊change-member-liquid-f-ct-eff/🧬️schema/🟦️.ts";
import { type ChangeCementType, parseChangeCementType } from "./🧪change-cement-type/🧬️schema/🟦️.ts";
import { type ChangeTendonPointX, parseChangeTendonPointX } from "./🧬️change-tendon-point-x/🧬️schema/🟦️.ts";
import { type ChangeTendonPointY, parseChangeTendonPointY } from "./🧬️change-tendon-point-y/🧬️schema/🟦️.ts";
import { type ChangeTendonPointZ, parseChangeTendonPointZ } from "./🧬️change-tendon-point-z/🧬️schema/🟦️.ts";
import { type InsertTendonPoint, parseInsertTendonPoint } from "./🧬️insert-tendon-point/🧬️schema/🟦️.ts";
import { type RemoveTendonPoint, parseRemoveTendonPoint } from "./🧬️remove-tendon-point/🧬️schema/🟦️.ts";
import { type ChangeConcreteFCk, parseChangeConcreteFCk } from "./🧱change-concrete-f-ck/🧬️schema/🟦️.ts";
import { type ChangeConcreteName, parseChangeConcreteName } from "./🧱️change-concrete-name/🧬️schema/🟦️.ts";
import { type InsertConcreteGrade, parseInsertConcreteGrade } from "./🧱️insert-concrete-grade/🧬️schema/🟦️.ts";
import { type RemoveConcreteGrade, parseRemoveConcreteGrade } from "./🧱️remove-concrete-grade/🧬️schema/🟦️.ts";
import { type ChangeMemberBucklingLength, parseChangeMemberBucklingLength } from "./🧵change-member-buckling-length/🧬️schema/🟦️.ts";
import { type ChangePrestressSteelFPk, parseChangePrestressSteelFPk } from "./🧵️change-prestress-steel-f-pk/🧬️schema/🟦️.ts";
import { type ChangePrestressSteelFP01k, parseChangePrestressSteelFP01k } from "./🧵️change-prestress-steel-fp01k/🧬️schema/🟦️.ts";
import { type ChangePrestressSteelName, parseChangePrestressSteelName } from "./🧵️change-prestress-steel-name/🧬️schema/🟦️.ts";
import { type InsertPrestressSteel, parseInsertPrestressSteel } from "./🧵️insert-prestress-steel/🧬️schema/🟦️.ts";
import { type RemovePrestressSteel, parseRemovePrestressSteel } from "./🧵️remove-prestress-steel/🧬️schema/🟦️.ts";
import { type ChangeAnchorAs, parseChangeAnchorAs } from "./🧷change-anchor-as/🧬️schema/🟦️.ts";
import { type ClearMemberTightness, parseClearMemberTightness } from "./🧽clear-member-tightness/🧬️schema/🟦️.ts";
import { type ChangeMemberUdl, parseChangeMemberUdl } from "./🪜change-member-udl/🧬️schema/🟦️.ts";
import { type ChangeMemberStirrupSpacing, parseChangeMemberStirrupSpacing } from "./🪢change-member-stirrup-spacing/🧬️schema/🟦️.ts";
import { type ChangeMemberLiquidRhoPEff, parseChangeMemberLiquidRhoPEff } from "./🫧change-member-liquid-rho-p-eff/🧬️schema/🟦️.ts";

export type En1992Mutation =
  | { ChangeAnnex: ChangeAnnex }
  | { ChangeReliabilityClass: ChangeReliabilityClass }
  | { ChangeTitle: ChangeTitle }
  | { ChangeDesignWorkingLife: ChangeDesignWorkingLife }
  | { ChangeDeltaCDev: ChangeDeltaCDev }
  | { ChangeCementType: ChangeCementType }
  | { ChangeConcreteFCk: ChangeConcreteFCk }
  | { ChangeReinforcementFYk: ChangeReinforcementFYk }
  | { InsertMember: InsertMember }
  | { RemoveMember: RemoveMember }
  | { ChangeMemberOrder: ChangeMemberOrder }
  | { ChangeMemberWidth: ChangeMemberWidth }
  | { ChangeMemberHeight: ChangeMemberHeight }
  | { ChangeMemberEffectiveDepth: ChangeMemberEffectiveDepth }
  | { ChangeMemberCover: ChangeMemberCover }
  | { ChangeMemberExposure: ChangeMemberExposure }
  | { ChangeMemberSpan: ChangeMemberSpan }
  | { ChangeMemberStirrupSpacing: ChangeMemberStirrupSpacing }
  | { ChangeMemberAxisDistance: ChangeMemberAxisDistance }
  | { ChangeMemberFireRating: ChangeMemberFireRating }
  | { ChangeBarLayerCount: ChangeBarLayerCount }
  | { ChangeBarLayerDiameter: ChangeBarLayerDiameter }
  | { ChangeActionMk: ChangeActionMk }
  | { ChangeActionNk: ChangeActionNk }
  | { ChangeActionVk: ChangeActionVk }
  | { InsertAnchor: InsertAnchor }
  | { RemoveAnchor: RemoveAnchor }
  | { ChangeAnchorHEf: ChangeAnchorHEf }
  | { ChangeAnchorAs: ChangeAnchorAs }
  | { ChangeMemberRelativeHumidity: ChangeMemberRelativeHumidity }
  | { ChangeMemberBucklingLength: ChangeMemberBucklingLength }
  | { ChangeMemberUdl: ChangeMemberUdl }
  | { ChangeMemberHdOverH: ChangeMemberHdOverH }
  | { ChangeMemberLiquidSigmaS: ChangeMemberLiquidSigmaS }
  | { ChangeMemberLiquidRhoPEff: ChangeMemberLiquidRhoPEff }
  | { ChangeMemberLiquidFCtEff: ChangeMemberLiquidFCtEff }
  | { ChangeMemberLiquidSRMax: ChangeMemberLiquidSRMax }
  | { ChangeMemberBridgeSigmaC: ChangeMemberBridgeSigmaC }
  | { ChangeMemberBridgeDeltaSigmaS: ChangeMemberBridgeDeltaSigmaS }
  | { ChangeConcreteName: ChangeConcreteName }
  | { ChangeReinforcementName: ChangeReinforcementName }
  | { ChangeReinforcementES: ChangeReinforcementES }
  | { ChangeReinforcementDuctility: ChangeReinforcementDuctility }
  | { ChangeReinforcementK: ChangeReinforcementK }
  | { ChangeReinforcementEpsUk: ChangeReinforcementEpsUk }
  | { ChangePrestressSteelName: ChangePrestressSteelName }
  | { ChangePrestressSteelFPk: ChangePrestressSteelFPk }
  | { ChangePrestressSteelFP01k: ChangePrestressSteelFP01k }
  | { ChangeMemberLabelEn: ChangeMemberLabelEn }
  | { ChangeMemberLabelDe: ChangeMemberLabelDe }
  | { ChangeMemberKind: ChangeMemberKind }
  | { ChangeMemberConcreteGradeId: ChangeMemberConcreteGradeId }
  | { ChangeMemberRebarGradeId: ChangeMemberRebarGradeId }
  | { ChangeMemberPrestressSteelId: ChangeMemberPrestressSteelId }
  | { ChangeMemberSupport: ChangeMemberSupport }
  | { ChangeMemberStirrupsDiameter: ChangeMemberStirrupsDiameter }
  | { ChangeMemberStirrupsLegs: ChangeMemberStirrupsLegs }
  | { ChangeMemberPunchColumnWidth: ChangeMemberPunchColumnWidth }
  | { ChangeMemberPunchColumnDepth: ChangeMemberPunchColumnDepth }
  | { ChangeMemberPunchColPosition: ChangeMemberPunchColPosition }
  | { ChangeMemberPunchingAsw: ChangeMemberPunchingAsw }
  | { ChangeMemberPrestressForce: ChangeMemberPrestressForce }
  | { ChangeMemberPrestressArea: ChangeMemberPrestressArea }
  | { ChangeMemberPrestressEcc: ChangeMemberPrestressEcc }
  | { ChangeMemberPrestressLossRatio: ChangeMemberPrestressLossRatio }
  | { ChangeMemberFireSlabSystem: ChangeMemberFireSlabSystem }
  | { ChangeMemberUseFem: ChangeMemberUseFem }
  | { ChangeMemberDeflectionSensitive: ChangeMemberDeflectionSensitive }
  | { ChangeBarLayerPosition: ChangeBarLayerPosition }
  | { ChangeBarLayerAnchorageLength: ChangeBarLayerAnchorageLength }
  | { ChangeBarLayerLapLength: ChangeBarLayerLapLength }
  | { ChangeBarLayerBondCondition: ChangeBarLayerBondCondition }
  | { ChangeBarLayerAggregateSize: ChangeBarLayerAggregateSize }
  | { ChangeBarLayerLapShare: ChangeBarLayerLapShare }
  | { ChangeActionKind: ChangeActionKind }
  | { ChangeActionCategory: ChangeActionCategory }
  | { ChangeActionSource: ChangeActionSource }
  | { ChangeActionGKLine: ChangeActionGKLine }
  | { ChangeActionQKLine: ChangeActionQKLine }
  | { ChangeActionPointForce: ChangeActionPointForce }
  | { ChangeActionTK: ChangeActionTK }
  | { ChangeActionVKPunch: ChangeActionVKPunch }
  | { ChangeSectionPointY: ChangeSectionPointY }
  | { ChangeSectionPointZ: ChangeSectionPointZ }
  | { ChangeBarPointLayerId: ChangeBarPointLayerId }
  | { ChangeBarPointY: ChangeBarPointY }
  | { ChangeBarPointZ: ChangeBarPointZ }
  | { ChangeBarPointDiameter: ChangeBarPointDiameter }
  | { ChangeTendonPointX: ChangeTendonPointX }
  | { ChangeTendonPointY: ChangeTendonPointY }
  | { ChangeTendonPointZ: ChangeTendonPointZ }
  | { ChangeAnchorCracked: ChangeAnchorCracked }
  | { ChangeAnchorFUk: ChangeAnchorFUk }
  | { ChangeAnchorFYk: ChangeAnchorFYk }
  | { ChangeAnchorD: ChangeAnchorD }
  | { ChangeAnchorC1: ChangeAnchorC1 }
  | { ChangeAnchorFCk: ChangeAnchorFCk }
  | { ChangeAnchorHeadDiameter: ChangeAnchorHeadDiameter }
  | { ChangeAnchorActionKind: ChangeAnchorActionKind }
  | { ChangeAnchorActionCategory: ChangeAnchorActionCategory }
  | { ChangeAnchorActionSource: ChangeAnchorActionSource }
  | { ChangeAnchorActionGKLine: ChangeAnchorActionGKLine }
  | { ChangeAnchorActionQKLine: ChangeAnchorActionQKLine }
  | { ChangeAnchorActionPointForce: ChangeAnchorActionPointForce }
  | { ChangeAnchorActionMK: ChangeAnchorActionMK }
  | { ChangeAnchorActionNK: ChangeAnchorActionNK }
  | { ChangeAnchorActionVK: ChangeAnchorActionVK }
  | { ChangeAnchorActionTK: ChangeAnchorActionTK }
  | { ChangeAnchorActionVKPunch: ChangeAnchorActionVKPunch }
  | { InsertConcreteGrade: InsertConcreteGrade }
  | { RemoveConcreteGrade: RemoveConcreteGrade }
  | { InsertReinforcementGrade: InsertReinforcementGrade }
  | { RemoveReinforcementGrade: RemoveReinforcementGrade }
  | { InsertPrestressSteel: InsertPrestressSteel }
  | { RemovePrestressSteel: RemovePrestressSteel }
  | { InsertBarLayer: InsertBarLayer }
  | { RemoveBarLayer: RemoveBarLayer }
  | { InsertAction: InsertAction }
  | { RemoveAction: RemoveAction }
  | { InsertSectionPoint: InsertSectionPoint }
  | { RemoveSectionPoint: RemoveSectionPoint }
  | { InsertBarPoint: InsertBarPoint }
  | { RemoveBarPoint: RemoveBarPoint }
  | { InsertTendonPoint: InsertTendonPoint }
  | { RemoveTendonPoint: RemoveTendonPoint }
  | { InsertAnchorAction: InsertAnchorAction }
  | { RemoveAnchorAction: RemoveAnchorAction }
  | { ChangePlacementX: ChangePlacementX }
  | { ChangePlacementY: ChangePlacementY }
  | { ChangePlacementZ: ChangePlacementZ }
  | { ChangePlacementYaw: ChangePlacementYaw }
  | { InsertPlacement: InsertPlacement }
  | { RemovePlacement: RemovePlacement }
  | { InsertCase: InsertCase }
  | { RemoveCase: RemoveCase }
  | { ChangeCaseNumber: ChangeCaseNumber }
  | { ChangeCaseText: ChangeCaseText }
  | { ChangeCaseLabelEn: ChangeCaseLabelEn }
  | { ChangeCaseLabelDe: ChangeCaseLabelDe }
  | { ChangeCaseEntryId: ChangeCaseEntryId }
  | { ChangeMemberTightness: ChangeMemberTightness }
  | { ClearMemberTightness: ClearMemberTightness }
  | { ChangeBarPointLength: ChangeBarPointLength }
  | { ChangeBarPointBend: ChangeBarPointBend }
  | { ChangeMemberPunchingBetaMethod: ChangeMemberPunchingBetaMethod }
  | { ChangeMemberPunchingMomentC1: ChangeMemberPunchingMomentC1 }
  | { ChangeMemberPunchingMomentC2: ChangeMemberPunchingMomentC2 }
  | { ChangeOpeningX: ChangeOpeningX }
  | { ChangeOpeningY: ChangeOpeningY }
  | { ChangeOpeningWidth: ChangeOpeningWidth }
  | { ChangeOpeningDepth: ChangeOpeningDepth }
  | { InsertOpening: InsertOpening }
  | { RemoveOpening: RemoveOpening }
  | { ChangeStudRowAngle: ChangeStudRowAngle }
  | { ChangeStudRowCount: ChangeStudRowCount }
  | { ChangeStudRowSpacingFirst: ChangeStudRowSpacingFirst }
  | { ChangeStudRowSpacingRadial: ChangeStudRowSpacingRadial }
  | { ChangeStudRowDiameter: ChangeStudRowDiameter }
  | { InsertStudRow: InsertStudRow }
  | { RemoveStudRow: RemoveStudRow }
  | { ChangeMemberBearingWidth: ChangeMemberBearingWidth };

export const parseEn1992Mutation: NormWireReader<En1992Mutation> = normWireExternal<En1992Mutation>({
  ChangeAnnex: parseChangeAnnex,
  ChangeReliabilityClass: parseChangeReliabilityClass,
  ChangeTitle: parseChangeTitle,
  ChangeDesignWorkingLife: parseChangeDesignWorkingLife,
  ChangeDeltaCDev: parseChangeDeltaCDev,
  ChangeCementType: parseChangeCementType,
  ChangeConcreteFCk: parseChangeConcreteFCk,
  ChangeReinforcementFYk: parseChangeReinforcementFYk,
  InsertMember: parseInsertMember,
  RemoveMember: parseRemoveMember,
  ChangeMemberOrder: parseChangeMemberOrder,
  ChangeMemberWidth: parseChangeMemberWidth,
  ChangeMemberHeight: parseChangeMemberHeight,
  ChangeMemberEffectiveDepth: parseChangeMemberEffectiveDepth,
  ChangeMemberCover: parseChangeMemberCover,
  ChangeMemberExposure: parseChangeMemberExposure,
  ChangeMemberSpan: parseChangeMemberSpan,
  ChangeMemberStirrupSpacing: parseChangeMemberStirrupSpacing,
  ChangeMemberAxisDistance: parseChangeMemberAxisDistance,
  ChangeMemberFireRating: parseChangeMemberFireRating,
  ChangeBarLayerCount: parseChangeBarLayerCount,
  ChangeBarLayerDiameter: parseChangeBarLayerDiameter,
  ChangeActionMk: parseChangeActionMk,
  ChangeActionNk: parseChangeActionNk,
  ChangeActionVk: parseChangeActionVk,
  InsertAnchor: parseInsertAnchor,
  RemoveAnchor: parseRemoveAnchor,
  ChangeAnchorHEf: parseChangeAnchorHEf,
  ChangeAnchorAs: parseChangeAnchorAs,
  ChangeMemberRelativeHumidity: parseChangeMemberRelativeHumidity,
  ChangeMemberBucklingLength: parseChangeMemberBucklingLength,
  ChangeMemberUdl: parseChangeMemberUdl,
  ChangeMemberHdOverH: parseChangeMemberHdOverH,
  ChangeMemberLiquidSigmaS: parseChangeMemberLiquidSigmaS,
  ChangeMemberLiquidRhoPEff: parseChangeMemberLiquidRhoPEff,
  ChangeMemberLiquidFCtEff: parseChangeMemberLiquidFCtEff,
  ChangeMemberLiquidSRMax: parseChangeMemberLiquidSRMax,
  ChangeMemberBridgeSigmaC: parseChangeMemberBridgeSigmaC,
  ChangeMemberBridgeDeltaSigmaS: parseChangeMemberBridgeDeltaSigmaS,
  ChangeConcreteName: parseChangeConcreteName,
  ChangeReinforcementName: parseChangeReinforcementName,
  ChangeReinforcementES: parseChangeReinforcementES,
  ChangeReinforcementDuctility: parseChangeReinforcementDuctility,
  ChangeReinforcementK: parseChangeReinforcementK,
  ChangeReinforcementEpsUk: parseChangeReinforcementEpsUk,
  ChangePrestressSteelName: parseChangePrestressSteelName,
  ChangePrestressSteelFPk: parseChangePrestressSteelFPk,
  ChangePrestressSteelFP01k: parseChangePrestressSteelFP01k,
  ChangeMemberLabelEn: parseChangeMemberLabelEn,
  ChangeMemberLabelDe: parseChangeMemberLabelDe,
  ChangeMemberKind: parseChangeMemberKind,
  ChangeMemberConcreteGradeId: parseChangeMemberConcreteGradeId,
  ChangeMemberRebarGradeId: parseChangeMemberRebarGradeId,
  ChangeMemberPrestressSteelId: parseChangeMemberPrestressSteelId,
  ChangeMemberSupport: parseChangeMemberSupport,
  ChangeMemberStirrupsDiameter: parseChangeMemberStirrupsDiameter,
  ChangeMemberStirrupsLegs: parseChangeMemberStirrupsLegs,
  ChangeMemberPunchColumnWidth: parseChangeMemberPunchColumnWidth,
  ChangeMemberPunchColumnDepth: parseChangeMemberPunchColumnDepth,
  ChangeMemberPunchColPosition: parseChangeMemberPunchColPosition,
  ChangeMemberPunchingAsw: parseChangeMemberPunchingAsw,
  ChangeMemberPrestressForce: parseChangeMemberPrestressForce,
  ChangeMemberPrestressArea: parseChangeMemberPrestressArea,
  ChangeMemberPrestressEcc: parseChangeMemberPrestressEcc,
  ChangeMemberPrestressLossRatio: parseChangeMemberPrestressLossRatio,
  ChangeMemberFireSlabSystem: parseChangeMemberFireSlabSystem,
  ChangeMemberUseFem: parseChangeMemberUseFem,
  ChangeMemberDeflectionSensitive: parseChangeMemberDeflectionSensitive,
  ChangeBarLayerPosition: parseChangeBarLayerPosition,
  ChangeBarLayerAnchorageLength: parseChangeBarLayerAnchorageLength,
  ChangeBarLayerLapLength: parseChangeBarLayerLapLength,
  ChangeBarLayerBondCondition: parseChangeBarLayerBondCondition,
  ChangeBarLayerAggregateSize: parseChangeBarLayerAggregateSize,
  ChangeBarLayerLapShare: parseChangeBarLayerLapShare,
  ChangeActionKind: parseChangeActionKind,
  ChangeActionCategory: parseChangeActionCategory,
  ChangeActionSource: parseChangeActionSource,
  ChangeActionGKLine: parseChangeActionGKLine,
  ChangeActionQKLine: parseChangeActionQKLine,
  ChangeActionPointForce: parseChangeActionPointForce,
  ChangeActionTK: parseChangeActionTK,
  ChangeActionVKPunch: parseChangeActionVKPunch,
  ChangeSectionPointY: parseChangeSectionPointY,
  ChangeSectionPointZ: parseChangeSectionPointZ,
  ChangeBarPointLayerId: parseChangeBarPointLayerId,
  ChangeBarPointY: parseChangeBarPointY,
  ChangeBarPointZ: parseChangeBarPointZ,
  ChangeBarPointDiameter: parseChangeBarPointDiameter,
  ChangeTendonPointX: parseChangeTendonPointX,
  ChangeTendonPointY: parseChangeTendonPointY,
  ChangeTendonPointZ: parseChangeTendonPointZ,
  ChangeAnchorCracked: parseChangeAnchorCracked,
  ChangeAnchorFUk: parseChangeAnchorFUk,
  ChangeAnchorFYk: parseChangeAnchorFYk,
  ChangeAnchorD: parseChangeAnchorD,
  ChangeAnchorC1: parseChangeAnchorC1,
  ChangeAnchorFCk: parseChangeAnchorFCk,
  ChangeAnchorHeadDiameter: parseChangeAnchorHeadDiameter,
  ChangeAnchorActionKind: parseChangeAnchorActionKind,
  ChangeAnchorActionCategory: parseChangeAnchorActionCategory,
  ChangeAnchorActionSource: parseChangeAnchorActionSource,
  ChangeAnchorActionGKLine: parseChangeAnchorActionGKLine,
  ChangeAnchorActionQKLine: parseChangeAnchorActionQKLine,
  ChangeAnchorActionPointForce: parseChangeAnchorActionPointForce,
  ChangeAnchorActionMK: parseChangeAnchorActionMK,
  ChangeAnchorActionNK: parseChangeAnchorActionNK,
  ChangeAnchorActionVK: parseChangeAnchorActionVK,
  ChangeAnchorActionTK: parseChangeAnchorActionTK,
  ChangeAnchorActionVKPunch: parseChangeAnchorActionVKPunch,
  InsertConcreteGrade: parseInsertConcreteGrade,
  RemoveConcreteGrade: parseRemoveConcreteGrade,
  InsertReinforcementGrade: parseInsertReinforcementGrade,
  RemoveReinforcementGrade: parseRemoveReinforcementGrade,
  InsertPrestressSteel: parseInsertPrestressSteel,
  RemovePrestressSteel: parseRemovePrestressSteel,
  InsertBarLayer: parseInsertBarLayer,
  RemoveBarLayer: parseRemoveBarLayer,
  InsertAction: parseInsertAction,
  RemoveAction: parseRemoveAction,
  InsertSectionPoint: parseInsertSectionPoint,
  RemoveSectionPoint: parseRemoveSectionPoint,
  InsertBarPoint: parseInsertBarPoint,
  RemoveBarPoint: parseRemoveBarPoint,
  InsertTendonPoint: parseInsertTendonPoint,
  RemoveTendonPoint: parseRemoveTendonPoint,
  InsertAnchorAction: parseInsertAnchorAction,
  RemoveAnchorAction: parseRemoveAnchorAction,
  ChangePlacementX: parseChangePlacementX,
  ChangePlacementY: parseChangePlacementY,
  ChangePlacementZ: parseChangePlacementZ,
  ChangePlacementYaw: parseChangePlacementYaw,
  InsertPlacement: parseInsertPlacement,
  RemovePlacement: parseRemovePlacement,
  InsertCase: parseInsertCase,
  RemoveCase: parseRemoveCase,
  ChangeCaseNumber: parseChangeCaseNumber,
  ChangeCaseText: parseChangeCaseText,
  ChangeCaseLabelEn: parseChangeCaseLabelEn,
  ChangeCaseLabelDe: parseChangeCaseLabelDe,
  ChangeCaseEntryId: parseChangeCaseEntryId,
  ChangeMemberTightness: parseChangeMemberTightness,
  ClearMemberTightness: parseClearMemberTightness,
  ChangeBarPointLength: parseChangeBarPointLength,
  ChangeBarPointBend: parseChangeBarPointBend,
  ChangeMemberPunchingBetaMethod: parseChangeMemberPunchingBetaMethod,
  ChangeMemberPunchingMomentC1: parseChangeMemberPunchingMomentC1,
  ChangeMemberPunchingMomentC2: parseChangeMemberPunchingMomentC2,
  ChangeOpeningX: parseChangeOpeningX,
  ChangeOpeningY: parseChangeOpeningY,
  ChangeOpeningWidth: parseChangeOpeningWidth,
  ChangeOpeningDepth: parseChangeOpeningDepth,
  InsertOpening: parseInsertOpening,
  RemoveOpening: parseRemoveOpening,
  ChangeStudRowAngle: parseChangeStudRowAngle,
  ChangeStudRowCount: parseChangeStudRowCount,
  ChangeStudRowSpacingFirst: parseChangeStudRowSpacingFirst,
  ChangeStudRowSpacingRadial: parseChangeStudRowSpacingRadial,
  ChangeStudRowDiameter: parseChangeStudRowDiameter,
  InsertStudRow: parseInsertStudRow,
  RemoveStudRow: parseRemoveStudRow,
  ChangeMemberBearingWidth: parseChangeMemberBearingWidth,
});
