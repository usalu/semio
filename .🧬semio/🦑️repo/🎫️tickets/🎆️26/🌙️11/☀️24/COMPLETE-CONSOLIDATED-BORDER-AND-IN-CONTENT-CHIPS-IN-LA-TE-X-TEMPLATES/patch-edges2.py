from semio_patch import patch
L = "🧰️framework/🛍️products/📓️print/🖋️latex/"
patch(L + "semio-core.sty", [
    (r"""\vrule width\dimexpr\hsize+2\semio@window@bodypad\relax height\semio@stroke@hairline}}}%
      \nointerlineskip
      \prevdepth\dimen@
      \endgroup
    \fi""", r"""\vrule width\dimexpr\linewidth+2\semio@window@bodypad\relax height\semio@stroke@hairline}}}%
      \nointerlineskip
      \prevdepth\dimen@
      \endgroup
    \fi"""),
])
print("ok")
