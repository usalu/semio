/** 🧩️ Program mutation payloads use the same canonical typed register entities. */
export * from "../🟦️.ts";
export * from "../🪪️entity-foundation/🟦️.ts";
export * from "../🏗️program-structure/🟦️.ts";
export * from "../📋️requirements/🟦️.ts";
export * from "../🗃️decision-records/🟦️.ts";
import type { ProgramMeta, ProjectDefinition, TraceLink } from "../🪪️entity-foundation/🟦️.ts";
import type { Adjacency } from "../🏗️program-structure/🟦️.ts";
import type { Governance } from "../🗃️decision-records/🟦️.ts";
import type { RegisterKey, RegisterRow } from "../🗄️registers/🗂️catalog/🟦️.ts";

export interface CreateRegisterRow { row: RegisterRow; index?: number; }
export interface CreateRegisterRowOp extends CreateRegisterRow { mutation: "createRegisterRow"; }
export interface DeleteRegisterRow { register: RegisterKey; id: string; }
export interface DeleteRegisterRowOp extends DeleteRegisterRow { mutation: "deleteRegisterRow"; }
export interface RenameRegisterRow { register: RegisterKey; id: string; newName: string; }
export interface RenameRegisterRowOp extends RenameRegisterRow { mutation: "renameRegisterRow"; }
export interface ReplaceRegisterRow { row: RegisterRow; }
export interface ReplaceRegisterRowOp extends ReplaceRegisterRow { mutation: "replaceRegisterRow"; }
export interface ConnectAdjacency { adjacency: Adjacency; index?: number; }
export interface ConnectAdjacencyOp extends ConnectAdjacency { mutation: "connectAdjacency"; }
export interface DisconnectAdjacency { id: string; }
export interface DisconnectAdjacencyOp extends DisconnectAdjacency { mutation: "disconnectAdjacency"; }
export interface ConnectTrace { trace: TraceLink; index?: number; }
export interface ConnectTraceOp extends ConnectTrace { mutation: "connectTrace"; }
export interface DisconnectTrace { id: string; }
export interface DisconnectTraceOp extends DisconnectTrace { mutation: "disconnectTrace"; }
export interface RenameMeta { newTitle: string; }
export interface RenameMetaOp extends RenameMeta { mutation: "renameMeta"; }
export interface ReplaceMeta { newMeta: ProgramMeta; }
export interface ReplaceMetaOp extends ReplaceMeta { mutation: "replaceMeta"; }
export interface RenameProject { newCode: string; }
export interface RenameProjectOp extends RenameProject { mutation: "renameProject"; }
export interface ReplaceProject { newProject: ProjectDefinition; }
export interface ReplaceProjectOp extends ReplaceProject { mutation: "replaceProject"; }
export interface RenameGovernance { newFramework: string; }
export interface RenameGovernanceOp extends RenameGovernance { mutation: "renameGovernance"; }
export interface ReplaceGovernance { newGovernance: Governance; }
export interface ReplaceGovernanceOp extends ReplaceGovernance { mutation: "replaceGovernance"; }

export type ProgramMutation =
   | CreateRegisterRowOp | DeleteRegisterRowOp | RenameRegisterRowOp | ReplaceRegisterRowOp | ConnectAdjacencyOp | DisconnectAdjacencyOp | ConnectTraceOp | DisconnectTraceOp | RenameMetaOp | ReplaceMetaOp | RenameProjectOp | ReplaceProjectOp | RenameGovernanceOp | ReplaceGovernanceOp;
