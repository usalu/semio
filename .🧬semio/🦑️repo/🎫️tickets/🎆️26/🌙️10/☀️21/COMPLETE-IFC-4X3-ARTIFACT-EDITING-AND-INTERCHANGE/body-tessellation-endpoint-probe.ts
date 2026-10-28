/** 🔺️ Actual concept endpoint versus independent physical IFC representation IDs. */
import {readFile,writeFile} from 'node:fs/promises';
const base='file:///C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/';
const {parseIfc4x3Step}=await import(base+'🚪️io/step.ts'),{validateIfcBodyTessellation}=await import(base+'🧬️schema/👁️mvd/🧩️concepts/🔺️tessellation/🟦️.ts');
const oracle=JSON.parse(await readFile(new URL('./🗑️generated/body-tessellation/oracle-v2.json',import.meta.url),'utf8')),cases=JSON.parse(await readFile(new URL('./body-tessellation-cases.json',import.meta.url),'utf8'));
const snapshot=await parseIfc4x3Step(await readFile(new URL('./🗑️generated/body-tessellation/graphs.ifc',import.meta.url),'utf8'),{batchSize:4096});
const report=await validateIfcBodyTessellation(snapshot,{targets:oracle.rows.map((row,i)=>({representation:String(row.representationId),applicable:cases.cases[i].applicable}))},{batchSize:4096});
const rows=report.targets.map((row,i)=>({id:oracle.rows[i].id,representation:row.representation,expected:oracle.rows[i].actual,actual:row.status,match:oracle.rows[i].actual===row.status,diagnostics:row.diagnostics}));
await writeFile(new URL('./🗑️generated/body-tessellation/endpoint.json',import.meta.url),JSON.stringify({rows,report,nativeExecution:false},null,2));console.log('[DEBUG] '+JSON.stringify({cases:rows.length,matches:rows.filter(row=>row.match).length,status:report.status,schema:report.schemaConformance,view:report.viewConformance}));if(rows.some(row=>!row.match))process.exitCode=1;
