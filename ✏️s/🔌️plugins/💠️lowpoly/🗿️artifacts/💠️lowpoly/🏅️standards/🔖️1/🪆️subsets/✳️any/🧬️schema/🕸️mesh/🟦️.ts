/** 🕸️ Complete managed halfedge topology and rich surface channels remain typed owned fields. */
import type{Binary32} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import type{IntrinsicValue}from"../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🌳️intrinsic/🟦️.ts";
import{parseIntrinsicValue}from"../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🌳️intrinsic/🟦️.ts";
import {parseBinary32} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import{parseSchemaRecord}from"../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import{decodeIntrinsicJson}from"../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🌳️intrinsic/🔣️json/🟦️.ts";
export interface LowpolyMeshState{vertices:LowpolyMeshVertex[];halfedges:LowpolyMeshHalfedge[];faces:LowpolyMeshFace[];uvSeams:number[];attributes:LowpolyMeshAttribute[];materials:LowpolyMeshMaterial[];textures:LowpolyMeshTexture[]}
export interface LowpolyMeshVertex{position:[Binary32,Binary32,Binary32];normal:[Binary32,Binary32,Binary32]|null;halfedge:number|null}
export interface LowpolyMeshHalfedge{vertex:number;twin:number|null;next:number;face:number|null;uv:[Binary32,Binary32]}
export interface LowpolyMeshFace{halfedge:number;smooth:boolean;flipped:boolean}
export interface LowpolyMeshAttribute{name:string;domain:"vertex"|"corner"|"face"|"edge";semantic:"normal"|"uv"|"color"|"material"|"custom";interpolation:"linear"|"nearest"|"constant";values:IntrinsicValue[];indices:number[]|null}
export interface LowpolyMeshMaterial{name:string;value:IntrinsicValue}
export interface LowpolyMeshTexture{name:string;mime:string;bytes:Uint8Array}
function array(value:unknown):unknown[]{if(!Array.isArray(value))throw Error("Lowpoly mesh requires an ordered collection");return value;}
function text(value:unknown):string{if(typeof value!=="string")throw Error("Lowpoly mesh requires literal text");return value;}
function flag(value:unknown):boolean{if(typeof value!=="boolean")throw Error("Lowpoly mesh requires boolean");return value;}
function index(value:unknown):number{if(typeof value!=="number"||!Number.isInteger(value)||value<0||value>0xffffffff)throw Error("Lowpoly mesh index exceeds UInt32");return value;}
function optionalIndex(value:unknown):number|null{return value===null?null:index(value);}
function vector(value:unknown,size:number):Binary32[]{const words=array(value);if(words.length!==size)throw Error("Lowpoly mesh vector width");return words.map(parseBinary32);}
function member<T extends string>(value:unknown,allowed:readonly T[]):T{if(!allowed.includes(value as T))throw Error("Lowpoly mesh channel enum");return value as T;}
function named<T extends{name:string}>(values:T[]):T[]{const names=new Set<string>();for(const value of values){if(names.has(value.name))throw Error("Lowpoly mesh named ownership must be unique");names.add(value.name);}return values;}
/** 🛂️ Preserve complete editable topology without inferring a mesh from source text. */
export function parseLowpolyMeshState(value:unknown):LowpolyMeshState{
 const root=parseSchemaRecord(value,["vertices","halfedges","faces","uvSeams","attributes","materials","textures"]);
 return{
  vertices:array(root.vertices).map(value=>{const row=parseSchemaRecord(value,["position","normal","halfedge"]);return{position:vector(row.position,3)as[Binary32,Binary32,Binary32],normal:row.normal===null?null:vector(row.normal,3)as[Binary32,Binary32,Binary32],halfedge:optionalIndex(row.halfedge)}}),
  halfedges:array(root.halfedges).map(value=>{const row=parseSchemaRecord(value,["vertex","twin","next","face","uv"]);return{vertex:index(row.vertex),twin:optionalIndex(row.twin),next:index(row.next),face:optionalIndex(row.face),uv:vector(row.uv,2)as[Binary32,Binary32]}}),
  faces:array(root.faces).map(value=>{const row=parseSchemaRecord(value,["halfedge","smooth","flipped"]);return{halfedge:index(row.halfedge),smooth:flag(row.smooth),flipped:flag(row.flipped)}}),
  uvSeams:(()=>{const seams=array(root.uvSeams).map(index);if(new Set(seams).size!==seams.length)throw Error("Lowpoly mesh seam identity must be unique");return seams;})(),
  attributes:named(array(root.attributes).map(value=>{const row=parseSchemaRecord(value,["name","domain","semantic","interpolation","values","indices"]);return{name:text(row.name),domain:member(row.domain,["vertex","corner","face","edge"]),semantic:member(row.semantic,["normal","uv","color","material","custom"]),interpolation:member(row.interpolation,["linear","nearest","constant"]),values:array(row.values).map(parseIntrinsicValue),indices:row.indices===null?null:array(row.indices).map(index)}})),
  materials:named(array(root.materials).map(value=>{const row=parseSchemaRecord(value,["name","value"]);return{name:text(row.name),value:parseIntrinsicValue(row.value)}})),
  textures:named(array(root.textures).map(value=>{const row=parseSchemaRecord(value,["name","mime","bytes"]);if(!(row.bytes instanceof Uint8Array))throw Error("Lowpoly texture requires owned octets");return{name:text(row.name),mime:text(row.mime),bytes:row.bytes.slice()}})),
 };
}

function jsonWord(source:unknown):Binary32{if(typeof source!=="string"||!/^[0-9a-f]{8}$/.test(source))throw Error("Lowpoly mesh JSON requires an exact binary32 word");return{bits:parseInt(source,16)};}
function jsonVector(source:unknown,width:number):Binary32[]{const values=array(source);if(values.length!==width)throw Error("Lowpoly mesh JSON vector width");return values.map(jsonWord);}
/** 🔣️ Decode complete managed mesh JSON through the canonical lossless intrinsic facet. */
export function decodeLowpolyMeshJson(source:unknown):LowpolyMeshState{
 const root=parseSchemaRecord(source,["vertices","halfedges","faces","uvSeams","attributes","materials","textures"]);
 return parseLowpolyMeshState({
  vertices:array(root.vertices).map(value=>{const row=parseSchemaRecord(value,["position","normal","halfedge"]);return{...row,position:jsonVector(row.position,3),normal:row.normal===null?null:jsonVector(row.normal,3)}}),
  halfedges:array(root.halfedges).map(value=>{const row=parseSchemaRecord(value,["vertex","twin","next","face","uv"]);return{...row,uv:jsonVector(row.uv,2)}}),
  faces:root.faces,uvSeams:root.uvSeams,
  attributes:array(root.attributes).map(value=>{const row=parseSchemaRecord(value,["name","domain","semantic","interpolation","values","indices"]);return{...row,values:array(row.values).map(decodeIntrinsicJson)}}),
  materials:array(root.materials).map(value=>{const row=parseSchemaRecord(value,["name","value"]);return{...row,value:decodeIntrinsicJson(row.value)}}),
  textures:array(root.textures).map(value=>{const row=parseSchemaRecord(value,["name","mime","bytes"]),bytes=array(row.bytes);if(bytes.some(value=>typeof value!=="number"||!Number.isInteger(value)||value<0||value>255))throw Error("Lowpoly mesh JSON octet range");return{...row,bytes:Uint8Array.from(bytes as number[])}}),
 });
}
