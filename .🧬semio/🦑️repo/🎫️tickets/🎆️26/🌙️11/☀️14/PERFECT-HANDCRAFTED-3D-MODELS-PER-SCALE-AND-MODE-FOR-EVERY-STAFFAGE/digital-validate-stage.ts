/** 🧠 Inspect the resident footprint at actual stages of the catalog validation without changing its result. */
import {readFileSync} from 'node:fs';
import {join} from 'node:path';
Bun.plugin({name:'validation-stages',setup(b){b.onLoad({filter:/🗂️catalog\/🟦️\.ts$/},args=>{
 let s=readFileSync(args.path,'utf8');
 const label=(tag:string)=>'console.error("[DEBUG] stage '+tag+'",Math.round(process.memoryUsage().rss/1048576));';
 const line='  out.push(...volumeIssues(asset), ...(asset.volumes === undefined ? [] : analyticRules(asset)), ...exportIssues(asset));';
 if(!s.includes(line))throw Error(args.path);
 s=s.replace(line,'  '+label('model')+'out.push(...volumeIssues(asset));'+label('analytics')+'if(asset.volumes!==undefined)out.push(...analyticRules(asset));'+label('exports')+'out.push(...exportIssues(asset));');
 for(const [match,tag] of [['  for (const gap of uncoveredDenominators(asset))','band'],['  for (const [bone, { ids, where, thetaDeg, errorMm }]','lateral']])s=s.replace(match,'  '+label(tag)+'\n'+match);
 return {loader:'ts',contents:s};
});}});
Bun.plugin({name:'run-document-entry',setup(b){b.onLoad({filter:/r3-validate\.ts$/},args=>({loader:'ts',contents:readFileSync(args.path,'utf8').replace('if (import.meta.main)', 'if (true)')}));}});
await import(join(import.meta.dir,'r3-validate.ts'));
