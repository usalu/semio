#!/usr/bin/env python3
"""🔀 Unions both sides of the remaining stash conflicts."""

import json
import os
import re
import subprocess
from collections import OrderedDict

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), *[".."] * 7))
REPORT = []


def show(path, stage):
    return subprocess.check_output(["git", "show", f":{stage}:{path}"], cwd=ROOT)


def parse_jsonc(raw):
    if isinstance(raw, bytes):
        raw = raw.decode("utf-8")
    out = []
    i = 0
    n = len(raw)
    while i < n:
        ch = raw[i]
        if ch == '"':
            out.append(ch)
            i += 1
            while i < n:
                out.append(raw[i])
                if raw[i] == "\\" and i + 1 < n:
                    out.append(raw[i + 1])
                    i += 2
                    continue
                if raw[i] == '"':
                    i += 1
                    break
                i += 1
            continue
        if ch == "/" and i + 1 < n and raw[i + 1] == "/":
            i += 2
            while i < n and raw[i] not in "\r\n":
                i += 1
            continue
        if ch == "/" and i + 1 < n and raw[i + 1] == "*":
            i += 2
            while i + 1 < n and not (raw[i] == "*" and raw[i + 1] == "/"):
                i += 1
            i += 2
            continue
        out.append(ch)
        i += 1
    text = "".join(out)
    text = re.sub(r",(\s*[\]}])", r"\1", text)
    return json.loads(text)


def dump_json(value):
    return json.dumps(value, ensure_ascii=False, indent=2) + "\n"


def nx_targets():
    names = subprocess.check_output(["git", "ls-files", "-z", "--", "*project.json"], cwd=ROOT)
    found = set()
    for raw in names.split(b"\0"):
        if not raw:
            continue
        path = os.path.join(ROOT, raw.decode())
        try:
            data = json.loads(open(path, encoding="utf-8").read())
        except (OSError, json.JSONDecodeError):
            continue
        name = data.get("name")
        if not name:
            continue
        for target in (data.get("targets") or {}):
            found.add(f"{name}:{target}")
    return found


TARGETS = None


def nx_run_exists(command):
    global TARGETS
    if TARGETS is None:
        TARGETS = nx_targets()
    match = re.search(r"\bbun nx run\s+(\S+)", command or "")
    if not match:
        return None
    spec = match.group(1)
    if ":" not in spec:
        return None
    project, target = spec.rsplit(":", 1)
    target = target.split("?", 1)[0]
    return f"{project}:{target}" in TARGETS


def same(left, right):
    return left == right


def union_list(base, ours, theirs):
    if all(isinstance(item, str) for item in [*base, *ours, *theirs]):
        merged = []
        for item in [*ours, *theirs]:
            if item not in merged:
                merged.append(item)
        return merged
    return None


def merge_value(base, ours, theirs, path):
    if same(ours, theirs):
        return ours
    if ours is None:
        return theirs
    if theirs is None:
        return ours
    if isinstance(ours, dict) and isinstance(theirs, dict):
        base = base if isinstance(base, dict) else {}
        merged = OrderedDict()
        for key in list(ours) + [key for key in theirs if key not in ours]:
            merged[key] = merge_value(base.get(key), ours.get(key), theirs.get(key), path + [key])
        return merged
    if isinstance(ours, list) and isinstance(theirs, list):
        base = base if isinstance(base, list) else []
        united = union_list(base, ours, theirs)
        if united is not None:
            return united
        if len(ours) == len(theirs):
            merged_list = []
            aligned = True
            for index, (left, right) in enumerate(zip(ours, theirs)):
                if left == right:
                    merged_list.append(left)
                elif isinstance(left, dict) and isinstance(right, dict):
                    base_item = base[index] if index < len(base) and isinstance(base[index], dict) else {}
                    merged_list.append(merge_value(base_item, left, right, path + [str(index)]))
                else:
                    aligned = False
                    break
            if aligned:
                return merged_list
    if same(ours, base):
        return theirs
    if same(theirs, base):
        return ours
    REPORT.append(f"HARD {'.'.join(str(part) for part in path)} ours={json.dumps(ours, ensure_ascii=False)[:180]} theirs={json.dumps(theirs, ensure_ascii=False)[:180]}")
    return ours


