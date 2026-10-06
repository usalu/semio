import{readFileSync,writeFileSync,mkdirSync}from"node:fs";import{createHash}from"node:crypto";
const ticket=import.meta.dir+"/../..",path="C:/git/semio/🧰️framework/🛍️products/📓️print/🧬️schema/🔣️.json",before=readFileSync(path,"utf8"),hash=(s:string)=>createHash("sha256").update(s).digest("hex");if(hash(before)!=="855cdd6b98cbba96308c3184a32af39e011779546abca1f38b75dea518dedce1")throw Error("Schema drift");
let i=0;const objects=new Map<string,any>();
function ws(){while(/\s/.test(before[i]??"")&&i<before.length)i++;}
function str(){const start=i++;while(i<before.length){const c=before[i++];if(c==='\\'){i++;continue;}if(c==='"')return JSON.parse(before.slice(start,i));}throw Error("String");}
function value(keys:string[]){ws();if(before[i]==='{'){const start=i++,members:any[]=[];ws();while(before[i]!=='}'){const keyStart=i,key=str();ws();if(before[i++]!==':')throw Error("Colon");const valueStart=i;value([...keys,key]);members.push({key,keyStart,valueStart,end:i});ws();if(before[i]===','){i++;ws();}else if(before[i]!=='}')throw Error("Delimiter");}i++;objects.set(keys.join("\0"),{start,end:i,members});}else if(before[i]==='['){i++;ws();while(before[i]!==']'){value(keys);ws();if(before[i]===','){i++;ws();}else if(before[i]!==']')throw Error("Array");}i++;}else if(before[i]==='"')str();else{while(i<before.length&&!/[\s,}\]]/.test(before[i]))i++;}}
value([]);ws();if(i!==before.length)throw Error("Trailing");


const original=JSON.parse(before),expected=structuredClone(original),changes:any[]=[],edits:any[]=[];
const patches:any[]=[
["geo-terrain","labels",{type:"boolean",syntax:"scalar",default:false}],
["geo-weather","cell",{type:"number",syntax:"expression",default:12,description:{en:"Raster cell size in millimetres.",de:"Rasterzellgröße in Millimetern."}}],
["geo-basemap","places",{type:"string",syntax:"identifier",default:"demo-geo-cities",description:{en:"Named geographic point collection used by the settlement layer; settlements enables that layer.",de:"Benannte geografische Punktsammlung für die Siedlungsebene; settlements aktiviert diese Ebene."}}],
["composition-bar","data",{default:"demo-parts"}],
["swimlane","bend",{type:"number",syntax:"expression",default:22,description:{en:"Bend angle in degrees for curved flow connectors; orthogonal routing does not use it.",de:"Krümmungswinkel in Grad für gekrümmte Ablaufverbinder; orthogonales Routing nutzt ihn nicht."}}]
];
for(const[family,key,patch]of patches){const target=expected["x-semio-family-options"][family].options[key],prior=structuredClone(target);Object.assign(target,patch);const node=objects.get(["x-semio-family-options",family,"options",key].join("\0"));if(!node)throw Error("Descriptor range");edits.push({start:node.start,end:node.end,text:JSON.stringify(target,null,2).replace(/\n/g,"\n        ")});changes.push({family,key,before:prior,after:target});}
let after=before;for(const edit of edits.sort((a,b)=>b.start-a.start))after=after.slice(0,edit.start)+edit.text+after.slice(edit.end);
const canonical=(v:any):any=>Array.isArray(v)?v.map(canonical):v&&typeof v==='object'?Object.fromEntries(Object.keys(v).sort().map(k=>[k,canonical(v[k])])):v;if(JSON.stringify(canonical(JSON.parse(after)))!==JSON.stringify(canonical(expected)))throw Error("Unexpected semantic delta");const retained=ticket+"/📥️authored-inputs/native-final-five-before";mkdirSync(retained,{recursive:true});writeFileSync(retained+"/🔣️.json",before);writeFileSync(retained+"/delta.json",JSON.stringify(changes,null,2));writeFileSync(import.meta.dir+"/candidate.json",after);console.log(JSON.stringify({before:hash(before),after:hash(after),descriptors:changes.length}));