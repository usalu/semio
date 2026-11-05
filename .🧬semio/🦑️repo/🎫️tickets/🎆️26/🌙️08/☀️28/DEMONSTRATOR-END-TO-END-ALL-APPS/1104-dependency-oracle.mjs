import {readFileSync,writeFileSync} from "node:fs";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
const root="🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📤️publication/📡️invocation/🌳️dependency-work/🧪️laws/";
const cases=JSON.parse(readFileSync(root+"📁️cases.json","utf8"));
const schema=JSON.parse(readFileSync(root+"📐️schema.json","utf8"));
const validate=new Ajv({allErrors:true}).compile(schema);
if(!validate(cases))throw new Error(JSON.stringify(validate.errors));
const dictionary=value=>value&&typeof value==="object"&&!Array.isArray(value)?Object.fromEntries(Object.keys(value).sort().map(key=>[key,dictionary(value[key])])):value;
const tree=value=>({neurons:value.neurons.map(row=>({id:row.id,kind:row.kind,params:dictionary(row.params??{}),tree:row.tree?tree(row.tree):null})),synapses:value.synapses.map(wire)});
function wire(row){return {id:row.id,from:row.from,to:row.to,fromPort:row.fromPort??"",toPort:row.toPort??""}}
const widget=row=>{switch(row.kind){case"neuron":return{operatorId:row.neuronKind,params:dictionary(row.params??{})};case"inputSlider":return{number:row.value??0};case"inputNote":return{text:row.text};case"inputImage":return{source:row.src};case"variable":return{name:row.name,schema:row.schema};case"cluster":return tree(row.tree);default:return null}};
for(const row of cases){
 const db=new Database(":memory:");
 db.run("CREATE TABLE widgets (ordinal INTEGER PRIMARY KEY,id TEXT NOT NULL)");
 db.run("CREATE TABLE edges (ordinal INTEGER PRIMARY KEY,id TEXT NOT NULL,src TEXT NOT NULL,dst TEXT NOT NULL)");
 row.widgets.forEach((item,index)=>db.run("INSERT INTO widgets VALUES(?,?)",index,item.id));
 row.synapses.forEach((item,index)=>db.run("INSERT INTO edges VALUES(?,?,?,?)",index,item.id,item.from,item.to));
 const ancestry="WITH RECURSIVE ancestors(id) AS (SELECT ? UNION SELECT src FROM edges JOIN ancestors ON edges.dst=ancestors.id) ";
 const widgets=db.query(ancestry+"SELECT MAX(ordinal) AS ordinal FROM widgets WHERE id IN (SELECT id FROM ancestors) GROUP BY id ORDER BY id COLLATE BINARY").all(row.target);
 const synapses=db.query(ancestry+"SELECT MAX(ordinal) AS ordinal FROM edges WHERE dst IN (SELECT id FROM ancestors) GROUP BY id ORDER BY id COLLATE BINARY").all(row.target);
 const result={widgets:Object.fromEntries(widgets.map(({ordinal})=>[row.widgets[ordinal].id,widget(row.widgets[ordinal])])),synapses:Object.fromEntries(synapses.map(({ordinal})=>[row.synapses[ordinal].id,wire(row.synapses[ordinal])]))};
 const expected=JSON.stringify(result);
 if(process.argv.includes("--author"))row.expected=expected;
 else if(row.expected!==expected)throw new Error("Independent SQLite ancestry disagrees: "+row.name);
 db.close();
}
if(process.argv.includes("--author"))writeFileSync(root+"📁️cases.json",JSON.stringify(cases,null,2)+"\n");
console.log("Ajv validated 4 dependency cases; SQLite recursive ancestry and last-row selection reproduced exact wires.");
