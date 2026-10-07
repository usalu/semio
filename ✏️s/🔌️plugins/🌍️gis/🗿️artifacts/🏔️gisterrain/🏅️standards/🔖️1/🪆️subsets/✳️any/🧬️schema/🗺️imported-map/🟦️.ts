/** 🗺️ Ordered complete feature objects and literal root properties. */
import {parseSchemaRecord} from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import {parseIntrinsicValue,type IntrinsicValue,type IntrinsicMember} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
export type ImportedFeature=Extract<IntrinsicValue,{kind:"object"}>;
export interface ImportedMap{positions:ImportedFeature[];routes:ImportedFeature[];regions:ImportedFeature[];properties:IntrinsicMember[]}
/** 📥️ A typed map transport normalizes missing collections to empty while null refuses. */
export function importedMapFromMedia(value:IntrinsicValue):ImportedMap{
 const root=parseIntrinsicValue(value);if(root.kind!=="object")throw Error("map intrinsic object required");const result:ImportedMap={positions:[],routes:[],regions:[],properties:[]},reserved=new Set<string>();
 for(const member of root.members){if(["positions","routes","regions"].includes(member.name)){if(reserved.has(member.name))throw Error("duplicate map collection");reserved.add(member.name);if(member.value.kind!=="array")throw Error("map collection array required");const records=result[member.name as"positions"|"routes"|"regions"];for(const value of member.value.items){if(value.kind!=="object")throw Error("imported feature object required");records.push(value);}}else result.properties.push(member);}return result;
}
/** 🛂️ Durable admission preserves occurrence order without renderer constraints. */
export function parseImportedMap(source:unknown):ImportedMap{
 const row=parseSchemaRecord(source,["positions","routes","regions","properties"]),result:ImportedMap={positions:[],routes:[],regions:[],properties:[]};
 for(const role of ["positions","routes","regions"] as const){if(!Array.isArray(row[role]))throw Error("imported map requires ordered collections");for(const source of row[role]){const value=parseIntrinsicValue(source);if(value.kind!=="object")throw Error("imported feature requires intrinsic object");result[role].push(value);}}
 if(!Array.isArray(row.properties))throw Error("imported map properties require ordered members");
 const properties=parseIntrinsicValue({kind:"object",members:row.properties});if(properties.kind!=="object")throw Error("imported map property object required");
 for(const member of properties.members){if(["positions","routes","regions"].includes(member.name))throw Error("reserved imported map property");result.properties.push(member);}return result;
}
