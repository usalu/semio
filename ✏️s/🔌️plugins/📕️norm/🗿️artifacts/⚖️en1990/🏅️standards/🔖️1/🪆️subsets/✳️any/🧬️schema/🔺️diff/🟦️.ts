/** 🔺️ `En1990Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireAny, normWireArray, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireOptional, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type CaseNumber, parseCaseNumber, type CaseRow, parseCaseRow, type CaseText, parseCaseText, type En1990AccidentalAction, parseEn1990AccidentalAction, type En1990BridgeSls, parseEn1990BridgeSls, type En1990DistributionKind, parseEn1990DistributionKind, type En1990FactorOverride, parseEn1990FactorOverride, type En1990FatigueSpectrum, parseEn1990FatigueSpectrum, type En1990ImportanceClass, parseEn1990ImportanceClass, type En1990InspectionLevel, parseEn1990InspectionLevel, type En1990Member, parseEn1990Member, type En1990MemberEffect, parseEn1990MemberEffect, type En1990PermanentAction, parseEn1990PermanentAction, type En1990PermanentKind, parseEn1990PermanentKind, type En1990Placement, parseEn1990Placement, type En1990ReliabilityAnalysis, parseEn1990ReliabilityAnalysis, type En1990SeismicAction, parseEn1990SeismicAction, type En1990StructureKind, parseEn1990StructureKind, type En1990SupervisionLevel, parseEn1990SupervisionLevel, type En1990TestDistribution, parseEn1990TestDistribution, type En1990TestSeries, parseEn1990TestSeries, type En1990TrafficGroup, parseEn1990TrafficGroup, type En1990VariableAction, parseEn1990VariableAction } from "../📸️snapshot/🟦️.ts";

export interface En1990Diff {
  /** @state artifact */
  annex?: ("En" | "De") | null;
  /** @state artifact */
  projectId?: string | null;
  /** @state artifact */
  structureKind?: En1990StructureKind | null;
  /** @state artifact */
  altitudeM?: number | null;
  /** @state artifact */
  consequenceClass?: number | null;
  /** @state artifact */
  reliabilityClass?: ("RC1" | "RC2" | "RC3") | null;
  /** @state artifact */
  designWorkingLifeCategory?: number | null;
  /** @state artifact */
  designWorkingLifeYears?: number | null;
  /** @state artifact */
  referencePeriodYears?: number | null;
  /** @state artifact */
  supervisionLevel?: En1990SupervisionLevel | null;
  /** @state artifact */
  inspectionLevel?: En1990InspectionLevel | null;
  /** @state artifact */
  betaComputed?: number | null;
  /** @state artifact */
  permanents: En1990PermanentDelta;
  /** @state artifact */
  variables: En1990VariableDelta;
  /** @state artifact */
  accidentals: En1990AccidentalDelta;
  /** @state artifact */
  seismics: En1990SeismicDelta;
  /** @state artifact */
  members: En1990MemberDelta;
  /** @state artifact */
  bridgeSls: En1990BridgeSlsDelta;
  /** @state artifact */
  effects: En1990EffectEdit[];
  /** @state artifact */
  fatigueSpectra: En1990FatigueSpectrumDelta;
  /** @state artifact */
  reliabilityAnalyses: En1990ReliabilityAnalysisDelta;
  /** @state artifact */
  testSeries: En1990TestSeriesDelta;
  /** @state artifact */
  placements: En1990PlacementDelta;
  /** @state artifact */
  cases: CaseRowDelta;
  /** @state artifact */
  factorOverrides: En1990FactorOverrideDelta;
  /** @state artifact */
  trafficGroups: En1990TrafficGroupDelta;
}

export interface En1990PermanentDelta {
  removed: En1990PermanentRemoval[];
  inserted: En1990PermanentInsertion[];
  moved: En1990PermanentRelocation[];
  modified: En1990PermanentModification[];
}

export interface En1990PermanentRemoval {
  id: string;
  index: number;
}

export interface En1990PermanentInsertion {
  index: number;
  row: En1990PermanentAction;
}

export interface En1990PermanentRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1990PermanentModification {
  id: string;
  patch: En1990PermanentPatch;
}

export interface En1990PermanentPatch {
  kind?: En1990PermanentKind | null;
  gk?: number | null;
}

export interface En1990VariableDelta {
  removed: En1990VariableRemoval[];
  inserted: En1990VariableInsertion[];
  moved: En1990VariableRelocation[];
  modified: En1990VariableModification[];
}

export interface En1990VariableRemoval {
  id: string;
  index: number;
}

export interface En1990VariableInsertion {
  index: number;
  row: En1990VariableAction;
}

export interface En1990VariableRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1990VariableModification {
  id: string;
  patch: En1990VariablePatch;
}

export interface En1990VariablePatch {
  category?: string | null;
  qk?: number | null;
  group?: string | null;
}

export interface En1990AccidentalDelta {
  removed: En1990AccidentalRemoval[];
  inserted: En1990AccidentalInsertion[];
  moved: En1990AccidentalRelocation[];
  modified: En1990AccidentalModification[];
}

