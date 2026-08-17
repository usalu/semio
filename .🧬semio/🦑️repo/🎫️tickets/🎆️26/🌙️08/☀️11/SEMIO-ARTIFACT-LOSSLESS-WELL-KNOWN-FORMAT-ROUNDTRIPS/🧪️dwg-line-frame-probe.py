from collections import Counter, defaultdict
from pathlib import Path
import importlib.util
import math
import struct


TICKET = Path(__file__).parent
FIXTURE = Path("/Users/ueli/Documents/semio/temp/architectural_example.dwg")
SPEC = importlib.util.spec_from_file_location("d2", TICKET / "🧪️dwg-d2-policy-probe.py")
D2 = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(D2)

OBJECT_PAGES = [
    (0x16260 + 32, 17145, 0x7400),
    (0x1A580 + 32, 11080, 0x7400),
    (0x1D100 + 32, 4380, 0x7400),
    (0x1E240 + 32, 2246, 0x7400),
    (0x1EB40 + 32, 3378, 0x7400),
    (0x1F8A0 + 32, 4448, 0x7400),
    (0x20A20 + 32, 3490, 0x7400),
    (0x21800 + 32, 1711, 213182 - 7 * 0x7400),
]
HANDLES_PAGE = (0x22080 + 32, 1907, 2085)


