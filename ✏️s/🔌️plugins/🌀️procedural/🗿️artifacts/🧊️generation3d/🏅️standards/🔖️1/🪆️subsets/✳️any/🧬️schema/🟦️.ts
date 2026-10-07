/** 🌀️ Owned Generation3d fields and complete typed primitive admission. */

export {ProceduralSnapshot as Generation3dArtifact,CameraJson,WidgetLayout,SynapseSpec,Widget,FlowTree,FlowNeuron,FlowUi,FlowNodeGui,NodeChrome,FlowPreviewGui,FlowChannelRef,FlowHostSnapshot,FormGeneration,GenerationPlayState,NeuralValue,NeuralDictionary,GenerationValue} from "../../../../../../../🫀️core/🧬️generation/🧬️schema/📸️snapshot/🟦️.ts";

export {binary64,binary64Value,parseBinary64,parseProceduralSnapshot as parseGeneration3dArtifact,parseCameraJson,parseWidgetLayout,parseSynapseSpec,parseWidget,parseFlowHostSnapshot,parseFormGeneration,parseGenerationPlayState} from "../../../../../../../🫀️core/🧬️generation/🧬️schema/📸️snapshot/🛡️admission/🟦️.ts";
export interface Generation3dStringList{readonly values:readonly string[]}
/** 🔤️ Admit the actual string-list field. */
export function parseGeneration3dStringList(v:unknown):Generation3dStringList{if(v===null||typeof v!=="object"||!("values"in v)||!Array.isArray(v.values)||v.values.some(x=>typeof x!=="string"))throw Error("Generation3d string list differs");return{values:v.values};}

export type Generation3dPreviewCamera={positionX:number;positionY:number;positionZ:number;targetX:number;targetY:number;targetZ:number;fov:number};
/** 📷️ Admit the separate nonpersisted preview camera settings. */
export function parseGeneration3dPreviewCamera(v:unknown):Generation3dPreviewCamera{if(v===null||typeof v!=="object")throw Error("Generation3d preview camera differs");const r=v as Record<string,unknown>;const n=(x:unknown):number=>{if(typeof x!=="number"||!Number.isFinite(x))throw Error("Generation3d preview coordinate differs");return x;};return{positionX:n(r.positionX),positionY:n(r.positionY),positionZ:n(r.positionZ),targetX:n(r.targetX),targetY:n(r.targetY),targetZ:n(r.targetZ),fov:n(r.fov)};}
