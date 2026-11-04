/** 📸️ `En1990Snapshot` wire twin: the persisted snapshot and every record it holds, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface En1990Snapshot {
  /** @state artifact */
  annex: "En" | "De";
  /** @state artifact */
  projectId: string;
  /** @state artifact */
  structureKind: En1990StructureKind;
  /** @state artifact */
  altitudeM: number;
  /** @state artifact */
  consequenceClass: number;
  /** @state artifact */
  reliabilityClass: "RC1" | "RC2" | "RC3";
  /** @state artifact */
  designWorkingLifeCategory: number;
  /** @state artifact */
  designWorkingLifeYears: number;
  /** @state artifact */
  referencePeriodYears: number;
  /** @state artifact */
  supervisionLevel: En1990SupervisionLevel;
  /** @state artifact */
  inspectionLevel: En1990InspectionLevel;
  /** @state artifact */
  betaComputed: number;
  /** @state artifact */
  permanents: En1990PermanentAction[];
  /** @state artifact */
  variables: En1990VariableAction[];
  /** @state artifact */
  accidentals: En1990AccidentalAction[];
  /** @state artifact */
  seismics: En1990SeismicAction[];
  /** @state artifact */
  members: En1990Member[];
  /** @state artifact */
  bridgeSls: En1990BridgeSls[];
  /** @state artifact */
  effects: En1990MemberEffect[];
  /** @state artifact */
  fatigueSpectra: En1990FatigueSpectrum[];
  /** @state artifact */
  reliabilityAnalyses: En1990ReliabilityAnalysis[];
  /** @state artifact */
  testSeries: En1990TestSeries[];
  /** @state artifact */
  placements: En1990Placement[];
  /** @state artifact */
  cases: CaseRow[];
  /** @state artifact */
  factorOverrides: En1990FactorOverride[];
  /** @state artifact */
  trafficGroups: En1990TrafficGroup[];
}

export type En1990InspectionLevel = "il1" | "il2" | "il3";

export type En1990SupervisionLevel = "dsl1" | "dsl2" | "dsl3";

export type En1990StructureKind = "building" | "roadBridge" | "footbridge" | "railBridge";

export type En1990PermanentKind = "gSup" | "gInf" | "prestress";

export interface En1990PermanentAction {
  id: string;
  kind: En1990PermanentKind;
  gk: number;
}

export interface En1990VariableAction {
  id: string;
  category: string;
  qk: number;
  group: string;
}

export interface En1990AccidentalAction {
  id: string;
  ad: number;
}

export interface En1990SeismicAction {
  id: string;
  aEk: number;
  importanceClass: En1990ImportanceClass;
}

export interface En1990Member {
  id: string;
  labelEn: string;
  labelDe: string;
  rdStr: number;
  rdGeo: number;
  rdEquStab: number;
  rdEquDestab: number;
  rdFat: number;
  span: number;
  precamber: number;
  deflectionW1: number;
  deflectionW2: number;
  deflectionW3: number;
  deflectionLimitRatio: number;
  deflectionLimitRatioW3: number;
  slsLimitChar: number;
  slsLimitFreq: number;
  slsLimitQp: number;
  vibrationFrequency: number;
  vibrationFrequencyMin: number;
  frameHeight: number;
  frameDrift: number;
  storeyHeight: number;
  storeyDrift: number;
  materialStrengthK: number;
  materialConversionFactor: number;
  materialPartialFactor: number;
  materialDesignStrength: number;
  nominalDimension: number;
  dimensionTolerance: number;
  designDimension: number;
  memberAlphaCr: number;
  secondOrderAnalysis: number;
}

export interface En1990MemberEffect {
  memberId: string;
  actionId: string;
  influence: number;
}

export interface En1990BridgeSls {
  id: string;
  memberId: string;
  deckAcceleration: number;
  deckAccelerationLimit: number;
  deckTwist: number;
  deckTwistLimit: number;
  bridgeDeflection: number;
  bridgeDeflectionLimit: number;
  trackGauge: number;
  lineSpeed: number;
  ballastDepth: number;
  deckMass: number;
  naturalFrequency: number;
  endRotation: number;
  basicWindSpeed: number;
  dampingRatio: number;
}

