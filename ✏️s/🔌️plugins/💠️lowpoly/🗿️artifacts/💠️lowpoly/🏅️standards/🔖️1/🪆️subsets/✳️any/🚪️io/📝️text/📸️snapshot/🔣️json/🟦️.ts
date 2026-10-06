/** 🔣️ Declared Lowpoly JSON wire fields lower into exact canonical scalars and intrinsic octets. */
import{parseLowpolyArtifact,lowpolyLowpolyArtifactGuardObject as object,lowpolyLowpolyArtifactGuardArray as array,lowpolyLowpolyArtifactGuardNumber as number,lowpolyLowpolyArtifactGuardString as text}from"../../../../🧬️schema/🟦️.ts";
import type{LowpolySnapshot}from"../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {binary32} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import{base64StandardDecode}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🔤️base64/🟦️.ts";
import{decodeLowpolyMeshJson}from"../../../../🧬️schema/🕸️mesh/🟦️.ts";
function vector(value:unknown):ReturnType<typeof binary32>[] {return array(value,"vector",{minItems:3,maxItems:3}).map(value=>binary32(number(value,"component")));}
/** 📥️ Decode the actual numeric/base64 JSON transport without widening the canonical schema. */
export function decodeLowpolyJsonSnapshot(value:unknown):LowpolySnapshot{
 const document=object(value,"document");return parseLowpolyArtifact({schema:document.schema,objects:array(document.objects,"objects").map(value=>{const row=object(value,"object"),transform=object(row.transform,"transform");return{id:row.id,name:row.name,smoothShading:row.smoothShading,mesh:row.mesh,meshContent:row.meshContent,meshState:row.meshState===null?null:decodeLowpolyMeshJson(row.meshState),transform:{position:vector(transform.position),rotation:vector(transform.rotation),scale:vector(transform.scale)},paintLayers:array(row.paintLayers,"paintLayers").map(value=>{const layer=object(value,"layer");return{name:layer.name,visible:layer.visible,blendMode:layer.blendMode,opacity:binary32(number(layer.opacity,"opacity")),pixels:base64StandardDecode(text(layer.pixels,"pixels"))};})};})});
}
