/** 🏭️ Artifact state uses the exact canonical Process3d Snapshot fields. */
import{parseProcess3dSnapshot,type Process3dSnapshot}from"./📸️snapshot/🟦️.ts";
export*from"./📸️snapshot/🟦️.ts";
export interface Process3dArtifact extends Process3dSnapshot{}
export function parseProcess3dArtifact(value:unknown):Process3dArtifact{return parseProcess3dSnapshot(value)}
