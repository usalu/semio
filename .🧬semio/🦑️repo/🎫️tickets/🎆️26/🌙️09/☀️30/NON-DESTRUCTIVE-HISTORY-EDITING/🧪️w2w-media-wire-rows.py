#!/usr/bin/env python3
"""🧾️ W2-W media: derives the wire-form `params` cells the media `🥒️.feature` rows carry from the committed real
fixtures, so every byte a row states (MPEG frames, rasters) is read out of the fixture rather than typed by hand.

Usage: python3 🧪️w2w-media-wire-rows.py <target>   — prints one `| id | params |` row per line.
"""
import json
import struct
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
ARTIFACTS = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"


def compact(value):
    return json.dumps(value, separators=(",", ":"), ensure_ascii=False)


def row(kind, params):
    return f"      | {kind} | {compact(params)} |"


def mp3_frames(data, start, count):
    frames = []
    pos = start
    for _ in range(count):
        b1, b2, b3 = data[pos + 1], data[pos + 2], data[pos + 3]
        version, layer = (b1 >> 3) & 3, (b1 >> 1) & 3
        bitrate_index, rate_index, padding = (b2 >> 4) & 15, (b2 >> 2) & 3, (b2 >> 1) & 1
        assert (version, layer) == (3, 1)
        size = 144 * [0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320][bitrate_index] * 1000 // [44100, 48000, 32000][rate_index] + padding
        frames.append({
            "header": {
                "mpegVersionId": version, "layer": layer, "protectionBit": bool(b1 & 1), "bitrateIndex": bitrate_index,
                "sampleRateIndex": rate_index, "padding": bool(padding), "privateBit": bool(b2 & 1), "channelMode": (b3 >> 6) & 3,
                "modeExtension": (b3 >> 4) & 3, "copyright": bool((b3 >> 3) & 1), "original": bool((b3 >> 2) & 1), "emphasis": b3 & 3,
            },
            "payload": list(data[pos + 4:pos + size]),
        })
        pos += size
    return frames


def id3_text(identifier, text):
    return {"id": identifier, "flags": 0, "data": [0, *text.encode("latin-1")]}


def id3v1(title, artist, album, year, comment, genre):
    raw = bytearray(128)
    raw[0:3] = b"TAG"
    for value, start, width in ((title, 3, 30), (artist, 33, 30), (album, 63, 30), (year, 93, 4), (comment, 97, 30)):
        encoded = value.encode("latin-1")
        assert len(encoded) <= width
        raw[start:start + len(encoded)] = encoded
    raw[127] = genre
    return {"raw": list(raw)}


def mp3():
    data = (ARTIFACTS / "🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧫️fixtures/🔊️.mp3").read_bytes()
    audio = 10 + ((data[6] << 21) | (data[7] << 14) | (data[8] << 7) | data[9])
    tag = lambda frames: {"majorVersion": 3, "minorVersion": 0, "flags": 0, "frames": frames}
    return [
        row("set-snapshot", {"snapshot": {"schema": "stdio.mp3", "id3v2": tag([id3_text("TALB", "replaced wholesale")]), "frames": mp3_frames(data, audio, 3), "id3v1": id3v1("snapshot", "semio", "", "2026", "", 12)}}),
        row("set-id3v2", {"id3v2": tag([id3_text("TIT2", "renamed by the oracle"), id3_text("TPE1", "semio")])}),
        row("set-frames", {"frames": mp3_frames(data, audio, 2)}),
        row("set-id3v1", {"id3v1": id3v1("added trailer", "semio", "", "2026", "", 12)}),
    ]


def avi_strh(**fields):
    base = {"fccType": "vids", "fccHandler": "MJPG", "flags": 0, "priority": 0, "language": 0, "initialFrames": 0, "scale": 1, "rate": 15, "start": 0, "length": 0,
            "suggestedBufferSize": 0, "quality": -1, "sampleSize": 0, "rcFrameLeft": 0, "rcFrameTop": 0, "rcFrameRight": 0, "rcFrameBottom": 0, "rcFrameWidth": 16, "strhExtra": []}
    return {**base, **fields}


def avi_main_header(**fields):
    base = {"microSecPerFrame": 40000, "maxBytesPerSec": 1000, "paddingGranularity": 0, "flags": 16, "totalFrames": 0, "initialFrames": 0, "streams": 0,
            "suggestedBufferSize": 0, "width": 64, "height": 64, "reserved": [0, 0, 0, 0]}
    return {**base, **fields}


