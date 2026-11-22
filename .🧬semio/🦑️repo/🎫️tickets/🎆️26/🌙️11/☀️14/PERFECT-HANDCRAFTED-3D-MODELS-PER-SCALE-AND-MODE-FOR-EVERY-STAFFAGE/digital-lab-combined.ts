/** 🦴 Rebuild the Labrador’s lumbar vertebrae and moving forequarter from one catalog input. */
import {join} from 'node:path'
import {rmSync} from 'node:fs'
const [source,destination]=process.argv.slice(2),temp=destination+'.lumbar-'+process.pid
try{
 for(const [script,input,output] of [['digital-lab-lumbar.ts',source,temp],['lab-shoulder-fit.ts',temp,destination]]){
  const run=Bun.spawnSync([process.execPath,'--smol',join(import.meta.dir,script),input,output],{stdout:'inherit',stderr:'inherit'})
  if(run.exitCode!==0)throw Error(script+' exited '+run.exitCode)
 }
}finally{rmSync(temp,{force:true})}
