import {splitUiTextChunks,joinUiTextChunks} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🔤️text-chunks/🟦️.ts";

export type PatchQuestions={questionIds:string[];field:string;paramKey?:string;valueJsonChunks:string[]};

/** 📥️ Canonical complete-value ingress shared by scalar controls and paged authoring. */
export function normalizePatchQuestions(input:unknown):PatchQuestions{
 if(!input||typeof input!=="object"||Array.isArray(input))throw Error("patch arguments required");
 const row=input as Record<string,unknown>,encoder=new TextEncoder();
 if(!Array.isArray(row.questionIds)||!row.questionIds.length||row.questionIds.length>64||row.questionIds.reduce((sum,id)=>sum+(typeof id==="string"?encoder.encode(id).length:8193),0)>8192||row.questionIds.some(id=>typeof id!=="string"||!id.length||encoder.encode(id).length>512)||typeof row.field!=="string"||!row.field.length||encoder.encode(row.field).length>128||row.paramKey!==undefined&&(typeof row.paramKey!=="string"||!row.paramKey.length||encoder.encode(row.paramKey).length>128))throw Error("invalid patch target");
 if(Object.hasOwn(row,"valueJson")||Object.hasOwn(row,"value_json"))throw Error("unsupported patch transport");
 const scalar=Object.hasOwn(row,"value"),paged=Object.hasOwn(row,"valueJsonChunks");
 if(scalar===paged)throw Error("mixed patch transports");
 const valueJsonChunks=paged?(joinUiTextChunks(row.valueJsonChunks,32770),row.valueJsonChunks as string[]):splitUiTextChunks(JSON.stringify(row.value),32770);
 if(valueJsonChunks.length>65)throw Error("patch chunk count exceeded");
 JSON.parse(joinUiTextChunks(valueJsonChunks,32770));
 return{questionIds:row.questionIds as string[],field:row.field,...(row.paramKey!==undefined?{paramKey:row.paramKey as string}:{}),valueJsonChunks};
}
