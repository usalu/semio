import{parseChangeBlockField,type ChangeBlockField,type BlockField}from"../../../../../🧬️schema/🧬️mutations/🎛️change-block-field/🦠️mutation/🟦️.ts";
import{parseSchemaRecord}from"../../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import {parseFormsJsonValue,formsValueJsonProjection,parseFormsJsonCondition,formsConditionJson} from "../../../📸️snapshot/🔣️json/🟦️.ts";

/** 🎛️ Decode only the literal payload fields of the JSON mutation transport. */
export function parseFormsJsonChangeBlockField(input:unknown):ChangeBlockField{
 const row={...parseSchemaRecord(input,["mutation","blockId","field","value"])};
 if(row.value!==null){if(row.field==="default"||row.field==="params")row.value=parseFormsJsonValue(row.value);else if(row.field==="condition")row.value=parseFormsJsonCondition(row.value);}
 return parseChangeBlockField(row);
}

/** 🎛️ Project the authored mutation value into JSON without changing metadata fields. */
export function formsBlockFieldJson(value:BlockField|ChangeBlockField):unknown{
 return{...value,value:value.value===null?null:value.field==="default"||value.field==="params"?formsValueJsonProjection(value.value):value.field==="condition"?formsConditionJson(value.value):value.value};
}
