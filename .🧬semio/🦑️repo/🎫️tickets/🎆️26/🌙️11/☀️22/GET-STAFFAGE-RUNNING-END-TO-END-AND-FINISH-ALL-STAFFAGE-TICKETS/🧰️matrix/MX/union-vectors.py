"""🧷 Adds the three prism unions that threw `PolygonUnionError` (requests "multiview MH-4" and "multiview MH-6") to the vectors of the polygon union, once: `python3 union-vectors.py <union-repro.json> <fixture>`; the two polygons sets of `pregnant-person` go to `cases` (checked against shapely and polygon-clipping), the one of `tour-guide` with its two edge-on slivers to `slivers`."""
import json, pathlib, sys

NAMES = {
    ("pregnant-person", "clip waddle-walk key waddle-contact @0.133 s"): ("cases", "pregnant-person clip waddle-walk at 0.133 s 1:50 front, a shank prism swept along v with one profile edge along the bone: 11 polygons whose sweep edges stand within 23 nanometres of the vertical, which the slab union could not close in the given frame"),
    ("pregnant-person", "clip waddle-walk uniform 2/9 @0.289 s"): ("cases", "pregnant-person clip waddle-walk at 0.289 s 1:50 front, the same shank prism: sweep edges within 0.5 nanometres of the vertical"),
    ("tour-guide", "clip gesture-point key holding-flag @7.992 s"): ("slivers", "tour-guide clip gesture-point at 7.992 s 1:50 plan, a hand prism swept along w seen along its profile: 14 faces and the two 53-point end outlines as slivers 2.2 micrometres wide"),
}


def main() -> None:
    """✍️ Splices one line per vector before the end of its list."""
    found = json.loads(pathlib.Path(sys.argv[1]).read_text("utf8"))
    target = pathlib.Path(sys.argv[2])
    lines = target.read_text("utf8").split("\n")
    for entry in found:
        key, name = NAMES[(entry["asset"], entry["subject"])]
        if any(name in line for line in lines):
            continue
        vector = {"name": name, **({"rings": 1} if key == "slivers" else {}), "polygons": entry["polygons"]}
        start = next(i for i, line in enumerate(lines) if line.startswith(f' "{key}": ['))
        end = next(i for i in range(start + 1, len(lines)) if lines[i] in (" ]", " ],"))
        last = max(i for i in range(start + 1, end) if lines[i].strip())
        if not lines[last].rstrip().endswith(","):
            lines[last] = lines[last].rstrip() + ","
        lines.insert(end, "    " + json.dumps(vector, separators=(",", ":"), ensure_ascii=False))
    target.write_text("\n".join(lines), "utf8")
    json.loads(target.read_text("utf8"))
    print("vectors written")


main()
