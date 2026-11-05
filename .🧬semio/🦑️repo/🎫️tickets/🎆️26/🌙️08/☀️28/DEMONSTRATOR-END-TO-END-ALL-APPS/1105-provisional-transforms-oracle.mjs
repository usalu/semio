import {readFileSync} from "node:fs";
import Ajv from "ajv";
import lodash from "lodash";
import {Database} from "bun:sqlite";
import {Quaternion,Vector3} from "three";
import {nativeLiteral} from "./1105-typed-oracle-lib.mjs";
const root="✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained-commands/📥️provisional/🧭️transform/🧪️laws";
const corpus=JSON.parse(readFileSync(root+"/📁️cases.json","utf8")),ajv=new Ajv({strict:false});
if(!ajv.compile(JSON.parse(readFileSync(root+"/📐️schema.json","utf8")))(corpus))throw Error("shape");
const mutationRoot="✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations";
const schemas={dragTransforms:ajv.compile(JSON.parse(readFileSync(mutationRoot+"/✋️drag-transforms/🧬️schema/🔣️.json","utf8"))),rotateTransforms:ajv.compile(JSON.parse(readFileSync(mutationRoot+"/🔃️rotate-transforms/🧬️schema/🔣️.json","utf8"))),scaleTransforms:ajv.compile(JSON.parse(readFileSync(mutationRoot+"/📏️scale-transforms/🧬️schema/🔣️.json","utf8")))};
const db=new Database(":memory:");
const find=(rows,id)=>db.query("SELECT CAST(key AS INTEGER) AS ordinal FROM json_each(?) WHERE json_extract(value,'$.id')=? ORDER BY ordinal LIMIT 1").get(JSON.stringify(rows),id)?.ordinal;
function materialize(c){const scene=structuredClone(corpus.base);for(const[id,params]of Object.entries(c.before.params??{}))scene.widgets[find(scene.widgets,id)].params=params;if(c.before.append)scene.widgets.push(...structuredClone(c.before.append));if(c.before.synapses)scene.synapses=structuredClone(c.before.synapses);return scene;}
function number(params,key,field,fallback){const p="$."+key+"."+field,row=db.query("SELECT json_type(?1,?2) AS kind,json_extract(?1,?2) AS value").get(JSON.stringify(params),p);return["integer","real"].includes(row.kind)?row.value:fallback;}
const vector=(params,key,fallback)=>["x","y","z"].map((field,index)=>number(params,key,field,fallback[index]));
function quaternion(axis,angle){if(!axis.every(Number.isFinite)||!Number.isFinite(angle))return null;const scale=Math.max(...axis.map(Math.abs));if(scale===0)return null;const v=new Vector3(...axis).multiplyScalar(1/scale).normalize();return new Quaternion().setFromAxisAngle(v,angle);}
let invalidSchemas=0;
for(const c of corpus.cases){const p=c.mutation,actual=materialize(c),expected=materialize(c);for(const[id,entries]of Object.entries(c.expectedParams))Object.assign(expected.widgets[find(expected.widgets,id)].params,entries);let applied=false;
const schemaValid=schemas[p.mutation](p);if(!schemaValid)invalidSchemas++;
const targets=db.query("SELECT COUNT(*) AS n,COUNT(DISTINCT value) AS unique_n,MIN(length(value)) AS shortest FROM json_each(?)").get(JSON.stringify(p.targets));
let inputValid=targets.n>0&&targets.n===targets.unique_n&&targets.shortest>0,identity=false,kinds;
if(p.mutation==="dragTransforms"){inputValid&&=[p.dx,p.dy,p.dz].every(Number.isFinite);identity=[p.dx,p.dy,p.dz].every(v=>v===0);kinds=["brep.xform.translate","brep.mesh.translate","brep.mesh.translateComponents"];}
else if(p.mutation==="rotateTransforms"){inputValid&&=[p.ax,p.ay,p.az,p.angle].every(Number.isFinite)&&[p.ax,p.ay,p.az].some(v=>v!==0);identity=p.angle===0;kinds=["brep.xform.rotate","brep.mesh.rotate","brep.mesh.rotateComponents"];}
else{inputValid&&=[p.sx,p.sy,p.sz].every(v=>Number.isFinite(v)&&v>0);identity=[p.sx,p.sy,p.sz].every(v=>v===1);kinds=["brep.xform.scale","brep.mesh.scale","brep.mesh.scaleComponents"];}
const patches=[];
if(inputValid)for(const id of p.targets){const index=find(actual.widgets,id);if(index===undefined)continue;const widget=actual.widgets[index];if(widget.kind!=="neuron"||!kinds.includes(widget.neuronKind))continue;let entries;
if(p.mutation==="dragTransforms"){const next=new Vector3(...vector(widget.params,"offset",[0,0,0])).add(new Vector3(p.dx,p.dy,p.dz)).toArray();if(next.every(Number.isFinite))entries={offset:nativeLiteral({type:"vector",value:next})};}
else if(p.mutation==="scaleTransforms"){const next=new Vector3(...vector(widget.params,"factor",[1,1,1])).multiply(new Vector3(p.sx,p.sy,p.sz)).toArray();if(next.every(v=>Number.isFinite(v)&&v!==0))entries={factor:nativeLiteral({type:"vector",value:next}),center:nativeLiteral({type:"point",value:[0,0,0]})};}
else{const current=quaternion(vector(widget.params,"axis",[0,0,1]),number(widget.params,"angle","value",0)),delta=quaternion([p.ax,p.ay,p.az],p.angle);if(current&&delta){const next=delta.multiply(current);if(next.w<0)next.set(-next.x,-next.y,-next.z,-next.w);const sine=Math.hypot(next.x,next.y,next.z),axis=sine===0?[0,0,1]:[next.x/sine,next.y/sine,next.z/sine],angle=sine===0?0:2*Math.atan2(sine,next.w);entries={axis:nativeLiteral({type:"vector",value:axis}),angle:nativeLiteral({type:"number",value:angle})};}}
if(entries)patches.push([index,entries]);}
if(patches.length&&!identity){for(const[index,entries]of patches)Object.assign(actual.widgets[index].params,entries);applied=true;}
const equal=lodash.isEqualWith(actual,expected,(a,b)=>typeof a==="number"&&typeof b==="number"?Object.is(a,b)||Number.isFinite(a)&&Number.isFinite(b)&&Math.abs(a-b)<=1e-12*Math.max(1,Math.abs(a),Math.abs(b)):undefined);
if(applied!==c.applied||!equal||c.applied&&!schemaValid)throw Error(c.name);
}
db.close();console.log("PASS original transform schema+Ajv+SQLite+ThreeQuaternion+lodash cases="+corpus.cases.length+"; authored schema refusals="+invalidSchemas+"; native original/one-fuel laws NOTRUN");
