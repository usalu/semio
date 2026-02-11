#!/usr/bin/env python
# region Header

# 📜semio/assets/grasshopper/build.py

# 2025 Ueli Saluz <ueli@semio-tech.com>

# region License

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.

# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

# endregion License

# region Specs
# endregion Specs

# endregion Header

import json
import xml.etree.ElementTree as ET

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

    props["kind"] = props.get("name", "unknown")
    return props

def is_numeric(s):
    try:
        float(s)
        return True
    except (TypeError, ValueError):
        return False

def get_pivot_y(container_chunk):
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
    return float("inf")

def parse_components_and_groups_xml(xml_file_path):
    tree = ET.parse(xml_file_path)
    root = tree.getroot()

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
                        component_props["icon"] = "icon"

                pivot_y = get_pivot_y(container_chunk)

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

            if "icon" not in component_props:
                component_props["icon"] = ""
            if instance_guid and component_props.get("name"):
                components_by_guid[instance_guid] = (component_props, pivot_y)
                all_components.append((component_props, pivot_y))

    groups = {}
    if definition_objects_chunk is not None:
        for obj_chunk in definition_objects_chunk.findall(
            "./chunks/chunk[@name='Object']"
        ):
            name = None
            group_nickname = None
            container_chunk = obj_chunk.find("./chunks/chunk[@name='Container']")
            if container_chunk is not None:
                for item in container_chunk.findall("./items/item"):
                    if item.get("name") == "Name":
                        name = item.text
                    if item.get("name") == "NickName":
                        group_nickname = item.text

                if (
                    name == "Group"
                    and group_nickname
                    and not is_numeric(group_nickname)
                ):
                    group_name = group_nickname

                    group_ids = []
                    for item in container_chunk.findall("./items/item"):
                        if item.get("name") == "ID":
                            group_ids.append(item.text)

                    subgroups = {}
                    for guid in group_ids:
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
                        if subgroup_obj is not None:
                            sub_container = subgroup_obj.find(
                                "./chunks/chunk[@name='Container']"
                            )
                            exposure_index = None
                            for sub_item in sub_container.findall("./items/item"):
                                if sub_item.get("name") == "NickName":
                                    exposure_index = sub_item.text

                            subgroup_ids = []
                            for sub_item in sub_container.findall("./items/item"):
                                if sub_item.get("name") == "ID":
                                    subgroup_ids.append(sub_item.text)
                            subgroup_components = []
                            for sub_guid in subgroup_ids:
                                if sub_guid in components_by_guid:
                                    comp, pivot_y = components_by_guid[sub_guid]
                                    subgroup_components.append((comp, pivot_y))

                            subgroup_components.sort(
                                key=lambda c: c[1] if c[1] is not None else float("inf")
                            )

                            subgroup_components = [c[0] for c in subgroup_components]
                            if exposure_index is not None:
                                subgroups[exposure_index] = subgroup_components
                        else:
                            if guid in components_by_guid:
                                comp, pivot_y = components_by_guid[guid]

                                if "1" not in subgroups:
                                    subgroups["1"] = []
                                subgroups["1"].append((comp, pivot_y))

                    for k in subgroups:
                        if subgroups[k] and isinstance(subgroups[k][0], tuple):
                            subgroups[k].sort(
                                key=lambda c: c[1] if c[1] is not None else float("inf")
                            )

                            subgroups[k] = [c[0] for c in subgroups[k]]
                    if group_name:
                        groups[group_name] = subgroups
    return groups

definition = "components"
xml_file = f"assets/grasshopper/{definition}.ghx"
extracted_data = parse_components_and_groups_xml(xml_file)

json_output = json.dumps(extracted_data, indent=4)
with open(f"assets/grasshopper/{definition}.json", "w") as f:
    json.dump(extracted_data, f, indent=4)
