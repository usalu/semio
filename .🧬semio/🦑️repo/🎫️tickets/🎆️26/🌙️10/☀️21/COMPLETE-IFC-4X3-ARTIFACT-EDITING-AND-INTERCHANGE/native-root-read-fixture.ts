/** 🧪 Author native probe ASTs from the exact shared test-only EXPRESS procedures. */
import {resolve} from 'node:path';
const repo=resolve(import.meta.dir,'../../../../../../..'),v=resolve(repo,'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation');
const {ExpressParser,parseExpressExpression}=await import(resolve(v,'index.ts'));
const data=await Bun.file(resolve(v,'🧪️vector-arithmetic/isolation.json')).json(),arithmetic=await Bun.file(resolve(v,'🧪️vector-arithmetic/cases.json')).json();
const q=(x:unknown)=>x==null?'None':'Some('+JSON.stringify(x)+')',list=(x:unknown[],f:(x:any)=>string)=>'&['+x.map(f).join(',')+']';
function expr(x:any):string{return 'ExpressExpression{kind:ExpressExpressionKind::'+({integer:'Integer',real:'Real',string:'String',logical:'Logical',unknown:'Unknown',identifier:'Identifier',unary:'Unary',binary:'Binary',attribute:'Attribute',qualify:'Qualify',call:'Call',index:'Index',slice:'Slice',aggregate:'Aggregate',query:'Query',interval:'Interval'} as any)[x.kind]+',offset:'+x.offset+',text:'+q(x.text)+',operator:'+q(x.operator)+',name:'+q(x.name)+',operands:'+list(x.operands??[],expr)+',entries:'+list(x.entries??[],(a:any)=>'ExpressAggregateEntry{value:&'+expr(a.value)+',repeat:'+(a.repeat?'Some(&'+expr(a.repeat)+')':'None')+'}')+'}';}
function variable(x:any):string{return 'ExpressVariable{names:'+list(x.names,JSON.stringify)+',type_source:'+JSON.stringify(x.type)+',initial:'+(x.initial?'Some(&'+expr(x.initial)+')':'None')+'}';}
function statement(x:any):string{return 'ExpressStatement{kind:ExpressStatementKind::'+x.kind[0].toUpperCase()+x.kind.slice(1)+',offset:'+x.offset+',name:'+q(x.name)+',expressions:'+list(x.expressions??[],expr)+',body:'+list(x.body??[],statement)+',otherwise:'+list(x.otherwise??[],statement)+',cases:'+list(x.cases??[],(a:any)=>'ExpressCase{labels:'+list(a.labels,expr)+',body:'+list(a.body,statement)+'}')+',while_condition:'+(x.while?'Some(&'+expr(x.while)+')':'None')+',until_condition:'+(x.until?'Some(&'+expr(x.until)+')':'None')+'}';}
function fn(source:string){const p=new ExpressParser(source.replace(/END_FUNCTION;\s*$/,''));p.require('FUNCTION');return p.function(p.take().text);}
function func(x:any):string{return 'ExpressFunction{name:'+JSON.stringify(x.name)+',parameters:'+list(x.parameters,variable)+',result_type:'+JSON.stringify(x.resultType)+',locals:'+list(x.locals,variable)+',body:'+list(x.body,statement)+'}';}
let output='//! 🧪 Test-only procedure syntax; canonical issued PROGRAM arrays remain borrowed and unchanged.\nuse super::*;\n';
const canonical=Array.from({length:48},(_,i)=>'super::super::super::PROGRAM.functions['+i+']').join(',');
for(const [i,row]of data.cases.entries())output+='pub(super) static PROBE_'+i+':ExpressProgram=ExpressProgram{functions:&['+func(fn(row.source))+','+func(fn(data.helper))+'],..super::super::super::PROGRAM};\n';
output+='pub(super) static PROBES:&[&ExpressProgram]=&['+data.cases.map((_:any,i:number)=>'&PROBE_'+i).join(',')+'];\n';
output+='pub(super) static PROBE_EXPRESSIONS:&[ExpressExpression]='+list(data.cases,(row:any)=>expr(parseExpressExpression('PROBE('+row.args.map((id:string)=>'E'+id).join(',')+')')))+';\n';
output+='pub(super) static ARITHMETIC:&[ExpressExpression]='+list(arithmetic.functions,(row:any)=>expr(parseExpressExpression(row.owner+'('+row.args.map((a:any)=>a.kind==='entity'?'E'+a.id:a.kind==='unknown'?'?':a.value.includes('.')?a.value:a.value+'.0').join(',')+')')))+';\n';
const memo=await Bun.file(resolve(v,'root-memo-cases.json')).json();
for(const[i,row]of memo.cases.entries())output+='pub(super) static MEMO_'+i+':ExpressProgram=ExpressProgram{functions:&['+canonical+','+func(fn(row.procedure))+'],..super::super::super::PROGRAM};\n';
output+='pub(super) static MEMOS:&[&ExpressProgram]=&['+memo.cases.map((_:any,i:number)=>'&MEMO_'+i).join(',')+'];\n';
output+='pub(super) static MEMO_EXPRESSIONS:&[ExpressExpression]='+list(memo.cases,(row:any)=>expr(parseExpressExpression('PROBE('+row.binding+')')))+';\n';
output+='pub(super) static MEMO_BINDINGS:&[&[(&str,u64)]]=&['+memo.cases.map((row:any)=>'&[('+JSON.stringify(row.binding)+','+row.entity+')]')+'];\n';
await Bun.write(resolve(v,'machine-root-fixtures.rs'),output);
await Bun.write(resolve(import.meta.dir,'🗑️generated/native-root-read-fixture.json'),JSON.stringify({arithmetic:arithmetic.functions.length,procedures:data.cases.length,canonicalProgramChanged:false},null,2)+'\n');
console.log('[DEBUG] native fixture ASTs authored from shared contracts');

