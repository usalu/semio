from collections import Counter, defaultdict
from contextlib import redirect_stdout
from pathlib import Path
import io
import runpy


TICKET = Path(__file__).parent
with redirect_stdout(io.StringIO()):
    SHARED = runpy.run_path(TICKET / "🧪️dwg-line-frame-probe.py")

Bits = SHARED["Bits"]
FIXTURE = SHARED["FIXTURE"]
OBJECT_PAGES = SHARED["OBJECT_PAGES"]
HANDLES_PAGE = SHARED["HANDLES_PAGE"]
decode_page = SHARED["decode_page"]
handle_map = SHARED["handle_map"]
frame_prefix = SHARED["frame_prefix"]
bot = SHARED["bot"]
skip_eed = SHARED["skip_eed"]
resolve = SHARED["resolve"]
crc16 = SHARED["crc16"]


def common_main(reader):
    result = {}
    result["graphic"] = reader.bit()
    if result["graphic"]:
        result["graphic_bytes"] = reader.bll()
        reader.position += result["graphic_bytes"] * 8
    result["entmode"] = reader.bb()
    selectors = Counter()
    result["reactors"] = reader.bl(selectors)
    result["reactor_bl"] = next(iter(selectors))
    result["xdic"] = not reader.bit()
    selectors = Counter()
    color = reader.bs(selectors)
    result["color_bs"] = next(iter(selectors))
    result["color_index"] = color & 0x1FF
    flags = color & 0xFE00
    result["color_alpha"] = bool(flags & 0x2000)
    if result["color_alpha"]:
        reader.bl()
    result["color_reference"] = bool(flags & 0x4000)
    result["color_rgb"] = not result["color_reference"] and bool(flags & 0x8000)
    if result["color_rgb"]:
        reader.bl()
    selectors = Counter()
    result["ltype_scale"] = reader.bd(selectors)
    result["ltype_scale_bd"] = next(iter(selectors))
    result["ltype"] = reader.bb()
    result["plotstyle"] = reader.bb()
    result["material"] = reader.bb()
    result["shadow"] = reader.byte()
    result["visual_full"] = reader.bit()
    result["visual_face"] = reader.bit()
    result["visual_edge"] = reader.bit()
    selectors = Counter()
    result["invisibility"] = reader.bs(selectors)
    result["invisibility_bs"] = next(iter(selectors))
    result["lineweight"] = reader.byte()
    return result


def common_handles(payload, handle_start, base, common):
    reader = Bits(payload, handle_start)
    roles = []
    if common["color_reference"]:
        roles.append(("color", reader.handle()))
    if common["entmode"] == 0:
        roles.append(("owner", reader.handle()))
    for index in range(common["reactors"]):
        roles.append((f"reactor[{index}]", reader.handle()))
    if common["xdic"]:
        roles.append(("xdic", reader.handle()))
    roles.append(("layer", reader.handle()))
    if common["ltype"] == 3:
        roles.append(("linetype", reader.handle()))
    if common["material"] == 3:
        roles.append(("material", reader.handle()))
    if common["shadow"] == 3:
        roles.append(("shadow", reader.handle()))
    if common["plotstyle"] == 3:
        roles.append(("plotstyle", reader.handle()))
    if common["visual_full"]:
        roles.append(("visual_full", reader.handle()))
    if common["visual_face"]:
        roles.append(("visual_face", reader.handle()))
    if common["visual_edge"]:
        roles.append(("visual_edge", reader.handle()))
    return reader, [(role, code, resolve(base, (code, value))) for role, (code, value) in roles]


def selector(reader, method, *arguments):
    counts = Counter()
    value = getattr(reader, method)(*arguments, counts)
    return value, next(iter(counts))


fixture = FIXTURE.read_bytes()
objects = b"".join(decode_page(fixture, page) for page in OBJECT_PAGES)
handles = decode_page(fixture, HANDLES_PAGE)
entries = handle_map(handles)
cohorts = {17: [], 77: []}

