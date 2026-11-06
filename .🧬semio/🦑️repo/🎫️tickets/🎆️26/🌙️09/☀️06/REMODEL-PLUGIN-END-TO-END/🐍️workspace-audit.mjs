import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
const run=spawnSync('git',['-c','core.quotepath=false','ls-files','--cached','--others','--exclude-standard','--','**/Cargo.toml','Cargo.toml'],{encoding:'utf8',maxBuffer:30_000_000});
if(run.status!==0)throw Error(run.stderr);
const files=[...new Set(run.stdout.split(/\r?\n/).filter(Boolean))];
const errors=[],roots=new Map();
for(const file of files){if(file.startsWith('.🧬semio/'))continue;try{const data=Bun.TOML.parse(fs.readFileSync(file,'utf8'));if(!data.package?.workspace)continue;const workspace=path.resolve(path.dirname(file),data.package.workspace,'Cargo.toml');if(!fs.existsSync(workspace)){errors.push({file,error:'missing workspace',workspace});continue}if(!roots.has(workspace))roots.set(workspace,Bun.TOML.parse(fs.readFileSync(workspace,'utf8')));const root=roots.get(workspace);const relative=path.relative(path.dirname(workspace),path.dirname(path.resolve(file))).replaceAll('\\','/');const matches=(patterns)=>(patterns??[]).some(pattern=>new Bun.Glob(pattern.replaceAll('\\','/')).match(relative));if(!matches(root.workspace?.members)||matches(root.workspace?.exclude))errors.push({file,error:'not explicit workspace member or excluded',relative,workspace});const groups=[data,...Object.values(data.target??{})];for(const group of groups)for(const section of ['dependencies','dev-dependencies','build-dependencies'])for(const [key,value] of Object.entries(group[section]??{})){if(value&&typeof value==='object'&&value.workspace===true&&!root.workspace?.dependencies?.[key])errors.push({file,error:'missing workspace dependency',key,workspace});}}catch(error){errors.push({file,error:String(error)})}}
console.log(JSON.stringify({count:files.length,errors},null,2));
