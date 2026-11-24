from semio_patch import patch
L = "🧰️framework/🛍️products/📓️print/🖋️latex/"
patch(L + "semio-core.sty", [
    (r"""\vrule width\semio@list@closing@width\relax height\semio@stroke@hairline}}}%""",
     r"""\vrule width\semio@list@closing@width\space height\semio@stroke@hairline}}}%"""),
])
print("ok")
