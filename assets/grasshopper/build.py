import xml.etree.ElementTree as ET
import json


def extract_param_props(param_element):
    props = {}
    for item in param_element.findall("./items/item"):
        name_attr = item.get("name")
        if name_attr == "Name":
            props["name"] = item.text
        elif name_attr == "NickName":
            props["nickname"] = item.text
        elif name_attr == "Description":
            props["description"] = item.text
    # Infer kind from Name if not directly available, otherwise use a placeholder or specific logic
    props["kind"] = props.get("name", "unknown")  # Using name as kind for now
    return props


def is_numeric(s):
    try:
        float(s)
        return True
    except (TypeError, ValueError):
        return False


def get_pivot_y(container_chunk):
    # Extracts the Pivot Y value from the Container chunk, if present
    attributes_chunk = container_chunk.find("./chunks/chunk[@name='Attributes']")
    if attributes_chunk is not None:
        for item in attributes_chunk.findall("./items/item"):
            if item.get("name") == "Pivot":
                y_elem = item.find("Y")
                if y_elem is not None:
                    try:
                        return float(y_elem.text)
                    except (TypeError, ValueError):
                        pass
    return float('inf')  # If not found, sort to end


def parse_components_and_groups_xml(xml_file_path):
    tree = ET.parse(xml_file_path)
    root = tree.getroot()

    # First, collect all components and map by InstanceGuid
    components_by_guid = {}
    all_components = []
    definition_objects_chunk = root.find(
        "./chunks/chunk[@name='Definition']/chunks/chunk[@name='DefinitionObjects']"
    )
    if definition_objects_chunk is not None:
        for obj_chunk in definition_objects_chunk.findall(
            "./chunks/chunk[@name='Object']"
        ):
            component_props = {}
            instance_guid = None
            # Extract top-level component info
            for item in obj_chunk.findall("./items/item"):
                if item.get("name") == "Name":
                    component_props["name"] = item.text
            container_chunk = obj_chunk.find("./chunks/chunk[@name='Container']")
            pivot_y = None
            if container_chunk is not None:
                for item in container_chunk.findall("./items/item"):
                    name_attr = item.get("name")
                    if name_attr == "NickName":
                        component_props["nickname"] = item.text
                    elif name_attr == "Description":
                        component_props["description"] = item.text
                    elif name_attr == "InstanceGuid":
                        instance_guid = item.text
                    elif name_attr == "IconOverride":
                        # Try to extract icon as base64 or placeholder
                        component_props["icon"] = "icon"  # Placeholder, can be improved
                # Extract Pivot Y (for sorting only)
                pivot_y = get_pivot_y(container_chunk)
                # Extract Inputs
                inputs = []
                input_param_chunks_normal = container_chunk.findall(
                    "./chunks/chunk[@name='param_input']"
                )
                if input_param_chunks_normal:
                    for param_chunk in input_param_chunks_normal:
                        inputs.append(extract_param_props(param_chunk))
                else:
                    parameter_data_chunk = container_chunk.find(
                        "./chunks/chunk[@name='ParameterData']"
                    )
                    if parameter_data_chunk is not None:
                        for param_chunk in parameter_data_chunk.findall(
                            "./chunks/chunk[@name='InputParam']"
                        ):
                            inputs.append(extract_param_props(param_chunk))
                if inputs:
                    component_props["inputs"] = inputs
                # Extract Outputs
                outputs = []
                output_param_chunks_normal = container_chunk.findall(
                    "./chunks/chunk[@name='param_output']"
                )
                if output_param_chunks_normal:
                    for param_chunk in output_param_chunks_normal:
                        outputs.append(extract_param_props(param_chunk))
                else:
                    parameter_data_chunk = container_chunk.find(
                        "./chunks/chunk[@name='ParameterData']"
                    )
                    if parameter_data_chunk is not None:
                        for param_chunk in parameter_data_chunk.findall(
                            "./chunks/chunk[@name='OutputParam']"
                        ):
                            outputs.append(extract_param_props(param_chunk))
                if outputs:
                    component_props["outputs"] = outputs
            # Set icon to empty string if not set
            if "icon" not in component_props:
                component_props["icon"] = ""
            if instance_guid and component_props.get("name"):
                # Store pivot_y only in a parallel dict for sorting
                components_by_guid[instance_guid] = (component_props, pivot_y)
                all_components.append((component_props, pivot_y))

    # Now, collect all groups and subgroups
    groups = {}
    if definition_objects_chunk is not None:
        for obj_chunk in definition_objects_chunk.findall(
            "./chunks/chunk[@name='Object']"
        ):
            # Only process objects whose name is 'Group'
            name = None
            group_nickname = None
            container_chunk = obj_chunk.find("./chunks/chunk[@name='Container']")
            if container_chunk is not None:
                for item in container_chunk.findall("./items/item"):
                    if item.get("name") == "Name":
                        name = item.text
                    if item.get("name") == "NickName":
                        group_nickname = item.text
                # Only create top-level group if NickName is not numeric
                if (
                    name == "Group"
                    and group_nickname
                    and not is_numeric(group_nickname)
                ):
                    group_name = group_nickname
                    # Get all referenced component IDs
                    group_ids = []
                    for item in container_chunk.findall("./items/item"):
                        if item.get("name") == "ID":
                            group_ids.append(item.text)
                    # Check for subgroups: if there are subgroups, they are also 'Group' objects referenced by ID
                    subgroups = {}
                    for guid in group_ids:
                        # If the referenced object is a subgroup (another Group), process as subgroup
                        subgroup_obj = None
                        for sub_obj_chunk in definition_objects_chunk.findall(
                            "./chunks/chunk[@name='Object']"
                        ):
                            sub_container = sub_obj_chunk.find(
                                "./chunks/chunk[@name='Container']"
                            )
                            if sub_container is not None:
                                sub_name = None
                                sub_nickname = None
                                for sub_item in sub_container.findall("./items/item"):
                                    if (
                                        sub_item.get("name") == "InstanceGuid"
                                        and sub_item.text == guid
                                    ):
                                        for sub_name_item in sub_container.findall(
                                            "./items/item"
                                        ):
                                            if sub_name_item.get("name") == "Name":
                                                sub_name = sub_name_item.text
                                            if sub_name_item.get("name") == "NickName":
                                                sub_nickname = sub_name_item.text
                                        if (
                                            sub_name == "Group"
                                            and sub_nickname
                                            and is_numeric(sub_nickname)
                                        ):
                                            subgroup_obj = sub_obj_chunk
                        if subgroup_obj:
                            # Get exposure index from NickName
                            sub_container = subgroup_obj.find(
                                "./chunks/chunk[@name='Container']"
                            )
                            exposure_index = None
                            for sub_item in sub_container.findall("./items/item"):
                                if sub_item.get("name") == "NickName":
                                    exposure_index = sub_item.text
                            # Get all referenced component IDs for this subgroup
                            subgroup_ids = []
                            for sub_item in sub_container.findall("./items/item"):
                                if sub_item.get("name") == "ID":
                                    subgroup_ids.append(sub_item.text)
                            subgroup_components = []
                            for sub_guid in subgroup_ids:
                                if sub_guid in components_by_guid:
                                    comp, pivot_y = components_by_guid[sub_guid]
                                    subgroup_components.append((comp, pivot_y))
                            # Sort subgroup components by pivot_y
                            subgroup_components.sort(key=lambda c: c[1] if c[1] is not None else float('inf'))
                            # Remove pivot_y before adding to output
                            subgroup_components = [c[0] for c in subgroup_components]
                            if exposure_index is not None:
                                subgroups[exposure_index] = subgroup_components
                        else:
                            # If not a subgroup, treat as direct component
                            if guid in components_by_guid:
                                comp, pivot_y = components_by_guid[guid]
                                # Use exposure index '1' if not in a subgroup
                                if "1" not in subgroups:
                                    subgroups["1"] = []
                                subgroups["1"].append((comp, pivot_y))
                    # Sort direct subgroup components by pivot_y
                    for k in subgroups:
                        # Only sort if items are tuples (component, pivot_y)
                        if subgroups[k] and isinstance(subgroups[k][0], tuple):
                            subgroups[k].sort(key=lambda c: c[1] if c[1] is not None else float('inf'))
                            # Remove pivot_y before adding to output
                            subgroups[k] = [c[0] for c in subgroups[k]]
                    if group_name:
                        groups[group_name] = subgroups
    return groups


# group = "test"
definition = "components"
xml_file = f"assets/grasshopper/{definition}.ghx"
extracted_data = parse_components_and_groups_xml(xml_file)

json_output = json.dumps(extracted_data, indent=4)
with open(f"assets/grasshopper/{definition}.json", "w") as f:
    json.dump(extracted_data, f, indent=4)
