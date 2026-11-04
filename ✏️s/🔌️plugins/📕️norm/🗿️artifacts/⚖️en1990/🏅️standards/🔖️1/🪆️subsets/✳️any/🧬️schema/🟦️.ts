/** 🧬️ `En1990Artifact` wire twin: the artifact document across its state lanes, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1990InspectionLevel, parseEn1990InspectionLevel, type En1990StructureKind, parseEn1990StructureKind, type En1990SupervisionLevel, parseEn1990SupervisionLevel } from "./📸️snapshot/🟦️.ts";

export interface En1990Artifact {
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
  permanents: PermanentAction[];
  /** @state artifact */
  variables: VariableAction[];
  /** @state artifact */
  accidentals: AccidentalAction[];
  /** @state artifact */
  seismics: SeismicAction[];
  /** @state artifact */
  members: Member[];
  /** @state artifact */
  bridgeSls: BridgeSls[];
  /** @state artifact */
  effects: MemberEffect[];
  /** @state artifact */
  fatigueSpectra: FatigueSpectrum[];
  /** @state artifact */
  reliabilityAnalyses: ReliabilityAnalysis[];
  /** @state artifact */
  testSeries: TestSeries[];
  /** @state artifact */
  placements: Placement[];
  /** @state artifact */
  cases: CaseRow[];
  /** @state artifact */
  factorOverrides: FactorOverride[];
  /** @state artifact */
  trafficGroups: TrafficGroup[];
}

export interface PermanentAction {
  id: string;
  kind: string;
  gk: number;
}

export interface VariableAction {
  id: string;
  category: string;
  qk: number;
  group: string;
}

export interface AccidentalAction {
  id: string;
  ad: number;
}

export interface SeismicAction {
  id: string;
  aEk: number;
  importanceClass: "I" | "II" | "III" | "IV";
}

