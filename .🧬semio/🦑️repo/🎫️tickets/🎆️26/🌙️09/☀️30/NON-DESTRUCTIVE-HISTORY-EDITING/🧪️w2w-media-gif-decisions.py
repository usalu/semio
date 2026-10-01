"""🖼️ W2-W-media follow-up: records the GIF raster-rule decisions in the 87a and 89a mutation features — representable
positive rows, a `@mode-error` refusal outline carrying the rows the specification forbids, and the decision text in place
of the former open-divergence notes. Idempotent only on a feature that still carries the old rows."""
import json

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards"
F87 = f"{ROOT}/7️⃣87a/🪆️subsets/✳️any/🧪️tests/🖼️mutate-gif-87a/🥒️.feature"
F89 = f"{ROOT}/9️⃣89a/🪆️subsets/🧱️base/🧪️tests/🎞️mutate-gif-89a/🥒️.feature"
FIXTURE87 = f"{ROOT}/7️⃣87a/🪆️subsets/✳️any/🧫️fixtures/🐘️dancing-87a-large/🖼️.gif"


def compact(value):
    return json.dumps(value, separators=(",", ":"))


def cut(text, start, end):
    a = text.index(start)
    b = text.index(end, a) + len(end)
    return text[:a], text[b:]


def replace_rows(text, kind, old, new):
    row = f"      | {kind} | {old} |\n"
    assert text.count(row) == 2, (kind, text.count(row))
    return text.replace(row, f"      | {kind} | {new} |\n")


def refuse_outline(given, title, rows):
    lines = ["  @id-refuse", "  @level-exhaustive", "  @mode-error", f"  Scenario Outline: {title} — <id>", f"    Given {given}", "    When the mutation is attempted with its parameters", '      """', '      {"kind": "<kind>", "params": <params>}', '      """', "    Then both implementations refuse it and the document is left exactly as it was", "    Examples:", "      | id | kind | params |"]
    lines += [f"      | {row_id} | {kind} | {params} |" for row_id, kind, params in rows]
    return "\n".join(lines) + "\n\n"


def gif87a():
    text = open(F87, encoding="utf-8").read()
    raw = open(FIXTURE87, "rb").read()
    size = 2 << (raw[10] & 7)
    table = raw[13 : 13 + 3 * size]
    inverted = {"gct": {"sorted": False, "colors": [{"r": 255 - table[i], "g": 255 - table[i + 1], "b": 255 - table[i + 2]} for i in range(0, 3 * size, 3)]}}
    four = '{"gct":{"sorted":false,"colors":[{"r":193,"g":108,"b":82},{"r":206,"g":161,"b":101},{"r":217,"g":120,"b":137},{"r":193,"g":108,"b":82}]}}'
    overhang = '{"index":0,"left":2,"top":2,"width":400,"height":400}'
    text = replace_rows(text, "set-global-color-table", four, compact(inverted))
    text = replace_rows(text, "set-image-geometry", overhang, '{"index":2,"left":10,"top":20,"width":32,"height":32}')
    head, tail = cut(text, "  ⚠️ KNOWN OPEN DIVERGENCE — `mutate-set-global-color-table`", "relaxed to make the row pass: not the profile, not the row's parameters, not the fixture.\n")
    text = head + """  ✅ DECIDED — what an edit may leave behind for the images that resolve through it. The GIF87a
  specification (https://www.w3.org/Graphics/GIF/spec-gif87.txt) states three rules a stream must keep:
  an image's "position and size must be confined to the dimensions defined by the Screen Descriptor"
  (Image Descriptor), its raster carries pixel indices "the number of which is equal to
  image-width*image-height" (Raster Data), and its pixel values index the colour map in force, whose
  entries number 2**(bits per pixel) (Global/Local Color Map). An edit that would break one of them for an
  image it touches is refused at mutation time with `mutation.target-mismatch`, never written and never
  repaired behind the caller's back: `set-global-color-table` refuses a table shorter than the highest
  index an image without a local map uses, `set-screen-size` refuses a screen an image would overhang,
  `set-image-geometry` refuses a rectangle the index buffer does not fill or the screen does not
  contain, and `set-image-pixels`/`insert-image`/`set-snapshot` refuse an image that breaks any of the
  three. The subject's vocabulary and the `gif` oracle implement the same rules independently, and the
  encoder keeps them as a codec guard. The mutate rows therefore install the fixture's own 256-entry
  table with every colour inverted and move the 32x32 image to (10,20); the rows the rules forbid —
  among them the former four-colour table, which image 0's indices up to 255 overrun — run in the
  `@id-refuse` outline below, where refusal by both sides is the passing outcome.
""" + tail
    marker = "  @id-identity-round-trip\n"
    assert text.count(marker) == 1
    rows = [
        ("set-global-color-table-orphaning-image-indices", "set-global-color-table", four),
        ("set-image-geometry-overhanging-the-screen", "set-image-geometry", overhang),
        ("set-image-geometry-without-matching-indices", "set-image-geometry", '{"index":2,"left":0,"top":0,"width":16,"height":16}'),
        ("set-screen-size-cropping-an-image", "set-screen-size", '{"width":300,"height":400}'),
    ]
    text = text.replace(marker, refuse_outline("the real GIF87a input document shared://🐘️dancing-87a-large/🖼️.gif", "An edit the GIF87a stream cannot carry is refused", rows) + marker)
    open(F87, "w", encoding="utf-8").write(text)


