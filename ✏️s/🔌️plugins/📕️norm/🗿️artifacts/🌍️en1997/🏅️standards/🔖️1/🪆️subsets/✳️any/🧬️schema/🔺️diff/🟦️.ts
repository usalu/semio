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

export interface En1997FootingsInserted {
  index: number;
  row: SpreadFoundation;
}

export interface En1997FootingsModified {
  id: string;
  patch: En1997FootingsPatch;
}

export interface En1997FootingsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997FootingsPatch {
  width: number | null;
  embedment: number | null;
}

export interface En1997FootingsRemoved {
  id: string;
  index: number;
}

export interface En1997FootingsRows {
  removed: En1997FootingsRemoved[];
  inserted: En1997FootingsInserted[];
  moved: En1997FootingsMoved[];
  modified: En1997FootingsModified[];
}

export interface En1997LayersInserted {
  index: number;
  row: SoilLayer;
}

export interface En1997LayersModified {
  id: string;
  patch: En1997LayersPatch;
}

export interface En1997LayersMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997LayersPatch {
  oedometricModulus: number | null;
  phiPrimeDeg: number | null;
}

export interface En1997LayersRemoved {
  id: string;
  index: number;
}

export interface En1997LayersRows {
  removed: En1997LayersRemoved[];
  inserted: En1997LayersInserted[];
  moved: En1997LayersMoved[];
  modified: En1997LayersModified[];
}

export interface En1997PilesInserted {
  index: number;
  row: Pile;
}

export interface En1997PilesModified {
  id: string;
  patch: En1997PilesPatch;
}

export interface En1997PilesMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997PilesPatch {
  length: number | null;
  count: number | null;
}

export interface En1997PilesRemoved {
  id: string;
  index: number;
}

export interface En1997PilesRows {
  removed: En1997PilesRemoved[];
  inserted: En1997PilesInserted[];
  moved: En1997PilesMoved[];
  modified: En1997PilesModified[];
}

export interface En1997RetainingWallsInserted {
  index: number;
  row: RetainingWall;
}

export interface En1997RetainingWallsModified {
  id: string;
  patch: En1997RetainingWallsPatch;
}

export interface En1997RetainingWallsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997RetainingWallsPatch {
  baseWidth: number | null;
}

export interface En1997RetainingWallsRemoved {
  id: string;
  index: number;
}

export interface En1997RetainingWallsRows {
  removed: En1997RetainingWallsRemoved[];
  inserted: En1997RetainingWallsInserted[];
  moved: En1997RetainingWallsMoved[];
  modified: En1997RetainingWallsModified[];
}

export interface En1997SlopesInserted {
  index: number;
  row: Slope;
}

export interface En1997SlopesModified {
  id: string;
  patch: En1997SlopesPatch;
}

export interface En1997SlopesMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997SlopesPatch {
  angleDeg: number | null;
}

export interface En1997SlopesRemoved {
  id: string;
  index: number;
}

export interface En1997SlopesRows {
  removed: En1997SlopesRemoved[];
  inserted: En1997SlopesInserted[];
  moved: En1997SlopesMoved[];
  modified: En1997SlopesModified[];
}

export interface En1997UpliftCasesInserted {
  index: number;
  row: UpliftCase;
}

export interface En1997UpliftCasesModified {
  id: string;
  patch: En1997UpliftCasesPatch;
}

export interface En1997UpliftCasesMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997UpliftCasesPatch {
  permanentStabilizing: number | null;
  permanentDestabilizing: number | null;
  variableDestabilizing: number | null;
  porePressure: number | null;
  totalStress: number | null;
}

export interface En1997UpliftCasesRemoved {
  id: string;
  index: number;
}

export interface En1997UpliftCasesRows {
  removed: En1997UpliftCasesRemoved[];
  inserted: En1997UpliftCasesInserted[];
  moved: En1997UpliftCasesMoved[];
  modified: En1997UpliftCasesModified[];
}

