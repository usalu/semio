import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {EventEmitter} from "node:events";
import {PassThrough} from "node:stream";
import {createInterface} from "node:readline";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import ts from "typescript";
import fixture from "./mcp-stage-coalescing-red.json";
test("same admitted product request joins one actual staging attempt (bounded recording ports)",async()=>{
 expect(new Ajv({strict:true}).validate({type:"object",additionalProperties:false,required:["requests","expectedBuilds"],properties:{expectedBuilds:{const:1},requests:{type:"array",minItems:2,items:{type:"object",additionalProperties:false,required:["root","capture","product","profile"],properties:{root:{type:"string"},capture:{type:"string"},product:{type:"string"},profile:{enum:["debug","release"]}}}}}},fixture)).toBe(true);
 const database=new Database(":memory:");database.exec("create table requests(root text,capture text,product text,profile text)");for(const row of fixture.requests)database.query("insert into requests values(?,?,?,?)").run(row.root,row.capture,row.product,row.profile);const oracle=(database.query("select count(*) count from(select distinct root,capture,product,profile from requests)").get()as{count:number}).count;database.close();expect(oracle).toBe(fixture.expectedBuilds);
 const path=join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts"),ast=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),names=["mcpStagingEnvironment","stageMcpBinaryAsync"],declarations=ast.statements.filter(node=>ts.isFunctionDeclaration(node)?names.includes(node.name?.text??""):ts.isVariableStatement(node)&&node.declarationList.declarations.some(row=>ts.isIdentifier(row.name)&&names.includes(row.name.text)));
 expect(declarations.length).toBe(2);const code=ts.transpileModule(declarations.map(row=>row.getText(ast).replace(/^export /u,"")).join("\n"),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;
 const calls:unknown[]=[];const start=(...args:unknown[])=>{calls.push(args);const child=Object.assign(new EventEmitter(),{stdout:new PassThrough(),stderr:new PassThrough()});queueMicrotask(()=>{child.stdout.end();child.stderr.end();child.emit("close",0);});return child;};
 const run=new Function("startToolV1","createInterface","process","CARGO_BUILD_OWNER_PID_ENV","MCP_BINARY_TARGET",code+";return stageMcpBinaryAsync;")(start,createInterface,{env:{},pid:7},"SEMIO_CARGO_BUILD_OWNER_PID","@semio-tech/framework-os-mcp-rs:build");
 await Promise.all(fixture.requests.map(row=>run(row.root,()=>{},new AbortController().signal)));expect(calls.length).toBe(oracle);
});
