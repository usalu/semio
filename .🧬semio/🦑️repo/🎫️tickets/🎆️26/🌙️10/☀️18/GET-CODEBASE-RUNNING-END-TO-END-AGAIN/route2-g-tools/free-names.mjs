/** 🔎 route2-g: lists the free (module-level) names an extracted function uses. usage: bun free-names.mjs <file> <functionName|ClassName.method>; a name resolved by a declaration inside the function is not free. */
import ts from "typescript";
import {readFileSync} from "node:fs";
const [file,target]=process.argv.slice(2),source=ts.createSourceFile(file,readFileSync(file,"utf8"),ts.ScriptTarget.Latest,true);
const globals=new Set("Error TypeError RangeError AggregateError Promise Map Set WeakMap WeakSet Object Array JSON Math Number String Boolean Symbol Date Buffer Uint8Array Int32Array BigInt process console URL AbortController AbortSignal setTimeout clearTimeout setImmediate queueMicrotask undefined NaN Infinity Reflect Proxy RegExp isNaN Atomics SharedArrayBuffer TextEncoder TextDecoder globalThis Bun structuredClone performance crypto".split(" "));
const find=(node)=>{let hit;const visit=(n)=>{if(hit)return;if((ts.isFunctionDeclaration(n)&&n.name?.text===target)||(ts.isVariableDeclaration(n)&&n.name.getText(source)===target&&n.initializer))hit=ts.isVariableDeclaration(n)?n.initializer:n;else ts.forEachChild(n,visit);};visit(node);return hit;};
const root=find(source);if(!root){console.log("not found");process.exit(1);}
const declared=new Set(),used=new Map();
const declare=(name)=>{if(ts.isIdentifier(name))declared.add(name.text);else if(ts.isObjectBindingPattern(name)||ts.isArrayBindingPattern(name))for(const e of name.elements)if(!ts.isOmittedExpression(e))declare(e.name);};
const walk=(n)=>{
 if(ts.isVariableDeclaration(n)||ts.isParameter(n))declare(n.name);
 if((ts.isFunctionDeclaration(n)||ts.isClassDeclaration(n)||ts.isFunctionExpression(n))&&n.name)declared.add(n.name.text);
 if(ts.isCatchClause(n)&&n.variableDeclaration)declare(n.variableDeclaration.name);
 if(ts.isIdentifier(n)){const p=n.parent;const isProp=(ts.isPropertyAccessExpression(p)&&p.name===n)||(ts.isPropertyAssignment(p)&&p.name===n)||(ts.isMethodDeclaration(p)&&p.name===n)||(ts.isPropertySignature(p)&&p.name===n)||(ts.isBindingElement(p)&&p.propertyName===n)||ts.isTypeReferenceNode(p)||ts.isLabeledStatement(p)||ts.isTypeQueryNode(p)||ts.isQualifiedName(p)||ts.isPropertyDeclaration(p)&&p.name===n||ts.isEnumMember(p);if(!isProp)used.set(n.text,(used.get(n.text)??0)+1);}
 ts.forEachChild(n,walk);};
walk(root);
console.log([...used.keys()].filter(name=>!declared.has(name)&&!globals.has(name)).sort().join("\n"));
