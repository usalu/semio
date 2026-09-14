"""🧹 Keeps the GUIDES probe fixtures minimal as the kernel loses its dependency on the document
class: semio-viz-theme resolves chrome colours from the design tokens and semio-viz-guide loads
semio-fonts itself, so a probe needs neither semio-core's begindocument hook nor a \\subtitle stub."""
import io, os, glob

DROP = [
    "\\ExplSyntaxOn\n\\tl_set:Nn \\l_semio_theme_tl { light }\n\\semio_theme_apply:\n\\ExplSyntaxOff\n",
    "\\makeatletter\\semio@set@main@fonts\\makeatother\n",
    "\\RemoveFromHook{begindocument/before}[semio-core]\n",
    "\\providecommand{\\subtitle}[1]{}\n",
]

ROOTS = [
    os.path.join(os.path.dirname(os.path.abspath(__file__)), "\U0001f5d1\ufe0fgenerated", "GUIDES"),
    os.path.join(r"C:\git\semio", "\U0001f9f0\ufe0fframework", "\U0001f6cd\ufe0fproducts", "\U0001f4d3\ufe0fprint", "\U0001f9ea\ufe0ftests"),
]

changed = []
for root in ROOTS:
    for path in glob.glob(os.path.join(root, "**", "*.tex"), recursive=True):
        with io.open(path, encoding="utf-8") as handle:
            text = handle.read()
        updated = text
        for fragment in DROP:
            updated = updated.replace(fragment, "")
        if updated != text:
            with io.open(path, "w", encoding="utf-8", newline="\n") as handle:
                handle.write(updated)
            changed.append(os.path.basename(path))
print("rewired", changed)
