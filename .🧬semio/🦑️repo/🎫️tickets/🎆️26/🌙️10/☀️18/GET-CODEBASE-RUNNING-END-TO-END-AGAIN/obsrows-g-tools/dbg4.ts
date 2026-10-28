import ts from "typescript";import {readFileSync} from "node:fs";
import {inferenceSourceFilesV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/📖️source/🟦️.ts";
for(const f of inferenceSourceFilesV1()){const text=readFileSync(f.path,"utf8");if(!/observeProjectInferenceV1|observeInference\b/.test(text))continue;
const file=ts.createSourceFile("x.mjs",text,ts.ScriptTarget.Latest,true);let n=0;const visit=(node:ts.Node)=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression)&&["observeProjectInferenceV1","observeInference"].includes(node.expression.text))n++;ts.forEachChild(node,visit);};visit(file);console.log(f.name,f.path.slice(-60),n);}
