"""🎨️ One-off derivation (ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 22).

Composes ONE SVG document out of the bodies of TWO real committed drawings of this repository,
byte for byte and in their own order, with no element, attribute or character invented:

* `🧰️framework/🔨️modules/🖼️assets/🪧️logos/🔣️logo_dark.svg` — the repository's real animated brand
  logo (136 854 bytes, 23 real `<g>` groups, 23 real `<path>` shapes, 69 real `<animate>` and 69 real
  `<animateTransform>` elements, a real `<title>`) — supplies the document element with its own real
  `viewBox`/`version`/`xmlns` attributes and the first 49 children.
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🧫️fixtures/mouse.svg` — the real onboarding mouse graphic
  the framework's own UI renders, and the ONLY committed SVG in this repository that declares a
  `<clipPath>` at all — supplies the remaining 15 children, its real comment, its real
  `<clipPath id="introduction-demo-mouse-clip">` inside a real `<defs>`, the real `<g>` that
  references it through `clip-path="url(#…)"`, and its four real `<path>` shapes.

Why compose rather than choose: SVG Basic 1.1's distinguishing rule is the clip-path mechanism, and
the mouse is the only real committed drawing that exercises it — but at 1 463 bytes it places every
mutation at the document's edge. Every larger real SVG in this repository (the logos, the QR code,
the metabolism icons) declares no clip path, so no single committed file is both. The composition
keeps both properties and invents neither: the output's body is the concatenation of the two real
bodies, and `mouse.svg` is still read where it is committed.
"""

from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
LOGO = ROOT / "🧰️framework/🔨️modules/🖼️assets/🪧️logos/🔣️logo_dark.svg"
MOUSE = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🧫️fixtures/mouse.svg"
OUT = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🧫️fixtures/🎨️semio-brand-and-onboarding.svg"


def body(text):
    """✂️ Everything between the document element's own start tag and its end tag, verbatim."""
    start = text.index(">", text.index("<svg")) + 1
    end = text.rindex("</svg>")
    return text[:start], text[start:end]


def main():
    logo_head, logo_body = body(LOGO.read_text(encoding="utf-8"))
    _, mouse_body = body(MOUSE.read_text(encoding="utf-8"))
    OUT.write_text(logo_head + logo_body + mouse_body + "</svg>", encoding="utf-8")
    print("wrote %d bytes (logo body %d + mouse body %d)" % (OUT.stat().st_size, len(logo_body.encode("utf-8")), len(mouse_body.encode("utf-8"))))


if __name__ == "__main__":
    main()
