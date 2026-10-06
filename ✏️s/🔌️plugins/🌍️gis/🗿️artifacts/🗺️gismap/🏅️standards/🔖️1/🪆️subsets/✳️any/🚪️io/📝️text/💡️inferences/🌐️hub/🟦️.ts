import {GIS_MAP_INFERENCE_REQUEST_MAX_BYTES,type GisMapInferenceJobRequestV1,type GisMapInferenceApprovalRequestV1} from "../../../../../../../../💡️inference/🧬️schema/🟦️.ts";
import {textUtf8ByteLength} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📝️text/🟦️.ts";

/** 📤️ Admits the physical JSON request before transport publication. */
export function gisMapInferenceRequestToJson(request:GisMapInferenceJobRequestV1|GisMapInferenceApprovalRequestV1):string{
 const text=JSON.stringify(request);if(textUtf8ByteLength(text)>GIS_MAP_INFERENCE_REQUEST_MAX_BYTES)throw new Error("gis-map-inference.oversized-request");return text;
}
