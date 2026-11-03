/** 📸️ `En1997Snapshot` wire twin: the persisted snapshot and every record it holds, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface En1997Snapshot {
  /** @state artifact */
  structureId: string;
  /** @state artifact */
  geotechnicalCategory: number;
  /** @state artifact */
  designSituation: SituationChoice;
  /** @state artifact */
  designApproach: ApproachChoice;
  /** @state artifact */
  annex: En1997AnnexChoice;
  /** @state artifact */
  groundwaterLevel: number;
  /** @state artifact */
  investigationDepth: number;
  /** @state artifact */
  layers: En1997SoilLayer[];
  /** @state artifact */
  footings: En1997SpreadFoundation[];
  /** @state artifact */
  piles: En1997Pile[];
  /** @state artifact */
  retainingWalls: En1997RetainingWall[];
  /** @state artifact */
  slopes: En1997Slope[];
  /** @state artifact */
  upliftCases: En1997UpliftCase[];
  /** @state artifact */
  anchors: En1997Anchor[];
  /** @state artifact */
  embeddedWalls: En1997EmbeddedWall[];
  /** @state artifact */
  pileGroups: En1997PileGroup[];
  /** @state artifact */
  labTests: En1997LabTest[];
  /** @state artifact */
  layerWaters: En1997LayerWater[];
  /** @state artifact */
  placements: En1997Placement[];
  /** @state artifact */
  cases: En1997CaseRow[];
  /** @state artifact */
  investigations: En1997Investigation[];
  /** @state artifact */
  characteristicBases: En1997CharacteristicBasis[];
}

export type ApproachChoice = "da1Str" | "da1Geo" | "da2" | "da2Star" | "da3" | "geo3";

export interface En1997SpreadFoundation {
  id: string;
  width: number;
  length: number;
  embedment: number;
  baseInclinationDeg: number;
  settlementLimit: number;
  loadCases: En1997FoundationLoadCase[];
}

export type SoilType = "sand" | "gravel" | "clay" | "silt" | "fill";

export interface En1997SoilLayer {
  id: string;
  soilType: SoilType;
  depthTop: number;
  depthBottom: number;
  gamma: number;
  gammaPrime: number;
  phiPrimeDeg: number;
  cohesionEffective: number;
  cohesionUndrained: number;
  oedometricModulus: number;
  poissonRatio: number;
  cptQc: number;
  sptN: number;
  cptNkt: number;
}

export type En1997AnnexChoice = "En" | "De";

export type PileType = "bored" | "driven" | "cfa";

export interface En1997Pile {
  id: string;
  pileType: PileType;
  diameter: number;
  length: number;
  count: number;
  alphaS: number;
  unitShaftResistance: number;
  unitBaseResistance: number;
  compressionPermanent: number;
  compressionVariable: number;
  tensionPermanent: number;
  tensionVariable: number;
  bendingStiffnessEi: number;
  horizontalSubgradeModulus: number;
  lateralHeadLoad: number;
  elasticModulus: number;
  settlingThickness: number;
  negativeFrictionBeta: number;
  dynamicTestCount: number;
  dynamicResistanceMean: number;
  dynamicResistanceMin: number;
  hammerWeight: number;
  dropHeight: number;
  setPerBlow: number;
  elasticCompression: number;
  hammerEfficiency: number;
  concreteClass: PileConcreteClass;
  testProfiles: En1997PileTestProfile[];
}

export type PileConcreteClass = "notGiven" | "c1215" | "c1620" | "c2025" | "c2530" | "c3037" | "c3545" | "c4050" | "c4555" | "c5060";

export type SituationChoice = "persistent" | "transient" | "accidental";

export interface En1997FoundationLoadCase {
  id: string;
  designSituation: SituationChoice;
  verticalPermanent: number;
  verticalVariable: number;
  horizontalPermanent: number;
  horizontalVariable: number;
  momentPermanent: number;
  momentVariable: number;
}

export interface En1997PileTestProfile {
  id: string;
  shaftResistance: number;
  baseResistance: number;
}

export type EarthPressureMode = "active" | "atRest" | "increasedActive";

