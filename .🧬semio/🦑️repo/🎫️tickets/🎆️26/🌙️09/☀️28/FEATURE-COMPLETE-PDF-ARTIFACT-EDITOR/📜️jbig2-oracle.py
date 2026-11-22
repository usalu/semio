"""🧪️ Encodes JBIG2 streams (generic, MMR, symbol dictionary and text regions), renders them in MuPDF and records its raster."""

import io
import random
import sys

import fitz
from PIL import Image

QE = [
    (0x5601, 1, 1, 1), (0x3401, 2, 6, 0), (0x1801, 3, 9, 0), (0x0AC1, 4, 12, 0), (0x0521, 5, 29, 0), (0x0221, 38, 33, 0), (0x5601, 7, 6, 1),
    (0x5401, 8, 14, 0), (0x4801, 9, 14, 0), (0x3801, 10, 14, 0), (0x3001, 11, 17, 0), (0x2401, 12, 18, 0), (0x1C01, 13, 20, 0), (0x1601, 29, 21, 0),
    (0x5601, 15, 14, 1), (0x5401, 16, 14, 0), (0x5101, 17, 15, 0), (0x4801, 18, 16, 0), (0x3801, 19, 17, 0), (0x3401, 20, 18, 0), (0x3001, 21, 19, 0),
    (0x2801, 22, 19, 0), (0x2401, 23, 20, 0), (0x2201, 24, 21, 0), (0x1C01, 25, 22, 0), (0x1801, 26, 23, 0), (0x1601, 27, 24, 0), (0x1401, 28, 25, 0),
    (0x1201, 29, 26, 0), (0x1101, 30, 27, 0), (0x0AC1, 31, 28, 0), (0x09C1, 32, 29, 0), (0x08A1, 33, 30, 0), (0x0521, 34, 31, 0), (0x0441, 35, 32, 0),
    (0x02A1, 36, 33, 0), (0x0221, 37, 34, 0), (0x0141, 38, 35, 0), (0x0111, 39, 36, 0), (0x0085, 40, 37, 0), (0x0049, 41, 38, 0), (0x0025, 42, 39, 0),
    (0x0015, 43, 40, 0), (0x0009, 44, 41, 0), (0x0005, 45, 42, 0), (0x0001, 45, 43, 0), (0x5601, 46, 46, 0),
]


class Ctx:
    def __init__(self, state=0):
        self.i, self.mps = state, 0


class MqEncoder:
    def __init__(self):
        self.a, self.c, self.ct = 0x8000, 0, 12
        self.out = bytearray([0])

    def _byteout(self):
        b = self.out[-1]
        if b == 0xFF:
            self.out.append(self.c >> 20 & 0xFF)
            self.c &= 0xFFFFF
            self.ct = 7
        elif self.c < 0x8000000:
            self.out.append(self.c >> 19 & 0xFF)
            self.c &= 0x7FFFF
            self.ct = 8
        else:
            self.out[-1] = b + 1
            if self.out[-1] == 0xFF:
                self.c &= 0x7FFFFFF
                self.out.append(self.c >> 20 & 0xFF)
                self.c &= 0xFFFFF
                self.ct = 7
            else:
                self.out.append(self.c >> 19 & 0xFF)
                self.c &= 0x7FFFF
                self.ct = 8

    def _renorm(self):
        while True:
            self.a <<= 1
            self.c <<= 1
            self.ct -= 1
            if self.ct == 0:
                self._byteout()
            if self.a & 0x8000:
                break

    def encode(self, cx, d):
        qe, nmps, nlps, switch = QE[cx.i]
        if d == cx.mps:
            self.a -= qe
            if self.a & 0x8000 == 0:
                if self.a < qe:
                    self.a = qe
                else:
                    self.c += qe
                cx.i = nmps
                self._renorm()
            else:
                self.c += qe
        else:
            self.a -= qe
            if self.a < qe:
                self.c += qe
            else:
                self.a = qe
            if switch:
                cx.mps = 1 - cx.mps
            cx.i = nlps
            self._renorm()

    def flush(self):
        temp = self.c + self.a
        self.c |= 0xFFFF
        if self.c >= temp:
            self.c -= 0x8000
        self.c <<= self.ct
        self._byteout()
        self.c <<= self.ct
        self._byteout()
        data = bytes(self.out[1:])
        if data and data[-1] == 0xFF:
            data = data[:-1]
        return data + b"\xff\xac"


