import {readFileSync,writeFileSync,renameSync} from 'node:fs';
const root='C:/git/semio/🧰️framework/🛍️products/📓️print';let path=root+'/🖋️latex/semio-viz-guide.sty',s=readFileSync(path,'utf8');
const marker='% 🎨 The categorical legend: one swatch and label per entry, down a column or across `columns`.';
s=s.replace(marker,String.raw`% 📏 Reserves the authored font em line box above each legend row.
\cs_new:Npn \semio_viz_legend_line_extent:n #1 {
  \fp_eval:n {max(\l_semio_viz_legend_swatch_fp,
    \str_if_eq:nnTF{#1}{title}
      {\tl_if_empty:NTF\l_semio_viz_guide_title_size_tl{7.2}{\l_semio_viz_guide_title_size_tl}}
      {\tl_if_empty:NTF\l_semio_viz_guide_label_size_tl{6.6}{\l_semio_viz_guide_label_size_tl}}
    *\dim_to_fp:n{1pt}/\dim_to_fp:n{1mm})}
}

`+marker);
s=s.replace('* ( \\l_semio_viz_legend_swatch_fp + \\l_semio_viz_legend_item_gap_fp )','* ( \\semio_viz_legend_line_extent:n{label} + \\l_semio_viz_legend_item_gap_fp )');
s=s.replace('\\fp_sub:Nn \\l_semio_viz_legend_y_fp { \\l_semio_viz_legend_swatch_fp + \\l_semio_viz_legend_item_gap_fp }','\\fp_sub:Nn \\l_semio_viz_legend_y_fp { \\semio_viz_legend_line_extent:n{title} + \\l_semio_viz_legend_item_gap_fp }');
writeFileSync(import.meta.dir+'/guide.sty',s);renameSync(import.meta.dir+'/guide.sty',path);
path=root+'/🧬️schema/💡️inferences/🖼️render/🟦️.ts';s=readFileSync(path,'utf8');const start=s.indexOf('function legendGuideItems'),end=s.indexOf('\n/**',start+1);const chunk=s.slice(start,end);const after=chunk.replace('y-=swatch+gap;','y-=Math.max(swatch,Number(o.titleSize??d.titleSize)*25.4/72.27)+gap;').replace('Math.floor(index/columns)*(swatch+gap)','Math.floor(index/columns)*(Math.max(swatch,size*25.4/72.27)+gap)');if(chunk===after)throw Error('legend line box patch did not match');s=s.slice(0,start)+after+s.slice(end);writeFileSync(import.meta.dir+'/render.ts',s);renameSync(import.meta.dir+'/render.ts',path);