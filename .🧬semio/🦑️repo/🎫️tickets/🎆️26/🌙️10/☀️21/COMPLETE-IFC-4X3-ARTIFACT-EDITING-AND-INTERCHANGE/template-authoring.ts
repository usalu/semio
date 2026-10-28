import {mkdir,writeFile} from 'node:fs/promises';
import {join} from 'node:path';
import {fixture,retained,s,a,r,real,entity} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️tests/template-fixtures.ts';
import cases from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏷️templates/✍️apply/cases.json';
import {ifc4x3Template,ifc4x3TemplateCatalog} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏷️templates/🟦️.ts';
import {applyIfcMutationAsync} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts';
import {writeIfc4x3Step} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/step.ts';
const output=join(import.meta.dir,'🗑️generated','template-authoring');await mkdir(output,{recursive:true});
const rows:any[]=[];
for(const row of cases.carriers){
 const source=fixture(row.target),entry=ifc4x3Template(row.template)!.entries.find(e=>e.name===row.entry)!;let value:any=entry.dataType==='IfcIdentifier'||entry.dataType==='IfcLabel'?s('retained'):real(1);
 if(row.form==='P_REFERENCEVALUE'){source.entities.push(entity('IfcIrregularTimeSeriesValue',11n,{TimeStamp:s('2026-10-21T12:00:00'),ListValues:a([{kind:'typedValue',value:{name:'IFCREAL',items:[real(1)]}}])}),entity('IfcIrregularTimeSeries',12n,{Name:s('Position'),StartTime:s('2026-10-21T12:00:00'),EndTime:s('2026-10-21T13:00:00'),TimeSeriesDataType:{kind:'enum',value:'CONTINUOUS'},DataOrigin:{kind:'enum',value:'MEASURED'},Values:a([r(11n)])}),entity('IfcDocumentReference',13n,{Name:s('Series specification'),Location:s('https://example.org/series')}),entity('IfcExternalReferenceRelationship',14n,{RelatingReference:r(13n),RelatedResourceObjects:a([r(12n)])}));value=r(12n);}
 rows.push({source,application:await retained(source,{template:row.template,target:1n,entries:[{name:entry.name,kind:row.form as any,value}],...(row.form==='P_REFERENCEVALUE'?{performance:{lifeCyclePhase:'OPERATION',predefinedType:'NOTDEFINED' as const}}:{})})});
}
for(const form of ['P_ENUMERATEDVALUE','P_LISTVALUE','P_BOUNDEDVALUE','P_TABLEVALUE','Q_LENGTH','Q_VOLUME','Q_COUNT','Q_TIME']){
 const template=ifc4x3TemplateCatalog().templates.find(t=>t.entries.some(e=>e.kind===form))!,entry=template.entries.find(e=>e.kind===form)!,target=template.classes[0]!.split('/')[0]!,source=fixture(target==='IfcProduct'?'IfcBuildingElementProxy':target);let selected:any={name:entry.name,kind:form};
 if(entry.kind==='P_ENUMERATEDVALUE')selected.values=[s(entry.enumeration.literals[0]!)];else if(form==='P_LISTVALUE')selected.values=[s('line one'),s('line two')];else if(form==='P_BOUNDEDVALUE'){selected.lower=real(1);selected.upper=real(2);selected.setPoint=real(1.5);}else if(form==='P_TABLEVALUE'){selected.defining=[real(1),real(2)];selected.defined=[real(3),real(4)];selected.interpolation='LINEAR';selected.expression='x';}else selected.value=form==='Q_COUNT'?{kind:'integer',value:1n}:real(1);
 rows.push({source,application:await retained(source,{template:template.name,target:1n,entries:[selected],...(template.kind==='PSET_PERFORMANCEDRIVEN'?{performance:{lifeCyclePhase:'OPERATION',predefinedType:'NOTDEFINED' as const}}:{})})});
}
const results=[];
for(let index=0;index<rows.length;index++){const row=rows[index]!,name=String(index).padStart(2,'0'),snapshot=await applyIfcMutationAsync(row.source,{mutation:'applyTemplate',application:row.application});await writeFile(join(output,`${name}-source.ifc`),await writeIfc4x3Step(row.source));await writeFile(join(output,`${name}-semio.ifc`),await writeIfc4x3Step(snapshot));results.push({name,application:row.application,template:ifc4x3Template(row.application.template)});}
await writeFile(join(output,'cases.json'),JSON.stringify(results,(_,value)=>typeof value==='bigint'?String(value):value,2));
console.log(JSON.stringify({cases:results.length,output}));