for map_handle, address in entries:
    if address >= len(objects):
        continue
    payload_size, handle_bits, prefix_bytes, payload = frame_prefix(objects, address)
    reader = Bits(payload)
    bot_selectors = Counter()
    object_type = bot(reader, bot_selectors)
    object_handle_code, object_handle = reader.handle()
    if object_type not in cohorts:
        continue
    assert object_handle == map_handle
    stored_crc = int.from_bytes(objects[address + prefix_bytes + payload_size:address + prefix_bytes + payload_size + 2], "little")
    assert crc16(objects[address:address + prefix_bytes + payload_size]) == stored_crc
    eed = skip_eed(reader)
    common = common_main(reader)
    body = {}

    if object_type == 17:
        body["center_x"], body["center_x_bd"] = selector(reader, "bd")
        body["center_y"], body["center_y_bd"] = selector(reader, "bd")
        body["center_z"], body["center_z_bd"] = selector(reader, "bd")
        body["radius"], body["radius_bd"] = selector(reader, "bd")
        body["thickness_default"] = reader.bit()
        if body["thickness_default"]:
            body["thickness"] = 0.0
            body["thickness_bd"] = "absent"
        else:
            body["thickness"], body["thickness_bd"] = selector(reader, "bd")
        body["extrusion_default"] = reader.bit()
        if body["extrusion_default"]:
            body["extrusion"] = (0.0, 0.0, 1.0)
            body["extrusion_bd"] = ("absent",) * 3
        else:
            values = [selector(reader, "bd") for _ in range(3)]
            body["extrusion"] = tuple(value for value, _ in values)
            body["extrusion_bd"] = tuple(branch for _, branch in values)
        body["start_angle"], body["start_angle_bd"] = selector(reader, "bd")
        body["end_angle"], body["end_angle_bd"] = selector(reader, "bd")
    else:
        body["flags"], body["flags_bs"] = selector(reader, "bs")
        flags = body["flags"]
        for name, mask in [("constant_width", 4), ("elevation", 8), ("thickness", 2)]:
            if flags & mask:
                body[name], body[f"{name}_bd"] = selector(reader, "bd")
            else:
                body[name] = 0.0
                body[f"{name}_bd"] = "absent"
        if flags & 1:
            values = [selector(reader, "bd") for _ in range(3)]
            body["extrusion"] = tuple(value for value, _ in values)
            body["extrusion_bd"] = tuple(branch for _, branch in values)
        else:
            body["extrusion"] = (0.0, 0.0, 1.0)
            body["extrusion_bd"] = ("absent",) * 3
        body["point_count"], body["point_count_bl"] = selector(reader, "bl")
        for name, mask in [("bulge_count", 16), ("vertex_id_count", 1024), ("width_count", 32)]:
            if flags & mask:
                body[name], body[f"{name}_bl"] = selector(reader, "bl")
            else:
                body[name] = 0
                body[f"{name}_bl"] = "absent"
        points = []
        point_dd = Counter()
        if body["point_count"]:
            first = (reader.rd(), reader.rd())
            points.append(first)
            previous = first
            for _ in range(1, body["point_count"]):
                x, branch = selector(reader, "dd", previous[0])
                point_dd[("x", branch)] += 1
                y, branch = selector(reader, "dd", previous[1])
                point_dd[("y", branch)] += 1
                previous = (x, y)
                points.append(previous)
        body["points"] = tuple(points)
        body["point_dd"] = tuple(sorted(point_dd.items()))
        bulge_bd = Counter()
        body["bulges"] = tuple(selector(reader, "bd") for _ in range(body["bulge_count"]))
        for _, branch in body["bulges"]:
            bulge_bd[branch] += 1
        body["bulge_bd"] = tuple(sorted(bulge_bd.items()))
        body["vertex_ids"] = tuple(selector(reader, "bl") for _ in range(body["vertex_id_count"]))
        width_bd = Counter()
        widths = []
        for _ in range(body["width_count"]):
            start = selector(reader, "bd")
            end = selector(reader, "bd")
            width_bd[start[1]] += 1
            width_bd[end[1]] += 1
            widths.append((start, end))
        body["widths"] = tuple(widths)
        body["width_bd"] = tuple(sorted(width_bd.items()))

    class_end = reader.position
    handle_start = payload_size * 8 - handle_bits
    handle_reader, roles = common_handles(payload, handle_start, map_handle, common)
    main_tail = handle_start - class_end
    main_tail_pattern = "".join(str(Bits(payload, class_end).bit()) for _ in range(main_tail))
    handle_tail = payload_size * 8 - handle_reader.position
    handle_tail_pattern = "".join(str(handle_reader.bit()) for _ in range(handle_tail))
    cohorts[object_type].append({
        "handle": map_handle,
        "address": address,
        "prefix_bytes": prefix_bytes,
        "payload_size": payload_size,
        "frame_size": prefix_bytes + payload_size + 2,
        "handle_bits": handle_bits,
        "data_bits": handle_start,
        "bot_selector": next(iter(bot_selectors)),
        "object_handle_code": object_handle_code,
        "eed": tuple(eed),
        "common": common,
        "body": body,
        "class_end": class_end,
        "main_tail": main_tail,
        "main_tail_pattern": main_tail_pattern,
        "handle_tail": handle_tail,
        "handle_tail_pattern": handle_tail_pattern,
        "roles": tuple(roles),
    })


