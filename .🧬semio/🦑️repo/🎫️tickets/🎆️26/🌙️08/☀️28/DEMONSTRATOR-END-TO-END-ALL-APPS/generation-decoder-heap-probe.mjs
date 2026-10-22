/** 🧪️ Corroborates the actual typed generation scanner with a bounded synthetic wire and independent small JSON oracle. */
import fs from 'node:fs/promises';
import {createRequire} from 'node:module';
import {join} from 'node:path';
import v8 from 'node:v8';
const root=process.cwd(),require=createRequire(join(root,'package.json')),ts=require('typescript');
const library=join(root,'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library');
const closure=join(library,'⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures');
const worker=await fs.readFile(join(closure,'👷️read.mjs'),'utf8'),generation=await fs.readFile(join(library,'🏗️build/🕸️graph/🕰️generations/🟨️.mjs'),'utf8');
const workerTree=ts.createSourceFile('worker.mjs',worker,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS),generationTree=ts.createSourceFile('generation.mjs',generation,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS);
const scanner=workerTree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==='readGenerationClosureV1');
let initializer;for(const statement of generationTree.statements)if(ts.isVariableStatement(statement))for(const declaration of statement.declarationList.declarations)if(ts.isIdentifier(declaration.name)&&declaration.name.text==='GENERATION_POLICY_V1')initializer=declaration.initializer;
if(!scanner||!initializer)throw Error('Actual generation scanner and policy required');
const document=JSON.parse(await fs.readFile(join(closure,'🧬️schema/🔣️.json'),'utf8')),P=document.definitions.Protocol.const,policy=Function('return ('+initializer.getText(generationTree)+')')();
const generationSchema=JSON.parse(await fs.readFile(join(library,'🏗️build/🕸️graph/🕰️generations/🧬️schema/🔣️.json'),'utf8'));
let phase='small',decoded=0,nextProgress=64*1024*1024;const peak={rss:0,heapUsed:0,external:0,arrayBuffers:0};
function sample(){const usage=process.memoryUsage();for(const key of Object.keys(peak))peak[key]=Math.max(peak[key],usage[key]??0);return usage;}
class ObservedDecoder extends TextDecoder{decode(...args){const value=super.decode(...args);decoded+=args[0]?.byteLength??0;sample();if(phase==='large'&&decoded>=nextProgress){process.stdout.write('[DEBUG] generation probe scanned '+decoded+' bytes\n');nextProgress+=64*1024*1024;}return value;}}
const scan=Function('P','document','GENERATION_POLICY_V1','generationSchema','TextDecoder',scanner.getText(workerTree)+';return readGenerationClosureV1')(P,document,policy,generationSchema,ObservedDecoder);
const ref='a'.repeat(64),marker='SEMIO_SOURCE_CLOSURE_'+ref,smallValue={version:1,owners:['owner'],rows:[{kind:'digest',path:'path',fact:{digest:marker}}],classes:[{r:[0],o:[0]}]},small=Buffer.from(JSON.stringify(smallValue));
const oracle=JSON.parse(small.toString('utf8'));if(oracle.rows[0].fact.digest!==marker)throw Error('Independent JSON oracle mismatch');
const smallResult=scan([small],'generation-index',policy.maximumIndexBytes);if(!smallResult.references.includes('source:'+ref)||smallResult.expandedBytes!==small.length)throw Error('Actual scanner parity required');
phase='large';decoded=0;const baseline=sample(),maximum=policy.maximumIndexBytes,referenceCount=P.maximumReferences;
if(maximum!==536870912||referenceCount!==262144)throw Error('Reviewed policy and reference boundary required');
let bytes=Buffer.alloc(maximum,0x61),offset=0;const put=text=>{offset+=bytes.write(text,offset,'utf8');};
put('{"classes":[{"r":[0],"o":['+(referenceCount-1)+']}],"owners":[');
for(let i=0;i<referenceCount;i++)put((i?',':'')+'"SEMIO_SOURCE_CLOSURE_'+i.toString(16).padStart(64,'0')+'"');
put('],"rows":[{"kind":"digest","path":"probe","fact":{"padding":"');
const suffix=Buffer.from('"}}],"version":1}');if(offset+suffix.length>=bytes.length)throw Error('Bounded synthetic body required');suffix.copy(bytes,bytes.length-suffix.length);sample();
function* chunks(){for(let i=0;i<bytes.length;i+=P.chunkBytes)yield bytes.subarray(i,Math.min(bytes.length,i+P.chunkBytes));}
const started=Date.now(),result=scan(chunks(),'generation-index',maximum);sample();
if(result.expandedBytes!==maximum||result.references.length!==referenceCount||result.tokens>maximum)throw Error('Complete bounded scan required');
if(peak.rss>1536*1024*1024)throw Error('Probe exceeded reviewed total RSS ceiling');
const output={version:1,purpose:'actual-generation-scanner-heap-corroboration',runtime:process.version,maximumOldSpaceMiB:1536,expandedBytes:result.expandedBytes,references:result.references.length,tokens:result.tokens,elapsedMilliseconds:Date.now()-started,baseline,peak,processMaxRssKiB:process.resourceUsage().maxRSS,heap:v8.getHeapStatistics(),limits:{expandedBytes:maximum,chunkBytes:P.chunkBytes,referenceCount},scope:'Synthetic wire and actual function; not a full closure-proof memory guarantee'};
process.stdout.write(JSON.stringify(output)+'\n');
