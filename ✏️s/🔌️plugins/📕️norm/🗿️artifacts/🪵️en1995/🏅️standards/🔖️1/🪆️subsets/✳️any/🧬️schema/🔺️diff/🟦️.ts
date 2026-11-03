/** 🔺️ `En1995Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireOptional, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionKind, parseActionKind, type Bracing, parseBracing, type BracingKind, parseBracingKind, type BracingProduct, parseBracingProduct, type CaseNumber, parseCaseNumber, type CaseRow, parseCaseRow, type CaseText, parseCaseText, type CharacteristicAction, parseCharacteristicAction, type ConnectionAction, parseConnectionAction, type Cutout, parseCutout, type CutoutFace, parseCutoutFace, type CutoutKind, parseCutoutKind, type Deck, parseDeck, type Diaphragm, parseDiaphragm, type DiaphragmKind, parseDiaphragmKind, type DiaphragmMethod, parseDiaphragmMethod, type Fastener, parseFastener, type FastenerPosition, parseFastenerPosition, type FastenerType, parseFastenerType, type Floor, parseFloor, type ImposedCategory, parseImposedCategory, type LaminationProduct, parseLaminationProduct, type LayerOrientation, parseLayerOrientation, type LoadDurationClass, parseLoadDurationClass, type MemberRole, parseMemberRole, type Panel, parsePanel, type PanelLayer, parsePanelLayer, type Placement, parsePlacement, type PrestressBarProduct, parsePrestressBarProduct, type StrengthClass, parseStrengthClass, type SupportType, parseSupportType, type Taper, parseTaper, type TaperKind, parseTaperKind, type TensionSide, parseTensionSide, type TimberConnection, parseTimberConnection, type TimberMember, parseTimberMember, type Truss, parseTruss, type TrussKind, parseTrussKind, type UlsEquation, parseUlsEquation } from "../📸️snapshot/🟦️.ts";

export interface En1995Diff {
  /** @state artifact */
  annex?: ("En" | "De") | null;
  /** @state artifact */
  members: En1995MemberDelta;
  /** @state artifact */
  connections: En1995ConnectionDelta;
  /** @state artifact */
  placements: En1995PlacementDelta;
  /** @state artifact */
  tapers: En1995TaperDelta;
  /** @state artifact */
  cutouts: En1995CutoutDelta;
  /** @state artifact */
  fasteners: En1995FastenerDelta;
  /** @state artifact */
  panels: En1995PanelDelta;
  /** @state artifact */
  floors: En1995FloorDelta;
  /** @state artifact */
  diaphragms: En1995DiaphragmDelta;
  /** @state artifact */
  trusses: En1995TrussDelta;
  /** @state artifact */
  bracings: En1995BracingDelta;
  /** @state artifact */
  decks: En1995DeckDelta;
  /** @state artifact */
  ulsEquation?: UlsEquation | null;
  /** @state artifact */
  cases: CaseRowDelta;
  /** @state artifact */
  reliabilityClass?: ("RC1" | "RC2" | "RC3") | null;
}

export interface En1995MemberActionPatch {
  kind?: ActionKind | null;
  category?: ImposedCategory | null;
  loadDuration?: LoadDurationClass | null;
  qLineNPerM?: number | null;
  fPointN?: number | null;
  mKNm?: number | null;
  vKN?: number | null;
  nKN?: number | null;
  nTKN?: number | null;
  fC90KN?: number | null;
}

export interface En1995MemberActionRemoval {
  id: string;
  index: number;
}

export interface En1995MemberActionInsertion {
  index: number;
  row: CharacteristicAction;
}

export interface En1995MemberActionRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1995MemberActionModification {
  id: string;
  patch: En1995MemberActionPatch;
}

export interface En1995MemberActionDelta {
  removed: En1995MemberActionRemoval[];
  inserted: En1995MemberActionInsertion[];
  moved: En1995MemberActionRelocation[];
  modified: En1995MemberActionModification[];
}

export interface En1995ConnectionActionPatch {
  kind?: ActionKind | null;
  loadDuration?: LoadDurationClass | null;
  fKN?: number | null;
}

export interface En1995ConnectionActionRemoval {
  id: string;
  index: number;
}

export interface En1995ConnectionActionInsertion {
  index: number;
  row: ConnectionAction;
}

export interface En1995ConnectionActionRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1995ConnectionActionModification {
  id: string;
  patch: En1995ConnectionActionPatch;
}

export interface En1995ConnectionActionDelta {
  removed: En1995ConnectionActionRemoval[];
  inserted: En1995ConnectionActionInsertion[];
  moved: En1995ConnectionActionRelocation[];
  modified: En1995ConnectionActionModification[];
}

export interface En1995MemberPatch {
  labelEn?: string | null;
  labelDe?: string | null;
  role?: MemberRole | null;
  strengthClass?: StrengthClass | null;
  serviceClass?: number | null;
  support?: SupportType | null;
  bM?: number | null;
  hM?: number | null;
  spanM?: number | null;
  supportLengthM?: number | null;
  bearingLengthM?: number | null;
  bucklingLengthYM?: number | null;
  bucklingLengthZM?: number | null;
  lateralRestraintSpacingM?: number | null;
  notchDepthM?: number | null;
  notchDistanceM?: number | null;
  massKgPerM?: number | null;
  massKgPerM2?: number | null;
  dampingXi?: number | null;
  fireDurationS?: number | null;
  bridgeNObs?: number | null;
  bridgeTLYears?: number | null;
  bridgeBeta?: number | null;
  bridgeA?: number | null;
  bridgeB?: number | null;
  bridgeCrowdPerM2?: number | null;
  actions: En1995MemberActionDelta;
}

