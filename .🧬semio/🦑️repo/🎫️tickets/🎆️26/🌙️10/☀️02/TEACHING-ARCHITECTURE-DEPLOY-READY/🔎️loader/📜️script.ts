import {test,expect} from "bun:test";
import{readFileSync}from"node:fs";
import{join,resolve}from"node:path";
import{createRequire}from"node:module";
const root=process.cwd(),require=createRequire(join(root,"package.json")),ts=require("typescript"),owner=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers"),file=join(owner,"🧪️tests/🟦️.ts"),source=readFileSync(file,"utf8"),tree=ts.createSourceFile(file,source,ts.ScriptTarget.Latest,true),label="immutable graph loader admits current captured manifest resolutions",node=tree.statements.find((node:any)=>ts.isExpressionStatement(node)&&ts.isCallExpression(node.expression)&&node.expression.arguments[0]?.text===label);
if(!node)throw Error("Actual permanent loader test required");
const ports={expect,require,Ajv:require("ajv").default,Database:require("bun:sqlite").Database,...require("node:fs/promises"),createHash:require("node:crypto").createHash,schema:JSON.parse(readFileSync(join(owner,"🧬️schema/🔣️.json"),"utf8")),fixture:JSON.parse(readFileSync(join(owner,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8"))},code=ts.transpileModule("async function run()"+node.expression.arguments[1].body.getText(tree)+";return run();",{compilerOptions:{target:ts.ScriptTarget.ESNext}}).outputText;
process.env.SEMIO_TEST_ARTIFACT_DIR=resolve(import.meta.dir,"../🗑️generated/loader-inspection");
test(label,()=>new Function(...Object.keys(ports),code)(...Object.values(ports)),30000);
