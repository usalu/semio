/** 🚄️ Compare the actual endpoint with independent physical IfcOpenShell layout traversals. */
import {readFile,writeFile} from 'node:fs/promises';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
const root=resolve(import.meta.dir,'../../../../../../..'),base=resolve(root,'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any');
const {validateIfcAlignmentTerminal}=await import(pathToFileURL(resolve(base,'🧬️schema/👁️mvd/🧩️concepts/📐️alignment/🟦️.ts')).href);
const {parseIfc4x3Step}=await import(pathToFileURL(resolve(base,'🚪️io/step.ts')).href);
const inputs=JSON.parse(await readFile(resolve(import.meta.dir,'alignment-terminal-cases.json'),'utf8'));
const oracle=JSON.parse(await readFile(resolve(import.meta.dir,'🗑️generated/alignment-terminal/oracle.json'),'utf8'));
const source=await parseIfc4x3Step(await readFile(resolve(import.meta.dir,'🗑️generated/alignment-terminal/graphs.ifc'),'utf8'),{batchSize:4096});
const request={targets:oracle.rows.map((row:any)=>{const input=inputs.cases.find((item:any)=>item.id===row.id);if(!input)throw Error('Unmatched independent case');return {layout:String(row.layoutId),kind:input.layout,applicable:input.applicable};})};
const report=await validateIfcAlignmentTerminal(source,request,{batchSize:4096});
const rows=oracle.rows.map((row:any,i:number)=>({id:row.id,layoutId:row.layoutId,oracle:row.actual,actual:report.targets[i]?.status,match:row.actual===report.targets[i]?.status,terminalAttribute:row.terminalAttribute}));
const result={oracle:oracle.oracle,sourceEntities:source.entities.length,rows,geometryConformance:report.geometryConformance,schemaConformance:report.schemaConformance,viewConformance:report.viewConformance,nativeExecution:false};
await writeFile(resolve(import.meta.dir,'🗑️generated/alignment-terminal/endpoint.json'),JSON.stringify(result,null,2));
console.log('[DEBUG] '+JSON.stringify({cases:rows.length,matches:rows.filter((row:any)=>row.match).length,sourceEntities:source.entities.length}));
if(rows.length!==11||rows.some((row:any)=>!row.match))throw Error('Independent physical endpoint mismatch');
const cantOracle=JSON.parse(await readFile(resolve(import.meta.dir,'🗑️generated/alignment-geometry/cant-oracle.json'),'utf8')),officialRows=[];
for(const file of cantOracle.files){if(!file.cantLayouts.length)continue;const bytes=await readFile(resolve(base,'🧪️fixtures/official',file.file));if(createHash('sha256').update(bytes).digest('hex')!==file.sha256)throw Error('Original official hash differs');const original=await parseIfc4x3Step(bytes.toString('utf8'),{batchSize:4096});
 const targets=file.cantLayouts.map((layout:any)=>({layout:String(layout.layout),kind:'cant',applicable:true})),verdict=await validateIfcAlignmentTerminal(original,{targets},{batchSize:4096});
 for(let i=0;i<targets.length;i++){const layout=file.cantLayouts[i],expected=layout.lists.length===1&&layout.lists[0].businessLength===0?'pass':'incomplete',actual=verdict.targets[i]?.status;officialRows.push({file:file.file,sourceSha256:file.sha256,layout:targets[i].layout,expected,actual,match:expected===actual,sourceEntities:original.entities.length,geometryConformance:verdict.geometryConformance});}
}
await writeFile(resolve(import.meta.dir,'🗑️generated/alignment-terminal/official-cant-endpoint.json'),JSON.stringify({oracle:cantOracle.oracle+' '+cantOracle.version,rows:officialRows,nativeExecution:false},null,2));
console.log('[DEBUG] '+JSON.stringify({officialCantCases:officialRows.length,matches:officialRows.filter(row=>row.match).length}));if(officialRows.length!==2||officialRows.some(row=>!row.match))throw Error('Original official cant endpoint mismatch');
