/** 🔺️ `En1994Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireOptional, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionKind, parseActionKind, type ActionStage, parseActionStage, type AnnexChoice, parseAnnexChoice, type BucklingCurve, parseBucklingCurve, type CaseNumber, parseCaseNumber, type CaseRow, parseCaseRow, type CaseText, parseCaseText, type CharacteristicAction, parseCharacteristicAction, type ColumnAction, parseColumnAction, type ColumnKind, parseColumnKind, type CompositeBeam, parseCompositeBeam, type CompositeColumn, parseCompositeColumn, type CompositeSlab, parseCompositeSlab, type ConstructionMethod, parseConstructionMethod, type FatigueDetail, parseFatigueDetail, type FireRating, parseFireRating, type Placement, parsePlacement, type ReinforcementProduct, parseReinforcementProduct, type SheetingProfile, parseSheetingProfile, type SlabAction, parseSlabAction, type SteelSection, parseSteelSection, type StructureKind, parseStructureKind, type Stud, parseStud, type SupportCondition, parseSupportCondition } from "../📸️snapshot/🟦️.ts";

export interface En1994Diff {
  /** @state artifact */
  annex?: AnnexChoice | null;
  /** @state artifact */
  structureKind?: StructureKind | null;
  /** @state artifact */
  steelFYPa?: number | null;
  /** @state artifact */
  beams?: En1994BeamsRows | null;
  /** @state artifact */
  columns?: En1994ColumnsRows | null;
  /** @state artifact */
  slabs?: En1994SlabsRows | null;
  /** @state artifact */
  placements?: En1994PlacementsRows | null;
  /** @state artifact */
  studs?: En1994StudsRows | null;
  /** @state artifact */
  fireRating?: FireRating | null;
  /** @state artifact */
  insulationThicknessM?: number | null;
  /** @state artifact */
  fatigueDetail?: FatigueDetail | null;
  /** @state artifact */
  reliabilityClass?: ("RC1" | "RC2" | "RC3") | null;
  /** @state artifact */
  cases?: CaseRowDelta;
}

export interface En1994BeamsActionsInserted {
  index: number;
  row: CharacteristicAction;
}

export interface En1994BeamsActionsModified {
  id: string;
  patch: En1994BeamsActionsPatch;
}

export interface En1994BeamsActionsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1994BeamsActionsPatch {
  kind?: ActionKind | null;
  category?: string | null;
  stage?: ActionStage | null;
  qAreaPa?: number | null;
  fKN?: number | null;
  deltaSigmaKPa?: number | null;
  deltaTauKPa?: number | null;
  xM?: number | null;
}

export interface En1994BeamsActionsRemoved {
  id: string;
  index: number;
}

export interface En1994BeamsActionsRows {
  removed: En1994BeamsActionsRemoved[];
  inserted: En1994BeamsActionsInserted[];
  moved: En1994BeamsActionsMoved[];
  modified: En1994BeamsActionsModified[];
}

export interface En1994BeamsInserted {
  index: number;
  row: CompositeBeam;
}

export interface En1994BeamsModified {
  id: string;
  patch: En1994BeamsPatch;
}