export interface En1990FatigueSpectrum {
  id: string;
  memberId: string;
  detailCategory: number;
  stressRange: number;
  cycles: number;
}

export type En1990DistributionKind = "normal" | "lognormal" | "gumbel" | "weibull";

export interface En1990ReliabilityAnalysis {
  id: string;
  resistanceDistribution: En1990DistributionKind;
  resistanceMean: number;
  resistanceSd: number;
  actionDistribution: En1990DistributionKind;
  actionMean: number;
  actionSd: number;
  secondActionDistribution: En1990DistributionKind;
  secondActionMean: number;
  secondActionSd: number;
  memberId: string;
}

export type En1990TestDistribution = "normal" | "lognormal";

export interface En1990TestSeries {
  id: string;
  quantity: string;
  distribution: En1990TestDistribution;
  sampleSize: number;
  mean: number;
  stdDev: number;
  variationKnown: number;
  conversionFactor: number;
  required: number;
  pairs: string;
  theoryVariation: number;
  theoryMean: number;
}

export interface En1990Placement {
  id: string;
  x: number;
  y: number;
  z: number;
  yaw: number;
}

export interface En1990FactorOverride {
  id: string;
  factor: string;
  target: string;
  value: number;
  justification: string;
}

export interface En1990TrafficGroup {
  id: string;
  group: "GR1A" | "GR1B" | "GR2" | "GR3" | "GR4" | "GR5" | "GR6";
  variableId: string;
}

export type En1990AnnexChoice = "En" | "De";

export type En1990ImportanceClass = "I" | "II" | "III" | "IV";

export interface CaseNumber {
  id: string;
  value: number;
}

export interface CaseText {
  id: string;
  value: string;
}

export interface CaseRow {
  id: string;
  labelEn: string;
  labelDe: string;
  entryId: string;
  numbers: CaseNumber[];
  texts: CaseText[];
}

