import type{LayoutSnapshot,LayoutDrawingBirthSource}from'../../../🧬️schema/📸️snapshot/🟦️.ts';
import type{ImportVectorScene}from'../🪆️drawing/🟦️.ts';
export type LayoutExternalDrawingFormat='svg'|'dxf'|'dwg';
export interface PendingDrawingSource{source:LayoutDrawingBirthSource;scene:ImportVectorScene}
export type LayoutImportSourcePlan={kind:'document';document:LayoutSnapshot;drawingSources:PendingDrawingSource[]}|{kind:'external';format:LayoutExternalDrawingFormat;source:Uint8Array;name:string};
/** 🧭️ Cold analysis keeps original external bytes distinct from persistent parent/member publication. */
export function externalDrawingSourcePlan(format:string,bytes:Uint8Array,name:string,control:(completed:number,total:number)=>boolean):LayoutImportSourcePlan{if(format!=='svg'&&format!=='dxf'&&format!=='dwg')throw Error('layout.import.drawing-format');if(bytes.byteLength>8<<20)throw Error('layout.interchange.byte-limit');if(!control(0,bytes.byteLength))throw Error('layout.interchange.cancelled');const source=new Uint8Array(bytes.byteLength);for(let at=0;at<bytes.byteLength;at+=4096){if(!control(at,bytes.byteLength))throw Error('layout.interchange.cancelled');source.set(bytes.subarray(at,Math.min(at+4096,bytes.byteLength)),at)}if(!control(bytes.byteLength,bytes.byteLength))throw Error('layout.interchange.cancelled');return{kind:'external',format,source,name}}
/** 📦️ A draft and its actual pending Drawing sources transfer without constructing placeholder references. */
export function documentSourcePlan(document:LayoutSnapshot,drawingSources:PendingDrawingSource[]):LayoutImportSourcePlan{return{kind:'document',document,drawingSources}}