FIXED = {
    0: [(-1, -2, 14), (0, -2, 13), (1, -2, 12), (-2, -1, 9), (-1, -1, 8), (0, -1, 7), (1, -1, 6), (2, -1, 5), (-4, 0, 3), (-3, 0, 2), (-2, 0, 1), (-1, 0, 0)],
    1: [(-1, -2, 12), (0, -2, 11), (1, -2, 10), (2, -2, 9), (-2, -1, 8), (-1, -1, 7), (0, -1, 6), (1, -1, 5), (2, -1, 4), (-3, 0, 2), (-2, 0, 1), (-1, 0, 0)],
    2: [(-1, -2, 9), (0, -2, 8), (1, -2, 7), (-2, -1, 6), (-1, -1, 5), (0, -1, 4), (1, -1, 3), (-2, 0, 1), (-1, 0, 0)],
    3: [(-3, -1, 9), (-2, -1, 8), (-1, -1, 7), (0, -1, 6), (1, -1, 5), (-4, 0, 3), (-3, 0, 2), (-2, 0, 1), (-1, 0, 0)],
}
AT_BITS = {0: [4, 10, 11, 15], 1: [3], 2: [2], 3: [4]}
AT_DEFAULT = {0: [(3, -1), (-3, -1), (2, -2), (-2, -2)], 1: [(3, -1)], 2: [(2, -1)], 3: [(2, -1)]}
PSEUDO = {0: 0x9B25, 1: 0x0795, 2: 0x00E5, 3: 0x0195}


def encode_generic(enc, ctxs, bitmap, template, typical, at):
    h, w = len(bitmap), len(bitmap[0]) if bitmap else 0
    get = lambda x, y: bitmap[y][x] if 0 <= x < w and 0 <= y < h else 0
    ltp = 0
    for y in range(h):
        if typical:
            same = y > 0 and bitmap[y] == bitmap[y - 1] or (y == 0 and not any(bitmap[0]))
            enc.encode(ctxs[PSEUDO[template]], int(same) ^ ltp)
            ltp = int(same)
            if same:
                continue
        for x in range(w):
            label = 0
            for dx, dy, bit in FIXED[template]:
                label |= get(x + dx, y + dy) << bit
            for (dx, dy), bit in zip(at, AT_BITS[template]):
                label |= get(x + dx, y + dy) << bit
            enc.encode(ctxs[label], bitmap[y][x])


def encode_int(enc, ctxs, value):
    prev = 1
    def put(bit):
        nonlocal prev
        enc.encode(ctxs[prev], bit)
        prev = (prev << 1 | bit) if prev < 256 else (((prev << 1 | bit) & 511) | 256)
    if value is None:
        put(1); put(0); put(0); put(0)
        return
    sign = int(value < 0)
    v = abs(value)
    put(sign)
    for count, offset, prefix in [(2, 0, [0]), (4, 4, [1, 0]), (6, 20, [1, 1, 0]), (8, 84, [1, 1, 1, 0]), (12, 340, [1, 1, 1, 1, 0]), (32, 4436, [1, 1, 1, 1, 1])]:
        if v < offset + (1 << count) or count == 32:
            for b in prefix:
                put(b)
            for i in range(count - 1, -1, -1):
                put((v - offset) >> i & 1)
            return


def encode_id(enc, ctxs, value, length):
    prev = 1
    for i in range(length - 1, -1, -1):
        bit = value >> i & 1
        enc.encode(ctxs[prev], bit)
        prev = prev << 1 | bit


def segment(number, kind, referred, data, page=1):
    header = number.to_bytes(4, "big") + bytes([kind]) + bytes([len(referred) << 5 | 0]) + bytes(referred) + bytes([page]) + len(data).to_bytes(4, "big")
    return header + data


def page_info(width, height):
    return segment(0, 48, [], width.to_bytes(4, "big") + height.to_bytes(4, "big") + bytes(8) + bytes([0]) + bytes(2))


def region_info(width, height, x, y, op=0):
    return width.to_bytes(4, "big") + height.to_bytes(4, "big") + x.to_bytes(4, "big") + y.to_bytes(4, "big") + bytes([op])


