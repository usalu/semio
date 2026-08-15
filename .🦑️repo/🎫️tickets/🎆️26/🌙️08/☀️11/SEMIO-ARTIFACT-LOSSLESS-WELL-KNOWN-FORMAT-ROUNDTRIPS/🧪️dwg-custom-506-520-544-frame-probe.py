from collections import Counter, defaultdict
from contextlib import redirect_stdout
from pathlib import Path
import io
import runpy


TICKET = Path(__file__).parent
with redirect_stdout(io.StringIO()):
    SHARED = runpy.run_path(TICKET / "🧪️dwg-insert-dimension-viewport-frame-probe.py")

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
read_string_stream = SHARED["read_string_stream"]
read_tu = SHARED["read_tu"]
selector = SHARED["selector"]


def signed(value, bits):
    sign = 1 << (bits - 1)
    return value - (1 << bits) if value & sign else value


def read_bl(reader):
    return selector(reader, "bl")


def read_bs(reader):
    return selector(reader, "bs")


def read_bd(reader):
    return selector(reader, "bd")


def read_cmc(reader, strings):
    index, index_branch = read_bs(reader)
    rgb, rgb_branch = read_bl(reader)
    flag = reader.byte()
    return {
        "index": index,
        "index_branch": index_branch,
        "rgb": rgb,
        "rgb_branch": rgb_branch,
        "flag": flag,
        "name": read_tu(strings) if flag & 1 else None,
        "book_name": read_tu(strings) if flag & 2 else None,
    }


def read_common_object(data, handles, base):
    reactor_count, reactor_branch = read_bl(data)
    xdic_missing = bool(data.bit())
    roles = []
    code, value = handles.handle()
    roles.append(("owner", code, resolve(base, (code, value))))
    for index in range(reactor_count):
        code, value = handles.handle()
        roles.append((f"reactor[{index}]", code, resolve(base, (code, value))))
    if not xdic_missing:
        code, value = handles.handle()
        roles.append(("extension_dictionary", code, resolve(base, (code, value))))
    return {"reactor_count": reactor_count, "reactor_branch": reactor_branch, "xdic_missing": xdic_missing}, roles


def read_visual_style(data, strings):
    body = {"description": read_tu(strings)}
    body["style_type"], body["style_type_branch"] = read_bl(data)
    body["extension_lighting_model"], body["extension_lighting_model_branch"] = read_bs(data)
    body["internal_only"] = data.bit()
    schema = [
        ("face_lighting_model", "bl"),
        ("face_lighting_quality", "bl"),
        ("face_color_mode", "bl"),
        ("face_modifier", "bs"),
        ("face_opacity", "bd"),
        ("face_specular", "bd"),
        ("face_mono_color", "cmc"),
        ("edge_model", "bl"),
        ("edge_style", "bl"),
        ("edge_intersection_color", "cmc"),
        ("edge_obscured_color", "cmc"),
        ("edge_obscured_linetype", "bl"),
        ("edge_intersection_linetype", "bl"),
        ("edge_crease_angle", "bd"),
        ("edge_modifier", "bl"),
        ("edge_color", "cmc"),
        ("edge_opacity", "bd"),
        ("edge_width", "bl"),
        ("edge_overhang", "bl"),
        ("edge_jitter", "bl"),
        ("edge_silhouette_color", "cmc"),
        ("edge_silhouette_width", "bl"),
        ("edge_halo_gap", "bl"),
        ("edge_isolines", "bl"),
        ("edge_hide_precision", "b"),
        ("display_settings", "bl"),
        ("display_brightness", "bd"),
        ("display_shadow_type", "bl"),
    ]
    for name, kind in schema:
        if kind == "cmc":
            body[name] = read_cmc(data, strings)
        elif kind == "b":
            body[name] = bool(data.bit())
        else:
            body[name], body[f"{name}_branch"] = globals()[f"read_{kind}"](data)
        body[f"{name}_modifier"], body[f"{name}_modifier_branch"] = read_bs(data)
    return body, []


