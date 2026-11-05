/** 🧺️ `En1998Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireTagged, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ChangeTowerMRdNm, parseChangeTowerMRdNm } from "./↪️change-tower-m-rd-nm/🧬️schema/🟦️.ts";
import { type ChangeStoreyPermanentGkN, parseChangeStoreyPermanentGkN } from "./⚖️change-storey-permanent-gk-n/🧬️schema/🟦️.ts";
import { type ChangeMemberDetailing, parseChangeMemberDetailing } from "./✅️change-member-detailing/🧬️schema/🟦️.ts";
import { type ChangeAccelerogramBuildingId, parseChangeAccelerogramBuildingId } from "./✏️change-accelerogram-building-id/🧬️schema/🟦️.ts";
import { type ChangeAccelerogramDtS, parseChangeAccelerogramDtS } from "./✏️change-accelerogram-dt-s/🧬️schema/🟦️.ts";
import { type ChangeAccelerogramLabel, parseChangeAccelerogramLabel } from "./✏️change-accelerogram-label/🧬️schema/🟦️.ts";
import { type ChangeAccelerogramSamples, parseChangeAccelerogramSamples } from "./✏️change-accelerogram-samples/🧬️schema/🟦️.ts";
import { type ChangeAccelerogramScaleFactor, parseChangeAccelerogramScaleFactor } from "./✏️change-accelerogram-scale-factor/🧬️schema/🟦️.ts";
import { type ChangeAssessmentAnalysisMethod, parseChangeAssessmentAnalysisMethod } from "./✏️change-assessment-analysis-method/🧬️schema/🟦️.ts";
import { type ChangeAssessmentAs1AreaM2, parseChangeAssessmentAs1AreaM2 } from "./✏️change-assessment-as1-area-m2/🧬️schema/🟦️.ts";
import { type ChangeAssessmentAs2AreaM2, parseChangeAssessmentAs2AreaM2 } from "./✏️change-assessment-as2-area-m2/🧬️schema/🟦️.ts";
import { type ChangeAssessmentAxialForceN, parseChangeAssessmentAxialForceN } from "./✏️change-assessment-axial-force-n/🧬️schema/🟦️.ts";
import { type ChangeAssessmentBarDiameterMm, parseChangeAssessmentBarDiameterMm } from "./✏️change-assessment-bar-diameter-mm/🧬️schema/🟦️.ts";
import { type ChangeAssessmentChordDemandRad, parseChangeAssessmentChordDemandRad } from "./✏️change-assessment-chord-demand-rad/🧬️schema/🟦️.ts";
import { type ChangeAssessmentChordUltRad, parseChangeAssessmentChordUltRad } from "./✏️change-assessment-chord-ult-rad/🧬️schema/🟦️.ts";
import { type ChangeAssessmentChordYieldRad, parseChangeAssessmentChordYieldRad } from "./✏️change-assessment-chord-yield-rad/🧬️schema/🟦️.ts";
import { type ChangeAssessmentCoreDepthM, parseChangeAssessmentCoreDepthM } from "./✏️change-assessment-core-depth-m/🧬️schema/🟦️.ts";
import { type ChangeAssessmentCoreWidthM, parseChangeAssessmentCoreWidthM } from "./✏️change-assessment-core-width-m/🧬️schema/🟦️.ts";
import { type ChangeAssessmentDiagReinfRatio, parseChangeAssessmentDiagReinfRatio } from "./✏️change-assessment-diag-reinf-ratio/🧬️schema/🟦️.ts";
import { type ChangeAssessmentEffDepthM, parseChangeAssessmentEffDepthM } from "./✏️change-assessment-eff-depth-m/🧬️schema/🟦️.ts";
import { type ChangeAssessmentFcMpa, parseChangeAssessmentFcMpa } from "./✏️change-assessment-fc-mpa/🧬️schema/🟦️.ts";
import { type ChangeAssessmentGammaEl, parseChangeAssessmentGammaEl } from "./✏️change-assessment-gamma-el/🧬️schema/🟦️.ts";
import { type ChangeAssessmentKnowledgeLevel, parseChangeAssessmentKnowledgeLevel } from "./✏️change-assessment-knowledge-level/🧬️schema/🟦️.ts";
import { type ChangeAssessmentLimitState, parseChangeAssessmentLimitState } from "./✏️change-assessment-limit-state/🧬️schema/🟦️.ts";
import { type ChangeAssessmentMidSteelAreaM2, parseChangeAssessmentMidSteelAreaM2 } from "./✏️change-assessment-mid-steel-area-m2/🧬️schema/🟦️.ts";
import { type ChangeAssessmentRbarCount, parseChangeAssessmentRbarCount } from "./✏️change-assessment-rbar-count/🧬️schema/🟦️.ts";
import { type ChangeAssessmentRbarSpacingM, parseChangeAssessmentRbarSpacingM } from "./✏️change-assessment-rbar-spacing-m/🧬️schema/🟦️.ts";
import { type ChangeAssessmentSectionDepthM, parseChangeAssessmentSectionDepthM } from "./✏️change-assessment-section-depth-m/🧬️schema/🟦️.ts";
import { type ChangeAssessmentSectionWidthM, parseChangeAssessmentSectionWidthM } from "./✏️change-assessment-section-width-m/🧬️schema/🟦️.ts";
import { type ChangeAssessmentShearCrack, parseChangeAssessmentShearCrack } from "./✏️change-assessment-shear-crack/🧬️schema/🟦️.ts";
import { type ChangeAssessmentShearForceDemandN, parseChangeAssessmentShearForceDemandN } from "./✏️change-assessment-shear-force-demand-n/🧬️schema/🟦️.ts";
import { type ChangeAssessmentShearSpanM, parseChangeAssessmentShearSpanM } from "./✏️change-assessment-shear-span-m/🧬️schema/🟦️.ts";
import { type ChangeAssessmentSteelYieldMpa, parseChangeAssessmentSteelYieldMpa } from "./✏️change-assessment-steel-yield-mpa/🧬️schema/🟦️.ts";
import { type ChangeAssessmentStrpLegAreaM2, parseChangeAssessmentStrpLegAreaM2 } from "./✏️change-assessment-strp-leg-area-m2/🧬️schema/🟦️.ts";
import { type ChangeAssessmentStrpSpacingM, parseChangeAssessmentStrpSpacingM } from "./✏️change-assessment-strp-spacing-m/🧬️schema/🟦️.ts";
import { type ChangeAssessmentStrpYieldMpa, parseChangeAssessmentStrpYieldMpa } from "./✏️change-assessment-strp-yield-mpa/🧬️schema/🟦️.ts";
import { type ChangeAssessmentSupportedBuildingId, parseChangeAssessmentSupportedBuildingId } from "./✏️change-assessment-supported-building-id/🧬️schema/🟦️.ts";
import { type ChangeBearingBridgeId, parseChangeBearingBridgeId } from "./✏️change-bearing-bridge-id/🧬️schema/🟦️.ts";
import { type ChangeBearingDRdM, parseChangeBearingDRdM } from "./✏️change-bearing-d-rd-m/🧬️schema/🟦️.ts";
import { type ChangeBearingDevice, parseChangeBearingDevice } from "./✏️change-bearing-device/🧬️schema/🟦️.ts";
import { type ChangeBearingMinSupportM, parseChangeBearingMinSupportM } from "./✏️change-bearing-min-support-m/🧬️schema/🟦️.ts";
import { type ChangeBearingPierId, parseChangeBearingPierId } from "./✏️change-bearing-pier-id/🧬️schema/🟦️.ts";
import { type ChangeBearingSeatLengthM, parseChangeBearingSeatLengthM } from "./✏️change-bearing-seat-length-m/🧬️schema/🟦️.ts";
import { type ChangeBearingStiffnessNPerM, parseChangeBearingStiffnessNPerM } from "./✏️change-bearing-stiffness-n-per-m/🧬️schema/🟦️.ts";
import { type ChangeBearingXM, parseChangeBearingXM } from "./✏️change-bearing-xm/🧬️schema/🟦️.ts";
import { type ChangeBridgeBearingDRdM, parseChangeBridgeBearingDRdM } from "./✏️change-bridge-bearing-d-rd-m/🧬️schema/🟦️.ts";
import { type ChangeBridgeCorrelatedOccupancy, parseChangeBridgeCorrelatedOccupancy } from "./✏️change-bridge-correlated-occupancy/🧬️schema/🟦️.ts";
import { type ChangeBridgeDeckEiNm2, parseChangeBridgeDeckEiNm2 } from "./✏️change-bridge-deck-ei-nm2/🧬️schema/🟦️.ts";
import { type ChangeBridgeDeckLengthM, parseChangeBridgeDeckLengthM } from "./✏️change-bridge-deck-length-m/🧬️schema/🟦️.ts";
import { type ChangeBridgeFundamentalPeriodS, parseChangeBridgeFundamentalPeriodS } from "./✏️change-bridge-fundamental-period-s/🧬️schema/🟦️.ts";
import { type ChangeBridgeImportanceClass, parseChangeBridgeImportanceClass } from "./✏️change-bridge-importance-class/🧬️schema/🟦️.ts";
import { type ChangeBridgePeriodRatio, parseChangeBridgePeriodRatio } from "./✏️change-bridge-period-ratio/🧬️schema/🟦️.ts";
import { type ChangeBridgePermanentGkN, parseChangeBridgePermanentGkN } from "./✏️change-bridge-permanent-gk-n/🧬️schema/🟦️.ts";
import { type ChangeBridgePierCrackedInertiaRatio, parseChangeBridgePierCrackedInertiaRatio } from "./✏️change-bridge-pier-cracked-inertia-ratio/🧬️schema/🟦️.ts";
import { type ChangeBridgePierHeightM, parseChangeBridgePierHeightM } from "./✏️change-bridge-pier-height-m/🧬️schema/🟦️.ts";
import { type ChangeBridgePierMomentRdNm, parseChangeBridgePierMomentRdNm } from "./✏️change-bridge-pier-moment-rd-nm/🧬️schema/🟦️.ts";
import { type ChangeBridgePierShearRdN, parseChangeBridgePierShearRdN } from "./✏️change-bridge-pier-shear-rd-n/🧬️schema/🟦️.ts";
import { type ChangeBridgeVariableCategory, parseChangeBridgeVariableCategory } from "./✏️change-bridge-variable-category/🧬️schema/🟦️.ts";
import { type ChangeBridgeVariableQkN, parseChangeBridgeVariableQkN } from "./✏️change-bridge-variable-qk-n/🧬️schema/🟦️.ts";
import { type ChangeBridgeVerticalPeriodS, parseChangeBridgeVerticalPeriodS } from "./✏️change-bridge-vertical-period-s/🧬️schema/🟦️.ts";
import { type ChangeBuildingAccidentalEccRatio, parseChangeBuildingAccidentalEccRatio } from "./✏️change-building-accidental-ecc-ratio/🧬️schema/🟦️.ts";
import { type ChangeBuildingBeamMomentSumNm, parseChangeBuildingBeamMomentSumNm } from "./✏️change-building-beam-moment-sum-nm/🧬️schema/🟦️.ts";
import { type ChangeBuildingClaimsSimpleMasonry, parseChangeBuildingClaimsSimpleMasonry } from "./✏️change-building-claims-simple-masonry/🧬️schema/🟦️.ts";
import { type ChangeBuildingColMomentSumNm, parseChangeBuildingColMomentSumNm } from "./✏️change-building-col-moment-sum-nm/🧬️schema/🟦️.ts";
import { type ChangeBuildingCt, parseChangeBuildingCt } from "./✏️change-building-ct/🧬️schema/🟦️.ts";
import { type ChangeBuildingDriftLimitClass, parseChangeBuildingDriftLimitClass } from "./✏️change-building-drift-limit-class/🧬️schema/🟦️.ts";
import { type ChangeBuildingJointGapM, parseChangeBuildingJointGapM } from "./✏️change-building-joint-gap-m/🧬️schema/🟦️.ts";
import { type ChangeBuildingMultiResistingSystems, parseChangeBuildingMultiResistingSystems } from "./✏️change-building-multi-resisting-systems/🧬️schema/🟦️.ts";
import { type ChangeBuildingName, parseChangeBuildingName } from "./✏️change-building-name/🧬️schema/🟦️.ts";
import { type ChangeBuildingNsHeightM, parseChangeBuildingNsHeightM } from "./✏️change-building-ns-height-m/🧬️schema/🟦️.ts";
import { type ChangeBuildingNsPeriodS, parseChangeBuildingNsPeriodS } from "./✏️change-building-ns-period-s/🧬️schema/🟦️.ts";
import { type ChangeBuildingNsQ, parseChangeBuildingNsQ } from "./✏️change-building-ns-q/🧬️schema/🟦️.ts";
import { type ChangeBuildingNsResistanceN, parseChangeBuildingNsResistanceN } from "./✏️change-building-ns-resistance-n/🧬️schema/🟦️.ts";
import { type ChangeBuildingNsWeightN, parseChangeBuildingNsWeightN } from "./✏️change-building-ns-weight-n/🧬️schema/🟦️.ts";
import { type ChangeBuildingNu, parseChangeBuildingNu } from "./✏️change-building-nu/🧬️schema/🟦️.ts";
import { type ChangeBuildingOtBaseXM, parseChangeBuildingOtBaseXM } from "./✏️change-building-ot-base-xm/🧬️schema/🟦️.ts";
import { type ChangeBuildingOtBaseYM, parseChangeBuildingOtBaseYM } from "./✏️change-building-ot-base-ym/🧬️schema/🟦️.ts";
import { type ChangeBuildingPlanEnvGapRatio, parseChangeBuildingPlanEnvGapRatio } from "./✏️change-building-plan-env-gap-ratio/🧬️schema/🟦️.ts";
import { type ChangeBuildingPlanLengthM, parseChangeBuildingPlanLengthM } from "./✏️change-building-plan-length-m/🧬️schema/🟦️.ts";
import { type ChangeBuildingPlanWidthM, parseChangeBuildingPlanWidthM } from "./✏️change-building-plan-width-m/🧬️schema/🟦️.ts";
import { type ChangeBuildingSetbackRatio, parseChangeBuildingSetbackRatio } from "./✏️change-building-setback-ratio/🧬️schema/🟦️.ts";
import { type ChangeBuildingStoreyStiffnessBasis, parseChangeBuildingStoreyStiffnessBasis } from "./✏️change-building-storey-stiffness-basis/🧬️schema/🟦️.ts";
import { type ChangeBuildingStoreyStiffnessRatio, parseChangeBuildingStoreyStiffnessRatio } from "./✏️change-building-storey-stiffness-ratio/🧬️schema/🟦️.ts";
import { type ChangeBuildingT1GivenS, parseChangeBuildingT1GivenS } from "./✏️change-building-t1-given-s/🧬️schema/🟦️.ts";
import { type ChangeBuildingT1Method, parseChangeBuildingT1Method } from "./✏️change-building-t1-method/🧬️schema/🟦️.ts";
import { type ChangeBuildingTiedownNm, parseChangeBuildingTiedownNm } from "./✏️change-building-tiedown-nm/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmBuildingId, parseChangeDiaphragmBuildingId } from "./✏️change-diaphragm-building-id/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmChordAreaM2, parseChangeDiaphragmChordAreaM2 } from "./✏️change-diaphragm-chord-area-m2/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmChordStrMpa, parseChangeDiaphragmChordStrMpa } from "./✏️change-diaphragm-chord-str-mpa/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmDepthM, parseChangeDiaphragmDepthM } from "./✏️change-diaphragm-depth-m/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmGammaD, parseChangeDiaphragmGammaD } from "./✏️change-diaphragm-gamma-d/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmMaterial, parseChangeDiaphragmMaterial } from "./✏️change-diaphragm-material/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmOpeningWidthM, parseChangeDiaphragmOpeningWidthM } from "./✏️change-diaphragm-opening-width-m/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmShearStrMpa, parseChangeDiaphragmShearStrMpa } from "./✏️change-diaphragm-shear-str-mpa/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmSpanM, parseChangeDiaphragmSpanM } from "./✏️change-diaphragm-span-m/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmStoreyId, parseChangeDiaphragmStoreyId } from "./✏️change-diaphragm-storey-id/🧬️schema/🟦️.ts";
import { type ChangeDiaphragmThicknessM, parseChangeDiaphragmThicknessM } from "./✏️change-diaphragm-thickness-m/🧬️schema/🟦️.ts";
import { type ChangeFoundationAreaM2, parseChangeFoundationAreaM2 } from "./✏️change-foundation-area-m2/🧬️schema/🟦️.ts";
import { type ChangeFoundationBaseFrictionDeg, parseChangeFoundationBaseFrictionDeg } from "./✏️change-foundation-base-friction-deg/🧬️schema/🟦️.ts";
import { type ChangeFoundationBasePorePressurePa, parseChangeFoundationBasePorePressurePa } from "./✏️change-foundation-base-pore-pressure-pa/🧬️schema/🟦️.ts";
import { type ChangeFoundationBearAxialN, parseChangeFoundationBearAxialN } from "./✏️change-foundation-bear-axial-n/🧬️schema/🟦️.ts";
import { type ChangeFoundationBearMomentNm, parseChangeFoundationBearMomentNm } from "./✏️change-foundation-bear-moment-nm/🧬️schema/🟦️.ts";
import { type ChangeFoundationBearShearN, parseChangeFoundationBearShearN } from "./✏️change-foundation-bear-shear-n/🧬️schema/🟦️.ts";
import { type ChangeFoundationBearingNMaxN, parseChangeFoundationBearingNMaxN } from "./✏️change-foundation-bearing-n-max-n/🧬️schema/🟦️.ts";
import { type ChangeFoundationFootingWidthM, parseChangeFoundationFootingWidthM } from "./✏️change-foundation-footing-width-m/🧬️schema/🟦️.ts";
import { type ChangeFoundationKFoundation, parseChangeFoundationKFoundation } from "./✏️change-foundation-k-foundation/🧬️schema/🟦️.ts";
import { type ChangeFoundationKSoil, parseChangeFoundationKSoil } from "./✏️change-foundation-k-soil/🧬️schema/🟦️.ts";
import { type ChangeFoundationLiqDepthM, parseChangeFoundationLiqDepthM } from "./✏️change-foundation-liq-depth-m/🧬️schema/🟦️.ts";
import { type ChangeFoundationLiqFinesPercent, parseChangeFoundationLiqFinesPercent } from "./✏️change-foundation-liq-fines-percent/🧬️schema/🟦️.ts";
import { type ChangeFoundationLiqLayerM, parseChangeFoundationLiqLayerM } from "./✏️change-foundation-liq-layer-m/🧬️schema/🟦️.ts";
import { type ChangeFoundationLiqMagnitude, parseChangeFoundationLiqMagnitude } from "./✏️change-foundation-liq-magnitude/🧬️schema/🟦️.ts";
import { type ChangeFoundationLiqN160, parseChangeFoundationLiqN160 } from "./✏️change-foundation-liq-n160/🧬️schema/🟦️.ts";
import { type ChangeFoundationLiqUnitWtNM3, parseChangeFoundationLiqUnitWtNM3 } from "./✏️change-foundation-liq-unit-wt-nm3/🧬️schema/🟦️.ts";
import { type ChangeFoundationLiqWaterDepthM, parseChangeFoundationLiqWaterDepthM } from "./✏️change-foundation-liq-water-depth-m/🧬️schema/🟦️.ts";
import { type ChangeFoundationPRdPa, parseChangeFoundationPRdPa } from "./✏️change-foundation-p-rd-pa/🧬️schema/🟦️.ts";
import { type ChangeFoundationPassiveResistanceN, parseChangeFoundationPassiveResistanceN } from "./✏️change-foundation-passive-resistance-n/🧬️schema/🟦️.ts";
import { type ChangeFoundationPileDiameterM, parseChangeFoundationPileDiameterM } from "./✏️change-foundation-pile-diameter-m/🧬️schema/🟦️.ts";
import { type ChangeFoundationPileHeadFixed, parseChangeFoundationPileHeadFixed } from "./✏️change-foundation-pile-head-fixed/🧬️schema/🟦️.ts";
import { type ChangeFoundationPileHeadShearN, parseChangeFoundationPileHeadShearN } from "./✏️change-foundation-pile-head-shear-n/🧬️schema/🟦️.ts";
import { type ChangeFoundationPileIfaceM, parseChangeFoundationPileIfaceM } from "./✏️change-foundation-pile-iface-m/🧬️schema/🟦️.ts";
import { type ChangeFoundationPileKsNM3, parseChangeFoundationPileKsNM3 } from "./✏️change-foundation-pile-ks-nm3/🧬️schema/🟦️.ts";
import { type ChangeFoundationPileLengthM, parseChangeFoundationPileLengthM } from "./✏️change-foundation-pile-length-m/🧬️schema/🟦️.ts";
import { type ChangeFoundationPileLowVsMS, parseChangeFoundationPileLowVsMS } from "./✏️change-foundation-pile-low-vs-ms/🧬️schema/🟦️.ts";
import { type ChangeFoundationPileModulusPa, parseChangeFoundationPileModulusPa } from "./✏️change-foundation-pile-modulus-pa/🧬️schema/🟦️.ts";
import { type ChangeFoundationPileMomentRdNm, parseChangeFoundationPileMomentRdNm } from "./✏️change-foundation-pile-moment-rd-nm/🧬️schema/🟦️.ts";
import { type ChangeFoundationPileUpVsMS, parseChangeFoundationPileUpVsMS } from "./✏️change-foundation-pile-up-vs-ms/🧬️schema/🟦️.ts";
import { type ChangeFoundationSoilPhiDeg, parseChangeFoundationSoilPhiDeg } from "./✏️change-foundation-soil-phi-deg/🧬️schema/🟦️.ts";
import { type ChangeFoundationSupportedBuildingId, parseChangeFoundationSupportedBuildingId } from "./✏️change-foundation-supported-building-id/🧬️schema/🟦️.ts";
import { type ChangeJointBeamId, parseChangeJointBeamId } from "./✏️change-joint-beam-id/🧬️schema/🟦️.ts";
import { type ChangeJointBeamMomentRdNm, parseChangeJointBeamMomentRdNm } from "./✏️change-joint-beam-moment-rd-nm/🧬️schema/🟦️.ts";
import { type ChangeJointBuildingId, parseChangeJointBuildingId } from "./✏️change-joint-building-id/🧬️schema/🟦️.ts";
import { type ChangeJointColMomentRdNm, parseChangeJointColMomentRdNm } from "./✏️change-joint-col-moment-rd-nm/🧬️schema/🟦️.ts";
import { type ChangeJointColumnId, parseChangeJointColumnId } from "./✏️change-joint-column-id/🧬️schema/🟦️.ts";
import { type ChangeJointColumnMomentRdNm, parseChangeJointColumnMomentRdNm } from "./✏️change-joint-column-moment-rd-nm/🧬️schema/🟦️.ts";
import { type ChangeJointStoreyId, parseChangeJointStoreyId } from "./✏️change-joint-storey-id/🧬️schema/🟦️.ts";
import { type ChangeJointXM, parseChangeJointXM } from "./✏️change-joint-xm/🧬️schema/🟦️.ts";
import { type ChangeJointYM, parseChangeJointYM } from "./✏️change-joint-ym/🧬️schema/🟦️.ts";
import { type ChangeMemberEffectiveStiffnessBasis, parseChangeMemberEffectiveStiffnessBasis } from "./✏️change-member-effective-stiffness-basis/🧬️schema/🟦️.ts";
import { type ChangeMemberEffectiveStiffnessRatio, parseChangeMemberEffectiveStiffnessRatio } from "./✏️change-member-effective-stiffness-ratio/🧬️schema/🟦️.ts";
import { type ChangeMemberFastenerDiameterMm, parseChangeMemberFastenerDiameterMm } from "./✏️change-member-fastener-diameter-mm/🧬️schema/🟦️.ts";
import { type ChangeMemberMaterial, parseChangeMemberMaterial } from "./✏️change-member-material/🧬️schema/🟦️.ts";
import { type ChangeMemberMinDimensionM, parseChangeMemberMinDimensionM } from "./✏️change-member-min-dimension-m/🧬️schema/🟦️.ts";
import { type ChangeMemberOmegaWd, parseChangeMemberOmegaWd } from "./✏️change-member-omega-wd/🧬️schema/🟦️.ts";
import { type ChangeMemberRhoPrime, parseChangeMemberRhoPrime } from "./✏️change-member-rho-prime/🧬️schema/🟦️.ts";
import { type ChangeMemberRho, parseChangeMemberRho } from "./✏️change-member-rho/🧬️schema/🟦️.ts";
import { type ChangeMemberRole, parseChangeMemberRole } from "./✏️change-member-role/🧬️schema/🟦️.ts";
import { type ChangeMemberSteelSectionClass, parseChangeMemberSteelSectionClass } from "./✏️change-member-steel-section-class/🧬️schema/🟦️.ts";
import { type ChangeMemberTimberThicknessMm, parseChangeMemberTimberThicknessMm } from "./✏️change-member-timber-thickness-mm/🧬️schema/🟦️.ts";
import { type ChangeMemberWallDirectionRad, parseChangeMemberWallDirectionRad } from "./✏️change-member-wall-direction-rad/🧬️schema/🟦️.ts";
import { type ChangeMemberWallEffHeightM, parseChangeMemberWallEffHeightM } from "./✏️change-member-wall-eff-height-m/🧬️schema/🟦️.ts";
import { type ChangeMemberWallLengthM, parseChangeMemberWallLengthM } from "./✏️change-member-wall-length-m/🧬️schema/🟦️.ts";
import { type ChangeMemberWallPoissonRatio, parseChangeMemberWallPoissonRatio } from "./✏️change-member-wall-poisson-ratio/🧬️schema/🟦️.ts";
import { type ChangeMemberWallStartXM, parseChangeMemberWallStartXM } from "./✏️change-member-wall-start-xm/🧬️schema/🟦️.ts";
import { type ChangeMemberWallStartYM, parseChangeMemberWallStartYM } from "./✏️change-member-wall-start-ym/🧬️schema/🟦️.ts";
import { type ChangeMemberWallStoreyId, parseChangeMemberWallStoreyId } from "./✏️change-member-wall-storey-id/🧬️schema/🟦️.ts";
import { type ChangeMemberWallThicknessM, parseChangeMemberWallThicknessM } from "./✏️change-member-wall-thickness-m/🧬️schema/🟦️.ts";
import { type ChangePierAxialLoadN, parseChangePierAxialLoadN } from "./✏️change-pier-axial-load-n/🧬️schema/🟦️.ts";
import { type ChangePierBridgeId, parseChangePierBridgeId } from "./✏️change-pier-bridge-id/🧬️schema/🟦️.ts";
import { type ChangePierConcreteFckMpa, parseChangePierConcreteFckMpa } from "./✏️change-pier-concrete-fck-mpa/🧬️schema/🟦️.ts";
import { type ChangePierCoverM, parseChangePierCoverM } from "./✏️change-pier-cover-m/🧬️schema/🟦️.ts";
import { type ChangePierHeightM, parseChangePierHeightM } from "./✏️change-pier-height-m/🧬️schema/🟦️.ts";
import { type ChangePierModulusMpa, parseChangePierModulusMpa } from "./✏️change-pier-modulus-mpa/🧬️schema/🟦️.ts";
import { type ChangePierMomentRdNm, parseChangePierMomentRdNm } from "./✏️change-pier-moment-rd-nm/🧬️schema/🟦️.ts";
import { type ChangePierReinforcementRatio, parseChangePierReinforcementRatio } from "./✏️change-pier-reinforcement-ratio/🧬️schema/🟦️.ts";
import { type ChangePierSectionBM, parseChangePierSectionBM } from "./✏️change-pier-section-bm/🧬️schema/🟦️.ts";
import { type ChangePierSectionHM, parseChangePierSectionHM } from "./✏️change-pier-section-hm/🧬️schema/🟦️.ts";
import { type ChangePierShearRdN, parseChangePierShearRdN } from "./✏️change-pier-shear-rd-n/🧬️schema/🟦️.ts";
import { type ChangePierSteelFykMpa, parseChangePierSteelFykMpa } from "./✏️change-pier-steel-fyk-mpa/🧬️schema/🟦️.ts";
import { type ChangePierStiffnessBasis, parseChangePierStiffnessBasis } from "./✏️change-pier-stiffness-basis/🧬️schema/🟦️.ts";
import { type ChangePierXM, parseChangePierXM } from "./✏️change-pier-xm/🧬️schema/🟦️.ts";
import { type ChangeRetainingWallHRdNPerM, parseChangeRetainingWallHRdNPerM } from "./✏️change-retaining-wall-h-rd-n-per-m/🧬️schema/🟦️.ts";
import { type ChangeRetainingWallHeightM, parseChangeRetainingWallHeightM } from "./✏️change-retaining-wall-height-m/🧬️schema/🟦️.ts";
import { type ChangeRetainingWallPermeabilityMS, parseChangeRetainingWallPermeabilityMS } from "./✏️change-retaining-wall-permeability-ms/🧬️schema/🟦️.ts";
import { type ChangeRetainingWallPhiDeg, parseChangeRetainingWallPhiDeg } from "./✏️change-retaining-wall-phi-deg/🧬️schema/🟦️.ts";
import { type ChangeRetainingWallR, parseChangeRetainingWallR } from "./✏️change-retaining-wall-r/🧬️schema/🟦️.ts";
import { type ChangeRetainingWallSaturatedGamma, parseChangeRetainingWallSaturatedGamma } from "./✏️change-retaining-wall-saturated-gamma/🧬️schema/🟦️.ts";
import { type ChangeRetainingWallSoilGamma, parseChangeRetainingWallSoilGamma } from "./✏️change-retaining-wall-soil-gamma/🧬️schema/🟦️.ts";
import { type ChangeRetainingWallWaterHeightM, parseChangeRetainingWallWaterHeightM } from "./✏️change-retaining-wall-water-height-m/🧬️schema/🟦️.ts";
import { type ChangeSeismicSlopeGoverningLayerId, parseChangeSeismicSlopeGoverningLayerId } from "./✏️change-seismic-slope-governing-layer-id/🧬️schema/🟦️.ts";
import { type ChangeSiloContentCategory, parseChangeSiloContentCategory } from "./✏️change-silo-content-category/🧬️schema/🟦️.ts";
import { type ChangeSiloContentQkN, parseChangeSiloContentQkN } from "./✏️change-silo-content-qk-n/🧬️schema/🟦️.ts";
import { type ChangeSiloFillingRatio, parseChangeSiloFillingRatio } from "./✏️change-silo-filling-ratio/🧬️schema/🟦️.ts";
import { type ChangeSiloHeightM, parseChangeSiloHeightM } from "./✏️change-silo-height-m/🧬️schema/🟦️.ts";
import { type ChangeSiloNRdN, parseChangeSiloNRdN } from "./✏️change-silo-n-rd-n/🧬️schema/🟦️.ts";
import { type ChangeSiloPermanentGkN, parseChangeSiloPermanentGkN } from "./✏️change-silo-permanent-gk-n/🧬️schema/🟦️.ts";
import { type ChangeSiloQNominal, parseChangeSiloQNominal } from "./✏️change-silo-q-nominal/🧬️schema/🟦️.ts";
import { type ChangeSiloRadiusM, parseChangeSiloRadiusM } from "./✏️change-silo-radius-m/🧬️schema/🟦️.ts";
import { type ChangeSiloVRdN, parseChangeSiloVRdN } from "./✏️change-silo-v-rd-n/🧬️schema/🟦️.ts";
import { type ChangeSiteAGr, parseChangeSiteAGr } from "./✏️change-site-a-gr/🧬️schema/🟦️.ts";
import { type ChangeSiteAlluviumThicknessM, parseChangeSiteAlluviumThicknessM } from "./✏️change-site-alluvium-thickness-m/🧬️schema/🟦️.ts";
import { type ChangeSiteAlluviumVSMS, parseChangeSiteAlluviumVSMS } from "./✏️change-site-alluvium-vsms/🧬️schema/🟦️.ts";
import { type ChangeSiteBelowAlluviumVSMS, parseChangeSiteBelowAlluviumVSMS } from "./✏️change-site-below-alluvium-vsms/🧬️schema/🟦️.ts";
import { type ChangeSiteCUKpa, parseChangeSiteCUKpa } from "./✏️change-site-cu-kpa/🧬️schema/🟦️.ts";
import { type ChangeSiteDeGroundCombo, parseChangeSiteDeGroundCombo } from "./✏️change-site-de-ground-combo/🧬️schema/🟦️.ts";
import { type ChangeSiteEnGroundType, parseChangeSiteEnGroundType } from "./✏️change-site-en-ground-type/🧬️schema/🟦️.ts";
import { type ChangeSiteEnSpectrumType, parseChangeSiteEnSpectrumType } from "./✏️change-site-en-spectrum-type/🧬️schema/🟦️.ts";
import { type ChangeSiteImportanceClass, parseChangeSiteImportanceClass } from "./✏️change-site-importance-class/🧬️schema/🟦️.ts";
import { type ChangeSiteLiquefiableLayer, parseChangeSiteLiquefiableLayer } from "./✏️change-site-liquefiable-layer/🧬️schema/🟦️.ts";
import { type ChangeSiteNSpt, parseChangeSiteNSpt } from "./✏️change-site-n-spt/🧬️schema/🟦️.ts";
import { type ChangeSiteSApR, parseChangeSiteSApR } from "./✏️change-site-s-ap-r/🧬️schema/🟦️.ts";
import { type ChangeSiteSoftClayPi, parseChangeSiteSoftClayPi } from "./✏️change-site-soft-clay-pi/🧬️schema/🟦️.ts";
import { type ChangeSiteSoftClayThicknessM, parseChangeSiteSoftClayThicknessM } from "./✏️change-site-soft-clay-thickness-m/🧬️schema/🟦️.ts";
import { type ChangeSiteVS30MS, parseChangeSiteVS30MS } from "./✏️change-site-vs30-ms/🧬️schema/🟦️.ts";
import { type ChangeSoilLayerFinesPercent, parseChangeSoilLayerFinesPercent } from "./✏️change-soil-layer-fines-percent/🧬️schema/🟦️.ts";
import { type ChangeSoilLayerFoundationId, parseChangeSoilLayerFoundationId } from "./✏️change-soil-layer-foundation-id/🧬️schema/🟦️.ts";
import { type ChangeSoilLayerN160, parseChangeSoilLayerN160 } from "./✏️change-soil-layer-n160/🧬️schema/🟦️.ts";
import { type ChangeSoilLayerThicknessM, parseChangeSoilLayerThicknessM } from "./✏️change-soil-layer-thickness-m/🧬️schema/🟦️.ts";
import { type ChangeSoilLayerVsMS, parseChangeSoilLayerVsMS } from "./✏️change-soil-layer-vs-ms/🧬️schema/🟦️.ts";
import { type ChangeSoilUnitWtNM3, parseChangeSoilUnitWtNM3 } from "./✏️change-soil-unit-wt-nm3/🧬️schema/🟦️.ts";
import { type ChangeStoreyCentreOfMassXM, parseChangeStoreyCentreOfMassXM } from "./✏️change-storey-centre-of-mass-xm/🧬️schema/🟦️.ts";
import { type ChangeStoreyCentreOfMassYM, parseChangeStoreyCentreOfMassYM } from "./✏️change-storey-centre-of-mass-ym/🧬️schema/🟦️.ts";
import { type ChangeStoreyCentreOfStiffnessXM, parseChangeStoreyCentreOfStiffnessXM } from "./✏️change-storey-centre-of-stiffness-xm/🧬️schema/🟦️.ts";
import { type ChangeStoreyCentreOfStiffnessYM, parseChangeStoreyCentreOfStiffnessYM } from "./✏️change-storey-centre-of-stiffness-ym/🧬️schema/🟦️.ts";
import { type ChangeStoreyCorrelatedOccupancy, parseChangeStoreyCorrelatedOccupancy } from "./✏️change-storey-correlated-occupancy/🧬️schema/🟦️.ts";
import { type ChangeStoreyDriftYM, parseChangeStoreyDriftYM } from "./✏️change-storey-drift-ym/🧬️schema/🟦️.ts";
import { type ChangeStoreyHeightM, parseChangeStoreyHeightM } from "./✏️change-storey-height-m/🧬️schema/🟦️.ts";
import { type ChangeStoreyShearResistanceN, parseChangeStoreyShearResistanceN } from "./✏️change-storey-shear-resistance-n/🧬️schema/🟦️.ts";
import { type ChangeStoreyStiffnessY, parseChangeStoreyStiffnessY } from "./✏️change-storey-stiffness-y/🧬️schema/🟦️.ts";
import { type ChangeStoreyVariableCategory, parseChangeStoreyVariableCategory } from "./✏️change-storey-variable-category/🧬️schema/🟦️.ts";
import { type ChangeStoreyVariableQkN, parseChangeStoreyVariableQkN } from "./✏️change-storey-variable-qk-n/🧬️schema/🟦️.ts";
import { type ChangeSystemAlphaUOverAlpha1, parseChangeSystemAlphaUOverAlpha1 } from "./✏️change-system-alpha-u-over-alpha1/🧬️schema/🟦️.ts";
import { type ChangeSystemDirection, parseChangeSystemDirection } from "./✏️change-system-direction/🧬️schema/🟦️.ts";
import { type ChangeSystemDuctilityClass, parseChangeSystemDuctilityClass } from "./✏️change-system-ductility-class/🧬️schema/🟦️.ts";
import { type ChangeSystemKW, parseChangeSystemKW } from "./✏️change-system-kw/🧬️schema/🟦️.ts";
import { type ChangeSystemMaterial, parseChangeSystemMaterial } from "./✏️change-system-material/🧬️schema/🟦️.ts";
import { type ChangeSystemQ0, parseChangeSystemQ0 } from "./✏️change-system-q0/🧬️schema/🟦️.ts";
import { type ChangeSystemSystemType, parseChangeSystemSystemType } from "./✏️change-system-system-type/🧬️schema/🟦️.ts";
import { type ChangeTankContentCategory, parseChangeTankContentCategory } from "./✏️change-tank-content-category/🧬️schema/🟦️.ts";
import { type ChangeTankContentQkN, parseChangeTankContentQkN } from "./✏️change-tank-content-qk-n/🧬️schema/🟦️.ts";
import { type ChangeTankFillingRatio, parseChangeTankFillingRatio } from "./✏️change-tank-filling-ratio/🧬️schema/🟦️.ts";
import { type ChangeTankHeightM, parseChangeTankHeightM } from "./✏️change-tank-height-m/🧬️schema/🟦️.ts";
import { type ChangeTankPermanentGkN, parseChangeTankPermanentGkN } from "./✏️change-tank-permanent-gk-n/🧬️schema/🟦️.ts";
import { type ChangeTankRadiusM, parseChangeTankRadiusM } from "./✏️change-tank-radius-m/🧬️schema/🟦️.ts";
import { type ChangeTankShellStrMpa, parseChangeTankShellStrMpa } from "./✏️change-tank-shell-str-mpa/🧬️schema/🟦️.ts";
import { type ChangeTankShellTM, parseChangeTankShellTM } from "./✏️change-tank-shell-tm/🧬️schema/🟦️.ts";
import { type ChangeTankVRdN, parseChangeTankVRdN } from "./✏️change-tank-v-rd-n/🧬️schema/🟦️.ts";
import { type ChangeTieBeamFoundationAId, parseChangeTieBeamFoundationAId } from "./✏️change-tie-beam-foundation-a-id/🧬️schema/🟦️.ts";
import { type ChangeTieBeamFoundationBId, parseChangeTieBeamFoundationBId } from "./✏️change-tie-beam-foundation-b-id/🧬️schema/🟦️.ts";
import { type ChangeTowerBaseRotStiffNm, parseChangeTowerBaseRotStiffNm } from "./✏️change-tower-base-rot-stiff-nm/🧬️schema/🟦️.ts";
import { type ChangeTowerCorrelatedOccupancy, parseChangeTowerCorrelatedOccupancy } from "./✏️change-tower-correlated-occupancy/🧬️schema/🟦️.ts";
import { type ChangeTowerEiNm2, parseChangeTowerEiNm2 } from "./✏️change-tower-ei-nm2/🧬️schema/🟦️.ts";
import { type ChangeTowerHeightM, parseChangeTowerHeightM } from "./✏️change-tower-height-m/🧬️schema/🟦️.ts";
import { type ChangeTowerIsChimney, parseChangeTowerIsChimney } from "./✏️change-tower-is-chimney/🧬️schema/🟦️.ts";
import { type ChangeTowerPermanentGkN, parseChangeTowerPermanentGkN } from "./✏️change-tower-permanent-gk-n/🧬️schema/🟦️.ts";
import { type ChangeTowerQNominal, parseChangeTowerQNominal } from "./✏️change-tower-q-nominal/🧬️schema/🟦️.ts";
import { type ChangeTowerVariableCategory, parseChangeTowerVariableCategory } from "./✏️change-tower-variable-category/🧬️schema/🟦️.ts";
import { type ChangeTowerVariableQkN, parseChangeTowerVariableQkN } from "./✏️change-tower-variable-qk-n/🧬️schema/🟦️.ts";
import { type InsertAccelerogram, parseInsertAccelerogram } from "./➕️insert-accelerogram/🧬️schema/🟦️.ts";
import { type InsertBearing, parseInsertBearing } from "./➕️insert-bearing/🧬️schema/🟦️.ts";
import { type InsertBridgeVariable, parseInsertBridgeVariable } from "./➕️insert-bridge-variable/🧬️schema/🟦️.ts";
import { type InsertBuilding, parseInsertBuilding } from "./➕️insert-building/🧬️schema/🟦️.ts";
import { type InsertDiaphragm, parseInsertDiaphragm } from "./➕️insert-diaphragm/🧬️schema/🟦️.ts";
import { type InsertJoint, parseInsertJoint } from "./➕️insert-joint/🧬️schema/🟦️.ts";
import { type InsertMember, parseInsertMember } from "./➕️insert-member/🧬️schema/🟦️.ts";
import { type InsertPier, parseInsertPier } from "./➕️insert-pier/🧬️schema/🟦️.ts";
import { type InsertSeismicSlope, parseInsertSeismicSlope } from "./➕️insert-seismic-slope/🧬️schema/🟦️.ts";
import { type InsertSoilLayer, parseInsertSoilLayer } from "./➕️insert-soil-layer/🧬️schema/🟦️.ts";
import { type InsertStoreyVariable, parseInsertStoreyVariable } from "./➕️insert-storey-variable/🧬️schema/🟦️.ts";
import { type InsertStorey, parseInsertStorey } from "./➕️insert-storey/🧬️schema/🟦️.ts";
import { type InsertSystem, parseInsertSystem } from "./➕️insert-system/🧬️schema/🟦️.ts";
import { type InsertTieBeam, parseInsertTieBeam } from "./➕️insert-tie-beam/🧬️schema/🟦️.ts";
import { type InsertTowerVariable, parseInsertTowerVariable } from "./➕️insert-tower-variable/🧬️schema/🟦️.ts";
import { type RemoveAccelerogram, parseRemoveAccelerogram } from "./➖️remove-accelerogram/🧬️schema/🟦️.ts";
import { type RemoveAssessment, parseRemoveAssessment } from "./➖️remove-assessment/🧬️schema/🟦️.ts";
import { type RemoveBearing, parseRemoveBearing } from "./➖️remove-bearing/🧬️schema/🟦️.ts";
import { type RemoveBridgeVariable, parseRemoveBridgeVariable } from "./➖️remove-bridge-variable/🧬️schema/🟦️.ts";
import { type RemoveBridge, parseRemoveBridge } from "./➖️remove-bridge/🧬️schema/🟦️.ts";
import { type RemoveBuilding, parseRemoveBuilding } from "./➖️remove-building/🧬️schema/🟦️.ts";
import { type RemoveDiaphragm, parseRemoveDiaphragm } from "./➖️remove-diaphragm/🧬️schema/🟦️.ts";
import { type RemoveFoundation, parseRemoveFoundation } from "./➖️remove-foundation/🧬️schema/🟦️.ts";
import { type RemoveJoint, parseRemoveJoint } from "./➖️remove-joint/🧬️schema/🟦️.ts";
import { type RemoveMember, parseRemoveMember } from "./➖️remove-member/🧬️schema/🟦️.ts";
import { type RemovePier, parseRemovePier } from "./➖️remove-pier/🧬️schema/🟦️.ts";
import { type RemoveRetainingWall, parseRemoveRetainingWall } from "./➖️remove-retaining-wall/🧬️schema/🟦️.ts";
import { type RemoveSeismicSlope, parseRemoveSeismicSlope } from "./➖️remove-seismic-slope/🧬️schema/🟦️.ts";
import { type RemoveSilo, parseRemoveSilo } from "./➖️remove-silo/🧬️schema/🟦️.ts";
import { type RemoveSoilLayer, parseRemoveSoilLayer } from "./➖️remove-soil-layer/🧬️schema/🟦️.ts";
import { type RemoveStoreyVariable, parseRemoveStoreyVariable } from "./➖️remove-storey-variable/🧬️schema/🟦️.ts";
import { type RemoveStorey, parseRemoveStorey } from "./➖️remove-storey/🧬️schema/🟦️.ts";
import { type RemoveSystem, parseRemoveSystem } from "./➖️remove-system/🧬️schema/🟦️.ts";
import { type RemoveTank, parseRemoveTank } from "./➖️remove-tank/🧬️schema/🟦️.ts";
import { type RemoveTieBeam, parseRemoveTieBeam } from "./➖️remove-tie-beam/🧬️schema/🟦️.ts";
import { type RemoveTowerVariable, parseRemoveTowerVariable } from "./➖️remove-tower-variable/🧬️schema/🟦️.ts";
import { type RemoveTower, parseRemoveTower } from "./➖️remove-tower/🧬️schema/🟦️.ts";
import { type InsertBridge, parseInsertBridge } from "./🌉insert-bridge/🧬️schema/🟦️.ts";
import { type ChangeAssessmentRKN, parseChangeAssessmentRKN } from "./🏋️change-assessment-rkn/🧬️schema/🟦️.ts";
import { type ChangeSystemVRdN, parseChangeSystemVRdN } from "./💪️change-system-v-rd-n/🧬️schema/🟦️.ts";
import { type ChangePlacementX, parseChangePlacementX } from "./📍️change-placement-x/🧬️schema/🟦️.ts";
import { type ChangePlacementY, parseChangePlacementY } from "./📍️change-placement-y/🧬️schema/🟦️.ts";
import { type ChangePlacementYaw, parseChangePlacementYaw } from "./📍️change-placement-yaw/🧬️schema/🟦️.ts";
import { type ChangePlacementZ, parseChangePlacementZ } from "./📍️change-placement-z/🧬️schema/🟦️.ts";
import { type ChangeSeismicSlopeAngle, parseChangeSeismicSlopeAngle } from "./📍️change-seismic-slope-angle/🧬️schema/🟦️.ts";
import { type ChangeSeismicSlopeCircleCentreX, parseChangeSeismicSlopeCircleCentreX } from "./📍️change-seismic-slope-circle-centre-x/🧬️schema/🟦️.ts";
import { type ChangeSeismicSlopeCircleCentreZ, parseChangeSeismicSlopeCircleCentreZ } from "./📍️change-seismic-slope-circle-centre-z/🧬️schema/🟦️.ts";
import { type ChangeSeismicSlopeCircleRadius, parseChangeSeismicSlopeCircleRadius } from "./📍️change-seismic-slope-circle-radius/🧬️schema/🟦️.ts";
import { type ChangeSeismicSlopeCohesion, parseChangeSeismicSlopeCohesion } from "./📍️change-seismic-slope-cohesion/🧬️schema/🟦️.ts";
import { type ChangeSeismicSlopeFrictionAngle, parseChangeSeismicSlopeFrictionAngle } from "./📍️change-seismic-slope-friction-angle/🧬️schema/🟦️.ts";
import { type ChangeSeismicSlopeHeight, parseChangeSeismicSlopeHeight } from "./📍️change-seismic-slope-height/🧬️schema/🟦️.ts";
import { type ChangeSeismicSlopeTopographicFactor, parseChangeSeismicSlopeTopographicFactor } from "./📍️change-seismic-slope-topographic-factor/🧬️schema/🟦️.ts";
import { type ChangeTieBeamDepth, parseChangeTieBeamDepth } from "./📍️change-tie-beam-depth/🧬️schema/🟦️.ts";
import { type ChangeTieBeamDisplacementLimit, parseChangeTieBeamDisplacementLimit } from "./📍️change-tie-beam-displacement-limit/🧬️schema/🟦️.ts";
import { type ChangeTieBeamRelativeDisplacement, parseChangeTieBeamRelativeDisplacement } from "./📍️change-tie-beam-relative-displacement/🧬️schema/🟦️.ts";
import { type ChangeTieBeamSpan, parseChangeTieBeamSpan } from "./📍️change-tie-beam-span/🧬️schema/🟦️.ts";
import { type ChangeTieBeamSteelArea, parseChangeTieBeamSteelArea } from "./📍️change-tie-beam-steel-area/🧬️schema/🟦️.ts";
import { type ChangeTieBeamSteelDesignStrength, parseChangeTieBeamSteelDesignStrength } from "./📍️change-tie-beam-steel-design-strength/🧬️schema/🟦️.ts";
import { type ChangeTieBeamWidth, parseChangeTieBeamWidth } from "./📍️change-tie-beam-width/🧬️schema/🟦️.ts";
import { type InsertPlacement, parseInsertPlacement } from "./📍️insert-placement/🧬️schema/🟦️.ts";
import { type RemovePlacement, parseRemovePlacement } from "./📍️remove-placement/🧬️schema/🟦️.ts";
import { type ChangeAnnex, parseChangeAnnex } from "./📎️change-annex/🧬️schema/🟦️.ts";
import { type ChangeElevationRegular, parseChangeElevationRegular } from "./📏️change-elevation-regular/🧬️schema/🟦️.ts";
import { type ChangeStoreyDriftXM, parseChangeStoreyDriftXM } from "./📏️change-storey-drift-xm/🧬️schema/🟦️.ts";
import { type ChangeStoreyStiffnessX, parseChangeStoreyStiffnessX } from "./📐️change-storey-stiffness-x/🧬️schema/🟦️.ts";
import { type InsertAssessment, parseInsertAssessment } from "./🔧insert-assessment/🧬️schema/🟦️.ts";
import { type InsertTower, parseInsertTower } from "./🗼insert-tower/🧬️schema/🟦️.ts";
import { type ChangeBridgeVRdN, parseChangeBridgeVRdN } from "./🛑️change-bridge-v-rd-n/🧬️schema/🟦️.ts";
import { type InsertTank, parseInsertTank } from "./🛢️insert-tank/🧬️schema/🟦️.ts";
import { type ChangeBuildingPlanRegular, parseChangeBuildingPlanRegular } from "./🧭️change-building-plan-regular/🧬️schema/🟦️.ts";
import { type ChangeMasonryWallRatio, parseChangeMasonryWallRatio } from "./🧱️change-masonry-wall-ratio/🧬️schema/🟦️.ts";
import { type InsertRetainingWall, parseInsertRetainingWall } from "./🧱️insert-retaining-wall/🧬️schema/🟦️.ts";
import { type InsertFoundation, parseInsertFoundation } from "./🪨insert-foundation/🧬️schema/🟦️.ts";
import { type InsertSilo, parseInsertSilo } from "./🫙insert-silo/🧬️schema/🟦️.ts";

export type En1998Mutation =
  | ChangeAnnex
  | InsertBuilding
  | RemoveBuilding
  | ChangeSystemVRdN
  | ChangeStoreyPermanentGkN
  | ChangeStoreyStiffnessX
  | ChangeStoreyDriftXM
  | ChangeBuildingPlanRegular
  | ChangeElevationRegular
  | ChangeMemberDetailing
  | ChangeMasonryWallRatio
  | InsertBridge
  | ChangeBridgeVRdN
  | InsertAssessment
  | ChangeAssessmentRKN
  | InsertSilo
  | InsertTank
  | InsertFoundation
  | InsertRetainingWall
  | InsertTower
  | ChangeTowerMRdNm
  | RemoveBridge
  | RemoveAssessment
  | RemoveSilo
  | RemoveTank
  | RemoveFoundation
  | RemoveRetainingWall
  | RemoveTower
  | ChangeSiteSApR
  | ChangeSiteAGr
  | ChangeSiteDeGroundCombo
  | ChangeSiteEnGroundType
  | ChangeSiteEnSpectrumType
  | ChangeSiteImportanceClass
  | ChangeBuildingName
  | ChangeBuildingPlanWidthM
  | ChangeBuildingPlanLengthM
  | ChangeBuildingT1Method
  | ChangeBuildingT1GivenS
  | ChangeBuildingCt
  | ChangeBuildingDriftLimitClass
  | ChangeBuildingNu
  | ChangeBuildingMultiResistingSystems
  | ChangeBuildingClaimsSimpleMasonry
  | ChangeBuildingAccidentalEccRatio
  | ChangeSystemDirection
  | ChangeSystemSystemType
  | ChangeSystemMaterial
  | ChangeSystemDuctilityClass
  | ChangeSystemQ0
  | ChangeSystemAlphaUOverAlpha1
  | ChangeSystemKW
  | ChangeStoreyHeightM
  | ChangeStoreyCorrelatedOccupancy
  | ChangeStoreyStiffnessY
  | ChangeStoreyCentreOfMassXM
  | ChangeStoreyCentreOfMassYM
  | ChangeStoreyCentreOfStiffnessXM
  | ChangeStoreyCentreOfStiffnessYM
  | ChangeStoreyDriftYM
  | ChangeStoreyShearResistanceN
  | ChangeMemberMaterial
  | ChangeMemberRole
  | ChangeMemberMinDimensionM
  | ChangeMemberRho
  | ChangeMemberRhoPrime
  | ChangeMemberOmegaWd
  | ChangeMemberSteelSectionClass
  | ChangeBridgePeriodRatio
  | ChangeBridgeFundamentalPeriodS
  | ChangeBridgeBearingDRdM
  | ChangeBridgePermanentGkN
  | ChangeBridgeCorrelatedOccupancy
  | ChangeAssessmentKnowledgeLevel
  | ChangeAssessmentLimitState
  | ChangeAssessmentAnalysisMethod
  | ChangeAssessmentSupportedBuildingId
  | ChangeAssessmentGammaEl
  | ChangeTowerHeightM
  | ChangeTowerIsChimney
  | ChangeTowerQNominal
  | ChangeTowerPermanentGkN
  | ChangeTowerCorrelatedOccupancy
  | ChangeSiloHeightM
  | ChangeSiloRadiusM
  | ChangeSiloPermanentGkN
  | ChangeSiloContentQkN
  | ChangeSiloContentCategory
  | ChangeSiloFillingRatio
  | ChangeSiloNRdN
  | ChangeSiloVRdN
  | ChangeSiloQNominal
  | ChangeTankHeightM
  | ChangeTankRadiusM
  | ChangeTankPermanentGkN
  | ChangeTankContentQkN
  | ChangeTankContentCategory
  | ChangeTankFillingRatio
  | ChangeTankVRdN
  | ChangeFoundationSupportedBuildingId
  | ChangeTieBeamFoundationBId
  | ChangeTieBeamFoundationAId
  | ChangeSeismicSlopeGoverningLayerId
  | ChangeFoundationAreaM2
  | ChangeFoundationPRdPa
  | ChangeFoundationPassiveResistanceN
  | ChangeFoundationKFoundation
  | ChangeFoundationKSoil
  | ChangeRetainingWallHeightM
  | ChangeRetainingWallPhiDeg
  | ChangeRetainingWallSoilGamma
  | ChangeRetainingWallR
  | ChangeRetainingWallHRdNPerM
  | ChangeSiteVS30MS
  | ChangeSiteNSpt
  | ChangeSiteCUKpa
  | ChangeSiteSoftClayThicknessM
  | ChangeSiteSoftClayPi
  | ChangeSiteLiquefiableLayer
  | ChangeSiteAlluviumThicknessM
  | ChangeSiteAlluviumVSMS
  | ChangeSiteBelowAlluviumVSMS
  | ChangeBuildingSetbackRatio
  | ChangeBuildingStoreyStiffnessRatio
  | ChangeBuildingStoreyStiffnessBasis
  | ChangeBuildingPlanEnvGapRatio
  | ChangeBuildingJointGapM
  | ChangeBuildingNsWeightN
  | ChangeBuildingNsHeightM
  | ChangeBuildingNsPeriodS
  | ChangeBuildingNsQ
  | ChangeBuildingNsResistanceN
  | ChangeBuildingColMomentSumNm
  | ChangeBuildingBeamMomentSumNm
  | ChangeMemberFastenerDiameterMm
  | ChangeMemberTimberThicknessMm
  | ChangeMemberWallThicknessM
  | ChangeMemberEffectiveStiffnessRatio
  | ChangeMemberEffectiveStiffnessBasis
  | ChangeMemberWallPoissonRatio
  | ChangeMemberWallStartXM
  | ChangeMemberWallStartYM
  | ChangeMemberWallLengthM
  | ChangeMemberWallDirectionRad
  | ChangeMemberWallStoreyId
  | ChangeMemberWallEffHeightM
  | ChangeBridgePierHeightM
  | ChangeBridgePierMomentRdNm
  | ChangeBridgePierShearRdN
  | ChangeBridgeImportanceClass
  | ChangeBridgeVerticalPeriodS
  | ChangeBridgePierCrackedInertiaRatio
  | ChangeBridgeDeckLengthM
  | ChangeBridgeDeckEiNm2
  | ChangeAssessmentChordDemandRad
  | ChangeAssessmentChordYieldRad
  | ChangeAssessmentChordUltRad
  | ChangeTowerEiNm2
  | ChangeFoundationLiqN160
  | ChangeFoundationLiqFinesPercent
  | ChangeFoundationLiqDepthM
  | ChangeFoundationLiqWaterDepthM
  | ChangeFoundationLiqUnitWtNM3
  | ChangeFoundationLiqMagnitude
  | ChangeAssessmentSectionDepthM
  | ChangeAssessmentSectionWidthM
  | ChangeAssessmentEffDepthM
  | ChangeAssessmentShearSpanM
  | ChangeAssessmentAxialForceN
  | ChangeAssessmentShearForceDemandN
  | ChangeAssessmentAs1AreaM2
  | ChangeAssessmentAs2AreaM2
  | ChangeAssessmentBarDiameterMm
  | ChangeAssessmentSteelYieldMpa
  | ChangeAssessmentFcMpa
  | ChangeAssessmentStrpLegAreaM2
  | ChangeAssessmentStrpSpacingM
  | ChangeAssessmentStrpYieldMpa
  | ChangeAssessmentCoreWidthM
  | ChangeAssessmentCoreDepthM
  | ChangeAssessmentMidSteelAreaM2
  | ChangeAssessmentRbarCount
  | ChangeAssessmentRbarSpacingM
  | ChangeAssessmentDiagReinfRatio
  | ChangeAssessmentShearCrack
  | ChangeFoundationLiqLayerM
  | ChangeFoundationFootingWidthM
  | ChangeFoundationBearAxialN
  | ChangeFoundationBearShearN
  | ChangeFoundationBearMomentNm
  | ChangeFoundationBearingNMaxN
  | ChangeFoundationSoilPhiDeg
  | ChangeFoundationPileDiameterM
  | ChangeFoundationPileLengthM
  | ChangeFoundationPileModulusPa
  | ChangeFoundationPileKsNM3
  | ChangeFoundationPileHeadShearN
  | ChangeFoundationPileHeadFixed
  | ChangeFoundationPileIfaceM
  | ChangeFoundationPileUpVsMS
  | ChangeFoundationPileLowVsMS
  | ChangeFoundationPileMomentRdNm
  | ChangeStoreyVariableCategory
  | ChangeStoreyVariableQkN
  | ChangeBridgeVariableCategory
  | ChangeBridgeVariableQkN
  | ChangeTowerVariableCategory
  | ChangeTowerVariableQkN
  | InsertStoreyVariable
  | RemoveStoreyVariable
  | InsertBridgeVariable
  | RemoveBridgeVariable
  | InsertTowerVariable
  | RemoveTowerVariable
  | ChangeSoilLayerFoundationId
  | ChangeSoilLayerThicknessM
  | ChangeSoilUnitWtNM3
  | ChangeSoilLayerVsMS
  | ChangeSoilLayerN160
  | ChangeSoilLayerFinesPercent
  | InsertSoilLayer
  | RemoveSoilLayer
  | ChangePlacementX
  | ChangePlacementY
  | ChangePlacementZ
  | ChangePlacementYaw
  | InsertPlacement
  | RemovePlacement
  | ChangeBuildingOtBaseXM
  | ChangeBuildingOtBaseYM
  | ChangeBuildingTiedownNm
  | ChangeDiaphragmBuildingId
  | ChangeDiaphragmStoreyId
  | ChangeDiaphragmMaterial
  | ChangeDiaphragmThicknessM
  | ChangeDiaphragmSpanM
  | ChangeDiaphragmDepthM
  | ChangeDiaphragmOpeningWidthM
  | ChangeDiaphragmShearStrMpa
  | ChangeDiaphragmChordAreaM2
  | ChangeDiaphragmChordStrMpa
  | ChangeDiaphragmGammaD
  | ChangeAccelerogramBuildingId
  | ChangeAccelerogramLabel
  | ChangeAccelerogramSamples
  | ChangeAccelerogramDtS
  | ChangeAccelerogramScaleFactor
  | InsertDiaphragm
  | RemoveDiaphragm
  | InsertAccelerogram
  | RemoveAccelerogram
  | ChangeTowerBaseRotStiffNm
  | ChangeTankShellTM
  | ChangeTankShellStrMpa
  | ChangePierBridgeId
  | ChangePierStiffnessBasis
  | ChangePierXM
  | ChangePierHeightM
  | ChangePierSectionBM
  | ChangePierSectionHM
  | ChangePierModulusMpa
  | ChangePierMomentRdNm
  | ChangePierShearRdN
  | ChangePierReinforcementRatio
  | ChangePierCoverM
  | ChangePierConcreteFckMpa
  | ChangePierSteelFykMpa
  | ChangePierAxialLoadN
  | ChangeBearingBridgeId
  | ChangeJointBuildingId
  | ChangeJointStoreyId
  | ChangeJointColumnId
  | ChangeJointBeamId
  | ChangeBearingPierId
  | ChangeBearingDevice
  | ChangeBearingXM
  | ChangeJointXM
  | ChangeJointYM
  | ChangeBearingStiffnessNPerM
  | ChangeBearingDRdM
  | ChangeBearingSeatLengthM
  | ChangeJointColumnMomentRdNm
  | ChangeJointBeamMomentRdNm
  | ChangeBearingMinSupportM
  | InsertPier
  | RemovePier
  | InsertBearing
  | InsertJoint
  | InsertTieBeam
  | InsertSeismicSlope
  | RemoveBearing
  | RemoveJoint
  | RemoveTieBeam
  | RemoveSeismicSlope
  | InsertMember
  | RemoveMember
  | InsertStorey
  | RemoveStorey
  | InsertSystem
  | RemoveSystem
  | ChangeJointColMomentRdNm
  | ChangeRetainingWallWaterHeightM
  | ChangeRetainingWallSaturatedGamma
  | ChangeRetainingWallPermeabilityMS
  | ChangeFoundationBaseFrictionDeg
  | ChangeFoundationBasePorePressurePa
  | ChangeSeismicSlopeAngle
  | ChangeSeismicSlopeCircleCentreX
  | ChangeSeismicSlopeCircleCentreZ
  | ChangeSeismicSlopeCircleRadius
  | ChangeSeismicSlopeCohesion
  | ChangeSeismicSlopeFrictionAngle
  | ChangeSeismicSlopeHeight
  | ChangeSeismicSlopeTopographicFactor
  | ChangeTieBeamDepth
  | ChangeTieBeamDisplacementLimit
  | ChangeTieBeamRelativeDisplacement
  | ChangeTieBeamSpan
  | ChangeTieBeamSteelArea
  | ChangeTieBeamSteelDesignStrength
  | ChangeTieBeamWidth;

export const parseEn1998Mutation: NormWireReader<En1998Mutation> = normWireTagged<En1998Mutation, "mutation">("mutation", {
  changeAnnex: parseChangeAnnex,
  insertBuilding: parseInsertBuilding,
  removeBuilding: parseRemoveBuilding,
  changeSystemVRdN: parseChangeSystemVRdN,
  changeStoreyPermanentGkN: parseChangeStoreyPermanentGkN,
  changeStoreyStiffnessX: parseChangeStoreyStiffnessX,
  changeStoreyDriftXM: parseChangeStoreyDriftXM,
  changeBuildingPlanRegular: parseChangeBuildingPlanRegular,
  changeElevationRegular: parseChangeElevationRegular,
  changeMemberDetailing: parseChangeMemberDetailing,
  changeMasonryWallRatio: parseChangeMasonryWallRatio,
  insertBridge: parseInsertBridge,
  changeBridgeVRdN: parseChangeBridgeVRdN,
  insertAssessment: parseInsertAssessment,
  changeAssessmentRKN: parseChangeAssessmentRKN,
  insertSilo: parseInsertSilo,
  insertTank: parseInsertTank,
  insertFoundation: parseInsertFoundation,
  insertRetainingWall: parseInsertRetainingWall,
  insertTower: parseInsertTower,
  changeTowerMRdNm: parseChangeTowerMRdNm,
  removeBridge: parseRemoveBridge,
  removeAssessment: parseRemoveAssessment,
  removeSilo: parseRemoveSilo,
  removeTank: parseRemoveTank,
  removeFoundation: parseRemoveFoundation,
  removeRetainingWall: parseRemoveRetainingWall,
  removeTower: parseRemoveTower,
  changeSiteSApR: parseChangeSiteSApR,
  changeSiteAGr: parseChangeSiteAGr,
  changeSiteDeGroundCombo: parseChangeSiteDeGroundCombo,
  changeSiteEnGroundType: parseChangeSiteEnGroundType,
  changeSiteEnSpectrumType: parseChangeSiteEnSpectrumType,
  changeSiteImportanceClass: parseChangeSiteImportanceClass,
  changeBuildingName: parseChangeBuildingName,
  changeBuildingPlanWidthM: parseChangeBuildingPlanWidthM,
  changeBuildingPlanLengthM: parseChangeBuildingPlanLengthM,
  changeBuildingT1Method: parseChangeBuildingT1Method,
  changeBuildingT1GivenS: parseChangeBuildingT1GivenS,
  changeBuildingCt: parseChangeBuildingCt,
  changeBuildingDriftLimitClass: parseChangeBuildingDriftLimitClass,
  changeBuildingNu: parseChangeBuildingNu,
  changeBuildingMultiResistingSystems: parseChangeBuildingMultiResistingSystems,
  changeBuildingClaimsSimpleMasonry: parseChangeBuildingClaimsSimpleMasonry,
  changeBuildingAccidentalEccRatio: parseChangeBuildingAccidentalEccRatio,
  changeSystemDirection: parseChangeSystemDirection,
  changeSystemSystemType: parseChangeSystemSystemType,
  changeSystemMaterial: parseChangeSystemMaterial,
  changeSystemDuctilityClass: parseChangeSystemDuctilityClass,
  changeSystemQ0: parseChangeSystemQ0,
  changeSystemAlphaUOverAlpha1: parseChangeSystemAlphaUOverAlpha1,
  changeSystemKW: parseChangeSystemKW,
  changeStoreyHeightM: parseChangeStoreyHeightM,
  changeStoreyCorrelatedOccupancy: parseChangeStoreyCorrelatedOccupancy,
  changeStoreyStiffnessY: parseChangeStoreyStiffnessY,
  changeStoreyCentreOfMassXM: parseChangeStoreyCentreOfMassXM,
  changeStoreyCentreOfMassYM: parseChangeStoreyCentreOfMassYM,
  changeStoreyCentreOfStiffnessXM: parseChangeStoreyCentreOfStiffnessXM,
  changeStoreyCentreOfStiffnessYM: parseChangeStoreyCentreOfStiffnessYM,
  changeStoreyDriftYM: parseChangeStoreyDriftYM,
  changeStoreyShearResistanceN: parseChangeStoreyShearResistanceN,
  changeMemberMaterial: parseChangeMemberMaterial,
  changeMemberRole: parseChangeMemberRole,
  changeMemberMinDimensionM: parseChangeMemberMinDimensionM,
  changeMemberRho: parseChangeMemberRho,
  changeMemberRhoPrime: parseChangeMemberRhoPrime,
  changeMemberOmegaWd: parseChangeMemberOmegaWd,
  changeMemberSteelSectionClass: parseChangeMemberSteelSectionClass,
  changeBridgePeriodRatio: parseChangeBridgePeriodRatio,
  changeBridgeFundamentalPeriodS: parseChangeBridgeFundamentalPeriodS,
  changeBridgeBearingDRdM: parseChangeBridgeBearingDRdM,
  changeBridgePermanentGkN: parseChangeBridgePermanentGkN,
  changeBridgeCorrelatedOccupancy: parseChangeBridgeCorrelatedOccupancy,
  changeAssessmentKnowledgeLevel: parseChangeAssessmentKnowledgeLevel,
  changeAssessmentLimitState: parseChangeAssessmentLimitState,
  changeAssessmentAnalysisMethod: parseChangeAssessmentAnalysisMethod,
  changeAssessmentSupportedBuildingId: parseChangeAssessmentSupportedBuildingId,
  changeAssessmentGammaEl: parseChangeAssessmentGammaEl,
  changeTowerHeightM: parseChangeTowerHeightM,
  changeTowerIsChimney: parseChangeTowerIsChimney,
  changeTowerQNominal: parseChangeTowerQNominal,
  changeTowerPermanentGkN: parseChangeTowerPermanentGkN,
  changeTowerCorrelatedOccupancy: parseChangeTowerCorrelatedOccupancy,
  changeSiloHeightM: parseChangeSiloHeightM,
  changeSiloRadiusM: parseChangeSiloRadiusM,
  changeSiloPermanentGkN: parseChangeSiloPermanentGkN,
  changeSiloContentQkN: parseChangeSiloContentQkN,
  changeSiloContentCategory: parseChangeSiloContentCategory,
  changeSiloFillingRatio: parseChangeSiloFillingRatio,
  changeSiloNRdN: parseChangeSiloNRdN,
  changeSiloVRdN: parseChangeSiloVRdN,
  changeSiloQNominal: parseChangeSiloQNominal,
  changeTankHeightM: parseChangeTankHeightM,
  changeTankRadiusM: parseChangeTankRadiusM,
  changeTankPermanentGkN: parseChangeTankPermanentGkN,
  changeTankContentQkN: parseChangeTankContentQkN,
  changeTankContentCategory: parseChangeTankContentCategory,
  changeTankFillingRatio: parseChangeTankFillingRatio,
  changeTankVRdN: parseChangeTankVRdN,
  changeFoundationSupportedBuildingId: parseChangeFoundationSupportedBuildingId,
  changeTieBeamFoundationBId: parseChangeTieBeamFoundationBId,
  changeTieBeamFoundationAId: parseChangeTieBeamFoundationAId,
  changeSeismicSlopeGoverningLayerId: parseChangeSeismicSlopeGoverningLayerId,
  changeFoundationAreaM2: parseChangeFoundationAreaM2,
  changeFoundationPRdPa: parseChangeFoundationPRdPa,
  changeFoundationPassiveResistanceN: parseChangeFoundationPassiveResistanceN,
  changeFoundationKFoundation: parseChangeFoundationKFoundation,
  changeFoundationKSoil: parseChangeFoundationKSoil,
  changeRetainingWallHeightM: parseChangeRetainingWallHeightM,
  changeRetainingWallPhiDeg: parseChangeRetainingWallPhiDeg,
  changeRetainingWallSoilGamma: parseChangeRetainingWallSoilGamma,
  changeRetainingWallR: parseChangeRetainingWallR,
  changeRetainingWallHRdNPerM: parseChangeRetainingWallHRdNPerM,
  changeSiteVS30MS: parseChangeSiteVS30MS,
  changeSiteNSpt: parseChangeSiteNSpt,
  changeSiteCUKpa: parseChangeSiteCUKpa,
  changeSiteSoftClayThicknessM: parseChangeSiteSoftClayThicknessM,
  changeSiteSoftClayPi: parseChangeSiteSoftClayPi,
  changeSiteLiquefiableLayer: parseChangeSiteLiquefiableLayer,
  changeSiteAlluviumThicknessM: parseChangeSiteAlluviumThicknessM,
  changeSiteAlluviumVSMS: parseChangeSiteAlluviumVSMS,
  changeSiteBelowAlluviumVSMS: parseChangeSiteBelowAlluviumVSMS,
  changeBuildingSetbackRatio: parseChangeBuildingSetbackRatio,
  changeBuildingStoreyStiffnessRatio: parseChangeBuildingStoreyStiffnessRatio,
  changeBuildingStoreyStiffnessBasis: parseChangeBuildingStoreyStiffnessBasis,
  changeBuildingPlanEnvGapRatio: parseChangeBuildingPlanEnvGapRatio,
  changeBuildingJointGapM: parseChangeBuildingJointGapM,
  changeBuildingNsWeightN: parseChangeBuildingNsWeightN,
  changeBuildingNsHeightM: parseChangeBuildingNsHeightM,
  changeBuildingNsPeriodS: parseChangeBuildingNsPeriodS,
  changeBuildingNsQ: parseChangeBuildingNsQ,
  changeBuildingNsResistanceN: parseChangeBuildingNsResistanceN,
  changeBuildingColMomentSumNm: parseChangeBuildingColMomentSumNm,
  changeBuildingBeamMomentSumNm: parseChangeBuildingBeamMomentSumNm,
  changeMemberFastenerDiameterMm: parseChangeMemberFastenerDiameterMm,
  changeMemberTimberThicknessMm: parseChangeMemberTimberThicknessMm,
  changeMemberWallThicknessM: parseChangeMemberWallThicknessM,
  changeMemberEffectiveStiffnessRatio: parseChangeMemberEffectiveStiffnessRatio,
  changeMemberEffectiveStiffnessBasis: parseChangeMemberEffectiveStiffnessBasis,
  changeMemberWallPoissonRatio: parseChangeMemberWallPoissonRatio,
  changeMemberWallStartXM: parseChangeMemberWallStartXM,
  changeMemberWallStartYM: parseChangeMemberWallStartYM,
  changeMemberWallLengthM: parseChangeMemberWallLengthM,
  changeMemberWallDirectionRad: parseChangeMemberWallDirectionRad,
  changeMemberWallStoreyId: parseChangeMemberWallStoreyId,
  changeMemberWallEffHeightM: parseChangeMemberWallEffHeightM,
  changeBridgePierHeightM: parseChangeBridgePierHeightM,
  changeBridgePierMomentRdNm: parseChangeBridgePierMomentRdNm,
  changeBridgePierShearRdN: parseChangeBridgePierShearRdN,
  changeBridgeImportanceClass: parseChangeBridgeImportanceClass,
  changeBridgeVerticalPeriodS: parseChangeBridgeVerticalPeriodS,
  changeBridgePierCrackedInertiaRatio: parseChangeBridgePierCrackedInertiaRatio,
  changeBridgeDeckLengthM: parseChangeBridgeDeckLengthM,
  changeBridgeDeckEiNm2: parseChangeBridgeDeckEiNm2,
  changeAssessmentChordDemandRad: parseChangeAssessmentChordDemandRad,
  changeAssessmentChordYieldRad: parseChangeAssessmentChordYieldRad,
  changeAssessmentChordUltRad: parseChangeAssessmentChordUltRad,
  changeTowerEiNm2: parseChangeTowerEiNm2,
  changeFoundationLiqN160: parseChangeFoundationLiqN160,
  changeFoundationLiqFinesPercent: parseChangeFoundationLiqFinesPercent,
  changeFoundationLiqDepthM: parseChangeFoundationLiqDepthM,
  changeFoundationLiqWaterDepthM: parseChangeFoundationLiqWaterDepthM,
  changeFoundationLiqUnitWtNM3: parseChangeFoundationLiqUnitWtNM3,
  changeFoundationLiqMagnitude: parseChangeFoundationLiqMagnitude,
  changeAssessmentSectionDepthM: parseChangeAssessmentSectionDepthM,
  changeAssessmentSectionWidthM: parseChangeAssessmentSectionWidthM,
  changeAssessmentEffDepthM: parseChangeAssessmentEffDepthM,
  changeAssessmentShearSpanM: parseChangeAssessmentShearSpanM,
  changeAssessmentAxialForceN: parseChangeAssessmentAxialForceN,
  changeAssessmentShearForceDemandN: parseChangeAssessmentShearForceDemandN,
  changeAssessmentAs1AreaM2: parseChangeAssessmentAs1AreaM2,
  changeAssessmentAs2AreaM2: parseChangeAssessmentAs2AreaM2,
  changeAssessmentBarDiameterMm: parseChangeAssessmentBarDiameterMm,
  changeAssessmentSteelYieldMpa: parseChangeAssessmentSteelYieldMpa,
  changeAssessmentFcMpa: parseChangeAssessmentFcMpa,
  changeAssessmentStrpLegAreaM2: parseChangeAssessmentStrpLegAreaM2,
  changeAssessmentStrpSpacingM: parseChangeAssessmentStrpSpacingM,
  changeAssessmentStrpYieldMpa: parseChangeAssessmentStrpYieldMpa,
  changeAssessmentCoreWidthM: parseChangeAssessmentCoreWidthM,
  changeAssessmentCoreDepthM: parseChangeAssessmentCoreDepthM,
  changeAssessmentMidSteelAreaM2: parseChangeAssessmentMidSteelAreaM2,
  changeAssessmentRbarCount: parseChangeAssessmentRbarCount,
  changeAssessmentRbarSpacingM: parseChangeAssessmentRbarSpacingM,
  changeAssessmentDiagReinfRatio: parseChangeAssessmentDiagReinfRatio,
  changeAssessmentShearCrack: parseChangeAssessmentShearCrack,
  changeFoundationLiqLayerM: parseChangeFoundationLiqLayerM,
  changeFoundationFootingWidthM: parseChangeFoundationFootingWidthM,
  changeFoundationBearAxialN: parseChangeFoundationBearAxialN,
  changeFoundationBearShearN: parseChangeFoundationBearShearN,
  changeFoundationBearMomentNm: parseChangeFoundationBearMomentNm,
  changeFoundationBearingNMaxN: parseChangeFoundationBearingNMaxN,
  changeFoundationSoilPhiDeg: parseChangeFoundationSoilPhiDeg,
  changeFoundationPileDiameterM: parseChangeFoundationPileDiameterM,
  changeFoundationPileLengthM: parseChangeFoundationPileLengthM,
  changeFoundationPileModulusPa: parseChangeFoundationPileModulusPa,
  changeFoundationPileKsNM3: parseChangeFoundationPileKsNM3,
  changeFoundationPileHeadShearN: parseChangeFoundationPileHeadShearN,
  changeFoundationPileHeadFixed: parseChangeFoundationPileHeadFixed,
  changeFoundationPileIfaceM: parseChangeFoundationPileIfaceM,
  changeFoundationPileUpVsMS: parseChangeFoundationPileUpVsMS,
  changeFoundationPileLowVsMS: parseChangeFoundationPileLowVsMS,
  changeFoundationPileMomentRdNm: parseChangeFoundationPileMomentRdNm,
  changeStoreyVariableCategory: parseChangeStoreyVariableCategory,
  changeStoreyVariableQkN: parseChangeStoreyVariableQkN,
  changeBridgeVariableCategory: parseChangeBridgeVariableCategory,
  changeBridgeVariableQkN: parseChangeBridgeVariableQkN,
  changeTowerVariableCategory: parseChangeTowerVariableCategory,
  changeTowerVariableQkN: parseChangeTowerVariableQkN,
  insertStoreyVariable: parseInsertStoreyVariable,
  removeStoreyVariable: parseRemoveStoreyVariable,
  insertBridgeVariable: parseInsertBridgeVariable,
  removeBridgeVariable: parseRemoveBridgeVariable,
  insertTowerVariable: parseInsertTowerVariable,
  removeTowerVariable: parseRemoveTowerVariable,
  changeSoilLayerFoundationId: parseChangeSoilLayerFoundationId,
  changeSoilLayerThicknessM: parseChangeSoilLayerThicknessM,
  changeSoilUnitWtNM3: parseChangeSoilUnitWtNM3,
  changeSoilLayerVsMS: parseChangeSoilLayerVsMS,
  changeSoilLayerN160: parseChangeSoilLayerN160,
  changeSoilLayerFinesPercent: parseChangeSoilLayerFinesPercent,
  insertSoilLayer: parseInsertSoilLayer,
  removeSoilLayer: parseRemoveSoilLayer,
  changePlacementX: parseChangePlacementX,
  changePlacementY: parseChangePlacementY,
  changePlacementZ: parseChangePlacementZ,
  changePlacementYaw: parseChangePlacementYaw,
  insertPlacement: parseInsertPlacement,
  removePlacement: parseRemovePlacement,
  changeBuildingOtBaseXM: parseChangeBuildingOtBaseXM,
  changeBuildingOtBaseYM: parseChangeBuildingOtBaseYM,
  changeBuildingTiedownNm: parseChangeBuildingTiedownNm,
  changeDiaphragmBuildingId: parseChangeDiaphragmBuildingId,
  changeDiaphragmStoreyId: parseChangeDiaphragmStoreyId,
  changeDiaphragmMaterial: parseChangeDiaphragmMaterial,
  changeDiaphragmThicknessM: parseChangeDiaphragmThicknessM,
  changeDiaphragmSpanM: parseChangeDiaphragmSpanM,
  changeDiaphragmDepthM: parseChangeDiaphragmDepthM,
  changeDiaphragmOpeningWidthM: parseChangeDiaphragmOpeningWidthM,
  changeDiaphragmShearStrMpa: parseChangeDiaphragmShearStrMpa,
  changeDiaphragmChordAreaM2: parseChangeDiaphragmChordAreaM2,
  changeDiaphragmChordStrMpa: parseChangeDiaphragmChordStrMpa,
  changeDiaphragmGammaD: parseChangeDiaphragmGammaD,
  changeAccelerogramBuildingId: parseChangeAccelerogramBuildingId,
  changeAccelerogramLabel: parseChangeAccelerogramLabel,
  changeAccelerogramSamples: parseChangeAccelerogramSamples,
  changeAccelerogramDtS: parseChangeAccelerogramDtS,
  changeAccelerogramScaleFactor: parseChangeAccelerogramScaleFactor,
  insertDiaphragm: parseInsertDiaphragm,
  removeDiaphragm: parseRemoveDiaphragm,
  insertAccelerogram: parseInsertAccelerogram,
  removeAccelerogram: parseRemoveAccelerogram,
  changeTowerBaseRotStiffNm: parseChangeTowerBaseRotStiffNm,
  changeTankShellTM: parseChangeTankShellTM,
  changeTankShellStrMpa: parseChangeTankShellStrMpa,
  changePierBridgeId: parseChangePierBridgeId,
  changePierStiffnessBasis: parseChangePierStiffnessBasis,
  changePierXM: parseChangePierXM,
  changePierHeightM: parseChangePierHeightM,
  changePierSectionBM: parseChangePierSectionBM,
  changePierSectionHM: parseChangePierSectionHM,
  changePierModulusMpa: parseChangePierModulusMpa,
  changePierMomentRdNm: parseChangePierMomentRdNm,
  changePierShearRdN: parseChangePierShearRdN,
  changePierReinforcementRatio: parseChangePierReinforcementRatio,
  changePierCoverM: parseChangePierCoverM,
  changePierConcreteFckMpa: parseChangePierConcreteFckMpa,
  changePierSteelFykMpa: parseChangePierSteelFykMpa,
  changePierAxialLoadN: parseChangePierAxialLoadN,
  changeBearingBridgeId: parseChangeBearingBridgeId,
  changeJointBuildingId: parseChangeJointBuildingId,
  changeJointStoreyId: parseChangeJointStoreyId,
  changeJointColumnId: parseChangeJointColumnId,
  changeJointBeamId: parseChangeJointBeamId,
  changeBearingPierId: parseChangeBearingPierId,
  changeBearingDevice: parseChangeBearingDevice,
  changeBearingXM: parseChangeBearingXM,
  changeJointXM: parseChangeJointXM,
  changeJointYM: parseChangeJointYM,
  changeBearingStiffnessNPerM: parseChangeBearingStiffnessNPerM,
  changeBearingDRdM: parseChangeBearingDRdM,
  changeBearingSeatLengthM: parseChangeBearingSeatLengthM,
  changeJointColumnMomentRdNm: parseChangeJointColumnMomentRdNm,
  changeJointBeamMomentRdNm: parseChangeJointBeamMomentRdNm,
  changeBearingMinSupportM: parseChangeBearingMinSupportM,
  insertPier: parseInsertPier,
  removePier: parseRemovePier,
  insertBearing: parseInsertBearing,
  insertJoint: parseInsertJoint,
  insertTieBeam: parseInsertTieBeam,
  insertSeismicSlope: parseInsertSeismicSlope,
  removeBearing: parseRemoveBearing,
  removeJoint: parseRemoveJoint,
  removeTieBeam: parseRemoveTieBeam,
  removeSeismicSlope: parseRemoveSeismicSlope,
  insertMember: parseInsertMember,
  removeMember: parseRemoveMember,
  insertStorey: parseInsertStorey,
  removeStorey: parseRemoveStorey,
  insertSystem: parseInsertSystem,
  removeSystem: parseRemoveSystem,
  changeJointColMomentRdNm: parseChangeJointColMomentRdNm,
  changeRetainingWallWaterHeightM: parseChangeRetainingWallWaterHeightM,
  changeRetainingWallSaturatedGamma: parseChangeRetainingWallSaturatedGamma,
  changeRetainingWallPermeabilityMS: parseChangeRetainingWallPermeabilityMS,
  changeFoundationBaseFrictionDeg: parseChangeFoundationBaseFrictionDeg,
  changeFoundationBasePorePressurePa: parseChangeFoundationBasePorePressurePa,
  changeSeismicSlopeAngle: parseChangeSeismicSlopeAngle,
  changeSeismicSlopeCircleCentreX: parseChangeSeismicSlopeCircleCentreX,
  changeSeismicSlopeCircleCentreZ: parseChangeSeismicSlopeCircleCentreZ,
  changeSeismicSlopeCircleRadius: parseChangeSeismicSlopeCircleRadius,
  changeSeismicSlopeCohesion: parseChangeSeismicSlopeCohesion,
  changeSeismicSlopeFrictionAngle: parseChangeSeismicSlopeFrictionAngle,
  changeSeismicSlopeHeight: parseChangeSeismicSlopeHeight,
  changeSeismicSlopeTopographicFactor: parseChangeSeismicSlopeTopographicFactor,
  changeTieBeamDepth: parseChangeTieBeamDepth,
  changeTieBeamDisplacementLimit: parseChangeTieBeamDisplacementLimit,
  changeTieBeamRelativeDisplacement: parseChangeTieBeamRelativeDisplacement,
  changeTieBeamSpan: parseChangeTieBeamSpan,
  changeTieBeamSteelArea: parseChangeTieBeamSteelArea,
  changeTieBeamSteelDesignStrength: parseChangeTieBeamSteelDesignStrength,
  changeTieBeamWidth: parseChangeTieBeamWidth,
});
