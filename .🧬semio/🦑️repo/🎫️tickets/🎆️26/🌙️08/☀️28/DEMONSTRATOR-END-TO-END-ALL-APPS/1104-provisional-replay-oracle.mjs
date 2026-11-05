import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
const root="✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🌊️flow-source/📥️provisional/🔁️replay/🧪️laws";
const cases=JSON.parse(readFileSync(root+"/📁️cases.json","utf8"));const validate=new Ajv({strict:false}).compile(JSON.parse(readFileSync(root+"/📐️schema.json","utf8")));if(!validate(cases))throw Error(JSON.stringify(validate.errors));
const db=new Database(":memory:");const set=(base,path,value)=>db.query("SELECT json_set(?,?,json(?)) AS value").get(JSON.stringify(base),path,JSON.stringify(value)).value;
for(const c of cases){let actual=structuredClone(c.base);for(const mutation of c.mutations){const[name,p]=Object.entries(mutation)[0];
if(name==="ChangeSchema")actual=JSON.parse(set(actual,"$.hostSnapshot.schema",p.schema));
else if(name==="UpdateCamera")actual=JSON.parse(set(actual,"$.hostSnapshot.camera",p.camera));
else if(name==="CreateGeneration"){actual=JSON.parse(db.query("SELECT json_insert(?,'$.generation.generations[#]',json(?)) AS value").get(JSON.stringify(actual),JSON.stringify(p.generation)).value);actual=JSON.parse(set(actual,"$.generation.selectedGenerationId",p.generation.id));}
else if(name==="RenameGeneration")actual=JSON.parse(set(actual,"$.generation.generations[0].name",p.name));
else if(name==="ChangeGenerationValue")actual=JSON.parse(set(actual,"$.generation.generations[0].values."+p.questionId,p.value));
else if(name==="SelectGeneration")actual=JSON.parse(db.query("SELECT json_remove(?,'$.generation.selectedGenerationId') AS value").get(JSON.stringify(actual)).value);
else if(["DeleteWidget","DeleteGeneration","CreateWidget"].includes(name)){
 const field=name==="DeleteGeneration"?"generation.generations":"hostSnapshot.widgets",id=name==="CreateWidget"?p.widget.id:p.id;
 const row=db.query("SELECT CAST(key AS INTEGER) AS ordinal FROM json_each(?,?) WHERE json_extract(value,'$.id')=? ORDER BY ordinal LIMIT 1").get(JSON.stringify(actual),"$."+field,id);
 if(name==="CreateWidget"){if(!row)throw Error("unique insert is independently covered by host corpus");}
 else if(row)actual=JSON.parse(db.query("SELECT json_remove(?,?) AS value").get(JSON.stringify(actual),"$."+field+"["+row.ordinal+"]").value);
}
else if(name!=="Unsupported")throw Error(name);
}if(JSON.stringify(actual)!==JSON.stringify(c.expected))throw Error(c.name);}
db.close();console.log("PASS provisional replay schema+Ajv+SQLite cases="+cases.length+"; actual native context/caller replay NOTRUN");
