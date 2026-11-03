/** 🔺️ `En1999Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireOptional, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionCategory, parseActionCategory, type ActionKind, parseActionKind, type ActionSource, parseActionSource, type AluminiumConnection, parseAluminiumConnection, type AluminiumMaterial, parseAluminiumMaterial, type AluminiumMember, parseAluminiumMember, type AluminiumSection, parseAluminiumSection, type AluminiumShell, parseAluminiumShell, type AnnexChoice, parseAnnexChoice, type ColdFormedSheet, parseColdFormedSheet, type ConnectionKind, parseConnectionKind, type DetailType, parseDetailType, type FabricationClass, parseFabricationClass, type FastenerForceMethod, parseFastenerForceMethod, type FatigueCategory, parseFatigueCategory, type FatigueDetail, parseFatigueDetail, type FireScenario, parseFireScenario, type MemberAction, parseMemberAction, type OutlineRing, parseOutlineRing, type PatchLoad, parsePatchLoad, type PatchLoadPosition, parsePatchLoadPosition, type Placement, parsePlacement, type PlateElement, parsePlateElement, type ProductForm, parseProductForm, type RingStiffener, parseRingStiffener, type SectionKind, parseSectionKind, type SectionVertex, parseSectionVertex, type StiffenerSides, parseStiffenerSides, type SupportCondition, parseSupportCondition, type TransverseStiffener, parseTransverseStiffener, type WeldDirection, parseWeldDirection, type WeldProcess, parseWeldProcess, type WeldRun, parseWeldRun } from "../📸️snapshot/🟦️.ts";

export interface En1999Diff {
  /** @state artifact */
  annex?: AnnexChoice | null;
  /** @state artifact */
  materials?: En1999MaterialsRows | null;
  /** @state artifact */
  sections?: En1999SectionsRows | null;
  /** @state artifact */
  members?: En1999MembersRows | null;
  /** @state artifact */
  connections?: En1999ConnectionsRows | null;
  /** @state artifact */
  fireScenarios?: En1999FireScenariosRows | null;
  /** @state artifact */
  fatigueDetails?: En1999FatigueDetailsRows | null;
  /** @state artifact */
  coldFormed?: En1999ColdFormedRows | null;
  /** @state artifact */
  shells?: En1999ShellsRows | null;
  /** @state artifact */
  placements?: En1999PlacementsRows | null;
  /** @state artifact */
  weldRuns?: En1999WeldRunsRows | null;
  /** @state artifact */
  outlineVertices?: En1999OutlineVerticesRows | null;
  /** @state artifact */
  fatigueCategories?: En1999FatigueCategoriesRows | null;
  /** @state artifact */
  patchLoads?: En1999PatchLoadsRows | null;
  /** @state artifact */
  ringStiffeners?: En1999RingStiffenersRows | null;
  /** @state artifact */
  transverseStiffeners?: En1999TransverseStiffenersRows | null;
  /** @state artifact */
  reliabilityClass?: ("RC1" | "RC2" | "RC3") | null;
}

export interface En1999ColdFormedActionsInserted {
  index: number;
  row: MemberAction;
}

export interface En1999ColdFormedActionsModified {
  id: string;
  patch: En1999ColdFormedActionsPatch;
}

export interface En1999ColdFormedActionsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1999ColdFormedActionsPatch {
  kind?: ActionKind | null;
  category?: ActionCategory | null;
  source?: ActionSource | null;
  gKLine?: number | null;
  qKLine?: number | null;
  nK?: number | null;
  vYK?: number | null;
  vZK?: number | null;
  mYK?: number | null;
  mZK?: number | null;
  torsionK?: number | null;
  localLoadK?: number | null;
}

export interface En1999ColdFormedActionsRemoved {
  id: string;
  index: number;
}

export interface En1999ColdFormedActionsRows {
  removed: En1999ColdFormedActionsRemoved[];
  inserted: En1999ColdFormedActionsInserted[];
  moved: En1999ColdFormedActionsMoved[];
  modified: En1999ColdFormedActionsModified[];
}

export interface En1999ColdFormedInserted {
  index: number;
  row: ColdFormedSheet;
}

export interface En1999ColdFormedModified {
  id: string;
  patch: En1999ColdFormedPatch;
}

export interface En1999ColdFormedMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1999ColdFormedPatch {
  materialId?: string | null;
  thickness?: number | null;
  width?: number | null;
  span?: number | null;
  welded?: boolean | null;
  actions?: En1999ColdFormedActionsRows | null;
}

export interface En1999ColdFormedRemoved {
  id: string;
  index: number;
}

export interface En1999ColdFormedRows {
  removed: En1999ColdFormedRemoved[];
  inserted: En1999ColdFormedInserted[];
  moved: En1999ColdFormedMoved[];
  modified: En1999ColdFormedModified[];
}

export interface En1999ConnectionsActionsInserted {
  index: number;
  row: MemberAction;
}

export interface En1999ConnectionsActionsModified {
  id: string;
  patch: En1999ConnectionsActionsPatch;
}

export interface En1999ConnectionsActionsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1999ConnectionsActionsPatch {
  kind?: ActionKind | null;
  category?: ActionCategory | null;
  source?: ActionSource | null;
  gKLine?: number | null;
  qKLine?: number | null;
  nK?: number | null;
  vYK?: number | null;
  vZK?: number | null;
  mYK?: number | null;
  mZK?: number | null;
  torsionK?: number | null;
  localLoadK?: number | null;
}

export interface En1999ConnectionsActionsRemoved {
  id: string;
  index: number;
}

export interface En1999ConnectionsActionsRows {
  removed: En1999ConnectionsActionsRemoved[];
  inserted: En1999ConnectionsActionsInserted[];
  moved: En1999ConnectionsActionsMoved[];
  modified: En1999ConnectionsActionsModified[];
}

export interface En1999ConnectionsInserted {
  index: number;
  row: AluminiumConnection;
}

export interface En1999ConnectionsModified {
  id: string;
  patch: En1999ConnectionsPatch;
}

export interface En1999ConnectionsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1999ConnectionsPatch {
  memberId?: string | null;
  materialId?: string | null;
  kind?: ConnectionKind | null;
  boltsMaterial?: string | null;
  boltsDiameter?: number | null;
  boltsRows?: number | null;
  boltsBoltsPerRow?: number | null;
  boltsEdgeDistance?: number | null;
  boltsPitch?: number | null;
  boltsGauge?: number | null;
  boltsPlateThickness?: number | null;
  weldsFillerAlloy?: string | null;
  weldsThroat?: number | null;
  weldsLength?: number | null;
  weldsBetaW?: number | null;
  weldsHazExtent?: number | null;
  actions?: En1999ConnectionsActionsRows | null;
  loadEccentricity?: number | null;
  holeStagger?: number | null;
  forceMethod?: FastenerForceMethod | null;
}