def picture(width, height, seed):
    rng = random.Random(seed)
    rows = [[0] * width for _ in range(height)]
    for _ in range(6):
        x0, y0 = rng.randrange(width - 8), rng.randrange(height - 8)
        for y in range(y0, y0 + rng.randint(3, 8)):
            for x in range(x0, x0 + rng.randint(3, 20)):
                if x < width and y < height:
                    rows[y][x] = 1
    for _ in range(width * height // 20):
        rows[rng.randrange(height)][rng.randrange(width)] ^= 1
    for y in range(height // 2, height // 2 + 3):
        rows[y] = rows[height // 2 - 1][:]
    return rows


def generic_stream(rows, template, typical, at=None):
    h, w = len(rows), len(rows[0])
    at = at or AT_DEFAULT[template]
    enc, ctxs = MqEncoder(), [Ctx() for _ in range(1 << 16)]
    encode_generic(enc, ctxs, rows, template, typical, at)
    flags = template << 1 | (8 if typical else 0)
    body = region_info(w, h, 0, 0) + bytes([flags]) + b"".join((dx & 0xFF).to_bytes(1, "big") + (dy & 0xFF).to_bytes(1, "big") for dx, dy in at) + enc.flush()
    return page_info(w, h) + segment(1, 38, [], body)


def encode_refinement(enc, ctxs, bitmap, reference, template, typical, at):
    h, w = len(bitmap), len(bitmap[0])
    get = lambda m, x, y: m[y][x] if 0 <= y < len(m) and 0 <= x < len(m[0]) else 0
    if template == 0:
        coding, refs = [(0, -1), (1, -1), (-1, 0), tuple(at[0])], [(0, -1), (1, -1), (-1, 0), (0, 0), (1, 0), (-1, 1), (0, 1), (1, 1), tuple(at[1])]
    else:
        coding, refs = [(-1, -1), (0, -1), (1, -1), (-1, 0)], [(0, -1), (-1, 0), (0, 0), (1, 0), (0, 1), (1, 1)]
    pseudo = 0x0020 if template == 0 else 0x0004
    ltp = 0
    for y in range(h):
        if typical:
            def predictable(x):
                c = get(reference, x, y)
                return all(get(reference, x + i, y + j) == c for i in (-1, 0, 1) for j in (-1, 0, 1)) and bitmap[y][x] == c
            def uniform(x):
                c = get(reference, x, y)
                return all(get(reference, x + i, y + j) == c for i in (-1, 0, 1) for j in (-1, 0, 1))
            row_ok = all(bitmap[y][x] == get(reference, x, y) for x in range(w) if uniform(x))
            enc.encode(ctxs[pseudo], int(row_ok) ^ ltp)
            ltp = int(row_ok)
        for x in range(w):
            if typical and ltp and all(get(reference, x + i, y + j) == get(reference, x, y) for i in (-1, 0, 1) for j in (-1, 0, 1)):
                continue
            label = 0
            for ox, oy in coding:
                label = label << 1 | get(bitmap, x + ox, y + oy)
            for ox, oy in refs:
                label = label << 1 | get(reference, x + ox, y + oy)
            enc.encode(ctxs[label], bitmap[y][x])


def refinement_stream(base, refined, template, typical):
    h, w = len(base), len(base[0])
    at = [(-1, -1), (-1, -1)]
    enc, ctxs = MqEncoder(), [Ctx() for _ in range(1 << 13)]
    encode_refinement(enc, ctxs, refined, base, template, typical, at)
    flags = template | (2 if typical else 0)
    body = region_info(w, h, 0, 0, 4) + bytes([flags]) + (b"".join((dx & 0xFF).to_bytes(1, "big") + (dy & 0xFF).to_bytes(1, "big") for dx, dy in at) if template == 0 else b"") + enc.flush()
    first = generic_stream(base, 0, False)
    return first + segment(2, 42, [], body)


def mmr_bits(rows):
    h, w = len(rows), len(rows[0])
    image = Image.new("1", (w, h), 1)
    px = image.load()
    for y in range(h):
        for x in range(w):
            px[x, y] = 0 if rows[y][x] else 1
    buffer = io.BytesIO()
    image.save(buffer, format="TIFF", compression="group4")
    buffer.seek(0)
    tags = Image.open(buffer).tag_v2
    raw = buffer.getvalue()
    return b"".join(raw[o:o + c] for o, c in zip(tags[273], tags[279]))


def halftone_stream(template, mmr=False):
    pw, ph, patterns_n = 4, 4, 8
    patterns = []
    for index in range(patterns_n):
        rows = [[1 if (x + y * 2 + index) % 8 < index else 0 for x in range(pw)] for y in range(ph)]
        patterns.append(rows)
    collective = [sum((patterns[i][y] for i in range(patterns_n)), []) for y in range(ph)]
    enc, ctxs = MqEncoder(), [Ctx() for _ in range(1 << 16)]
    at = [(-pw, 0), (-3, -1), (2, -2), (-2, -2)] if template == 0 else [(-pw, 0)]
    encode_generic(enc, ctxs, collective, template, False, at)
    dictionary = bytes([template << 1, pw, ph]) + (patterns_n - 1).to_bytes(4, "big") + enc.flush()
    if mmr:
        dictionary = bytes([1, pw, ph]) + (patterns_n - 1).to_bytes(4, "big") + mmr_bits(collective)
    grid_w, grid_h = 12, 9
    gray = [[(m * 3 + n * 5) % patterns_n for n in range(grid_w)] for m in range(grid_h)]
    planes = 3
    enc2, ctxs2 = MqEncoder(), [Ctx() for _ in range(1 << 16)]
    at2 = [(3, -1), (-3, -1), (2, -2), (-2, -2)] if template == 0 else [(3 if template <= 1 else 2, -1)]
    previous = None
    mmr_planes = []
    for j in range(planes - 1, -1, -1):
        bit = [[(g >> j) & 1 for g in row] for row in gray]
        coded = [[bit[m][n] ^ (previous[m][n] if previous else 0) for n in range(grid_w)] for m in range(grid_h)]
        if mmr:
            mmr_planes.append(mmr_bits(coded))
        else:
            encode_generic(enc2, ctxs2, coded, template, False, at2)
        previous = bit
    width, height = grid_w * pw, grid_h * ph
    flags = 1 if mmr else template << 1
    body = region_info(width, height, 0, 0, 0) + bytes([flags]) + grid_w.to_bytes(4, "big") + grid_h.to_bytes(4, "big") + (0).to_bytes(4, "big") + (0).to_bytes(4, "big") + (pw * 256).to_bytes(2, "big") + (0).to_bytes(2, "big") + (b"".join(mmr_planes) if mmr else enc2.flush())
    return page_info(width, height) + segment(1, 16, [], dictionary) + segment(2, 22, [1], body), width, height


class Bits:
    def __init__(self):
        self.bits = []

    def put(self, value, count):
        self.bits += [(value >> i) & 1 for i in range(count - 1, -1, -1)]

    def align(self):
        while len(self.bits) % 8:
            self.bits.append(0)

    def bytes(self):
        self.align()
        return bytes(int("".join(map(str, self.bits[i:i + 8])), 2) for i in range(0, len(self.bits), 8))


class HuffTable:
    """Canonical prefix codes of ITU-T T.88 Annex B built from the jbig2dec table values."""

    def __init__(self, lines):
        self.lines = lines
        longest = max(l[0] for l in lines)
        counts = [0] * (longest + 2)
        for l in lines:
            counts[l[0]] += 1
        counts[0] = 0
        first = [0] * (longest + 2)
        nxt = [0] * (longest + 2)
        for n in range(1, longest + 1):
            first[n] = (first[n - 1] + counts[n - 1]) << 1
            nxt[n] = first[n]
        self.codes = []
        for l in lines:
            if l[0]:
                self.codes.append((nxt[l[0]], l[0]))
                nxt[l[0]] += 1
            else:
                self.codes.append(None)

    def encode(self, bits, value):
        for (plen, rlen, low, kind), code in zip(self.lines, self.codes):
            if code is None:
                continue
            if value is None and kind == 3:
                bits.put(code[0], code[1])
                return
            if value is not None and kind == 0 and low <= value < low + (1 << rlen):
                bits.put(code[0], code[1])
                bits.put(value - low, rlen)
                return
        for (plen, rlen, low, kind), code in zip(self.lines, self.codes):
            if code is None:
                continue
            if value is not None and kind == 1 and value <= low:
                bits.put(code[0], code[1])
                bits.put(low - value, 32)
                return
            if value is not None and kind == 2 and value >= low:
                bits.put(code[0], code[1])
                bits.put(value - low, 32)
                return
        raise ValueError(f"value {value} not encodable")


import json as _json
import os as _os
STANDARD = [HuffTable([tuple(l) for l in table]) for table in _json.load(open(_os.path.join(_os.path.dirname(_os.path.abspath(__file__)), "🔣️jbig2-huffman-tables.json")))]


def huffman_stream(width, height, placements, custom=False, refine=False):
    symbols = [[[int(c) for c in row] for row in glyph] for glyph in GLYPHS]
    b = Bits()
    sym_h, sym_w = len(symbols[0]), len(symbols[0][0])
    STANDARD[3].encode(b, sym_h)
    previous = 0
    for symbol in symbols:
        STANDARD[1].encode(b, sym_w - previous)
        previous = sym_w
    STANDARD[1].encode(b, None)
    STANDARD[0].encode(b, 0)
    b.align()
    for y in range(sym_h):
        for symbol in symbols:
            for x in range(sym_w):
                b.put(symbol[y][x], 1)
        b.align()
    STANDARD[0].encode(b, 0)
    STANDARD[0].encode(b, len(symbols))
    dictionary = (1).to_bytes(2, "big") + len(symbols).to_bytes(4, "big") + len(symbols).to_bytes(4, "big") + b.bytes()
    t = Bits()
    run_lengths = [0] * 35
    run_lengths[1] = run_lengths[2] = 1
    for length in run_lengths:
        t.put(length, 4)
    t.put(0, 1); t.put(1, 1); t.put(1, 1)
    t.align()
    STANDARD[10].encode(t, 1)
    rows = {}
    for sym, x, y in placements:
        rows.setdefault(y, []).append((x, sym))
    stripe_t, first_s = -1, 0
    id_codes = {0: (0, 1), 1: (2, 2), 2: (3, 2)}
    for y in sorted(rows):
        STANDARD[10].encode(t, y - stripe_t)
        stripe_t = y
        cur = None
        for n, (x, sym) in enumerate(sorted(rows[y])):
            if n == 0:
                (CUSTOM if custom else STANDARD[5]).encode(t, x - first_s)
                first_s = x
            else:
                STANDARD[7].encode(t, x - cur)
            t.put(*id_codes[sym])
            if refine:
                modified = n == 1 and sym == 1
                t.put(int(modified), 1)
                if modified:
                    target = [row[:] for row in symbols[sym]]
                    target[2][1] ^= 1
                    target[4][2] ^= 1
                    encoder, contexts = MqEncoder(), [Ctx() for _ in range(1 << 13)]
                    encode_refinement(encoder, contexts, target, symbols[sym], 0, False, [(-1, -1), (-1, -1)])
                    payload = encoder.flush()
                    for _ in range(4):
                        STANDARD[13].encode(t, 0)
                    STANDARD[0].encode(t, len(payload))
                    t.align()
                    for byte in payload:
                        t.put(byte, 8)
            cur = x + sym_w - 1
        STANDARD[7].encode(t, None)
    flags = 1 | (1 << 4) | (2 if refine else 0)
    selectors = 3 if custom else 0
    text = region_info(width, height, 0, 0) + flags.to_bytes(2, "big") + selectors.to_bytes(2, "big") + (bytes([0xFF] * 4) if refine else b"") + len(placements).to_bytes(4, "big") + t.bytes()
    if custom:
        return page_info(width, height) + segment(1, 0, [], dictionary) + segment(3, 53, [], CUSTOM_SEGMENT) + segment(4, 6, [1, 3], text)
    return page_info(width, height) + segment(1, 0, [], dictionary) + segment(2, 6, [1], text)


CUSTOM = HuffTable([(2, 6, -64, 0), (2, 6, 0, 0), (3, 32, -65, 1), (3, 32, 64, 2)])
_c = Bits()
for _length, _range in [(2, 6), (2, 6)]:
    _c.put(_length, 2)
    _c.put(_range, 3)
_c.put(3, 2)
_c.put(3, 2)
CUSTOM_SEGMENT = bytes([0x22]) + (-64 & 0xFFFFFFFF).to_bytes(4, "big") + (64).to_bytes(4, "big") + _c.bytes()


def aggregate_stream():
    glyphs = [[[int(c) for c in row] for row in glyph] for glyph in GLYPHS]
    gw, gh = 4, 7
    at = AT_DEFAULT[0]
    at_bytes = b"".join((dx & 0xFF).to_bytes(1, "big") + (dy & 0xFF).to_bytes(1, "big") for dx, dy in at)
    ctx = {name: [Ctx() for _ in range(512)] for name in ["dh", "dw", "ex", "ai", "dt", "fs", "ds", "ri", "rdx", "rdy"]}
    enc, generic = MqEncoder(), [Ctx() for _ in range(1 << 16)]
    encode_int(enc, ctx["dh"], gh)
    for n, glyph in enumerate(glyphs):
        encode_int(enc, ctx["dw"], gw if n == 0 else 0)
        encode_generic(enc, generic, glyph, 0, False, at)
    encode_int(enc, ctx["dw"], None)
    encode_int(enc, ctx["ex"], 0)
    encode_int(enc, ctx["ex"], 3)
    first = (0).to_bytes(2, "big") + at_bytes + (3).to_bytes(4, "big") + (3).to_bytes(4, "big") + enc.flush()

    ctx = {name: [Ctx() for _ in range(512)] for name in ctx}
    enc, generic, refinement = MqEncoder(), [Ctx() for _ in range(1 << 16)], [Ctx() for _ in range(1 << 13)]
    code_len = 3
    ids = [Ctx() for _ in range(1 << (code_len + 1))]
    aggregate_width = 9
    encode_int(enc, ctx["dh"], gh)
    encode_int(enc, ctx["dw"], aggregate_width)
    encode_int(enc, ctx["ai"], 2)
    encode_int(enc, ctx["dt"], 0)
    encode_int(enc, ctx["dt"], 0)
    encode_int(enc, ctx["fs"], 0)
    encode_id(enc, ids, 0, code_len)
    encode_int(enc, ctx["ri"], 0)
    encode_int(enc, ctx["ds"], 5 - (0 + gw - 1))
    encode_id(enc, ids, 1, code_len)
    encode_int(enc, ctx["ri"], 0)
    encode_int(enc, ctx["ds"], None)
    refined = [row[:] for row in glyphs[2]]
    refined[3][1] ^= 1
    refined[5][2] ^= 1
    encode_int(enc, ctx["dw"], gw - aggregate_width)
    encode_int(enc, ctx["ai"], 1)
    encode_id(enc, ids, 2, code_len)
    encode_int(enc, ctx["rdx"], 0)
    encode_int(enc, ctx["rdy"], 0)
    encode_refinement(enc, refinement, refined, glyphs[2], 0, False, [(-1, -1), (-1, -1)])
    encode_int(enc, ctx["dw"], None)
    encode_int(enc, ctx["ex"], 3)
    encode_int(enc, ctx["ex"], 2)
    second = (0x0002).to_bytes(2, "big") + at_bytes + bytes([0xFF, 0xFF, 0xFF, 0xFF]) + (2).to_bytes(4, "big") + (2).to_bytes(4, "big") + enc.flush()

    placements = [(0, 2, 2), (1, 16, 2), (0, 5, 14), (1, 20, 20)]
    widths = [aggregate_width, gw]
    enc, tctx, ids = MqEncoder(), {name: [Ctx() for _ in range(512)] for name in ["dt", "fs", "ds"]}, [Ctx() for _ in range(4)]
    rows = {}
    for sym, x, y in placements:
        rows.setdefault(y, []).append((x, sym))
    stripe_t, first_s = 0, 0
    encode_int(enc, tctx["dt"], 0)
    for y in sorted(rows):
        encode_int(enc, tctx["dt"], y - stripe_t)
        stripe_t = y
        cur = None
        for n, (x, sym) in enumerate(sorted(rows[y])):
            if n == 0:
                encode_int(enc, tctx["fs"], x - first_s)
                first_s = x
            else:
                encode_int(enc, tctx["ds"], x - cur)
            encode_id(enc, ids, sym, 1)
            cur = x + widths[sym] - 1
        encode_int(enc, tctx["ds"], None)
    text = region_info(40, 34, 0, 0) + (1 << 4).to_bytes(2, "big") + len(placements).to_bytes(4, "big") + enc.flush()
    return page_info(40, 34) + segment(1, 0, [], first) + segment(2, 0, [1], second) + segment(3, 6, [2], text)


def mmr_stream(rows):
    h, w = len(rows), len(rows[0])
    image = Image.new("1", (w, h), 1)
    px = image.load()
    for y in range(h):
        for x in range(w):
            px[x, y] = 0 if rows[y][x] else 1
    buffer = io.BytesIO()
    image.save(buffer, format="TIFF", compression="group4")
    buffer.seek(0)
    tags = Image.open(buffer).tag_v2
    raw = buffer.getvalue()
    data = b"".join(raw[o:o + c] for o, c in zip(tags[273], tags[279]))
    body = region_info(w, h, 0, 0) + bytes([1]) + data
    return page_info(w, h) + segment(1, 38, [], body)


GLYPHS = [
    ["0110", "1001", "1001", "1111", "1001", "1001", "1001"],
    ["1110", "1001", "1110", "1001", "1001", "1001", "1110"],
    ["0111", "1000", "1000", "1000", "1000", "1000", "0111"],
]


def text_streams(width, height, placements):
    symbols = [[[int(c) for c in row] for row in glyph] for glyph in GLYPHS]
    enc, ctxs = MqEncoder(), [Ctx() for _ in range(1 << 16)]
    ctx = {name: [Ctx() for _ in range(512)] for name in ["dh", "dw", "ex"]}
    by_height = {}
    for index, symbol in enumerate(symbols):
        by_height.setdefault(len(symbol), []).append(index)
    prev_h = 0
    order = []
    for h in sorted(by_height):
        encode_int(enc, ctx["dh"], h - prev_h)
        prev_h = h
        prev_w = 0
        for index in by_height[h]:
            w = len(symbols[index][0])
            encode_int(enc, ctx["dw"], w - prev_w)
            prev_w = w
            encode_generic(enc, ctxs, symbols[index], 0, False, AT_DEFAULT[0])
            order.append(index)
        encode_int(enc, ctx["dw"], None)
    encode_int(enc, ctx["ex"], 0)
    encode_int(enc, ctx["ex"], len(symbols))
    dictionary = (0).to_bytes(2, "big") + b"".join((dx & 0xFF).to_bytes(1, "big") + (dy & 0xFF).to_bytes(1, "big") for dx, dy in AT_DEFAULT[0]) + len(symbols).to_bytes(4, "big") + len(symbols).to_bytes(4, "big") + enc.flush()
    new_order = order
    enc2 = MqEncoder()
    tctx = {name: [Ctx() for _ in range(512)] for name in ["dt", "fs", "ds"]}
    code_len = max(1, (len(symbols) - 1).bit_length())
    idc = [Ctx() for _ in range(1 << (code_len + 1))]
    rows = {}
    for sym, x, y in placements:
        rows.setdefault(y, []).append((x, sym))
    stripe_t = 0
    encode_int(enc2, tctx["dt"], 0)
    first_s = 0
    for y in sorted(rows):
        encode_int(enc2, tctx["dt"], y - stripe_t)
        stripe_t = y
        cur = None
        for n, (x, sym) in enumerate(sorted(rows[y])):
            width_symbol = len(symbols[new_order.index(sym) if False else sym][0])
            if n == 0:
                encode_int(enc2, tctx["fs"], x - first_s)
                first_s = x
            else:
                encode_int(enc2, tctx["ds"], x - cur)
            encode_id(enc2, idc, new_order.index(sym), code_len)
            cur = x + width_symbol - 1
        encode_int(enc2, tctx["ds"], None)
    flags = 1 << 4 | 0 << 7
    text = region_info(width, height, 0, 0) + flags.to_bytes(2, "big") + len(placements).to_bytes(4, "big") + enc2.flush()
    return page_info(width, height) + segment(1, 0, [], dictionary) + segment(2, 6, [1], text), symbols


def pdf_for(stream, width, height):
    image_dict = f"<< /Type /XObject /Subtype /Image /Width {width} /Height {height} /ColorSpace /DeviceGray /BitsPerComponent 1 /Filter /JBIG2Decode /Length {len(stream)} >>"
    scale = 4
    content = f"q {width * scale} 0 0 {height * scale} 0 0 cm /Im1 Do Q"
    objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
        f"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {width * scale} {height * scale}] /Contents 4 0 R /Resources << /XObject << /Im1 5 0 R >> >> >>".encode(),
        f"<< /Length {len(content)} >>\nstream\n{content}\nendstream".encode(),
        image_dict.encode() + b"\nstream\n" + stream + b"\nendstream",
    ]
    pdf = bytearray(b"%PDF-1.7\n")
    offsets = []
    for i, body in enumerate(objects):
        offsets.append(len(pdf))
        pdf += f"{i + 1} 0 obj\n".encode() + body + b"\nendobj\n"
    xref = len(pdf)
    pdf += f"xref\n0 {len(objects) + 1}\n0000000000 65535 f \n".encode() + b"".join(f"{o:010} 00000 n \n".encode() for o in offsets)
    pdf += f"trailer\n<< /Root 1 0 R /Size {len(objects) + 1} >>\nstartxref\n{xref}\n%%EOF\n".encode()
    return bytes(pdf), scale


def render(stream, width, height):
    pdf, scale = pdf_for(stream, width, height)
    with fitz.open(stream=pdf, filetype="pdf") as document:
        pixmap = document[0].get_pixmap(colorspace=fitz.csGRAY)
        rows = []
        for y in range(height):
            rows.append([1 if pixmap.pixel(x * scale + scale // 2, y * scale + scale // 2)[0] < 128 else 0 for x in range(width)])
        return rows


def main(path):
    lines = []
    cases = []
    base = picture(61, 40, 5)
    for template in range(4):
        cases.append((f"generic-t{template}", generic_stream(base, template, False), 61, 40))
        cases.append((f"generic-t{template}-tpgd", generic_stream(base, template, True), 61, 40))
    cases.append(("generic-custom-at", generic_stream(base, 0, False, [(2, -1), (-2, -1), (3, -2), (-3, -2)]), 61, 40))
    cases.append(("mmr", mmr_stream(base), 61, 40))
    cases.append(("huffman-symbols-and-text", huffman_stream(40, 34, [(0, 2, 2), (1, 8, 2), (2, 14, 2), (0, 3, 14), (2, 10, 14), (1, 20, 14), (1, 30, 25)]), 40, 34))
    cases.append(("huffman-custom-table", huffman_stream(40, 34, [(0, 2, 2), (1, 8, 2), (2, 14, 2), (0, 3, 14), (2, 10, 14), (1, 20, 14), (1, 30, 25)], True), 40, 34))
    stream, hw, hh = halftone_stream(0, True)
    cases.append(("halftone-mmr", stream, hw, hh))
    cases.append(("aggregate-symbols", aggregate_stream(), 40, 34))
    # jbig2dec rejects Huffman text regions with refinement, so no MuPDF oracle exists for them
    for template in (0, 1, 2, 3):
        stream, hw, hh = halftone_stream(template)
        cases.append((f"halftone-t{template}", stream, hw, hh))
    refined = [row[:] for row in base]
    for y in range(0, 40, 3):
        for x in range(0, 61, 5):
            refined[y][x] ^= 1
    for template in (0, 1):
        cases.append((f"refinement-t{template}", refinement_stream(base, refined, template, False), 61, 40))
        cases.append((f"refinement-t{template}-tpgr", refinement_stream(base, refined, template, True), 61, 40))
    placements = [(0, 2, 2), (1, 8, 2), (2, 14, 2), (0, 3, 14), (2, 10, 14), (1, 20, 14), (1, 30, 25)]
    stream, _ = text_streams(40, 34, placements)
    cases.append(("symbols-and-text", stream, 40, 34))
    for name, data, width, height in cases:
        rows = render(data, width, height)
        lines.append("\t".join([name, str(width), str(height), data.hex(), "".join("".join(map(str, r)) for r in rows)]))
    open(path, "w").write("\n".join(lines))
    print(len(lines), "JBIG2 cases")


main(sys.argv[1])
