from collections import Counter, defaultdict
from contextlib import redirect_stdout
from pathlib import Path
import io
import runpy


TICKET = Path(__file__).parent
with redirect_stdout(io.StringIO()):
    SHARED = runpy.run_path(TICKET / "🧪️dwg-custom-506-520-544-frame-probe.py")

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
read_common_object = SHARED["read_common_object"]
read_eval_expression = SHARED["read_eval_expression"]
read_bl = SHARED["read_bl"]
read_bs = SHARED["read_bs"]
read_bd = SHARED["read_bd"]
signed = SHARED["signed"]
freeze = SHARED["freeze"]


def read_dependency(data, strings):
    body = {}
    body["class_version"], body["class_version_branch"] = read_bs(data)
    body["status"], body["status_branch"] = read_bl(data)
    for field in ["read", "write", "attached", "delegating"]:
        body[field] = bool(data.bit())
    value, body["order_branch"] = read_bl(data)
    body["order"] = signed(value, 32)
    body["has_name"] = bool(data.bit())
    body["name"] = read_tu(strings) if body["has_name"] else None
    value, body["body_id_branch"] = read_bl(data)
    body["body_id"] = signed(value, 32)
    return body, [
        ("dependent_on", 3),
        ("dependency_link_a", 4),
        ("dependency_link_b", 3),
        ("dependency_body", 4),
    ]


def read_value_dependency(data, strings):
    dependency, roles = read_dependency(data, strings)
    class_version, class_version_branch = read_bs(data)
    cached_value, value_roles = read_eval_variant(data, strings)
    roles.extend(value_roles)
    return {
        "dependency": dependency,
        "class_version": class_version,
        "class_version_branch": class_version_branch,
        "cached_value": cached_value,
        "value_name": read_tu(strings),
    }, roles


def read_eval_variant(data, strings):
    encoded_code, code_branch = read_bs(data)
    code = signed(encoded_code, 16)
    roles = []
    if 40 <= code <= 59:
        value, value_branch = read_bd(data)
        kind = "real"
    elif 90 <= code <= 99:
        value, value_branch = read_bl(data)
        kind = "integer32"
    elif 70 <= code <= 79:
        value, value_branch = read_bs(data)
        kind = "integer16"
    elif 280 <= code <= 289:
        value, value_branch = data.byte(), "RC"
        kind = "integer8"
    elif code == 0:
        value, value_branch = None, "none"
        kind = "none"
    elif code in range(1, 10) or code in range(100, 110) or code in range(300, 310):
        value, value_branch = read_tu(strings), "T"
        kind = "text"
    elif 330 <= code <= 369:
        value, value_branch = None, "H"
        kind = "object_reference"
        roles.append(("evaluated_value", 5))
    else:
        raise ValueError(f"unsupported evaluation variant code {code}")
    return {
        "code": code,
        "code_branch": code_branch,
        "kind": kind,
        "value": value,
        "value_branch": value_branch,
    }, roles


def read_assoc_variable(data, strings):
    action = {}
    action["class_version"], action["class_version_branch"] = read_bs(data)
    action["status"], action["status_branch"] = read_bl(data)
    roles = [("owning_network", 4), ("action_body", 3)]
    action["action_index"], action["action_index_branch"] = read_bl(data)
    action["maximum_dependency_index"], action["maximum_dependency_index_branch"] = read_bl(data)
    dependency_count, action["dependency_count_branch"] = read_bl(data)
    action["dependencies"] = []
    for index in range(dependency_count):
        owned = bool(data.bit())
        action["dependencies"].append({"owned": owned})
        roles.append((f"dependency[{index}]", 3 if owned else 4))
    if action["class_version"] > 1:
        raise ValueError("R2013 action extension is outside the AC1024 fixture probe")
    body = {"action": action}
    body["class_version"], body["class_version_branch"] = read_bl(data)
    body["name"] = read_tu(strings)
    body["expression"] = read_tu(strings)
    body["evaluator_id"] = read_tu(strings)
    body["description"] = read_tu(strings)
    body["value"], value_roles = read_eval_variant(data, strings)
    roles.extend(value_roles)
    body["mergeable"] = bool(data.bit())
    body["mergeable_variable_name"] = read_tu(strings) if body["mergeable"] else ""
    body["must_merge"] = bool(data.bit())
    if action["maximum_dependency_index"] > 0:
        body["referenced_value_dependency_count"], body["referenced_value_dependency_count_branch"] = read_bl(data)
        roles.extend((f"referenced_value_dependency[{index}]", 3) for index in range(body["referenced_value_dependency_count"]))
    else:
        body["referenced_value_dependency_count"] = 0
        body["referenced_value_dependency_count_branch"] = "absent"
    body["reference_binding_version"], body["reference_binding_version_branch"] = read_bs(data)
    return body, roles


