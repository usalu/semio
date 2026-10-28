/** 🎨️ Probe the pinned MaxOneColour rule through the actual full reference population validator. */
import {mkdir,writeFile} from 'node:fs/promises';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
const io=resolve(process.cwd(),'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io');
const {parseIfc4x3Step}=await import(pathToFileURL(resolve(io,'step.ts')).href),{validateIfc4x3Express}=await import(pathToFileURL(resolve(io,'../🧬️schema/⚖️validation/population.ts')).href);
const header="ISO-10303-21;HEADER;FILE_DESCRIPTION((''),'2;1');FILE_NAME('','',(''),(''),'','','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;",footer="ENDSEC;END-ISO-10303-21;",output=resolve(import.meta.dir,'🗑️generated/colour-select');await mkdir(output,{recursive:true});const rows=[];
for(const count of [1,2]){
 const body="#1=IFCCOLOURRGB($,1.,0.,0.);#2=IFCCOLOURRGB($,0.,1.,0.);#3=IFCFILLAREASTYLE('Colour',("+Array.from({length:count},(_,index)=>'#'+(index+1)).join(',')+"),.T.);",text=header+body+footer,snapshot=await parseIfc4x3Step(text),result=await validateIfc4x3Express(snapshot);
 const selected=result.results.filter(row=>row.owner==='IFCFILLAREASTYLE');rows.push({count,expectedMaxOneColour:count===1?'true':'false',complete:result.complete,valid:result.valid,results:selected,diagnostics:result.diagnostics});
 await writeFile(resolve(output,'colours-'+count+'.ifc'),text);
}
await writeFile(resolve(output,process.argv.includes('--after')?'reference-after.json':'reference.json'),JSON.stringify({nativeExecution:false,rows},(_,value)=>typeof value==='bigint'?value.toString():value,2)+'\n');console.log('[DEBUG] '+JSON.stringify(rows.map(row=>({count:row.count,valid:row.valid,complete:row.complete,maxOneColour:row.results.find(rule=>rule.rule==='MAXONECOLOUR')?.result,expected:row.expectedMaxOneColour}))));

