from collections import Counter, defaultdict
from contextlib import redirect_stdout
from pathlib import Path
import io
import runpy


TICKET = Path(__file__).parent
with redirect_stdout(io.StringIO()):
    SHARED = runpy.run_path(TICKET / "🧪️dwg-custom-541-545-thin-frame-probe.py")
    MAP = runpy.run_path(TICKET / "🧪️dwg-object-handles-reconstruction-probe.py")

Bits = SHARED["Bits"]
objects = SHARED["objects"]
frame_prefix = SHARED["frame_prefix"]
bot = SHARED["bot"]
skip_eed = SHARED["skip_eed"]
crc16 = SHARED["crc16"]
resolve = SHARED["resolve"]
read_string_stream = SHARED["read_string_stream"]
read_tu = SHARED["read_tu"]
read_common_object = SHARED["read_common_object"]
read_eval_expression = SHARED["read_eval_expression"]
read_bl = SHARED["read_bl"]
read_bs = SHARED["read_bs"]
read_bd = SHARED["read_bd"]
signed = SHARED["signed"]
freeze = SHARED["freeze"]
entries = MAP["entries"]

NAMES = {
    504: "TABLESTYLE",
    517: "ACAD_EVALUATION_GRAPH",
    529: "BLOCKFLIPPARAMETER",
    531: "BLOCKVISIBILITYPARAMETER",
    540: "ACDBASSOC2DCONSTRAINTGROUP",
}


def read_3bd(data):
    values = [read_bd(data) for _ in range(3)]
    return tuple(value for value, _ in values), tuple(branch for _, branch in values)


def read_connection_group(data, strings):
    count, count_branch = read_bl(data)
    values = []
    for _ in range(count):
        code, code_branch = read_bl(data)
        values.append({"code": code, "code_branch": code_branch, "name": read_tu(strings)})
    return values, count_branch


def read_block_element(data, strings):
    expression, roles = read_eval_expression(data, strings)
    name = read_tu(strings)
    major, major_branch = read_bl(data)
    minor, minor_branch = read_bl(data)
    class_marker, class_marker_branch = read_bl(data)
    return {
        "expression": expression,
        "name": name,
        "format_major": major,
        "format_major_branch": major_branch,
        "format_minor": minor,
        "format_minor_branch": minor_branch,
        "class_marker": class_marker,
        "class_marker_branch": class_marker_branch,
    }, roles


def read_block_parameter(data, strings):
    element, roles = read_block_element(data, strings)
    return {
        "element": element,
        "show_properties": bool(data.bit()),
        "chain_actions": bool(data.bit()),
    }, roles


def read_block_2pt_parameter(data, strings):
    parameter, roles = read_block_parameter(data, strings)
    base, base_branches = read_3bd(data)
    end, end_branches = read_3bd(data)
    properties = []
    for _ in range(4):
        values, count_branch = read_connection_group(data, strings)
        properties.append({"connections": values, "count_branch": count_branch})
    property_states = []
    property_state_branches = []
    for _ in range(4):
        value, branch = read_bl(data)
        property_states.append(value)
        property_state_branches.append(branch)
    base_location, base_location_branch = read_bs(data)
    return {
        "parameter": parameter,
        "definition_base": base,
        "definition_base_branches": base_branches,
        "definition_end": end,
        "definition_end_branches": end_branches,
        "properties": properties,
        "property_states": tuple(property_states),
        "property_state_branches": tuple(property_state_branches),
        "base_location": base_location,
        "base_location_branch": base_location_branch,
    }, roles


def read_block_1pt_parameter(data, strings):
    parameter, roles = read_block_parameter(data, strings)
    point, point_branches = read_3bd(data)
    properties = []
    for _ in range(2):
        values, count_branch = read_connection_group(data, strings)
        properties.append({"connections": values, "count_branch": count_branch})
    property_count, property_count_branch = read_bl(data)
    return {
        "parameter": parameter,
        "definition_point": point,
        "definition_point_branches": point_branches,
        "properties": properties,
        "property_count": property_count,
        "property_count_branch": property_count_branch,
    }, roles


