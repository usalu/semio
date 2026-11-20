import {readFileSync, existsSync} from 'node:fs';
import {resolve, dirname, join} from 'node:path';
const root=process.cwd(),catalog=readFileSync('✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🧩️plugins/🟦️.ts','utf8');
const initial=[...catalog.matchAll(/pluginId: "([^"]+)"[^\n]*?cratePath: "([^"]+)"/g)].map(m=>({from:m[1]!,path:resolve(root,m[2]!,'Cargo.toml')}));
const workspaces=new Map<string,any>();
function workspace(dir:string,declared?:string):{dir:string;data:any}|undefined{let d=declared?resolve(dir,declared):dir;while(true){const f=join(d,'Cargo.toml');if(existsSync(f)){let data=workspaces.get(f);if(!data){data=Bun.TOML.parse(readFileSync(f,'utf8'));workspaces.set(f,data);}if(data.workspace)return {dir:d,data};}const parent=dirname(d);if(declared||parent===d)return undefined;d=parent;}}
const seen=new Set<string>(), queue=[...initial], errors:string[]=[], inherited:string[]=[]; let deps=0;
while(queue.length){const row=queue.shift()!, file=row.path;if(seen.has(file))continue;seen.add(file);if(!existsSync(file)){errors.push(`missing manifest ${file} from ${row.from}`);continue;}let data:any;try{data=Bun.TOML.parse(readFileSync(file,'utf8'));}catch(e){errors.push(`invalid TOML ${file}: ${e}`);continue;}
 const dir=dirname(file),lib=data.lib;if(lib?.path && !existsSync(resolve(dir,lib.path))) errors.push(`missing lib ${resolve(dir,lib.path)} from ${file}`);
 const groups=[data.dependencies,data['build-dependencies'],...Object.values(data.target??{}).flatMap((x:any)=>[x.dependencies,x['build-dependencies']])];
 for(const group of groups)for(const [name,def] of Object.entries(group??{}) as [string,any][]){if(def&&typeof def==='object'&&def.path){deps++;queue.push({from:file+':'+name,path:resolve(dir,def.path,'Cargo.toml')});}else if(def&&typeof def==='object'&&def.workspace){const w=workspace(dir,data.package?.workspace),entry=w?.data.workspace?.dependencies?.[name];if(!w||!entry){inherited.push(`${file}:${name}`);continue;}if(typeof entry==='object'&&entry.path){deps++;queue.push({from:file+':'+name+' (workspace)',path:resolve(w.dir,entry.path,'Cargo.toml')});}}}
}
console.log(`catalog roots=${initial.length} traversed manifests=${seen.size} explicit path edges=${deps} unresolved inherited dependencies=${inherited.length}`);for(const error of errors)console.log('ERROR '+error);