export interface Member {
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

export interface MemberEffect {
  memberId: string;
  actionId: string;
  influence: number;
}

export interface BridgeSls {
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

export interface FatigueSpectrum {
  id: string;
  memberId: string;
  detailCategory: number;
  stressRange: number;
  cycles: number;
}

export interface ReliabilityAnalysis {
  id: string;
  resistanceDistribution: string;
  resistanceMean: number;
  resistanceSd: number;
  actionDistribution: string;
  actionMean: number;
  actionSd: number;
  secondActionDistribution: string;
  secondActionMean: number;
  secondActionSd: number;
  memberId: string;
}

export interface TestSeries {
  id: string;
  quantity: string;
  distribution: string;
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

export interface Placement {
  id: string;
  x: number;
  y: number;
  z: number;
  yaw: number;
}

export interface FactorOverride {
  id: string;
  factor: string;
  target: string;
  value: number;
  justification: string;
}

export interface TrafficGroup {
  id: string;
  group: "GR1A" | "GR1B" | "GR2" | "GR3" | "GR4" | "GR5" | "GR6";
  variableId: string;
}

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

export const parseEn1990Artifact: NormWireReader<En1990Artifact> = normWireObject<En1990Artifact>({ annex: normWireRequired(normWireLiteral("En", "De")), projectId: normWireRequired(normWireString), structureKind: normWireRequired(parseEn1990StructureKind), altitudeM: normWireRequired(normWireNumber), consequenceClass: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), reliabilityClass: normWireRequired(normWireLiteral("RC1", "RC2", "RC3")), designWorkingLifeCategory: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), designWorkingLifeYears: normWireRequired(normWireNumber), referencePeriodYears: normWireRequired(normWireNumber), supervisionLevel: normWireRequired(parseEn1990SupervisionLevel), inspectionLevel: normWireRequired(parseEn1990InspectionLevel), betaComputed: normWireRequired(normWireNumber), permanents: normWireRequired(normWireArray(normWireRef(() => parsePermanentAction))), variables: normWireRequired(normWireArray(normWireRef(() => parseVariableAction))), accidentals: normWireRequired(normWireArray(normWireRef(() => parseAccidentalAction))), seismics: normWireRequired(normWireArray(normWireRef(() => parseSeismicAction))), members: normWireRequired(normWireArray(normWireRef(() => parseMember))), bridgeSls: normWireRequired(normWireArray(normWireRef(() => parseBridgeSls))), effects: normWireRequired(normWireArray(normWireRef(() => parseMemberEffect))), fatigueSpectra: normWireRequired(normWireArray(normWireRef(() => parseFatigueSpectrum))), reliabilityAnalyses: normWireRequired(normWireArray(normWireRef(() => parseReliabilityAnalysis))), testSeries: normWireRequired(normWireArray(normWireRef(() => parseTestSeries))), placements: normWireRequired(normWireArray(normWireRef(() => parsePlacement))), cases: normWireRequired(normWireArray(normWireRef(() => parseCaseRow))), factorOverrides: normWireRequired(normWireArray(normWireRef(() => parseFactorOverride))), trafficGroups: normWireRequired(normWireArray(normWireRef(() => parseTrafficGroup))) });
export const parsePermanentAction: NormWireReader<PermanentAction> = normWireObject<PermanentAction>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireString), gk: normWireRequired(normWireNumber) });
export const parseVariableAction: NormWireReader<VariableAction> = normWireObject<VariableAction>({ id: normWireRequired(normWireString), category: normWireRequired(normWireString), qk: normWireRequired(normWireNumber), group: normWireRequired(normWireString) });
export const parseAccidentalAction: NormWireReader<AccidentalAction> = normWireObject<AccidentalAction>({ id: normWireRequired(normWireString), ad: normWireRequired(normWireNumber) });
export const parseSeismicAction: NormWireReader<SeismicAction> = normWireObject<SeismicAction>({ id: normWireRequired(normWireString), aEk: normWireRequired(normWireNumber), importanceClass: normWireRequired(normWireLiteral("I", "II", "III", "IV")) });
export const parseMember: NormWireReader<Member> = normWireObject<Member>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), rdStr: normWireRequired(normWireNumber), rdGeo: normWireRequired(normWireNumber), rdEquStab: normWireRequired(normWireNumber), rdEquDestab: normWireRequired(normWireNumber), rdFat: normWireRequired(normWireNumber), span: normWireRequired(normWireNumber), precamber: normWireRequired(normWireNumber), deflectionW1: normWireRequired(normWireNumber), deflectionW2: normWireRequired(normWireNumber), deflectionW3: normWireRequired(normWireNumber), deflectionLimitRatio: normWireRequired(normWireNumber), deflectionLimitRatioW3: normWireRequired(normWireNumber), slsLimitChar: normWireRequired(normWireNumber), slsLimitFreq: normWireRequired(normWireNumber), slsLimitQp: normWireRequired(normWireNumber), vibrationFrequency: normWireRequired(normWireNumber), vibrationFrequencyMin: normWireRequired(normWireNumber), frameHeight: normWireRequired(normWireNumber), frameDrift: normWireRequired(normWireNumber), storeyHeight: normWireRequired(normWireNumber), storeyDrift: normWireRequired(normWireNumber), materialStrengthK: normWireRequired(normWireNumber), materialConversionFactor: normWireRequired(normWireNumber), materialPartialFactor: normWireRequired(normWireNumber), materialDesignStrength: normWireRequired(normWireNumber), nominalDimension: normWireRequired(normWireNumber), dimensionTolerance: normWireRequired(normWireNumber), designDimension: normWireRequired(normWireNumber), memberAlphaCr: normWireRequired(normWireNumber), secondOrderAnalysis: normWireRequired(normWireNumber) });
export const parseMemberEffect: NormWireReader<MemberEffect> = normWireObject<MemberEffect>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), influence: normWireRequired(normWireNumber) });
export const parseBridgeSls: NormWireReader<BridgeSls> = normWireObject<BridgeSls>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), deckAcceleration: normWireRequired(normWireNumber), deckAccelerationLimit: normWireRequired(normWireNumber), deckTwist: normWireRequired(normWireNumber), deckTwistLimit: normWireRequired(normWireNumber), bridgeDeflection: normWireRequired(normWireNumber), bridgeDeflectionLimit: normWireRequired(normWireNumber), trackGauge: normWireRequired(normWireNumber), lineSpeed: normWireRequired(normWireNumber), ballastDepth: normWireRequired(normWireNumber), deckMass: normWireRequired(normWireNumber), naturalFrequency: normWireRequired(normWireNumber), endRotation: normWireRequired(normWireNumber), basicWindSpeed: normWireRequired(normWireNumber), dampingRatio: normWireRequired(normWireNumber) });
export const parseFatigueSpectrum: NormWireReader<FatigueSpectrum> = normWireObject<FatigueSpectrum>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), detailCategory: normWireRequired(normWireNumber), stressRange: normWireRequired(normWireNumber), cycles: normWireRequired(normWireNumber) });
export const parseReliabilityAnalysis: NormWireReader<ReliabilityAnalysis> = normWireObject<ReliabilityAnalysis>({ id: normWireRequired(normWireString), resistanceDistribution: normWireRequired(normWireString), resistanceMean: normWireRequired(normWireNumber), resistanceSd: normWireRequired(normWireNumber), actionDistribution: normWireRequired(normWireString), actionMean: normWireRequired(normWireNumber), actionSd: normWireRequired(normWireNumber), secondActionDistribution: normWireRequired(normWireString), secondActionMean: normWireRequired(normWireNumber), secondActionSd: normWireRequired(normWireNumber), memberId: normWireRequired(normWireString) });
export const parseTestSeries: NormWireReader<TestSeries> = normWireObject<TestSeries>({ id: normWireRequired(normWireString), quantity: normWireRequired(normWireString), distribution: normWireRequired(normWireString), sampleSize: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), mean: normWireRequired(normWireNumber), stdDev: normWireRequired(normWireNumber), variationKnown: normWireRequired(normWireNumber), conversionFactor: normWireRequired(normWireNumber), required: normWireRequired(normWireNumber), pairs: normWireRequired(normWireString), theoryVariation: normWireRequired(normWireNumber), theoryMean: normWireRequired(normWireNumber) });
export const parsePlacement: NormWireReader<Placement> = normWireObject<Placement>({ id: normWireRequired(normWireString), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber), z: normWireRequired(normWireNumber), yaw: normWireRequired(normWireNumber) });
export const parseFactorOverride: NormWireReader<FactorOverride> = normWireObject<FactorOverride>({ id: normWireRequired(normWireString), factor: normWireRequired(normWireString), target: normWireRequired(normWireString), value: normWireRequired(normWireNumber), justification: normWireRequired(normWireString) });
export const parseTrafficGroup: NormWireReader<TrafficGroup> = normWireObject<TrafficGroup>({ id: normWireRequired(normWireString), group: normWireRequired(normWireLiteral("GR1A", "GR1B", "GR2", "GR3", "GR4", "GR5", "GR6")), variableId: normWireRequired(normWireString) });
export const parseCaseNumber: NormWireReader<CaseNumber> = normWireObject<CaseNumber>({ id: normWireRequired(normWireString), value: normWireRequired(normWireNumber) });
export const parseCaseText: NormWireReader<CaseText> = normWireObject<CaseText>({ id: normWireRequired(normWireString), value: normWireRequired(normWireString) });
export const parseCaseRow: NormWireReader<CaseRow> = normWireObject<CaseRow>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), entryId: normWireRequired(normWireString), numbers: normWireRequired(normWireArray(normWireRef(() => parseCaseNumber))), texts: normWireRequired(normWireArray(normWireRef(() => parseCaseText))) });