export interface En1995MemberRemoval {
  id: string;
  index: number;
}

export interface En1995MemberInsertion {
  index: number;
  row: TimberMember;
}

export interface En1995MemberRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1995MemberModification {
  id: string;
  patch: En1995MemberPatch;
}

export interface En1995MemberDelta {
  removed: En1995MemberRemoval[];
  inserted: En1995MemberInsertion[];
  moved: En1995MemberRelocation[];
  modified: En1995MemberModification[];
}

export interface En1995ConnectionPatch {
  labelEn?: string | null;
  labelDe?: string | null;
  fastenerType?: FastenerType | null;
  strengthClass?: StrengthClass | null;
  serviceClass?: number | null;
  diameterM?: number | null;
  number?: number | null;
  rows?: number | null;
  spacingM?: number | null;
  edgeDistanceM?: number | null;
  endDistanceM?: number | null;
  t1M?: number | null;
  t2M?: number | null;
  steelPlate?: boolean | null;
  steelPlateThicknessM?: number | null;
  shearPlanes?: number | null;
  fUK?: number | null;
  actions: En1995ConnectionActionDelta;
}

export interface En1995ConnectionRemoval {
  id: string;
  index: number;
}

export interface En1995ConnectionInsertion {
  index: number;
  row: TimberConnection;
}

export interface En1995ConnectionRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1995ConnectionModification {
  id: string;
  patch: En1995ConnectionPatch;
}

export interface En1995ConnectionDelta {
  removed: En1995ConnectionRemoval[];
  inserted: En1995ConnectionInsertion[];
  moved: En1995ConnectionRelocation[];
  modified: En1995ConnectionModification[];
}

export interface En1995PlacementPatch {
  x?: number | null;
  y?: number | null;
  z?: number | null;
  yaw?: number | null;
}

export interface En1995PlacementRemoval {
  id: string;
  index: number;
}

export interface En1995PlacementInsertion {
  index: number;
  row: Placement;
}

export interface En1995PlacementRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1995PlacementModification {
  id: string;
  patch: En1995PlacementPatch;
}

export interface En1995PlacementDelta {
  removed: En1995PlacementRemoval[];
  inserted: En1995PlacementInsertion[];
  moved: En1995PlacementRelocation[];
  modified: En1995PlacementModification[];
}

export interface En1995TaperPatch {
  memberId?: string | null;
  kind?: TaperKind | null;
  taperAngleRad?: number | null;
  apexDepthM?: number | null;
  innerRadiusM?: number | null;
  lamellaThicknessM?: number | null;
  apexAngleRad?: number | null;
  tensionSide?: TensionSide | null;
  rods?: number | null;
  rodResistanceN?: number | null;
}

export interface En1995TaperRemoval {
  id: string;
  index: number;
}

export interface En1995TaperInsertion {
  index: number;
  row: Taper;
}

export interface En1995TaperRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1995TaperModification {
  id: string;
  patch: En1995TaperPatch;
}

export interface En1995TaperDelta {
  removed: En1995TaperRemoval[];
  inserted: En1995TaperInsertion[];
  moved: En1995TaperRelocation[];
  modified: En1995TaperModification[];
}

export interface En1995CutoutPatch {
  memberId?: string | null;
  kind?: CutoutKind | null;
  face?: CutoutFace | null;
  distanceM?: number | null;
  lengthM?: number | null;
  depthM?: number | null;
  centreHeightM?: number | null;
  slope?: number | null;
  rods?: number | null;
  rodResistanceN?: number | null;
}

export interface En1995CutoutRemoval {
  id: string;
  index: number;
}

export interface En1995CutoutInsertion {
  index: number;
  row: Cutout;
}

export interface En1995CutoutRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1995CutoutModification {
  id: string;
  patch: En1995CutoutPatch;
}

export interface En1995CutoutDelta {
  removed: En1995CutoutRemoval[];
  inserted: En1995CutoutInsertion[];
  moved: En1995CutoutRelocation[];
  modified: En1995CutoutModification[];
}

export interface En1995FastenerPatch {
  connectionId?: string | null;
  memberId?: string | null;
  columns?: number | null;
  lines?: number | null;
  spacingParallelM?: number | null;
  spacingPerpendicularM?: number | null;
  endDistanceM?: number | null;
  edgeDistanceM?: number | null;
  offsetXM?: number | null;
  offsetYM?: number | null;
  loadAngleRad?: number | null;
  positions: En1995FastenerPositionDelta;
}

export interface En1995FastenerRemoval {
  id: string;
  index: number;
}

export interface En1995FastenerInsertion {
  index: number;
  row: Fastener;
}

export interface En1995FastenerRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1995FastenerModification {
  id: string;
  patch: En1995FastenerPatch;
}

export interface En1995FastenerDelta {
  removed: En1995FastenerRemoval[];
  inserted: En1995FastenerInsertion[];
  moved: En1995FastenerRelocation[];
  modified: En1995FastenerModification[];
}

export interface En1995FastenerPositionPatch {
  xM?: number | null;
  yM?: number | null;
}

export interface En1995FastenerPositionRemoval {
  id: string;
  index: number;
}

export interface En1995FastenerPositionInsertion {
  index: number;
  row: FastenerPosition;
}

export interface En1995FastenerPositionRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1995FastenerPositionModification {
  id: string;
  patch: En1995FastenerPositionPatch;
}