def read_eval_expression(data, strings):
    body = {}
    value, body["parent_id_branch"] = read_bl(data)
    body["parent_id"] = signed(value, 32)
    body["major"], body["major_branch"] = read_bl(data)
    body["minor"], body["minor_branch"] = read_bl(data)
    value, body["value_code_branch"] = read_bs(data)
    body["value_code"] = signed(value, 16)
    roles = []
    if body["value_code"] == 40:
        body["value"], body["value_branch"] = read_bd(data)
    elif body["value_code"] in (10, 11):
        body["value"] = (data.rd(), data.rd())
        body["value_branch"] = "2RD"
    elif body["value_code"] == 1:
        body["value"] = read_tu(strings)
        body["value_branch"] = "T"
    elif body["value_code"] == 90:
        body["value"], body["value_branch"] = read_bl(data)
    elif body["value_code"] == 91:
        body["value"] = None
        body["value_branch"] = "H"
        roles.append(("evaluation_value", 5))
    elif body["value_code"] == 70:
        body["value"], body["value_branch"] = read_bs(data)
    elif body["value_code"] == -9999:
        body["value"] = None
        body["value_branch"] = "none"
    else:
        raise ValueError(f"unsupported evaluation value code {body['value_code']}")
    body["node_id"], body["node_id_branch"] = read_bl(data)
    return body, roles


def read_block_grip_location(data, strings):
    expression, roles = read_eval_expression(data, strings)
    grip_type, grip_type_branch = read_bl(data)
    return {
        "expression": expression,
        "grip_type": grip_type,
        "grip_type_branch": grip_type_branch,
        "grip_expression": read_tu(strings),
    }, roles


def read_assoc_geom_dependency(data, strings):
    dependency = {}
    dependency["class_version"], dependency["class_version_branch"] = read_bs(data)
    dependency["status"], dependency["status_branch"] = read_bl(data)
    for field in ["read", "write", "attached", "delegating"]:
        dependency[field] = bool(data.bit())
    value, dependency["order_branch"] = read_bl(data)
    dependency["order"] = signed(value, 32)
    dependency["has_name"] = bool(data.bit())
    dependency["name"] = read_tu(strings) if dependency["has_name"] else None
    value, dependency["body_id_branch"] = read_bl(data)
    dependency["body_id"] = signed(value, 32)
    class_version, class_version_branch = read_bs(data)
    enabled = bool(data.bit())
    class_name = read_tu(strings)
    compound = bool(data.bit())
    return {
        "dependency": dependency,
        "class_version": class_version,
        "class_version_branch": class_version_branch,
        "enabled": enabled,
        "persistent_subentity_class": class_name,
        "dependent_on_compound_object": compound,
    }, [("dependent_on", 3), ("read_dependency", 4), ("action_node", 3), ("dependency_body", 4)]


def freeze(value):
    if isinstance(value, dict):
        return tuple((key, freeze(item)) for key, item in sorted(value.items()))
    if isinstance(value, (list, tuple)):
        return tuple(freeze(item) for item in value)
    return value


fixture = FIXTURE.read_bytes()
objects = b"".join(decode_page(fixture, page) for page in OBJECT_PAGES)
handles_page = decode_page(fixture, HANDLES_PAGE)
entries = handle_map(handles_page)
cohorts = {506: [], 520: [], 544: []}

for map_handle, address in entries:
    if address >= len(objects):
        continue
    payload_size, handle_bits, prefix_bytes, payload = frame_prefix(objects, address)
    data = Bits(payload)
    bot_selectors = Counter()
    object_type = bot(data, bot_selectors)
    object_handle_code, object_handle = data.handle()
    if object_type not in cohorts:
        continue
    assert object_handle == map_handle
    stored_crc = int.from_bytes(
        objects[address + prefix_bytes + payload_size:address + prefix_bytes + payload_size + 2], "little"
    )
    assert crc16(objects[address:address + prefix_bytes + payload_size]) == stored_crc
    eed = skip_eed(data)
    handle_start = payload_size * 8 - handle_bits
    strings, string_start, string_bits, strings_present = read_string_stream(payload, handle_start)
    handles = Bits(payload, handle_start)
    common, roles = read_common_object(data, handles, map_handle)
    if object_type == 506:
        body, class_roles = read_visual_style(data, strings)
    elif object_type == 520:
        body, class_roles = read_block_grip_location(data, strings)
    else:
        body, class_roles = read_assoc_geom_dependency(data, strings)
    for role, expected_code in class_roles:
        code, value = handles.handle()
        roles.append((role, code, resolve(map_handle, (code, value))))
    class_end = data.position
    assert class_end == string_start, (object_type, hex(map_handle), class_end, string_start)
    assert strings.position == string_start + string_bits, (
        object_type, hex(map_handle), strings.position, string_start, string_bits
    )
    handle_tail = len(payload) * 8 - handles.position
    handle_tail_pattern = "".join(str(handles.bit()) for _ in range(handle_tail))
    cohorts[object_type].append({
        "handle": map_handle,
        "prefix_bytes": prefix_bytes,
        "payload_size": payload_size,
        "frame_size": prefix_bytes + payload_size + 2,
        "handle_bits": handle_bits,
        "data_bits": handle_start,
        "class_end": class_end,
        "bot_selector": next(iter(bot_selectors)),
        "object_handle_code": object_handle_code,
        "stored_crc": stored_crc,
        "eed": tuple(eed),
        "common": common,
        "body": body,
        "strings_present": strings_present,
        "string_bits": string_bits,
        "roles": tuple(roles),
        "handle_tail": handle_tail,
        "handle_tail_pattern": handle_tail_pattern,
    })


