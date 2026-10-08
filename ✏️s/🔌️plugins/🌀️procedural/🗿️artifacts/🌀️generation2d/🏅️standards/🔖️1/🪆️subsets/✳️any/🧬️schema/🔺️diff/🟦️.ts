/** 🔺️ Sparse procedural field delta using the same owned primitive model. */
import type{CameraJson,WidgetLayout,SynapseSpec,Widget,FormGeneration}from"../🟦️.ts";
export type{Generation2dArtifact,CameraJson,WidgetLayout,SynapseSpec,Widget,FlowTree,FlowNeuron,FlowUi,FlowNodeGui,NodeChrome,FlowPreviewGui,FlowChannelRef,FlowHostSnapshot,FormGeneration,GenerationPlayState}from"../🟦️.ts";
export interface Generation2dDiff{schema?:string;camera?:CameraJson;widgets?:Generation2dWidgetsDelta;synapses?:Generation2dSynapsesDelta;layout?:Generation2dLayoutDelta;generations?:Generation2dGenerationsDelta;selectedGeneration?:Generation2dSelectionChange;previewText?:Generation2dPreviewChange}
export interface Generation2dWidgetRemoval{id:string;index:number}
export interface Generation2dWidgetInsertion{index:number;row:Widget}
export interface Generation2dWidgetRelocation{id:string;from:number;to:number}
export interface Generation2dWidgetsDelta{removed:Generation2dWidgetRemoval[];inserted:Generation2dWidgetInsertion[];moved:Generation2dWidgetRelocation[];patched:Generation2dWidgetModification[]}
export interface Generation2dWidgetModification{id:string;patch:Generation2dWidgetPatch}
export type Generation2dWidgetPatch={kind:"replace";widget:Widget}|{kind:"slider";value:number;min:number;max:number;step:number};
export interface Generation2dSynapseRemoval{id:string;index:number}
export interface Generation2dSynapseInsertion{index:number;row:SynapseSpec}
export interface Generation2dSynapseRelocation{id:string;from:number;to:number}
export interface Generation2dSynapsesDelta{removed:Generation2dSynapseRemoval[];inserted:Generation2dSynapseInsertion[];moved:Generation2dSynapseRelocation[];patched:Generation2dSynapseModification[]}
export interface Generation2dSynapseModification{id:string;patch:SynapseSpec}
export interface Generation2dLayoutRow{id:string;layout:WidgetLayout}
export interface Generation2dLayoutDelta{added:Generation2dLayoutRow[];removed:string[];patched:Generation2dLayoutRow[]}
export interface Generation2dGenerationRemoval{id:string;index:number}
export interface Generation2dGenerationInsertion{index:number;row:FormGeneration}
export interface Generation2dGenerationRelocation{id:string;from:number;to:number}
export interface Generation2dGenerationsDelta{removed:Generation2dGenerationRemoval[];inserted:Generation2dGenerationInsertion[];moved:Generation2dGenerationRelocation[];modified:Generation2dGenerationModification[]}
export interface Generation2dGenerationModification{id:string;patch:Generation2dGenerationPatch}
export interface Generation2dGenerationPatch{name?:string;values?:Generation2dValuesDelta}
export interface Generation2dValueRow{questionId:string;value:unknown}
export interface Generation2dValuesDelta{added:Generation2dValueRow[];removed:string[];patched:Generation2dValueRow[]}
export interface Generation2dSelectionChange{id?:string}
export interface Generation2dPreviewChange{text?:string}
export type Generation2dStringList={values:readonly string[]};
