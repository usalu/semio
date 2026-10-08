/** 🔺️ `En1999Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireDefault, normWireInteger, normWireNullable, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type AluminiumConnection, type AluminiumMaterial, type AluminiumMember, type AluminiumSection, type AluminiumShell, type AnnexChoice, type ColdFormedSheet, type FatigueDetail, type FireScenario, type MemberAction, parseAluminiumConnection, parseAluminiumMaterial, parseAluminiumMember, parseAluminiumSection, parseAluminiumShell, parseAnnexChoice, parseColdFormedSheet, parseFatigueDetail, parseFireScenario, parseMemberAction, parsePlateElement, type PlateElement } from "../📸️snapshot/🟦️.ts";

export interface En1999Diff {
  /** @state artifact */
  annex: AnnexChoice | null;
  /** @state artifact */
  materials: En1999MaterialsRows | null;
  /** @state artifact */
  sections: En1999SectionsRows | null;
  /** @state artifact */
  members: En1999MembersRows | null;
  /** @state artifact */
  connections: En1999ConnectionsRows | null;
  /** @state artifact */
  fireScenarios: En1999FireScenariosRows | null;
  /** @state artifact */
  fatigueDetails: En1999FatigueDetailsRows | null;
  /** @state artifact */
  coldFormed: En1999ColdFormedRows | null;
  /** @state artifact */
  shells: En1999ShellsRows | null;
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
  materialId: string | null;
  thickness: number | null;
  width: number | null;
  span: number | null;
  welded: boolean | null;
  actions: MemberAction[] | null;
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
  weldsThroat: number | null;
  boltsRows: number | null;
  boltsBoltsPerRow: number | null;
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
  memberId: string | null;
  detailCategory: string | null;
  deltaSigmaC: number | null;
  deltaSigmaEd: number | null;
  nCycles: number | null;
  m1: number | null;
  m2: number | null;
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
  memberId: string | null;
  thetaA: number | null;
  durationS: number | null;
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
  designation: string | null;
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
  nK: number | null;
  mYK: number | null;
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
  bucklingLengthY: number | null;
  bucklingLengthZ: number | null;
  bucklingLengthT: number | null;
  ltbLength: number | null;
  actions: En1999MembersActionsRows | null;
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
  thickness: number | null;
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
  kind: string | null;
  height: number | null;
  width: number | null;
  flangeThickness: number | null;
  webThickness: number | null;
  outerDiameter: number | null;
  elements: En1999SectionsElementsRows | null;
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
  materialId: string | null;
  radius: number | null;
  thickness: number | null;
  length: number | null;
  actions: MemberAction[] | null;
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

