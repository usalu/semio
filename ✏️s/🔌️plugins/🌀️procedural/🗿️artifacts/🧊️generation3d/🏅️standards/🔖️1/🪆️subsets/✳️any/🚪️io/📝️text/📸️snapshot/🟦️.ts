/** 📝️ Owned Generation3d native text representation. */
export type Generation3dSnapshotText=string;
/** 🔤️ Admit the actual text primitive without coercion. */
export function parseGeneration3dSnapshotText(v:unknown):Generation3dSnapshotText{if(typeof v!=="string")throw Error("Generation3d snapshot text differs");return v;}

import type { Generation3dStringList, Generation3dPreviewCamera } from "../../../🧬️schema/🟦️.ts";

/** 🔤️ Admit the actual string-list field. */
export function parseGeneration3dStringList(v:unknown):Generation3dStringList{if(v===null||typeof v!=="object"||!("values"in v)||!Array.isArray(v.values)||v.values.some(x=>typeof x!=="string"))throw Error("Generation3d string list differs");return{values:v.values};}

/** 📷️ Admit the separate nonpersisted preview camera settings. */
export function parseGeneration3dPreviewCamera(v:unknown):Generation3dPreviewCamera{if(v===null||typeof v!=="object")throw Error("Generation3d preview camera differs");const r=v as Record<string,unknown>;const n=(x:unknown):number=>{if(typeof x!=="number"||!Number.isFinite(x))throw Error("Generation3d preview coordinate differs");return x;};return{positionX:n(r.positionX),positionY:n(r.positionY),positionZ:n(r.positionZ),targetX:n(r.targetX),targetY:n(r.targetY),targetZ:n(r.targetZ),fov:n(r.fov)};}