class Bits:
    def __init__(self, data, position=0):
        self.data = data
        self.position = position

    def bit(self):
        value = self.data[self.position // 8] >> (7 - self.position % 8) & 1
        self.position += 1
        return value

    def bits(self, count):
        value = 0
        for _ in range(count):
            value = value << 1 | self.bit()
        return value

    def byte(self):
        return self.bits(8)

    def rs(self):
        return self.byte() | self.byte() << 8

    def rl(self):
        return self.rs() | self.rs() << 16

    def rll(self):
        return self.rl() | self.rl() << 32

    def rd(self):
        return struct.unpack("<d", bytes(self.byte() for _ in range(8)))[0]

    def bb(self):
        return self.bits(2)

    def bs(self, selectors=None):
        selector = self.bb()
        if selectors is not None:
            selectors[selector] += 1
        if selector == 0:
            return self.rs()
        if selector == 1:
            return self.byte()
        if selector == 2:
            return 0
        return 256

    def bl(self, selectors=None):
        selector = self.bb()
        if selectors is not None:
            selectors[selector] += 1
        if selector == 0:
            return self.rl()
        if selector == 1:
            return self.byte()
        if selector == 2:
            return 0
        raise ValueError("invalid BL selector")

    def bll(self):
        count = 0
        while count < 3 and self.bit():
            count += 1
        return sum(self.byte() << (8 * index) for index in range(count))

    def bd(self, selectors=None):
        selector = self.bb()
        if selectors is not None:
            selectors[selector] += 1
        if selector == 0:
            return self.rd()
        if selector == 1:
            return 1.0
        if selector == 2:
            return 0.0
        raise ValueError("invalid BD selector")

    def dd(self, default, selectors=None):
        selector = self.bb()
        if selectors is not None:
            selectors[selector] += 1
        raw = bytearray(struct.pack("<d", default))
        if selector == 0:
            return default
        if selector == 1:
            raw[:4] = bytes(self.byte() for _ in range(4))
        elif selector == 2:
            raw[4:6] = bytes(self.byte() for _ in range(2))
            raw[:4] = bytes(self.byte() for _ in range(4))
        else:
            return self.rd()
        return struct.unpack("<d", raw)[0]

    def handle(self):
        head = self.byte()
        code = head >> 4
        value = 0
        for _ in range(head & 0x0F):
            value = value << 8 | self.byte()
        return code, value


def decode_page(fixture, page):
    offset, compressed, semantic = page
    return D2.decompress(fixture[offset:offset + compressed])[0][:semantic]


def modular_char(data, position, signed=False):
    value = 0
    shift = 0
    while True:
        byte = data[position]
        position += 1
        negative = signed and byte & 0x80 == 0 and byte & 0x40 != 0
        value |= (byte & (0x3F if negative else 0x7F)) << shift
        if byte & 0x80 == 0:
            return (-value if negative else value), position
        shift += 7


def handle_map(data):
    position = 0
    handle = 0
    address = 0
    entries = []
    while position + 2 <= len(data):
        start = position
        size = int.from_bytes(data[position:position + 2], "big")
        position += 2
        if size <= 2:
            break
        end = start + size
        while position < end:
            delta, position = modular_char(data, position)
            handle += delta
            delta, position = modular_char(data, position, True)
            address += delta
            entries.append((handle, address))
        position = end + 2
    return entries


def frame_prefix(data, address):
    reader = Bits(data[address:])
    payload_size = 0
    shift = 0
    while True:
        chunk = reader.rs()
        payload_size |= (chunk & 0x7FFF) << shift
        if chunk & 0x8000 == 0:
            break
        shift += 15
    ms_bytes = reader.position // 8
    handle_bits, umc_end = modular_char(data, address + ms_bytes)
    prefix_bytes = umc_end - address
    payload = data[address + prefix_bytes:address + prefix_bytes + payload_size]
    return payload_size, handle_bits, prefix_bytes, payload


def bot(reader, selectors=None):
    selector = reader.bb()
    if selectors is not None:
        selectors[selector] += 1
    if selector == 0:
        return reader.byte()
    if selector == 1:
        return reader.byte() + 0x1F0
    return reader.rs()


def skip_eed(reader):
    records = []
    while True:
        size = reader.bs()
        if size == 0:
            return records
        application = reader.handle()
        end = reader.position + size * 8
        values = []
        while reader.position < end:
            code = reader.byte()
            if code == 0:
                length = reader.rs()
                values.append((code, tuple(reader.rs() for _ in range(length))))
            elif code == 2:
                values.append((code, reader.byte()))
            elif code == 3:
                values.append((code, reader.rll()))
            elif code == 4:
                length = reader.byte()
                values.append((code, tuple(reader.byte() for _ in range(length))))
            elif code == 5:
                values.append((code, tuple(reader.byte() for _ in range(8))))
            elif 10 <= code <= 15:
                values.append((code, (reader.rd(), reader.rd(), reader.rd())))
            elif 40 <= code <= 42:
                values.append((code, reader.rd()))
            elif code == 70:
                values.append((code, reader.rs()))
            elif code == 71:
                values.append((code, reader.rl()))
            else:
                raise ValueError(f"unsupported EED code {code}")
        assert reader.position == end
        records.append((size, application, tuple(values)))


def resolve(base, encoded):
    code, value = encoded
    if code == 6:
        return base + 1
    if code == 8:
        return base - 1
    if code == 10:
        return base + value
    if code == 12:
        return base - value
    return value


def crc16(data):
    value = 0xC0C1
    for byte in data:
        value ^= byte
        for _ in range(8):
            value = value >> 1 ^ 0xA001 if value & 1 else value >> 1
    return value


fixture = FIXTURE.read_bytes()
objects = b"".join(decode_page(fixture, page) for page in OBJECT_PAGES)
handles = decode_page(fixture, HANDLES_PAGE)
entries = handle_map(handles)

histograms = {
    name: Counter()
    for name in [
        "prefix_bytes", "payload_size", "frame_size", "handle_bits", "data_bits", "bot_selector", "object_handle_code",
        "eed_count", "eed_layout", "graphic", "entmode", "reactors", "reactor_bl", "xdic",
        "color_index", "color_bs", "color_rgb", "color_alpha",
        "color_reference", "ltype_scale_bd", "ltype", "plotstyle", "material", "shadow",
        "visual_full", "visual_face", "visual_edge", "invisibility", "invisibility_bs", "lineweight", "z_zero",
        "end_x_dd", "end_y_dd", "end_z_dd", "thickness_default", "thickness_bd",
        "extrusion_default", "extrusion_x_bd", "extrusion_y_bd", "extrusion_z_bd",
        "handle_count", "handle_codes", "handle_layout", "owner", "layer", "terminal_main_bits", "terminal_handle_bits", "terminal_handle_pattern",
    ]
}
rows = []
for map_handle, address in entries:
    if address >= len(objects):
        continue
    payload_size, handle_bits, prefix_bytes, payload = frame_prefix(objects, address)
    reader = Bits(payload)
    bot_selectors = Counter()
    object_type = bot(reader, bot_selectors)
    code, object_handle = reader.handle()
    if object_type != 19:
        continue
    stored_crc = int.from_bytes(objects[address + prefix_bytes + payload_size:address + prefix_bytes + payload_size + 2], "little")
    assert crc16(objects[address:address + prefix_bytes + payload_size]) == stored_crc
    assert object_handle == map_handle
    eed = skip_eed(reader)
    graphic = reader.bit()
    if graphic:
        reader.position += reader.bll() * 8
    entmode = reader.bb()
    reactor_bl = Counter()
    reactors = reader.bl(reactor_bl)
    xdic_missing = reader.bit()
    color_bs = Counter()
    color_raw = reader.bs(color_bs)
    color_index = color_raw & 0x1FF
    color_flags = color_raw & 0xFE00
    alpha = reader.bl() if color_flags & 0x2000 else None
    color_reference = bool(color_flags & 0x4000)
    rgb = reader.bl() if not color_reference and color_flags & 0x8000 else None
    ltype_bd = Counter()
    ltype_scale = reader.bd(ltype_bd)
    ltype = reader.bb()
    plotstyle = reader.bb()
    material = reader.bb()
    shadow = reader.byte()
    visual_full = reader.bit()
    visual_face = reader.bit()
    visual_edge = reader.bit()
    invisibility_bs = Counter()
    invisibility = reader.bs(invisibility_bs)
    lineweight = reader.byte()

    z_zero = reader.bit()
    start_x = reader.rd()
    dd_x = Counter()
    end_x = reader.dd(start_x, dd_x)
    start_y = reader.rd()
    dd_y = Counter()
    end_y = reader.dd(start_y, dd_y)
    dd_z = Counter()
    if z_zero:
        start_z = end_z = 0.0
    else:
        start_z = reader.rd()
        end_z = reader.dd(start_z, dd_z)
    thickness_default = reader.bit()
    thickness_bd = Counter()
    thickness = 0.0 if thickness_default else reader.bd(thickness_bd)
    extrusion_default = reader.bit()
    extrusion_selectors = [Counter(), Counter(), Counter()]
    extrusion = [0.0, 0.0, 1.0] if extrusion_default else [reader.bd(extrusion_selectors[index]) for index in range(3)]
    class_end = reader.position

    handle_start = payload_size * 8 - handle_bits
    handle_reader = Bits(payload, handle_start)
    roles = []
    if color_reference:
        roles.append(("color", handle_reader.handle()))
    if entmode == 0:
        roles.append(("owner", handle_reader.handle()))
    for index in range(reactors):
        roles.append((f"reactor[{index}]", handle_reader.handle()))
    if not xdic_missing:
        roles.append(("xdic", handle_reader.handle()))
    roles.append(("layer", handle_reader.handle()))
    if ltype == 3:
        roles.append(("linetype", handle_reader.handle()))
    if material == 3:
        roles.append(("material", handle_reader.handle()))
    if shadow == 3:
        roles.append(("shadow", handle_reader.handle()))
    if plotstyle == 3:
        roles.append(("plotstyle", handle_reader.handle()))
    if visual_full:
        roles.append(("visual_full", handle_reader.handle()))
    if visual_face:
        roles.append(("visual_face", handle_reader.handle()))
    if visual_edge:
        roles.append(("visual_edge", handle_reader.handle()))

    resolved = [(role, code, resolve(map_handle, (code, value))) for role, (code, value) in roles]
    total_frame_size = prefix_bytes + payload_size + 2
    data_bits = handle_start
    terminal_main = handle_start - class_end
    terminal_handles = payload_size * 8 - handle_reader.position
    assert terminal_main == 1 and Bits(payload, class_end).bit() == 0
    terminal_handle_pattern = "".join(str(handle_reader.bit()) for _ in range(terminal_handles))
    rows.append((map_handle, address, payload_size, total_frame_size, handle_bits, data_bits, class_end,
                 terminal_main, terminal_handles, start_x, start_y, start_z, end_x, end_y, end_z,
                 thickness, extrusion, resolved))

    values = {
        "prefix_bytes": prefix_bytes, "payload_size": payload_size, "frame_size": total_frame_size, "handle_bits": handle_bits,
        "data_bits": data_bits, "bot_selector": next(iter(bot_selectors)), "object_handle_code": code,
        "eed_count": len(eed), "eed_layout": tuple(eed), "graphic": graphic, "entmode": entmode,
        "reactors": reactors, "reactor_bl": next(iter(reactor_bl)), "xdic": not xdic_missing,
        "color_index": color_index, "color_bs": next(iter(color_bs)),
        "color_rgb": rgb is not None, "color_alpha": alpha is not None,
        "color_reference": color_reference, "ltype_scale_bd": next(iter(ltype_bd)), "ltype": ltype,
        "plotstyle": plotstyle, "material": material, "shadow": shadow, "visual_full": visual_full,
        "visual_face": visual_face, "visual_edge": visual_edge, "invisibility": invisibility,
        "invisibility_bs": next(iter(invisibility_bs)),
        "lineweight": lineweight, "z_zero": z_zero, "end_x_dd": next(iter(dd_x)),
        "end_y_dd": next(iter(dd_y)), "end_z_dd": "absent" if z_zero else next(iter(dd_z)),
        "thickness_default": thickness_default,
        "thickness_bd": "absent" if thickness_default else next(iter(thickness_bd)),
        "extrusion_default": extrusion_default,
        "extrusion_x_bd": "absent" if extrusion_default else next(iter(extrusion_selectors[0])),
        "extrusion_y_bd": "absent" if extrusion_default else next(iter(extrusion_selectors[1])),
        "extrusion_z_bd": "absent" if extrusion_default else next(iter(extrusion_selectors[2])),
        "handle_count": len(roles), "handle_codes": tuple(code for _, code, _ in resolved),
        "handle_layout": tuple(role for role, _, _ in resolved),
        "owner": next((value for role, _, value in resolved if role == "owner"), None),
        "layer": next(value for role, _, value in resolved if role == "layer"),
        "terminal_main_bits": terminal_main, "terminal_handle_bits": terminal_handles,
        "terminal_handle_pattern": terminal_handle_pattern,
    }
    for name, value in values.items():
        histograms[name][value] += 1

assert len(rows) == 40
assert all(math.isfinite(value) for row in rows for value in row[9:15])
print(f"handle_map_entries={len(entries)} valid_object_addresses={sum(address < len(objects) for _, address in entries)} line_frames={len(rows)} object_bytes={len(objects)}")
for name, counts in histograms.items():
    print(f"{name}={dict(sorted(counts.items(), key=lambda item: str(item[0])))}")
groups = defaultdict(list)
for row in rows:
    handle, _, payload_size, frame_size, handle_bits, data_bits, class_end, terminal_main, terminal_handles, *_, roles = row
    signature = (payload_size, frame_size, handle_bits, data_bits, class_end, terminal_main, terminal_handles, tuple(role for role, _, _ in roles))
    groups[signature].append(handle)
print("frame_groups=payload,total,handle_bits,data_bits,class_end,main_tail,handle_tail,roles:handles")
for signature, object_handles in sorted(groups.items()):
    print(f"{signature}:{[hex(handle) for handle in object_handles]}")
print("handle,address,payload,total,handle_bits,data_bits,class_end,main_tail,handle_tail,start,end,thickness,extrusion,roles")
for row in rows:
    handle, address, payload_size, frame_size, handle_bits, data_bits, class_end, terminal_main, terminal_handles, sx, sy, sz, ex, ey, ez, thickness, extrusion, roles = row
    print(f"{handle:#x},{address},{payload_size},{frame_size},{handle_bits},{data_bits},{class_end},{terminal_main},{terminal_handles},({sx:g} {sy:g} {sz:g}),({ex:g} {ey:g} {ez:g}),{thickness:g},{tuple(extrusion)},{roles}")