def read_dependency_body(data, strings):
    body = {}
    body["dependency_body_version"], body["dependency_body_version_branch"] = read_bs(data)
    body["dimension_base_version"], body["dimension_base_version_branch"] = read_bs(data)
    body["name"] = read_tu(strings)
    body["class_version"], body["class_version_branch"] = read_bs(data)
    return body, []


def read_thin(data, strings, object_type):
    if object_type in (522, 559):
        flag, branch = read_bs(data)
        return {"flag": flag, "flag_branch": branch}, [
            ("protected_block" if object_type == 522 else "represented_block", 5 if object_type == 522 else 3)
        ]
    if object_type in (543, 549):
        return read_dependency_body(data, strings)
    if object_type == 547:
        expression, roles = read_eval_expression(data, strings)
        return {"expression": expression}, roles
    raise ValueError(object_type)


fixture = FIXTURE.read_bytes()
objects = b"".join(decode_page(fixture, page) for page in OBJECT_PAGES)
handles_page = decode_page(fixture, HANDLES_PAGE)
entries = handle_map(handles_page)
counts = {522: 2, 541: 23, 543: 6, 545: 18, 547: 1, 549: 12, 559: 12}
cohorts = {object_type: [] for object_type in counts}

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
    if object_type == 541:
        body, class_roles = read_value_dependency(data, strings)
    elif object_type == 545:
        body, class_roles = read_assoc_variable(data, strings)
    else:
        body, class_roles = read_thin(data, strings, object_type)
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


for object_type, expected in counts.items():
    assert len(cohorts[object_type]) == expected, (object_type, len(cohorts[object_type]), expected)

dependency_paths = [
    ("dependency", "class_version"), ("dependency", "class_version_branch"),
    ("dependency", "status"), ("dependency", "status_branch"),
    ("dependency", "read"), ("dependency", "write"), ("dependency", "attached"),
    ("dependency", "delegating"), ("dependency", "order"), ("dependency", "order_branch"),
    ("dependency", "has_name"), ("dependency", "name"),
    ("dependency", "body_id"), ("dependency", "body_id_branch"),
    ("class_version",), ("class_version_branch",),
    ("cached_value", "code"), ("cached_value", "code_branch"),
    ("cached_value", "kind"), ("cached_value", "value"),
    ("cached_value", "value_branch"), ("value_name",),
]
print_cohort(541, "ACDBASSOCVALUEDEPENDENCY", dependency_paths)
print_cohort(545, "ACDBASSOCVARIABLE", [
    ("action", "class_version"), ("action", "class_version_branch"),
    ("action", "status"), ("action", "status_branch"),
    ("action", "action_index"), ("action", "action_index_branch"),
    ("action", "maximum_dependency_index"), ("action", "maximum_dependency_index_branch"),
    ("action", "dependencies"), ("class_version",), ("class_version_branch",),
    ("name",), ("expression",), ("evaluator_id",), ("description",),
    ("value", "code"), ("value", "code_branch"), ("value", "kind"),
    ("value", "value"), ("value", "value_branch"),
    ("mergeable",), ("mergeable_variable_name",), ("must_merge",),
    ("referenced_value_dependency_count",), ("referenced_value_dependency_count_branch",),
    ("reference_binding_version",), ("reference_binding_version_branch",),
])
print_cohort(522, "ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION", [("flag",), ("flag_branch",)])
print_cohort(543, "BLOCKPARAMDEPENDENCYBODY", [
    ("dependency_body_version",), ("dependency_body_version_branch",),
    ("dimension_base_version",), ("dimension_base_version_branch",),
    ("name",), ("class_version",), ("class_version_branch",),
])
print_cohort(547, "ACDB_DYNAMICBLOCKPROXYNODE", [
    ("expression", "parent_id"), ("expression", "parent_id_branch"),
    ("expression", "major"), ("expression", "major_branch"),
    ("expression", "minor"), ("expression", "minor_branch"),
    ("expression", "value_code"), ("expression", "value_code_branch"),
    ("expression", "value"), ("expression", "value_branch"),
    ("expression", "node_id"), ("expression", "node_id_branch"),
])
print_cohort(549, "ASSOCDIMDEPENDENCYBODY", [
    ("dependency_body_version",), ("dependency_body_version_branch",),
    ("dimension_base_version",), ("dimension_base_version_branch",),
    ("name",), ("class_version",), ("class_version_branch",),
])
print_cohort(559, "ACDB_BLOCKREPRESENTATION_DATA", [("flag",), ("flag_branch",)])
