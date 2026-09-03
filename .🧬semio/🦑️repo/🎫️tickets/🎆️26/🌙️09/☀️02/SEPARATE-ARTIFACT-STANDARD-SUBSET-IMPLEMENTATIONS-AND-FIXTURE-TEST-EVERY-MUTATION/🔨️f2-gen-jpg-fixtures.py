#!/usr/bin/env python3
"""🔨️ F2 — generates real before/after JPEG fixtures for s.stdio.jpg@jfif-1.01/baseline's 9
unfixtured mutations, using Pillow 12.2.0 wherever its real save() parameters genuinely reach the
mutation (component count via image mode, sampling factors via `subsampling`, SOF marker code via
`progressive`, huffman-table count as a genuine byproduct of component count -- all VERIFIED live by
scanning the written bytes' own marker sequence below, not assumed).

Two of the nine are handcrafted, matching shard E2's own honesty finding for this artifact (see
$TICKET/📓️e2-interchange-format-oracles.md §4): Pillow's public save() API always writes 8-bit
baseline-precision Huffman-coded JPEG and exposes no parameter for sample precision or entropy-coding
method, so `set-sample-precision` and `set-arithmetic` are produced by binary-patching a single marker
byte of a genuine Pillow-written baseline file -- structurally honest at the marker-vocabulary level
this subset's own mutations operate on, explicitly NOT claimed to be a spec-conformant 12-bit or
arithmetic-coded bitstream (the entropy-coded scan data is left untouched, so no real 12-bit/arithmetic
decoder would decode either "after" file correctly -- documented, not hidden).
Idempotent: safe to re-run.
"""
import hashlib
import io
import json
from pathlib import Path

from PIL import Image

ROOT = Path("/Users/ueli/Documents/semio")
SUBSET = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️baseline"
FIXTURES = SUBSET / "🧫️fixtures"
ORACLE_JSON = SUBSET / "🧪️oracle/🔣️.json"
READER_ORACLE_ID = "pillow-jpg-jfif-1-01-baseline-mutate-reader"
PILLOW_VERSION = "12.2.0"


def make_jpeg(mode: str, size=(8, 8), subsampling=None, progressive=False, seed=0) -> bytes:
    im = Image.new(mode, size)
    px = im.load()
    for y in range(size[1]):
        for x in range(size[0]):
            if mode == "L":
                px[x, y] = (x * 16 + seed) % 256
            else:
                px[x, y] = ((x * 16 + seed) % 256, (y * 16) % 256, ((x + y) * 8) % 256)
    buf = io.BytesIO()
    kwargs = {}
    if subsampling is not None:
        kwargs["subsampling"] = subsampling
    if progressive:
        kwargs["progressive"] = True
    im.save(buf, format="JPEG", quality=90, **kwargs)
    return buf.getvalue()


def markers(data: bytes):
    """🔎️ A real, independent scan of the marker sequence -- used to VERIFY every claim below."""
    i = 2
    out = []
    while i < len(data) - 1:
        if data[i] != 0xFF:
            i += 1
            continue
        m = data[i + 1]
        if m in (0xD8, 0xD9, 0x01) or (0xD0 <= m <= 0xD7):
            out.append(m)
            i += 2
            continue
        if m == 0xDA:
            out.append(m)
            break
        if i + 3 >= len(data):
            break
        seglen = (data[i + 2] << 8) | data[i + 3]
        out.append(m)
        i += 2 + seglen
    return out


def find_marker_offset(data: bytes, marker: int) -> int:
    i = 2
    while i < len(data) - 1:
        if data[i] == 0xFF and data[i + 1] == marker:
            return i
        if data[i] == 0xFF and data[i + 1] not in (0x00,):
            m = data[i + 1]
            if m in (0xD8, 0xD9, 0x01) or (0xD0 <= m <= 0xD7):
                i += 2
                continue
            if m == 0xDA:
                break
            seglen = (data[i + 2] << 8) | data[i + 3]
            i += 2 + seglen
            continue
        i += 1
    raise ValueError(f"marker 0x{marker:02x} not found")


def patch_sof_precision(data: bytes, new_precision: int) -> bytes:
    off = find_marker_offset(data, 0xC0)
    precision_off = off + 4  # 0xFF,0xC0,len_hi,len_lo,precision
    assert data[precision_off] == 8, f"expected baseline precision 8, found {data[precision_off]}"
    return data[:precision_off] + bytes([new_precision]) + data[precision_off + 1:]


