"""🔍 Checks the native paged archive against independent Pillow RGB and alpha decoding."""

from hashlib import sha256
from pathlib import Path
from struct import unpack_from
from PIL import Image

OUT = Path(__file__).resolve().parent / "🗑️generated/raw-png"


def crc32c(data):
    """🧮 Calculates the Castagnoli checksum independently of the native Pack engine."""
    state = 0xFFFFFFFF
    for byte in data:
        state ^= byte
        for _ in range(8):
            state = (state >> 1) ^ (0x82F63B78 if state & 1 else 0)
    return state ^ 0xFFFFFFFF


def inspect(file):
    """📚 Validate the fixed header, every segment CRC, and the terminal footer."""
    raw = file.read_bytes()
    assert raw[:8] == b"\x89SPK\r\n\x1a\n"
    assert unpack_from("<II", raw, 12) == (2, 5)
    assert unpack_from("<I", raw, 20)[0] == crc32c(raw[:20])
    footer = raw[-84:]
    assert footer[:8] == b"SPKFOOT1" and unpack_from("<Q", footer, 32)[0] == len(raw)
    assert unpack_from("<I", footer, 80)[0] == crc32c(footer[:80])
    pos, limit, chunks = 32, len(raw) - 84, []
    while pos < limit:
        start = pos
        kind, flags = raw[pos:pos + 2]
        assert flags == 0
        pos += 2
        shift, count = 0, 0
        while True:
            octet = raw[pos]
            pos += 1
            count |= (octet & 0x7F) << shift
            if octet < 0x80:
                break
            shift += 7
            assert shift < 35
        end = pos + count
        assert end + 4 <= limit
        assert unpack_from("<I", raw, end)[0] == crc32c(raw[start:end])
        if kind == 5:
            chunks.append(raw[pos:end])
        pos = end + 4
    assert kind == 0 and pos == limit
    return chunks, len(raw)


def verify():
    """🎨 Validate every native sample, digest byte, and whether the paired soft mask is needed."""
    with Image.open(OUT / "rgba.png") as image:
        rgba = image.convert("RGBA")
        rgb, opacity = rgba.convert("RGB").tobytes(), rgba.getchannel("A").tobytes()
    for alpha, filename in ((True, "archive-alpha.pack"), (False, "archive-opaque.pack")):
        chunks, length = inspect(OUT / filename)
        facts, color, *tail = chunks
        assert len(facts) == 42 and unpack_from("<IIBB", facts) == (32, 32, 3, int(alpha))
        assert color == rgb
        mask = opacity if alpha else b""
        assert tail == ([mask] if alpha else []) + [bytes([0x35]) * 32 + b"typed placement control"]
        expected = sha256(b"s.stdio.pdf.png-placement.v1\n" + facts[:10] + rgb + mask).digest()
        assert expected == facts[10:]
        print(f"[DEBUG] {filename}: {length} canonical bytes, {len(rgb)} RGB bytes, {len(mask)} alpha bytes, exact digest {expected.hex()}")


if __name__ == "__main__":
    verify()
