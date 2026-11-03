/** 📸️ `En1995Snapshot` wire twin: the persisted snapshot and every record it holds, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface En1995Snapshot {
  /** @state artifact */
  annex: "En" | "De";
  /** @state artifact */
  members: TimberMember[];
  /** @state artifact */
  connections: TimberConnection[];
  /** @state artifact */
  ulsEquation: UlsEquation;
  /** @state artifact */
  cases: CaseRow[];
  /** @state artifact */
  placements: Placement[];
  /** @state artifact */
  tapers: Taper[];
  /** @state artifact */
  cutouts: Cutout[];
  /** @state artifact */
  fasteners: Fastener[];
  /** @state artifact */
  panels: Panel[];
  /** @state artifact */
  floors: Floor[];
  /** @state artifact */
  diaphragms: Diaphragm[];
  /** @state artifact */
  trusses: Truss[];
  /** @state artifact */
  bracings: Bracing[];
  /** @state artifact */
  decks: Deck[];
  /** @state artifact */
  reliabilityClass: "RC1" | "RC2" | "RC3";
}

export type UlsEquation = "eq610" | "eq610ab";

export type FastenerType = "nail" | "screw" | "bolt" | "dowel";

export interface TimberConnection {
  id: string;
  labelEn: string;
  labelDe: string;
  fastenerType: FastenerType;
  strengthClass: StrengthClass;
  serviceClass: number;
  diameterM: number;
  number: number;
  rows: number;
  spacingM: number;
  edgeDistanceM: number;
  endDistanceM: number;
  t1M: number;
  t2M: number;
  steelPlate: boolean;
  steelPlateThicknessM: number;
  shearPlanes: number;
  fUK: number;
  actions: ConnectionAction[];
}

export interface Placement {
  id: string;
  x: number;
  y: number;
  z: number;
  yaw: number;
}

export interface Taper {
  id: string;
  memberId: string;
  kind: TaperKind;
  taperAngleRad: number;
  apexDepthM: number;
  innerRadiusM: number;
  lamellaThicknessM: number;
  apexAngleRad: number;
  tensionSide: TensionSide;
  rods: number;
  rodResistanceN: number;
}

export interface Cutout {
  id: string;
  memberId: string;
  kind: CutoutKind;
  face: CutoutFace;
  distanceM: number;
  lengthM: number;
  depthM: number;
  centreHeightM: number;
  slope: number;
  rods: number;
  rodResistanceN: number;
}

export interface FastenerPosition {
  id: string;
  xM: number;
  yM: number;
}

export interface Fastener {
  id: string;
  connectionId: string;
  memberId: string;
  columns: number;
  lines: number;
  spacingParallelM: number;
  spacingPerpendicularM: number;
  endDistanceM: number;
  edgeDistanceM: number;
  offsetXM: number;
  offsetYM: number;
  loadAngleRad: number;
  positions: FastenerPosition[];
}

export interface PanelLayer {
  id: string;
  thicknessM: number;
  orientation: LayerOrientation;
  grade: string;
}

export interface Panel {
  id: string;
  strengthClass: StrengthClass;
  layers: PanelLayer[];
  spanM: number;
  widthM: number;
  loadPa: number;
  fireDurationS: number;
}

export interface Floor {
  id: string;
  memberId: string;
  spanM: number;
  widthM: number;
  joistSpacingM: number;
  eiLNM2PerM: number;
  eiBNM2PerM: number;
  massKgPerM2: number;
  dampingRatio: number;
  permanentLoadPa: number;
  imposedLoadPa: number;
  psi2: number;
}

export interface Diaphragm {
  id: string;
  kind: DiaphragmKind;
  method: DiaphragmMethod;
  panelWidthM: number;
  wallHeightM: number;
  fastenerSpacingM: number;
  panels: number;
  fastenerCapacityN: number;
  shearEdN: number;
  holdDownN: number;
  diaphragmForceN: number;
  diaphragmLengthM: number;
  shearResistanceNPerM: number;
}

export interface Truss {
  id: string;
  kind: TrussKind;
  product: BracingProduct;
  spanM: number;
  heightM: number;
  bays: number;
  chordWidthM: number;
  chordDepthM: number;
  webWidthM: number;
  webDepthM: number;
  chordForceN: number;
  restraintLengthM: number;
  bracingCapacityNPerM: number;
}

export interface Bracing {
  id: string;
  kind: BracingKind;
  memberId: string;
  stabilisedCount: number;
  spacingM: number;
  lengthM: number;
  braceWidthM: number;
  braceDepthM: number;
  compressionForceN: number;
  resistanceN: number;
  stiffnessNPerM: number;
}

