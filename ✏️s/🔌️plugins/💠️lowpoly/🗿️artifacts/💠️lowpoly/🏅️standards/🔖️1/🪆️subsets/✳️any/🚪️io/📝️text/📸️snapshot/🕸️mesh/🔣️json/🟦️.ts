/** 🔣️ Physical member JSON representation. */
import type{Binary32} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import type{IntrinsicValue}from"../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🌳️intrinsic/🟦️.ts";
import{parseSchemaRecord}from"../../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import{decodeIntrinsicJson}from"../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🌳️intrinsic/🔣️json/🟦️.ts";
import {parseLowpolyMeshState,type LowpolyMeshState} from "../../../../../🧬️schema/🕸️mesh/🟦️.ts";
function array(value:unknown):unknown[]{if(!Array.isArray(value))throw Error("Lowpoly mesh requires ordered arrays");return value;}
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
