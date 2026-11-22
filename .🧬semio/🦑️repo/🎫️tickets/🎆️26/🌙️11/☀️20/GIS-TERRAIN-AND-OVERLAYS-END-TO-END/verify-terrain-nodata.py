"""🗻 Exercise the NoData fixture with exact decimal and triangular topology."""
import json
import re
import sys
from decimal import Decimal
from fractions import Fraction
from pathlib import Path

DECIMAL_TOKEN = re.compile(r"[+-]?(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)(?:[Ee][+-]?[0-9]+)?")
HEADERS = set("ncols nrows xllcenter yllcenter xllcorner yllcorner cellsize nodata_value".split())


def decimal(token):
    """🔢 Read one bounded base-ten token."""
    if len(token) > 64 or DECIMAL_TOKEN.fullmatch(token) is None:
        raise ValueError("decimal token")
    return Decimal(token)


def millimetres(value):
    """📏 Admit exact signed millimetres."""
    if not value:
        return 0
    if not -3 <= value.adjusted() <= 6:
        raise ValueError("millimetre range or precision")
    integral = Fraction(value) * 1000
    if integral.denominator != 1 or not -2147483648 <= integral <= 2147483647:
        raise ValueError("millimetre range or precision")
    return integral.numerator


def source_from_ascii(text):
    """🌐 Decode one bounded square-cell Esri grid."""
    tokens = iter(text.lstrip("\ufeff").split())
    header = {}
    for token in tokens:
        key = token.lower()
        if key not in HEADERS:
            samples = [token, *tokens]
            break
        if key in header:
            raise ValueError("duplicate header field")
        header[key] = next(tokens)
    else:
        raise ValueError("missing samples")
    size = decimal(header["cellsize"])
    width, height = (int(header[key]) for key in ("ncols", "nrows"))
    if not (0 < size and 2 <= width <= 65 and 2 <= height <= 65 and len(samples) == width * height):
        raise ValueError("grid bounds or dimensions")
    origins = {key for key in ("xllcenter", "yllcenter", "xllcorner", "yllcorner") if key in header}
    if origins == {"xllcenter", "yllcenter"}:
        west, south = (decimal(header[key]) for key in ("xllcenter", "yllcenter"))
    elif origins == {"xllcorner", "yllcorner"}:
        west, south = (decimal(header[key]) + size / 2 for key in ("xllcorner", "yllcorner"))
    else:
        raise ValueError("origin fields")
    nodata = decimal(header.get("nodata_value", "-9999"))
    mask = bytearray((width * height + 7) // 8)
    rows = []
    for row in range(height):
        heights = []
        for column in range(width):
            index = row * width + column
            value = decimal(samples[(height - 1 - row) * width + column])
            if value == nodata:
                mask[index // 8] |= 1 << (index % 8)
                heights.append(0)
            else:
                heights.append(millimetres(value))
        rows.append(heights)
    bounds = [west, south, west + (width - 1) * size, south + (height - 1) * size]
    if not (-180 <= bounds[0] < bounds[2] <= 180 and -90 <= bounds[1] < bounds[3] <= 90):
        raise ValueError("geographic bounds")
    result = {"bounds": list(map(float, bounds)), "rowsMillimetres": rows}
    if any(mask):
        result["missingSamples"] = mask.hex()
    return result


def missing(source):
    """🧩 Validate canonical missing bits and return south-first sample ordinals."""
    rows = source["rowsMillimetres"]
    heights = [height for row in rows for height in row]
    if "missingSamples" not in source:
        return set()
    text = source["missingSamples"]
    if not isinstance(text, str) or len(text) != 2 * ((len(heights) + 7) // 8) or re.fullmatch(r"[0-9a-f]+", text) is None:
        raise ValueError("invalid mask length or spelling")
    bits = int.from_bytes(bytes.fromhex(text), "little")
    if not 0 < bits < 1 << len(heights):
        raise ValueError("empty mask or nonzero tail bits")
    absent = {index for index, _ in enumerate(heights) if bits & (1 << index)}
    if any(heights[index] for index in absent):
        raise ValueError("nonzero payload at missing sample")
    return absent


def export_ascii(source, marker):
    """📤 Export north-first rows with a collision-free fixed marker."""
    rows = source["rowsMillimetres"]
    west, south, east, north = map(lambda value: Fraction(str(value)), source["bounds"])
    width, height = len(rows[0]), len(rows)
    size = (east - west) / (width - 1)
    if size != (north - south) / (height - 1):
        raise ValueError("non-square cells")
    header = f"NCOLS {width}\nNROWS {height}\nXLLCENTER {float(west)}\nYLLCENTER {float(south)}\nCELLSIZE {float(size)}\nNODATA_VALUE {marker}\n"
    absent = missing(source)
    return header + "".join(
        " ".join(str(marker) if row * width + column in absent else format(Decimal(sample).scaleb(-3), "f") for column, sample in enumerate(rows[row])) + "\n"
        for row in reversed(range(height))
    )


def faces(source):
    """🔺 Keep each existing-diagonal face with three valid vertices."""
    rows = source["rowsMillimetres"]
    width, height = len(rows[0]), len(rows)
    absent = {(height - 1 - row) * width + column for row, column in (divmod(index, width) for index in missing(source))}
    for row in range(height - 1):
        for column in range(width - 1):
            i = row * width + column
            for face in ([i, i + width, i + 1], [i + 1, i + width, i + width + 1]):
                if absent.isdisjoint(face):
                    yield face


def point_heights(source, xy):
    """📍 Return every incident face interpolation at a grid-space point."""
    rows = source["rowsMillimetres"]
    width = len(rows[0])
    heights = [sample for row in rows[::-1] for sample in row]
    x, y = map(lambda value: Fraction(str(value)), xy)
    for face in faces(source):
        (ay, ax), (by, bx), (cy, cx) = (divmod(vertex, width) for vertex in face)
        determinant = (by - cy) * (ax - cx) + (cx - bx) * (ay - cy)
        a = ((by - cy) * (x - cx) + (cx - bx) * (y - cy)) / determinant
        b = ((cy - ay) * (x - cx) + (ax - cx) * (y - cy)) / determinant
        c = 1 - a - b
        if min(a, b, c) >= 0:
            yield sum(weight * heights[vertex] for weight, vertex in zip((a, b, c), face))


def horizontal_drape(source, y):
    """🧭 Return connected horizontal portions supported by rendered faces."""
    width = len(source["rowsMillimetres"][0])
    y = Fraction(str(y))
    intervals = []
    for face in faces(source):
        positions = [divmod(vertex, width) for vertex in face]
        xs = []
        for (ay, ax), (by, bx) in zip(positions, positions[1:] + positions[:1]):
            if ay == by == y:
                xs.extend([ax, bx])
            elif ay != by and min(ay, by) <= y <= max(ay, by):
                xs.append(ax + (y - ay) * (bx - ax) / (by - ay))
        if xs and min(xs) < max(xs):
            intervals.append([min(xs), max(xs)])
    union = []
    for start, end in sorted(intervals):
        if union and start <= union[-1][1]:
            union[-1][1] = max(end, union[-1][1])
        else:
            union.append([start, end])
    return union


def compare_gdal(text, source, data_type="Float64", exact=True):
    """🛰️ Compare the same ASCII fixture with GDAL AAIGrid through Rasterio."""
    import numpy
    from rasterio import Env
    from rasterio.io import MemoryFile

    data = source["rowsMillimetres"]
    with Env(AAIGRID_DATATYPE=data_type), MemoryFile(text.encode("ascii")) as file, file.open() as raster:
        assert raster.driver == "AAIGrid"
        assert (raster.width, raster.height) == (len(data[0]), len(data))
        west, south = raster.xy(raster.height - 1, 0)
        east, north = raster.xy(0, raster.width - 1)
        numpy.testing.assert_allclose([west, south, east, north], source["bounds"], rtol=0, atol=1e-12)
        heights = raster.read(1, masked=True)[::-1]
        mask = numpy.ma.getmaskarray(heights)
        assert set(numpy.flatnonzero(mask)) == missing(source)
        if exact:
            numpy.testing.assert_array_equal(numpy.rint(heights.compressed() * 1000), numpy.asarray(data)[~mask])


def main():
    """🧪 Verify exact exchanges, faces, drape gaps and optional third-party agreement."""
    fixture = json.loads(Path(__file__).with_name("terrain-nodata.json").read_text(encoding="utf-8"))
    exports = Path(sys.argv[sys.argv.index("--exports") + 1]) if "--exports" in sys.argv else None
    for case in fixture["cases"]:
        source = source_from_ascii(case["asciiGrid"])
        assert source == case["source"], case["name"]
        assert list(faces(source)) == case["facesNorthFirst"], case["name"]
        for query in case.get("surfaceQueriesNorthFirst", []):
            expected = query["millimetres"]
            assert set(point_heights(source, query["xy"])) == (set() if expected is None else {expected}), query
        for route in case.get("horizontalDrapeNorthFirst", []):
            assert horizontal_drape(source, route["y"]) == route["intervals"], route
        export = export_ascii(source, fixture["exportNoDataMetres"])
        assert source_from_ascii(export) == source
        if "--gdal" in sys.argv:
            if case.get("gdal"):
                compare_gdal(case["asciiGrid"], source)
            compare_gdal(export, source)
            compare_gdal(export, source, "Float32", exact=False)
            if exports is not None:
                rust_export = (exports / f"{case['name']}.asc").read_text(encoding="ascii")
                assert source_from_ascii(rust_export) == source
                compare_gdal(rust_export, source)
                compare_gdal(rust_export, source, "Float32", exact=False)
    precise = fixture["cases"][2]["asciiGrid"]
    for token in ("1.000000000000000000000000000000001", "2e-400", "NaN"):
        try:
            source_from_ascii(precise.replace("1.0 -0", f"{token} -0"))
        except ValueError:
            continue
        raise AssertionError(f"accepted unrepresentable valid sample {token!r}")
    source = fixture["cases"][-1]["source"]
    for mask in fixture["invalidMasks"] + ["01"]:
        try:
            missing({**source, "missingSamples": mask})
        except ValueError:
            continue
        raise AssertionError(f"accepted malformed mask {mask!r}")
    large_header = "NCOLS 65\nNROWS 65\nXLLCENTER 5\nYLLCENTER 50\nCELLSIZE 0.125\nNODATA_VALUE -7\n"
    large_row = " ".join(["0"] * 65) + "\n"
    large = source_from_ascii(large_header + "-7" + large_row[1:] + large_row * 63 + large_row[:-2] + "-7\n")
    assert len(large["missingSamples"]) == 1058
    assert missing(large) == {64, 4160}
    assert len(list(faces(large))) == 8190
    assert source_from_ascii(export_ascii(large, fixture["exportNoDataMetres"])) == large
    assert large["rowsMillimetres"] == [[0] * 65 for _ in range(65)]
    try:
        missing({**fixture["cases"][0]["source"], "missingSamples": "02f4"})
    except ValueError:
        pass
    else:
        raise AssertionError("accepted nonzero padding in the second mask byte")
    print(f"Passed {len(fixture['cases'])} terrain NoData cases and the largest supported mask" + (" with GDAL comparisons" if "--gdal" in sys.argv else ""))


if __name__ == "__main__":
    main()
