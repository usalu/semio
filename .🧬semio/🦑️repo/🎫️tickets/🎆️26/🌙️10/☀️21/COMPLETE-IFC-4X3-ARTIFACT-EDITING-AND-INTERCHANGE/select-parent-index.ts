/** 🕸️ Reproduce the immutable source table from pinned owned ADD2 SELECT declarations. */
import {readFile,writeFile} from 'node:fs/promises';
import {resolve} from 'node:path';
import {createHash} from 'node:crypto';
const base=resolve(process.cwd(),'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema'),catalog=JSON.parse(await readFile(resolve(base,'📚️catalog/🔣️.json'),'utf8')),parents=new Map<string,string[]>();let selects=0,edges=0;
for(const type of Object.values(catalog.types) as {kind:string;name:string;members:string[]}[])if(type.kind==='select'){selects++;for(const member of type.members){const owners=parents.get(member)??[];owners.push(type.name.toUpperCase());parents.set(member,owners);edges++;}}
if(catalog.schema!=='IFC4X3_ADD2'||selects!==61||edges!==271)throw Error('Pinned SELECT release changed');
const rows=[...parents].map(([name,owners])=>[name,owners.sort()] as const).sort(([a],[b])=>a<b?-1:a>b?1:0),digest=createHash('sha256').update(JSON.stringify(rows)).digest('hex');
const source='/// 🕸️ Immutable direct SELECT parents from the pinned ADD2 catalog.\n'+'pub const SELECT_PARENTS_SHA256: &str = "'+digest+'";\n'+'pub static SELECT_PARENTS: &[(&str, &[&str])] = &[\n'+rows.map(([name,owners])=>'    ('+JSON.stringify(name)+', &['+owners.map(owner=>JSON.stringify(owner)).join(',')+']),').join('\n')+'\n];\n',path=resolve(base,'⚖️validation/select-parents.rs');
if(process.argv.includes('--check')){if(await readFile(path,'utf8')!==source)throw Error('SELECT parent table stale');}else await writeFile(path,source);
console.log('[DEBUG] '+JSON.stringify({selects,edges,memberKeys:rows.length,sha256:digest,check:process.argv.includes('--check')}));

