/** 🧺️ `En1990Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireTagged, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ChangeReferencePeriodYears, parseChangeReferencePeriodYears } from "./⏱️change-reference-period-years/🧬️schema/🟦️.ts";
import { type ChangePermanentGk, parseChangePermanentGk } from "./⚓️change-permanent-gk/🧬️schema/🟦️.ts";
import { type ChangePermanentKind, parseChangePermanentKind } from "./⚓️change-permanent-kind/🧬️schema/🟦️.ts";
import { type ChangeConsequenceClass, parseChangeConsequenceClass } from "./⚠️change-consequence-class/🧬️schema/🟦️.ts";
import { type ChangeAltitudeM, parseChangeAltitudeM } from "./⛰️change-altitude-m/🧬️schema/🟦️.ts";
import { type RemoveEffect, parseRemoveEffect } from "./✂️remove-effect/🧬️schema/🟦️.ts";
import { type InsertPermanent, parseInsertPermanent } from "./➕insert-permanent/🧬️schema/🟦️.ts";
import { type InsertCase, parseInsertCase } from "./➕️insert-case/🧬️schema/🟦️.ts";
import { type RemovePermanent, parseRemovePermanent } from "./➖remove-permanent/🧬️schema/🟦️.ts";
import { type RemoveCase, parseRemoveCase } from "./➖️remove-case/🧬️schema/🟦️.ts";
import { type ChangeBridgeBallastDepth, parseChangeBridgeBallastDepth } from "./🌉️change-bridge-ballast-depth/🧬️schema/🟦️.ts";
import { type ChangeBridgeBasicWindSpeed, parseChangeBridgeBasicWindSpeed } from "./🌉️change-bridge-basic-wind-speed/🧬️schema/🟦️.ts";
import { type ChangeBridgeDampingRatio, parseChangeBridgeDampingRatio } from "./🌉️change-bridge-damping-ratio/🧬️schema/🟦️.ts";
import { type ChangeBridgeDeckAccelLimit, parseChangeBridgeDeckAccelLimit } from "./🌉️change-bridge-deck-accel-limit/🧬️schema/🟦️.ts";
import { type ChangeBridgeDeckAcceleration, parseChangeBridgeDeckAcceleration } from "./🌉️change-bridge-deck-acceleration/🧬️schema/🟦️.ts";
import { type ChangeBridgeDeckMass, parseChangeBridgeDeckMass } from "./🌉️change-bridge-deck-mass/🧬️schema/🟦️.ts";
import { type ChangeBridgeDeckTwistLimit, parseChangeBridgeDeckTwistLimit } from "./🌉️change-bridge-deck-twist-limit/🧬️schema/🟦️.ts";
import { type ChangeBridgeDeckTwist, parseChangeBridgeDeckTwist } from "./🌉️change-bridge-deck-twist/🧬️schema/🟦️.ts";
import { type ChangeBridgeDeflectionLimit, parseChangeBridgeDeflectionLimit } from "./🌉️change-bridge-deflection-limit/🧬️schema/🟦️.ts";
import { type ChangeBridgeDeflection, parseChangeBridgeDeflection } from "./🌉️change-bridge-deflection/🧬️schema/🟦️.ts";
import { type ChangeBridgeEndRotation, parseChangeBridgeEndRotation } from "./🌉️change-bridge-end-rotation/🧬️schema/🟦️.ts";
import { type ChangeBridgeLineSpeed, parseChangeBridgeLineSpeed } from "./🌉️change-bridge-line-speed/🧬️schema/🟦️.ts";
import { type ChangeBridgeMemberId, parseChangeBridgeMemberId } from "./🌉️change-bridge-member-id/🧬️schema/🟦️.ts";
import { type ChangeBridgeNaturalFrequency, parseChangeBridgeNaturalFrequency } from "./🌉️change-bridge-natural-frequency/🧬️schema/🟦️.ts";
import { type ChangeBridgeTrackGauge, parseChangeBridgeTrackGauge } from "./🌉️change-bridge-track-gauge/🧬️schema/🟦️.ts";
import { type InsertBridgeSls, parseInsertBridgeSls } from "./🌉️insert-bridge-sls/🧬️schema/🟦️.ts";
import { type RemoveBridgeSls, parseRemoveBridgeSls } from "./🌉️remove-bridge-sls/🧬️schema/🟦️.ts";
import { type InsertSeismic, parseInsertSeismic } from "./🌋insert-seismic/🧬️schema/🟦️.ts";
import { type ChangeSeismicAEk, parseChangeSeismicAEk } from "./🌋️change-seismic-a-ek/🧬️schema/🟦️.ts";
import { type ChangeSeismicImportanceClass, parseChangeSeismicImportanceClass } from "./🌋️change-seismic-importance-class/🧬️schema/🟦️.ts";
import { type ChangeAnnex, parseChangeAnnex } from "./🌍️change-annex/🧬️schema/🟦️.ts";
import { type ChangeFactorOverrideFactor, parseChangeFactorOverrideFactor } from "./🎚️change-factor-override-factor/🧬️schema/🟦️.ts";
import { type ChangeFactorOverrideJustif, parseChangeFactorOverrideJustif } from "./🎚️change-factor-override-justif/🧬️schema/🟦️.ts";
import { type ChangeFactorOverrideTarget, parseChangeFactorOverrideTarget } from "./🎚️change-factor-override-target/🧬️schema/🟦️.ts";
import { type ChangeFactorOverrideValue, parseChangeFactorOverrideValue } from "./🎚️change-factor-override-value/🧬️schema/🟦️.ts";
import { type InsertFactorOverride, parseInsertFactorOverride } from "./🎚️insert-factor-override/🧬️schema/🟦️.ts";
import { type RemoveFactorOverride, parseRemoveFactorOverride } from "./🎚️remove-factor-override/🧬️schema/🟦️.ts";
import { type ChangeReliabilityClass, parseChangeReliabilityClass } from "./🎯change-reliability-class/🧬️schema/🟦️.ts";
import { type ChangeReliabilityActionDist, parseChangeReliabilityActionDist } from "./🎲️change-reliability-action-dist/🧬️schema/🟦️.ts";
import { type ChangeReliabilityActionMean, parseChangeReliabilityActionMean } from "./🎲️change-reliability-action-mean/🧬️schema/🟦️.ts";
import { type ChangeReliabilityActionSd, parseChangeReliabilityActionSd } from "./🎲️change-reliability-action-sd/🧬️schema/🟦️.ts";
import { type ChangeReliabilityResistanceDist, parseChangeReliabilityResistanceDist } from "./🎲️change-reliability-resistance-dist/🧬️schema/🟦️.ts";
import { type ChangeReliabilityResistanceMean, parseChangeReliabilityResistanceMean } from "./🎲️change-reliability-resistance-mean/🧬️schema/🟦️.ts";
import { type ChangeReliabilityResistanceSd, parseChangeReliabilityResistanceSd } from "./🎲️change-reliability-resistance-sd/🧬️schema/🟦️.ts";
import { type InsertReliabilityAnalysis, parseInsertReliabilityAnalysis } from "./🎲️insert-reliability-analysis/🧬️schema/🟦️.ts";
import { type RemoveReliabilityAnalysis, parseRemoveReliabilityAnalysis } from "./🎲️remove-reliability-analysis/🧬️schema/🟦️.ts";
import { type ChangeVariableCategory, parseChangeVariableCategory } from "./🏋️change-variable-category/🧬️schema/🟦️.ts";
import { type ChangeVariableGroup, parseChangeVariableGroup } from "./🏋️change-variable-group/🧬️schema/🟦️.ts";
import { type ChangeVariableQk, parseChangeVariableQk } from "./🏋️change-variable-qk/🧬️schema/🟦️.ts";
import { type ChangeMemberDeflLimitRatioW3, parseChangeMemberDeflLimitRatioW3 } from "./🏗️change-member-defl-limit-ratio-w3/🧬️schema/🟦️.ts";
import { type ChangeMemberDeflLimitRatio, parseChangeMemberDeflLimitRatio } from "./🏗️change-member-defl-limit-ratio/🧬️schema/🟦️.ts";
import { type ChangeMemberDeflectionW1, parseChangeMemberDeflectionW1 } from "./🏗️change-member-deflection-w1/🧬️schema/🟦️.ts";
import { type ChangeMemberDeflectionW2, parseChangeMemberDeflectionW2 } from "./🏗️change-member-deflection-w2/🧬️schema/🟦️.ts";
import { type ChangeMemberDeflectionW3, parseChangeMemberDeflectionW3 } from "./🏗️change-member-deflection-w3/🧬️schema/🟦️.ts";
import { type ChangeMemberDesignDimension, parseChangeMemberDesignDimension } from "./🏗️change-member-design-dimension/🧬️schema/🟦️.ts";
import { type ChangeMemberDimensionTolerance, parseChangeMemberDimensionTolerance } from "./🏗️change-member-dimension-tolerance/🧬️schema/🟦️.ts";
import { type ChangeMemberFrameDrift, parseChangeMemberFrameDrift } from "./🏗️change-member-frame-drift/🧬️schema/🟦️.ts";
import { type ChangeMemberFrameHeight, parseChangeMemberFrameHeight } from "./🏗️change-member-frame-height/🧬️schema/🟦️.ts";
import { type ChangeMemberLabelDe, parseChangeMemberLabelDe } from "./🏗️change-member-label-de/🧬️schema/🟦️.ts";
import { type ChangeMemberLabelEn, parseChangeMemberLabelEn } from "./🏗️change-member-label-en/🧬️schema/🟦️.ts";
import { type ChangeMemberMatDesignStrength, parseChangeMemberMatDesignStrength } from "./🏗️change-member-mat-design-strength/🧬️schema/🟦️.ts";
import { type ChangeMemberMatPartialFactor, parseChangeMemberMatPartialFactor } from "./🏗️change-member-mat-partial-factor/🧬️schema/🟦️.ts";
import { type ChangeMemberMaterialConvFactor, parseChangeMemberMaterialConvFactor } from "./🏗️change-member-material-conv-factor/🧬️schema/🟦️.ts";
import { type ChangeMemberMaterialStrengthK, parseChangeMemberMaterialStrengthK } from "./🏗️change-member-material-strength-k/🧬️schema/🟦️.ts";
import { type ChangeMemberMemberAlphaCr, parseChangeMemberMemberAlphaCr } from "./🏗️change-member-member-alpha-cr/🧬️schema/🟦️.ts";
import { type ChangeMemberNominalDimension, parseChangeMemberNominalDimension } from "./🏗️change-member-nominal-dimension/🧬️schema/🟦️.ts";
import { type ChangeMemberPrecamber, parseChangeMemberPrecamber } from "./🏗️change-member-precamber/🧬️schema/🟦️.ts";
import { type ChangeMemberRdEquDestab, parseChangeMemberRdEquDestab } from "./🏗️change-member-rd-equ-destab/🧬️schema/🟦️.ts";
import { type ChangeMemberRdEquStab, parseChangeMemberRdEquStab } from "./🏗️change-member-rd-equ-stab/🧬️schema/🟦️.ts";
import { type ChangeMemberRdFat, parseChangeMemberRdFat } from "./🏗️change-member-rd-fat/🧬️schema/🟦️.ts";
import { type ChangeMemberRdGeo, parseChangeMemberRdGeo } from "./🏗️change-member-rd-geo/🧬️schema/🟦️.ts";
import { type ChangeMemberRdStr, parseChangeMemberRdStr } from "./🏗️change-member-rd-str/🧬️schema/🟦️.ts";
import { type ChangeMemberSecondOrderAnalysis, parseChangeMemberSecondOrderAnalysis } from "./🏗️change-member-second-order-analysis/🧬️schema/🟦️.ts";
import { type ChangeMemberSlsLimitChar, parseChangeMemberSlsLimitChar } from "./🏗️change-member-sls-limit-char/🧬️schema/🟦️.ts";
import { type ChangeMemberSlsLimitFreq, parseChangeMemberSlsLimitFreq } from "./🏗️change-member-sls-limit-freq/🧬️schema/🟦️.ts";
import { type ChangeMemberSlsLimitQp, parseChangeMemberSlsLimitQp } from "./🏗️change-member-sls-limit-qp/🧬️schema/🟦️.ts";
import { type ChangeMemberSpan, parseChangeMemberSpan } from "./🏗️change-member-span/🧬️schema/🟦️.ts";
import { type ChangeMemberStoreyDrift, parseChangeMemberStoreyDrift } from "./🏗️change-member-storey-drift/🧬️schema/🟦️.ts";
import { type ChangeMemberStoreyHeight, parseChangeMemberStoreyHeight } from "./🏗️change-member-storey-height/🧬️schema/🟦️.ts";
import { type ChangeMemberVibFreqMin, parseChangeMemberVibFreqMin } from "./🏗️change-member-vib-freq-min/🧬️schema/🟦️.ts";
import { type ChangeMemberVibrationFrequency, parseChangeMemberVibrationFrequency } from "./🏗️change-member-vibration-frequency/🧬️schema/🟦️.ts";
import { type ChangeStructureKind, parseChangeStructureKind } from "./🏛️change-structure-kind/🧬️schema/🟦️.ts";
import { type ChangeCaseLabelDe, parseChangeCaseLabelDe } from "./🏷️change-case-label-de/🧬️schema/🟦️.ts";
import { type ChangeCaseLabelEn, parseChangeCaseLabelEn } from "./🏷️change-case-label-en/🧬️schema/🟦️.ts";
import { type ChangeProjectId, parseChangeProjectId } from "./🏷️change-project-id/🧬️schema/🟦️.ts";
import { type ChangeSupervisionLevel, parseChangeSupervisionLevel } from "./👁️change-supervision-level/🧬️schema/🟦️.ts";
import { type InsertAccidental, parseInsertAccidental } from "./💣insert-accidental/🧬️schema/🟦️.ts";
import { type ChangeAccidentalAd, parseChangeAccidentalAd } from "./💥️change-accidental-ad/🧬️schema/🟦️.ts";
import { type ChangeDesignWorkingLifeCategory, parseChangeDesignWorkingLifeCategory } from "./📅change-design-working-life-category/🧬️schema/🟦️.ts";
import { type ChangeDesignWorkingLifeYears, parseChangeDesignWorkingLifeYears } from "./📆change-design-working-life-years/🧬️schema/🟦️.ts";
import { type ChangeCaseEntryId, parseChangeCaseEntryId } from "./📇️change-case-entry-id/🧬️schema/🟦️.ts";
import { type ChangePlacementX, parseChangePlacementX } from "./📍️change-placement-x/🧬️schema/🟦️.ts";
import { type ChangePlacementY, parseChangePlacementY } from "./📍️change-placement-y/🧬️schema/🟦️.ts";
import { type ChangePlacementYaw, parseChangePlacementYaw } from "./📍️change-placement-yaw/🧬️schema/🟦️.ts";
import { type ChangePlacementZ, parseChangePlacementZ } from "./📍️change-placement-z/🧬️schema/🟦️.ts";
import { type InsertPlacement, parseInsertPlacement } from "./📍️insert-placement/🧬️schema/🟦️.ts";
import { type RemovePlacement, parseRemovePlacement } from "./📍️remove-placement/🧬️schema/🟦️.ts";
import { type InsertEffect, parseInsertEffect } from "./📎insert-effect/🧬️schema/🟦️.ts";
import { type ChangeBetaComputed, parseChangeBetaComputed } from "./📐change-beta-computed/🧬️schema/🟦️.ts";
import { type RemoveVariable, parseRemoveVariable } from "./📤remove-variable/🧬️schema/🟦️.ts";
import { type InsertVariable, parseInsertVariable } from "./📥insert-variable/🧬️schema/🟦️.ts";
import { type ChangeFatigueCycles, parseChangeFatigueCycles } from "./🔄️change-fatigue-cycles/🧬️schema/🟦️.ts";
import { type ChangeFatigueDetailCategory, parseChangeFatigueDetailCategory } from "./🔄️change-fatigue-detail-category/🧬️schema/🟦️.ts";
import { type ChangeFatigueMemberId, parseChangeFatigueMemberId } from "./🔄️change-fatigue-member-id/🧬️schema/🟦️.ts";
import { type ChangeFatigueStressRange, parseChangeFatigueStressRange } from "./🔄️change-fatigue-stress-range/🧬️schema/🟦️.ts";
import { type InsertFatigueSpectrum, parseInsertFatigueSpectrum } from "./🔄️insert-fatigue-spectrum/🧬️schema/🟦️.ts";
import { type RemoveFatigueSpectrum, parseRemoveFatigueSpectrum } from "./🔄️remove-fatigue-spectrum/🧬️schema/🟦️.ts";
import { type ChangeInspectionLevel, parseChangeInspectionLevel } from "./🔍change-inspection-level/🧬️schema/🟦️.ts";
import { type ChangeEffectActionId, parseChangeEffectActionId } from "./🔗️change-effect-action-id/🧬️schema/🟦️.ts";
import { type ChangeEffectInfluence, parseChangeEffectInfluence } from "./🔗️change-effect-influence/🧬️schema/🟦️.ts";
import { type ChangeEffectMemberId, parseChangeEffectMemberId } from "./🔗️change-effect-member-id/🧬️schema/🟦️.ts";
import { type ChangeCaseNumber, parseChangeCaseNumber } from "./🔢️change-case-number/🧬️schema/🟦️.ts";
import { type ChangeCaseText, parseChangeCaseText } from "./🔤️change-case-text/🧬️schema/🟦️.ts";
import { type InsertMember, parseInsertMember } from "./🔩insert-member/🧬️schema/🟦️.ts";
import { type RemoveSeismic, parseRemoveSeismic } from "./🕳️remove-seismic/🧬️schema/🟦️.ts";
import { type ChangeReliabilityMemberId, parseChangeReliabilityMemberId } from "./🚦️change-reliability-member-id/🧬️schema/🟦️.ts";
import { type ChangeTrafficGroupGroup, parseChangeTrafficGroupGroup } from "./🚦️change-traffic-group-group/🧬️schema/🟦️.ts";
import { type ChangeTrafficGroupVariableId, parseChangeTrafficGroupVariableId } from "./🚦️change-traffic-group-variable-id/🧬️schema/🟦️.ts";
import { type InsertTrafficGroup, parseInsertTrafficGroup } from "./🚦️insert-traffic-group/🧬️schema/🟦️.ts";
import { type RemoveTrafficGroup, parseRemoveTrafficGroup } from "./🚦️remove-traffic-group/🧬️schema/🟦️.ts";
import { type ChangeTestConversionFactor, parseChangeTestConversionFactor } from "./🧪️change-test-conversion-factor/🧬️schema/🟦️.ts";
import { type ChangeTestDistribution, parseChangeTestDistribution } from "./🧪️change-test-distribution/🧬️schema/🟦️.ts";
import { type ChangeTestMean, parseChangeTestMean } from "./🧪️change-test-mean/🧬️schema/🟦️.ts";
import { type ChangeTestPairs, parseChangeTestPairs } from "./🧪️change-test-pairs/🧬️schema/🟦️.ts";
import { type ChangeTestQuantity, parseChangeTestQuantity } from "./🧪️change-test-quantity/🧬️schema/🟦️.ts";
import { type ChangeTestRequired, parseChangeTestRequired } from "./🧪️change-test-required/🧬️schema/🟦️.ts";
import { type ChangeTestSampleSize, parseChangeTestSampleSize } from "./🧪️change-test-sample-size/🧬️schema/🟦️.ts";
import { type ChangeTestStdDev, parseChangeTestStdDev } from "./🧪️change-test-std-dev/🧬️schema/🟦️.ts";
import { type ChangeTestTheoryVariation, parseChangeTestTheoryVariation } from "./🧪️change-test-theory-variation/🧬️schema/🟦️.ts";
import { type ChangeTestVariationKnown, parseChangeTestVariationKnown } from "./🧪️change-test-variation-known/🧬️schema/🟦️.ts";
import { type InsertTestSeries, parseInsertTestSeries } from "./🧪️insert-test-series/🧬️schema/🟦️.ts";
import { type RemoveTestSeries, parseRemoveTestSeries } from "./🧪️remove-test-series/🧬️schema/🟦️.ts";
import { type RemoveAccidental, parseRemoveAccidental } from "./🧯remove-accidental/🧬️schema/🟦️.ts";
import { type RemoveMember, parseRemoveMember } from "./🪚remove-member/🧬️schema/🟦️.ts";

export type En1990Mutation =
  | ChangeAnnex
  | ChangeProjectId
  | ChangeAltitudeM
  | ChangeConsequenceClass
  | ChangeReliabilityClass
  | ChangeDesignWorkingLifeCategory
  | ChangeDesignWorkingLifeYears
  | ChangeReferencePeriodYears
  | ChangeSupervisionLevel
  | ChangeInspectionLevel
  | ChangeBetaComputed
  | RemoveEffect
  | RemoveMember
  | RemoveSeismic
  | RemoveAccidental
  | RemoveVariable
  | RemovePermanent
  | InsertEffect
  | InsertMember
  | InsertSeismic
  | InsertAccidental
  | InsertVariable
  | InsertPermanent
  | ChangePermanentKind
  | ChangePermanentGk
  | ChangeVariableCategory
  | ChangeVariableQk
  | ChangeVariableGroup
  | ChangeAccidentalAd
  | ChangeSeismicAEk
  | ChangeSeismicImportanceClass
  | ChangeMemberLabelEn
  | ChangeMemberLabelDe
  | ChangeMemberRdStr
  | ChangeMemberRdGeo
  | ChangeMemberRdEquStab
  | ChangeMemberRdEquDestab
  | ChangeMemberRdFat
  | ChangeMemberSpan
  | ChangeMemberPrecamber
  | ChangeMemberDeflectionW1
  | ChangeMemberDeflectionW2
  | ChangeMemberDeflectionW3
  | ChangeMemberDeflLimitRatio
  | ChangeMemberDeflLimitRatioW3
  | ChangeMemberSlsLimitChar
  | ChangeMemberSlsLimitFreq
  | ChangeMemberSlsLimitQp
  | ChangeMemberVibrationFrequency
  | ChangeMemberVibFreqMin
  | ChangeBridgeMemberId
  | ChangeBridgeDeckAcceleration
  | ChangeBridgeDeckAccelLimit
  | ChangeBridgeDeckTwist
  | ChangeBridgeDeckTwistLimit
  | ChangeBridgeDeflection
  | ChangeBridgeDeflectionLimit
  | ChangeEffectMemberId
  | ChangeEffectActionId
  | ChangeEffectInfluence
  | ChangeFatigueMemberId
  | ChangeFatigueDetailCategory
  | ChangeFatigueStressRange
  | ChangeFatigueCycles
  | ChangeReliabilityResistanceDist
  | ChangeReliabilityResistanceMean
  | ChangeReliabilityResistanceSd
  | ChangeReliabilityActionDist
  | ChangeReliabilityActionMean
  | ChangeReliabilityActionSd
  | ChangeTestQuantity
  | ChangeTestPairs
  | ChangeTestDistribution
  | ChangeTestSampleSize
  | ChangeTestMean
  | ChangeTestStdDev
  | ChangeTestVariationKnown
  | ChangeTestConversionFactor
  | ChangeTestRequired
  | ChangeTestTheoryVariation
  | ChangePlacementX
  | ChangePlacementY
  | ChangePlacementZ
  | ChangePlacementYaw
  | ChangeStructureKind
  | InsertBridgeSls
  | RemoveBridgeSls
  | InsertFatigueSpectrum
  | RemoveFatigueSpectrum
  | InsertReliabilityAnalysis
  | RemoveReliabilityAnalysis
  | InsertTestSeries
  | RemoveTestSeries
  | InsertPlacement
  | RemovePlacement
  | InsertCase
  | RemoveCase
  | ChangeCaseNumber
  | ChangeCaseText
  | ChangeCaseLabelEn
  | ChangeCaseLabelDe
  | ChangeCaseEntryId
  | ChangeMemberFrameHeight
  | ChangeMemberFrameDrift
  | ChangeMemberStoreyHeight
  | ChangeMemberStoreyDrift
  | ChangeBridgeTrackGauge
  | ChangeBridgeLineSpeed
  | ChangeBridgeBallastDepth
  | ChangeBridgeDeckMass
  | ChangeBridgeNaturalFrequency
  | ChangeBridgeEndRotation
  | ChangeBridgeBasicWindSpeed
  | ChangeMemberMaterialStrengthK
  | ChangeMemberMaterialConvFactor
  | ChangeMemberMatPartialFactor
  | ChangeMemberMatDesignStrength
  | ChangeMemberNominalDimension
  | ChangeMemberDimensionTolerance
  | ChangeMemberDesignDimension
  | ChangeMemberMemberAlphaCr
  | ChangeMemberSecondOrderAnalysis
  | ChangeBridgeDampingRatio
  | ChangeFactorOverrideFactor
  | ChangeFactorOverrideTarget
  | ChangeFactorOverrideValue
  | ChangeFactorOverrideJustif
  | InsertFactorOverride
  | RemoveFactorOverride
  | ChangeTrafficGroupGroup
  | ChangeTrafficGroupVariableId
  | ChangeReliabilityMemberId
  | InsertTrafficGroup
  | RemoveTrafficGroup;

export const parseEn1990Mutation: NormWireReader<En1990Mutation> = normWireTagged<En1990Mutation, "mutation">("mutation", {
  changeAnnex: parseChangeAnnex,
  changeProjectId: parseChangeProjectId,
  changeAltitudeM: parseChangeAltitudeM,
  changeConsequenceClass: parseChangeConsequenceClass,
  changeReliabilityClass: parseChangeReliabilityClass,
  changeDesignWorkingLifeCategory: parseChangeDesignWorkingLifeCategory,
  changeDesignWorkingLifeYears: parseChangeDesignWorkingLifeYears,
  changeReferencePeriodYears: parseChangeReferencePeriodYears,
  changeSupervisionLevel: parseChangeSupervisionLevel,
  changeInspectionLevel: parseChangeInspectionLevel,
  changeBetaComputed: parseChangeBetaComputed,
  removeEffect: parseRemoveEffect,
  removeMember: parseRemoveMember,
  removeSeismic: parseRemoveSeismic,
  removeAccidental: parseRemoveAccidental,
  removeVariable: parseRemoveVariable,
  removePermanent: parseRemovePermanent,
  insertEffect: parseInsertEffect,
  insertMember: parseInsertMember,
  insertSeismic: parseInsertSeismic,
  insertAccidental: parseInsertAccidental,
  insertVariable: parseInsertVariable,
  insertPermanent: parseInsertPermanent,
  changePermanentKind: parseChangePermanentKind,
  changePermanentGk: parseChangePermanentGk,
  changeVariableCategory: parseChangeVariableCategory,
  changeVariableQk: parseChangeVariableQk,
  changeVariableGroup: parseChangeVariableGroup,
  changeAccidentalAd: parseChangeAccidentalAd,
  changeSeismicAEk: parseChangeSeismicAEk,
  changeSeismicImportanceClass: parseChangeSeismicImportanceClass,
  changeMemberLabelEn: parseChangeMemberLabelEn,
  changeMemberLabelDe: parseChangeMemberLabelDe,
  changeMemberRdStr: parseChangeMemberRdStr,
  changeMemberRdGeo: parseChangeMemberRdGeo,
  changeMemberRdEquStab: parseChangeMemberRdEquStab,
  changeMemberRdEquDestab: parseChangeMemberRdEquDestab,
  changeMemberRdFat: parseChangeMemberRdFat,
  changeMemberSpan: parseChangeMemberSpan,
  changeMemberPrecamber: parseChangeMemberPrecamber,
  changeMemberDeflectionW1: parseChangeMemberDeflectionW1,
  changeMemberDeflectionW2: parseChangeMemberDeflectionW2,
  changeMemberDeflectionW3: parseChangeMemberDeflectionW3,
  changeMemberDeflLimitRatio: parseChangeMemberDeflLimitRatio,
  changeMemberDeflLimitRatioW3: parseChangeMemberDeflLimitRatioW3,
  changeMemberSlsLimitChar: parseChangeMemberSlsLimitChar,
  changeMemberSlsLimitFreq: parseChangeMemberSlsLimitFreq,
  changeMemberSlsLimitQp: parseChangeMemberSlsLimitQp,
  changeMemberVibrationFrequency: parseChangeMemberVibrationFrequency,
  changeMemberVibFreqMin: parseChangeMemberVibFreqMin,
  changeBridgeMemberId: parseChangeBridgeMemberId,
  changeBridgeDeckAcceleration: parseChangeBridgeDeckAcceleration,
  changeBridgeDeckAccelLimit: parseChangeBridgeDeckAccelLimit,
  changeBridgeDeckTwist: parseChangeBridgeDeckTwist,
  changeBridgeDeckTwistLimit: parseChangeBridgeDeckTwistLimit,
  changeBridgeDeflection: parseChangeBridgeDeflection,
  changeBridgeDeflectionLimit: parseChangeBridgeDeflectionLimit,
  changeEffectMemberId: parseChangeEffectMemberId,
  changeEffectActionId: parseChangeEffectActionId,
  changeEffectInfluence: parseChangeEffectInfluence,
  changeFatigueMemberId: parseChangeFatigueMemberId,
  changeFatigueDetailCategory: parseChangeFatigueDetailCategory,
  changeFatigueStressRange: parseChangeFatigueStressRange,
  changeFatigueCycles: parseChangeFatigueCycles,
  changeReliabilityResistanceDist: parseChangeReliabilityResistanceDist,
  changeReliabilityResistanceMean: parseChangeReliabilityResistanceMean,
  changeReliabilityResistanceSd: parseChangeReliabilityResistanceSd,
  changeReliabilityActionDist: parseChangeReliabilityActionDist,
  changeReliabilityActionMean: parseChangeReliabilityActionMean,
  changeReliabilityActionSd: parseChangeReliabilityActionSd,
  changeTestQuantity: parseChangeTestQuantity,
  changeTestPairs: parseChangeTestPairs,
  changeTestDistribution: parseChangeTestDistribution,
  changeTestSampleSize: parseChangeTestSampleSize,
  changeTestMean: parseChangeTestMean,
  changeTestStdDev: parseChangeTestStdDev,
  changeTestVariationKnown: parseChangeTestVariationKnown,
  changeTestConversionFactor: parseChangeTestConversionFactor,
  changeTestRequired: parseChangeTestRequired,
  changeTestTheoryVariation: parseChangeTestTheoryVariation,
  changePlacementX: parseChangePlacementX,
  changePlacementY: parseChangePlacementY,
  changePlacementZ: parseChangePlacementZ,
  changePlacementYaw: parseChangePlacementYaw,
  changeStructureKind: parseChangeStructureKind,
  insertBridgeSls: parseInsertBridgeSls,
  removeBridgeSls: parseRemoveBridgeSls,
  insertFatigueSpectrum: parseInsertFatigueSpectrum,
  removeFatigueSpectrum: parseRemoveFatigueSpectrum,
  insertReliabilityAnalysis: parseInsertReliabilityAnalysis,
  removeReliabilityAnalysis: parseRemoveReliabilityAnalysis,
  insertTestSeries: parseInsertTestSeries,
  removeTestSeries: parseRemoveTestSeries,
  insertPlacement: parseInsertPlacement,
  removePlacement: parseRemovePlacement,
  insertCase: parseInsertCase,
  removeCase: parseRemoveCase,
  changeCaseNumber: parseChangeCaseNumber,
  changeCaseText: parseChangeCaseText,
  changeCaseLabelEn: parseChangeCaseLabelEn,
  changeCaseLabelDe: parseChangeCaseLabelDe,
  changeCaseEntryId: parseChangeCaseEntryId,
  changeMemberFrameHeight: parseChangeMemberFrameHeight,
  changeMemberFrameDrift: parseChangeMemberFrameDrift,
  changeMemberStoreyHeight: parseChangeMemberStoreyHeight,
  changeMemberStoreyDrift: parseChangeMemberStoreyDrift,
  changeBridgeTrackGauge: parseChangeBridgeTrackGauge,
  changeBridgeLineSpeed: parseChangeBridgeLineSpeed,
  changeBridgeBallastDepth: parseChangeBridgeBallastDepth,
  changeBridgeDeckMass: parseChangeBridgeDeckMass,
  changeBridgeNaturalFrequency: parseChangeBridgeNaturalFrequency,
  changeBridgeEndRotation: parseChangeBridgeEndRotation,
  changeBridgeBasicWindSpeed: parseChangeBridgeBasicWindSpeed,
  changeMemberMaterialStrengthK: parseChangeMemberMaterialStrengthK,
  changeMemberMaterialConvFactor: parseChangeMemberMaterialConvFactor,
  changeMemberMatPartialFactor: parseChangeMemberMatPartialFactor,
  changeMemberMatDesignStrength: parseChangeMemberMatDesignStrength,
  changeMemberNominalDimension: parseChangeMemberNominalDimension,
  changeMemberDimensionTolerance: parseChangeMemberDimensionTolerance,
  changeMemberDesignDimension: parseChangeMemberDesignDimension,
  changeMemberMemberAlphaCr: parseChangeMemberMemberAlphaCr,
  changeMemberSecondOrderAnalysis: parseChangeMemberSecondOrderAnalysis,
  changeBridgeDampingRatio: parseChangeBridgeDampingRatio,
  changeFactorOverrideFactor: parseChangeFactorOverrideFactor,
  changeFactorOverrideTarget: parseChangeFactorOverrideTarget,
  changeFactorOverrideValue: parseChangeFactorOverrideValue,
  changeFactorOverrideJustif: parseChangeFactorOverrideJustif,
  insertFactorOverride: parseInsertFactorOverride,
  removeFactorOverride: parseRemoveFactorOverride,
  changeTrafficGroupGroup: parseChangeTrafficGroupGroup,
  changeTrafficGroupVariableId: parseChangeTrafficGroupVariableId,
  changeReliabilityMemberId: parseChangeReliabilityMemberId,
  insertTrafficGroup: parseInsertTrafficGroup,
  removeTrafficGroup: parseRemoveTrafficGroup,
});