export interface Deck {
  id: string;
  laminationProduct: LaminationProduct;
  barProduct: PrestressBarProduct;
  spanM: number;
  deckWidthM: number;
  deckDepthM: number;
  laminationWidthM: number;
  barSpacingM: number;
  barDiameterM: number;
  plateSideM: number;
  initialPrestressPa: number;
  lossRelaxation: number;
  lossCreep: number;
  lossMoisture: number;
  barResistancePa: number;
  shearStressPa: number;
  compressionPerpendicularPa: number;
  wheelLoadN: number;
  wheelContactM: number;
}

export interface ConnectionAction {
  id: string;
  kind: ActionKind;
  loadDuration: LoadDurationClass;
  fKN: number;
}

export type StrengthClass = "c14" | "c16" | "c18" | "c20" | "c22" | "c24" | "c27" | "c30" | "c35" | "c40" | "c45" | "c50" | "d18" | "d24" | "d30" | "d35" | "d40" | "d50" | "d60" | "d70" | "gl20h" | "gl22h" | "gl24h" | "gl26h" | "gl28h" | "gl30h" | "gl32h" | "gl20c" | "gl22c" | "gl24c" | "gl26c" | "gl28c" | "gl30c" | "gl32c" | "lvl32" | "clt100";

export interface TimberMember {
  id: string;
  labelEn: string;
  labelDe: string;
  role: MemberRole;
  strengthClass: StrengthClass;
  serviceClass: number;
  support: SupportType;
  bM: number;
  hM: number;
  spanM: number;
  supportLengthM: number;
  bearingLengthM: number;
  bucklingLengthYM: number;
  bucklingLengthZM: number;
  lateralRestraintSpacingM: number;
  notchDepthM: number;
  notchDistanceM: number;
  massKgPerM: number;
  massKgPerM2: number;
  dampingXi: number;
  fireDurationS: number;
  bridgeNObs: number;
  bridgeTLYears: number;
  bridgeBeta: number;
  bridgeA: number;
  bridgeB: number;
  bridgeCrowdPerM2: number;
  actions: CharacteristicAction[];
}

export type ActionKind = "permanent" | "imposed" | "snow" | "wind" | "accidental";

export type ImposedCategory = "notImposed" | "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "snowHigh";

export type LoadDurationClass = "permanent" | "long" | "medium" | "short" | "instantaneous";

export interface CharacteristicAction {
  id: string;
  kind: ActionKind;
  category: ImposedCategory;
  loadDuration: LoadDurationClass;
  qLineNPerM: number;
  fPointN: number;
  mKNm: number;
  vKN: number;
  nKN: number;
  nTKN: number;
  fC90KN: number;
}

export type AnnexChoice = "En" | "De";

export type MemberRole = "beam" | "column" | "floor" | "bridge";

export type SupportType = "simplySupported" | "cantilever" | "continuousTwoSpan";

export type LaminationProduct = "solid" | "glulam";

export type PrestressBarProduct = "threaded-bar" | "smooth-bar";

export type TaperKind = "single-tapered" | "double-tapered" | "curved" | "pitched-cambered";

export type TensionSide = "tension" | "compression";

export type CutoutKind = "notch" | "hole";

export type CutoutFace = "bottom" | "top";

export type DiaphragmKind = "wall" | "floor" | "roof";

export type DiaphragmMethod = "method-a" | "method-b";

export type TrussKind = "warren" | "pratt" | "howe" | "king-post";

export type BracingProduct = "solid" | "engineered";

export type BracingKind = "plan-bracing" | "bridging" | "elastic-restraint";

export type LayerOrientation = "longitudinal" | "transverse";

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

