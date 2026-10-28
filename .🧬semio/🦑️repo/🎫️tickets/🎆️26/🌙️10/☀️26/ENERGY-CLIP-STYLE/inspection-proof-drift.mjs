import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import {fileURLToPath} from 'node:url';
const ticket=path.dirname(fileURLToPath(import.meta.url));
let workspace=ticket;while(!fs.existsSync(path.join(workspace,'🎓️teaching'))||!fs.existsSync(path.join(workspace,'🧰️framework'))){const parent=path.dirname(workspace);if(parent===workspace)throw Error('Workspace not found');workspace=parent}
const results=[];
for(const name of ['chapter-provenance.json','bim-closure.json']){const proof=JSON.parse(fs.readFileSync(path.join(ticket,'🗑️generated','iv',name),'utf8'));const issues=[];for(const [file,expected]of Object.entries(proof.files)){try{const real=fs.realpathSync(file),relative=path.relative(workspace,real),inside=Boolean(relative)&&!relative.startsWith('..')&&!path.isAbsolute(relative),actual=crypto.createHash('sha256').update(fs.readFileSync(real)).digest('hex');if(actual!==expected||!inside)issues.push({file,expected,actual,inside})}catch(error){issues.push({file,error:String(error)})}}results.push({name,files:Object.keys(proof.files).length,issues})}
fs.writeFileSync(path.join(ticket,'🗑️generated','io','proof-drift.json'),JSON.stringify(results,null,2));
fs.writeFileSync(path.join(ticket,'inspection-proof-drift.md'),'# Retention Proof Diagnostic\n\n'+results.map(result=>'## '+result.name+'\n\n'+JSON.stringify({files:result.files,issues:result.issues.length,examples:result.issues.slice(0,3)},null,2)).join('\n\n')+'\n');
console.log(JSON.stringify(results.map(result=>({name:result.name,files:result.files,issues:result.issues.length,examples:result.issues.slice(0,3)}))));