export interface En1990AccidentalRemoval {
  id: string;
  index: number;
}

export interface En1990AccidentalInsertion {
  index: number;
  row: En1990AccidentalAction;
}

export interface En1990AccidentalRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1990AccidentalModification {
  id: string;
  patch: En1990AccidentalPatch;
}

export interface En1990AccidentalPatch {
  ad?: number | null;
}

export interface En1990SeismicDelta {
  removed: En1990SeismicRemoval[];
  inserted: En1990SeismicInsertion[];
  moved: En1990SeismicRelocation[];
  modified: En1990SeismicModification[];
}

export interface En1990SeismicRemoval {
  id: string;
  index: number;
}

export interface En1990SeismicInsertion {
  index: number;
  row: En1990SeismicAction;
}

export interface En1990SeismicRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1990SeismicModification {
  id: string;
  patch: En1990SeismicPatch;
}

export interface En1990SeismicPatch {
  aEk?: number | null;
  importanceClass?: En1990ImportanceClass | null;
}

export interface En1990MemberDelta {
  removed: En1990MemberRemoval[];
  inserted: En1990MemberInsertion[];
  moved: En1990MemberRelocation[];
  modified: En1990MemberModification[];
}

export interface En1990MemberRemoval {
  id: string;
  index: number;
}

export interface En1990MemberInsertion {
  index: number;
  row: En1990Member;
}

export interface En1990MemberRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1990MemberModification {
  id: string;
  patch: En1990MemberPatch;
}

export interface En1990MemberPatch {
  labelEn?: string | null;
  labelDe?: string | null;
  rdStr?: number | null;
  rdGeo?: number | null;
  rdEquStab?: number | null;
  rdEquDestab?: number | null;
  rdFat?: number | null;
  span?: number | null;
  precamber?: number | null;
  deflectionW1?: number | null;
  deflectionW2?: number | null;
  deflectionW3?: number | null;
  deflectionLimitRatio?: number | null;
  deflectionLimitRatioW3?: number | null;
  slsLimitChar?: number | null;
  slsLimitFreq?: number | null;
  slsLimitQp?: number | null;
  vibrationFrequency?: number | null;
  vibrationFrequencyMin?: number | null;
  frameHeight?: number | null;
  frameDrift?: number | null;
  storeyHeight?: number | null;
  storeyDrift?: number | null;
  materialStrengthK?: number | null;
  materialConversionFactor?: number | null;
  materialPartialFactor?: number | null;
  materialDesignStrength?: number | null;
  nominalDimension?: number | null;
  dimensionTolerance?: number | null;
  designDimension?: number | null;
  memberAlphaCr?: number | null;
  secondOrderAnalysis?: number | null;
}

export interface En1990BridgeSlsDelta {
  removed: En1990BridgeSlsRemoval[];
  inserted: En1990BridgeSlsInsertion[];
  moved: En1990BridgeSlsRelocation[];
  modified: En1990BridgeSlsModification[];
}

export interface En1990BridgeSlsRemoval {
  id: string;
  index: number;
}

export interface En1990BridgeSlsInsertion {
  index: number;
  row: En1990BridgeSls;
}

export interface En1990BridgeSlsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1990BridgeSlsModification {
  id: string;
  patch: En1990BridgeSlsPatch;
}

export interface En1990BridgeSlsPatch {
  memberId?: string | null;
  deckAcceleration?: number | null;
  deckAccelerationLimit?: number | null;
  deckTwist?: number | null;
  deckTwistLimit?: number | null;
  bridgeDeflection?: number | null;
  bridgeDeflectionLimit?: number | null;
  trackGauge?: number | null;
  lineSpeed?: number | null;
  ballastDepth?: number | null;
  deckMass?: number | null;
  naturalFrequency?: number | null;
  endRotation?: number | null;
  basicWindSpeed?: number | null;
  dampingRatio?: number | null;
}

export interface En1990FatigueSpectrumDelta {
  removed: En1990FatigueSpectrumRemoval[];
  inserted: En1990FatigueSpectrumInsertion[];
  moved: En1990FatigueSpectrumRelocation[];
  modified: En1990FatigueSpectrumModification[];
}

export interface En1990FatigueSpectrumRemoval {
  id: string;
  index: number;
}

export interface En1990FatigueSpectrumInsertion {
  index: number;
  row: En1990FatigueSpectrum;
}

export interface En1990FatigueSpectrumRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1990FatigueSpectrumModification {
  id: string;
  patch: En1990FatigueSpectrumPatch;
}

export interface En1990FatigueSpectrumPatch {
  memberId?: string | null;
  detailCategory?: number | null;
  stressRange?: number | null;
  cycles?: number | null;
}

export interface En1990ReliabilityAnalysisDelta {
  removed: En1990ReliabilityAnalysisRemoval[];
  inserted: En1990ReliabilityAnalysisInsertion[];
  moved: En1990ReliabilityAnalysisRelocation[];
  modified: En1990ReliabilityAnalysisModification[];
}