def avi():
    bitmap = {"format": "bitmapInfo", "size": 40, "width": 480, "height": 432, "planes": 1, "bitCount": 24, "compression": "MJPG", "sizeImage": 0, "xPelsPerMeter": 0,
              "yPelsPerMeter": 0, "colorsUsed": 0, "colorsImportant": 0}
    return [
        row("set-snapshot", {"snapshot": {"schema": "stdio.avi", "mainHeader": avi_main_header(), "streams": [], "idx1Present": False, "unknownChunks": [], "hdrlExtra": []}}),
        row("set-main-header", {"mainHeader": avi_main_header(microSecPerFrame=66666, maxBytesPerSec=25000, flags=2320, totalFrames=45, streams=1, suggestedBufferSize=1048576, width=960, height=864)}),
        row("insert-stream", {"index": 1, "stream": {"strh": avi_strh(), "strf": bitmap, "chunks": [], "strlExtra": []}}),
        row("remove-stream", {"index": 0}),
        row("set-stream-header", {"streamIndex": 0, "strh": avi_strh(priority=100, rate=30, length=45, suggestedBufferSize=21828, rcFrameRight=480, rcFrameBottom=432)}),
        row("set-stream-format", {"streamIndex": 0, "strf": {"format": "raw", "data": list(bytes.fromhex("deadbeef"))}}),
        row("insert-chunk", {"streamIndex": 0, "index": 1, "chunk": {"fourcc": "00dc", "data": list(bytes.fromhex("ffd8ffe0")), "keyframe": False}}),
        row("remove-chunk", {"streamIndex": 0, "index": 0}),
        row("set-chunk-keyframe", {"streamIndex": 0, "index": 0, "keyframe": False}),
        row("add-unknown-chunk", {"index": 2, "item": {"fourcc": "XTRA", "data": list(bytes.fromhex("cafef00d"))}}),
        row("remove-unknown-chunk", {"index": 1}),
        row("set-idx1-present", {"idx1Present": False}),
    ]


def mp4():
    committed = json.loads((ARTIFACTS / "🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📸️set-snapshot/🔑️promotes-the-second-sample-to-a-sync-frame/🦠️mutation/🔣️.json").read_text())["snapshot"]
    small = {**committed, "ftyp": {"majorBrand": "isom", "minorVersion": 42, "compatibleBrands": ["isom", "iso2", "avc1", "mp41"]}}
    track = {**committed["tracks"][0], "trackId": 2}
    return [
        row("set-snapshot", {"snapshot": small}),
        row("set-ftyp", {"ftyp": {"majorBrand": "mp42", "minorVersion": 1, "compatibleBrands": ["mp42", "isom"]}}),
        row("insert-track", {"index": 1, "track": track}),
        row("remove-track", {"index": 0}),
        row("set-track-dimensions", {"trackIndex": 0, "width": 640, "height": 480}),
        row("set-track-codec", {"trackIndex": 0, "codec": {"sps": [[103, 66, 0, 30, 140, 141, 64]], "pps": [[104, 206, 60, 128]], "nalLengthSize": 4, "extension": None}}),
        row("insert-sample", {"trackIndex": 0, "index": 10, "sample": {"data": [0, 0, 0, 4, 101, 1, 2, 3], "duration": 512, "ctsOffset": 0, "sync": False}}),
        row("remove-sample", {"trackIndex": 0, "index": 10}),
        row("set-sample-sync", {"trackIndex": 0, "index": 27, "sync": False}),
    ]


def bmp_pixels(path):
    data = path.read_bytes()
    width, height = struct.unpack_from("<ii", data, 18)
    return width, abs(height)


def bmp():
    width, height = bmp_pixels(ARTIFACTS / "🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🎨️replace-palette-entry-applied/⬅️before.bmp")
    return [
        row("change-header-fields", {"rowOrder": "topDown", "xPixelsPerMeter": 2835, "yPixelsPerMeter": 2835}),
        row("insert-palette-entry", {"index": 240, "entry": {"b": 10, "g": 20, "r": 30, "reserved": 0}}),
        row("remove-palette-entry", {"index": 239}),
        row("replace-palette-entry", {"index": 239, "entry": {"b": 1, "g": 2, "r": 3, "reserved": 0}}),
        row("replace-pixel-data", {"pixels": [200, 40, 40, 255] * (width * height)}),
    ]


def png_size(path):
    data = path.read_bytes()
    return struct.unpack_from(">II", data, 16)