def read_flip_parameter(data, strings):
    base, roles = read_block_2pt_parameter(data, strings)
    label_point, label_point_branches = read_3bd(data)
    state_index, state_index_branch = read_bl(data)
    return {
        "base": base,
        "flip_label": read_tu(strings),
        "description": read_tu(strings),
        "base_state_label": read_tu(strings),
        "flipped_state_label": read_tu(strings),
        "label_point": label_point,
        "label_point_branches": label_point_branches,
        "current_state_index": state_index,
        "current_state_index_branch": state_index_branch,
        "tooltip": read_tu(strings),
    }, roles


def read_visibility_parameter(data, strings):
    base, roles = read_block_1pt_parameter(data, strings)
    initialized = bool(data.bit())
    name = read_tu(strings)
    description = read_tu(strings)
    history_required = bool(data.bit())
    block_count, block_count_branch = read_bl(data)
    roles.extend((f"eligible_block[{index}]", 4) for index in range(block_count))
    state_count, state_count_branch = read_bl(data)
    states = []
    for state_index in range(state_count):
        state_name = read_tu(strings)
        visible_count, visible_count_branch = read_bl(data)
        roles.extend((f"state[{state_index}].visible_block[{index}]", 4) for index in range(visible_count))
        parameter_count, parameter_count_branch = read_bl(data)
        roles.extend((f"state[{state_index}].controlled_parameter[{index}]", 4) for index in range(parameter_count))
        states.append({
            "name": state_name,
            "visible_block_count": visible_count,
            "visible_block_count_branch": visible_count_branch,
            "controlled_parameter_count": parameter_count,
            "controlled_parameter_count_branch": parameter_count_branch,
        })
    return {
        "base": base,
        "initialized": initialized,
        "name": name,
        "description": description,
        "history_required": history_required,
        "eligible_block_count": block_count,
        "eligible_block_count_branch": block_count_branch,
        "states": states,
        "state_count_branch": state_count_branch,
    }, roles


def read_assoc_action(data, strings):
    class_version, class_version_branch = read_bs(data)
    status, status_branch = read_bl(data)
    action_index, action_index_branch = read_bl(data)
    maximum_dependency_index, maximum_dependency_index_branch = read_bl(data)
    dependency_count, dependency_count_branch = read_bl(data)
    dependencies = []
    roles = [("owning_network", 4), ("action_body", 3)]
    for index in range(dependency_count):
        owned = bool(data.bit())
        dependencies.append(owned)
        roles.append((f"dependency[{index}]", 3 if owned else 4))
    if class_version > 1:
        raise ValueError("R2013 action extension in AC1024 fixture")
    return {
        "class_version": class_version,
        "class_version_branch": class_version_branch,
        "status": status,
        "status_branch": status_branch,
        "action_index": action_index,
        "action_index_branch": action_index_branch,
        "maximum_dependency_index": maximum_dependency_index,
        "maximum_dependency_index_branch": maximum_dependency_index_branch,
        "dependencies": tuple(dependencies),
        "dependency_count_branch": dependency_count_branch,
    }, roles


