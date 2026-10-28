import {writeFile} from 'node:fs/promises';
const base='file:///C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/';
const {ExpressEvaluator}=await import(base+'evaluator.ts');
const {parseExpressExpression}=await import(base+'parser.ts');
const rows=[];
for(const a of ['FALSE','UNKNOWN','TRUE'])for(const b of ['FALSE','UNKNOWN','TRUE'])for(const op of ['AND','OR','XOR']){const source=`${a} ${op} ${b}`;rows.push({source,result:await new ExpressEvaluator().evaluate(parseExpressExpression(source))});}
for(const source of ['UNKNOWN = UNKNOWN','? = ?','SIZEOF(QUERY(x <* [1,2,3] | UNKNOWN))','SIZEOF(QUERY(x <* [1,2,3] | x > 1))',"'can''t' = 'can''t'",'SIZEOF([1:3])','9007199254740993 > 9007199254740992.'])rows.push({source,result:await new ExpressEvaluator().evaluate(parseExpressExpression(source))});
await writeFile(new URL('./🗑️generated/retained-logical-probe.json',import.meta.url),JSON.stringify({producer:'reference-typescript',nativeExecution:false,rows},(_,v)=>typeof v==='bigint'?v.toString():v,2));
console.log(JSON.stringify({rows:rows.length,tail:rows.slice(27)},(_,v)=>typeof v==='bigint'?v.toString():v));

const {compileIfc4x3Express}=await import(base+'compiler.ts');
const compilation=await compileIfc4x3Express({batchSize:1000000});
const kinds={},ops={},examples={};
const walk=(value,owner='')=>{if(!value||typeof value!=='object')return;if(Array.isArray(value)){for(const item of value)walk(item,owner);return;}owner=value.owner??owner;if(value.kind){kinds[value.kind]=(kinds[value.kind]??0)+1;if(value.operator){ops[value.operator]=(ops[value.operator]??0)+1;(examples[value.operator]??=[]);if(examples[value.operator].length<3)examples[value.operator].push({owner,offset:value.offset});}}for(const v of Object.values(value))if(v&&typeof v==='object')walk(v,owner);};walk(compilation.program);
await writeFile(new URL('./🗑️generated/retained-logical-program.json',import.meta.url),JSON.stringify({complete:compilation.complete,kinds,ops,examples},null,2));
console.log(JSON.stringify({programComplete:compilation.complete,kinds,ops}));