export const parseEn1999Diff: NormWireReader<En1999Diff> = normWireObject<En1999Diff>({ annex: normWireDefault(normWireNullable(parseAnnexChoice), () => null), materials: normWireDefault(normWireNullable(normWireRef(() => parseEn1999MaterialsRows)), () => null), sections: normWireDefault(normWireNullable(normWireRef(() => parseEn1999SectionsRows)), () => null), members: normWireDefault(normWireNullable(normWireRef(() => parseEn1999MembersRows)), () => null), connections: normWireDefault(normWireNullable(normWireRef(() => parseEn1999ConnectionsRows)), () => null), fireScenarios: normWireDefault(normWireNullable(normWireRef(() => parseEn1999FireScenariosRows)), () => null), fatigueDetails: normWireDefault(normWireNullable(normWireRef(() => parseEn1999FatigueDetailsRows)), () => null), coldFormed: normWireDefault(normWireNullable(normWireRef(() => parseEn1999ColdFormedRows)), () => null), shells: normWireDefault(normWireNullable(normWireRef(() => parseEn1999ShellsRows)), () => null) });
export const parseEn1999ColdFormedInserted: NormWireReader<En1999ColdFormedInserted> = normWireObject<En1999ColdFormedInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseColdFormedSheet) });
export const parseEn1999ColdFormedModified: NormWireReader<En1999ColdFormedModified> = normWireObject<En1999ColdFormedModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999ColdFormedPatch)) });
export const parseEn1999ColdFormedMoved: NormWireReader<En1999ColdFormedMoved> = normWireObject<En1999ColdFormedMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999ColdFormedPatch: NormWireReader<En1999ColdFormedPatch> = normWireObject<En1999ColdFormedPatch>({ materialId: normWireRequired(normWireNullable(normWireString)), thickness: normWireRequired(normWireNullable(normWireNumber)), width: normWireRequired(normWireNullable(normWireNumber)), span: normWireRequired(normWireNullable(normWireNumber)), welded: normWireRequired(normWireNullable(normWireBoolean)), actions: normWireRequired(normWireNullable(normWireArray(parseMemberAction))) });
export const parseEn1999ColdFormedRemoved: NormWireReader<En1999ColdFormedRemoved> = normWireObject<En1999ColdFormedRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999ColdFormedRows: NormWireReader<En1999ColdFormedRows> = normWireObject<En1999ColdFormedRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999ColdFormedRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999ColdFormedInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999ColdFormedMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999ColdFormedModified))) });
export const parseEn1999ConnectionsInserted: NormWireReader<En1999ConnectionsInserted> = normWireObject<En1999ConnectionsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseAluminiumConnection) });
export const parseEn1999ConnectionsModified: NormWireReader<En1999ConnectionsModified> = normWireObject<En1999ConnectionsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999ConnectionsPatch)) });
export const parseEn1999ConnectionsMoved: NormWireReader<En1999ConnectionsMoved> = normWireObject<En1999ConnectionsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999ConnectionsPatch: NormWireReader<En1999ConnectionsPatch> = normWireObject<En1999ConnectionsPatch>({ weldsThroat: normWireRequired(normWireNullable(normWireNumber)), boltsRows: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), boltsBoltsPerRow: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))) });
export const parseEn1999ConnectionsRemoved: NormWireReader<En1999ConnectionsRemoved> = normWireObject<En1999ConnectionsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999ConnectionsRows: NormWireReader<En1999ConnectionsRows> = normWireObject<En1999ConnectionsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999ConnectionsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999ConnectionsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999ConnectionsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999ConnectionsModified))) });
export const parseEn1999FatigueDetailsInserted: NormWireReader<En1999FatigueDetailsInserted> = normWireObject<En1999FatigueDetailsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseFatigueDetail) });
export const parseEn1999FatigueDetailsModified: NormWireReader<En1999FatigueDetailsModified> = normWireObject<En1999FatigueDetailsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999FatigueDetailsPatch)) });
export const parseEn1999FatigueDetailsMoved: NormWireReader<En1999FatigueDetailsMoved> = normWireObject<En1999FatigueDetailsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999FatigueDetailsPatch: NormWireReader<En1999FatigueDetailsPatch> = normWireObject<En1999FatigueDetailsPatch>({ memberId: normWireRequired(normWireNullable(normWireString)), detailCategory: normWireRequired(normWireNullable(normWireString)), deltaSigmaC: normWireRequired(normWireNullable(normWireNumber)), deltaSigmaEd: normWireRequired(normWireNullable(normWireNumber)), nCycles: normWireRequired(normWireNullable(normWireNumber)), m1: normWireRequired(normWireNullable(normWireNumber)), m2: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1999FatigueDetailsRemoved: NormWireReader<En1999FatigueDetailsRemoved> = normWireObject<En1999FatigueDetailsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999FatigueDetailsRows: NormWireReader<En1999FatigueDetailsRows> = normWireObject<En1999FatigueDetailsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999FatigueDetailsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999FatigueDetailsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999FatigueDetailsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999FatigueDetailsModified))) });
export const parseEn1999FireScenariosInserted: NormWireReader<En1999FireScenariosInserted> = normWireObject<En1999FireScenariosInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseFireScenario) });
export const parseEn1999FireScenariosModified: NormWireReader<En1999FireScenariosModified> = normWireObject<En1999FireScenariosModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999FireScenariosPatch)) });
export const parseEn1999FireScenariosMoved: NormWireReader<En1999FireScenariosMoved> = normWireObject<En1999FireScenariosMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999FireScenariosPatch: NormWireReader<En1999FireScenariosPatch> = normWireObject<En1999FireScenariosPatch>({ memberId: normWireRequired(normWireNullable(normWireString)), thetaA: normWireRequired(normWireNullable(normWireNumber)), durationS: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1999FireScenariosRemoved: NormWireReader<En1999FireScenariosRemoved> = normWireObject<En1999FireScenariosRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999FireScenariosRows: NormWireReader<En1999FireScenariosRows> = normWireObject<En1999FireScenariosRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999FireScenariosRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999FireScenariosInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999FireScenariosMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999FireScenariosModified))) });
export const parseEn1999MaterialsInserted: NormWireReader<En1999MaterialsInserted> = normWireObject<En1999MaterialsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseAluminiumMaterial) });
export const parseEn1999MaterialsModified: NormWireReader<En1999MaterialsModified> = normWireObject<En1999MaterialsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999MaterialsPatch)) });
export const parseEn1999MaterialsMoved: NormWireReader<En1999MaterialsMoved> = normWireObject<En1999MaterialsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999MaterialsPatch: NormWireReader<En1999MaterialsPatch> = normWireObject<En1999MaterialsPatch>({ designation: normWireRequired(normWireNullable(normWireString)) });
export const parseEn1999MaterialsRemoved: NormWireReader<En1999MaterialsRemoved> = normWireObject<En1999MaterialsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999MaterialsRows: NormWireReader<En1999MaterialsRows> = normWireObject<En1999MaterialsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999MaterialsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999MaterialsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999MaterialsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999MaterialsModified))) });
export const parseEn1999MembersActionsInserted: NormWireReader<En1999MembersActionsInserted> = normWireObject<En1999MembersActionsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseMemberAction) });
export const parseEn1999MembersActionsModified: NormWireReader<En1999MembersActionsModified> = normWireObject<En1999MembersActionsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999MembersActionsPatch)) });
export const parseEn1999MembersActionsMoved: NormWireReader<En1999MembersActionsMoved> = normWireObject<En1999MembersActionsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999MembersActionsPatch: NormWireReader<En1999MembersActionsPatch> = normWireObject<En1999MembersActionsPatch>({ nK: normWireRequired(normWireNullable(normWireNumber)), mYK: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1999MembersActionsRemoved: NormWireReader<En1999MembersActionsRemoved> = normWireObject<En1999MembersActionsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999MembersActionsRows: NormWireReader<En1999MembersActionsRows> = normWireObject<En1999MembersActionsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999MembersActionsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999MembersActionsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999MembersActionsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999MembersActionsModified))) });
export const parseEn1999MembersInserted: NormWireReader<En1999MembersInserted> = normWireObject<En1999MembersInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseAluminiumMember) });
export const parseEn1999MembersModified: NormWireReader<En1999MembersModified> = normWireObject<En1999MembersModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999MembersPatch)) });
export const parseEn1999MembersMoved: NormWireReader<En1999MembersMoved> = normWireObject<En1999MembersMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999MembersPatch: NormWireReader<En1999MembersPatch> = normWireObject<En1999MembersPatch>({ bucklingLengthY: normWireRequired(normWireNullable(normWireNumber)), bucklingLengthZ: normWireRequired(normWireNullable(normWireNumber)), bucklingLengthT: normWireRequired(normWireNullable(normWireNumber)), ltbLength: normWireRequired(normWireNullable(normWireNumber)), actions: normWireRequired(normWireNullable(normWireRef(() => parseEn1999MembersActionsRows))) });
export const parseEn1999MembersRemoved: NormWireReader<En1999MembersRemoved> = normWireObject<En1999MembersRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999MembersRows: NormWireReader<En1999MembersRows> = normWireObject<En1999MembersRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999MembersRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999MembersInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999MembersMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999MembersModified))) });
export const parseEn1999SectionsElementsInserted: NormWireReader<En1999SectionsElementsInserted> = normWireObject<En1999SectionsElementsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parsePlateElement) });
export const parseEn1999SectionsElementsModified: NormWireReader<En1999SectionsElementsModified> = normWireObject<En1999SectionsElementsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999SectionsElementsPatch)) });
export const parseEn1999SectionsElementsMoved: NormWireReader<En1999SectionsElementsMoved> = normWireObject<En1999SectionsElementsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999SectionsElementsPatch: NormWireReader<En1999SectionsElementsPatch> = normWireObject<En1999SectionsElementsPatch>({ thickness: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1999SectionsElementsRemoved: NormWireReader<En1999SectionsElementsRemoved> = normWireObject<En1999SectionsElementsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999SectionsElementsRows: NormWireReader<En1999SectionsElementsRows> = normWireObject<En1999SectionsElementsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999SectionsElementsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999SectionsElementsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999SectionsElementsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999SectionsElementsModified))) });
export const parseEn1999SectionsInserted: NormWireReader<En1999SectionsInserted> = normWireObject<En1999SectionsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseAluminiumSection) });
export const parseEn1999SectionsModified: NormWireReader<En1999SectionsModified> = normWireObject<En1999SectionsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999SectionsPatch)) });
export const parseEn1999SectionsMoved: NormWireReader<En1999SectionsMoved> = normWireObject<En1999SectionsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999SectionsPatch: NormWireReader<En1999SectionsPatch> = normWireObject<En1999SectionsPatch>({ kind: normWireRequired(normWireNullable(normWireString)), height: normWireRequired(normWireNullable(normWireNumber)), width: normWireRequired(normWireNullable(normWireNumber)), flangeThickness: normWireRequired(normWireNullable(normWireNumber)), webThickness: normWireRequired(normWireNullable(normWireNumber)), outerDiameter: normWireRequired(normWireNullable(normWireNumber)), elements: normWireRequired(normWireNullable(normWireRef(() => parseEn1999SectionsElementsRows))) });
export const parseEn1999SectionsRemoved: NormWireReader<En1999SectionsRemoved> = normWireObject<En1999SectionsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999SectionsRows: NormWireReader<En1999SectionsRows> = normWireObject<En1999SectionsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999SectionsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999SectionsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999SectionsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999SectionsModified))) });
export const parseEn1999ShellsInserted: NormWireReader<En1999ShellsInserted> = normWireObject<En1999ShellsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseAluminiumShell) });
export const parseEn1999ShellsModified: NormWireReader<En1999ShellsModified> = normWireObject<En1999ShellsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1999ShellsPatch)) });
export const parseEn1999ShellsMoved: NormWireReader<En1999ShellsMoved> = normWireObject<En1999ShellsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999ShellsPatch: NormWireReader<En1999ShellsPatch> = normWireObject<En1999ShellsPatch>({ materialId: normWireRequired(normWireNullable(normWireString)), radius: normWireRequired(normWireNullable(normWireNumber)), thickness: normWireRequired(normWireNullable(normWireNumber)), length: normWireRequired(normWireNullable(normWireNumber)), actions: normWireRequired(normWireNullable(normWireArray(parseMemberAction))) });
export const parseEn1999ShellsRemoved: NormWireReader<En1999ShellsRemoved> = normWireObject<En1999ShellsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1999ShellsRows: NormWireReader<En1999ShellsRows> = normWireObject<En1999ShellsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1999ShellsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1999ShellsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1999ShellsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999ShellsModified))) });
