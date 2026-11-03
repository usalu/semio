/** 🧬️ `En1997Artifact` wire twin: the artifact document across its state lanes, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface En1997Artifact {
  /** @state artifact */
  structureId: string;
  /** @state artifact */
  geotechnicalCategory: number;
  /** @state artifact */
  designSituation: string;
  /** @state artifact */
  designApproach: string;
  /** @state artifact */
  annex: "En" | "De";
  /** @state artifact */
  groundwaterLevel: number;
  /** @state artifact */
  investigationDepth: number;
  /** @state artifact */
  layers: SoilLayer[];
  /** @state artifact */
  footings: SpreadFoundation[];
  /** @state artifact */
  piles: Pile[];
  /** @state artifact */
  retainingWalls: RetainingWall[];
  /** @state artifact */
  slopes: Slope[];
  /** @state artifact */
  upliftCases: UpliftCase[];
  /** @state artifact */
  anchors: Anchor[];
  /** @state artifact */
  embeddedWalls: EmbeddedWall[];
  /** @state artifact */
  pileGroups: PileGroup[];
  /** @state artifact */
  labTests: LabTest[];
  /** @state artifact */
  layerWaters: LayerWater[];
  /** @state artifact */
  placements: Placement[];
  /** @state artifact */
  cases: CaseRow[];
  /** @state artifact */
  investigations: Investigation[];
  /** @state artifact */
  characteristicBases: CharacteristicBasis[];
}

export type SoilType = "sand" | "gravel" | "clay" | "silt" | "fill";