export interface En1999ConnectionsRemoved {
  id: string;
  index: number;
}

export interface En1999ConnectionsRows {
  removed: En1999ConnectionsRemoved[];
  inserted: En1999ConnectionsInserted[];
  moved: En1999ConnectionsMoved[];
  modified: En1999ConnectionsModified[];
}

export interface En1999FatigueCategoriesInserted {
  index: number;
  row: FatigueCategory;
}

export interface En1999FatigueCategoriesModified {
  id: string;
  patch: En1999FatigueCategoriesPatch;
}

export interface En1999FatigueCategoriesMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1999FatigueCategoriesPatch {
  memberId?: string | null;
  elementId?: string | null;
  detailCategory?: string | null;
  detailType?: DetailType | null;
  locationX?: number | null;
  m1?: number | null;
  m2?: number | null;
}

export interface En1999FatigueCategoriesRemoved {
  id: string;
  index: number;
}

export interface En1999FatigueCategoriesRows {
  removed: En1999FatigueCategoriesRemoved[];
  inserted: En1999FatigueCategoriesInserted[];
  moved: En1999FatigueCategoriesMoved[];
  modified: En1999FatigueCategoriesModified[];
}

export interface En1999FatigueDetailsInserted {
  index: number;
  row: FatigueDetail;
}

export interface En1999FatigueDetailsModified {
  id: string;
  patch: En1999FatigueDetailsPatch;
}

export interface En1999FatigueDetailsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1999FatigueDetailsPatch {
  memberId?: string | null;
  detailCategory?: string | null;
  deltaSigmaC?: number | null;
  deltaSigmaEd?: number | null;
  nCycles?: number | null;
  m1?: number | null;
  m2?: number | null;
}

export interface En1999FatigueDetailsRemoved {
  id: string;
  index: number;
}

export interface En1999FatigueDetailsRows {
  removed: En1999FatigueDetailsRemoved[];
  inserted: En1999FatigueDetailsInserted[];
  moved: En1999FatigueDetailsMoved[];
  modified: En1999FatigueDetailsModified[];
}

export interface En1999FireScenariosInserted {
  index: number;
  row: FireScenario;
}

export interface En1999FireScenariosModified {
  id: string;
  patch: En1999FireScenariosPatch;
}

export interface En1999FireScenariosMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1999FireScenariosPatch {
  memberId?: string | null;
  thetaA?: number | null;
  durationS?: number | null;
}

export interface En1999FireScenariosRemoved {
  id: string;
  index: number;
}

export interface En1999FireScenariosRows {
  removed: En1999FireScenariosRemoved[];
  inserted: En1999FireScenariosInserted[];
  moved: En1999FireScenariosMoved[];
  modified: En1999FireScenariosModified[];
}

export interface En1999MaterialsInserted {
  index: number;
  row: AluminiumMaterial;
}

export interface En1999MaterialsModified {
  id: string;
  patch: En1999MaterialsPatch;
}

export interface En1999MaterialsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1999MaterialsPatch {
  designation?: string | null;
  productForm?: ProductForm | null;
}

export interface En1999MaterialsRemoved {
  id: string;
  index: number;
}

export interface En1999MaterialsRows {
  removed: En1999MaterialsRemoved[];
  inserted: En1999MaterialsInserted[];
  moved: En1999MaterialsMoved[];
  modified: En1999MaterialsModified[];
}

export interface En1999MembersActionsInserted {
  index: number;
  row: MemberAction;
}

export interface En1999MembersActionsModified {
  id: string;
  patch: En1999MembersActionsPatch;
}

export interface En1999MembersActionsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1999MembersActionsPatch {
  kind?: ActionKind | null;
  category?: ActionCategory | null;
  source?: ActionSource | null;
  gKLine?: number | null;
  qKLine?: number | null;
  nK?: number | null;
  vYK?: number | null;
  vZK?: number | null;
  mYK?: number | null;
  mZK?: number | null;
  torsionK?: number | null;
  localLoadK?: number | null;
}

export interface En1999MembersActionsRemoved {
  id: string;
  index: number;
}

export interface En1999MembersActionsRows {
  removed: En1999MembersActionsRemoved[];
  inserted: En1999MembersActionsInserted[];
  moved: En1999MembersActionsMoved[];
  modified: En1999MembersActionsModified[];
}

export interface En1999MembersInserted {
  index: number;
  row: AluminiumMember;
}

export interface En1999MembersModified {
  id: string;
  patch: En1999MembersPatch;
}

export interface En1999MembersMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1999MembersPatch {
  sectionId?: string | null;
  materialId?: string | null;
  length?: number | null;
  support?: SupportCondition | null;
  bucklingLengthY?: number | null;
  bucklingLengthZ?: number | null;
  bucklingLengthT?: number | null;
  ltbLength?: number | null;
  c1?: number | null;
  restrainedLtb?: boolean | null;
  bearingLength?: number | null;
  transverseWeld?: boolean | null;
  transverseWeldOffset?: number | null;
  actions?: En1999MembersActionsRows | null;
}

export interface En1999MembersRemoved {
  id: string;
  index: number;
}

export interface En1999MembersRows {
  removed: En1999MembersRemoved[];
  inserted: En1999MembersInserted[];
  moved: En1999MembersMoved[];
  modified: En1999MembersModified[];
}

export interface En1999OutlineVerticesInserted {
  index: number;
  row: SectionVertex;
}

export interface En1999OutlineVerticesModified {
  id: string;
  patch: En1999OutlineVerticesPatch;
}

export interface En1999OutlineVerticesMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1999OutlineVerticesPatch {
  sectionId?: string | null;
  ring?: OutlineRing | null;
  elementId?: string | null;
  y?: number | null;
  z?: number | null;
}

export interface En1999OutlineVerticesRemoved {
  id: string;
  index: number;
}

export interface En1999OutlineVerticesRows {
  removed: En1999OutlineVerticesRemoved[];
  inserted: En1999OutlineVerticesInserted[];
  moved: En1999OutlineVerticesMoved[];
  modified: En1999OutlineVerticesModified[];
}

export interface En1999PatchLoadsInserted {
  index: number;
  row: PatchLoad;
}

