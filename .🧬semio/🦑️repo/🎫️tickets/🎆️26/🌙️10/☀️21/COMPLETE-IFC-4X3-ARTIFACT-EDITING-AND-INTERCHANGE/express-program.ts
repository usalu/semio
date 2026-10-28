/** 🦀 Retain a schema-admitted canonical EXPRESS program as immutable native data. */
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
import {readFile,writeFile,mkdir} from "node:fs/promises";
import {createHash} from "node:crypto";
import Ajv from "ajv";
import assert from "node:assert/strict";

const root=resolve(import.meta.dir,"../../../../../../.."),base=resolve(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation");
const {compileIfc4x3Express}=await import(pathToFileURL(resolve(base,"compiler.ts")).href),compiled=await compileIfc4x3Express();
assert.equal(compiled.complete,true,JSON.stringify(compiled.diagnostics));
const program=compiled.program,schema=JSON.parse(await readFile(resolve(base,"ast.schema.json"),"utf8")),admit=new Ajv({strict:true,allErrors:true}).compile(schema);
assert.equal(admit(program),true,JSON.stringify(admit.errors));
const keys=(value:Record<string,unknown>,allowed:string[])=>{for(const key of Object.keys(value))assert.ok(allowed.includes(key),`Unmapped AST field ${key}`);};
const quoted=(value:string)=>JSON.stringify(value);
const optional=(value:string|undefined)=>value===undefined?"None":`Some(${quoted(value)})`;
const list=<T>(values:T[]|undefined,emit:(value:T)=>string)=>`&[${(values??[]).map(emit).join(",")}]`;
const names=(values:string[])=>list(values,quoted);
const kind=(value:string)=>value[0]!.toUpperCase()+value.slice(1);
const maybeExpr=(value:any)=>value?`Some(&${expression(value)})`:"None";
function expression(node:any):string{
 keys(node,["kind","offset","text","operator","name","operands","entries"]);
 return `ExpressExpression {kind: ExpressExpressionKind::${kind(node.kind)},offset:${node.offset},text:${optional(node.text)},operator:${optional(node.operator)},name:${optional(node.name)},operands:${list(node.operands,expression)},entries:${list(node.entries,(row:any)=>{keys(row,["value","repeat"]);return `ExpressAggregateEntry {value:&${expression(row.value)},repeat:${maybeExpr(row.repeat)}}`;})}}`;
}
function statement(node:any):string{
 keys(node,["kind","offset","name","expressions","body","otherwise","cases","while","until"]);
 return `ExpressStatement {kind: ExpressStatementKind::${kind(node.kind)},offset:${node.offset},name:${optional(node.name)},expressions:${list(node.expressions,expression)},body:${list(node.body,statement)},otherwise:${list(node.otherwise,statement)},cases:${list(node.cases,(row:any)=>{keys(row,["labels","body"]);return `ExpressCase {labels:${list(row.labels,expression)},body:${list(row.body,statement)}}`;})},while_condition:${maybeExpr(node.while)},until_condition:${maybeExpr(node.until)}}`;
}
function variable(node:any):string{
 keys(node,["names","type","initial"]);
 return `ExpressVariable {names:${names(node.names)},type_source:${quoted(node.type)},initial:${maybeExpr(node.initial)}}`;
}
function rule(node:any):string{
 keys(node,["owner","label","expression"]);
 return `ExpressRule {owner:${quoted(node.owner)},label:${quoted(node.label)},expression:&${expression(node.expression)}}`;
}
function fn(node:any):string{
 keys(node,["name","parameters","resultType","locals","body"]);
 return `ExpressFunction {name:${quoted(node.name)},parameters:${list(node.parameters,variable)},result_type:${quoted(node.resultType)},locals:${list(node.locals,variable)},body:${list(node.body,statement)}}`;
}
function global(node:any):string{
 keys(node,["name","populations","locals","body","rules"]);
 return `ExpressGlobalRule {name:${quoted(node.name)},populations:${names(node.populations)},locals:${list(node.locals,variable)},body:${list(node.body,statement)},rules:${list(node.rules,rule)}}`;
}
function inverse(node:any):string{
 keys(node,["owner","name","type","attribute","aggregate","lower","upper"]);
 return `ExpressInverse {owner:${quoted(node.owner)},name:${quoted(node.name)},entity_type:${quoted(node.type)},attribute:${quoted(node.attribute)},aggregate:${!!node.aggregate},lower:${maybeExpr(node.lower)},upper:${maybeExpr(node.upper)}}`;
}
keys(program,["schema","functions","globalRules","rules","derived","unique","bounds","inverse"]);
const json=JSON.stringify(program),digest=createHash("sha256").update(json).digest("hex"),native=[
 "/// 📚️ Schema-admitted canonical ADD2 executable syntax retained as immutable native data.",
 `pub const EXPRESS_PROGRAM_SHA256: &str = ${quoted(digest)};`,
 `static PROGRAM: ExpressProgram = ExpressProgram {schema:${quoted(program.schema)},functions:${list(program.functions,fn)},global_rules:${list(program.globalRules,global)},rules:${list(program.rules,rule)},derived:${list(program.derived,rule)},unique:${list(program.unique,rule)},bounds:${list(program.bounds,rule)},inverse:${list(program.inverse,inverse)}};`
].join("\n")+"\n";
await writeFile(resolve(base,"program.rs"),native);
const generated=resolve(import.meta.dir,"🗑️generated");await mkdir(generated,{recursive:true});await writeFile(resolve(generated,"express-program.json"),json+"\n");
console.log(JSON.stringify({sha256:digest,bytes:native.length,rules:program.rules.length,derived:program.derived.length,functions:program.functions.length,globalRules:program.globalRules.length,inverse:program.inverse.length,bounds:program.bounds.length,unique:program.unique.length}));
