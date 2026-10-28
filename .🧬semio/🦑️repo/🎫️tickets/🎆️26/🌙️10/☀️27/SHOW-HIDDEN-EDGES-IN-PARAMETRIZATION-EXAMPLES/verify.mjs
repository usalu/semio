import fs from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
const t=path.dirname(fileURLToPath(import.meta.url)),s=fs.readFileSync('🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/parametrization-geometry.tex','utf8');
for(const item of JSON.parse(fs.readFileSync(path.join(t,'cases.json'),'utf8')).cases){
  const start=s.indexOf('\\newcommand{\\'+item.macro+'}'),end=s.indexOf('\\newcommand',start+1),body=s.slice(start,end<0?s.length:end),paths=[...body.matchAll(/\\draw\[densely dashed\]([^;]+);/g)].map(m=>m[1]);
  if(start<0||paths.length!==item.paths)throw Error(item.macro+': expected '+item.paths+' hidden paths, got '+paths.length);
  if(item.segments&&JSON.stringify(paths.map(p=>(p.match(/--/g)||[]).length))!==JSON.stringify(item.segments))throw Error('Hidden segments mismatch');
  if(item.domain&&!paths[0].includes('domain='+item.domain))throw Error('Hidden ellipse mismatch');
}
console.log('PASS3 hidden-line cases');