export interface En1999PatchLoadsModified {
  id: string;
  patch: En1999PatchLoadsPatch;
}

export interface En1999PatchLoadsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1999PatchLoadsPatch {
  subjectId?: string | null;
  position?: PatchLoadPosition | null;
  x?: number | null;
  bearingLength?: number | null;
  webHeight?: number | null;
  webAngle?: number | null;
  cornerRadius?: number | null;
  load?: number | null;
}

export interface En1999PatchLoadsRemoved {
  id: string;
  index: number;
}

export interface En1999PatchLoadsRows {
  removed: En1999PatchLoadsRemoved[];
  inserted: En1999PatchLoadsInserted[];
  moved: En1999PatchLoadsMoved[];
  modified: En1999PatchLoadsModified[];
}

export interface En1999PlacementsInserted {
  index: number;
  row: Placement;
}

export interface En1999PlacementsModified {
  id: string;
  patch: En1999PlacementsPatch;
}

export interface En1999PlacementsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1999PlacementsPatch {
  x?: number | null;
  y?: number | null;
  z?: number | null;
  yaw?: number | null;
}

export interface En1999PlacementsRemoved {
  id: string;
  index: number;
}

export interface En1999PlacementsRows {
  removed: En1999PlacementsRemoved[];
  inserted: En1999PlacementsInserted[];
  moved: En1999PlacementsMoved[];
  modified: En1999PlacementsModified[];
}

export interface En1999RingStiffenersInserted {
  index: number;
  row: RingStiffener;
}

export interface En1999RingStiffenersModified {
  id: string;
  patch: En1999RingStiffenersPatch;
}

export interface En1999RingStiffenersMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1999RingStiffenersPatch {
  shellId?: string | null;
  x?: number | null;
  depth?: number | null;
  thickness?: number | null;
  internal?: boolean | null;
}

export interface En1999RingStiffenersRemoved {
  id: string;
  index: number;
}

export interface En1999RingStiffenersRows {
  removed: En1999RingStiffenersRemoved[];
  inserted: En1999RingStiffenersInserted[];
  moved: En1999RingStiffenersMoved[];
  modified: En1999RingStiffenersModified[];
}

export interface En1999SectionsElementsInserted {
  index: number;
  row: PlateElement;
}

export interface En1999SectionsElementsModified {
  id: string;
  patch: En1999SectionsElementsPatch;
}

export interface En1999SectionsElementsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1999SectionsElementsPatch {
  width?: number | null;
  thickness?: number | null;
  outstand?: boolean | null;
  welded?: boolean | null;
  weldPosition?: number | null;
  weldProcess?: WeldProcess | null;
  weldTJunction?: boolean | null;
  weldDirection?: WeldDirection | null;
  edgeStiffenerDepth?: number | null;
  edgeStiffenerThickness?: number | null;
}

export interface En1999SectionsElementsRemoved {
  id: string;
  index: number;
}

export interface En1999SectionsElementsRows {
  removed: En1999SectionsElementsRemoved[];
  inserted: En1999SectionsElementsInserted[];
  moved: En1999SectionsElementsMoved[];
  modified: En1999SectionsElementsModified[];
}

export interface En1999SectionsInserted {
  index: number;
  row: AluminiumSection;
}

export interface En1999SectionsModified {
  id: string;
  patch: En1999SectionsPatch;
}

export interface En1999SectionsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1999SectionsPatch {
  kind?: SectionKind | null;
  height?: number | null;
  width?: number | null;
  flangeThickness?: number | null;
  webThickness?: number | null;
  outerDiameter?: number | null;
  elements?: En1999SectionsElementsRows | null;
}

export interface En1999SectionsRemoved {
  id: string;
  index: number;
}

export interface En1999SectionsRows {
  removed: En1999SectionsRemoved[];
  inserted: En1999SectionsInserted[];
  moved: En1999SectionsMoved[];
  modified: En1999SectionsModified[];
}

export interface En1999ShellsActionsInserted {
  index: number;
  row: MemberAction;
}

export interface En1999ShellsActionsModified {
  id: string;
  patch: En1999ShellsActionsPatch;
}

export interface En1999ShellsActionsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1999ShellsActionsPatch {
  kind?: ActionKind | null;
  category?: ActionCategory | null;
  source?: ActionSource | null;
  gKLine?: number | null;
  qKLine?: number | null;
  nK?: number | null;
  vYK?: number | null;
  vZK?: number | null;
  mYK?: number | null;
  mZK?: number | null;
  torsionK?: number | null;
  localLoadK?: number | null;
}

export interface En1999ShellsActionsRemoved {
  id: string;
  index: number;
}

export interface En1999ShellsActionsRows {
  removed: En1999ShellsActionsRemoved[];
  inserted: En1999ShellsActionsInserted[];
  moved: En1999ShellsActionsMoved[];
  modified: En1999ShellsActionsModified[];
}

export interface En1999ShellsInserted {
  index: number;
  row: AluminiumShell;
}

export interface En1999ShellsModified {
  id: string;
  patch: En1999ShellsPatch;
}

export interface En1999ShellsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1999ShellsPatch {
  materialId?: string | null;
  radius?: number | null;
  thickness?: number | null;
  length?: number | null;
  welded?: boolean | null;
  fabricationClass?: FabricationClass | null;
  actions?: En1999ShellsActionsRows | null;
}

export interface En1999ShellsRemoved {
  id: string;
  index: number;
}

export interface En1999ShellsRows {
  removed: En1999ShellsRemoved[];
  inserted: En1999ShellsInserted[];
  moved: En1999ShellsMoved[];
  modified: En1999ShellsModified[];
}

export interface En1999TransverseStiffenersInserted {
  index: number;
  row: TransverseStiffener;
}

export interface En1999TransverseStiffenersModified {
  id: string;
  patch: En1999TransverseStiffenersPatch;
}

export interface En1999TransverseStiffenersMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1999TransverseStiffenersPatch {
  girderId?: string | null;
  position?: number | null;
  width?: number | null;
  thickness?: number | null;
  sides?: StiffenerSides | null;
}

export interface En1999TransverseStiffenersRemoved {
  id: string;
  index: number;
}

export interface En1999TransverseStiffenersRows {
  removed: En1999TransverseStiffenersRemoved[];
  inserted: En1999TransverseStiffenersInserted[];
  moved: En1999TransverseStiffenersMoved[];
  modified: En1999TransverseStiffenersModified[];
}

export interface En1999WeldRunsInserted {
  index: number;
  row: WeldRun;
}

