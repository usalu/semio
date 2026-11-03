/** 🧺️ `En1997Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireTagged, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ChangeFootingWidth, parseChangeFootingWidth } from "./↔️change-footing-width/🧬️schema/🟦️.ts";
import { type ChangeAnchorBondStress, parseChangeAnchorBondStress } from "./⚓️change-anchor-bond-stress/🧬️schema/🟦️.ts";
import { type ChangeAnchorDesignForce, parseChangeAnchorDesignForce } from "./⚓️change-anchor-design-force/🧬️schema/🟦️.ts";
import { type ChangeAnchorFixedDiameter, parseChangeAnchorFixedDiameter } from "./⚓️change-anchor-fixed-diameter/🧬️schema/🟦️.ts";
import { type ChangeAnchorFixedLength, parseChangeAnchorFixedLength } from "./⚓️change-anchor-fixed-length/🧬️schema/🟦️.ts";
import { type ChangeAnchorFreeLength, parseChangeAnchorFreeLength } from "./⚓️change-anchor-free-length/🧬️schema/🟦️.ts";
import { type ChangeAnchorHeadDepth, parseChangeAnchorHeadDepth } from "./⚓️change-anchor-head-depth/🧬️schema/🟦️.ts";
import { type ChangeAnchorInclinationDeg, parseChangeAnchorInclinationDeg } from "./⚓️change-anchor-inclination-deg/🧬️schema/🟦️.ts";
import { type ChangeAnchorMinFreeLength, parseChangeAnchorMinFreeLength } from "./⚓️change-anchor-min-free-length/🧬️schema/🟦️.ts";
import { type ChangeAnchorTendonYieldForce, parseChangeAnchorTendonYieldForce } from "./⚓️change-anchor-tendon-yield-force/🧬️schema/🟦️.ts";
import { type ChangeAnchorTendon, parseChangeAnchorTendon } from "./⚓️change-anchor-tendon/🧬️schema/🟦️.ts";
import { type ChangeAnchorTestFactor, parseChangeAnchorTestFactor } from "./⚓️change-anchor-test-factor/🧬️schema/🟦️.ts";
import { type ChangeAnchorTestPullOutRes, parseChangeAnchorTestPullOutRes } from "./⚓️change-anchor-test-pull-out-res/🧬️schema/🟦️.ts";
import { type ChangeAnchorWallId, parseChangeAnchorWallId } from "./⚓️change-anchor-wall-id/🧬️schema/🟦️.ts";
import { type ChangeSlopeAngle, parseChangeSlopeAngle } from "./⛰️change-slope-angle/🧬️schema/🟦️.ts";
import { type ChangeSlopeCircleCentreX, parseChangeSlopeCircleCentreX } from "./⛰️change-slope-circle-centre-x/🧬️schema/🟦️.ts";
import { type ChangeSlopeCircleCentreZ, parseChangeSlopeCircleCentreZ } from "./⛰️change-slope-circle-centre-z/🧬️schema/🟦️.ts";
import { type ChangeSlopeCircleRadius, parseChangeSlopeCircleRadius } from "./⛰️change-slope-circle-radius/🧬️schema/🟦️.ts";
import { type ChangeSlopeGoverningLayerId, parseChangeSlopeGoverningLayerId } from "./⛰️change-slope-governing-layer-id/🧬️schema/🟦️.ts";
import { type ChangeSlopeHeight, parseChangeSlopeHeight } from "./⛰️change-slope-height/🧬️schema/🟦️.ts";
import { type ChangeSlopeLength, parseChangeSlopeLength } from "./⛰️change-slope-length/🧬️schema/🟦️.ts";
import { type ChangeSlopeWidth, parseChangeSlopeWidth } from "./⛰️change-slope-width/🧬️schema/🟦️.ts";
import { type InsertAnchor, parseInsertAnchor } from "./➕insert-anchor/🧬️schema/🟦️.ts";
import { type InsertCharacteristicBasis, parseInsertCharacteristicBasis } from "./➕insert-characteristic-basis/🧬️schema/🟦️.ts";
import { type InsertEmbeddedWall, parseInsertEmbeddedWall } from "./➕insert-embedded-wall/🧬️schema/🟦️.ts";
import { type InsertFooting, parseInsertFooting } from "./➕insert-footing/🧬️schema/🟦️.ts";
import { type InsertGroundInvestigation, parseInsertGroundInvestigation } from "./➕insert-ground-investigation/🧬️schema/🟦️.ts";
import { type InsertLabTest, parseInsertLabTest } from "./➕insert-lab-test/🧬️schema/🟦️.ts";
import { type InsertLayerWater, parseInsertLayerWater } from "./➕insert-layer-water/🧬️schema/🟦️.ts";
import { type InsertPileGroup, parseInsertPileGroup } from "./➕insert-pile-group/🧬️schema/🟦️.ts";
import { type InsertRetainingWall, parseInsertRetainingWall } from "./➕insert-retaining-wall/🧬️schema/🟦️.ts";
import { type InsertSlope, parseInsertSlope } from "./➕insert-slope/🧬️schema/🟦️.ts";
import { type InsertUpliftCase, parseInsertUpliftCase } from "./➕insert-uplift-case/🧬️schema/🟦️.ts";
import { type InsertAnchorTest, parseInsertAnchorTest } from "./➕️insert-anchor-test/🧬️schema/🟦️.ts";
import { type InsertCase, parseInsertCase } from "./➕️insert-case/🧬️schema/🟦️.ts";
import { type InsertGroundInvestigationReading, parseInsertGroundInvestigationReading } from "./➕️insert-ground-investigation-reading/🧬️schema/🟦️.ts";
import { type InsertLabPoint, parseInsertLabPoint } from "./➕️insert-lab-point/🧬️schema/🟦️.ts";
import { type InsertLayer, parseInsertLayer } from "./➕️insert-layer/🧬️schema/🟦️.ts";
import { type InsertLoadCase, parseInsertLoadCase } from "./➕️insert-load-case/🧬️schema/🟦️.ts";
import { type InsertTestProfile, parseInsertTestProfile } from "./➕️insert-test-profile/🧬️schema/🟦️.ts";
import { type RemoveAnchor, parseRemoveAnchor } from "./➖remove-anchor/🧬️schema/🟦️.ts";
import { type RemoveCharacteristicBasis, parseRemoveCharacteristicBasis } from "./➖remove-characteristic-basis/🧬️schema/🟦️.ts";
import { type RemoveEmbeddedWall, parseRemoveEmbeddedWall } from "./➖remove-embedded-wall/🧬️schema/🟦️.ts";
import { type RemoveFooting, parseRemoveFooting } from "./➖remove-footing/🧬️schema/🟦️.ts";
import { type RemoveGroundInvestigation, parseRemoveGroundInvestigation } from "./➖remove-ground-investigation/🧬️schema/🟦️.ts";
import { type RemoveLabTest, parseRemoveLabTest } from "./➖remove-lab-test/🧬️schema/🟦️.ts";
import { type RemoveLayerWater, parseRemoveLayerWater } from "./➖remove-layer-water/🧬️schema/🟦️.ts";
import { type RemovePileGroup, parseRemovePileGroup } from "./➖remove-pile-group/🧬️schema/🟦️.ts";
import { type RemoveRetainingWall, parseRemoveRetainingWall } from "./➖remove-retaining-wall/🧬️schema/🟦️.ts";
import { type RemoveSlope, parseRemoveSlope } from "./➖remove-slope/🧬️schema/🟦️.ts";
import { type RemoveUpliftCase, parseRemoveUpliftCase } from "./➖remove-uplift-case/🧬️schema/🟦️.ts";
import { type RemoveAnchorTest, parseRemoveAnchorTest } from "./➖️remove-anchor-test/🧬️schema/🟦️.ts";
import { type RemoveCase, parseRemoveCase } from "./➖️remove-case/🧬️schema/🟦️.ts";
import { type RemoveGroundInvestigationReading, parseRemoveGroundInvestigationReading } from "./➖️remove-ground-investigation-reading/🧬️schema/🟦️.ts";
import { type RemoveLabPoint, parseRemoveLabPoint } from "./➖️remove-lab-point/🧬️schema/🟦️.ts";
import { type RemoveLayer, parseRemoveLayer } from "./➖️remove-layer/🧬️schema/🟦️.ts";
import { type RemoveLoadCase, parseRemoveLoadCase } from "./➖️remove-load-case/🧬️schema/🟦️.ts";
import { type RemoveTestProfile, parseRemoveTestProfile } from "./➖️remove-test-profile/🧬️schema/🟦️.ts";
import { type ChangeUpliftPermDestabilizing, parseChangeUpliftPermDestabilizing } from "./⬆️change-uplift-perm-destabilizing/🧬️schema/🟦️.ts";
import { type ChangeUpliftPermStabilizing, parseChangeUpliftPermStabilizing } from "./⬆️change-uplift-perm-stabilizing/🧬️schema/🟦️.ts";
import { type ChangeUpliftPorePressure, parseChangeUpliftPorePressure } from "./⬆️change-uplift-pore-pressure/🧬️schema/🟦️.ts";
import { type ChangeUpliftTotalStress, parseChangeUpliftTotalStress } from "./⬆️change-uplift-total-stress/🧬️schema/🟦️.ts";
import { type ChangeUpliftVarDestabilizing, parseChangeUpliftVarDestabilizing } from "./⬆️change-uplift-var-destabilizing/🧬️schema/🟦️.ts";
import { type ChangeFootingEmbedment, parseChangeFootingEmbedment } from "./⬇️change-footing-embedment/🧬️schema/🟦️.ts";
import { type ChangeLayerOedometricModulus, parseChangeLayerOedometricModulus } from "./🌀️change-layer-oedometric-modulus/🧬️schema/🟦️.ts";
import { type ChangeAnnex, parseChangeAnnex } from "./🌍️change-annex/🧬️schema/🟦️.ts";
import { type ChangePileGroupBaseCohesion, parseChangePileGroupBaseCohesion } from "./🏗️change-pile-group-base-cohesion/🧬️schema/🟦️.ts";
import { type ChangePileGroupCapMomentX, parseChangePileGroupCapMomentX } from "./🏗️change-pile-group-cap-moment-x/🧬️schema/🟦️.ts";
import { type ChangePileGroupCapMomentY, parseChangePileGroupCapMomentY } from "./🏗️change-pile-group-cap-moment-y/🧬️schema/🟦️.ts";
import { type ChangePileGroupCapVertical, parseChangePileGroupCapVertical } from "./🏗️change-pile-group-cap-vertical/🧬️schema/🟦️.ts";
import { type ChangePileGroupColumns, parseChangePileGroupColumns } from "./🏗️change-pile-group-columns/🧬️schema/🟦️.ts";
import { type ChangePileGroupPileId, parseChangePileGroupPileId } from "./🏗️change-pile-group-pile-id/🧬️schema/🟦️.ts";
import { type ChangePileGroupRows, parseChangePileGroupRows } from "./🏗️change-pile-group-rows/🧬️schema/🟦️.ts";
import { type ChangePileGroupShaftCohSum, parseChangePileGroupShaftCohSum } from "./🏗️change-pile-group-shaft-coh-sum/🧬️schema/🟦️.ts";
import { type ChangePileGroupSingleRes, parseChangePileGroupSingleRes } from "./🏗️change-pile-group-single-res/🧬️schema/🟦️.ts";
import { type ChangePileGroupSpacing, parseChangePileGroupSpacing } from "./🏗️change-pile-group-spacing/🧬️schema/🟦️.ts";
import { type ChangeStructureId, parseChangeStructureId } from "./🏗️change-structure-id/🧬️schema/🟦️.ts";
import { type ChangeFootingBaseInclDeg, parseChangeFootingBaseInclDeg } from "./🏛️change-footing-base-incl-deg/🧬️schema/🟦️.ts";
import { type ChangeFootingLength, parseChangeFootingLength } from "./🏛️change-footing-length/🧬️schema/🟦️.ts";
import { type ChangeFootingSettlementLimit, parseChangeFootingSettlementLimit } from "./🏛️change-footing-settlement-limit/🧬️schema/🟦️.ts";
import { type ChangeLoadCaseDesignSituation, parseChangeLoadCaseDesignSituation } from "./🏛️change-load-case-design-situation/🧬️schema/🟦️.ts";
import { type ChangeLoadCaseHorizPermanent, parseChangeLoadCaseHorizPermanent } from "./🏛️change-load-case-horiz-permanent/🧬️schema/🟦️.ts";
import { type ChangeLoadCaseHorizVariable, parseChangeLoadCaseHorizVariable } from "./🏛️change-load-case-horiz-variable/🧬️schema/🟦️.ts";
import { type ChangeLoadCaseMomentPermanent, parseChangeLoadCaseMomentPermanent } from "./🏛️change-load-case-moment-permanent/🧬️schema/🟦️.ts";
import { type ChangeLoadCaseMomentVariable, parseChangeLoadCaseMomentVariable } from "./🏛️change-load-case-moment-variable/🧬️schema/🟦️.ts";
import { type ChangeLoadCaseVerticalPerm, parseChangeLoadCaseVerticalPerm } from "./🏛️change-load-case-vertical-perm/🧬️schema/🟦️.ts";
import { type ChangeLoadCaseVerticalVar, parseChangeLoadCaseVerticalVar } from "./🏛️change-load-case-vertical-var/🧬️schema/🟦️.ts";
import { type ChangeCaseLabelDe, parseChangeCaseLabelDe } from "./🏷️change-case-label-de/🧬️schema/🟦️.ts";
import { type ChangeCaseLabelEn, parseChangeCaseLabelEn } from "./🏷️change-case-label-en/🧬️schema/🟦️.ts";
import { type ChangeGroundwaterLevel, parseChangeGroundwaterLevel } from "./💧change-groundwater-level/🧬️schema/🟦️.ts";
import { type ChangeWaterFlowLength, parseChangeWaterFlowLength } from "./💧️change-water-flow-length/🧬️schema/🟦️.ts";
import { type ChangeWaterHeadDifference, parseChangeWaterHeadDifference } from "./💧️change-water-head-difference/🧬️schema/🟦️.ts";
import { type ChangeWaterInfluenceRadius, parseChangeWaterInfluenceRadius } from "./💧️change-water-influence-radius/🧬️schema/🟦️.ts";
import { type ChangeWaterLayerId, parseChangeWaterLayerId } from "./💧️change-water-layer-id/🧬️schema/🟦️.ts";
import { type ChangeWaterPermeability, parseChangeWaterPermeability } from "./💧️change-water-permeability/🧬️schema/🟦️.ts";
import { type ChangeWaterPiezometricDepth, parseChangeWaterPiezometricDepth } from "./💧️change-water-piezometric-depth/🧬️schema/🟦️.ts";
import { type ChangeWaterWellCapacity, parseChangeWaterWellCapacity } from "./💧️change-water-well-capacity/🧬️schema/🟦️.ts";
import { type ChangeWaterWellRadius, parseChangeWaterWellRadius } from "./💧️change-water-well-radius/🧬️schema/🟦️.ts";
import { type ChangeWaterWellWaterDepth, parseChangeWaterWellWaterDepth } from "./💧️change-water-well-water-depth/🧬️schema/🟦️.ts";
import { type ChangeDesignSituation, parseChangeDesignSituation } from "./📅️change-design-situation/🧬️schema/🟦️.ts";
import { type ChangeCaseEntryId, parseChangeCaseEntryId } from "./📇️change-case-entry-id/🧬️schema/🟦️.ts";
import { type ChangeGroundReadingDepth, parseChangeGroundReadingDepth } from "./📈️change-ground-reading-depth/🧬️schema/🟦️.ts";
import { type ChangeGroundReadingMeasurement, parseChangeGroundReadingMeasurement } from "./📈️change-ground-reading-measurement/🧬️schema/🟦️.ts";
import { type ChangePlacementX, parseChangePlacementX } from "./📍️change-placement-x/🧬️schema/🟦️.ts";
import { type ChangePlacementY, parseChangePlacementY } from "./📍️change-placement-y/🧬️schema/🟦️.ts";
import { type ChangePlacementYaw, parseChangePlacementYaw } from "./📍️change-placement-yaw/🧬️schema/🟦️.ts";
import { type ChangePlacementZ, parseChangePlacementZ } from "./📍️change-placement-z/🧬️schema/🟦️.ts";
import { type InsertPlacement, parseInsertPlacement } from "./📍️insert-placement/🧬️schema/🟦️.ts";
import { type RemovePlacement, parseRemovePlacement } from "./📍️remove-placement/🧬️schema/🟦️.ts";
import { type ChangePileLength, parseChangePileLength } from "./📏️change-pile-length/🧬️schema/🟦️.ts";
import { type ChangeLayerPhiPrime, parseChangeLayerPhiPrime } from "./📐️change-layer-phi-prime/🧬️schema/🟦️.ts";
import { type RemovePile, parseRemovePile } from "./📤remove-pile/🧬️schema/🟦️.ts";
import { type InsertPile, parseInsertPile } from "./📥insert-pile/🧬️schema/🟦️.ts";
import { type ChangePileAlphaS, parseChangePileAlphaS } from "./🔄️change-pile-alpha-s/🧬️schema/🟦️.ts";
import { type ChangePileBendingStiffnessEi, parseChangePileBendingStiffnessEi } from "./🔄️change-pile-bending-stiffness-ei/🧬️schema/🟦️.ts";
import { type ChangePileCompressionPermanent, parseChangePileCompressionPermanent } from "./🔄️change-pile-compression-permanent/🧬️schema/🟦️.ts";
import { type ChangePileCompressionVariable, parseChangePileCompressionVariable } from "./🔄️change-pile-compression-variable/🧬️schema/🟦️.ts";
import { type ChangePileConcreteClass, parseChangePileConcreteClass } from "./🔄️change-pile-concrete-class/🧬️schema/🟦️.ts";
import { type ChangePileDiameter, parseChangePileDiameter } from "./🔄️change-pile-diameter/🧬️schema/🟦️.ts";
import { type ChangePileDropHeight, parseChangePileDropHeight } from "./🔄️change-pile-drop-height/🧬️schema/🟦️.ts";
import { type ChangePileDynamicResMean, parseChangePileDynamicResMean } from "./🔄️change-pile-dynamic-res-mean/🧬️schema/🟦️.ts";
import { type ChangePileDynamicResMin, parseChangePileDynamicResMin } from "./🔄️change-pile-dynamic-res-min/🧬️schema/🟦️.ts";
import { type ChangePileDynamicTestCount, parseChangePileDynamicTestCount } from "./🔄️change-pile-dynamic-test-count/🧬️schema/🟦️.ts";
import { type ChangePileElasticCompression, parseChangePileElasticCompression } from "./🔄️change-pile-elastic-compression/🧬️schema/🟦️.ts";
import { type ChangePileElasticModulus, parseChangePileElasticModulus } from "./🔄️change-pile-elastic-modulus/🧬️schema/🟦️.ts";
import { type ChangePileHammerEfficiency, parseChangePileHammerEfficiency } from "./🔄️change-pile-hammer-efficiency/🧬️schema/🟦️.ts";
import { type ChangePileHammerWeight, parseChangePileHammerWeight } from "./🔄️change-pile-hammer-weight/🧬️schema/🟦️.ts";
import { type ChangePileHorizBedModulus, parseChangePileHorizBedModulus } from "./🔄️change-pile-horiz-bed-modulus/🧬️schema/🟦️.ts";
import { type ChangePileLateralHeadLoad, parseChangePileLateralHeadLoad } from "./🔄️change-pile-lateral-head-load/🧬️schema/🟦️.ts";
import { type ChangePileNegFrictionBeta, parseChangePileNegFrictionBeta } from "./🔄️change-pile-neg-friction-beta/🧬️schema/🟦️.ts";
import { type ChangePilePileType, parseChangePilePileType } from "./🔄️change-pile-pile-type/🧬️schema/🟦️.ts";
import { type ChangePileSetPerBlow, parseChangePileSetPerBlow } from "./🔄️change-pile-set-per-blow/🧬️schema/🟦️.ts";
import { type ChangePileSettlingThickness, parseChangePileSettlingThickness } from "./🔄️change-pile-settling-thickness/🧬️schema/🟦️.ts";
import { type ChangePileTensionPermanent, parseChangePileTensionPermanent } from "./🔄️change-pile-tension-permanent/🧬️schema/🟦️.ts";
import { type ChangePileTensionVariable, parseChangePileTensionVariable } from "./🔄️change-pile-tension-variable/🧬️schema/🟦️.ts";
import { type ChangePileUnitBaseResistance, parseChangePileUnitBaseResistance } from "./🔄️change-pile-unit-base-resistance/🧬️schema/🟦️.ts";
import { type ChangePileUnitShaftResistance, parseChangePileUnitShaftResistance } from "./🔄️change-pile-unit-shaft-resistance/🧬️schema/🟦️.ts";
import { type ChangeProfileBaseResistance, parseChangeProfileBaseResistance } from "./🔄️change-profile-base-resistance/🧬️schema/🟦️.ts";
import { type ChangeProfileShaftResistance, parseChangeProfileShaftResistance } from "./🔄️change-profile-shaft-resistance/🧬️schema/🟦️.ts";
import { type ChangeInvestigationDepth, parseChangeInvestigationDepth } from "./🔎️change-investigation-depth/🧬️schema/🟦️.ts";
import { type ChangePileCount, parseChangePileCount } from "./🔢change-pile-count/🧬️schema/🟦️.ts";
import { type ChangeCaseNumber, parseChangeCaseNumber } from "./🔢️change-case-number/🧬️schema/🟦️.ts";
import { type ChangeCaseText, parseChangeCaseText } from "./🔤️change-case-text/🧬️schema/🟦️.ts";
import { type ChangeGroundInvestigationDepth, parseChangeGroundInvestigationDepth } from "./🕳️change-ground-investigation-depth/🧬️schema/🟦️.ts";
import { type ChangeGroundInvestigationKind, parseChangeGroundInvestigationKind } from "./🕳️change-ground-investigation-kind/🧬️schema/🟦️.ts";
import { type ChangeGroundInvestigationLevel, parseChangeGroundInvestigationLevel } from "./🕳️change-ground-investigation-level/🧬️schema/🟦️.ts";
import { type ChangeGroundInvestigationPositionX, parseChangeGroundInvestigationPositionX } from "./🕳️change-ground-investigation-position-x/🧬️schema/🟦️.ts";
import { type ChangeGroundInvestigationPositionY, parseChangeGroundInvestigationPositionY } from "./🕳️change-ground-investigation-position-y/🧬️schema/🟦️.ts";
import { type ChangeGeotechnicalCategory, parseChangeGeotechnicalCategory } from "./🗂️change-geotechnical-category/🧬️schema/🟦️.ts";
import { type ChangeWallBackfillGamma, parseChangeWallBackfillGamma } from "./🗄️change-wall-backfill-gamma/🧬️schema/🟦️.ts";
import { type ChangeWallBackfillPhiDeg, parseChangeWallBackfillPhiDeg } from "./🗄️change-wall-backfill-phi-deg/🧬️schema/🟦️.ts";
import { type ChangeWallConcreteGamma, parseChangeWallConcreteGamma } from "./🗄️change-wall-concrete-gamma/🧬️schema/🟦️.ts";
import { type ChangeWallEarthPressureMode, parseChangeWallEarthPressureMode } from "./🗄️change-wall-earth-pressure-mode/🧬️schema/🟦️.ts";
import { type ChangeWallEmbedment, parseChangeWallEmbedment } from "./🗄️change-wall-embedment/🧬️schema/🟦️.ts";
import { type ChangeWallHeight, parseChangeWallHeight } from "./🗄️change-wall-height/🧬️schema/🟦️.ts";
import { type ChangeWallHorizontalPermanent, parseChangeWallHorizontalPermanent } from "./🗄️change-wall-horizontal-permanent/🧬️schema/🟦️.ts";
import { type ChangeWallOcr, parseChangeWallOcr } from "./🗄️change-wall-ocr/🧬️schema/🟦️.ts";
import { type ChangeWallRebarFriction, parseChangeWallRebarFriction } from "./🗄️change-wall-rebar-friction/🧬️schema/🟦️.ts";
import { type ChangeWallReinforcementSpacing, parseChangeWallReinforcementSpacing } from "./🗄️change-wall-reinforcement-spacing/🧬️schema/🟦️.ts";
import { type ChangeWallReinforcementWidth, parseChangeWallReinforcementWidth } from "./🗄️change-wall-reinforcement-width/🧬️schema/🟦️.ts";
import { type ChangeWallSeismicKh, parseChangeWallSeismicKh } from "./🗄️change-wall-seismic-kh/🧬️schema/🟦️.ts";
import { type ChangeWallSeismicKv, parseChangeWallSeismicKv } from "./🗄️change-wall-seismic-kv/🧬️schema/🟦️.ts";
import { type ChangeWallStemThickness, parseChangeWallStemThickness } from "./🗄️change-wall-stem-thickness/🧬️schema/🟦️.ts";
import { type ChangeWallSurcharge, parseChangeWallSurcharge } from "./🗄️change-wall-surcharge/🧬️schema/🟦️.ts";
import { type ChangeWallVerticalPermanent, parseChangeWallVerticalPermanent } from "./🗄️change-wall-vertical-permanent/🧬️schema/🟦️.ts";
import { type ChangeWallWallFrictionDeg, parseChangeWallWallFrictionDeg } from "./🗄️change-wall-wall-friction-deg/🧬️schema/🟦️.ts";
import { type ChangeWallWallMovement, parseChangeWallWallMovement } from "./🗄️change-wall-wall-movement/🧬️schema/🟦️.ts";
import { type ChangeLayerCohesionEffective, parseChangeLayerCohesionEffective } from "./🥞️change-layer-cohesion-effective/🧬️schema/🟦️.ts";
import { type ChangeLayerCohesionUndrained, parseChangeLayerCohesionUndrained } from "./🥞️change-layer-cohesion-undrained/🧬️schema/🟦️.ts";
import { type ChangeLayerCptQc, parseChangeLayerCptQc } from "./🥞️change-layer-cpt-qc/🧬️schema/🟦️.ts";
import { type ChangeLayerDepthBottom, parseChangeLayerDepthBottom } from "./🥞️change-layer-depth-bottom/🧬️schema/🟦️.ts";
import { type ChangeLayerDepthTop, parseChangeLayerDepthTop } from "./🥞️change-layer-depth-top/🧬️schema/🟦️.ts";
import { type ChangeLayerGammaPrime, parseChangeLayerGammaPrime } from "./🥞️change-layer-gamma-prime/🧬️schema/🟦️.ts";
import { type ChangeLayerGamma, parseChangeLayerGamma } from "./🥞️change-layer-gamma/🧬️schema/🟦️.ts";
import { type ChangeLayerPoissonRatio, parseChangeLayerPoissonRatio } from "./🥞️change-layer-poisson-ratio/🧬️schema/🟦️.ts";
import { type ChangeLayerSoilType, parseChangeLayerSoilType } from "./🥞️change-layer-soil-type/🧬️schema/🟦️.ts";
import { type ChangeLayerSptN, parseChangeLayerSptN } from "./🥞️change-layer-spt-n/🧬️schema/🟦️.ts";
import { type ChangeLabPointNormalStress, parseChangeLabPointNormalStress } from "./🧪️change-lab-point-normal-stress/🧬️schema/🟦️.ts";
import { type ChangeLabPointShearStress, parseChangeLabPointShearStress } from "./🧪️change-lab-point-shear-stress/🧬️schema/🟦️.ts";
import { type ChangeLabTestCompressionIndex, parseChangeLabTestCompressionIndex } from "./🧪️change-lab-test-compression-index/🧬️schema/🟦️.ts";
import { type ChangeLabTestConsolTime, parseChangeLabTestConsolTime } from "./🧪️change-lab-test-consol-time/🧬️schema/🟦️.ts";
import { type ChangeLabTestDesignLife, parseChangeLabTestDesignLife } from "./🧪️change-lab-test-design-life/🧬️schema/🟦️.ts";
import { type ChangeLabTestDrainageHeight, parseChangeLabTestDrainageHeight } from "./🧪️change-lab-test-drainage-height/🧬️schema/🟦️.ts";
import { type ChangeLabTestKind, parseChangeLabTestKind } from "./🧪️change-lab-test-kind/🧬️schema/🟦️.ts";
import { type ChangeLabTestLayerId, parseChangeLabTestLayerId } from "./🧪️change-lab-test-layer-id/🧬️schema/🟦️.ts";
import { type ChangeLabTestPreloadStress, parseChangeLabTestPreloadStress } from "./🧪️change-lab-test-preload-stress/🧬️schema/🟦️.ts";
import { type ChangeLabTestSecComprIndex, parseChangeLabTestSecComprIndex } from "./🧪️change-lab-test-sec-compr-index/🧬️schema/🟦️.ts";
import { type ChangeLabTestStressIncrement, parseChangeLabTestStressIncrement } from "./🧪️change-lab-test-stress-increment/🧬️schema/🟦️.ts";
import { type ChangeLabTestSwellingIndex, parseChangeLabTestSwellingIndex } from "./🧪️change-lab-test-swelling-index/🧬️schema/🟦️.ts";
import { type ChangeLabTestVoidRatio, parseChangeLabTestVoidRatio } from "./🧪️change-lab-test-void-ratio/🧬️schema/🟦️.ts";
import { type ChangeDesignApproach, parseChangeDesignApproach } from "./🧭️change-design-approach/🧬️schema/🟦️.ts";
import { type ChangeCharacteristicBasisBasis, parseChangeCharacteristicBasisBasis } from "./🧮️change-characteristic-basis-basis/🧬️schema/🟦️.ts";
import { type ChangeCharacteristicBasisLayerId, parseChangeCharacteristicBasisLayerId } from "./🧮️change-characteristic-basis-layer-id/🧬️schema/🟦️.ts";
import { type ChangeCharacteristicBasisParameter, parseChangeCharacteristicBasisParameter } from "./🧮️change-characteristic-basis-parameter/🧬️schema/🟦️.ts";
import { type ChangeWallBaseWidth, parseChangeWallBaseWidth } from "./🧱change-wall-base-width/🧬️schema/🟦️.ts";
import { type ChangeEmbWallBackfillGamma, parseChangeEmbWallBackfillGamma } from "./🧱️change-emb-wall-backfill-gamma/🧬️schema/🟦️.ts";
import { type ChangeEmbWallDeltaActiveDeg, parseChangeEmbWallDeltaActiveDeg } from "./🧱️change-emb-wall-delta-active-deg/🧬️schema/🟦️.ts";
import { type ChangeEmbWallDeltaPassiveDeg, parseChangeEmbWallDeltaPassiveDeg } from "./🧱️change-emb-wall-delta-passive-deg/🧬️schema/🟦️.ts";
import { type ChangeEmbWallEmbedment, parseChangeEmbWallEmbedment } from "./🧱️change-emb-wall-embedment/🧬️schema/🟦️.ts";
import { type ChangeEmbWallKind, parseChangeEmbWallKind } from "./🧱️change-emb-wall-kind/🧬️schema/🟦️.ts";
import { type ChangeEmbWallPhiDeg, parseChangeEmbWallPhiDeg } from "./🧱️change-emb-wall-phi-deg/🧬️schema/🟦️.ts";
import { type ChangeEmbWallPropDepth, parseChangeEmbWallPropDepth } from "./🧱️change-emb-wall-prop-depth/🧬️schema/🟦️.ts";
import { type ChangeEmbWallRetainedHeight, parseChangeEmbWallRetainedHeight } from "./🧱️change-emb-wall-retained-height/🧬️schema/🟦️.ts";
import { type ChangeEmbWallSurcharge, parseChangeEmbWallSurcharge } from "./🧱️change-emb-wall-surcharge/🧬️schema/🟦️.ts";
import { type ChangeEmbWallThickness, parseChangeEmbWallThickness } from "./🧱️change-emb-wall-thickness/🧬️schema/🟦️.ts";
import { type ChangeEmbWallUndrStrength, parseChangeEmbWallUndrStrength } from "./🧱️change-emb-wall-undr-strength/🧬️schema/🟦️.ts";

export type En1997Mutation =
  | ChangeAnnex
  | ChangeGeotechnicalCategory
  | ChangeDesignSituation
  | ChangeDesignApproach
  | ChangeGroundwaterLevel
  | ChangeInvestigationDepth
  | ChangeFootingWidth
  | ChangeFootingEmbedment
  | ChangePileLength
  | ChangePileCount
  | ChangeWallBaseWidth
  | ChangeSlopeAngle
  | ChangeLayerPhiPrime
  | ChangeLayerOedometricModulus
  | InsertLayer
  | RemoveLayer
  | InsertFooting
  | RemoveFooting
  | InsertPile
  | RemovePile
  | InsertRetainingWall
  | RemoveRetainingWall
  | InsertSlope
  | RemoveSlope
  | InsertUpliftCase
  | RemoveUpliftCase
  | InsertAnchor
  | RemoveAnchor
  | InsertEmbeddedWall
  | RemoveEmbeddedWall
  | InsertPileGroup
  | RemovePileGroup
  | InsertLabTest
  | RemoveLabTest
  | InsertLayerWater
  | InsertCharacteristicBasis
  | InsertGroundInvestigation
  | RemoveLayerWater
  | RemoveCharacteristicBasis
  | RemoveGroundInvestigation
  | ChangeLayerSoilType
  | ChangeLayerDepthTop
  | ChangeLayerDepthBottom
  | ChangeLayerGamma
  | ChangeLayerGammaPrime
  | ChangeLayerCohesionEffective
  | ChangeLayerCohesionUndrained
  | ChangeLayerPoissonRatio
  | ChangeLayerCptQc
  | ChangeLayerSptN
  | ChangeFootingLength
  | ChangeFootingBaseInclDeg
  | ChangeFootingSettlementLimit
  | ChangeLoadCaseDesignSituation
  | ChangeLoadCaseVerticalPerm
  | ChangeLoadCaseVerticalVar
  | ChangeLoadCaseHorizPermanent
  | ChangeLoadCaseHorizVariable
  | ChangeLoadCaseMomentPermanent
  | ChangeLoadCaseMomentVariable
  | ChangePilePileType
  | ChangePileDiameter
  | ChangePileAlphaS
  | ChangePileUnitShaftResistance
  | ChangePileUnitBaseResistance
  | ChangePileCompressionPermanent
  | ChangePileCompressionVariable
  | ChangePileTensionPermanent
  | ChangePileTensionVariable
  | ChangeProfileShaftResistance
  | ChangeProfileBaseResistance
  | ChangeWallHeight
  | ChangeWallEmbedment
  | ChangeWallStemThickness
  | ChangeWallBackfillPhiDeg
  | ChangeWallBackfillGamma
  | ChangeWallWallFrictionDeg
  | ChangeWallEarthPressureMode
  | ChangeWallWallMovement
  | ChangeWallOcr
  | ChangeWallConcreteGamma
  | ChangeWallSurcharge
  | ChangeWallVerticalPermanent
  | ChangeWallHorizontalPermanent
  | ChangeSlopeHeight
  | ChangeSlopeLength
  | ChangeSlopeGoverningLayerId
  | ChangeSlopeCircleCentreX
  | ChangeSlopeCircleCentreZ
  | ChangeSlopeCircleRadius
  | ChangeUpliftPermStabilizing
  | ChangeUpliftPermDestabilizing
  | ChangeUpliftVarDestabilizing
  | ChangeUpliftPorePressure
  | ChangeUpliftTotalStress
  | ChangeAnchorWallId
  | ChangeAnchorTendon
  | ChangeAnchorHeadDepth
  | ChangeAnchorInclinationDeg
  | ChangeAnchorFreeLength
  | ChangeAnchorFixedLength
  | ChangeAnchorFixedDiameter
  | ChangeAnchorBondStress
  | ChangeAnchorDesignForce
  | ChangeAnchorTendonYieldForce
  | ChangeAnchorMinFreeLength
  | ChangeAnchorTestFactor
  | ChangeAnchorTestPullOutRes
  | ChangeEmbWallKind
  | ChangeEmbWallRetainedHeight
  | ChangeEmbWallEmbedment
  | ChangeEmbWallPropDepth
  | ChangeEmbWallThickness
  | ChangeEmbWallBackfillGamma
  | ChangeEmbWallPhiDeg
  | ChangeEmbWallDeltaActiveDeg
  | ChangeEmbWallDeltaPassiveDeg
  | ChangeEmbWallSurcharge
  | ChangeEmbWallUndrStrength
  | ChangePileGroupPileId
  | ChangePileGroupRows
  | ChangePileGroupColumns
  | ChangePileGroupSpacing
  | ChangePileGroupCapVertical
  | ChangePileGroupCapMomentX
  | ChangePileGroupCapMomentY
  | ChangePileGroupSingleRes
  | ChangePileGroupShaftCohSum
  | ChangePileGroupBaseCohesion
  | ChangeLabTestLayerId
  | ChangeLabTestKind
  | ChangeLabTestVoidRatio
  | ChangeLabTestPreloadStress
  | ChangeLabTestDrainageHeight
  | ChangeLabTestConsolTime
  | ChangeLabPointNormalStress
  | ChangeLabPointShearStress
  | ChangeWaterLayerId
  | ChangeWaterPiezometricDepth
  | ChangeWaterPermeability
  | ChangeWaterHeadDifference
  | ChangeWaterFlowLength
  | ChangeStructureId
  | InsertLoadCase
  | RemoveLoadCase
  | InsertTestProfile
  | RemoveTestProfile
  | InsertAnchorTest
  | RemoveAnchorTest
  | InsertLabPoint
  | InsertGroundInvestigationReading
  | RemoveLabPoint
  | RemoveGroundInvestigationReading
  | ChangePlacementX
  | ChangePlacementY
  | ChangePlacementZ
  | ChangePlacementYaw
  | InsertPlacement
  | RemovePlacement
  | InsertCase
  | RemoveCase
  | ChangeCaseNumber
  | ChangeCaseText
  | ChangeCaseLabelEn
  | ChangeCaseLabelDe
  | ChangeCaseEntryId
  | ChangeSlopeWidth
  | ChangePileBendingStiffnessEi
  | ChangePileHorizBedModulus
  | ChangePileLateralHeadLoad
  | ChangeWallSeismicKh
  | ChangeWallSeismicKv
  | ChangePileElasticModulus
  | ChangePileSettlingThickness
  | ChangePileNegFrictionBeta
  | ChangePileDynamicTestCount
  | ChangePileDynamicResMean
  | ChangePileDynamicResMin
  | ChangePileHammerWeight
  | ChangePileDropHeight
  | ChangePileSetPerBlow
  | ChangePileElasticCompression
  | ChangePileHammerEfficiency
  | ChangeWallReinforcementSpacing
  | ChangeWallReinforcementWidth
  | ChangeWallRebarFriction
  | ChangeLabTestCompressionIndex
  | ChangeLabTestSwellingIndex
  | ChangeLabTestStressIncrement
  | ChangeLabTestSecComprIndex
  | ChangeLabTestDesignLife
  | ChangeWaterWellRadius
  | ChangeWaterWellWaterDepth
  | ChangeWaterInfluenceRadius
  | ChangeWaterWellCapacity
  | ChangePileConcreteClass
  | ChangeGroundInvestigationPositionX
  | ChangeGroundInvestigationPositionY
  | ChangeGroundInvestigationLevel
  | ChangeGroundInvestigationDepth
  | ChangeGroundInvestigationKind
  | ChangeGroundReadingDepth
  | ChangeGroundReadingMeasurement
  | ChangeCharacteristicBasisLayerId
  | ChangeCharacteristicBasisParameter
  | ChangeCharacteristicBasisBasis;

export const parseEn1997Mutation: NormWireReader<En1997Mutation> = normWireTagged<En1997Mutation, "mutation">("mutation", {
  changeAnnex: parseChangeAnnex,
  changeGeotechnicalCategory: parseChangeGeotechnicalCategory,
  changeDesignSituation: parseChangeDesignSituation,
  changeDesignApproach: parseChangeDesignApproach,
  changeGroundwaterLevel: parseChangeGroundwaterLevel,
  changeInvestigationDepth: parseChangeInvestigationDepth,
  changeFootingWidth: parseChangeFootingWidth,
  changeFootingEmbedment: parseChangeFootingEmbedment,
  changePileLength: parseChangePileLength,
  changePileCount: parseChangePileCount,
  changeWallBaseWidth: parseChangeWallBaseWidth,
  changeSlopeAngle: parseChangeSlopeAngle,
  changeLayerPhiPrime: parseChangeLayerPhiPrime,
  changeLayerOedometricModulus: parseChangeLayerOedometricModulus,
  insertLayer: parseInsertLayer,
  removeLayer: parseRemoveLayer,
  insertFooting: parseInsertFooting,
  removeFooting: parseRemoveFooting,
  insertPile: parseInsertPile,
  removePile: parseRemovePile,
  insertRetainingWall: parseInsertRetainingWall,
  removeRetainingWall: parseRemoveRetainingWall,
  insertSlope: parseInsertSlope,
  removeSlope: parseRemoveSlope,
  insertUpliftCase: parseInsertUpliftCase,
  removeUpliftCase: parseRemoveUpliftCase,
  insertAnchor: parseInsertAnchor,
  removeAnchor: parseRemoveAnchor,
  insertEmbeddedWall: parseInsertEmbeddedWall,
  removeEmbeddedWall: parseRemoveEmbeddedWall,
  insertPileGroup: parseInsertPileGroup,
  removePileGroup: parseRemovePileGroup,
  insertLabTest: parseInsertLabTest,
  removeLabTest: parseRemoveLabTest,
  insertLayerWater: parseInsertLayerWater,
  insertCharacteristicBasis: parseInsertCharacteristicBasis,
  insertGroundInvestigation: parseInsertGroundInvestigation,
  removeLayerWater: parseRemoveLayerWater,
  removeCharacteristicBasis: parseRemoveCharacteristicBasis,
  removeGroundInvestigation: parseRemoveGroundInvestigation,
  changeLayerSoilType: parseChangeLayerSoilType,
  changeLayerDepthTop: parseChangeLayerDepthTop,
  changeLayerDepthBottom: parseChangeLayerDepthBottom,
  changeLayerGamma: parseChangeLayerGamma,
  changeLayerGammaPrime: parseChangeLayerGammaPrime,
  changeLayerCohesionEffective: parseChangeLayerCohesionEffective,
  changeLayerCohesionUndrained: parseChangeLayerCohesionUndrained,
  changeLayerPoissonRatio: parseChangeLayerPoissonRatio,
  changeLayerCptQc: parseChangeLayerCptQc,
  changeLayerSptN: parseChangeLayerSptN,
  changeFootingLength: parseChangeFootingLength,
  changeFootingBaseInclDeg: parseChangeFootingBaseInclDeg,
  changeFootingSettlementLimit: parseChangeFootingSettlementLimit,
  changeLoadCaseDesignSituation: parseChangeLoadCaseDesignSituation,
  changeLoadCaseVerticalPerm: parseChangeLoadCaseVerticalPerm,
  changeLoadCaseVerticalVar: parseChangeLoadCaseVerticalVar,
  changeLoadCaseHorizPermanent: parseChangeLoadCaseHorizPermanent,
  changeLoadCaseHorizVariable: parseChangeLoadCaseHorizVariable,
  changeLoadCaseMomentPermanent: parseChangeLoadCaseMomentPermanent,
  changeLoadCaseMomentVariable: parseChangeLoadCaseMomentVariable,
  changePilePileType: parseChangePilePileType,
  changePileDiameter: parseChangePileDiameter,
  changePileAlphaS: parseChangePileAlphaS,
  changePileUnitShaftResistance: parseChangePileUnitShaftResistance,
  changePileUnitBaseResistance: parseChangePileUnitBaseResistance,
  changePileCompressionPermanent: parseChangePileCompressionPermanent,
  changePileCompressionVariable: parseChangePileCompressionVariable,
  changePileTensionPermanent: parseChangePileTensionPermanent,
  changePileTensionVariable: parseChangePileTensionVariable,
  changeProfileShaftResistance: parseChangeProfileShaftResistance,
  changeProfileBaseResistance: parseChangeProfileBaseResistance,
  changeWallHeight: parseChangeWallHeight,
  changeWallEmbedment: parseChangeWallEmbedment,
  changeWallStemThickness: parseChangeWallStemThickness,
  changeWallBackfillPhiDeg: parseChangeWallBackfillPhiDeg,
  changeWallBackfillGamma: parseChangeWallBackfillGamma,
  changeWallWallFrictionDeg: parseChangeWallWallFrictionDeg,
  changeWallEarthPressureMode: parseChangeWallEarthPressureMode,
  changeWallWallMovement: parseChangeWallWallMovement,
  changeWallOcr: parseChangeWallOcr,
  changeWallConcreteGamma: parseChangeWallConcreteGamma,
  changeWallSurcharge: parseChangeWallSurcharge,
  changeWallVerticalPermanent: parseChangeWallVerticalPermanent,
  changeWallHorizontalPermanent: parseChangeWallHorizontalPermanent,
  changeSlopeHeight: parseChangeSlopeHeight,
  changeSlopeLength: parseChangeSlopeLength,
  changeSlopeGoverningLayerId: parseChangeSlopeGoverningLayerId,
  changeSlopeCircleCentreX: parseChangeSlopeCircleCentreX,
  changeSlopeCircleCentreZ: parseChangeSlopeCircleCentreZ,
  changeSlopeCircleRadius: parseChangeSlopeCircleRadius,
  changeUpliftPermStabilizing: parseChangeUpliftPermStabilizing,
  changeUpliftPermDestabilizing: parseChangeUpliftPermDestabilizing,
  changeUpliftVarDestabilizing: parseChangeUpliftVarDestabilizing,
  changeUpliftPorePressure: parseChangeUpliftPorePressure,
  changeUpliftTotalStress: parseChangeUpliftTotalStress,
  changeAnchorWallId: parseChangeAnchorWallId,
  changeAnchorTendon: parseChangeAnchorTendon,
  changeAnchorHeadDepth: parseChangeAnchorHeadDepth,
  changeAnchorInclinationDeg: parseChangeAnchorInclinationDeg,
  changeAnchorFreeLength: parseChangeAnchorFreeLength,
  changeAnchorFixedLength: parseChangeAnchorFixedLength,
  changeAnchorFixedDiameter: parseChangeAnchorFixedDiameter,
  changeAnchorBondStress: parseChangeAnchorBondStress,
  changeAnchorDesignForce: parseChangeAnchorDesignForce,
  changeAnchorTendonYieldForce: parseChangeAnchorTendonYieldForce,
  changeAnchorMinFreeLength: parseChangeAnchorMinFreeLength,
  changeAnchorTestFactor: parseChangeAnchorTestFactor,
  changeAnchorTestPullOutRes: parseChangeAnchorTestPullOutRes,
  changeEmbWallKind: parseChangeEmbWallKind,
  changeEmbWallRetainedHeight: parseChangeEmbWallRetainedHeight,
  changeEmbWallEmbedment: parseChangeEmbWallEmbedment,
  changeEmbWallPropDepth: parseChangeEmbWallPropDepth,
  changeEmbWallThickness: parseChangeEmbWallThickness,
  changeEmbWallBackfillGamma: parseChangeEmbWallBackfillGamma,
  changeEmbWallPhiDeg: parseChangeEmbWallPhiDeg,
  changeEmbWallDeltaActiveDeg: parseChangeEmbWallDeltaActiveDeg,
  changeEmbWallDeltaPassiveDeg: parseChangeEmbWallDeltaPassiveDeg,
  changeEmbWallSurcharge: parseChangeEmbWallSurcharge,
  changeEmbWallUndrStrength: parseChangeEmbWallUndrStrength,
  changePileGroupPileId: parseChangePileGroupPileId,
  changePileGroupRows: parseChangePileGroupRows,
  changePileGroupColumns: parseChangePileGroupColumns,
  changePileGroupSpacing: parseChangePileGroupSpacing,
  changePileGroupCapVertical: parseChangePileGroupCapVertical,
  changePileGroupCapMomentX: parseChangePileGroupCapMomentX,
  changePileGroupCapMomentY: parseChangePileGroupCapMomentY,
  changePileGroupSingleRes: parseChangePileGroupSingleRes,
  changePileGroupShaftCohSum: parseChangePileGroupShaftCohSum,
  changePileGroupBaseCohesion: parseChangePileGroupBaseCohesion,
  changeLabTestLayerId: parseChangeLabTestLayerId,
  changeLabTestKind: parseChangeLabTestKind,
  changeLabTestVoidRatio: parseChangeLabTestVoidRatio,
  changeLabTestPreloadStress: parseChangeLabTestPreloadStress,
  changeLabTestDrainageHeight: parseChangeLabTestDrainageHeight,
  changeLabTestConsolTime: parseChangeLabTestConsolTime,
  changeLabPointNormalStress: parseChangeLabPointNormalStress,
  changeLabPointShearStress: parseChangeLabPointShearStress,
  changeWaterLayerId: parseChangeWaterLayerId,
  changeWaterPiezometricDepth: parseChangeWaterPiezometricDepth,
  changeWaterPermeability: parseChangeWaterPermeability,
  changeWaterHeadDifference: parseChangeWaterHeadDifference,
  changeWaterFlowLength: parseChangeWaterFlowLength,
  changeStructureId: parseChangeStructureId,
  insertLoadCase: parseInsertLoadCase,
  removeLoadCase: parseRemoveLoadCase,
  insertTestProfile: parseInsertTestProfile,
  removeTestProfile: parseRemoveTestProfile,
  insertAnchorTest: parseInsertAnchorTest,
  removeAnchorTest: parseRemoveAnchorTest,
  insertLabPoint: parseInsertLabPoint,
  insertGroundInvestigationReading: parseInsertGroundInvestigationReading,
  removeLabPoint: parseRemoveLabPoint,
  removeGroundInvestigationReading: parseRemoveGroundInvestigationReading,
  changePlacementX: parseChangePlacementX,
  changePlacementY: parseChangePlacementY,
  changePlacementZ: parseChangePlacementZ,
  changePlacementYaw: parseChangePlacementYaw,
  insertPlacement: parseInsertPlacement,
  removePlacement: parseRemovePlacement,
  insertCase: parseInsertCase,
  removeCase: parseRemoveCase,
  changeCaseNumber: parseChangeCaseNumber,
  changeCaseText: parseChangeCaseText,
  changeCaseLabelEn: parseChangeCaseLabelEn,
  changeCaseLabelDe: parseChangeCaseLabelDe,
  changeCaseEntryId: parseChangeCaseEntryId,
  changeSlopeWidth: parseChangeSlopeWidth,
  changePileBendingStiffnessEi: parseChangePileBendingStiffnessEi,
  changePileHorizBedModulus: parseChangePileHorizBedModulus,
  changePileLateralHeadLoad: parseChangePileLateralHeadLoad,
  changeWallSeismicKh: parseChangeWallSeismicKh,
  changeWallSeismicKv: parseChangeWallSeismicKv,
  changePileElasticModulus: parseChangePileElasticModulus,
  changePileSettlingThickness: parseChangePileSettlingThickness,
  changePileNegFrictionBeta: parseChangePileNegFrictionBeta,
  changePileDynamicTestCount: parseChangePileDynamicTestCount,
  changePileDynamicResMean: parseChangePileDynamicResMean,
  changePileDynamicResMin: parseChangePileDynamicResMin,
  changePileHammerWeight: parseChangePileHammerWeight,
  changePileDropHeight: parseChangePileDropHeight,
  changePileSetPerBlow: parseChangePileSetPerBlow,
  changePileElasticCompression: parseChangePileElasticCompression,
  changePileHammerEfficiency: parseChangePileHammerEfficiency,
  changeWallReinforcementSpacing: parseChangeWallReinforcementSpacing,
  changeWallReinforcementWidth: parseChangeWallReinforcementWidth,
  changeWallRebarFriction: parseChangeWallRebarFriction,
  changeLabTestCompressionIndex: parseChangeLabTestCompressionIndex,
  changeLabTestSwellingIndex: parseChangeLabTestSwellingIndex,
  changeLabTestStressIncrement: parseChangeLabTestStressIncrement,
  changeLabTestSecComprIndex: parseChangeLabTestSecComprIndex,
  changeLabTestDesignLife: parseChangeLabTestDesignLife,
  changeWaterWellRadius: parseChangeWaterWellRadius,
  changeWaterWellWaterDepth: parseChangeWaterWellWaterDepth,
  changeWaterInfluenceRadius: parseChangeWaterInfluenceRadius,
  changeWaterWellCapacity: parseChangeWaterWellCapacity,
  changePileConcreteClass: parseChangePileConcreteClass,
  changeGroundInvestigationPositionX: parseChangeGroundInvestigationPositionX,
  changeGroundInvestigationPositionY: parseChangeGroundInvestigationPositionY,
  changeGroundInvestigationLevel: parseChangeGroundInvestigationLevel,
  changeGroundInvestigationDepth: parseChangeGroundInvestigationDepth,
  changeGroundInvestigationKind: parseChangeGroundInvestigationKind,
  changeGroundReadingDepth: parseChangeGroundReadingDepth,
  changeGroundReadingMeasurement: parseChangeGroundReadingMeasurement,
  changeCharacteristicBasisLayerId: parseChangeCharacteristicBasisLayerId,
  changeCharacteristicBasisParameter: parseChangeCharacteristicBasisParameter,
  changeCharacteristicBasisBasis: parseChangeCharacteristicBasisBasis,
});
