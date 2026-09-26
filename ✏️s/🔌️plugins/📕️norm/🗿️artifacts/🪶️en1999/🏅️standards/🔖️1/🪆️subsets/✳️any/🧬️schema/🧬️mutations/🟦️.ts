/** 🧬️ En1999 document mutations — discriminated union mirroring `En1999Mutation` / `KINDS`. */

import type {
  AluminiumConnection,
  AluminiumMaterial,
  AluminiumMember,
  AluminiumSection,
  AluminiumShell,
  AnnexChoice,
  ColdFormedSheet,
  FatigueDetail,
  FireScenario,
} from "../📸️snapshot/🟦️.ts";

export interface ChangeAnnex {
  newAnnex: AnnexChoice;
}

export interface ChangeMaterials {
  materials: AluminiumMaterial[];
}

export interface ChangeSections {
  sections: AluminiumSection[];
}

export interface ChangeMembers {
  members: AluminiumMember[];
}

export interface ChangeConnections {
  connections: AluminiumConnection[];
}

export interface ChangeFireScenarios {
  fireScenarios: FireScenario[];
}

export interface ChangeFatigueDetails {
  fatigueDetails: FatigueDetail[];
}

export interface ChangeColdFormed {
  coldFormed: ColdFormedSheet[];
}

export interface ChangeShells {
  shells: AluminiumShell[];
}

export interface AddMember {
  index: number;
  member: AluminiumMember;
}

export interface RemoveMember {
  id: string;
}

export interface ChangeMemberNEd {
  memberId: string;
  actionId: string;
  newNK: number;
}

export interface ChangeMemberMYEd {
  memberId: string;
  actionId: string;
  newMYK: number;
}

export interface ChangeMemberBucklingLength {
  memberId: string;
  axis: string;
  newLength: number;
}

export interface ChangeMaterialDesignation {
  materialId: string;
  newDesignation: string;
}

export interface ChangePlateThickness {
  sectionId: string;
  elementId: string;
  newThickness: number;
}

export interface ChangeWeldThroat {
  connectionId: string;
  newThroat: number;
}

export interface ChangeBoltCount {
  connectionId: string;
  newRows: number;
  newBoltsPerRow: number;
}

/** 🏷️ Semantic mutation kind strings — must match Rust `KINDS`. */
export const EN1999_MUTATION_KINDS = [
  "change-annex",
  "change-materials",
  "change-sections",
  "change-members",
  "change-connections",
  "change-fire-scenarios",
  "change-fatigue-details",
  "change-cold-formed",
  "change-shells",
  "add-member",
  "remove-member",
  "change-member-n-ed",
  "change-member-my-ed",
  "change-member-buckling-length",
  "change-material-designation",
  "change-plate-thickness",
  "change-weld-throat",
  "change-bolt-count",
] as const;

export type En1999MutationKind = (typeof EN1999_MUTATION_KINDS)[number];

export type En1999Mutation =
    ({ mutation: "changeAnnex" } & ChangeAnnex)
  | ({ mutation: "changeMaterials" } & ChangeMaterials)
  | ({ mutation: "changeSections" } & ChangeSections)
  | ({ mutation: "changeMembers" } & ChangeMembers)
  | ({ mutation: "changeConnections" } & ChangeConnections)
  | ({ mutation: "changeFireScenarios" } & ChangeFireScenarios)
  | ({ mutation: "changeFatigueDetails" } & ChangeFatigueDetails)
  | ({ mutation: "changeColdFormed" } & ChangeColdFormed)
  | ({ mutation: "changeShells" } & ChangeShells)
  | ({ mutation: "addMember" } & AddMember)
  | ({ mutation: "removeMember" } & RemoveMember)
  | ({ mutation: "changeMemberNEd" } & ChangeMemberNEd)
  | ({ mutation: "changeMemberMYEd" } & ChangeMemberMYEd)
  | ({ mutation: "changeMemberBucklingLength" } & ChangeMemberBucklingLength)
  | ({ mutation: "changeMaterialDesignation" } & ChangeMaterialDesignation)
  | ({ mutation: "changePlateThickness" } & ChangePlateThickness)
  | ({ mutation: "changeWeldThroat" } & ChangeWeldThroat)
  | ({ mutation: "changeBoltCount" } & ChangeBoltCount)
;
