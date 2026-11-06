import {readFileSync} from "node:fs";
import {Database} from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020.js";
const path="✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️native-singletons/🔗️root/🧪️tests/🔣️.json";
const law=JSON.parse(readFileSync(path,"utf8"));
const schema=JSON.parse(readFileSync(path.replace("🔣️.json","📐️.json"),"utf8"));
const validate=new Ajv2020({strict:true}).compile(schema);
if(!validate(law))throw Error(JSON.stringify(validate.errors));
const db=new Database(":memory:");
try{let count=0;for(const row of law.cases){
 const left=JSON.stringify(row.before),right=JSON.stringify(row.value);
 const actual=db.query(`SELECT CASE
  WHEN json_type(?2)='null' THEN CASE WHEN json_type(?1)='null' THEN 'missing' ELSE 'changed' END
  WHEN length(json_extract(?2,'$.target.artifactId'))=0 OR length(json_extract(?2,'$.target.dialect.artifactKind'))=0 OR length(json_extract(?2,'$.target.dialect.standard'))=0 OR length(json_extract(?2,'$.target.dialect.subset'))=0 OR length(json_extract(?2,'$.role'))=0 THEN 'invalid'
  WHEN json_extract(?2,'$.pin.kind')='checkpoint' AND length(json_extract(?2,'$.pin.id'))=0 THEN 'invalid'
  WHEN json_extract(?2,'$.pin.kind')='snapshot' AND (length(json_extract(?2,'$.pin.blob.hash'))=0 OR length(json_extract(?2,'$.pin.blob.mediaType'))=0) THEN 'invalid'
  WHEN json(?1)=json(?2) THEN 'no-op' ELSE 'changed' END AS outcome`).get(left,right).outcome;
 if(actual!==row.outcome)throw Error(`link outcome disagreement: ${row.id}: ${actual} versus ${row.outcome}`);count++;
}console.log(`${count} existing root-link outcomes agree with SQLite JSON`);}finally{db.close();}
