from collections import Counter, defaultdict
from contextlib import redirect_stdout
from pathlib import Path
import io
import runpy


TICKET = Path(__file__).parent
with redirect_stdout(io.StringIO()):
    SHARED = runpy.run_path(TICKET / "🧪️dwg-arc-lwpolyline-frame-probe.py")

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
common_main = SHARED["common_main"]
common_handles = SHARED["common_handles"]
selector = SHARED["selector"]


def read_3bd(reader, prefix, output):
    values = [selector(reader, "bd") for _ in range(3)]
    output[prefix] = tuple(value for value, _ in values)
    output[f"{prefix}_bd"] = tuple(branch for _, branch in values)


def read_string_stream(payload, end_bit):
    present = Bits(payload, end_bit - 1).bit()
    if not present:
        return Bits(payload, end_bit - 1), end_bit - 1, 0, False
    low = Bits(payload, end_bit - 17).rs()
    if low & 0x8000:
        high = Bits(payload, end_bit - 33).rs()
        size = (low & 0x7FFF) | high << 15
        header = 33
    else:
        size = low
        header = 17
    start = end_bit - header - size
    return Bits(payload, start), start, size, True


def read_tu(reader):
    length = reader.bs()
    units = [reader.rs() for _ in range(length)]
    return bytes(byte for unit in units for byte in unit.to_bytes(2, "little")).decode("utf-16le")


def read_cmc_main(reader):
    index, index_bs = selector(reader, "bs")
    rgb, rgb_bl = selector(reader, "bl")
    flag = reader.byte()
    return {"index": index, "index_bs": index_bs, "rgb": rgb, "rgb_bl": rgb_bl, "flag": flag}


def finish_handles(payload, handle_start, base, common, class_roles):
    reader, roles = common_handles(payload, handle_start, base, common)
    for role, expected_code in class_roles:
        code, value = reader.handle()
        roles.append((role, code, resolve(base, (code, value))))
    tail = len(payload) * 8 - reader.position
    pattern = "".join(str(reader.bit()) for _ in range(tail))
    return tuple(roles), tail, pattern