def png():
    width, height = png_size(ARTIFACTS / "📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧫️fixtures/🎨️replace-palette-applied/⬅️before.png")
    text = lambda keyword, value: {"keyword": keyword, "value": value, "compressed": False, "kind": "text", "languageTag": "", "translatedKeyword": ""}
    return [
        row("change-header", {"width": 2334, "height": 2560, "bitDepth": 16, "colorType": "grayscale", "interlace": True}),
        row("replace-palette", {"plte": [{"r": 255, "g": 0, "b": 0}, {"r": 0, "g": 255, "b": 0}, {"r": 0, "g": 0, "b": 255}, {"r": 255, "g": 255, "b": 0}]}),
        row("change-transparency", {"trns": None}),
        row("change-gamma", {"gama": 45455}),
        row("change-chromaticities", {"chrm": {"whiteX": 31270, "whiteY": 32900, "redX": 64000, "redY": 33000, "greenX": 30000, "greenY": 60000, "blueX": 15000, "blueY": 6000}}),
        row("change-srgb-intent", {"srgb": "perceptual"}),
        row("change-physical-dims", {"phys": {"ppuX": 2835, "ppuY": 2835, "unitIsMeter": True}}),
        row("change-timestamp", {"time": {"year": 2024, "month": 1, "day": 2, "hour": 3, "minute": 4, "second": 5}}),
        row("change-background", {"bkgd": {"colorType": "rgb", "r": 255, "g": 255, "b": 255}}),
        row("insert-text-chunk", {"index": 0, "chunk": text("Comment", "Wave 7 oracle probe")}),
        row("remove-text-chunk", {"index": 0}),
        row("replace-text-chunk", {"index": 0, "chunk": text("Author", "replaces the arranged chunk outright")}),
        row("replace-pixels", {"pixels": [200, 40, 40, 255] * (width * height)}),
        row("insert-unknown-chunk", {"index": 0, "chunk": {"kind": list(b"waVe"), "data": list(b"wave7-probe")}}),
        row("remove-unknown-chunk", {"index": 0}),
    ]


TIFF_TYPES = ["byte", "ascii", "short", "long", "rational", "sByte", "undefined", "sShort", "sLong", "sRational", "float", "double"]


def tiff_tag(tag, code, values):
    kind = TIFF_TYPES[code - 1]
    return {"tag": tag, "kind": kind, "values": {"kind": kind, "value": values}}


def tiff_rgba(path):
    data = path.read_bytes()
    offset = struct.unpack_from("<I", data, 4)[0]
    tags = {}
    for entry in range(struct.unpack_from("<H", data, offset)[0]):
        tag, _, _, value = struct.unpack_from("<HHII", data, offset + 2 + 12 * entry)
        tags[tag] = value
    strip = data[tags[273]:tags[273] + tags[279]]
    assert len(strip) == tags[256] * tags[257] * 3
    return [channel for pixel in range(0, len(strip), 3) for channel in (*strip[pixel:pixel + 3], 255)]


def tiff():
    document = ARTIFACTS / "🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document"
    strip = bytes.fromhex("fefefefefefefefefefefefefefefef9f7f7f9f7f7fefefefefefefefefefefefefefefefcfbfbfaf7f7fbf7f7fbfafafefefefefefefefefefefefef7f4f4f8f3f3f9f6f6fefefefefefefefefefefefefbf9f9faf6f6faf7f7fdfdfdfefefefefefefefefefefefefaf7f7f8f8f8f8f6f6fbfafafefefefbf9f9fbf8f8fbf8f8f8f6f6fbfbfbfbfafaf9f6f6fdfcfcf9f6f6f9f5f5f8f4f4faf7f7f8f6f6faf8f8f8f4f4f8f5f5fcfcfcfbfafafbfafafbfafafcfbfbfcfbfbf8f4f4f9f6f6")
    return [
        row("change-byte-order", {"byteOrder": "bigEndian"}),
        row("insert-ifd", {"index": 2, "ifd": {"entries": [tiff_tag(256, 4, [8]), tiff_tag(257, 4, [8]), tiff_tag(258, 3, [8, 8, 8]), tiff_tag(259, 3, [1]), tiff_tag(262, 3, [2]), tiff_tag(277, 3, [3])], "pixels": list(strip)}}),
        row("remove-ifd", {"index": 1}),
        row("replace-tag", {"ifdIndex": 0, **tiff_tag(315, 2, "Derived for ticket 26/08/23/END-TO-END-TESTING-REFACTOR")}),
        row("remove-tag", {"ifdIndex": 0, "tag": 282}),
        row("replace-pixels", {"pixels": tiff_rgba(document / "🧫️fixtures/🔲️replace-pixels-applied/➡️after.tiff")}),
    ]


