import itertools
import math
import re
import sys
from pathlib import Path
from collections import Counter
import pdfplumber
from pypdf import PdfReader
from pypdf.generic import ContentStream

MM = 72 / 25.4
OLD_HEIGHT_MM = 83.60209269444441


def forms(page):
    resources = page["/Resources"].get_object().get("/XObject", {})
    result = []
    for operands, operator in ContentStream(page.get_contents(), page.pdf).operations:
        if operator == b"Do":
            obj = resources[operands[0]]
            if obj.get("/Subtype") == "/Form":
                result.append((obj.indirect_reference.idnum, obj.indirect_reference.generation))
    return result


def no_raster(resources, seen):
    refs = resources["/XObject"].get_object().values() if "/XObject" in resources else []
    for ref in refs:
        obj = ref.get_object()
        identity = (ref.idnum, ref.generation)
        if identity in seen:
            continue
        seen.add(identity)
        assert obj.get("/Subtype") != "/Image", "Artwork must remain native vector"
        if "/Resources" in obj:
            no_raster(obj["/Resources"], seen)


def graph(page):
    candidates = [line for line in page.lines if abs(line.get("linewidth", 0)-.35) < .02 and line["top"] > 90]
    groups, edges = Counter(), []
    for line in candidates:
        pts = line.get("pts")
        assert pts and len(pts) == 2, "Graph edges must be straight two-point paths"
        a, b = [tuple(round(value, 2) for value in point) for point in pts]
        if a != b:
            edges.append((a, b))
            groups.update((a, b))
    centers = {point for point, degree in groups.items() if degree == 15}
    actual = [frozenset((a, b)) for a, b in edges if a in centers and b in centers]
    assert len(centers) == 16, f"Expected sixteen degree-15 vertices, got {len(centers)}: {groups}"
    assert len(actual) == 120 and len(set(actual)) == 120, f"Expected 120 unique straight edges, got {len(actual)}"
    assert set(actual) == {frozenset(pair) for pair in itertools.combinations(centers, 2)}, "Actual PDF graph is not K16"
    xs, ys = zip(*centers)
    cx, cy = (min(xs)+max(xs))/2, (min(ys)+max(ys))/2
    rx, ry = (max(xs)-min(xs))/2, (max(ys)-min(ys))/2
    assert abs(ry/MM-32) < .05, f"Vertical radius changed: {ry/MM:.3f}mm"
    assert rx > ry*2, "Graph must be a wide ellipse"
    for x, y in centers:
        assert abs(((x-cx)/rx)**2+((y-cy)/ry)**2-1) < .001, "Vertex is outside the common ellipse"
    graphic = [obj for kind in ("line", "curve", "rect") for obj in page.objects.get(kind, []) if obj["top"] > min(ys)-10*MM and obj["bottom"] < max(ys)+10*MM]
    labels = [c for c in page.chars if c["text"].isdigit() and c["top"] > min(ys)-12*MM and c["bottom"] < max(ys)+12*MM]
    bounds = (min(o["x0"] for o in graphic+labels), min(o["top"] for o in graphic+labels), max(o["x1"] for o in graphic+labels), max(o["bottom"] for o in graphic+labels))
    number_words = [word["text"] for word in page.extract_words() if word["text"].isdigit() and word["top"] >= bounds[1]-.1 and word["bottom"] <= bounds[3]+.1]
    assert Counter(number_words) == Counter(map(str, range(1,17))), f"Global vertex labels must appear once each: {number_words}"
    height = (bounds[3]-bounds[1])/MM
    assert abs(height-OLD_HEIGHT_MM) < 1, f"Graph height {height:.3f}mm differs from old {OLD_HEIGHT_MM:.3f}mm"
    assert abs(bounds[0]/MM-20) < .5 and abs(bounds[2]/MM-190) < .5, f"Graph must fill 170mm text width: {bounds}"
    ordered = sorted(centers, key=lambda p: math.atan2(p[0]-cx, cy-p[1]) % (2*math.pi))
    top_index = min(range(16), key=lambda i: ordered[i][1])
    ordered = ordered[top_index:]+ordered[:top_index]
    numbers = {point: i+1 for i, point in enumerate(ordered)}
    selected = []
    for line in page.lines:
        if abs(line.get("linewidth", 0)-.8) > .02 or len(line.get("pts", [])) != 2:
            continue
        endpoints = [tuple(round(value, 2) for value in point) for point in line["pts"]]
        if all(point in centers for point in endpoints):
            selected.append(tuple(sorted(numbers[point] for point in endpoints)))
    assert len(selected) == 20 and len(set(selected)) == 20, f"Expected twenty unique highlighted segments, got {selected}"
    ring = {(i, i+1) for i in range(1,16)} | {(1,16)}
    assert ring <= set(selected), "Every outer-ring edge must be selected"
    chords = set(selected)-ring
    assert chords == {(1,10), (2,12), (4,10), (5,13)}, f"Selected middle chords differ from fixed-seed sample: {chords}"
    return bounds, height, set(selected)


def ledger(page):
    entries = []
    for line in page.lines:
        if not (abs(line.get("linewidth", 0)-.3) < .02 and 20*MM < line["width"] < 90*MM and line["height"] < .1):
            continue
        crop = page.crop((max(0, line["x0"]-23*MM), line["top"]-5*MM, line["x0"]+1, line["top"]+1))
        text = crop.extract_text() or ""
        digits = re.findall(r"\d+", text)
        assert len(digits) == 2, f"Ledger line lacks one numeric edge label: {text!r}"
        a, b = map(int, digits)
        assert 1 <= a < b <= 16, f"Invalid global edge label: {a}, {b}"
        entries.append((a, b))
        blank = page.crop((line["x0"]+.5, line["top"]-3*MM, line["x1"]-.5, line["top"]-.2))
        assert not blank.chars, f"Ledger entry {a},{b} must remain writable"
    assert len(entries) == 20, f"The ledger needs 20 blank entries, found {len(entries)}"
    return entries


def verify(path):
    reader = PdfReader(path)
    assert len(reader.pages) == 3, "Expected three pages"
    assert reader.trailer["/Root"].get("/Lang") in ("de-DE", "en-US"), "Native language metadata missing"
    assert reader.metadata.title and "Parametr" in reader.metadata.title, "Assignment title metadata missing"
    matrix_forms, graph_forms = forms(reader.pages[1]), forms(reader.pages[2])
    for ref in reader.pages[2]["/Resources"]["/XObject"].get_object().values():
        obj = ref.get_object()
        if obj.get("/Subtype") == "/Form":
            no_raster(obj.get("/Resources", {}), set())
    assert len(graph_forms) == 16 and len(set(graph_forms)) == 16, f"Graph must place sixteen canonical vector forms exactly once: {graph_forms}"
    assert matrix_forms == graph_forms, "Graph icons must preserve canonical global matrix resource order"
    with pdfplumber.open(path) as pdf:
        bounds, height, selected = graph(pdf.pages[2])
        entries = ledger(pdf.pages[2])
        assert len(set(entries)) == 20 and set(entries) == selected, "Twenty ledger entries must match twenty highlighted graph edges exactly"
        print(f"PASS {Path(path).name}: 16 canonical vector nodes; 120 unique straight K16 edges; 16 ring edges + 4 middle chords; 20 matching blank ledger entries; graph {((bounds[2]-bounds[0])/MM):.3f} x {height:.3f}mm; vertical radius 32mm; native resources and metadata")


if __name__ == "__main__":
    for argument in sys.argv[1:]:
        verify(argument)
