/** 📍️ GIS feature data preserves the full intrinsic native Value domain. */
import {parseSchemaRecord} from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import {parseIntrinsicValue,type IntrinsicValue} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🌳️intrinsic/🟦️.ts";
export type GisMapValue=IntrinsicValue;
export interface GisMapFeature{id:string;data:GisMapValue}
export interface GisMapFeaturePropertyValue{value:GisMapValue}
export interface GisMapFeaturePropertyEdit{key:string;before?:string;set?:GisMapFeaturePropertyValue}
export interface GisMapFeaturePatch{data:GisMapValue|null;properties:GisMapFeaturePropertyEdit[]}
/** 🧬️ GIS feature data owns the canonical full intrinsic primitive domain. */
export const parseGisMapValue=parseIntrinsicValue;
/** 📍️ Bind a feature without imposing spatial or identifier restrictions. */
export function parseGisMapFeature(value:unknown,at="$"):GisMapFeature{const row=parseSchemaRecord(value,["id","data"],at);if(typeof row.id!=="string"||/[\uD800-\uDBFF](?![\uDC00-\uDFFF])|(?<![\uD800-\uDBFF])[\uDC00-\uDFFF]/.test(row.id as string))throw Error(`${at}.id: string required`);return{id:row.id,data:parseGisMapValue(row.data)}}
/** 🩹️ Preserve unchanged absence independently from an explicit native Null value. */
export function parseGisMapFeaturePropertyEdit(value:unknown,at="$"):GisMapFeaturePropertyEdit{
 const row=parseSchemaRecord(value,["key","before","set"],at);if(typeof row.key!=="string"||row.before!==undefined&&typeof row.before!=="string")throw Error(`${at}: property identity required`);
 const set=row.set===undefined?undefined:parseSchemaRecord(row.set,["value"],`${at}.set`);
 return{key:row.key,...(row.before===undefined?{}:{before:row.before}),...(set===undefined?{}:{set:{value:parseGisMapValue(set.value)}})};
}
/** 🩹️ Preserve independent payload replacement and ordered property intent. */
export function parseGisMapFeaturePatch(value:unknown,at="$"):GisMapFeaturePatch{
 const row=parseSchemaRecord(value,["data","properties"],at);if(row.properties!==undefined&&!Array.isArray(row.properties))throw Error(`${at}.properties: array required`);
 return{data:row.data==null?null:parseGisMapValue(row.data),properties:(row.properties===undefined?[]:row.properties as unknown[]).map((edit,index)=>parseGisMapFeaturePropertyEdit(edit,`${at}.properties[${index}]`))};
}
/** ✏️ Apply direct ordered property intent to the original intrinsic object vocabulary. */
export function applyGisMapFeaturePatch(feature:GisMapFeature,patch:GisMapFeaturePatch):GisMapFeature{
 let data=patch.data??feature.data;
 if(data.kind==="object"){
  const members=data.members.map(member=>({...member}));
  for(const edit of patch.properties){const index=members.findIndex(member=>member.name===edit.key);if(edit.set){if(index>=0)members[index]={name:edit.key,value:edit.set.value};else{const before=edit.before===undefined?-1:members.findIndex(member=>member.name===edit.before);members.splice(before<0?members.length:before,0,{name:edit.key,value:edit.set.value});}}else if(index>=0)members.splice(index,1);}
  data={kind:"object",members};
 }
 return{id:feature.id,data};
}
