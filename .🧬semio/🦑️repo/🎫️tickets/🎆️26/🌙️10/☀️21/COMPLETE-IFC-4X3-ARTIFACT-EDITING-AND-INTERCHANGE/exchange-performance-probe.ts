import {readFile,writeFile} from 'node:fs/promises';
const base='C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/';
const {parseIfc4x3Step}=await import(base+'🚪️io/step.ts');
const {captureIfcGraphSteps,runIfcEdit}=await import(base+'🧬️schema/🔺️diff/🟦️.ts');
const {captureIfcExchangeGraph}=await import(base+'🚪️io/exchange-source.ts');
const report:any={nativeExecution:false,timers:[],fixtures:[]};
for(const count of [16,64]){const start=performance.now();for(let i=0;i<count;i++)await new Promise(resolve=>setTimeout(resolve,0));report.timers.push({count,milliseconds:performance.now()-start});}
for(const name of ['triangulated','alignment']){
 const text=await readFile(base+'🚪️io/🧪️tests/'+name+'.ifc','utf8');const phases:Record<string,number>={};let start=performance.now();
 const source=await parseIfc4x3Step(text,{progress:({phase}:any)=>phases[phase]=(phases[phase]??0)+1});const parseMs=performance.now()-start;
 const steps:Record<string,number>={};start=performance.now();runIfcEdit(captureIfcGraphSteps(source,'validateBase',{}),{onProgress:({phase}:any)=>steps[phase]=(steps[phase]??0)+1});const synchronousCaptureMs=performance.now()-start;
 const runs=[];for(const batchSize of [256,4096]){let callbacks=0;start=performance.now();const result=await captureIfcExchangeGraph(source,{batchSize,progress:()=>callbacks++});if(!result.artifact)throw Error('probe admission refused');runs.push({batchSize,callbacks,milliseconds:performance.now()-start});}
 report.fixtures.push({name,bytes:text.length,entities:source.entities.length,parseMs,parsePhases:phases,captureSteps:steps,synchronousCaptureMs,runs});
}
const output=new URL('./🗑️generated/exchange-performance-probe.json',import.meta.url);await writeFile(output,JSON.stringify(report,null,2));console.log(JSON.stringify(report));