def gif89a():
    text = open(F89, encoding="utf-8").read()
    shrink = '{"width":801,"height":799}'
    resize = '{"index":0,"left":5,"top":5,"width":100,"height":100}'
    text = replace_rows(text, "set-screen-size", shrink, '{"width":820,"height":810}')
    text = replace_rows(text, "set-frame-geometry", resize, '{"index":1,"left":0,"top":0,"width":352,"height":401}')
    head, tail = cut(text, "  ⚠️ TWO KNOWN OPEN DIVERGENCES", "widen the profile, change these rows' parameters, or swap the fixture to close them.\n")
    text = head + """  ✅ DECIDED — what `set-screen-size` and `set-frame-geometry` do to the raster they would invalidate:
  they refuse. GIF89a (https://www.w3.org/Graphics/GIF/spec-gif89a.txt) says "Each image must fit within
  the boundaries of the Logical Screen, as defined in the Logical Screen Descriptor" (§20), and its Table
  Based Image Data holds "an index into the active color table, for each pixel in the image" (§22) — so
  a frame that overhangs the screen, an index buffer that does not fill the frame's rectangle, or an
  index past the active table (the frame's Local Color Table, else the Global Color Table, §19/§21) is
  not a GIF89a stream. An edit that would leave one of those behind for a frame it touches is refused at
  mutation time with `mutation.target-mismatch` — never clipped, resized, padded or written anyway. The
  subject's vocabulary and the `gif` oracle implement the same three rules independently (the oracle's
  former truncate-or-zero-pad of a resized frame's indices is gone), and `encode_gif` keeps them as a
  codec guard. The mutate rows therefore grow the screen to 820x810 and move frame 1 (352x401) to the
  origin; the former rows — the 801x799 screen frame 0 overhangs and frame 0 re-declared 100x100 over
  its 640 000 indices — run in the `@id-refuse` outline below, where refusal by both sides is the
  passing outcome.
""" + tail
    marker = "  @id-identity-round-trip\n"
    assert text.count(marker) == 1
    rows = [
        ("set-screen-size-cropping-frame-0", "set-screen-size", shrink),
        ("set-frame-geometry-without-matching-indices", "set-frame-geometry", resize),
        ("set-frame-geometry-overhanging-the-screen", "set-frame-geometry", '{"index":1,"left":500,"top":0,"width":352,"height":401}'),
    ]
    text = text.replace(marker, refuse_outline("the real input document asset://💃️dancing/🧪️dancing/🖼️.gif", "An edit the GIF89a stream cannot carry is refused", rows) + marker)
    open(F89, "w", encoding="utf-8").write(text)


gif87a()
gif89a()
print("[w2w-media] gif features updated")
