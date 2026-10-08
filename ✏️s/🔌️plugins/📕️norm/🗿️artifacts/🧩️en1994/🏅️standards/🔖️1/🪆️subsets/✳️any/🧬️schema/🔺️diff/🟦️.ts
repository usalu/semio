/** 🔺️ `En1994Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireInteger, normWireNullable, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type AnnexChoice, type CompositeBeam, type CompositeColumn, type CompositeSlab, parseAnnexChoice, parseCompositeBeam, parseCompositeColumn, parseCompositeSlab } from "../📸️snapshot/🟦️.ts";

export interface En1994Diff {
  /** @state artifact */
  annex: AnnexChoice | null;
  /** @state artifact */
  structureKind: string | null;
  /** @state artifact */
  steelFYPa: number | null;
  /** @state artifact */
  beams: En1994BeamsRows | null;
  /** @state artifact */
  columns: En1994ColumnsRows | null;
  /** @state artifact */
  slabs: En1994SlabsRows | null;
  /** @state artifact */
  fireRating: string | null;
  /** @state artifact */
  insulationThicknessM: number | null;
  /** @state artifact */
  fatigueDetail: string | null;
}

export interface En1994BeamsActionsPatch {
  index: number;
  qAreaPa: number | null;
}

export interface En1994BeamsActionsRows {
  modified: En1994BeamsActionsPatch[];
}

export interface En1994BeamsInserted {
  index: number;
  row: CompositeBeam;
}

export interface En1994BeamsPatch {
  index: number;
  spanM: number | null;
  slabThicknessM: number | null;
  construction: string | null;
  transverseAsM2PerM: number | null;
  studsDiameterM: number | null;
  studsFUPa: number | null;
  studsSpacingM: number | null;
  studsTotalCount: number | null;
  actions: En1994BeamsActionsRows | null;
}

export interface En1994BeamsRows {
  removed: number[];
  inserted: En1994BeamsInserted[];
  modified: En1994BeamsPatch[];
}

export interface En1994ColumnsActionsPatch {
  index: number;
  nKN: number | null;
}

export interface En1994ColumnsActionsRows {
  modified: En1994ColumnsActionsPatch[];
}

export interface En1994ColumnsInserted {
  index: number;
  row: CompositeColumn;
}

export interface En1994ColumnsPatch {
  index: number;
  kind: string | null;
  actions: En1994ColumnsActionsRows | null;
}

export interface En1994ColumnsRows {
  removed: number[];
  inserted: En1994ColumnsInserted[];
  modified: En1994ColumnsPatch[];
}

export interface En1994SlabsActionsPatch {
  index: number;
  qAreaPa: number | null;
}

export interface En1994SlabsActionsRows {
  modified: En1994SlabsActionsPatch[];
}

export interface En1994SlabsInserted {
  index: number;
  row: CompositeSlab;
}

export interface En1994SlabsPatch {
  index: number;
  concreteThicknessM: number | null;
  actions: En1994SlabsActionsRows | null;
}

export interface En1994SlabsRows {
  removed: number[];
  inserted: En1994SlabsInserted[];
  modified: En1994SlabsPatch[];
}

