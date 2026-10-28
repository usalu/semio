/** 📋️ Inventory issued owners and static branch shapes without equating lexical mentions with coverage. */
import {readFile,readdir} from 'node:fs/promises';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
const root=resolve(import.meta.dir,'../../../../../../..');
const directory=resolve(root,'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation');
const {compileIfc4x3Express}=await import(pathToFileURL(resolve(directory,'compiler.ts')).href);
const compilation=await compileIfc4x3Express();
if(!compilation.complete||compilation.diagnostics.length)throw Error('Issued canonical compilation is incomplete');
const files=(await readdir(directory)).filter(name=>name.endsWith('.test.ts'));
const tests=await Promise.all(files.map(async name=>({name,text:await readFile(resolve(directory,name),'utf8')})));
const source=await readFile(resolve(directory,'../📚️catalog/IFC4X3_ADD2.exp'),'utf8');
function syntax(value:unknown,counts:Record<string,number>={},calls=new Set<string>()):{counts:Record<string,number>;calls:string[]}{if(value&&typeof value==='object'){const node=value as Record<string,unknown>;if(typeof node.kind==='string')counts[node.kind]=(counts[node.kind]??0)+1;if(node.kind==='call'&&Array.isArray(node.operands)){const name=node.operands[0]?.name;if(typeof name==='string')calls.add(name);}for(const child of Object.values(node))if(child&&typeof child==='object')syntax(child,counts,calls);}return{counts,calls:[...calls].sort()};}
function row(owner:any,index:number,kind:'function'|'global'){const match=new RegExp('^'+(kind==='function'?'FUNCTION':'RULE')+'\\s+'+owner.name+'\\b','im').exec(source),start=match?source.slice(0,match.index).split('\n').length:null;return{index,kind,owner:owner.name,sourceStartLine:start,...syntax(owner),parameters:owner.parameters??[],populations:owner.populations??[],staticTestMentions:tests.filter(test=>test.text.toUpperCase().includes(owner.name.toUpperCase())).map(test=>test.name),coverageClaims:{issuedIndeterminateInvocation:kind==='function',concreteBranchExecution:'not-established-by-this-census',staticMentionsAreRuntimeProof:false}};}
const output={schema:'IFC4X3_ADD2',staticTestScan:{directory,fileCount:files.length,recursive:false,scope:'direct validation tests only; broader test evidence is unscored'},functions:compilation.program.functions.map((owner:any,index:number)=>row(owner,index,'function')),globals:compilation.program.globalRules.map((owner:any,index:number)=>row(owner,index,'global')),nativeExecuted:false};
process.stdout.write(JSON.stringify(output,null,2)+'\n');