def histogram(frames, path):
    values = Counter()
    for frame in frames:
        value = frame
        for key in path:
            value = value[key]
        values[value] += 1
    return dict(sorted(values.items(), key=lambda item: str(item[0])))


def print_cohort(object_type, name):
    frames = cohorts[object_type]
    print(f"{name}_frames={len(frames)}")
    frame_paths = [
        ("prefix_bytes",), ("payload_size",), ("frame_size",), ("handle_bits",), ("data_bits",),
        ("bot_selector",), ("object_handle_code",), ("eed",), ("class_end",), ("main_tail",),
        ("main_tail_pattern",), ("handle_tail",), ("handle_tail_pattern",),
    ]
    common_paths = [
        "graphic", "entmode", "reactors", "reactor_bl", "xdic", "color_index", "color_bs",
        "color_alpha", "color_reference", "color_rgb", "ltype_scale", "ltype_scale_bd", "ltype",
        "plotstyle", "material", "shadow", "visual_full", "visual_face", "visual_edge",
        "invisibility", "invisibility_bs", "lineweight",
    ]
    for path in frame_paths:
        print(f"{path[-1]}={histogram(frames, path)}")
    for name in common_paths:
        print(f"common.{name}={histogram(frames, ('common', name))}")
    print(f"handle_layout={Counter(tuple(role for role, _, _ in frame['roles']) for frame in frames)}")
    print(f"handle_codes={Counter(tuple(code for _, code, _ in frame['roles']) for frame in frames)}")
    print(f"owners={Counter(next((value for role, _, value in frame['roles'] if role == 'owner'), None) for frame in frames)}")
    print(f"layers={Counter(next(value for role, _, value in frame['roles'] if role == 'layer') for frame in frames)}")
    groups = defaultdict(list)
    for frame in frames:
        signature = (
            frame["payload_size"], frame["frame_size"], frame["handle_bits"], frame["data_bits"],
            frame["class_end"], frame["main_tail_pattern"], frame["handle_tail_pattern"],
            tuple(role for role, _, _ in frame["roles"]),
        )
        groups[signature].append(frame["handle"])
    print("frame_groups=payload,total,handle_bits,data_bits,class_end,main_tail,handle_tail,roles:handles")
    for signature, object_handles in sorted(groups.items()):
        print(f"{signature}:{[hex(handle) for handle in object_handles]}")


assert len(cohorts[17]) == 12
assert len(cohorts[77]) == 16
print_cohort(17, "ARC")
for field in [
    "center_x_bd", "center_y_bd", "center_z_bd", "radius_bd", "thickness_default", "thickness_bd",
    "extrusion_default", "extrusion_bd", "start_angle_bd", "end_angle_bd",
]:
    print(f"ARC.body.{field}={histogram(cohorts[17], ('body', field))}")
print_cohort(77, "LWPOLYLINE")
for field in [
    "flags", "flags_bs", "constant_width_bd", "elevation_bd", "thickness_bd", "extrusion_bd",
    "point_count", "point_count_bl", "bulge_count", "bulge_count_bl", "vertex_id_count",
    "vertex_id_count_bl", "width_count", "width_count_bl", "point_dd", "bulge_bd", "width_bd",
]:
    print(f"LWPOLYLINE.body.{field}={histogram(cohorts[77], ('body', field))}")
