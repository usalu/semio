import ts from "typescript";
import {existsSync,readdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {gitSyncV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import rulesDocument from "./🔣️.json" with {type:"json"};
import rulesSchema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📒️ The language-agnostic rules of the test-store-isolation law. @see ./🧬️schema/🔣️.json */
export type TestStoreRulesV1=Readonly<{version:1;roots:readonly string[];testPath:string;sourceExtensions:readonly string[];checkoutCalls:readonly string[];checkoutEnvironment:readonly string[];storeCalls:readonly string[];storeLiterals:readonly string[];exceptions:readonly Readonly<{path:string;reason:Readonly<{en:string;de:string}>}>[]}>;
/** 🚩️ One place a test reaches the checkout's live store: `checkout-store-call` passes the checkout to a store call, `checkout-store-path` joins the checkout with a live store directory. */
export type TestStoreFindingV1=Readonly<{path:string;line:number;rule:"checkout-store-call"|"checkout-store-path";text:string}>;
/** 🧾️ The verdict over the repository: scanned test files, findings and exception rows that no longer match a finding. */
export type TestStoreReportV1=Readonly<{scanned:number;findings:readonly TestStoreFindingV1[];stale:readonly string[]}>;

/** 📋️ Validates the rules document against its schema and its regular expressions. */
export function readTestStoreRulesV1(value:unknown):TestStoreRulesV1{
 const problems=validateJsonSchemaSubset(rulesSchema,value);if(problems.length)throw Error("Test store isolation rules invalid: "+JSON.stringify(problems).slice(0,400));
 const rules=value as TestStoreRulesV1;for(const pattern of[rules.testPath,...rules.storeCalls,...rules.storeLiterals])new RegExp(pattern,"u");
 return rules;
}
export const TEST_STORE_RULES_V1=readTestStoreRulesV1(rulesDocument);
/** 🏷️ The texts a callee is matched by: the whole dotted path of a plain property chain and its last identifier, through parentheses and both branches of a conditional. */
const calleeTextV1=(node:ts.Expression):readonly string[]=>ts.isParenthesizedExpression(node)?calleeTextV1(node.expression):ts.isConditionalExpression(node)?[...calleeTextV1(node.whenTrue),...calleeTextV1(node.whenFalse)]:[node.getText().replace(/\s+/gu,""),ts.isPropertyAccessExpression(node)?node.name.text:ts.isIdentifier(node)?node.text:""];
/** 🛤️ Calls whose result is a path inside or around their path argument, so a checkout argument makes the result the checkout. */
const PATH_CALLS_V1=new Set(["join","resolve","normalize","realpathSync","dirname"]);
/** 🔗️ The declaration an identifier is bound to: the nearest enclosing parameter, catch variable or variable declared in an enclosing block, loop or source file; `undefined` for a global. */
function bindingV1(identifier:ts.Identifier):ts.Node|undefined{
 const name=identifier.text,named=(node:ts.Node|undefined)=>node!==undefined&&(ts.isVariableDeclaration(node)||ts.isParameter(node))&&ts.isIdentifier(node.name)&&node.name.text===name?node:undefined,list=(declarations:ts.VariableDeclarationList|undefined)=>declarations?.declarations.find(declaration=>named(declaration));
 for(let scope:ts.Node|undefined=identifier.parent;scope;scope=scope.parent){
  if(ts.isFunctionLike(scope)){const parameter=scope.parameters.find(candidate=>named(candidate));if(parameter)return parameter;}
  if(ts.isCatchClause(scope)&&named(scope.variableDeclaration))return scope.variableDeclaration;
  if((ts.isForStatement(scope)||ts.isForOfStatement(scope)||ts.isForInStatement(scope))&&scope.initializer&&ts.isVariableDeclarationList(scope.initializer)){const found=list(scope.initializer);if(found)return found;}
  if(ts.isBlock(scope)||ts.isSourceFile(scope)||ts.isModuleBlock(scope)||ts.isCaseClause(scope)||ts.isDefaultClause(scope))for(const statement of scope.statements)if(ts.isVariableStatement(statement)){const found=list(statement.declarationList);if(found)return found;}
 }
 return undefined;
}
/** 🔎️ Every place one source reaches the checkout's live store (a path is reported at the innermost `join`/`resolve` call, template or `+` chain that joins both); a `resolve`/`join` of the file's own directory with exactly as many `..` as it lies deep is the checkout too; an identifier bound to a variable whose initial value is a checkout path (the reference itself, a `join`/`resolve`/`normalize`/`realpathSync`/`dirname` of one, or a branch of `?:`, `??`, `||` that is one) counts as the checkout. @see https://github.com/microsoft/TypeScript/wiki/Using-the-Compiler-API */
export function scanTestStoreReachesV1(source:string,rules:TestStoreRulesV1=TEST_STORE_RULES_V1,path="source.ts"):readonly Omit<TestStoreFindingV1,"path">[]{
 const file=ts.createSourceFile(path,source,ts.ScriptTarget.Latest,true,path.endsWith(".tsx")?ts.ScriptKind.TSX:/\.m?js$/u.test(path)?ts.ScriptKind.JS:ts.ScriptKind.TS),calls=new Set(rules.checkoutCalls),environment=new Set(rules.checkoutEnvironment),store=rules.storeCalls.map(pattern=>new RegExp(pattern,"u")),literals=rules.storeLiterals.map(pattern=>new RegExp(pattern,"u")),found=new Map<string,Omit<TestStoreFindingV1,"path">>();
 const depth=path.split("/").length-1,upward=(node:ts.CallExpression)=>{const parts=node.arguments.slice(1).map(argument=>ts.isStringLiteralLike(argument)?argument.text:"x").join("/").split("/").filter(Boolean);return parts.length===depth&&parts.every(part=>part==="..");};
 const checkout=(node:ts.Node):boolean=>ts.isCallExpression(node)&&calleeTextV1(node.expression).some(text=>calls.has(text))||ts.isCallExpression(node)&&node.arguments.length>1&&calleeTextV1(node.expression).some(name=>name==="resolve"||name==="join")&&/^(import\.meta\.(dir|dirname)|__dirname)$/u.test(node.arguments[0]!.getText())&&upward(node)||ts.isPropertyAccessExpression(node)&&environment.has(node.name.text)&&/(^|\.)env$/u.test(node.expression.getText())||ts.isElementAccessExpression(node)&&ts.isStringLiteralLike(node.argumentExpression)&&environment.has(node.argumentExpression.text)||ts.isIdentifier(node)&&!(ts.isPropertyAccessExpression(node.parent)&&node.parent.name===node)&&!(ts.isPropertyAssignment(node.parent)&&node.parent.name===node)&&tainted.has(bindingV1(node)!);
 const literal=(node:ts.Node):boolean=>(ts.isStringLiteralLike(node)||ts.isTemplateHead(node)||ts.isTemplateMiddle(node)||ts.isTemplateTail(node))&&literals.some(pattern=>pattern.test(node.text));
 const contains=(node:ts.Node,predicate:(node:ts.Node)=>boolean):boolean=>predicate(node)||ts.forEachChild(node,child=>contains(child,predicate)||undefined)===true,value=(node:ts.Node):boolean=>checkout(node)||!ts.isFunctionLike(node)&&ts.forEachChild(node,child=>value(child)||undefined)===true,checkoutPath=(node:ts.Expression):boolean=>checkout(node)||(ts.isParenthesizedExpression(node)||ts.isNonNullExpression(node)||ts.isAsExpression(node)||ts.isAwaitExpression(node)||ts.isSatisfiesExpression(node))&&checkoutPath(node.expression)||ts.isConditionalExpression(node)&&(checkoutPath(node.whenTrue)||checkoutPath(node.whenFalse))||ts.isBinaryExpression(node)&&[ts.SyntaxKind.QuestionQuestionToken,ts.SyntaxKind.BarBarToken].includes(node.operatorToken.kind)&&(checkoutPath(node.left)||checkoutPath(node.right))||ts.isCallExpression(node)&&calleeTextV1(node.expression).some(name=>PATH_CALLS_V1.has(name))&&node.arguments.some(argument=>checkoutPath(argument)),both=(node:ts.Node)=>contains(node,literal)&&contains(node,checkout),joins=(node:ts.Node)=>ts.isCallExpression(node)&&calleeTextV1(node.expression).some(name=>name==="join"||name==="resolve")||ts.isTemplateExpression(node)||ts.isBinaryExpression(node)&&node.operatorToken.kind===ts.SyntaxKind.PlusToken,inner=(node:ts.Node):boolean=>joins(node)&&both(node)||ts.forEachChild(node,child=>inner(child)||undefined)===true;
 const declarations:ts.VariableDeclaration[]=[],tainted=new Set<ts.Node>();const collect=(node:ts.Node):void=>{if(ts.isVariableDeclaration(node)&&ts.isIdentifier(node.name)&&node.initializer)declarations.push(node);ts.forEachChild(node,collect);};collect(file);
 for(let grown=true;grown;){grown=false;for(const declaration of declarations)if(!tainted.has(declaration)&&checkoutPath(declaration.initializer!)){tainted.add(declaration);grown=true;}}
 const report=(node:ts.Node,rule:TestStoreFindingV1["rule"])=>{const line=file.getLineAndCharacterOfPosition(node.getStart()).line+1;found.set(line+rule,{line,rule,text:node.getText().replace(/\s+/gu," ").slice(0,160)});};
 const visit=(node:ts.Node):void=>{
  if(ts.isCallExpression(node)&&calleeTextV1(node.expression).some(name=>store.some(pattern=>pattern.test(name)))&&node.arguments.some(argument=>value(argument)))report(node,"checkout-store-call");
  if(joins(node)&&both(node)&&ts.forEachChild(node,child=>inner(child)||undefined)!==true)report(node,"checkout-store-path");
  ts.forEachChild(node,visit);
 };
 visit(file);
 return[...found.values()].sort((left,right)=>left.line-right.line||left.rule.localeCompare(right.rule));
}
/** 🗂️ The test files below the rule roots: tracked and untracked-not-ignored files from git, or a walk that skips dot directories and installed packages where no git is available. */
export function listTestStoreFilesV1(root:string,rules:TestStoreRulesV1=TEST_STORE_RULES_V1):readonly string[]{
 const test=new RegExp(rules.testPath,"u"),keep=(path:string)=>test.test("/"+path)&&rules.sourceExtensions.some(extension=>path.endsWith(extension))&&!path.endsWith(".d.ts")&&!path.endsWith(".d.mts");
 const listed=existsSync(join(root,".git"))?gitSyncV1(["ls-files","-co","--exclude-standard","-z","--",...rules.roots],{cwd:root,maxBuffer:512*1024*1024}):undefined;
 if(listed!==undefined){if(listed.status!==0)throw Error("git-ls-files-failed:"+listed.stderr.trim());return listed.stdout.split("\0").filter(path=>path!==""&&keep(path));}
 const files:string[]=[],walk=(relative:string):void=>{for(const entry of readdirSync(join(root,relative),{withFileTypes:true})){if(entry.name.startsWith(".")||entry.name==="node_modules")continue;const path=relative+"/"+entry.name;if(entry.isDirectory())walk(path);else if(entry.isFile()&&keep(path))files.push(path);}};
 for(const directory of rules.roots)if(existsSync(join(root,directory)))walk(directory);
 return files.sort();
}
/** ⚖️ Scans every test file of the repository; files that mention no checkout reference are skipped unparsed, exception files are not judged. */
export function testStoreIsolationReportV1(root:string,rules:TestStoreRulesV1=TEST_STORE_RULES_V1):TestStoreReportV1{
 const files=listTestStoreFilesV1(root,rules),excepted=new Set(rules.exceptions.map(row=>row.path)),hints=[...rules.checkoutCalls.map(call=>call.split(".").at(-1)!),...rules.checkoutEnvironment],findings:TestStoreFindingV1[]=[],matched=new Set<string>();
 for(const path of files){
  let source:string;try{source=readFileSync(join(root,path),"utf8");}catch{continue;}
  if(!hints.some(hint=>source.includes(hint)))continue;
  const reaches=scanTestStoreReachesV1(source,rules,path);if(reaches.length===0)continue;
  if(excepted.has(path)){matched.add(path);continue;}
  for(const reach of reaches)findings.push({path,...reach});
 }
 return{scanned:files.length,findings,stale:[...excepted].filter(path=>!matched.has(path))};
}