export interface En1990ReliabilityAnalysisRemoval {
  id: string;
  index: number;
}

export interface En1990ReliabilityAnalysisInsertion {
  index: number;
  row: En1990ReliabilityAnalysis;
}

export interface En1990ReliabilityAnalysisRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1990ReliabilityAnalysisModification {
  id: string;
  patch: En1990ReliabilityAnalysisPatch;
}

export interface En1990ReliabilityAnalysisPatch {
  resistanceDistribution?: En1990DistributionKind | null;
  resistanceMean?: number | null;
  resistanceSd?: number | null;
  actionDistribution?: En1990DistributionKind | null;
  actionMean?: number | null;
  actionSd?: number | null;
}

export interface En1990TestSeriesDelta {
  removed: En1990TestSeriesRemoval[];
  inserted: En1990TestSeriesInsertion[];
  moved: En1990TestSeriesRelocation[];
  modified: En1990TestSeriesModification[];
}

export interface En1990TestSeriesRemoval {
  id: string;
  index: number;
}

export interface En1990TestSeriesInsertion {
  index: number;
  row: En1990TestSeries;
}

export interface En1990TestSeriesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1990TestSeriesModification {
  id: string;
  patch: En1990TestSeriesPatch;
}

export interface En1990TestSeriesPatch {
  quantity?: string | null;
  distribution?: En1990TestDistribution | null;
  sampleSize?: number | null;
  mean?: number | null;
  stdDev?: number | null;
  variationKnown?: number | null;
  conversionFactor?: number | null;
  required?: number | null;
}

export interface En1990PlacementDelta {
  removed: En1990PlacementRemoval[];
  inserted: En1990PlacementInsertion[];
  moved: En1990PlacementRelocation[];
  modified: En1990PlacementModification[];
}

export interface En1990PlacementRemoval {
  id: string;
  index: number;
}

export interface En1990PlacementInsertion {
  index: number;
  row: En1990Placement;
}

export interface En1990PlacementRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1990PlacementModification {
  id: string;
  patch: En1990PlacementPatch;
}

export interface En1990PlacementPatch {
  x?: number | null;
  y?: number | null;
  z?: number | null;
  yaw?: number | null;
}

export interface En1990FactorOverrideDelta {
  removed: En1990FactorOverrideRemoval[];
  inserted: En1990FactorOverrideInsertion[];
  moved: En1990FactorOverrideRelocation[];
  modified: En1990FactorOverrideModification[];
}

export interface En1990FactorOverrideRemoval {
  id: string;
  index: number;
}

export interface En1990FactorOverrideInsertion {
  index: number;
  row: En1990FactorOverride;
}

export interface En1990FactorOverrideRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1990FactorOverrideModification {
  id: string;
  patch: En1990FactorOverridePatch;
}

export interface En1990FactorOverridePatch {
  factor?: string | null;
  target?: string | null;
  value?: number | null;
  justification?: string | null;
}

export interface En1990TrafficGroupDelta {
  removed?: En1990TrafficGroupRemoval[];
  inserted?: En1990TrafficGroupInsertion[];
  moved?: En1990TrafficGroupRelocation[];
  modified?: En1990TrafficGroupModification[];
}

export interface En1990TrafficGroupRemoval {
  id: string;
  index: number;
}

export interface En1990TrafficGroupInsertion {
  index: number;
  row: En1990TrafficGroup;
}

export interface En1990TrafficGroupRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1990TrafficGroupModification {
  id: string;
  patch: En1990TrafficGroupPatch;
}

export interface En1990TrafficGroupPatch {
  group?: ("GR1A" | "GR1B" | "GR2" | "GR3" | "GR4" | "GR5" | "GR6") | null;
  variableId?: string | null;
}

export type En1990RowOp = "Insert" | "Remove" | "Replace";

export interface En1990EffectEdit {
  op: En1990RowOp;
  index: number;
  id: string;
  value?: En1990MemberEffect | null;
}

export interface CaseNumberPatch {
  value?: number | null;
}

export interface CaseNumberRemoval {
  id: string;
  index: number;
}

export interface CaseNumberInsertion {
  index: number;
  row: CaseNumber;
}

export interface CaseNumberRelocation {
  id: string;
  from: number;
  to: number;
}

export interface CaseNumberModification {
  id: string;
  patch: CaseNumberPatch;
}

export interface CaseNumberDelta {
  removed: CaseNumberRemoval[];
  inserted: CaseNumberInsertion[];
  moved: CaseNumberRelocation[];
  modified: CaseNumberModification[];
}

export interface CaseTextPatch {
  value?: string | null;
}

export interface CaseTextRemoval {
  id: string;
  index: number;
}

export interface CaseTextInsertion {
  index: number;
  row: CaseText;
}

export interface CaseTextRelocation {
  id: string;
  from: number;
  to: number;
}

