import io, os

P = os.path.join(r"C:\git\semio", "\U0001f9f0\ufe0fframework", "\U0001f6cd\ufe0fproducts", "\U0001f4d3\ufe0fprint", "\U0001f58b\ufe0flatex", "semio-viz-guide.sty")
with io.open(P, encoding="utf-8") as h:
    s = h.read()


def rep(a, b):
    global s
    assert a in s, a[:90]
    s = s.replace(a, b)


rep(
    r"""  \draw [ semio-chrome-border-normal, line~width=\semio@stroke@hairline ]
    ( \fp_eval:n { \l_semio_viz_pad_fp } , \fp_eval:n { \l_semio_viz_pad_fp } )
    rectangle ( \fp_eval:n { #1 - 2 } , \fp_eval:n { #2 - 2 } ) ;""",
    r"""  \semio_viz_guide_style:nN { grid } \l_semio_viz_guide_style_tl
  \use:x {
    \exp_not:N \draw [ \exp_not:V \l_semio_viz_guide_style_tl ]
      ( \fp_eval:n { \l_semio_viz_pad_fp } , \fp_eval:n { \l_semio_viz_pad_fp } )
      rectangle ( \fp_eval:n { #1 - 2 } , \fp_eval:n { #2 - 2 } ) ;
  }""",
)

rep(
    r"""      \exp_not:N \draw [ semio-chrome-border-emphasized , line~width=\semio@stroke@hairline ]
        ( \fp_eval:n { \l_semio_viz_legend_col_fp } , \fp_eval:n { \l_semio_viz_legend_row_fp } )""",
    r"""      \exp_not:N \draw [ \exp_not:V \l_semio_viz_guide_style_tl ]
        ( \fp_eval:n { \l_semio_viz_legend_col_fp } , \fp_eval:n { \l_semio_viz_legend_row_fp } )""",
)

rep(
    r"""    \semio_viz_scale_map:VnN \l_semio_viz_legend_scale_tl {##1} \l_semio_viz_legend_value_fp""",
    r"""    \semio_viz_guide_style:nN { domain } \l_semio_viz_guide_style_tl
    \semio_viz_scale_map:VnN \l_semio_viz_legend_scale_tl {##1} \l_semio_viz_legend_value_fp""",
)

rep(
    r"""      \exp_not:N \fill [ semio-secondary ! \fp_eval:n { 100 - ( ##1 - 0.5 ) / \l_semio_viz_legend_slices_int * 100 } ! semio-primary ]""",
    r"""      \exp_not:N \fill [ \exp_not:V \l_semio_viz_legend_color_tl ]""",
)

rep(
    r"""  \int_step_inline:nn { \l_semio_viz_legend_slices_int } {
    \use:x {""",
    r"""  \int_step_inline:nn { \l_semio_viz_legend_slices_int } {
    \semio_viz_theme_ramp_color:nN
      { \fp_eval:n { ( ##1 - 0.5 ) / \l_semio_viz_legend_slices_int } } \l_semio_viz_legend_color_tl
    \use:x {""",
)

rep(
    r"""    \semio_viz_scale_color:VnN \l_semio_viz_legend_scale_tl {##1} \l_semio_viz_legend_color_tl""",
    r"""    \semio_viz_theme_color:nN { \l_semio_viz_legend_index_int } \l_semio_viz_legend_color_tl""",
)

rep(
    "% \U0001f3a8 The categorical colour of a domain entry, taken from a scale name held in a tl.\n"
    r"\cs_generate_variant:Nn \semio_viz_scale_color:nnN { VnN }" + "\n",
    "",
)

rep(
    r"""  \use:x {
    \exp_not:N \draw [ semio-primary , line~width=\exp_not:N \semio@stroke@default ]
      \exp_not:V \l_semio_viz_guide_arc_tl ;
  }""",
    r"""  \semio_viz_theme_color:nN { 0 } \l_semio_viz_guide_chrome_tl
  \use:x {
    \exp_not:N \draw [ \exp_not:V \l_semio_viz_guide_chrome_tl ,
                       line~width=\semio_viz_theme_stroke: ]
      \exp_not:V \l_semio_viz_guide_arc_tl ;
  }""",
)

with io.open(P, "w", encoding="utf-8", newline="\n") as h:
    h.write(s)
print("ok")