export type WallMovement = "free" | "rigid" | "propped";

export interface En1997RetainingWall {
  id: string;
  height: number;
  embedment: number;
  baseWidth: number;
  stemThickness: number;
  backfillPhiDeg: number;
  backfillGamma: number;
  wallFrictionDeg: number;
  earthPressureMode: EarthPressureMode;
  wallMovement: WallMovement;
  ocr: number;
  concreteGamma: number;
  surcharge: number;
  verticalPermanent: number;
  horizontalPermanent: number;
  seismicKh: number;
  seismicKv: number;
  reinforcementSpacing: number;
  reinforcementWidth: number;
  reinforcementFriction: number;
}

export interface En1997Slope {
  id: string;
  angleDeg: number;
  height: number;
  length: number;
  governingLayerId: string;
  circleCentreX: number;
  circleCentreZ: number;
  circleRadius: number;
  width: number;
}

export interface En1997UpliftCase {
  id: string;
  permanentStabilizing: number;
  permanentDestabilizing: number;
  variableDestabilizing: number;
  porePressure: number;
  totalStress: number;
}

export interface En1997AnchorTest {
  id: string;
  pullOutResistance: number;
}

export type TendonType = "strand" | "bar";

export interface En1997Anchor {
  id: string;
  wallId: string;
  tendon: TendonType;
  headDepth: number;
  inclinationDeg: number;
  freeLength: number;
  fixedLength: number;
  fixedDiameter: number;
  bondStress: number;
  designForce: number;
  tendonYieldForce: number;
  minFreeLength: number;
  testFactor: number;
  tests: En1997AnchorTest[];
}

export type EmbeddedWallKind = "freeEarth" | "cantilever";

export interface En1997EmbeddedWall {
  id: string;
  kind: EmbeddedWallKind;
  retainedHeight: number;
  embedment: number;
  propDepth: number;
  thickness: number;
  backfillGamma: number;
  phiDeg: number;
  deltaActiveDeg: number;
  deltaPassiveDeg: number;
  surcharge: number;
  undrainedStrength: number;
}

export interface En1997PileGroup {
  id: string;
  pileId: string;
  rows: number;
  columns: number;
  spacing: number;
  capVertical: number;
  capMomentX: number;
  capMomentY: number;
  singleResistance: number;
  shaftCohesionSum: number;
  baseCohesion: number;
}

export interface En1997LabPoint {
  id: string;
  normalStress: number;
  shearStress: number;
}

export type LabTestKind = "oedometer" | "triaxial" | "directShear";

export interface En1997LabTest {
  id: string;
  layerId: string;
  kind: LabTestKind;
  voidRatio: number;
  preloadStress: number;
  drainageHeight: number;
  consolidationTime: number;
  compressionIndex: number;
  swellingIndex: number;
  stressIncrement: number;
  secondaryCompressionIndex: number;
  designLife: number;
  points: En1997LabPoint[];
}

export interface En1997LayerWater {
  id: string;
  layerId: string;
  piezometricDepth: number;
  permeability: number;
  headDifference: number;
  flowLength: number;
  wellRadius: number;
  wellWaterDepth: number;
  influenceRadius: number;
  wellCapacity: number;
}

export interface En1997Placement {
  id: string;
  x: number;
  y: number;
  z: number;
  yaw: number;
}

export interface En1997CaseNumber {
  id: string;
  value: number;
}

export interface En1997CaseText {
  id: string;
  value: string;
}

export interface En1997CaseRow {
  id: string;
  labelEn: string;
  labelDe: string;
  entryId: string;
  numbers: En1997CaseNumber[];
  texts: En1997CaseText[];
}

export type InvestigationKind = "borehole" | "cpt" | "spt";

export interface En1997InvestigationReading {
  id: string;
  depth: number;
  measurement: number;
}

export interface En1997Investigation {
  id: string;
  positionX: number;
  positionY: number;
  groundLevel: number;
  depth: number;
  kind: InvestigationKind;
  readings: En1997InvestigationReading[];
}

export type CharacteristicParameter = "undrainedStrength" | "frictionAngle";

