/** 🔺️ `En1999Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireInteger, normWireNullable, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type AluminiumConnection, type AluminiumMaterial, type AluminiumMember, type AluminiumSection, type AluminiumShell, type AnnexChoice, type ColdFormedSheet, type FatigueDetail, type FireScenario, parseAluminiumConnection, parseAluminiumMaterial, parseAluminiumMember, parseAluminiumSection, parseAluminiumShell, parseAnnexChoice, parseColdFormedSheet, parseFatigueDetail, parseFireScenario } from "../📸️snapshot/🟦️.ts";

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

export interface En1999ColdFormedRows {
  added: ColdFormedSheet[];
  removed: string[];
  order: string[] | null;
}

export interface En1999ConnectionsPatch {
  id: string;
  weldsThroat: number | null;
  boltsRows: number | null;
  boltsBoltsPerRow: number | null;
}

export interface En1999ConnectionsRows {
  added: AluminiumConnection[];
  removed: string[];
  modified: En1999ConnectionsPatch[];
  order: string[] | null;
}

export interface En1999FatigueDetailsRows {
  added: FatigueDetail[];
  removed: string[];
  order: string[] | null;
}

export interface En1999FireScenariosRows {
  added: FireScenario[];
  removed: string[];
  order: string[] | null;
}

export interface En1999MaterialsPatch {
  id: string;
  designation: string | null;
}

export interface En1999MaterialsRows {
  added: AluminiumMaterial[];
  removed: string[];
  modified: En1999MaterialsPatch[];
  order: string[] | null;
}

export interface En1999MembersActionsPatch {
  id: string;
  nK: number | null;
  mYK: number | null;
}

export interface En1999MembersActionsRows {
  modified: En1999MembersActionsPatch[];
}

export interface En1999MembersPatch {
  id: string;
  bucklingLengthY: number | null;
  bucklingLengthZ: number | null;
  bucklingLengthT: number | null;
  ltbLength: number | null;
  actions: En1999MembersActionsRows | null;
}

export interface En1999MembersRows {
  added: AluminiumMember[];
  removed: string[];
  modified: En1999MembersPatch[];
  order: string[] | null;
}

export interface En1999SectionsElementsPatch {
  id: string;
  thickness: number | null;
}

export interface En1999SectionsElementsRows {
  modified: En1999SectionsElementsPatch[];
}

export interface En1999SectionsPatch {
  id: string;
  elements: En1999SectionsElementsRows | null;
}

export interface En1999SectionsRows {
  added: AluminiumSection[];
  removed: string[];
  modified: En1999SectionsPatch[];
  order: string[] | null;
}

export interface En1999ShellsRows {
  added: AluminiumShell[];
  removed: string[];
  order: string[] | null;
}

export const parseEn1999Diff: NormWireReader<En1999Diff> = normWireObject<En1999Diff>({ annex: normWireDefault(normWireNullable(parseAnnexChoice), () => null), materials: normWireDefault(normWireNullable(normWireRef(() => parseEn1999MaterialsRows)), () => null), sections: normWireDefault(normWireNullable(normWireRef(() => parseEn1999SectionsRows)), () => null), members: normWireDefault(normWireNullable(normWireRef(() => parseEn1999MembersRows)), () => null), connections: normWireDefault(normWireNullable(normWireRef(() => parseEn1999ConnectionsRows)), () => null), fireScenarios: normWireDefault(normWireNullable(normWireRef(() => parseEn1999FireScenariosRows)), () => null), fatigueDetails: normWireDefault(normWireNullable(normWireRef(() => parseEn1999FatigueDetailsRows)), () => null), coldFormed: normWireDefault(normWireNullable(normWireRef(() => parseEn1999ColdFormedRows)), () => null), shells: normWireDefault(normWireNullable(normWireRef(() => parseEn1999ShellsRows)), () => null) });
export const parseEn1999ColdFormedRows: NormWireReader<En1999ColdFormedRows> = normWireObject<En1999ColdFormedRows>({ added: normWireRequired(normWireArray(parseColdFormedSheet)), removed: normWireRequired(normWireArray(normWireString)), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseEn1999ConnectionsPatch: NormWireReader<En1999ConnectionsPatch> = normWireObject<En1999ConnectionsPatch>({ id: normWireRequired(normWireString), weldsThroat: normWireRequired(normWireNullable(normWireNumber)), boltsRows: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), boltsBoltsPerRow: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))) });
export const parseEn1999ConnectionsRows: NormWireReader<En1999ConnectionsRows> = normWireObject<En1999ConnectionsRows>({ added: normWireRequired(normWireArray(parseAluminiumConnection)), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999ConnectionsPatch))), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseEn1999FatigueDetailsRows: NormWireReader<En1999FatigueDetailsRows> = normWireObject<En1999FatigueDetailsRows>({ added: normWireRequired(normWireArray(parseFatigueDetail)), removed: normWireRequired(normWireArray(normWireString)), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseEn1999FireScenariosRows: NormWireReader<En1999FireScenariosRows> = normWireObject<En1999FireScenariosRows>({ added: normWireRequired(normWireArray(parseFireScenario)), removed: normWireRequired(normWireArray(normWireString)), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseEn1999MaterialsPatch: NormWireReader<En1999MaterialsPatch> = normWireObject<En1999MaterialsPatch>({ id: normWireRequired(normWireString), designation: normWireRequired(normWireNullable(normWireString)) });
export const parseEn1999MaterialsRows: NormWireReader<En1999MaterialsRows> = normWireObject<En1999MaterialsRows>({ added: normWireRequired(normWireArray(parseAluminiumMaterial)), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999MaterialsPatch))), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseEn1999MembersActionsPatch: NormWireReader<En1999MembersActionsPatch> = normWireObject<En1999MembersActionsPatch>({ id: normWireRequired(normWireString), nK: normWireRequired(normWireNullable(normWireNumber)), mYK: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1999MembersActionsRows: NormWireReader<En1999MembersActionsRows> = normWireObject<En1999MembersActionsRows>({ modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999MembersActionsPatch))) });
export const parseEn1999MembersPatch: NormWireReader<En1999MembersPatch> = normWireObject<En1999MembersPatch>({ id: normWireRequired(normWireString), bucklingLengthY: normWireRequired(normWireNullable(normWireNumber)), bucklingLengthZ: normWireRequired(normWireNullable(normWireNumber)), bucklingLengthT: normWireRequired(normWireNullable(normWireNumber)), ltbLength: normWireRequired(normWireNullable(normWireNumber)), actions: normWireRequired(normWireNullable(normWireRef(() => parseEn1999MembersActionsRows))) });
export const parseEn1999MembersRows: NormWireReader<En1999MembersRows> = normWireObject<En1999MembersRows>({ added: normWireRequired(normWireArray(parseAluminiumMember)), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999MembersPatch))), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseEn1999SectionsElementsPatch: NormWireReader<En1999SectionsElementsPatch> = normWireObject<En1999SectionsElementsPatch>({ id: normWireRequired(normWireString), thickness: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1999SectionsElementsRows: NormWireReader<En1999SectionsElementsRows> = normWireObject<En1999SectionsElementsRows>({ modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999SectionsElementsPatch))) });
export const parseEn1999SectionsPatch: NormWireReader<En1999SectionsPatch> = normWireObject<En1999SectionsPatch>({ id: normWireRequired(normWireString), elements: normWireRequired(normWireNullable(normWireRef(() => parseEn1999SectionsElementsRows))) });
export const parseEn1999SectionsRows: NormWireReader<En1999SectionsRows> = normWireObject<En1999SectionsRows>({ added: normWireRequired(normWireArray(parseAluminiumSection)), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1999SectionsPatch))), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseEn1999ShellsRows: NormWireReader<En1999ShellsRows> = normWireObject<En1999ShellsRows>({ added: normWireRequired(normWireArray(parseAluminiumShell)), removed: normWireRequired(normWireArray(normWireString)), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
