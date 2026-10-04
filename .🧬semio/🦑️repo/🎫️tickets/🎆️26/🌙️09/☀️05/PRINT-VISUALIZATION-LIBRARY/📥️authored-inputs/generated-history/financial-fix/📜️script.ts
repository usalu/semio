import {readFileSync,writeFileSync,renameSync,mkdirSync} from 'node:fs';
const root='C:/git/semio/🧰️framework/🛍️products/📓️print',path=root+'/🖋️latex/semio-viz-charts-financial.sty';let source=readFileSync(path,'utf8');
source=source.replace('\\fp_new:N \\l_semio_viz_fin_box_fp','\\fp_new:N \\l_semio_viz_fin_box_fp\n\\fp_new:N \\l_semio_viz_fin_deviation_fp');
source=source.replace('  boxSize   .fp_set:N = \\l_semio_viz_fin_box_fp,','  boxSize   .fp_set:N = \\l_semio_viz_fin_box_fp,\n  deviation .fp_set:N = \\l_semio_viz_fin_deviation_fp,');
source=source.replace('  \\fp_set:Nn \\l_semio_viz_fin_box_fp { 1 }','  \\fp_set:Nn \\l_semio_viz_fin_box_fp { 1 }\n  \\fp_set:Nn \\l_semio_viz_fin_deviation_fp { 2 }');
const start=source.indexOf('% 〰️ The moving-average overlay'),end=source.indexOf('% 📐 The value axis of a price chart',start);
if(start<0||end<0)throw Error('financial replacement markers absent');
source=source.slice(0,start)+String.raw`% 📈 Trailing population deviations and price envelopes, retained independently of cartesian scratch state.
\seq_new:N \l_semio_viz_fin_deviations_seq
\seq_new:N \l_semio_viz_fin_upper_seq
\seq_new:N \l_semio_viz_fin_lower_seq
\seq_new:N \l_semio_viz_fin_mean_points_seq
\seq_new:N \l_semio_viz_fin_upper_points_seq
\seq_new:N \l_semio_viz_fin_lower_points_seq
\int_new:N \l_semio_viz_fin_index_int
\int_new:N \l_semio_viz_fin_first_int
\int_new:N \l_semio_viz_fin_sample_int
\fp_new:N \l_semio_viz_fin_mean_fp
\fp_new:N \l_semio_viz_fin_variance_fp
\fp_new:N \l_semio_viz_fin_x_fp
\fp_new:N \l_semio_viz_fin_y_fp
\msg_new:nnn {semio-viz}{financial-options}{Financial~window~must~be~positive~and~deviation~nonnegative.}

% 📊 Resolves moving means and population deviation before defining the price scale.
\cs_new_protected:Npn \semio_viz_fin_overlay_prepare: {
  \int_compare:nNnT {\l_semio_viz_fin_window_int} < {1} {\msg_fatal:nn{semio-viz}{financial-options}}
  \fp_compare:nNnT {\l_semio_viz_fin_deviation_fp} < {0} {\msg_fatal:nn{semio-viz}{financial-options}}
  \semio_viz_cart_window_mean:Nn \l_semio_viz_fin_closes_seq {\l_semio_viz_fin_window_int}
  \seq_set_eq:NN \l_semio_viz_fin_aux_seq \l_semio_viz_cart_out_seq
  \seq_clear:N \l_semio_viz_fin_deviations_seq
  \seq_clear:N \l_semio_viz_fin_upper_seq
  \seq_clear:N \l_semio_viz_fin_lower_seq
  \int_step_variable:nNn {\seq_count:N\l_semio_viz_fin_closes_seq} \l_semio_viz_fin_index_int {
    \fp_set:Nn \l_semio_viz_fin_mean_fp {\seq_item:Nn\l_semio_viz_fin_aux_seq{\l_semio_viz_fin_index_int}}
    \int_set:Nn \l_semio_viz_fin_first_int {max(1,\l_semio_viz_fin_index_int-\l_semio_viz_fin_window_int+1)}
    \fp_zero:N \l_semio_viz_fin_variance_fp
    \int_step_variable:nnNn {\l_semio_viz_fin_first_int}{\l_semio_viz_fin_index_int}\l_semio_viz_fin_sample_int {
      \fp_add:Nn \l_semio_viz_fin_variance_fp {(\seq_item:Nn\l_semio_viz_fin_closes_seq{\l_semio_viz_fin_sample_int}-\l_semio_viz_fin_mean_fp)^2}
    }
    \fp_set:Nn \l_semio_viz_fin_variance_fp {sqrt(\l_semio_viz_fin_variance_fp/(\l_semio_viz_fin_index_int-\l_semio_viz_fin_first_int+1))}
    \seq_put_right:Nx \l_semio_viz_fin_deviations_seq {\fp_use:N\l_semio_viz_fin_variance_fp}
    \seq_put_right:Nx \l_semio_viz_fin_upper_seq {\fp_eval:n{\l_semio_viz_fin_mean_fp+\l_semio_viz_fin_deviation_fp*\l_semio_viz_fin_variance_fp}}
    \seq_put_right:Nx \l_semio_viz_fin_lower_seq {\fp_eval:n{\l_semio_viz_fin_mean_fp-\l_semio_viz_fin_deviation_fp*\l_semio_viz_fin_variance_fp}}
    \str_if_eq:VnT \l_semio_viz_fin_overlay_tl {bollinger} {
      \fp_set:Nn \l_semio_viz_cart_ymin_fp {min(\l_semio_viz_cart_ymin_fp,\seq_item:Nn\l_semio_viz_fin_lower_seq{\l_semio_viz_fin_index_int})}
      \fp_set:Nn \l_semio_viz_cart_ymax_fp {max(\l_semio_viz_cart_ymax_fp,\seq_item:Nn\l_semio_viz_fin_upper_seq{\l_semio_viz_fin_index_int})}
    }
  }
}

% 🎯 Resolves each overlay value at its own authored period coordinate.
\cs_new_protected:Npn \semio_viz_fin_overlay_map:NN #1#2 {
  \seq_clear:N #2
  \int_zero:N \l_semio_viz_fin_index_int
  \seq_map_inline:Nn #1 {
    \int_incr:N \l_semio_viz_fin_index_int
    \tl_set:Nx \l_semio_viz_fin_cell_tl {\seq_item:Nn\l_semio_viz_fin_rows_seq{\l_semio_viz_fin_index_int}}
    \semio_viz_cart_xmap:xN {\clist_item:Vn\l_semio_viz_fin_cell_tl{1}}\l_semio_viz_fin_x_fp
    \semio_viz_cart_ymap:xN {\fp_eval:n{##1}}\l_semio_viz_fin_y_fp
    \seq_put_right:Nx #2 {\fp_use:N\l_semio_viz_fin_x_fp,\fp_use:N\l_semio_viz_fin_y_fp}
  }
}

% 〰️ Paints the population-deviation envelope and strokes its moving mean.
\cs_new_protected:Npn \semio_viz_fin_overlay_draw: {
  \semio_viz_probe_geometry:nx {window-mean}{\seq_use:Nn\l_semio_viz_fin_aux_seq{,}}
  \semio_viz_fin_overlay_map:NN \l_semio_viz_fin_aux_seq \l_semio_viz_fin_mean_points_seq
  \str_if_eq:VnT \l_semio_viz_fin_overlay_tl {bollinger} {
    \semio_viz_probe_values:nx {financial/deviation}{\seq_use:Nn\l_semio_viz_fin_deviations_seq{,}}
    \semio_viz_probe_values:nx {financial/upper}{\seq_use:Nn\l_semio_viz_fin_upper_seq{,}}
    \semio_viz_probe_values:nx {financial/lower}{\seq_use:Nn\l_semio_viz_fin_lower_seq{,}}
    \semio_viz_fin_overlay_map:NN \l_semio_viz_fin_upper_seq \l_semio_viz_fin_upper_points_seq
    \semio_viz_fin_overlay_map:NN \l_semio_viz_fin_lower_seq \l_semio_viz_fin_lower_points_seq
    \tl_set:Nn \l_semio_viz_cart_fill_tl {\semio_viz_theme_color:n{1}}
    \semio_viz_cart_draw_band:NN \l_semio_viz_fin_upper_points_seq \l_semio_viz_fin_lower_points_seq
  }
  \tl_set:Nn \l_semio_viz_cart_stroke_tl {\semio_viz_theme_color:n{2}}
  \semio_viz_cart_draw_path:N \l_semio_viz_fin_mean_points_seq
}

% 〰️ Maps a derived indicator through the same typed arithmetic boundary as price overlays.
\cs_new_protected:Npn \semio_viz_fin_overlay_points:n #1 {
  \semio_viz_fin_overlay_map:NN \l_semio_viz_fin_aux_seq \l_semio_viz_cart_pts_seq
}

`+source.slice(end);
source=source.replace('  \\semio_viz_cart_scale_value:nn { linear } { v }\n  \\semio_viz_cart_grid_y:',String.raw`  \str_if_eq:VnT \l_semio_viz_fin_indicator_tl {none} {
    \str_if_eq:VnF \l_semio_viz_fin_overlay_tl {none} {\semio_viz_fin_overlay_prepare:}
  }
  \semio_viz_cart_scale_value:nn { linear } { v }
  \semio_viz_cart_grid_y:`);
writeFileSync(import.meta.dir+'/financial.sty',source);renameSync(import.meta.dir+'/financial.sty',path);