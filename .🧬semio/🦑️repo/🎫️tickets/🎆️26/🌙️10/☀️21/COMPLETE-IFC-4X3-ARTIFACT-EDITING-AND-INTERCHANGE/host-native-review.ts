/** 🔬 Independently read native literal projections and compare every retained canonical field. */
import {readFile} from "node:fs/promises";
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
import assert from "node:assert/strict";
const root=resolve(import.meta.dir,"../../../../../../.."),base=resolve(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema");
function literal(source:string){
 const tokens=source.match(/"(?:\\.|[^"\\])*"|[A-Za-z_][A-Za-z_0-9]*|[0-9]+|::|[^\s]/g)!;let at=0;
 const take=()=>tokens[at++]!,expect=(value:string)=>assert.equal(take(),value);
 function value():any{
  const token=take();
  if(token==="&")return value();
  if(token==="["){const result=[];while(tokens[at]!=="]"){result.push(value());if(tokens[at]===",")take();else break;}expect("]");return result;}
  if(token==="Some"){expect("(");const result=value();expect(")");return result;}
  if(token==="None")return null;
  if(token==="true"||token==="false")return token==="true";
  if(token.startsWith('"'))return JSON.parse(token);
  if(/^\d+$/.test(token))return Number(token);
  if(tokens[at]==="::"){take();return take();}
  expect("{");const result:any={$type:token};while(tokens[at]!=="}"){const name=take();expect(":");result[name]=value();if(tokens[at]===",")take();else break;}expect("}");return result;
 }
 const result=value();assert.equal(at,tokens.length);return result;
}
const native=await readFile(resolve(base,"⚖️validation/program.rs"),"utf8"),actual=literal(native.slice(native.indexOf("= ExpressProgram")+2).trim().replace(/;$/,""));
const {compileIfc4x3Express}=await import(pathToFileURL(resolve(base,"⚖️validation/compiler.ts")).href),compiled=await compileIfc4x3Express();assert.deepEqual(compiled.diagnostics,[]);
const cap=(value:string)=>value[0]!.toUpperCase()+value.slice(1),arr=(rows:any[]|undefined,fn:(row:any)=>any)=>(rows??[]).map(fn),maybe=(value:any,fn:(row:any)=>any)=>value===undefined?null:fn(value);
const ex=(n:any):any=>({$type:"ExpressExpression",kind:cap(n.kind),offset:n.offset,text:n.text??null,operator:n.operator??null,name:n.name??null,operands:arr(n.operands,ex),entries:arr(n.entries,r=>({$type:"ExpressAggregateEntry",value:ex(r.value),repeat:maybe(r.repeat,ex)}))});
const st=(n:any):any=>({$type:"ExpressStatement",kind:cap(n.kind),offset:n.offset,name:n.name??null,expressions:arr(n.expressions,ex),body:arr(n.body,st),otherwise:arr(n.otherwise,st),cases:arr(n.cases,r=>({$type:"ExpressCase",labels:arr(r.labels,ex),body:arr(r.body,st)})),while_condition:maybe(n.while,ex),until_condition:maybe(n.until,ex)});
const variable=(n:any)=>({$type:"ExpressVariable",names:n.names,type_source:n.type,initial:maybe(n.initial,ex)});
const rule=(n:any)=>({$type:"ExpressRule",owner:n.owner,label:n.label,expression:ex(n.expression)});
const p=compiled.program,expected={$type:"ExpressProgram",schema:p.schema,functions:p.functions.map((n:any)=>({$type:"ExpressFunction",name:n.name,parameters:arr(n.parameters,variable),result_type:n.resultType,locals:arr(n.locals,variable),body:arr(n.body,st)})),global_rules:p.globalRules.map((n:any)=>({$type:"ExpressGlobalRule",name:n.name,populations:n.populations,locals:arr(n.locals,variable),body:arr(n.body,st),rules:arr(n.rules,rule)})),rules:p.rules.map(rule),derived:p.derived.map(rule),unique:p.unique.map(rule),bounds:p.bounds.map(rule),inverse:p.inverse.map((n:any)=>({$type:"ExpressInverse",owner:n.owner,name:n.name,entity_type:n.type,attribute:n.attribute,aggregate:!!n.aggregate,lower:maybe(n.lower,ex),upper:maybe(n.upper,ex)}))};
assert.deepEqual(actual,expected);
const mvd=JSON.parse(await readFile(resolve(base,"👁️mvd/🔣️.json"),"utf8")),rust=await readFile(resolve(base,"👁️mvd/data.rs"),"utf8");
for(const name of ["entities","concepts"]){
 const marker=`const ${name.toUpperCase()}: &[IfcMvdEntry] = `,start=rust.indexOf(marker)+marker.length,end=rust.indexOf(";",start),rows=literal(rust.slice(start,end));
 const canonical=mvd[name].map((n:any)=>({$type:"IfcMvdEntry",id:n.id,name:n.name,href:n.href,universal:n.universal,reference:n.views.ReferenceView??null,alignment:n.views["Alignment-basedReferenceView"]??null})).sort((a:any,b:any)=>a.id<b.id?-1:a.id>b.id?1:0);
 assert.deepEqual(rows,canonical);
}
console.log(JSON.stringify({nativeProgramExact:true,nativeMvdRowsExact:1188}));
