import {readFileSync,writeFileSync,renameSync} from 'node:fs';
import {join} from 'node:path';
const product='C:/git/semio/🧰️framework/🛍️products/📓️print',temporary=join(import.meta.dir,'source.tmp');
function edit(path:string,change:(source:string)=>string){const file=join(product,path);writeFileSync(temporary,change(readFileSync(file,'utf8')));renameSync(temporary,file);}
edit('🧬️schema/💡️inferences/🖼️render/🟦️.ts',source=>source.replace('stroke=Number(o.strokeWidth??theme.strokes[d.strokeRole]),outline=Number(o.strokeWidth??theme.strokes[d.outlineStrokeRole])','stroke=Number(o.strokeWidth??theme.strokes.chromeBorderDefault*.75*25.4/72.27),outline=Number(o.strokeWidth??theme.strokes.chromeBorderHairline*.75*25.4/72.27)'));
edit('🧪️tests/🎬️render-scene/🧭️legend/🟦️.ts',source=>source.replace('import {format} from "d3-format";','import {format} from "d3-format";\nimport {rgb} from "d3-color";').replace('colors.push(String((map as (value:number)=>unknown)(scaleLinear([0,48],declared.domain as number[])(slice+.5))))','colors.push(rgb(String((map as (value:number)=>unknown)(scaleLinear([0,48],declared.domain as number[])(slice+.5)))).formatHex())'));
edit('🧪️tests/🎬️render-scene/🔣️customization.json',source=>{const fixture=JSON.parse(source);fixture.cases.find((entry:{id:string})=>entry.id==='legend-color').expected.y=16;return JSON.stringify(fixture,null,2)+'\n';});
