import sys
from pathlib import Path
import pdfplumber
from pypdf import PdfReader

MM = 72 / 25.4


def unique(values):
    return sorted({round(value, 2) for value in values})


def grids(page):
    horizontal = [e for e in page.edges if e["orientation"] == "h" and e["width"] > 400]
    groups = {}
    for edge in horizontal:
        groups.setdefault((round(edge["x0"], 1), round(edge["x1"], 1)), []).append(edge)
    result = []
    for full in groups.values():
        boundaries = unique(e["top"] for e in full)
        rows = []
        for top, bottom in zip(boundaries, boundaries[1:]):
            vertical = [e for e in page.edges if e["orientation"] == "v" and abs(e["top"]-top) < .5 and abs(e["bottom"]-bottom) < .5]
            xs = unique(e["x0"] for e in vertical)
            if len(xs) == 10:
                rows.append((top, bottom, xs))
        runs = []
        for row in rows:
            if not runs or abs(runs[-1][-1][1]-row[0]) > .3:
                runs.append([])
            runs[-1].append(row)
        for run in runs:
            if len(run) == 9:
                assert all(row[2] == run[0][2] for row in run), "Column borders must align"
                result.append((run[0][2], [run[0][0]] + [row[1] for row in run]))
    return result


def artwork(page, cell):
    left, top, right, bottom = cell
    curves = [c for c in page.curves if c["x0"] > left+.2 and c["x1"] < right-.2 and c["top"] > top+.2 and c["bottom"] < bottom-.2]
    assert curves, "Each header requires vector artwork"
    return (min(c["x0"] for c in curves), min(c["top"] for c in curves), max(c["x1"] for c in curves), max(c["bottom"] for c in curves))


def verify(path):
    reader = PdfReader(path)
    assert reader.trailer["/Root"].get("/Lang") in ("de-DE", "en-US"), "Missing native document language"
    assert reader.metadata.title and "Parametr" in reader.metadata.title, "Missing assignment title metadata"
    with pdfplumber.open(path) as pdf:
        assert len(pdf.pages) == 3, f"Expected three pages, found {len(pdf.pages)}"
        text = "\n".join(page.extract_text() or "" for page in pdf.pages)
        for obsolete in ("Übung 1", "Übung 2", "Exercise 1", "Exercise 2"):
            assert obsolete not in text, f"Obsolete exercise numbering: {obsolete}"
        page = pdf.pages[1]
        matrices = grids(page)
        assert len(matrices) == 2, f"Page 2: expected two disjoint 9×9 grids, found {len(matrices)}"
        matrices.sort(key=lambda grid: grid[1][0])
        assert matrices[0][1][-1] < matrices[1][1][0], "Matrices must be vertically disjoint"
        minimum_insets = []
        for index, (xs, ys) in enumerate(matrices, 1):
            heights = [(b-a)/MM for a,b in zip(ys[1:], ys[2:])]
            assert all(5.8 < h < 6.7 for h in heights), f"Matrix {index}: writing rows must be single-line height, got {heights}"
            blank = page.crop((xs[0]+1, ys[1]+1, xs[-1]-1, ys[-1]-1))
            assert not blank.chars and not blank.curves, f"Matrix {index}: parameter rows must be blank"
            insets = []
            offsets = []
            for column in range(1, 9):
                cell = (xs[column], ys[0], xs[column+1], ys[1])
                art = artwork(page, cell)
                dx = abs((art[0]+art[2]-cell[0]-cell[2])/2)/MM
                dy = abs((art[1]+art[3]-cell[1]-cell[3])/2)/MM
                assert dx < .3 and dy < .3, f"Matrix {index}, icon {column}: artwork off center {dx:.3f}×{dy:.3f}mm"
                offsets.append((dx, dy))
                inset = min(art[0]-cell[0], art[1]-cell[1], cell[2]-art[2], cell[3]-art[3])/MM
                assert inset >= 1.8, f"Matrix {index}, icon {column}: insufficient artwork padding {inset:.3f}mm"
                insets.append(inset)
            assert max(insets)-min(insets) < .35, f"Matrix {index}: inconsistent artwork minimum inset {insets}"
            minimum_insets.extend(insets)
            print(f"Matrix {index}: 9×9 grid; rows {min(heights):.2f}–{max(heights):.2f}mm; icon minimum padding {min(insets):.2f}–{max(insets):.2f}mm; maximum center offset {max(dx for dx, dy in offsets):.3f}×{max(dy for dx, dy in offsets):.3f}mm")
        assert max(minimum_insets)-min(minimum_insets) < .35, "Both matrices must use uniform minimum artwork padding"
        assert not grids(pdf.pages[2]), "The second matrix must remain on page 2"
    print(f"PASS {Path(path).name}: three pages, two compact matrices, centered vector artwork")


if __name__ == "__main__":
    for argument in sys.argv[1:]:
        verify(argument)
