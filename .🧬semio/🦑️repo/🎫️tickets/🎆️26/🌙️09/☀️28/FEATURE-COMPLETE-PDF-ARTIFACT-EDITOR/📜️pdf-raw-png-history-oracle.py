"""🖼️ Build a masked PNG and independently verify the mounted PDF placement history."""

import argparse
import json
from pathlib import Path
import re

import fitz
from PIL import Image

TICKET = Path(__file__).resolve().parent
CASE = json.loads((TICKET / "🔣️pdf-raw-png-history.json").read_text())
DOWNLOADS = CASE["downloads"] | CASE["archiveDownloads"]
GEOMETRY = json.loads((TICKET / "image-authoring.json").read_text())["existing"]
INLINE = b"BI /W 1 /H 1 /CS /RGB /BPC 8 ID \x00\x00\x00 EI"


def make_input(destination):
    """🎨 Render deterministic color and alpha, with enough entropy to cross the small picker path."""
    width, height = CASE["width"], CASE["height"]
    palette = CASE["palette"]
    raster = bytearray()
    state = 1
    for i in range(width * height):
        state = (1664525 * state + 1013904223) & 0xffff_ffff
        raster.extend((state >> 24, state >> 16 & 255, state >> 8 & 255, palette[(state ^ i) & 3]))
    image = Image.frombytes("RGBA", (width, height), bytes(raster))
    image.save(destination, format="PNG")
    assert destination.stat().st_size > CASE["smallPickerByteLimit"]
    assert width * height * 3 * 4 > CASE["contiguousSampleByteLimit"]
    with Image.open(destination) as reopened:
        assert reopened.tobytes() == image.tobytes()
    return image


def image_row(book, size, samples, space):
    """🖼️ Write an independent DeviceRGB image or DeviceGray mask."""
    row = book.get_new_xref()
    width, height = size
    book.update_object(row, f"<< /Type /XObject /Subtype /Image /Width {width} /Height {height} /ColorSpace /{space} /BitsPerComponent 8 >>")
    book.update_stream(row, samples)
    return row


def paint(matrix, operation):
    """🎯 Paint one isolated test placement with its neutral matrix."""
    return ("q " + " ".join(map(str, GEOMETRY[matrix])) + " cm ").encode() + operation + b" Q\n"


def set_page(book, index, program, names):
    """📄 Write a reference content stream and its named resources."""
    page = book[index]
    contents = book.get_new_xref()
    book.update_object(contents, "<<>>")
    book.update_stream(contents, program)
    page.set_contents(contents)
    rows = " ".join(f"/{name} {xref} 0 R" for name, xref in names.items())
    book.xref_set_key(page.xref, "Resources", f"<< /XObject << {rows} >> >>")


def expected_output(baseline, source, names):
    """🎨 Build reference placements through MuPDF's independent PDF object interface."""
    with fitz.open(baseline) as book:
        prior, = [row[0] for row in book[0].get_images(full=True) if row[7] == "Before"]
        mask = image_row(book, source.size, source.getchannel("A").tobytes(), "DeviceGray")
        color = image_row(book, source.size, source.convert("RGB").tobytes(), "DeviceRGB")
        book.xref_set_key(color, "SMask", f"{mask} 0 R")
        resources = {"Before": prior, **dict.fromkeys(names, color)}
        reference = b"/SemioImage3 Do" if "SemioImage3" in names else b"/Before Do"
        inline = b"/SemioImage5 Do" if "SemioImage5" in names else INLINE
        content = paint("placement", reference) + paint("inlinePlacement", inline)
        content += paint("insertPlacement", b"/SemioImage1 Do")
        set_page(book, 0, content, resources)
        return book.tobytes()


def check_saved(directory, source):
    """🔎 Independently check the effects and exact sample owners of all mounted downloads."""
    baseline = directory / "image-source.pdf"
    with fitz.open(baseline) as original:
        untouched = original[CASE["untouchedPage"]].get_pixmap().samples
    pixels = source.convert("RGB").tobytes()
    alpha = source.getchannel("A").tobytes()
    for filename, names in DOWNLOADS.items():
        with fitz.open(directory / filename) as saved:
            assert not saved.is_repaired and len(saved) == 2, filename
            assert saved[CASE["untouchedPage"]].get_pixmap().samples == untouched, filename
            page = saved[0]
            painted = re.findall(rb"/(SemioImage[0-9]+)\s+Do\b", page.read_contents())
            assert sorted(name.decode("ascii") for name in painted) == names, (filename, painted)
            with fitz.open(stream=expected_output(baseline, source, names), filetype="pdf") as expected:
                assert page.get_pixmap().samples == expected[0].get_pixmap().samples, filename
            for name in names:
                rows = [row for row in page.get_images(full=True) if row[7] == name]
                assert len(rows) == 1, (filename, name, rows)
                xref, mask, width, height, *_ = rows[0]
                assert (width, height) == source.size and mask > 0, (filename, name)
                assert fitz.Pixmap(saved, xref).samples == pixels, (filename, name)
                assert fitz.Pixmap(saved, mask).samples == alpha, (filename, name)
    assert (directory / "image-inline.pdf").read_bytes() == (directory / "image-cancelled.pdf").read_bytes()
    print("[DEBUG] MuPDF verified all placed RGB and mask samples, both replacements, history, cancellation and the untouched page")


