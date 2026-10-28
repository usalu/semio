import {readFile,writeFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
const base='C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/';
const catalog=JSON.parse(await readFile(base+'📚️catalog/🔣️.json','utf8'));
const source=await readFile(base+'⚖️validation/select-parents.rs','utf8');
const rows=[...source.matchAll(/\("([A-Z0-9_]+)", &\[([^\]]*)\]\)/g)].map(m=>[m[1],JSON.parse('['+m[2]+']')]);
const actual=new Map(rows),expected=new Map();
let edges=0,selects=0;
for(const type of Object.values(catalog.types))if(type.kind==='select'){selects++;for(const child of type.members){let parents=expected.get(child);if(!parents)expected.set(child,parents=[]);parents.push(type.name.toUpperCase());edges++;}}
const problems=[];
for(const [child,parents] of expected){if(JSON.stringify([...parents].sort())!==JSON.stringify(actual.get(child)))problems.push(child);}
if(actual.size!==expected.size||rows.length!==actual.size)problems.push('key-cardinality');
for(let i=1;i<rows.length;i++)if(rows[i-1][0]>=rows[i][0])problems.push('sort:'+i);
const digest=createHash('sha256').update(JSON.stringify(rows)).digest('hex');
if(!source.includes('"'+digest+'"'))problems.push('digest');
const result={producer:'independent source-table audit',nativeExecution:false,keys:actual.size,edges,selects,maxKeyBytes:Math.max(...rows.map(r=>r[0].length)),maxParents:Math.max(...rows.map(r=>r[1].length)),sha256:digest,problems};
await writeFile(new URL('./🗑️generated/select-parent-index-audit.json',import.meta.url),JSON.stringify(result,null,2));console.log('[DEBUG] '+JSON.stringify(result));if(problems.length)process.exitCode=1;