def read_constraint_group(data, strings):
    action, roles = read_assoc_action(data, strings)
    version, version_branch = read_bl(data)
    solution_initialized = bool(data.bit())
    work_plane = []
    work_plane_branches = []
    for _ in range(3):
        point, branches = read_3bd(data)
        work_plane.append(point)
        work_plane_branches.append(branches)
    roles.append(("dimension_dependency", 4))
    action_count, action_count_branch = read_bl(data)
    roles.extend((f"member_action[{index}]", 4) for index in range(action_count))
    node_count, node_count_branch = read_bl(data)
    nodes = []
    for node_index in range(node_count):
        node_kind = read_tu(strings)
        encoded_id, id_branch = read_bl(data)
        connection_count, connection_count_branch = read_bl(data)
        connections = []
        connection_branches = []
        for connection_index in range(connection_count):
            try:
                value, branch = read_bl(data)
            except ValueError as error:
                raise ValueError(
                    f"constraint node {node_index} id {signed(encoded_id, 32)} "
                    f"count {connection_count} connection {connection_index} at bit {data.position}; "
                    f"connections={connections}; "
                    f"version={version} bool={solution_initialized} work={work_plane} "
                    f"actions={action_count} nodes={node_count} action={action}"
                ) from error
            connections.append(value)
            connection_branches.append(branch)
        nodes.append({
            "kind": node_kind,
            "id": signed(encoded_id, 32),
            "id_branch": id_branch,
            "connections": tuple(connections),
            "connection_count_branch": connection_count_branch,
            "connection_branches": tuple(connection_branches),
        })
    return {
        "action": action,
        "version": version,
        "version_branch": version_branch,
        "solution_initialized": solution_initialized,
        "work_plane": tuple(work_plane),
        "work_plane_branches": tuple(work_plane_branches),
        "member_action_count": action_count,
        "member_action_count_branch": action_count_branch,
        "nodes": nodes,
        "node_count_branch": node_count_branch,
    }, roles


def read_evaluation_graph(data, strings):
    encoded_root, root_branch = read_bl(data)
    encoded_copy, copy_branch = read_bl(data)
    node_count, node_count_branch = read_bl(data)
    nodes = []
    roles = []
    for index in range(node_count):
        node_id, node_id_branch = read_bl(data)
        edge_flags, edge_flags_branch = read_bl(data)
        encoded_next, next_branch = read_bl(data)
        relations = []
        relation_branches = []
        for _ in range(4):
            value, branch = read_bl(data)
            relations.append(signed(value, 32))
            relation_branches.append(branch)
        nodes.append({
            "id": node_id,
            "id_branch": node_id_branch,
            "edge_flags": edge_flags,
            "edge_flags_branch": edge_flags_branch,
            "next_id": signed(encoded_next, 32),
            "next_id_branch": next_branch,
            "relations": tuple(relations),
            "relation_branches": tuple(relation_branches),
        })
        roles.append((f"node[{index}].expression", 5))
    edge_count, edge_count_branch = read_bl(data)
    edges = []
    for _ in range(edge_count):
        values = []
        branches = []
        for _ in range(10):
            value, branch = read_bl(data)
            values.append(value)
            branches.append(branch)
        edges.append({
            "id": values[0],
            "next_id": signed(values[1], 32),
            "relations": tuple(signed(value, 32) for value in values[2:5]),
            "outgoing": tuple(signed(value, 32) for value in values[5:]),
            "branches": tuple(branches),
        })
    return {
        "root_node_id": signed(encoded_root, 32),
        "root_node_id_branch": root_branch,
        "root_copy": signed(encoded_copy, 32),
        "root_copy_branch": copy_branch,
        "nodes": nodes,
        "node_count_branch": node_count_branch,
        "edges": edges,
        "edge_count_branch": edge_count_branch,
    }, roles


