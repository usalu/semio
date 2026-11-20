"""🌿 Build a browser/native clip fixture and independently render its PDF in MuPDF."""
import json
from pathlib import Path
import sys


WIDTH, HEIGHT = 180, 120


def rect(x, y, width, height):
    return ["re", x, y, width, height]


def scene_fixture():
    operations = []
    layers = []
    left = [["m", 10, 20], ["c", 10, 10, 70, 10, 80, 20], ["l", 80, 110], ["l", 10, 110], ["h"], rect(35, 45, 20, 30)]
    right = [["m", 100, 20], ["c", 100, 10, 160, 10, 170, 20], ["l", 170, 110], ["l", 100, 110], ["h"], rect(125, 45, 20, 30)]

    def segments(path):
        result = []
        point = lambda x, y: [float(x), float(HEIGHT - y)]
        for op, *args in path:
            if op == "re":
                x, y, w, h = args
                result.extend(segments([["m", x, y], ["l", x+w, y], ["l", x+w, y+h], ["l", x, y+h], ["h"]]))
            elif op == "c":
                result.append({"kind": "cubic", "ctrl1": point(*args[:2]), "ctrl2": point(*args[2:4]), "to": point(*args[4:])})
            elif op == "h":
                result.append({"kind": "close"})
            else:
                result.append({"kind": "move" if op == "m" else "line", "to": point(*args)})
        return result

    even = {"fillRule": "evenodd", "segments": segments(left)}
    winding = {"fillRule": "nonzero", "segments": segments(right)}
    quad = [[0.0, 120.0], [49.0, 120.0], [49.0, 0.0], [0.0, 0.0]]

    def paint(color, path, extra=None):
        operations.extend([["rg", *color], *path, ["f"]])
        layers.append({"segments": segments(path), "fill": {"color": list(map(float, color+[1]))}, "fillRule": "nonzero", **(extra or {})})

    operations.extend([["q"], *left, ["W*"], ["n"]])
    paint([1, 0, 0], [rect(0, 0, 90, 120)], {"clipPaths": [even]})
    operations.extend([["q"], rect(0, 0, 49, 120), ["W"], ["n"]])
    paint([0, 0, 1], [rect(0, 0, 90, 120)], {"clipPaths": [even], "clipPolygons": [quad]})
    operations.extend([["Q"]])
    paint([1, 0, 1], [rect(64, 60, 12, 20)], {"clipPaths": [even]})
    operations.extend([["Q"], ["q"], *right, ["W"], ["n"]])
    paint([1, 1, 0], [rect(90, 0, 90, 120)], {"clipPaths": [winding]})
    operations.extend([["Q"]])
    paint([0, 1, 0], [rect(2, 2, 8, 8)])
    return {
        "width": WIDTH, "height": HEIGHT, "ops": operations, "layers": layers,
        "probes": [
            {"at": [20, 30], "rgb": [0, 0, 255], "object": 1},
            {"at": [70, 30], "rgb": [255, 0, 0], "object": 0},
            {"at": [44, 50], "rgb": [255, 255, 255], "object": None},
            {"at": [51, 52], "rgb": [255, 255, 255], "object": None},
            {"at": [70, 50], "rgb": [255, 0, 255], "object": 2},
            {"at": [45, 103], "rgb": [0, 0, 255], "object": 1},
            {"at": [12, 107], "rgb": [255, 255, 255], "object": None},
            {"at": [135, 52], "rgb": [255, 255, 0], "object": 3},
            {"at": [175, 52], "rgb": [255, 255, 255], "object": None},
            {"at": [5, 115], "rgb": [0, 255, 0], "object": 4},
        ],
    }


def verify(path, raster):
    import fitz

    fixture = json.loads(path.read_text())
    with fitz.open() as doc:
        page = doc.new_page(width=fixture["width"], height=fixture["height"])
        stream = doc.get_new_xref()
        doc.update_object(stream, "<<>>")
        operators = fixture["ops"]
        if not isinstance(operators, str):
            operators = " ".join(" ".join([*map(str, args), op]) for op, *args in operators)
        doc.update_stream(stream, operators.encode("ascii"))
        page.set_contents(stream)
        pixmap = page.get_pixmap(colorspace=fitz.csRGB)
        for probe in fixture["probes"]:
            actual = list(pixmap.pixel(*probe["at"]))
            assert actual == probe["rgb"], (probe, actual)
        pixmap.save(raster)
    print(f"MuPDF matched {len(fixture['probes'])} PDF paint probes")


if __name__ == "__main__":
    path = Path(sys.argv[2])
    if sys.argv[1] == "build":
        path.write_text(json.dumps(scene_fixture(), indent=2) + "\n")
    else:
        verify(path, sys.argv[3])