def tiff_baseline():
    snapshot = json.loads((ARTIFACTS / "🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/🧫️fixtures/📸️set-snapshot/⬅️before.json").read_text())
    stamps = {258: [16], 259: [5], 262: [6]}
    for entry in snapshot["ifds"][0]["entries"]:
        if entry["tag"] in stamps:
            entry["values"]["value"] = stamps[entry["tag"]]
    snapshot["pixels"] = [channel for gray in snapshot["pixels"] for channel in (gray, gray, gray, 255)]
    return [
        row("set-snapshot", {"snapshot": snapshot}),
        row("set-compression", {"compression": 5}),
        row("set-photometric-interpretation", {"photometric": 6}),
        row("set-bits-per-sample", {"bits": [16, 16, 16]}),
        row("insert-tile-tags", {"tileWidth": 256, "tileLength": 256}),
        row("remove-tile-tags", {}),
        row("set-strip-offsets", {"offsets": [8, 65536]}),
        row("remove-strip-offsets", {}),
    ]


def jpg_table(table_id, klass):
    return {"id": table_id, "class": klass, "bits": [0] * 16, "values": []}


def jpg_component(component_id):
    return {"id": component_id, "hSampling": 1, "vSampling": 1, "quantTableId": 0}


def jpg_width_height(path):
    data = path.read_bytes()
    at = 2
    while at < len(data):
        marker, length = data[at + 1], struct.unpack(">H", data[at + 2:at + 4])[0]
        if marker in (0xC0, 0xC1, 0xC2):
            height, width = struct.unpack(">HH", data[at + 5:at + 9])
            return width, height
        at += 2 + length
    raise ValueError(path)


def jpg():
    width, height = jpg_width_height(ARTIFACTS / "📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧫️fixtures/🔲️replace-pixels-applied/⬅️before.jpg")
    return [
        row("change-jfif-header", {"version": [1, 2], "densityUnits": "pixelsPerCm", "xDensity": 300, "yDensity": 300, "thumbnail": None}),
        row("replace-quant-table", {"table": {"id": 0, "precision": 0, "values": [12] * 64}}),
        row("remove-quant-table", {"id": 1}),
        row("replace-huffman-table", {"table": {"id": 0, "class": "dc", "bits": [9] * 16, "values": [9, 10]}}),
        row("remove-huffman-table", {"key": {"class": "ac", "id": 0}}),
        row("change-restart-interval", {"restartInterval": 16}),
        row("insert-other-segment", {"index": 0, "segment": {"marker": 226, "data": [7, 8]}}),
        row("remove-other-segment", {"index": 0}),
        row("replace-pixels", {"pixels": [9, 9, 9, 255] * (width * height)}),
        row("change-re-encode-quality", {"quality": 50}),
    ]


def jpg_baseline():
    snapshot = {
        "schema": "stdio.jpg", "width": 1, "height": 1, "pixels": [128, 128, 128, 255], "jfifVersion": [1, 1], "jfifDensityUnits": "aspect", "jfifXDensity": 1, "jfifYDensity": 1,
        "frame": {"precision": 12, "width": 1, "height": 1, "components": [jpg_component(1), jpg_component(2), jpg_component(3)]}, "sofMarker": 194, "arithmetic": True,
        "quantTables": [{"id": 0, "precision": 0, "values": [16] * 64}, {"id": 1, "precision": 0, "values": [17] * 64}],
        "huffmanTables": [jpg_table(0, "dc"), jpg_table(0, "ac"), jpg_table(1, "dc"), jpg_table(1, "ac")], "otherSegments": [],
    }
    return [
        row("set-snapshot", {"snapshot": snapshot}),
        row("set-sof-marker", {"marker": 194}),
        row("set-sample-precision", {"precision": 12}),
        row("set-arithmetic", {"arithmetic": True}),
        row("insert-huffman-table", {"index": 4, "table": jpg_table(2, "dc")}),
        row("remove-huffman-table", {"key": {"class": "dc", "id": 0}}),
        row("insert-frame-component", {"index": 3, "component": {**jpg_component(4)}}),
        row("remove-frame-component", {"id": 3}),
        row("set-component-sampling", {"id": 1, "hSampling": 5, "vSampling": 1}),
    ]


def gif_table(colors):
    return {"sorted": False, "colors": [{"r": r, "g": g, "b": b} for r, g, b in colors]}


