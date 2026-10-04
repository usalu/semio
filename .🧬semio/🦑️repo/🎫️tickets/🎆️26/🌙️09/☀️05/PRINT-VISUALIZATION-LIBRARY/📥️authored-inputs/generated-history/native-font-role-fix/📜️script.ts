import {readFileSync,writeFileSync,renameSync} from 'node:fs';
const file='C:/git/semio/🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-guide.sty';
let source=readFileSync(file,'utf8');
source=source.replace('font=\\exp_not:N \\fontsize {\\l_semio_viz_guide_label_size_tl}','font=\\exp_not:N \\normalfont \\exp_not:N \\SemioSans \\exp_not:N \\fontsize {\\l_semio_viz_guide_label_size_tl}');
source=source.replace('font=\\exp_not:N \\fontsize {\\l_semio_viz_guide_title_size_tl}','font=\\exp_not:N \\normalfont \\exp_not:N \\SemioMono \\exp_not:N \\fontsize {\\l_semio_viz_guide_title_size_tl}');
const temporary=import.meta.dir+'/source.tmp';writeFileSync(temporary,source);renameSync(temporary,file);
