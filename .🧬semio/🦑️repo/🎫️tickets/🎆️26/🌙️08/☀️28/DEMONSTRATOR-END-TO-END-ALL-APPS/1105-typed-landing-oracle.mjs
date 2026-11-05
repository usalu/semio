import {readFileSync} from "node:fs";
import Ajv from "ajv";
import lodash from "lodash";
import {Database} from "bun:sqlite";
import {compareCurrent,nativeLiteral} from "./1105-typed-oracle-lib.mjs";
const root="✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained-commands/📥️provisional/🎛️input/🧪️laws";
const cases=JSON.parse(readFileSync(root+"/📁️cases.json","utf8"));const ajv=new Ajv({strict:false});if(!ajv.compile(JSON.parse(readFileSync(root+"/📐️schema.json","utf8")))(cases))throw Error("shape");
const admit=ajv.compile(JSON.parse(readFileSync("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-widget-input/🧬️schema/🔣️.json","utf8")));
const db=new Database(":memory:");
for(const c of cases){let actual=structuredClone(c.before),applied=false;const p=c.mutation;
if(admit(p)){
 const target=db.query("SELECT CAST(key AS INTEGER) AS ordinal FROM json_each(?) WHERE json_extract(value,'$.id')=? ORDER BY ordinal LIMIT 1").get(JSON.stringify(actual.widgets),p.id);
 const wired=db.query("SELECT 1 FROM json_each(?) WHERE json_extract(value,'$.to')=? AND json_extract(value,'$.toPort')=? LIMIT 1").get(JSON.stringify(actual.synapses),p.id,p.channel);
 if(target&&!wired){
  const widget=actual.widgets[target.ordinal];
  if(widget.kind==="inputNote"&&p.channel==="text"&&p.type==="text"&&widget.text!==p.value){widget.text=p.value;applied=true;}
  else if(widget.kind==="neuron"&&Object.hasOwn(widget.params,p.channel)&&compareCurrent(widget.params[p.channel],{type:p.type,value:p.value})==="different"){widget.params[p.channel]=nativeLiteral({type:p.type,value:p.value});applied=true;}
 }
}
if(applied!==c.applied||!lodash.isEqual(actual,c.after))throw Error(c.name);
}
db.close();console.log("PASS provisional typed landing schema+Ajv+SQLite+lodash cases="+cases.length+"; native exact central/physical/caller laws NOTRUN");
