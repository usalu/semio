/** 🔺️ Sparse procedural field delta using the same owned primitive model. */
import type{CameraJson,WidgetLayout,SynapseSpec,Widget,FormGeneration}from"../🟦️.ts";
export type{Generation2dArtifact,CameraJson,WidgetLayout,SynapseSpec,Widget,FlowTree,FlowNeuron,FlowUi,FlowNodeGui,NodeChrome,FlowPreviewGui,FlowChannelRef,FlowHostSnapshot,FormGeneration,GenerationPlayState}from"../🟦️.ts";
export interface Generation2dDiff{schema?:string;camera?:CameraJson;widgets?:Generation2dWidgetsDelta;synapses?:Generation2dSynapsesDelta;layout?:Generation2dLayoutDelta;generations?:Generation2dGenerationsDelta;selectedGeneration?:Generation2dSelectionChange;previewText?:Generation2dPreviewChange}
export interface Generation2dWidgetsDelta{added:Widget[];removed:string[];patched:Generation2dWidgetPatchEntry[];reordered?:string[]}
export interface Generation2dWidgetPatchEntry{id:string;patch:Generation2dWidgetPatch}
export type Generation2dWidgetPatch={kind:"replace";widget:Widget}|{kind:"slider";value:number;min:number;max:number;step:number};
export interface Generation2dSynapsesDelta{added:SynapseSpec[];removed:string[];patched:Generation2dSynapsePatchEntry[];reordered?:string[]}
export interface Generation2dSynapsePatchEntry{id:string;item:SynapseSpec}
export interface Generation2dLayoutRow{id:string;layout:WidgetLayout}
export interface Generation2dLayoutDelta{added:Generation2dLayoutRow[];removed:string[];patched:Generation2dLayoutRow[]}
export interface Generation2dGenerationsDelta{added:FormGeneration[];removed:string[];patched:Generation2dGenerationPatchEntry[];reordered?:string[]}
export interface Generation2dGenerationPatchEntry{id:string;patch:Generation2dGenerationPatch}
export interface Generation2dGenerationPatch{name?:string;values?:Generation2dValuesDelta}
export interface Generation2dValueRow{questionId:string;value:unknown}
export interface Generation2dValuesDelta{added:Generation2dValueRow[];removed:string[];patched:Generation2dValueRow[]}
export interface Generation2dSelectionChange{id?:string}
export interface Generation2dPreviewChange{text?:string}
export type Generation2dStringList={values:readonly string[]};
