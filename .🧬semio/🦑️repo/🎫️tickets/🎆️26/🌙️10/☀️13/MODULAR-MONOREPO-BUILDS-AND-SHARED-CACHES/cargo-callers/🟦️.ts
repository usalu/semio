import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{resolve}from"node:path";
import ts from"typescript";
import Ajv from"ajv";
import fixture from"./fixture.json"with{type:"json"};
import schema from"./schema.json"with{type:"json"};
const root=resolve(import.meta.dir,"../../../../../../../..");
test("every selected repository Cargo test caller retains its complete Original invocation",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);let calls=0;
 for(const row of fixture.callers){const body=readFileSync(resolve(root,row.path),"utf8"),source=ts.createSourceFile(row.path,body,ts.ScriptTarget.Latest,true),found:ts.CallExpression[]=[];function visit(node:ts.Node){if(ts.isCallExpression(node)&&node.expression.getText(source)===fixture.callee)found.push(node);ts.forEachChild(node,visit);}visit(source);expect(found.length,row.path).toBe(row.calls);expect([...body.matchAll(/\brunRepositoryCargoTests\s*\(/gu)].length).toBe(row.calls);for(const call of found){expect(call.arguments[fixture.argument]?.getText(source),row.path).toBe(fixture.original);calls++;}}
 console.log(`[DEBUG] Complete Original invocation preserved by ${calls} actual Cargo test calls in ${fixture.callers.length} package scripts`);
});
