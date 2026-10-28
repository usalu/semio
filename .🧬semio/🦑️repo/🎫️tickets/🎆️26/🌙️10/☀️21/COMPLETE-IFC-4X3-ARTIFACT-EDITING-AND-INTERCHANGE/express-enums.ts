import models from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/models.json';
import {ifc4x3Catalog} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
import {compileIfc4x3Express,IfcExpressContext} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/index.ts';
import {parseIfc4x3Step} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/step.ts';
const declarations=Object.entries(ifc4x3Catalog.entities).filter(([name,entity])=>!entity.abstractEntity&&entity.allAttributes.some(attribute=>attribute.name==='PredefinedType'&&ifc4x3Catalog.types[attribute.typeExpr.name!]?.members.includes('USERDEFINED')));
const body=declarations.map(([name,entity],index)=>`#${index+1}=${name}(${entity.allAttributes.map(attribute=>attribute.derived?'*':attribute.name==='PredefinedType'?'.USERDEFINED.':attribute.name==='GlobalId'?`'${String(index+1).padStart(22,'0')}'`:'$').join(',')});`).join('\n');
const source=models.header+body+models.footer;await Bun.write(new URL('./🗑️generated/enums.ifc',import.meta.url),source);
const program=(await compileIfc4x3Express()).program,context=new IfcExpressContext(await parseIfc4x3Step(source),program);await context.initialize();
const diagnostics:any[]=[];
for(const [id,value] of context.entities)for(const rule of program.rules)if(rule.label==='CORRECTPREDEFINEDTYPE'&&(value.types??[]).includes(rule.owner)){const result=await context.evaluator.evaluate(rule.expression,context.scope(value));if(result.kind==='logical'&&result.value==='false')diagnostics.push({code:'express.rule',owner:rule.owner,rule:rule.label,entityId:id});}
await Bun.write(new URL('./🗑️generated/express-enums.json',import.meta.url),JSON.stringify({scope:'isolated CorrectPredefinedType clauses; other mandatory domains deliberately unset',diagnostics},(_,v)=>typeof v==='bigint'?String(v):v));console.log(declarations.length,diagnostics.length);