def self_test(directory, source):
    """🧪 Verify the independent oracle accepts six correct states and refuses a lost replacement."""
    directory.mkdir(parents=True, exist_ok=True)
    with fitz.open() as book:
        book.new_page(width=150, height=100)
        book.new_page(width=150, height=100)
        prior = image_row(book, (1, 1), b"\x00\x00\x00", "DeviceRGB")
        original = paint("placement", b"/Before Do")
        set_page(book, 0, original + paint("inlinePlacement", INLINE), {"Before": prior})
        set_page(book, 1, original, {"Before": prior})
        baseline = directory / "image-source.pdf"
        baseline.write_bytes(book.tobytes())
    for filename, names in DOWNLOADS.items():
        output = expected_output(baseline, source, names)
        (directory / filename).write_bytes(output)
    (directory / "image-cancelled.pdf").write_bytes((directory / "image-inline.pdf").read_bytes())
    check_saved(directory, source)
    for target, prior in (("image-replaced.pdf", "image-inserted.pdf"), ("image-archive-redo.pdf", "image-archive-undo.pdf")):
        path = directory / target
        accepted = path.read_bytes()
        path.write_bytes((directory / prior).read_bytes())
        try:
            check_saved(directory, source)
        except AssertionError as refusal:
            assert refusal.args[0][0] == path.name
        else:
            raise AssertionError("a lost replacement satisfied the mounted oracle")
        finally:
            path.write_bytes(accepted)

    with fitz.open() as reference:
        reference.new_page(width=612, height=792)
        color = image_row(reference, source.size, source.convert("RGB").tobytes(), "DeviceRGB")
        mask = image_row(reference, source.size, source.getchannel("A").tobytes(), "DeviceGray")
        reference.xref_set_key(color, "SMask", f"{mask} 0 R")
        set_page(reference, 0, paint("insertPlacement", b"/SemioImage1 Do"), {"SemioImage1": color})
        sample = directory / "hub-reference.pdf"
        sample.write_bytes(reference.tobytes())
    check_hub_export(sample, source)
    wrong_alpha = source.copy()
    alpha = wrong_alpha.getchannel("A")
    alpha.putpixel((0, 0), 255 - alpha.getpixel((0, 0)))
    wrong_alpha.putalpha(alpha)
    try:
        check_hub_export(sample, wrong_alpha)
    except AssertionError:
        pass
    else:
        raise AssertionError("Hub export oracle accepted a changed alpha sample")


def check_hub_export(path, source):
    """🛰️ Verify the downloaded pixels, alpha and placement after the peer reopens Check In."""
    with fitz.open(path) as book:
        assert not book.is_repaired and len(book) == 1
        page = book[0]
        painted = re.findall(rb"/(SemioImage[0-9]+)\s+Do\b", page.read_contents())
        assert len(painted) == 1, painted
        name = painted[0].decode("ascii")
        xref, mask, width, height, *_ = next(row for row in page.get_images(full=True) if row[7] == name)
        pixels = source.convert("RGB").tobytes()
        alpha = source.getchannel("A").tobytes()
        assert (width, height) == source.size and mask > 0
        assert fitz.Pixmap(book, xref).samples == pixels
        assert fitz.Pixmap(book, mask).samples == alpha
        matrix = re.search(rb"((?:[-+0-9.eE]+\s+){6})cm\s+/" + painted[0] + rb"\s+Do\b", page.read_contents())
        assert matrix is not None, "expected a direct image placement matrix"
        assert [float(value) for value in matrix.group(1).split()] == GEOMETRY["insertPlacement"]
    print("[DEBUG] MuPDF and Pillow verified all original RGB and alpha samples after Hub Check In")


def main():
    """🧪 Materialize the neutral input, then optionally check all mounted browser downloads."""
    cli = argparse.ArgumentParser()
    cli.add_argument("work", type=Path)
    mode = cli.add_mutually_exclusive_group()
    mode.add_argument("--check", action="store_true")
    mode.add_argument("--self-test", action="store_true")
    mode.add_argument("--hub-export", type=Path)
    args = cli.parse_args()
    args.work.mkdir(parents=True, exist_ok=True)
    output = args.work / "large-picked.png"
    source = make_input(output)
    print(f"[DEBUG] Prepared {source.width}×{source.height} masked PNG ({output.stat().st_size} bytes)")
    if args.check:
        check_saved(args.work, source)
    elif args.hub_export:
        check_hub_export(args.hub_export, source)
    elif args.self_test:
        self_test(args.work / "oracle-proof", source)


if __name__ == "__main__":
    main()