export type ParameterBasis = "declared" | "derived";

export interface En1997CharacteristicBasis {
  id: string;
  layerId: string;
  parameter: CharacteristicParameter;
  basis: ParameterBasis;
}

export const parseEn1997Snapshot: NormWireReader<En1997Snapshot> = normWireObject<En1997Snapshot>({ structureId: normWireRequired(normWireString), geotechnicalCategory: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), designSituation: normWireRequired(normWireRef(() => parseSituationChoice)), designApproach: normWireRequired(normWireRef(() => parseApproachChoice)), annex: normWireRequired(normWireRef(() => parseEn1997AnnexChoice)), groundwaterLevel: normWireRequired(normWireNumber), investigationDepth: normWireRequired(normWireNumber), layers: normWireRequired(normWireArray(normWireRef(() => parseEn1997SoilLayer))), footings: normWireRequired(normWireArray(normWireRef(() => parseEn1997SpreadFoundation))), piles: normWireRequired(normWireArray(normWireRef(() => parseEn1997Pile))), retainingWalls: normWireRequired(normWireArray(normWireRef(() => parseEn1997RetainingWall))), slopes: normWireRequired(normWireArray(normWireRef(() => parseEn1997Slope))), upliftCases: normWireRequired(normWireArray(normWireRef(() => parseEn1997UpliftCase))), anchors: normWireRequired(normWireArray(normWireRef(() => parseEn1997Anchor))), embeddedWalls: normWireRequired(normWireArray(normWireRef(() => parseEn1997EmbeddedWall))), pileGroups: normWireRequired(normWireArray(normWireRef(() => parseEn1997PileGroup))), labTests: normWireRequired(normWireArray(normWireRef(() => parseEn1997LabTest))), layerWaters: normWireRequired(normWireArray(normWireRef(() => parseEn1997LayerWater))), placements: normWireRequired(normWireArray(normWireRef(() => parseEn1997Placement))), cases: normWireRequired(normWireArray(normWireRef(() => parseEn1997CaseRow))), investigations: normWireRequired(normWireArray(normWireRef(() => parseEn1997Investigation))), characteristicBases: normWireRequired(normWireArray(normWireRef(() => parseEn1997CharacteristicBasis))) });
export const parseApproachChoice: NormWireReader<ApproachChoice> = normWireLiteral("da1Str", "da1Geo", "da2", "da2Star", "da3", "geo3");
export const parseEn1997SpreadFoundation: NormWireReader<En1997SpreadFoundation> = normWireObject<En1997SpreadFoundation>({ id: normWireRequired(normWireString), width: normWireRequired(normWireNumber), length: normWireRequired(normWireNumber), embedment: normWireRequired(normWireNumber), baseInclinationDeg: normWireRequired(normWireNumber), settlementLimit: normWireRequired(normWireNumber), loadCases: normWireRequired(normWireArray(normWireRef(() => parseEn1997FoundationLoadCase))) });
export const parseSoilType: NormWireReader<SoilType> = normWireLiteral("sand", "gravel", "clay", "silt", "fill");
export const parseEn1997SoilLayer: NormWireReader<En1997SoilLayer> = normWireObject<En1997SoilLayer>({ id: normWireRequired(normWireString), soilType: normWireRequired(normWireRef(() => parseSoilType)), depthTop: normWireRequired(normWireNumber), depthBottom: normWireRequired(normWireNumber), gamma: normWireRequired(normWireNumber), gammaPrime: normWireRequired(normWireNumber), phiPrimeDeg: normWireRequired(normWireNumber), cohesionEffective: normWireRequired(normWireNumber), cohesionUndrained: normWireRequired(normWireNumber), oedometricModulus: normWireRequired(normWireNumber), poissonRatio: normWireRequired(normWireNumber), cptQc: normWireRequired(normWireNumber), sptN: normWireRequired(normWireNumber), cptNkt: normWireRequired(normWireNumber) });
export const parseEn1997AnnexChoice: NormWireReader<En1997AnnexChoice> = normWireLiteral("En", "De");
export const parsePileType: NormWireReader<PileType> = normWireLiteral("bored", "driven", "cfa");
export const parseEn1997Pile: NormWireReader<En1997Pile> = normWireObject<En1997Pile>({ id: normWireRequired(normWireString), pileType: normWireRequired(normWireRef(() => parsePileType)), diameter: normWireRequired(normWireNumber), length: normWireRequired(normWireNumber), count: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), alphaS: normWireRequired(normWireNumber), unitShaftResistance: normWireRequired(normWireNumber), unitBaseResistance: normWireRequired(normWireNumber), compressionPermanent: normWireRequired(normWireNumber), compressionVariable: normWireRequired(normWireNumber), tensionPermanent: normWireRequired(normWireNumber), tensionVariable: normWireRequired(normWireNumber), bendingStiffnessEi: normWireRequired(normWireNumber), horizontalSubgradeModulus: normWireRequired(normWireNumber), lateralHeadLoad: normWireRequired(normWireNumber), elasticModulus: normWireRequired(normWireNumber), settlingThickness: normWireRequired(normWireNumber), negativeFrictionBeta: normWireRequired(normWireNumber), dynamicTestCount: normWireRequired(normWireNumber), dynamicResistanceMean: normWireRequired(normWireNumber), dynamicResistanceMin: normWireRequired(normWireNumber), hammerWeight: normWireRequired(normWireNumber), dropHeight: normWireRequired(normWireNumber), setPerBlow: normWireRequired(normWireNumber), elasticCompression: normWireRequired(normWireNumber), hammerEfficiency: normWireRequired(normWireNumber), concreteClass: normWireRequired(normWireRef(() => parsePileConcreteClass)), testProfiles: normWireRequired(normWireArray(normWireRef(() => parseEn1997PileTestProfile))) });
export const parsePileConcreteClass: NormWireReader<PileConcreteClass> = normWireLiteral("notGiven", "c1215", "c1620", "c2025", "c2530", "c3037", "c3545", "c4050", "c4555", "c5060");
export const parseSituationChoice: NormWireReader<SituationChoice> = normWireLiteral("persistent", "transient", "accidental");
export const parseEn1997FoundationLoadCase: NormWireReader<En1997FoundationLoadCase> = normWireObject<En1997FoundationLoadCase>({ id: normWireRequired(normWireString), designSituation: normWireRequired(normWireRef(() => parseSituationChoice)), verticalPermanent: normWireRequired(normWireNumber), verticalVariable: normWireRequired(normWireNumber), horizontalPermanent: normWireRequired(normWireNumber), horizontalVariable: normWireRequired(normWireNumber), momentPermanent: normWireRequired(normWireNumber), momentVariable: normWireRequired(normWireNumber) });
export const parseEn1997PileTestProfile: NormWireReader<En1997PileTestProfile> = normWireObject<En1997PileTestProfile>({ id: normWireRequired(normWireString), shaftResistance: normWireRequired(normWireNumber), baseResistance: normWireRequired(normWireNumber) });
export const parseEarthPressureMode: NormWireReader<EarthPressureMode> = normWireLiteral("active", "atRest", "increasedActive");
export const parseWallMovement: NormWireReader<WallMovement> = normWireLiteral("free", "rigid", "propped");
export const parseEn1997RetainingWall: NormWireReader<En1997RetainingWall> = normWireObject<En1997RetainingWall>({ id: normWireRequired(normWireString), height: normWireRequired(normWireNumber), embedment: normWireRequired(normWireNumber), baseWidth: normWireRequired(normWireNumber), stemThickness: normWireRequired(normWireNumber), backfillPhiDeg: normWireRequired(normWireNumber), backfillGamma: normWireRequired(normWireNumber), wallFrictionDeg: normWireRequired(normWireNumber), earthPressureMode: normWireRequired(normWireRef(() => parseEarthPressureMode)), wallMovement: normWireRequired(normWireRef(() => parseWallMovement)), ocr: normWireRequired(normWireNumber), concreteGamma: normWireRequired(normWireNumber), surcharge: normWireRequired(normWireNumber), verticalPermanent: normWireRequired(normWireNumber), horizontalPermanent: normWireRequired(normWireNumber), seismicKh: normWireRequired(normWireNumber), seismicKv: normWireRequired(normWireNumber), reinforcementSpacing: normWireRequired(normWireNumber), reinforcementWidth: normWireRequired(normWireNumber), reinforcementFriction: normWireRequired(normWireNumber) });
export const parseEn1997Slope: NormWireReader<En1997Slope> = normWireObject<En1997Slope>({ id: normWireRequired(normWireString), angleDeg: normWireRequired(normWireNumber), height: normWireRequired(normWireNumber), length: normWireRequired(normWireNumber), governingLayerId: normWireRequired(normWireString), circleCentreX: normWireRequired(normWireNumber), circleCentreZ: normWireRequired(normWireNumber), circleRadius: normWireRequired(normWireNumber), width: normWireRequired(normWireNumber) });
export const parseEn1997UpliftCase: NormWireReader<En1997UpliftCase> = normWireObject<En1997UpliftCase>({ id: normWireRequired(normWireString), permanentStabilizing: normWireRequired(normWireNumber), permanentDestabilizing: normWireRequired(normWireNumber), variableDestabilizing: normWireRequired(normWireNumber), porePressure: normWireRequired(normWireNumber), totalStress: normWireRequired(normWireNumber) });
export const parseEn1997AnchorTest: NormWireReader<En1997AnchorTest> = normWireObject<En1997AnchorTest>({ id: normWireRequired(normWireString), pullOutResistance: normWireRequired(normWireNumber) });
export const parseTendonType: NormWireReader<TendonType> = normWireLiteral("strand", "bar");
export const parseEn1997Anchor: NormWireReader<En1997Anchor> = normWireObject<En1997Anchor>({ id: normWireRequired(normWireString), wallId: normWireRequired(normWireString), tendon: normWireRequired(normWireRef(() => parseTendonType)), headDepth: normWireRequired(normWireNumber), inclinationDeg: normWireRequired(normWireNumber), freeLength: normWireRequired(normWireNumber), fixedLength: normWireRequired(normWireNumber), fixedDiameter: normWireRequired(normWireNumber), bondStress: normWireRequired(normWireNumber), designForce: normWireRequired(normWireNumber), tendonYieldForce: normWireRequired(normWireNumber), minFreeLength: normWireRequired(normWireNumber), testFactor: normWireRequired(normWireNumber), tests: normWireRequired(normWireArray(normWireRef(() => parseEn1997AnchorTest))) });
export const parseEmbeddedWallKind: NormWireReader<EmbeddedWallKind> = normWireLiteral("freeEarth", "cantilever");
export const parseEn1997EmbeddedWall: NormWireReader<En1997EmbeddedWall> = normWireObject<En1997EmbeddedWall>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseEmbeddedWallKind)), retainedHeight: normWireRequired(normWireNumber), embedment: normWireRequired(normWireNumber), propDepth: normWireRequired(normWireNumber), thickness: normWireRequired(normWireNumber), backfillGamma: normWireRequired(normWireNumber), phiDeg: normWireRequired(normWireNumber), deltaActiveDeg: normWireRequired(normWireNumber), deltaPassiveDeg: normWireRequired(normWireNumber), surcharge: normWireRequired(normWireNumber), undrainedStrength: normWireRequired(normWireNumber) });
export const parseEn1997PileGroup: NormWireReader<En1997PileGroup> = normWireObject<En1997PileGroup>({ id: normWireRequired(normWireString), pileId: normWireRequired(normWireString), rows: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), columns: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), spacing: normWireRequired(normWireNumber), capVertical: normWireRequired(normWireNumber), capMomentX: normWireRequired(normWireNumber), capMomentY: normWireRequired(normWireNumber), singleResistance: normWireRequired(normWireNumber), shaftCohesionSum: normWireRequired(normWireNumber), baseCohesion: normWireRequired(normWireNumber) });
export const parseEn1997LabPoint: NormWireReader<En1997LabPoint> = normWireObject<En1997LabPoint>({ id: normWireRequired(normWireString), normalStress: normWireRequired(normWireNumber), shearStress: normWireRequired(normWireNumber) });
export const parseLabTestKind: NormWireReader<LabTestKind> = normWireLiteral("oedometer", "triaxial", "directShear");
export const parseEn1997LabTest: NormWireReader<En1997LabTest> = normWireObject<En1997LabTest>({ id: normWireRequired(normWireString), layerId: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseLabTestKind)), voidRatio: normWireRequired(normWireNumber), preloadStress: normWireRequired(normWireNumber), drainageHeight: normWireRequired(normWireNumber), consolidationTime: normWireRequired(normWireNumber), compressionIndex: normWireRequired(normWireNumber), swellingIndex: normWireRequired(normWireNumber), stressIncrement: normWireRequired(normWireNumber), secondaryCompressionIndex: normWireRequired(normWireNumber), designLife: normWireRequired(normWireNumber), points: normWireRequired(normWireArray(normWireRef(() => parseEn1997LabPoint))) });
export const parseEn1997LayerWater: NormWireReader<En1997LayerWater> = normWireObject<En1997LayerWater>({ id: normWireRequired(normWireString), layerId: normWireRequired(normWireString), piezometricDepth: normWireRequired(normWireNumber), permeability: normWireRequired(normWireNumber), headDifference: normWireRequired(normWireNumber), flowLength: normWireRequired(normWireNumber), wellRadius: normWireRequired(normWireNumber), wellWaterDepth: normWireRequired(normWireNumber), influenceRadius: normWireRequired(normWireNumber), wellCapacity: normWireRequired(normWireNumber) });
export const parseEn1997Placement: NormWireReader<En1997Placement> = normWireObject<En1997Placement>({ id: normWireRequired(normWireString), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber), z: normWireRequired(normWireNumber), yaw: normWireRequired(normWireNumber) });
export const parseEn1997CaseNumber: NormWireReader<En1997CaseNumber> = normWireObject<En1997CaseNumber>({ id: normWireRequired(normWireString), value: normWireRequired(normWireNumber) });
export const parseEn1997CaseText: NormWireReader<En1997CaseText> = normWireObject<En1997CaseText>({ id: normWireRequired(normWireString), value: normWireRequired(normWireString) });
export const parseEn1997CaseRow: NormWireReader<En1997CaseRow> = normWireObject<En1997CaseRow>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), entryId: normWireRequired(normWireString), numbers: normWireRequired(normWireArray(normWireRef(() => parseEn1997CaseNumber))), texts: normWireRequired(normWireArray(normWireRef(() => parseEn1997CaseText))) });
export const parseInvestigationKind: NormWireReader<InvestigationKind> = normWireLiteral("borehole", "cpt", "spt");
export const parseEn1997InvestigationReading: NormWireReader<En1997InvestigationReading> = normWireObject<En1997InvestigationReading>({ id: normWireRequired(normWireString), depth: normWireRequired(normWireNumber), measurement: normWireRequired(normWireNumber) });
export const parseEn1997Investigation: NormWireReader<En1997Investigation> = normWireObject<En1997Investigation>({ id: normWireRequired(normWireString), positionX: normWireRequired(normWireNumber), positionY: normWireRequired(normWireNumber), groundLevel: normWireRequired(normWireNumber), depth: normWireRequired(normWireNumber), kind: normWireRequired(normWireRef(() => parseInvestigationKind)), readings: normWireRequired(normWireArray(normWireRef(() => parseEn1997InvestigationReading))) });
export const parseCharacteristicParameter: NormWireReader<CharacteristicParameter> = normWireLiteral("undrainedStrength", "frictionAngle");
export const parseParameterBasis: NormWireReader<ParameterBasis> = normWireLiteral("declared", "derived");
export const parseEn1997CharacteristicBasis: NormWireReader<En1997CharacteristicBasis> = normWireObject<En1997CharacteristicBasis>({ id: normWireRequired(normWireString), layerId: normWireRequired(normWireString), parameter: normWireRequired(normWireRef(() => parseCharacteristicParameter)), basis: normWireRequired(normWireRef(() => parseParameterBasis)) });
