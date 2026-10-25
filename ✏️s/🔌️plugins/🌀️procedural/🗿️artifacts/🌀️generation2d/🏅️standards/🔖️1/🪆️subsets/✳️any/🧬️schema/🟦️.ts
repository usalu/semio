/** 🌀️ Owned Generation2d fields and complete typed primitive admission. */

export type {ProceduralSnapshot,CameraJson,WidgetLayout,SynapseSpec,Widget,FlowTree,FlowNeuron,FlowUi,FlowNodeGui,NodeChrome,FlowPreviewGui,FlowChannelRef,FlowHostSnapshot,FormGeneration,GenerationPlayState,NeuralValue,NeuralDictionary,GenerationValue} from "@semio-tech/procedural-js/foundation/generation/schema/snapshot";

import type {FlowHostSnapshot as ArtifactFlowHostSnapshot,GenerationPlayState as ArtifactGenerationPlayState} from "@semio-tech/procedural-js/foundation/generation/schema/snapshot";

/** 🧬️ Generation2d document: the artifact state of its host snapshot and play state. */
export interface Generation2dArtifact{readonly hostSnapshot:ArtifactFlowHostSnapshot;readonly generation:ArtifactGenerationPlayState}

export {binary64,binary64Value,parseBinary64,parseProceduralSnapshot,parseCameraJson,parseWidgetLayout,parseSynapseSpec,parseWidget,parseFlowHostSnapshot,parseFormGeneration,parseGenerationPlayState} from "@semio-tech/procedural-js/foundation/generation/schema/snapshot/admission";
export interface Generation2dStringList{readonly values:readonly string[]}
/** 🔤️ Admit the actual string-list field. */
export function parseGeneration2dStringList(v:unknown):Generation2dStringList{if(v===null||typeof v!=="object"||!("values"in v)||!Array.isArray(v.values)||v.values.some(x=>typeof x!=="string"))throw Error("Generation2d string list differs");return{values:v.values};}
