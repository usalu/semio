/** 🔺️ `En1997Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireInteger, normWireNullable, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type AnnexChoice, parseAnnexChoice, parsePile, parseRetainingWall, parseSlope, parseSoilLayer, parseSpreadFoundation, parseUpliftCase, type Pile, type RetainingWall, type Slope, type SoilLayer, type SpreadFoundation, type UpliftCase } from "../📸️snapshot/🟦️.ts";

export interface En1997Diff {
  /** @state artifact */
  structureId: string | null;
  /** @state artifact */
  geotechnicalCategory: number | null;
  /** @state artifact */
  designSituation: string | null;
  /** @state artifact */
  designApproach: string | null;
  /** @state artifact */
  annex: AnnexChoice | null;
  /** @state artifact */
  groundwaterLevel: number | null;
  /** @state artifact */
  investigationDepth: number | null;
  /** @state artifact */
  layers: En1997LayersRows | null;
  /** @state artifact */
  footings: En1997FootingsRows | null;
  /** @state artifact */
  piles: En1997PilesRows | null;
  /** @state artifact */
  retainingWalls: En1997RetainingWallsRows | null;
  /** @state artifact */
  slopes: En1997SlopesRows | null;
  /** @state artifact */
  upliftCases: En1997UpliftCasesRows | null;
}

export interface En1997FootingsPatch {
  id: string;
  width: number | null;
  embedment: number | null;
}

export interface En1997FootingsRows {
  added: SpreadFoundation[];
  removed: string[];
  modified: En1997FootingsPatch[];
  order: string[] | null;
}

export interface En1997LayersPatch {
  id: string;
  oedometricModulus: number | null;
  phiPrimeDeg: number | null;
}

export interface En1997LayersRows {
  added: SoilLayer[];
  removed: string[];
  modified: En1997LayersPatch[];
  order: string[] | null;
}

export interface En1997PilesPatch {
  id: string;
  length: number | null;
  count: number | null;
}

export interface En1997PilesRows {
  added: Pile[];
  removed: string[];
  modified: En1997PilesPatch[];
  order: string[] | null;
}

export interface En1997RetainingWallsPatch {
  id: string;
  baseWidth: number | null;
}

export interface En1997RetainingWallsRows {
  added: RetainingWall[];
  removed: string[];
  modified: En1997RetainingWallsPatch[];
  order: string[] | null;
}

export interface En1997SlopesPatch {
  id: string;
  angleDeg: number | null;
}

export interface En1997SlopesRows {
  added: Slope[];
  removed: string[];
  modified: En1997SlopesPatch[];
  order: string[] | null;
}

export interface En1997UpliftCasesRows {
  added: UpliftCase[];
  removed: string[];
  order: string[] | null;
}

export const parseEn1997Diff: NormWireReader<En1997Diff> = normWireObject<En1997Diff>({ structureId: normWireDefault(normWireNullable(normWireString), () => null), geotechnicalCategory: normWireDefault(normWireNullable(normWireInteger), () => null), designSituation: normWireDefault(normWireNullable(normWireString), () => null), designApproach: normWireDefault(normWireNullable(normWireString), () => null), annex: normWireDefault(normWireNullable(parseAnnexChoice), () => null), groundwaterLevel: normWireDefault(normWireNullable(normWireNumber), () => null), investigationDepth: normWireDefault(normWireNullable(normWireNumber), () => null), layers: normWireDefault(normWireNullable(normWireRef(() => parseEn1997LayersRows)), () => null), footings: normWireDefault(normWireNullable(normWireRef(() => parseEn1997FootingsRows)), () => null), piles: normWireDefault(normWireNullable(normWireRef(() => parseEn1997PilesRows)), () => null), retainingWalls: normWireDefault(normWireNullable(normWireRef(() => parseEn1997RetainingWallsRows)), () => null), slopes: normWireDefault(normWireNullable(normWireRef(() => parseEn1997SlopesRows)), () => null), upliftCases: normWireDefault(normWireNullable(normWireRef(() => parseEn1997UpliftCasesRows)), () => null) });
export const parseEn1997FootingsPatch: NormWireReader<En1997FootingsPatch> = normWireObject<En1997FootingsPatch>({ id: normWireRequired(normWireString), width: normWireRequired(normWireNullable(normWireNumber)), embedment: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1997FootingsRows: NormWireReader<En1997FootingsRows> = normWireObject<En1997FootingsRows>({ added: normWireRequired(normWireArray(parseSpreadFoundation)), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997FootingsPatch))), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseEn1997LayersPatch: NormWireReader<En1997LayersPatch> = normWireObject<En1997LayersPatch>({ id: normWireRequired(normWireString), oedometricModulus: normWireRequired(normWireNullable(normWireNumber)), phiPrimeDeg: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1997LayersRows: NormWireReader<En1997LayersRows> = normWireObject<En1997LayersRows>({ added: normWireRequired(normWireArray(parseSoilLayer)), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997LayersPatch))), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseEn1997PilesPatch: NormWireReader<En1997PilesPatch> = normWireObject<En1997PilesPatch>({ id: normWireRequired(normWireString), length: normWireRequired(normWireNullable(normWireNumber)), count: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))) });
export const parseEn1997PilesRows: NormWireReader<En1997PilesRows> = normWireObject<En1997PilesRows>({ added: normWireRequired(normWireArray(parsePile)), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997PilesPatch))), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseEn1997RetainingWallsPatch: NormWireReader<En1997RetainingWallsPatch> = normWireObject<En1997RetainingWallsPatch>({ id: normWireRequired(normWireString), baseWidth: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1997RetainingWallsRows: NormWireReader<En1997RetainingWallsRows> = normWireObject<En1997RetainingWallsRows>({ added: normWireRequired(normWireArray(parseRetainingWall)), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997RetainingWallsPatch))), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseEn1997SlopesPatch: NormWireReader<En1997SlopesPatch> = normWireObject<En1997SlopesPatch>({ id: normWireRequired(normWireString), angleDeg: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1997SlopesRows: NormWireReader<En1997SlopesRows> = normWireObject<En1997SlopesRows>({ added: normWireRequired(normWireArray(parseSlope)), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997SlopesPatch))), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseEn1997UpliftCasesRows: NormWireReader<En1997UpliftCasesRows> = normWireObject<En1997UpliftCasesRows>({ added: normWireRequired(normWireArray(parseUpliftCase)), removed: normWireRequired(normWireArray(normWireString)), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
