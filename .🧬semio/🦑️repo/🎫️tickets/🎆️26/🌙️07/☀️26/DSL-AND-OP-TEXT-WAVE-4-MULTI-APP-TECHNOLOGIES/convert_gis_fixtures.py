#!/usr/bin/env python3
"""One-off conversion of gis/2d + gis/3d JSON example fixtures into the new .gismap/.gisterrain
DSL text formats implemented in gis/plugin/rs/lib.rs (domain::gis_map_text / domain::gis_terrain_text).
Scratch script for ticket DSL-AND-OP-TEXT-WAVE-4-MULTI-APP-TECHNOLOGIES — not part of the build."""
import json

def quote(s: str) -> str:
    out = ['"']
    for ch in s:
        if ch == '\\':
            out.append('\\\\')
        elif ch == '"':
            out.append('\\"')
        elif ch == '\n':
            out.append('\\n')
        else:
            out.append(ch)
    out.append('"')
    return ''.join(out)

def print_num(v):
    if isinstance(v, bool):
        raise TypeError("bool handled separately")
    if isinstance(v, int):
        return str(v)
    # float: repr gives a valid, round-trippable decimal literal
    return repr(v)

def print_value(v) -> str:
    if v is None:
        return "null"
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, (int, float)):
        return print_num(v)
    if isinstance(v, str):
        return quote(v)
    if isinstance(v, list):
        return "[" + " ".join(print_value(x) for x in v) + "]"
    if isinstance(v, dict):
        return "{ " + " ".join(f"{k}={print_value(val)}" for k, val in v.items()) + " }"
    raise TypeError(f"unsupported value type: {type(v)}")

def print_feature(kind: str, item: dict) -> str:
    feature_id = item["id"]
    return f'{kind} {quote(feature_id)} {print_value(item)}'

def convert_map():
    src = "/Users/ueli/Documents/semio/gis/2d/example/reuse.map.gis.json"
    dst = "/Users/ueli/Documents/semio/gis/2d/example/reuse.map.gismap"
    with open(src) as f:
        data = json.load(f)
    lines = []
    for p in data.get("positions", []):
        lines.append(print_feature("position", p))
    for r in data.get("routes", []):
        lines.append(print_feature("route", r))
    for r in data.get("regions", []):
        lines.append(print_feature("region", r))
    with open(dst, "w") as f:
        f.write("\n".join(lines) + "\n")
    print(f"wrote {dst}: {len(lines)} feature lines")

def convert_terrain():
    src = "/Users/ueli/Documents/semio/gis/3d/example/reuse.terrain.gis.json"
    dst = "/Users/ueli/Documents/semio/gis/3d/example/reuse.terrain.gisterrain"
    with open(src) as f:
        data = json.load(f)
    lines = [f'gisterrain exaggeration={print_num(data["exaggeration"])}']
    origin = data["projectOrigin"]
    lines.append(f'origin lon={print_num(origin["lon"])} lat={print_num(origin["lat"])}')
    for p in data.get("positions", []):
        tokens = [f'position id={p["id"]}', f'lon={print_num(p["lon"])}', f'lat={print_num(p["lat"])}']
        if "label" in p:
            tokens.append(f'label={quote(p["label"])}')
        if "icon" in p:
            tokens.append(f'icon={p["icon"]}')
        lines.append(" ".join(tokens))
    with open(dst, "w") as f:
        f.write("\n".join(lines) + "\n")
    print(f"wrote {dst}: {len(lines)} lines")

if __name__ == "__main__":
    convert_map()
    convert_terrain()