export interface En1994BeamsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1994BeamsPatch {
  spanM?: number | null;
  spacingM?: number | null;
  support?: SupportCondition | null;
  construction?: ConstructionMethod | null;
  steel?: SteelSection | null;
  steelHeightM?: number | null;
  steelWidthM?: number | null;
  steelTwM?: number | null;
  steelTfM?: number | null;
  steelAM2?: number | null;
  steelWPlYM3?: number | null;
  steelIYM4?: number | null;
  steelAVM2?: number | null;
  steelRM?: number | null;
  slabThicknessM?: number | null;
  concreteFCkPa?: number | null;
  concreteECmPa?: number | null;
  sheetingProfile?: SheetingProfile | null;
  sheetingHeightM?: number | null;
  sheetingRibWidthM?: number | null;
  sheetingThicknessM?: number | null;
  sheetingRibsParallelToBeam?: boolean | null;
  transverseAsM2PerM?: number | null;
  ltbLengthM?: number | null;
  asHoggingM2PerM?: number | null;
  barSpacingM?: number | null;
  wkLimitM?: number | null;
  nCycles?: number | null;
  adjacentSpanM?: number | null;
  patchBearingLengthM?: number | null;
  bridgeLanes?: number | null;
  crackedLengthFraction?: number | null;
  studTransverseSpacingM?: number | null;
  studEdgeDistanceM?: number | null;
  transverseAsTopM2PerM?: number | null;
  endPlateWidthM?: number | null;
  endPlateHeightM?: number | null;
  endZoneLengthM?: number | null;
  reinforcementProduct?: ReinforcementProduct | null;
  actions?: En1994BeamsActionsRows | null;
}

export interface En1994BeamsRemoved {
  id: string;
  index: number;
}

export interface En1994BeamsRows {
  removed: En1994BeamsRemoved[];
  inserted: En1994BeamsInserted[];
  moved: En1994BeamsMoved[];
  modified: En1994BeamsModified[];
}

export interface En1994ColumnsActionsInserted {
  index: number;
  row: ColumnAction;
}

export interface En1994ColumnsActionsModified {
  id: string;
  patch: En1994ColumnsActionsPatch;
}

export interface En1994ColumnsActionsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1994ColumnsActionsPatch {
  kind?: ActionKind | null;
  category?: string | null;
  stage?: ActionStage | null;
  nKN?: number | null;
  mKNm?: number | null;
}

export interface En1994ColumnsActionsRemoved {
  id: string;
  index: number;
}

export interface En1994ColumnsActionsRows {
  removed: En1994ColumnsActionsRemoved[];
  inserted: En1994ColumnsActionsInserted[];
  moved: En1994ColumnsActionsMoved[];
  modified: En1994ColumnsActionsModified[];
}

export interface En1994ColumnsInserted {
  index: number;
  row: CompositeColumn;
}

export interface En1994ColumnsModified {
  id: string;
  patch: En1994ColumnsPatch;
}

export interface En1994ColumnsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1994ColumnsPatch {
  kind?: ColumnKind | null;
  lengthM?: number | null;
  outerSizeM?: number | null;
  wallThicknessM?: number | null;
  steelAM2?: number | null;
  steelFYPa?: number | null;
  concreteAM2?: number | null;
  concreteFCkPa?: number | null;
  reinforcementAsM2?: number | null;
  reinforcementFYkPa?: number | null;
  iM4?: number | null;
  bucklingCurve?: BucklingCurve | null;
  introducedForceN?: number | null;
  sectionHeightM?: number | null;
  sectionWidthM?: number | null;
  sectionTwM?: number | null;
  sectionTfM?: number | null;
  rebarCoverM?: number | null;
  reinforcementProduct?: ReinforcementProduct | null;
  actions?: En1994ColumnsActionsRows | null;
}

export interface En1994ColumnsRemoved {
  id: string;
  index: number;
}

export interface En1994ColumnsRows {
  removed: En1994ColumnsRemoved[];
  inserted: En1994ColumnsInserted[];
  moved: En1994ColumnsMoved[];
  modified: En1994ColumnsModified[];
}

export interface En1994PlacementsInserted {
  index: number;
  row: Placement;
}

export interface En1994PlacementsModified {
  id: string;
  patch: En1994PlacementsPatch;
}

export interface En1994PlacementsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1994PlacementsPatch {
  x?: number | null;
  y?: number | null;
  z?: number | null;
  yaw?: number | null;
}

export interface En1994PlacementsRemoved {
  id: string;
  index: number;
}

export interface En1994PlacementsRows {
  removed: En1994PlacementsRemoved[];
  inserted: En1994PlacementsInserted[];
  moved: En1994PlacementsMoved[];
  modified: En1994PlacementsModified[];
}

