import {readFileSync,writeFileSync,renameSync} from 'node:fs';
const path='C:/git/semio/🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-charts-financial.sty';let s=readFileSync(path,'utf8');
s=s.replace('\\int_step_variable:nNn {\\seq_count:N\\l_semio_viz_fin_closes_seq} \\l_semio_viz_fin_index_int {','\\int_step_inline:nn {\\seq_count:N\\l_semio_viz_fin_closes_seq} {\n    \\int_set:Nn \\l_semio_viz_fin_index_int {##1}');
s=s.replace('\\int_step_variable:nnNn {\\l_semio_viz_fin_first_int}{\\l_semio_viz_fin_index_int}\\l_semio_viz_fin_sample_int {','\\int_step_inline:nnn {\\l_semio_viz_fin_first_int}{\\l_semio_viz_fin_index_int} {');
s=s.replace('\\seq_item:Nn\\l_semio_viz_fin_closes_seq{\\l_semio_viz_fin_sample_int}','\\seq_item:Nn\\l_semio_viz_fin_closes_seq{####1}');
s=s.replace('\\int_new:N \\l_semio_viz_fin_sample_int\n','');writeFileSync(import.meta.dir+'/financial.sty',s);renameSync(import.meta.dir+'/financial.sty',path);