/** 🌀️ Owned Generation2d fields and complete typed primitive admission. */

export type {ProceduralSnapshot as Generation2dArtifact,CameraJson,WidgetLayout,SynapseSpec,Widget,FlowTree,FlowNeuron,FlowUi,FlowNodeGui,NodeChrome,FlowPreviewGui,FlowChannelRef,FlowHostSnapshot,FormGeneration,GenerationPlayState,NeuralValue,NeuralDictionary,GenerationValue} from "../../../../../../../🫀️core/🧬️generation/🧬️schema/📸️snapshot/🟦️.ts";

export {binary64,binary64Value,parseBinary64,parseProceduralSnapshot as parseGeneration2dArtifact,parseCameraJson,parseWidgetLayout,parseSynapseSpec,parseWidget,parseFlowHostSnapshot,parseFormGeneration,parseGenerationPlayState} from "../../../../../../../🫀️core/🧬️generation/🧬️schema/📸️snapshot/🛡️admission/🟦️.ts";
export interface Generation2dStringList{readonly values:readonly string[]}
/** 🔤️ Admit the actual string-list field. */
export function parseGeneration2dStringList(v:unknown):Generation2dStringList{if(v===null||typeof v!=="object"||!("values"in v)||!Array.isArray(v.values)||v.values.some(x=>typeof x!=="string"))throw Error("Generation2d string list differs");return{values:v.values};}
