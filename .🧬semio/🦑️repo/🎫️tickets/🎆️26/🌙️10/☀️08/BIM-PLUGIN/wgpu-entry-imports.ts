import {dirname, resolve, relative} from 'node:path';
import {realpathSync} from 'node:fs';
const workspace=process.cwd();
const entry=resolve('🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts');
const edges=new Map<string,Set<string>>();
const result=await Bun.build({entrypoints:[entry],target:'browser',format:'esm',define:{'import.meta.vitest':'undefined'},plugins:[{name:'imports',setup(build){build.onResolve({filter:/.*/},row=>{if(!row.importer)return;try{const source=realpathSync(row.importer),child=realpathSync(Bun.resolveSync(row.path,dirname(source)));if(!edges.has(source)) edges.set(source,new Set());edges.get(source)!.add(child);}catch{}})}}]});
if(!result.success)throw Error(result.logs.join('\n'));
const paths=[[entry]],seen=new Set<string>();for(let i=0;i<paths.length;i++){const path=paths[i]!,last=path.at(-1)!;if(seen.has(last))continue;seen.add(last);if(last.includes('🎓️first-run')){console.log(path.map(p=>relative(workspace,p)).join('\n'));break;}for(const p of edges.get(last)??[])paths.push([...path,p]);}
for(const [a,values] of edges)for(const v of values)if(v.includes('🎓️first-run')&&a!==v)console.log('direct',relative(workspace,a),'=>',relative(workspace,v));
for(const [a,values] of edges)for(const v of values)if(v.endsWith('💻️os/📦️packages/🟦️typescript/🟦️.ts')&&!a.endsWith('/🧪️tests/🟦️.ts'))console.log('facade',relative(workspace,a));
