/** 🔺️ Sparse procedural field delta using the same owned primitive model. */
import type{CameraJson,WidgetLayout,SynapseSpec,Widget,FormGeneration}from"../🟦️.ts";
export type{Generation3dArtifact,CameraJson,WidgetLayout,SynapseSpec,Widget,FlowTree,FlowNeuron,FlowUi,FlowNodeGui,NodeChrome,FlowPreviewGui,FlowChannelRef,FlowHostSnapshot,FormGeneration,GenerationPlayState}from"../🟦️.ts";
export interface Generation3dDiff{schema?:string;camera?:CameraJson;widgets?:Generation3dWidgetsDelta;synapses?:Generation3dSynapsesDelta;layout?:Generation3dLayoutDelta;generations?:Generation3dGenerationsDelta;selectedGeneration?:Generation3dSelectionChange;previewText?:Generation3dPreviewChange}
export interface Generation3dWidgetRemoval{id:string;index:number}
export interface Generation3dWidgetInsertion{index:number;row:Widget}
export interface Generation3dWidgetRelocation{id:string;from:number;to:number}
export interface Generation3dWidgetsDelta{removed:Generation3dWidgetRemoval[];inserted:Generation3dWidgetInsertion[];moved:Generation3dWidgetRelocation[];patched:Generation3dWidgetModification[]}
export interface Generation3dWidgetModification{id:string;patch:Generation3dWidgetPatch}
export type Generation3dWidgetPatch={kind:"replace";widget:Widget}|{kind:"slider";value:number;min:number;max:number;step:number};
export interface Generation3dSynapseRemoval{id:string;index:number}
export interface Generation3dSynapseInsertion{index:number;row:SynapseSpec}
export interface Generation3dSynapseRelocation{id:string;from:number;to:number}
export interface Generation3dSynapsesDelta{removed:Generation3dSynapseRemoval[];inserted:Generation3dSynapseInsertion[];moved:Generation3dSynapseRelocation[];patched:Generation3dSynapseModification[]}
export interface Generation3dSynapseModification{id:string;patch:SynapseSpec}
export interface Generation3dLayoutRow{id:string;layout:WidgetLayout}
export interface Generation3dLayoutDelta{added:Generation3dLayoutRow[];removed:string[];patched:Generation3dLayoutRow[]}
export interface Generation3dGenerationRemoval{id:string;index:number}
export interface Generation3dGenerationInsertion{index:number;row:FormGeneration}
export interface Generation3dGenerationRelocation{id:string;from:number;to:number}
export interface Generation3dGenerationsDelta{removed:Generation3dGenerationRemoval[];inserted:Generation3dGenerationInsertion[];moved:Generation3dGenerationRelocation[];modified:Generation3dGenerationModification[]}
export interface Generation3dGenerationModification{id:string;patch:Generation3dGenerationPatch}
export interface Generation3dGenerationPatch{name?:string;values?:Generation3dValuesDelta}
export interface Generation3dValueRow{questionId:string;value:unknown}
export interface Generation3dValuesDelta{added:Generation3dValueRow[];removed:string[];patched:Generation3dValueRow[]}
export interface Generation3dSelectionChange{id?:string}
export interface Generation3dPreviewChange{text?:string}
export type Generation3dStringList={values:readonly string[]};