fixture = FIXTURE.read_bytes()
objects = b"".join(decode_page(fixture, page) for page in OBJECT_PAGES)
handles = decode_page(fixture, HANDLES_PAGE)
entries = handle_map(handles)
cohorts = {7: [], 21: [], 34: []}

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
    class_roles = []
    handle_start = payload_size * 8 - handle_bits
    strings, string_start, string_bits, strings_present = read_string_stream(payload, handle_start)

    if object_type == 7:
        read_3bd(reader, "insertion", body)
        body["scale_flag"] = reader.bb()
        if body["scale_flag"] == 3:
            body["scale"] = (1.0, 1.0, 1.0)
            body["scale_dd"] = ("absent", "absent")
        elif body["scale_flag"] == 2:
            x = reader.rd()
            body["scale"] = (x, x, x)
            body["scale_dd"] = ("absent", "absent")
        elif body["scale_flag"] == 1:
            y, y_branch = selector(reader, "dd", 1.0)
            z, z_branch = selector(reader, "dd", 1.0)
            body["scale"] = (1.0, y, z)
            body["scale_dd"] = (y_branch, z_branch)
        else:
            x = reader.rd()
            y, y_branch = selector(reader, "dd", x)
            z, z_branch = selector(reader, "dd", x)
            body["scale"] = (x, y, z)
            body["scale_dd"] = (y_branch, z_branch)
        body["rotation"], body["rotation_bd"] = selector(reader, "bd")
        read_3bd(reader, "extrusion", body)
        body["has_attributes"] = reader.bit()
        if body["has_attributes"]:
            body["attribute_count"], body["attribute_count_bl"] = selector(reader, "bl")
        else:
            body["attribute_count"] = 0
            body["attribute_count_bl"] = "absent"
        class_roles.append(("block_header", 5))
        class_roles.extend((f"attribute[{index}]", 4) for index in range(body["attribute_count"]))
        if body["has_attributes"]:
            class_roles.append(("sequence_end", 3))
    elif object_type == 21:
        body["class_version"] = reader.byte()
        read_3bd(reader, "extrusion", body)
        body["text_midpoint"] = (reader.rd(), reader.rd())
        body["elevation"], body["elevation_bd"] = selector(reader, "bd")
        body["flag1"] = reader.byte()
        body["text_rotation"], body["text_rotation_bd"] = selector(reader, "bd")
        body["horizontal_direction"], body["horizontal_direction_bd"] = selector(reader, "bd")
        read_3bd(reader, "insertion_scale", body)
        body["insertion_rotation"], body["insertion_rotation_bd"] = selector(reader, "bd")
        body["attachment"], body["attachment_bs"] = selector(reader, "bs")
        body["line_space_style"], body["line_space_style_bs"] = selector(reader, "bs")
        body["line_space_factor"], body["line_space_factor_bd"] = selector(reader, "bd")
        body["measurement"], body["measurement_bd"] = selector(reader, "bd")
        body["reserved"] = reader.bit()
        body["flip_arrow_1"] = reader.bit()
        body["flip_arrow_2"] = reader.bit()
        body["clone_insertion"] = (reader.rd(), reader.rd())
        read_3bd(reader, "extension_line_1", body)
        read_3bd(reader, "extension_line_2", body)
        read_3bd(reader, "definition_point", body)
        body["oblique_angle"], body["oblique_angle_bd"] = selector(reader, "bd")
        body["dimension_rotation"], body["dimension_rotation_bd"] = selector(reader, "bd")
        body["user_text"] = read_tu(strings)
        class_roles.extend([("dimension_style", 5), ("dimension_block", 5)])
    else:
        read_3bd(reader, "center", body)
        body["width"], body["width_bd"] = selector(reader, "bd")
        body["height"], body["height_bd"] = selector(reader, "bd")
        read_3bd(reader, "view_target", body)
        read_3bd(reader, "view_direction", body)
        for name in ["twist", "view_height", "lens_length", "front_clip", "back_clip", "snap_angle"]:
            body[name], body[f"{name}_bd"] = selector(reader, "bd")
        body["view_center"] = (reader.rd(), reader.rd())
        body["snap_base"] = (reader.rd(), reader.rd())
        body["snap_unit"] = (reader.rd(), reader.rd())
        body["grid_unit"] = (reader.rd(), reader.rd())
        body["circle_zoom"], body["circle_zoom_bs"] = selector(reader, "bs")
        body["grid_major"], body["grid_major_bs"] = selector(reader, "bs")
        body["frozen_count"], body["frozen_count_bl"] = selector(reader, "bl")
        body["status_flags"], body["status_flags_bl"] = selector(reader, "bl")
        body["render_mode"] = reader.byte()
        body["ucs_at_origin"] = reader.bit()
        body["ucs_per_viewport"] = reader.bit()
        read_3bd(reader, "ucs_origin", body)
        read_3bd(reader, "ucs_x_axis", body)
        read_3bd(reader, "ucs_y_axis", body)
        body["ucs_elevation"], body["ucs_elevation_bd"] = selector(reader, "bd")
        body["orthographic_view"], body["orthographic_view_bs"] = selector(reader, "bs")
        body["shade_plot_mode"], body["shade_plot_mode_bs"] = selector(reader, "bs")
        body["default_lights"] = reader.bit()
        body["lighting_type"] = reader.byte()
        body["brightness"], body["brightness_bd"] = selector(reader, "bd")
        body["contrast"], body["contrast_bd"] = selector(reader, "bd")
        body["ambient_color"] = read_cmc_main(reader)
        body["style_sheet"] = read_tu(strings)
        if body["ambient_color"]["flag"] & 1:
            body["ambient_color"]["name"] = read_tu(strings)
        if body["ambient_color"]["flag"] & 2:
            body["ambient_color"]["book_name"] = read_tu(strings)
        class_roles.extend((f"frozen_layer[{index}]", 4) for index in range(body["frozen_count"]))
        class_roles.extend([
            ("clip_boundary", 5), ("named_ucs", 5), ("base_ucs", 5), ("background", 4),
            ("visual_style", 5), ("shade_plot", 4), ("sun", 3),
        ])

    class_end = reader.position
    assert class_end == string_start
    assert strings.position == string_start + string_bits
    roles, handle_tail, handle_tail_pattern = finish_handles(payload, handle_start, map_handle, common, class_roles)
    cohorts[object_type].append({
        "handle": map_handle, "address": address, "prefix_bytes": prefix_bytes,
        "payload_size": payload_size, "frame_size": prefix_bytes + payload_size + 2,
        "handle_bits": handle_bits, "data_bits": handle_start, "class_end": class_end,
        "bot_selector": next(iter(bot_selectors)), "object_handle_code": object_handle_code,
        "eed": tuple(eed), "common": common, "body": body, "strings_present": strings_present,
        "string_bits": string_bits, "roles": roles, "handle_tail": handle_tail,
        "handle_tail_pattern": handle_tail_pattern,
    })