READERS = {
    517: read_evaluation_graph,
    529: read_flip_parameter,
    531: read_visibility_parameter,
    540: read_constraint_group,
}

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
    object_type = bot(data)
    object_handle_code, object_handle = data.handle()
    assert object_handle == handle
    eed = tuple(skip_eed(data))
    handle_start = payload_size * 8 - handle_bits
    strings, string_start, string_bits, strings_present = read_string_stream(payload, handle_start)
    handles = Bits(payload, handle_start)
    common, resolved_roles = read_common_object(data, handles, handle)
    body, class_roles = READERS[object_type](data, strings)
    for role, expected_code in class_roles:
        code, value = handles.handle()
        resolved_roles.append((role, code, resolve(handle, (code, value))))
    assert data.position == string_start, (object_type, hex(handle), data.position, string_start)
    assert strings.position == string_start + string_bits
    tail = len(payload) * 8 - handles.position
    tail_pattern = "".join(str(handles.bit()) for _ in range(tail))
    frame_end = address + prefix_bytes + payload_size + 2
    stored_crc = int.from_bytes(objects[frame_end - 2:frame_end], "little")
    assert stored_crc == crc16(objects[address:frame_end - 2])
    return {
        "handle": handle,
        "type": object_type,
        "payload_size": payload_size,
        "frame_size": prefix_bytes + payload_size + 2,
        "handle_bits": handle_bits,
        "data_bits": handle_start,
        "class_end": data.position,
        "object_handle_code": object_handle_code,
        "eed": eed,
        "common": common,
        "body": body,
        "strings_present": strings_present,
        "string_bits": string_bits,
        "roles": tuple(resolved_roles),
        "tail_pattern": tail_pattern,
        "crc": stored_crc,
    }


def inspect_constraint_prefix(handle):
    address, payload_size, handle_bits, prefix_bytes, payload = frame_by_handle[handle]
    data = Bits(payload)
    assert bot(data) == 540
    _, object_handle = data.handle()
    assert object_handle == handle
    skip_eed(data)
    handle_start = payload_size * 8 - handle_bits
    strings, string_start, string_bits, strings_present = read_string_stream(payload, handle_start)
    handles = Bits(payload, handle_start)
    common, resolved_roles = read_common_object(data, handles, handle)
    action, class_roles = read_assoc_action(data, strings)
    version, version_branch = read_bl(data)
    do_not_check_new_constraints = bool(data.bit())
    work_plane = tuple(read_3bd(data)[0] for _ in range(3))
    class_roles.append(("dimension_dependency", 4))
    action_count, action_count_branch = read_bl(data)
    class_roles.extend((f"member_action[{index}]", 4) for index in range(action_count))
    node_count, node_count_branch = read_bl(data)
    node_class_reader = Bits(payload, strings.position)
    node_classes = []
    while node_class_reader.position < string_start + string_bits:
        node_classes.append(read_tu(node_class_reader))
    for role, expected_code in class_roles:
        code, value = handles.handle()
        resolved_roles.append((role, code, resolve(handle, (code, value))))
    frame_end = address + prefix_bytes + payload_size + 2
    stored_crc = int.from_bytes(objects[frame_end - 2:frame_end], "little")
    assert stored_crc == crc16(objects[address:frame_end - 2])
    return {
        "handle": handle,
        "payload_size": payload_size,
        "frame_size": prefix_bytes + payload_size + 2,
        "handle_bits": handle_bits,
        "data_bits": handle_start,
        "string_bits": string_bits,
        "strings_present": strings_present,
        "common": common,
        "action": action,
        "version": version,
        "version_branch": version_branch,
        "do_not_check_new_constraints": do_not_check_new_constraints,
        "work_plane": work_plane,
        "action_count_branch": action_count_branch,
        "node_count": node_count,
        "node_count_branch": node_count_branch,
        "node_classes": tuple(node_classes),
        "node_data_start": data.position,
        "string_start": string_start,
        "roles": tuple(resolved_roles),
        "crc": stored_crc,
    }


