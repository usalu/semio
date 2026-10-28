import ts from "typescript";
import {readInferenceGraphSourceV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/📖️source/🟦️.ts";
const text=readInferenceGraphSourceV1();
const file=ts.createSourceFile("inference.mjs",text,ts.ScriptTarget.Latest,true),calls:ts.CallExpression[]=[];
const visit=(node:ts.Node)=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression)&&["observeProjectInferenceV1","observeInference"].includes(node.expression.text))calls.push(node);ts.forEachChild(node,visit);};visit(file);
console.log(calls.length);for(const c of calls){const o=c.arguments[0];console.log(file.getLineAndCharacterOfPosition(c.getStart(file)).line+1,o&&ts.isObjectLiteralExpression(o)?o.properties.map(p=>p.name?.getText(file)).join(","):o?.getText(file).slice(0,80));}
const entry=file.statements.find((n):n is ts.FunctionDeclaration=>ts.isFunctionDeclaration(n)&&n.name?.text==="invokeCurrentImplementation");console.log(entry?.body?.statements[0]?.getText(file),entry?.body?.statements[1]?.getText(file));
const lines=text.split("\n");for(const n of [3552,3654,4091])console.log(n,lines[n-1].slice(0,700));
