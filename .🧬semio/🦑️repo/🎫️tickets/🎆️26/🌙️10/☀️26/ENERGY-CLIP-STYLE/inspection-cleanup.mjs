import fs from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
const ticket=fs.realpathSync(path.dirname(fileURLToPath(import.meta.url)));
const inside=file=>{const relative=path.relative(ticket,file);return relative&&!relative.startsWith('..')&&!path.isAbsolute(relative)};
const scan=root=>fs.readdirSync(root,{withFileTypes:true}).flatMap(entry=>{const file=path.join(root,entry.name);if(entry.isSymbolicLink()||!inside(fs.realpathSync(file)))throw Error('Unsafe cleanup or delivery path');return entry.isDirectory()?scan(file):[file]});
const retained=scan(path.join(ticket,'inspection-videos'));
const counts={movies:retained.filter(file=>file.endsWith('.mp4')).length,sidecars:retained.filter(file=>file.endsWith('.json')).length,images:retained.filter(file=>file.endsWith('.png')).length};
if(retained.length!==281||counts.movies!==55||counts.sidecars!==93||counts.images!==133||retained.some(file=>file.length>256))throw Error('Incomplete retained delivery');
for(const name of ['inspection.md','inspection-delivery.md','inspection-evidence.md','inspection-final-file-inventory.md'])if(!fs.statSync(path.join(ticket,name)).size)throw Error('Missing retained report');
const generated=path.join(ticket,'🗑️generated'),bytecode=path.join(ticket,'__pycache__');
const removable=[];
if(fs.existsSync(generated)){if(!inside(fs.realpathSync(generated))||fs.lstatSync(generated).isSymbolicLink())throw Error('Unsafe generated root');scan(generated);removable.push(generated)}
if(fs.existsSync(bytecode)){const files=scan(bytecode);if(files.some(file=>path.extname(file)!=='.pyc'))throw Error('Unexpected bytecode cache contents');removable.push(bytecode)}
if(!process.argv.includes('--confirmed')){console.log(JSON.stringify({ticket,counts,removable,dry_run:true}));process.exit(0)}
for(const target of removable)fs.rmSync(target,{recursive:true,maxRetries:6,retryDelay:100});
if(removable.some(target=>fs.existsSync(target)))throw Error('Generated cleanup incomplete');
const report=`# Inspection Cleanup

Retained 55 movies, 93 publication sidecars and 133 inspection PNGs (281 files). All destination paths are at most 256 characters. Verified resolved ticket containment and rejected symbolic links before deleting the owned generated tree and Python bytecode cache. Preserved scripts, fixtures, Markdown audits, intended previews and final delivery.
`;
const output=path.join(ticket,'inspection-cleanup.md');fs.writeFileSync(output+'.pending',report);fs.renameSync(output+'.pending',output);
console.log(JSON.stringify({counts,deleted:removable,complete:true}));