export interface En1994SlabsActionsInserted {
  index: number;
  row: SlabAction;
}

export interface En1994SlabsActionsModified {
  id: string;
  patch: En1994SlabsActionsPatch;
}

export interface En1994SlabsActionsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1994SlabsActionsPatch {
  kind?: ActionKind | null;
  category?: string | null;
  stage?: ActionStage | null;
  qAreaPa?: number | null;
  fKN?: number | null;
  deltaSigmaKPa?: number | null;
  deltaTauKPa?: number | null;
  xM?: number | null;
  yM?: number | null;
}

export interface En1994SlabsActionsRemoved {
  id: string;
  index: number;
}

export interface En1994SlabsActionsRows {
  removed: En1994SlabsActionsRemoved[];
  inserted: En1994SlabsActionsInserted[];
  moved: En1994SlabsActionsMoved[];
  modified: En1994SlabsActionsModified[];
}

export interface En1994SlabsInserted {
  index: number;
  row: CompositeSlab;
}

export interface En1994SlabsModified {
  id: string;
  patch: En1994SlabsPatch;
}

export interface En1994SlabsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1994SlabsPatch {
  spanM?: number | null;
  support?: SupportCondition | null;
  sheetingProfile?: SheetingProfile | null;
  sheetingHeightM?: number | null;
  sheetingRibWidthM?: number | null;
  sheetingThicknessM?: number | null;
  sheetingRibsParallelToBeam?: boolean | null;
  concreteThicknessM?: number | null;
  fCkPa?: number | null;
  mFactor?: number | null;
  kFactor?: number | null;
  asM2PerM?: number | null;
  actions?: En1994SlabsActionsRows | null;
}

export interface En1994SlabsRemoved {
  id: string;
  index: number;
}

export interface En1994SlabsRows {
  removed: En1994SlabsRemoved[];
  inserted: En1994SlabsInserted[];
  moved: En1994SlabsMoved[];
  modified: En1994SlabsModified[];
}

export interface En1994StudsInserted {
  index: number;
  row: Stud;
}

export interface En1994StudsModified {
  id: string;
  patch: En1994StudsPatch;
}

export interface En1994StudsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1994StudsRemoved {
  id: string;
  index: number;
}

export interface En1994StudsRows {
  removed: En1994StudsRemoved[];
  inserted: En1994StudsInserted[];
  moved: En1994StudsMoved[];
  modified: En1994StudsModified[];
}

