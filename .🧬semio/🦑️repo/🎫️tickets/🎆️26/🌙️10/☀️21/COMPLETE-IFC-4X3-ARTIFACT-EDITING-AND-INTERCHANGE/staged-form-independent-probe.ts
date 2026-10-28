import {readFile,writeFile} from 'node:fs/promises';
const base='file:///C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/';
const {produceIfcTemplateStagedForm}=await import(base+'🧬️schema/🏷️templates/✍️apply/staging.ts');
const {parseIfc4x3Step}=await import(base+'🚪️io/step.ts');
const {stagedFormInvalidFields}=await import('file:///C:/git/semio/🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/🟦️.ts');
const cases=JSON.parse(await readFile(new URL('🧬️schema/🏷️templates/✍️apply/native-cases.json',base),'utf8')).cases,rows=[];
for(const row of cases){const source=await parseIfc4x3Step(row.source),request={token:{action:'applyIfcTemplate',generation:'1',revision:'rev:1',window:'details'},args:row.application,locale:'en'},response=await produceIfcTemplateStagedForm(source,'rev:1',request,{signal:new AbortController().signal});rows.push({id:row.id,template:row.application.template,kinds:row.application.entries.map(entry=>entry.kind),ready:response.ready,invalid:stagedFormInvalidFields(request.args,response)});}
await writeFile(new URL('./🗑️generated/staged-form-independent-probe.json',import.meta.url),JSON.stringify({mounted:false,rows},null,2));console.log('[DEBUG] '+JSON.stringify(rows.filter(row=>row.invalid.length)));
const bounded=cases.find(row=>row.application.entries.some(entry=>entry.kind==='P_BOUNDEDVALUE'));
const args=structuredClone(bounded.application);delete args.entries[0].upper;delete args.entries[0].setPoint;
const source=await parseIfc4x3Step(bounded.source),request={token:{action:'applyIfcTemplate',generation:'1',revision:'rev:1',window:'details'},args,locale:'en'},response=await produceIfcTemplateStagedForm(source,'rev:1',request,{signal:new AbortController().signal});
const optionalResult={mounted:false,template:args.template,ready:response.ready,upperPresent:Object.hasOwn(args.entries[0],'upper'),setPointPresent:Object.hasOwn(args.entries[0],'setPoint'),invalid:stagedFormInvalidFields(args,response)};
await writeFile(new URL('./🗑️generated/staged-form-optional-probe.json',import.meta.url),JSON.stringify(optionalResult,null,2));console.log('[DEBUG] '+JSON.stringify(optionalResult));
const {ifc4x3Template}=await import(base+'🧬️schema/🏷️templates/🟦️.ts');
const domain=ifc4x3Template(args.template).entries.find(entry=>entry.name===args.entries[0].name).dataType;
args.entries[0].lower={kind:'typedValue',value:{name:domain.toUpperCase(),items:[args.entries[0].lower]}};
const normalized=await produceIfcTemplateStagedForm(source,'rev:1',request,{signal:new AbortController().signal});
const normalizedResult={mounted:false,template:args.template,lowerDomain:domain,ready:normalized.ready,invalid:stagedFormInvalidFields(args,normalized)};
await writeFile(new URL('./🗑️generated/staged-form-optional-wrapped-probe.json',import.meta.url),JSON.stringify(normalizedResult,null,2));console.log('[DEBUG] '+JSON.stringify(normalizedResult));
