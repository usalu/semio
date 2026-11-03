/** 🧺️ `En1996Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireExternal, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ChangeInPlaneShear, parseChangeInPlaneShear } from "./↔️change-in-plane-shear/🧬️schema/🟦️.ts";
import { type ChangeSlabSpan, parseChangeSlabSpan } from "./↔️change-slab-span/🧬️schema/🟦️.ts";
import { type ChangeWallLength, parseChangeWallLength } from "./↔️change-wall-length/🧬️schema/🟦️.ts";
import { type ChangeWallHeight, parseChangeWallHeight } from "./↕️change-wall-height/🧬️schema/🟦️.ts";
import { type ChangeWallThickness, parseChangeWallThickness } from "./↕️change-wall-thickness/🧬️schema/🟦️.ts";
import { type ChangeEccentricityBottom, parseChangeEccentricityBottom } from "./↗️change-eccentricity-bottom/🧬️schema/🟦️.ts";
import { type ChangeEccentricityTop, parseChangeEccentricityTop } from "./↘️change-eccentricity-top/🧬️schema/🟦️.ts";
import { type ChangePhiInfinity, parseChangePhiInfinity } from "./♾️change-phi-infinity/🧬️schema/🟦️.ts";
import { type ChangeFrostClass, parseChangeFrostClass } from "./❄️change-frost-class/🧬️schema/🟦️.ts";
import { type ChangeQKSnow, parseChangeQKSnow } from "./❄️change-qk-snow/🧬️schema/🟦️.ts";
import { type InsertBar, parseInsertBar } from "./➕️insert-bar/🧬️schema/🟦️.ts";
import { type InsertCase, parseInsertCase } from "./➕️insert-case/🧬️schema/🟦️.ts";
import { type InsertConcentrated, parseInsertConcentrated } from "./➕️insert-concentrated/🧬️schema/🟦️.ts";
import { type InsertConfined, parseInsertConfined } from "./➕️insert-confined/🧬️schema/🟦️.ts";
import { type InsertFlange, parseInsertFlange } from "./➕️insert-flange/🧬️schema/🟦️.ts";
import { type InsertLintel, parseInsertLintel } from "./➕️insert-lintel/🧬️schema/🟦️.ts";
import { type InsertLoadCase, parseInsertLoadCase } from "./➕️insert-load-case/🧬️schema/🟦️.ts";
import { type InsertOpening, parseInsertOpening } from "./➕️insert-opening/🧬️schema/🟦️.ts";
import { type InsertPier, parseInsertPier } from "./➕️insert-pier/🧬️schema/🟦️.ts";
import { type InsertPrestress, parseInsertPrestress } from "./➕️insert-prestress/🧬️schema/🟦️.ts";
import { type InsertTie, parseInsertTie } from "./➕️insert-tie/🧬️schema/🟦️.ts";
import { type InsertWall, parseInsertWall } from "./➕️insert-wall/🧬️schema/🟦️.ts";
import { type RemoveBar, parseRemoveBar } from "./➖️remove-bar/🧬️schema/🟦️.ts";
import { type RemoveCase, parseRemoveCase } from "./➖️remove-case/🧬️schema/🟦️.ts";
import { type RemoveConcentrated, parseRemoveConcentrated } from "./➖️remove-concentrated/🧬️schema/🟦️.ts";
import { type RemoveConfined, parseRemoveConfined } from "./➖️remove-confined/🧬️schema/🟦️.ts";
import { type RemoveFlange, parseRemoveFlange } from "./➖️remove-flange/🧬️schema/🟦️.ts";
import { type RemoveLintel, parseRemoveLintel } from "./➖️remove-lintel/🧬️schema/🟦️.ts";
import { type RemoveLoadCase, parseRemoveLoadCase } from "./➖️remove-load-case/🧬️schema/🟦️.ts";
import { type RemoveOpening, parseRemoveOpening } from "./➖️remove-opening/🧬️schema/🟦️.ts";
import { type RemovePier, parseRemovePier } from "./➖️remove-pier/🧬️schema/🟦️.ts";
import { type RemovePrestress, parseRemovePrestress } from "./➖️remove-prestress/🧬️schema/🟦️.ts";
import { type RemoveTie, parseRemoveTie } from "./➖️remove-tie/🧬️schema/🟦️.ts";
import { type RemoveWall, parseRemoveWall } from "./➖️remove-wall/🧬️schema/🟦️.ts";
import { type ChangeAnnex, parseChangeAnnex } from "./🌍️change-annex/🧬️schema/🟦️.ts";
import { type ChangeQPWind, parseChangeQPWind } from "./🌬️change-qp-wind/🧬️schema/🟦️.ts";
import { type ChangeDesignSituation, parseChangeDesignSituation } from "./🎭️change-design-situation/🧬️schema/🟦️.ts";
import { type ChangeLoadCaseSituation, parseChangeLoadCaseSituation } from "./🎭️change-load-case-situation/🧬️schema/🟦️.ts";
import { type ChangeReliabilityClass, parseChangeReliabilityClass } from "./🎯change-reliability-class/🧬️schema/🟦️.ts";
import { type ChangePrestressArea, parseChangePrestressArea } from "./🎻change-prestress-area/🧬️schema/🟦️.ts";
import { type ChangePrestressEccentricity, parseChangePrestressEccentricity } from "./🎻change-prestress-eccentricity/🧬️schema/🟦️.ts";
import { type ChangePrestressForce, parseChangePrestressForce } from "./🎻change-prestress-force/🧬️schema/🟦️.ts";
import { type ChangePrestressLoss, parseChangePrestressLoss } from "./🎻change-prestress-loss/🧬️schema/🟦️.ts";
import { type ChangePrestressPosition, parseChangePrestressPosition } from "./🎻change-prestress-position/🧬️schema/🟦️.ts";
import { type ChangePrestressStrength, parseChangePrestressStrength } from "./🎻change-prestress-strength/🧬️schema/🟦️.ts";
import { type ChangeConcentratedForce, parseChangeConcentratedForce } from "./🏋️change-concentrated-force/🧬️schema/🟦️.ts";
import { type ChangeGKSlab, parseChangeGKSlab } from "./🏋️change-gk-slab/🧬️schema/🟦️.ts";
import { type ChangeQKImposed, parseChangeQKImposed } from "./🏋️change-qk-imposed/🧬️schema/🟦️.ts";
import { type ChangeIsBasement, parseChangeIsBasement } from "./🏗️change-is-basement/🧬️schema/🟦️.ts";
import { type ChangePierPosition, parseChangePierPosition } from "./🏛️change-pier-position/🧬️schema/🟦️.ts";
import { type ChangePierWidth, parseChangePierWidth } from "./🏛️change-pier-width/🧬️schema/🟦️.ts";
import { type ChangeSoilHeight, parseChangeSoilHeight } from "./🏞️change-soil-height/🧬️schema/🟦️.ts";
import { type ChangeStoreys, parseChangeStoreys } from "./🏢️change-storeys/🧬️schema/🟦️.ts";
import { type ChangeMasonryClass, parseChangeMasonryClass } from "./🏭️change-masonry-class/🧬️schema/🟦️.ts";
import { type ChangeCaseLabelDe, parseChangeCaseLabelDe } from "./🏷️change-case-label-de/🧬️schema/🟦️.ts";
import { type ChangeCaseLabelEn, parseChangeCaseLabelEn } from "./🏷️change-case-label-en/🧬️schema/🟦️.ts";
import { type ChangeImposedCategory, parseChangeImposedCategory } from "./🏷️change-imposed-category/🧬️schema/🟦️.ts";
import { type ChangeWallLabelDe, parseChangeWallLabelDe } from "./🏷️change-wall-label-de/🧬️schema/🟦️.ts";
import { type ChangeWallLabelEn, parseChangeWallLabelEn } from "./🏷️change-wall-label-en/🧬️schema/🟦️.ts";
import { type ChangeExposure, parseChangeExposure } from "./💧️change-exposure/🧬️schema/🟦️.ts";
import { type ChangeCaseEntryId, parseChangeCaseEntryId } from "./📇️change-case-entry-id/🧬️schema/🟦️.ts";
import { type ChangeDeflectionLimit, parseChangeDeflectionLimit } from "./📉change-deflection-limit/🧬️schema/🟦️.ts";
import { type ChangeConcentratedPosition, parseChangeConcentratedPosition } from "./📍️change-concentrated-position/🧬️schema/🟦️.ts";
import { type ChangePlacementX, parseChangePlacementX } from "./📍️change-placement-x/🧬️schema/🟦️.ts";
import { type ChangePlacementY, parseChangePlacementY } from "./📍️change-placement-y/🧬️schema/🟦️.ts";
import { type ChangePlacementYaw, parseChangePlacementYaw } from "./📍️change-placement-yaw/🧬️schema/🟦️.ts";
import { type ChangePlacementZ, parseChangePlacementZ } from "./📍️change-placement-z/🧬️schema/🟦️.ts";
import { type InsertPlacement, parseInsertPlacement } from "./📍️insert-placement/🧬️schema/🟦️.ts";
import { type RemovePlacement, parseRemovePlacement } from "./📍️remove-placement/🧬️schema/🟦️.ts";
import { type ChangeConcentratedBearingLength, parseChangeConcentratedBearingLength } from "./📏change-concentrated-bearing-length/🧬️schema/🟦️.ts";
import { type ChangeConcentratedDistanceA1, parseChangeConcentratedDistanceA1 } from "./📏change-concentrated-distance-a1/🧬️schema/🟦️.ts";
import { type ChangeConcentratedBearingArea, parseChangeConcentratedBearingArea } from "./📐️change-concentrated-bearing-area/🧬️schema/🟦️.ts";
import { type ChangePlanAngle, parseChangePlanAngle } from "./📐️change-plan-angle/🧬️schema/🟦️.ts";
import { type ChangeSlabBearingDepth, parseChangeSlabBearingDepth } from "./📐️change-slab-bearing-depth/🧬️schema/🟦️.ts";
import { type ChangeTributaryArea, parseChangeTributaryArea } from "./📐️change-tributary-area/🧬️schema/🟦️.ts";
import { type ChangeInPlaneMoment, parseChangeInPlaneMoment } from "./🔄change-in-plane-moment/🧬️schema/🟦️.ts";
import { type ChangeTieCapacity, parseChangeTieCapacity } from "./🔗️change-tie-capacity/🧬️schema/🟦️.ts";
import { type ChangeTieKind, parseChangeTieKind } from "./🔗️change-tie-kind/🧬️schema/🟦️.ts";
import { type ChangeTieLevel, parseChangeTieLevel } from "./🔗️change-tie-level/🧬️schema/🟦️.ts";
import { type ChangeTiePosition, parseChangeTiePosition } from "./🔗️change-tie-position/🧬️schema/🟦️.ts";
import { type ChangeCaseNumber, parseChangeCaseNumber } from "./🔢️change-case-number/🧬️schema/🟦️.ts";
import { type ChangeCaseText, parseChangeCaseText } from "./🔤️change-case-text/🧬️schema/🟦️.ts";
import { type ChangeIsothermDepth, parseChangeIsothermDepth } from "./🔥change-isotherm-depth/🧬️schema/🟦️.ts";
import { type ChangeFireRei, parseChangeFireRei } from "./🔥️change-fire-rei/🧬️schema/🟦️.ts";
import { type ChangeAsHorizontal, parseChangeAsHorizontal } from "./🔩change-as-horizontal/🧬️schema/🟦️.ts";
import { type ChangeAsVertical, parseChangeAsVertical } from "./🔩change-as-vertical/🧬️schema/🟦️.ts";
import { type ChangeFYd, parseChangeFYd } from "./🔩change-f-yd/🧬️schema/🟦️.ts";
import { type ChangeReinforced, parseChangeReinforced } from "./🔩change-reinforced/🧬️schema/🟦️.ts";
import { type ChangeFlangeLength, parseChangeFlangeLength } from "./🔲change-flange-length/🧬️schema/🟦️.ts";
import { type ChangeFlangePosition, parseChangeFlangePosition } from "./🔲change-flange-position/🧬️schema/🟦️.ts";
import { type ChangeFlangeThickness, parseChangeFlangeThickness } from "./🔲change-flange-thickness/🧬️schema/🟦️.ts";
import { type ChangeCover, parseChangeCover } from "./🛡️change-cover/🧬️schema/🟦️.ts";
import { type ChangeLintelBearingLength, parseChangeLintelBearingLength } from "./🛤️change-lintel-bearing-length/🧬️schema/🟦️.ts";
import { type ChangeLintelDepth, parseChangeLintelDepth } from "./🛤️change-lintel-depth/🧬️schema/🟦️.ts";
import { type ChangeLintelPosition, parseChangeLintelPosition } from "./🛤️change-lintel-position/🧬️schema/🟦️.ts";
import { type ChangeLintelSpan, parseChangeLintelSpan } from "./🛤️change-lintel-span/🧬️schema/🟦️.ts";
import { type ChangeBedJointThickness, parseChangeBedJointThickness } from "./🥪️change-bed-joint-thickness/🧬️schema/🟦️.ts";
import { type ChangeFm, parseChangeFm } from "./🧈change-fm/🧬️schema/🟦️.ts";
import { type ChangeMortarClass, parseChangeMortarClass } from "./🧈change-mortar-class/🧬️schema/🟦️.ts";
import { type ChangeMortarType, parseChangeMortarType } from "./🧈change-mortar-type/🧬️schema/🟦️.ts";
import { type ChangePlanX, parseChangePlanX } from "./🧭change-plan-x/🧬️schema/🟦️.ts";
import { type ChangePlanY, parseChangePlanY } from "./🧭change-plan-y/🧬️schema/🟦️.ts";
import { type ChangeCPe, parseChangeCPe } from "./🧮change-c-pe/🧬️schema/🟦️.ts";
import { type ChangeConfinedDepth, parseChangeConfinedDepth } from "./🧰️change-confined-depth/🧬️schema/🟦️.ts";
import { type ChangeConfinedPosition, parseChangeConfinedPosition } from "./🧰️change-confined-position/🧬️schema/🟦️.ts";
import { type ChangeConfinedSteel, parseChangeConfinedSteel } from "./🧰️change-confined-steel/🧬️schema/🟦️.ts";
import { type ChangeConfinedWidth, parseChangeConfinedWidth } from "./🧰️change-confined-width/🧬️schema/🟦️.ts";
import { type ChangeDensity, parseChangeDensity } from "./🧱change-density/🧬️schema/🟦️.ts";
import { type ChangeSupportSides, parseChangeSupportSides } from "./🧱change-support-sides/🧬️schema/🟦️.ts";
import { type ChangeUnitFb, parseChangeUnitFb } from "./🧱change-unit-fb/🧬️schema/🟦️.ts";
import { type ChangeUnitGroup, parseChangeUnitGroup } from "./🧱change-unit-group/🧬️schema/🟦️.ts";
import { type ChangeUnitHeight, parseChangeUnitHeight } from "./🧱change-unit-height/🧬️schema/🟦️.ts";
import { type ChangeUnitLength, parseChangeUnitLength } from "./🧱change-unit-length/🧬️schema/🟦️.ts";
import { type ChangeUnitMaterial, parseChangeUnitMaterial } from "./🧱change-unit-material/🧬️schema/🟦️.ts";
import { type ChangeUnitWidth, parseChangeUnitWidth } from "./🧱change-unit-width/🧬️schema/🟦️.ts";
import { type ChangeWallType, parseChangeWallType } from "./🧱change-wall-type/🧬️schema/🟦️.ts";
import { type ChangeMu, parseChangeMu } from "./🧲️change-mu/🧬️schema/🟦️.ts";
import { type ChangeMovementJointPosition, parseChangeMovementJointPosition } from "./🧵️change-movement-joint-position/🧬️schema/🟦️.ts";
import { type ChangeMovementJointWall, parseChangeMovementJointWall } from "./🧵️change-movement-joint-wall/🧬️schema/🟦️.ts";
import { type InsertMovementJoint, parseInsertMovementJoint } from "./🧵️insert-movement-joint/🧬️schema/🟦️.ts";
import { type RemoveMovementJoint, parseRemoveMovementJoint } from "./🧵️remove-movement-joint/🧬️schema/🟦️.ts";
import { type ChangeBracingSystem, parseChangeBracingSystem } from "./🧷️change-bracing-system/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmStiffness, parseChangeDiaphragmStiffness } from "./🧷️change-diaphragm-stiffness/🧬️schema/🟦️.ts";
import { type ChangeChaseDepth, parseChangeChaseDepth } from "./🪚️change-chase-depth/🧬️schema/🟦️.ts";
import { type ChangeChaseKind, parseChangeChaseKind } from "./🪚️change-chase-kind/🧬️schema/🟦️.ts";
import { type ChangeChaseLength, parseChangeChaseLength } from "./🪚️change-chase-length/🧬️schema/🟦️.ts";
import { type ChangeChaseWall, parseChangeChaseWall } from "./🪚️change-chase-wall/🧬️schema/🟦️.ts";
import { type ChangeChaseWidth, parseChangeChaseWidth } from "./🪚️change-chase-width/🧬️schema/🟦️.ts";
import { type ChangeChaseX, parseChangeChaseX } from "./🪚️change-chase-x/🧬️schema/🟦️.ts";
import { type ChangeChaseY, parseChangeChaseY } from "./🪚️change-chase-y/🧬️schema/🟦️.ts";
import { type InsertChase, parseInsertChase } from "./🪚️insert-chase/🧬️schema/🟦️.ts";
import { type RemoveChase, parseRemoveChase } from "./🪚️remove-chase/🧬️schema/🟦️.ts";
import { type ChangeOpeningHeight, parseChangeOpeningHeight } from "./🪟change-opening-height/🧬️schema/🟦️.ts";
import { type ChangeOpeningPosition, parseChangeOpeningPosition } from "./🪟change-opening-position/🧬️schema/🟦️.ts";
import { type ChangeOpeningSill, parseChangeOpeningSill } from "./🪟change-opening-sill/🧬️schema/🟦️.ts";
import { type ChangeOpeningWidth, parseChangeOpeningWidth } from "./🪟change-opening-width/🧬️schema/🟦️.ts";
import { type ChangeBarDiameter, parseChangeBarDiameter } from "./🪢️change-bar-diameter/🧬️schema/🟦️.ts";
import { type ChangeBarKind, parseChangeBarKind } from "./🪢️change-bar-kind/🧬️schema/🟦️.ts";
import { type ChangeBarLength, parseChangeBarLength } from "./🪢️change-bar-length/🧬️schema/🟦️.ts";
import { type ChangeBarSpacing, parseChangeBarSpacing } from "./🪢️change-bar-spacing/🧬️schema/🟦️.ts";
import { type ChangeHKEarth, parseChangeHKEarth } from "./🪨change-hk-earth/🧬️schema/🟦️.ts";
import { type ChangeSoilDensity, parseChangeSoilDensity } from "./🪨change-soil-density/🧬️schema/🟦️.ts";

export type En1996Mutation =
  | { ChangeConcentratedBearingLength: ChangeConcentratedBearingLength }
  | { ChangeSlabSpan: ChangeSlabSpan }
  | { ChangeWallLength: ChangeWallLength }
  | { ChangeWallHeight: ChangeWallHeight }
  | { ChangeWallThickness: ChangeWallThickness }
  | { ChangeEccentricityBottom: ChangeEccentricityBottom }
  | { ChangeEccentricityTop: ChangeEccentricityTop }
  | { ChangePhiInfinity: ChangePhiInfinity }
  | { ChangeQKSnow: ChangeQKSnow }
  | { InsertConcentrated: InsertConcentrated }
  | { InsertLoadCase: InsertLoadCase }
  | { InsertOpening: InsertOpening }
  | { InsertWall: InsertWall }
  | { RemoveConcentrated: RemoveConcentrated }
  | { RemoveLoadCase: RemoveLoadCase }
  | { RemoveOpening: RemoveOpening }
  | { RemoveWall: RemoveWall }
  | { ChangeAnnex: ChangeAnnex }
  | { ChangeReliabilityClass: ChangeReliabilityClass }
  | { ChangeQPWind: ChangeQPWind }
  | { ChangeDesignSituation: ChangeDesignSituation }
  | { ChangeLoadCaseSituation: ChangeLoadCaseSituation }
  | { ChangeConcentratedForce: ChangeConcentratedForce }
  | { ChangeGKSlab: ChangeGKSlab }
  | { ChangeQKImposed: ChangeQKImposed }
  | { ChangeIsBasement: ChangeIsBasement }
  | { ChangeStoreys: ChangeStoreys }
  | { ChangeMasonryClass: ChangeMasonryClass }
  | { ChangeImposedCategory: ChangeImposedCategory }
  | { ChangeWallLabelDe: ChangeWallLabelDe }
  | { ChangeWallLabelEn: ChangeWallLabelEn }
  | { ChangeExposure: ChangeExposure }
  | { ChangeConcentratedBearingArea: ChangeConcentratedBearingArea }
  | { ChangeSlabBearingDepth: ChangeSlabBearingDepth }
  | { ChangeTributaryArea: ChangeTributaryArea }
  | { ChangeFireRei: ChangeFireRei }
  | { ChangeAsHorizontal: ChangeAsHorizontal }
  | { ChangeAsVertical: ChangeAsVertical }
  | { ChangeFYd: ChangeFYd }
  | { ChangeReinforced: ChangeReinforced }
  | { ChangeBedJointThickness: ChangeBedJointThickness }
  | { ChangeFm: ChangeFm }
  | { ChangeMortarClass: ChangeMortarClass }
  | { ChangeMortarType: ChangeMortarType }
  | { ChangeCPe: ChangeCPe }
  | { ChangeDensity: ChangeDensity }
  | { ChangeSupportSides: ChangeSupportSides }
  | { ChangeUnitFb: ChangeUnitFb }
  | { ChangeUnitGroup: ChangeUnitGroup }
  | { ChangeUnitHeight: ChangeUnitHeight }
  | { ChangeUnitLength: ChangeUnitLength }
  | { ChangeUnitMaterial: ChangeUnitMaterial }
  | { ChangeUnitWidth: ChangeUnitWidth }
  | { ChangeWallType: ChangeWallType }
  | { ChangeMu: ChangeMu }
  | { ChangeOpeningHeight: ChangeOpeningHeight }
  | { ChangeOpeningSill: ChangeOpeningSill }
  | { ChangeOpeningWidth: ChangeOpeningWidth }
  | { ChangeHKEarth: ChangeHKEarth }
  | { ChangePierPosition: ChangePierPosition }
  | { ChangePierWidth: ChangePierWidth }
  | { InsertPier: InsertPier }
  | { RemovePier: RemovePier }
  | { ChangeLintelPosition: ChangeLintelPosition }
  | { ChangeLintelSpan: ChangeLintelSpan }
  | { ChangeLintelDepth: ChangeLintelDepth }
  | { ChangeLintelBearingLength: ChangeLintelBearingLength }
  | { InsertLintel: InsertLintel }
  | { RemoveLintel: RemoveLintel }
  | { ChangeTieKind: ChangeTieKind }
  | { ChangeTiePosition: ChangeTiePosition }
  | { ChangeTieLevel: ChangeTieLevel }
  | { ChangeTieCapacity: ChangeTieCapacity }
  | { InsertTie: InsertTie }
  | { RemoveTie: RemoveTie }
  | { ChangePrestressForce: ChangePrestressForce }
  | { ChangePrestressArea: ChangePrestressArea }
  | { ChangePrestressStrength: ChangePrestressStrength }
  | { ChangePrestressPosition: ChangePrestressPosition }
  | { ChangePrestressEccentricity: ChangePrestressEccentricity }
  | { ChangePrestressLoss: ChangePrestressLoss }
  | { InsertPrestress: InsertPrestress }
  | { RemovePrestress: RemovePrestress }
  | { ChangeConfinedPosition: ChangeConfinedPosition }
  | { ChangeConfinedWidth: ChangeConfinedWidth }
  | { ChangeConfinedDepth: ChangeConfinedDepth }
  | { ChangeConfinedSteel: ChangeConfinedSteel }
  | { InsertConfined: InsertConfined }
  | { RemoveConfined: RemoveConfined }
  | { ChangeFlangeLength: ChangeFlangeLength }
  | { ChangeFlangeThickness: ChangeFlangeThickness }
  | { ChangeFlangePosition: ChangeFlangePosition }
  | { InsertFlange: InsertFlange }
  | { RemoveFlange: RemoveFlange }
  | { ChangeBarKind: ChangeBarKind }
  | { ChangeBarDiameter: ChangeBarDiameter }
  | { ChangeBarLength: ChangeBarLength }
  | { ChangeBarSpacing: ChangeBarSpacing }
  | { InsertBar: InsertBar }
  | { RemoveBar: RemoveBar }
  | { ChangeFrostClass: ChangeFrostClass }
  | { ChangeCover: ChangeCover }
  | { ChangeDeflectionLimit: ChangeDeflectionLimit }
  | { ChangeSoilHeight: ChangeSoilHeight }
  | { ChangeSoilDensity: ChangeSoilDensity }
  | { ChangeIsothermDepth: ChangeIsothermDepth }
  | { ChangePlanX: ChangePlanX }
  | { ChangePlanY: ChangePlanY }
  | { ChangePlanAngle: ChangePlanAngle }
  | { ChangeInPlaneShear: ChangeInPlaneShear }
  | { ChangeInPlaneMoment: ChangeInPlaneMoment }
  | { ChangeOpeningPosition: ChangeOpeningPosition }
  | { ChangeConcentratedPosition: ChangeConcentratedPosition }
  | { ChangeConcentratedDistanceA1: ChangeConcentratedDistanceA1 }
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
  | { ChangeBracingSystem: ChangeBracingSystem }
  | { ChangeDiaphragmStiffness: ChangeDiaphragmStiffness }
  | { ChangeChaseWall: ChangeChaseWall }
  | { ChangeChaseKind: ChangeChaseKind }
  | { ChangeChaseX: ChangeChaseX }
  | { ChangeChaseY: ChangeChaseY }
  | { ChangeChaseWidth: ChangeChaseWidth }
  | { ChangeChaseLength: ChangeChaseLength }
  | { ChangeChaseDepth: ChangeChaseDepth }
  | { InsertChase: InsertChase }
  | { RemoveChase: RemoveChase }
  | { ChangeMovementJointWall: ChangeMovementJointWall }
  | { ChangeMovementJointPosition: ChangeMovementJointPosition }
  | { InsertMovementJoint: InsertMovementJoint }
  | { RemoveMovementJoint: RemoveMovementJoint };

export const parseEn1996Mutation: NormWireReader<En1996Mutation> = normWireExternal<En1996Mutation>({
  ChangeConcentratedBearingLength: parseChangeConcentratedBearingLength,
  ChangeSlabSpan: parseChangeSlabSpan,
  ChangeWallLength: parseChangeWallLength,
  ChangeWallHeight: parseChangeWallHeight,
  ChangeWallThickness: parseChangeWallThickness,
  ChangeEccentricityBottom: parseChangeEccentricityBottom,
  ChangeEccentricityTop: parseChangeEccentricityTop,
  ChangePhiInfinity: parseChangePhiInfinity,
  ChangeQKSnow: parseChangeQKSnow,
  InsertConcentrated: parseInsertConcentrated,
  InsertLoadCase: parseInsertLoadCase,
  InsertOpening: parseInsertOpening,
  InsertWall: parseInsertWall,
  RemoveConcentrated: parseRemoveConcentrated,
  RemoveLoadCase: parseRemoveLoadCase,
  RemoveOpening: parseRemoveOpening,
  RemoveWall: parseRemoveWall,
  ChangeAnnex: parseChangeAnnex,
  ChangeReliabilityClass: parseChangeReliabilityClass,
  ChangeQPWind: parseChangeQPWind,
  ChangeDesignSituation: parseChangeDesignSituation,
  ChangeLoadCaseSituation: parseChangeLoadCaseSituation,
  ChangeConcentratedForce: parseChangeConcentratedForce,
  ChangeGKSlab: parseChangeGKSlab,
  ChangeQKImposed: parseChangeQKImposed,
  ChangeIsBasement: parseChangeIsBasement,
  ChangeStoreys: parseChangeStoreys,
  ChangeMasonryClass: parseChangeMasonryClass,
  ChangeImposedCategory: parseChangeImposedCategory,
  ChangeWallLabelDe: parseChangeWallLabelDe,
  ChangeWallLabelEn: parseChangeWallLabelEn,
  ChangeExposure: parseChangeExposure,
  ChangeConcentratedBearingArea: parseChangeConcentratedBearingArea,
  ChangeSlabBearingDepth: parseChangeSlabBearingDepth,
  ChangeTributaryArea: parseChangeTributaryArea,
  ChangeFireRei: parseChangeFireRei,
  ChangeAsHorizontal: parseChangeAsHorizontal,
  ChangeAsVertical: parseChangeAsVertical,
  ChangeFYd: parseChangeFYd,
  ChangeReinforced: parseChangeReinforced,
  ChangeBedJointThickness: parseChangeBedJointThickness,
  ChangeFm: parseChangeFm,
  ChangeMortarClass: parseChangeMortarClass,
  ChangeMortarType: parseChangeMortarType,
  ChangeCPe: parseChangeCPe,
  ChangeDensity: parseChangeDensity,
  ChangeSupportSides: parseChangeSupportSides,
  ChangeUnitFb: parseChangeUnitFb,
  ChangeUnitGroup: parseChangeUnitGroup,
  ChangeUnitHeight: parseChangeUnitHeight,
  ChangeUnitLength: parseChangeUnitLength,
  ChangeUnitMaterial: parseChangeUnitMaterial,
  ChangeUnitWidth: parseChangeUnitWidth,
  ChangeWallType: parseChangeWallType,
  ChangeMu: parseChangeMu,
  ChangeOpeningHeight: parseChangeOpeningHeight,
  ChangeOpeningSill: parseChangeOpeningSill,
  ChangeOpeningWidth: parseChangeOpeningWidth,
  ChangeHKEarth: parseChangeHKEarth,
  ChangePierPosition: parseChangePierPosition,
  ChangePierWidth: parseChangePierWidth,
  InsertPier: parseInsertPier,
  RemovePier: parseRemovePier,
  ChangeLintelPosition: parseChangeLintelPosition,
  ChangeLintelSpan: parseChangeLintelSpan,
  ChangeLintelDepth: parseChangeLintelDepth,
  ChangeLintelBearingLength: parseChangeLintelBearingLength,
  InsertLintel: parseInsertLintel,
  RemoveLintel: parseRemoveLintel,
  ChangeTieKind: parseChangeTieKind,
  ChangeTiePosition: parseChangeTiePosition,
  ChangeTieLevel: parseChangeTieLevel,
  ChangeTieCapacity: parseChangeTieCapacity,
  InsertTie: parseInsertTie,
  RemoveTie: parseRemoveTie,
  ChangePrestressForce: parseChangePrestressForce,
  ChangePrestressArea: parseChangePrestressArea,
  ChangePrestressStrength: parseChangePrestressStrength,
  ChangePrestressPosition: parseChangePrestressPosition,
  ChangePrestressEccentricity: parseChangePrestressEccentricity,
  ChangePrestressLoss: parseChangePrestressLoss,
  InsertPrestress: parseInsertPrestress,
  RemovePrestress: parseRemovePrestress,
  ChangeConfinedPosition: parseChangeConfinedPosition,
  ChangeConfinedWidth: parseChangeConfinedWidth,
  ChangeConfinedDepth: parseChangeConfinedDepth,
  ChangeConfinedSteel: parseChangeConfinedSteel,
  InsertConfined: parseInsertConfined,
  RemoveConfined: parseRemoveConfined,
  ChangeFlangeLength: parseChangeFlangeLength,
  ChangeFlangeThickness: parseChangeFlangeThickness,
  ChangeFlangePosition: parseChangeFlangePosition,
  InsertFlange: parseInsertFlange,
  RemoveFlange: parseRemoveFlange,
  ChangeBarKind: parseChangeBarKind,
  ChangeBarDiameter: parseChangeBarDiameter,
  ChangeBarLength: parseChangeBarLength,
  ChangeBarSpacing: parseChangeBarSpacing,
  InsertBar: parseInsertBar,
  RemoveBar: parseRemoveBar,
  ChangeFrostClass: parseChangeFrostClass,
  ChangeCover: parseChangeCover,
  ChangeDeflectionLimit: parseChangeDeflectionLimit,
  ChangeSoilHeight: parseChangeSoilHeight,
  ChangeSoilDensity: parseChangeSoilDensity,
  ChangeIsothermDepth: parseChangeIsothermDepth,
  ChangePlanX: parseChangePlanX,
  ChangePlanY: parseChangePlanY,
  ChangePlanAngle: parseChangePlanAngle,
  ChangeInPlaneShear: parseChangeInPlaneShear,
  ChangeInPlaneMoment: parseChangeInPlaneMoment,
  ChangeOpeningPosition: parseChangeOpeningPosition,
  ChangeConcentratedPosition: parseChangeConcentratedPosition,
  ChangeConcentratedDistanceA1: parseChangeConcentratedDistanceA1,
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
  ChangeBracingSystem: parseChangeBracingSystem,
  ChangeDiaphragmStiffness: parseChangeDiaphragmStiffness,
  ChangeChaseWall: parseChangeChaseWall,
  ChangeChaseKind: parseChangeChaseKind,
  ChangeChaseX: parseChangeChaseX,
  ChangeChaseY: parseChangeChaseY,
  ChangeChaseWidth: parseChangeChaseWidth,
  ChangeChaseLength: parseChangeChaseLength,
  ChangeChaseDepth: parseChangeChaseDepth,
  InsertChase: parseInsertChase,
  RemoveChase: parseRemoveChase,
  ChangeMovementJointWall: parseChangeMovementJointWall,
  ChangeMovementJointPosition: parseChangeMovementJointPosition,
  InsertMovementJoint: parseInsertMovementJoint,
  RemoveMovementJoint: parseRemoveMovementJoint,
});
