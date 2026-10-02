/** 🧺️ `En1999Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireTagged } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ChangeMaterialDesignation, parseChangeMaterialDesignation } from "./⚗️change-material-designation/🧬️schema/🟦️.ts";
import { type ChangeColdFormed, parseChangeColdFormed } from "./❄️change-cold-formed/🧬️schema/🟦️.ts";
import { type AddMember, parseAddMember } from "./➕add-member/🧬️schema/🟦️.ts";
import { parseRemoveMember, type RemoveMember } from "./➖remove-member/🧬️schema/🟦️.ts";
import { type ChangeMemberMYEd, parseChangeMemberMYEd } from "./⤴️change-member-my-ed/🧬️schema/🟦️.ts";
import { type ChangeAnnex, parseChangeAnnex } from "./🌍️change-annex/🧬️schema/🟦️.ts";
import { type ChangeMemberNEd, parseChangeMemberNEd } from "./🏋️change-member-n-ed/🧬️schema/🟦️.ts";
import { type ChangeMembers, parseChangeMembers } from "./🏗️change-members/🧬️schema/🟦️.ts";
import { type ChangeMemberBucklingLength, parseChangeMemberBucklingLength } from "./📏️change-member-buckling-length/🧬️schema/🟦️.ts";
import { type ChangeSections, parseChangeSections } from "./📐️change-sections/🧬️schema/🟦️.ts";
import { type ChangeFatigueDetails, parseChangeFatigueDetails } from "./🔄️change-fatigue-details/🧬️schema/🟦️.ts";
import { type ChangeConnections, parseChangeConnections } from "./🔗change-connections/🧬️schema/🟦️.ts";
import { type ChangeFireScenarios, parseChangeFireScenarios } from "./🔥️change-fire-scenarios/🧬️schema/🟦️.ts";
import { type ChangeWeldThroat, parseChangeWeldThroat } from "./🔥️change-weld-throat/🧬️schema/🟦️.ts";
import { type ChangeBoltCount, parseChangeBoltCount } from "./🔩change-bolt-count/🧬️schema/🟦️.ts";
import { type ChangeMaterials, parseChangeMaterials } from "./🧱change-materials/🧬️schema/🟦️.ts";
import { type ChangePlateThickness, parseChangePlateThickness } from "./🧱change-plate-thickness/🧬️schema/🟦️.ts";
import { type ChangeShells, parseChangeShells } from "./🫙change-shells/🧬️schema/🟦️.ts";

export type En1999Mutation =
  | ChangeAnnex
  | ChangeMaterials
  | ChangeSections
  | ChangeMembers
  | ChangeConnections
  | ChangeFireScenarios
  | ChangeFatigueDetails
  | ChangeColdFormed
  | ChangeShells
  | AddMember
  | RemoveMember
  | ChangeMemberNEd
  | ChangeMemberMYEd
  | ChangeMemberBucklingLength
  | ChangeMaterialDesignation
  | ChangePlateThickness
  | ChangeWeldThroat
  | ChangeBoltCount;

export const parseEn1999Mutation: NormWireReader<En1999Mutation> = normWireTagged<En1999Mutation, "mutation">("mutation", {
  changeAnnex: parseChangeAnnex,
  changeMaterials: parseChangeMaterials,
  changeSections: parseChangeSections,
  changeMembers: parseChangeMembers,
  changeConnections: parseChangeConnections,
  changeFireScenarios: parseChangeFireScenarios,
  changeFatigueDetails: parseChangeFatigueDetails,
  changeColdFormed: parseChangeColdFormed,
  changeShells: parseChangeShells,
  addMember: parseAddMember,
  removeMember: parseRemoveMember,
  changeMemberNEd: parseChangeMemberNEd,
  changeMemberMYEd: parseChangeMemberMYEd,
  changeMemberBucklingLength: parseChangeMemberBucklingLength,
  changeMaterialDesignation: parseChangeMaterialDesignation,
  changePlateThickness: parseChangePlateThickness,
  changeWeldThroat: parseChangeWeldThroat,
  changeBoltCount: parseChangeBoltCount,
});
