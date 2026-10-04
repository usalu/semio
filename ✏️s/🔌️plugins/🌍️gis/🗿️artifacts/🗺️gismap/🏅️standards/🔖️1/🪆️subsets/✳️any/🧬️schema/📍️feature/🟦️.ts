/** 📍️ GIS feature data preserves the full intrinsic native Value domain. */
import {parseSchemaRecord} from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import {parseIntrinsicValue,type IntrinsicValue} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🌳️intrinsic/🟦️.ts";
export type GisMapValue=IntrinsicValue;
export interface GisMapFeature{id:string;data:GisMapValue}
export interface GisMapFeaturePatch{data:GisMapValue|null}
/** 🧬️ GIS feature data owns the canonical full intrinsic primitive domain. */
export const parseGisMapValue=parseIntrinsicValue;
/** 📍️ Bind a feature without imposing spatial or identifier restrictions. */
export function parseGisMapFeature(value:unknown,at="$"):GisMapFeature{const row=parseSchemaRecord(value,["id","data"],at);if(typeof row.id!=="string"||/[\uD800-\uDBFF](?![\uDC00-\uDFFF])|(?<![\uD800-\uDBFF])[\uDC00-\uDFFF]/.test(row.id as string))throw Error(`${at}.id: string required`);return{id:row.id,data:parseGisMapValue(row.data)}}
/** 🩹️ Preserve unchanged absence independently from an explicit native Null value. */
export function parseGisMapFeaturePatch(value:unknown,at="$"):GisMapFeaturePatch{const row=parseSchemaRecord(value,["data"],at);return{data:row.data==null?null:parseGisMapValue(row.data)}}