export const parseEn1995Snapshot: NormWireReader<En1995Snapshot> = normWireObject<En1995Snapshot>({ annex: normWireRequired(normWireLiteral("En", "De")), members: normWireRequired(normWireArray(normWireRef(() => parseTimberMember))), connections: normWireRequired(normWireArray(normWireRef(() => parseTimberConnection))), ulsEquation: normWireRequired(normWireRef(() => parseUlsEquation)), cases: normWireRequired(normWireArray(normWireRef(() => parseCaseRow))), placements: normWireRequired(normWireArray(normWireRef(() => parsePlacement))), tapers: normWireRequired(normWireArray(normWireRef(() => parseTaper))), cutouts: normWireRequired(normWireArray(normWireRef(() => parseCutout))), fasteners: normWireRequired(normWireArray(normWireRef(() => parseFastener))), panels: normWireRequired(normWireArray(normWireRef(() => parsePanel))), floors: normWireRequired(normWireArray(normWireRef(() => parseFloor))), diaphragms: normWireRequired(normWireArray(normWireRef(() => parseDiaphragm))), trusses: normWireRequired(normWireArray(normWireRef(() => parseTruss))), bracings: normWireRequired(normWireArray(normWireRef(() => parseBracing))), decks: normWireRequired(normWireArray(normWireRef(() => parseDeck))), reliabilityClass: normWireRequired(normWireLiteral("RC1", "RC2", "RC3")) });
export const parseUlsEquation: NormWireReader<UlsEquation> = normWireLiteral("eq610", "eq610ab");
export const parseFastenerType: NormWireReader<FastenerType> = normWireLiteral("nail", "screw", "bolt", "dowel");
export const parseTimberConnection: NormWireReader<TimberConnection> = normWireObject<TimberConnection>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), fastenerType: normWireRequired(normWireRef(() => parseFastenerType)), strengthClass: normWireRequired(normWireRef(() => parseStrengthClass)), serviceClass: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), diameterM: normWireRequired(normWireNumber), number: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), rows: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), spacingM: normWireRequired(normWireNumber), edgeDistanceM: normWireRequired(normWireNumber), endDistanceM: normWireRequired(normWireNumber), t1M: normWireRequired(normWireNumber), t2M: normWireRequired(normWireNumber), steelPlate: normWireRequired(normWireBoolean), steelPlateThicknessM: normWireRequired(normWireNumber), shearPlanes: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), fUK: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseConnectionAction))) });
export const parsePlacement: NormWireReader<Placement> = normWireObject<Placement>({ id: normWireRequired(normWireString), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber), z: normWireRequired(normWireNumber), yaw: normWireRequired(normWireNumber) });
export const parseTaper: NormWireReader<Taper> = normWireObject<Taper>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseTaperKind)), taperAngleRad: normWireRequired(normWireNumber), apexDepthM: normWireRequired(normWireNumber), innerRadiusM: normWireRequired(normWireNumber), lamellaThicknessM: normWireRequired(normWireNumber), apexAngleRad: normWireRequired(normWireNumber), tensionSide: normWireRequired(normWireRef(() => parseTensionSide)), rods: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), rodResistanceN: normWireRequired(normWireNumber) });
export const parseCutout: NormWireReader<Cutout> = normWireObject<Cutout>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseCutoutKind)), face: normWireRequired(normWireRef(() => parseCutoutFace)), distanceM: normWireRequired(normWireNumber), lengthM: normWireRequired(normWireNumber), depthM: normWireRequired(normWireNumber), centreHeightM: normWireRequired(normWireNumber), slope: normWireRequired(normWireNumber), rods: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), rodResistanceN: normWireRequired(normWireNumber) });
export const parseFastenerPosition: NormWireReader<FastenerPosition> = normWireObject<FastenerPosition>({ id: normWireRequired(normWireString), xM: normWireRequired(normWireNumber), yM: normWireRequired(normWireNumber) });
export const parseFastener: NormWireReader<Fastener> = normWireObject<Fastener>({ id: normWireRequired(normWireString), connectionId: normWireRequired(normWireString), memberId: normWireRequired(normWireString), columns: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), lines: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), spacingParallelM: normWireRequired(normWireNumber), spacingPerpendicularM: normWireRequired(normWireNumber), endDistanceM: normWireRequired(normWireNumber), edgeDistanceM: normWireRequired(normWireNumber), offsetXM: normWireRequired(normWireNumber), offsetYM: normWireRequired(normWireNumber), loadAngleRad: normWireRequired(normWireNumber), positions: normWireRequired(normWireArray(normWireRef(() => parseFastenerPosition))) });
export const parsePanelLayer: NormWireReader<PanelLayer> = normWireObject<PanelLayer>({ id: normWireRequired(normWireString), thicknessM: normWireRequired(normWireNumber), orientation: normWireRequired(normWireRef(() => parseLayerOrientation)), grade: normWireRequired(normWireString) });
export const parsePanel: NormWireReader<Panel> = normWireObject<Panel>({ id: normWireRequired(normWireString), strengthClass: normWireRequired(normWireRef(() => parseStrengthClass)), layers: normWireRequired(normWireArray(normWireRef(() => parsePanelLayer))), spanM: normWireRequired(normWireNumber), widthM: normWireRequired(normWireNumber), loadPa: normWireRequired(normWireNumber), fireDurationS: normWireRequired(normWireNumber) });
export const parseFloor: NormWireReader<Floor> = normWireObject<Floor>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), spanM: normWireRequired(normWireNumber), widthM: normWireRequired(normWireNumber), joistSpacingM: normWireRequired(normWireNumber), eiLNM2PerM: normWireRequired(normWireNumber), eiBNM2PerM: normWireRequired(normWireNumber), massKgPerM2: normWireRequired(normWireNumber), dampingRatio: normWireRequired(normWireNumber), permanentLoadPa: normWireRequired(normWireNumber), imposedLoadPa: normWireRequired(normWireNumber), psi2: normWireRequired(normWireNumber) });
export const parseDiaphragm: NormWireReader<Diaphragm> = normWireObject<Diaphragm>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseDiaphragmKind)), method: normWireRequired(normWireRef(() => parseDiaphragmMethod)), panelWidthM: normWireRequired(normWireNumber), wallHeightM: normWireRequired(normWireNumber), fastenerSpacingM: normWireRequired(normWireNumber), panels: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), fastenerCapacityN: normWireRequired(normWireNumber), shearEdN: normWireRequired(normWireNumber), holdDownN: normWireRequired(normWireNumber), diaphragmForceN: normWireRequired(normWireNumber), diaphragmLengthM: normWireRequired(normWireNumber), shearResistanceNPerM: normWireRequired(normWireNumber) });
export const parseTruss: NormWireReader<Truss> = normWireObject<Truss>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseTrussKind)), product: normWireRequired(normWireRef(() => parseBracingProduct)), spanM: normWireRequired(normWireNumber), heightM: normWireRequired(normWireNumber), bays: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), chordWidthM: normWireRequired(normWireNumber), chordDepthM: normWireRequired(normWireNumber), webWidthM: normWireRequired(normWireNumber), webDepthM: normWireRequired(normWireNumber), chordForceN: normWireRequired(normWireNumber), restraintLengthM: normWireRequired(normWireNumber), bracingCapacityNPerM: normWireRequired(normWireNumber) });
export const parseBracing: NormWireReader<Bracing> = normWireObject<Bracing>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseBracingKind)), memberId: normWireRequired(normWireString), stabilisedCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), spacingM: normWireRequired(normWireNumber), lengthM: normWireRequired(normWireNumber), braceWidthM: normWireRequired(normWireNumber), braceDepthM: normWireRequired(normWireNumber), compressionForceN: normWireRequired(normWireNumber), resistanceN: normWireRequired(normWireNumber), stiffnessNPerM: normWireRequired(normWireNumber) });
export const parseDeck: NormWireReader<Deck> = normWireObject<Deck>({ id: normWireRequired(normWireString), laminationProduct: normWireRequired(normWireRef(() => parseLaminationProduct)), barProduct: normWireRequired(normWireRef(() => parsePrestressBarProduct)), spanM: normWireRequired(normWireNumber), deckWidthM: normWireRequired(normWireNumber), deckDepthM: normWireRequired(normWireNumber), laminationWidthM: normWireRequired(normWireNumber), barSpacingM: normWireRequired(normWireNumber), barDiameterM: normWireRequired(normWireNumber), plateSideM: normWireRequired(normWireNumber), initialPrestressPa: normWireRequired(normWireNumber), lossRelaxation: normWireRequired(normWireNumber), lossCreep: normWireRequired(normWireNumber), lossMoisture: normWireRequired(normWireNumber), barResistancePa: normWireRequired(normWireNumber), shearStressPa: normWireRequired(normWireNumber), compressionPerpendicularPa: normWireRequired(normWireNumber), wheelLoadN: normWireRequired(normWireNumber), wheelContactM: normWireRequired(normWireNumber) });
export const parseConnectionAction: NormWireReader<ConnectionAction> = normWireObject<ConnectionAction>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseActionKind)), loadDuration: normWireRequired(normWireRef(() => parseLoadDurationClass)), fKN: normWireRequired(normWireNumber) });
export const parseStrengthClass: NormWireReader<StrengthClass> = normWireLiteral("c14", "c16", "c18", "c20", "c22", "c24", "c27", "c30", "c35", "c40", "c45", "c50", "d18", "d24", "d30", "d35", "d40", "d50", "d60", "d70", "gl20h", "gl22h", "gl24h", "gl26h", "gl28h", "gl30h", "gl32h", "gl20c", "gl22c", "gl24c", "gl26c", "gl28c", "gl30c", "gl32c", "lvl32", "clt100");
export const parseTimberMember: NormWireReader<TimberMember> = normWireObject<TimberMember>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), role: normWireRequired(normWireRef(() => parseMemberRole)), strengthClass: normWireRequired(normWireRef(() => parseStrengthClass)), serviceClass: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), support: normWireRequired(normWireRef(() => parseSupportType)), bM: normWireRequired(normWireNumber), hM: normWireRequired(normWireNumber), spanM: normWireRequired(normWireNumber), supportLengthM: normWireRequired(normWireNumber), bearingLengthM: normWireRequired(normWireNumber), bucklingLengthYM: normWireRequired(normWireNumber), bucklingLengthZM: normWireRequired(normWireNumber), lateralRestraintSpacingM: normWireRequired(normWireNumber), notchDepthM: normWireRequired(normWireNumber), notchDistanceM: normWireRequired(normWireNumber), massKgPerM: normWireRequired(normWireNumber), massKgPerM2: normWireRequired(normWireNumber), dampingXi: normWireRequired(normWireNumber), fireDurationS: normWireRequired(normWireNumber), bridgeNObs: normWireRequired(normWireNumber), bridgeTLYears: normWireRequired(normWireNumber), bridgeBeta: normWireRequired(normWireNumber), bridgeA: normWireRequired(normWireNumber), bridgeB: normWireRequired(normWireNumber), bridgeCrowdPerM2: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseCharacteristicAction))) });
export const parseActionKind: NormWireReader<ActionKind> = normWireLiteral("permanent", "imposed", "snow", "wind", "accidental");
export const parseImposedCategory: NormWireReader<ImposedCategory> = normWireLiteral("notImposed", "a", "b", "c", "d", "e", "f", "g", "h", "snowHigh");
export const parseLoadDurationClass: NormWireReader<LoadDurationClass> = normWireLiteral("permanent", "long", "medium", "short", "instantaneous");
export const parseCharacteristicAction: NormWireReader<CharacteristicAction> = normWireObject<CharacteristicAction>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseActionKind)), category: normWireRequired(normWireRef(() => parseImposedCategory)), loadDuration: normWireRequired(normWireRef(() => parseLoadDurationClass)), qLineNPerM: normWireRequired(normWireNumber), fPointN: normWireRequired(normWireNumber), mKNm: normWireRequired(normWireNumber), vKN: normWireRequired(normWireNumber), nKN: normWireRequired(normWireNumber), nTKN: normWireRequired(normWireNumber), fC90KN: normWireRequired(normWireNumber) });
export const parseAnnexChoice: NormWireReader<AnnexChoice> = normWireLiteral("En", "De");
export const parseMemberRole: NormWireReader<MemberRole> = normWireLiteral("beam", "column", "floor", "bridge");
export const parseSupportType: NormWireReader<SupportType> = normWireLiteral("simplySupported", "cantilever", "continuousTwoSpan");
export const parseLaminationProduct: NormWireReader<LaminationProduct> = normWireLiteral("solid", "glulam");
export const parsePrestressBarProduct: NormWireReader<PrestressBarProduct> = normWireLiteral("threaded-bar", "smooth-bar");
export const parseTaperKind: NormWireReader<TaperKind> = normWireLiteral("single-tapered", "double-tapered", "curved", "pitched-cambered");
export const parseTensionSide: NormWireReader<TensionSide> = normWireLiteral("tension", "compression");
export const parseCutoutKind: NormWireReader<CutoutKind> = normWireLiteral("notch", "hole");
export const parseCutoutFace: NormWireReader<CutoutFace> = normWireLiteral("bottom", "top");
export const parseDiaphragmKind: NormWireReader<DiaphragmKind> = normWireLiteral("wall", "floor", "roof");
export const parseDiaphragmMethod: NormWireReader<DiaphragmMethod> = normWireLiteral("method-a", "method-b");
export const parseTrussKind: NormWireReader<TrussKind> = normWireLiteral("warren", "pratt", "howe", "king-post");
export const parseBracingProduct: NormWireReader<BracingProduct> = normWireLiteral("solid", "engineered");
export const parseBracingKind: NormWireReader<BracingKind> = normWireLiteral("plan-bracing", "bridging", "elastic-restraint");
export const parseLayerOrientation: NormWireReader<LayerOrientation> = normWireLiteral("longitudinal", "transverse");
export const parseCaseNumber: NormWireReader<CaseNumber> = normWireObject<CaseNumber>({ id: normWireRequired(normWireString), value: normWireRequired(normWireNumber) });
export const parseCaseText: NormWireReader<CaseText> = normWireObject<CaseText>({ id: normWireRequired(normWireString), value: normWireRequired(normWireString) });
export const parseCaseRow: NormWireReader<CaseRow> = normWireObject<CaseRow>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), entryId: normWireRequired(normWireString), numbers: normWireRequired(normWireArray(normWireRef(() => parseCaseNumber))), texts: normWireRequired(normWireArray(normWireRef(() => parseCaseText))) });