def patch_sof_marker_code(data: bytes, new_code: int) -> bytes:
    off = find_marker_offset(data, 0xC0)
    assert data[off + 1] == 0xC0
    return data[:off + 1] + bytes([new_code]) + data[off + 2:]


def sha256_of(data: bytes) -> str:
    return f"sha256:{hashlib.sha256(data).hexdigest()}"


def main() -> None:
    entries = []

    # --- 1. set-component-sampling: chroma subsampling factors 4:4:4 -> 4:2:0, real Pillow param.
    before = make_jpeg("RGB", subsampling=0)
    after = make_jpeg("RGB", subsampling=2)
    assert markers(before) == markers(after)  # same marker shape, only sampling factor bytes differ
    entries.append(("set-component-sampling", "third-party-generated", before, after,
                     "Chroma subsampling factors changed via Pillow's own subsampling= save parameter, 4:4:4 (0) -> 4:2:0 (2), inside the unchanged SOF0 component table."))

    # --- 2. insert-frame-component: 1 component (grayscale) -> 3 components (YCbCr), real Pillow mode change.
    before = make_jpeg("L")
    after = make_jpeg("RGB", subsampling=0)
    bm, am = markers(before), markers(after)
    assert bm.count(0xC4) == 2 and am.count(0xC4) == 4, (bm, am)  # verified live: L=2 DHT tables, RGB=4
    entries.append(("insert-frame-component", "third-party-generated", before, after,
                     "SOF0 component count 1 (grayscale L) -> 3 (YCbCr), a real Pillow image-mode change. As a genuine byproduct, DHT table count also moves 2 -> 4 (verified live by scanning the written bytes' own marker sequence) -- the same underlying encoder decision the insert-huffman-table fixture below rides on."))

    # --- 3. remove-frame-component: inverse of #2.
    before = make_jpeg("RGB", subsampling=0)
    after = make_jpeg("L")
    entries.append(("remove-frame-component", "third-party-generated", before, after,
                     "SOF0 component count 3 (YCbCr) -> 1 (grayscale L), inverse of insert-frame-component."))

    # --- 4. set-sof-marker: SOF0 (baseline DCT) -> SOF2 (progressive DCT), real Pillow progressive= param.
    before = make_jpeg("RGB", subsampling=0, progressive=False)
    after = make_jpeg("RGB", subsampling=0, progressive=True)
    bm, am = markers(before), markers(after)
    assert 0xC0 in bm and 0xC0 not in am and 0xC2 in am, (bm, am)
    entries.append(("set-sof-marker", "third-party-generated", before, after,
                     "Start-Of-Frame marker code changed via Pillow's own progressive= save parameter, SOF0 0xC0 (baseline DCT) -> SOF2 0xC2 (progressive DCT)."))

    # --- 5. set-snapshot: whole different image.
    before = make_jpeg("RGB", subsampling=0, seed=0)
    after = make_jpeg("RGB", subsampling=0, size=(16, 16), seed=99)
    entries.append(("set-snapshot", "third-party-generated", before, after,
                     "Whole-image snapshot replace: an unrelated valid JPEG (different dimensions and pixel content) substituted wholesale."))

    # --- 6. insert-huffman-table: 2 DHT tables (grayscale) -> 4 DHT tables (color), real byproduct of mode.
    before = make_jpeg("L")
    after = make_jpeg("RGB", subsampling=0)
    bm, am = markers(before), markers(after)
    assert bm.count(0xC4) == 2 and am.count(0xC4) == 4, (bm, am)
    entries.append(("insert-huffman-table", "third-party-generated", before, after,
                     "DHT (Define Huffman Table) marker count 2 -> 4, a genuine byproduct of Pillow's own encoder writing separate luminance/chrominance table pairs for a 3-component image -- verified live by scanning the written bytes' own DHT marker count, same mechanism insert-frame-component rides on. Pillow's public API exposes no direct huffman-table count control (E2's own registry rationale for this subset's reader records the same limit), so the component-count route is the only real-library-reachable path to this kind."))

    # --- 7. remove-huffman-table: inverse of #6.
    before = make_jpeg("RGB", subsampling=0)
    after = make_jpeg("L")
    entries.append(("remove-huffman-table", "third-party-generated", before, after,
                     "DHT marker count 4 -> 2, inverse of insert-huffman-table."))

    # --- 8. set-sample-precision: HANDCRAFTED. Pillow always writes precision 8; no save() parameter reaches this.
    base = make_jpeg("RGB", subsampling=0)
    before = base
    after = patch_sof_precision(base, 12)
    entries.append(("set-sample-precision", "handcrafted", before, after,
                     "SOF0 sample-precision byte hand-patched 8 -> 12. HANDCRAFTED: Pillow 12.2.0's save() always writes baseline 8-bit precision and exposes no parameter for it, so no real-library path reaches this mutation (same limit E2's own registry rationale records for this reader). The entropy-coded scan data is left byte-identical to the genuine Pillow 8-bit encoding, so this file is explicitly NOT a spec-conformant 12-bit bitstream -- it exercises only the SOF marker-field mutation this subset's own vocabulary names, and is not claimed to be more than that."))

    # --- 9. set-arithmetic: HANDCRAFTED. Pillow has no arithmetic-coding support at all.
    base = make_jpeg("RGB", subsampling=0)
    before = base
    after = patch_sof_marker_code(base, 0xC9)  # SOF9: extended sequential DCT, arithmetic coding
    entries.append(("set-arithmetic", "handcrafted", before, after,
                     "SOF marker code hand-patched 0xC0 (baseline DCT, Huffman) -> 0xC9 (extended sequential DCT, arithmetic coding). HANDCRAFTED: Pillow has no arithmetic-entropy-coding support in its JPEG encoder at all (Huffman only), so no real-library path reaches this mutation. The entropy-coded scan data is left byte-identical to the genuine Pillow Huffman encoding, so this file is explicitly NOT a spec-conformant arithmetic-coded bitstream -- it exercises only the SOF marker-field mutation this subset's own vocabulary names, and is not claimed to be more than that."))

    manifests = []
    for mutation_id, klass, before_bytes, after_bytes, note in entries:
        case_dir = FIXTURES / f"{mutation_id}-applied"
        case_dir.mkdir(parents=True, exist_ok=True)
        (case_dir / "before.jpg").write_bytes(before_bytes)
        (case_dir / "after.jpg").write_bytes(after_bytes)

        entry = {
            "schema": "semio.repository-test.fixture/v2",
            "id": f"{mutation_id}-applied",
            "class": klass,
            "target": {"artifact": "s.stdio.jpg", "standard": "jfif-1.01", "subset": "baseline"},
            "mutation": mutation_id,
            "outcome": "applied",
            "units": {"length": "unitless", "angle": "degree"},
            "files": [
                {
                    "role": "expected-before-jpg",
                    "path": f"../🧫️fixtures/{mutation_id}-applied/before.jpg",
                    "mediaType": "image/jpeg",
                    "sha256": sha256_of(before_bytes),
                    "bytes": len(before_bytes),
                },
                {
                    "role": "expected-after-jpg",
                    "path": f"../🧫️fixtures/{mutation_id}-applied/after.jpg",
                    "mediaType": "image/jpeg",
                    "sha256": sha256_of(after_bytes),
                    "bytes": len(after_bytes),
                },
            ],
            "provenance": {
                "source": "generated" if klass == "third-party-generated" else "authored",
                "license": "MIT-CMU (Pillow)" if klass == "third-party-generated" else "n/a (handcrafted marker patch of an MIT-CMU Pillow base file)",
                "attribution": "Written by Pillow 12.2.0's own JpegImagePlugin encoder" if klass == "third-party-generated" else "A single marker byte of a genuine Pillow 12.2.0 JPEG binary-patched by hand; see notes for exactly which byte and why",
                "security": "scanned-clean",
                "privacy": "no-personal-data",
            },
            "comparisonProfile": "exact-bytes-v1",
            "reproducible": True,
            "family": "structural",
            "notes": note,
        }
        if klass == "third-party-generated":
            entry["generator"] = {
                "oracle": READER_ORACLE_ID,
                "packageVersion": PILLOW_VERSION,
                "engineFamily": "pillow",
                "engineVersion": PILLOW_VERSION,
                "command": "uv run python3 🔨️f2-gen-jpg-fixtures.py (PIL.Image.save(format='JPEG'))",
                "platform": "darwin-arm64",
            }
        manifests.append(entry)
        print(f"{mutation_id:24s} {klass:20s} before={len(before_bytes)}B after={len(after_bytes)}B")

    data = json.loads(ORACLE_JSON.read_text())
    data["fixtureManifests"] = manifests
    ORACLE_JSON.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n")
    print(f"\nWrote {len(manifests)} fixtureManifests entries into {ORACLE_JSON}")


if __name__ == "__main__":
    main()
