import {readFileSync} from "node:fs";
import Ajv from "ajv";
const editor="✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor";
const read=(path:string)=>JSON.parse(readFileSync(path,"utf8"));
const bake=read(editor+"/🎮️commands/🥞️flatten-layers/🧪️tests/execution.json");
const mask=read(editor+"/🎮️commands/🎭️mask-from-selection/🧪️tests/🧫️fixtures/🔣️.json").execution;
const ajv=new Ajv({strict:false});
let schemas=0,vectors=0;
for(const path of ["🥞️flatten-layers","🫳️merge-down"]){const schema=read(editor+"/🎮️commands/"+path+"/🧬️schema/🔣️.json");ajv.compile(schema);for(const key of ["maximumRawBytes","maximumWorkUnitsPerStep","maximumOutputBytes","maximumSelectedLayerNodes","maximumLayerNodes"]){if(schema["x-semio-execution"][key]!==bake[key])throw Error(path+":"+key);}schemas++;}
const bytes=(value:unknown)=>new TextEncoder().encode(JSON.stringify(value)).length;
const fit=(value:unknown,maximum:number)=>{if(!ajv.compile({type:"number",maximum})(bytes(value)))throw Error("Encoded envelope exceeded");vectors++;};
for(const character of [...Array.from({length:32},(_,i)=>String.fromCodePoint(i)),'"',"\\","é","🖨"]){const width=new TextEncoder().encode(character).length;const field=character.repeat(Math.floor(bake.maximumIdentifierUtf8Bytes/width));fit(field,bake.maximumIdentifierUtf8Bytes*6+2);fit({deleteLayer:{layerId:field}},bake.maximumIdentifierUtf8Bytes*6+bake.maximumLayerRowSyntaxBytes);fit({removeLayerAsset:{assetId:field}},bake.maximumIdentifierUtf8Bytes*6+bake.maximumAssetRowSyntaxBytes);fit({layerId:field,expectedImageKey:field,windowId:field},mask.maximumRawBytes);}
for(const sample of [0,128,255])fit(Array(4096).fill(sample),4096*bake.maximumEncodedBytesPerSample+2);
console.log(JSON.stringify({oracle:"ECMAScript JSON encoding with independent Ajv envelope/schema validation",schemas,vectors,matched:true}));
