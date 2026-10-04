/** 🧾️ Authored configured defaults preserve ordered unique question identities and intrinsic values. */
import{parseSchemaRecord}from"../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import{parseIntrinsicValue,type IntrinsicValue}from"../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
export interface FormDictionaryEntry{questionId:string;value:IntrinsicValue}
export interface FormDictionary{entries:FormDictionaryEntry[]}
/** 🛂️ Bind the complete first-party values without interpreting or serializing a source string. */
export function parseFormDictionary(source:unknown,at="$"):FormDictionary{
 const row=parseSchemaRecord(source,["entries"],at);if(!Array.isArray(row.entries))throw Error(at+": dictionary entries required");
 const names=new Set<string>(),entries:FormDictionaryEntry[]=[];
 for(let index=0;index<row.entries.length;index++){const entry=parseSchemaRecord(row.entries[index],["questionId","value"],at+".entries["+index+"]");if(typeof entry.questionId!=="string"||names.has(entry.questionId))throw Error(at+": duplicate or invalid question identity");names.add(entry.questionId);entries.push({questionId:entry.questionId,value:parseIntrinsicValue(entry.value)});}
 return{entries};
}