export interface SoilLayer {
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

export interface SpreadFoundation {
  id: string;
  width: number;
  length: number;
  embedment: number;
  baseInclinationDeg: number;
  settlementLimit: number;
  loadCases: FoundationLoadCase[];
}

export type PileType = "bored" | "driven" | "cfa";

export interface Pile {
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
  testProfiles: PileTestProfile[];
}

export type PileConcreteClass = "notGiven" | "c1215" | "c1620" | "c2025" | "c2530" | "c3037" | "c3545" | "c4050" | "c4555" | "c5060";

export type EarthPressureMode = "active" | "atRest" | "increasedActive";

export type WallMovement = "free" | "rigid" | "propped";

export interface RetainingWall {
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

export interface Slope {
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

export interface UpliftCase {
  id: string;
  permanentStabilizing: number;
  permanentDestabilizing: number;
  variableDestabilizing: number;
  porePressure: number;
  totalStress: number;
}

export type TendonType = "strand" | "bar";

export interface Anchor {
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
  tests: AnchorTest[];
}

export type EmbeddedWallKind = "freeEarth" | "cantilever";

export interface EmbeddedWall {
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

export interface PileGroup {
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

export type LabTestKind = "oedometer" | "triaxial" | "directShear";

export interface LabTest {
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
  points: LabPoint[];
}

export interface LayerWater {
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

export interface Placement {
  id: string;
  x: number;
  y: number;
  z: number;
  yaw: number;
}

export interface CaseRow {
  id: string;
  labelEn: string;
  labelDe: string;
  entryId: string;
  numbers: CaseNumber[];
  texts: CaseText[];
}

export interface CaseNumber {
  id: string;
  value: number;
}

export interface CaseText {
  id: string;
  value: string;
}

export interface LabPoint {
  id: string;
  normalStress: number;
  shearStress: number;
}

export interface AnchorTest {
  id: string;
  pullOutResistance: number;
}

export interface PileTestProfile {
  id: string;
  shaftResistance: number;
  baseResistance: number;
}

export type SituationChoice = "persistent" | "transient" | "accidental";

export interface FoundationLoadCase {
  id: string;
  designSituation: SituationChoice;
  verticalPermanent: number;
  verticalVariable: number;
  horizontalPermanent: number;
  horizontalVariable: number;
  momentPermanent: number;
  momentVariable: number;
}

export type InvestigationKind = "borehole" | "cpt" | "spt";

export interface InvestigationReading {
  id: string;
  depth: number;
  measurement: number;
}

export interface Investigation {
  id: string;
  positionX: number;
  positionY: number;
  groundLevel: number;
  depth: number;
  kind: InvestigationKind;
  readings: InvestigationReading[];
}

export type CharacteristicParameter = "undrainedStrength" | "frictionAngle";

export type ParameterBasis = "declared" | "derived";

export interface CharacteristicBasis {
  id: string;
  layerId: string;
  parameter: CharacteristicParameter;
  basis: ParameterBasis;
}

export const parseEn1997Artifact: NormWireReader<En1997Artifact> = normWireObject<En1997Artifact>({ structureId: normWireRequired(normWireString), geotechnicalCategory: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), designSituation: normWireRequired(normWireString), designApproach: normWireRequired(normWireString), annex: normWireRequired(normWireLiteral("En", "De")), groundwaterLevel: normWireRequired(normWireNumber), investigationDepth: normWireRequired(normWireNumber), layers: normWireRequired(normWireArray(normWireRef(() => parseSoilLayer))), footings: normWireRequired(normWireArray(normWireRef(() => parseSpreadFoundation))), piles: normWireRequired(normWireArray(normWireRef(() => parsePile))), retainingWalls: normWireRequired(normWireArray(normWireRef(() => parseRetainingWall))), slopes: normWireRequired(normWireArray(normWireRef(() => parseSlope))), upliftCases: normWireRequired(normWireArray(normWireRef(() => parseUpliftCase))), anchors: normWireRequired(normWireArray(normWireRef(() => parseAnchor))), embeddedWalls: normWireRequired(normWireArray(normWireRef(() => parseEmbeddedWall))), pileGroups: normWireRequired(normWireArray(normWireRef(() => parsePileGroup))), labTests: normWireRequired(normWireArray(normWireRef(() => parseLabTest))), layerWaters: normWireRequired(normWireArray(normWireRef(() => parseLayerWater))), placements: normWireRequired(normWireArray(normWireRef(() => parsePlacement))), cases: normWireRequired(normWireArray(normWireRef(() => parseCaseRow))), investigations: normWireRequired(normWireArray(normWireRef(() => parseInvestigation))), characteristicBases: normWireRequired(normWireArray(normWireRef(() => parseCharacteristicBasis))) });
export const parseSoilType: NormWireReader<SoilType> = normWireLiteral("sand", "gravel", "clay", "silt", "fill");
export const parseSoilLayer: NormWireReader<SoilLayer> = normWireObject<SoilLayer>({ id: normWireRequired(normWireString), soilType: normWireRequired(normWireRef(() => parseSoilType)), depthTop: normWireRequired(normWireNumber), depthBottom: normWireRequired(normWireNumber), gamma: normWireRequired(normWireNumber), gammaPrime: normWireRequired(normWireNumber), phiPrimeDeg: normWireRequired(normWireNumber), cohesionEffective: normWireRequired(normWireNumber), cohesionUndrained: normWireRequired(normWireNumber), oedometricModulus: normWireRequired(normWireNumber), poissonRatio: normWireRequired(normWireNumber), cptQc: normWireRequired(normWireNumber), sptN: normWireRequired(normWireNumber), cptNkt: normWireRequired(normWireNumber) });
export const parseSpreadFoundation: NormWireReader<SpreadFoundation> = normWireObject<SpreadFoundation>({ id: normWireRequired(normWireString), width: normWireRequired(normWireNumber), length: normWireRequired(normWireNumber), embedment: normWireRequired(normWireNumber), baseInclinationDeg: normWireRequired(normWireNumber), settlementLimit: normWireRequired(normWireNumber), loadCases: normWireRequired(normWireArray(normWireRef(() => parseFoundationLoadCase))) });
export const parsePileType: NormWireReader<PileType> = normWireLiteral("bored", "driven", "cfa");
export const parsePile: NormWireReader<Pile> = normWireObject<Pile>({ id: normWireRequired(normWireString), pileType: normWireRequired(normWireRef(() => parsePileType)), diameter: normWireRequired(normWireNumber), length: normWireRequired(normWireNumber), count: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), alphaS: normWireRequired(normWireNumber), unitShaftResistance: normWireRequired(normWireNumber), unitBaseResistance: normWireRequired(normWireNumber), compressionPermanent: normWireRequired(normWireNumber), compressionVariable: normWireRequired(normWireNumber), tensionPermanent: normWireRequired(normWireNumber), tensionVariable: normWireRequired(normWireNumber), bendingStiffnessEi: normWireRequired(normWireNumber), horizontalSubgradeModulus: normWireRequired(normWireNumber), lateralHeadLoad: normWireRequired(normWireNumber), elasticModulus: normWireRequired(normWireNumber), settlingThickness: normWireRequired(normWireNumber), negativeFrictionBeta: normWireRequired(normWireNumber), dynamicTestCount: normWireRequired(normWireNumber), dynamicResistanceMean: normWireRequired(normWireNumber), dynamicResistanceMin: normWireRequired(normWireNumber), hammerWeight: normWireRequired(normWireNumber), dropHeight: normWireRequired(normWireNumber), setPerBlow: normWireRequired(normWireNumber), elasticCompression: normWireRequired(normWireNumber), hammerEfficiency: normWireRequired(normWireNumber), concreteClass: normWireRequired(normWireRef(() => parsePileConcreteClass)), testProfiles: normWireRequired(normWireArray(normWireRef(() => parsePileTestProfile))) });
export const parsePileConcreteClass: NormWireReader<PileConcreteClass> = normWireLiteral("notGiven", "c1215", "c1620", "c2025", "c2530", "c3037", "c3545", "c4050", "c4555", "c5060");
export const parseEarthPressureMode: NormWireReader<EarthPressureMode> = normWireLiteral("active", "atRest", "increasedActive");
export const parseWallMovement: NormWireReader<WallMovement> = normWireLiteral("free", "rigid", "propped");
export const parseRetainingWall: NormWireReader<RetainingWall> = normWireObject<RetainingWall>({ id: normWireRequired(normWireString), height: normWireRequired(normWireNumber), embedment: normWireRequired(normWireNumber), baseWidth: normWireRequired(normWireNumber), stemThickness: normWireRequired(normWireNumber), backfillPhiDeg: normWireRequired(normWireNumber), backfillGamma: normWireRequired(normWireNumber), wallFrictionDeg: normWireRequired(normWireNumber), earthPressureMode: normWireRequired(normWireRef(() => parseEarthPressureMode)), wallMovement: normWireRequired(normWireRef(() => parseWallMovement)), ocr: normWireRequired(normWireNumber), concreteGamma: normWireRequired(normWireNumber), surcharge: normWireRequired(normWireNumber), verticalPermanent: normWireRequired(normWireNumber), horizontalPermanent: normWireRequired(normWireNumber), seismicKh: normWireRequired(normWireNumber), seismicKv: normWireRequired(normWireNumber), reinforcementSpacing: normWireRequired(normWireNumber), reinforcementWidth: normWireRequired(normWireNumber), reinforcementFriction: normWireRequired(normWireNumber) });
export const parseSlope: NormWireReader<Slope> = normWireObject<Slope>({ id: normWireRequired(normWireString), angleDeg: normWireRequired(normWireNumber), height: normWireRequired(normWireNumber), length: normWireRequired(normWireNumber), governingLayerId: normWireRequired(normWireString), circleCentreX: normWireRequired(normWireNumber), circleCentreZ: normWireRequired(normWireNumber), circleRadius: normWireRequired(normWireNumber), width: normWireRequired(normWireNumber) });
export const parseUpliftCase: NormWireReader<UpliftCase> = normWireObject<UpliftCase>({ id: normWireRequired(normWireString), permanentStabilizing: normWireRequired(normWireNumber), permanentDestabilizing: normWireRequired(normWireNumber), variableDestabilizing: normWireRequired(normWireNumber), porePressure: normWireRequired(normWireNumber), totalStress: normWireRequired(normWireNumber) });
export const parseTendonType: NormWireReader<TendonType> = normWireLiteral("strand", "bar");
export const parseAnchor: NormWireReader<Anchor> = normWireObject<Anchor>({ id: normWireRequired(normWireString), wallId: normWireRequired(normWireString), tendon: normWireRequired(normWireRef(() => parseTendonType)), headDepth: normWireRequired(normWireNumber), inclinationDeg: normWireRequired(normWireNumber), freeLength: normWireRequired(normWireNumber), fixedLength: normWireRequired(normWireNumber), fixedDiameter: normWireRequired(normWireNumber), bondStress: normWireRequired(normWireNumber), designForce: normWireRequired(normWireNumber), tendonYieldForce: normWireRequired(normWireNumber), minFreeLength: normWireRequired(normWireNumber), testFactor: normWireRequired(normWireNumber), tests: normWireRequired(normWireArray(normWireRef(() => parseAnchorTest))) });
export const parseEmbeddedWallKind: NormWireReader<EmbeddedWallKind> = normWireLiteral("freeEarth", "cantilever");
export const parseEmbeddedWall: NormWireReader<EmbeddedWall> = normWireObject<EmbeddedWall>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseEmbeddedWallKind)), retainedHeight: normWireRequired(normWireNumber), embedment: normWireRequired(normWireNumber), propDepth: normWireRequired(normWireNumber), thickness: normWireRequired(normWireNumber), backfillGamma: normWireRequired(normWireNumber), phiDeg: normWireRequired(normWireNumber), deltaActiveDeg: normWireRequired(normWireNumber), deltaPassiveDeg: normWireRequired(normWireNumber), surcharge: normWireRequired(normWireNumber), undrainedStrength: normWireRequired(normWireNumber) });
export const parsePileGroup: NormWireReader<PileGroup> = normWireObject<PileGroup>({ id: normWireRequired(normWireString), pileId: normWireRequired(normWireString), rows: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), columns: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), spacing: normWireRequired(normWireNumber), capVertical: normWireRequired(normWireNumber), capMomentX: normWireRequired(normWireNumber), capMomentY: normWireRequired(normWireNumber), singleResistance: normWireRequired(normWireNumber), shaftCohesionSum: normWireRequired(normWireNumber), baseCohesion: normWireRequired(normWireNumber) });
export const parseLabTestKind: NormWireReader<LabTestKind> = normWireLiteral("oedometer", "triaxial", "directShear");
export const parseLabTest: NormWireReader<LabTest> = normWireObject<LabTest>({ id: normWireRequired(normWireString), layerId: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseLabTestKind)), voidRatio: normWireRequired(normWireNumber), preloadStress: normWireRequired(normWireNumber), drainageHeight: normWireRequired(normWireNumber), consolidationTime: normWireRequired(normWireNumber), compressionIndex: normWireRequired(normWireNumber), swellingIndex: normWireRequired(normWireNumber), stressIncrement: normWireRequired(normWireNumber), secondaryCompressionIndex: normWireRequired(normWireNumber), designLife: normWireRequired(normWireNumber), points: normWireRequired(normWireArray(normWireRef(() => parseLabPoint))) });
export const parseLayerWater: NormWireReader<LayerWater> = normWireObject<LayerWater>({ id: normWireRequired(normWireString), layerId: normWireRequired(normWireString), piezometricDepth: normWireRequired(normWireNumber), permeability: normWireRequired(normWireNumber), headDifference: normWireRequired(normWireNumber), flowLength: normWireRequired(normWireNumber), wellRadius: normWireRequired(normWireNumber), wellWaterDepth: normWireRequired(normWireNumber), influenceRadius: normWireRequired(normWireNumber), wellCapacity: normWireRequired(normWireNumber) });
export const parsePlacement: NormWireReader<Placement> = normWireObject<Placement>({ id: normWireRequired(normWireString), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber), z: normWireRequired(normWireNumber), yaw: normWireRequired(normWireNumber) });
export const parseCaseRow: NormWireReader<CaseRow> = normWireObject<CaseRow>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), entryId: normWireRequired(normWireString), numbers: normWireRequired(normWireArray(normWireRef(() => parseCaseNumber))), texts: normWireRequired(normWireArray(normWireRef(() => parseCaseText))) });
export const parseCaseNumber: NormWireReader<CaseNumber> = normWireObject<CaseNumber>({ id: normWireRequired(normWireString), value: normWireRequired(normWireNumber) });
export const parseCaseText: NormWireReader<CaseText> = normWireObject<CaseText>({ id: normWireRequired(normWireString), value: normWireRequired(normWireString) });
export const parseLabPoint: NormWireReader<LabPoint> = normWireObject<LabPoint>({ id: normWireRequired(normWireString), normalStress: normWireRequired(normWireNumber), shearStress: normWireRequired(normWireNumber) });
export const parseAnchorTest: NormWireReader<AnchorTest> = normWireObject<AnchorTest>({ id: normWireRequired(normWireString), pullOutResistance: normWireRequired(normWireNumber) });
export const parsePileTestProfile: NormWireReader<PileTestProfile> = normWireObject<PileTestProfile>({ id: normWireRequired(normWireString), shaftResistance: normWireRequired(normWireNumber), baseResistance: normWireRequired(normWireNumber) });
export const parseSituationChoice: NormWireReader<SituationChoice> = normWireLiteral("persistent", "transient", "accidental");
export const parseFoundationLoadCase: NormWireReader<FoundationLoadCase> = normWireObject<FoundationLoadCase>({ id: normWireRequired(normWireString), designSituation: normWireRequired(normWireRef(() => parseSituationChoice)), verticalPermanent: normWireRequired(normWireNumber), verticalVariable: normWireRequired(normWireNumber), horizontalPermanent: normWireRequired(normWireNumber), horizontalVariable: normWireRequired(normWireNumber), momentPermanent: normWireRequired(normWireNumber), momentVariable: normWireRequired(normWireNumber) });
export const parseInvestigationKind: NormWireReader<InvestigationKind> = normWireLiteral("borehole", "cpt", "spt");
export const parseInvestigationReading: NormWireReader<InvestigationReading> = normWireObject<InvestigationReading>({ id: normWireRequired(normWireString), depth: normWireRequired(normWireNumber), measurement: normWireRequired(normWireNumber) });
export const parseInvestigation: NormWireReader<Investigation> = normWireObject<Investigation>({ id: normWireRequired(normWireString), positionX: normWireRequired(normWireNumber), positionY: normWireRequired(normWireNumber), groundLevel: normWireRequired(normWireNumber), depth: normWireRequired(normWireNumber), kind: normWireRequired(normWireRef(() => parseInvestigationKind)), readings: normWireRequired(normWireArray(normWireRef(() => parseInvestigationReading))) });
export const parseCharacteristicParameter: NormWireReader<CharacteristicParameter> = normWireLiteral("undrainedStrength", "frictionAngle");
export const parseParameterBasis: NormWireReader<ParameterBasis> = normWireLiteral("declared", "derived");
export const parseCharacteristicBasis: NormWireReader<CharacteristicBasis> = normWireObject<CharacteristicBasis>({ id: normWireRequired(normWireString), layerId: normWireRequired(normWireString), parameter: normWireRequired(normWireRef(() => parseCharacteristicParameter)), basis: normWireRequired(normWireRef(() => parseParameterBasis)) });
