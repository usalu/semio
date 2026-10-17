/** 🧱️ ProgramSnapshot diff schema: the sparse deltas of the stakeholder, user, activity, function, element, quantity, relationship, adjacency, process, flow and access-rule registers. */
import type { AccessRule, Activity, Adjacency, FlowRequirement, Function, Process, ProgramElement, QuantityRequirement, Relationship, Stakeholder, UserProfile } from "../../🏗️program-structure/🟦️.ts";
import type { AccessRulePatch, ActivityPatch, AdjacencyPatch, FlowRequirementPatch, FunctionPatch, ProcessPatch, ProgramElementPatch, QuantityRequirementPatch, RelationshipPatch, StakeholderPatch, UserProfilePatch } from "../🩹️patch-types/🟦️.ts";

export interface ProgramStakeholdersDelta {
  removed: ProgramStakeholdersRemoval[];
  inserted: ProgramStakeholdersInsertion[];
  moved: ProgramStakeholdersRelocation[];
  modified: ProgramStakeholdersPatchEntry[];
}

export interface ProgramStakeholdersInsertion {
  index: number;
  row: Stakeholder;
}

export interface ProgramStakeholdersRemoval {
  id: string;
  index: number;
}

export interface ProgramStakeholdersRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramStakeholdersPatchEntry {
  id: string;
  patch: StakeholderPatch;
}

export interface ProgramUsersDelta {
  removed: ProgramUsersRemoval[];
  inserted: ProgramUsersInsertion[];
  moved: ProgramUsersRelocation[];
  modified: ProgramUsersPatchEntry[];
}

export interface ProgramUsersInsertion {
  index: number;
  row: UserProfile;
}

export interface ProgramUsersRemoval {
  id: string;
  index: number;
}

export interface ProgramUsersRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramUsersPatchEntry {
  id: string;
  patch: UserProfilePatch;
}

export interface ProgramActivitiesDelta {
  removed: ProgramActivitiesRemoval[];
  inserted: ProgramActivitiesInsertion[];
  moved: ProgramActivitiesRelocation[];
  modified: ProgramActivitiesPatchEntry[];
}

export interface ProgramActivitiesInsertion {
  index: number;
  row: Activity;
}

export interface ProgramActivitiesRemoval {
  id: string;
  index: number;
}

export interface ProgramActivitiesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramActivitiesPatchEntry {
  id: string;
  patch: ActivityPatch;
}

export interface ProgramFunctionsDelta {
  removed: ProgramFunctionsRemoval[];
  inserted: ProgramFunctionsInsertion[];
  moved: ProgramFunctionsRelocation[];
  modified: ProgramFunctionsPatchEntry[];
}

export interface ProgramFunctionsInsertion {
  index: number;
  row: Function;
}

export interface ProgramFunctionsRemoval {
  id: string;
  index: number;
}

export interface ProgramFunctionsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramFunctionsPatchEntry {
  id: string;
  patch: FunctionPatch;
}

export interface ProgramElementsDelta {
  removed: ProgramElementsRemoval[];
  inserted: ProgramElementsInsertion[];
  moved: ProgramElementsRelocation[];
  modified: ProgramElementsPatchEntry[];
}

export interface ProgramElementsInsertion {
  index: number;
  row: ProgramElement;
}

export interface ProgramElementsRemoval {
  id: string;
  index: number;
}

export interface ProgramElementsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramElementsPatchEntry {
  id: string;
  patch: ProgramElementPatch;
}

export interface ProgramQuantitiesDelta {
  removed: ProgramQuantitiesRemoval[];
  inserted: ProgramQuantitiesInsertion[];
  moved: ProgramQuantitiesRelocation[];
  modified: ProgramQuantitiesPatchEntry[];
}

export interface ProgramQuantitiesInsertion {
  index: number;
  row: QuantityRequirement;
}

export interface ProgramQuantitiesRemoval {
  id: string;
  index: number;
}

export interface ProgramQuantitiesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramQuantitiesPatchEntry {
  id: string;
  patch: QuantityRequirementPatch;
}

export interface ProgramRelationshipsDelta {
  removed: ProgramRelationshipsRemoval[];
  inserted: ProgramRelationshipsInsertion[];
  moved: ProgramRelationshipsRelocation[];
  modified: ProgramRelationshipsPatchEntry[];
}

export interface ProgramRelationshipsInsertion {
  index: number;
  row: Relationship;
}

export interface ProgramRelationshipsRemoval {
  id: string;
  index: number;
}

export interface ProgramRelationshipsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramRelationshipsPatchEntry {
  id: string;
  patch: RelationshipPatch;
}

export interface ProgramAdjacenciesDelta {
  removed: ProgramAdjacenciesRemoval[];
  inserted: ProgramAdjacenciesInsertion[];
  moved: ProgramAdjacenciesRelocation[];
  modified: ProgramAdjacenciesPatchEntry[];
}

export interface ProgramAdjacenciesInsertion {
  index: number;
  row: Adjacency;
}

export interface ProgramAdjacenciesRemoval {
  id: string;
  index: number;
}

export interface ProgramAdjacenciesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramAdjacenciesPatchEntry {
  id: string;
  patch: AdjacencyPatch;
}

export interface ProgramProcessesDelta {
  removed: ProgramProcessesRemoval[];
  inserted: ProgramProcessesInsertion[];
  moved: ProgramProcessesRelocation[];
  modified: ProgramProcessesPatchEntry[];
}

export interface ProgramProcessesInsertion {
  index: number;
  row: Process;
}

export interface ProgramProcessesRemoval {
  id: string;
  index: number;
}

export interface ProgramProcessesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramProcessesPatchEntry {
  id: string;
  patch: ProcessPatch;
}

export interface ProgramFlowsDelta {
  removed: ProgramFlowsRemoval[];
  inserted: ProgramFlowsInsertion[];
  moved: ProgramFlowsRelocation[];
  modified: ProgramFlowsPatchEntry[];
}

export interface ProgramFlowsInsertion {
  index: number;
  row: FlowRequirement;
}

export interface ProgramFlowsRemoval {
  id: string;
  index: number;
}

export interface ProgramFlowsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramFlowsPatchEntry {
  id: string;
  patch: FlowRequirementPatch;
}

export interface ProgramAccessRulesDelta {
  removed: ProgramAccessRulesRemoval[];
  inserted: ProgramAccessRulesInsertion[];
  moved: ProgramAccessRulesRelocation[];
  modified: ProgramAccessRulesPatchEntry[];
}

export interface ProgramAccessRulesInsertion {
  index: number;
  row: AccessRule;
}

export interface ProgramAccessRulesRemoval {
  id: string;
  index: number;
}

export interface ProgramAccessRulesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramAccessRulesPatchEntry {
  id: string;
  patch: AccessRulePatch;
}