export interface En1999WeldRunsModified {
  id: string;
  patch: En1999WeldRunsPatch;
}

export interface En1999WeldRunsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1999WeldRunsPatch {
  memberId?: string | null;
  elementId?: string | null;
  start?: number | null;
  length?: number | null;
  offset?: number | null;
  throat?: number | null;
  process?: WeldProcess | null;
  tJunction?: boolean | null;
  direction?: WeldDirection | null;
  hazExtent?: number | null;
}

export interface En1999WeldRunsRemoved {
  id: string;
  index: number;
}

export interface En1999WeldRunsRows {
  removed: En1999WeldRunsRemoved[];
  inserted: En1999WeldRunsInserted[];
  moved: En1999WeldRunsMoved[];
  modified: En1999WeldRunsModified[];
}

export const parseEn1999Diff: NormWireReader<En1999Diff> = normWireObject<En1999Diff>({ annex: normWireOptional(normWireNullable(parseAnnexChoice)), materials: normWireOptional(normWireNullable(normWireRef(() => parseEn1999MaterialsRows))), sections: normWireOptional(normWireNullable(normWireRef(() => parseEn1999SectionsRows))), members: normWireOptional(normWireNullable(normWireRef(() => parseEn1999MembersRows))), connections: normWireOptional(normWireNullable(normWireRef(() => parseEn1999ConnectionsRows))), fireScenarios: normWireOptional(normWireNullable(normWireRef(() => parseEn1999FireScenariosRows))), fatigueDetails: normWireOptional(normWireNullable(normWireRef(() => parseEn1999FatigueDetailsRows))), coldFormed: normWireOptional(normWireNullable(normWireRef(() => parseEn1999ColdFormedRows))), shells: normWireOptional(normWireNullable(normWireRef(() => parseEn1999ShellsRows))), placements: normWireOptional(normWireNullable(normWireRef(() => parseEn1999PlacementsRows))), weldRuns: normWireOptional(normWireNullable(normWireRef(() => parseEn1999WeldRunsRows))), outlineVertices: normWireOptional(normWireNullable(normWireRef(() => parseEn1999OutlineVerticesRows))), fatigueCategories: normWireOptional(normWireNullable(normWireRef(() => parseEn1999FatigueCategoriesRows))), patchLoads: normWireOptional(normWireNullable(normWireRef(() => parseEn1999PatchLoadsRows))), ringStiffeners: normWireOptional(normWireNullable(normWireRef(() => parseEn1999RingStiffenersRows))), transverseStiffeners: normWireOptional(normWireNullable(normWireRef(() => parseEn1999TransverseStiffenersRows))), reliabilityClass: normWireOptional(normWireNullable(normWireLiteral("RC1", "RC2", "RC3"))) });
export const parseEn1999ColdFormedActionsInserted: NormWireReader<En1999ColdFormedActionsInserted> = normWireObject<En1999ColdFormedActionsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseMemberAction) });
export const parseEn1999ColdFormedActionsModified: NormWireReader<En1999ColdFormedActionsModified> = normWireObject<En1999ColdFormedActionsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999ColdFormedActionsPatch)) });
export const parseEn1999ColdFormedActionsMoved: NormWireReader<En1999ColdFormedActionsMoved> = normWireObject<En1999ColdFormedActionsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999ColdFormedActionsPatch: NormWireReader<En1999ColdFormedActionsPatch> = normWireObject<En1999ColdFormedActionsPatch>({ kind: normWireOptional(normWireNullable(parseActionKind)), category: normWireOptional(normWireNullable(parseActionCategory)), source: normWireOptional(normWireNullable(parseActionSource)), gKLine: normWireOptional(normWireNullable(normWireNumber)), qKLine: normWireOptional(normWireNullable(normWireNumber)), nK: normWireOptional(normWireNullable(normWireNumber)), vYK: normWireOptional(normWireNullable(normWireNumber)), vZK: normWireOptional(normWireNullable(normWireNumber)), mYK: normWireOptional(normWireNullable(normWireNumber)), mZK: normWireOptional(normWireNullable(normWireNumber)), torsionK: normWireOptional(normWireNullable(normWireNumber)), localLoadK: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1999ColdFormedActionsRemoved: NormWireReader<En1999ColdFormedActionsRemoved> = normWireObject<En1999ColdFormedActionsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999ColdFormedActionsRows: NormWireReader<En1999ColdFormedActionsRows> = normWireObject<En1999ColdFormedActionsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999ColdFormedActionsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999ColdFormedActionsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999ColdFormedActionsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999ColdFormedActionsModified))) });
export const parseEn1999ColdFormedInserted: NormWireReader<En1999ColdFormedInserted> = normWireObject<En1999ColdFormedInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseColdFormedSheet) });
export const parseEn1999ColdFormedModified: NormWireReader<En1999ColdFormedModified> = normWireObject<En1999ColdFormedModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999ColdFormedPatch)) });
export const parseEn1999ColdFormedMoved: NormWireReader<En1999ColdFormedMoved> = normWireObject<En1999ColdFormedMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999ColdFormedPatch: NormWireReader<En1999ColdFormedPatch> = normWireObject<En1999ColdFormedPatch>({ materialId: normWireOptional(normWireNullable(normWireString)), thickness: normWireOptional(normWireNullable(normWireNumber)), width: normWireOptional(normWireNullable(normWireNumber)), span: normWireOptional(normWireNullable(normWireNumber)), welded: normWireOptional(normWireNullable(normWireBoolean)), actions: normWireOptional(normWireNullable(normWireRef(() => parseEn1999ColdFormedActionsRows))) });
export const parseEn1999ColdFormedRemoved: NormWireReader<En1999ColdFormedRemoved> = normWireObject<En1999ColdFormedRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999ColdFormedRows: NormWireReader<En1999ColdFormedRows> = normWireObject<En1999ColdFormedRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999ColdFormedRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999ColdFormedInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999ColdFormedMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999ColdFormedModified))) });
export const parseEn1999ConnectionsActionsInserted: NormWireReader<En1999ConnectionsActionsInserted> = normWireObject<En1999ConnectionsActionsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseMemberAction) });
export const parseEn1999ConnectionsActionsModified: NormWireReader<En1999ConnectionsActionsModified> = normWireObject<En1999ConnectionsActionsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999ConnectionsActionsPatch)) });
export const parseEn1999ConnectionsActionsMoved: NormWireReader<En1999ConnectionsActionsMoved> = normWireObject<En1999ConnectionsActionsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999ConnectionsActionsPatch: NormWireReader<En1999ConnectionsActionsPatch> = normWireObject<En1999ConnectionsActionsPatch>({ kind: normWireOptional(normWireNullable(parseActionKind)), category: normWireOptional(normWireNullable(parseActionCategory)), source: normWireOptional(normWireNullable(parseActionSource)), gKLine: normWireOptional(normWireNullable(normWireNumber)), qKLine: normWireOptional(normWireNullable(normWireNumber)), nK: normWireOptional(normWireNullable(normWireNumber)), vYK: normWireOptional(normWireNullable(normWireNumber)), vZK: normWireOptional(normWireNullable(normWireNumber)), mYK: normWireOptional(normWireNullable(normWireNumber)), mZK: normWireOptional(normWireNullable(normWireNumber)), torsionK: normWireOptional(normWireNullable(normWireNumber)), localLoadK: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1999ConnectionsActionsRemoved: NormWireReader<En1999ConnectionsActionsRemoved> = normWireObject<En1999ConnectionsActionsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999ConnectionsActionsRows: NormWireReader<En1999ConnectionsActionsRows> = normWireObject<En1999ConnectionsActionsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999ConnectionsActionsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999ConnectionsActionsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999ConnectionsActionsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999ConnectionsActionsModified))) });
export const parseEn1999ConnectionsInserted: NormWireReader<En1999ConnectionsInserted> = normWireObject<En1999ConnectionsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseAluminiumConnection) });
export const parseEn1999ConnectionsModified: NormWireReader<En1999ConnectionsModified> = normWireObject<En1999ConnectionsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999ConnectionsPatch)) });
export const parseEn1999ConnectionsMoved: NormWireReader<En1999ConnectionsMoved> = normWireObject<En1999ConnectionsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999ConnectionsPatch: NormWireReader<En1999ConnectionsPatch> = normWireObject<En1999ConnectionsPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), materialId: normWireOptional(normWireNullable(normWireString)), kind: normWireOptional(normWireNullable(parseConnectionKind)), boltsMaterial: normWireOptional(normWireNullable(normWireString)), boltsDiameter: normWireOptional(normWireNullable(normWireNumber)), boltsRows: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), boltsBoltsPerRow: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), boltsEdgeDistance: normWireOptional(normWireNullable(normWireNumber)), boltsPitch: normWireOptional(normWireNullable(normWireNumber)), boltsGauge: normWireOptional(normWireNullable(normWireNumber)), boltsPlateThickness: normWireOptional(normWireNullable(normWireNumber)), weldsFillerAlloy: normWireOptional(normWireNullable(normWireString)), weldsThroat: normWireOptional(normWireNullable(normWireNumber)), weldsLength: normWireOptional(normWireNullable(normWireNumber)), weldsBetaW: normWireOptional(normWireNullable(normWireNumber)), weldsHazExtent: normWireOptional(normWireNullable(normWireNumber)), actions: normWireOptional(normWireNullable(normWireRef(() => parseEn1999ConnectionsActionsRows))), loadEccentricity: normWireOptional(normWireNullable(normWireNumber)), holeStagger: normWireOptional(normWireNullable(normWireNumber)), forceMethod: normWireOptional(normWireNullable(parseFastenerForceMethod)) });
export const parseEn1999ConnectionsRemoved: NormWireReader<En1999ConnectionsRemoved> = normWireObject<En1999ConnectionsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999ConnectionsRows: NormWireReader<En1999ConnectionsRows> = normWireObject<En1999ConnectionsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999ConnectionsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999ConnectionsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999ConnectionsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999ConnectionsModified))) });
export const parseEn1999FatigueCategoriesInserted: NormWireReader<En1999FatigueCategoriesInserted> = normWireObject<En1999FatigueCategoriesInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseFatigueCategory) });
export const parseEn1999FatigueCategoriesModified: NormWireReader<En1999FatigueCategoriesModified> = normWireObject<En1999FatigueCategoriesModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999FatigueCategoriesPatch)) });
export const parseEn1999FatigueCategoriesMoved: NormWireReader<En1999FatigueCategoriesMoved> = normWireObject<En1999FatigueCategoriesMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999FatigueCategoriesPatch: NormWireReader<En1999FatigueCategoriesPatch> = normWireObject<En1999FatigueCategoriesPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), elementId: normWireOptional(normWireNullable(normWireString)), detailCategory: normWireOptional(normWireNullable(normWireString)), detailType: normWireOptional(normWireNullable(parseDetailType)), locationX: normWireOptional(normWireNullable(normWireNumber)), m1: normWireOptional(normWireNullable(normWireNumber)), m2: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1999FatigueCategoriesRemoved: NormWireReader<En1999FatigueCategoriesRemoved> = normWireObject<En1999FatigueCategoriesRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999FatigueCategoriesRows: NormWireReader<En1999FatigueCategoriesRows> = normWireObject<En1999FatigueCategoriesRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999FatigueCategoriesRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999FatigueCategoriesInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999FatigueCategoriesMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999FatigueCategoriesModified))) });
export const parseEn1999FatigueDetailsInserted: NormWireReader<En1999FatigueDetailsInserted> = normWireObject<En1999FatigueDetailsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseFatigueDetail) });
export const parseEn1999FatigueDetailsModified: NormWireReader<En1999FatigueDetailsModified> = normWireObject<En1999FatigueDetailsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999FatigueDetailsPatch)) });
export const parseEn1999FatigueDetailsMoved: NormWireReader<En1999FatigueDetailsMoved> = normWireObject<En1999FatigueDetailsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999FatigueDetailsPatch: NormWireReader<En1999FatigueDetailsPatch> = normWireObject<En1999FatigueDetailsPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), detailCategory: normWireOptional(normWireNullable(normWireString)), deltaSigmaC: normWireOptional(normWireNullable(normWireNumber)), deltaSigmaEd: normWireOptional(normWireNullable(normWireNumber)), nCycles: normWireOptional(normWireNullable(normWireNumber)), m1: normWireOptional(normWireNullable(normWireNumber)), m2: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1999FatigueDetailsRemoved: NormWireReader<En1999FatigueDetailsRemoved> = normWireObject<En1999FatigueDetailsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999FatigueDetailsRows: NormWireReader<En1999FatigueDetailsRows> = normWireObject<En1999FatigueDetailsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999FatigueDetailsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999FatigueDetailsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999FatigueDetailsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999FatigueDetailsModified))) });
export const parseEn1999FireScenariosInserted: NormWireReader<En1999FireScenariosInserted> = normWireObject<En1999FireScenariosInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseFireScenario) });
export const parseEn1999FireScenariosModified: NormWireReader<En1999FireScenariosModified> = normWireObject<En1999FireScenariosModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999FireScenariosPatch)) });
export const parseEn1999FireScenariosMoved: NormWireReader<En1999FireScenariosMoved> = normWireObject<En1999FireScenariosMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999FireScenariosPatch: NormWireReader<En1999FireScenariosPatch> = normWireObject<En1999FireScenariosPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), thetaA: normWireOptional(normWireNullable(normWireNumber)), durationS: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1999FireScenariosRemoved: NormWireReader<En1999FireScenariosRemoved> = normWireObject<En1999FireScenariosRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999FireScenariosRows: NormWireReader<En1999FireScenariosRows> = normWireObject<En1999FireScenariosRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999FireScenariosRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999FireScenariosInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999FireScenariosMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999FireScenariosModified))) });
export const parseEn1999MaterialsInserted: NormWireReader<En1999MaterialsInserted> = normWireObject<En1999MaterialsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseAluminiumMaterial) });
export const parseEn1999MaterialsModified: NormWireReader<En1999MaterialsModified> = normWireObject<En1999MaterialsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999MaterialsPatch)) });
export const parseEn1999MaterialsMoved: NormWireReader<En1999MaterialsMoved> = normWireObject<En1999MaterialsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999MaterialsPatch: NormWireReader<En1999MaterialsPatch> = normWireObject<En1999MaterialsPatch>({ designation: normWireOptional(normWireNullable(normWireString)), productForm: normWireOptional(normWireNullable(parseProductForm)) });
export const parseEn1999MaterialsRemoved: NormWireReader<En1999MaterialsRemoved> = normWireObject<En1999MaterialsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999MaterialsRows: NormWireReader<En1999MaterialsRows> = normWireObject<En1999MaterialsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999MaterialsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999MaterialsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999MaterialsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999MaterialsModified))) });
export const parseEn1999MembersActionsInserted: NormWireReader<En1999MembersActionsInserted> = normWireObject<En1999MembersActionsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseMemberAction) });
export const parseEn1999MembersActionsModified: NormWireReader<En1999MembersActionsModified> = normWireObject<En1999MembersActionsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999MembersActionsPatch)) });
export const parseEn1999MembersActionsMoved: NormWireReader<En1999MembersActionsMoved> = normWireObject<En1999MembersActionsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999MembersActionsPatch: NormWireReader<En1999MembersActionsPatch> = normWireObject<En1999MembersActionsPatch>({ kind: normWireOptional(normWireNullable(parseActionKind)), category: normWireOptional(normWireNullable(parseActionCategory)), source: normWireOptional(normWireNullable(parseActionSource)), gKLine: normWireOptional(normWireNullable(normWireNumber)), qKLine: normWireOptional(normWireNullable(normWireNumber)), nK: normWireOptional(normWireNullable(normWireNumber)), vYK: normWireOptional(normWireNullable(normWireNumber)), vZK: normWireOptional(normWireNullable(normWireNumber)), mYK: normWireOptional(normWireNullable(normWireNumber)), mZK: normWireOptional(normWireNullable(normWireNumber)), torsionK: normWireOptional(normWireNullable(normWireNumber)), localLoadK: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1999MembersActionsRemoved: NormWireReader<En1999MembersActionsRemoved> = normWireObject<En1999MembersActionsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999MembersActionsRows: NormWireReader<En1999MembersActionsRows> = normWireObject<En1999MembersActionsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999MembersActionsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999MembersActionsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999MembersActionsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999MembersActionsModified))) });
export const parseEn1999MembersInserted: NormWireReader<En1999MembersInserted> = normWireObject<En1999MembersInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseAluminiumMember) });
export const parseEn1999MembersModified: NormWireReader<En1999MembersModified> = normWireObject<En1999MembersModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999MembersPatch)) });
export const parseEn1999MembersMoved: NormWireReader<En1999MembersMoved> = normWireObject<En1999MembersMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999MembersPatch: NormWireReader<En1999MembersPatch> = normWireObject<En1999MembersPatch>({ sectionId: normWireOptional(normWireNullable(normWireString)), materialId: normWireOptional(normWireNullable(normWireString)), length: normWireOptional(normWireNullable(normWireNumber)), support: normWireOptional(normWireNullable(parseSupportCondition)), bucklingLengthY: normWireOptional(normWireNullable(normWireNumber)), bucklingLengthZ: normWireOptional(normWireNullable(normWireNumber)), bucklingLengthT: normWireOptional(normWireNullable(normWireNumber)), ltbLength: normWireOptional(normWireNullable(normWireNumber)), c1: normWireOptional(normWireNullable(normWireNumber)), restrainedLtb: normWireOptional(normWireNullable(normWireBoolean)), bearingLength: normWireOptional(normWireNullable(normWireNumber)), transverseWeld: normWireOptional(normWireNullable(normWireBoolean)), transverseWeldOffset: normWireOptional(normWireNullable(normWireNumber)), actions: normWireOptional(normWireNullable(normWireRef(() => parseEn1999MembersActionsRows))) });
export const parseEn1999MembersRemoved: NormWireReader<En1999MembersRemoved> = normWireObject<En1999MembersRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999MembersRows: NormWireReader<En1999MembersRows> = normWireObject<En1999MembersRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999MembersRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999MembersInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999MembersMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999MembersModified))) });
export const parseEn1999OutlineVerticesInserted: NormWireReader<En1999OutlineVerticesInserted> = normWireObject<En1999OutlineVerticesInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseSectionVertex) });
export const parseEn1999OutlineVerticesModified: NormWireReader<En1999OutlineVerticesModified> = normWireObject<En1999OutlineVerticesModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999OutlineVerticesPatch)) });
export const parseEn1999OutlineVerticesMoved: NormWireReader<En1999OutlineVerticesMoved> = normWireObject<En1999OutlineVerticesMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999OutlineVerticesPatch: NormWireReader<En1999OutlineVerticesPatch> = normWireObject<En1999OutlineVerticesPatch>({ sectionId: normWireOptional(normWireNullable(normWireString)), ring: normWireOptional(normWireNullable(parseOutlineRing)), elementId: normWireOptional(normWireNullable(normWireString)), y: normWireOptional(normWireNullable(normWireNumber)), z: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1999OutlineVerticesRemoved: NormWireReader<En1999OutlineVerticesRemoved> = normWireObject<En1999OutlineVerticesRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999OutlineVerticesRows: NormWireReader<En1999OutlineVerticesRows> = normWireObject<En1999OutlineVerticesRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999OutlineVerticesRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999OutlineVerticesInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999OutlineVerticesMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999OutlineVerticesModified))) });
export const parseEn1999PatchLoadsInserted: NormWireReader<En1999PatchLoadsInserted> = normWireObject<En1999PatchLoadsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parsePatchLoad) });
export const parseEn1999PatchLoadsModified: NormWireReader<En1999PatchLoadsModified> = normWireObject<En1999PatchLoadsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999PatchLoadsPatch)) });
export const parseEn1999PatchLoadsMoved: NormWireReader<En1999PatchLoadsMoved> = normWireObject<En1999PatchLoadsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999PatchLoadsPatch: NormWireReader<En1999PatchLoadsPatch> = normWireObject<En1999PatchLoadsPatch>({ subjectId: normWireOptional(normWireNullable(normWireString)), position: normWireOptional(normWireNullable(parsePatchLoadPosition)), x: normWireOptional(normWireNullable(normWireNumber)), bearingLength: normWireOptional(normWireNullable(normWireNumber)), webHeight: normWireOptional(normWireNullable(normWireNumber)), webAngle: normWireOptional(normWireNullable(normWireNumber)), cornerRadius: normWireOptional(normWireNullable(normWireNumber)), load: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1999PatchLoadsRemoved: NormWireReader<En1999PatchLoadsRemoved> = normWireObject<En1999PatchLoadsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999PatchLoadsRows: NormWireReader<En1999PatchLoadsRows> = normWireObject<En1999PatchLoadsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999PatchLoadsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999PatchLoadsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999PatchLoadsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999PatchLoadsModified))) });
export const parseEn1999PlacementsInserted: NormWireReader<En1999PlacementsInserted> = normWireObject<En1999PlacementsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parsePlacement) });
export const parseEn1999PlacementsModified: NormWireReader<En1999PlacementsModified> = normWireObject<En1999PlacementsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999PlacementsPatch)) });
export const parseEn1999PlacementsMoved: NormWireReader<En1999PlacementsMoved> = normWireObject<En1999PlacementsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999PlacementsPatch: NormWireReader<En1999PlacementsPatch> = normWireObject<En1999PlacementsPatch>({ x: normWireOptional(normWireNullable(normWireNumber)), y: normWireOptional(normWireNullable(normWireNumber)), z: normWireOptional(normWireNullable(normWireNumber)), yaw: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1999PlacementsRemoved: NormWireReader<En1999PlacementsRemoved> = normWireObject<En1999PlacementsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999PlacementsRows: NormWireReader<En1999PlacementsRows> = normWireObject<En1999PlacementsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999PlacementsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999PlacementsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999PlacementsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999PlacementsModified))) });
export const parseEn1999RingStiffenersInserted: NormWireReader<En1999RingStiffenersInserted> = normWireObject<En1999RingStiffenersInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseRingStiffener) });
export const parseEn1999RingStiffenersModified: NormWireReader<En1999RingStiffenersModified> = normWireObject<En1999RingStiffenersModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999RingStiffenersPatch)) });
export const parseEn1999RingStiffenersMoved: NormWireReader<En1999RingStiffenersMoved> = normWireObject<En1999RingStiffenersMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999RingStiffenersPatch: NormWireReader<En1999RingStiffenersPatch> = normWireObject<En1999RingStiffenersPatch>({ shellId: normWireOptional(normWireNullable(normWireString)), x: normWireOptional(normWireNullable(normWireNumber)), depth: normWireOptional(normWireNullable(normWireNumber)), thickness: normWireOptional(normWireNullable(normWireNumber)), internal: normWireOptional(normWireNullable(normWireBoolean)) });
export const parseEn1999RingStiffenersRemoved: NormWireReader<En1999RingStiffenersRemoved> = normWireObject<En1999RingStiffenersRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999RingStiffenersRows: NormWireReader<En1999RingStiffenersRows> = normWireObject<En1999RingStiffenersRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999RingStiffenersRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999RingStiffenersInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999RingStiffenersMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999RingStiffenersModified))) });
export const parseEn1999SectionsElementsInserted: NormWireReader<En1999SectionsElementsInserted> = normWireObject<En1999SectionsElementsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parsePlateElement) });
export const parseEn1999SectionsElementsModified: NormWireReader<En1999SectionsElementsModified> = normWireObject<En1999SectionsElementsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999SectionsElementsPatch)) });
export const parseEn1999SectionsElementsMoved: NormWireReader<En1999SectionsElementsMoved> = normWireObject<En1999SectionsElementsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999SectionsElementsPatch: NormWireReader<En1999SectionsElementsPatch> = normWireObject<En1999SectionsElementsPatch>({ width: normWireOptional(normWireNullable(normWireNumber)), thickness: normWireOptional(normWireNullable(normWireNumber)), outstand: normWireOptional(normWireNullable(normWireBoolean)), welded: normWireOptional(normWireNullable(normWireBoolean)), weldPosition: normWireOptional(normWireNullable(normWireNumber)), weldProcess: normWireOptional(normWireNullable(parseWeldProcess)), weldTJunction: normWireOptional(normWireNullable(normWireBoolean)), weldDirection: normWireOptional(normWireNullable(parseWeldDirection)), edgeStiffenerDepth: normWireOptional(normWireNullable(normWireNumber)), edgeStiffenerThickness: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1999SectionsElementsRemoved: NormWireReader<En1999SectionsElementsRemoved> = normWireObject<En1999SectionsElementsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999SectionsElementsRows: NormWireReader<En1999SectionsElementsRows> = normWireObject<En1999SectionsElementsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999SectionsElementsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999SectionsElementsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999SectionsElementsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999SectionsElementsModified))) });
export const parseEn1999SectionsInserted: NormWireReader<En1999SectionsInserted> = normWireObject<En1999SectionsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseAluminiumSection) });
export const parseEn1999SectionsModified: NormWireReader<En1999SectionsModified> = normWireObject<En1999SectionsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999SectionsPatch)) });
export const parseEn1999SectionsMoved: NormWireReader<En1999SectionsMoved> = normWireObject<En1999SectionsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999SectionsPatch: NormWireReader<En1999SectionsPatch> = normWireObject<En1999SectionsPatch>({ kind: normWireOptional(normWireNullable(parseSectionKind)), height: normWireOptional(normWireNullable(normWireNumber)), width: normWireOptional(normWireNullable(normWireNumber)), flangeThickness: normWireOptional(normWireNullable(normWireNumber)), webThickness: normWireOptional(normWireNullable(normWireNumber)), outerDiameter: normWireOptional(normWireNullable(normWireNumber)), elements: normWireOptional(normWireNullable(normWireRef(() => parseEn1999SectionsElementsRows))) });
export const parseEn1999SectionsRemoved: NormWireReader<En1999SectionsRemoved> = normWireObject<En1999SectionsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999SectionsRows: NormWireReader<En1999SectionsRows> = normWireObject<En1999SectionsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999SectionsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999SectionsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999SectionsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999SectionsModified))) });
export const parseEn1999ShellsActionsInserted: NormWireReader<En1999ShellsActionsInserted> = normWireObject<En1999ShellsActionsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseMemberAction) });
export const parseEn1999ShellsActionsModified: NormWireReader<En1999ShellsActionsModified> = normWireObject<En1999ShellsActionsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999ShellsActionsPatch)) });
export const parseEn1999ShellsActionsMoved: NormWireReader<En1999ShellsActionsMoved> = normWireObject<En1999ShellsActionsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999ShellsActionsPatch: NormWireReader<En1999ShellsActionsPatch> = normWireObject<En1999ShellsActionsPatch>({ kind: normWireOptional(normWireNullable(parseActionKind)), category: normWireOptional(normWireNullable(parseActionCategory)), source: normWireOptional(normWireNullable(parseActionSource)), gKLine: normWireOptional(normWireNullable(normWireNumber)), qKLine: normWireOptional(normWireNullable(normWireNumber)), nK: normWireOptional(normWireNullable(normWireNumber)), vYK: normWireOptional(normWireNullable(normWireNumber)), vZK: normWireOptional(normWireNullable(normWireNumber)), mYK: normWireOptional(normWireNullable(normWireNumber)), mZK: normWireOptional(normWireNullable(normWireNumber)), torsionK: normWireOptional(normWireNullable(normWireNumber)), localLoadK: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1999ShellsActionsRemoved: NormWireReader<En1999ShellsActionsRemoved> = normWireObject<En1999ShellsActionsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999ShellsActionsRows: NormWireReader<En1999ShellsActionsRows> = normWireObject<En1999ShellsActionsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999ShellsActionsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999ShellsActionsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999ShellsActionsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999ShellsActionsModified))) });
export const parseEn1999ShellsInserted: NormWireReader<En1999ShellsInserted> = normWireObject<En1999ShellsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseAluminiumShell) });
export const parseEn1999ShellsModified: NormWireReader<En1999ShellsModified> = normWireObject<En1999ShellsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999ShellsPatch)) });
export const parseEn1999ShellsMoved: NormWireReader<En1999ShellsMoved> = normWireObject<En1999ShellsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999ShellsPatch: NormWireReader<En1999ShellsPatch> = normWireObject<En1999ShellsPatch>({ materialId: normWireOptional(normWireNullable(normWireString)), radius: normWireOptional(normWireNullable(normWireNumber)), thickness: normWireOptional(normWireNullable(normWireNumber)), length: normWireOptional(normWireNullable(normWireNumber)), welded: normWireOptional(normWireNullable(normWireBoolean)), fabricationClass: normWireOptional(normWireNullable(parseFabricationClass)), actions: normWireOptional(normWireNullable(normWireRef(() => parseEn1999ShellsActionsRows))) });
export const parseEn1999ShellsRemoved: NormWireReader<En1999ShellsRemoved> = normWireObject<En1999ShellsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999ShellsRows: NormWireReader<En1999ShellsRows> = normWireObject<En1999ShellsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999ShellsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999ShellsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999ShellsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999ShellsModified))) });
export const parseEn1999TransverseStiffenersInserted: NormWireReader<En1999TransverseStiffenersInserted> = normWireObject<En1999TransverseStiffenersInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseTransverseStiffener) });
export const parseEn1999TransverseStiffenersModified: NormWireReader<En1999TransverseStiffenersModified> = normWireObject<En1999TransverseStiffenersModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999TransverseStiffenersPatch)) });
export const parseEn1999TransverseStiffenersMoved: NormWireReader<En1999TransverseStiffenersMoved> = normWireObject<En1999TransverseStiffenersMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999TransverseStiffenersPatch: NormWireReader<En1999TransverseStiffenersPatch> = normWireObject<En1999TransverseStiffenersPatch>({ girderId: normWireOptional(normWireNullable(normWireString)), position: normWireOptional(normWireNullable(normWireNumber)), width: normWireOptional(normWireNullable(normWireNumber)), thickness: normWireOptional(normWireNullable(normWireNumber)), sides: normWireOptional(normWireNullable(parseStiffenerSides)) });
export const parseEn1999TransverseStiffenersRemoved: NormWireReader<En1999TransverseStiffenersRemoved> = normWireObject<En1999TransverseStiffenersRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999TransverseStiffenersRows: NormWireReader<En1999TransverseStiffenersRows> = normWireObject<En1999TransverseStiffenersRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999TransverseStiffenersRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999TransverseStiffenersInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999TransverseStiffenersMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999TransverseStiffenersModified))) });
export const parseEn1999WeldRunsInserted: NormWireReader<En1999WeldRunsInserted> = normWireObject<En1999WeldRunsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseWeldRun) });
export const parseEn1999WeldRunsModified: NormWireReader<En1999WeldRunsModified> = normWireObject<En1999WeldRunsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999WeldRunsPatch)) });
export const parseEn1999WeldRunsMoved: NormWireReader<En1999WeldRunsMoved> = normWireObject<En1999WeldRunsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999WeldRunsPatch: NormWireReader<En1999WeldRunsPatch> = normWireObject<En1999WeldRunsPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), elementId: normWireOptional(normWireNullable(normWireString)), start: normWireOptional(normWireNullable(normWireNumber)), length: normWireOptional(normWireNullable(normWireNumber)), offset: normWireOptional(normWireNullable(normWireNumber)), throat: normWireOptional(normWireNullable(normWireNumber)), process: normWireOptional(normWireNullable(parseWeldProcess)), tJunction: normWireOptional(normWireNullable(normWireBoolean)), direction: normWireOptional(normWireNullable(parseWeldDirection)), hazExtent: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1999WeldRunsRemoved: NormWireReader<En1999WeldRunsRemoved> = normWireObject<En1999WeldRunsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999WeldRunsRows: NormWireReader<En1999WeldRunsRows> = normWireObject<En1999WeldRunsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999WeldRunsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999WeldRunsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999WeldRunsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999WeldRunsModified))) });
