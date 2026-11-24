from semio_patch import patch
L = "🧰️framework/🛍️products/📓️print/🖋️latex/"
patch(L + "semio-core.sty", [
    (r"""\vrule width\dimexpr\linewidth+2\semio@window@bodypad\relax height\semio@stroke@hairline}}}%""",
     r"""\vrule width\semio@list@closing@width\relax height\semio@stroke@hairline}}}%"""),
    (r"""    \par\global\semio@list@closingtrue""",
     r"""    \par\global\semio@list@closingtrue
    \xdef\semio@list@closing@width{\the\dimexpr\hsize+2\dimexpr\semio@window@bodypad\relax\relax}%"""),
])
print("ok")
