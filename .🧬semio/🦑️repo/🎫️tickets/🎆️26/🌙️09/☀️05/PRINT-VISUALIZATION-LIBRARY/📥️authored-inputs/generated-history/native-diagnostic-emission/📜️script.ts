import { readFileSync } from 'node:fs';
import { compileVizProbeDocument } from 'C:/git/semio/🧰️framework/🛍️products/📓️print/🔨️modules/🧪️viz-probe/🟦️.ts';
import fixture from 'C:/git/semio/🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🔣️.json';
import { scaleLinear } from 'C:/git/semio/node_modules/d3-scale/src/index.js';
const root='C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/PRINT-VISUALIZATION-LIBRARY/🗑️generated';
const text=readFileSync(root+'/native-final-2026-10-04/native-chart.tex','utf8');
const probes=fixture.typedValues.map((_,index)=>`\\ExplSyntaxOn\\semio_viz_table_cell:nnnN{native-typed}{${index+1}}{x}\\l_tmpa_tl\\semio_viz_scale_value:nVN{native-typed-x}\\l_tmpa_tl\\l_tmpb_tl\\semio_viz_probe_values:nV{native/typed/${index}}\\l_tmpb_tl\\semio_viz_scale_value:nVN{native-typed-color}\\l_tmpa_tl\\l_tmpb_tl\\semio_viz_probe_values:nx{native/typed-alpha/${index}}{\\semio_viz_color_alpha:n{\\l_tmpb_tl}}\\ExplSyntaxOff`).join('\n');
const records=await compileVizProbeDocument({case:'native-chart-grammar',scenario:'native-emission',documentClass:'semio',documentClassOptions:'type=paper,language=en',packages:['semio-viz'],geometry:true,preamble:['\\title{Native Chart Grammar}','\\author{Semio}','\\date{}','\\errorcontextlines=200'],body:[{raw:text.replace('\\end{VizFigure}',probes+'\n\\end{VizFigure}')}]},{workDir:root+'/native-diagnostic-emission',keepWorkDir:true});
const scale=scaleLinear([0,3],[0,30]).unknown(-5);
for(const [index,value] of fixture.typedValues.entries()) {
  const actual=Number(records.find(record=>record.key===`native/typed/${index}`)!.values[0]);
  const alpha=Number(records.find(record=>record.key===`native/typed-alpha/${index}`)!.values[0]);
  if(Math.abs(actual-scale(value))>1e-4 || Math.abs(alpha-(value===null||value==='bad'?.35:1))>1e-4) throw new Error(JSON.stringify({index,actual,alpha}));
}
console.log('[native-grammar] actual Rust emission and all six typed values/functional alpha passed');