def freeze(value):
    if isinstance(value, dict):
        return tuple((key, freeze(item)) for key, item in sorted(value.items()))
    if isinstance(value, (list, tuple)):
        return tuple(freeze(item) for item in value)
    return value


def histogram(frames, *path):
    values = Counter()
    for frame in frames:
        value = frame
        for key in path:
            value = value[key]
        values[freeze(value)] += 1
    return dict(sorted(values.items(), key=lambda item: str(item[0])))


def print_cohort(object_type, name, body_fields):
    frames = cohorts[object_type]
    print(f"{name}_frames={len(frames)}")
    for field in ["prefix_bytes", "payload_size", "frame_size", "handle_bits", "data_bits", "class_end", "eed", "strings_present", "string_bits", "handle_tail", "handle_tail_pattern"]:
        print(f"{field}={histogram(frames, field)}")
    for field in ["graphic", "entmode", "reactors", "reactor_bl", "xdic", "color_index", "color_bs", "ltype_scale", "ltype_scale_bd", "ltype", "plotstyle", "material", "shadow", "visual_full", "visual_face", "visual_edge", "invisibility", "invisibility_bs", "lineweight"]:
        print(f"common.{field}={histogram(frames, 'common', field)}")
    for field in body_fields:
        print(f"body.{field}={histogram(frames, 'body', field)}")
    print(f"handle_layout={Counter(tuple(role for role, _, _ in frame['roles']) for frame in frames)}")
    print(f"handle_codes={Counter(tuple(code for _, code, _ in frame['roles']) for frame in frames)}")
    groups = defaultdict(list)
    for frame in frames:
        signature = (
            frame["payload_size"], frame["frame_size"], frame["handle_bits"], frame["data_bits"],
            frame["class_end"], frame["string_bits"], frame["handle_tail_pattern"],
            tuple(role for role, _, _ in frame["roles"]),
        )
        groups[signature].append(frame["handle"])
    print("frame_groups=payload,total,handle_bits,data_bits,class_end,string_bits,handle_tail,roles:handles")
    for signature, object_handles in sorted(groups.items()):
        print(f"{signature}:{[hex(handle) for handle in object_handles]}")


assert len(cohorts[7]) == 12
assert len(cohorts[21]) == 12
assert len(cohorts[34]) == 2
print_cohort(7, "INSERT", ["insertion_bd", "scale_flag", "scale", "scale_dd", "rotation_bd", "extrusion_bd", "has_attributes", "attribute_count", "attribute_count_bl"])
print_cohort(21, "DIMENSION_LINEAR", ["class_version", "extrusion_bd", "elevation_bd", "flag1", "user_text", "text_rotation_bd", "horizontal_direction_bd", "insertion_scale_bd", "insertion_rotation_bd", "attachment", "attachment_bs", "line_space_style", "line_space_style_bs", "line_space_factor_bd", "measurement_bd", "reserved", "flip_arrow_1", "flip_arrow_2", "extension_line_1_bd", "extension_line_2_bd", "definition_point_bd", "oblique_angle_bd", "dimension_rotation_bd"])
print_cohort(34, "VIEWPORT", ["center_bd", "width_bd", "height_bd", "view_target_bd", "view_direction_bd", "twist_bd", "view_height_bd", "lens_length_bd", "front_clip_bd", "back_clip_bd", "snap_angle_bd", "circle_zoom", "circle_zoom_bs", "grid_major", "grid_major_bs", "frozen_count", "frozen_count_bl", "status_flags", "status_flags_bl", "style_sheet", "render_mode", "ucs_at_origin", "ucs_per_viewport", "ucs_origin_bd", "ucs_x_axis_bd", "ucs_y_axis_bd", "ucs_elevation_bd", "orthographic_view", "orthographic_view_bs", "shade_plot_mode", "shade_plot_mode_bs", "default_lights", "lighting_type", "brightness_bd", "contrast_bd", "ambient_color"])
