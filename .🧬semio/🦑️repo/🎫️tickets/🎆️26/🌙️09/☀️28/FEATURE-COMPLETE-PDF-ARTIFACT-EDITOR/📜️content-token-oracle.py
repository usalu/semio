"""🔬 Independently renders the authored PDF operator round-trip fixture using MuPDF."""

import fitz

PROGRAM = b"q 1.25 w 20 30 m 70 90 l S BT /F1 14 Tf 1 0 0 1 30 100 Tm (Hello PDF) Tj ET Q"


def main():
    with fitz.open() as document:
        page = document.new_page(width=144, height=144)
        font = document.get_new_xref()
        document.update_object(font, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>")
        document.xref_set_key(page.xref, "Resources", f"<< /Font << /F1 {font} 0 R >> >>")
        stream = document.get_new_xref()
        document.update_object(stream, "<<>>")
        document.update_stream(stream, PROGRAM)
        document.xref_set_key(page.xref, "Contents", f"{stream} 0 R")
        original = document.tobytes()
    with fitz.open(stream=original, filetype="pdf") as reopened:
        page = reopened[0]
        assert page.get_text().strip() == "Hello PDF"
        picture = page.get_pixmap()
        painted = picture.pixel(45, 84)
        background = picture.pixel(48, 87)
        assert max(painted) < 160 and min(background) == 255, (painted, background)
        print(f"PyMuPDF {fitz.VersionBind}: Hello PDF, line sample {painted}, background {background}")


if __name__ == "__main__":
    main()