def choose_command(base_command, ours_command, theirs_command):
    if ours_command == theirs_command:
        return ours_command
    ours_live = nx_run_exists(ours_command)
    theirs_live = nx_run_exists(theirs_command)
    if ours_live is True and theirs_live is False:
        REPORT.append(f"command kept upstream live target: {ours_command}")
        return ours_command
    if theirs_live is True and ours_live is False:
        REPORT.append(f"command kept stashed live target: {theirs_command}")
        return theirs_command
    if ours_live is False and theirs_live is False:
        REPORT.append(f"command both targets missing, kept upstream: {ours_command} | {theirs_command}")
        return ours_command
    chosen = merge_value(base_command, ours_command, theirs_command, ["command"])
    return chosen


def entry_key(config):
    if isinstance(config, str):
        return ("str", config)
    if isinstance(config, dict) and config.get("name"):
        return ("name", config["name"])
    return ("raw", json.dumps(config, ensure_ascii=False, sort_keys=True))


def config_map(doc):
    mapped = OrderedDict()
    extras = []
    for config in doc.get("configurations") or []:
        key = entry_key(config)
        if key in mapped:
            extras.append(config)
            continue
        mapped[key] = config
    return mapped, extras


def merge_config(base, ours, theirs):
    merged = OrderedDict(ours)
    for key in list(theirs):
        if key == "presentation":
            presentation = OrderedDict(ours.get("presentation") or {})
            their_presentation = theirs.get("presentation") or {}
            base_presentation = (base or {}).get("presentation") or {}
            for field, value in their_presentation.items():
                if field == "order":
                    continue
                if field not in presentation:
                    presentation[field] = value
                elif presentation[field] != value:
                    presentation[field] = merge_value(base_presentation.get(field), presentation[field], value, ["presentation", field])
            if presentation:
                merged["presentation"] = presentation
            continue
        if key == "command":
            merged["command"] = choose_command((base or {}).get("command"), ours.get("command"), theirs.get("command"))
            continue
        if key == "env":
            env = OrderedDict(ours.get("env") or {})
            for field, value in (theirs.get("env") or {}).items():
                if field not in env:
                    env[field] = value
                elif env[field] != value:
                    base_env = ((base or {}).get("env") or {})
                    env[field] = merge_value(base_env.get(field), env[field], value, ["env", field])
            if env:
                merged["env"] = env
            continue
        if key not in merged:
            merged[key] = theirs[key]
        elif merged[key] != theirs[key]:
            merged[key] = merge_value((base or {}).get(key), merged[key], theirs[key], [key])
    return merged


def insert_missing(spine, foreign):
    result = list(spine)
    positions = {name: index for index, name in enumerate(result)}
    for index, name in enumerate(foreign):
        if name in positions:
            continue
        anchor = None
        for previous in reversed(foreign[:index]):
            if previous in positions:
                anchor = positions[previous]
                break
        if anchor is None:
            for following in foreign[index + 1 :]:
                if following in positions:
                    anchor = positions[following]
                    result.insert(anchor, name)
                    positions = {item: at for at, item in enumerate(result)}
                    break
            else:
                result.append(name)
                positions[name] = len(result) - 1
            continue
        result.insert(anchor + 1, name)
        positions = {item: at for at, item in enumerate(result)}
    return result