export const parseEn1997Diff: NormWireReader<En1997Diff> = normWireObject<En1997Diff>({ structureId: normWireDefault(normWireNullable(normWireString), () => null), geotechnicalCategory: normWireDefault(normWireNullable(normWireInteger), () => null), designSituation: normWireDefault(normWireNullable(normWireString), () => null), designApproach: normWireDefault(normWireNullable(normWireString), () => null), annex: normWireDefault(normWireNullable(parseAnnexChoice), () => null), groundwaterLevel: normWireDefault(normWireNullable(normWireNumber), () => null), investigationDepth: normWireDefault(normWireNullable(normWireNumber), () => null), layers: normWireDefault(normWireNullable(normWireRef(() => parseEn1997LayersRows)), () => null), footings: normWireDefault(normWireNullable(normWireRef(() => parseEn1997FootingsRows)), () => null), piles: normWireDefault(normWireNullable(normWireRef(() => parseEn1997PilesRows)), () => null), retainingWalls: normWireDefault(normWireNullable(normWireRef(() => parseEn1997RetainingWallsRows)), () => null), slopes: normWireDefault(normWireNullable(normWireRef(() => parseEn1997SlopesRows)), () => null), upliftCases: normWireDefault(normWireNullable(normWireRef(() => parseEn1997UpliftCasesRows)), () => null) });
export const parseEn1997FootingsInserted: NormWireReader<En1997FootingsInserted> = normWireObject<En1997FootingsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseSpreadFoundation) });
export const parseEn1997FootingsModified: NormWireReader<En1997FootingsModified> = normWireObject<En1997FootingsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997FootingsPatch)) });
export const parseEn1997FootingsMoved: NormWireReader<En1997FootingsMoved> = normWireObject<En1997FootingsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997FootingsPatch: NormWireReader<En1997FootingsPatch> = normWireObject<En1997FootingsPatch>({ width: normWireRequired(normWireNullable(normWireNumber)), embedment: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1997FootingsRemoved: NormWireReader<En1997FootingsRemoved> = normWireObject<En1997FootingsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997FootingsRows: NormWireReader<En1997FootingsRows> = normWireObject<En1997FootingsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997FootingsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997FootingsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997FootingsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997FootingsModified))) });
export const parseEn1997LayersInserted: NormWireReader<En1997LayersInserted> = normWireObject<En1997LayersInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseSoilLayer) });
export const parseEn1997LayersModified: NormWireReader<En1997LayersModified> = normWireObject<En1997LayersModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997LayersPatch)) });
export const parseEn1997LayersMoved: NormWireReader<En1997LayersMoved> = normWireObject<En1997LayersMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997LayersPatch: NormWireReader<En1997LayersPatch> = normWireObject<En1997LayersPatch>({ oedometricModulus: normWireRequired(normWireNullable(normWireNumber)), phiPrimeDeg: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1997LayersRemoved: NormWireReader<En1997LayersRemoved> = normWireObject<En1997LayersRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997LayersRows: NormWireReader<En1997LayersRows> = normWireObject<En1997LayersRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997LayersRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997LayersInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997LayersMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997LayersModified))) });
export const parseEn1997PilesInserted: NormWireReader<En1997PilesInserted> = normWireObject<En1997PilesInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parsePile) });
export const parseEn1997PilesModified: NormWireReader<En1997PilesModified> = normWireObject<En1997PilesModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997PilesPatch)) });
export const parseEn1997PilesMoved: NormWireReader<En1997PilesMoved> = normWireObject<En1997PilesMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997PilesPatch: NormWireReader<En1997PilesPatch> = normWireObject<En1997PilesPatch>({ length: normWireRequired(normWireNullable(normWireNumber)), count: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))) });
export const parseEn1997PilesRemoved: NormWireReader<En1997PilesRemoved> = normWireObject<En1997PilesRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997PilesRows: NormWireReader<En1997PilesRows> = normWireObject<En1997PilesRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997PilesRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997PilesInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997PilesMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997PilesModified))) });
export const parseEn1997RetainingWallsInserted: NormWireReader<En1997RetainingWallsInserted> = normWireObject<En1997RetainingWallsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseRetainingWall) });
export const parseEn1997RetainingWallsModified: NormWireReader<En1997RetainingWallsModified> = normWireObject<En1997RetainingWallsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997RetainingWallsPatch)) });
export const parseEn1997RetainingWallsMoved: NormWireReader<En1997RetainingWallsMoved> = normWireObject<En1997RetainingWallsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997RetainingWallsPatch: NormWireReader<En1997RetainingWallsPatch> = normWireObject<En1997RetainingWallsPatch>({ baseWidth: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1997RetainingWallsRemoved: NormWireReader<En1997RetainingWallsRemoved> = normWireObject<En1997RetainingWallsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997RetainingWallsRows: NormWireReader<En1997RetainingWallsRows> = normWireObject<En1997RetainingWallsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997RetainingWallsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997RetainingWallsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997RetainingWallsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997RetainingWallsModified))) });
export const parseEn1997SlopesInserted: NormWireReader<En1997SlopesInserted> = normWireObject<En1997SlopesInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseSlope) });
export const parseEn1997SlopesModified: NormWireReader<En1997SlopesModified> = normWireObject<En1997SlopesModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997SlopesPatch)) });
export const parseEn1997SlopesMoved: NormWireReader<En1997SlopesMoved> = normWireObject<En1997SlopesMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997SlopesPatch: NormWireReader<En1997SlopesPatch> = normWireObject<En1997SlopesPatch>({ angleDeg: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1997SlopesRemoved: NormWireReader<En1997SlopesRemoved> = normWireObject<En1997SlopesRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997SlopesRows: NormWireReader<En1997SlopesRows> = normWireObject<En1997SlopesRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997SlopesRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997SlopesInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997SlopesMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997SlopesModified))) });
export const parseEn1997UpliftCasesInserted: NormWireReader<En1997UpliftCasesInserted> = normWireObject<En1997UpliftCasesInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseUpliftCase) });
export const parseEn1997UpliftCasesModified: NormWireReader<En1997UpliftCasesModified> = normWireObject<En1997UpliftCasesModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997UpliftCasesPatch)) });
export const parseEn1997UpliftCasesMoved: NormWireReader<En1997UpliftCasesMoved> = normWireObject<En1997UpliftCasesMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997UpliftCasesPatch: NormWireReader<En1997UpliftCasesPatch> = normWireObject<En1997UpliftCasesPatch>({ permanentStabilizing: normWireRequired(normWireNullable(normWireNumber)), permanentDestabilizing: normWireRequired(normWireNullable(normWireNumber)), variableDestabilizing: normWireRequired(normWireNullable(normWireNumber)), porePressure: normWireRequired(normWireNullable(normWireNumber)), totalStress: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1997UpliftCasesRemoved: NormWireReader<En1997UpliftCasesRemoved> = normWireObject<En1997UpliftCasesRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997UpliftCasesRows: NormWireReader<En1997UpliftCasesRows> = normWireObject<En1997UpliftCasesRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997UpliftCasesRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997UpliftCasesInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997UpliftCasesMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997UpliftCasesModified))) });