def histogram(frames, *path):
    values = Counter()
    for frame in frames:
        value = frame
        for key in path:
            value = value[key]
        values[freeze(value)] += 1
    return dict(sorted(values.items(), key=lambda item: str(item[0])))


def print_cohort(object_type, name, body_paths):
    frames = cohorts[object_type]
    print(f"{name}_frames={len(frames)}")
    for field in [
        "prefix_bytes", "payload_size", "frame_size", "handle_bits", "data_bits", "class_end",
        "bot_selector", "object_handle_code", "eed", "strings_present", "string_bits", "handle_tail",
        "handle_tail_pattern",
    ]:
        print(f"{field}={histogram(frames, field)}")
    for field in ["reactor_count", "reactor_branch", "xdic_missing"]:
        print(f"common.{field}={histogram(frames, 'common', field)}")
    for path in body_paths:
        print(f"body.{'.'.join(path)}={histogram(frames, 'body', *path)}")
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
    print("frame_groups=payload,total,handle_bits,data_bits,class_end,string_bits,tail,roles:handles")
    for signature, object_handles in sorted(groups.items()):
        print(f"{signature}:{[hex(handle) for handle in object_handles]}")
    print("crc=handle:value")
    print(" ".join(f"{frame['handle']:x}:{frame['stored_crc']:04x}" for frame in frames))


assert len(cohorts[506]) == 19
assert len(cohorts[520]) == 23
assert len(cohorts[544]) == 31
print_cohort(506, "VISUALSTYLE", [
    ("description",), ("style_type",), ("style_type_branch",),
    ("extension_lighting_model",), ("extension_lighting_model_branch",), ("internal_only",),
    ("face_lighting_model",), ("face_lighting_model_branch",), ("face_lighting_model_modifier",),
    ("face_lighting_quality",), ("face_color_mode",), ("face_modifier",),
    ("face_opacity",), ("face_specular",), ("edge_model",), ("edge_style",),
    ("edge_crease_angle",), ("edge_modifier",), ("edge_opacity",), ("edge_width",),
    ("edge_overhang",), ("edge_jitter",), ("edge_silhouette_width",), ("edge_halo_gap",),
    ("edge_isolines",), ("edge_hide_precision",), ("display_settings",),
    ("display_brightness",), ("display_shadow_type",),
])
print_cohort(520, "BLOCKGRIPLOCATIONCOMPONENT", [
    ("expression", "parent_id"), ("expression", "parent_id_branch"),
    ("expression", "major"), ("expression", "major_branch"),
    ("expression", "minor"), ("expression", "minor_branch"),
    ("expression", "value_code"), ("expression", "value_code_branch"),
    ("expression", "value_branch"), ("expression", "node_id"),
    ("expression", "node_id_branch"), ("grip_type",), ("grip_type_branch",),
    ("grip_expression",),
])
print_cohort(544, "ACDBASSOCGEOMDEPENDENCY", [
    ("dependency", "class_version"), ("dependency", "class_version_branch"),
    ("dependency", "status"), ("dependency", "status_branch"),
    ("dependency", "read"), ("dependency", "write"), ("dependency", "attached"),
    ("dependency", "delegating"), ("dependency", "order"), ("dependency", "order_branch"),
    ("dependency", "has_name"), ("dependency", "name"),
    ("dependency", "body_id"), ("dependency", "body_id_branch"),
    ("class_version",), ("class_version_branch",), ("enabled",),
    ("persistent_subentity_class",), ("dependent_on_compound_object",),
])