def merge_launch(path):
    base = parse_jsonc(show(path, 1))
    ours = parse_jsonc(show(path, 2))
    theirs = parse_jsonc(show(path, 3))
    base_map, _ = config_map(base)
    ours_map, ours_extras = config_map(ours)
    theirs_map, theirs_extras = config_map(theirs)
    names = insert_missing(list(ours_map), list(theirs_map))
    configurations = []
    for name in names:
        left = ours_map.get(name)
        right = theirs_map.get(name)
        if isinstance(left, str) or isinstance(right, str) or not isinstance(left or right, dict):
            configurations.append(left if left is not None else right)
        elif left is not None and right is not None:
            configurations.append(merge_config(base_map.get(name), left, right))
        else:
            configurations.append(left if left is not None else right)
    configurations.extend(ours_extras)
    configurations.extend(theirs_extras)
    merged = OrderedDict(ours)
    merged["configurations"] = configurations
    for key in list(theirs):
        if key in ("configurations", "inputs", "compounds"):
            continue
        if key not in merged:
            merged[key] = theirs[key]
        elif merged[key] != theirs[key]:
            merged[key] = merge_value(base.get(key), merged[key], theirs[key], [path, key])
    if "inputs" in ours or "inputs" in theirs:
        merged["inputs"] = merge_inputs(base.get("inputs") or [], ours.get("inputs") or [], theirs.get("inputs") or [])
    if "compounds" in ours or "compounds" in theirs:
        merged["compounds"] = merge_named(base.get("compounds") or [], ours.get("compounds") or [], theirs.get("compounds") or [], "name")
    only_ours = [name for name in ours_map if name not in theirs_map]
    only_theirs = [name for name in theirs_map if name not in ours_map]
    REPORT.append(f"{path}: configs {len(configurations)} kept upstream-only {len(only_ours)} stashed-only {len(only_theirs)}")
    write(path, dump_json(merged))


def input_key(item):
    if isinstance(item, dict) and item.get("id"):
        return item["id"]
    return json.dumps(item, ensure_ascii=False, sort_keys=True)


def merge_inputs(base, ours, theirs):
    base_map = {input_key(item): item for item in base}
    ours_map = OrderedDict((input_key(item), item) for item in ours)
    theirs_map = OrderedDict((input_key(item), item) for item in theirs)
    order = insert_missing(list(ours_map), list(theirs_map))
    merged = []
    for key in order:
        if key in ours_map and key in theirs_map:
            merged.append(merge_value(base_map.get(key), ours_map[key], theirs_map[key], ["inputs", str(key)]))
        elif key in ours_map:
            merged.append(ours_map[key])
        else:
            merged.append(theirs_map[key])
    return merged


def merge_named(base, ours, theirs, field):
    def keyed(items):
        mapped = OrderedDict()
        for item in items:
            mapped[item.get(field)] = item
        return mapped

    base_map, ours_map, theirs_map = keyed(base), keyed(ours), keyed(theirs)
    order = insert_missing(list(ours_map), list(theirs_map))
    merged = []
    for key in order:
        if key in ours_map and key in theirs_map:
            merged.append(merge_value(base_map.get(key), ours_map[key], theirs_map[key], [field, str(key)]))
        elif key in ours_map:
            merged.append(ours_map[key])
        else:
            merged.append(theirs_map[key])
    return merged


def merge_oracles(path):
    base = parse_jsonc(show(path, 1))
    ours = parse_jsonc(show(path, 2))
    theirs = parse_jsonc(show(path, 3))
    merged = OrderedDict(ours)
    merged["oracles"] = merge_named(base.get("oracles") or [], ours.get("oracles") or [], theirs.get("oracles") or [], "id")
    ids = [item.get("id") for item in merged["oracles"]]
    REPORT.append(f"{path}: oracles {len(ids)}")
    write(path, dump_json(merged))


def parse_cargo(raw):
    text = raw.decode("utf-8") if isinstance(raw, bytes) else raw
    parts = re.split(r"(?=^\[\[package\]\]$)", text, flags=re.M)
    header = parts[0]
    packages = []
    for part in parts[1:]:
        packages.append(parse_package(part.strip() + "\n"))
    return header, packages


