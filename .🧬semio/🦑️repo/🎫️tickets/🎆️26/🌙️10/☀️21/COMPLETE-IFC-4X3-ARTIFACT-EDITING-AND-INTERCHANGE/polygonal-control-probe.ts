/** 🧪️ Observe actual shared source admission versus item output limits without replacing either owner. */
import {pathToFileURL} from 'node:url';
const base=process.cwd()+'/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/';
const {ifcPolygonalFixture}=await import(pathToFileURL(base+'🧪️tests/polygonal-fixtures.ts').href),{extractIfc4x3Triangles}=await import(pathToFileURL(base+'geometry.ts').href),cases=await Bun.file(base+'🧪️tests/polygonal-neutral-cases.json').json();
const source=ifcPolygonalFixture(cases.cases.find((row:any)=>row.id==='through-hole')).snapshot,result=[];
for(const maxValues of [143,144,200,256,512]){let polygonal=0;const out=await extractIfc4x3Triangles(source,{maxValues,batchSize:1,progress:(event:any)=>{if(event.phase==='polygonal')polygonal++;}});result.push({maxValues,polygonal,values:out.artifact?.map((m:any)=>m.positions.length*3+m.indices.length),diagnostics:out.diagnostics});}
console.log(JSON.stringify(result,null,2));