def inspect_constraint_full(handle):
    address, payload_size, handle_bits, prefix_bytes, payload = frame_by_handle[handle]
    data = Bits(payload)
    assert bot(data) == 540
    _, object_handle = data.handle()
    assert object_handle == handle
    skip_eed(data)
    handle_start = payload_size * 8 - handle_bits
    strings, string_start, string_bits, _ = read_string_stream(payload, handle_start)
    handles = Bits(payload, handle_start)
    common, roles = read_common_object(data, handles, handle)
    action, action_roles = read_assoc_action(data, strings)
    version, _ = read_bl(data)
    policy = bool(data.bit())
    work_plane = tuple(read_3bd(data)[0] for _ in range(3))
    outer_roles = action_roles + [("dimension_dependency", 3)]
    member_count, _ = read_bl(data)
    outer_roles.extend((f"member[{index}]", 3) for index in range(member_count))
    watermark, _ = read_bl(data)
    node_count, _ = read_bl(data)
    class_reader = Bits(payload, strings.position)
    classes = []
    while class_reader.position < string_start + string_bits:
        classes.append(read_tu(class_reader))
    for role, expected in outer_roles:
        code, value = handles.handle()
        roles.append((role, code, resolve(handle, (code, value))))

    def node_core():
        start = data.position
        node_id, _ = read_bl(data)
        count, _ = read_bl(data)
        connections = []
        for connection_index in range(count):
            connections.append(read_bl(data)[0])
        connections = tuple(connections)
        return {"id": signed(node_id, 32), "connections": connections}

    def geometric():
        node = node_core()
        owner, _ = read_bl(data)
        return {"node": node, "owner": owner, "implied": bool(data.bit()), "active": True}

    def geometry(index):
        node = node_core()
        code, value = handles.handle()
        target = resolve(handle, (code, value))
        roles.append((f"node[{index}].geometry", code, target))
        geometry_node, _ = read_bl(data)
        return {"node": node, "dependency": target or None, "geometry_node": geometry_node}

    nodes = []
    assert node_count == len(classes), (node_count, len(classes))
    for index, kind in enumerate(classes):
        print(f"[DEBUG] constraint-node index={index} kind={kind} data={data.position} handles={handles.position}")
        if kind == "AcConstrainedImplicitPoint":
            base = geometry(index)
            point = read_3bd(data)[0] if base["dependency"] else None
            body = {"geometry": base, "point": point, "point_kind": data.byte(), "point_index": signed(read_bl(data)[0], 32), "curve_id": signed(read_bl(data)[0], 32)}
        elif kind == "AcConstrainedBoundedLine":
            base = geometry(index)
            origin = read_3bd(data)[0]
            direction = read_3bd(data)[0]
            ray = bool(data.bit())
            body = {
                "geometry": base,
                "origin": origin,
                "direction": direction,
                "ray": ray,
                "start": read_3bd(data)[0],
                "end": read_3bd(data)[0],
            }
        elif kind == "AcConstrainedDatumLine":
            base = geometry(index)
            body = {"geometry": base, "origin": read_3bd(data)[0], "direction": read_3bd(data)[0]}
        elif kind == "AcDistanceConstraint":
            base = geometric()
            dependency_roles = []
            for role in ("value", "dimension"):
                code, value = handles.handle()
                target = resolve(handle, (code, value))
                roles.append((f"node[{index}].{role}", code, target))
                dependency_roles.append(target)
            direction_kind = data.byte()
            body = {"geometric": base, "value": dependency_roles[0], "dimension": dependency_roles[1], "direction_kind": direction_kind, "direction": read_3bd(data)[0] if direction_kind else None}
        elif kind in ("AcHorizontalConstraint", "AcVerticalConstraint"):
            body = {"geometric": geometric(), "datum": signed(read_bl(data)[0], 32)}
        else:
            body = geometric()
        nodes.append((kind, body))
    tail = len(payload) * 8 - handles.position
    tail_pattern = "".join(str(handles.bit()) for _ in range(tail))
    return {"handle": handle, "data_end": data.position, "expected_data_end": string_start, "string_end": class_reader.position, "expected_string_end": string_start + string_bits, "watermark": watermark, "version": version, "policy": policy, "work_plane": work_plane, "action": action, "nodes": nodes, "roles": roles, "tail": tail_pattern}