def parse_package(block):
    fields = OrderedDict()
    lines = block.splitlines()
    index = 1
    while index < len(lines):
        line = lines[index]
        if line.startswith("dependencies = ["):
            deps = []
            index += 1
            while index < len(lines) and lines[index].strip() != "]":
                item = lines[index].strip().strip(",")
                if item.startswith('"') and item.endswith('"'):
                    deps.append(json.loads(item))
                index += 1
            fields["dependencies"] = deps
        elif "=" in line:
            key, value = line.split("=", 1)
            fields[key.strip()] = json.loads(value.strip())
        index += 1
    return fields


def package_key(package):
    return (package.get("name"), package.get("version"), package.get("source", ""))


def emit_package(package):
    lines = ["[[package]]", f"name = {json.dumps(package['name'], ensure_ascii=False)}", f"version = {json.dumps(package['version'], ensure_ascii=False)}"]
    for key in ("source", "checksum"):
        if key in package:
            lines.append(f"{key} = {json.dumps(package[key], ensure_ascii=False)}")
    if package.get("dependencies"):
        lines.append("dependencies = [")
        for dep in package["dependencies"]:
            lines.append(f" {json.dumps(dep, ensure_ascii=False)},")
        lines.append("]")
    return "\n".join(lines)


def merge_cargo(path):
    _, base_packages = parse_cargo(show(path, 1))
    header, ours_packages = parse_cargo(show(path, 2))
    _, theirs_packages = parse_cargo(show(path, 3))
    base = {package_key(item): item for item in base_packages}
    ours = {package_key(item): item for item in ours_packages}
    theirs = {package_key(item): item for item in theirs_packages}
    keys = list(ours)
    for key in theirs:
        if key not in ours:
            keys.append(key)
    keys.sort(key=lambda item: (item[0] or "", item[1] or "", item[2] or ""))
    merged = []
    for key in keys:
        if key in ours and key in theirs:
            left, right = ours[key], theirs[key]
            package = OrderedDict(left)
            if left.get("checksum") != right.get("checksum") and "checksum" in left and "checksum" in right:
                REPORT.append(f"HARD cargo checksum {key}")
            deps = union_list(base.get(key, {}).get("dependencies") or [], left.get("dependencies") or [], right.get("dependencies") or [])
            if deps:
                package["dependencies"] = deps
            elif "dependencies" in package and not deps:
                package.pop("dependencies", None)
            merged.append(package)
        else:
            merged.append(ours.get(key) or theirs[key])
    body = header.rstrip() + "\n\n" + "\n\n".join(emit_package(item) for item in merged) + "\n"
    REPORT.append(f"{path}: packages {len(merged)} upstream {len(ours)} stashed {len(theirs)}")
    write(path, body)


def merge_bun(path):
    base = parse_jsonc(show(path, 1))
    ours = parse_jsonc(show(path, 2))
    theirs = parse_jsonc(show(path, 3))
    merged = merge_value(base, ours, theirs, ["bun.lock"])
    REPORT.append(f"{path}: workspaces {len(merged.get('workspaces') or {})}")
    write(path, dump_json(merged))


def write(path, text):
    target = os.path.join(ROOT, path)
    with open(target, "w", encoding="utf-8", newline="\n") as handle:
        handle.write(text)


def main():
    os.chdir(ROOT)
    merge_launch(".vscode/launch.json")
    merge_launch(".vscode/🧩️launch.seed.jsonc")
    merge_launch(".claude/launch.json")
    merge_oracles("🧰️framework/🔨️modules/🖱️ui/🔮️oracles/🔣️.json")
    merge_cargo("Cargo.lock")
    merge_bun("bun.lock")
    hard = [line for line in REPORT if line.startswith("HARD")]
    print("\n".join(REPORT))
    print(f"hard {len(hard)}")
    if hard:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
