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
common_main = SHARED["common_main"]
common_handles = SHARED["common_handles"]
read_string_stream = SHARED["read_string_stream"]
read_tu = SHARED["read_tu"]


fixture = FIXTURE.read_bytes()
objects = b"".join(decode_page(fixture, page) for page in OBJECT_PAGES)
handles = decode_page(fixture, HANDLES_PAGE)
entries = handle_map(handles)
cohorts = {4: [], 5: []}

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
    stored_crc = int.from_bytes(
        objects[address + prefix_bytes + payload_size:address + prefix_bytes + payload_size + 2],
        "little",
    )
    assert crc16(objects[address:address + prefix_bytes + payload_size]) == stored_crc
    eed = skip_eed(reader)
    common = common_main(reader)
    handle_start = payload_size * 8 - handle_bits
    strings, string_start, string_bits, strings_present = read_string_stream(payload, handle_start)
    body = {"name": read_tu(strings)} if object_type == 4 else {}
    class_end = reader.position
    assert class_end == string_start
    assert strings.position == string_start + string_bits
    handle_reader, roles = common_handles(payload, handle_start, map_handle, common)
    handle_tail = len(payload) * 8 - handle_reader.position
    handle_tail_pattern = "".join(str(handle_reader.bit()) for _ in range(handle_tail))
    cohorts[object_type].append({
        "handle": map_handle,
        "address": address,
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


def print_cohort(object_type, name):
    frames = cohorts[object_type]
    print(f"{name}_frames={len(frames)}")
    for field in [
        "prefix_bytes", "payload_size", "frame_size", "handle_bits", "data_bits", "class_end",
        "bot_selector", "object_handle_code", "eed", "strings_present", "string_bits", "handle_tail",
        "handle_tail_pattern", "stored_crc",
    ]:
        print(f"{field}={histogram(frames, field)}")
    for field in [
        "graphic", "entmode", "reactors", "reactor_bl", "xdic", "color_index", "color_bs",
        "ltype_scale", "ltype_scale_bd", "ltype", "plotstyle", "material", "shadow", "visual_full",
        "visual_face", "visual_edge", "invisibility", "invisibility_bs", "lineweight",
    ]:
        print(f"common.{field}={histogram(frames, 'common', field)}")
    if object_type == 4:
        print(f"body.name={histogram(frames, 'body', 'name')}")
    print(f"handle_layout={Counter(tuple(role for role, _, _ in frame['roles']) for frame in frames)}")
    print(f"handle_codes={Counter(tuple(code for _, code, _ in frame['roles']) for frame in frames)}")
    print(f"owner_handles={Counter(next((target for role, _, target in frame['roles'] if role == 'owner'), None) for frame in frames)}")
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
    print("frame_rows=handle,name,entmode,owner,layer,payload,total,handle_bits,data_bits,class_end,string_bits,tail,crc")
    for frame in frames:
        targets = {role: target for role, _, target in frame["roles"]}
        print((
            hex(frame["handle"]), frame["body"].get("name"), frame["common"]["entmode"],
            hex(targets["owner"]) if "owner" in targets else None,
            hex(targets["layer"]), frame["payload_size"], frame["frame_size"],
            frame["handle_bits"], frame["data_bits"], frame["class_end"], frame["string_bits"],
            frame["handle_tail_pattern"], hex(frame["stored_crc"]),
        ))


assert len(cohorts[4]) == 10
assert len(cohorts[5]) == 10
def semantic_owner(frame):
    owner = next((target for role, _, target in frame["roles"] if role == "owner"), None)
    return ("handle", owner) if owner is not None else ("space", frame["common"]["entmode"])


block_owners = {semantic_owner(frame) for frame in cohorts[4]}
endblk_owners = {semantic_owner(frame) for frame in cohorts[5]}
assert len(block_owners) == 10
assert block_owners == endblk_owners
print_cohort(4, "BLOCK")
print_cohort(5, "ENDBLK")
print(f"paired_owners={sorted(block_owners, key=str)}")