export interface CaseTextModification {
  id: string;
  patch: CaseTextPatch;
}

export interface CaseTextDelta {
  removed: CaseTextRemoval[];
  inserted: CaseTextInsertion[];
  moved: CaseTextRelocation[];
  modified: CaseTextModification[];
}

export interface CaseRowPatch {
  labelEn?: string | null;
  labelDe?: string | null;
  entryId?: string | null;
  numbers: CaseNumberDelta;
  texts: CaseTextDelta;
}

export interface CaseRowRemoval {
  id: string;
  index: number;
}

export interface CaseRowInsertion {
  index: number;
  row: CaseRow;
}

export interface CaseRowRelocation {
  id: string;
  from: number;
  to: number;
}

export interface CaseRowModification {
  id: string;
  patch: CaseRowPatch;
}

export interface CaseRowDelta {
  removed: CaseRowRemoval[];
  inserted: CaseRowInsertion[];
  moved: CaseRowRelocation[];
  modified: CaseRowModification[];
}

export const parseEn1990Diff: NormWireReader<En1990Diff> = normWireObject<En1990Diff>({ annex: normWireOptional(normWireNullable(normWireLiteral("En", "De"))), projectId: normWireOptional(normWireNullable(normWireString)), structureKind: normWireOptional(normWireNullable(parseEn1990StructureKind)), altitudeM: normWireOptional(normWireNullable(normWireNumber)), consequenceClass: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), reliabilityClass: normWireOptional(normWireNullable(normWireLiteral("RC1", "RC2", "RC3"))), designWorkingLifeCategory: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), designWorkingLifeYears: normWireOptional(normWireNullable(normWireNumber)), referencePeriodYears: normWireOptional(normWireNullable(normWireNumber)), supervisionLevel: normWireOptional(normWireNullable(parseEn1990SupervisionLevel)), inspectionLevel: normWireOptional(normWireNullable(parseEn1990InspectionLevel)), betaComputed: normWireOptional(normWireNullable(normWireNumber)), permanents: normWireRequired(normWireAny<[En1990PermanentDelta]>(normWireRef(() => parseEn1990PermanentDelta))), variables: normWireRequired(normWireAny<[En1990VariableDelta]>(normWireRef(() => parseEn1990VariableDelta))), accidentals: normWireRequired(normWireAny<[En1990AccidentalDelta]>(normWireRef(() => parseEn1990AccidentalDelta))), seismics: normWireRequired(normWireAny<[En1990SeismicDelta]>(normWireRef(() => parseEn1990SeismicDelta))), members: normWireRequired(normWireAny<[En1990MemberDelta]>(normWireRef(() => parseEn1990MemberDelta))), bridgeSls: normWireRequired(normWireAny<[En1990BridgeSlsDelta]>(normWireRef(() => parseEn1990BridgeSlsDelta))), effects: normWireRequired(normWireArray(normWireRef(() => parseEn1990EffectEdit))), fatigueSpectra: normWireRequired(normWireAny<[En1990FatigueSpectrumDelta]>(normWireRef(() => parseEn1990FatigueSpectrumDelta))), reliabilityAnalyses: normWireRequired(normWireAny<[En1990ReliabilityAnalysisDelta]>(normWireRef(() => parseEn1990ReliabilityAnalysisDelta))), testSeries: normWireRequired(normWireAny<[En1990TestSeriesDelta]>(normWireRef(() => parseEn1990TestSeriesDelta))), placements: normWireRequired(normWireAny<[En1990PlacementDelta]>(normWireRef(() => parseEn1990PlacementDelta))), cases: normWireRequired(normWireRef(() => parseCaseRowDelta)), factorOverrides: normWireRequired(normWireAny<[En1990FactorOverrideDelta]>(normWireRef(() => parseEn1990FactorOverrideDelta))), trafficGroups: normWireRequired(normWireAny<[En1990TrafficGroupDelta]>(normWireRef(() => parseEn1990TrafficGroupDelta))) });
export const parseEn1990PermanentDelta: NormWireReader<En1990PermanentDelta> = normWireObject<En1990PermanentDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1990PermanentRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1990PermanentInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1990PermanentRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1990PermanentModification))) });
export const parseEn1990PermanentRemoval: NormWireReader<En1990PermanentRemoval> = normWireObject<En1990PermanentRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990PermanentInsertion: NormWireReader<En1990PermanentInsertion> = normWireObject<En1990PermanentInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1990PermanentAction) });
export const parseEn1990PermanentRelocation: NormWireReader<En1990PermanentRelocation> = normWireObject<En1990PermanentRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990PermanentModification: NormWireReader<En1990PermanentModification> = normWireObject<En1990PermanentModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1990PermanentPatch)) });
export const parseEn1990PermanentPatch: NormWireReader<En1990PermanentPatch> = normWireObject<En1990PermanentPatch>({ kind: normWireOptional(normWireNullable(parseEn1990PermanentKind)), gk: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1990VariableDelta: NormWireReader<En1990VariableDelta> = normWireObject<En1990VariableDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1990VariableRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1990VariableInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1990VariableRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1990VariableModification))) });
export const parseEn1990VariableRemoval: NormWireReader<En1990VariableRemoval> = normWireObject<En1990VariableRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990VariableInsertion: NormWireReader<En1990VariableInsertion> = normWireObject<En1990VariableInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1990VariableAction) });
export const parseEn1990VariableRelocation: NormWireReader<En1990VariableRelocation> = normWireObject<En1990VariableRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990VariableModification: NormWireReader<En1990VariableModification> = normWireObject<En1990VariableModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1990VariablePatch)) });
export const parseEn1990VariablePatch: NormWireReader<En1990VariablePatch> = normWireObject<En1990VariablePatch>({ category: normWireOptional(normWireNullable(normWireString)), qk: normWireOptional(normWireNullable(normWireNumber)), group: normWireOptional(normWireNullable(normWireString)) });
export const parseEn1990AccidentalDelta: NormWireReader<En1990AccidentalDelta> = normWireObject<En1990AccidentalDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1990AccidentalRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1990AccidentalInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1990AccidentalRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1990AccidentalModification))) });
export const parseEn1990AccidentalRemoval: NormWireReader<En1990AccidentalRemoval> = normWireObject<En1990AccidentalRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990AccidentalInsertion: NormWireReader<En1990AccidentalInsertion> = normWireObject<En1990AccidentalInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1990AccidentalAction) });
export const parseEn1990AccidentalRelocation: NormWireReader<En1990AccidentalRelocation> = normWireObject<En1990AccidentalRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990AccidentalModification: NormWireReader<En1990AccidentalModification> = normWireObject<En1990AccidentalModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1990AccidentalPatch)) });
export const parseEn1990AccidentalPatch: NormWireReader<En1990AccidentalPatch> = normWireObject<En1990AccidentalPatch>({ ad: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1990SeismicDelta: NormWireReader<En1990SeismicDelta> = normWireObject<En1990SeismicDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1990SeismicRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1990SeismicInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1990SeismicRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1990SeismicModification))) });
export const parseEn1990SeismicRemoval: NormWireReader<En1990SeismicRemoval> = normWireObject<En1990SeismicRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990SeismicInsertion: NormWireReader<En1990SeismicInsertion> = normWireObject<En1990SeismicInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1990SeismicAction) });
export const parseEn1990SeismicRelocation: NormWireReader<En1990SeismicRelocation> = normWireObject<En1990SeismicRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990SeismicModification: NormWireReader<En1990SeismicModification> = normWireObject<En1990SeismicModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1990SeismicPatch)) });
export const parseEn1990SeismicPatch: NormWireReader<En1990SeismicPatch> = normWireObject<En1990SeismicPatch>({ aEk: normWireOptional(normWireNullable(normWireNumber)), importanceClass: normWireOptional(normWireNullable(parseEn1990ImportanceClass)) });
export const parseEn1990MemberDelta: NormWireReader<En1990MemberDelta> = normWireObject<En1990MemberDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1990MemberRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1990MemberInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1990MemberRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1990MemberModification))) });
export const parseEn1990MemberRemoval: NormWireReader<En1990MemberRemoval> = normWireObject<En1990MemberRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990MemberInsertion: NormWireReader<En1990MemberInsertion> = normWireObject<En1990MemberInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1990Member) });
export const parseEn1990MemberRelocation: NormWireReader<En1990MemberRelocation> = normWireObject<En1990MemberRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990MemberModification: NormWireReader<En1990MemberModification> = normWireObject<En1990MemberModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1990MemberPatch)) });
export const parseEn1990MemberPatch: NormWireReader<En1990MemberPatch> = normWireObject<En1990MemberPatch>({ labelEn: normWireOptional(normWireNullable(normWireString)), labelDe: normWireOptional(normWireNullable(normWireString)), rdStr: normWireOptional(normWireNullable(normWireNumber)), rdGeo: normWireOptional(normWireNullable(normWireNumber)), rdEquStab: normWireOptional(normWireNullable(normWireNumber)), rdEquDestab: normWireOptional(normWireNullable(normWireNumber)), rdFat: normWireOptional(normWireNullable(normWireNumber)), span: normWireOptional(normWireNullable(normWireNumber)), precamber: normWireOptional(normWireNullable(normWireNumber)), deflectionW1: normWireOptional(normWireNullable(normWireNumber)), deflectionW2: normWireOptional(normWireNullable(normWireNumber)), deflectionW3: normWireOptional(normWireNullable(normWireNumber)), deflectionLimitRatio: normWireOptional(normWireNullable(normWireNumber)), deflectionLimitRatioW3: normWireOptional(normWireNullable(normWireNumber)), slsLimitChar: normWireOptional(normWireNullable(normWireNumber)), slsLimitFreq: normWireOptional(normWireNullable(normWireNumber)), slsLimitQp: normWireOptional(normWireNullable(normWireNumber)), vibrationFrequency: normWireOptional(normWireNullable(normWireNumber)), vibrationFrequencyMin: normWireOptional(normWireNullable(normWireNumber)), frameHeight: normWireOptional(normWireNullable(normWireNumber)), frameDrift: normWireOptional(normWireNullable(normWireNumber)), storeyHeight: normWireOptional(normWireNullable(normWireNumber)), storeyDrift: normWireOptional(normWireNullable(normWireNumber)), materialStrengthK: normWireOptional(normWireNullable(normWireNumber)), materialConversionFactor: normWireOptional(normWireNullable(normWireNumber)), materialPartialFactor: normWireOptional(normWireNullable(normWireNumber)), materialDesignStrength: normWireOptional(normWireNullable(normWireNumber)), nominalDimension: normWireOptional(normWireNullable(normWireNumber)), dimensionTolerance: normWireOptional(normWireNullable(normWireNumber)), designDimension: normWireOptional(normWireNullable(normWireNumber)), memberAlphaCr: normWireOptional(normWireNullable(normWireNumber)), secondOrderAnalysis: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1990BridgeSlsDelta: NormWireReader<En1990BridgeSlsDelta> = normWireObject<En1990BridgeSlsDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1990BridgeSlsRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1990BridgeSlsInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1990BridgeSlsRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1990BridgeSlsModification))) });
export const parseEn1990BridgeSlsRemoval: NormWireReader<En1990BridgeSlsRemoval> = normWireObject<En1990BridgeSlsRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990BridgeSlsInsertion: NormWireReader<En1990BridgeSlsInsertion> = normWireObject<En1990BridgeSlsInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1990BridgeSls) });
export const parseEn1990BridgeSlsRelocation: NormWireReader<En1990BridgeSlsRelocation> = normWireObject<En1990BridgeSlsRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990BridgeSlsModification: NormWireReader<En1990BridgeSlsModification> = normWireObject<En1990BridgeSlsModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1990BridgeSlsPatch)) });
export const parseEn1990BridgeSlsPatch: NormWireReader<En1990BridgeSlsPatch> = normWireObject<En1990BridgeSlsPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), deckAcceleration: normWireOptional(normWireNullable(normWireNumber)), deckAccelerationLimit: normWireOptional(normWireNullable(normWireNumber)), deckTwist: normWireOptional(normWireNullable(normWireNumber)), deckTwistLimit: normWireOptional(normWireNullable(normWireNumber)), bridgeDeflection: normWireOptional(normWireNullable(normWireNumber)), bridgeDeflectionLimit: normWireOptional(normWireNullable(normWireNumber)), trackGauge: normWireOptional(normWireNullable(normWireNumber)), lineSpeed: normWireOptional(normWireNullable(normWireNumber)), ballastDepth: normWireOptional(normWireNullable(normWireNumber)), deckMass: normWireOptional(normWireNullable(normWireNumber)), naturalFrequency: normWireOptional(normWireNullable(normWireNumber)), endRotation: normWireOptional(normWireNullable(normWireNumber)), basicWindSpeed: normWireOptional(normWireNullable(normWireNumber)), dampingRatio: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1990FatigueSpectrumDelta: NormWireReader<En1990FatigueSpectrumDelta> = normWireObject<En1990FatigueSpectrumDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1990FatigueSpectrumRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1990FatigueSpectrumInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1990FatigueSpectrumRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1990FatigueSpectrumModification))) });
export const parseEn1990FatigueSpectrumRemoval: NormWireReader<En1990FatigueSpectrumRemoval> = normWireObject<En1990FatigueSpectrumRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990FatigueSpectrumInsertion: NormWireReader<En1990FatigueSpectrumInsertion> = normWireObject<En1990FatigueSpectrumInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1990FatigueSpectrum) });
export const parseEn1990FatigueSpectrumRelocation: NormWireReader<En1990FatigueSpectrumRelocation> = normWireObject<En1990FatigueSpectrumRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990FatigueSpectrumModification: NormWireReader<En1990FatigueSpectrumModification> = normWireObject<En1990FatigueSpectrumModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1990FatigueSpectrumPatch)) });
export const parseEn1990FatigueSpectrumPatch: NormWireReader<En1990FatigueSpectrumPatch> = normWireObject<En1990FatigueSpectrumPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), detailCategory: normWireOptional(normWireNullable(normWireNumber)), stressRange: normWireOptional(normWireNullable(normWireNumber)), cycles: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1990ReliabilityAnalysisDelta: NormWireReader<En1990ReliabilityAnalysisDelta> = normWireObject<En1990ReliabilityAnalysisDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1990ReliabilityAnalysisRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1990ReliabilityAnalysisInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1990ReliabilityAnalysisRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1990ReliabilityAnalysisModification))) });
export const parseEn1990ReliabilityAnalysisRemoval: NormWireReader<En1990ReliabilityAnalysisRemoval> = normWireObject<En1990ReliabilityAnalysisRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990ReliabilityAnalysisInsertion: NormWireReader<En1990ReliabilityAnalysisInsertion> = normWireObject<En1990ReliabilityAnalysisInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1990ReliabilityAnalysis) });
export const parseEn1990ReliabilityAnalysisRelocation: NormWireReader<En1990ReliabilityAnalysisRelocation> = normWireObject<En1990ReliabilityAnalysisRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990ReliabilityAnalysisModification: NormWireReader<En1990ReliabilityAnalysisModification> = normWireObject<En1990ReliabilityAnalysisModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1990ReliabilityAnalysisPatch)) });
export const parseEn1990ReliabilityAnalysisPatch: NormWireReader<En1990ReliabilityAnalysisPatch> = normWireObject<En1990ReliabilityAnalysisPatch>({ resistanceDistribution: normWireOptional(normWireNullable(parseEn1990DistributionKind)), resistanceMean: normWireOptional(normWireNullable(normWireNumber)), resistanceSd: normWireOptional(normWireNullable(normWireNumber)), actionDistribution: normWireOptional(normWireNullable(parseEn1990DistributionKind)), actionMean: normWireOptional(normWireNullable(normWireNumber)), actionSd: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1990TestSeriesDelta: NormWireReader<En1990TestSeriesDelta> = normWireObject<En1990TestSeriesDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1990TestSeriesRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1990TestSeriesInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1990TestSeriesRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1990TestSeriesModification))) });
export const parseEn1990TestSeriesRemoval: NormWireReader<En1990TestSeriesRemoval> = normWireObject<En1990TestSeriesRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990TestSeriesInsertion: NormWireReader<En1990TestSeriesInsertion> = normWireObject<En1990TestSeriesInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1990TestSeries) });
export const parseEn1990TestSeriesRelocation: NormWireReader<En1990TestSeriesRelocation> = normWireObject<En1990TestSeriesRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990TestSeriesModification: NormWireReader<En1990TestSeriesModification> = normWireObject<En1990TestSeriesModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1990TestSeriesPatch)) });
export const parseEn1990TestSeriesPatch: NormWireReader<En1990TestSeriesPatch> = normWireObject<En1990TestSeriesPatch>({ quantity: normWireOptional(normWireNullable(normWireString)), distribution: normWireOptional(normWireNullable(parseEn1990TestDistribution)), sampleSize: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), mean: normWireOptional(normWireNullable(normWireNumber)), stdDev: normWireOptional(normWireNullable(normWireNumber)), variationKnown: normWireOptional(normWireNullable(normWireNumber)), conversionFactor: normWireOptional(normWireNullable(normWireNumber)), required: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1990PlacementDelta: NormWireReader<En1990PlacementDelta> = normWireObject<En1990PlacementDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1990PlacementRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1990PlacementInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1990PlacementRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1990PlacementModification))) });
export const parseEn1990PlacementRemoval: NormWireReader<En1990PlacementRemoval> = normWireObject<En1990PlacementRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990PlacementInsertion: NormWireReader<En1990PlacementInsertion> = normWireObject<En1990PlacementInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1990Placement) });
export const parseEn1990PlacementRelocation: NormWireReader<En1990PlacementRelocation> = normWireObject<En1990PlacementRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990PlacementModification: NormWireReader<En1990PlacementModification> = normWireObject<En1990PlacementModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1990PlacementPatch)) });
export const parseEn1990PlacementPatch: NormWireReader<En1990PlacementPatch> = normWireObject<En1990PlacementPatch>({ x: normWireOptional(normWireNullable(normWireNumber)), y: normWireOptional(normWireNullable(normWireNumber)), z: normWireOptional(normWireNullable(normWireNumber)), yaw: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1990FactorOverrideDelta: NormWireReader<En1990FactorOverrideDelta> = normWireObject<En1990FactorOverrideDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1990FactorOverrideRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1990FactorOverrideInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1990FactorOverrideRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1990FactorOverrideModification))) });
export const parseEn1990FactorOverrideRemoval: NormWireReader<En1990FactorOverrideRemoval> = normWireObject<En1990FactorOverrideRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990FactorOverrideInsertion: NormWireReader<En1990FactorOverrideInsertion> = normWireObject<En1990FactorOverrideInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1990FactorOverride) });
export const parseEn1990FactorOverrideRelocation: NormWireReader<En1990FactorOverrideRelocation> = normWireObject<En1990FactorOverrideRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990FactorOverrideModification: NormWireReader<En1990FactorOverrideModification> = normWireObject<En1990FactorOverrideModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1990FactorOverridePatch)) });
export const parseEn1990FactorOverridePatch: NormWireReader<En1990FactorOverridePatch> = normWireObject<En1990FactorOverridePatch>({ factor: normWireOptional(normWireNullable(normWireString)), target: normWireOptional(normWireNullable(normWireString)), value: normWireOptional(normWireNullable(normWireNumber)), justification: normWireOptional(normWireNullable(normWireString)) });
export const parseEn1990TrafficGroupDelta: NormWireReader<En1990TrafficGroupDelta> = normWireObject<En1990TrafficGroupDelta>({ removed: normWireOptional(normWireArray(normWireRef(() => parseEn1990TrafficGroupRemoval))), inserted: normWireOptional(normWireArray(normWireRef(() => parseEn1990TrafficGroupInsertion))), moved: normWireOptional(normWireArray(normWireRef(() => parseEn1990TrafficGroupRelocation))), modified: normWireOptional(normWireArray(normWireRef(() => parseEn1990TrafficGroupModification))) });
export const parseEn1990TrafficGroupRemoval: NormWireReader<En1990TrafficGroupRemoval> = normWireObject<En1990TrafficGroupRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990TrafficGroupInsertion: NormWireReader<En1990TrafficGroupInsertion> = normWireObject<En1990TrafficGroupInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1990TrafficGroup) });
export const parseEn1990TrafficGroupRelocation: NormWireReader<En1990TrafficGroupRelocation> = normWireObject<En1990TrafficGroupRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1990TrafficGroupModification: NormWireReader<En1990TrafficGroupModification> = normWireObject<En1990TrafficGroupModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1990TrafficGroupPatch)) });
export const parseEn1990TrafficGroupPatch: NormWireReader<En1990TrafficGroupPatch> = normWireObject<En1990TrafficGroupPatch>({ group: normWireOptional(normWireNullable(normWireLiteral("GR1A", "GR1B", "GR2", "GR3", "GR4", "GR5", "GR6"))), variableId: normWireOptional(normWireNullable(normWireString)) });
export const parseEn1990RowOp: NormWireReader<En1990RowOp> = normWireLiteral("Insert", "Remove", "Replace");
export const parseEn1990EffectEdit: NormWireReader<En1990EffectEdit> = normWireObject<En1990EffectEdit>({ op: normWireRequired(normWireRef(() => parseEn1990RowOp)), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), id: normWireRequired(normWireString), value: normWireOptional(normWireNullable(parseEn1990MemberEffect)) });
export const parseCaseNumberPatch: NormWireReader<CaseNumberPatch> = normWireObject<CaseNumberPatch>({ value: normWireOptional(normWireNullable(normWireNumber)) });
export const parseCaseNumberRemoval: NormWireReader<CaseNumberRemoval> = normWireObject<CaseNumberRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseNumberInsertion: NormWireReader<CaseNumberInsertion> = normWireObject<CaseNumberInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCaseNumber) });
export const parseCaseNumberRelocation: NormWireReader<CaseNumberRelocation> = normWireObject<CaseNumberRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseNumberModification: NormWireReader<CaseNumberModification> = normWireObject<CaseNumberModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseCaseNumberPatch)) });
export const parseCaseNumberDelta: NormWireReader<CaseNumberDelta> = normWireObject<CaseNumberDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseCaseNumberRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseCaseNumberInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseCaseNumberRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseCaseNumberModification))) });
export const parseCaseTextPatch: NormWireReader<CaseTextPatch> = normWireObject<CaseTextPatch>({ value: normWireOptional(normWireNullable(normWireString)) });
export const parseCaseTextRemoval: NormWireReader<CaseTextRemoval> = normWireObject<CaseTextRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseTextInsertion: NormWireReader<CaseTextInsertion> = normWireObject<CaseTextInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCaseText) });
export const parseCaseTextRelocation: NormWireReader<CaseTextRelocation> = normWireObject<CaseTextRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseTextModification: NormWireReader<CaseTextModification> = normWireObject<CaseTextModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseCaseTextPatch)) });
export const parseCaseTextDelta: NormWireReader<CaseTextDelta> = normWireObject<CaseTextDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseCaseTextRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseCaseTextInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseCaseTextRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseCaseTextModification))) });
export const parseCaseRowPatch: NormWireReader<CaseRowPatch> = normWireObject<CaseRowPatch>({ labelEn: normWireOptional(normWireNullable(normWireString)), labelDe: normWireOptional(normWireNullable(normWireString)), entryId: normWireOptional(normWireNullable(normWireString)), numbers: normWireRequired(normWireRef(() => parseCaseNumberDelta)), texts: normWireRequired(normWireRef(() => parseCaseTextDelta)) });
export const parseCaseRowRemoval: NormWireReader<CaseRowRemoval> = normWireObject<CaseRowRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseRowInsertion: NormWireReader<CaseRowInsertion> = normWireObject<CaseRowInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCaseRow) });
export const parseCaseRowRelocation: NormWireReader<CaseRowRelocation> = normWireObject<CaseRowRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseRowModification: NormWireReader<CaseRowModification> = normWireObject<CaseRowModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseCaseRowPatch)) });
export const parseCaseRowDelta: NormWireReader<CaseRowDelta> = normWireObject<CaseRowDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseCaseRowRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseCaseRowInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseCaseRowRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseCaseRowModification))) });