export const parseEn1994Diff: NormWireReader<En1994Diff> = normWireObject<En1994Diff>({ annex: normWireDefault(normWireNullable(parseAnnexChoice), () => null), structureKind: normWireDefault(normWireNullable(normWireString), () => null), steelFYPa: normWireDefault(normWireNullable(normWireNumber), () => null), beams: normWireDefault(normWireNullable(normWireRef(() => parseEn1994BeamsRows)), () => null), columns: normWireDefault(normWireNullable(normWireRef(() => parseEn1994ColumnsRows)), () => null), slabs: normWireDefault(normWireNullable(normWireRef(() => parseEn1994SlabsRows)), () => null), fireRating: normWireDefault(normWireNullable(normWireString), () => null), insulationThicknessM: normWireDefault(normWireNullable(normWireNumber), () => null), fatigueDetail: normWireDefault(normWireNullable(normWireString), () => null) });
export const parseEn1994BeamsActionsPatch: NormWireReader<En1994BeamsActionsPatch> = normWireObject<En1994BeamsActionsPatch>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), qAreaPa: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1994BeamsActionsRows: NormWireReader<En1994BeamsActionsRows> = normWireObject<En1994BeamsActionsRows>({ modified: normWireRequired(normWireArray(normWireRef(() => parseEn1994BeamsActionsPatch))) });
export const parseEn1994BeamsInserted: NormWireReader<En1994BeamsInserted> = normWireObject<En1994BeamsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCompositeBeam) });
export const parseEn1994BeamsPatch: NormWireReader<En1994BeamsPatch> = normWireObject<En1994BeamsPatch>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), spanM: normWireRequired(normWireNullable(normWireNumber)), slabThicknessM: normWireRequired(normWireNullable(normWireNumber)), construction: normWireRequired(normWireNullable(normWireString)), transverseAsM2PerM: normWireRequired(normWireNullable(normWireNumber)), studsDiameterM: normWireRequired(normWireNullable(normWireNumber)), studsFUPa: normWireRequired(normWireNullable(normWireNumber)), studsSpacingM: normWireRequired(normWireNullable(normWireNumber)), studsTotalCount: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), actions: normWireRequired(normWireNullable(normWireRef(() => parseEn1994BeamsActionsRows))) });
export const parseEn1994BeamsRows: NormWireReader<En1994BeamsRows> = normWireObject<En1994BeamsRows>({ removed: normWireRequired(normWireArray(normWireRange(normWireInteger, {"minimum":0}))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1994BeamsInserted))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1994BeamsPatch))) });
export const parseEn1994ColumnsActionsPatch: NormWireReader<En1994ColumnsActionsPatch> = normWireObject<En1994ColumnsActionsPatch>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), nKN: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1994ColumnsActionsRows: NormWireReader<En1994ColumnsActionsRows> = normWireObject<En1994ColumnsActionsRows>({ modified: normWireRequired(normWireArray(normWireRef(() => parseEn1994ColumnsActionsPatch))) });
export const parseEn1994ColumnsInserted: NormWireReader<En1994ColumnsInserted> = normWireObject<En1994ColumnsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCompositeColumn) });
export const parseEn1994ColumnsPatch: NormWireReader<En1994ColumnsPatch> = normWireObject<En1994ColumnsPatch>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), kind: normWireRequired(normWireNullable(normWireString)), actions: normWireRequired(normWireNullable(normWireRef(() => parseEn1994ColumnsActionsRows))) });
export const parseEn1994ColumnsRows: NormWireReader<En1994ColumnsRows> = normWireObject<En1994ColumnsRows>({ removed: normWireRequired(normWireArray(normWireRange(normWireInteger, {"minimum":0}))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1994ColumnsInserted))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1994ColumnsPatch))) });
export const parseEn1994SlabsActionsPatch: NormWireReader<En1994SlabsActionsPatch> = normWireObject<En1994SlabsActionsPatch>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), qAreaPa: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1994SlabsActionsRows: NormWireReader<En1994SlabsActionsRows> = normWireObject<En1994SlabsActionsRows>({ modified: normWireRequired(normWireArray(normWireRef(() => parseEn1994SlabsActionsPatch))) });
export const parseEn1994SlabsInserted: NormWireReader<En1994SlabsInserted> = normWireObject<En1994SlabsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCompositeSlab) });
export const parseEn1994SlabsPatch: NormWireReader<En1994SlabsPatch> = normWireObject<En1994SlabsPatch>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), concreteThicknessM: normWireRequired(normWireNullable(normWireNumber)), actions: normWireRequired(normWireNullable(normWireRef(() => parseEn1994SlabsActionsRows))) });
export const parseEn1994SlabsRows: NormWireReader<En1994SlabsRows> = normWireObject<En1994SlabsRows>({ removed: normWireRequired(normWireArray(normWireRange(normWireInteger, {"minimum":0}))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1994SlabsInserted))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1994SlabsPatch))) });
