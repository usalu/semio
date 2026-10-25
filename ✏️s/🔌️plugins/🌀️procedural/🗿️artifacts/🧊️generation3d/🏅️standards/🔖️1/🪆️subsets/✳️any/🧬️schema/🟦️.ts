/** 🌀️ Owned Generation3d fields and native primitive guards. */

export type {ProceduralSnapshot,CameraJson,WidgetLayout,SynapseSpec,Widget,FlowTree,FlowNeuron,FlowUi,FlowNodeGui,NodeChrome,FlowPreviewGui,FlowChannelRef,FlowHostSnapshot,FormGeneration,GenerationPlayState,NeuralValue,NeuralDictionary,GenerationValue} from "@semio-tech/procedural-js/foundation/generation/schema/snapshot";

import type {FlowHostSnapshot as ArtifactFlowHostSnapshot,GenerationPlayState as ArtifactGenerationPlayState} from "@semio-tech/procedural-js/foundation/generation/schema/snapshot";

/** 🧬️ Generation3d document: the artifact state of its host snapshot and play state. */
export interface Generation3dArtifact{readonly hostSnapshot:ArtifactFlowHostSnapshot;readonly generation:ArtifactGenerationPlayState}

export {binary64,binary64Value,parseBinary64,parseProceduralSnapshot,parseCameraJson,parseWidgetLayout,parseSynapseSpec,parseWidget,parseFlowHostSnapshot,parseFormGeneration,parseGenerationPlayState} from "@semio-tech/procedural-js/foundation/generation/schema/snapshot/admission";
export interface Generation3dStringList{readonly values:readonly string[]}

export type Generation3dPreviewCamera={positionX:number;positionY:number;positionZ:number;targetX:number;targetY:number;targetZ:number;fov:number};