export interface En1995FastenerPositionDelta {
  removed: En1995FastenerPositionRemoval[];
  inserted: En1995FastenerPositionInsertion[];
  moved: En1995FastenerPositionRelocation[];
  modified: En1995FastenerPositionModification[];
}

export interface En1995PanelPatch {
  strengthClass?: StrengthClass | null;
  spanM?: number | null;
  widthM?: number | null;
  loadPa?: number | null;
  fireDurationS?: number | null;
  layers: En1995PanelLayerDelta;
}

export interface En1995PanelRemoval {
  id: string;
  index: number;
}

export interface En1995PanelInsertion {
  index: number;
  row: Panel;
}

export interface En1995PanelRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1995PanelModification {
  id: string;
  patch: En1995PanelPatch;
}

export interface En1995PanelDelta {
  removed: En1995PanelRemoval[];
  inserted: En1995PanelInsertion[];
  moved: En1995PanelRelocation[];
  modified: En1995PanelModification[];
}

export interface En1995PanelLayerPatch {
  thicknessM?: number | null;
  orientation?: LayerOrientation | null;
  grade?: string | null;
}

export interface En1995PanelLayerRemoval {
  id: string;
  index: number;
}

export interface En1995PanelLayerInsertion {
  index: number;
  row: PanelLayer;
}

export interface En1995PanelLayerRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1995PanelLayerModification {
  id: string;
  patch: En1995PanelLayerPatch;
}

export interface En1995PanelLayerDelta {
  removed: En1995PanelLayerRemoval[];
  inserted: En1995PanelLayerInsertion[];
  moved: En1995PanelLayerRelocation[];
  modified: En1995PanelLayerModification[];
}

export interface En1995FloorPatch {
  memberId?: string | null;
  spanM?: number | null;
  widthM?: number | null;
  joistSpacingM?: number | null;
  eiLNM2PerM?: number | null;
  eiBNM2PerM?: number | null;
  massKgPerM2?: number | null;
  dampingRatio?: number | null;
  permanentLoadPa?: number | null;
  imposedLoadPa?: number | null;
  psi2?: number | null;
}

export interface En1995FloorRemoval {
  id: string;
  index: number;
}

export interface En1995FloorInsertion {
  index: number;
  row: Floor;
}

export interface En1995FloorRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1995FloorModification {
  id: string;
  patch: En1995FloorPatch;
}

export interface En1995FloorDelta {
  removed: En1995FloorRemoval[];
  inserted: En1995FloorInsertion[];
  moved: En1995FloorRelocation[];
  modified: En1995FloorModification[];
}

export interface En1995DiaphragmPatch {
  kind?: DiaphragmKind | null;
  method?: DiaphragmMethod | null;
  panelWidthM?: number | null;
  wallHeightM?: number | null;
  fastenerSpacingM?: number | null;
  panels?: number | null;
  fastenerCapacityN?: number | null;
  shearEdN?: number | null;
  holdDownN?: number | null;
  diaphragmForceN?: number | null;
  diaphragmLengthM?: number | null;
  shearResistanceNPerM?: number | null;
}

export interface En1995DiaphragmRemoval {
  id: string;
  index: number;
}

export interface En1995DiaphragmInsertion {
  index: number;
  row: Diaphragm;
}

export interface En1995DiaphragmRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1995DiaphragmModification {
  id: string;
  patch: En1995DiaphragmPatch;
}

export interface En1995DiaphragmDelta {
  removed: En1995DiaphragmRemoval[];
  inserted: En1995DiaphragmInsertion[];
  moved: En1995DiaphragmRelocation[];
  modified: En1995DiaphragmModification[];
}

export interface En1995TrussPatch {
  kind?: TrussKind | null;
  product?: BracingProduct | null;
  spanM?: number | null;
  heightM?: number | null;
  bays?: number | null;
  chordWidthM?: number | null;
  chordDepthM?: number | null;
  webWidthM?: number | null;
  webDepthM?: number | null;
  chordForceN?: number | null;
  restraintLengthM?: number | null;
  bracingCapacityNPerM?: number | null;
}

export interface En1995TrussRemoval {
  id: string;
  index: number;
}

export interface En1995TrussInsertion {
  index: number;
  row: Truss;
}

export interface En1995TrussRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1995TrussModification {
  id: string;
  patch: En1995TrussPatch;
}

export interface En1995TrussDelta {
  removed: En1995TrussRemoval[];
  inserted: En1995TrussInsertion[];
  moved: En1995TrussRelocation[];
  modified: En1995TrussModification[];
}

export interface En1995BracingPatch {
  kind?: BracingKind | null;
  memberId?: string | null;
  stabilisedCount?: number | null;
  spacingM?: number | null;
  lengthM?: number | null;
  braceWidthM?: number | null;
  braceDepthM?: number | null;
  compressionForceN?: number | null;
  resistanceN?: number | null;
  stiffnessNPerM?: number | null;
}

export interface En1995BracingRemoval {
  id: string;
  index: number;
}

export interface En1995BracingInsertion {
  index: number;
  row: Bracing;
}

export interface En1995BracingRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1995BracingModification {
  id: string;
  patch: En1995BracingPatch;
}

export interface En1995BracingDelta {
  removed: En1995BracingRemoval[];
  inserted: En1995BracingInsertion[];
  moved: En1995BracingRelocation[];
  modified: En1995BracingModification[];
}