def gif_frame(left, top, width, height, lct, indices, delay, disposal="unspecified"):
    return {"left": left, "top": top, "width": width, "height": height, "interlace": False, "lct": lct, "indices": indices, "delayCs": delay, "disposal": disposal, "transparentIndex": None, "userInput": False, "plainText": None}


def gif_crop(path, frame, left, top, size):
    from PIL import Image
    image = Image.open(path)
    image.seek(frame)
    palette = image.getpalette()
    crop = [image.getpixel((left + x, top + y)) for y in range(size) for x in range(size)]
    used = sorted(set(crop))
    table = [tuple(palette[3 * index:3 * index + 3]) for index in used]
    while len(table) & (len(table) - 1) or len(table) < 2:
        table.append(table[-1])
    return gif_table(table), [used.index(index) for index in crop]


def gif89a():
    base = ARTIFACTS / "🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base"
    lct, indices = gif_crop(base / "🖼️assets/💃️dancing/🧪️dancing/🖼️.gif", 0, 398, 398, 4)
    return [
        row("set-snapshot", {"snapshot": {"schema": "stdio.gif.89a", "width": 2, "height": 2, "gct": gif_table([(4, 5, 6), (4, 5, 6)]), "backgroundColorIndex": 0, "pixelAspectRatio": 0, "loopCount": 0,
                                         "frames": [gif_frame(0, 0, 2, 2, gif_table([(9, 9, 9), (9, 9, 9)]), [0, 1, 1, 0], 10, "doNotDispose")], "comments": ["c0"], "appExtensions": []}}),
        row("set-screen-size", {"width": 801, "height": 799}),
        row("set-global-color-table", {"gct": gif_table([(10, 20, 30), (40, 50, 60)])}),
        row("set-background-color-index", {"index": 3}),
        row("set-pixel-aspect-ratio", {"ratio": 12}),
        row("insert-frame", {"index": 10, "frame": gif_frame(398, 398, 4, 4, lct, indices, 33)}),
        row("remove-frame", {"index": 10}),
        row("move-frame", {"from": 5, "to": 20}),
        row("set-frame-geometry", {"index": 0, "left": 5, "top": 5, "width": 100, "height": 100}),
        row("set-frame-pixels", {"index": 0, "indices": [3, 2, 1, 0, 3, 2, 1, 0, 3, 2, 1, 0]}),
        row("set-frame-interlace", {"index": 1, "interlace": True}),
        row("set-frame-delay", {"index": 1, "delayCs": 250}),
        row("set-frame-disposal", {"index": 1, "disposal": "restoreToBackground"}),
        row("set-frame-transparency", {"index": 1, "transparentIndex": 3}),
        row("set-frame-user-input", {"index": 1, "userInput": True}),
        row("set-loop-count", {"loopCount": 5}),
        row("add-app-extension", {"index": 0, "extension": {"identifier": list(b"XMPDATA1"), "authCode": list(b"XMP"), "data": [1, 2, 3]}}),
        row("remove-app-extension", {"index": 0}),
    ]


def gif87a():
    feature = ARTIFACTS / "🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🧪️tests/🖼️mutate-gif-87a/🥒️.feature"
    rows, seen = [], set()
    for line in feature.read_text().splitlines():
        cells = [cell.strip() for cell in line.strip().strip("|").split("|")] if line.strip().startswith("|") else []
        if len(cells) != 2 or cells[0] in ("id", "no-mutation") or cells[0] in seen:
            continue
        seen.add(cells[0])
        params = json.loads(cells[1])
        if cells[0] == "set-snapshot":
            snapshot = params["snapshot"]
            params = {"snapshot": {"schema": "stdio.gif", "width": snapshot["width"], "height": snapshot["height"], "gct": snapshot["gct"], "backgroundColorIndex": snapshot["backgroundColorIndex"], "pixelAspectRatio": 0, "images": snapshot["images"]}}
        if cells[0] == "set-global-color-table":
            params = {"gct": {"sorted": False, **params["gct"]}}
        rows.append(row(cells[0], params))
    return rows


TARGETS = {"mp3": mp3, "avi": avi, "mp4": mp4, "bmp": bmp, "png": png, "tiff": tiff, "tiff-baseline": tiff_baseline, "jpg": jpg, "jpg-baseline": jpg_baseline, "gif89a": gif89a, "gif87a": gif87a}

if __name__ == "__main__":
    print("\n".join(TARGETS[sys.argv[1]]()))
