import {readFileSync,writeFileSync,renameSync} from 'node:fs';
import {join} from 'node:path';
const ticket=process.env.SEMIO_TICKET_DIR!;
const path=join(process.cwd(),'🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-network-graph.sty');
let text=readFileSync(path,'utf8');
if(!text.includes('\\semio_viz_net_directed_loop:n')){
  text=text.replace('\\cs_new_protected:Npn \\semio_viz_net_link_draw:n',readFileSync(join(ticket,'🗑️generated/graph-loop.txt'),'utf8')+'\n\\cs_new_protected:Npn \\semio_viz_net_link_draw:n');
  text=text.replace('{ \\semio_viz_net_loop_draw:n { \\semio_viz_network_n:nn { es } {#1} } }','{ \\bool_if:NTF \\l_semio_viz_net_directed_bool\n        { \\semio_viz_net_directed_loop:n { \\semio_viz_network_n:nn { es } {#1} } }\n        { \\semio_viz_net_loop_draw:n { \\semio_viz_network_n:nn { es } {#1} } } }');
  text=text.replace('{ 1.3*\\l_semio_viz_net_tip_r_fp }','{ max(1e-15,1.3*\\l_semio_viz_net_tip_r_fp) }');
  const temporary=join(ticket,'🗑️generated/graph-loop-owner.tmp');writeFileSync(temporary,text);renameSync(temporary,path);
}
