/** 🔺️ Sparse procedural field delta using the same owned primitive model. */
import type{CameraJson,WidgetLayout,SynapseSpec,Widget,FormGeneration}from"../🟦️.ts";
export type{Generation3dArtifact,CameraJson,WidgetLayout,SynapseSpec,Widget,FlowTree,FlowNeuron,FlowUi,FlowNodeGui,NodeChrome,FlowPreviewGui,FlowChannelRef,FlowHostSnapshot,FormGeneration,GenerationPlayState}from"../🟦️.ts";
export interface Generation3dDiff{schema?:string;camera?:CameraJson;widgets?:Generation3dWidgetsDelta;synapses?:Generation3dSynapsesDelta;layout?:Generation3dLayoutDelta;generations?:Generation3dGenerationsDelta;selectedGeneration?:Generation3dSelectionChange;previewText?:Generation3dPreviewChange}
export interface Generation3dWidgetsDelta{added:Widget[];removed:string[];patched:Generation3dWidgetPatchEntry[];reordered?:string[]}
export interface Generation3dWidgetPatchEntry{id:string;patch:Generation3dWidgetPatch}
export type Generation3dWidgetPatch={kind:"replace";widget:Widget}|{kind:"slider";value:number;min:number;max:number;step:number};
export interface Generation3dSynapsesDelta{added:SynapseSpec[];removed:string[];patched:Generation3dSynapsePatchEntry[];reordered?:string[]}
export interface Generation3dSynapsePatchEntry{id:string;item:SynapseSpec}
export interface Generation3dLayoutRow{id:string;layout:WidgetLayout}
export interface Generation3dLayoutDelta{added:Generation3dLayoutRow[];removed:string[];patched:Generation3dLayoutRow[]}
export interface Generation3dGenerationsDelta{added:FormGeneration[];removed:string[];patched:Generation3dGenerationPatchEntry[];reordered?:string[]}
export interface Generation3dGenerationPatchEntry{id:string;patch:Generation3dGenerationPatch}
export interface Generation3dGenerationPatch{name?:string;values?:Generation3dValuesDelta}
export interface Generation3dValueRow{questionId:string;value:unknown}
export interface Generation3dValuesDelta{added:Generation3dValueRow[];removed:string[];patched:Generation3dValueRow[]}
export interface Generation3dSelectionChange{id?:string}
export interface Generation3dPreviewChange{text?:string}
export type Generation3dStringList={values:readonly string[]};