export interface En1994StudsPatch {
  beamId?: string | null;
  xM?: number | null;
  diameterM?: number | null;
  heightM?: number | null;
  fUPa?: number | null;
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
  numbers?: CaseNumberDelta;
  texts?: CaseTextDelta;
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

export const parseEn1994Diff: NormWireReader<En1994Diff> = normWireObject<En1994Diff>({ annex: normWireOptional(normWireNullable(parseAnnexChoice)), structureKind: normWireOptional(normWireNullable(parseStructureKind)), steelFYPa: normWireOptional(normWireNullable(normWireNumber)), beams: normWireOptional(normWireNullable(normWireRef(() => parseEn1994BeamsRows))), columns: normWireOptional(normWireNullable(normWireRef(() => parseEn1994ColumnsRows))), slabs: normWireOptional(normWireNullable(normWireRef(() => parseEn1994SlabsRows))), placements: normWireOptional(normWireNullable(normWireRef(() => parseEn1994PlacementsRows))), studs: normWireOptional(normWireNullable(normWireRef(() => parseEn1994StudsRows))), fireRating: normWireOptional(normWireNullable(parseFireRating)), insulationThicknessM: normWireOptional(normWireNullable(normWireNumber)), fatigueDetail: normWireOptional(normWireNullable(parseFatigueDetail)), reliabilityClass: normWireOptional(normWireNullable(normWireLiteral("RC1", "RC2", "RC3"))), cases: normWireOptional(normWireRef(() => parseCaseRowDelta)) });
export const parseEn1994BeamsActionsInserted: NormWireReader<En1994BeamsActionsInserted> = normWireObject<En1994BeamsActionsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCharacteristicAction) });
export const parseEn1994BeamsActionsModified: NormWireReader<En1994BeamsActionsModified> = normWireObject<En1994BeamsActionsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1994BeamsActionsPatch)) });
export const parseEn1994BeamsActionsMoved: NormWireReader<En1994BeamsActionsMoved> = normWireObject<En1994BeamsActionsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1994BeamsActionsPatch: NormWireReader<En1994BeamsActionsPatch> = normWireObject<En1994BeamsActionsPatch>({ kind: normWireOptional(normWireNullable(parseActionKind)), category: normWireOptional(normWireNullable(normWireString)), stage: normWireOptional(normWireNullable(parseActionStage)), qAreaPa: normWireOptional(normWireNullable(normWireNumber)), fKN: normWireOptional(normWireNullable(normWireNumber)), deltaSigmaKPa: normWireOptional(normWireNullable(normWireNumber)), deltaTauKPa: normWireOptional(normWireNullable(normWireNumber)), xM: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1994BeamsActionsRemoved: NormWireReader<En1994BeamsActionsRemoved> = normWireObject<En1994BeamsActionsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1994BeamsActionsRows: NormWireReader<En1994BeamsActionsRows> = normWireObject<En1994BeamsActionsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1994BeamsActionsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1994BeamsActionsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1994BeamsActionsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1994BeamsActionsModified))) });
export const parseEn1994BeamsInserted: NormWireReader<En1994BeamsInserted> = normWireObject<En1994BeamsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCompositeBeam) });
export const parseEn1994BeamsModified: NormWireReader<En1994BeamsModified> = normWireObject<En1994BeamsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1994BeamsPatch)) });
export const parseEn1994BeamsMoved: NormWireReader<En1994BeamsMoved> = normWireObject<En1994BeamsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1994BeamsPatch: NormWireReader<En1994BeamsPatch> = normWireObject<En1994BeamsPatch>({ spanM: normWireOptional(normWireNullable(normWireNumber)), spacingM: normWireOptional(normWireNullable(normWireNumber)), support: normWireOptional(normWireNullable(parseSupportCondition)), construction: normWireOptional(normWireNullable(parseConstructionMethod)), steel: normWireOptional(normWireNullable(parseSteelSection)), steelHeightM: normWireOptional(normWireNullable(normWireNumber)), steelWidthM: normWireOptional(normWireNullable(normWireNumber)), steelTwM: normWireOptional(normWireNullable(normWireNumber)), steelTfM: normWireOptional(normWireNullable(normWireNumber)), steelAM2: normWireOptional(normWireNullable(normWireNumber)), steelWPlYM3: normWireOptional(normWireNullable(normWireNumber)), steelIYM4: normWireOptional(normWireNullable(normWireNumber)), steelAVM2: normWireOptional(normWireNullable(normWireNumber)), steelRM: normWireOptional(normWireNullable(normWireNumber)), slabThicknessM: normWireOptional(normWireNullable(normWireNumber)), concreteFCkPa: normWireOptional(normWireNullable(normWireNumber)), concreteECmPa: normWireOptional(normWireNullable(normWireNumber)), sheetingProfile: normWireOptional(normWireNullable(parseSheetingProfile)), sheetingHeightM: normWireOptional(normWireNullable(normWireNumber)), sheetingRibWidthM: normWireOptional(normWireNullable(normWireNumber)), sheetingThicknessM: normWireOptional(normWireNullable(normWireNumber)), sheetingRibsParallelToBeam: normWireOptional(normWireNullable(normWireBoolean)), transverseAsM2PerM: normWireOptional(normWireNullable(normWireNumber)), ltbLengthM: normWireOptional(normWireNullable(normWireNumber)), asHoggingM2PerM: normWireOptional(normWireNullable(normWireNumber)), barSpacingM: normWireOptional(normWireNullable(normWireNumber)), wkLimitM: normWireOptional(normWireNullable(normWireNumber)), nCycles: normWireOptional(normWireNullable(normWireNumber)), adjacentSpanM: normWireOptional(normWireNullable(normWireNumber)), patchBearingLengthM: normWireOptional(normWireNullable(normWireNumber)), bridgeLanes: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), crackedLengthFraction: normWireOptional(normWireNullable(normWireNumber)), studTransverseSpacingM: normWireOptional(normWireNullable(normWireNumber)), studEdgeDistanceM: normWireOptional(normWireNullable(normWireNumber)), transverseAsTopM2PerM: normWireOptional(normWireNullable(normWireNumber)), endPlateWidthM: normWireOptional(normWireNullable(normWireNumber)), endPlateHeightM: normWireOptional(normWireNullable(normWireNumber)), endZoneLengthM: normWireOptional(normWireNullable(normWireNumber)), reinforcementProduct: normWireOptional(normWireNullable(parseReinforcementProduct)), actions: normWireOptional(normWireNullable(normWireRef(() => parseEn1994BeamsActionsRows))) });
export const parseEn1994BeamsRemoved: NormWireReader<En1994BeamsRemoved> = normWireObject<En1994BeamsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1994BeamsRows: NormWireReader<En1994BeamsRows> = normWireObject<En1994BeamsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1994BeamsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1994BeamsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1994BeamsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1994BeamsModified))) });
export const parseEn1994ColumnsActionsInserted: NormWireReader<En1994ColumnsActionsInserted> = normWireObject<En1994ColumnsActionsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseColumnAction) });
export const parseEn1994ColumnsActionsModified: NormWireReader<En1994ColumnsActionsModified> = normWireObject<En1994ColumnsActionsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1994ColumnsActionsPatch)) });
export const parseEn1994ColumnsActionsMoved: NormWireReader<En1994ColumnsActionsMoved> = normWireObject<En1994ColumnsActionsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1994ColumnsActionsPatch: NormWireReader<En1994ColumnsActionsPatch> = normWireObject<En1994ColumnsActionsPatch>({ kind: normWireOptional(normWireNullable(parseActionKind)), category: normWireOptional(normWireNullable(normWireString)), stage: normWireOptional(normWireNullable(parseActionStage)), nKN: normWireOptional(normWireNullable(normWireNumber)), mKNm: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1994ColumnsActionsRemoved: NormWireReader<En1994ColumnsActionsRemoved> = normWireObject<En1994ColumnsActionsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1994ColumnsActionsRows: NormWireReader<En1994ColumnsActionsRows> = normWireObject<En1994ColumnsActionsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1994ColumnsActionsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1994ColumnsActionsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1994ColumnsActionsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1994ColumnsActionsModified))) });
export const parseEn1994ColumnsInserted: NormWireReader<En1994ColumnsInserted> = normWireObject<En1994ColumnsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCompositeColumn) });
export const parseEn1994ColumnsModified: NormWireReader<En1994ColumnsModified> = normWireObject<En1994ColumnsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1994ColumnsPatch)) });
export const parseEn1994ColumnsMoved: NormWireReader<En1994ColumnsMoved> = normWireObject<En1994ColumnsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1994ColumnsPatch: NormWireReader<En1994ColumnsPatch> = normWireObject<En1994ColumnsPatch>({ kind: normWireOptional(normWireNullable(parseColumnKind)), lengthM: normWireOptional(normWireNullable(normWireNumber)), outerSizeM: normWireOptional(normWireNullable(normWireNumber)), wallThicknessM: normWireOptional(normWireNullable(normWireNumber)), steelAM2: normWireOptional(normWireNullable(normWireNumber)), steelFYPa: normWireOptional(normWireNullable(normWireNumber)), concreteAM2: normWireOptional(normWireNullable(normWireNumber)), concreteFCkPa: normWireOptional(normWireNullable(normWireNumber)), reinforcementAsM2: normWireOptional(normWireNullable(normWireNumber)), reinforcementFYkPa: normWireOptional(normWireNullable(normWireNumber)), iM4: normWireOptional(normWireNullable(normWireNumber)), bucklingCurve: normWireOptional(normWireNullable(parseBucklingCurve)), introducedForceN: normWireOptional(normWireNullable(normWireNumber)), sectionHeightM: normWireOptional(normWireNullable(normWireNumber)), sectionWidthM: normWireOptional(normWireNullable(normWireNumber)), sectionTwM: normWireOptional(normWireNullable(normWireNumber)), sectionTfM: normWireOptional(normWireNullable(normWireNumber)), rebarCoverM: normWireOptional(normWireNullable(normWireNumber)), reinforcementProduct: normWireOptional(normWireNullable(parseReinforcementProduct)), actions: normWireOptional(normWireNullable(normWireRef(() => parseEn1994ColumnsActionsRows))) });
export const parseEn1994ColumnsRemoved: NormWireReader<En1994ColumnsRemoved> = normWireObject<En1994ColumnsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1994ColumnsRows: NormWireReader<En1994ColumnsRows> = normWireObject<En1994ColumnsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1994ColumnsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1994ColumnsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1994ColumnsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1994ColumnsModified))) });
export const parseEn1994PlacementsInserted: NormWireReader<En1994PlacementsInserted> = normWireObject<En1994PlacementsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parsePlacement) });
export const parseEn1994PlacementsModified: NormWireReader<En1994PlacementsModified> = normWireObject<En1994PlacementsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1994PlacementsPatch)) });
export const parseEn1994PlacementsMoved: NormWireReader<En1994PlacementsMoved> = normWireObject<En1994PlacementsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1994PlacementsPatch: NormWireReader<En1994PlacementsPatch> = normWireObject<En1994PlacementsPatch>({ x: normWireOptional(normWireNullable(normWireNumber)), y: normWireOptional(normWireNullable(normWireNumber)), z: normWireOptional(normWireNullable(normWireNumber)), yaw: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1994PlacementsRemoved: NormWireReader<En1994PlacementsRemoved> = normWireObject<En1994PlacementsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1994PlacementsRows: NormWireReader<En1994PlacementsRows> = normWireObject<En1994PlacementsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1994PlacementsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1994PlacementsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1994PlacementsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1994PlacementsModified))) });
export const parseEn1994SlabsActionsInserted: NormWireReader<En1994SlabsActionsInserted> = normWireObject<En1994SlabsActionsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseSlabAction) });
export const parseEn1994SlabsActionsModified: NormWireReader<En1994SlabsActionsModified> = normWireObject<En1994SlabsActionsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1994SlabsActionsPatch)) });
export const parseEn1994SlabsActionsMoved: NormWireReader<En1994SlabsActionsMoved> = normWireObject<En1994SlabsActionsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1994SlabsActionsPatch: NormWireReader<En1994SlabsActionsPatch> = normWireObject<En1994SlabsActionsPatch>({ kind: normWireOptional(normWireNullable(parseActionKind)), category: normWireOptional(normWireNullable(normWireString)), stage: normWireOptional(normWireNullable(parseActionStage)), qAreaPa: normWireOptional(normWireNullable(normWireNumber)), fKN: normWireOptional(normWireNullable(normWireNumber)), deltaSigmaKPa: normWireOptional(normWireNullable(normWireNumber)), deltaTauKPa: normWireOptional(normWireNullable(normWireNumber)), xM: normWireOptional(normWireNullable(normWireNumber)), yM: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1994SlabsActionsRemoved: NormWireReader<En1994SlabsActionsRemoved> = normWireObject<En1994SlabsActionsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1994SlabsActionsRows: NormWireReader<En1994SlabsActionsRows> = normWireObject<En1994SlabsActionsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1994SlabsActionsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1994SlabsActionsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1994SlabsActionsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1994SlabsActionsModified))) });
export const parseEn1994SlabsInserted: NormWireReader<En1994SlabsInserted> = normWireObject<En1994SlabsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCompositeSlab) });
export const parseEn1994SlabsModified: NormWireReader<En1994SlabsModified> = normWireObject<En1994SlabsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1994SlabsPatch)) });
export const parseEn1994SlabsMoved: NormWireReader<En1994SlabsMoved> = normWireObject<En1994SlabsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1994SlabsPatch: NormWireReader<En1994SlabsPatch> = normWireObject<En1994SlabsPatch>({ spanM: normWireOptional(normWireNullable(normWireNumber)), support: normWireOptional(normWireNullable(parseSupportCondition)), sheetingProfile: normWireOptional(normWireNullable(parseSheetingProfile)), sheetingHeightM: normWireOptional(normWireNullable(normWireNumber)), sheetingRibWidthM: normWireOptional(normWireNullable(normWireNumber)), sheetingThicknessM: normWireOptional(normWireNullable(normWireNumber)), sheetingRibsParallelToBeam: normWireOptional(normWireNullable(normWireBoolean)), concreteThicknessM: normWireOptional(normWireNullable(normWireNumber)), fCkPa: normWireOptional(normWireNullable(normWireNumber)), mFactor: normWireOptional(normWireNullable(normWireNumber)), kFactor: normWireOptional(normWireNullable(normWireNumber)), asM2PerM: normWireOptional(normWireNullable(normWireNumber)), actions: normWireOptional(normWireNullable(normWireRef(() => parseEn1994SlabsActionsRows))) });
export const parseEn1994SlabsRemoved: NormWireReader<En1994SlabsRemoved> = normWireObject<En1994SlabsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1994SlabsRows: NormWireReader<En1994SlabsRows> = normWireObject<En1994SlabsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1994SlabsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1994SlabsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1994SlabsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1994SlabsModified))) });
export const parseEn1994StudsInserted: NormWireReader<En1994StudsInserted> = normWireObject<En1994StudsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseStud) });
export const parseEn1994StudsModified: NormWireReader<En1994StudsModified> = normWireObject<En1994StudsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1994StudsPatch)) });
export const parseEn1994StudsMoved: NormWireReader<En1994StudsMoved> = normWireObject<En1994StudsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1994StudsRemoved: NormWireReader<En1994StudsRemoved> = normWireObject<En1994StudsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1994StudsRows: NormWireReader<En1994StudsRows> = normWireObject<En1994StudsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1994StudsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1994StudsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1994StudsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1994StudsModified))) });
export const parseEn1994StudsPatch: NormWireReader<En1994StudsPatch> = normWireObject<En1994StudsPatch>({ beamId: normWireOptional(normWireNullable(normWireString)), xM: normWireOptional(normWireNullable(normWireNumber)), diameterM: normWireOptional(normWireNullable(normWireNumber)), heightM: normWireOptional(normWireNullable(normWireNumber)), fUPa: normWireOptional(normWireNullable(normWireNumber)) });
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
export const parseCaseRowPatch: NormWireReader<CaseRowPatch> = normWireObject<CaseRowPatch>({ labelEn: normWireOptional(normWireNullable(normWireString)), labelDe: normWireOptional(normWireNullable(normWireString)), entryId: normWireOptional(normWireNullable(normWireString)), numbers: normWireOptional(normWireRef(() => parseCaseNumberDelta)), texts: normWireOptional(normWireRef(() => parseCaseTextDelta)) });
export const parseCaseRowRemoval: NormWireReader<CaseRowRemoval> = normWireObject<CaseRowRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseRowInsertion: NormWireReader<CaseRowInsertion> = normWireObject<CaseRowInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCaseRow) });
export const parseCaseRowRelocation: NormWireReader<CaseRowRelocation> = normWireObject<CaseRowRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseRowModification: NormWireReader<CaseRowModification> = normWireObject<CaseRowModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseCaseRowPatch)) });
export const parseCaseRowDelta: NormWireReader<CaseRowDelta> = normWireObject<CaseRowDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseCaseRowRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseCaseRowInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseCaseRowRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseCaseRowModification))) });