export interface En1995DeckPatch {
  laminationProduct?: LaminationProduct | null;
  barProduct?: PrestressBarProduct | null;
  spanM?: number | null;
  deckWidthM?: number | null;
  deckDepthM?: number | null;
  laminationWidthM?: number | null;
  barSpacingM?: number | null;
  barDiameterM?: number | null;
  plateSideM?: number | null;
  initialPrestressPa?: number | null;
  lossRelaxation?: number | null;
  lossCreep?: number | null;
  lossMoisture?: number | null;
  barResistancePa?: number | null;
  shearStressPa?: number | null;
  compressionPerpendicularPa?: number | null;
  wheelLoadN?: number | null;
  wheelContactM?: number | null;
}

export interface En1995DeckRemoval {
  id: string;
  index: number;
}

export interface En1995DeckInsertion {
  index: number;
  row: Deck;
}

export interface En1995DeckRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1995DeckModification {
  id: string;
  patch: En1995DeckPatch;
}

export interface En1995DeckDelta {
  removed: En1995DeckRemoval[];
  inserted: En1995DeckInsertion[];
  moved: En1995DeckRelocation[];
  modified: En1995DeckModification[];
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

export const parseEn1995Diff: NormWireReader<En1995Diff> = normWireObject<En1995Diff>({ annex: normWireOptional(normWireNullable(normWireLiteral("En", "De"))), members: normWireRequired(normWireRef(() => parseEn1995MemberDelta)), connections: normWireRequired(normWireRef(() => parseEn1995ConnectionDelta)), placements: normWireRequired(normWireRef(() => parseEn1995PlacementDelta)), tapers: normWireRequired(normWireRef(() => parseEn1995TaperDelta)), cutouts: normWireRequired(normWireRef(() => parseEn1995CutoutDelta)), fasteners: normWireRequired(normWireRef(() => parseEn1995FastenerDelta)), panels: normWireRequired(normWireRef(() => parseEn1995PanelDelta)), floors: normWireRequired(normWireRef(() => parseEn1995FloorDelta)), diaphragms: normWireRequired(normWireRef(() => parseEn1995DiaphragmDelta)), trusses: normWireRequired(normWireRef(() => parseEn1995TrussDelta)), bracings: normWireRequired(normWireRef(() => parseEn1995BracingDelta)), decks: normWireRequired(normWireRef(() => parseEn1995DeckDelta)), ulsEquation: normWireOptional(normWireNullable(parseUlsEquation)), cases: normWireRequired(normWireRef(() => parseCaseRowDelta)), reliabilityClass: normWireOptional(normWireNullable(normWireLiteral("RC1", "RC2", "RC3"))) });
export const parseEn1995MemberActionPatch: NormWireReader<En1995MemberActionPatch> = normWireObject<En1995MemberActionPatch>({ kind: normWireOptional(normWireNullable(parseActionKind)), category: normWireOptional(normWireNullable(parseImposedCategory)), loadDuration: normWireOptional(normWireNullable(parseLoadDurationClass)), qLineNPerM: normWireOptional(normWireNullable(normWireNumber)), fPointN: normWireOptional(normWireNullable(normWireNumber)), mKNm: normWireOptional(normWireNullable(normWireNumber)), vKN: normWireOptional(normWireNullable(normWireNumber)), nKN: normWireOptional(normWireNullable(normWireNumber)), nTKN: normWireOptional(normWireNullable(normWireNumber)), fC90KN: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1995MemberActionRemoval: NormWireReader<En1995MemberActionRemoval> = normWireObject<En1995MemberActionRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995MemberActionInsertion: NormWireReader<En1995MemberActionInsertion> = normWireObject<En1995MemberActionInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCharacteristicAction) });
export const parseEn1995MemberActionRelocation: NormWireReader<En1995MemberActionRelocation> = normWireObject<En1995MemberActionRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995MemberActionModification: NormWireReader<En1995MemberActionModification> = normWireObject<En1995MemberActionModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1995MemberActionPatch)) });
export const parseEn1995MemberActionDelta: NormWireReader<En1995MemberActionDelta> = normWireObject<En1995MemberActionDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1995MemberActionRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1995MemberActionInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1995MemberActionRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1995MemberActionModification))) });
export const parseEn1995ConnectionActionPatch: NormWireReader<En1995ConnectionActionPatch> = normWireObject<En1995ConnectionActionPatch>({ kind: normWireOptional(normWireNullable(parseActionKind)), loadDuration: normWireOptional(normWireNullable(parseLoadDurationClass)), fKN: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1995ConnectionActionRemoval: NormWireReader<En1995ConnectionActionRemoval> = normWireObject<En1995ConnectionActionRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995ConnectionActionInsertion: NormWireReader<En1995ConnectionActionInsertion> = normWireObject<En1995ConnectionActionInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseConnectionAction) });
export const parseEn1995ConnectionActionRelocation: NormWireReader<En1995ConnectionActionRelocation> = normWireObject<En1995ConnectionActionRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995ConnectionActionModification: NormWireReader<En1995ConnectionActionModification> = normWireObject<En1995ConnectionActionModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1995ConnectionActionPatch)) });
export const parseEn1995ConnectionActionDelta: NormWireReader<En1995ConnectionActionDelta> = normWireObject<En1995ConnectionActionDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1995ConnectionActionRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1995ConnectionActionInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1995ConnectionActionRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1995ConnectionActionModification))) });
export const parseEn1995MemberPatch: NormWireReader<En1995MemberPatch> = normWireObject<En1995MemberPatch>({ labelEn: normWireOptional(normWireNullable(normWireString)), labelDe: normWireOptional(normWireNullable(normWireString)), role: normWireOptional(normWireNullable(parseMemberRole)), strengthClass: normWireOptional(normWireNullable(parseStrengthClass)), serviceClass: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), support: normWireOptional(normWireNullable(parseSupportType)), bM: normWireOptional(normWireNullable(normWireNumber)), hM: normWireOptional(normWireNullable(normWireNumber)), spanM: normWireOptional(normWireNullable(normWireNumber)), supportLengthM: normWireOptional(normWireNullable(normWireNumber)), bearingLengthM: normWireOptional(normWireNullable(normWireNumber)), bucklingLengthYM: normWireOptional(normWireNullable(normWireNumber)), bucklingLengthZM: normWireOptional(normWireNullable(normWireNumber)), lateralRestraintSpacingM: normWireOptional(normWireNullable(normWireNumber)), notchDepthM: normWireOptional(normWireNullable(normWireNumber)), notchDistanceM: normWireOptional(normWireNullable(normWireNumber)), massKgPerM: normWireOptional(normWireNullable(normWireNumber)), massKgPerM2: normWireOptional(normWireNullable(normWireNumber)), dampingXi: normWireOptional(normWireNullable(normWireNumber)), fireDurationS: normWireOptional(normWireNullable(normWireNumber)), bridgeNObs: normWireOptional(normWireNullable(normWireNumber)), bridgeTLYears: normWireOptional(normWireNullable(normWireNumber)), bridgeBeta: normWireOptional(normWireNullable(normWireNumber)), bridgeA: normWireOptional(normWireNullable(normWireNumber)), bridgeB: normWireOptional(normWireNullable(normWireNumber)), bridgeCrowdPerM2: normWireOptional(normWireNullable(normWireNumber)), actions: normWireRequired(normWireRef(() => parseEn1995MemberActionDelta)) });
export const parseEn1995MemberRemoval: NormWireReader<En1995MemberRemoval> = normWireObject<En1995MemberRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995MemberInsertion: NormWireReader<En1995MemberInsertion> = normWireObject<En1995MemberInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseTimberMember) });
export const parseEn1995MemberRelocation: NormWireReader<En1995MemberRelocation> = normWireObject<En1995MemberRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995MemberModification: NormWireReader<En1995MemberModification> = normWireObject<En1995MemberModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1995MemberPatch)) });
export const parseEn1995MemberDelta: NormWireReader<En1995MemberDelta> = normWireObject<En1995MemberDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1995MemberRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1995MemberInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1995MemberRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1995MemberModification))) });
export const parseEn1995ConnectionPatch: NormWireReader<En1995ConnectionPatch> = normWireObject<En1995ConnectionPatch>({ labelEn: normWireOptional(normWireNullable(normWireString)), labelDe: normWireOptional(normWireNullable(normWireString)), fastenerType: normWireOptional(normWireNullable(parseFastenerType)), strengthClass: normWireOptional(normWireNullable(parseStrengthClass)), serviceClass: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), diameterM: normWireOptional(normWireNullable(normWireNumber)), number: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), rows: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), spacingM: normWireOptional(normWireNullable(normWireNumber)), edgeDistanceM: normWireOptional(normWireNullable(normWireNumber)), endDistanceM: normWireOptional(normWireNullable(normWireNumber)), t1M: normWireOptional(normWireNullable(normWireNumber)), t2M: normWireOptional(normWireNullable(normWireNumber)), steelPlate: normWireOptional(normWireNullable(normWireBoolean)), steelPlateThicknessM: normWireOptional(normWireNullable(normWireNumber)), shearPlanes: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), fUK: normWireOptional(normWireNullable(normWireNumber)), actions: normWireRequired(normWireRef(() => parseEn1995ConnectionActionDelta)) });
export const parseEn1995ConnectionRemoval: NormWireReader<En1995ConnectionRemoval> = normWireObject<En1995ConnectionRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995ConnectionInsertion: NormWireReader<En1995ConnectionInsertion> = normWireObject<En1995ConnectionInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseTimberConnection) });
export const parseEn1995ConnectionRelocation: NormWireReader<En1995ConnectionRelocation> = normWireObject<En1995ConnectionRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995ConnectionModification: NormWireReader<En1995ConnectionModification> = normWireObject<En1995ConnectionModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1995ConnectionPatch)) });
export const parseEn1995ConnectionDelta: NormWireReader<En1995ConnectionDelta> = normWireObject<En1995ConnectionDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1995ConnectionRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1995ConnectionInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1995ConnectionRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1995ConnectionModification))) });
export const parseEn1995PlacementPatch: NormWireReader<En1995PlacementPatch> = normWireObject<En1995PlacementPatch>({ x: normWireOptional(normWireNullable(normWireNumber)), y: normWireOptional(normWireNullable(normWireNumber)), z: normWireOptional(normWireNullable(normWireNumber)), yaw: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1995PlacementRemoval: NormWireReader<En1995PlacementRemoval> = normWireObject<En1995PlacementRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995PlacementInsertion: NormWireReader<En1995PlacementInsertion> = normWireObject<En1995PlacementInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parsePlacement) });
export const parseEn1995PlacementRelocation: NormWireReader<En1995PlacementRelocation> = normWireObject<En1995PlacementRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995PlacementModification: NormWireReader<En1995PlacementModification> = normWireObject<En1995PlacementModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1995PlacementPatch)) });
export const parseEn1995PlacementDelta: NormWireReader<En1995PlacementDelta> = normWireObject<En1995PlacementDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1995PlacementRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1995PlacementInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1995PlacementRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1995PlacementModification))) });
export const parseEn1995TaperPatch: NormWireReader<En1995TaperPatch> = normWireObject<En1995TaperPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), kind: normWireOptional(normWireNullable(parseTaperKind)), taperAngleRad: normWireOptional(normWireNullable(normWireNumber)), apexDepthM: normWireOptional(normWireNullable(normWireNumber)), innerRadiusM: normWireOptional(normWireNullable(normWireNumber)), lamellaThicknessM: normWireOptional(normWireNullable(normWireNumber)), apexAngleRad: normWireOptional(normWireNullable(normWireNumber)), tensionSide: normWireOptional(normWireNullable(parseTensionSide)), rods: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), rodResistanceN: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1995TaperRemoval: NormWireReader<En1995TaperRemoval> = normWireObject<En1995TaperRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995TaperInsertion: NormWireReader<En1995TaperInsertion> = normWireObject<En1995TaperInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseTaper) });
export const parseEn1995TaperRelocation: NormWireReader<En1995TaperRelocation> = normWireObject<En1995TaperRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995TaperModification: NormWireReader<En1995TaperModification> = normWireObject<En1995TaperModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1995TaperPatch)) });
export const parseEn1995TaperDelta: NormWireReader<En1995TaperDelta> = normWireObject<En1995TaperDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1995TaperRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1995TaperInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1995TaperRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1995TaperModification))) });
export const parseEn1995CutoutPatch: NormWireReader<En1995CutoutPatch> = normWireObject<En1995CutoutPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), kind: normWireOptional(normWireNullable(parseCutoutKind)), face: normWireOptional(normWireNullable(parseCutoutFace)), distanceM: normWireOptional(normWireNullable(normWireNumber)), lengthM: normWireOptional(normWireNullable(normWireNumber)), depthM: normWireOptional(normWireNullable(normWireNumber)), centreHeightM: normWireOptional(normWireNullable(normWireNumber)), slope: normWireOptional(normWireNullable(normWireNumber)), rods: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), rodResistanceN: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1995CutoutRemoval: NormWireReader<En1995CutoutRemoval> = normWireObject<En1995CutoutRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995CutoutInsertion: NormWireReader<En1995CutoutInsertion> = normWireObject<En1995CutoutInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCutout) });
export const parseEn1995CutoutRelocation: NormWireReader<En1995CutoutRelocation> = normWireObject<En1995CutoutRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995CutoutModification: NormWireReader<En1995CutoutModification> = normWireObject<En1995CutoutModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1995CutoutPatch)) });
export const parseEn1995CutoutDelta: NormWireReader<En1995CutoutDelta> = normWireObject<En1995CutoutDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1995CutoutRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1995CutoutInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1995CutoutRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1995CutoutModification))) });
export const parseEn1995FastenerPatch: NormWireReader<En1995FastenerPatch> = normWireObject<En1995FastenerPatch>({ connectionId: normWireOptional(normWireNullable(normWireString)), memberId: normWireOptional(normWireNullable(normWireString)), columns: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), lines: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), spacingParallelM: normWireOptional(normWireNullable(normWireNumber)), spacingPerpendicularM: normWireOptional(normWireNullable(normWireNumber)), endDistanceM: normWireOptional(normWireNullable(normWireNumber)), edgeDistanceM: normWireOptional(normWireNullable(normWireNumber)), offsetXM: normWireOptional(normWireNullable(normWireNumber)), offsetYM: normWireOptional(normWireNullable(normWireNumber)), loadAngleRad: normWireOptional(normWireNullable(normWireNumber)), positions: normWireRequired(normWireRef(() => parseEn1995FastenerPositionDelta)) });
export const parseEn1995FastenerRemoval: NormWireReader<En1995FastenerRemoval> = normWireObject<En1995FastenerRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995FastenerInsertion: NormWireReader<En1995FastenerInsertion> = normWireObject<En1995FastenerInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseFastener) });
export const parseEn1995FastenerRelocation: NormWireReader<En1995FastenerRelocation> = normWireObject<En1995FastenerRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995FastenerModification: NormWireReader<En1995FastenerModification> = normWireObject<En1995FastenerModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1995FastenerPatch)) });
export const parseEn1995FastenerDelta: NormWireReader<En1995FastenerDelta> = normWireObject<En1995FastenerDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1995FastenerRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1995FastenerInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1995FastenerRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1995FastenerModification))) });
export const parseEn1995FastenerPositionPatch: NormWireReader<En1995FastenerPositionPatch> = normWireObject<En1995FastenerPositionPatch>({ xM: normWireOptional(normWireNullable(normWireNumber)), yM: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1995FastenerPositionRemoval: NormWireReader<En1995FastenerPositionRemoval> = normWireObject<En1995FastenerPositionRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995FastenerPositionInsertion: NormWireReader<En1995FastenerPositionInsertion> = normWireObject<En1995FastenerPositionInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseFastenerPosition) });
export const parseEn1995FastenerPositionRelocation: NormWireReader<En1995FastenerPositionRelocation> = normWireObject<En1995FastenerPositionRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995FastenerPositionModification: NormWireReader<En1995FastenerPositionModification> = normWireObject<En1995FastenerPositionModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1995FastenerPositionPatch)) });
export const parseEn1995FastenerPositionDelta: NormWireReader<En1995FastenerPositionDelta> = normWireObject<En1995FastenerPositionDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1995FastenerPositionRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1995FastenerPositionInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1995FastenerPositionRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1995FastenerPositionModification))) });
export const parseEn1995PanelPatch: NormWireReader<En1995PanelPatch> = normWireObject<En1995PanelPatch>({ strengthClass: normWireOptional(normWireNullable(parseStrengthClass)), spanM: normWireOptional(normWireNullable(normWireNumber)), widthM: normWireOptional(normWireNullable(normWireNumber)), loadPa: normWireOptional(normWireNullable(normWireNumber)), fireDurationS: normWireOptional(normWireNullable(normWireNumber)), layers: normWireRequired(normWireRef(() => parseEn1995PanelLayerDelta)) });
export const parseEn1995PanelRemoval: NormWireReader<En1995PanelRemoval> = normWireObject<En1995PanelRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995PanelInsertion: NormWireReader<En1995PanelInsertion> = normWireObject<En1995PanelInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parsePanel) });
export const parseEn1995PanelRelocation: NormWireReader<En1995PanelRelocation> = normWireObject<En1995PanelRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995PanelModification: NormWireReader<En1995PanelModification> = normWireObject<En1995PanelModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1995PanelPatch)) });
export const parseEn1995PanelDelta: NormWireReader<En1995PanelDelta> = normWireObject<En1995PanelDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1995PanelRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1995PanelInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1995PanelRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1995PanelModification))) });
export const parseEn1995PanelLayerPatch: NormWireReader<En1995PanelLayerPatch> = normWireObject<En1995PanelLayerPatch>({ thicknessM: normWireOptional(normWireNullable(normWireNumber)), orientation: normWireOptional(normWireNullable(parseLayerOrientation)), grade: normWireOptional(normWireNullable(normWireString)) });
export const parseEn1995PanelLayerRemoval: NormWireReader<En1995PanelLayerRemoval> = normWireObject<En1995PanelLayerRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995PanelLayerInsertion: NormWireReader<En1995PanelLayerInsertion> = normWireObject<En1995PanelLayerInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parsePanelLayer) });
export const parseEn1995PanelLayerRelocation: NormWireReader<En1995PanelLayerRelocation> = normWireObject<En1995PanelLayerRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995PanelLayerModification: NormWireReader<En1995PanelLayerModification> = normWireObject<En1995PanelLayerModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1995PanelLayerPatch)) });
export const parseEn1995PanelLayerDelta: NormWireReader<En1995PanelLayerDelta> = normWireObject<En1995PanelLayerDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1995PanelLayerRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1995PanelLayerInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1995PanelLayerRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1995PanelLayerModification))) });
export const parseEn1995FloorPatch: NormWireReader<En1995FloorPatch> = normWireObject<En1995FloorPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), spanM: normWireOptional(normWireNullable(normWireNumber)), widthM: normWireOptional(normWireNullable(normWireNumber)), joistSpacingM: normWireOptional(normWireNullable(normWireNumber)), eiLNM2PerM: normWireOptional(normWireNullable(normWireNumber)), eiBNM2PerM: normWireOptional(normWireNullable(normWireNumber)), massKgPerM2: normWireOptional(normWireNullable(normWireNumber)), dampingRatio: normWireOptional(normWireNullable(normWireNumber)), permanentLoadPa: normWireOptional(normWireNullable(normWireNumber)), imposedLoadPa: normWireOptional(normWireNullable(normWireNumber)), psi2: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1995FloorRemoval: NormWireReader<En1995FloorRemoval> = normWireObject<En1995FloorRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995FloorInsertion: NormWireReader<En1995FloorInsertion> = normWireObject<En1995FloorInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseFloor) });
export const parseEn1995FloorRelocation: NormWireReader<En1995FloorRelocation> = normWireObject<En1995FloorRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995FloorModification: NormWireReader<En1995FloorModification> = normWireObject<En1995FloorModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1995FloorPatch)) });
export const parseEn1995FloorDelta: NormWireReader<En1995FloorDelta> = normWireObject<En1995FloorDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1995FloorRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1995FloorInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1995FloorRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1995FloorModification))) });
export const parseEn1995DiaphragmPatch: NormWireReader<En1995DiaphragmPatch> = normWireObject<En1995DiaphragmPatch>({ kind: normWireOptional(normWireNullable(parseDiaphragmKind)), method: normWireOptional(normWireNullable(parseDiaphragmMethod)), panelWidthM: normWireOptional(normWireNullable(normWireNumber)), wallHeightM: normWireOptional(normWireNullable(normWireNumber)), fastenerSpacingM: normWireOptional(normWireNullable(normWireNumber)), panels: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), fastenerCapacityN: normWireOptional(normWireNullable(normWireNumber)), shearEdN: normWireOptional(normWireNullable(normWireNumber)), holdDownN: normWireOptional(normWireNullable(normWireNumber)), diaphragmForceN: normWireOptional(normWireNullable(normWireNumber)), diaphragmLengthM: normWireOptional(normWireNullable(normWireNumber)), shearResistanceNPerM: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1995DiaphragmRemoval: NormWireReader<En1995DiaphragmRemoval> = normWireObject<En1995DiaphragmRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995DiaphragmInsertion: NormWireReader<En1995DiaphragmInsertion> = normWireObject<En1995DiaphragmInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseDiaphragm) });
export const parseEn1995DiaphragmRelocation: NormWireReader<En1995DiaphragmRelocation> = normWireObject<En1995DiaphragmRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995DiaphragmModification: NormWireReader<En1995DiaphragmModification> = normWireObject<En1995DiaphragmModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1995DiaphragmPatch)) });
export const parseEn1995DiaphragmDelta: NormWireReader<En1995DiaphragmDelta> = normWireObject<En1995DiaphragmDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1995DiaphragmRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1995DiaphragmInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1995DiaphragmRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1995DiaphragmModification))) });
export const parseEn1995TrussPatch: NormWireReader<En1995TrussPatch> = normWireObject<En1995TrussPatch>({ kind: normWireOptional(normWireNullable(parseTrussKind)), product: normWireOptional(normWireNullable(parseBracingProduct)), spanM: normWireOptional(normWireNullable(normWireNumber)), heightM: normWireOptional(normWireNullable(normWireNumber)), bays: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), chordWidthM: normWireOptional(normWireNullable(normWireNumber)), chordDepthM: normWireOptional(normWireNullable(normWireNumber)), webWidthM: normWireOptional(normWireNullable(normWireNumber)), webDepthM: normWireOptional(normWireNullable(normWireNumber)), chordForceN: normWireOptional(normWireNullable(normWireNumber)), restraintLengthM: normWireOptional(normWireNullable(normWireNumber)), bracingCapacityNPerM: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1995TrussRemoval: NormWireReader<En1995TrussRemoval> = normWireObject<En1995TrussRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995TrussInsertion: NormWireReader<En1995TrussInsertion> = normWireObject<En1995TrussInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseTruss) });
export const parseEn1995TrussRelocation: NormWireReader<En1995TrussRelocation> = normWireObject<En1995TrussRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995TrussModification: NormWireReader<En1995TrussModification> = normWireObject<En1995TrussModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1995TrussPatch)) });
export const parseEn1995TrussDelta: NormWireReader<En1995TrussDelta> = normWireObject<En1995TrussDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1995TrussRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1995TrussInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1995TrussRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1995TrussModification))) });
export const parseEn1995BracingPatch: NormWireReader<En1995BracingPatch> = normWireObject<En1995BracingPatch>({ kind: normWireOptional(normWireNullable(parseBracingKind)), memberId: normWireOptional(normWireNullable(normWireString)), stabilisedCount: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), spacingM: normWireOptional(normWireNullable(normWireNumber)), lengthM: normWireOptional(normWireNullable(normWireNumber)), braceWidthM: normWireOptional(normWireNullable(normWireNumber)), braceDepthM: normWireOptional(normWireNullable(normWireNumber)), compressionForceN: normWireOptional(normWireNullable(normWireNumber)), resistanceN: normWireOptional(normWireNullable(normWireNumber)), stiffnessNPerM: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1995BracingRemoval: NormWireReader<En1995BracingRemoval> = normWireObject<En1995BracingRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995BracingInsertion: NormWireReader<En1995BracingInsertion> = normWireObject<En1995BracingInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseBracing) });
export const parseEn1995BracingRelocation: NormWireReader<En1995BracingRelocation> = normWireObject<En1995BracingRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995BracingModification: NormWireReader<En1995BracingModification> = normWireObject<En1995BracingModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1995BracingPatch)) });
export const parseEn1995BracingDelta: NormWireReader<En1995BracingDelta> = normWireObject<En1995BracingDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1995BracingRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1995BracingInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1995BracingRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1995BracingModification))) });
export const parseEn1995DeckPatch: NormWireReader<En1995DeckPatch> = normWireObject<En1995DeckPatch>({ laminationProduct: normWireOptional(normWireNullable(parseLaminationProduct)), barProduct: normWireOptional(normWireNullable(parsePrestressBarProduct)), spanM: normWireOptional(normWireNullable(normWireNumber)), deckWidthM: normWireOptional(normWireNullable(normWireNumber)), deckDepthM: normWireOptional(normWireNullable(normWireNumber)), laminationWidthM: normWireOptional(normWireNullable(normWireNumber)), barSpacingM: normWireOptional(normWireNullable(normWireNumber)), barDiameterM: normWireOptional(normWireNullable(normWireNumber)), plateSideM: normWireOptional(normWireNullable(normWireNumber)), initialPrestressPa: normWireOptional(normWireNullable(normWireNumber)), lossRelaxation: normWireOptional(normWireNullable(normWireNumber)), lossCreep: normWireOptional(normWireNullable(normWireNumber)), lossMoisture: normWireOptional(normWireNullable(normWireNumber)), barResistancePa: normWireOptional(normWireNullable(normWireNumber)), shearStressPa: normWireOptional(normWireNullable(normWireNumber)), compressionPerpendicularPa: normWireOptional(normWireNullable(normWireNumber)), wheelLoadN: normWireOptional(normWireNullable(normWireNumber)), wheelContactM: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1995DeckRemoval: NormWireReader<En1995DeckRemoval> = normWireObject<En1995DeckRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995DeckInsertion: NormWireReader<En1995DeckInsertion> = normWireObject<En1995DeckInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseDeck) });
export const parseEn1995DeckRelocation: NormWireReader<En1995DeckRelocation> = normWireObject<En1995DeckRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995DeckModification: NormWireReader<En1995DeckModification> = normWireObject<En1995DeckModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1995DeckPatch)) });
export const parseEn1995DeckDelta: NormWireReader<En1995DeckDelta> = normWireObject<En1995DeckDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1995DeckRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1995DeckInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1995DeckRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1995DeckModification))) });
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
