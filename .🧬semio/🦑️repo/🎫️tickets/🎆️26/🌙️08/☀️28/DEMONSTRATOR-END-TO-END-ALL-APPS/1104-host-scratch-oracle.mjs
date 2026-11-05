import {readFileSync,readdirSync,existsSync} from "node:fs";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
const root="✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🌊️flow-source/✏️host-scratch/🧪️laws";
const mutationRoot="✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations";
const ajv=new Ajv({strict:false,allErrors:true,validateFormats:false});
for(const dir of readdirSync(mutationRoot,{withFileTypes:true})){if(!dir.isDirectory())continue;const file=mutationRoot+"/"+dir.name+"/🧬️schema/🔣️.json";if(existsSync(file))ajv.addSchema(JSON.parse(readFileSync(file,"utf8")));}
const corpus=JSON.parse(readFileSync(root+"/📁️cases.json","utf8"));const validate=ajv.compile(JSON.parse(readFileSync(root+"/📐️schema.json","utf8")));if(!validate(corpus))throw Error(JSON.stringify(validate.errors?.slice(0,3)));
const db=new Database(":memory:");
const find=(rows,id)=>db.query("SELECT CAST(key AS INTEGER) AS ordinal FROM json_each(?) WHERE json_extract(value,'$.id')=? ORDER BY ordinal LIMIT 1").get(JSON.stringify(rows),id)?.ordinal;
function splice(rows,kind,at,value){return db.query("SELECT value FROM (SELECT CASE WHEN ?='insert' AND CAST(key AS INTEGER)>=? THEN CAST(key AS INTEGER)+1 ELSE CAST(key AS INTEGER) END AS ordinal,CASE WHEN ?='replace' AND CAST(key AS INTEGER)=? THEN ? ELSE value END AS value FROM json_each(?) WHERE NOT (?='remove' AND CAST(key AS INTEGER)=?) UNION ALL SELECT ?,? WHERE ?='insert') ORDER BY ordinal").all(kind,at,kind,at,JSON.stringify(value),JSON.stringify(rows),kind,at,at,JSON.stringify(value),kind).map(row=>JSON.parse(row.value));}
let count=0;
for(const c of corpus.cases)for(const domain of c.domains){
 let actual=structuredClone(corpus.base),applied=false;const [name,p]=Object.entries(c.edit)[0];const widgets=actual.widgets,synapses=actual.synapses;
 const rowKinds={InsertWidget:["widgets","insert",p.widget?.id,p.widget],ReplaceWidget:["widgets","replace",p.widget?.id,p.widget],RemoveWidget:["widgets","remove",p.id,null],InsertSynapse:["synapses","insert",p.synapse?.id,p.synapse],ReplaceSynapse:["synapses","replace",p.synapse?.id,p.synapse],RemoveSynapse:["synapses","remove",p.id,null]};
 if(rowKinds[name]){
  const [field,kind,id,value]=rowKinds[name],rows=actual[field],old=find(rows,id);
  let valid=kind==="insert"?old===undefined:old!==undefined;
  if(name==="InsertSynapse"){
   valid&&=find(widgets,p.synapse.from)!==undefined&&find(widgets,p.synapse.to)!==undefined;
   if(domain==="2d"&&valid){const v=p.synapse;valid=!db.query("SELECT 1 FROM json_each(?) WHERE json_extract(value,'$.from')=? AND json_extract(value,'$.fromPort')=? AND json_extract(value,'$.to')=? AND json_extract(value,'$.toPort')=? LIMIT 1").get(JSON.stringify(rows),v.from,v.fromPort,v.to,v.toPort);}
  }
  if(valid){actual[field]=splice(rows,kind,kind==="insert"?Math.min(p.index,rows.length):old,value);applied=true;}
 }else if(name==="SetLayout"){if(find(widgets,p.id)!==undefined){actual=JSON.parse(db.query("SELECT json_set(?, ?, json(?)) AS value").get(JSON.stringify(actual),"$.layout."+p.id,JSON.stringify(p.layout)).value);applied=true;}}
 else if(name==="RemoveLayout"){if(Object.hasOwn(actual.layout,p.id)){actual=JSON.parse(db.query("SELECT json_remove(?,?) AS value").get(JSON.stringify(actual),"$.layout."+p.id).value);applied=true;}}
 else if(name==="SetCamera"){if(!["x","y","zoom"].every(key=>actual.camera[key]===p.camera[key])){actual.camera=p.camera;applied=true;}}
 else if(name==="SetSchema"){if(!(domain==="3d"&&/^\p{White_Space}*$/u.test(p.schema))&&actual.schema!==p.schema){actual.schema=p.schema;applied=true;}}
 else if(name==="SetSlider"){const index=find(widgets,p.id);if(index!==undefined&&widgets[index].kind==="inputSlider"&&widgets[index].value!==p.value){if(p.value<widgets[index].min||p.value>widgets[index].max)throw Error("range extension uses its independent native oracle");widgets[index].value=p.value;applied=true;}}
 else if(name==="MoveNodes"){const valid=p.ids.filter(id=>find(widgets,id)!==undefined&&Object.hasOwn(actual.layout,id));if(valid.length&&(p.dx!==0||p.dy!==0)){for(const id of valid){actual.layout[id]={x:actual.layout[id].x+p.dx,y:actual.layout[id].y+p.dy};}applied=true;}}
 else throw Error(name);
 const expected=Object.assign(structuredClone(corpus.base),c.expected);
 if(JSON.stringify(actual)!==JSON.stringify(expected)||applied!==c.applied)throw Error(domain+" "+c.name);
 count++;
}
db.close();console.log("PASS procedural host edit schema+Ajv+SQLite cases="+count+"; native cold/retained equality NOTRUN");
