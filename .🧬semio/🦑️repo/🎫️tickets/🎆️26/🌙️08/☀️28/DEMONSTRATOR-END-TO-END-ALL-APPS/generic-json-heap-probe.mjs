/** 🧪️ Corroborates actual generic parse/reference memory under an unchanged standalone budget. */
import fs from 'node:fs/promises';
import {createRequire} from 'node:module';
import {createHash} from 'node:crypto';
import {join} from 'node:path';
const root=process.cwd(),require=createRequire(join(root,'package.json')),ts=require('typescript'),Ajv=require('ajv');
const input=JSON.parse(await fs.readFile(new URL('./generic-json-heap-cases.json',import.meta.url),'utf8')),schema=JSON.parse(await fs.readFile(new URL('./generic-json-heap-schema.json',import.meta.url),'utf8'));
if(!new Ajv({strict:true}).validate(schema,input))throw Error('Closed neutral heap corpus required');
const selected=input.cases.find(row=>row.id===process.argv[2]);if(!selected||process.argv.length!==3)throw Error('Exact neutral heap case required');
const library=join(root,'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library'),closure=join(library,'⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures');
const worker=await fs.readFile(join(closure,'👷️read.mjs'),'utf8'),document=JSON.parse(await fs.readFile(join(closure,'🧬️schema/🔣️.json'),'utf8')),P=document.definitions.Protocol.const;
for(const[key,value]of Object.entries({maximumExpandedBytes:input.limits.expandedBytes,heapMiB:input.limits.heapMiB,maximumNodes:input.limits.maximumNodes,maximumReferences:input.limits.maximumReferences,maximumDepth:input.limits.maximumDepth}))if(P[key]!==value)throw Error('Current reviewed generic policy required: '+key);
const tree=ts.createSourceFile('worker.mjs',worker,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS),names=['parse','references'],nodes=names.map(name=>tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text===name));if(nodes.some(node=>!node))throw Error('Actual worker parse/reference definitions required');
const code=nodes.map(node=>node.getText(tree)).join('\n'),api=Function('P',code+';return {parse,references}')(P),sha=value=>createHash('sha256').update(value).digest('hex');
/** 🔍️ Uses the independent TypeScript JSON syntax tree for a small key/value reference and node oracle. */
function oracle(text){const parsed=ts.parseJsonText('neutral.json',text);if(parsed.parseDiagnostics.length)throw Error('Independent JSON syntax required');let count=0;const retained=new Set();const string=value=>{for(const match of value.matchAll(/SEMIO_(SOURCE_CLOSURE|CARGO_LOCK)_([a-f0-9]{64})(?![a-zA-Z0-9_])/gu))retained.add((match[1]==='SOURCE_CLOSURE'?'source:':'lock:')+match[2]);};const visit=node=>{count++;if(ts.isObjectLiteralExpression(node))for(const property of node.properties){count++;string(property.name.text);visit(property.initializer);}else if(ts.isArrayLiteralExpression(node))for(const element of node.elements){count++;visit(element);}else if(ts.isStringLiteral(node))string(node.text);};visit(parsed.statements[0].expression);return{nodes:count,references:[...retained].sort()};}
const marker='SEMIO_SOURCE_CLOSURE_'+'a'.repeat(64),small=JSON.stringify({padding:'x',fact:[marker,{'SEMIO_CARGO_LOCK_bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb':true}]}),independent=oracle(small);
if(JSON.stringify(api.references(api.parse(Buffer.from(small))))!==JSON.stringify(independent.references))throw Error('Actual reference/third-party AST parity required');
const phases=[];const sample=phase=>{const memory=process.memoryUsage();phases.push({phase,...memory});process.stdout.write('[DEBUG] '+selected.id+' '+phase+' '+JSON.stringify(memory)+'\n');};sample('before-construction');
const maximum=P.maximumExpandedBytes,entries=selected.nodes?(selected.nodes-1)/2:0;let bytes,offset=0;
const put=text=>{offset+=bytes.write(text,offset,'utf8');};
if(selected.shape==='wide-array'){
 bytes=Buffer.alloc(entries*2+1);bytes.fill('0,',1,bytes.length-1);bytes[0]=91;bytes[bytes.length-1]=93;
}else{
 bytes=Buffer.alloc(maximum,97);
 if(selected.shape==='one-string'){put('{"padding":"');Buffer.from('"}').copy(bytes,bytes.length-2);if(selected.encoding!=='ascii')Buffer.from('Ā').copy(bytes,offset);}
 else if(selected.shape==='wide-array-with-padding-string'){put('["');const tailBytes=2+2*(entries-1);offset=bytes.length-tailBytes;put('"');for(let i=0;i<entries-1;i++)put(',0');put(']');}
 else if(selected.shape==='wide-object-with-padding-string'){put('{"padding":"');let tailBytes=2;for(let i=0;i<entries-1;i++)tailBytes+=Buffer.byteLength(',"k'+i+'":0');offset=bytes.length-tailBytes;put('"');for(let i=0;i<entries-1;i++)put(',"k'+i+'":0');put('}');}
 else if(selected.shape==='distinct-source-markers-with-padding-string'){put('["');const count=selected.references,unit=Buffer.byteLength(',"SEMIO_SOURCE_CLOSURE_'+'0'.repeat(64)+'"');offset=bytes.length-(2+unit*count);put('"');for(let i=0;i<count;i++)put(',"SEMIO_SOURCE_CLOSURE_'+i.toString(16).padStart(64,'0')+'"');put(']');}
 else throw Error('Closed neutral heap shape required');
}
if(selected.wireBytes!==undefined&&bytes.length!==selected.wireBytes||selected.nodes!==undefined&&(!Number.isInteger(entries)||1+2*entries!==selected.nodes))throw Error('Exact neutral wire/node boundary required');
const wireSha256=sha(bytes),started=Date.now();sample('before-parse');let value=api.parse(bytes);sample('after-parse');let failure,failed=false,references;try{references=api.references(value);}catch(error){failed=true;failure=error;}sample('after-references');
if(selected.expected==='accepted'&&failed)throw failure;if(selected.expected==='bounded-node-refusal'&&(!failed||!(failure instanceof Error)||failure.message!=='Bounded closure reference DATA required'))throw Error('Exact node-budget refusal required');
if(!failed){const count=selected.references??0;if(references.length!==count)throw Error('Complete reference count required');for(let i=0;i<count;i++)if(references[i]!=='source:'+i.toString(16).padStart(64,'0'))throw Error('Every distinct reference required');}
const maxRssKiB=process.resourceUsage().maxRSS,maximumRssBytes=maxRssKiB*1024,output={version:1,purpose:input.purpose,id:selected.id,runtime:process.version,executable:process.execPath,platform:process.platform,workerSha256:sha(worker),functionsSha256:sha(code),expandedBytes:bytes.length,wireSha256,expectedNodes:selected.nodes??null,expectedReferences:selected.references??0,outcome:failed?'bounded-node-refusal':'accepted',failure:failed?failure.message:null,references:references?.length??null,elapsedMilliseconds:Date.now()-started,phases,maximumRssBytes,maxRssKiB,withinAdmission:maximumRssBytes<=input.limits.heapMiB*1048576,limits:input.limits,scope:'Synthetic actual parse/reference function corroboration; not a full closure-process guarantee'};
process.stdout.write(JSON.stringify(output)+'\n');if(!output.withinAdmission)process.exitCode=1;value=undefined;bytes=undefined;references=undefined;
