from collections import Counter
from contextlib import redirect_stdout
from pathlib import Path
import io
import runpy


TICKET = Path(__file__).parent
with redirect_stdout(io.StringIO()):
    CUSTOM = runpy.run_path(TICKET / "🧪️dwg-custom-541-545-thin-frame-probe.py")
    ENTITY = runpy.run_path(TICKET / "🧪️dwg-insert-dimension-viewport-frame-probe.py")
    MAP = runpy.run_path(TICKET / "🧪️dwg-object-handles-reconstruction-probe.py")

Bits = CUSTOM["Bits"]
objects = CUSTOM["objects"]
frame_prefix = CUSTOM["frame_prefix"]
bot = CUSTOM["bot"]
skip_eed = CUSTOM["skip_eed"]
crc16 = CUSTOM["crc16"]
resolve = CUSTOM["resolve"]
read_string_stream = CUSTOM["read_string_stream"]
read_common_object = CUSTOM["read_common_object"]
read_value_dependency = CUSTOM["read_value_dependency"]
read_dependency = CUSTOM["read_dependency"]
read_assoc_variable = CUSTOM["read_assoc_variable"]
read_dependency_body = CUSTOM["read_dependency_body"]
selector = ENTITY["selector"]
common_main = ENTITY["common_main"]
common_handles = ENTITY["common_handles"]
read_tu = ENTITY["read_tu"]
entries = MAP["entries"]

KNOWN = {
    21: "DIMENSION_LINEAR",
    42: "DICTIONARY",
    49: "BLOCK_HEADER",
    69: "DIMSTYLE",
    539: "ACDBASSOCNETWORK",
    541: "ACDBASSOCVALUEDEPENDENCY",
    542: "ACDBASSOCDEPENDENCY",
    545: "ACDBASSOCVARIABLE",
    549: "ASSOCDIMDEPENDENCYBODY",
}


def read_3bd(reader):
    values = [selector(reader, "bd") for _ in range(3)]
    return tuple(value for value, _ in values), tuple(branch for _, branch in values)


def read_dimension(data, strings):
    body = {"class_version": data.byte()}
    body["extrusion"], body["extrusion_branches"] = read_3bd(data)
    body["text_midpoint"] = (data.rd(), data.rd())
    body["elevation"], body["elevation_branch"] = selector(data, "bd")
    body["flag1"] = data.byte()
    body["text_rotation"], body["text_rotation_branch"] = selector(data, "bd")
    body["horizontal_direction"], body["horizontal_direction_branch"] = selector(data, "bd")
    body["insertion_scale"], body["insertion_scale_branches"] = read_3bd(data)
    body["insertion_rotation"], body["insertion_rotation_branch"] = selector(data, "bd")
    body["attachment"], body["attachment_branch"] = selector(data, "bs")
    body["line_space_style"], body["line_space_style_branch"] = selector(data, "bs")
    body["line_space_factor"], body["line_space_factor_branch"] = selector(data, "bd")
    body["measurement"], body["measurement_branch"] = selector(data, "bd")
    body["reserved"] = bool(data.bit())
    body["flip_arrow_1"] = bool(data.bit())
    body["flip_arrow_2"] = bool(data.bit())
    body["clone_insertion"] = (data.rd(), data.rd())
    body["extension_line_1"], body["extension_line_1_branches"] = read_3bd(data)
    body["extension_line_2"], body["extension_line_2_branches"] = read_3bd(data)
    body["definition_point"], body["definition_point_branches"] = read_3bd(data)
    body["oblique_angle"], body["oblique_angle_branch"] = selector(data, "bd")
    body["dimension_rotation"], body["dimension_rotation_branch"] = selector(data, "bd")
    body["user_text"] = read_tu(strings)
    return body, [("dimension_style", 5), ("dimension_block", 5)]


def freeze(value):
    if isinstance(value, dict):
        return tuple((key, freeze(item)) for key, item in sorted(value.items()))
    if isinstance(value, (list, tuple)):
        return tuple(freeze(item) for item in value)
    return value


type_by_handle = {}
frame_by_handle = {}
for handle, address in entries:
    payload_size, handle_bits, prefix_bytes, payload = frame_prefix(objects, address)
    reader = Bits(payload)
    object_type = bot(reader)
    type_by_handle[handle] = object_type
    frame_by_handle[handle] = (address, payload_size, handle_bits, prefix_bytes, payload)


