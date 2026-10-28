/** 🚆️ Exercise the actual public endpoint and emit authored fixtures for independent comparison. */
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
const any=resolve(process.argv[2]!),dir=resolve(any,'🧬️schema/👁️mvd/🧩️concepts/📐️alignment'),out=resolve(import.meta.dir,'🗑️generated/alignment-direct-actual');await mkdir(out,{recursive:true});
const api=await import(pathToFileURL(resolve(any,'🟦️.ts')).href),{ifcAlignmentDirectFixture}=await import(pathToFileURL(resolve(dir,'direct-fixture.ts')).href),cases=JSON.parse(await readFile(resolve(dir,'direct-cases.json'),'utf8')),rows=[];
for(const row of cases.cases){
 const graph=ifcAlignmentDirectFixture(row),segment=String(graph.segment.value),request={targets:[{segment,kind:row.id==='wrong-kind'?'vertical':row.kind,applicable:row.applicable}]},report=await api.validateIfcAlignmentDirectCounterpart(graph.snapshot,request,{batchSize:4096});
 const result:Record<string,unknown>={id:row.id,request,expected:row.expected,actual:report.status,curve:String(graph.curve.value),diagnostics:report.targets[0].diagnostics};
 try{const step=await api.writeIfc4x3Step(graph.snapshot,{batchSize:4096});await writeFile(resolve(out,row.id+'.ifc'),step);result.written=true;result.sha256=createHash('sha256').update(step).digest('hex');}catch(error){result.written=false;result.refusal=String(error);}
 rows.push(result);
}
const originals=[];
for(const [file,segment,kind]of [['sectioned-solid-horizontal.ifc','1101','horizontal'],['sectioned-solid-horizontal.ifc','2001','vertical'],['linear-placement-of-signal.ifc','6001','cant'],['segmented-reference-curve.ifc','149','cant']]){
 const bytes=await readFile(resolve(any,'🧪️fixtures/official',file!)),source=await api.parseIfc4x3Step(bytes.toString('utf8'),{batchSize:4096}),report=await api.validateIfcAlignmentDirectCounterpart(source,{targets:[{segment,kind,applicable:true}]},{batchSize:4096});originals.push({file,segment,kind,sha256:createHash('sha256').update(bytes).digest('hex'),status:report.status,curve:report.targets[0].curve,diagnostics:report.targets[0].diagnostics});
}
await writeFile(resolve(out,'owned.json'),JSON.stringify({rows,originals,publicEndpoint:true,nativeExecution:false},null,2));console.log('[DEBUG] '+JSON.stringify({neutral:rows.length,matches:rows.filter(row=>row.actual===row.expected).length,written:rows.filter(row=>row.written).length,originals:originals.length}));