def inspect_table_style_prefix(handle):
    address, payload_size, handle_bits, prefix_bytes, payload = frame_by_handle[handle]
    data = Bits(payload)
    assert bot(data) == 504
    _, object_handle = data.handle()
    assert object_handle == handle
    skip_eed(data)
    handle_start = payload_size * 8 - handle_bits
    strings, string_start, string_bits, strings_present = read_string_stream(payload, handle_start)
    handles = Bits(payload, handle_start)
    common, roles = read_common_object(data, handles, handle)
    format_discriminator = data.byte()
    description = read_tu(strings)
    format_version, format_version_branch = read_bl(data)
    style_flags, style_flags_branch = read_bl(data)
    code, value = handles.handle()
    roles.append(("template_style", code, resolve(handle, (code, value))))
    frame_end = address + prefix_bytes + payload_size + 2
    stored_crc = int.from_bytes(objects[frame_end - 2:frame_end], "little")
    assert stored_crc == crc16(objects[address:frame_end - 2])
    return {
        "payload_size": payload_size,
        "frame_size": prefix_bytes + payload_size + 2,
        "handle_bits": handle_bits,
        "data_bits": handle_start,
        "string_bits": string_bits,
        "strings_present": strings_present,
        "common": common,
        "format_discriminator": format_discriminator,
        "description": description,
        "format_version": format_version,
        "format_version_branch": format_version_branch,
        "style_flags": style_flags,
        "style_flags_branch": style_flags_branch,
        "prefix_end": data.position,
        "string_start": string_start,
        "roles": tuple(roles),
        "crc": stored_crc,
    }


cohorts = defaultdict(list)
failures = []
for handle, _ in entries:
    object_type = type_by_handle[handle]
    if object_type in READERS:
        try:
            cohorts[object_type].append(parse(handle))
        except ValueError as error:
            failures.append((handle, object_type, str(error)))

for object_type, frames in sorted(cohorts.items()):
    print(f"cohort={NAMES[object_type]} type={object_type} count={len(frames)}")
    print("signatures=" + repr(Counter((frame["payload_size"], frame["frame_size"], frame["handle_bits"], frame["data_bits"], frame["class_end"], frame["string_bits"], frame["tail_pattern"]) for frame in frames)))
    print("crcs=" + " ".join(f"{frame['handle']:x}:{frame['crc']:04x}" for frame in frames))
    print("roles=" + repr(Counter(tuple((role, code, type_by_handle.get(target)) for role, code, target in frame["roles"]) for frame in frames)))
    for frame in frames:
        print(f"frame={frame['handle']:#x} body={freeze(frame['body'])}")
for handle, object_type, failure in failures:
    print(f"failure={handle:#x} type={object_type} class={NAMES[object_type]} diagnostic={failure}")
for handle, _ in entries:
    if type_by_handle[handle] == 540:
        prefix = inspect_constraint_prefix(handle)
        print(
            f"constraint_prefix={handle:#x} payload={prefix['payload_size']} frame={prefix['frame_size']} "
            f"handle_bits={prefix['handle_bits']} data_bits={prefix['data_bits']} strings={prefix['string_bits']} "
            f"version={prefix['version']} skip_check={prefix['do_not_check_new_constraints']} "
            f"nodes={prefix['node_count']} node_data={prefix['node_data_start']}..{prefix['string_start']} "
            f"crc={prefix['crc']:04x}"
        )
        print("constraint_classes=" + repr(Counter(prefix["node_classes"])))
        print(
            "constraint_roles="
            + repr(tuple((role, code, target, type_by_handle.get(target)) for role, code, target in prefix["roles"]))
        )
        try:
            full = inspect_constraint_full(handle)
            print("constraint_full=" + repr(freeze(full)))
        except Exception as error:
            print(f"[DEBUG] constraint_full_failure={handle:#x} {error!r}")
for handle, _ in entries:
    if type_by_handle[handle] == 504:
        prefix = inspect_table_style_prefix(handle)
        print("table_style_prefix=" + repr(prefix))