export const parseEn1990Snapshot: NormWireReader<En1990Snapshot> = normWireObject<En1990Snapshot>({ annex: normWireRequired(normWireLiteral("En", "De")), projectId: normWireRequired(normWireString), structureKind: normWireRequired(normWireRef(() => parseEn1990StructureKind)), altitudeM: normWireRequired(normWireNumber), consequenceClass: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), reliabilityClass: normWireRequired(normWireLiteral("RC1", "RC2", "RC3")), designWorkingLifeCategory: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), designWorkingLifeYears: normWireRequired(normWireNumber), referencePeriodYears: normWireRequired(normWireNumber), supervisionLevel: normWireRequired(normWireRef(() => parseEn1990SupervisionLevel)), inspectionLevel: normWireRequired(normWireRef(() => parseEn1990InspectionLevel)), betaComputed: normWireRequired(normWireNumber), permanents: normWireRequired(normWireArray(normWireRef(() => parseEn1990PermanentAction))), variables: normWireRequired(normWireArray(normWireRef(() => parseEn1990VariableAction))), accidentals: normWireRequired(normWireArray(normWireRef(() => parseEn1990AccidentalAction))), seismics: normWireRequired(normWireArray(normWireRef(() => parseEn1990SeismicAction))), members: normWireRequired(normWireArray(normWireRef(() => parseEn1990Member))), bridgeSls: normWireRequired(normWireArray(normWireRef(() => parseEn1990BridgeSls))), effects: normWireRequired(normWireArray(normWireRef(() => parseEn1990MemberEffect))), fatigueSpectra: normWireRequired(normWireArray(normWireRef(() => parseEn1990FatigueSpectrum))), reliabilityAnalyses: normWireRequired(normWireArray(normWireRef(() => parseEn1990ReliabilityAnalysis))), testSeries: normWireRequired(normWireArray(normWireRef(() => parseEn1990TestSeries))), placements: normWireRequired(normWireArray(normWireRef(() => parseEn1990Placement))), cases: normWireRequired(normWireArray(normWireRef(() => parseCaseRow))), factorOverrides: normWireRequired(normWireArray(normWireRef(() => parseEn1990FactorOverride))), trafficGroups: normWireRequired(normWireArray(normWireRef(() => parseEn1990TrafficGroup))) });
export const parseEn1990InspectionLevel: NormWireReader<En1990InspectionLevel> = normWireLiteral("il1", "il2", "il3");
export const parseEn1990SupervisionLevel: NormWireReader<En1990SupervisionLevel> = normWireLiteral("dsl1", "dsl2", "dsl3");
export const parseEn1990StructureKind: NormWireReader<En1990StructureKind> = normWireLiteral("building", "roadBridge", "footbridge", "railBridge");
export const parseEn1990PermanentKind: NormWireReader<En1990PermanentKind> = normWireLiteral("gSup", "gInf", "prestress");
export const parseEn1990PermanentAction: NormWireReader<En1990PermanentAction> = normWireObject<En1990PermanentAction>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseEn1990PermanentKind)), gk: normWireRequired(normWireNumber) });
export const parseEn1990VariableAction: NormWireReader<En1990VariableAction> = normWireObject<En1990VariableAction>({ id: normWireRequired(normWireString), category: normWireRequired(normWireString), qk: normWireRequired(normWireNumber), group: normWireRequired(normWireString) });
export const parseEn1990AccidentalAction: NormWireReader<En1990AccidentalAction> = normWireObject<En1990AccidentalAction>({ id: normWireRequired(normWireString), ad: normWireRequired(normWireNumber) });
export const parseEn1990SeismicAction: NormWireReader<En1990SeismicAction> = normWireObject<En1990SeismicAction>({ id: normWireRequired(normWireString), aEk: normWireRequired(normWireNumber), importanceClass: normWireRequired(normWireRef(() => parseEn1990ImportanceClass)) });
export const parseEn1990Member: NormWireReader<En1990Member> = normWireObject<En1990Member>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), rdStr: normWireRequired(normWireNumber), rdGeo: normWireRequired(normWireNumber), rdEquStab: normWireRequired(normWireNumber), rdEquDestab: normWireRequired(normWireNumber), rdFat: normWireRequired(normWireNumber), span: normWireRequired(normWireNumber), precamber: normWireRequired(normWireNumber), deflectionW1: normWireRequired(normWireNumber), deflectionW2: normWireRequired(normWireNumber), deflectionW3: normWireRequired(normWireNumber), deflectionLimitRatio: normWireRequired(normWireNumber), deflectionLimitRatioW3: normWireRequired(normWireNumber), slsLimitChar: normWireRequired(normWireNumber), slsLimitFreq: normWireRequired(normWireNumber), slsLimitQp: normWireRequired(normWireNumber), vibrationFrequency: normWireRequired(normWireNumber), vibrationFrequencyMin: normWireRequired(normWireNumber), frameHeight: normWireRequired(normWireNumber), frameDrift: normWireRequired(normWireNumber), storeyHeight: normWireRequired(normWireNumber), storeyDrift: normWireRequired(normWireNumber), materialStrengthK: normWireRequired(normWireNumber), materialConversionFactor: normWireRequired(normWireNumber), materialPartialFactor: normWireRequired(normWireNumber), materialDesignStrength: normWireRequired(normWireNumber), nominalDimension: normWireRequired(normWireNumber), dimensionTolerance: normWireRequired(normWireNumber), designDimension: normWireRequired(normWireNumber), memberAlphaCr: normWireRequired(normWireNumber), secondOrderAnalysis: normWireRequired(normWireNumber) });
export const parseEn1990MemberEffect: NormWireReader<En1990MemberEffect> = normWireObject<En1990MemberEffect>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), influence: normWireRequired(normWireNumber) });
export const parseEn1990BridgeSls: NormWireReader<En1990BridgeSls> = normWireObject<En1990BridgeSls>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), deckAcceleration: normWireRequired(normWireNumber), deckAccelerationLimit: normWireRequired(normWireNumber), deckTwist: normWireRequired(normWireNumber), deckTwistLimit: normWireRequired(normWireNumber), bridgeDeflection: normWireRequired(normWireNumber), bridgeDeflectionLimit: normWireRequired(normWireNumber), trackGauge: normWireRequired(normWireNumber), lineSpeed: normWireRequired(normWireNumber), ballastDepth: normWireRequired(normWireNumber), deckMass: normWireRequired(normWireNumber), naturalFrequency: normWireRequired(normWireNumber), endRotation: normWireRequired(normWireNumber), basicWindSpeed: normWireRequired(normWireNumber), dampingRatio: normWireRequired(normWireNumber) });
export const parseEn1990FatigueSpectrum: NormWireReader<En1990FatigueSpectrum> = normWireObject<En1990FatigueSpectrum>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), detailCategory: normWireRequired(normWireNumber), stressRange: normWireRequired(normWireNumber), cycles: normWireRequired(normWireNumber) });
export const parseEn1990DistributionKind: NormWireReader<En1990DistributionKind> = normWireLiteral("normal", "lognormal", "gumbel", "weibull");
export const parseEn1990ReliabilityAnalysis: NormWireReader<En1990ReliabilityAnalysis> = normWireObject<En1990ReliabilityAnalysis>({ id: normWireRequired(normWireString), resistanceDistribution: normWireRequired(normWireRef(() => parseEn1990DistributionKind)), resistanceMean: normWireRequired(normWireNumber), resistanceSd: normWireRequired(normWireNumber), actionDistribution: normWireRequired(normWireRef(() => parseEn1990DistributionKind)), actionMean: normWireRequired(normWireNumber), actionSd: normWireRequired(normWireNumber), secondActionDistribution: normWireRequired(normWireRef(() => parseEn1990DistributionKind)), secondActionMean: normWireRequired(normWireNumber), secondActionSd: normWireRequired(normWireNumber), memberId: normWireRequired(normWireString) });
export const parseEn1990TestDistribution: NormWireReader<En1990TestDistribution> = normWireLiteral("normal", "lognormal");
export const parseEn1990TestSeries: NormWireReader<En1990TestSeries> = normWireObject<En1990TestSeries>({ id: normWireRequired(normWireString), quantity: normWireRequired(normWireString), distribution: normWireRequired(normWireRef(() => parseEn1990TestDistribution)), sampleSize: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), mean: normWireRequired(normWireNumber), stdDev: normWireRequired(normWireNumber), variationKnown: normWireRequired(normWireNumber), conversionFactor: normWireRequired(normWireNumber), required: normWireRequired(normWireNumber), pairs: normWireRequired(normWireString), theoryVariation: normWireRequired(normWireNumber), theoryMean: normWireRequired(normWireNumber) });
export const parseEn1990Placement: NormWireReader<En1990Placement> = normWireObject<En1990Placement>({ id: normWireRequired(normWireString), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber), z: normWireRequired(normWireNumber), yaw: normWireRequired(normWireNumber) });
export const parseEn1990FactorOverride: NormWireReader<En1990FactorOverride> = normWireObject<En1990FactorOverride>({ id: normWireRequired(normWireString), factor: normWireRequired(normWireString), target: normWireRequired(normWireString), value: normWireRequired(normWireNumber), justification: normWireRequired(normWireString) });
export const parseEn1990TrafficGroup: NormWireReader<En1990TrafficGroup> = normWireObject<En1990TrafficGroup>({ id: normWireRequired(normWireString), group: normWireRequired(normWireLiteral("GR1A", "GR1B", "GR2", "GR3", "GR4", "GR5", "GR6")), variableId: normWireRequired(normWireString) });
export const parseEn1990AnnexChoice: NormWireReader<En1990AnnexChoice> = normWireLiteral("En", "De");
export const parseEn1990ImportanceClass: NormWireReader<En1990ImportanceClass> = normWireLiteral("I", "II", "III", "IV");
export const parseCaseNumber: NormWireReader<CaseNumber> = normWireObject<CaseNumber>({ id: normWireRequired(normWireString), value: normWireRequired(normWireNumber) });
export const parseCaseText: NormWireReader<CaseText> = normWireObject<CaseText>({ id: normWireRequired(normWireString), value: normWireRequired(normWireString) });
export const parseCaseRow: NormWireReader<CaseRow> = normWireObject<CaseRow>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), entryId: normWireRequired(normWireString), numbers: normWireRequired(normWireArray(normWireRef(() => parseCaseNumber))), texts: normWireRequired(normWireArray(normWireRef(() => parseCaseText))) });
