/** 📸️ `En1997Snapshot` wire twin: the persisted snapshot and every record it holds, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1997Snapshot {
  /** @state artifact */
  structureId: string;
  /** @state artifact */
  geotechnicalCategory: number;
  /** @state artifact */
  designSituation: string;
  /** @state artifact */
  designApproach: string;
  /** @state artifact */
  annex: AnnexChoice;
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

export interface SoilLayer {
  id: string;
  soilType: string;
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
}

export type AnnexChoice = "En" | "De";

export interface Pile {
  id: string;
  pileType: string;
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
  testProfiles: PileTestProfile[];
}

export interface FoundationLoadCase {
  id: string;
  designSituation: string;
  verticalPermanent: number;
  verticalVariable: number;
  horizontalPermanent: number;
  horizontalVariable: number;
  momentPermanent: number;
  momentVariable: number;
}

export interface PileTestProfile {
  id: string;
  shaftResistance: number;
  baseResistance: number;
}

export interface RetainingWall {
  id: string;
  height: number;
  embedment: number;
  baseWidth: number;
  stemThickness: number;
  backfillPhiDeg: number;
  backfillGamma: number;
  wallFrictionDeg: number;
  earthPressureMode: string;
  wallMovement: string;
  ocr: number;
  concreteGamma: number;
  surcharge: number;
  verticalPermanent: number;
  horizontalPermanent: number;
}

export interface Slope {
  id: string;
  angleDeg: number;
  height: number;
  length: number;
  governingLayerId: string;
}

export interface UpliftCase {
  id: string;
  permanentStabilizing: number;
  permanentDestabilizing: number;
  variableDestabilizing: number;
  porePressure: number;
  totalStress: number;
}

export const parseEn1997Snapshot: NormWireReader<En1997Snapshot> = normWireObject<En1997Snapshot>({ structureId: normWireRequired(normWireString), geotechnicalCategory: normWireRequired(normWireInteger), designSituation: normWireRequired(normWireString), designApproach: normWireRequired(normWireString), annex: normWireRequired(normWireRef(() => parseAnnexChoice)), groundwaterLevel: normWireRequired(normWireNumber), investigationDepth: normWireRequired(normWireNumber), layers: normWireRequired(normWireArray(normWireRef(() => parseSoilLayer))), footings: normWireRequired(normWireArray(normWireRef(() => parseSpreadFoundation))), piles: normWireRequired(normWireArray(normWireRef(() => parsePile))), retainingWalls: normWireRequired(normWireArray(normWireRef(() => parseRetainingWall))), slopes: normWireRequired(normWireArray(normWireRef(() => parseSlope))), upliftCases: normWireRequired(normWireArray(normWireRef(() => parseUpliftCase))) });
export const parseSpreadFoundation: NormWireReader<SpreadFoundation> = normWireObject<SpreadFoundation>({ id: normWireRequired(normWireString), width: normWireRequired(normWireNumber), length: normWireRequired(normWireNumber), embedment: normWireRequired(normWireNumber), baseInclinationDeg: normWireRequired(normWireNumber), settlementLimit: normWireRequired(normWireNumber), loadCases: normWireRequired(normWireArray(normWireRef(() => parseFoundationLoadCase))) });
export const parseSoilLayer: NormWireReader<SoilLayer> = normWireObject<SoilLayer>({ id: normWireRequired(normWireString), soilType: normWireRequired(normWireString), depthTop: normWireRequired(normWireNumber), depthBottom: normWireRequired(normWireNumber), gamma: normWireRequired(normWireNumber), gammaPrime: normWireRequired(normWireNumber), phiPrimeDeg: normWireRequired(normWireNumber), cohesionEffective: normWireRequired(normWireNumber), cohesionUndrained: normWireRequired(normWireNumber), oedometricModulus: normWireRequired(normWireNumber), poissonRatio: normWireRequired(normWireNumber), cptQc: normWireRequired(normWireNumber), sptN: normWireRequired(normWireNumber) });
export const parseAnnexChoice: NormWireReader<AnnexChoice> = normWireLiteral("En", "De");
export const parsePile: NormWireReader<Pile> = normWireObject<Pile>({ id: normWireRequired(normWireString), pileType: normWireRequired(normWireString), diameter: normWireRequired(normWireNumber), length: normWireRequired(normWireNumber), count: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), alphaS: normWireRequired(normWireNumber), unitShaftResistance: normWireRequired(normWireNumber), unitBaseResistance: normWireRequired(normWireNumber), compressionPermanent: normWireRequired(normWireNumber), compressionVariable: normWireRequired(normWireNumber), tensionPermanent: normWireRequired(normWireNumber), tensionVariable: normWireRequired(normWireNumber), testProfiles: normWireRequired(normWireArray(normWireRef(() => parsePileTestProfile))) });
export const parseFoundationLoadCase: NormWireReader<FoundationLoadCase> = normWireObject<FoundationLoadCase>({ id: normWireRequired(normWireString), designSituation: normWireRequired(normWireString), verticalPermanent: normWireRequired(normWireNumber), verticalVariable: normWireRequired(normWireNumber), horizontalPermanent: normWireRequired(normWireNumber), horizontalVariable: normWireRequired(normWireNumber), momentPermanent: normWireRequired(normWireNumber), momentVariable: normWireRequired(normWireNumber) });
export const parsePileTestProfile: NormWireReader<PileTestProfile> = normWireObject<PileTestProfile>({ id: normWireRequired(normWireString), shaftResistance: normWireRequired(normWireNumber), baseResistance: normWireRequired(normWireNumber) });
export const parseRetainingWall: NormWireReader<RetainingWall> = normWireObject<RetainingWall>({ id: normWireRequired(normWireString), height: normWireRequired(normWireNumber), embedment: normWireRequired(normWireNumber), baseWidth: normWireRequired(normWireNumber), stemThickness: normWireRequired(normWireNumber), backfillPhiDeg: normWireRequired(normWireNumber), backfillGamma: normWireRequired(normWireNumber), wallFrictionDeg: normWireRequired(normWireNumber), earthPressureMode: normWireRequired(normWireString), wallMovement: normWireRequired(normWireString), ocr: normWireRequired(normWireNumber), concreteGamma: normWireRequired(normWireNumber), surcharge: normWireRequired(normWireNumber), verticalPermanent: normWireRequired(normWireNumber), horizontalPermanent: normWireRequired(normWireNumber) });
export const parseSlope: NormWireReader<Slope> = normWireObject<Slope>({ id: normWireRequired(normWireString), angleDeg: normWireRequired(normWireNumber), height: normWireRequired(normWireNumber), length: normWireRequired(normWireNumber), governingLayerId: normWireRequired(normWireString) });
export const parseUpliftCase: NormWireReader<UpliftCase> = normWireObject<UpliftCase>({ id: normWireRequired(normWireString), permanentStabilizing: normWireRequired(normWireNumber), permanentDestabilizing: normWireRequired(normWireNumber), variableDestabilizing: normWireRequired(normWireNumber), porePressure: normWireRequired(normWireNumber), totalStress: normWireRequired(normWireNumber) });
