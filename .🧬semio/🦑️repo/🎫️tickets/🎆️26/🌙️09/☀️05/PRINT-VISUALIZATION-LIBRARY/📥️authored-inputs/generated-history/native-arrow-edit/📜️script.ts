import { readFileSync, writeFileSync, renameSync } from 'node:fs';
import { join } from 'node:path';
const root = process.cwd();
const ticket = process.env.SEMIO_TICKET_DIR!;
const product = join(root,'🧰️framework/🛍️products/📓️print');
const write = (path:string,text:string) => {const temporary=join(ticket,'🗑️generated','native-arrow-owner.tmp');writeFileSync(temporary,text);renameSync(temporary,path);};
const graphPath=join(product,'🖋️latex/semio-viz-network-graph.sty');
let graph=readFileSync(graphPath,'utf8');
const graphHelper=readFileSync(join(ticket,'🗑️generated/graph-native.txt'),'utf8');
if(!graph.includes('\\semio_viz_net_directed_draw:n')){
  const begin=graph.indexOf('\\cs_new_protected:Npn \\semio_viz_net_link_draw:n');
  const end=graph.indexOf('% 🔵 A self-loop',begin);
  const owned=graph.slice(begin,end);
  const compare=owned.indexOf('      \\fp_compare:nNnTF');
  const tail=owned.lastIndexOf('    }');
  const replaced=owned.slice(0,compare)+'      \\bool_if:NTF \\l_semio_viz_net_directed_bool\n        { \\semio_viz_net_directed_draw:n {#1} }\n        {\n'+owned.slice(compare,tail)+'        }\n'+owned.slice(tail);
  graph=graph.slice(0,begin)+graphHelper+replaced+graph.slice(end);
  write(graphPath,graph);
}
const guidePath=join(product,'🖋️latex/semio-viz-guide.sty');
let guide=readFileSync(guidePath,'utf8');
if(!guide.includes('\\semio_viz_guide_title_position:')){
  const begin=guide.indexOf('\\cs_new_protected:Npn \\semio_viz_guide_title_draw:');
  const end=guide.indexOf('% 📐 The whole axis',begin);
  const owned=guide.slice(begin,end);
  const at=owned.indexOf('        at (');
  const text=owned.indexOf('        { \\exp_not:V \\l_semio_viz_axis_title_tl }',at);
  let replaced=owned.slice(0,at)+'        at ( \\fp_use:N \\l_semio_viz_guide_px_fp , \\fp_use:N \\l_semio_viz_guide_py_fp )\n'+owned.slice(text);
  replaced=replaced.replace('    \\semio_viz_guide_style:nN { title }','    \\semio_viz_guide_title_position:\n    \\semio_viz_guide_probe_title:\n    \\semio_viz_guide_style:nN { title }');
  const probe='\\cs_new_protected:Npn \\semio_viz_guide_probe_title: {\n  \\semio_viz_probe_geometry:nx { axis-title } { \\fp_use:N \\l_semio_viz_guide_px_fp , \\fp_use:N \\l_semio_viz_guide_py_fp }\n}\n';
  guide=guide.slice(0,begin)+readFileSync(join(ticket,'🗑️generated/guide-title.txt'),'utf8')+probe+replaced+guide.slice(end);
  write(guidePath,guide);
}
const runnerPath=join(product,'🧪️tests/🧬️native-chart-grammar/🟦️.ts');
let runner=readFileSync(runnerPath,'utf8');
if(!runner.includes('await compileNativeDirectedGraphGrammar(join(workDir')){
  runner=runner.replace('  const transforms = await nativeTransformGrammarChecks','  await compileNativeDirectedGraphGrammar(join(workDir, "directed-graph"));\n  const transforms = await nativeTransformGrammarChecks');
  write(runnerPath,runner);
}
