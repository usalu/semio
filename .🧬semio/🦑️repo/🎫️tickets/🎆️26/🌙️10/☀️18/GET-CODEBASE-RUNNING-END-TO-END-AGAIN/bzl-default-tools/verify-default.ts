import {readFileSync} from "node:fs";
import {join} from "node:path";
import ts from "typescript";
const [root]=process.argv.slice(2);
const B=join(root!,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build");
const source=readFileSync(join(B,"🧭️invocation/🟦️.ts"),"utf8"),tree=ts.createSourceFile("invocation.ts",source,ts.ScriptTarget.Latest,true);
const declaration=tree.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text==="parseBuildInvocationV1")!;
const code=ts.transpileModule(declaration.getText(tree).replace(/^export\s+/u,""),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
const policy=JSON.parse(readFileSync(join(B,"🔣️policy.json"),"utf8")),readWorkerSwitchV1=(value:unknown)=>value===undefined||value===""?undefined:value==="on";
const parse=new Function("policy","readWorkerSwitchV1",code+";return parseBuildInvocationV1;")(policy,readWorkerSwitchV1) as (args:string[],environment:Record<string,string|undefined>)=>{tool:string};
const rows:[string[],Record<string,string>,string][]=[[["run","a:b"],{},policy.defaultTool],[["run","a:b"],{SEMIO_BUILD_TOOL:"nx"},"nx"],[["run","a:b"],{SEMIO_BUILD_TOOL:"bazel"},"bazel"],[["--tool=nx","run","a:b"],{SEMIO_BUILD_TOOL:"bazel"},"nx"],[["--tool=bazel","run","a:b"],{SEMIO_BUILD_TOOL:"nx"},"bazel"],[["run","a:b","--","--tool=nx"],{},policy.defaultTool],[["tools","setup"],{},policy.defaultTool]];
const failures=rows.filter(([args,environment,expected])=>parse(args,environment).tool!==expected).map(([args])=>args.join(" "));
let unknownRefused=false;try{parse(["--tool=other"],{});}catch{unknownRefused=true;}
console.log(JSON.stringify({defaultTool:policy.defaultTool,rows:rows.length,failures,unknownRefused}));
process.exit(failures.length||!unknownRefused?1:0);