def parse(handle):
    address, payload_size, handle_bits, prefix_bytes, payload = frame_by_handle[handle]
    data = Bits(payload)
    selectors = Counter()
    object_type = bot(data, selectors)
    object_handle_code, object_handle = data.handle()
    assert object_handle == handle
    eed = tuple(skip_eed(data))
    handle_start = payload_size * 8 - handle_bits
    strings, string_start, string_bits, strings_present = read_string_stream(payload, handle_start)
    handles = Bits(payload, handle_start)
    if object_type == 21:
        common = common_main(data)
        body, class_roles = read_dimension(data, strings)
        handles, roles = common_handles(payload, handle_start, handle, common)
    else:
        common, roles = read_common_object(data, handles, handle)
        if object_type == 541:
            body, class_roles = read_value_dependency(data, strings)
        elif object_type == 542:
            body, class_roles = read_dependency(data, strings)
        elif object_type == 545:
            body, class_roles = read_assoc_variable(data, strings)
        elif object_type == 549:
            body, class_roles = read_dependency_body(data, strings)
        else:
            raise ValueError(object_type)
    for role, expected_code in class_roles:
        code, value = handles.handle()
        target = resolve(handle, (code, value))
        roles.append((role, code, target))
    assert data.position == string_start, (hex(handle), data.position, string_start)
    assert strings.position == string_start + string_bits
    tail = len(payload) * 8 - handles.position
    tail_pattern = "".join(str(handles.bit()) for _ in range(tail))
    frame_end = address + prefix_bytes + payload_size + 2
    stored_crc = int.from_bytes(objects[frame_end - 2:frame_end], "little")
    assert stored_crc == crc16(objects[address:frame_end - 2])
    return {
        "handle": handle,
        "address": address,
        "type": object_type,
        "class": KNOWN[object_type],
        "prefix_bytes": prefix_bytes,
        "payload_size": payload_size,
        "frame_size": prefix_bytes + payload_size + 2,
        "handle_bits": handle_bits,
        "data_bits": handle_start,
        "class_end": data.position,
        "bot_selector": next(iter(selectors)),
        "object_handle_code": object_handle_code,
        "eed": eed,
        "common": common,
        "body": body,
        "strings_present": strings_present,
        "string_bits": string_bits,
        "roles": tuple(roles),
        "tail": tail,
        "tail_pattern": tail_pattern,
        "crc": stored_crc,
    }


recovered_handles = [0x2255, 0x2256, 0x2257, 0x2258, 0x2259, 0x225A, 0x2266, 0x2267, 0x2268, 0x2269, 0x226A]
frames = [parse(handle) for handle in recovered_handles]
for frame in frames:
    print(f"frame={frame['handle']:#x} address={frame['address']} class={frame['class']} type={frame['type']} payload={frame['payload_size']} total={frame['frame_size']} handle_bits={frame['handle_bits']} data_bits={frame['data_bits']} class_end={frame['class_end']} string_bits={frame['string_bits']} tail={frame['tail_pattern']} crc={frame['crc']:04x}")
    print(f"  common={frame['common']}")
    print(f"  body={frame['body']}")
    graph = []
    for role, code, target in frame["roles"]:
        target_type = type_by_handle.get(target)
        graph.append(f"{role}:code{code}->{target:#x}:type{target_type}:{KNOWN.get(target_type, 'OTHER')}")
    print("  graph=" + ",".join(graph))

print("recovered_type_counts=" + repr(dict(sorted(Counter(frame["type"] for frame in frames).items()))))
for object_type in [21, 541, 542, 545, 549]:
    cohort = [parse(handle) for handle, _ in entries if type_by_handle[handle] == object_type]
    print(f"cohort={KNOWN[object_type]} count={len(cohort)} payloads={dict(sorted(Counter(frame['payload_size'] for frame in cohort).items()))} totals={dict(sorted(Counter(frame['frame_size'] for frame in cohort).items()))} tails={dict(sorted(Counter(frame['tail_pattern'] for frame in cohort).items()))}")
    print("  crcs=" + " ".join(f"{frame['handle']:x}:{frame['crc']:04x}" for frame in cohort))
